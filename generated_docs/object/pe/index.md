*[object](../index.md) / [pe](index.md)*

---

# Module `pe`

PE/COFF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on "winnt.h" (10.0.17763.0).

## Contents

- [Structs](#structs)
  - [`ImageDosHeader`](#imagedosheader)
  - [`ImageOs2Header`](#imageos2header)
  - [`ImageVxdHeader`](#imagevxdheader)
  - [`MaskedRichHeaderEntry`](#maskedrichheaderentry)
  - [`ImageFileHeader`](#imagefileheader)
  - [`ImageDataDirectory`](#imagedatadirectory)
  - [`ImageOptionalHeader32`](#imageoptionalheader32)
  - [`ImageRomOptionalHeader`](#imageromoptionalheader)
  - [`ImageOptionalHeader64`](#imageoptionalheader64)
  - [`ImageNtHeaders64`](#imagentheaders64)
  - [`ImageNtHeaders32`](#imagentheaders32)
  - [`ImageRomHeaders`](#imageromheaders)
  - [`Guid`](#guid)
  - [`ClsId`](#clsid)
  - [`AnonObjectHeader`](#anonobjectheader)
  - [`AnonObjectHeaderV2`](#anonobjectheaderv2)
  - [`AnonObjectHeaderBigobj`](#anonobjectheaderbigobj)
  - [`ImageSectionHeader`](#imagesectionheader)
  - [`ImageSymbol`](#imagesymbol)
  - [`ImageSymbolBytes`](#imagesymbolbytes)
  - [`ImageSymbolEx`](#imagesymbolex)
  - [`ImageSymbolExBytes`](#imagesymbolexbytes)
  - [`ImageAuxSymbolTokenDef`](#imageauxsymboltokendef)
  - [`ImageAuxSymbolFunction`](#imageauxsymbolfunction)
  - [`ImageAuxSymbolFunctionBeginEnd`](#imageauxsymbolfunctionbeginend)
  - [`ImageAuxSymbolWeak`](#imageauxsymbolweak)
  - [`ImageAuxSymbolSection`](#imageauxsymbolsection)
  - [`ImageAuxSymbolCrc`](#imageauxsymbolcrc)
  - [`ImageRelocation`](#imagerelocation)
  - [`ImageLinenumber`](#imagelinenumber)
  - [`ImageBaseRelocation`](#imagebaserelocation)
  - [`ImageArchiveMemberHeader`](#imagearchivememberheader)
  - [`ImageExportDirectory`](#imageexportdirectory)
  - [`ImageImportByName`](#imageimportbyname)
  - [`ImageThunkData64`](#imagethunkdata64)
  - [`ImageThunkData32`](#imagethunkdata32)
  - [`ImageTlsDirectory64`](#imagetlsdirectory64)
  - [`ImageTlsDirectory32`](#imagetlsdirectory32)
  - [`ImageImportDescriptor`](#imageimportdescriptor)
  - [`ImageBoundImportDescriptor`](#imageboundimportdescriptor)
  - [`ImageBoundForwarderRef`](#imageboundforwarderref)
  - [`ImageDelayloadDescriptor`](#imagedelayloaddescriptor)
  - [`ImageResourceDirectory`](#imageresourcedirectory)
  - [`ImageResourceDirectoryEntry`](#imageresourcedirectoryentry)
  - [`ImageResourceDirectoryString`](#imageresourcedirectorystring)
  - [`ImageResourceDirStringU`](#imageresourcedirstringu)
  - [`ImageResourceDataEntry`](#imageresourcedataentry)
  - [`ImageLoadConfigCodeIntegrity`](#imageloadconfigcodeintegrity)
  - [`ImageDynamicRelocationTable`](#imagedynamicrelocationtable)
  - [`ImageDynamicRelocation32`](#imagedynamicrelocation32)
  - [`ImageDynamicRelocation64`](#imagedynamicrelocation64)
  - [`ImageDynamicRelocation32V2`](#imagedynamicrelocation32v2)
  - [`ImageDynamicRelocation64V2`](#imagedynamicrelocation64v2)
  - [`ImagePrologueDynamicRelocationHeader`](#imageprologuedynamicrelocationheader)
  - [`ImageEpilogueDynamicRelocationHeader`](#imageepiloguedynamicrelocationheader)
  - [`ImageLoadConfigDirectory32`](#imageloadconfigdirectory32)
  - [`ImageLoadConfigDirectory64`](#imageloadconfigdirectory64)
  - [`ImageHotPatchInfo`](#imagehotpatchinfo)
  - [`ImageHotPatchBase`](#imagehotpatchbase)
  - [`ImageHotPatchHashes`](#imagehotpatchhashes)
  - [`ImageArmRuntimeFunctionEntry`](#imagearmruntimefunctionentry)
  - [`ImageArm64RuntimeFunctionEntry`](#imagearm64runtimefunctionentry)
  - [`ImageAlpha64RuntimeFunctionEntry`](#imagealpha64runtimefunctionentry)
  - [`ImageAlphaRuntimeFunctionEntry`](#imagealpharuntimefunctionentry)
  - [`ImageRuntimeFunctionEntry`](#imageruntimefunctionentry)
  - [`ImageEnclaveConfig32`](#imageenclaveconfig32)
  - [`ImageEnclaveConfig64`](#imageenclaveconfig64)
  - [`ImageEnclaveImport`](#imageenclaveimport)
  - [`ImageDebugDirectory`](#imagedebugdirectory)
  - [`ImageCoffSymbolsHeader`](#imagecoffsymbolsheader)
  - [`ImageDebugMisc`](#imagedebugmisc)
  - [`ImageFunctionEntry`](#imagefunctionentry)
  - [`ImageFunctionEntry64`](#imagefunctionentry64)
  - [`ImageSeparateDebugHeader`](#imageseparatedebugheader)
  - [`NonPagedDebugInfo`](#nonpageddebuginfo)
  - [`ImageArchitectureEntry`](#imagearchitectureentry)
  - [`ImportObjectHeader`](#importobjectheader)
  - [`ImageCor20Header`](#imagecor20header)
- [Constants](#constants)
  - [`IMAGE_DOS_SIGNATURE`](#image-dos-signature)
  - [`IMAGE_OS2_SIGNATURE`](#image-os2-signature)
  - [`IMAGE_OS2_SIGNATURE_LE`](#image-os2-signature-le)
  - [`IMAGE_VXD_SIGNATURE`](#image-vxd-signature)
  - [`IMAGE_NT_SIGNATURE`](#image-nt-signature)
  - [`IMAGE_SIZEOF_FILE_HEADER`](#image-sizeof-file-header)
  - [`IMAGE_FILE_RELOCS_STRIPPED`](#image-file-relocs-stripped)
  - [`IMAGE_FILE_EXECUTABLE_IMAGE`](#image-file-executable-image)
  - [`IMAGE_FILE_LINE_NUMS_STRIPPED`](#image-file-line-nums-stripped)
  - [`IMAGE_FILE_LOCAL_SYMS_STRIPPED`](#image-file-local-syms-stripped)
  - [`IMAGE_FILE_AGGRESIVE_WS_TRIM`](#image-file-aggresive-ws-trim)
  - [`IMAGE_FILE_LARGE_ADDRESS_AWARE`](#image-file-large-address-aware)
  - [`IMAGE_FILE_BYTES_REVERSED_LO`](#image-file-bytes-reversed-lo)
  - [`IMAGE_FILE_32BIT_MACHINE`](#image-file-32bit-machine)
  - [`IMAGE_FILE_DEBUG_STRIPPED`](#image-file-debug-stripped)
  - [`IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP`](#image-file-removable-run-from-swap)
  - [`IMAGE_FILE_NET_RUN_FROM_SWAP`](#image-file-net-run-from-swap)
  - [`IMAGE_FILE_SYSTEM`](#image-file-system)
  - [`IMAGE_FILE_DLL`](#image-file-dll)
  - [`IMAGE_FILE_UP_SYSTEM_ONLY`](#image-file-up-system-only)
  - [`IMAGE_FILE_BYTES_REVERSED_HI`](#image-file-bytes-reversed-hi)
  - [`IMAGE_FILE_MACHINE_UNKNOWN`](#image-file-machine-unknown)
  - [`IMAGE_FILE_MACHINE_TARGET_HOST`](#image-file-machine-target-host)
  - [`IMAGE_FILE_MACHINE_I386`](#image-file-machine-i386)
  - [`IMAGE_FILE_MACHINE_R3000`](#image-file-machine-r3000)
  - [`IMAGE_FILE_MACHINE_R4000`](#image-file-machine-r4000)
  - [`IMAGE_FILE_MACHINE_R10000`](#image-file-machine-r10000)
  - [`IMAGE_FILE_MACHINE_WCEMIPSV2`](#image-file-machine-wcemipsv2)
  - [`IMAGE_FILE_MACHINE_ALPHA`](#image-file-machine-alpha)
  - [`IMAGE_FILE_MACHINE_SH3`](#image-file-machine-sh3)
  - [`IMAGE_FILE_MACHINE_SH3DSP`](#image-file-machine-sh3dsp)
  - [`IMAGE_FILE_MACHINE_SH3E`](#image-file-machine-sh3e)
  - [`IMAGE_FILE_MACHINE_SH4`](#image-file-machine-sh4)
  - [`IMAGE_FILE_MACHINE_SH5`](#image-file-machine-sh5)
  - [`IMAGE_FILE_MACHINE_ARM`](#image-file-machine-arm)
  - [`IMAGE_FILE_MACHINE_THUMB`](#image-file-machine-thumb)
  - [`IMAGE_FILE_MACHINE_ARMNT`](#image-file-machine-armnt)
  - [`IMAGE_FILE_MACHINE_AM33`](#image-file-machine-am33)
  - [`IMAGE_FILE_MACHINE_POWERPC`](#image-file-machine-powerpc)
  - [`IMAGE_FILE_MACHINE_POWERPCFP`](#image-file-machine-powerpcfp)
  - [`IMAGE_FILE_MACHINE_POWERPCBE`](#image-file-machine-powerpcbe)
  - [`IMAGE_FILE_MACHINE_IA64`](#image-file-machine-ia64)
  - [`IMAGE_FILE_MACHINE_MIPS16`](#image-file-machine-mips16)
  - [`IMAGE_FILE_MACHINE_ALPHA64`](#image-file-machine-alpha64)
  - [`IMAGE_FILE_MACHINE_MIPSFPU`](#image-file-machine-mipsfpu)
  - [`IMAGE_FILE_MACHINE_MIPSFPU16`](#image-file-machine-mipsfpu16)
  - [`IMAGE_FILE_MACHINE_AXP64`](#image-file-machine-axp64)
  - [`IMAGE_FILE_MACHINE_TRICORE`](#image-file-machine-tricore)
  - [`IMAGE_FILE_MACHINE_CEF`](#image-file-machine-cef)
  - [`IMAGE_FILE_MACHINE_EBC`](#image-file-machine-ebc)
  - [`IMAGE_FILE_MACHINE_AMD64`](#image-file-machine-amd64)
  - [`IMAGE_FILE_MACHINE_M32R`](#image-file-machine-m32r)
  - [`IMAGE_FILE_MACHINE_ARM64`](#image-file-machine-arm64)
  - [`IMAGE_FILE_MACHINE_ARM64EC`](#image-file-machine-arm64ec)
  - [`IMAGE_FILE_MACHINE_CEE`](#image-file-machine-cee)
  - [`IMAGE_FILE_MACHINE_RISCV32`](#image-file-machine-riscv32)
  - [`IMAGE_FILE_MACHINE_RISCV64`](#image-file-machine-riscv64)
  - [`IMAGE_FILE_MACHINE_RISCV128`](#image-file-machine-riscv128)
  - [`IMAGE_FILE_MACHINE_ARM64X`](#image-file-machine-arm64x)
  - [`IMAGE_FILE_MACHINE_CHPE_X86`](#image-file-machine-chpe-x86)
  - [`IMAGE_NUMBEROF_DIRECTORY_ENTRIES`](#image-numberof-directory-entries)
  - [`IMAGE_NT_OPTIONAL_HDR32_MAGIC`](#image-nt-optional-hdr32-magic)
  - [`IMAGE_NT_OPTIONAL_HDR64_MAGIC`](#image-nt-optional-hdr64-magic)
  - [`IMAGE_ROM_OPTIONAL_HDR_MAGIC`](#image-rom-optional-hdr-magic)
  - [`IMAGE_SUBSYSTEM_UNKNOWN`](#image-subsystem-unknown)
  - [`IMAGE_SUBSYSTEM_NATIVE`](#image-subsystem-native)
  - [`IMAGE_SUBSYSTEM_WINDOWS_GUI`](#image-subsystem-windows-gui)
  - [`IMAGE_SUBSYSTEM_WINDOWS_CUI`](#image-subsystem-windows-cui)
  - [`IMAGE_SUBSYSTEM_OS2_CUI`](#image-subsystem-os2-cui)
  - [`IMAGE_SUBSYSTEM_POSIX_CUI`](#image-subsystem-posix-cui)
  - [`IMAGE_SUBSYSTEM_NATIVE_WINDOWS`](#image-subsystem-native-windows)
  - [`IMAGE_SUBSYSTEM_WINDOWS_CE_GUI`](#image-subsystem-windows-ce-gui)
  - [`IMAGE_SUBSYSTEM_EFI_APPLICATION`](#image-subsystem-efi-application)
  - [`IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER`](#image-subsystem-efi-boot-service-driver)
  - [`IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER`](#image-subsystem-efi-runtime-driver)
  - [`IMAGE_SUBSYSTEM_EFI_ROM`](#image-subsystem-efi-rom)
  - [`IMAGE_SUBSYSTEM_XBOX`](#image-subsystem-xbox)
  - [`IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION`](#image-subsystem-windows-boot-application)
  - [`IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG`](#image-subsystem-xbox-code-catalog)
  - [`IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA`](#image-dllcharacteristics-high-entropy-va)
  - [`IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE`](#image-dllcharacteristics-dynamic-base)
  - [`IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY`](#image-dllcharacteristics-force-integrity)
  - [`IMAGE_DLLCHARACTERISTICS_NX_COMPAT`](#image-dllcharacteristics-nx-compat)
  - [`IMAGE_DLLCHARACTERISTICS_NO_ISOLATION`](#image-dllcharacteristics-no-isolation)
  - [`IMAGE_DLLCHARACTERISTICS_NO_SEH`](#image-dllcharacteristics-no-seh)
  - [`IMAGE_DLLCHARACTERISTICS_NO_BIND`](#image-dllcharacteristics-no-bind)
  - [`IMAGE_DLLCHARACTERISTICS_APPCONTAINER`](#image-dllcharacteristics-appcontainer)
  - [`IMAGE_DLLCHARACTERISTICS_WDM_DRIVER`](#image-dllcharacteristics-wdm-driver)
  - [`IMAGE_DLLCHARACTERISTICS_GUARD_CF`](#image-dllcharacteristics-guard-cf)
  - [`IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE`](#image-dllcharacteristics-terminal-server-aware)
  - [`IMAGE_DIRECTORY_ENTRY_EXPORT`](#image-directory-entry-export)
  - [`IMAGE_DIRECTORY_ENTRY_IMPORT`](#image-directory-entry-import)
  - [`IMAGE_DIRECTORY_ENTRY_RESOURCE`](#image-directory-entry-resource)
  - [`IMAGE_DIRECTORY_ENTRY_EXCEPTION`](#image-directory-entry-exception)
  - [`IMAGE_DIRECTORY_ENTRY_SECURITY`](#image-directory-entry-security)
  - [`IMAGE_DIRECTORY_ENTRY_BASERELOC`](#image-directory-entry-basereloc)
  - [`IMAGE_DIRECTORY_ENTRY_DEBUG`](#image-directory-entry-debug)
  - [`IMAGE_DIRECTORY_ENTRY_ARCHITECTURE`](#image-directory-entry-architecture)
  - [`IMAGE_DIRECTORY_ENTRY_GLOBALPTR`](#image-directory-entry-globalptr)
  - [`IMAGE_DIRECTORY_ENTRY_TLS`](#image-directory-entry-tls)
  - [`IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG`](#image-directory-entry-load-config)
  - [`IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT`](#image-directory-entry-bound-import)
  - [`IMAGE_DIRECTORY_ENTRY_IAT`](#image-directory-entry-iat)
  - [`IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`](#image-directory-entry-delay-import)
  - [`IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`](#image-directory-entry-com-descriptor)
  - [`ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`](#anon-object-header-bigobj-class-id)
  - [`IMAGE_SIZEOF_SHORT_NAME`](#image-sizeof-short-name)
  - [`IMAGE_SIZEOF_SECTION_HEADER`](#image-sizeof-section-header)
  - [`IMAGE_SCN_TYPE_NO_PAD`](#image-scn-type-no-pad)
  - [`IMAGE_SCN_CNT_CODE`](#image-scn-cnt-code)
  - [`IMAGE_SCN_CNT_INITIALIZED_DATA`](#image-scn-cnt-initialized-data)
  - [`IMAGE_SCN_CNT_UNINITIALIZED_DATA`](#image-scn-cnt-uninitialized-data)
  - [`IMAGE_SCN_LNK_OTHER`](#image-scn-lnk-other)
  - [`IMAGE_SCN_LNK_INFO`](#image-scn-lnk-info)
  - [`IMAGE_SCN_LNK_REMOVE`](#image-scn-lnk-remove)
  - [`IMAGE_SCN_LNK_COMDAT`](#image-scn-lnk-comdat)
  - [`IMAGE_SCN_NO_DEFER_SPEC_EXC`](#image-scn-no-defer-spec-exc)
  - [`IMAGE_SCN_GPREL`](#image-scn-gprel)
  - [`IMAGE_SCN_MEM_FARDATA`](#image-scn-mem-fardata)
  - [`IMAGE_SCN_MEM_PURGEABLE`](#image-scn-mem-purgeable)
  - [`IMAGE_SCN_MEM_16BIT`](#image-scn-mem-16bit)
  - [`IMAGE_SCN_MEM_LOCKED`](#image-scn-mem-locked)
  - [`IMAGE_SCN_MEM_PRELOAD`](#image-scn-mem-preload)
  - [`IMAGE_SCN_ALIGN_1BYTES`](#image-scn-align-1bytes)
  - [`IMAGE_SCN_ALIGN_2BYTES`](#image-scn-align-2bytes)
  - [`IMAGE_SCN_ALIGN_4BYTES`](#image-scn-align-4bytes)
  - [`IMAGE_SCN_ALIGN_8BYTES`](#image-scn-align-8bytes)
  - [`IMAGE_SCN_ALIGN_16BYTES`](#image-scn-align-16bytes)
  - [`IMAGE_SCN_ALIGN_32BYTES`](#image-scn-align-32bytes)
  - [`IMAGE_SCN_ALIGN_64BYTES`](#image-scn-align-64bytes)
  - [`IMAGE_SCN_ALIGN_128BYTES`](#image-scn-align-128bytes)
  - [`IMAGE_SCN_ALIGN_256BYTES`](#image-scn-align-256bytes)
  - [`IMAGE_SCN_ALIGN_512BYTES`](#image-scn-align-512bytes)
  - [`IMAGE_SCN_ALIGN_1024BYTES`](#image-scn-align-1024bytes)
  - [`IMAGE_SCN_ALIGN_2048BYTES`](#image-scn-align-2048bytes)
  - [`IMAGE_SCN_ALIGN_4096BYTES`](#image-scn-align-4096bytes)
  - [`IMAGE_SCN_ALIGN_8192BYTES`](#image-scn-align-8192bytes)
  - [`IMAGE_SCN_ALIGN_MASK`](#image-scn-align-mask)
  - [`IMAGE_SCN_LNK_NRELOC_OVFL`](#image-scn-lnk-nreloc-ovfl)
  - [`IMAGE_SCN_MEM_DISCARDABLE`](#image-scn-mem-discardable)
  - [`IMAGE_SCN_MEM_NOT_CACHED`](#image-scn-mem-not-cached)
  - [`IMAGE_SCN_MEM_NOT_PAGED`](#image-scn-mem-not-paged)
  - [`IMAGE_SCN_MEM_SHARED`](#image-scn-mem-shared)
  - [`IMAGE_SCN_MEM_EXECUTE`](#image-scn-mem-execute)
  - [`IMAGE_SCN_MEM_READ`](#image-scn-mem-read)
  - [`IMAGE_SCN_MEM_WRITE`](#image-scn-mem-write)
  - [`IMAGE_SCN_SCALE_INDEX`](#image-scn-scale-index)
  - [`IMAGE_SIZEOF_SYMBOL`](#image-sizeof-symbol)
  - [`IMAGE_SIZEOF_SYMBOL_EX`](#image-sizeof-symbol-ex)
  - [`IMAGE_SYM_UNDEFINED`](#image-sym-undefined)
  - [`IMAGE_SYM_ABSOLUTE`](#image-sym-absolute)
  - [`IMAGE_SYM_DEBUG`](#image-sym-debug)
  - [`IMAGE_SYM_SECTION_MAX`](#image-sym-section-max)
  - [`IMAGE_SYM_SECTION_MAX_EX`](#image-sym-section-max-ex)
  - [`IMAGE_SYM_TYPE_NULL`](#image-sym-type-null)
  - [`IMAGE_SYM_TYPE_VOID`](#image-sym-type-void)
  - [`IMAGE_SYM_TYPE_CHAR`](#image-sym-type-char)
  - [`IMAGE_SYM_TYPE_SHORT`](#image-sym-type-short)
  - [`IMAGE_SYM_TYPE_INT`](#image-sym-type-int)
  - [`IMAGE_SYM_TYPE_LONG`](#image-sym-type-long)
  - [`IMAGE_SYM_TYPE_FLOAT`](#image-sym-type-float)
  - [`IMAGE_SYM_TYPE_DOUBLE`](#image-sym-type-double)
  - [`IMAGE_SYM_TYPE_STRUCT`](#image-sym-type-struct)
  - [`IMAGE_SYM_TYPE_UNION`](#image-sym-type-union)
  - [`IMAGE_SYM_TYPE_ENUM`](#image-sym-type-enum)
  - [`IMAGE_SYM_TYPE_MOE`](#image-sym-type-moe)
  - [`IMAGE_SYM_TYPE_BYTE`](#image-sym-type-byte)
  - [`IMAGE_SYM_TYPE_WORD`](#image-sym-type-word)
  - [`IMAGE_SYM_TYPE_UINT`](#image-sym-type-uint)
  - [`IMAGE_SYM_TYPE_DWORD`](#image-sym-type-dword)
  - [`IMAGE_SYM_TYPE_PCODE`](#image-sym-type-pcode)
  - [`IMAGE_SYM_DTYPE_NULL`](#image-sym-dtype-null)
  - [`IMAGE_SYM_DTYPE_POINTER`](#image-sym-dtype-pointer)
  - [`IMAGE_SYM_DTYPE_FUNCTION`](#image-sym-dtype-function)
  - [`IMAGE_SYM_DTYPE_ARRAY`](#image-sym-dtype-array)
  - [`IMAGE_SYM_CLASS_END_OF_FUNCTION`](#image-sym-class-end-of-function)
  - [`IMAGE_SYM_CLASS_NULL`](#image-sym-class-null)
  - [`IMAGE_SYM_CLASS_AUTOMATIC`](#image-sym-class-automatic)
  - [`IMAGE_SYM_CLASS_EXTERNAL`](#image-sym-class-external)
  - [`IMAGE_SYM_CLASS_STATIC`](#image-sym-class-static)
  - [`IMAGE_SYM_CLASS_REGISTER`](#image-sym-class-register)
  - [`IMAGE_SYM_CLASS_EXTERNAL_DEF`](#image-sym-class-external-def)
  - [`IMAGE_SYM_CLASS_LABEL`](#image-sym-class-label)
  - [`IMAGE_SYM_CLASS_UNDEFINED_LABEL`](#image-sym-class-undefined-label)
  - [`IMAGE_SYM_CLASS_MEMBER_OF_STRUCT`](#image-sym-class-member-of-struct)
  - [`IMAGE_SYM_CLASS_ARGUMENT`](#image-sym-class-argument)
  - [`IMAGE_SYM_CLASS_STRUCT_TAG`](#image-sym-class-struct-tag)
  - [`IMAGE_SYM_CLASS_MEMBER_OF_UNION`](#image-sym-class-member-of-union)
  - [`IMAGE_SYM_CLASS_UNION_TAG`](#image-sym-class-union-tag)
  - [`IMAGE_SYM_CLASS_TYPE_DEFINITION`](#image-sym-class-type-definition)
  - [`IMAGE_SYM_CLASS_UNDEFINED_STATIC`](#image-sym-class-undefined-static)
  - [`IMAGE_SYM_CLASS_ENUM_TAG`](#image-sym-class-enum-tag)
  - [`IMAGE_SYM_CLASS_MEMBER_OF_ENUM`](#image-sym-class-member-of-enum)
  - [`IMAGE_SYM_CLASS_REGISTER_PARAM`](#image-sym-class-register-param)
  - [`IMAGE_SYM_CLASS_BIT_FIELD`](#image-sym-class-bit-field)
  - [`IMAGE_SYM_CLASS_FAR_EXTERNAL`](#image-sym-class-far-external)
  - [`IMAGE_SYM_CLASS_BLOCK`](#image-sym-class-block)
  - [`IMAGE_SYM_CLASS_FUNCTION`](#image-sym-class-function)
  - [`IMAGE_SYM_CLASS_END_OF_STRUCT`](#image-sym-class-end-of-struct)
  - [`IMAGE_SYM_CLASS_FILE`](#image-sym-class-file)
  - [`IMAGE_SYM_CLASS_SECTION`](#image-sym-class-section)
  - [`IMAGE_SYM_CLASS_WEAK_EXTERNAL`](#image-sym-class-weak-external)
  - [`IMAGE_SYM_CLASS_CLR_TOKEN`](#image-sym-class-clr-token)
  - [`N_BTMASK`](#n-btmask)
  - [`N_TMASK`](#n-tmask)
  - [`N_TMASK1`](#n-tmask1)
  - [`N_TMASK2`](#n-tmask2)
  - [`N_BTSHFT`](#n-btshft)
  - [`N_TSHIFT`](#n-tshift)
  - [`IMAGE_SYM_DTYPE_SHIFT`](#image-sym-dtype-shift)
  - [`IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF`](#image-aux-symbol-type-token-def)
  - [`IMAGE_COMDAT_SELECT_NODUPLICATES`](#image-comdat-select-noduplicates)
  - [`IMAGE_COMDAT_SELECT_ANY`](#image-comdat-select-any)
  - [`IMAGE_COMDAT_SELECT_SAME_SIZE`](#image-comdat-select-same-size)
  - [`IMAGE_COMDAT_SELECT_EXACT_MATCH`](#image-comdat-select-exact-match)
  - [`IMAGE_COMDAT_SELECT_ASSOCIATIVE`](#image-comdat-select-associative)
  - [`IMAGE_COMDAT_SELECT_LARGEST`](#image-comdat-select-largest)
  - [`IMAGE_COMDAT_SELECT_NEWEST`](#image-comdat-select-newest)
  - [`IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY`](#image-weak-extern-search-nolibrary)
  - [`IMAGE_WEAK_EXTERN_SEARCH_LIBRARY`](#image-weak-extern-search-library)
  - [`IMAGE_WEAK_EXTERN_SEARCH_ALIAS`](#image-weak-extern-search-alias)
  - [`IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY`](#image-weak-extern-anti-dependency)
  - [`IMAGE_REL_I386_ABSOLUTE`](#image-rel-i386-absolute)
  - [`IMAGE_REL_I386_DIR16`](#image-rel-i386-dir16)
  - [`IMAGE_REL_I386_REL16`](#image-rel-i386-rel16)
  - [`IMAGE_REL_I386_DIR32`](#image-rel-i386-dir32)
  - [`IMAGE_REL_I386_DIR32NB`](#image-rel-i386-dir32nb)
  - [`IMAGE_REL_I386_SEG12`](#image-rel-i386-seg12)
  - [`IMAGE_REL_I386_SECTION`](#image-rel-i386-section)
  - [`IMAGE_REL_I386_SECREL`](#image-rel-i386-secrel)
  - [`IMAGE_REL_I386_TOKEN`](#image-rel-i386-token)
  - [`IMAGE_REL_I386_SECREL7`](#image-rel-i386-secrel7)
  - [`IMAGE_REL_I386_REL32`](#image-rel-i386-rel32)
  - [`IMAGE_REL_MIPS_ABSOLUTE`](#image-rel-mips-absolute)
  - [`IMAGE_REL_MIPS_REFHALF`](#image-rel-mips-refhalf)
  - [`IMAGE_REL_MIPS_REFWORD`](#image-rel-mips-refword)
  - [`IMAGE_REL_MIPS_JMPADDR`](#image-rel-mips-jmpaddr)
  - [`IMAGE_REL_MIPS_REFHI`](#image-rel-mips-refhi)
  - [`IMAGE_REL_MIPS_REFLO`](#image-rel-mips-reflo)
  - [`IMAGE_REL_MIPS_GPREL`](#image-rel-mips-gprel)
  - [`IMAGE_REL_MIPS_LITERAL`](#image-rel-mips-literal)
  - [`IMAGE_REL_MIPS_SECTION`](#image-rel-mips-section)
  - [`IMAGE_REL_MIPS_SECREL`](#image-rel-mips-secrel)
  - [`IMAGE_REL_MIPS_SECRELLO`](#image-rel-mips-secrello)
  - [`IMAGE_REL_MIPS_SECRELHI`](#image-rel-mips-secrelhi)
  - [`IMAGE_REL_MIPS_TOKEN`](#image-rel-mips-token)
  - [`IMAGE_REL_MIPS_JMPADDR16`](#image-rel-mips-jmpaddr16)
  - [`IMAGE_REL_MIPS_REFWORDNB`](#image-rel-mips-refwordnb)
  - [`IMAGE_REL_MIPS_PAIR`](#image-rel-mips-pair)
  - [`IMAGE_REL_ALPHA_ABSOLUTE`](#image-rel-alpha-absolute)
  - [`IMAGE_REL_ALPHA_REFLONG`](#image-rel-alpha-reflong)
  - [`IMAGE_REL_ALPHA_REFQUAD`](#image-rel-alpha-refquad)
  - [`IMAGE_REL_ALPHA_GPREL32`](#image-rel-alpha-gprel32)
  - [`IMAGE_REL_ALPHA_LITERAL`](#image-rel-alpha-literal)
  - [`IMAGE_REL_ALPHA_LITUSE`](#image-rel-alpha-lituse)
  - [`IMAGE_REL_ALPHA_GPDISP`](#image-rel-alpha-gpdisp)
  - [`IMAGE_REL_ALPHA_BRADDR`](#image-rel-alpha-braddr)
  - [`IMAGE_REL_ALPHA_HINT`](#image-rel-alpha-hint)
  - [`IMAGE_REL_ALPHA_INLINE_REFLONG`](#image-rel-alpha-inline-reflong)
  - [`IMAGE_REL_ALPHA_REFHI`](#image-rel-alpha-refhi)
  - [`IMAGE_REL_ALPHA_REFLO`](#image-rel-alpha-reflo)
  - [`IMAGE_REL_ALPHA_PAIR`](#image-rel-alpha-pair)
  - [`IMAGE_REL_ALPHA_MATCH`](#image-rel-alpha-match)
  - [`IMAGE_REL_ALPHA_SECTION`](#image-rel-alpha-section)
  - [`IMAGE_REL_ALPHA_SECREL`](#image-rel-alpha-secrel)
  - [`IMAGE_REL_ALPHA_REFLONGNB`](#image-rel-alpha-reflongnb)
  - [`IMAGE_REL_ALPHA_SECRELLO`](#image-rel-alpha-secrello)
  - [`IMAGE_REL_ALPHA_SECRELHI`](#image-rel-alpha-secrelhi)
  - [`IMAGE_REL_ALPHA_REFQ3`](#image-rel-alpha-refq3)
  - [`IMAGE_REL_ALPHA_REFQ2`](#image-rel-alpha-refq2)
  - [`IMAGE_REL_ALPHA_REFQ1`](#image-rel-alpha-refq1)
  - [`IMAGE_REL_ALPHA_GPRELLO`](#image-rel-alpha-gprello)
  - [`IMAGE_REL_ALPHA_GPRELHI`](#image-rel-alpha-gprelhi)
  - [`IMAGE_REL_PPC_ABSOLUTE`](#image-rel-ppc-absolute)
  - [`IMAGE_REL_PPC_ADDR64`](#image-rel-ppc-addr64)
  - [`IMAGE_REL_PPC_ADDR32`](#image-rel-ppc-addr32)
  - [`IMAGE_REL_PPC_ADDR24`](#image-rel-ppc-addr24)
  - [`IMAGE_REL_PPC_ADDR16`](#image-rel-ppc-addr16)
  - [`IMAGE_REL_PPC_ADDR14`](#image-rel-ppc-addr14)
  - [`IMAGE_REL_PPC_REL24`](#image-rel-ppc-rel24)
  - [`IMAGE_REL_PPC_REL14`](#image-rel-ppc-rel14)
  - [`IMAGE_REL_PPC_TOCREL16`](#image-rel-ppc-tocrel16)
  - [`IMAGE_REL_PPC_TOCREL14`](#image-rel-ppc-tocrel14)
  - [`IMAGE_REL_PPC_ADDR32NB`](#image-rel-ppc-addr32nb)
  - [`IMAGE_REL_PPC_SECREL`](#image-rel-ppc-secrel)
  - [`IMAGE_REL_PPC_SECTION`](#image-rel-ppc-section)
  - [`IMAGE_REL_PPC_IFGLUE`](#image-rel-ppc-ifglue)
  - [`IMAGE_REL_PPC_IMGLUE`](#image-rel-ppc-imglue)
  - [`IMAGE_REL_PPC_SECREL16`](#image-rel-ppc-secrel16)
  - [`IMAGE_REL_PPC_REFHI`](#image-rel-ppc-refhi)
  - [`IMAGE_REL_PPC_REFLO`](#image-rel-ppc-reflo)
  - [`IMAGE_REL_PPC_PAIR`](#image-rel-ppc-pair)
  - [`IMAGE_REL_PPC_SECRELLO`](#image-rel-ppc-secrello)
  - [`IMAGE_REL_PPC_SECRELHI`](#image-rel-ppc-secrelhi)
  - [`IMAGE_REL_PPC_GPREL`](#image-rel-ppc-gprel)
  - [`IMAGE_REL_PPC_TOKEN`](#image-rel-ppc-token)
  - [`IMAGE_REL_PPC_TYPEMASK`](#image-rel-ppc-typemask)
  - [`IMAGE_REL_PPC_NEG`](#image-rel-ppc-neg)
  - [`IMAGE_REL_PPC_BRTAKEN`](#image-rel-ppc-brtaken)
  - [`IMAGE_REL_PPC_BRNTAKEN`](#image-rel-ppc-brntaken)
  - [`IMAGE_REL_PPC_TOCDEFN`](#image-rel-ppc-tocdefn)
  - [`IMAGE_REL_SH3_ABSOLUTE`](#image-rel-sh3-absolute)
  - [`IMAGE_REL_SH3_DIRECT16`](#image-rel-sh3-direct16)
  - [`IMAGE_REL_SH3_DIRECT32`](#image-rel-sh3-direct32)
  - [`IMAGE_REL_SH3_DIRECT8`](#image-rel-sh3-direct8)
  - [`IMAGE_REL_SH3_DIRECT8_WORD`](#image-rel-sh3-direct8-word)
  - [`IMAGE_REL_SH3_DIRECT8_LONG`](#image-rel-sh3-direct8-long)
  - [`IMAGE_REL_SH3_DIRECT4`](#image-rel-sh3-direct4)
  - [`IMAGE_REL_SH3_DIRECT4_WORD`](#image-rel-sh3-direct4-word)
  - [`IMAGE_REL_SH3_DIRECT4_LONG`](#image-rel-sh3-direct4-long)
  - [`IMAGE_REL_SH3_PCREL8_WORD`](#image-rel-sh3-pcrel8-word)
  - [`IMAGE_REL_SH3_PCREL8_LONG`](#image-rel-sh3-pcrel8-long)
  - [`IMAGE_REL_SH3_PCREL12_WORD`](#image-rel-sh3-pcrel12-word)
  - [`IMAGE_REL_SH3_STARTOF_SECTION`](#image-rel-sh3-startof-section)
  - [`IMAGE_REL_SH3_SIZEOF_SECTION`](#image-rel-sh3-sizeof-section)
  - [`IMAGE_REL_SH3_SECTION`](#image-rel-sh3-section)
  - [`IMAGE_REL_SH3_SECREL`](#image-rel-sh3-secrel)
  - [`IMAGE_REL_SH3_DIRECT32_NB`](#image-rel-sh3-direct32-nb)
  - [`IMAGE_REL_SH3_GPREL4_LONG`](#image-rel-sh3-gprel4-long)
  - [`IMAGE_REL_SH3_TOKEN`](#image-rel-sh3-token)
  - [`IMAGE_REL_SHM_PCRELPT`](#image-rel-shm-pcrelpt)
  - [`IMAGE_REL_SHM_REFLO`](#image-rel-shm-reflo)
  - [`IMAGE_REL_SHM_REFHALF`](#image-rel-shm-refhalf)
  - [`IMAGE_REL_SHM_RELLO`](#image-rel-shm-rello)
  - [`IMAGE_REL_SHM_RELHALF`](#image-rel-shm-relhalf)
  - [`IMAGE_REL_SHM_PAIR`](#image-rel-shm-pair)
  - [`IMAGE_REL_SH_NOMODE`](#image-rel-sh-nomode)
  - [`IMAGE_REL_ARM_ABSOLUTE`](#image-rel-arm-absolute)
  - [`IMAGE_REL_ARM_ADDR32`](#image-rel-arm-addr32)
  - [`IMAGE_REL_ARM_ADDR32NB`](#image-rel-arm-addr32nb)
  - [`IMAGE_REL_ARM_BRANCH24`](#image-rel-arm-branch24)
  - [`IMAGE_REL_ARM_BRANCH11`](#image-rel-arm-branch11)
  - [`IMAGE_REL_ARM_TOKEN`](#image-rel-arm-token)
  - [`IMAGE_REL_ARM_GPREL12`](#image-rel-arm-gprel12)
  - [`IMAGE_REL_ARM_GPREL7`](#image-rel-arm-gprel7)
  - [`IMAGE_REL_ARM_BLX24`](#image-rel-arm-blx24)
  - [`IMAGE_REL_ARM_BLX11`](#image-rel-arm-blx11)
  - [`IMAGE_REL_ARM_REL32`](#image-rel-arm-rel32)
  - [`IMAGE_REL_ARM_SECTION`](#image-rel-arm-section)
  - [`IMAGE_REL_ARM_SECREL`](#image-rel-arm-secrel)
  - [`IMAGE_REL_ARM_MOV32A`](#image-rel-arm-mov32a)
  - [`IMAGE_REL_ARM_MOV32`](#image-rel-arm-mov32)
  - [`IMAGE_REL_ARM_MOV32T`](#image-rel-arm-mov32t)
  - [`IMAGE_REL_THUMB_MOV32`](#image-rel-thumb-mov32)
  - [`IMAGE_REL_ARM_BRANCH20T`](#image-rel-arm-branch20t)
  - [`IMAGE_REL_THUMB_BRANCH20`](#image-rel-thumb-branch20)
  - [`IMAGE_REL_ARM_BRANCH24T`](#image-rel-arm-branch24t)
  - [`IMAGE_REL_THUMB_BRANCH24`](#image-rel-thumb-branch24)
  - [`IMAGE_REL_ARM_BLX23T`](#image-rel-arm-blx23t)
  - [`IMAGE_REL_THUMB_BLX23`](#image-rel-thumb-blx23)
  - [`IMAGE_REL_AM_ABSOLUTE`](#image-rel-am-absolute)
  - [`IMAGE_REL_AM_ADDR32`](#image-rel-am-addr32)
  - [`IMAGE_REL_AM_ADDR32NB`](#image-rel-am-addr32nb)
  - [`IMAGE_REL_AM_CALL32`](#image-rel-am-call32)
  - [`IMAGE_REL_AM_FUNCINFO`](#image-rel-am-funcinfo)
  - [`IMAGE_REL_AM_REL32_1`](#image-rel-am-rel32-1)
  - [`IMAGE_REL_AM_REL32_2`](#image-rel-am-rel32-2)
  - [`IMAGE_REL_AM_SECREL`](#image-rel-am-secrel)
  - [`IMAGE_REL_AM_SECTION`](#image-rel-am-section)
  - [`IMAGE_REL_AM_TOKEN`](#image-rel-am-token)
  - [`IMAGE_REL_ARM64_ABSOLUTE`](#image-rel-arm64-absolute)
  - [`IMAGE_REL_ARM64_ADDR32`](#image-rel-arm64-addr32)
  - [`IMAGE_REL_ARM64_ADDR32NB`](#image-rel-arm64-addr32nb)
  - [`IMAGE_REL_ARM64_BRANCH26`](#image-rel-arm64-branch26)
  - [`IMAGE_REL_ARM64_PAGEBASE_REL21`](#image-rel-arm64-pagebase-rel21)
  - [`IMAGE_REL_ARM64_REL21`](#image-rel-arm64-rel21)
  - [`IMAGE_REL_ARM64_PAGEOFFSET_12A`](#image-rel-arm64-pageoffset-12a)
  - [`IMAGE_REL_ARM64_PAGEOFFSET_12L`](#image-rel-arm64-pageoffset-12l)
  - [`IMAGE_REL_ARM64_SECREL`](#image-rel-arm64-secrel)
  - [`IMAGE_REL_ARM64_SECREL_LOW12A`](#image-rel-arm64-secrel-low12a)
  - [`IMAGE_REL_ARM64_SECREL_HIGH12A`](#image-rel-arm64-secrel-high12a)
  - [`IMAGE_REL_ARM64_SECREL_LOW12L`](#image-rel-arm64-secrel-low12l)
  - [`IMAGE_REL_ARM64_TOKEN`](#image-rel-arm64-token)
  - [`IMAGE_REL_ARM64_SECTION`](#image-rel-arm64-section)
  - [`IMAGE_REL_ARM64_ADDR64`](#image-rel-arm64-addr64)
  - [`IMAGE_REL_ARM64_BRANCH19`](#image-rel-arm64-branch19)
  - [`IMAGE_REL_ARM64_BRANCH14`](#image-rel-arm64-branch14)
  - [`IMAGE_REL_ARM64_REL32`](#image-rel-arm64-rel32)
  - [`IMAGE_REL_AMD64_ABSOLUTE`](#image-rel-amd64-absolute)
  - [`IMAGE_REL_AMD64_ADDR64`](#image-rel-amd64-addr64)
  - [`IMAGE_REL_AMD64_ADDR32`](#image-rel-amd64-addr32)
  - [`IMAGE_REL_AMD64_ADDR32NB`](#image-rel-amd64-addr32nb)
  - [`IMAGE_REL_AMD64_REL32`](#image-rel-amd64-rel32)
  - [`IMAGE_REL_AMD64_REL32_1`](#image-rel-amd64-rel32-1)
  - [`IMAGE_REL_AMD64_REL32_2`](#image-rel-amd64-rel32-2)
  - [`IMAGE_REL_AMD64_REL32_3`](#image-rel-amd64-rel32-3)
  - [`IMAGE_REL_AMD64_REL32_4`](#image-rel-amd64-rel32-4)
  - [`IMAGE_REL_AMD64_REL32_5`](#image-rel-amd64-rel32-5)
  - [`IMAGE_REL_AMD64_SECTION`](#image-rel-amd64-section)
  - [`IMAGE_REL_AMD64_SECREL`](#image-rel-amd64-secrel)
  - [`IMAGE_REL_AMD64_SECREL7`](#image-rel-amd64-secrel7)
  - [`IMAGE_REL_AMD64_TOKEN`](#image-rel-amd64-token)
  - [`IMAGE_REL_AMD64_SREL32`](#image-rel-amd64-srel32)
  - [`IMAGE_REL_AMD64_PAIR`](#image-rel-amd64-pair)
  - [`IMAGE_REL_AMD64_SSPAN32`](#image-rel-amd64-sspan32)
  - [`IMAGE_REL_AMD64_EHANDLER`](#image-rel-amd64-ehandler)
  - [`IMAGE_REL_AMD64_IMPORT_BR`](#image-rel-amd64-import-br)
  - [`IMAGE_REL_AMD64_IMPORT_CALL`](#image-rel-amd64-import-call)
  - [`IMAGE_REL_AMD64_CFG_BR`](#image-rel-amd64-cfg-br)
  - [`IMAGE_REL_AMD64_CFG_BR_REX`](#image-rel-amd64-cfg-br-rex)
  - [`IMAGE_REL_AMD64_CFG_CALL`](#image-rel-amd64-cfg-call)
  - [`IMAGE_REL_AMD64_INDIR_BR`](#image-rel-amd64-indir-br)
  - [`IMAGE_REL_AMD64_INDIR_BR_REX`](#image-rel-amd64-indir-br-rex)
  - [`IMAGE_REL_AMD64_INDIR_CALL`](#image-rel-amd64-indir-call)
  - [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST`](#image-rel-amd64-indir-br-switchtable-first)
  - [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST`](#image-rel-amd64-indir-br-switchtable-last)
  - [`IMAGE_REL_IA64_ABSOLUTE`](#image-rel-ia64-absolute)
  - [`IMAGE_REL_IA64_IMM14`](#image-rel-ia64-imm14)
  - [`IMAGE_REL_IA64_IMM22`](#image-rel-ia64-imm22)
  - [`IMAGE_REL_IA64_IMM64`](#image-rel-ia64-imm64)
  - [`IMAGE_REL_IA64_DIR32`](#image-rel-ia64-dir32)
  - [`IMAGE_REL_IA64_DIR64`](#image-rel-ia64-dir64)
  - [`IMAGE_REL_IA64_PCREL21B`](#image-rel-ia64-pcrel21b)
  - [`IMAGE_REL_IA64_PCREL21M`](#image-rel-ia64-pcrel21m)
  - [`IMAGE_REL_IA64_PCREL21F`](#image-rel-ia64-pcrel21f)
  - [`IMAGE_REL_IA64_GPREL22`](#image-rel-ia64-gprel22)
  - [`IMAGE_REL_IA64_LTOFF22`](#image-rel-ia64-ltoff22)
  - [`IMAGE_REL_IA64_SECTION`](#image-rel-ia64-section)
  - [`IMAGE_REL_IA64_SECREL22`](#image-rel-ia64-secrel22)
  - [`IMAGE_REL_IA64_SECREL64I`](#image-rel-ia64-secrel64i)
  - [`IMAGE_REL_IA64_SECREL32`](#image-rel-ia64-secrel32)
  - [`IMAGE_REL_IA64_DIR32NB`](#image-rel-ia64-dir32nb)
  - [`IMAGE_REL_IA64_SREL14`](#image-rel-ia64-srel14)
  - [`IMAGE_REL_IA64_SREL22`](#image-rel-ia64-srel22)
  - [`IMAGE_REL_IA64_SREL32`](#image-rel-ia64-srel32)
  - [`IMAGE_REL_IA64_UREL32`](#image-rel-ia64-urel32)
  - [`IMAGE_REL_IA64_PCREL60X`](#image-rel-ia64-pcrel60x)
  - [`IMAGE_REL_IA64_PCREL60B`](#image-rel-ia64-pcrel60b)
  - [`IMAGE_REL_IA64_PCREL60F`](#image-rel-ia64-pcrel60f)
  - [`IMAGE_REL_IA64_PCREL60I`](#image-rel-ia64-pcrel60i)
  - [`IMAGE_REL_IA64_PCREL60M`](#image-rel-ia64-pcrel60m)
  - [`IMAGE_REL_IA64_IMMGPREL64`](#image-rel-ia64-immgprel64)
  - [`IMAGE_REL_IA64_TOKEN`](#image-rel-ia64-token)
  - [`IMAGE_REL_IA64_GPREL32`](#image-rel-ia64-gprel32)
  - [`IMAGE_REL_IA64_ADDEND`](#image-rel-ia64-addend)
  - [`IMAGE_REL_CEF_ABSOLUTE`](#image-rel-cef-absolute)
  - [`IMAGE_REL_CEF_ADDR32`](#image-rel-cef-addr32)
  - [`IMAGE_REL_CEF_ADDR64`](#image-rel-cef-addr64)
  - [`IMAGE_REL_CEF_ADDR32NB`](#image-rel-cef-addr32nb)
  - [`IMAGE_REL_CEF_SECTION`](#image-rel-cef-section)
  - [`IMAGE_REL_CEF_SECREL`](#image-rel-cef-secrel)
  - [`IMAGE_REL_CEF_TOKEN`](#image-rel-cef-token)
  - [`IMAGE_REL_CEE_ABSOLUTE`](#image-rel-cee-absolute)
  - [`IMAGE_REL_CEE_ADDR32`](#image-rel-cee-addr32)
  - [`IMAGE_REL_CEE_ADDR64`](#image-rel-cee-addr64)
  - [`IMAGE_REL_CEE_ADDR32NB`](#image-rel-cee-addr32nb)
  - [`IMAGE_REL_CEE_SECTION`](#image-rel-cee-section)
  - [`IMAGE_REL_CEE_SECREL`](#image-rel-cee-secrel)
  - [`IMAGE_REL_CEE_TOKEN`](#image-rel-cee-token)
  - [`IMAGE_REL_M32R_ABSOLUTE`](#image-rel-m32r-absolute)
  - [`IMAGE_REL_M32R_ADDR32`](#image-rel-m32r-addr32)
  - [`IMAGE_REL_M32R_ADDR32NB`](#image-rel-m32r-addr32nb)
  - [`IMAGE_REL_M32R_ADDR24`](#image-rel-m32r-addr24)
  - [`IMAGE_REL_M32R_GPREL16`](#image-rel-m32r-gprel16)
  - [`IMAGE_REL_M32R_PCREL24`](#image-rel-m32r-pcrel24)
  - [`IMAGE_REL_M32R_PCREL16`](#image-rel-m32r-pcrel16)
  - [`IMAGE_REL_M32R_PCREL8`](#image-rel-m32r-pcrel8)
  - [`IMAGE_REL_M32R_REFHALF`](#image-rel-m32r-refhalf)
  - [`IMAGE_REL_M32R_REFHI`](#image-rel-m32r-refhi)
  - [`IMAGE_REL_M32R_REFLO`](#image-rel-m32r-reflo)
  - [`IMAGE_REL_M32R_PAIR`](#image-rel-m32r-pair)
  - [`IMAGE_REL_M32R_SECTION`](#image-rel-m32r-section)
  - [`IMAGE_REL_M32R_SECREL32`](#image-rel-m32r-secrel32)
  - [`IMAGE_REL_M32R_TOKEN`](#image-rel-m32r-token)
  - [`IMAGE_REL_EBC_ABSOLUTE`](#image-rel-ebc-absolute)
  - [`IMAGE_REL_EBC_ADDR32NB`](#image-rel-ebc-addr32nb)
  - [`IMAGE_REL_EBC_REL32`](#image-rel-ebc-rel32)
  - [`IMAGE_REL_EBC_SECTION`](#image-rel-ebc-section)
  - [`IMAGE_REL_EBC_SECREL`](#image-rel-ebc-secrel)
  - [`EMARCH_ENC_I17_IMM7B_INST_WORD_X`](#emarch-enc-i17-imm7b-inst-word-x)
  - [`EMARCH_ENC_I17_IMM7B_SIZE_X`](#emarch-enc-i17-imm7b-size-x)
  - [`EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X`](#emarch-enc-i17-imm7b-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM7B_VAL_POS_X`](#emarch-enc-i17-imm7b-val-pos-x)
  - [`EMARCH_ENC_I17_IMM9D_INST_WORD_X`](#emarch-enc-i17-imm9d-inst-word-x)
  - [`EMARCH_ENC_I17_IMM9D_SIZE_X`](#emarch-enc-i17-imm9d-size-x)
  - [`EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X`](#emarch-enc-i17-imm9d-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM9D_VAL_POS_X`](#emarch-enc-i17-imm9d-val-pos-x)
  - [`EMARCH_ENC_I17_IMM5C_INST_WORD_X`](#emarch-enc-i17-imm5c-inst-word-x)
  - [`EMARCH_ENC_I17_IMM5C_SIZE_X`](#emarch-enc-i17-imm5c-size-x)
  - [`EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X`](#emarch-enc-i17-imm5c-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM5C_VAL_POS_X`](#emarch-enc-i17-imm5c-val-pos-x)
  - [`EMARCH_ENC_I17_IC_INST_WORD_X`](#emarch-enc-i17-ic-inst-word-x)
  - [`EMARCH_ENC_I17_IC_SIZE_X`](#emarch-enc-i17-ic-size-x)
  - [`EMARCH_ENC_I17_IC_INST_WORD_POS_X`](#emarch-enc-i17-ic-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IC_VAL_POS_X`](#emarch-enc-i17-ic-val-pos-x)
  - [`EMARCH_ENC_I17_IMM41A_INST_WORD_X`](#emarch-enc-i17-imm41a-inst-word-x)
  - [`EMARCH_ENC_I17_IMM41A_SIZE_X`](#emarch-enc-i17-imm41a-size-x)
  - [`EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X`](#emarch-enc-i17-imm41a-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM41A_VAL_POS_X`](#emarch-enc-i17-imm41a-val-pos-x)
  - [`EMARCH_ENC_I17_IMM41B_INST_WORD_X`](#emarch-enc-i17-imm41b-inst-word-x)
  - [`EMARCH_ENC_I17_IMM41B_SIZE_X`](#emarch-enc-i17-imm41b-size-x)
  - [`EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X`](#emarch-enc-i17-imm41b-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM41B_VAL_POS_X`](#emarch-enc-i17-imm41b-val-pos-x)
  - [`EMARCH_ENC_I17_IMM41C_INST_WORD_X`](#emarch-enc-i17-imm41c-inst-word-x)
  - [`EMARCH_ENC_I17_IMM41C_SIZE_X`](#emarch-enc-i17-imm41c-size-x)
  - [`EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X`](#emarch-enc-i17-imm41c-inst-word-pos-x)
  - [`EMARCH_ENC_I17_IMM41C_VAL_POS_X`](#emarch-enc-i17-imm41c-val-pos-x)
  - [`EMARCH_ENC_I17_SIGN_INST_WORD_X`](#emarch-enc-i17-sign-inst-word-x)
  - [`EMARCH_ENC_I17_SIGN_SIZE_X`](#emarch-enc-i17-sign-size-x)
  - [`EMARCH_ENC_I17_SIGN_INST_WORD_POS_X`](#emarch-enc-i17-sign-inst-word-pos-x)
  - [`EMARCH_ENC_I17_SIGN_VAL_POS_X`](#emarch-enc-i17-sign-val-pos-x)
  - [`X3_OPCODE_INST_WORD_X`](#x3-opcode-inst-word-x)
  - [`X3_OPCODE_SIZE_X`](#x3-opcode-size-x)
  - [`X3_OPCODE_INST_WORD_POS_X`](#x3-opcode-inst-word-pos-x)
  - [`X3_OPCODE_SIGN_VAL_POS_X`](#x3-opcode-sign-val-pos-x)
  - [`X3_I_INST_WORD_X`](#x3-i-inst-word-x)
  - [`X3_I_SIZE_X`](#x3-i-size-x)
  - [`X3_I_INST_WORD_POS_X`](#x3-i-inst-word-pos-x)
  - [`X3_I_SIGN_VAL_POS_X`](#x3-i-sign-val-pos-x)
  - [`X3_D_WH_INST_WORD_X`](#x3-d-wh-inst-word-x)
  - [`X3_D_WH_SIZE_X`](#x3-d-wh-size-x)
  - [`X3_D_WH_INST_WORD_POS_X`](#x3-d-wh-inst-word-pos-x)
  - [`X3_D_WH_SIGN_VAL_POS_X`](#x3-d-wh-sign-val-pos-x)
  - [`X3_IMM20_INST_WORD_X`](#x3-imm20-inst-word-x)
  - [`X3_IMM20_SIZE_X`](#x3-imm20-size-x)
  - [`X3_IMM20_INST_WORD_POS_X`](#x3-imm20-inst-word-pos-x)
  - [`X3_IMM20_SIGN_VAL_POS_X`](#x3-imm20-sign-val-pos-x)
  - [`X3_IMM39_1_INST_WORD_X`](#x3-imm39-1-inst-word-x)
  - [`X3_IMM39_1_SIZE_X`](#x3-imm39-1-size-x)
  - [`X3_IMM39_1_INST_WORD_POS_X`](#x3-imm39-1-inst-word-pos-x)
  - [`X3_IMM39_1_SIGN_VAL_POS_X`](#x3-imm39-1-sign-val-pos-x)
  - [`X3_IMM39_2_INST_WORD_X`](#x3-imm39-2-inst-word-x)
  - [`X3_IMM39_2_SIZE_X`](#x3-imm39-2-size-x)
  - [`X3_IMM39_2_INST_WORD_POS_X`](#x3-imm39-2-inst-word-pos-x)
  - [`X3_IMM39_2_SIGN_VAL_POS_X`](#x3-imm39-2-sign-val-pos-x)
  - [`X3_P_INST_WORD_X`](#x3-p-inst-word-x)
  - [`X3_P_SIZE_X`](#x3-p-size-x)
  - [`X3_P_INST_WORD_POS_X`](#x3-p-inst-word-pos-x)
  - [`X3_P_SIGN_VAL_POS_X`](#x3-p-sign-val-pos-x)
  - [`X3_TMPLT_INST_WORD_X`](#x3-tmplt-inst-word-x)
  - [`X3_TMPLT_SIZE_X`](#x3-tmplt-size-x)
  - [`X3_TMPLT_INST_WORD_POS_X`](#x3-tmplt-inst-word-pos-x)
  - [`X3_TMPLT_SIGN_VAL_POS_X`](#x3-tmplt-sign-val-pos-x)
  - [`X3_BTYPE_QP_INST_WORD_X`](#x3-btype-qp-inst-word-x)
  - [`X3_BTYPE_QP_SIZE_X`](#x3-btype-qp-size-x)
  - [`X3_BTYPE_QP_INST_WORD_POS_X`](#x3-btype-qp-inst-word-pos-x)
  - [`X3_BTYPE_QP_INST_VAL_POS_X`](#x3-btype-qp-inst-val-pos-x)
  - [`X3_EMPTY_INST_WORD_X`](#x3-empty-inst-word-x)
  - [`X3_EMPTY_SIZE_X`](#x3-empty-size-x)
  - [`X3_EMPTY_INST_WORD_POS_X`](#x3-empty-inst-word-pos-x)
  - [`X3_EMPTY_INST_VAL_POS_X`](#x3-empty-inst-val-pos-x)
  - [`IMAGE_REL_BASED_ABSOLUTE`](#image-rel-based-absolute)
  - [`IMAGE_REL_BASED_HIGH`](#image-rel-based-high)
  - [`IMAGE_REL_BASED_LOW`](#image-rel-based-low)
  - [`IMAGE_REL_BASED_HIGHLOW`](#image-rel-based-highlow)
  - [`IMAGE_REL_BASED_HIGHADJ`](#image-rel-based-highadj)
  - [`IMAGE_REL_BASED_MACHINE_SPECIFIC_5`](#image-rel-based-machine-specific-5)
  - [`IMAGE_REL_BASED_RESERVED`](#image-rel-based-reserved)
  - [`IMAGE_REL_BASED_MACHINE_SPECIFIC_7`](#image-rel-based-machine-specific-7)
  - [`IMAGE_REL_BASED_MACHINE_SPECIFIC_8`](#image-rel-based-machine-specific-8)
  - [`IMAGE_REL_BASED_MACHINE_SPECIFIC_9`](#image-rel-based-machine-specific-9)
  - [`IMAGE_REL_BASED_DIR64`](#image-rel-based-dir64)
  - [`IMAGE_REL_BASED_IA64_IMM64`](#image-rel-based-ia64-imm64)
  - [`IMAGE_REL_BASED_MIPS_JMPADDR`](#image-rel-based-mips-jmpaddr)
  - [`IMAGE_REL_BASED_MIPS_JMPADDR16`](#image-rel-based-mips-jmpaddr16)
  - [`IMAGE_REL_BASED_ARM_MOV32`](#image-rel-based-arm-mov32)
  - [`IMAGE_REL_BASED_THUMB_MOV32`](#image-rel-based-thumb-mov32)
  - [`IMAGE_REL_BASED_RISCV_HIGH20`](#image-rel-based-riscv-high20)
  - [`IMAGE_REL_BASED_RISCV_LOW12I`](#image-rel-based-riscv-low12i)
  - [`IMAGE_REL_BASED_RISCV_LOW12S`](#image-rel-based-riscv-low12s)
  - [`IMAGE_ARCHIVE_START_SIZE`](#image-archive-start-size)
  - [`IMAGE_ARCHIVE_START`](#image-archive-start)
  - [`IMAGE_ARCHIVE_END`](#image-archive-end)
  - [`IMAGE_ARCHIVE_PAD`](#image-archive-pad)
  - [`IMAGE_ARCHIVE_LINKER_MEMBER`](#image-archive-linker-member)
  - [`IMAGE_ARCHIVE_LONGNAMES_MEMBER`](#image-archive-longnames-member)
  - [`IMAGE_ARCHIVE_HYBRIDMAP_MEMBER`](#image-archive-hybridmap-member)
  - [`IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR`](#image-sizeof-archive-member-hdr)
  - [`IMAGE_ORDINAL_FLAG64`](#image-ordinal-flag64)
  - [`IMAGE_ORDINAL_FLAG32`](#image-ordinal-flag32)
  - [`IMAGE_DELAYLOAD_RVA_BASED`](#image-delayload-rva-based)
  - [`IMAGE_RESOURCE_NAME_IS_STRING`](#image-resource-name-is-string)
  - [`IMAGE_RESOURCE_DATA_IS_DIRECTORY`](#image-resource-data-is-directory)
  - [`RT_CURSOR`](#rt-cursor)
  - [`RT_BITMAP`](#rt-bitmap)
  - [`RT_ICON`](#rt-icon)
  - [`RT_MENU`](#rt-menu)
  - [`RT_DIALOG`](#rt-dialog)
  - [`RT_STRING`](#rt-string)
  - [`RT_FONTDIR`](#rt-fontdir)
  - [`RT_FONT`](#rt-font)
  - [`RT_ACCELERATOR`](#rt-accelerator)
  - [`RT_RCDATA`](#rt-rcdata)
  - [`RT_MESSAGETABLE`](#rt-messagetable)
  - [`RT_GROUP_CURSOR`](#rt-group-cursor)
  - [`RT_GROUP_ICON`](#rt-group-icon)
  - [`RT_VERSION`](#rt-version)
  - [`RT_DLGINCLUDE`](#rt-dlginclude)
  - [`RT_PLUGPLAY`](#rt-plugplay)
  - [`RT_VXD`](#rt-vxd)
  - [`RT_ANICURSOR`](#rt-anicursor)
  - [`RT_ANIICON`](#rt-aniicon)
  - [`RT_HTML`](#rt-html)
  - [`RT_MANIFEST`](#rt-manifest)
  - [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE`](#image-dynamic-relocation-guard-rf-prologue)
  - [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE`](#image-dynamic-relocation-guard-rf-epilogue)
  - [`IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER`](#image-dynamic-relocation-guard-import-control-transfer)
  - [`IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER`](#image-dynamic-relocation-guard-indir-control-transfer)
  - [`IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH`](#image-dynamic-relocation-guard-switchtable-branch)
  - [`IMAGE_HOT_PATCH_BASE_OBLIGATORY`](#image-hot-patch-base-obligatory)
  - [`IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK`](#image-hot-patch-base-can-roll-back)
  - [`IMAGE_HOT_PATCH_CHUNK_INVERSE`](#image-hot-patch-chunk-inverse)
  - [`IMAGE_HOT_PATCH_CHUNK_OBLIGATORY`](#image-hot-patch-chunk-obligatory)
  - [`IMAGE_HOT_PATCH_CHUNK_RESERVED`](#image-hot-patch-chunk-reserved)
  - [`IMAGE_HOT_PATCH_CHUNK_TYPE`](#image-hot-patch-chunk-type)
  - [`IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA`](#image-hot-patch-chunk-source-rva)
  - [`IMAGE_HOT_PATCH_CHUNK_TARGET_RVA`](#image-hot-patch-chunk-target-rva)
  - [`IMAGE_HOT_PATCH_CHUNK_SIZE`](#image-hot-patch-chunk-size)
  - [`IMAGE_HOT_PATCH_NONE`](#image-hot-patch-none)
  - [`IMAGE_HOT_PATCH_FUNCTION`](#image-hot-patch-function)
  - [`IMAGE_HOT_PATCH_ABSOLUTE`](#image-hot-patch-absolute)
  - [`IMAGE_HOT_PATCH_REL32`](#image-hot-patch-rel32)
  - [`IMAGE_HOT_PATCH_CALL_TARGET`](#image-hot-patch-call-target)
  - [`IMAGE_HOT_PATCH_INDIRECT`](#image-hot-patch-indirect)
  - [`IMAGE_HOT_PATCH_NO_CALL_TARGET`](#image-hot-patch-no-call-target)
  - [`IMAGE_HOT_PATCH_DYNAMIC_VALUE`](#image-hot-patch-dynamic-value)
  - [`IMAGE_GUARD_CF_INSTRUMENTED`](#image-guard-cf-instrumented)
  - [`IMAGE_GUARD_CFW_INSTRUMENTED`](#image-guard-cfw-instrumented)
  - [`IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT`](#image-guard-cf-function-table-present)
  - [`IMAGE_GUARD_SECURITY_COOKIE_UNUSED`](#image-guard-security-cookie-unused)
  - [`IMAGE_GUARD_PROTECT_DELAYLOAD_IAT`](#image-guard-protect-delayload-iat)
  - [`IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION`](#image-guard-delayload-iat-in-its-own-section)
  - [`IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT`](#image-guard-cf-export-suppression-info-present)
  - [`IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION`](#image-guard-cf-enable-export-suppression)
  - [`IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT`](#image-guard-cf-longjump-table-present)
  - [`IMAGE_GUARD_RF_INSTRUMENTED`](#image-guard-rf-instrumented)
  - [`IMAGE_GUARD_RF_ENABLE`](#image-guard-rf-enable)
  - [`IMAGE_GUARD_RF_STRICT`](#image-guard-rf-strict)
  - [`IMAGE_GUARD_RETPOLINE_PRESENT`](#image-guard-retpoline-present)
  - [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK`](#image-guard-cf-function-table-size-mask)
  - [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT`](#image-guard-cf-function-table-size-shift)
  - [`IMAGE_GUARD_FLAG_FID_SUPPRESSED`](#image-guard-flag-fid-suppressed)
  - [`IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED`](#image-guard-flag-export-suppressed)
  - [`IMAGE_ENCLAVE_LONG_ID_LENGTH`](#image-enclave-long-id-length)
  - [`IMAGE_ENCLAVE_SHORT_ID_LENGTH`](#image-enclave-short-id-length)
  - [`IMAGE_ENCLAVE_POLICY_DEBUGGABLE`](#image-enclave-policy-debuggable)
  - [`IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE`](#image-enclave-flag-primary-image)
  - [`IMAGE_ENCLAVE_IMPORT_MATCH_NONE`](#image-enclave-import-match-none)
  - [`IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID`](#image-enclave-import-match-unique-id)
  - [`IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID`](#image-enclave-import-match-author-id)
  - [`IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID`](#image-enclave-import-match-family-id)
  - [`IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID`](#image-enclave-import-match-image-id)
  - [`IMAGE_DEBUG_TYPE_UNKNOWN`](#image-debug-type-unknown)
  - [`IMAGE_DEBUG_TYPE_COFF`](#image-debug-type-coff)
  - [`IMAGE_DEBUG_TYPE_CODEVIEW`](#image-debug-type-codeview)
  - [`IMAGE_DEBUG_TYPE_FPO`](#image-debug-type-fpo)
  - [`IMAGE_DEBUG_TYPE_MISC`](#image-debug-type-misc)
  - [`IMAGE_DEBUG_TYPE_EXCEPTION`](#image-debug-type-exception)
  - [`IMAGE_DEBUG_TYPE_FIXUP`](#image-debug-type-fixup)
  - [`IMAGE_DEBUG_TYPE_OMAP_TO_SRC`](#image-debug-type-omap-to-src)
  - [`IMAGE_DEBUG_TYPE_OMAP_FROM_SRC`](#image-debug-type-omap-from-src)
  - [`IMAGE_DEBUG_TYPE_BORLAND`](#image-debug-type-borland)
  - [`IMAGE_DEBUG_TYPE_RESERVED10`](#image-debug-type-reserved10)
  - [`IMAGE_DEBUG_TYPE_CLSID`](#image-debug-type-clsid)
  - [`IMAGE_DEBUG_TYPE_VC_FEATURE`](#image-debug-type-vc-feature)
  - [`IMAGE_DEBUG_TYPE_POGO`](#image-debug-type-pogo)
  - [`IMAGE_DEBUG_TYPE_ILTCG`](#image-debug-type-iltcg)
  - [`IMAGE_DEBUG_TYPE_MPX`](#image-debug-type-mpx)
  - [`IMAGE_DEBUG_TYPE_REPRO`](#image-debug-type-repro)
  - [`FRAME_FPO`](#frame-fpo)
  - [`FRAME_TRAP`](#frame-trap)
  - [`FRAME_TSS`](#frame-tss)
  - [`FRAME_NONFPO`](#frame-nonfpo)
  - [`IMAGE_DEBUG_MISC_EXENAME`](#image-debug-misc-exename)
  - [`IMAGE_SEPARATE_DEBUG_SIGNATURE`](#image-separate-debug-signature)
  - [`NON_PAGED_DEBUG_SIGNATURE`](#non-paged-debug-signature)
  - [`IMAGE_SEPARATE_DEBUG_FLAGS_MASK`](#image-separate-debug-flags-mask)
  - [`IMAGE_SEPARATE_DEBUG_MISMATCH`](#image-separate-debug-mismatch)
  - [`IMPORT_OBJECT_HDR_SIG2`](#import-object-hdr-sig2)
  - [`IMPORT_OBJECT_TYPE_MASK`](#import-object-type-mask)
  - [`IMPORT_OBJECT_TYPE_SHIFT`](#import-object-type-shift)
  - [`IMPORT_OBJECT_CODE`](#import-object-code)
  - [`IMPORT_OBJECT_DATA`](#import-object-data)
  - [`IMPORT_OBJECT_CONST`](#import-object-const)
  - [`IMPORT_OBJECT_NAME_MASK`](#import-object-name-mask)
  - [`IMPORT_OBJECT_NAME_SHIFT`](#import-object-name-shift)
  - [`IMPORT_OBJECT_ORDINAL`](#import-object-ordinal)
  - [`IMPORT_OBJECT_NAME`](#import-object-name)
  - [`IMPORT_OBJECT_NAME_NO_PREFIX`](#import-object-name-no-prefix)
  - [`IMPORT_OBJECT_NAME_UNDECORATE`](#import-object-name-undecorate)
  - [`IMPORT_OBJECT_NAME_EXPORTAS`](#import-object-name-exportas)
  - [`COMIMAGE_FLAGS_ILONLY`](#comimage-flags-ilonly)
  - [`COMIMAGE_FLAGS_32BITREQUIRED`](#comimage-flags-32bitrequired)
  - [`COMIMAGE_FLAGS_IL_LIBRARY`](#comimage-flags-il-library)
  - [`COMIMAGE_FLAGS_STRONGNAMESIGNED`](#comimage-flags-strongnamesigned)
  - [`COMIMAGE_FLAGS_NATIVE_ENTRYPOINT`](#comimage-flags-native-entrypoint)
  - [`COMIMAGE_FLAGS_TRACKDEBUGDATA`](#comimage-flags-trackdebugdata)
  - [`COMIMAGE_FLAGS_32BITPREFERRED`](#comimage-flags-32bitpreferred)
  - [`COR_VERSION_MAJOR_V2`](#cor-version-major-v2)
  - [`COR_VERSION_MAJOR`](#cor-version-major)
  - [`COR_VERSION_MINOR`](#cor-version-minor)
  - [`COR_DELETED_NAME_LENGTH`](#cor-deleted-name-length)
  - [`COR_VTABLEGAP_NAME_LENGTH`](#cor-vtablegap-name-length)
  - [`NATIVE_TYPE_MAX_CB`](#native-type-max-cb)
  - [`COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE`](#cor-ilmethod-sect-small-max-datasize)
  - [`IMAGE_COR_MIH_METHODRVA`](#image-cor-mih-methodrva)
  - [`IMAGE_COR_MIH_EHRVA`](#image-cor-mih-ehrva)
  - [`IMAGE_COR_MIH_BASICBLOCK`](#image-cor-mih-basicblock)
  - [`COR_VTABLE_32BIT`](#cor-vtable-32bit)
  - [`COR_VTABLE_64BIT`](#cor-vtable-64bit)
  - [`COR_VTABLE_FROM_UNMANAGED`](#cor-vtable-from-unmanaged)
  - [`COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN`](#cor-vtable-from-unmanaged-retain-appdomain)
  - [`COR_VTABLE_CALL_MOST_DERIVED`](#cor-vtable-call-most-derived)
  - [`IMAGE_COR_EATJ_THUNK_SIZE`](#image-cor-eatj-thunk-size)
  - [`MAX_CLASS_NAME`](#max-class-name)
  - [`MAX_PACKAGE_NAME`](#max-package-name)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImageDosHeader`](#imagedosheader) | struct | DOS .EXE header |
| [`ImageOs2Header`](#imageos2header) | struct | OS/2 .EXE header |
| [`ImageVxdHeader`](#imagevxdheader) | struct | Windows VXD header |
| [`MaskedRichHeaderEntry`](#maskedrichheaderentry) | struct | A PE rich header entry. |
| [`ImageFileHeader`](#imagefileheader) | struct |  |
| [`ImageDataDirectory`](#imagedatadirectory) | struct |  |
| [`ImageOptionalHeader32`](#imageoptionalheader32) | struct |  |
| [`ImageRomOptionalHeader`](#imageromoptionalheader) | struct |  |
| [`ImageOptionalHeader64`](#imageoptionalheader64) | struct |  |
| [`ImageNtHeaders64`](#imagentheaders64) | struct |  |
| [`ImageNtHeaders32`](#imagentheaders32) | struct |  |
| [`ImageRomHeaders`](#imageromheaders) | struct |  |
| [`Guid`](#guid) | struct |  |
| [`ClsId`](#clsid) | struct |  |
| [`AnonObjectHeader`](#anonobjectheader) | struct | Non-COFF Object file header |
| [`AnonObjectHeaderV2`](#anonobjectheaderv2) | struct |  |
| [`AnonObjectHeaderBigobj`](#anonobjectheaderbigobj) | struct |  |
| [`ImageSectionHeader`](#imagesectionheader) | struct |  |
| [`ImageSymbol`](#imagesymbol) | struct |  |
| [`ImageSymbolBytes`](#imagesymbolbytes) | struct |  |
| [`ImageSymbolEx`](#imagesymbolex) | struct |  |
| [`ImageSymbolExBytes`](#imagesymbolexbytes) | struct |  |
| [`ImageAuxSymbolTokenDef`](#imageauxsymboltokendef) | struct |  |
| [`ImageAuxSymbolFunction`](#imageauxsymbolfunction) | struct | Auxiliary symbol format 1: function definitions. |
| [`ImageAuxSymbolFunctionBeginEnd`](#imageauxsymbolfunctionbeginend) | struct | Auxiliary symbol format 2: .bf and .ef symbols. |
| [`ImageAuxSymbolWeak`](#imageauxsymbolweak) | struct | Auxiliary symbol format 3: weak externals. |
| [`ImageAuxSymbolSection`](#imageauxsymbolsection) | struct | Auxiliary symbol format 5: sections. |
| [`ImageAuxSymbolCrc`](#imageauxsymbolcrc) | struct |  |
| [`ImageRelocation`](#imagerelocation) | struct |  |
| [`ImageLinenumber`](#imagelinenumber) | struct |  |
| [`ImageBaseRelocation`](#imagebaserelocation) | struct |  |
| [`ImageArchiveMemberHeader`](#imagearchivememberheader) | struct |  |
| [`ImageExportDirectory`](#imageexportdirectory) | struct |  |
| [`ImageImportByName`](#imageimportbyname) | struct |  |
| [`ImageThunkData64`](#imagethunkdata64) | struct |  |
| [`ImageThunkData32`](#imagethunkdata32) | struct |  |
| [`ImageTlsDirectory64`](#imagetlsdirectory64) | struct |  |
| [`ImageTlsDirectory32`](#imagetlsdirectory32) | struct |  |
| [`ImageImportDescriptor`](#imageimportdescriptor) | struct |  |
| [`ImageBoundImportDescriptor`](#imageboundimportdescriptor) | struct |  |
| [`ImageBoundForwarderRef`](#imageboundforwarderref) | struct |  |
| [`ImageDelayloadDescriptor`](#imagedelayloaddescriptor) | struct |  |
| [`ImageResourceDirectory`](#imageresourcedirectory) | struct |  |
| [`ImageResourceDirectoryEntry`](#imageresourcedirectoryentry) | struct |  |
| [`ImageResourceDirectoryString`](#imageresourcedirectorystring) | struct |  |
| [`ImageResourceDirStringU`](#imageresourcedirstringu) | struct |  |
| [`ImageResourceDataEntry`](#imageresourcedataentry) | struct |  |
| [`ImageLoadConfigCodeIntegrity`](#imageloadconfigcodeintegrity) | struct |  |
| [`ImageDynamicRelocationTable`](#imagedynamicrelocationtable) | struct |  |
| [`ImageDynamicRelocation32`](#imagedynamicrelocation32) | struct |  |
| [`ImageDynamicRelocation64`](#imagedynamicrelocation64) | struct |  |
| [`ImageDynamicRelocation32V2`](#imagedynamicrelocation32v2) | struct |  |
| [`ImageDynamicRelocation64V2`](#imagedynamicrelocation64v2) | struct |  |
| [`ImagePrologueDynamicRelocationHeader`](#imageprologuedynamicrelocationheader) | struct |  |
| [`ImageEpilogueDynamicRelocationHeader`](#imageepiloguedynamicrelocationheader) | struct |  |
| [`ImageLoadConfigDirectory32`](#imageloadconfigdirectory32) | struct |  |
| [`ImageLoadConfigDirectory64`](#imageloadconfigdirectory64) | struct |  |
| [`ImageHotPatchInfo`](#imagehotpatchinfo) | struct |  |
| [`ImageHotPatchBase`](#imagehotpatchbase) | struct |  |
| [`ImageHotPatchHashes`](#imagehotpatchhashes) | struct |  |
| [`ImageArmRuntimeFunctionEntry`](#imagearmruntimefunctionentry) | struct |  |
| [`ImageArm64RuntimeFunctionEntry`](#imagearm64runtimefunctionentry) | struct |  |
| [`ImageAlpha64RuntimeFunctionEntry`](#imagealpha64runtimefunctionentry) | struct |  |
| [`ImageAlphaRuntimeFunctionEntry`](#imagealpharuntimefunctionentry) | struct |  |
| [`ImageRuntimeFunctionEntry`](#imageruntimefunctionentry) | struct |  |
| [`ImageEnclaveConfig32`](#imageenclaveconfig32) | struct |  |
| [`ImageEnclaveConfig64`](#imageenclaveconfig64) | struct |  |
| [`ImageEnclaveImport`](#imageenclaveimport) | struct |  |
| [`ImageDebugDirectory`](#imagedebugdirectory) | struct |  |
| [`ImageCoffSymbolsHeader`](#imagecoffsymbolsheader) | struct |  |
| [`ImageDebugMisc`](#imagedebugmisc) | struct |  |
| [`ImageFunctionEntry`](#imagefunctionentry) | struct |  |
| [`ImageFunctionEntry64`](#imagefunctionentry64) | struct |  |
| [`ImageSeparateDebugHeader`](#imageseparatedebugheader) | struct |  |
| [`NonPagedDebugInfo`](#nonpageddebuginfo) | struct |  |
| [`ImageArchitectureEntry`](#imagearchitectureentry) | struct |  |
| [`ImportObjectHeader`](#importobjectheader) | struct |  |
| [`ImageCor20Header`](#imagecor20header) | struct |  |
| [`IMAGE_DOS_SIGNATURE`](#image-dos-signature) | const | MZ |
| [`IMAGE_OS2_SIGNATURE`](#image-os2-signature) | const | NE |
| [`IMAGE_OS2_SIGNATURE_LE`](#image-os2-signature-le) | const | LE |
| [`IMAGE_VXD_SIGNATURE`](#image-vxd-signature) | const | LE |
| [`IMAGE_NT_SIGNATURE`](#image-nt-signature) | const | PE00 |
| [`IMAGE_SIZEOF_FILE_HEADER`](#image-sizeof-file-header) | const |  |
| [`IMAGE_FILE_RELOCS_STRIPPED`](#image-file-relocs-stripped) | const | Relocation info stripped from file. |
| [`IMAGE_FILE_EXECUTABLE_IMAGE`](#image-file-executable-image) | const | File is executable  (i.e. no unresolved external references). |
| [`IMAGE_FILE_LINE_NUMS_STRIPPED`](#image-file-line-nums-stripped) | const | Line numbers stripped from file. |
| [`IMAGE_FILE_LOCAL_SYMS_STRIPPED`](#image-file-local-syms-stripped) | const | Local symbols stripped from file. |
| [`IMAGE_FILE_AGGRESIVE_WS_TRIM`](#image-file-aggresive-ws-trim) | const | Aggressively trim working set |
| [`IMAGE_FILE_LARGE_ADDRESS_AWARE`](#image-file-large-address-aware) | const | App can handle >2gb addresses |
| [`IMAGE_FILE_BYTES_REVERSED_LO`](#image-file-bytes-reversed-lo) | const | Bytes of machine word are reversed. |
| [`IMAGE_FILE_32BIT_MACHINE`](#image-file-32bit-machine) | const | 32 bit word machine. |
| [`IMAGE_FILE_DEBUG_STRIPPED`](#image-file-debug-stripped) | const | Debugging info stripped from file in .DBG file |
| [`IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP`](#image-file-removable-run-from-swap) | const | If Image is on removable media, copy and run from the swap file. |
| [`IMAGE_FILE_NET_RUN_FROM_SWAP`](#image-file-net-run-from-swap) | const | If Image is on Net, copy and run from the swap file. |
| [`IMAGE_FILE_SYSTEM`](#image-file-system) | const | System File. |
| [`IMAGE_FILE_DLL`](#image-file-dll) | const | File is a DLL. |
| [`IMAGE_FILE_UP_SYSTEM_ONLY`](#image-file-up-system-only) | const | File should only be run on a UP machine |
| [`IMAGE_FILE_BYTES_REVERSED_HI`](#image-file-bytes-reversed-hi) | const | Bytes of machine word are reversed. |
| [`IMAGE_FILE_MACHINE_UNKNOWN`](#image-file-machine-unknown) | const |  |
| [`IMAGE_FILE_MACHINE_TARGET_HOST`](#image-file-machine-target-host) | const | Useful for indicating we want to interact with the host and not a WoW guest. |
| [`IMAGE_FILE_MACHINE_I386`](#image-file-machine-i386) | const | Intel 386. |
| [`IMAGE_FILE_MACHINE_R3000`](#image-file-machine-r3000) | const | MIPS little-endian, 0x160 big-endian |
| [`IMAGE_FILE_MACHINE_R4000`](#image-file-machine-r4000) | const | MIPS little-endian |
| [`IMAGE_FILE_MACHINE_R10000`](#image-file-machine-r10000) | const | MIPS little-endian |
| [`IMAGE_FILE_MACHINE_WCEMIPSV2`](#image-file-machine-wcemipsv2) | const | MIPS little-endian WCE v2 |
| [`IMAGE_FILE_MACHINE_ALPHA`](#image-file-machine-alpha) | const | Alpha_AXP |
| [`IMAGE_FILE_MACHINE_SH3`](#image-file-machine-sh3) | const | SH3 little-endian |
| [`IMAGE_FILE_MACHINE_SH3DSP`](#image-file-machine-sh3dsp) | const |  |
| [`IMAGE_FILE_MACHINE_SH3E`](#image-file-machine-sh3e) | const | SH3E little-endian |
| [`IMAGE_FILE_MACHINE_SH4`](#image-file-machine-sh4) | const | SH4 little-endian |
| [`IMAGE_FILE_MACHINE_SH5`](#image-file-machine-sh5) | const | SH5 |
| [`IMAGE_FILE_MACHINE_ARM`](#image-file-machine-arm) | const | ARM Little-Endian |
| [`IMAGE_FILE_MACHINE_THUMB`](#image-file-machine-thumb) | const | ARM Thumb/Thumb-2 Little-Endian |
| [`IMAGE_FILE_MACHINE_ARMNT`](#image-file-machine-armnt) | const | ARM Thumb-2 Little-Endian |
| [`IMAGE_FILE_MACHINE_AM33`](#image-file-machine-am33) | const |  |
| [`IMAGE_FILE_MACHINE_POWERPC`](#image-file-machine-powerpc) | const | IBM PowerPC Little-Endian |
| [`IMAGE_FILE_MACHINE_POWERPCFP`](#image-file-machine-powerpcfp) | const |  |
| [`IMAGE_FILE_MACHINE_POWERPCBE`](#image-file-machine-powerpcbe) | const | IBM PowerPC Big-Endian |
| [`IMAGE_FILE_MACHINE_IA64`](#image-file-machine-ia64) | const | Intel 64 |
| [`IMAGE_FILE_MACHINE_MIPS16`](#image-file-machine-mips16) | const | MIPS |
| [`IMAGE_FILE_MACHINE_ALPHA64`](#image-file-machine-alpha64) | const | ALPHA64 |
| [`IMAGE_FILE_MACHINE_MIPSFPU`](#image-file-machine-mipsfpu) | const | MIPS |
| [`IMAGE_FILE_MACHINE_MIPSFPU16`](#image-file-machine-mipsfpu16) | const | MIPS |
| [`IMAGE_FILE_MACHINE_AXP64`](#image-file-machine-axp64) | const |  |
| [`IMAGE_FILE_MACHINE_TRICORE`](#image-file-machine-tricore) | const | Infineon |
| [`IMAGE_FILE_MACHINE_CEF`](#image-file-machine-cef) | const |  |
| [`IMAGE_FILE_MACHINE_EBC`](#image-file-machine-ebc) | const | EFI Byte Code |
| [`IMAGE_FILE_MACHINE_AMD64`](#image-file-machine-amd64) | const | AMD64 (K8) |
| [`IMAGE_FILE_MACHINE_M32R`](#image-file-machine-m32r) | const | M32R little-endian |
| [`IMAGE_FILE_MACHINE_ARM64`](#image-file-machine-arm64) | const | ARM64 Little-Endian |
| [`IMAGE_FILE_MACHINE_ARM64EC`](#image-file-machine-arm64ec) | const | ARM64EC ("Emulation Compatible") |
| [`IMAGE_FILE_MACHINE_CEE`](#image-file-machine-cee) | const |  |
| [`IMAGE_FILE_MACHINE_RISCV32`](#image-file-machine-riscv32) | const | RISCV32 |
| [`IMAGE_FILE_MACHINE_RISCV64`](#image-file-machine-riscv64) | const | RISCV64 |
| [`IMAGE_FILE_MACHINE_RISCV128`](#image-file-machine-riscv128) | const | RISCV128 |
| [`IMAGE_FILE_MACHINE_ARM64X`](#image-file-machine-arm64x) | const | ARM64X (Mixed ARM64 and ARM64EC) |
| [`IMAGE_FILE_MACHINE_CHPE_X86`](#image-file-machine-chpe-x86) | const | CHPE x86 ("Compiled Hybrid Portable Executable") |
| [`IMAGE_NUMBEROF_DIRECTORY_ENTRIES`](#image-numberof-directory-entries) | const |  |
| [`IMAGE_NT_OPTIONAL_HDR32_MAGIC`](#image-nt-optional-hdr32-magic) | const |  |
| [`IMAGE_NT_OPTIONAL_HDR64_MAGIC`](#image-nt-optional-hdr64-magic) | const |  |
| [`IMAGE_ROM_OPTIONAL_HDR_MAGIC`](#image-rom-optional-hdr-magic) | const |  |
| [`IMAGE_SUBSYSTEM_UNKNOWN`](#image-subsystem-unknown) | const | Unknown subsystem. |
| [`IMAGE_SUBSYSTEM_NATIVE`](#image-subsystem-native) | const | Image doesn't require a subsystem. |
| [`IMAGE_SUBSYSTEM_WINDOWS_GUI`](#image-subsystem-windows-gui) | const | Image runs in the Windows GUI subsystem. |
| [`IMAGE_SUBSYSTEM_WINDOWS_CUI`](#image-subsystem-windows-cui) | const | Image runs in the Windows character subsystem. |
| [`IMAGE_SUBSYSTEM_OS2_CUI`](#image-subsystem-os2-cui) | const | image runs in the OS/2 character subsystem. |
| [`IMAGE_SUBSYSTEM_POSIX_CUI`](#image-subsystem-posix-cui) | const | image runs in the Posix character subsystem. |
| [`IMAGE_SUBSYSTEM_NATIVE_WINDOWS`](#image-subsystem-native-windows) | const | image is a native Win9x driver. |
| [`IMAGE_SUBSYSTEM_WINDOWS_CE_GUI`](#image-subsystem-windows-ce-gui) | const | Image runs in the Windows CE subsystem. |
| [`IMAGE_SUBSYSTEM_EFI_APPLICATION`](#image-subsystem-efi-application) | const |  |
| [`IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER`](#image-subsystem-efi-boot-service-driver) | const |  |
| [`IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER`](#image-subsystem-efi-runtime-driver) | const |  |
| [`IMAGE_SUBSYSTEM_EFI_ROM`](#image-subsystem-efi-rom) | const |  |
| [`IMAGE_SUBSYSTEM_XBOX`](#image-subsystem-xbox) | const |  |
| [`IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION`](#image-subsystem-windows-boot-application) | const |  |
| [`IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG`](#image-subsystem-xbox-code-catalog) | const |  |
| [`IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA`](#image-dllcharacteristics-high-entropy-va) | const | Image can handle a high entropy 64-bit virtual address space. |
| [`IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE`](#image-dllcharacteristics-dynamic-base) | const | DLL can move. |
| [`IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY`](#image-dllcharacteristics-force-integrity) | const | Code Integrity Image |
| [`IMAGE_DLLCHARACTERISTICS_NX_COMPAT`](#image-dllcharacteristics-nx-compat) | const | Image is NX compatible |
| [`IMAGE_DLLCHARACTERISTICS_NO_ISOLATION`](#image-dllcharacteristics-no-isolation) | const | Image understands isolation and doesn't want it |
| [`IMAGE_DLLCHARACTERISTICS_NO_SEH`](#image-dllcharacteristics-no-seh) | const | Image does not use SEH. |
| [`IMAGE_DLLCHARACTERISTICS_NO_BIND`](#image-dllcharacteristics-no-bind) | const | Do not bind this image. |
| [`IMAGE_DLLCHARACTERISTICS_APPCONTAINER`](#image-dllcharacteristics-appcontainer) | const | Image should execute in an AppContainer |
| [`IMAGE_DLLCHARACTERISTICS_WDM_DRIVER`](#image-dllcharacteristics-wdm-driver) | const | Driver uses WDM model |
| [`IMAGE_DLLCHARACTERISTICS_GUARD_CF`](#image-dllcharacteristics-guard-cf) | const | Image supports Control Flow Guard. |
| [`IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE`](#image-dllcharacteristics-terminal-server-aware) | const |  |
| [`IMAGE_DIRECTORY_ENTRY_EXPORT`](#image-directory-entry-export) | const | Export Directory |
| [`IMAGE_DIRECTORY_ENTRY_IMPORT`](#image-directory-entry-import) | const | Import Directory |
| [`IMAGE_DIRECTORY_ENTRY_RESOURCE`](#image-directory-entry-resource) | const | Resource Directory |
| [`IMAGE_DIRECTORY_ENTRY_EXCEPTION`](#image-directory-entry-exception) | const | Exception Directory |
| [`IMAGE_DIRECTORY_ENTRY_SECURITY`](#image-directory-entry-security) | const | Security Directory |
| [`IMAGE_DIRECTORY_ENTRY_BASERELOC`](#image-directory-entry-basereloc) | const | Base Relocation Table |
| [`IMAGE_DIRECTORY_ENTRY_DEBUG`](#image-directory-entry-debug) | const | Debug Directory |
| [`IMAGE_DIRECTORY_ENTRY_ARCHITECTURE`](#image-directory-entry-architecture) | const | Architecture Specific Data |
| [`IMAGE_DIRECTORY_ENTRY_GLOBALPTR`](#image-directory-entry-globalptr) | const | RVA of GP |
| [`IMAGE_DIRECTORY_ENTRY_TLS`](#image-directory-entry-tls) | const | TLS Directory |
| [`IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG`](#image-directory-entry-load-config) | const | Load Configuration Directory |
| [`IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT`](#image-directory-entry-bound-import) | const | Bound Import Directory in headers |
| [`IMAGE_DIRECTORY_ENTRY_IAT`](#image-directory-entry-iat) | const | Import Address Table |
| [`IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`](#image-directory-entry-delay-import) | const | Delay Load Import Descriptors |
| [`IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`](#image-directory-entry-com-descriptor) | const | COM Runtime descriptor |
| [`ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`](#anon-object-header-bigobj-class-id) | const | The required value of `AnonObjectHeaderBigobj::class_id`. |
| [`IMAGE_SIZEOF_SHORT_NAME`](#image-sizeof-short-name) | const |  |
| [`IMAGE_SIZEOF_SECTION_HEADER`](#image-sizeof-section-header) | const |  |
| [`IMAGE_SCN_TYPE_NO_PAD`](#image-scn-type-no-pad) | const | Reserved. |
| [`IMAGE_SCN_CNT_CODE`](#image-scn-cnt-code) | const | Section contains code. |
| [`IMAGE_SCN_CNT_INITIALIZED_DATA`](#image-scn-cnt-initialized-data) | const | Section contains initialized data. |
| [`IMAGE_SCN_CNT_UNINITIALIZED_DATA`](#image-scn-cnt-uninitialized-data) | const | Section contains uninitialized data. |
| [`IMAGE_SCN_LNK_OTHER`](#image-scn-lnk-other) | const | Reserved. |
| [`IMAGE_SCN_LNK_INFO`](#image-scn-lnk-info) | const | Section contains comments or some other type of information. |
| [`IMAGE_SCN_LNK_REMOVE`](#image-scn-lnk-remove) | const | Section contents will not become part of image. |
| [`IMAGE_SCN_LNK_COMDAT`](#image-scn-lnk-comdat) | const | Section contents comdat. |
| [`IMAGE_SCN_NO_DEFER_SPEC_EXC`](#image-scn-no-defer-spec-exc) | const | Reset speculative exceptions handling bits in the TLB entries for this section. |
| [`IMAGE_SCN_GPREL`](#image-scn-gprel) | const | Section content can be accessed relative to GP |
| [`IMAGE_SCN_MEM_FARDATA`](#image-scn-mem-fardata) | const |  |
| [`IMAGE_SCN_MEM_PURGEABLE`](#image-scn-mem-purgeable) | const |  |
| [`IMAGE_SCN_MEM_16BIT`](#image-scn-mem-16bit) | const |  |
| [`IMAGE_SCN_MEM_LOCKED`](#image-scn-mem-locked) | const |  |
| [`IMAGE_SCN_MEM_PRELOAD`](#image-scn-mem-preload) | const |  |
| [`IMAGE_SCN_ALIGN_1BYTES`](#image-scn-align-1bytes) | const |  |
| [`IMAGE_SCN_ALIGN_2BYTES`](#image-scn-align-2bytes) | const |  |
| [`IMAGE_SCN_ALIGN_4BYTES`](#image-scn-align-4bytes) | const |  |
| [`IMAGE_SCN_ALIGN_8BYTES`](#image-scn-align-8bytes) | const |  |
| [`IMAGE_SCN_ALIGN_16BYTES`](#image-scn-align-16bytes) | const | Default alignment if no others are specified. |
| [`IMAGE_SCN_ALIGN_32BYTES`](#image-scn-align-32bytes) | const |  |
| [`IMAGE_SCN_ALIGN_64BYTES`](#image-scn-align-64bytes) | const |  |
| [`IMAGE_SCN_ALIGN_128BYTES`](#image-scn-align-128bytes) | const |  |
| [`IMAGE_SCN_ALIGN_256BYTES`](#image-scn-align-256bytes) | const |  |
| [`IMAGE_SCN_ALIGN_512BYTES`](#image-scn-align-512bytes) | const |  |
| [`IMAGE_SCN_ALIGN_1024BYTES`](#image-scn-align-1024bytes) | const |  |
| [`IMAGE_SCN_ALIGN_2048BYTES`](#image-scn-align-2048bytes) | const |  |
| [`IMAGE_SCN_ALIGN_4096BYTES`](#image-scn-align-4096bytes) | const |  |
| [`IMAGE_SCN_ALIGN_8192BYTES`](#image-scn-align-8192bytes) | const |  |
| [`IMAGE_SCN_ALIGN_MASK`](#image-scn-align-mask) | const |  |
| [`IMAGE_SCN_LNK_NRELOC_OVFL`](#image-scn-lnk-nreloc-ovfl) | const | Section contains extended relocations. |
| [`IMAGE_SCN_MEM_DISCARDABLE`](#image-scn-mem-discardable) | const | Section can be discarded. |
| [`IMAGE_SCN_MEM_NOT_CACHED`](#image-scn-mem-not-cached) | const | Section is not cacheable. |
| [`IMAGE_SCN_MEM_NOT_PAGED`](#image-scn-mem-not-paged) | const | Section is not pageable. |
| [`IMAGE_SCN_MEM_SHARED`](#image-scn-mem-shared) | const | Section is shareable. |
| [`IMAGE_SCN_MEM_EXECUTE`](#image-scn-mem-execute) | const | Section is executable. |
| [`IMAGE_SCN_MEM_READ`](#image-scn-mem-read) | const | Section is readable. |
| [`IMAGE_SCN_MEM_WRITE`](#image-scn-mem-write) | const | Section is writeable. |
| [`IMAGE_SCN_SCALE_INDEX`](#image-scn-scale-index) | const | Tls index is scaled |
| [`IMAGE_SIZEOF_SYMBOL`](#image-sizeof-symbol) | const |  |
| [`IMAGE_SIZEOF_SYMBOL_EX`](#image-sizeof-symbol-ex) | const |  |
| [`IMAGE_SYM_UNDEFINED`](#image-sym-undefined) | const | Symbol is undefined or is common. |
| [`IMAGE_SYM_ABSOLUTE`](#image-sym-absolute) | const | Symbol is an absolute value. |
| [`IMAGE_SYM_DEBUG`](#image-sym-debug) | const | Symbol is a special debug item. |
| [`IMAGE_SYM_SECTION_MAX`](#image-sym-section-max) | const | Values 0xFF00-0xFFFF are special |
| [`IMAGE_SYM_SECTION_MAX_EX`](#image-sym-section-max-ex) | const |  |
| [`IMAGE_SYM_TYPE_NULL`](#image-sym-type-null) | const | no type. |
| [`IMAGE_SYM_TYPE_VOID`](#image-sym-type-void) | const |  |
| [`IMAGE_SYM_TYPE_CHAR`](#image-sym-type-char) | const | type character. |
| [`IMAGE_SYM_TYPE_SHORT`](#image-sym-type-short) | const | type short integer. |
| [`IMAGE_SYM_TYPE_INT`](#image-sym-type-int) | const |  |
| [`IMAGE_SYM_TYPE_LONG`](#image-sym-type-long) | const |  |
| [`IMAGE_SYM_TYPE_FLOAT`](#image-sym-type-float) | const |  |
| [`IMAGE_SYM_TYPE_DOUBLE`](#image-sym-type-double) | const |  |
| [`IMAGE_SYM_TYPE_STRUCT`](#image-sym-type-struct) | const |  |
| [`IMAGE_SYM_TYPE_UNION`](#image-sym-type-union) | const |  |
| [`IMAGE_SYM_TYPE_ENUM`](#image-sym-type-enum) | const | enumeration. |
| [`IMAGE_SYM_TYPE_MOE`](#image-sym-type-moe) | const | member of enumeration. |
| [`IMAGE_SYM_TYPE_BYTE`](#image-sym-type-byte) | const |  |
| [`IMAGE_SYM_TYPE_WORD`](#image-sym-type-word) | const |  |
| [`IMAGE_SYM_TYPE_UINT`](#image-sym-type-uint) | const |  |
| [`IMAGE_SYM_TYPE_DWORD`](#image-sym-type-dword) | const |  |
| [`IMAGE_SYM_TYPE_PCODE`](#image-sym-type-pcode) | const |  |
| [`IMAGE_SYM_DTYPE_NULL`](#image-sym-dtype-null) | const | no derived type. |
| [`IMAGE_SYM_DTYPE_POINTER`](#image-sym-dtype-pointer) | const | pointer. |
| [`IMAGE_SYM_DTYPE_FUNCTION`](#image-sym-dtype-function) | const | function. |
| [`IMAGE_SYM_DTYPE_ARRAY`](#image-sym-dtype-array) | const | array. |
| [`IMAGE_SYM_CLASS_END_OF_FUNCTION`](#image-sym-class-end-of-function) | const |  |
| [`IMAGE_SYM_CLASS_NULL`](#image-sym-class-null) | const |  |
| [`IMAGE_SYM_CLASS_AUTOMATIC`](#image-sym-class-automatic) | const |  |
| [`IMAGE_SYM_CLASS_EXTERNAL`](#image-sym-class-external) | const |  |
| [`IMAGE_SYM_CLASS_STATIC`](#image-sym-class-static) | const |  |
| [`IMAGE_SYM_CLASS_REGISTER`](#image-sym-class-register) | const |  |
| [`IMAGE_SYM_CLASS_EXTERNAL_DEF`](#image-sym-class-external-def) | const |  |
| [`IMAGE_SYM_CLASS_LABEL`](#image-sym-class-label) | const |  |
| [`IMAGE_SYM_CLASS_UNDEFINED_LABEL`](#image-sym-class-undefined-label) | const |  |
| [`IMAGE_SYM_CLASS_MEMBER_OF_STRUCT`](#image-sym-class-member-of-struct) | const |  |
| [`IMAGE_SYM_CLASS_ARGUMENT`](#image-sym-class-argument) | const |  |
| [`IMAGE_SYM_CLASS_STRUCT_TAG`](#image-sym-class-struct-tag) | const |  |
| [`IMAGE_SYM_CLASS_MEMBER_OF_UNION`](#image-sym-class-member-of-union) | const |  |
| [`IMAGE_SYM_CLASS_UNION_TAG`](#image-sym-class-union-tag) | const |  |
| [`IMAGE_SYM_CLASS_TYPE_DEFINITION`](#image-sym-class-type-definition) | const |  |
| [`IMAGE_SYM_CLASS_UNDEFINED_STATIC`](#image-sym-class-undefined-static) | const |  |
| [`IMAGE_SYM_CLASS_ENUM_TAG`](#image-sym-class-enum-tag) | const |  |
| [`IMAGE_SYM_CLASS_MEMBER_OF_ENUM`](#image-sym-class-member-of-enum) | const |  |
| [`IMAGE_SYM_CLASS_REGISTER_PARAM`](#image-sym-class-register-param) | const |  |
| [`IMAGE_SYM_CLASS_BIT_FIELD`](#image-sym-class-bit-field) | const |  |
| [`IMAGE_SYM_CLASS_FAR_EXTERNAL`](#image-sym-class-far-external) | const |  |
| [`IMAGE_SYM_CLASS_BLOCK`](#image-sym-class-block) | const |  |
| [`IMAGE_SYM_CLASS_FUNCTION`](#image-sym-class-function) | const |  |
| [`IMAGE_SYM_CLASS_END_OF_STRUCT`](#image-sym-class-end-of-struct) | const |  |
| [`IMAGE_SYM_CLASS_FILE`](#image-sym-class-file) | const |  |
| [`IMAGE_SYM_CLASS_SECTION`](#image-sym-class-section) | const |  |
| [`IMAGE_SYM_CLASS_WEAK_EXTERNAL`](#image-sym-class-weak-external) | const |  |
| [`IMAGE_SYM_CLASS_CLR_TOKEN`](#image-sym-class-clr-token) | const |  |
| [`N_BTMASK`](#n-btmask) | const |  |
| [`N_TMASK`](#n-tmask) | const |  |
| [`N_TMASK1`](#n-tmask1) | const |  |
| [`N_TMASK2`](#n-tmask2) | const |  |
| [`N_BTSHFT`](#n-btshft) | const |  |
| [`N_TSHIFT`](#n-tshift) | const |  |
| [`IMAGE_SYM_DTYPE_SHIFT`](#image-sym-dtype-shift) | const |  |
| [`IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF`](#image-aux-symbol-type-token-def) | const |  |
| [`IMAGE_COMDAT_SELECT_NODUPLICATES`](#image-comdat-select-noduplicates) | const |  |
| [`IMAGE_COMDAT_SELECT_ANY`](#image-comdat-select-any) | const |  |
| [`IMAGE_COMDAT_SELECT_SAME_SIZE`](#image-comdat-select-same-size) | const |  |
| [`IMAGE_COMDAT_SELECT_EXACT_MATCH`](#image-comdat-select-exact-match) | const |  |
| [`IMAGE_COMDAT_SELECT_ASSOCIATIVE`](#image-comdat-select-associative) | const |  |
| [`IMAGE_COMDAT_SELECT_LARGEST`](#image-comdat-select-largest) | const |  |
| [`IMAGE_COMDAT_SELECT_NEWEST`](#image-comdat-select-newest) | const |  |
| [`IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY`](#image-weak-extern-search-nolibrary) | const |  |
| [`IMAGE_WEAK_EXTERN_SEARCH_LIBRARY`](#image-weak-extern-search-library) | const |  |
| [`IMAGE_WEAK_EXTERN_SEARCH_ALIAS`](#image-weak-extern-search-alias) | const |  |
| [`IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY`](#image-weak-extern-anti-dependency) | const |  |
| [`IMAGE_REL_I386_ABSOLUTE`](#image-rel-i386-absolute) | const | Reference is absolute, no relocation is necessary |
| [`IMAGE_REL_I386_DIR16`](#image-rel-i386-dir16) | const | Direct 16-bit reference to the symbols virtual address |
| [`IMAGE_REL_I386_REL16`](#image-rel-i386-rel16) | const | PC-relative 16-bit reference to the symbols virtual address |
| [`IMAGE_REL_I386_DIR32`](#image-rel-i386-dir32) | const | Direct 32-bit reference to the symbols virtual address |
| [`IMAGE_REL_I386_DIR32NB`](#image-rel-i386-dir32nb) | const | Direct 32-bit reference to the symbols virtual address, base not included |
| [`IMAGE_REL_I386_SEG12`](#image-rel-i386-seg12) | const | Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address |
| [`IMAGE_REL_I386_SECTION`](#image-rel-i386-section) | const |  |
| [`IMAGE_REL_I386_SECREL`](#image-rel-i386-secrel) | const |  |
| [`IMAGE_REL_I386_TOKEN`](#image-rel-i386-token) | const | clr token |
| [`IMAGE_REL_I386_SECREL7`](#image-rel-i386-secrel7) | const | 7 bit offset from base of section containing target |
| [`IMAGE_REL_I386_REL32`](#image-rel-i386-rel32) | const | PC-relative 32-bit reference to the symbols virtual address |
| [`IMAGE_REL_MIPS_ABSOLUTE`](#image-rel-mips-absolute) | const | Reference is absolute, no relocation is necessary |
| [`IMAGE_REL_MIPS_REFHALF`](#image-rel-mips-refhalf) | const |  |
| [`IMAGE_REL_MIPS_REFWORD`](#image-rel-mips-refword) | const |  |
| [`IMAGE_REL_MIPS_JMPADDR`](#image-rel-mips-jmpaddr) | const |  |
| [`IMAGE_REL_MIPS_REFHI`](#image-rel-mips-refhi) | const |  |
| [`IMAGE_REL_MIPS_REFLO`](#image-rel-mips-reflo) | const |  |
| [`IMAGE_REL_MIPS_GPREL`](#image-rel-mips-gprel) | const |  |
| [`IMAGE_REL_MIPS_LITERAL`](#image-rel-mips-literal) | const |  |
| [`IMAGE_REL_MIPS_SECTION`](#image-rel-mips-section) | const |  |
| [`IMAGE_REL_MIPS_SECREL`](#image-rel-mips-secrel) | const |  |
| [`IMAGE_REL_MIPS_SECRELLO`](#image-rel-mips-secrello) | const | Low 16-bit section relative reference (used for >32k TLS) |
| [`IMAGE_REL_MIPS_SECRELHI`](#image-rel-mips-secrelhi) | const | High 16-bit section relative reference (used for >32k TLS) |
| [`IMAGE_REL_MIPS_TOKEN`](#image-rel-mips-token) | const | clr token |
| [`IMAGE_REL_MIPS_JMPADDR16`](#image-rel-mips-jmpaddr16) | const |  |
| [`IMAGE_REL_MIPS_REFWORDNB`](#image-rel-mips-refwordnb) | const |  |
| [`IMAGE_REL_MIPS_PAIR`](#image-rel-mips-pair) | const |  |
| [`IMAGE_REL_ALPHA_ABSOLUTE`](#image-rel-alpha-absolute) | const |  |
| [`IMAGE_REL_ALPHA_REFLONG`](#image-rel-alpha-reflong) | const |  |
| [`IMAGE_REL_ALPHA_REFQUAD`](#image-rel-alpha-refquad) | const |  |
| [`IMAGE_REL_ALPHA_GPREL32`](#image-rel-alpha-gprel32) | const |  |
| [`IMAGE_REL_ALPHA_LITERAL`](#image-rel-alpha-literal) | const |  |
| [`IMAGE_REL_ALPHA_LITUSE`](#image-rel-alpha-lituse) | const |  |
| [`IMAGE_REL_ALPHA_GPDISP`](#image-rel-alpha-gpdisp) | const |  |
| [`IMAGE_REL_ALPHA_BRADDR`](#image-rel-alpha-braddr) | const |  |
| [`IMAGE_REL_ALPHA_HINT`](#image-rel-alpha-hint) | const |  |
| [`IMAGE_REL_ALPHA_INLINE_REFLONG`](#image-rel-alpha-inline-reflong) | const |  |
| [`IMAGE_REL_ALPHA_REFHI`](#image-rel-alpha-refhi) | const |  |
| [`IMAGE_REL_ALPHA_REFLO`](#image-rel-alpha-reflo) | const |  |
| [`IMAGE_REL_ALPHA_PAIR`](#image-rel-alpha-pair) | const |  |
| [`IMAGE_REL_ALPHA_MATCH`](#image-rel-alpha-match) | const |  |
| [`IMAGE_REL_ALPHA_SECTION`](#image-rel-alpha-section) | const |  |
| [`IMAGE_REL_ALPHA_SECREL`](#image-rel-alpha-secrel) | const |  |
| [`IMAGE_REL_ALPHA_REFLONGNB`](#image-rel-alpha-reflongnb) | const |  |
| [`IMAGE_REL_ALPHA_SECRELLO`](#image-rel-alpha-secrello) | const | Low 16-bit section relative reference |
| [`IMAGE_REL_ALPHA_SECRELHI`](#image-rel-alpha-secrelhi) | const | High 16-bit section relative reference |
| [`IMAGE_REL_ALPHA_REFQ3`](#image-rel-alpha-refq3) | const | High 16 bits of 48 bit reference |
| [`IMAGE_REL_ALPHA_REFQ2`](#image-rel-alpha-refq2) | const | Middle 16 bits of 48 bit reference |
| [`IMAGE_REL_ALPHA_REFQ1`](#image-rel-alpha-refq1) | const | Low 16 bits of 48 bit reference |
| [`IMAGE_REL_ALPHA_GPRELLO`](#image-rel-alpha-gprello) | const | Low 16-bit GP relative reference |
| [`IMAGE_REL_ALPHA_GPRELHI`](#image-rel-alpha-gprelhi) | const | High 16-bit GP relative reference |
| [`IMAGE_REL_PPC_ABSOLUTE`](#image-rel-ppc-absolute) | const | NOP |
| [`IMAGE_REL_PPC_ADDR64`](#image-rel-ppc-addr64) | const | 64-bit address |
| [`IMAGE_REL_PPC_ADDR32`](#image-rel-ppc-addr32) | const | 32-bit address |
| [`IMAGE_REL_PPC_ADDR24`](#image-rel-ppc-addr24) | const | 26-bit address, shifted left 2 (branch absolute) |
| [`IMAGE_REL_PPC_ADDR16`](#image-rel-ppc-addr16) | const | 16-bit address |
| [`IMAGE_REL_PPC_ADDR14`](#image-rel-ppc-addr14) | const | 16-bit address, shifted left 2 (load doubleword) |
| [`IMAGE_REL_PPC_REL24`](#image-rel-ppc-rel24) | const | 26-bit PC-relative offset, shifted left 2 (branch relative) |
| [`IMAGE_REL_PPC_REL14`](#image-rel-ppc-rel14) | const | 16-bit PC-relative offset, shifted left 2 (br cond relative) |
| [`IMAGE_REL_PPC_TOCREL16`](#image-rel-ppc-tocrel16) | const | 16-bit offset from TOC base |
| [`IMAGE_REL_PPC_TOCREL14`](#image-rel-ppc-tocrel14) | const | 16-bit offset from TOC base, shifted left 2 (load doubleword) |
| [`IMAGE_REL_PPC_ADDR32NB`](#image-rel-ppc-addr32nb) | const | 32-bit addr w/o image base |
| [`IMAGE_REL_PPC_SECREL`](#image-rel-ppc-secrel) | const | va of containing section (as in an image sectionhdr) |
| [`IMAGE_REL_PPC_SECTION`](#image-rel-ppc-section) | const | sectionheader number |
| [`IMAGE_REL_PPC_IFGLUE`](#image-rel-ppc-ifglue) | const | substitute TOC restore instruction iff symbol is glue code |
| [`IMAGE_REL_PPC_IMGLUE`](#image-rel-ppc-imglue) | const | symbol is glue code; virtual address is TOC restore instruction |
| [`IMAGE_REL_PPC_SECREL16`](#image-rel-ppc-secrel16) | const | va of containing section (limited to 16 bits) |
| [`IMAGE_REL_PPC_REFHI`](#image-rel-ppc-refhi) | const |  |
| [`IMAGE_REL_PPC_REFLO`](#image-rel-ppc-reflo) | const |  |
| [`IMAGE_REL_PPC_PAIR`](#image-rel-ppc-pair) | const |  |
| [`IMAGE_REL_PPC_SECRELLO`](#image-rel-ppc-secrello) | const | Low 16-bit section relative reference (used for >32k TLS) |
| [`IMAGE_REL_PPC_SECRELHI`](#image-rel-ppc-secrelhi) | const | High 16-bit section relative reference (used for >32k TLS) |
| [`IMAGE_REL_PPC_GPREL`](#image-rel-ppc-gprel) | const |  |
| [`IMAGE_REL_PPC_TOKEN`](#image-rel-ppc-token) | const | clr token |
| [`IMAGE_REL_PPC_TYPEMASK`](#image-rel-ppc-typemask) | const | mask to isolate above values in IMAGE_RELOCATION.Type |
| [`IMAGE_REL_PPC_NEG`](#image-rel-ppc-neg) | const | subtract reloc value rather than adding it |
| [`IMAGE_REL_PPC_BRTAKEN`](#image-rel-ppc-brtaken) | const | fix branch prediction bit to predict branch taken |
| [`IMAGE_REL_PPC_BRNTAKEN`](#image-rel-ppc-brntaken) | const | fix branch prediction bit to predict branch not taken |
| [`IMAGE_REL_PPC_TOCDEFN`](#image-rel-ppc-tocdefn) | const | toc slot defined in file (or, data in toc) |
| [`IMAGE_REL_SH3_ABSOLUTE`](#image-rel-sh3-absolute) | const | No relocation |
| [`IMAGE_REL_SH3_DIRECT16`](#image-rel-sh3-direct16) | const | 16 bit direct |
| [`IMAGE_REL_SH3_DIRECT32`](#image-rel-sh3-direct32) | const | 32 bit direct |
| [`IMAGE_REL_SH3_DIRECT8`](#image-rel-sh3-direct8) | const | 8 bit direct, -128..255 |
| [`IMAGE_REL_SH3_DIRECT8_WORD`](#image-rel-sh3-direct8-word) | const | 8 bit direct .W (0 ext.) |
| [`IMAGE_REL_SH3_DIRECT8_LONG`](#image-rel-sh3-direct8-long) | const | 8 bit direct .L (0 ext.) |
| [`IMAGE_REL_SH3_DIRECT4`](#image-rel-sh3-direct4) | const | 4 bit direct (0 ext.) |
| [`IMAGE_REL_SH3_DIRECT4_WORD`](#image-rel-sh3-direct4-word) | const | 4 bit direct .W (0 ext.) |
| [`IMAGE_REL_SH3_DIRECT4_LONG`](#image-rel-sh3-direct4-long) | const | 4 bit direct .L (0 ext.) |
| [`IMAGE_REL_SH3_PCREL8_WORD`](#image-rel-sh3-pcrel8-word) | const | 8 bit PC relative .W |
| [`IMAGE_REL_SH3_PCREL8_LONG`](#image-rel-sh3-pcrel8-long) | const | 8 bit PC relative .L |
| [`IMAGE_REL_SH3_PCREL12_WORD`](#image-rel-sh3-pcrel12-word) | const | 12 LSB PC relative .W |
| [`IMAGE_REL_SH3_STARTOF_SECTION`](#image-rel-sh3-startof-section) | const | Start of EXE section |
| [`IMAGE_REL_SH3_SIZEOF_SECTION`](#image-rel-sh3-sizeof-section) | const | Size of EXE section |
| [`IMAGE_REL_SH3_SECTION`](#image-rel-sh3-section) | const | Section table index |
| [`IMAGE_REL_SH3_SECREL`](#image-rel-sh3-secrel) | const | Offset within section |
| [`IMAGE_REL_SH3_DIRECT32_NB`](#image-rel-sh3-direct32-nb) | const | 32 bit direct not based |
| [`IMAGE_REL_SH3_GPREL4_LONG`](#image-rel-sh3-gprel4-long) | const | GP-relative addressing |
| [`IMAGE_REL_SH3_TOKEN`](#image-rel-sh3-token) | const | clr token |
| [`IMAGE_REL_SHM_PCRELPT`](#image-rel-shm-pcrelpt) | const | Offset from current instruction in longwords if not NOMODE, insert the inverse of the low bit at bit 32 to select PTA/PTB |
| [`IMAGE_REL_SHM_REFLO`](#image-rel-shm-reflo) | const | Low bits of 32-bit address |
| [`IMAGE_REL_SHM_REFHALF`](#image-rel-shm-refhalf) | const | High bits of 32-bit address |
| [`IMAGE_REL_SHM_RELLO`](#image-rel-shm-rello) | const | Low bits of relative reference |
| [`IMAGE_REL_SHM_RELHALF`](#image-rel-shm-relhalf) | const | High bits of relative reference |
| [`IMAGE_REL_SHM_PAIR`](#image-rel-shm-pair) | const | offset operand for relocation |
| [`IMAGE_REL_SH_NOMODE`](#image-rel-sh-nomode) | const | relocation ignores section mode |
| [`IMAGE_REL_ARM_ABSOLUTE`](#image-rel-arm-absolute) | const | No relocation required |
| [`IMAGE_REL_ARM_ADDR32`](#image-rel-arm-addr32) | const | 32 bit address |
| [`IMAGE_REL_ARM_ADDR32NB`](#image-rel-arm-addr32nb) | const | 32 bit address w/o image base |
| [`IMAGE_REL_ARM_BRANCH24`](#image-rel-arm-branch24) | const | 24 bit offset << 2 & sign ext. |
| [`IMAGE_REL_ARM_BRANCH11`](#image-rel-arm-branch11) | const | Thumb: 2 11 bit offsets |
| [`IMAGE_REL_ARM_TOKEN`](#image-rel-arm-token) | const | clr token |
| [`IMAGE_REL_ARM_GPREL12`](#image-rel-arm-gprel12) | const | GP-relative addressing (ARM) |
| [`IMAGE_REL_ARM_GPREL7`](#image-rel-arm-gprel7) | const | GP-relative addressing (Thumb) |
| [`IMAGE_REL_ARM_BLX24`](#image-rel-arm-blx24) | const |  |
| [`IMAGE_REL_ARM_BLX11`](#image-rel-arm-blx11) | const |  |
| [`IMAGE_REL_ARM_REL32`](#image-rel-arm-rel32) | const | 32-bit relative address from byte following reloc |
| [`IMAGE_REL_ARM_SECTION`](#image-rel-arm-section) | const | Section table index |
| [`IMAGE_REL_ARM_SECREL`](#image-rel-arm-secrel) | const | Offset within section |
| [`IMAGE_REL_ARM_MOV32A`](#image-rel-arm-mov32a) | const | ARM: MOVW/MOVT |
| [`IMAGE_REL_ARM_MOV32`](#image-rel-arm-mov32) | const | ARM: MOVW/MOVT (deprecated) |
| [`IMAGE_REL_ARM_MOV32T`](#image-rel-arm-mov32t) | const | Thumb: MOVW/MOVT |
| [`IMAGE_REL_THUMB_MOV32`](#image-rel-thumb-mov32) | const | Thumb: MOVW/MOVT (deprecated) |
| [`IMAGE_REL_ARM_BRANCH20T`](#image-rel-arm-branch20t) | const | Thumb: 32-bit conditional B |
| [`IMAGE_REL_THUMB_BRANCH20`](#image-rel-thumb-branch20) | const | Thumb: 32-bit conditional B (deprecated) |
| [`IMAGE_REL_ARM_BRANCH24T`](#image-rel-arm-branch24t) | const | Thumb: 32-bit B or BL |
| [`IMAGE_REL_THUMB_BRANCH24`](#image-rel-thumb-branch24) | const | Thumb: 32-bit B or BL (deprecated) |
| [`IMAGE_REL_ARM_BLX23T`](#image-rel-arm-blx23t) | const | Thumb: BLX immediate |
| [`IMAGE_REL_THUMB_BLX23`](#image-rel-thumb-blx23) | const | Thumb: BLX immediate (deprecated) |
| [`IMAGE_REL_AM_ABSOLUTE`](#image-rel-am-absolute) | const |  |
| [`IMAGE_REL_AM_ADDR32`](#image-rel-am-addr32) | const |  |
| [`IMAGE_REL_AM_ADDR32NB`](#image-rel-am-addr32nb) | const |  |
| [`IMAGE_REL_AM_CALL32`](#image-rel-am-call32) | const |  |
| [`IMAGE_REL_AM_FUNCINFO`](#image-rel-am-funcinfo) | const |  |
| [`IMAGE_REL_AM_REL32_1`](#image-rel-am-rel32-1) | const |  |
| [`IMAGE_REL_AM_REL32_2`](#image-rel-am-rel32-2) | const |  |
| [`IMAGE_REL_AM_SECREL`](#image-rel-am-secrel) | const |  |
| [`IMAGE_REL_AM_SECTION`](#image-rel-am-section) | const |  |
| [`IMAGE_REL_AM_TOKEN`](#image-rel-am-token) | const |  |
| [`IMAGE_REL_ARM64_ABSOLUTE`](#image-rel-arm64-absolute) | const | No relocation required |
| [`IMAGE_REL_ARM64_ADDR32`](#image-rel-arm64-addr32) | const | 32 bit address. |
| [`IMAGE_REL_ARM64_ADDR32NB`](#image-rel-arm64-addr32nb) | const | 32 bit address w/o image base (RVA: for Data/PData/XData) |
| [`IMAGE_REL_ARM64_BRANCH26`](#image-rel-arm64-branch26) | const | 26 bit offset << 2 & sign ext. |
| [`IMAGE_REL_ARM64_PAGEBASE_REL21`](#image-rel-arm64-pagebase-rel21) | const | ADRP |
| [`IMAGE_REL_ARM64_REL21`](#image-rel-arm64-rel21) | const | ADR |
| [`IMAGE_REL_ARM64_PAGEOFFSET_12A`](#image-rel-arm64-pageoffset-12a) | const | ADD/ADDS (immediate) with zero shift, for page offset |
| [`IMAGE_REL_ARM64_PAGEOFFSET_12L`](#image-rel-arm64-pageoffset-12l) | const | LDR (indexed, unsigned immediate), for page offset |
| [`IMAGE_REL_ARM64_SECREL`](#image-rel-arm64-secrel) | const | Offset within section |
| [`IMAGE_REL_ARM64_SECREL_LOW12A`](#image-rel-arm64-secrel-low12a) | const | ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset |
| [`IMAGE_REL_ARM64_SECREL_HIGH12A`](#image-rel-arm64-secrel-high12a) | const | ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset |
| [`IMAGE_REL_ARM64_SECREL_LOW12L`](#image-rel-arm64-secrel-low12l) | const | LDR (indexed, unsigned immediate), for bit 0:11 of section offset |
| [`IMAGE_REL_ARM64_TOKEN`](#image-rel-arm64-token) | const |  |
| [`IMAGE_REL_ARM64_SECTION`](#image-rel-arm64-section) | const | Section table index |
| [`IMAGE_REL_ARM64_ADDR64`](#image-rel-arm64-addr64) | const | 64 bit address |
| [`IMAGE_REL_ARM64_BRANCH19`](#image-rel-arm64-branch19) | const | 19 bit offset << 2 & sign ext. |
| [`IMAGE_REL_ARM64_BRANCH14`](#image-rel-arm64-branch14) | const | TBZ/TBNZ |
| [`IMAGE_REL_ARM64_REL32`](#image-rel-arm64-rel32) | const | 32-bit relative address from byte following reloc |
| [`IMAGE_REL_AMD64_ABSOLUTE`](#image-rel-amd64-absolute) | const | Reference is absolute, no relocation is necessary |
| [`IMAGE_REL_AMD64_ADDR64`](#image-rel-amd64-addr64) | const | 64-bit address (VA). |
| [`IMAGE_REL_AMD64_ADDR32`](#image-rel-amd64-addr32) | const | 32-bit address (VA). |
| [`IMAGE_REL_AMD64_ADDR32NB`](#image-rel-amd64-addr32nb) | const | 32-bit address w/o image base (RVA). |
| [`IMAGE_REL_AMD64_REL32`](#image-rel-amd64-rel32) | const | 32-bit relative address from byte following reloc |
| [`IMAGE_REL_AMD64_REL32_1`](#image-rel-amd64-rel32-1) | const | 32-bit relative address from byte distance 1 from reloc |
| [`IMAGE_REL_AMD64_REL32_2`](#image-rel-amd64-rel32-2) | const | 32-bit relative address from byte distance 2 from reloc |
| [`IMAGE_REL_AMD64_REL32_3`](#image-rel-amd64-rel32-3) | const | 32-bit relative address from byte distance 3 from reloc |
| [`IMAGE_REL_AMD64_REL32_4`](#image-rel-amd64-rel32-4) | const | 32-bit relative address from byte distance 4 from reloc |
| [`IMAGE_REL_AMD64_REL32_5`](#image-rel-amd64-rel32-5) | const | 32-bit relative address from byte distance 5 from reloc |
| [`IMAGE_REL_AMD64_SECTION`](#image-rel-amd64-section) | const | Section index |
| [`IMAGE_REL_AMD64_SECREL`](#image-rel-amd64-secrel) | const | 32 bit offset from base of section containing target |
| [`IMAGE_REL_AMD64_SECREL7`](#image-rel-amd64-secrel7) | const | 7 bit unsigned offset from base of section containing target |
| [`IMAGE_REL_AMD64_TOKEN`](#image-rel-amd64-token) | const | 32 bit metadata token |
| [`IMAGE_REL_AMD64_SREL32`](#image-rel-amd64-srel32) | const | 32 bit signed span-dependent value emitted into object |
| [`IMAGE_REL_AMD64_PAIR`](#image-rel-amd64-pair) | const |  |
| [`IMAGE_REL_AMD64_SSPAN32`](#image-rel-amd64-sspan32) | const | 32 bit signed span-dependent value applied at link time |
| [`IMAGE_REL_AMD64_EHANDLER`](#image-rel-amd64-ehandler) | const |  |
| [`IMAGE_REL_AMD64_IMPORT_BR`](#image-rel-amd64-import-br) | const | Indirect branch to an import |
| [`IMAGE_REL_AMD64_IMPORT_CALL`](#image-rel-amd64-import-call) | const | Indirect call to an import |
| [`IMAGE_REL_AMD64_CFG_BR`](#image-rel-amd64-cfg-br) | const | Indirect branch to a CFG check |
| [`IMAGE_REL_AMD64_CFG_BR_REX`](#image-rel-amd64-cfg-br-rex) | const | Indirect branch to a CFG check, with REX.W prefix |
| [`IMAGE_REL_AMD64_CFG_CALL`](#image-rel-amd64-cfg-call) | const | Indirect call to a CFG check |
| [`IMAGE_REL_AMD64_INDIR_BR`](#image-rel-amd64-indir-br) | const | Indirect branch to a target in RAX (no CFG) |
| [`IMAGE_REL_AMD64_INDIR_BR_REX`](#image-rel-amd64-indir-br-rex) | const | Indirect branch to a target in RAX, with REX.W prefix (no CFG) |
| [`IMAGE_REL_AMD64_INDIR_CALL`](#image-rel-amd64-indir-call) | const | Indirect call to a target in RAX (no CFG) |
| [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST`](#image-rel-amd64-indir-br-switchtable-first) | const | Indirect branch for a switch table using Reg 0 (RAX) |
| [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST`](#image-rel-amd64-indir-br-switchtable-last) | const | Indirect branch for a switch table using Reg 15 (R15) |
| [`IMAGE_REL_IA64_ABSOLUTE`](#image-rel-ia64-absolute) | const |  |
| [`IMAGE_REL_IA64_IMM14`](#image-rel-ia64-imm14) | const |  |
| [`IMAGE_REL_IA64_IMM22`](#image-rel-ia64-imm22) | const |  |
| [`IMAGE_REL_IA64_IMM64`](#image-rel-ia64-imm64) | const |  |
| [`IMAGE_REL_IA64_DIR32`](#image-rel-ia64-dir32) | const |  |
| [`IMAGE_REL_IA64_DIR64`](#image-rel-ia64-dir64) | const |  |
| [`IMAGE_REL_IA64_PCREL21B`](#image-rel-ia64-pcrel21b) | const |  |
| [`IMAGE_REL_IA64_PCREL21M`](#image-rel-ia64-pcrel21m) | const |  |
| [`IMAGE_REL_IA64_PCREL21F`](#image-rel-ia64-pcrel21f) | const |  |
| [`IMAGE_REL_IA64_GPREL22`](#image-rel-ia64-gprel22) | const |  |
| [`IMAGE_REL_IA64_LTOFF22`](#image-rel-ia64-ltoff22) | const |  |
| [`IMAGE_REL_IA64_SECTION`](#image-rel-ia64-section) | const |  |
| [`IMAGE_REL_IA64_SECREL22`](#image-rel-ia64-secrel22) | const |  |
| [`IMAGE_REL_IA64_SECREL64I`](#image-rel-ia64-secrel64i) | const |  |
| [`IMAGE_REL_IA64_SECREL32`](#image-rel-ia64-secrel32) | const |  |
| [`IMAGE_REL_IA64_DIR32NB`](#image-rel-ia64-dir32nb) | const |  |
| [`IMAGE_REL_IA64_SREL14`](#image-rel-ia64-srel14) | const |  |
| [`IMAGE_REL_IA64_SREL22`](#image-rel-ia64-srel22) | const |  |
| [`IMAGE_REL_IA64_SREL32`](#image-rel-ia64-srel32) | const |  |
| [`IMAGE_REL_IA64_UREL32`](#image-rel-ia64-urel32) | const |  |
| [`IMAGE_REL_IA64_PCREL60X`](#image-rel-ia64-pcrel60x) | const | This is always a BRL and never converted |
| [`IMAGE_REL_IA64_PCREL60B`](#image-rel-ia64-pcrel60b) | const | If possible, convert to MBB bundle with NOP.B in slot 1 |
| [`IMAGE_REL_IA64_PCREL60F`](#image-rel-ia64-pcrel60f) | const | If possible, convert to MFB bundle with NOP.F in slot 1 |
| [`IMAGE_REL_IA64_PCREL60I`](#image-rel-ia64-pcrel60i) | const | If possible, convert to MIB bundle with NOP.I in slot 1 |
| [`IMAGE_REL_IA64_PCREL60M`](#image-rel-ia64-pcrel60m) | const | If possible, convert to MMB bundle with NOP.M in slot 1 |
| [`IMAGE_REL_IA64_IMMGPREL64`](#image-rel-ia64-immgprel64) | const |  |
| [`IMAGE_REL_IA64_TOKEN`](#image-rel-ia64-token) | const | clr token |
| [`IMAGE_REL_IA64_GPREL32`](#image-rel-ia64-gprel32) | const |  |
| [`IMAGE_REL_IA64_ADDEND`](#image-rel-ia64-addend) | const |  |
| [`IMAGE_REL_CEF_ABSOLUTE`](#image-rel-cef-absolute) | const | Reference is absolute, no relocation is necessary |
| [`IMAGE_REL_CEF_ADDR32`](#image-rel-cef-addr32) | const | 32-bit address (VA). |
| [`IMAGE_REL_CEF_ADDR64`](#image-rel-cef-addr64) | const | 64-bit address (VA). |
| [`IMAGE_REL_CEF_ADDR32NB`](#image-rel-cef-addr32nb) | const | 32-bit address w/o image base (RVA). |
| [`IMAGE_REL_CEF_SECTION`](#image-rel-cef-section) | const | Section index |
| [`IMAGE_REL_CEF_SECREL`](#image-rel-cef-secrel) | const | 32 bit offset from base of section containing target |
| [`IMAGE_REL_CEF_TOKEN`](#image-rel-cef-token) | const | 32 bit metadata token |
| [`IMAGE_REL_CEE_ABSOLUTE`](#image-rel-cee-absolute) | const | Reference is absolute, no relocation is necessary |
| [`IMAGE_REL_CEE_ADDR32`](#image-rel-cee-addr32) | const | 32-bit address (VA). |
| [`IMAGE_REL_CEE_ADDR64`](#image-rel-cee-addr64) | const | 64-bit address (VA). |
| [`IMAGE_REL_CEE_ADDR32NB`](#image-rel-cee-addr32nb) | const | 32-bit address w/o image base (RVA). |
| [`IMAGE_REL_CEE_SECTION`](#image-rel-cee-section) | const | Section index |
| [`IMAGE_REL_CEE_SECREL`](#image-rel-cee-secrel) | const | 32 bit offset from base of section containing target |
| [`IMAGE_REL_CEE_TOKEN`](#image-rel-cee-token) | const | 32 bit metadata token |
| [`IMAGE_REL_M32R_ABSOLUTE`](#image-rel-m32r-absolute) | const | No relocation required |
| [`IMAGE_REL_M32R_ADDR32`](#image-rel-m32r-addr32) | const | 32 bit address |
| [`IMAGE_REL_M32R_ADDR32NB`](#image-rel-m32r-addr32nb) | const | 32 bit address w/o image base |
| [`IMAGE_REL_M32R_ADDR24`](#image-rel-m32r-addr24) | const | 24 bit address |
| [`IMAGE_REL_M32R_GPREL16`](#image-rel-m32r-gprel16) | const | GP relative addressing |
| [`IMAGE_REL_M32R_PCREL24`](#image-rel-m32r-pcrel24) | const | 24 bit offset << 2 & sign ext. |
| [`IMAGE_REL_M32R_PCREL16`](#image-rel-m32r-pcrel16) | const | 16 bit offset << 2 & sign ext. |
| [`IMAGE_REL_M32R_PCREL8`](#image-rel-m32r-pcrel8) | const | 8 bit offset << 2 & sign ext. |
| [`IMAGE_REL_M32R_REFHALF`](#image-rel-m32r-refhalf) | const | 16 MSBs |
| [`IMAGE_REL_M32R_REFHI`](#image-rel-m32r-refhi) | const | 16 MSBs; adj for LSB sign ext. |
| [`IMAGE_REL_M32R_REFLO`](#image-rel-m32r-reflo) | const | 16 LSBs |
| [`IMAGE_REL_M32R_PAIR`](#image-rel-m32r-pair) | const | Link HI and LO |
| [`IMAGE_REL_M32R_SECTION`](#image-rel-m32r-section) | const | Section table index |
| [`IMAGE_REL_M32R_SECREL32`](#image-rel-m32r-secrel32) | const | 32 bit section relative reference |
| [`IMAGE_REL_M32R_TOKEN`](#image-rel-m32r-token) | const | clr token |
| [`IMAGE_REL_EBC_ABSOLUTE`](#image-rel-ebc-absolute) | const | No relocation required |
| [`IMAGE_REL_EBC_ADDR32NB`](#image-rel-ebc-addr32nb) | const | 32 bit address w/o image base |
| [`IMAGE_REL_EBC_REL32`](#image-rel-ebc-rel32) | const | 32-bit relative address from byte following reloc |
| [`IMAGE_REL_EBC_SECTION`](#image-rel-ebc-section) | const | Section table index |
| [`IMAGE_REL_EBC_SECREL`](#image-rel-ebc-secrel) | const | Offset within section |
| [`EMARCH_ENC_I17_IMM7B_INST_WORD_X`](#emarch-enc-i17-imm7b-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM7B_SIZE_X`](#emarch-enc-i17-imm7b-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X`](#emarch-enc-i17-imm7b-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM7B_VAL_POS_X`](#emarch-enc-i17-imm7b-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM9D_INST_WORD_X`](#emarch-enc-i17-imm9d-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM9D_SIZE_X`](#emarch-enc-i17-imm9d-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X`](#emarch-enc-i17-imm9d-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM9D_VAL_POS_X`](#emarch-enc-i17-imm9d-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM5C_INST_WORD_X`](#emarch-enc-i17-imm5c-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM5C_SIZE_X`](#emarch-enc-i17-imm5c-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X`](#emarch-enc-i17-imm5c-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM5C_VAL_POS_X`](#emarch-enc-i17-imm5c-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IC_INST_WORD_X`](#emarch-enc-i17-ic-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IC_SIZE_X`](#emarch-enc-i17-ic-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IC_INST_WORD_POS_X`](#emarch-enc-i17-ic-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IC_VAL_POS_X`](#emarch-enc-i17-ic-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41A_INST_WORD_X`](#emarch-enc-i17-imm41a-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41A_SIZE_X`](#emarch-enc-i17-imm41a-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X`](#emarch-enc-i17-imm41a-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41A_VAL_POS_X`](#emarch-enc-i17-imm41a-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41B_INST_WORD_X`](#emarch-enc-i17-imm41b-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41B_SIZE_X`](#emarch-enc-i17-imm41b-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X`](#emarch-enc-i17-imm41b-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41B_VAL_POS_X`](#emarch-enc-i17-imm41b-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41C_INST_WORD_X`](#emarch-enc-i17-imm41c-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41C_SIZE_X`](#emarch-enc-i17-imm41c-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X`](#emarch-enc-i17-imm41c-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_IMM41C_VAL_POS_X`](#emarch-enc-i17-imm41c-val-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_SIGN_INST_WORD_X`](#emarch-enc-i17-sign-inst-word-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_SIGN_SIZE_X`](#emarch-enc-i17-sign-size-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_SIGN_INST_WORD_POS_X`](#emarch-enc-i17-sign-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`EMARCH_ENC_I17_SIGN_VAL_POS_X`](#emarch-enc-i17-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_OPCODE_INST_WORD_X`](#x3-opcode-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_OPCODE_SIZE_X`](#x3-opcode-size-x) | const | Intel-IA64-Filler |
| [`X3_OPCODE_INST_WORD_POS_X`](#x3-opcode-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_OPCODE_SIGN_VAL_POS_X`](#x3-opcode-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_I_INST_WORD_X`](#x3-i-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_I_SIZE_X`](#x3-i-size-x) | const | Intel-IA64-Filler |
| [`X3_I_INST_WORD_POS_X`](#x3-i-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_I_SIGN_VAL_POS_X`](#x3-i-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_D_WH_INST_WORD_X`](#x3-d-wh-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_D_WH_SIZE_X`](#x3-d-wh-size-x) | const | Intel-IA64-Filler |
| [`X3_D_WH_INST_WORD_POS_X`](#x3-d-wh-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_D_WH_SIGN_VAL_POS_X`](#x3-d-wh-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM20_INST_WORD_X`](#x3-imm20-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_IMM20_SIZE_X`](#x3-imm20-size-x) | const | Intel-IA64-Filler |
| [`X3_IMM20_INST_WORD_POS_X`](#x3-imm20-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM20_SIGN_VAL_POS_X`](#x3-imm20-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_1_INST_WORD_X`](#x3-imm39-1-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_1_SIZE_X`](#x3-imm39-1-size-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_1_INST_WORD_POS_X`](#x3-imm39-1-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_1_SIGN_VAL_POS_X`](#x3-imm39-1-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_2_INST_WORD_X`](#x3-imm39-2-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_2_SIZE_X`](#x3-imm39-2-size-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_2_INST_WORD_POS_X`](#x3-imm39-2-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_IMM39_2_SIGN_VAL_POS_X`](#x3-imm39-2-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_P_INST_WORD_X`](#x3-p-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_P_SIZE_X`](#x3-p-size-x) | const | Intel-IA64-Filler |
| [`X3_P_INST_WORD_POS_X`](#x3-p-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_P_SIGN_VAL_POS_X`](#x3-p-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_TMPLT_INST_WORD_X`](#x3-tmplt-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_TMPLT_SIZE_X`](#x3-tmplt-size-x) | const | Intel-IA64-Filler |
| [`X3_TMPLT_INST_WORD_POS_X`](#x3-tmplt-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_TMPLT_SIGN_VAL_POS_X`](#x3-tmplt-sign-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_BTYPE_QP_INST_WORD_X`](#x3-btype-qp-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_BTYPE_QP_SIZE_X`](#x3-btype-qp-size-x) | const | Intel-IA64-Filler |
| [`X3_BTYPE_QP_INST_WORD_POS_X`](#x3-btype-qp-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_BTYPE_QP_INST_VAL_POS_X`](#x3-btype-qp-inst-val-pos-x) | const | Intel-IA64-Filler |
| [`X3_EMPTY_INST_WORD_X`](#x3-empty-inst-word-x) | const | Intel-IA64-Filler |
| [`X3_EMPTY_SIZE_X`](#x3-empty-size-x) | const | Intel-IA64-Filler |
| [`X3_EMPTY_INST_WORD_POS_X`](#x3-empty-inst-word-pos-x) | const | Intel-IA64-Filler |
| [`X3_EMPTY_INST_VAL_POS_X`](#x3-empty-inst-val-pos-x) | const | Intel-IA64-Filler |
| [`IMAGE_REL_BASED_ABSOLUTE`](#image-rel-based-absolute) | const |  |
| [`IMAGE_REL_BASED_HIGH`](#image-rel-based-high) | const |  |
| [`IMAGE_REL_BASED_LOW`](#image-rel-based-low) | const |  |
| [`IMAGE_REL_BASED_HIGHLOW`](#image-rel-based-highlow) | const |  |
| [`IMAGE_REL_BASED_HIGHADJ`](#image-rel-based-highadj) | const |  |
| [`IMAGE_REL_BASED_MACHINE_SPECIFIC_5`](#image-rel-based-machine-specific-5) | const |  |
| [`IMAGE_REL_BASED_RESERVED`](#image-rel-based-reserved) | const |  |
| [`IMAGE_REL_BASED_MACHINE_SPECIFIC_7`](#image-rel-based-machine-specific-7) | const |  |
| [`IMAGE_REL_BASED_MACHINE_SPECIFIC_8`](#image-rel-based-machine-specific-8) | const |  |
| [`IMAGE_REL_BASED_MACHINE_SPECIFIC_9`](#image-rel-based-machine-specific-9) | const |  |
| [`IMAGE_REL_BASED_DIR64`](#image-rel-based-dir64) | const |  |
| [`IMAGE_REL_BASED_IA64_IMM64`](#image-rel-based-ia64-imm64) | const |  |
| [`IMAGE_REL_BASED_MIPS_JMPADDR`](#image-rel-based-mips-jmpaddr) | const |  |
| [`IMAGE_REL_BASED_MIPS_JMPADDR16`](#image-rel-based-mips-jmpaddr16) | const |  |
| [`IMAGE_REL_BASED_ARM_MOV32`](#image-rel-based-arm-mov32) | const |  |
| [`IMAGE_REL_BASED_THUMB_MOV32`](#image-rel-based-thumb-mov32) | const |  |
| [`IMAGE_REL_BASED_RISCV_HIGH20`](#image-rel-based-riscv-high20) | const |  |
| [`IMAGE_REL_BASED_RISCV_LOW12I`](#image-rel-based-riscv-low12i) | const |  |
| [`IMAGE_REL_BASED_RISCV_LOW12S`](#image-rel-based-riscv-low12s) | const |  |
| [`IMAGE_ARCHIVE_START_SIZE`](#image-archive-start-size) | const |  |
| [`IMAGE_ARCHIVE_START`](#image-archive-start) | const |  |
| [`IMAGE_ARCHIVE_END`](#image-archive-end) | const |  |
| [`IMAGE_ARCHIVE_PAD`](#image-archive-pad) | const |  |
| [`IMAGE_ARCHIVE_LINKER_MEMBER`](#image-archive-linker-member) | const |  |
| [`IMAGE_ARCHIVE_LONGNAMES_MEMBER`](#image-archive-longnames-member) | const |  |
| [`IMAGE_ARCHIVE_HYBRIDMAP_MEMBER`](#image-archive-hybridmap-member) | const |  |
| [`IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR`](#image-sizeof-archive-member-hdr) | const |  |
| [`IMAGE_ORDINAL_FLAG64`](#image-ordinal-flag64) | const |  |
| [`IMAGE_ORDINAL_FLAG32`](#image-ordinal-flag32) | const |  |
| [`IMAGE_DELAYLOAD_RVA_BASED`](#image-delayload-rva-based) | const | Delay load version 2 flag for `ImageDelayloadDescriptor::attributes`. |
| [`IMAGE_RESOURCE_NAME_IS_STRING`](#image-resource-name-is-string) | const |  |
| [`IMAGE_RESOURCE_DATA_IS_DIRECTORY`](#image-resource-data-is-directory) | const |  |
| [`RT_CURSOR`](#rt-cursor) | const | ID for: Hardware-dependent cursor resource. |
| [`RT_BITMAP`](#rt-bitmap) | const | ID for: Bitmap resource. |
| [`RT_ICON`](#rt-icon) | const | ID for: Hardware-dependent icon resource. |
| [`RT_MENU`](#rt-menu) | const | ID for: Menu resource. |
| [`RT_DIALOG`](#rt-dialog) | const | ID for: Dialog box. |
| [`RT_STRING`](#rt-string) | const | ID for: String-table entry. |
| [`RT_FONTDIR`](#rt-fontdir) | const | ID for: Font directory resource. |
| [`RT_FONT`](#rt-font) | const | ID for: Font resource. |
| [`RT_ACCELERATOR`](#rt-accelerator) | const | ID for: Accelerator table. |
| [`RT_RCDATA`](#rt-rcdata) | const | ID for: Application-defined resource (raw data). |
| [`RT_MESSAGETABLE`](#rt-messagetable) | const | ID for: Message-table entry. |
| [`RT_GROUP_CURSOR`](#rt-group-cursor) | const | ID for: Hardware-independent cursor resource. |
| [`RT_GROUP_ICON`](#rt-group-icon) | const | ID for: Hardware-independent icon resource. |
| [`RT_VERSION`](#rt-version) | const | ID for: Version resource. |
| [`RT_DLGINCLUDE`](#rt-dlginclude) | const | ID for: Allows a resource editing tool to associate a string with an .rc file. |
| [`RT_PLUGPLAY`](#rt-plugplay) | const | ID for: Plug and Play resource. |
| [`RT_VXD`](#rt-vxd) | const | ID for: VXD. |
| [`RT_ANICURSOR`](#rt-anicursor) | const | ID for: Animated cursor. |
| [`RT_ANIICON`](#rt-aniicon) | const | ID for: Animated icon. |
| [`RT_HTML`](#rt-html) | const | ID for: HTML resource. |
| [`RT_MANIFEST`](#rt-manifest) | const | ID for: Side-by-Side Assembly Manifest. |
| [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE`](#image-dynamic-relocation-guard-rf-prologue) | const |  |
| [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE`](#image-dynamic-relocation-guard-rf-epilogue) | const |  |
| [`IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER`](#image-dynamic-relocation-guard-import-control-transfer) | const |  |
| [`IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER`](#image-dynamic-relocation-guard-indir-control-transfer) | const |  |
| [`IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH`](#image-dynamic-relocation-guard-switchtable-branch) | const |  |
| [`IMAGE_HOT_PATCH_BASE_OBLIGATORY`](#image-hot-patch-base-obligatory) | const |  |
| [`IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK`](#image-hot-patch-base-can-roll-back) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_INVERSE`](#image-hot-patch-chunk-inverse) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_OBLIGATORY`](#image-hot-patch-chunk-obligatory) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_RESERVED`](#image-hot-patch-chunk-reserved) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_TYPE`](#image-hot-patch-chunk-type) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA`](#image-hot-patch-chunk-source-rva) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_TARGET_RVA`](#image-hot-patch-chunk-target-rva) | const |  |
| [`IMAGE_HOT_PATCH_CHUNK_SIZE`](#image-hot-patch-chunk-size) | const |  |
| [`IMAGE_HOT_PATCH_NONE`](#image-hot-patch-none) | const |  |
| [`IMAGE_HOT_PATCH_FUNCTION`](#image-hot-patch-function) | const |  |
| [`IMAGE_HOT_PATCH_ABSOLUTE`](#image-hot-patch-absolute) | const |  |
| [`IMAGE_HOT_PATCH_REL32`](#image-hot-patch-rel32) | const |  |
| [`IMAGE_HOT_PATCH_CALL_TARGET`](#image-hot-patch-call-target) | const |  |
| [`IMAGE_HOT_PATCH_INDIRECT`](#image-hot-patch-indirect) | const |  |
| [`IMAGE_HOT_PATCH_NO_CALL_TARGET`](#image-hot-patch-no-call-target) | const |  |
| [`IMAGE_HOT_PATCH_DYNAMIC_VALUE`](#image-hot-patch-dynamic-value) | const |  |
| [`IMAGE_GUARD_CF_INSTRUMENTED`](#image-guard-cf-instrumented) | const | Module performs control flow integrity checks using system-supplied support |
| [`IMAGE_GUARD_CFW_INSTRUMENTED`](#image-guard-cfw-instrumented) | const | Module performs control flow and write integrity checks |
| [`IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT`](#image-guard-cf-function-table-present) | const | Module contains valid control flow target metadata |
| [`IMAGE_GUARD_SECURITY_COOKIE_UNUSED`](#image-guard-security-cookie-unused) | const | Module does not make use of the /GS security cookie |
| [`IMAGE_GUARD_PROTECT_DELAYLOAD_IAT`](#image-guard-protect-delayload-iat) | const | Module supports read only delay load IAT |
| [`IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION`](#image-guard-delayload-iat-in-its-own-section) | const | Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected |
| [`IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT`](#image-guard-cf-export-suppression-info-present) | const | Module contains suppressed export information. |
| [`IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION`](#image-guard-cf-enable-export-suppression) | const | Module enables suppression of exports |
| [`IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT`](#image-guard-cf-longjump-table-present) | const | Module contains longjmp target information |
| [`IMAGE_GUARD_RF_INSTRUMENTED`](#image-guard-rf-instrumented) | const | Module contains return flow instrumentation and metadata |
| [`IMAGE_GUARD_RF_ENABLE`](#image-guard-rf-enable) | const | Module requests that the OS enable return flow protection |
| [`IMAGE_GUARD_RF_STRICT`](#image-guard-rf-strict) | const | Module requests that the OS enable return flow protection in strict mode |
| [`IMAGE_GUARD_RETPOLINE_PRESENT`](#image-guard-retpoline-present) | const | Module was built with retpoline support |
| [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK`](#image-guard-cf-function-table-size-mask) | const | Stride of Guard CF function table encoded in these bits (additional count of bytes per element) |
| [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT`](#image-guard-cf-function-table-size-shift) | const | Shift to right-justify Guard CF function table stride |
| [`IMAGE_GUARD_FLAG_FID_SUPPRESSED`](#image-guard-flag-fid-suppressed) | const | The containing GFID entry is suppressed |
| [`IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED`](#image-guard-flag-export-suppressed) | const | The containing GFID entry is export suppressed |
| [`IMAGE_ENCLAVE_LONG_ID_LENGTH`](#image-enclave-long-id-length) | const |  |
| [`IMAGE_ENCLAVE_SHORT_ID_LENGTH`](#image-enclave-short-id-length) | const |  |
| [`IMAGE_ENCLAVE_POLICY_DEBUGGABLE`](#image-enclave-policy-debuggable) | const |  |
| [`IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE`](#image-enclave-flag-primary-image) | const |  |
| [`IMAGE_ENCLAVE_IMPORT_MATCH_NONE`](#image-enclave-import-match-none) | const |  |
| [`IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID`](#image-enclave-import-match-unique-id) | const |  |
| [`IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID`](#image-enclave-import-match-author-id) | const |  |
| [`IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID`](#image-enclave-import-match-family-id) | const |  |
| [`IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID`](#image-enclave-import-match-image-id) | const |  |
| [`IMAGE_DEBUG_TYPE_UNKNOWN`](#image-debug-type-unknown) | const |  |
| [`IMAGE_DEBUG_TYPE_COFF`](#image-debug-type-coff) | const |  |
| [`IMAGE_DEBUG_TYPE_CODEVIEW`](#image-debug-type-codeview) | const |  |
| [`IMAGE_DEBUG_TYPE_FPO`](#image-debug-type-fpo) | const |  |
| [`IMAGE_DEBUG_TYPE_MISC`](#image-debug-type-misc) | const |  |
| [`IMAGE_DEBUG_TYPE_EXCEPTION`](#image-debug-type-exception) | const |  |
| [`IMAGE_DEBUG_TYPE_FIXUP`](#image-debug-type-fixup) | const |  |
| [`IMAGE_DEBUG_TYPE_OMAP_TO_SRC`](#image-debug-type-omap-to-src) | const |  |
| [`IMAGE_DEBUG_TYPE_OMAP_FROM_SRC`](#image-debug-type-omap-from-src) | const |  |
| [`IMAGE_DEBUG_TYPE_BORLAND`](#image-debug-type-borland) | const |  |
| [`IMAGE_DEBUG_TYPE_RESERVED10`](#image-debug-type-reserved10) | const |  |
| [`IMAGE_DEBUG_TYPE_CLSID`](#image-debug-type-clsid) | const |  |
| [`IMAGE_DEBUG_TYPE_VC_FEATURE`](#image-debug-type-vc-feature) | const |  |
| [`IMAGE_DEBUG_TYPE_POGO`](#image-debug-type-pogo) | const |  |
| [`IMAGE_DEBUG_TYPE_ILTCG`](#image-debug-type-iltcg) | const |  |
| [`IMAGE_DEBUG_TYPE_MPX`](#image-debug-type-mpx) | const |  |
| [`IMAGE_DEBUG_TYPE_REPRO`](#image-debug-type-repro) | const |  |
| [`FRAME_FPO`](#frame-fpo) | const |  |
| [`FRAME_TRAP`](#frame-trap) | const |  |
| [`FRAME_TSS`](#frame-tss) | const |  |
| [`FRAME_NONFPO`](#frame-nonfpo) | const |  |
| [`IMAGE_DEBUG_MISC_EXENAME`](#image-debug-misc-exename) | const |  |
| [`IMAGE_SEPARATE_DEBUG_SIGNATURE`](#image-separate-debug-signature) | const |  |
| [`NON_PAGED_DEBUG_SIGNATURE`](#non-paged-debug-signature) | const |  |
| [`IMAGE_SEPARATE_DEBUG_FLAGS_MASK`](#image-separate-debug-flags-mask) | const |  |
| [`IMAGE_SEPARATE_DEBUG_MISMATCH`](#image-separate-debug-mismatch) | const | when DBG was updated, the old checksum didn't match. |
| [`IMPORT_OBJECT_HDR_SIG2`](#import-object-hdr-sig2) | const |  |
| [`IMPORT_OBJECT_TYPE_MASK`](#import-object-type-mask) | const |  |
| [`IMPORT_OBJECT_TYPE_SHIFT`](#import-object-type-shift) | const |  |
| [`IMPORT_OBJECT_CODE`](#import-object-code) | const |  |
| [`IMPORT_OBJECT_DATA`](#import-object-data) | const |  |
| [`IMPORT_OBJECT_CONST`](#import-object-const) | const |  |
| [`IMPORT_OBJECT_NAME_MASK`](#import-object-name-mask) | const |  |
| [`IMPORT_OBJECT_NAME_SHIFT`](#import-object-name-shift) | const |  |
| [`IMPORT_OBJECT_ORDINAL`](#import-object-ordinal) | const | Import by ordinal |
| [`IMPORT_OBJECT_NAME`](#import-object-name) | const | Import name == public symbol name. |
| [`IMPORT_OBJECT_NAME_NO_PREFIX`](#import-object-name-no-prefix) | const | Import name == public symbol name skipping leading ?, @, or optionally _. |
| [`IMPORT_OBJECT_NAME_UNDECORATE`](#import-object-name-undecorate) | const | Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @. |
| [`IMPORT_OBJECT_NAME_EXPORTAS`](#import-object-name-exportas) | const | Import name == a name is explicitly provided after the DLL name. |
| [`COMIMAGE_FLAGS_ILONLY`](#comimage-flags-ilonly) | const |  |
| [`COMIMAGE_FLAGS_32BITREQUIRED`](#comimage-flags-32bitrequired) | const |  |
| [`COMIMAGE_FLAGS_IL_LIBRARY`](#comimage-flags-il-library) | const |  |
| [`COMIMAGE_FLAGS_STRONGNAMESIGNED`](#comimage-flags-strongnamesigned) | const |  |
| [`COMIMAGE_FLAGS_NATIVE_ENTRYPOINT`](#comimage-flags-native-entrypoint) | const |  |
| [`COMIMAGE_FLAGS_TRACKDEBUGDATA`](#comimage-flags-trackdebugdata) | const |  |
| [`COMIMAGE_FLAGS_32BITPREFERRED`](#comimage-flags-32bitpreferred) | const |  |
| [`COR_VERSION_MAJOR_V2`](#cor-version-major-v2) | const |  |
| [`COR_VERSION_MAJOR`](#cor-version-major) | const |  |
| [`COR_VERSION_MINOR`](#cor-version-minor) | const |  |
| [`COR_DELETED_NAME_LENGTH`](#cor-deleted-name-length) | const |  |
| [`COR_VTABLEGAP_NAME_LENGTH`](#cor-vtablegap-name-length) | const |  |
| [`NATIVE_TYPE_MAX_CB`](#native-type-max-cb) | const |  |
| [`COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE`](#cor-ilmethod-sect-small-max-datasize) | const |  |
| [`IMAGE_COR_MIH_METHODRVA`](#image-cor-mih-methodrva) | const |  |
| [`IMAGE_COR_MIH_EHRVA`](#image-cor-mih-ehrva) | const |  |
| [`IMAGE_COR_MIH_BASICBLOCK`](#image-cor-mih-basicblock) | const |  |
| [`COR_VTABLE_32BIT`](#cor-vtable-32bit) | const | V-table slots are 32-bits in size. |
| [`COR_VTABLE_64BIT`](#cor-vtable-64bit) | const | V-table slots are 64-bits in size. |
| [`COR_VTABLE_FROM_UNMANAGED`](#cor-vtable-from-unmanaged) | const | If set, transition from unmanaged. |
| [`COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN`](#cor-vtable-from-unmanaged-retain-appdomain) | const | If set, transition from unmanaged with keeping the current appdomain. |
| [`COR_VTABLE_CALL_MOST_DERIVED`](#cor-vtable-call-most-derived) | const | Call most derived method described by |
| [`IMAGE_COR_EATJ_THUNK_SIZE`](#image-cor-eatj-thunk-size) | const | Size of a jump thunk reserved range. |
| [`MAX_CLASS_NAME`](#max-class-name) | const |  |
| [`MAX_PACKAGE_NAME`](#max-package-name) | const |  |

## Structs

### `ImageDosHeader`

```rust
struct ImageDosHeader {
    pub e_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cblp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_crlc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cparhdr: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_minalloc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_maxalloc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ss: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_sp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_csum: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ip: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cs: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_lfarlc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ovno: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_res: [crate::endian::U16<crate::endian::LittleEndian>; 4],
    pub e_oemid: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_oeminfo: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_res2: [crate::endian::U16<crate::endian::LittleEndian>; 10],
    pub e_lfanew: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:29-68`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L29-L68)*

DOS .EXE header

#### Fields

- **`e_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`e_cblp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Bytes on last page of file

- **`e_cp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Pages in file

- **`e_crlc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Relocations

- **`e_cparhdr`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Size of header in paragraphs

- **`e_minalloc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Minimum extra paragraphs needed

- **`e_maxalloc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Maximum extra paragraphs needed

- **`e_ss`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial (relative) SS value

- **`e_sp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial SP value

- **`e_csum`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Checksum

- **`e_ip`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial IP value

- **`e_cs`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial (relative) CS value

- **`e_lfarlc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  File address of relocation table

- **`e_ovno`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Overlay number

- **`e_res`**: `[crate::endian::U16<crate::endian::LittleEndian>; 4]`

  Reserved words

- **`e_oemid`**: `crate::endian::U16<crate::endian::LittleEndian>`

  OEM identifier (for e_oeminfo)

- **`e_oeminfo`**: `crate::endian::U16<crate::endian::LittleEndian>`

  OEM information; e_oemid specific

- **`e_res2`**: `[crate::endian::U16<crate::endian::LittleEndian>; 10]`

  Reserved words

- **`e_lfanew`**: `crate::endian::U32<crate::endian::LittleEndian>`

  File address of new exe header

#### Implementations

- <span id="peimagedosheader-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self>` — [`Result`](../index.md#result)

- <span id="peimagedosheader-nt-headers-offset"></span>`fn nt_headers_offset(&self) -> u32`

#### Trait Implementations

##### `impl Clone for ImageDosHeader`

- <span id="imagedosheader-clone"></span>`fn clone(&self) -> ImageDosHeader` — [`ImageDosHeader`](#imagedosheader)

##### `impl Copy for ImageDosHeader`

##### `impl Debug for ImageDosHeader`

- <span id="imagedosheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDosHeader`

### `ImageOs2Header`

```rust
struct ImageOs2Header {
    pub ne_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_ver: i8,
    pub ne_rev: i8,
    pub ne_enttab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cbenttab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_crc: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_autodata: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_heap: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_stack: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_csip: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_sssp: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_cseg: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cmod: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cbnrestab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_segtab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_rsrctab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_restab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_modtab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_imptab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_nrestab: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_cmovent: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_align: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cres: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_psegrefbytes: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_swaparea: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_expver: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:73-134`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L73-L134)*

OS/2 .EXE header

#### Fields

- **`ne_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`ne_ver`**: `i8`

  Version number

- **`ne_rev`**: `i8`

  Revision number

- **`ne_enttab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Entry Table

- **`ne_cbenttab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Number of bytes in Entry Table

- **`ne_crc`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Checksum of whole file

- **`ne_flags`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Flag word

- **`ne_autodata`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Automatic data segment number

- **`ne_heap`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial heap allocation

- **`ne_stack`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial stack allocation

- **`ne_csip`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Initial CS:IP setting

- **`ne_sssp`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Initial SS:SP setting

- **`ne_cseg`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of file segments

- **`ne_cmod`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Entries in Module Reference Table

- **`ne_cbnrestab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Size of non-resident name table

- **`ne_segtab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Segment Table

- **`ne_rsrctab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Resource Table

- **`ne_restab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of resident name table

- **`ne_modtab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Module Reference Table

- **`ne_imptab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Imported Names Table

- **`ne_nrestab`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Offset of Non-resident Names Table

- **`ne_cmovent`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of movable entries

- **`ne_align`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Segment alignment shift count

- **`ne_cres`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of resource segments

- **`ne_exetyp`**: `u8`

  Target Operating system

- **`ne_flagsothers`**: `u8`

  Other .EXE flags

- **`ne_pretthunks`**: `crate::endian::U16<crate::endian::LittleEndian>`

  offset to return thunks

- **`ne_psegrefbytes`**: `crate::endian::U16<crate::endian::LittleEndian>`

  offset to segment ref. bytes

- **`ne_swaparea`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Minimum code swap area size

- **`ne_expver`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Expected Windows version number

#### Trait Implementations

##### `impl Clone for ImageOs2Header`

- <span id="imageos2header-clone"></span>`fn clone(&self) -> ImageOs2Header` — [`ImageOs2Header`](#imageos2header)

##### `impl Copy for ImageOs2Header`

##### `impl Debug for ImageOs2Header`

- <span id="imageos2header-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageOs2Header`

### `ImageVxdHeader`

```rust
struct ImageVxdHeader {
    pub e32_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_border: u8,
    pub e32_worder: u8,
    pub e32_level: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_cpu: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_os: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_ver: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_mflags: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_mpages: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_startobj: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_eip: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_stackobj: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_esp: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_pagesize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_lastpagesize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fixupsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fixupsum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_ldrsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_ldrsum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objtab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objcnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objmap: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_itermap: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_rsrctab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_rsrccnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_restab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_enttab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_dirtab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_dircnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fpagetab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_frectab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impmod: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impmodcnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impproc: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_pagesum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_datapage: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_preload: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_nrestab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_cbnrestab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_nressum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_autodata: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_debuginfo: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_debuglen: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_instpreload: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_instdemand: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_heapsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_res3: [u8; 12],
    pub e32_winresoff: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_winreslen: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_devid: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_ddkver: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:139-240`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L139-L240)*

Windows VXD header

#### Fields

- **`e32_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`e32_border`**: `u8`

  The byte ordering for the VXD

- **`e32_worder`**: `u8`

  The word ordering for the VXD

- **`e32_level`**: `crate::endian::U32<crate::endian::LittleEndian>`

  The EXE format level for now = 0

- **`e32_cpu`**: `crate::endian::U16<crate::endian::LittleEndian>`

  The CPU type

- **`e32_os`**: `crate::endian::U16<crate::endian::LittleEndian>`

  The OS type

- **`e32_ver`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module version

- **`e32_mflags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module flags

- **`e32_mpages`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module # pages

- **`e32_startobj`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for instruction pointer

- **`e32_eip`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Extended instruction pointer

- **`e32_stackobj`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for stack pointer

- **`e32_esp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Extended stack pointer

- **`e32_pagesize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VXD page size

- **`e32_lastpagesize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Last page size in VXD

- **`e32_fixupsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Fixup section size

- **`e32_fixupsum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Fixup section checksum

- **`e32_ldrsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Loader section size

- **`e32_ldrsum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Loader section checksum

- **`e32_objtab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object table offset

- **`e32_objcnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of objects in module

- **`e32_objmap`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object page map offset

- **`e32_itermap`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object iterated data map offset

- **`e32_rsrctab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Resource Table

- **`e32_rsrccnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of resource entries

- **`e32_restab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of resident name table

- **`e32_enttab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Entry Table

- **`e32_dirtab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Module Directive Table

- **`e32_dircnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of module directives

- **`e32_fpagetab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Fixup Page Table

- **`e32_frectab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Fixup Record Table

- **`e32_impmod`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Import Module Name Table

- **`e32_impmodcnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of entries in Import Module Name Table

- **`e32_impproc`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Import Procedure Name Table

- **`e32_pagesum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Per-Page Checksum Table

- **`e32_datapage`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Enumerated Data Pages

- **`e32_preload`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of preload pages

- **`e32_nrestab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Non-resident Names Table

- **`e32_cbnrestab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of Non-resident Name Table

- **`e32_nressum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Non-resident Name Table Checksum

- **`e32_autodata`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for automatic data object

- **`e32_debuginfo`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of the debugging information

- **`e32_debuglen`**: `crate::endian::U32<crate::endian::LittleEndian>`

  The length of the debugging info. in bytes

- **`e32_instpreload`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of instance pages in preload section of VXD file

- **`e32_instdemand`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of instance pages in demand load section of VXD file

- **`e32_heapsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of heap - for 16-bit apps

- **`e32_res3`**: `[u8; 12]`

  Reserved words

- **`e32_devid`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Device ID for VxD

- **`e32_ddkver`**: `crate::endian::U16<crate::endian::LittleEndian>`

  DDK version for VxD

#### Trait Implementations

##### `impl Clone for ImageVxdHeader`

- <span id="imagevxdheader-clone"></span>`fn clone(&self) -> ImageVxdHeader` — [`ImageVxdHeader`](#imagevxdheader)

##### `impl Copy for ImageVxdHeader`

##### `impl Debug for ImageVxdHeader`

- <span id="imagevxdheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageVxdHeader`

### `MaskedRichHeaderEntry`

```rust
struct MaskedRichHeaderEntry {
    pub masked_comp_id: crate::endian::U32<crate::endian::LittleEndian>,
    pub masked_count: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:252-255`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L252-L255)*

A PE rich header entry.

Rich headers have no official documentation, but have been heavily
reversed-engineered and documented in the wild, e.g.:
* `http://www.ntcore.com/files/richsign.htm`
* `https://www.researchgate.net/figure/Structure-of-the-Rich-Header_fig1_318145388`

This data is "masked", i.e. XORed with a checksum derived from the file data.

#### Trait Implementations

##### `impl Clone for MaskedRichHeaderEntry`

- <span id="maskedrichheaderentry-clone"></span>`fn clone(&self) -> MaskedRichHeaderEntry` — [`MaskedRichHeaderEntry`](#maskedrichheaderentry)

##### `impl Copy for MaskedRichHeaderEntry`

##### `impl Debug for MaskedRichHeaderEntry`

- <span id="maskedrichheaderentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for MaskedRichHeaderEntry`

### `ImageFileHeader`

```rust
struct ImageFileHeader {
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_optional_header: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:263-271`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L263-L271)*

#### Trait Implementations

##### `impl Clone for ImageFileHeader`

- <span id="imagefileheader-clone"></span>`fn clone(&self) -> ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

##### `impl CoffHeader for pe::ImageFileHeader`

- <span id="peimagefileheader-coffheader-type-imagesymbol"></span>`type ImageSymbol = ImageSymbol`

- <span id="peimagefileheader-coffheader-type-imagesymbolbytes"></span>`type ImageSymbolBytes = ImageSymbolBytes`

- <span id="peimagefileheader-is-type-bigobj"></span>`fn is_type_bigobj() -> bool`

- <span id="peimagefileheader-machine"></span>`fn machine(&self) -> u16`

- <span id="peimagefileheader-number-of-sections"></span>`fn number_of_sections(&self) -> u32`

- <span id="peimagefileheader-pointer-to-symbol-table"></span>`fn pointer_to_symbol_table(&self) -> u32`

- <span id="peimagefileheader-number-of-symbols"></span>`fn number_of_symbols(&self) -> u32`

- <span id="peimagefileheader-characteristics"></span>`fn characteristics(&self) -> u16`

- <span id="peimagefileheader-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>` — [`Result`](../index.md#result)

##### `impl Copy for ImageFileHeader`

##### `impl Debug for ImageFileHeader`

- <span id="imagefileheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageFileHeader`

### `ImageDataDirectory`

```rust
struct ImageDataDirectory {
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:384-387`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L384-L387)*

#### Implementations

- <span id="peimagedatadirectory-address-range"></span>`fn address_range(&self) -> (u32, u32)`

- <span id="peimagedatadirectory-file-range"></span>`fn file_range(&self, sections: &SectionTable<'_>) -> Result<(u32, u32)>` — [`SectionTable`](../read/coff/index.md#sectiontable), [`Result`](../index.md#result)

- <span id="peimagedatadirectory-data"></span>`fn data<'data, R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<&'data [u8]>` — [`SectionTable`](../read/coff/index.md#sectiontable), [`Result`](../index.md#result)

#### Trait Implementations

##### `impl Clone for ImageDataDirectory`

- <span id="imagedatadirectory-clone"></span>`fn clone(&self) -> ImageDataDirectory` — [`ImageDataDirectory`](#imagedatadirectory)

##### `impl Copy for ImageDataDirectory`

##### `impl Debug for ImageDataDirectory`

- <span id="imagedatadirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDataDirectory`

### `ImageOptionalHeader32`

```rust
struct ImageOptionalHeader32 {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U32<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub win32_version_value: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_headers: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub subsystem: crate::endian::U16<crate::endian::LittleEndian>,
    pub dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub size_of_stack_reserve: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_stack_commit: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_heap_reserve: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_heap_commit: crate::endian::U32<crate::endian::LittleEndian>,
    pub loader_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:397-432`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L397-L432)*

#### Trait Implementations

##### `impl Clone for ImageOptionalHeader32`

- <span id="imageoptionalheader32-clone"></span>`fn clone(&self) -> ImageOptionalHeader32` — [`ImageOptionalHeader32`](#imageoptionalheader32)

##### `impl Copy for ImageOptionalHeader32`

##### `impl Debug for ImageOptionalHeader32`

- <span id="imageoptionalheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageOptionalHeader for pe::ImageOptionalHeader32`

- <span id="peimageoptionalheader32-magic"></span>`fn magic(&self) -> u16`

- <span id="peimageoptionalheader32-major-linker-version"></span>`fn major_linker_version(&self) -> u8`

- <span id="peimageoptionalheader32-minor-linker-version"></span>`fn minor_linker_version(&self) -> u8`

- <span id="peimageoptionalheader32-size-of-code"></span>`fn size_of_code(&self) -> u32`

- <span id="peimageoptionalheader32-size-of-initialized-data"></span>`fn size_of_initialized_data(&self) -> u32`

- <span id="peimageoptionalheader32-size-of-uninitialized-data"></span>`fn size_of_uninitialized_data(&self) -> u32`

- <span id="peimageoptionalheader32-address-of-entry-point"></span>`fn address_of_entry_point(&self) -> u32`

- <span id="peimageoptionalheader32-base-of-code"></span>`fn base_of_code(&self) -> u32`

- <span id="peimageoptionalheader32-base-of-data"></span>`fn base_of_data(&self) -> Option<u32>`

- <span id="peimageoptionalheader32-image-base"></span>`fn image_base(&self) -> u64`

- <span id="peimageoptionalheader32-section-alignment"></span>`fn section_alignment(&self) -> u32`

- <span id="peimageoptionalheader32-file-alignment"></span>`fn file_alignment(&self) -> u32`

- <span id="peimageoptionalheader32-major-operating-system-version"></span>`fn major_operating_system_version(&self) -> u16`

- <span id="peimageoptionalheader32-minor-operating-system-version"></span>`fn minor_operating_system_version(&self) -> u16`

- <span id="peimageoptionalheader32-major-image-version"></span>`fn major_image_version(&self) -> u16`

- <span id="peimageoptionalheader32-minor-image-version"></span>`fn minor_image_version(&self) -> u16`

- <span id="peimageoptionalheader32-major-subsystem-version"></span>`fn major_subsystem_version(&self) -> u16`

- <span id="peimageoptionalheader32-minor-subsystem-version"></span>`fn minor_subsystem_version(&self) -> u16`

- <span id="peimageoptionalheader32-win32-version-value"></span>`fn win32_version_value(&self) -> u32`

- <span id="peimageoptionalheader32-size-of-image"></span>`fn size_of_image(&self) -> u32`

- <span id="peimageoptionalheader32-size-of-headers"></span>`fn size_of_headers(&self) -> u32`

- <span id="peimageoptionalheader32-check-sum"></span>`fn check_sum(&self) -> u32`

- <span id="peimageoptionalheader32-subsystem"></span>`fn subsystem(&self) -> u16`

- <span id="peimageoptionalheader32-dll-characteristics"></span>`fn dll_characteristics(&self) -> u16`

- <span id="peimageoptionalheader32-size-of-stack-reserve"></span>`fn size_of_stack_reserve(&self) -> u64`

- <span id="peimageoptionalheader32-size-of-stack-commit"></span>`fn size_of_stack_commit(&self) -> u64`

- <span id="peimageoptionalheader32-size-of-heap-reserve"></span>`fn size_of_heap_reserve(&self) -> u64`

- <span id="peimageoptionalheader32-size-of-heap-commit"></span>`fn size_of_heap_commit(&self) -> u64`

- <span id="peimageoptionalheader32-loader-flags"></span>`fn loader_flags(&self) -> u32`

- <span id="peimageoptionalheader32-number-of-rva-and-sizes"></span>`fn number_of_rva_and_sizes(&self) -> u32`

##### `impl Pod for ImageOptionalHeader32`

### `ImageRomOptionalHeader`

```rust
struct ImageRomOptionalHeader {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_bss: crate::endian::U32<crate::endian::LittleEndian>,
    pub gpr_mask: crate::endian::U32<crate::endian::LittleEndian>,
    pub cpr_mask: [crate::endian::U32<crate::endian::LittleEndian>; 4],
    pub gp_value: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:436-450`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L436-L450)*

#### Trait Implementations

##### `impl Clone for ImageRomOptionalHeader`

- <span id="imageromoptionalheader-clone"></span>`fn clone(&self) -> ImageRomOptionalHeader` — [`ImageRomOptionalHeader`](#imageromoptionalheader)

##### `impl Copy for ImageRomOptionalHeader`

##### `impl Debug for ImageRomOptionalHeader`

- <span id="imageromoptionalheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageRomOptionalHeader`

### `ImageOptionalHeader64`

```rust
struct ImageOptionalHeader64 {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U64<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub win32_version_value: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_headers: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub subsystem: crate::endian::U16<crate::endian::LittleEndian>,
    pub dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub size_of_stack_reserve: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_stack_commit: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_heap_reserve: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_heap_commit: crate::endian::U64<crate::endian::LittleEndian>,
    pub loader_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:454-485`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L454-L485)*

#### Trait Implementations

##### `impl Clone for ImageOptionalHeader64`

- <span id="imageoptionalheader64-clone"></span>`fn clone(&self) -> ImageOptionalHeader64` — [`ImageOptionalHeader64`](#imageoptionalheader64)

##### `impl Copy for ImageOptionalHeader64`

##### `impl Debug for ImageOptionalHeader64`

- <span id="imageoptionalheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageOptionalHeader for pe::ImageOptionalHeader64`

- <span id="peimageoptionalheader64-magic"></span>`fn magic(&self) -> u16`

- <span id="peimageoptionalheader64-major-linker-version"></span>`fn major_linker_version(&self) -> u8`

- <span id="peimageoptionalheader64-minor-linker-version"></span>`fn minor_linker_version(&self) -> u8`

- <span id="peimageoptionalheader64-size-of-code"></span>`fn size_of_code(&self) -> u32`

- <span id="peimageoptionalheader64-size-of-initialized-data"></span>`fn size_of_initialized_data(&self) -> u32`

- <span id="peimageoptionalheader64-size-of-uninitialized-data"></span>`fn size_of_uninitialized_data(&self) -> u32`

- <span id="peimageoptionalheader64-address-of-entry-point"></span>`fn address_of_entry_point(&self) -> u32`

- <span id="peimageoptionalheader64-base-of-code"></span>`fn base_of_code(&self) -> u32`

- <span id="peimageoptionalheader64-base-of-data"></span>`fn base_of_data(&self) -> Option<u32>`

- <span id="peimageoptionalheader64-image-base"></span>`fn image_base(&self) -> u64`

- <span id="peimageoptionalheader64-section-alignment"></span>`fn section_alignment(&self) -> u32`

- <span id="peimageoptionalheader64-file-alignment"></span>`fn file_alignment(&self) -> u32`

- <span id="peimageoptionalheader64-major-operating-system-version"></span>`fn major_operating_system_version(&self) -> u16`

- <span id="peimageoptionalheader64-minor-operating-system-version"></span>`fn minor_operating_system_version(&self) -> u16`

- <span id="peimageoptionalheader64-major-image-version"></span>`fn major_image_version(&self) -> u16`

- <span id="peimageoptionalheader64-minor-image-version"></span>`fn minor_image_version(&self) -> u16`

- <span id="peimageoptionalheader64-major-subsystem-version"></span>`fn major_subsystem_version(&self) -> u16`

- <span id="peimageoptionalheader64-minor-subsystem-version"></span>`fn minor_subsystem_version(&self) -> u16`

- <span id="peimageoptionalheader64-win32-version-value"></span>`fn win32_version_value(&self) -> u32`

- <span id="peimageoptionalheader64-size-of-image"></span>`fn size_of_image(&self) -> u32`

- <span id="peimageoptionalheader64-size-of-headers"></span>`fn size_of_headers(&self) -> u32`

- <span id="peimageoptionalheader64-check-sum"></span>`fn check_sum(&self) -> u32`

- <span id="peimageoptionalheader64-subsystem"></span>`fn subsystem(&self) -> u16`

- <span id="peimageoptionalheader64-dll-characteristics"></span>`fn dll_characteristics(&self) -> u16`

- <span id="peimageoptionalheader64-size-of-stack-reserve"></span>`fn size_of_stack_reserve(&self) -> u64`

- <span id="peimageoptionalheader64-size-of-stack-commit"></span>`fn size_of_stack_commit(&self) -> u64`

- <span id="peimageoptionalheader64-size-of-heap-reserve"></span>`fn size_of_heap_reserve(&self) -> u64`

- <span id="peimageoptionalheader64-size-of-heap-commit"></span>`fn size_of_heap_commit(&self) -> u64`

- <span id="peimageoptionalheader64-loader-flags"></span>`fn loader_flags(&self) -> u32`

- <span id="peimageoptionalheader64-number-of-rva-and-sizes"></span>`fn number_of_rva_and_sizes(&self) -> u32`

##### `impl Pod for ImageOptionalHeader64`

### `ImageNtHeaders64`

```rust
struct ImageNtHeaders64 {
    pub signature: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader64,
}
```

*Defined in [`object-0.37.3/src/pe.rs:493-497`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L493-L497)*

#### Trait Implementations

##### `impl Clone for ImageNtHeaders64`

- <span id="imagentheaders64-clone"></span>`fn clone(&self) -> ImageNtHeaders64` — [`ImageNtHeaders64`](#imagentheaders64)

##### `impl Copy for ImageNtHeaders64`

##### `impl Debug for ImageNtHeaders64`

- <span id="imagentheaders64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageNtHeaders for pe::ImageNtHeaders64`

- <span id="peimagentheaders64-imagentheaders-type-imageoptionalheader"></span>`type ImageOptionalHeader = ImageOptionalHeader64`

- <span id="peimagentheaders64-imagentheaders-type-imagethunkdata"></span>`type ImageThunkData = ImageThunkData64`

- <span id="peimagentheaders64-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="peimagentheaders64-is-valid-optional-magic"></span>`fn is_valid_optional_magic(&self) -> bool`

- <span id="peimagentheaders64-signature"></span>`fn signature(&self) -> u32`

- <span id="peimagentheaders64-file-header"></span>`fn file_header(&self) -> &pe::ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

- <span id="peimagentheaders64-optional-header"></span>`fn optional_header(&self) -> &<Self as >::ImageOptionalHeader` — [`ImageNtHeaders`](../read/pe/index.md#imagentheaders)

##### `impl Pod for ImageNtHeaders64`

### `ImageNtHeaders32`

```rust
struct ImageNtHeaders32 {
    pub signature: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader32,
}
```

*Defined in [`object-0.37.3/src/pe.rs:501-505`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L501-L505)*

#### Trait Implementations

##### `impl Clone for ImageNtHeaders32`

- <span id="imagentheaders32-clone"></span>`fn clone(&self) -> ImageNtHeaders32` — [`ImageNtHeaders32`](#imagentheaders32)

##### `impl Copy for ImageNtHeaders32`

##### `impl Debug for ImageNtHeaders32`

- <span id="imagentheaders32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageNtHeaders for pe::ImageNtHeaders32`

- <span id="peimagentheaders32-imagentheaders-type-imageoptionalheader"></span>`type ImageOptionalHeader = ImageOptionalHeader32`

- <span id="peimagentheaders32-imagentheaders-type-imagethunkdata"></span>`type ImageThunkData = ImageThunkData32`

- <span id="peimagentheaders32-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="peimagentheaders32-is-valid-optional-magic"></span>`fn is_valid_optional_magic(&self) -> bool`

- <span id="peimagentheaders32-signature"></span>`fn signature(&self) -> u32`

- <span id="peimagentheaders32-file-header"></span>`fn file_header(&self) -> &pe::ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

- <span id="peimagentheaders32-optional-header"></span>`fn optional_header(&self) -> &<Self as >::ImageOptionalHeader` — [`ImageNtHeaders`](../read/pe/index.md#imagentheaders)

##### `impl Pod for ImageNtHeaders32`

### `ImageRomHeaders`

```rust
struct ImageRomHeaders {
    pub file_header: ImageFileHeader,
    pub optional_header: ImageRomOptionalHeader,
}
```

*Defined in [`object-0.37.3/src/pe.rs:509-512`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L509-L512)*

#### Trait Implementations

##### `impl Clone for ImageRomHeaders`

- <span id="imageromheaders-clone"></span>`fn clone(&self) -> ImageRomHeaders` — [`ImageRomHeaders`](#imageromheaders)

##### `impl Copy for ImageRomHeaders`

##### `impl Debug for ImageRomHeaders`

- <span id="imageromheaders-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageRomHeaders`

### `Guid`

```rust
struct Guid([u8; 16]);
```

*Defined in [`object-0.37.3/src/pe.rs:604`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L604)*

#### Implementations

- <span id="guid-data1"></span>`fn data1(self) -> U32<LE>` — [`U32`](../index.md#u32), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data2"></span>`fn data2(self) -> U16<LE>` — [`U16`](../index.md#u16), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data3"></span>`fn data3(self) -> U16<LE>` — [`U16`](../index.md#u16), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data4"></span>`fn data4(self) -> [u8; 8]`

#### Trait Implementations

##### `impl Clone for Guid`

- <span id="guid-clone"></span>`fn clone(&self) -> Guid` — [`Guid`](#guid)

##### `impl Copy for Guid`

##### `impl Debug for Guid`

- <span id="guid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Guid`

##### `impl PartialEq for Guid`

- <span id="guid-eq"></span>`fn eq(&self, other: &Guid) -> bool` — [`Guid`](#guid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

### `ClsId`

```rust
struct ClsId([u8; 16]);
```

*Defined in [`object-0.37.3/src/pe.rs:604`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L604)*

#### Implementations

- <span id="guid-data1"></span>`fn data1(self) -> U32<LE>` — [`U32`](../index.md#u32), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data2"></span>`fn data2(self) -> U16<LE>` — [`U16`](../index.md#u16), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data3"></span>`fn data3(self) -> U16<LE>` — [`U16`](../index.md#u16), [`LittleEndian`](../index.md#littleendian)

- <span id="guid-data4"></span>`fn data4(self) -> [u8; 8]`

#### Trait Implementations

##### `impl Clone for Guid`

- <span id="guid-clone"></span>`fn clone(&self) -> Guid` — [`Guid`](#guid)

##### `impl Copy for Guid`

##### `impl Debug for Guid`

- <span id="guid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Guid`

##### `impl PartialEq for Guid`

- <span id="guid-eq"></span>`fn eq(&self, other: &Guid) -> bool` — [`Guid`](#guid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

### `AnonObjectHeader`

```rust
struct AnonObjectHeader {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:633-646`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L633-L646)*

Non-COFF Object file header

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 1 (implies the ClsId field is present)

- **`class_id`**: `ClsId`

  Used to invoke CoCreateInstance

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

#### Trait Implementations

##### `impl Clone for AnonObjectHeader`

- <span id="anonobjectheader-clone"></span>`fn clone(&self) -> AnonObjectHeader` — [`AnonObjectHeader`](#anonobjectheader)

##### `impl Copy for AnonObjectHeader`

##### `impl Debug for AnonObjectHeader`

- <span id="anonobjectheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for AnonObjectHeader`

### `AnonObjectHeaderV2`

```rust
struct AnonObjectHeaderV2 {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:650-669`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L650-L669)*

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 2 (implies the Flags field is present - otherwise V1)

- **`class_id`**: `ClsId`

  Used to invoke CoCreateInstance

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

- **`flags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0x1 -> contains metadata

- **`meta_data_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of CLR metadata

- **`meta_data_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of CLR metadata

#### Trait Implementations

##### `impl Clone for AnonObjectHeaderV2`

- <span id="anonobjectheaderv2-clone"></span>`fn clone(&self) -> AnonObjectHeaderV2` — [`AnonObjectHeaderV2`](#anonobjectheaderv2)

##### `impl Copy for AnonObjectHeaderV2`

##### `impl Debug for AnonObjectHeaderV2`

- <span id="anonobjectheaderv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for AnonObjectHeaderV2`

### `AnonObjectHeaderBigobj`

```rust
struct AnonObjectHeaderBigobj {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:678-705`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L678-L705)*

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 2 (implies the Flags field is present)

- **`machine`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Actual machine - IMAGE_FILE_MACHINE_xxx

- **`class_id`**: `ClsId`

  Must be `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`.

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

- **`flags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0x1 -> contains metadata

- **`meta_data_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of CLR metadata

- **`meta_data_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of CLR metadata

- **`number_of_sections`**: `crate::endian::U32<crate::endian::LittleEndian>`

  extended from WORD

#### Trait Implementations

##### `impl Clone for AnonObjectHeaderBigobj`

- <span id="anonobjectheaderbigobj-clone"></span>`fn clone(&self) -> AnonObjectHeaderBigobj` — [`AnonObjectHeaderBigobj`](#anonobjectheaderbigobj)

##### `impl CoffHeader for pe::AnonObjectHeaderBigobj`

- <span id="peanonobjectheaderbigobj-coffheader-type-imagesymbol"></span>`type ImageSymbol = ImageSymbolEx`

- <span id="peanonobjectheaderbigobj-coffheader-type-imagesymbolbytes"></span>`type ImageSymbolBytes = ImageSymbolExBytes`

- <span id="peanonobjectheaderbigobj-is-type-bigobj"></span>`fn is_type_bigobj() -> bool`

- <span id="peanonobjectheaderbigobj-machine"></span>`fn machine(&self) -> u16`

- <span id="peanonobjectheaderbigobj-number-of-sections"></span>`fn number_of_sections(&self) -> u32`

- <span id="peanonobjectheaderbigobj-pointer-to-symbol-table"></span>`fn pointer_to_symbol_table(&self) -> u32`

- <span id="peanonobjectheaderbigobj-number-of-symbols"></span>`fn number_of_symbols(&self) -> u32`

- <span id="peanonobjectheaderbigobj-characteristics"></span>`fn characteristics(&self) -> u16`

- <span id="peanonobjectheaderbigobj-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>` — [`Result`](../index.md#result)

##### `impl Copy for AnonObjectHeaderBigobj`

##### `impl Debug for AnonObjectHeaderBigobj`

- <span id="anonobjectheaderbigobj-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for AnonObjectHeaderBigobj`

### `ImageSectionHeader`

```rust
struct ImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_relocations: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_linenumbers: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_relocations: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:715-726`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L715-L726)*

#### Implementations

- <span id="peimagesectionheader-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

#### Trait Implementations

##### `impl Clone for ImageSectionHeader`

- <span id="imagesectionheader-clone"></span>`fn clone(&self) -> ImageSectionHeader` — [`ImageSectionHeader`](#imagesectionheader)

##### `impl Copy for ImageSectionHeader`

##### `impl Debug for ImageSectionHeader`

- <span id="imagesectionheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ImageSectionHeader`

- <span id="imagesectionheader-default"></span>`fn default() -> ImageSectionHeader` — [`ImageSectionHeader`](#imagesectionheader)

##### `impl Pod for ImageSectionHeader`

### `ImageSymbol`

```rust
struct ImageSymbol {
    pub name: [u8; 8],
    pub value: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub section_number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub storage_class: u8,
    pub number_of_aux_symbols: u8,
}
```

*Defined in [`object-0.37.3/src/pe.rs:817-825`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L817-L825)*

#### Fields

- **`name`**: `[u8; 8]`

  If first 4 bytes are 0, then second 4 bytes are offset into string table.

#### Trait Implementations

##### `impl Clone for ImageSymbol`

- <span id="imagesymbol-clone"></span>`fn clone(&self) -> ImageSymbol` — [`ImageSymbol`](#imagesymbol)

##### `impl Copy for ImageSymbol`

##### `impl Debug for ImageSymbol`

- <span id="imagesymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageSymbol for pe::ImageSymbol`

- <span id="peimagesymbol-raw-name"></span>`fn raw_name(&self) -> &[u8; 8]`

- <span id="peimagesymbol-value"></span>`fn value(&self) -> u32`

- <span id="peimagesymbol-section-number"></span>`fn section_number(&self) -> i32`

- <span id="peimagesymbol-typ"></span>`fn typ(&self) -> u16`

- <span id="peimagesymbol-storage-class"></span>`fn storage_class(&self) -> u8`

- <span id="peimagesymbol-number-of-aux-symbols"></span>`fn number_of_aux_symbols(&self) -> u8`

##### `impl Pod for ImageSymbol`

### `ImageSymbolBytes`

```rust
struct ImageSymbolBytes([u8; 18]);
```

*Defined in [`object-0.37.3/src/pe.rs:831`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L831)*

#### Trait Implementations

##### `impl Clone for ImageSymbolBytes`

- <span id="imagesymbolbytes-clone"></span>`fn clone(&self) -> ImageSymbolBytes` — [`ImageSymbolBytes`](#imagesymbolbytes)

##### `impl Copy for ImageSymbolBytes`

##### `impl Debug for ImageSymbolBytes`

- <span id="imagesymbolbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageSymbolBytes`

### `ImageSymbolEx`

```rust
struct ImageSymbolEx {
    pub name: [u8; 8],
    pub value: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub section_number: crate::endian::I32Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub storage_class: u8,
    pub number_of_aux_symbols: u8,
}
```

*Defined in [`object-0.37.3/src/pe.rs:836-844`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L836-L844)*

#### Fields

- **`name`**: `[u8; 8]`

  If first 4 bytes are 0, then second 4 bytes are offset into string table.

#### Trait Implementations

##### `impl Clone for ImageSymbolEx`

- <span id="imagesymbolex-clone"></span>`fn clone(&self) -> ImageSymbolEx` — [`ImageSymbolEx`](#imagesymbolex)

##### `impl Copy for ImageSymbolEx`

##### `impl Debug for ImageSymbolEx`

- <span id="imagesymbolex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageSymbol for pe::ImageSymbolEx`

- <span id="peimagesymbolex-raw-name"></span>`fn raw_name(&self) -> &[u8; 8]`

- <span id="peimagesymbolex-value"></span>`fn value(&self) -> u32`

- <span id="peimagesymbolex-section-number"></span>`fn section_number(&self) -> i32`

- <span id="peimagesymbolex-typ"></span>`fn typ(&self) -> u16`

- <span id="peimagesymbolex-storage-class"></span>`fn storage_class(&self) -> u8`

- <span id="peimagesymbolex-number-of-aux-symbols"></span>`fn number_of_aux_symbols(&self) -> u8`

##### `impl Pod for ImageSymbolEx`

### `ImageSymbolExBytes`

```rust
struct ImageSymbolExBytes([u8; 20]);
```

*Defined in [`object-0.37.3/src/pe.rs:850`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L850)*

#### Trait Implementations

##### `impl Clone for ImageSymbolExBytes`

- <span id="imagesymbolexbytes-clone"></span>`fn clone(&self) -> ImageSymbolExBytes` — [`ImageSymbolExBytes`](#imagesymbolexbytes)

##### `impl Copy for ImageSymbolExBytes`

##### `impl Debug for ImageSymbolExBytes`

- <span id="imagesymbolexbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageSymbolExBytes`

### `ImageAuxSymbolTokenDef`

```rust
struct ImageAuxSymbolTokenDef {
    pub aux_type: u8,
    pub reserved1: u8,
    pub symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub reserved2: [u8; 12],
}
```

*Defined in [`object-0.37.3/src/pe.rs:956-964`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L956-L964)*

#### Fields

- **`aux_type`**: `u8`

  IMAGE_AUX_SYMBOL_TYPE

- **`reserved1`**: `u8`

  Must be 0

- **`reserved2`**: `[u8; 12]`

  Must be 0

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolTokenDef`

- <span id="imageauxsymboltokendef-clone"></span>`fn clone(&self) -> ImageAuxSymbolTokenDef` — [`ImageAuxSymbolTokenDef`](#imageauxsymboltokendef)

##### `impl Copy for ImageAuxSymbolTokenDef`

##### `impl Debug for ImageAuxSymbolTokenDef`

- <span id="imageauxsymboltokendef-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolTokenDef`

### `ImageAuxSymbolFunction`

```rust
struct ImageAuxSymbolFunction {
    pub tag_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub total_size: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub pointer_to_linenumber: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub unused: [u8; 2],
}
```

*Defined in [`object-0.37.3/src/pe.rs:972-978`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L972-L978)*

Auxiliary symbol format 1: function definitions.

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolFunction`

- <span id="imageauxsymbolfunction-clone"></span>`fn clone(&self) -> ImageAuxSymbolFunction` — [`ImageAuxSymbolFunction`](#imageauxsymbolfunction)

##### `impl Copy for ImageAuxSymbolFunction`

##### `impl Debug for ImageAuxSymbolFunction`

- <span id="imageauxsymbolfunction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolFunction`

### `ImageAuxSymbolFunctionBeginEnd`

```rust
struct ImageAuxSymbolFunctionBeginEnd {
    pub unused1: [u8; 4],
    pub linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub unused2: [u8; 6],
    pub pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub unused3: [u8; 2],
}
```

*Defined in [`object-0.37.3/src/pe.rs:984-991`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L984-L991)*

Auxiliary symbol format 2: .bf and .ef symbols.

#### Fields

- **`linenumber`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  declaration line number

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolFunctionBeginEnd`

- <span id="imageauxsymbolfunctionbeginend-clone"></span>`fn clone(&self) -> ImageAuxSymbolFunctionBeginEnd` — [`ImageAuxSymbolFunctionBeginEnd`](#imageauxsymbolfunctionbeginend)

##### `impl Copy for ImageAuxSymbolFunctionBeginEnd`

##### `impl Debug for ImageAuxSymbolFunctionBeginEnd`

- <span id="imageauxsymbolfunctionbeginend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolFunctionBeginEnd`

### `ImageAuxSymbolWeak`

```rust
struct ImageAuxSymbolWeak {
    pub weak_default_sym_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub weak_search_type: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:999-1003`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L999-L1003)*

Auxiliary symbol format 3: weak externals.

Used for both `ImageSymbol` and `ImageSymbolEx` (both with padding).

#### Fields

- **`weak_default_sym_index`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  the weak extern default symbol index

#### Implementations

- <span id="peimageauxsymbolweak-default-symbol"></span>`fn default_symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolWeak`

- <span id="imageauxsymbolweak-clone"></span>`fn clone(&self) -> ImageAuxSymbolWeak` — [`ImageAuxSymbolWeak`](#imageauxsymbolweak)

##### `impl Copy for ImageAuxSymbolWeak`

##### `impl Debug for ImageAuxSymbolWeak`

- <span id="imageauxsymbolweak-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolWeak`

### `ImageAuxSymbolSection`

```rust
struct ImageAuxSymbolSection {
    pub length: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub number_of_relocations: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub selection: u8,
    pub reserved: u8,
    pub high_number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1011-1027`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1011-L1027)*

Auxiliary symbol format 5: sections.

Used for both `ImageSymbol` and `ImageSymbolEx` (with padding).

#### Fields

- **`length`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  section length

- **`number_of_relocations`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  number of relocation entries

- **`number_of_linenumbers`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  number of line numbers

- **`check_sum`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  checksum for communal

- **`number`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  section number to associate with

- **`selection`**: `u8`

  communal selection type

- **`high_number`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  high bits of the section number

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolSection`

- <span id="imageauxsymbolsection-clone"></span>`fn clone(&self) -> ImageAuxSymbolSection` — [`ImageAuxSymbolSection`](#imageauxsymbolsection)

##### `impl Copy for ImageAuxSymbolSection`

##### `impl Debug for ImageAuxSymbolSection`

- <span id="imageauxsymbolsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolSection`

### `ImageAuxSymbolCrc`

```rust
struct ImageAuxSymbolCrc {
    pub crc: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1033-1035`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1033-L1035)*

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolCrc`

- <span id="imageauxsymbolcrc-clone"></span>`fn clone(&self) -> ImageAuxSymbolCrc` — [`ImageAuxSymbolCrc`](#imageauxsymbolcrc)

##### `impl Copy for ImageAuxSymbolCrc`

##### `impl Debug for ImageAuxSymbolCrc`

- <span id="imageauxsymbolcrc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAuxSymbolCrc`

### `ImageRelocation`

```rust
struct ImageRelocation {
    pub virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1061-1066`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1061-L1066)*

#### Fields

- **`virtual_address`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  Also `RelocCount` when IMAGE_SCN_LNK_NRELOC_OVFL is set

#### Implementations

- <span id="peimagerelocation-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

#### Trait Implementations

##### `impl Clone for ImageRelocation`

- <span id="imagerelocation-clone"></span>`fn clone(&self) -> ImageRelocation` — [`ImageRelocation`](#imagerelocation)

##### `impl Copy for ImageRelocation`

##### `impl Debug for ImageRelocation`

- <span id="imagerelocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageRelocation`

### `ImageLinenumber`

```rust
struct ImageLinenumber {
    pub symbol_table_index_or_virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1721-1727`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1721-L1727)*

#### Fields

- **`symbol_table_index_or_virtual_address`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  Symbol table index of function name if Linenumber is 0.
  Otherwise virtual address of line number.

- **`linenumber`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  Line number.

#### Trait Implementations

##### `impl Clone for ImageLinenumber`

- <span id="imagelinenumber-clone"></span>`fn clone(&self) -> ImageLinenumber` — [`ImageLinenumber`](#imagelinenumber)

##### `impl Copy for ImageLinenumber`

##### `impl Debug for ImageLinenumber`

- <span id="imagelinenumber-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageLinenumber`

### `ImageBaseRelocation`

```rust
struct ImageBaseRelocation {
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_block: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1735-1739`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1735-L1739)*

#### Trait Implementations

##### `impl Clone for ImageBaseRelocation`

- <span id="imagebaserelocation-clone"></span>`fn clone(&self) -> ImageBaseRelocation` — [`ImageBaseRelocation`](#imagebaserelocation)

##### `impl Copy for ImageBaseRelocation`

##### `impl Debug for ImageBaseRelocation`

- <span id="imagebaserelocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageBaseRelocation`

### `ImageArchiveMemberHeader`

```rust
struct ImageArchiveMemberHeader {
    pub name: [u8; 16],
    pub date: [u8; 12],
    pub user_id: [u8; 6],
    pub group_id: [u8; 6],
    pub mode: [u8; 8],
    pub size: [u8; 10],
    pub end_header: [u8; 2],
}
```

*Defined in [`object-0.37.3/src/pe.rs:1787-1802`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1787-L1802)*

#### Fields

- **`name`**: `[u8; 16]`

  File member name - `/' terminated.

- **`date`**: `[u8; 12]`

  File member date - decimal.

- **`user_id`**: `[u8; 6]`

  File member user id - decimal.

- **`group_id`**: `[u8; 6]`

  File member group id - decimal.

- **`mode`**: `[u8; 8]`

  File member mode - octal.

- **`size`**: `[u8; 10]`

  File member size - decimal.

- **`end_header`**: `[u8; 2]`

  String to end header.

#### Trait Implementations

##### `impl Clone for ImageArchiveMemberHeader`

- <span id="imagearchivememberheader-clone"></span>`fn clone(&self) -> ImageArchiveMemberHeader` — [`ImageArchiveMemberHeader`](#imagearchivememberheader)

##### `impl Copy for ImageArchiveMemberHeader`

##### `impl Debug for ImageArchiveMemberHeader`

- <span id="imagearchivememberheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageArchiveMemberHeader`

### `ImageExportDirectory`

```rust
struct ImageExportDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub name: crate::endian::U32<crate::endian::LittleEndian>,
    pub base: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_functions: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_names: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_functions: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_names: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_name_ordinals: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1816-1831`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1816-L1831)*

#### Fields

- **`address_of_functions`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

- **`address_of_names`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

- **`address_of_name_ordinals`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

#### Trait Implementations

##### `impl Clone for ImageExportDirectory`

- <span id="imageexportdirectory-clone"></span>`fn clone(&self) -> ImageExportDirectory` — [`ImageExportDirectory`](#imageexportdirectory)

##### `impl Copy for ImageExportDirectory`

##### `impl Debug for ImageExportDirectory`

- <span id="imageexportdirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageExportDirectory`

### `ImageImportByName`

```rust
struct ImageImportByName {
    pub hint: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1839-1842`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1839-L1842)*

#### Trait Implementations

##### `impl Clone for ImageImportByName`

- <span id="imageimportbyname-clone"></span>`fn clone(&self) -> ImageImportByName` — [`ImageImportByName`](#imageimportbyname)

##### `impl Copy for ImageImportByName`

##### `impl Debug for ImageImportByName`

- <span id="imageimportbyname-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageImportByName`

### `ImageThunkData64`

```rust
struct ImageThunkData64(crate::endian::U64<crate::endian::LittleEndian>);
```

*Defined in [`object-0.37.3/src/pe.rs:1846`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1846)*

#### Trait Implementations

##### `impl Clone for ImageThunkData64`

- <span id="imagethunkdata64-clone"></span>`fn clone(&self) -> ImageThunkData64` — [`ImageThunkData64`](#imagethunkdata64)

##### `impl Copy for ImageThunkData64`

##### `impl Debug for ImageThunkData64`

- <span id="imagethunkdata64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageThunkData for pe::ImageThunkData64`

- <span id="peimagethunkdata64-raw"></span>`fn raw(self) -> u64`

- <span id="peimagethunkdata64-is-ordinal"></span>`fn is_ordinal(self) -> bool`

- <span id="peimagethunkdata64-ordinal"></span>`fn ordinal(self) -> u16`

- <span id="peimagethunkdata64-address"></span>`fn address(self) -> u32`

##### `impl Pod for ImageThunkData64`

### `ImageThunkData32`

```rust
struct ImageThunkData32(crate::endian::U32<crate::endian::LittleEndian>);
```

*Defined in [`object-0.37.3/src/pe.rs:1861`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1861)*

#### Trait Implementations

##### `impl Clone for ImageThunkData32`

- <span id="imagethunkdata32-clone"></span>`fn clone(&self) -> ImageThunkData32` — [`ImageThunkData32`](#imagethunkdata32)

##### `impl Copy for ImageThunkData32`

##### `impl Debug for ImageThunkData32`

- <span id="imagethunkdata32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ImageThunkData for pe::ImageThunkData32`

- <span id="peimagethunkdata32-raw"></span>`fn raw(self) -> u64`

- <span id="peimagethunkdata32-is-ordinal"></span>`fn is_ordinal(self) -> bool`

- <span id="peimagethunkdata32-ordinal"></span>`fn ordinal(self) -> u16`

- <span id="peimagethunkdata32-address"></span>`fn address(self) -> u32`

##### `impl Pod for ImageThunkData32`

### `ImageTlsDirectory64`

```rust
struct ImageTlsDirectory64 {
    pub start_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub address_of_index: crate::endian::U64<crate::endian::LittleEndian>,
    pub address_of_call_backs: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1892-1901`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1892-L1901)*

#### Fields

- **`address_of_index`**: `crate::endian::U64<crate::endian::LittleEndian>`

  PDWORD

- **`address_of_call_backs`**: `crate::endian::U64<crate::endian::LittleEndian>`

  PIMAGE_TLS_CALLBACK *;

#### Trait Implementations

##### `impl Clone for ImageTlsDirectory64`

- <span id="imagetlsdirectory64-clone"></span>`fn clone(&self) -> ImageTlsDirectory64` — [`ImageTlsDirectory64`](#imagetlsdirectory64)

##### `impl Copy for ImageTlsDirectory64`

##### `impl Debug for ImageTlsDirectory64`

- <span id="imagetlsdirectory64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageTlsDirectory64`

### `ImageTlsDirectory32`

```rust
struct ImageTlsDirectory32 {
    pub start_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_index: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_call_backs: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1905-1914`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1905-L1914)*

#### Fields

- **`address_of_index`**: `crate::endian::U32<crate::endian::LittleEndian>`

  PDWORD

- **`address_of_call_backs`**: `crate::endian::U32<crate::endian::LittleEndian>`

  PIMAGE_TLS_CALLBACK *

#### Trait Implementations

##### `impl Clone for ImageTlsDirectory32`

- <span id="imagetlsdirectory32-clone"></span>`fn clone(&self) -> ImageTlsDirectory32` — [`ImageTlsDirectory32`](#imagetlsdirectory32)

##### `impl Copy for ImageTlsDirectory32`

##### `impl Debug for ImageTlsDirectory32`

- <span id="imagetlsdirectory32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageTlsDirectory32`

### `ImageImportDescriptor`

```rust
struct ImageImportDescriptor {
    pub original_first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub forwarder_chain: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub name: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1918-1932`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1918-L1932)*

#### Fields

- **`original_first_thunk`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  RVA to original unbound IAT (`ImageThunkData32`/`ImageThunkData64`)
  0 for terminating null import descriptor

- **`time_date_stamp`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  0 if not bound,
  -1 if bound, and real date\time stamp
      in IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT (new BIND)
  O.W. date/time stamp of DLL bound to (Old BIND)

- **`forwarder_chain`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  -1 if no forwarders

- **`first_thunk`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  RVA to IAT (if bound this IAT has actual addresses)

#### Implementations

- <span id="imageimportdescriptor-is-null"></span>`fn is_null(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ImageImportDescriptor`

- <span id="imageimportdescriptor-clone"></span>`fn clone(&self) -> ImageImportDescriptor` — [`ImageImportDescriptor`](#imageimportdescriptor)

##### `impl Copy for ImageImportDescriptor`

##### `impl Debug for ImageImportDescriptor`

- <span id="imageimportdescriptor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageImportDescriptor`

### `ImageBoundImportDescriptor`

```rust
struct ImageBoundImportDescriptor {
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_module_name: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_module_forwarder_refs: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1952-1957`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1952-L1957)*

#### Trait Implementations

##### `impl Clone for ImageBoundImportDescriptor`

- <span id="imageboundimportdescriptor-clone"></span>`fn clone(&self) -> ImageBoundImportDescriptor` — [`ImageBoundImportDescriptor`](#imageboundimportdescriptor)

##### `impl Copy for ImageBoundImportDescriptor`

##### `impl Debug for ImageBoundImportDescriptor`

- <span id="imageboundimportdescriptor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageBoundImportDescriptor`

### `ImageBoundForwarderRef`

```rust
struct ImageBoundForwarderRef {
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_module_name: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1961-1965`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1961-L1965)*

#### Trait Implementations

##### `impl Clone for ImageBoundForwarderRef`

- <span id="imageboundforwarderref-clone"></span>`fn clone(&self) -> ImageBoundForwarderRef` — [`ImageBoundForwarderRef`](#imageboundforwarderref)

##### `impl Copy for ImageBoundForwarderRef`

##### `impl Debug for ImageBoundForwarderRef`

- <span id="imageboundforwarderref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageBoundForwarderRef`

### `ImageDelayloadDescriptor`

```rust
struct ImageDelayloadDescriptor {
    pub attributes: crate::endian::U32<crate::endian::LittleEndian>,
    pub dll_name_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub module_handle_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_name_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub bound_import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub unload_information_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:1969-1986`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1969-L1986)*

#### Fields

- **`dll_name_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the name of the target library (NULL-terminate ASCII string)

- **`module_handle_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the HMODULE caching location (PHMODULE)

- **`import_address_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the start of the IAT (PIMAGE_THUNK_DATA)

- **`import_name_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the start of the name table (PIMAGE_THUNK_DATA::AddressOfData)

- **`bound_import_address_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to an optional bound IAT

- **`unload_information_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to an optional unload info table

- **`time_date_stamp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0 if not bound, otherwise, date/time of the target DLL

#### Implementations

- <span id="imagedelayloaddescriptor-is-null"></span>`fn is_null(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ImageDelayloadDescriptor`

- <span id="imagedelayloaddescriptor-clone"></span>`fn clone(&self) -> ImageDelayloadDescriptor` — [`ImageDelayloadDescriptor`](#imagedelayloaddescriptor)

##### `impl Copy for ImageDelayloadDescriptor`

##### `impl Debug for ImageDelayloadDescriptor`

- <span id="imagedelayloaddescriptor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDelayloadDescriptor`

### `ImageResourceDirectory`

```rust
struct ImageResourceDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_named_entries: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_id_entries: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2026-2033`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2026-L2033)*

#### Trait Implementations

##### `impl Clone for ImageResourceDirectory`

- <span id="imageresourcedirectory-clone"></span>`fn clone(&self) -> ImageResourceDirectory` — [`ImageResourceDirectory`](#imageresourcedirectory)

##### `impl Copy for ImageResourceDirectory`

##### `impl Debug for ImageResourceDirectory`

- <span id="imageresourcedirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageResourceDirectory`

### `ImageResourceDirectoryEntry`

```rust
struct ImageResourceDirectoryEntry {
    pub name_or_id: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_to_data_or_directory: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2054-2057`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2054-L2057)*

#### Implementations

- <span id="peimageresourcedirectoryentry-has-name"></span>`fn has_name(&self) -> bool`

- <span id="peimageresourcedirectoryentry-name"></span>`fn name(&self) -> ResourceName` — [`ResourceName`](../read/pe/index.md#resourcename)

- <span id="peimageresourcedirectoryentry-id"></span>`fn id(&self) -> u16`

- <span id="peimageresourcedirectoryentry-name-or-id"></span>`fn name_or_id(&self) -> ResourceNameOrId` — [`ResourceNameOrId`](../read/pe/index.md#resourcenameorid)

- <span id="peimageresourcedirectoryentry-is-table"></span>`fn is_table(&self) -> bool`

- <span id="peimageresourcedirectoryentry-data-offset"></span>`fn data_offset(&self) -> u32`

- <span id="peimageresourcedirectoryentry-data"></span>`fn data<'data>(&self, section: ResourceDirectory<'data>) -> Result<ResourceDirectoryEntryData<'data>>` — [`ResourceDirectory`](../read/pe/index.md#resourcedirectory), [`Result`](../index.md#result), [`ResourceDirectoryEntryData`](../read/pe/index.md#resourcedirectoryentrydata)

#### Trait Implementations

##### `impl Clone for ImageResourceDirectoryEntry`

- <span id="imageresourcedirectoryentry-clone"></span>`fn clone(&self) -> ImageResourceDirectoryEntry` — [`ImageResourceDirectoryEntry`](#imageresourcedirectoryentry)

##### `impl Copy for ImageResourceDirectoryEntry`

##### `impl Debug for ImageResourceDirectoryEntry`

- <span id="imageresourcedirectoryentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageResourceDirectoryEntry`

### `ImageResourceDirectoryString`

```rust
struct ImageResourceDirectoryString {
    pub length: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2070-2073`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2070-L2073)*

#### Trait Implementations

##### `impl Clone for ImageResourceDirectoryString`

- <span id="imageresourcedirectorystring-clone"></span>`fn clone(&self) -> ImageResourceDirectoryString` — [`ImageResourceDirectoryString`](#imageresourcedirectorystring)

##### `impl Copy for ImageResourceDirectoryString`

##### `impl Debug for ImageResourceDirectoryString`

- <span id="imageresourcedirectorystring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageResourceDirectoryString`

### `ImageResourceDirStringU`

```rust
struct ImageResourceDirStringU {
    pub length: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2077-2080`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2077-L2080)*

#### Trait Implementations

##### `impl Clone for ImageResourceDirStringU`

- <span id="imageresourcedirstringu-clone"></span>`fn clone(&self) -> ImageResourceDirStringU` — [`ImageResourceDirStringU`](#imageresourcedirstringu)

##### `impl Copy for ImageResourceDirStringU`

##### `impl Debug for ImageResourceDirStringU`

- <span id="imageresourcedirstringu-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageResourceDirStringU`

### `ImageResourceDataEntry`

```rust
struct ImageResourceDataEntry {
    pub offset_to_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_page: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2093-2099`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2093-L2099)*

#### Fields

- **`offset_to_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA of the data.

#### Trait Implementations

##### `impl Clone for ImageResourceDataEntry`

- <span id="imageresourcedataentry-clone"></span>`fn clone(&self) -> ImageResourceDataEntry` — [`ImageResourceDataEntry`](#imageresourcedataentry)

##### `impl Copy for ImageResourceDataEntry`

##### `impl Debug for ImageResourceDataEntry`

- <span id="imageresourcedataentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageResourceDataEntry`

### `ImageLoadConfigCodeIntegrity`

```rust
struct ImageLoadConfigCodeIntegrity {
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub catalog: crate::endian::U16<crate::endian::LittleEndian>,
    pub catalog_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2152-2160`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2152-L2160)*

#### Fields

- **`flags`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Flags to indicate if CI information is available, etc.

- **`catalog`**: `crate::endian::U16<crate::endian::LittleEndian>`

  0xFFFF means not available

- **`reserved`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Additional bitmask to be defined later

#### Trait Implementations

##### `impl Clone for ImageLoadConfigCodeIntegrity`

- <span id="imageloadconfigcodeintegrity-clone"></span>`fn clone(&self) -> ImageLoadConfigCodeIntegrity` — [`ImageLoadConfigCodeIntegrity`](#imageloadconfigcodeintegrity)

##### `impl Copy for ImageLoadConfigCodeIntegrity`

##### `impl Debug for ImageLoadConfigCodeIntegrity`

- <span id="imageloadconfigcodeintegrity-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageLoadConfigCodeIntegrity`

### `ImageDynamicRelocationTable`

```rust
struct ImageDynamicRelocationTable {
    pub version: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2168-2172`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2168-L2172)*

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocationTable`

- <span id="imagedynamicrelocationtable-clone"></span>`fn clone(&self) -> ImageDynamicRelocationTable` — [`ImageDynamicRelocationTable`](#imagedynamicrelocationtable)

##### `impl Copy for ImageDynamicRelocationTable`

##### `impl Debug for ImageDynamicRelocationTable`

- <span id="imagedynamicrelocationtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDynamicRelocationTable`

### `ImageDynamicRelocation32`

```rust
struct ImageDynamicRelocation32 {
    pub symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2180-2184`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2180-L2184)*

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation32`

- <span id="imagedynamicrelocation32-clone"></span>`fn clone(&self) -> ImageDynamicRelocation32` — [`ImageDynamicRelocation32`](#imagedynamicrelocation32)

##### `impl Copy for ImageDynamicRelocation32`

##### `impl Debug for ImageDynamicRelocation32`

- <span id="imagedynamicrelocation32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDynamicRelocation32`

### `ImageDynamicRelocation64`

```rust
struct ImageDynamicRelocation64 {
    pub symbol: crate::endian::U64<crate::endian::LittleEndian>,
    pub base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2188-2192`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2188-L2192)*

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation64`

- <span id="imagedynamicrelocation64-clone"></span>`fn clone(&self) -> ImageDynamicRelocation64` — [`ImageDynamicRelocation64`](#imagedynamicrelocation64)

##### `impl Copy for ImageDynamicRelocation64`

##### `impl Debug for ImageDynamicRelocation64`

- <span id="imagedynamicrelocation64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDynamicRelocation64`

### `ImageDynamicRelocation32V2`

```rust
struct ImageDynamicRelocation32V2 {
    pub header_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol_group: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2196-2204`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2196-L2204)*

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation32V2`

- <span id="imagedynamicrelocation32v2-clone"></span>`fn clone(&self) -> ImageDynamicRelocation32V2` — [`ImageDynamicRelocation32V2`](#imagedynamicrelocation32v2)

##### `impl Copy for ImageDynamicRelocation32V2`

##### `impl Debug for ImageDynamicRelocation32V2`

- <span id="imagedynamicrelocation32v2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDynamicRelocation32V2`

### `ImageDynamicRelocation64V2`

```rust
struct ImageDynamicRelocation64V2 {
    pub header_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol: crate::endian::U64<crate::endian::LittleEndian>,
    pub symbol_group: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2208-2216`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2208-L2216)*

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation64V2`

- <span id="imagedynamicrelocation64v2-clone"></span>`fn clone(&self) -> ImageDynamicRelocation64V2` — [`ImageDynamicRelocation64V2`](#imagedynamicrelocation64v2)

##### `impl Copy for ImageDynamicRelocation64V2`

##### `impl Debug for ImageDynamicRelocation64V2`

- <span id="imagedynamicrelocation64v2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDynamicRelocation64V2`

### `ImagePrologueDynamicRelocationHeader`

```rust
struct ImagePrologueDynamicRelocationHeader {
    pub prologue_byte_count: u8,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2231-2234`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2231-L2234)*

#### Trait Implementations

##### `impl Clone for ImagePrologueDynamicRelocationHeader`

- <span id="imageprologuedynamicrelocationheader-clone"></span>`fn clone(&self) -> ImagePrologueDynamicRelocationHeader` — [`ImagePrologueDynamicRelocationHeader`](#imageprologuedynamicrelocationheader)

##### `impl Copy for ImagePrologueDynamicRelocationHeader`

##### `impl Debug for ImagePrologueDynamicRelocationHeader`

- <span id="imageprologuedynamicrelocationheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImagePrologueDynamicRelocationHeader`

### `ImageEpilogueDynamicRelocationHeader`

```rust
struct ImageEpilogueDynamicRelocationHeader {
    pub epilogue_count: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub epilogue_byte_count: u8,
    pub branch_descriptor_element_size: u8,
    pub branch_descriptor_count: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2239-2246`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2239-L2246)*

#### Trait Implementations

##### `impl Clone for ImageEpilogueDynamicRelocationHeader`

- <span id="imageepiloguedynamicrelocationheader-clone"></span>`fn clone(&self) -> ImageEpilogueDynamicRelocationHeader` — [`ImageEpilogueDynamicRelocationHeader`](#imageepiloguedynamicrelocationheader)

##### `impl Copy for ImageEpilogueDynamicRelocationHeader`

##### `impl Debug for ImageEpilogueDynamicRelocationHeader`

- <span id="imageepiloguedynamicrelocationheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageEpilogueDynamicRelocationHeader`

### `ImageLoadConfigDirectory32`

```rust
struct ImageLoadConfigDirectory32 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>,
    pub global_flags_set: crate::endian::U32<crate::endian::LittleEndian>,
    pub critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_free_block_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_total_free_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub lock_prefix_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub maximum_allocation_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub virtual_memory_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub process_affinity_mask: crate::endian::U32<crate::endian::LittleEndian>,
    pub csd_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub edit_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_cookie: crate::endian::U32<crate::endian::LittleEndian>,
    pub sehandler_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub sehandler_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_check_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_dispatch_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_function_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_function_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity: ImageLoadConfigCodeIntegrity,
    pub guard_address_taken_iat_entry_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_address_taken_iat_entry_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_long_jump_target_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_long_jump_target_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub chpe_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved2: crate::endian::U16<crate::endian::LittleEndian>,
    pub guard_rf_verify_stack_pointer_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved3: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_configuration_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub volatile_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2285-2343`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2285-L2343)*

#### Fields

- **`lock_prefix_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`edit_list`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`security_cookie`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`sehandler_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_check_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_dispatch_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_function_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_address_taken_iat_entry_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_long_jump_target_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`dynamic_value_reloc_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_verify_stack_pointer_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`enclave_configuration_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`volatile_metadata_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

#### Trait Implementations

##### `impl Clone for ImageLoadConfigDirectory32`

- <span id="imageloadconfigdirectory32-clone"></span>`fn clone(&self) -> ImageLoadConfigDirectory32` — [`ImageLoadConfigDirectory32`](#imageloadconfigdirectory32)

##### `impl Copy for ImageLoadConfigDirectory32`

##### `impl Debug for ImageLoadConfigDirectory32`

- <span id="imageloadconfigdirectory32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageLoadConfigDirectory32`

### `ImageLoadConfigDirectory64`

```rust
struct ImageLoadConfigDirectory64 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>,
    pub global_flags_set: crate::endian::U32<crate::endian::LittleEndian>,
    pub critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_free_block_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub de_commit_total_free_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub lock_prefix_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub maximum_allocation_size: crate::endian::U64<crate::endian::LittleEndian>,
    pub virtual_memory_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub process_affinity_mask: crate::endian::U64<crate::endian::LittleEndian>,
    pub process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub csd_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub edit_list: crate::endian::U64<crate::endian::LittleEndian>,
    pub security_cookie: crate::endian::U64<crate::endian::LittleEndian>,
    pub sehandler_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub sehandler_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_check_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_dispatch_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_function_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_function_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity: ImageLoadConfigCodeIntegrity,
    pub guard_address_taken_iat_entry_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_address_taken_iat_entry_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_long_jump_target_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_long_jump_target_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub chpe_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved2: crate::endian::U16<crate::endian::LittleEndian>,
    pub guard_rf_verify_stack_pointer_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved3: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_configuration_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub volatile_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2347-2406`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2347-L2406)*

#### Fields

- **`lock_prefix_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`edit_list`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`security_cookie`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`sehandler_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_check_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_dispatch_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_function_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_address_taken_iat_entry_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_long_jump_target_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`dynamic_value_reloc_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`chpe_metadata_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_verify_stack_pointer_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`enclave_configuration_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`volatile_metadata_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

#### Trait Implementations

##### `impl Clone for ImageLoadConfigDirectory64`

- <span id="imageloadconfigdirectory64-clone"></span>`fn clone(&self) -> ImageLoadConfigDirectory64` — [`ImageLoadConfigDirectory64`](#imageloadconfigdirectory64)

##### `impl Copy for ImageLoadConfigDirectory64`

##### `impl Debug for ImageLoadConfigDirectory64`

- <span id="imageloadconfigdirectory64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageLoadConfigDirectory64`

### `ImageHotPatchInfo`

```rust
struct ImageHotPatchInfo {
    pub version: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub sequence_number: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_image_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_image_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub buffer_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub extra_patch_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2410-2420`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2410-L2420)*

#### Fields

- **`buffer_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 2 and later

- **`extra_patch_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 3 and later

#### Trait Implementations

##### `impl Clone for ImageHotPatchInfo`

- <span id="imagehotpatchinfo-clone"></span>`fn clone(&self) -> ImageHotPatchInfo` — [`ImageHotPatchInfo`](#imagehotpatchinfo)

##### `impl Copy for ImageHotPatchInfo`

##### `impl Debug for ImageHotPatchInfo`

- <span id="imagehotpatchinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageHotPatchInfo`

### `ImageHotPatchBase`

```rust
struct ImageHotPatchBase {
    pub sequence_number: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub original_time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub original_check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity_info: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub patch_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub buffer_offset: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2424-2434`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2424-L2434)*

#### Fields

- **`buffer_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 2 and later

#### Trait Implementations

##### `impl Clone for ImageHotPatchBase`

- <span id="imagehotpatchbase-clone"></span>`fn clone(&self) -> ImageHotPatchBase` — [`ImageHotPatchBase`](#imagehotpatchbase)

##### `impl Copy for ImageHotPatchBase`

##### `impl Debug for ImageHotPatchBase`

- <span id="imagehotpatchbase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageHotPatchBase`

### `ImageHotPatchHashes`

```rust
struct ImageHotPatchHashes {
    pub sha256: [u8; 32],
    pub sha1: [u8; 20],
}
```

*Defined in [`object-0.37.3/src/pe.rs:2438-2441`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2438-L2441)*

#### Trait Implementations

##### `impl Clone for ImageHotPatchHashes`

- <span id="imagehotpatchhashes-clone"></span>`fn clone(&self) -> ImageHotPatchHashes` — [`ImageHotPatchHashes`](#imagehotpatchhashes)

##### `impl Copy for ImageHotPatchHashes`

##### `impl Debug for ImageHotPatchHashes`

- <span id="imagehotpatchhashes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageHotPatchHashes`

### `ImageArmRuntimeFunctionEntry`

```rust
struct ImageArmRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2530-2533`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2530-L2533)*

#### Trait Implementations

##### `impl Clone for ImageArmRuntimeFunctionEntry`

- <span id="imagearmruntimefunctionentry-clone"></span>`fn clone(&self) -> ImageArmRuntimeFunctionEntry` — [`ImageArmRuntimeFunctionEntry`](#imagearmruntimefunctionentry)

##### `impl Copy for ImageArmRuntimeFunctionEntry`

##### `impl Debug for ImageArmRuntimeFunctionEntry`

- <span id="imagearmruntimefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageArmRuntimeFunctionEntry`

### `ImageArm64RuntimeFunctionEntry`

```rust
struct ImageArm64RuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2537-2540`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2537-L2540)*

#### Trait Implementations

##### `impl Clone for ImageArm64RuntimeFunctionEntry`

- <span id="imagearm64runtimefunctionentry-clone"></span>`fn clone(&self) -> ImageArm64RuntimeFunctionEntry` — [`ImageArm64RuntimeFunctionEntry`](#imagearm64runtimefunctionentry)

##### `impl Copy for ImageArm64RuntimeFunctionEntry`

##### `impl Debug for ImageArm64RuntimeFunctionEntry`

- <span id="imagearm64runtimefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageArm64RuntimeFunctionEntry`

### `ImageAlpha64RuntimeFunctionEntry`

```rust
struct ImageAlpha64RuntimeFunctionEntry {
    pub begin_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub exception_handler: crate::endian::U64<crate::endian::LittleEndian>,
    pub handler_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub prolog_end_address: crate::endian::U64<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2544-2550`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2544-L2550)*

#### Trait Implementations

##### `impl Clone for ImageAlpha64RuntimeFunctionEntry`

- <span id="imagealpha64runtimefunctionentry-clone"></span>`fn clone(&self) -> ImageAlpha64RuntimeFunctionEntry` — [`ImageAlpha64RuntimeFunctionEntry`](#imagealpha64runtimefunctionentry)

##### `impl Copy for ImageAlpha64RuntimeFunctionEntry`

##### `impl Debug for ImageAlpha64RuntimeFunctionEntry`

- <span id="imagealpha64runtimefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAlpha64RuntimeFunctionEntry`

### `ImageAlphaRuntimeFunctionEntry`

```rust
struct ImageAlphaRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub exception_handler: crate::endian::U32<crate::endian::LittleEndian>,
    pub handler_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub prolog_end_address: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2554-2560`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2554-L2560)*

#### Trait Implementations

##### `impl Clone for ImageAlphaRuntimeFunctionEntry`

- <span id="imagealpharuntimefunctionentry-clone"></span>`fn clone(&self) -> ImageAlphaRuntimeFunctionEntry` — [`ImageAlphaRuntimeFunctionEntry`](#imagealpharuntimefunctionentry)

##### `impl Copy for ImageAlphaRuntimeFunctionEntry`

##### `impl Debug for ImageAlphaRuntimeFunctionEntry`

- <span id="imagealpharuntimefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageAlphaRuntimeFunctionEntry`

### `ImageRuntimeFunctionEntry`

```rust
struct ImageRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_info_address_or_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2564-2568`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2564-L2568)*

#### Trait Implementations

##### `impl Clone for ImageRuntimeFunctionEntry`

- <span id="imageruntimefunctionentry-clone"></span>`fn clone(&self) -> ImageRuntimeFunctionEntry` — [`ImageRuntimeFunctionEntry`](#imageruntimefunctionentry)

##### `impl Copy for ImageRuntimeFunctionEntry`

##### `impl Debug for ImageRuntimeFunctionEntry`

- <span id="imageruntimefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageRuntimeFunctionEntry`

### `ImageEnclaveConfig32`

```rust
struct ImageEnclaveConfig32 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub policy_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_imports: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_entry_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub image_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_threads: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2579-2593`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2579-L2593)*

#### Trait Implementations

##### `impl Clone for ImageEnclaveConfig32`

- <span id="imageenclaveconfig32-clone"></span>`fn clone(&self) -> ImageEnclaveConfig32` — [`ImageEnclaveConfig32`](#imageenclaveconfig32)

##### `impl Copy for ImageEnclaveConfig32`

##### `impl Debug for ImageEnclaveConfig32`

- <span id="imageenclaveconfig32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageEnclaveConfig32`

### `ImageEnclaveConfig64`

```rust
struct ImageEnclaveConfig64 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub policy_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_imports: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_entry_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub image_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_size: crate::endian::U64<crate::endian::LittleEndian>,
    pub number_of_threads: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2597-2611`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2597-L2611)*

#### Trait Implementations

##### `impl Clone for ImageEnclaveConfig64`

- <span id="imageenclaveconfig64-clone"></span>`fn clone(&self) -> ImageEnclaveConfig64` — [`ImageEnclaveConfig64`](#imageenclaveconfig64)

##### `impl Copy for ImageEnclaveConfig64`

##### `impl Debug for ImageEnclaveConfig64`

- <span id="imageenclaveconfig64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageEnclaveConfig64`

### `ImageEnclaveImport`

```rust
struct ImageEnclaveImport {
    pub match_type: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub unique_or_author_id: [u8; 32],
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub import_name: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2621-2629`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2621-L2629)*

#### Trait Implementations

##### `impl Clone for ImageEnclaveImport`

- <span id="imageenclaveimport-clone"></span>`fn clone(&self) -> ImageEnclaveImport` — [`ImageEnclaveImport`](#imageenclaveimport)

##### `impl Copy for ImageEnclaveImport`

##### `impl Debug for ImageEnclaveImport`

- <span id="imageenclaveimport-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageEnclaveImport`

### `ImageDebugDirectory`

```rust
struct ImageDebugDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub typ: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2643-2652`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2643-L2652)*

#### Trait Implementations

##### `impl Clone for ImageDebugDirectory`

- <span id="imagedebugdirectory-clone"></span>`fn clone(&self) -> ImageDebugDirectory` — [`ImageDebugDirectory`](#imagedebugdirectory)

##### `impl Copy for ImageDebugDirectory`

##### `impl Debug for ImageDebugDirectory`

- <span id="imagedebugdirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDebugDirectory`

### `ImageCoffSymbolsHeader`

```rust
struct ImageCoffSymbolsHeader {
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
    pub lva_to_first_symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U32<crate::endian::LittleEndian>,
    pub lva_to_first_linenumber: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_first_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_last_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_first_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_last_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2674-2683`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2674-L2683)*

#### Trait Implementations

##### `impl Clone for ImageCoffSymbolsHeader`

- <span id="imagecoffsymbolsheader-clone"></span>`fn clone(&self) -> ImageCoffSymbolsHeader` — [`ImageCoffSymbolsHeader`](#imagecoffsymbolsheader)

##### `impl Copy for ImageCoffSymbolsHeader`

##### `impl Debug for ImageCoffSymbolsHeader`

- <span id="imagecoffsymbolsheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageCoffSymbolsHeader`

### `ImageDebugMisc`

```rust
struct ImageDebugMisc {
    pub data_type: crate::endian::U32<crate::endian::LittleEndian>,
    pub length: crate::endian::U32<crate::endian::LittleEndian>,
    pub unicode: u8,
    pub reserved: [u8; 3],
}
```

*Defined in [`object-0.37.3/src/pe.rs:2723-2733`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2723-L2733)*

#### Fields

- **`data_type`**: `crate::endian::U32<crate::endian::LittleEndian>`

  type of misc data, see defines

- **`length`**: `crate::endian::U32<crate::endian::LittleEndian>`

  total length of record, rounded to four byte multiple.

- **`unicode`**: `u8`

  TRUE if data is unicode string

#### Trait Implementations

##### `impl Clone for ImageDebugMisc`

- <span id="imagedebugmisc-clone"></span>`fn clone(&self) -> ImageDebugMisc` — [`ImageDebugMisc`](#imagedebugmisc)

##### `impl Copy for ImageDebugMisc`

##### `impl Debug for ImageDebugMisc`

- <span id="imagedebugmisc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageDebugMisc`

### `ImageFunctionEntry`

```rust
struct ImageFunctionEntry {
    pub starting_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub ending_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_of_prologue: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2743-2747`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2743-L2747)*

#### Trait Implementations

##### `impl Clone for ImageFunctionEntry`

- <span id="imagefunctionentry-clone"></span>`fn clone(&self) -> ImageFunctionEntry` — [`ImageFunctionEntry`](#imagefunctionentry)

##### `impl Copy for ImageFunctionEntry`

##### `impl Debug for ImageFunctionEntry`

- <span id="imagefunctionentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageFunctionEntry`

### `ImageFunctionEntry64`

```rust
struct ImageFunctionEntry64 {
    pub starting_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub ending_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_of_prologue_or_unwind_info_address: crate::endian::U64<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2751-2755`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2751-L2755)*

#### Trait Implementations

##### `impl Clone for ImageFunctionEntry64`

- <span id="imagefunctionentry64-clone"></span>`fn clone(&self) -> ImageFunctionEntry64` — [`ImageFunctionEntry64`](#imagefunctionentry64)

##### `impl Copy for ImageFunctionEntry64`

##### `impl Debug for ImageFunctionEntry64`

- <span id="imagefunctionentry64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageFunctionEntry64`

### `ImageSeparateDebugHeader`

```rust
struct ImageSeparateDebugHeader {
    pub signature: crate::endian::U16<crate::endian::LittleEndian>,
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U32<crate::endian::LittleEndian>,
    pub exported_names_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub debug_directory_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: [crate::endian::U32<crate::endian::LittleEndian>; 2],
}
```

*Defined in [`object-0.37.3/src/pe.rs:2779-2793`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2779-L2793)*

#### Trait Implementations

##### `impl Clone for ImageSeparateDebugHeader`

- <span id="imageseparatedebugheader-clone"></span>`fn clone(&self) -> ImageSeparateDebugHeader` — [`ImageSeparateDebugHeader`](#imageseparatedebugheader)

##### `impl Copy for ImageSeparateDebugHeader`

##### `impl Debug for ImageSeparateDebugHeader`

- <span id="imageseparatedebugheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageSeparateDebugHeader`

### `NonPagedDebugInfo`

```rust
struct NonPagedDebugInfo {
    pub signature: crate::endian::U16<crate::endian::LittleEndian>,
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U64<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2797-2809`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2797-L2809)*

#### Trait Implementations

##### `impl Clone for NonPagedDebugInfo`

- <span id="nonpageddebuginfo-clone"></span>`fn clone(&self) -> NonPagedDebugInfo` — [`NonPagedDebugInfo`](#nonpageddebuginfo)

##### `impl Copy for NonPagedDebugInfo`

##### `impl Debug for NonPagedDebugInfo`

- <span id="nonpageddebuginfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for NonPagedDebugInfo`

### `ImageArchitectureEntry`

```rust
struct ImageArchitectureEntry {
    pub fixup_inst_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub new_inst: crate::endian::U32<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2847-2852`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2847-L2852)*

#### Fields

- **`fixup_inst_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA of instruction to fixup

- **`new_inst`**: `crate::endian::U32<crate::endian::LittleEndian>`

  fixup instruction (see alphaops.h)

#### Trait Implementations

##### `impl Clone for ImageArchitectureEntry`

- <span id="imagearchitectureentry-clone"></span>`fn clone(&self) -> ImageArchitectureEntry` — [`ImageArchitectureEntry`](#imagearchitectureentry)

##### `impl Copy for ImageArchitectureEntry`

##### `impl Debug for ImageArchitectureEntry`

- <span id="imagearchitectureentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageArchitectureEntry`

### `ImportObjectHeader`

```rust
struct ImportObjectHeader {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub ordinal_or_hint: crate::endian::U16<crate::endian::LittleEndian>,
    pub name_type: crate::endian::U16<crate::endian::LittleEndian>,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2863-2882`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2863-L2882)*

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMPORT_OBJECT_HDR_SIG2.

- **`time_date_stamp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Time/date stamp

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  particularly useful for incremental links

- **`ordinal_or_hint`**: `crate::endian::U16<crate::endian::LittleEndian>`

  if grf & IMPORT_OBJECT_ORDINAL

#### Implementations

- <span id="peimportobjectheader-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>` — [`Result`](../index.md#result)

- <span id="peimportobjectheader-parse-data"></span>`fn parse_data<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<ImportObjectData<'data>>` — [`Result`](../index.md#result), [`ImportObjectData`](../read/coff/index.md#importobjectdata)

- <span id="peimportobjectheader-import-type"></span>`fn import_type(&self) -> u16`

- <span id="peimportobjectheader-name-type"></span>`fn name_type(&self) -> u16`

#### Trait Implementations

##### `impl Clone for ImportObjectHeader`

- <span id="importobjectheader-clone"></span>`fn clone(&self) -> ImportObjectHeader` — [`ImportObjectHeader`](#importobjectheader)

##### `impl Copy for ImportObjectHeader`

##### `impl Debug for ImportObjectHeader`

- <span id="importobjectheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImportObjectHeader`

### `ImageCor20Header`

```rust
struct ImageCor20Header {
    pub cb: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_runtime_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_runtime_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub meta_data: ImageDataDirectory,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub entry_point_token_or_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub resources: ImageDataDirectory,
    pub strong_name_signature: ImageDataDirectory,
    pub code_manager_table: ImageDataDirectory,
    pub vtable_fixups: ImageDataDirectory,
    pub export_address_table_jumps: ImageDataDirectory,
    pub managed_native_header: ImageDataDirectory,
}
```

*Defined in [`object-0.37.3/src/pe.rs:2951-2976`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2951-L2976)*

#### Trait Implementations

##### `impl Clone for ImageCor20Header`

- <span id="imagecor20header-clone"></span>`fn clone(&self) -> ImageCor20Header` — [`ImageCor20Header`](#imagecor20header)

##### `impl Copy for ImageCor20Header`

##### `impl Debug for ImageCor20Header`

- <span id="imagecor20header-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ImageCor20Header`

## Constants

### `IMAGE_DOS_SIGNATURE`
```rust
const IMAGE_DOS_SIGNATURE: u16 = 23_117u16;
```

*Defined in [`object-0.37.3/src/pe.rs:16`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L16)*

MZ

### `IMAGE_OS2_SIGNATURE`
```rust
const IMAGE_OS2_SIGNATURE: u16 = 17_742u16;
```

*Defined in [`object-0.37.3/src/pe.rs:18`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L18)*

NE

### `IMAGE_OS2_SIGNATURE_LE`
```rust
const IMAGE_OS2_SIGNATURE_LE: u16 = 17_740u16;
```

*Defined in [`object-0.37.3/src/pe.rs:20`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L20)*

LE

### `IMAGE_VXD_SIGNATURE`
```rust
const IMAGE_VXD_SIGNATURE: u16 = 17_740u16;
```

*Defined in [`object-0.37.3/src/pe.rs:22`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L22)*

LE

### `IMAGE_NT_SIGNATURE`
```rust
const IMAGE_NT_SIGNATURE: u32 = 17_744u32;
```

*Defined in [`object-0.37.3/src/pe.rs:24`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L24)*

PE00

### `IMAGE_SIZEOF_FILE_HEADER`
```rust
const IMAGE_SIZEOF_FILE_HEADER: usize = 20usize;
```

*Defined in [`object-0.37.3/src/pe.rs:273`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L273)*

### `IMAGE_FILE_RELOCS_STRIPPED`
```rust
const IMAGE_FILE_RELOCS_STRIPPED: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:276`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L276)*

Relocation info stripped from file.

### `IMAGE_FILE_EXECUTABLE_IMAGE`
```rust
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:278`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L278)*

File is executable  (i.e. no unresolved external references).

### `IMAGE_FILE_LINE_NUMS_STRIPPED`
```rust
const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:280`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L280)*

Line numbers stripped from file.

### `IMAGE_FILE_LOCAL_SYMS_STRIPPED`
```rust
const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:282`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L282)*

Local symbols stripped from file.

### `IMAGE_FILE_AGGRESIVE_WS_TRIM`
```rust
const IMAGE_FILE_AGGRESIVE_WS_TRIM: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:284`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L284)*

Aggressively trim working set

### `IMAGE_FILE_LARGE_ADDRESS_AWARE`
```rust
const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/pe.rs:286`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L286)*

App can handle >2gb addresses

### `IMAGE_FILE_BYTES_REVERSED_LO`
```rust
const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 128u16;
```

*Defined in [`object-0.37.3/src/pe.rs:288`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L288)*

Bytes of machine word are reversed.

### `IMAGE_FILE_32BIT_MACHINE`
```rust
const IMAGE_FILE_32BIT_MACHINE: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/pe.rs:290`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L290)*

32 bit word machine.

### `IMAGE_FILE_DEBUG_STRIPPED`
```rust
const IMAGE_FILE_DEBUG_STRIPPED: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/pe.rs:292`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L292)*

Debugging info stripped from file in .DBG file

### `IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP`
```rust
const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 1_024u16;
```

*Defined in [`object-0.37.3/src/pe.rs:294`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L294)*

If Image is on removable media, copy and run from the swap file.

### `IMAGE_FILE_NET_RUN_FROM_SWAP`
```rust
const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 2_048u16;
```

*Defined in [`object-0.37.3/src/pe.rs:296`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L296)*

If Image is on Net, copy and run from the swap file.

### `IMAGE_FILE_SYSTEM`
```rust
const IMAGE_FILE_SYSTEM: u16 = 4_096u16;
```

*Defined in [`object-0.37.3/src/pe.rs:298`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L298)*

System File.

### `IMAGE_FILE_DLL`
```rust
const IMAGE_FILE_DLL: u16 = 8_192u16;
```

*Defined in [`object-0.37.3/src/pe.rs:300`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L300)*

File is a DLL.

### `IMAGE_FILE_UP_SYSTEM_ONLY`
```rust
const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/pe.rs:302`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L302)*

File should only be run on a UP machine

### `IMAGE_FILE_BYTES_REVERSED_HI`
```rust
const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:304`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L304)*

Bytes of machine word are reversed.

### `IMAGE_FILE_MACHINE_UNKNOWN`
```rust
const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:306`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L306)*

### `IMAGE_FILE_MACHINE_TARGET_HOST`
```rust
const IMAGE_FILE_MACHINE_TARGET_HOST: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:308`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L308)*

Useful for indicating we want to interact with the host and not a WoW guest.

### `IMAGE_FILE_MACHINE_I386`
```rust
const IMAGE_FILE_MACHINE_I386: u16 = 332u16;
```

*Defined in [`object-0.37.3/src/pe.rs:310`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L310)*

Intel 386.

### `IMAGE_FILE_MACHINE_R3000`
```rust
const IMAGE_FILE_MACHINE_R3000: u16 = 354u16;
```

*Defined in [`object-0.37.3/src/pe.rs:312`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L312)*

MIPS little-endian, 0x160 big-endian

### `IMAGE_FILE_MACHINE_R4000`
```rust
const IMAGE_FILE_MACHINE_R4000: u16 = 358u16;
```

*Defined in [`object-0.37.3/src/pe.rs:314`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L314)*

MIPS little-endian

### `IMAGE_FILE_MACHINE_R10000`
```rust
const IMAGE_FILE_MACHINE_R10000: u16 = 360u16;
```

*Defined in [`object-0.37.3/src/pe.rs:316`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L316)*

MIPS little-endian

### `IMAGE_FILE_MACHINE_WCEMIPSV2`
```rust
const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 361u16;
```

*Defined in [`object-0.37.3/src/pe.rs:318`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L318)*

MIPS little-endian WCE v2

### `IMAGE_FILE_MACHINE_ALPHA`
```rust
const IMAGE_FILE_MACHINE_ALPHA: u16 = 388u16;
```

*Defined in [`object-0.37.3/src/pe.rs:320`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L320)*

Alpha_AXP

### `IMAGE_FILE_MACHINE_SH3`
```rust
const IMAGE_FILE_MACHINE_SH3: u16 = 418u16;
```

*Defined in [`object-0.37.3/src/pe.rs:322`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L322)*

SH3 little-endian

### `IMAGE_FILE_MACHINE_SH3DSP`
```rust
const IMAGE_FILE_MACHINE_SH3DSP: u16 = 419u16;
```

*Defined in [`object-0.37.3/src/pe.rs:323`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L323)*

### `IMAGE_FILE_MACHINE_SH3E`
```rust
const IMAGE_FILE_MACHINE_SH3E: u16 = 420u16;
```

*Defined in [`object-0.37.3/src/pe.rs:325`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L325)*

SH3E little-endian

### `IMAGE_FILE_MACHINE_SH4`
```rust
const IMAGE_FILE_MACHINE_SH4: u16 = 422u16;
```

*Defined in [`object-0.37.3/src/pe.rs:327`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L327)*

SH4 little-endian

### `IMAGE_FILE_MACHINE_SH5`
```rust
const IMAGE_FILE_MACHINE_SH5: u16 = 424u16;
```

*Defined in [`object-0.37.3/src/pe.rs:329`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L329)*

SH5

### `IMAGE_FILE_MACHINE_ARM`
```rust
const IMAGE_FILE_MACHINE_ARM: u16 = 448u16;
```

*Defined in [`object-0.37.3/src/pe.rs:331`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L331)*

ARM Little-Endian

### `IMAGE_FILE_MACHINE_THUMB`
```rust
const IMAGE_FILE_MACHINE_THUMB: u16 = 450u16;
```

*Defined in [`object-0.37.3/src/pe.rs:333`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L333)*

ARM Thumb/Thumb-2 Little-Endian

### `IMAGE_FILE_MACHINE_ARMNT`
```rust
const IMAGE_FILE_MACHINE_ARMNT: u16 = 452u16;
```

*Defined in [`object-0.37.3/src/pe.rs:335`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L335)*

ARM Thumb-2 Little-Endian

### `IMAGE_FILE_MACHINE_AM33`
```rust
const IMAGE_FILE_MACHINE_AM33: u16 = 467u16;
```

*Defined in [`object-0.37.3/src/pe.rs:336`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L336)*

### `IMAGE_FILE_MACHINE_POWERPC`
```rust
const IMAGE_FILE_MACHINE_POWERPC: u16 = 496u16;
```

*Defined in [`object-0.37.3/src/pe.rs:338`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L338)*

IBM PowerPC Little-Endian

### `IMAGE_FILE_MACHINE_POWERPCFP`
```rust
const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 497u16;
```

*Defined in [`object-0.37.3/src/pe.rs:339`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L339)*

### `IMAGE_FILE_MACHINE_POWERPCBE`
```rust
const IMAGE_FILE_MACHINE_POWERPCBE: u16 = 498u16;
```

*Defined in [`object-0.37.3/src/pe.rs:341`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L341)*

IBM PowerPC Big-Endian

### `IMAGE_FILE_MACHINE_IA64`
```rust
const IMAGE_FILE_MACHINE_IA64: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/pe.rs:343`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L343)*

Intel 64

### `IMAGE_FILE_MACHINE_MIPS16`
```rust
const IMAGE_FILE_MACHINE_MIPS16: u16 = 614u16;
```

*Defined in [`object-0.37.3/src/pe.rs:345`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L345)*

MIPS

### `IMAGE_FILE_MACHINE_ALPHA64`
```rust
const IMAGE_FILE_MACHINE_ALPHA64: u16 = 644u16;
```

*Defined in [`object-0.37.3/src/pe.rs:347`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L347)*

ALPHA64

### `IMAGE_FILE_MACHINE_MIPSFPU`
```rust
const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 870u16;
```

*Defined in [`object-0.37.3/src/pe.rs:349`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L349)*

MIPS

### `IMAGE_FILE_MACHINE_MIPSFPU16`
```rust
const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 1_126u16;
```

*Defined in [`object-0.37.3/src/pe.rs:351`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L351)*

MIPS

### `IMAGE_FILE_MACHINE_AXP64`
```rust
const IMAGE_FILE_MACHINE_AXP64: u16 = 644u16;
```

*Defined in [`object-0.37.3/src/pe.rs:352`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L352)*

### `IMAGE_FILE_MACHINE_TRICORE`
```rust
const IMAGE_FILE_MACHINE_TRICORE: u16 = 1_312u16;
```

*Defined in [`object-0.37.3/src/pe.rs:354`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L354)*

Infineon

### `IMAGE_FILE_MACHINE_CEF`
```rust
const IMAGE_FILE_MACHINE_CEF: u16 = 3_311u16;
```

*Defined in [`object-0.37.3/src/pe.rs:355`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L355)*

### `IMAGE_FILE_MACHINE_EBC`
```rust
const IMAGE_FILE_MACHINE_EBC: u16 = 3_772u16;
```

*Defined in [`object-0.37.3/src/pe.rs:357`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L357)*

EFI Byte Code

### `IMAGE_FILE_MACHINE_AMD64`
```rust
const IMAGE_FILE_MACHINE_AMD64: u16 = 34_404u16;
```

*Defined in [`object-0.37.3/src/pe.rs:359`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L359)*

AMD64 (K8)

### `IMAGE_FILE_MACHINE_M32R`
```rust
const IMAGE_FILE_MACHINE_M32R: u16 = 36_929u16;
```

*Defined in [`object-0.37.3/src/pe.rs:361`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L361)*

M32R little-endian

### `IMAGE_FILE_MACHINE_ARM64`
```rust
const IMAGE_FILE_MACHINE_ARM64: u16 = 43_620u16;
```

*Defined in [`object-0.37.3/src/pe.rs:363`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L363)*

ARM64 Little-Endian

### `IMAGE_FILE_MACHINE_ARM64EC`
```rust
const IMAGE_FILE_MACHINE_ARM64EC: u16 = 42_561u16;
```

*Defined in [`object-0.37.3/src/pe.rs:365`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L365)*

ARM64EC ("Emulation Compatible")

### `IMAGE_FILE_MACHINE_CEE`
```rust
const IMAGE_FILE_MACHINE_CEE: u16 = 49_390u16;
```

*Defined in [`object-0.37.3/src/pe.rs:366`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L366)*

### `IMAGE_FILE_MACHINE_RISCV32`
```rust
const IMAGE_FILE_MACHINE_RISCV32: u16 = 20_530u16;
```

*Defined in [`object-0.37.3/src/pe.rs:368`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L368)*

RISCV32

### `IMAGE_FILE_MACHINE_RISCV64`
```rust
const IMAGE_FILE_MACHINE_RISCV64: u16 = 20_580u16;
```

*Defined in [`object-0.37.3/src/pe.rs:370`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L370)*

RISCV64

### `IMAGE_FILE_MACHINE_RISCV128`
```rust
const IMAGE_FILE_MACHINE_RISCV128: u16 = 20_776u16;
```

*Defined in [`object-0.37.3/src/pe.rs:372`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L372)*

RISCV128

### `IMAGE_FILE_MACHINE_ARM64X`
```rust
const IMAGE_FILE_MACHINE_ARM64X: u16 = 42_574u16;
```

*Defined in [`object-0.37.3/src/pe.rs:374`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L374)*

ARM64X (Mixed ARM64 and ARM64EC)

### `IMAGE_FILE_MACHINE_CHPE_X86`
```rust
const IMAGE_FILE_MACHINE_CHPE_X86: u16 = 14_948u16;
```

*Defined in [`object-0.37.3/src/pe.rs:376`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L376)*

CHPE x86 ("Compiled Hybrid Portable Executable")

### `IMAGE_NUMBEROF_DIRECTORY_ENTRIES`
```rust
const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16usize;
```

*Defined in [`object-0.37.3/src/pe.rs:389`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L389)*

### `IMAGE_NT_OPTIONAL_HDR32_MAGIC`
```rust
const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 267u16;
```

*Defined in [`object-0.37.3/src/pe.rs:487`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L487)*

### `IMAGE_NT_OPTIONAL_HDR64_MAGIC`
```rust
const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 523u16;
```

*Defined in [`object-0.37.3/src/pe.rs:488`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L488)*

### `IMAGE_ROM_OPTIONAL_HDR_MAGIC`
```rust
const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 263u16;
```

*Defined in [`object-0.37.3/src/pe.rs:489`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L489)*

### `IMAGE_SUBSYSTEM_UNKNOWN`
```rust
const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:517`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L517)*

Unknown subsystem.

### `IMAGE_SUBSYSTEM_NATIVE`
```rust
const IMAGE_SUBSYSTEM_NATIVE: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:519`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L519)*

Image doesn't require a subsystem.

### `IMAGE_SUBSYSTEM_WINDOWS_GUI`
```rust
const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:521`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L521)*

Image runs in the Windows GUI subsystem.

### `IMAGE_SUBSYSTEM_WINDOWS_CUI`
```rust
const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:523`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L523)*

Image runs in the Windows character subsystem.

### `IMAGE_SUBSYSTEM_OS2_CUI`
```rust
const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:525`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L525)*

image runs in the OS/2 character subsystem.

### `IMAGE_SUBSYSTEM_POSIX_CUI`
```rust
const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:527`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L527)*

image runs in the Posix character subsystem.

### `IMAGE_SUBSYSTEM_NATIVE_WINDOWS`
```rust
const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:529`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L529)*

image is a native Win9x driver.

### `IMAGE_SUBSYSTEM_WINDOWS_CE_GUI`
```rust
const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:531`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L531)*

Image runs in the Windows CE subsystem.

### `IMAGE_SUBSYSTEM_EFI_APPLICATION`
```rust
const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:532`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L532)*

### `IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER`
```rust
const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:533`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L533)*

### `IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER`
```rust
const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:534`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L534)*

### `IMAGE_SUBSYSTEM_EFI_ROM`
```rust
const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:535`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L535)*

### `IMAGE_SUBSYSTEM_XBOX`
```rust
const IMAGE_SUBSYSTEM_XBOX: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:536`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L536)*

### `IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION`
```rust
const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:537`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L537)*

### `IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG`
```rust
const IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:538`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L538)*

### `IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA`
```rust
const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/pe.rs:547`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L547)*

Image can handle a high entropy 64-bit virtual address space.

### `IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE`
```rust
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 64u16;
```

*Defined in [`object-0.37.3/src/pe.rs:549`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L549)*

DLL can move.

### `IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY`
```rust
const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 128u16;
```

*Defined in [`object-0.37.3/src/pe.rs:551`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L551)*

Code Integrity Image

### `IMAGE_DLLCHARACTERISTICS_NX_COMPAT`
```rust
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/pe.rs:553`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L553)*

Image is NX compatible

### `IMAGE_DLLCHARACTERISTICS_NO_ISOLATION`
```rust
const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/pe.rs:555`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L555)*

Image understands isolation and doesn't want it

### `IMAGE_DLLCHARACTERISTICS_NO_SEH`
```rust
const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 1_024u16;
```

*Defined in [`object-0.37.3/src/pe.rs:557`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L557)*

Image does not use SEH.  No SE handler may reside in this image

### `IMAGE_DLLCHARACTERISTICS_NO_BIND`
```rust
const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 2_048u16;
```

*Defined in [`object-0.37.3/src/pe.rs:559`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L559)*

Do not bind this image.

### `IMAGE_DLLCHARACTERISTICS_APPCONTAINER`
```rust
const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 4_096u16;
```

*Defined in [`object-0.37.3/src/pe.rs:561`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L561)*

Image should execute in an AppContainer

### `IMAGE_DLLCHARACTERISTICS_WDM_DRIVER`
```rust
const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 8_192u16;
```

*Defined in [`object-0.37.3/src/pe.rs:563`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L563)*

Driver uses WDM model

### `IMAGE_DLLCHARACTERISTICS_GUARD_CF`
```rust
const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/pe.rs:565`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L565)*

Image supports Control Flow Guard.

### `IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE`
```rust
const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:566`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L566)*

### `IMAGE_DIRECTORY_ENTRY_EXPORT`
```rust
const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0usize;
```

*Defined in [`object-0.37.3/src/pe.rs:571`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L571)*

Export Directory

### `IMAGE_DIRECTORY_ENTRY_IMPORT`
```rust
const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1usize;
```

*Defined in [`object-0.37.3/src/pe.rs:573`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L573)*

Import Directory

### `IMAGE_DIRECTORY_ENTRY_RESOURCE`
```rust
const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2usize;
```

*Defined in [`object-0.37.3/src/pe.rs:575`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L575)*

Resource Directory

### `IMAGE_DIRECTORY_ENTRY_EXCEPTION`
```rust
const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3usize;
```

*Defined in [`object-0.37.3/src/pe.rs:577`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L577)*

Exception Directory

### `IMAGE_DIRECTORY_ENTRY_SECURITY`
```rust
const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4usize;
```

*Defined in [`object-0.37.3/src/pe.rs:579`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L579)*

Security Directory

### `IMAGE_DIRECTORY_ENTRY_BASERELOC`
```rust
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5usize;
```

*Defined in [`object-0.37.3/src/pe.rs:581`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L581)*

Base Relocation Table

### `IMAGE_DIRECTORY_ENTRY_DEBUG`
```rust
const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6usize;
```

*Defined in [`object-0.37.3/src/pe.rs:583`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L583)*

Debug Directory

### `IMAGE_DIRECTORY_ENTRY_ARCHITECTURE`
```rust
const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize = 7usize;
```

*Defined in [`object-0.37.3/src/pe.rs:586`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L586)*

Architecture Specific Data

### `IMAGE_DIRECTORY_ENTRY_GLOBALPTR`
```rust
const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize = 8usize;
```

*Defined in [`object-0.37.3/src/pe.rs:588`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L588)*

RVA of GP

### `IMAGE_DIRECTORY_ENTRY_TLS`
```rust
const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9usize;
```

*Defined in [`object-0.37.3/src/pe.rs:590`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L590)*

TLS Directory

### `IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG`
```rust
const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10usize;
```

*Defined in [`object-0.37.3/src/pe.rs:592`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L592)*

Load Configuration Directory

### `IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT`
```rust
const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize = 11usize;
```

*Defined in [`object-0.37.3/src/pe.rs:594`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L594)*

Bound Import Directory in headers

### `IMAGE_DIRECTORY_ENTRY_IAT`
```rust
const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12usize;
```

*Defined in [`object-0.37.3/src/pe.rs:596`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L596)*

Import Address Table

### `IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`
```rust
const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize = 13usize;
```

*Defined in [`object-0.37.3/src/pe.rs:598`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L598)*

Delay Load Import Descriptors

### `IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`
```rust
const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14usize;
```

*Defined in [`object-0.37.3/src/pe.rs:600`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L600)*

COM Runtime descriptor

### `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`
```rust
const ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID: ClsId;
```

*Defined in [`object-0.37.3/src/pe.rs:672-674`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L672-L674)*

The required value of `AnonObjectHeaderBigobj::class_id`.

### `IMAGE_SIZEOF_SHORT_NAME`
```rust
const IMAGE_SIZEOF_SHORT_NAME: usize = 8usize;
```

*Defined in [`object-0.37.3/src/pe.rs:707`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L707)*

### `IMAGE_SIZEOF_SECTION_HEADER`
```rust
const IMAGE_SIZEOF_SECTION_HEADER: usize = 40usize;
```

*Defined in [`object-0.37.3/src/pe.rs:728`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L728)*

### `IMAGE_SCN_TYPE_NO_PAD`
```rust
const IMAGE_SCN_TYPE_NO_PAD: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/pe.rs:737`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L737)*

Reserved.

### `IMAGE_SCN_CNT_CODE`
```rust
const IMAGE_SCN_CNT_CODE: u32 = 32u32;
```

*Defined in [`object-0.37.3/src/pe.rs:741`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L741)*

Section contains code.

### `IMAGE_SCN_CNT_INITIALIZED_DATA`
```rust
const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 64u32;
```

*Defined in [`object-0.37.3/src/pe.rs:743`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L743)*

Section contains initialized data.

### `IMAGE_SCN_CNT_UNINITIALIZED_DATA`
```rust
const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 128u32;
```

*Defined in [`object-0.37.3/src/pe.rs:745`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L745)*

Section contains uninitialized data.

### `IMAGE_SCN_LNK_OTHER`
```rust
const IMAGE_SCN_LNK_OTHER: u32 = 256u32;
```

*Defined in [`object-0.37.3/src/pe.rs:748`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L748)*

Reserved.

### `IMAGE_SCN_LNK_INFO`
```rust
const IMAGE_SCN_LNK_INFO: u32 = 512u32;
```

*Defined in [`object-0.37.3/src/pe.rs:750`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L750)*

Section contains comments or some other type of information.

### `IMAGE_SCN_LNK_REMOVE`
```rust
const IMAGE_SCN_LNK_REMOVE: u32 = 2_048u32;
```

*Defined in [`object-0.37.3/src/pe.rs:753`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L753)*

Section contents will not become part of image.

### `IMAGE_SCN_LNK_COMDAT`
```rust
const IMAGE_SCN_LNK_COMDAT: u32 = 4_096u32;
```

*Defined in [`object-0.37.3/src/pe.rs:755`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L755)*

Section contents comdat.

### `IMAGE_SCN_NO_DEFER_SPEC_EXC`
```rust
const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32 = 16_384u32;
```

*Defined in [`object-0.37.3/src/pe.rs:759`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L759)*

Reset speculative exceptions handling bits in the TLB entries for this section.

### `IMAGE_SCN_GPREL`
```rust
const IMAGE_SCN_GPREL: u32 = 32_768u32;
```

*Defined in [`object-0.37.3/src/pe.rs:761`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L761)*

Section content can be accessed relative to GP

### `IMAGE_SCN_MEM_FARDATA`
```rust
const IMAGE_SCN_MEM_FARDATA: u32 = 32_768u32;
```

*Defined in [`object-0.37.3/src/pe.rs:762`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L762)*

### `IMAGE_SCN_MEM_PURGEABLE`
```rust
const IMAGE_SCN_MEM_PURGEABLE: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/pe.rs:764`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L764)*

### `IMAGE_SCN_MEM_16BIT`
```rust
const IMAGE_SCN_MEM_16BIT: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/pe.rs:765`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L765)*

### `IMAGE_SCN_MEM_LOCKED`
```rust
const IMAGE_SCN_MEM_LOCKED: u32 = 262_144u32;
```

*Defined in [`object-0.37.3/src/pe.rs:766`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L766)*

### `IMAGE_SCN_MEM_PRELOAD`
```rust
const IMAGE_SCN_MEM_PRELOAD: u32 = 524_288u32;
```

*Defined in [`object-0.37.3/src/pe.rs:767`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L767)*

### `IMAGE_SCN_ALIGN_1BYTES`
```rust
const IMAGE_SCN_ALIGN_1BYTES: u32 = 1_048_576u32;
```

*Defined in [`object-0.37.3/src/pe.rs:769`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L769)*

### `IMAGE_SCN_ALIGN_2BYTES`
```rust
const IMAGE_SCN_ALIGN_2BYTES: u32 = 2_097_152u32;
```

*Defined in [`object-0.37.3/src/pe.rs:770`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L770)*

### `IMAGE_SCN_ALIGN_4BYTES`
```rust
const IMAGE_SCN_ALIGN_4BYTES: u32 = 3_145_728u32;
```

*Defined in [`object-0.37.3/src/pe.rs:771`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L771)*

### `IMAGE_SCN_ALIGN_8BYTES`
```rust
const IMAGE_SCN_ALIGN_8BYTES: u32 = 4_194_304u32;
```

*Defined in [`object-0.37.3/src/pe.rs:772`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L772)*

### `IMAGE_SCN_ALIGN_16BYTES`
```rust
const IMAGE_SCN_ALIGN_16BYTES: u32 = 5_242_880u32;
```

*Defined in [`object-0.37.3/src/pe.rs:774`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L774)*

Default alignment if no others are specified.

### `IMAGE_SCN_ALIGN_32BYTES`
```rust
const IMAGE_SCN_ALIGN_32BYTES: u32 = 6_291_456u32;
```

*Defined in [`object-0.37.3/src/pe.rs:775`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L775)*

### `IMAGE_SCN_ALIGN_64BYTES`
```rust
const IMAGE_SCN_ALIGN_64BYTES: u32 = 7_340_032u32;
```

*Defined in [`object-0.37.3/src/pe.rs:776`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L776)*

### `IMAGE_SCN_ALIGN_128BYTES`
```rust
const IMAGE_SCN_ALIGN_128BYTES: u32 = 8_388_608u32;
```

*Defined in [`object-0.37.3/src/pe.rs:777`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L777)*

### `IMAGE_SCN_ALIGN_256BYTES`
```rust
const IMAGE_SCN_ALIGN_256BYTES: u32 = 9_437_184u32;
```

*Defined in [`object-0.37.3/src/pe.rs:778`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L778)*

### `IMAGE_SCN_ALIGN_512BYTES`
```rust
const IMAGE_SCN_ALIGN_512BYTES: u32 = 10_485_760u32;
```

*Defined in [`object-0.37.3/src/pe.rs:779`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L779)*

### `IMAGE_SCN_ALIGN_1024BYTES`
```rust
const IMAGE_SCN_ALIGN_1024BYTES: u32 = 11_534_336u32;
```

*Defined in [`object-0.37.3/src/pe.rs:780`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L780)*

### `IMAGE_SCN_ALIGN_2048BYTES`
```rust
const IMAGE_SCN_ALIGN_2048BYTES: u32 = 12_582_912u32;
```

*Defined in [`object-0.37.3/src/pe.rs:781`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L781)*

### `IMAGE_SCN_ALIGN_4096BYTES`
```rust
const IMAGE_SCN_ALIGN_4096BYTES: u32 = 13_631_488u32;
```

*Defined in [`object-0.37.3/src/pe.rs:782`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L782)*

### `IMAGE_SCN_ALIGN_8192BYTES`
```rust
const IMAGE_SCN_ALIGN_8192BYTES: u32 = 14_680_064u32;
```

*Defined in [`object-0.37.3/src/pe.rs:783`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L783)*

### `IMAGE_SCN_ALIGN_MASK`
```rust
const IMAGE_SCN_ALIGN_MASK: u32 = 15_728_640u32;
```

*Defined in [`object-0.37.3/src/pe.rs:785`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L785)*

### `IMAGE_SCN_LNK_NRELOC_OVFL`
```rust
const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 16_777_216u32;
```

*Defined in [`object-0.37.3/src/pe.rs:788`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L788)*

Section contains extended relocations.

### `IMAGE_SCN_MEM_DISCARDABLE`
```rust
const IMAGE_SCN_MEM_DISCARDABLE: u32 = 33_554_432u32;
```

*Defined in [`object-0.37.3/src/pe.rs:790`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L790)*

Section can be discarded.

### `IMAGE_SCN_MEM_NOT_CACHED`
```rust
const IMAGE_SCN_MEM_NOT_CACHED: u32 = 67_108_864u32;
```

*Defined in [`object-0.37.3/src/pe.rs:792`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L792)*

Section is not cacheable.

### `IMAGE_SCN_MEM_NOT_PAGED`
```rust
const IMAGE_SCN_MEM_NOT_PAGED: u32 = 134_217_728u32;
```

*Defined in [`object-0.37.3/src/pe.rs:794`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L794)*

Section is not pageable.

### `IMAGE_SCN_MEM_SHARED`
```rust
const IMAGE_SCN_MEM_SHARED: u32 = 268_435_456u32;
```

*Defined in [`object-0.37.3/src/pe.rs:796`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L796)*

Section is shareable.

### `IMAGE_SCN_MEM_EXECUTE`
```rust
const IMAGE_SCN_MEM_EXECUTE: u32 = 536_870_912u32;
```

*Defined in [`object-0.37.3/src/pe.rs:798`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L798)*

Section is executable.

### `IMAGE_SCN_MEM_READ`
```rust
const IMAGE_SCN_MEM_READ: u32 = 1_073_741_824u32;
```

*Defined in [`object-0.37.3/src/pe.rs:800`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L800)*

Section is readable.

### `IMAGE_SCN_MEM_WRITE`
```rust
const IMAGE_SCN_MEM_WRITE: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:802`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L802)*

Section is writeable.

### `IMAGE_SCN_SCALE_INDEX`
```rust
const IMAGE_SCN_SCALE_INDEX: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:808`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L808)*

Tls index is scaled

### `IMAGE_SIZEOF_SYMBOL`
```rust
const IMAGE_SIZEOF_SYMBOL: usize = 18usize;
```

*Defined in [`object-0.37.3/src/pe.rs:827`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L827)*

### `IMAGE_SIZEOF_SYMBOL_EX`
```rust
const IMAGE_SIZEOF_SYMBOL_EX: usize = 20usize;
```

*Defined in [`object-0.37.3/src/pe.rs:846`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L846)*

### `IMAGE_SYM_UNDEFINED`
```rust
const IMAGE_SYM_UNDEFINED: i32 = 0i32;
```

*Defined in [`object-0.37.3/src/pe.rs:858`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L858)*

Symbol is undefined or is common.

### `IMAGE_SYM_ABSOLUTE`
```rust
const IMAGE_SYM_ABSOLUTE: i32 = -1i32;
```

*Defined in [`object-0.37.3/src/pe.rs:860`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L860)*

Symbol is an absolute value.

### `IMAGE_SYM_DEBUG`
```rust
const IMAGE_SYM_DEBUG: i32 = -2i32;
```

*Defined in [`object-0.37.3/src/pe.rs:862`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L862)*

Symbol is a special debug item.

### `IMAGE_SYM_SECTION_MAX`
```rust
const IMAGE_SYM_SECTION_MAX: u16 = 65_279u16;
```

*Defined in [`object-0.37.3/src/pe.rs:864`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L864)*

Values 0xFF00-0xFFFF are special

### `IMAGE_SYM_SECTION_MAX_EX`
```rust
const IMAGE_SYM_SECTION_MAX_EX: u32 = 2_147_483_647u32;
```

*Defined in [`object-0.37.3/src/pe.rs:865`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L865)*

### `IMAGE_SYM_TYPE_NULL`
```rust
const IMAGE_SYM_TYPE_NULL: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:870`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L870)*

no type.

### `IMAGE_SYM_TYPE_VOID`
```rust
const IMAGE_SYM_TYPE_VOID: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:871`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L871)*

### `IMAGE_SYM_TYPE_CHAR`
```rust
const IMAGE_SYM_TYPE_CHAR: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:873`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L873)*

type character.

### `IMAGE_SYM_TYPE_SHORT`
```rust
const IMAGE_SYM_TYPE_SHORT: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:875`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L875)*

type short integer.

### `IMAGE_SYM_TYPE_INT`
```rust
const IMAGE_SYM_TYPE_INT: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:876`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L876)*

### `IMAGE_SYM_TYPE_LONG`
```rust
const IMAGE_SYM_TYPE_LONG: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:877`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L877)*

### `IMAGE_SYM_TYPE_FLOAT`
```rust
const IMAGE_SYM_TYPE_FLOAT: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:878`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L878)*

### `IMAGE_SYM_TYPE_DOUBLE`
```rust
const IMAGE_SYM_TYPE_DOUBLE: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:879`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L879)*

### `IMAGE_SYM_TYPE_STRUCT`
```rust
const IMAGE_SYM_TYPE_STRUCT: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:880`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L880)*

### `IMAGE_SYM_TYPE_UNION`
```rust
const IMAGE_SYM_TYPE_UNION: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:881`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L881)*

### `IMAGE_SYM_TYPE_ENUM`
```rust
const IMAGE_SYM_TYPE_ENUM: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:883`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L883)*

enumeration.

### `IMAGE_SYM_TYPE_MOE`
```rust
const IMAGE_SYM_TYPE_MOE: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:885`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L885)*

member of enumeration.

### `IMAGE_SYM_TYPE_BYTE`
```rust
const IMAGE_SYM_TYPE_BYTE: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:886`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L886)*

### `IMAGE_SYM_TYPE_WORD`
```rust
const IMAGE_SYM_TYPE_WORD: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:887`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L887)*

### `IMAGE_SYM_TYPE_UINT`
```rust
const IMAGE_SYM_TYPE_UINT: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:888`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L888)*

### `IMAGE_SYM_TYPE_DWORD`
```rust
const IMAGE_SYM_TYPE_DWORD: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:889`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L889)*

### `IMAGE_SYM_TYPE_PCODE`
```rust
const IMAGE_SYM_TYPE_PCODE: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:890`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L890)*

### `IMAGE_SYM_DTYPE_NULL`
```rust
const IMAGE_SYM_DTYPE_NULL: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:895`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L895)*

no derived type.

### `IMAGE_SYM_DTYPE_POINTER`
```rust
const IMAGE_SYM_DTYPE_POINTER: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:897`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L897)*

pointer.

### `IMAGE_SYM_DTYPE_FUNCTION`
```rust
const IMAGE_SYM_DTYPE_FUNCTION: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:899`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L899)*

function.

### `IMAGE_SYM_DTYPE_ARRAY`
```rust
const IMAGE_SYM_DTYPE_ARRAY: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:901`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L901)*

array.

### `IMAGE_SYM_CLASS_END_OF_FUNCTION`
```rust
const IMAGE_SYM_CLASS_END_OF_FUNCTION: u8 = 255u8;
```

*Defined in [`object-0.37.3/src/pe.rs:904`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L904)*

### `IMAGE_SYM_CLASS_NULL`
```rust
const IMAGE_SYM_CLASS_NULL: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/pe.rs:905`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L905)*

### `IMAGE_SYM_CLASS_AUTOMATIC`
```rust
const IMAGE_SYM_CLASS_AUTOMATIC: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/pe.rs:906`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L906)*

### `IMAGE_SYM_CLASS_EXTERNAL`
```rust
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/pe.rs:907`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L907)*

### `IMAGE_SYM_CLASS_STATIC`
```rust
const IMAGE_SYM_CLASS_STATIC: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/pe.rs:908`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L908)*

### `IMAGE_SYM_CLASS_REGISTER`
```rust
const IMAGE_SYM_CLASS_REGISTER: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/pe.rs:909`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L909)*

### `IMAGE_SYM_CLASS_EXTERNAL_DEF`
```rust
const IMAGE_SYM_CLASS_EXTERNAL_DEF: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/pe.rs:910`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L910)*

### `IMAGE_SYM_CLASS_LABEL`
```rust
const IMAGE_SYM_CLASS_LABEL: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/pe.rs:911`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L911)*

### `IMAGE_SYM_CLASS_UNDEFINED_LABEL`
```rust
const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/pe.rs:912`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L912)*

### `IMAGE_SYM_CLASS_MEMBER_OF_STRUCT`
```rust
const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/pe.rs:913`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L913)*

### `IMAGE_SYM_CLASS_ARGUMENT`
```rust
const IMAGE_SYM_CLASS_ARGUMENT: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/pe.rs:914`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L914)*

### `IMAGE_SYM_CLASS_STRUCT_TAG`
```rust
const IMAGE_SYM_CLASS_STRUCT_TAG: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/pe.rs:915`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L915)*

### `IMAGE_SYM_CLASS_MEMBER_OF_UNION`
```rust
const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u8 = 11u8;
```

*Defined in [`object-0.37.3/src/pe.rs:916`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L916)*

### `IMAGE_SYM_CLASS_UNION_TAG`
```rust
const IMAGE_SYM_CLASS_UNION_TAG: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/pe.rs:917`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L917)*

### `IMAGE_SYM_CLASS_TYPE_DEFINITION`
```rust
const IMAGE_SYM_CLASS_TYPE_DEFINITION: u8 = 13u8;
```

*Defined in [`object-0.37.3/src/pe.rs:918`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L918)*

### `IMAGE_SYM_CLASS_UNDEFINED_STATIC`
```rust
const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u8 = 14u8;
```

*Defined in [`object-0.37.3/src/pe.rs:919`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L919)*

### `IMAGE_SYM_CLASS_ENUM_TAG`
```rust
const IMAGE_SYM_CLASS_ENUM_TAG: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/pe.rs:920`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L920)*

### `IMAGE_SYM_CLASS_MEMBER_OF_ENUM`
```rust
const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/pe.rs:921`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L921)*

### `IMAGE_SYM_CLASS_REGISTER_PARAM`
```rust
const IMAGE_SYM_CLASS_REGISTER_PARAM: u8 = 17u8;
```

*Defined in [`object-0.37.3/src/pe.rs:922`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L922)*

### `IMAGE_SYM_CLASS_BIT_FIELD`
```rust
const IMAGE_SYM_CLASS_BIT_FIELD: u8 = 18u8;
```

*Defined in [`object-0.37.3/src/pe.rs:923`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L923)*

### `IMAGE_SYM_CLASS_FAR_EXTERNAL`
```rust
const IMAGE_SYM_CLASS_FAR_EXTERNAL: u8 = 68u8;
```

*Defined in [`object-0.37.3/src/pe.rs:925`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L925)*

### `IMAGE_SYM_CLASS_BLOCK`
```rust
const IMAGE_SYM_CLASS_BLOCK: u8 = 100u8;
```

*Defined in [`object-0.37.3/src/pe.rs:927`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L927)*

### `IMAGE_SYM_CLASS_FUNCTION`
```rust
const IMAGE_SYM_CLASS_FUNCTION: u8 = 101u8;
```

*Defined in [`object-0.37.3/src/pe.rs:928`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L928)*

### `IMAGE_SYM_CLASS_END_OF_STRUCT`
```rust
const IMAGE_SYM_CLASS_END_OF_STRUCT: u8 = 102u8;
```

*Defined in [`object-0.37.3/src/pe.rs:929`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L929)*

### `IMAGE_SYM_CLASS_FILE`
```rust
const IMAGE_SYM_CLASS_FILE: u8 = 103u8;
```

*Defined in [`object-0.37.3/src/pe.rs:930`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L930)*

### `IMAGE_SYM_CLASS_SECTION`
```rust
const IMAGE_SYM_CLASS_SECTION: u8 = 104u8;
```

*Defined in [`object-0.37.3/src/pe.rs:932`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L932)*

### `IMAGE_SYM_CLASS_WEAK_EXTERNAL`
```rust
const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u8 = 105u8;
```

*Defined in [`object-0.37.3/src/pe.rs:933`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L933)*

### `IMAGE_SYM_CLASS_CLR_TOKEN`
```rust
const IMAGE_SYM_CLASS_CLR_TOKEN: u8 = 107u8;
```

*Defined in [`object-0.37.3/src/pe.rs:935`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L935)*

### `N_BTMASK`
```rust
const N_BTMASK: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:939`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L939)*

### `N_TMASK`
```rust
const N_TMASK: u16 = 48u16;
```

*Defined in [`object-0.37.3/src/pe.rs:940`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L940)*

### `N_TMASK1`
```rust
const N_TMASK1: u16 = 192u16;
```

*Defined in [`object-0.37.3/src/pe.rs:941`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L941)*

### `N_TMASK2`
```rust
const N_TMASK2: u16 = 240u16;
```

*Defined in [`object-0.37.3/src/pe.rs:942`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L942)*

### `N_BTSHFT`
```rust
const N_BTSHFT: usize = 4usize;
```

*Defined in [`object-0.37.3/src/pe.rs:943`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L943)*

### `N_TSHIFT`
```rust
const N_TSHIFT: usize = 2usize;
```

*Defined in [`object-0.37.3/src/pe.rs:944`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L944)*

### `IMAGE_SYM_DTYPE_SHIFT`
```rust
const IMAGE_SYM_DTYPE_SHIFT: usize = 4usize;
```

*Defined in [`object-0.37.3/src/pe.rs:946`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L946)*

### `IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF`
```rust
const IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:966`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L966)*

### `IMAGE_COMDAT_SELECT_NODUPLICATES`
```rust
const IMAGE_COMDAT_SELECT_NODUPLICATES: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1041`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1041)*

### `IMAGE_COMDAT_SELECT_ANY`
```rust
const IMAGE_COMDAT_SELECT_ANY: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1042`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1042)*

### `IMAGE_COMDAT_SELECT_SAME_SIZE`
```rust
const IMAGE_COMDAT_SELECT_SAME_SIZE: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1043`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1043)*

### `IMAGE_COMDAT_SELECT_EXACT_MATCH`
```rust
const IMAGE_COMDAT_SELECT_EXACT_MATCH: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1044`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1044)*

### `IMAGE_COMDAT_SELECT_ASSOCIATIVE`
```rust
const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1045`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1045)*

### `IMAGE_COMDAT_SELECT_LARGEST`
```rust
const IMAGE_COMDAT_SELECT_LARGEST: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1046`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1046)*

### `IMAGE_COMDAT_SELECT_NEWEST`
```rust
const IMAGE_COMDAT_SELECT_NEWEST: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/pe.rs:1047`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1047)*

### `IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY`
```rust
const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:1049`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1049)*

### `IMAGE_WEAK_EXTERN_SEARCH_LIBRARY`
```rust
const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:1050`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1050)*

### `IMAGE_WEAK_EXTERN_SEARCH_ALIAS`
```rust
const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/pe.rs:1051`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1051)*

### `IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY`
```rust
const IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/pe.rs:1052`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1052)*

### `IMAGE_REL_I386_ABSOLUTE`
```rust
const IMAGE_REL_I386_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1072`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1072)*

Reference is absolute, no relocation is necessary

### `IMAGE_REL_I386_DIR16`
```rust
const IMAGE_REL_I386_DIR16: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1074`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1074)*

Direct 16-bit reference to the symbols virtual address

### `IMAGE_REL_I386_REL16`
```rust
const IMAGE_REL_I386_REL16: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1076`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1076)*

PC-relative 16-bit reference to the symbols virtual address

### `IMAGE_REL_I386_DIR32`
```rust
const IMAGE_REL_I386_DIR32: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1078`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1078)*

Direct 32-bit reference to the symbols virtual address

### `IMAGE_REL_I386_DIR32NB`
```rust
const IMAGE_REL_I386_DIR32NB: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1080`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1080)*

Direct 32-bit reference to the symbols virtual address, base not included

### `IMAGE_REL_I386_SEG12`
```rust
const IMAGE_REL_I386_SEG12: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1082`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1082)*

Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address

### `IMAGE_REL_I386_SECTION`
```rust
const IMAGE_REL_I386_SECTION: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1083`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1083)*

### `IMAGE_REL_I386_SECREL`
```rust
const IMAGE_REL_I386_SECREL: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1084`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1084)*

### `IMAGE_REL_I386_TOKEN`
```rust
const IMAGE_REL_I386_TOKEN: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1086`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1086)*

clr token

### `IMAGE_REL_I386_SECREL7`
```rust
const IMAGE_REL_I386_SECREL7: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1088`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1088)*

7 bit offset from base of section containing target

### `IMAGE_REL_I386_REL32`
```rust
const IMAGE_REL_I386_REL32: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1090`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1090)*

PC-relative 32-bit reference to the symbols virtual address

### `IMAGE_REL_MIPS_ABSOLUTE`
```rust
const IMAGE_REL_MIPS_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1096`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1096)*

Reference is absolute, no relocation is necessary

### `IMAGE_REL_MIPS_REFHALF`
```rust
const IMAGE_REL_MIPS_REFHALF: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1097`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1097)*

### `IMAGE_REL_MIPS_REFWORD`
```rust
const IMAGE_REL_MIPS_REFWORD: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1098`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1098)*

### `IMAGE_REL_MIPS_JMPADDR`
```rust
const IMAGE_REL_MIPS_JMPADDR: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1099`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1099)*

### `IMAGE_REL_MIPS_REFHI`
```rust
const IMAGE_REL_MIPS_REFHI: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1100`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1100)*

### `IMAGE_REL_MIPS_REFLO`
```rust
const IMAGE_REL_MIPS_REFLO: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1101`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1101)*

### `IMAGE_REL_MIPS_GPREL`
```rust
const IMAGE_REL_MIPS_GPREL: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1102`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1102)*

### `IMAGE_REL_MIPS_LITERAL`
```rust
const IMAGE_REL_MIPS_LITERAL: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1103`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1103)*

### `IMAGE_REL_MIPS_SECTION`
```rust
const IMAGE_REL_MIPS_SECTION: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1104`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1104)*

### `IMAGE_REL_MIPS_SECREL`
```rust
const IMAGE_REL_MIPS_SECREL: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1105`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1105)*

### `IMAGE_REL_MIPS_SECRELLO`
```rust
const IMAGE_REL_MIPS_SECRELLO: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1107`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1107)*

Low 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_MIPS_SECRELHI`
```rust
const IMAGE_REL_MIPS_SECRELHI: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1109`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1109)*

High 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_MIPS_TOKEN`
```rust
const IMAGE_REL_MIPS_TOKEN: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1111`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1111)*

clr token

### `IMAGE_REL_MIPS_JMPADDR16`
```rust
const IMAGE_REL_MIPS_JMPADDR16: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1112`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1112)*

### `IMAGE_REL_MIPS_REFWORDNB`
```rust
const IMAGE_REL_MIPS_REFWORDNB: u16 = 34u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1113`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1113)*

### `IMAGE_REL_MIPS_PAIR`
```rust
const IMAGE_REL_MIPS_PAIR: u16 = 37u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1114`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1114)*

### `IMAGE_REL_ALPHA_ABSOLUTE`
```rust
const IMAGE_REL_ALPHA_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1119`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1119)*

### `IMAGE_REL_ALPHA_REFLONG`
```rust
const IMAGE_REL_ALPHA_REFLONG: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1120`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1120)*

### `IMAGE_REL_ALPHA_REFQUAD`
```rust
const IMAGE_REL_ALPHA_REFQUAD: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1121`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1121)*

### `IMAGE_REL_ALPHA_GPREL32`
```rust
const IMAGE_REL_ALPHA_GPREL32: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1122`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1122)*

### `IMAGE_REL_ALPHA_LITERAL`
```rust
const IMAGE_REL_ALPHA_LITERAL: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1123`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1123)*

### `IMAGE_REL_ALPHA_LITUSE`
```rust
const IMAGE_REL_ALPHA_LITUSE: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1124`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1124)*

### `IMAGE_REL_ALPHA_GPDISP`
```rust
const IMAGE_REL_ALPHA_GPDISP: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1125`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1125)*

### `IMAGE_REL_ALPHA_BRADDR`
```rust
const IMAGE_REL_ALPHA_BRADDR: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1126`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1126)*

### `IMAGE_REL_ALPHA_HINT`
```rust
const IMAGE_REL_ALPHA_HINT: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1127`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1127)*

### `IMAGE_REL_ALPHA_INLINE_REFLONG`
```rust
const IMAGE_REL_ALPHA_INLINE_REFLONG: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1128`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1128)*

### `IMAGE_REL_ALPHA_REFHI`
```rust
const IMAGE_REL_ALPHA_REFHI: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1129`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1129)*

### `IMAGE_REL_ALPHA_REFLO`
```rust
const IMAGE_REL_ALPHA_REFLO: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1130`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1130)*

### `IMAGE_REL_ALPHA_PAIR`
```rust
const IMAGE_REL_ALPHA_PAIR: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1131`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1131)*

### `IMAGE_REL_ALPHA_MATCH`
```rust
const IMAGE_REL_ALPHA_MATCH: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1132`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1132)*

### `IMAGE_REL_ALPHA_SECTION`
```rust
const IMAGE_REL_ALPHA_SECTION: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1133`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1133)*

### `IMAGE_REL_ALPHA_SECREL`
```rust
const IMAGE_REL_ALPHA_SECREL: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1134`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1134)*

### `IMAGE_REL_ALPHA_REFLONGNB`
```rust
const IMAGE_REL_ALPHA_REFLONGNB: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1135`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1135)*

### `IMAGE_REL_ALPHA_SECRELLO`
```rust
const IMAGE_REL_ALPHA_SECRELLO: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1137`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1137)*

Low 16-bit section relative reference

### `IMAGE_REL_ALPHA_SECRELHI`
```rust
const IMAGE_REL_ALPHA_SECRELHI: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1139`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1139)*

High 16-bit section relative reference

### `IMAGE_REL_ALPHA_REFQ3`
```rust
const IMAGE_REL_ALPHA_REFQ3: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1141`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1141)*

High 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_REFQ2`
```rust
const IMAGE_REL_ALPHA_REFQ2: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1143`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1143)*

Middle 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_REFQ1`
```rust
const IMAGE_REL_ALPHA_REFQ1: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1145`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1145)*

Low 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_GPRELLO`
```rust
const IMAGE_REL_ALPHA_GPRELLO: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1147`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1147)*

Low 16-bit GP relative reference

### `IMAGE_REL_ALPHA_GPRELHI`
```rust
const IMAGE_REL_ALPHA_GPRELHI: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1149`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1149)*

High 16-bit GP relative reference

### `IMAGE_REL_PPC_ABSOLUTE`
```rust
const IMAGE_REL_PPC_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1155`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1155)*

NOP

### `IMAGE_REL_PPC_ADDR64`
```rust
const IMAGE_REL_PPC_ADDR64: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1157`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1157)*

64-bit address

### `IMAGE_REL_PPC_ADDR32`
```rust
const IMAGE_REL_PPC_ADDR32: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1159`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1159)*

32-bit address

### `IMAGE_REL_PPC_ADDR24`
```rust
const IMAGE_REL_PPC_ADDR24: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1161`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1161)*

26-bit address, shifted left 2 (branch absolute)

### `IMAGE_REL_PPC_ADDR16`
```rust
const IMAGE_REL_PPC_ADDR16: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1163`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1163)*

16-bit address

### `IMAGE_REL_PPC_ADDR14`
```rust
const IMAGE_REL_PPC_ADDR14: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1165`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1165)*

16-bit address, shifted left 2 (load doubleword)

### `IMAGE_REL_PPC_REL24`
```rust
const IMAGE_REL_PPC_REL24: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1167`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1167)*

26-bit PC-relative offset, shifted left 2 (branch relative)

### `IMAGE_REL_PPC_REL14`
```rust
const IMAGE_REL_PPC_REL14: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1169`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1169)*

16-bit PC-relative offset, shifted left 2 (br cond relative)

### `IMAGE_REL_PPC_TOCREL16`
```rust
const IMAGE_REL_PPC_TOCREL16: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1171`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1171)*

16-bit offset from TOC base

### `IMAGE_REL_PPC_TOCREL14`
```rust
const IMAGE_REL_PPC_TOCREL14: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1173`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1173)*

16-bit offset from TOC base, shifted left 2 (load doubleword)

### `IMAGE_REL_PPC_ADDR32NB`
```rust
const IMAGE_REL_PPC_ADDR32NB: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1176`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1176)*

32-bit addr w/o image base

### `IMAGE_REL_PPC_SECREL`
```rust
const IMAGE_REL_PPC_SECREL: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1178`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1178)*

va of containing section (as in an image sectionhdr)

### `IMAGE_REL_PPC_SECTION`
```rust
const IMAGE_REL_PPC_SECTION: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1180`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1180)*

sectionheader number

### `IMAGE_REL_PPC_IFGLUE`
```rust
const IMAGE_REL_PPC_IFGLUE: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1182`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1182)*

substitute TOC restore instruction iff symbol is glue code

### `IMAGE_REL_PPC_IMGLUE`
```rust
const IMAGE_REL_PPC_IMGLUE: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1184`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1184)*

symbol is glue code; virtual address is TOC restore instruction

### `IMAGE_REL_PPC_SECREL16`
```rust
const IMAGE_REL_PPC_SECREL16: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1186`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1186)*

va of containing section (limited to 16 bits)

### `IMAGE_REL_PPC_REFHI`
```rust
const IMAGE_REL_PPC_REFHI: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1187`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1187)*

### `IMAGE_REL_PPC_REFLO`
```rust
const IMAGE_REL_PPC_REFLO: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1188`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1188)*

### `IMAGE_REL_PPC_PAIR`
```rust
const IMAGE_REL_PPC_PAIR: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1189`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1189)*

### `IMAGE_REL_PPC_SECRELLO`
```rust
const IMAGE_REL_PPC_SECRELLO: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1191`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1191)*

Low 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_PPC_SECRELHI`
```rust
const IMAGE_REL_PPC_SECRELHI: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1193`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1193)*

High 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_PPC_GPREL`
```rust
const IMAGE_REL_PPC_GPREL: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1194`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1194)*

### `IMAGE_REL_PPC_TOKEN`
```rust
const IMAGE_REL_PPC_TOKEN: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1196`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1196)*

clr token

### `IMAGE_REL_PPC_TYPEMASK`
```rust
const IMAGE_REL_PPC_TYPEMASK: u16 = 255u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1199`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1199)*

mask to isolate above values in IMAGE_RELOCATION.Type

### `IMAGE_REL_PPC_NEG`
```rust
const IMAGE_REL_PPC_NEG: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1204`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1204)*

subtract reloc value rather than adding it

### `IMAGE_REL_PPC_BRTAKEN`
```rust
const IMAGE_REL_PPC_BRTAKEN: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1206`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1206)*

fix branch prediction bit to predict branch taken

### `IMAGE_REL_PPC_BRNTAKEN`
```rust
const IMAGE_REL_PPC_BRNTAKEN: u16 = 1_024u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1208`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1208)*

fix branch prediction bit to predict branch not taken

### `IMAGE_REL_PPC_TOCDEFN`
```rust
const IMAGE_REL_PPC_TOCDEFN: u16 = 2_048u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1210`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1210)*

toc slot defined in file (or, data in toc)

### `IMAGE_REL_SH3_ABSOLUTE`
```rust
const IMAGE_REL_SH3_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1216`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1216)*

No relocation

### `IMAGE_REL_SH3_DIRECT16`
```rust
const IMAGE_REL_SH3_DIRECT16: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1218`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1218)*

16 bit direct

### `IMAGE_REL_SH3_DIRECT32`
```rust
const IMAGE_REL_SH3_DIRECT32: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1220`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1220)*

32 bit direct

### `IMAGE_REL_SH3_DIRECT8`
```rust
const IMAGE_REL_SH3_DIRECT8: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1222`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1222)*

8 bit direct, -128..255

### `IMAGE_REL_SH3_DIRECT8_WORD`
```rust
const IMAGE_REL_SH3_DIRECT8_WORD: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1224`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1224)*

8 bit direct .W (0 ext.)

### `IMAGE_REL_SH3_DIRECT8_LONG`
```rust
const IMAGE_REL_SH3_DIRECT8_LONG: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1226`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1226)*

8 bit direct .L (0 ext.)

### `IMAGE_REL_SH3_DIRECT4`
```rust
const IMAGE_REL_SH3_DIRECT4: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1228`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1228)*

4 bit direct (0 ext.)

### `IMAGE_REL_SH3_DIRECT4_WORD`
```rust
const IMAGE_REL_SH3_DIRECT4_WORD: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1230`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1230)*

4 bit direct .W (0 ext.)

### `IMAGE_REL_SH3_DIRECT4_LONG`
```rust
const IMAGE_REL_SH3_DIRECT4_LONG: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1232`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1232)*

4 bit direct .L (0 ext.)

### `IMAGE_REL_SH3_PCREL8_WORD`
```rust
const IMAGE_REL_SH3_PCREL8_WORD: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1234`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1234)*

8 bit PC relative .W

### `IMAGE_REL_SH3_PCREL8_LONG`
```rust
const IMAGE_REL_SH3_PCREL8_LONG: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1236`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1236)*

8 bit PC relative .L

### `IMAGE_REL_SH3_PCREL12_WORD`
```rust
const IMAGE_REL_SH3_PCREL12_WORD: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1238`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1238)*

12 LSB PC relative .W

### `IMAGE_REL_SH3_STARTOF_SECTION`
```rust
const IMAGE_REL_SH3_STARTOF_SECTION: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1240`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1240)*

Start of EXE section

### `IMAGE_REL_SH3_SIZEOF_SECTION`
```rust
const IMAGE_REL_SH3_SIZEOF_SECTION: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1242`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1242)*

Size of EXE section

### `IMAGE_REL_SH3_SECTION`
```rust
const IMAGE_REL_SH3_SECTION: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1244`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1244)*

Section table index

### `IMAGE_REL_SH3_SECREL`
```rust
const IMAGE_REL_SH3_SECREL: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1246`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1246)*

Offset within section

### `IMAGE_REL_SH3_DIRECT32_NB`
```rust
const IMAGE_REL_SH3_DIRECT32_NB: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1248`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1248)*

32 bit direct not based

### `IMAGE_REL_SH3_GPREL4_LONG`
```rust
const IMAGE_REL_SH3_GPREL4_LONG: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1250`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1250)*

GP-relative addressing

### `IMAGE_REL_SH3_TOKEN`
```rust
const IMAGE_REL_SH3_TOKEN: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1252`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1252)*

clr token

### `IMAGE_REL_SHM_PCRELPT`
```rust
const IMAGE_REL_SHM_PCRELPT: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1255`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1255)*

Offset from current instruction in longwords
if not NOMODE, insert the inverse of the low bit at bit 32 to select PTA/PTB

### `IMAGE_REL_SHM_REFLO`
```rust
const IMAGE_REL_SHM_REFLO: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1257`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1257)*

Low bits of 32-bit address

### `IMAGE_REL_SHM_REFHALF`
```rust
const IMAGE_REL_SHM_REFHALF: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1259`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1259)*

High bits of 32-bit address

### `IMAGE_REL_SHM_RELLO`
```rust
const IMAGE_REL_SHM_RELLO: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1261`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1261)*

Low bits of relative reference

### `IMAGE_REL_SHM_RELHALF`
```rust
const IMAGE_REL_SHM_RELHALF: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1263`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1263)*

High bits of relative reference

### `IMAGE_REL_SHM_PAIR`
```rust
const IMAGE_REL_SHM_PAIR: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1265`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1265)*

offset operand for relocation

### `IMAGE_REL_SH_NOMODE`
```rust
const IMAGE_REL_SH_NOMODE: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1268`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1268)*

relocation ignores section mode

### `IMAGE_REL_ARM_ABSOLUTE`
```rust
const IMAGE_REL_ARM_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1271`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1271)*

No relocation required

### `IMAGE_REL_ARM_ADDR32`
```rust
const IMAGE_REL_ARM_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1273`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1273)*

32 bit address

### `IMAGE_REL_ARM_ADDR32NB`
```rust
const IMAGE_REL_ARM_ADDR32NB: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1275`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1275)*

32 bit address w/o image base

### `IMAGE_REL_ARM_BRANCH24`
```rust
const IMAGE_REL_ARM_BRANCH24: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1277`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1277)*

24 bit offset << 2 & sign ext.

### `IMAGE_REL_ARM_BRANCH11`
```rust
const IMAGE_REL_ARM_BRANCH11: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1279`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1279)*

Thumb: 2 11 bit offsets

### `IMAGE_REL_ARM_TOKEN`
```rust
const IMAGE_REL_ARM_TOKEN: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1281`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1281)*

clr token

### `IMAGE_REL_ARM_GPREL12`
```rust
const IMAGE_REL_ARM_GPREL12: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1283`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1283)*

GP-relative addressing (ARM)

### `IMAGE_REL_ARM_GPREL7`
```rust
const IMAGE_REL_ARM_GPREL7: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1285`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1285)*

GP-relative addressing (Thumb)

### `IMAGE_REL_ARM_BLX24`
```rust
const IMAGE_REL_ARM_BLX24: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1286`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1286)*

### `IMAGE_REL_ARM_BLX11`
```rust
const IMAGE_REL_ARM_BLX11: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1287`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1287)*

### `IMAGE_REL_ARM_REL32`
```rust
const IMAGE_REL_ARM_REL32: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1289`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1289)*

32-bit relative address from byte following reloc

### `IMAGE_REL_ARM_SECTION`
```rust
const IMAGE_REL_ARM_SECTION: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1291`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1291)*

Section table index

### `IMAGE_REL_ARM_SECREL`
```rust
const IMAGE_REL_ARM_SECREL: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1293`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1293)*

Offset within section

### `IMAGE_REL_ARM_MOV32A`
```rust
const IMAGE_REL_ARM_MOV32A: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1295`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1295)*

ARM: MOVW/MOVT

### `IMAGE_REL_ARM_MOV32`
```rust
const IMAGE_REL_ARM_MOV32: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1297`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1297)*

ARM: MOVW/MOVT (deprecated)

### `IMAGE_REL_ARM_MOV32T`
```rust
const IMAGE_REL_ARM_MOV32T: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1299`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1299)*

Thumb: MOVW/MOVT

### `IMAGE_REL_THUMB_MOV32`
```rust
const IMAGE_REL_THUMB_MOV32: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1301`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1301)*

Thumb: MOVW/MOVT (deprecated)

### `IMAGE_REL_ARM_BRANCH20T`
```rust
const IMAGE_REL_ARM_BRANCH20T: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1303`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1303)*

Thumb: 32-bit conditional B

### `IMAGE_REL_THUMB_BRANCH20`
```rust
const IMAGE_REL_THUMB_BRANCH20: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1305`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1305)*

Thumb: 32-bit conditional B (deprecated)

### `IMAGE_REL_ARM_BRANCH24T`
```rust
const IMAGE_REL_ARM_BRANCH24T: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1307`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1307)*

Thumb: 32-bit B or BL

### `IMAGE_REL_THUMB_BRANCH24`
```rust
const IMAGE_REL_THUMB_BRANCH24: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1309`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1309)*

Thumb: 32-bit B or BL (deprecated)

### `IMAGE_REL_ARM_BLX23T`
```rust
const IMAGE_REL_ARM_BLX23T: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1311`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1311)*

Thumb: BLX immediate

### `IMAGE_REL_THUMB_BLX23`
```rust
const IMAGE_REL_THUMB_BLX23: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1313`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1313)*

Thumb: BLX immediate (deprecated)

### `IMAGE_REL_AM_ABSOLUTE`
```rust
const IMAGE_REL_AM_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1315`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1315)*

### `IMAGE_REL_AM_ADDR32`
```rust
const IMAGE_REL_AM_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1316`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1316)*

### `IMAGE_REL_AM_ADDR32NB`
```rust
const IMAGE_REL_AM_ADDR32NB: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1317`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1317)*

### `IMAGE_REL_AM_CALL32`
```rust
const IMAGE_REL_AM_CALL32: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1318`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1318)*

### `IMAGE_REL_AM_FUNCINFO`
```rust
const IMAGE_REL_AM_FUNCINFO: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1319`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1319)*

### `IMAGE_REL_AM_REL32_1`
```rust
const IMAGE_REL_AM_REL32_1: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1320`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1320)*

### `IMAGE_REL_AM_REL32_2`
```rust
const IMAGE_REL_AM_REL32_2: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1321`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1321)*

### `IMAGE_REL_AM_SECREL`
```rust
const IMAGE_REL_AM_SECREL: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1322`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1322)*

### `IMAGE_REL_AM_SECTION`
```rust
const IMAGE_REL_AM_SECTION: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1323`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1323)*

### `IMAGE_REL_AM_TOKEN`
```rust
const IMAGE_REL_AM_TOKEN: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1324`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1324)*

### `IMAGE_REL_ARM64_ABSOLUTE`
```rust
const IMAGE_REL_ARM64_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1331`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1331)*

No relocation required

### `IMAGE_REL_ARM64_ADDR32`
```rust
const IMAGE_REL_ARM64_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1333`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1333)*

32 bit address. Review! do we need it?

### `IMAGE_REL_ARM64_ADDR32NB`
```rust
const IMAGE_REL_ARM64_ADDR32NB: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1335`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1335)*

32 bit address w/o image base (RVA: for Data/PData/XData)

### `IMAGE_REL_ARM64_BRANCH26`
```rust
const IMAGE_REL_ARM64_BRANCH26: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1337`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1337)*

26 bit offset << 2 & sign ext. for B & BL

### `IMAGE_REL_ARM64_PAGEBASE_REL21`
```rust
const IMAGE_REL_ARM64_PAGEBASE_REL21: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1339`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1339)*

ADRP

### `IMAGE_REL_ARM64_REL21`
```rust
const IMAGE_REL_ARM64_REL21: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1341`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1341)*

ADR

### `IMAGE_REL_ARM64_PAGEOFFSET_12A`
```rust
const IMAGE_REL_ARM64_PAGEOFFSET_12A: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1343`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1343)*

ADD/ADDS (immediate) with zero shift, for page offset

### `IMAGE_REL_ARM64_PAGEOFFSET_12L`
```rust
const IMAGE_REL_ARM64_PAGEOFFSET_12L: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1345`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1345)*

LDR (indexed, unsigned immediate), for page offset

### `IMAGE_REL_ARM64_SECREL`
```rust
const IMAGE_REL_ARM64_SECREL: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1347`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1347)*

Offset within section

### `IMAGE_REL_ARM64_SECREL_LOW12A`
```rust
const IMAGE_REL_ARM64_SECREL_LOW12A: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1349`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1349)*

ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset

### `IMAGE_REL_ARM64_SECREL_HIGH12A`
```rust
const IMAGE_REL_ARM64_SECREL_HIGH12A: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1351`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1351)*

ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset

### `IMAGE_REL_ARM64_SECREL_LOW12L`
```rust
const IMAGE_REL_ARM64_SECREL_LOW12L: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1353`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1353)*

LDR (indexed, unsigned immediate), for bit 0:11 of section offset

### `IMAGE_REL_ARM64_TOKEN`
```rust
const IMAGE_REL_ARM64_TOKEN: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1354`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1354)*

### `IMAGE_REL_ARM64_SECTION`
```rust
const IMAGE_REL_ARM64_SECTION: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1356`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1356)*

Section table index

### `IMAGE_REL_ARM64_ADDR64`
```rust
const IMAGE_REL_ARM64_ADDR64: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1358`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1358)*

64 bit address

### `IMAGE_REL_ARM64_BRANCH19`
```rust
const IMAGE_REL_ARM64_BRANCH19: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1360`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1360)*

19 bit offset << 2 & sign ext. for conditional B

### `IMAGE_REL_ARM64_BRANCH14`
```rust
const IMAGE_REL_ARM64_BRANCH14: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1362`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1362)*

TBZ/TBNZ

### `IMAGE_REL_ARM64_REL32`
```rust
const IMAGE_REL_ARM64_REL32: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1364`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1364)*

32-bit relative address from byte following reloc

### `IMAGE_REL_AMD64_ABSOLUTE`
```rust
const IMAGE_REL_AMD64_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1370`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1370)*

Reference is absolute, no relocation is necessary

### `IMAGE_REL_AMD64_ADDR64`
```rust
const IMAGE_REL_AMD64_ADDR64: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1372`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1372)*

64-bit address (VA).

### `IMAGE_REL_AMD64_ADDR32`
```rust
const IMAGE_REL_AMD64_ADDR32: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1374`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1374)*

32-bit address (VA).

### `IMAGE_REL_AMD64_ADDR32NB`
```rust
const IMAGE_REL_AMD64_ADDR32NB: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1376`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1376)*

32-bit address w/o image base (RVA).

### `IMAGE_REL_AMD64_REL32`
```rust
const IMAGE_REL_AMD64_REL32: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1378`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1378)*

32-bit relative address from byte following reloc

### `IMAGE_REL_AMD64_REL32_1`
```rust
const IMAGE_REL_AMD64_REL32_1: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1380`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1380)*

32-bit relative address from byte distance 1 from reloc

### `IMAGE_REL_AMD64_REL32_2`
```rust
const IMAGE_REL_AMD64_REL32_2: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1382`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1382)*

32-bit relative address from byte distance 2 from reloc

### `IMAGE_REL_AMD64_REL32_3`
```rust
const IMAGE_REL_AMD64_REL32_3: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1384`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1384)*

32-bit relative address from byte distance 3 from reloc

### `IMAGE_REL_AMD64_REL32_4`
```rust
const IMAGE_REL_AMD64_REL32_4: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1386`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1386)*

32-bit relative address from byte distance 4 from reloc

### `IMAGE_REL_AMD64_REL32_5`
```rust
const IMAGE_REL_AMD64_REL32_5: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1388`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1388)*

32-bit relative address from byte distance 5 from reloc

### `IMAGE_REL_AMD64_SECTION`
```rust
const IMAGE_REL_AMD64_SECTION: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1390`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1390)*

Section index

### `IMAGE_REL_AMD64_SECREL`
```rust
const IMAGE_REL_AMD64_SECREL: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1392`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1392)*

32 bit offset from base of section containing target

### `IMAGE_REL_AMD64_SECREL7`
```rust
const IMAGE_REL_AMD64_SECREL7: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1394`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1394)*

7 bit unsigned offset from base of section containing target

### `IMAGE_REL_AMD64_TOKEN`
```rust
const IMAGE_REL_AMD64_TOKEN: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1396`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1396)*

32 bit metadata token

### `IMAGE_REL_AMD64_SREL32`
```rust
const IMAGE_REL_AMD64_SREL32: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1398`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1398)*

32 bit signed span-dependent value emitted into object

### `IMAGE_REL_AMD64_PAIR`
```rust
const IMAGE_REL_AMD64_PAIR: u16 = 15u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1399`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1399)*

### `IMAGE_REL_AMD64_SSPAN32`
```rust
const IMAGE_REL_AMD64_SSPAN32: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1401`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1401)*

32 bit signed span-dependent value applied at link time

### `IMAGE_REL_AMD64_EHANDLER`
```rust
const IMAGE_REL_AMD64_EHANDLER: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1402`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1402)*

### `IMAGE_REL_AMD64_IMPORT_BR`
```rust
const IMAGE_REL_AMD64_IMPORT_BR: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1404`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1404)*

Indirect branch to an import

### `IMAGE_REL_AMD64_IMPORT_CALL`
```rust
const IMAGE_REL_AMD64_IMPORT_CALL: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1406`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1406)*

Indirect call to an import

### `IMAGE_REL_AMD64_CFG_BR`
```rust
const IMAGE_REL_AMD64_CFG_BR: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1408`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1408)*

Indirect branch to a CFG check

### `IMAGE_REL_AMD64_CFG_BR_REX`
```rust
const IMAGE_REL_AMD64_CFG_BR_REX: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1410`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1410)*

Indirect branch to a CFG check, with REX.W prefix

### `IMAGE_REL_AMD64_CFG_CALL`
```rust
const IMAGE_REL_AMD64_CFG_CALL: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1412`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1412)*

Indirect call to a CFG check

### `IMAGE_REL_AMD64_INDIR_BR`
```rust
const IMAGE_REL_AMD64_INDIR_BR: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1414`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1414)*

Indirect branch to a target in RAX (no CFG)

### `IMAGE_REL_AMD64_INDIR_BR_REX`
```rust
const IMAGE_REL_AMD64_INDIR_BR_REX: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1416`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1416)*

Indirect branch to a target in RAX, with REX.W prefix (no CFG)

### `IMAGE_REL_AMD64_INDIR_CALL`
```rust
const IMAGE_REL_AMD64_INDIR_CALL: u16 = 25u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1418`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1418)*

Indirect call to a target in RAX (no CFG)

### `IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST`
```rust
const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1420`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1420)*

Indirect branch for a switch table using Reg 0 (RAX)

### `IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST`
```rust
const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST: u16 = 47u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1422`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1422)*

Indirect branch for a switch table using Reg 15 (R15)

### `IMAGE_REL_IA64_ABSOLUTE`
```rust
const IMAGE_REL_IA64_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1427`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1427)*

### `IMAGE_REL_IA64_IMM14`
```rust
const IMAGE_REL_IA64_IMM14: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1428`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1428)*

### `IMAGE_REL_IA64_IMM22`
```rust
const IMAGE_REL_IA64_IMM22: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1429`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1429)*

### `IMAGE_REL_IA64_IMM64`
```rust
const IMAGE_REL_IA64_IMM64: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1430`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1430)*

### `IMAGE_REL_IA64_DIR32`
```rust
const IMAGE_REL_IA64_DIR32: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1431`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1431)*

### `IMAGE_REL_IA64_DIR64`
```rust
const IMAGE_REL_IA64_DIR64: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1432`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1432)*

### `IMAGE_REL_IA64_PCREL21B`
```rust
const IMAGE_REL_IA64_PCREL21B: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1433`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1433)*

### `IMAGE_REL_IA64_PCREL21M`
```rust
const IMAGE_REL_IA64_PCREL21M: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1434`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1434)*

### `IMAGE_REL_IA64_PCREL21F`
```rust
const IMAGE_REL_IA64_PCREL21F: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1435`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1435)*

### `IMAGE_REL_IA64_GPREL22`
```rust
const IMAGE_REL_IA64_GPREL22: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1436`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1436)*

### `IMAGE_REL_IA64_LTOFF22`
```rust
const IMAGE_REL_IA64_LTOFF22: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1437`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1437)*

### `IMAGE_REL_IA64_SECTION`
```rust
const IMAGE_REL_IA64_SECTION: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1438`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1438)*

### `IMAGE_REL_IA64_SECREL22`
```rust
const IMAGE_REL_IA64_SECREL22: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1439`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1439)*

### `IMAGE_REL_IA64_SECREL64I`
```rust
const IMAGE_REL_IA64_SECREL64I: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1440`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1440)*

### `IMAGE_REL_IA64_SECREL32`
```rust
const IMAGE_REL_IA64_SECREL32: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1441`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1441)*

### `IMAGE_REL_IA64_DIR32NB`
```rust
const IMAGE_REL_IA64_DIR32NB: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1443`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1443)*

### `IMAGE_REL_IA64_SREL14`
```rust
const IMAGE_REL_IA64_SREL14: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1444`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1444)*

### `IMAGE_REL_IA64_SREL22`
```rust
const IMAGE_REL_IA64_SREL22: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1445`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1445)*

### `IMAGE_REL_IA64_SREL32`
```rust
const IMAGE_REL_IA64_SREL32: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1446`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1446)*

### `IMAGE_REL_IA64_UREL32`
```rust
const IMAGE_REL_IA64_UREL32: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1447`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1447)*

### `IMAGE_REL_IA64_PCREL60X`
```rust
const IMAGE_REL_IA64_PCREL60X: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1449`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1449)*

This is always a BRL and never converted

### `IMAGE_REL_IA64_PCREL60B`
```rust
const IMAGE_REL_IA64_PCREL60B: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1451`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1451)*

If possible, convert to MBB bundle with NOP.B in slot 1

### `IMAGE_REL_IA64_PCREL60F`
```rust
const IMAGE_REL_IA64_PCREL60F: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1453`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1453)*

If possible, convert to MFB bundle with NOP.F in slot 1

### `IMAGE_REL_IA64_PCREL60I`
```rust
const IMAGE_REL_IA64_PCREL60I: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1455`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1455)*

If possible, convert to MIB bundle with NOP.I in slot 1

### `IMAGE_REL_IA64_PCREL60M`
```rust
const IMAGE_REL_IA64_PCREL60M: u16 = 25u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1457`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1457)*

If possible, convert to MMB bundle with NOP.M in slot 1

### `IMAGE_REL_IA64_IMMGPREL64`
```rust
const IMAGE_REL_IA64_IMMGPREL64: u16 = 26u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1458`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1458)*

### `IMAGE_REL_IA64_TOKEN`
```rust
const IMAGE_REL_IA64_TOKEN: u16 = 27u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1460`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1460)*

clr token

### `IMAGE_REL_IA64_GPREL32`
```rust
const IMAGE_REL_IA64_GPREL32: u16 = 28u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1461`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1461)*

### `IMAGE_REL_IA64_ADDEND`
```rust
const IMAGE_REL_IA64_ADDEND: u16 = 31u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1462`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1462)*

### `IMAGE_REL_CEF_ABSOLUTE`
```rust
const IMAGE_REL_CEF_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1468`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1468)*

Reference is absolute, no relocation is necessary

### `IMAGE_REL_CEF_ADDR32`
```rust
const IMAGE_REL_CEF_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1470`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1470)*

32-bit address (VA).

### `IMAGE_REL_CEF_ADDR64`
```rust
const IMAGE_REL_CEF_ADDR64: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1472`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1472)*

64-bit address (VA).

### `IMAGE_REL_CEF_ADDR32NB`
```rust
const IMAGE_REL_CEF_ADDR32NB: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1474`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1474)*

32-bit address w/o image base (RVA).

### `IMAGE_REL_CEF_SECTION`
```rust
const IMAGE_REL_CEF_SECTION: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1476`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1476)*

Section index

### `IMAGE_REL_CEF_SECREL`
```rust
const IMAGE_REL_CEF_SECREL: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1478`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1478)*

32 bit offset from base of section containing target

### `IMAGE_REL_CEF_TOKEN`
```rust
const IMAGE_REL_CEF_TOKEN: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1480`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1480)*

32 bit metadata token

### `IMAGE_REL_CEE_ABSOLUTE`
```rust
const IMAGE_REL_CEE_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1486`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1486)*

Reference is absolute, no relocation is necessary

### `IMAGE_REL_CEE_ADDR32`
```rust
const IMAGE_REL_CEE_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1488`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1488)*

32-bit address (VA).

### `IMAGE_REL_CEE_ADDR64`
```rust
const IMAGE_REL_CEE_ADDR64: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1490`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1490)*

64-bit address (VA).

### `IMAGE_REL_CEE_ADDR32NB`
```rust
const IMAGE_REL_CEE_ADDR32NB: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1492`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1492)*

32-bit address w/o image base (RVA).

### `IMAGE_REL_CEE_SECTION`
```rust
const IMAGE_REL_CEE_SECTION: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1494`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1494)*

Section index

### `IMAGE_REL_CEE_SECREL`
```rust
const IMAGE_REL_CEE_SECREL: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1496`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1496)*

32 bit offset from base of section containing target

### `IMAGE_REL_CEE_TOKEN`
```rust
const IMAGE_REL_CEE_TOKEN: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1498`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1498)*

32 bit metadata token

### `IMAGE_REL_M32R_ABSOLUTE`
```rust
const IMAGE_REL_M32R_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1501`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1501)*

No relocation required

### `IMAGE_REL_M32R_ADDR32`
```rust
const IMAGE_REL_M32R_ADDR32: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1503`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1503)*

32 bit address

### `IMAGE_REL_M32R_ADDR32NB`
```rust
const IMAGE_REL_M32R_ADDR32NB: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1505`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1505)*

32 bit address w/o image base

### `IMAGE_REL_M32R_ADDR24`
```rust
const IMAGE_REL_M32R_ADDR24: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1507`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1507)*

24 bit address

### `IMAGE_REL_M32R_GPREL16`
```rust
const IMAGE_REL_M32R_GPREL16: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1509`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1509)*

GP relative addressing

### `IMAGE_REL_M32R_PCREL24`
```rust
const IMAGE_REL_M32R_PCREL24: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1511`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1511)*

24 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_PCREL16`
```rust
const IMAGE_REL_M32R_PCREL16: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1513`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1513)*

16 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_PCREL8`
```rust
const IMAGE_REL_M32R_PCREL8: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1515`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1515)*

8 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_REFHALF`
```rust
const IMAGE_REL_M32R_REFHALF: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1517`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1517)*

16 MSBs

### `IMAGE_REL_M32R_REFHI`
```rust
const IMAGE_REL_M32R_REFHI: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1519`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1519)*

16 MSBs; adj for LSB sign ext.

### `IMAGE_REL_M32R_REFLO`
```rust
const IMAGE_REL_M32R_REFLO: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1521`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1521)*

16 LSBs

### `IMAGE_REL_M32R_PAIR`
```rust
const IMAGE_REL_M32R_PAIR: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1523`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1523)*

Link HI and LO

### `IMAGE_REL_M32R_SECTION`
```rust
const IMAGE_REL_M32R_SECTION: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1525`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1525)*

Section table index

### `IMAGE_REL_M32R_SECREL32`
```rust
const IMAGE_REL_M32R_SECREL32: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1527`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1527)*

32 bit section relative reference

### `IMAGE_REL_M32R_TOKEN`
```rust
const IMAGE_REL_M32R_TOKEN: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1529`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1529)*

clr token

### `IMAGE_REL_EBC_ABSOLUTE`
```rust
const IMAGE_REL_EBC_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1532`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1532)*

No relocation required

### `IMAGE_REL_EBC_ADDR32NB`
```rust
const IMAGE_REL_EBC_ADDR32NB: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1534`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1534)*

32 bit address w/o image base

### `IMAGE_REL_EBC_REL32`
```rust
const IMAGE_REL_EBC_REL32: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1536`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1536)*

32-bit relative address from byte following reloc

### `IMAGE_REL_EBC_SECTION`
```rust
const IMAGE_REL_EBC_SECTION: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1538`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1538)*

Section table index

### `IMAGE_REL_EBC_SECREL`
```rust
const IMAGE_REL_EBC_SECREL: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1540`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1540)*

Offset within section

### `EMARCH_ENC_I17_IMM7B_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM7B_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1553`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1553)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM7B_SIZE_X: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1555`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1555)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1557`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1557)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM7B_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1559`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1559)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM9D_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1562`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1562)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM9D_SIZE_X: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1564`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1564)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: u16 = 18u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1566`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1566)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM9D_VAL_POS_X: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1568`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1568)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM5C_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1571`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1571)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM5C_SIZE_X: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1573`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1573)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: u16 = 13u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1575`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1575)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM5C_VAL_POS_X: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1577`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1577)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IC_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1580`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1580)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_SIZE_X`
```rust
const EMARCH_ENC_I17_IC_SIZE_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1582`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1582)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IC_INST_WORD_POS_X: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1584`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1584)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IC_VAL_POS_X: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1586`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1586)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM41A_INST_WORD_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1589`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1589)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM41A_SIZE_X: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1591`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1591)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1593`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1593)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM41A_VAL_POS_X: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1595`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1595)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM41B_INST_WORD_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1598`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1598)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM41B_SIZE_X: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1600`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1600)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1602`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1602)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM41B_VAL_POS_X: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1604`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1604)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_INST_WORD_X`
```rust
const EMARCH_ENC_I17_IMM41C_INST_WORD_X: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1607`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1607)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_SIZE_X`
```rust
const EMARCH_ENC_I17_IMM41C_SIZE_X: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1609`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1609)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1611`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1611)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_VAL_POS_X`
```rust
const EMARCH_ENC_I17_IMM41C_VAL_POS_X: u16 = 40u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1613`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1613)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_INST_WORD_X`
```rust
const EMARCH_ENC_I17_SIGN_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1616`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1616)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_SIZE_X`
```rust
const EMARCH_ENC_I17_SIGN_SIZE_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1618`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1618)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_INST_WORD_POS_X`
```rust
const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: u16 = 27u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1620`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1620)*

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_VAL_POS_X`
```rust
const EMARCH_ENC_I17_SIGN_VAL_POS_X: u16 = 63u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1622`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1622)*

Intel-IA64-Filler

### `X3_OPCODE_INST_WORD_X`
```rust
const X3_OPCODE_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1625`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1625)*

Intel-IA64-Filler

### `X3_OPCODE_SIZE_X`
```rust
const X3_OPCODE_SIZE_X: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1627`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1627)*

Intel-IA64-Filler

### `X3_OPCODE_INST_WORD_POS_X`
```rust
const X3_OPCODE_INST_WORD_POS_X: u16 = 28u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1629`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1629)*

Intel-IA64-Filler

### `X3_OPCODE_SIGN_VAL_POS_X`
```rust
const X3_OPCODE_SIGN_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1631`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1631)*

Intel-IA64-Filler

### `X3_I_INST_WORD_X`
```rust
const X3_I_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1634`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1634)*

Intel-IA64-Filler

### `X3_I_SIZE_X`
```rust
const X3_I_SIZE_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1636`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1636)*

Intel-IA64-Filler

### `X3_I_INST_WORD_POS_X`
```rust
const X3_I_INST_WORD_POS_X: u16 = 27u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1638`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1638)*

Intel-IA64-Filler

### `X3_I_SIGN_VAL_POS_X`
```rust
const X3_I_SIGN_VAL_POS_X: u16 = 59u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1640`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1640)*

Intel-IA64-Filler

### `X3_D_WH_INST_WORD_X`
```rust
const X3_D_WH_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1643`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1643)*

Intel-IA64-Filler

### `X3_D_WH_SIZE_X`
```rust
const X3_D_WH_SIZE_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1645`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1645)*

Intel-IA64-Filler

### `X3_D_WH_INST_WORD_POS_X`
```rust
const X3_D_WH_INST_WORD_POS_X: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1647`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1647)*

Intel-IA64-Filler

### `X3_D_WH_SIGN_VAL_POS_X`
```rust
const X3_D_WH_SIGN_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1649`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1649)*

Intel-IA64-Filler

### `X3_IMM20_INST_WORD_X`
```rust
const X3_IMM20_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1652`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1652)*

Intel-IA64-Filler

### `X3_IMM20_SIZE_X`
```rust
const X3_IMM20_SIZE_X: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1654`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1654)*

Intel-IA64-Filler

### `X3_IMM20_INST_WORD_POS_X`
```rust
const X3_IMM20_INST_WORD_POS_X: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1656`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1656)*

Intel-IA64-Filler

### `X3_IMM20_SIGN_VAL_POS_X`
```rust
const X3_IMM20_SIGN_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1658`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1658)*

Intel-IA64-Filler

### `X3_IMM39_1_INST_WORD_X`
```rust
const X3_IMM39_1_INST_WORD_X: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1661`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1661)*

Intel-IA64-Filler

### `X3_IMM39_1_SIZE_X`
```rust
const X3_IMM39_1_SIZE_X: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1663`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1663)*

Intel-IA64-Filler

### `X3_IMM39_1_INST_WORD_POS_X`
```rust
const X3_IMM39_1_INST_WORD_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1665`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1665)*

Intel-IA64-Filler

### `X3_IMM39_1_SIGN_VAL_POS_X`
```rust
const X3_IMM39_1_SIGN_VAL_POS_X: u16 = 36u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1667`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1667)*

Intel-IA64-Filler

### `X3_IMM39_2_INST_WORD_X`
```rust
const X3_IMM39_2_INST_WORD_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1670`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1670)*

Intel-IA64-Filler

### `X3_IMM39_2_SIZE_X`
```rust
const X3_IMM39_2_SIZE_X: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1672`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1672)*

Intel-IA64-Filler

### `X3_IMM39_2_INST_WORD_POS_X`
```rust
const X3_IMM39_2_INST_WORD_POS_X: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1674`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1674)*

Intel-IA64-Filler

### `X3_IMM39_2_SIGN_VAL_POS_X`
```rust
const X3_IMM39_2_SIGN_VAL_POS_X: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1676`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1676)*

Intel-IA64-Filler

### `X3_P_INST_WORD_X`
```rust
const X3_P_INST_WORD_X: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1679`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1679)*

Intel-IA64-Filler

### `X3_P_SIZE_X`
```rust
const X3_P_SIZE_X: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1681`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1681)*

Intel-IA64-Filler

### `X3_P_INST_WORD_POS_X`
```rust
const X3_P_INST_WORD_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1683`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1683)*

Intel-IA64-Filler

### `X3_P_SIGN_VAL_POS_X`
```rust
const X3_P_SIGN_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1685`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1685)*

Intel-IA64-Filler

### `X3_TMPLT_INST_WORD_X`
```rust
const X3_TMPLT_INST_WORD_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1688`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1688)*

Intel-IA64-Filler

### `X3_TMPLT_SIZE_X`
```rust
const X3_TMPLT_SIZE_X: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1690`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1690)*

Intel-IA64-Filler

### `X3_TMPLT_INST_WORD_POS_X`
```rust
const X3_TMPLT_INST_WORD_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1692`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1692)*

Intel-IA64-Filler

### `X3_TMPLT_SIGN_VAL_POS_X`
```rust
const X3_TMPLT_SIGN_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1694`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1694)*

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_WORD_X`
```rust
const X3_BTYPE_QP_INST_WORD_X: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1697`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1697)*

Intel-IA64-Filler

### `X3_BTYPE_QP_SIZE_X`
```rust
const X3_BTYPE_QP_SIZE_X: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1699`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1699)*

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_WORD_POS_X`
```rust
const X3_BTYPE_QP_INST_WORD_POS_X: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1701`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1701)*

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_VAL_POS_X`
```rust
const X3_BTYPE_QP_INST_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1703`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1703)*

Intel-IA64-Filler

### `X3_EMPTY_INST_WORD_X`
```rust
const X3_EMPTY_INST_WORD_X: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1706`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1706)*

Intel-IA64-Filler

### `X3_EMPTY_SIZE_X`
```rust
const X3_EMPTY_SIZE_X: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1708`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1708)*

Intel-IA64-Filler

### `X3_EMPTY_INST_WORD_POS_X`
```rust
const X3_EMPTY_INST_WORD_POS_X: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1710`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1710)*

Intel-IA64-Filler

### `X3_EMPTY_INST_VAL_POS_X`
```rust
const X3_EMPTY_INST_VAL_POS_X: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1712`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1712)*

Intel-IA64-Filler

### `IMAGE_REL_BASED_ABSOLUTE`
```rust
const IMAGE_REL_BASED_ABSOLUTE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1745`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1745)*

### `IMAGE_REL_BASED_HIGH`
```rust
const IMAGE_REL_BASED_HIGH: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1746`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1746)*

### `IMAGE_REL_BASED_LOW`
```rust
const IMAGE_REL_BASED_LOW: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1747`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1747)*

### `IMAGE_REL_BASED_HIGHLOW`
```rust
const IMAGE_REL_BASED_HIGHLOW: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1748`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1748)*

### `IMAGE_REL_BASED_HIGHADJ`
```rust
const IMAGE_REL_BASED_HIGHADJ: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1749`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1749)*

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_5`
```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1750`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1750)*

### `IMAGE_REL_BASED_RESERVED`
```rust
const IMAGE_REL_BASED_RESERVED: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1751`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1751)*

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_7`
```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1752`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1752)*

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_8`
```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1753`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1753)*

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_9`
```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1754`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1754)*

### `IMAGE_REL_BASED_DIR64`
```rust
const IMAGE_REL_BASED_DIR64: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1755`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1755)*

### `IMAGE_REL_BASED_IA64_IMM64`
```rust
const IMAGE_REL_BASED_IA64_IMM64: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1761`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1761)*

### `IMAGE_REL_BASED_MIPS_JMPADDR`
```rust
const IMAGE_REL_BASED_MIPS_JMPADDR: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1763`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1763)*

### `IMAGE_REL_BASED_MIPS_JMPADDR16`
```rust
const IMAGE_REL_BASED_MIPS_JMPADDR16: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1764`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1764)*

### `IMAGE_REL_BASED_ARM_MOV32`
```rust
const IMAGE_REL_BASED_ARM_MOV32: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1766`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1766)*

### `IMAGE_REL_BASED_THUMB_MOV32`
```rust
const IMAGE_REL_BASED_THUMB_MOV32: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1767`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1767)*

### `IMAGE_REL_BASED_RISCV_HIGH20`
```rust
const IMAGE_REL_BASED_RISCV_HIGH20: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1769`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1769)*

### `IMAGE_REL_BASED_RISCV_LOW12I`
```rust
const IMAGE_REL_BASED_RISCV_LOW12I: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1770`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1770)*

### `IMAGE_REL_BASED_RISCV_LOW12S`
```rust
const IMAGE_REL_BASED_RISCV_LOW12S: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1771`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1771)*

### `IMAGE_ARCHIVE_START_SIZE`
```rust
const IMAGE_ARCHIVE_START_SIZE: usize = 8usize;
```

*Defined in [`object-0.37.3/src/pe.rs:1777`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1777)*

### `IMAGE_ARCHIVE_START`
```rust
const IMAGE_ARCHIVE_START: &[u8; 8];
```

*Defined in [`object-0.37.3/src/pe.rs:1778`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1778)*

### `IMAGE_ARCHIVE_END`
```rust
const IMAGE_ARCHIVE_END: &[u8];
```

*Defined in [`object-0.37.3/src/pe.rs:1779`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1779)*

### `IMAGE_ARCHIVE_PAD`
```rust
const IMAGE_ARCHIVE_PAD: &[u8];
```

*Defined in [`object-0.37.3/src/pe.rs:1780`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1780)*

### `IMAGE_ARCHIVE_LINKER_MEMBER`
```rust
const IMAGE_ARCHIVE_LINKER_MEMBER: &[u8; 16];
```

*Defined in [`object-0.37.3/src/pe.rs:1781`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1781)*

### `IMAGE_ARCHIVE_LONGNAMES_MEMBER`
```rust
const IMAGE_ARCHIVE_LONGNAMES_MEMBER: &[u8; 16];
```

*Defined in [`object-0.37.3/src/pe.rs:1782`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1782)*

### `IMAGE_ARCHIVE_HYBRIDMAP_MEMBER`
```rust
const IMAGE_ARCHIVE_HYBRIDMAP_MEMBER: &[u8; 16];
```

*Defined in [`object-0.37.3/src/pe.rs:1783`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1783)*

### `IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR`
```rust
const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: u16 = 60u16;
```

*Defined in [`object-0.37.3/src/pe.rs:1804`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1804)*

### `IMAGE_ORDINAL_FLAG64`
```rust
const IMAGE_ORDINAL_FLAG64: u64 = 9_223_372_036_854_775_808u64;
```

*Defined in [`object-0.37.3/src/pe.rs:1875`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1875)*

### `IMAGE_ORDINAL_FLAG32`
```rust
const IMAGE_ORDINAL_FLAG32: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:1876`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L1876)*

### `IMAGE_DELAYLOAD_RVA_BASED`
```rust
const IMAGE_DELAYLOAD_RVA_BASED: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2004`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2004)*

Delay load version 2 flag for `ImageDelayloadDescriptor::attributes`.

### `IMAGE_RESOURCE_NAME_IS_STRING`
```rust
const IMAGE_RESOURCE_NAME_IS_STRING: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2035`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2035)*

### `IMAGE_RESOURCE_DATA_IS_DIRECTORY`
```rust
const IMAGE_RESOURCE_DATA_IS_DIRECTORY: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2036`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2036)*

### `RT_CURSOR`
```rust
const RT_CURSOR: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2104`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2104)*

ID for: Hardware-dependent cursor resource.

### `RT_BITMAP`
```rust
const RT_BITMAP: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2106`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2106)*

ID for: Bitmap resource.

### `RT_ICON`
```rust
const RT_ICON: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2108`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2108)*

ID for: Hardware-dependent icon resource.

### `RT_MENU`
```rust
const RT_MENU: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2110`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2110)*

ID for: Menu resource.

### `RT_DIALOG`
```rust
const RT_DIALOG: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2112`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2112)*

ID for: Dialog box.

### `RT_STRING`
```rust
const RT_STRING: u16 = 6u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2114`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2114)*

ID for: String-table entry.

### `RT_FONTDIR`
```rust
const RT_FONTDIR: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2116`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2116)*

ID for: Font directory resource.

### `RT_FONT`
```rust
const RT_FONT: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2118`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2118)*

ID for: Font resource.

### `RT_ACCELERATOR`
```rust
const RT_ACCELERATOR: u16 = 9u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2120`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2120)*

ID for: Accelerator table.

### `RT_RCDATA`
```rust
const RT_RCDATA: u16 = 10u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2122`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2122)*

ID for: Application-defined resource (raw data).

### `RT_MESSAGETABLE`
```rust
const RT_MESSAGETABLE: u16 = 11u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2124`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2124)*

ID for: Message-table entry.

### `RT_GROUP_CURSOR`
```rust
const RT_GROUP_CURSOR: u16 = 12u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2126`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2126)*

ID for: Hardware-independent cursor resource.

### `RT_GROUP_ICON`
```rust
const RT_GROUP_ICON: u16 = 14u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2128`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2128)*

ID for: Hardware-independent icon resource.

### `RT_VERSION`
```rust
const RT_VERSION: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2130`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2130)*

ID for: Version resource.

### `RT_DLGINCLUDE`
```rust
const RT_DLGINCLUDE: u16 = 17u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2132`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2132)*

ID for: Allows a resource editing tool to associate a string with an .rc file.

### `RT_PLUGPLAY`
```rust
const RT_PLUGPLAY: u16 = 19u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2134`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2134)*

ID for: Plug and Play resource.

### `RT_VXD`
```rust
const RT_VXD: u16 = 20u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2136`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2136)*

ID for: VXD.

### `RT_ANICURSOR`
```rust
const RT_ANICURSOR: u16 = 21u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2138`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2138)*

ID for: Animated cursor.

### `RT_ANIICON`
```rust
const RT_ANIICON: u16 = 22u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2140`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2140)*

ID for: Animated icon.

### `RT_HTML`
```rust
const RT_HTML: u16 = 23u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2142`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2142)*

ID for: HTML resource.

### `RT_MANIFEST`
```rust
const RT_MANIFEST: u16 = 24u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2144`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2144)*

ID for: Side-by-Side Assembly Manifest.

### `IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE`
```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2222`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2222)*

### `IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE`
```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2223`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2223)*

### `IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER`
```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2224`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2224)*

### `IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER`
```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2225`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2225)*

### `IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH`
```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2226`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2226)*

### `IMAGE_HOT_PATCH_BASE_OBLIGATORY`
```rust
const IMAGE_HOT_PATCH_BASE_OBLIGATORY: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2443`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2443)*

### `IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK`
```rust
const IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2444`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2444)*

### `IMAGE_HOT_PATCH_CHUNK_INVERSE`
```rust
const IMAGE_HOT_PATCH_CHUNK_INVERSE: u32 = 2_147_483_648u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2446`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2446)*

### `IMAGE_HOT_PATCH_CHUNK_OBLIGATORY`
```rust
const IMAGE_HOT_PATCH_CHUNK_OBLIGATORY: u32 = 1_073_741_824u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2447`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2447)*

### `IMAGE_HOT_PATCH_CHUNK_RESERVED`
```rust
const IMAGE_HOT_PATCH_CHUNK_RESERVED: u32 = 1_072_705_536u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2448`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2448)*

### `IMAGE_HOT_PATCH_CHUNK_TYPE`
```rust
const IMAGE_HOT_PATCH_CHUNK_TYPE: u32 = 1_032_192u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2449`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2449)*

### `IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA`
```rust
const IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA: u32 = 32_768u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2450`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2450)*

### `IMAGE_HOT_PATCH_CHUNK_TARGET_RVA`
```rust
const IMAGE_HOT_PATCH_CHUNK_TARGET_RVA: u32 = 16_384u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2451`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2451)*

### `IMAGE_HOT_PATCH_CHUNK_SIZE`
```rust
const IMAGE_HOT_PATCH_CHUNK_SIZE: u32 = 4_095u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2452`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2452)*

### `IMAGE_HOT_PATCH_NONE`
```rust
const IMAGE_HOT_PATCH_NONE: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2454`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2454)*

### `IMAGE_HOT_PATCH_FUNCTION`
```rust
const IMAGE_HOT_PATCH_FUNCTION: u32 = 114_688u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2455`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2455)*

### `IMAGE_HOT_PATCH_ABSOLUTE`
```rust
const IMAGE_HOT_PATCH_ABSOLUTE: u32 = 180_224u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2456`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2456)*

### `IMAGE_HOT_PATCH_REL32`
```rust
const IMAGE_HOT_PATCH_REL32: u32 = 245_760u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2457`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2457)*

### `IMAGE_HOT_PATCH_CALL_TARGET`
```rust
const IMAGE_HOT_PATCH_CALL_TARGET: u32 = 278_528u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2458`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2458)*

### `IMAGE_HOT_PATCH_INDIRECT`
```rust
const IMAGE_HOT_PATCH_INDIRECT: u32 = 376_832u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2459`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2459)*

### `IMAGE_HOT_PATCH_NO_CALL_TARGET`
```rust
const IMAGE_HOT_PATCH_NO_CALL_TARGET: u32 = 409_600u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2460`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2460)*

### `IMAGE_HOT_PATCH_DYNAMIC_VALUE`
```rust
const IMAGE_HOT_PATCH_DYNAMIC_VALUE: u32 = 491_520u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2461`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2461)*

### `IMAGE_GUARD_CF_INSTRUMENTED`
```rust
const IMAGE_GUARD_CF_INSTRUMENTED: u32 = 256u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2464`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2464)*

Module performs control flow integrity checks using system-supplied support

### `IMAGE_GUARD_CFW_INSTRUMENTED`
```rust
const IMAGE_GUARD_CFW_INSTRUMENTED: u32 = 512u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2466`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2466)*

Module performs control flow and write integrity checks

### `IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT`
```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 1_024u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2468`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2468)*

Module contains valid control flow target metadata

### `IMAGE_GUARD_SECURITY_COOKIE_UNUSED`
```rust
const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32 = 2_048u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2470`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2470)*

Module does not make use of the /GS security cookie

### `IMAGE_GUARD_PROTECT_DELAYLOAD_IAT`
```rust
const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32 = 4_096u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2472`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2472)*

Module supports read only delay load IAT

### `IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION`
```rust
const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32 = 8_192u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2474`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2474)*

Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected

### `IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT`
```rust
const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 16_384u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2478`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2478)*

Module contains suppressed export information.

This also infers that the address taken taken IAT table is also present in the load config.

### `IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION`
```rust
const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32 = 32_768u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2480`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2480)*

Module enables suppression of exports

### `IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT`
```rust
const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 65_536u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2482`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2482)*

Module contains longjmp target information

### `IMAGE_GUARD_RF_INSTRUMENTED`
```rust
const IMAGE_GUARD_RF_INSTRUMENTED: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2484`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2484)*

Module contains return flow instrumentation and metadata

### `IMAGE_GUARD_RF_ENABLE`
```rust
const IMAGE_GUARD_RF_ENABLE: u32 = 262_144u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2486`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2486)*

Module requests that the OS enable return flow protection

### `IMAGE_GUARD_RF_STRICT`
```rust
const IMAGE_GUARD_RF_STRICT: u32 = 524_288u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2488`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2488)*

Module requests that the OS enable return flow protection in strict mode

### `IMAGE_GUARD_RETPOLINE_PRESENT`
```rust
const IMAGE_GUARD_RETPOLINE_PRESENT: u32 = 1_048_576u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2490`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2490)*

Module was built with retpoline support

### `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK`
```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32 = 4_026_531_840u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2493`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2493)*

Stride of Guard CF function table encoded in these bits (additional count of bytes per element)

### `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT`
```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2495`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2495)*

Shift to right-justify Guard CF function table stride

### `IMAGE_GUARD_FLAG_FID_SUPPRESSED`
```rust
const IMAGE_GUARD_FLAG_FID_SUPPRESSED: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2502`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2502)*

The containing GFID entry is suppressed

### `IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED`
```rust
const IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2504`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2504)*

The containing GFID entry is export suppressed

### `IMAGE_ENCLAVE_LONG_ID_LENGTH`
```rust
const IMAGE_ENCLAVE_LONG_ID_LENGTH: usize = 32usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2574`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2574)*

### `IMAGE_ENCLAVE_SHORT_ID_LENGTH`
```rust
const IMAGE_ENCLAVE_SHORT_ID_LENGTH: usize = 16usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2575`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2575)*

### `IMAGE_ENCLAVE_POLICY_DEBUGGABLE`
```rust
const IMAGE_ENCLAVE_POLICY_DEBUGGABLE: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2615`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2615)*

### `IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE`
```rust
const IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2617`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2617)*

### `IMAGE_ENCLAVE_IMPORT_MATCH_NONE`
```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_NONE: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2631`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2631)*

### `IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID`
```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2632`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2632)*

### `IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID`
```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2633`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2633)*

### `IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID`
```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2634`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2634)*

### `IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID`
```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2635`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2635)*

### `IMAGE_DEBUG_TYPE_UNKNOWN`
```rust
const IMAGE_DEBUG_TYPE_UNKNOWN: u32 = 0u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2654`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2654)*

### `IMAGE_DEBUG_TYPE_COFF`
```rust
const IMAGE_DEBUG_TYPE_COFF: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2655`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2655)*

### `IMAGE_DEBUG_TYPE_CODEVIEW`
```rust
const IMAGE_DEBUG_TYPE_CODEVIEW: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2656`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2656)*

### `IMAGE_DEBUG_TYPE_FPO`
```rust
const IMAGE_DEBUG_TYPE_FPO: u32 = 3u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2657`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2657)*

### `IMAGE_DEBUG_TYPE_MISC`
```rust
const IMAGE_DEBUG_TYPE_MISC: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2658`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2658)*

### `IMAGE_DEBUG_TYPE_EXCEPTION`
```rust
const IMAGE_DEBUG_TYPE_EXCEPTION: u32 = 5u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2659`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2659)*

### `IMAGE_DEBUG_TYPE_FIXUP`
```rust
const IMAGE_DEBUG_TYPE_FIXUP: u32 = 6u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2660`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2660)*

### `IMAGE_DEBUG_TYPE_OMAP_TO_SRC`
```rust
const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32 = 7u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2661`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2661)*

### `IMAGE_DEBUG_TYPE_OMAP_FROM_SRC`
```rust
const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2662`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2662)*

### `IMAGE_DEBUG_TYPE_BORLAND`
```rust
const IMAGE_DEBUG_TYPE_BORLAND: u32 = 9u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2663`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2663)*

### `IMAGE_DEBUG_TYPE_RESERVED10`
```rust
const IMAGE_DEBUG_TYPE_RESERVED10: u32 = 10u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2664`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2664)*

### `IMAGE_DEBUG_TYPE_CLSID`
```rust
const IMAGE_DEBUG_TYPE_CLSID: u32 = 11u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2665`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2665)*

### `IMAGE_DEBUG_TYPE_VC_FEATURE`
```rust
const IMAGE_DEBUG_TYPE_VC_FEATURE: u32 = 12u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2666`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2666)*

### `IMAGE_DEBUG_TYPE_POGO`
```rust
const IMAGE_DEBUG_TYPE_POGO: u32 = 13u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2667`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2667)*

### `IMAGE_DEBUG_TYPE_ILTCG`
```rust
const IMAGE_DEBUG_TYPE_ILTCG: u32 = 14u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2668`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2668)*

### `IMAGE_DEBUG_TYPE_MPX`
```rust
const IMAGE_DEBUG_TYPE_MPX: u32 = 15u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2669`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2669)*

### `IMAGE_DEBUG_TYPE_REPRO`
```rust
const IMAGE_DEBUG_TYPE_REPRO: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2670`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2670)*

### `FRAME_FPO`
```rust
const FRAME_FPO: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2685`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2685)*

### `FRAME_TRAP`
```rust
const FRAME_TRAP: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2686`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2686)*

### `FRAME_TSS`
```rust
const FRAME_TSS: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2687`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2687)*

### `FRAME_NONFPO`
```rust
const FRAME_NONFPO: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2688`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2688)*

### `IMAGE_DEBUG_MISC_EXENAME`
```rust
const IMAGE_DEBUG_MISC_EXENAME: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2719`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2719)*

### `IMAGE_SEPARATE_DEBUG_SIGNATURE`
```rust
const IMAGE_SEPARATE_DEBUG_SIGNATURE: u16 = 18_756u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2811`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2811)*

### `NON_PAGED_DEBUG_SIGNATURE`
```rust
const NON_PAGED_DEBUG_SIGNATURE: u16 = 18_766u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2812`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2812)*

### `IMAGE_SEPARATE_DEBUG_FLAGS_MASK`
```rust
const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2814`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2814)*

### `IMAGE_SEPARATE_DEBUG_MISMATCH`
```rust
const IMAGE_SEPARATE_DEBUG_MISMATCH: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2816`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2816)*

when DBG was updated, the old checksum didn't match.

### `IMPORT_OBJECT_HDR_SIG2`
```rust
const IMPORT_OBJECT_HDR_SIG2: u16 = 65_535u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2859`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2859)*

### `IMPORT_OBJECT_TYPE_MASK`
```rust
const IMPORT_OBJECT_TYPE_MASK: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2884`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2884)*

### `IMPORT_OBJECT_TYPE_SHIFT`
```rust
const IMPORT_OBJECT_TYPE_SHIFT: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2885`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2885)*

### `IMPORT_OBJECT_CODE`
```rust
const IMPORT_OBJECT_CODE: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2886`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2886)*

### `IMPORT_OBJECT_DATA`
```rust
const IMPORT_OBJECT_DATA: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2887`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2887)*

### `IMPORT_OBJECT_CONST`
```rust
const IMPORT_OBJECT_CONST: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2888`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2888)*

### `IMPORT_OBJECT_NAME_MASK`
```rust
const IMPORT_OBJECT_NAME_MASK: u16 = 7u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2890`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2890)*

### `IMPORT_OBJECT_NAME_SHIFT`
```rust
const IMPORT_OBJECT_NAME_SHIFT: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2891`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2891)*

### `IMPORT_OBJECT_ORDINAL`
```rust
const IMPORT_OBJECT_ORDINAL: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2893`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2893)*

Import by ordinal

### `IMPORT_OBJECT_NAME`
```rust
const IMPORT_OBJECT_NAME: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2895`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2895)*

Import name == public symbol name.

### `IMPORT_OBJECT_NAME_NO_PREFIX`
```rust
const IMPORT_OBJECT_NAME_NO_PREFIX: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2897`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2897)*

Import name == public symbol name skipping leading ?, @, or optionally _.

### `IMPORT_OBJECT_NAME_UNDECORATE`
```rust
const IMPORT_OBJECT_NAME_UNDECORATE: u16 = 3u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2899`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2899)*

Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @.

### `IMPORT_OBJECT_NAME_EXPORTAS`
```rust
const IMPORT_OBJECT_NAME_EXPORTAS: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2901`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2901)*

Import name == a name is explicitly provided after the DLL name.

### `COMIMAGE_FLAGS_ILONLY`
```rust
const COMIMAGE_FLAGS_ILONLY: u32 = 1u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2904`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2904)*

### `COMIMAGE_FLAGS_32BITREQUIRED`
```rust
const COMIMAGE_FLAGS_32BITREQUIRED: u32 = 2u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2905`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2905)*

### `COMIMAGE_FLAGS_IL_LIBRARY`
```rust
const COMIMAGE_FLAGS_IL_LIBRARY: u32 = 4u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2906`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2906)*

### `COMIMAGE_FLAGS_STRONGNAMESIGNED`
```rust
const COMIMAGE_FLAGS_STRONGNAMESIGNED: u32 = 8u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2907`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2907)*

### `COMIMAGE_FLAGS_NATIVE_ENTRYPOINT`
```rust
const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: u32 = 16u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2908`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2908)*

### `COMIMAGE_FLAGS_TRACKDEBUGDATA`
```rust
const COMIMAGE_FLAGS_TRACKDEBUGDATA: u32 = 65_536u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2909`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2909)*

### `COMIMAGE_FLAGS_32BITPREFERRED`
```rust
const COMIMAGE_FLAGS_32BITPREFERRED: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/pe.rs:2910`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2910)*

### `COR_VERSION_MAJOR_V2`
```rust
const COR_VERSION_MAJOR_V2: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2913`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2913)*

### `COR_VERSION_MAJOR`
```rust
const COR_VERSION_MAJOR: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2914`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2914)*

### `COR_VERSION_MINOR`
```rust
const COR_VERSION_MINOR: u16 = 5u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2915`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2915)*

### `COR_DELETED_NAME_LENGTH`
```rust
const COR_DELETED_NAME_LENGTH: usize = 8usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2916`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2916)*

### `COR_VTABLEGAP_NAME_LENGTH`
```rust
const COR_VTABLEGAP_NAME_LENGTH: usize = 8usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2917`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2917)*

### `NATIVE_TYPE_MAX_CB`
```rust
const NATIVE_TYPE_MAX_CB: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2920`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2920)*

### `COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE`
```rust
const COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE: u16 = 255u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2921`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2921)*

### `IMAGE_COR_MIH_METHODRVA`
```rust
const IMAGE_COR_MIH_METHODRVA: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2924`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2924)*

### `IMAGE_COR_MIH_EHRVA`
```rust
const IMAGE_COR_MIH_EHRVA: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2925`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2925)*

### `IMAGE_COR_MIH_BASICBLOCK`
```rust
const IMAGE_COR_MIH_BASICBLOCK: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2926`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2926)*

### `COR_VTABLE_32BIT`
```rust
const COR_VTABLE_32BIT: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2930`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2930)*

V-table slots are 32-bits in size.

### `COR_VTABLE_64BIT`
```rust
const COR_VTABLE_64BIT: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2932`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2932)*

V-table slots are 64-bits in size.

### `COR_VTABLE_FROM_UNMANAGED`
```rust
const COR_VTABLE_FROM_UNMANAGED: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2934`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2934)*

If set, transition from unmanaged.

### `COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN`
```rust
const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2936`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2936)*

If set, transition from unmanaged with keeping the current appdomain.

### `COR_VTABLE_CALL_MOST_DERIVED`
```rust
const COR_VTABLE_CALL_MOST_DERIVED: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/pe.rs:2938`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2938)*

Call most derived method described by

### `IMAGE_COR_EATJ_THUNK_SIZE`
```rust
const IMAGE_COR_EATJ_THUNK_SIZE: usize = 32usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2942`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2942)*

Size of a jump thunk reserved range.

### `MAX_CLASS_NAME`
```rust
const MAX_CLASS_NAME: usize = 1_024usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2945`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2945)*

### `MAX_PACKAGE_NAME`
```rust
const MAX_PACKAGE_NAME: usize = 1_024usize;
```

*Defined in [`object-0.37.3/src/pe.rs:2946`](../../../.source_1765521767/object-0.37.3/src/pe.rs#L2946)*

