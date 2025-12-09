*[object](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

XCOFF definitions

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/xcoff.h, and is based heavily on it.

## Contents

- [Structs](#structs)
  - [`FileHeader32`](#fileheader32)
  - [`FileHeader64`](#fileheader64)
  - [`AuxHeader32`](#auxheader32)
  - [`AuxHeader64`](#auxheader64)
  - [`SectionHeader32`](#sectionheader32)
  - [`SectionHeader64`](#sectionheader64)
  - [`SymbolBytes`](#symbolbytes)
  - [`Symbol32`](#symbol32)
  - [`Symbol64`](#symbol64)
  - [`FileAux32`](#fileaux32)
  - [`FileAux64`](#fileaux64)
  - [`CsectAux32`](#csectaux32)
  - [`CsectAux64`](#csectaux64)
  - [`FunAux32`](#funaux32)
  - [`FunAux64`](#funaux64)
  - [`ExpAux`](#expaux)
  - [`BlockAux32`](#blockaux32)
  - [`BlockAux64`](#blockaux64)
  - [`StatAux`](#stataux)
  - [`DwarfAux32`](#dwarfaux32)
  - [`DwarfAux64`](#dwarfaux64)
  - [`Rel32`](#rel32)
  - [`Rel64`](#rel64)
- [Constants](#constants)
  - [`MAGIC_64`](#magic_64)
  - [`MAGIC_32`](#magic_32)
  - [`F_RELFLG`](#f_relflg)
  - [`F_EXEC`](#f_exec)
  - [`F_LNNO`](#f_lnno)
  - [`F_FDPR_PROF`](#f_fdpr_prof)
  - [`F_FDPR_OPTI`](#f_fdpr_opti)
  - [`F_DSA`](#f_dsa)
  - [`F_VARPG`](#f_varpg)
  - [`F_DYNLOAD`](#f_dynload)
  - [`F_SHROBJ`](#f_shrobj)
  - [`F_LOADONLY`](#f_loadonly)
  - [`AOUTHSZ_SHORT`](#aouthsz_short)
  - [`STYP_REG`](#styp_reg)
  - [`STYP_PAD`](#styp_pad)
  - [`STYP_DWARF`](#styp_dwarf)
  - [`STYP_TEXT`](#styp_text)
  - [`STYP_DATA`](#styp_data)
  - [`STYP_BSS`](#styp_bss)
  - [`STYP_EXCEPT`](#styp_except)
  - [`STYP_INFO`](#styp_info)
  - [`STYP_TDATA`](#styp_tdata)
  - [`STYP_TBSS`](#styp_tbss)
  - [`STYP_LOADER`](#styp_loader)
  - [`STYP_DEBUG`](#styp_debug)
  - [`STYP_TYPCHK`](#styp_typchk)
  - [`STYP_OVRFLO`](#styp_ovrflo)
  - [`SSUBTYP_DWINFO`](#ssubtyp_dwinfo)
  - [`SSUBTYP_DWLINE`](#ssubtyp_dwline)
  - [`SSUBTYP_DWPBNMS`](#ssubtyp_dwpbnms)
  - [`SSUBTYP_DWPBTYP`](#ssubtyp_dwpbtyp)
  - [`SSUBTYP_DWARNGE`](#ssubtyp_dwarnge)
  - [`SSUBTYP_DWABREV`](#ssubtyp_dwabrev)
  - [`SSUBTYP_DWSTR`](#ssubtyp_dwstr)
  - [`SSUBTYP_DWRNGES`](#ssubtyp_dwrnges)
  - [`SSUBTYP_DWLOC`](#ssubtyp_dwloc)
  - [`SSUBTYP_DWFRAME`](#ssubtyp_dwframe)
  - [`SSUBTYP_DWMAC`](#ssubtyp_dwmac)
  - [`SIZEOF_SYMBOL`](#sizeof_symbol)
  - [`N_DEBUG`](#n_debug)
  - [`N_ABS`](#n_abs)
  - [`N_UNDEF`](#n_undef)
  - [`SYM_V_MASK`](#sym_v_mask)
  - [`SYM_V_INTERNAL`](#sym_v_internal)
  - [`SYM_V_HIDDEN`](#sym_v_hidden)
  - [`SYM_V_PROTECTED`](#sym_v_protected)
  - [`SYM_V_EXPORTED`](#sym_v_exported)
  - [`C_FILE`](#c_file)
  - [`C_BINCL`](#c_bincl)
  - [`C_EINCL`](#c_eincl)
  - [`C_GSYM`](#c_gsym)
  - [`C_STSYM`](#c_stsym)
  - [`C_BCOMM`](#c_bcomm)
  - [`C_ECOMM`](#c_ecomm)
  - [`C_ENTRY`](#c_entry)
  - [`C_BSTAT`](#c_bstat)
  - [`C_ESTAT`](#c_estat)
  - [`C_GTLS`](#c_gtls)
  - [`C_STTLS`](#c_sttls)
  - [`C_DWARF`](#c_dwarf)
  - [`C_LSYM`](#c_lsym)
  - [`C_PSYM`](#c_psym)
  - [`C_RSYM`](#c_rsym)
  - [`C_RPSYM`](#c_rpsym)
  - [`C_ECOML`](#c_ecoml)
  - [`C_FUN`](#c_fun)
  - [`C_EXT`](#c_ext)
  - [`C_WEAKEXT`](#c_weakext)
  - [`C_NULL`](#c_null)
  - [`C_STAT`](#c_stat)
  - [`C_BLOCK`](#c_block)
  - [`C_FCN`](#c_fcn)
  - [`C_HIDEXT`](#c_hidext)
  - [`C_INFO`](#c_info)
  - [`C_DECL`](#c_decl)
  - [`C_AUTO`](#c_auto)
  - [`C_REG`](#c_reg)
  - [`C_EXTDEF`](#c_extdef)
  - [`C_LABEL`](#c_label)
  - [`C_ULABEL`](#c_ulabel)
  - [`C_MOS`](#c_mos)
  - [`C_ARG`](#c_arg)
  - [`C_STRTAG`](#c_strtag)
  - [`C_MOU`](#c_mou)
  - [`C_UNTAG`](#c_untag)
  - [`C_TPDEF`](#c_tpdef)
  - [`C_USTATIC`](#c_ustatic)
  - [`C_ENTAG`](#c_entag)
  - [`C_MOE`](#c_moe)
  - [`C_REGPARM`](#c_regparm)
  - [`C_FIELD`](#c_field)
  - [`C_EOS`](#c_eos)
  - [`C_ALIAS`](#c_alias)
  - [`C_HIDDEN`](#c_hidden)
  - [`C_EFCN`](#c_efcn)
  - [`C_TCSYM`](#c_tcsym)
  - [`XFT_FN`](#xft_fn)
  - [`XFT_CT`](#xft_ct)
  - [`XFT_CV`](#xft_cv)
  - [`XFT_CD`](#xft_cd)
  - [`XTY_ER`](#xty_er)
  - [`XTY_SD`](#xty_sd)
  - [`XTY_LD`](#xty_ld)
  - [`XTY_CM`](#xty_cm)
  - [`XMC_PR`](#xmc_pr)
  - [`XMC_RO`](#xmc_ro)
  - [`XMC_DB`](#xmc_db)
  - [`XMC_GL`](#xmc_gl)
  - [`XMC_XO`](#xmc_xo)
  - [`XMC_SV`](#xmc_sv)
  - [`XMC_SV64`](#xmc_sv64)
  - [`XMC_SV3264`](#xmc_sv3264)
  - [`XMC_TI`](#xmc_ti)
  - [`XMC_TB`](#xmc_tb)
  - [`XMC_RW`](#xmc_rw)
  - [`XMC_TC0`](#xmc_tc0)
  - [`XMC_TC`](#xmc_tc)
  - [`XMC_TD`](#xmc_td)
  - [`XMC_DS`](#xmc_ds)
  - [`XMC_UA`](#xmc_ua)
  - [`XMC_BS`](#xmc_bs)
  - [`XMC_UC`](#xmc_uc)
  - [`XMC_TL`](#xmc_tl)
  - [`XMC_UL`](#xmc_ul)
  - [`XMC_TE`](#xmc_te)
  - [`AUX_EXCEPT`](#aux_except)
  - [`AUX_FCN`](#aux_fcn)
  - [`AUX_SYM`](#aux_sym)
  - [`AUX_FILE`](#aux_file)
  - [`AUX_CSECT`](#aux_csect)
  - [`AUX_SECT`](#aux_sect)
  - [`R_POS`](#r_pos)
  - [`R_RL`](#r_rl)
  - [`R_RLA`](#r_rla)
  - [`R_NEG`](#r_neg)
  - [`R_REL`](#r_rel)
  - [`R_TOC`](#r_toc)
  - [`R_TRL`](#r_trl)
  - [`R_TRLA`](#r_trla)
  - [`R_GL`](#r_gl)
  - [`R_TCL`](#r_tcl)
  - [`R_REF`](#r_ref)
  - [`R_BA`](#r_ba)
  - [`R_BR`](#r_br)
  - [`R_RBA`](#r_rba)
  - [`R_RBR`](#r_rbr)
  - [`R_TLS`](#r_tls)
  - [`R_TLS_IE`](#r_tls_ie)
  - [`R_TLS_LD`](#r_tls_ld)
  - [`R_TLS_LE`](#r_tls_le)
  - [`R_TLSM`](#r_tlsm)
  - [`R_TLSML`](#r_tlsml)
  - [`R_TOCU`](#r_tocu)
  - [`R_TOCL`](#r_tocl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FileHeader32`](#fileheader32) | struct | The header at the start of every 32-bit XCOFF file. |
| [`FileHeader64`](#fileheader64) | struct | The header at the start of every 64-bit XCOFF file. |
| [`AuxHeader32`](#auxheader32) | struct | The auxiliary header immediately following file header. |
| [`AuxHeader64`](#auxheader64) | struct | The auxiliary header immediately following file header. |
| [`SectionHeader32`](#sectionheader32) | struct | Section header. |
| [`SectionHeader64`](#sectionheader64) | struct | Section header. |
| [`SymbolBytes`](#symbolbytes) | struct |  |
| [`Symbol32`](#symbol32) | struct | Symbol table entry. |
| [`Symbol64`](#symbol64) | struct | Symbol table entry. |
| [`FileAux32`](#fileaux32) | struct | File Auxiliary Entry for C_FILE Symbols. |
| [`FileAux64`](#fileaux64) | struct | File Auxiliary Entry for C_FILE Symbols. |
| [`CsectAux32`](#csectaux32) | struct | Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols. |
| [`CsectAux64`](#csectaux64) | struct | Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols. |
| [`FunAux32`](#funaux32) | struct | Function auxiliary entry. |
| [`FunAux64`](#funaux64) | struct | Function auxiliary entry. |
| [`ExpAux`](#expaux) | struct | Exception auxiliary entry. |
| [`BlockAux32`](#blockaux32) | struct | Block auxiliary entry for the C_BLOCK and C_FCN Symbols. |
| [`BlockAux64`](#blockaux64) | struct | Block auxiliary entry for the C_BLOCK and C_FCN Symbols. |
| [`StatAux`](#stataux) | struct | Section auxiliary entry for the C_STAT Symbol. |
| [`DwarfAux32`](#dwarfaux32) | struct | Section auxiliary entry Format for C_DWARF symbols. |
| [`DwarfAux64`](#dwarfaux64) | struct | Section auxiliary entry Format for C_DWARF symbols. |
| [`Rel32`](#rel32) | struct | Relocation table entry |
| [`Rel64`](#rel64) | struct | Relocation table entry |
| [`MAGIC_64`](#magic_64) | const | the 64-bit mach magic number |
| [`MAGIC_32`](#magic_32) | const | the 32-bit mach magic number |
| [`F_RELFLG`](#f_relflg) | const | Indicates that the relocation information for binding has been removed from the file. |
| [`F_EXEC`](#f_exec) | const | Indicates that the file is executable. |
| [`F_LNNO`](#f_lnno) | const | Indicates that line numbers have been stripped from the file by a utility program. |
| [`F_FDPR_PROF`](#f_fdpr_prof) | const | Indicates that the file was profiled with the fdpr command. |
| [`F_FDPR_OPTI`](#f_fdpr_opti) | const | Indicates that the file was reordered with the fdpr command. |
| [`F_DSA`](#f_dsa) | const | Indicates that the file uses Very Large Program Support. |
| [`F_VARPG`](#f_varpg) | const | Indicates that one of the members of the auxiliary header specifying the medium page sizes is non-zero. |
| [`F_DYNLOAD`](#f_dynload) | const | Indicates the file is dynamically loadable and executable. |
| [`F_SHROBJ`](#f_shrobj) | const | Indicates the file is a shared object (shared library). |
| [`F_LOADONLY`](#f_loadonly) | const | If the object file is a member of an archive, it can be loaded by the system loader, but the member is ignored by the binder. |
| [`AOUTHSZ_SHORT`](#aouthsz_short) | const | Some AIX programs generate auxiliary headers for 32-bit object files that end after the data_start field. |
| [`STYP_REG`](#styp_reg) | const | "regular" section |
| [`STYP_PAD`](#styp_pad) | const | Specifies a pad section. |
| [`STYP_DWARF`](#styp_dwarf) | const | Specifies a DWARF debugging section, which provide source file and symbol information for the symbolic debugger. |
| [`STYP_TEXT`](#styp_text) | const | Specifies an executable text (code) section. |
| [`STYP_DATA`](#styp_data) | const | Specifies an initialized data section. |
| [`STYP_BSS`](#styp_bss) | const | Specifies an uninitialized data section. |
| [`STYP_EXCEPT`](#styp_except) | const | Specifies an exception section. |
| [`STYP_INFO`](#styp_info) | const | Specifies a comment section. |
| [`STYP_TDATA`](#styp_tdata) | const | Specifies an initialized thread-local data section. |
| [`STYP_TBSS`](#styp_tbss) | const | Specifies an uninitialized thread-local data section. |
| [`STYP_LOADER`](#styp_loader) | const | Specifies a loader section. |
| [`STYP_DEBUG`](#styp_debug) | const | Specifies a debug section. |
| [`STYP_TYPCHK`](#styp_typchk) | const | Specifies a type-check section. |
| [`STYP_OVRFLO`](#styp_ovrflo) | const | Specifies a relocation or line-number field overflow section. |
| [`SSUBTYP_DWINFO`](#ssubtyp_dwinfo) | const |  |
| [`SSUBTYP_DWLINE`](#ssubtyp_dwline) | const |  |
| [`SSUBTYP_DWPBNMS`](#ssubtyp_dwpbnms) | const |  |
| [`SSUBTYP_DWPBTYP`](#ssubtyp_dwpbtyp) | const |  |
| [`SSUBTYP_DWARNGE`](#ssubtyp_dwarnge) | const |  |
| [`SSUBTYP_DWABREV`](#ssubtyp_dwabrev) | const |  |
| [`SSUBTYP_DWSTR`](#ssubtyp_dwstr) | const |  |
| [`SSUBTYP_DWRNGES`](#ssubtyp_dwrnges) | const |  |
| [`SSUBTYP_DWLOC`](#ssubtyp_dwloc) | const |  |
| [`SSUBTYP_DWFRAME`](#ssubtyp_dwframe) | const |  |
| [`SSUBTYP_DWMAC`](#ssubtyp_dwmac) | const |  |
| [`SIZEOF_SYMBOL`](#sizeof_symbol) | const |  |
| [`N_DEBUG`](#n_debug) | const | A special symbolic debugging symbol. |
| [`N_ABS`](#n_abs) | const | An absolute symbol. |
| [`N_UNDEF`](#n_undef) | const | An undefined external symbol. |
| [`SYM_V_MASK`](#sym_v_mask) | const | Values for visibility as they would appear when encoded in the high 4 bits of the 16-bit unsigned n_type field of symbol table entries. |
| [`SYM_V_INTERNAL`](#sym_v_internal) | const |  |
| [`SYM_V_HIDDEN`](#sym_v_hidden) | const |  |
| [`SYM_V_PROTECTED`](#sym_v_protected) | const |  |
| [`SYM_V_EXPORTED`](#sym_v_exported) | const |  |
| [`C_FILE`](#c_file) | const | Source file name and compiler information. |
| [`C_BINCL`](#c_bincl) | const | Beginning of include file. |
| [`C_EINCL`](#c_eincl) | const | Ending of include file. |
| [`C_GSYM`](#c_gsym) | const | Global variable. |
| [`C_STSYM`](#c_stsym) | const | Statically allocated symbol. |
| [`C_BCOMM`](#c_bcomm) | const | Beginning of common block. |
| [`C_ECOMM`](#c_ecomm) | const | End of common block. |
| [`C_ENTRY`](#c_entry) | const | Alternate entry. |
| [`C_BSTAT`](#c_bstat) | const | Beginning of static block. |
| [`C_ESTAT`](#c_estat) | const | End of static block. |
| [`C_GTLS`](#c_gtls) | const | Global thread-local variable. |
| [`C_STTLS`](#c_sttls) | const | Static thread-local variable. |
| [`C_DWARF`](#c_dwarf) | const | DWARF section symbol. |
| [`C_LSYM`](#c_lsym) | const | Automatic variable allocated on stack. |
| [`C_PSYM`](#c_psym) | const | Argument to subroutine allocated on stack. |
| [`C_RSYM`](#c_rsym) | const | Register variable. |
| [`C_RPSYM`](#c_rpsym) | const | Argument to function or procedure stored in register. |
| [`C_ECOML`](#c_ecoml) | const | Local member of common block. |
| [`C_FUN`](#c_fun) | const | Function or procedure. |
| [`C_EXT`](#c_ext) | const | External symbol. |
| [`C_WEAKEXT`](#c_weakext) | const | Weak external symbol. |
| [`C_NULL`](#c_null) | const | Symbol table entry marked for deletion. |
| [`C_STAT`](#c_stat) | const | Static. |
| [`C_BLOCK`](#c_block) | const | Beginning or end of inner block. |
| [`C_FCN`](#c_fcn) | const | Beginning or end of function. |
| [`C_HIDEXT`](#c_hidext) | const | Un-named external symbol. |
| [`C_INFO`](#c_info) | const | Comment string in .info section. |
| [`C_DECL`](#c_decl) | const | Declaration of object (type). |
| [`C_AUTO`](#c_auto) | const | Automatic variable. |
| [`C_REG`](#c_reg) | const | Register variable. |
| [`C_EXTDEF`](#c_extdef) | const | External definition. |
| [`C_LABEL`](#c_label) | const | Label. |
| [`C_ULABEL`](#c_ulabel) | const | Undefined label. |
| [`C_MOS`](#c_mos) | const | Member of structure. |
| [`C_ARG`](#c_arg) | const | Function argument. |
| [`C_STRTAG`](#c_strtag) | const | Structure tag. |
| [`C_MOU`](#c_mou) | const | Member of union. |
| [`C_UNTAG`](#c_untag) | const | Union tag. |
| [`C_TPDEF`](#c_tpdef) | const | Type definition. |
| [`C_USTATIC`](#c_ustatic) | const | Undefined static. |
| [`C_ENTAG`](#c_entag) | const | Enumeration tag. |
| [`C_MOE`](#c_moe) | const | Member of enumeration. |
| [`C_REGPARM`](#c_regparm) | const | Register parameter. |
| [`C_FIELD`](#c_field) | const | Bit field. |
| [`C_EOS`](#c_eos) | const | End of structure. |
| [`C_ALIAS`](#c_alias) | const | Duplicate tag. |
| [`C_HIDDEN`](#c_hidden) | const | Special storage class for external. |
| [`C_EFCN`](#c_efcn) | const | Physical end of function. |
| [`C_TCSYM`](#c_tcsym) | const | Reserved. |
| [`XFT_FN`](#xft_fn) | const | Specifies the source-file name. |
| [`XFT_CT`](#xft_ct) | const | Specifies the compiler time stamp. |
| [`XFT_CV`](#xft_cv) | const | Specifies the compiler version number. |
| [`XFT_CD`](#xft_cd) | const | Specifies compiler-defined information. |
| [`XTY_ER`](#xty_er) | const | External reference. |
| [`XTY_SD`](#xty_sd) | const | Csect definition for initialized storage. |
| [`XTY_LD`](#xty_ld) | const | Defines an entry point to an initialized csect. |
| [`XTY_CM`](#xty_cm) | const | Common csect definition. |
| [`XMC_PR`](#xmc_pr) | const | Program Code |
| [`XMC_RO`](#xmc_ro) | const | Read Only Constant |
| [`XMC_DB`](#xmc_db) | const | Debug Dictionary Table |
| [`XMC_GL`](#xmc_gl) | const | Global Linkage (Interfile Interface Code) |
| [`XMC_XO`](#xmc_xo) | const | Extended Operation (Pseudo Machine Instruction) |
| [`XMC_SV`](#xmc_sv) | const | Supervisor Call (32-bit process only) |
| [`XMC_SV64`](#xmc_sv64) | const | Supervisor Call for 64-bit process |
| [`XMC_SV3264`](#xmc_sv3264) | const | Supervisor Call for both 32- and 64-bit processes |
| [`XMC_TI`](#xmc_ti) | const | Traceback Index csect |
| [`XMC_TB`](#xmc_tb) | const | Traceback Table csect |
| [`XMC_RW`](#xmc_rw) | const | Read Write Data |
| [`XMC_TC0`](#xmc_tc0) | const | TOC Anchor for TOC Addressability |
| [`XMC_TC`](#xmc_tc) | const | General TOC item |
| [`XMC_TD`](#xmc_td) | const | Scalar data item in the TOC |
| [`XMC_DS`](#xmc_ds) | const | Descriptor csect |
| [`XMC_UA`](#xmc_ua) | const | Unclassified - Treated as Read Write |
| [`XMC_BS`](#xmc_bs) | const | BSS class (uninitialized static internal) |
| [`XMC_UC`](#xmc_uc) | const | Un-named Fortran Common |
| [`XMC_TL`](#xmc_tl) | const | Initialized thread-local variable |
| [`XMC_UL`](#xmc_ul) | const | Uninitialized thread-local variable |
| [`XMC_TE`](#xmc_te) | const | Symbol mapped at the end of TOC |
| [`AUX_EXCEPT`](#aux_except) | const | Identifies an exception auxiliary entry. |
| [`AUX_FCN`](#aux_fcn) | const | Identifies a function auxiliary entry. |
| [`AUX_SYM`](#aux_sym) | const | Identifies a symbol auxiliary entry. |
| [`AUX_FILE`](#aux_file) | const | Identifies a file auxiliary entry. |
| [`AUX_CSECT`](#aux_csect) | const | Identifies a csect auxiliary entry. |
| [`AUX_SECT`](#aux_sect) | const | Identifies a SECT auxiliary entry. |
| [`R_POS`](#r_pos) | const | Positive relocation. |
| [`R_RL`](#r_rl) | const | Positive indirect load relocation. |
| [`R_RLA`](#r_rla) | const | Positive load address relocation. |
| [`R_NEG`](#r_neg) | const | Negative relocation. |
| [`R_REL`](#r_rel) | const | Relative to self relocation. |
| [`R_TOC`](#r_toc) | const | Relative to the TOC relocation. |
| [`R_TRL`](#r_trl) | const | TOC relative indirect load relocation. |
| [`R_TRLA`](#r_trla) | const | Relative to the TOC or to the thread-local storage base relocation. |
| [`R_GL`](#r_gl) | const | Global linkage-external TOC address relocation. |
| [`R_TCL`](#r_tcl) | const | Local object TOC address relocation. |
| [`R_REF`](#r_ref) | const | A non-relocating relocation. |
| [`R_BA`](#r_ba) | const | Branch absolute relocation. |
| [`R_BR`](#r_br) | const | Branch relative to self relocation. |
| [`R_RBA`](#r_rba) | const | Branch absolute relocation. |
| [`R_RBR`](#r_rbr) | const | Branch relative to self relocation. |
| [`R_TLS`](#r_tls) | const | General-dynamic reference to TLS symbol. |
| [`R_TLS_IE`](#r_tls_ie) | const | Initial-exec reference to TLS symbol. |
| [`R_TLS_LD`](#r_tls_ld) | const | Local-dynamic reference to TLS symbol. |
| [`R_TLS_LE`](#r_tls_le) | const | Local-exec reference to TLS symbol. |
| [`R_TLSM`](#r_tlsm) | const | Module reference to TLS. |
| [`R_TLSML`](#r_tlsml) | const | Module reference to the local TLS storage. |
| [`R_TOCU`](#r_tocu) | const | Relative to TOC upper. |
| [`R_TOCL`](#r_tocl) | const | Relative to TOC lower. |

## Structs

### `FileHeader32`

```rust
struct FileHeader32 {
    pub f_magic: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nscns: crate::endian::U16<crate::endian::BigEndian>,
    pub f_timdat: crate::endian::U32<crate::endian::BigEndian>,
    pub f_symptr: crate::endian::U32<crate::endian::BigEndian>,
    pub f_nsyms: crate::endian::U32<crate::endian::BigEndian>,
    pub f_opthdr: crate::endian::U16<crate::endian::BigEndian>,
    pub f_flags: crate::endian::U16<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:16-31`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L16-L31)*

The header at the start of every 32-bit XCOFF file.

#### Fields

- **`f_magic`**: `crate::endian::U16<crate::endian::BigEndian>`

  Magic number. Must be 0x01DF.

- **`f_nscns`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of sections.

- **`f_timdat`**: `crate::endian::U32<crate::endian::BigEndian>`

  Time and date of file creation.

- **`f_symptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Byte offset to symbol table start.

- **`f_nsyms`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of entries in symbol table.

- **`f_opthdr`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of bytes in optional header

- **`f_flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  Extra flags.

#### Trait Implementations

##### `impl Clone for FileHeader32`

- <span id="fileheader32-clone"></span>`fn clone(&self) -> FileHeader32` — [`FileHeader32`](#fileheader32)

##### `impl Copy for FileHeader32`

##### `impl Debug for FileHeader32`

- <span id="fileheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileHeader for xcoff::FileHeader32`

- <span id="xcofffileheader32-type-word"></span>`type Word = u32`

- <span id="xcofffileheader32-type-auxheader"></span>`type AuxHeader = AuxHeader32`

- <span id="xcofffileheader32-type-sectionheader"></span>`type SectionHeader = SectionHeader32`

- <span id="xcofffileheader32-type-symbol"></span>`type Symbol = Symbol32`

- <span id="xcofffileheader32-type-fileaux"></span>`type FileAux = FileAux32`

- <span id="xcofffileheader32-type-csectaux"></span>`type CsectAux = CsectAux32`

- <span id="xcofffileheader32-type-rel"></span>`type Rel = Rel32`

- <span id="xcofffileheader32-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="xcofffileheader32-f-magic"></span>`fn f_magic(&self) -> u16`

- <span id="xcofffileheader32-f-nscns"></span>`fn f_nscns(&self) -> u16`

- <span id="xcofffileheader32-f-timdat"></span>`fn f_timdat(&self) -> u32`

- <span id="xcofffileheader32-f-symptr"></span>`fn f_symptr(&self) -> <Self as >::Word` — [`FileHeader`](../read/xcoff/index.md)

- <span id="xcofffileheader32-f-nsyms"></span>`fn f_nsyms(&self) -> u32`

- <span id="xcofffileheader32-f-opthdr"></span>`fn f_opthdr(&self) -> u16`

- <span id="xcofffileheader32-f-flags"></span>`fn f_flags(&self) -> u16`

##### `impl Pod for FileHeader32`

### `FileHeader64`

```rust
struct FileHeader64 {
    pub f_magic: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nscns: crate::endian::U16<crate::endian::BigEndian>,
    pub f_timdat: crate::endian::U32<crate::endian::BigEndian>,
    pub f_symptr: crate::endian::U64<crate::endian::BigEndian>,
    pub f_opthdr: crate::endian::U16<crate::endian::BigEndian>,
    pub f_flags: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nsyms: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:36-51`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L36-L51)*

The header at the start of every 64-bit XCOFF file.

#### Fields

- **`f_magic`**: `crate::endian::U16<crate::endian::BigEndian>`

  Magic number. Must be 0x01F7.

- **`f_nscns`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of sections.

- **`f_timdat`**: `crate::endian::U32<crate::endian::BigEndian>`

  Time and date of file creation

- **`f_symptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Byte offset to symbol table start.

- **`f_opthdr`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of bytes in optional header

- **`f_flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  Extra flags.

- **`f_nsyms`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of entries in symbol table.

#### Trait Implementations

##### `impl Clone for FileHeader64`

- <span id="fileheader64-clone"></span>`fn clone(&self) -> FileHeader64` — [`FileHeader64`](#fileheader64)

##### `impl Copy for FileHeader64`

##### `impl Debug for FileHeader64`

- <span id="fileheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileHeader for xcoff::FileHeader64`

- <span id="xcofffileheader64-type-word"></span>`type Word = u64`

- <span id="xcofffileheader64-type-auxheader"></span>`type AuxHeader = AuxHeader64`

- <span id="xcofffileheader64-type-sectionheader"></span>`type SectionHeader = SectionHeader64`

- <span id="xcofffileheader64-type-symbol"></span>`type Symbol = Symbol64`

- <span id="xcofffileheader64-type-fileaux"></span>`type FileAux = FileAux64`

- <span id="xcofffileheader64-type-csectaux"></span>`type CsectAux = CsectAux64`

- <span id="xcofffileheader64-type-rel"></span>`type Rel = Rel64`

- <span id="xcofffileheader64-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="xcofffileheader64-f-magic"></span>`fn f_magic(&self) -> u16`

- <span id="xcofffileheader64-f-nscns"></span>`fn f_nscns(&self) -> u16`

- <span id="xcofffileheader64-f-timdat"></span>`fn f_timdat(&self) -> u32`

- <span id="xcofffileheader64-f-symptr"></span>`fn f_symptr(&self) -> <Self as >::Word` — [`FileHeader`](../read/xcoff/index.md)

- <span id="xcofffileheader64-f-nsyms"></span>`fn f_nsyms(&self) -> u32`

- <span id="xcofffileheader64-f-opthdr"></span>`fn f_opthdr(&self) -> u16`

- <span id="xcofffileheader64-f-flags"></span>`fn f_flags(&self) -> u16`

##### `impl Pod for FileHeader64`

### `AuxHeader32`

```rust
struct AuxHeader32 {
    pub o_mflag: crate::endian::U16<crate::endian::BigEndian>,
    pub o_vstamp: crate::endian::U16<crate::endian::BigEndian>,
    pub o_tsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_dsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_bsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_entry: crate::endian::U32<crate::endian::BigEndian>,
    pub o_text_start: crate::endian::U32<crate::endian::BigEndian>,
    pub o_data_start: crate::endian::U32<crate::endian::BigEndian>,
    pub o_toc: crate::endian::U32<crate::endian::BigEndian>,
    pub o_snentry: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntoc: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snloader: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_modtype: crate::endian::U16<crate::endian::BigEndian>,
    pub o_cpuflag: u8,
    pub o_cputype: u8,
    pub o_maxstack: crate::endian::U32<crate::endian::BigEndian>,
    pub o_maxdata: crate::endian::U32<crate::endian::BigEndian>,
    pub o_debugger: crate::endian::U32<crate::endian::BigEndian>,
    pub o_textpsize: u8,
    pub o_datapsize: u8,
    pub o_stackpsize: u8,
    pub o_flags: u8,
    pub o_sntdata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntbss: crate::endian::U16<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:95-154`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L95-L154)*

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

#### Fields

- **`o_mflag`**: `crate::endian::U16<crate::endian::BigEndian>`

  Flags.

- **`o_vstamp`**: `crate::endian::U16<crate::endian::BigEndian>`

  Version.

- **`o_tsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Text size in bytes.

- **`o_dsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Initialized data size in bytes.

- **`o_bsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Uninitialized data size in bytes.

- **`o_entry`**: `crate::endian::U32<crate::endian::BigEndian>`

  Entry point descriptor (virtual address).

- **`o_text_start`**: `crate::endian::U32<crate::endian::BigEndian>`

  Base address of text (virtual address).

- **`o_data_start`**: `crate::endian::U32<crate::endian::BigEndian>`

  Base address of data (virtual address).

- **`o_toc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Address of TOC anchor.

- **`o_snentry`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for entry point.

- **`o_sntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .text.

- **`o_sndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .data.

- **`o_sntoc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for TOC.

- **`o_snloader`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for loader data.

- **`o_snbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .bss.

- **`o_algntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .text.

- **`o_algndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .data.

- **`o_modtype`**: `crate::endian::U16<crate::endian::BigEndian>`

  Module type field.

- **`o_cpuflag`**: `u8`

  Bit flags - cpu types of objects.

- **`o_cputype`**: `u8`

  Reserved for CPU type.

- **`o_maxstack`**: `crate::endian::U32<crate::endian::BigEndian>`

  Maximum stack size allowed (bytes).

- **`o_maxdata`**: `crate::endian::U32<crate::endian::BigEndian>`

  Maximum data size allowed (bytes).

- **`o_debugger`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved for debuggers.

- **`o_textpsize`**: `u8`

  Requested text page size.

- **`o_datapsize`**: `u8`

  Requested data page size.

- **`o_stackpsize`**: `u8`

  Requested stack page size.

- **`o_flags`**: `u8`

  Flags and thread-local storage alignment.

- **`o_sntdata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tdata.

- **`o_sntbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tbss.

#### Trait Implementations

##### `impl AuxHeader for xcoff::AuxHeader32`

- <span id="xcoffauxheader32-type-word"></span>`type Word = u32`

- <span id="xcoffauxheader32-o-mflag"></span>`fn o_mflag(&self) -> u16`

- <span id="xcoffauxheader32-o-vstamp"></span>`fn o_vstamp(&self) -> u16`

- <span id="xcoffauxheader32-o-tsize"></span>`fn o_tsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-dsize"></span>`fn o_dsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-bsize"></span>`fn o_bsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-entry"></span>`fn o_entry(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-text-start"></span>`fn o_text_start(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-data-start"></span>`fn o_data_start(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-toc"></span>`fn o_toc(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-snentry"></span>`fn o_snentry(&self) -> u16`

- <span id="xcoffauxheader32-o-sntext"></span>`fn o_sntext(&self) -> u16`

- <span id="xcoffauxheader32-o-sndata"></span>`fn o_sndata(&self) -> u16`

- <span id="xcoffauxheader32-o-sntoc"></span>`fn o_sntoc(&self) -> u16`

- <span id="xcoffauxheader32-o-snloader"></span>`fn o_snloader(&self) -> u16`

- <span id="xcoffauxheader32-o-snbss"></span>`fn o_snbss(&self) -> u16`

- <span id="xcoffauxheader32-o-algntext"></span>`fn o_algntext(&self) -> u16`

- <span id="xcoffauxheader32-o-algndata"></span>`fn o_algndata(&self) -> u16`

- <span id="xcoffauxheader32-o-modtype"></span>`fn o_modtype(&self) -> u16`

- <span id="xcoffauxheader32-o-cpuflag"></span>`fn o_cpuflag(&self) -> u8`

- <span id="xcoffauxheader32-o-cputype"></span>`fn o_cputype(&self) -> u8`

- <span id="xcoffauxheader32-o-maxstack"></span>`fn o_maxstack(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-maxdata"></span>`fn o_maxdata(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader32-o-debugger"></span>`fn o_debugger(&self) -> u32`

- <span id="xcoffauxheader32-o-textpsize"></span>`fn o_textpsize(&self) -> u8`

- <span id="xcoffauxheader32-o-datapsize"></span>`fn o_datapsize(&self) -> u8`

- <span id="xcoffauxheader32-o-stackpsize"></span>`fn o_stackpsize(&self) -> u8`

- <span id="xcoffauxheader32-o-flags"></span>`fn o_flags(&self) -> u8`

- <span id="xcoffauxheader32-o-sntdata"></span>`fn o_sntdata(&self) -> u16`

- <span id="xcoffauxheader32-o-sntbss"></span>`fn o_sntbss(&self) -> u16`

- <span id="xcoffauxheader32-o-x64flags"></span>`fn o_x64flags(&self) -> Option<u16>`

##### `impl Clone for AuxHeader32`

- <span id="auxheader32-clone"></span>`fn clone(&self) -> AuxHeader32` — [`AuxHeader32`](#auxheader32)

##### `impl Copy for AuxHeader32`

##### `impl Debug for AuxHeader32`

- <span id="auxheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for AuxHeader32`

### `AuxHeader64`

```rust
struct AuxHeader64 {
    pub o_mflag: crate::endian::U16<crate::endian::BigEndian>,
    pub o_vstamp: crate::endian::U16<crate::endian::BigEndian>,
    pub o_debugger: crate::endian::U32<crate::endian::BigEndian>,
    pub o_text_start: crate::endian::U64<crate::endian::BigEndian>,
    pub o_data_start: crate::endian::U64<crate::endian::BigEndian>,
    pub o_toc: crate::endian::U64<crate::endian::BigEndian>,
    pub o_snentry: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntoc: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snloader: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_modtype: crate::endian::U16<crate::endian::BigEndian>,
    pub o_cpuflag: u8,
    pub o_cputype: u8,
    pub o_textpsize: u8,
    pub o_datapsize: u8,
    pub o_stackpsize: u8,
    pub o_flags: u8,
    pub o_tsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_dsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_bsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_entry: crate::endian::U64<crate::endian::BigEndian>,
    pub o_maxstack: crate::endian::U64<crate::endian::BigEndian>,
    pub o_maxdata: crate::endian::U64<crate::endian::BigEndian>,
    pub o_sntdata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_x64flags: crate::endian::U16<crate::endian::BigEndian>,
    pub o_resv3a: crate::endian::U16<crate::endian::BigEndian>,
    pub o_resv3: [crate::endian::U32<crate::endian::BigEndian>; 2],
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:160-225`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L160-L225)*

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

#### Fields

- **`o_mflag`**: `crate::endian::U16<crate::endian::BigEndian>`

  Flags.

- **`o_vstamp`**: `crate::endian::U16<crate::endian::BigEndian>`

  Version.

- **`o_debugger`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved for debuggers.

- **`o_text_start`**: `crate::endian::U64<crate::endian::BigEndian>`

  Base address of text (virtual address).

- **`o_data_start`**: `crate::endian::U64<crate::endian::BigEndian>`

  Base address of data (virtual address).

- **`o_toc`**: `crate::endian::U64<crate::endian::BigEndian>`

  Address of TOC anchor.

- **`o_snentry`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for entry point.

- **`o_sntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .text.

- **`o_sndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .data.

- **`o_sntoc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for TOC.

- **`o_snloader`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for loader data.

- **`o_snbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .bss.

- **`o_algntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .text.

- **`o_algndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .data.

- **`o_modtype`**: `crate::endian::U16<crate::endian::BigEndian>`

  Module type field.

- **`o_cpuflag`**: `u8`

  Bit flags - cpu types of objects.

- **`o_cputype`**: `u8`

  Reserved for CPU type.

- **`o_textpsize`**: `u8`

  Requested text page size.

- **`o_datapsize`**: `u8`

  Requested data page size.

- **`o_stackpsize`**: `u8`

  Requested stack page size.

- **`o_flags`**: `u8`

  Flags and thread-local storage alignment.

- **`o_tsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Text size in bytes.

- **`o_dsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Initialized data size in bytes.

- **`o_bsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Uninitialized data size in bytes.

- **`o_entry`**: `crate::endian::U64<crate::endian::BigEndian>`

  Entry point descriptor (virtual address).

- **`o_maxstack`**: `crate::endian::U64<crate::endian::BigEndian>`

  Maximum stack size allowed (bytes).

- **`o_maxdata`**: `crate::endian::U64<crate::endian::BigEndian>`

  Maximum data size allowed (bytes).

- **`o_sntdata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tdata.

- **`o_sntbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tbss.

- **`o_x64flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  XCOFF64 flags.

- **`o_resv3a`**: `crate::endian::U16<crate::endian::BigEndian>`

  Reserved.

- **`o_resv3`**: `[crate::endian::U32<crate::endian::BigEndian>; 2]`

  Reserved.

#### Trait Implementations

##### `impl AuxHeader for xcoff::AuxHeader64`

- <span id="xcoffauxheader64-type-word"></span>`type Word = u64`

- <span id="xcoffauxheader64-o-mflag"></span>`fn o_mflag(&self) -> u16`

- <span id="xcoffauxheader64-o-vstamp"></span>`fn o_vstamp(&self) -> u16`

- <span id="xcoffauxheader64-o-tsize"></span>`fn o_tsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-dsize"></span>`fn o_dsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-bsize"></span>`fn o_bsize(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-entry"></span>`fn o_entry(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-text-start"></span>`fn o_text_start(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-data-start"></span>`fn o_data_start(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-toc"></span>`fn o_toc(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-snentry"></span>`fn o_snentry(&self) -> u16`

- <span id="xcoffauxheader64-o-sntext"></span>`fn o_sntext(&self) -> u16`

- <span id="xcoffauxheader64-o-sndata"></span>`fn o_sndata(&self) -> u16`

- <span id="xcoffauxheader64-o-sntoc"></span>`fn o_sntoc(&self) -> u16`

- <span id="xcoffauxheader64-o-snloader"></span>`fn o_snloader(&self) -> u16`

- <span id="xcoffauxheader64-o-snbss"></span>`fn o_snbss(&self) -> u16`

- <span id="xcoffauxheader64-o-algntext"></span>`fn o_algntext(&self) -> u16`

- <span id="xcoffauxheader64-o-algndata"></span>`fn o_algndata(&self) -> u16`

- <span id="xcoffauxheader64-o-modtype"></span>`fn o_modtype(&self) -> u16`

- <span id="xcoffauxheader64-o-cpuflag"></span>`fn o_cpuflag(&self) -> u8`

- <span id="xcoffauxheader64-o-cputype"></span>`fn o_cputype(&self) -> u8`

- <span id="xcoffauxheader64-o-maxstack"></span>`fn o_maxstack(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-maxdata"></span>`fn o_maxdata(&self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- <span id="xcoffauxheader64-o-debugger"></span>`fn o_debugger(&self) -> u32`

- <span id="xcoffauxheader64-o-textpsize"></span>`fn o_textpsize(&self) -> u8`

- <span id="xcoffauxheader64-o-datapsize"></span>`fn o_datapsize(&self) -> u8`

- <span id="xcoffauxheader64-o-stackpsize"></span>`fn o_stackpsize(&self) -> u8`

- <span id="xcoffauxheader64-o-flags"></span>`fn o_flags(&self) -> u8`

- <span id="xcoffauxheader64-o-sntdata"></span>`fn o_sntdata(&self) -> u16`

- <span id="xcoffauxheader64-o-sntbss"></span>`fn o_sntbss(&self) -> u16`

- <span id="xcoffauxheader64-o-x64flags"></span>`fn o_x64flags(&self) -> Option<u16>`

##### `impl Clone for AuxHeader64`

- <span id="auxheader64-clone"></span>`fn clone(&self) -> AuxHeader64` — [`AuxHeader64`](#auxheader64)

##### `impl Copy for AuxHeader64`

##### `impl Debug for AuxHeader64`

- <span id="auxheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for AuxHeader64`

### `SectionHeader32`

```rust
struct SectionHeader32 {
    pub s_name: [u8; 8],
    pub s_paddr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_vaddr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_size: crate::endian::U32<crate::endian::BigEndian>,
    pub s_scnptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_relptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_lnnoptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_nreloc: crate::endian::U16<crate::endian::BigEndian>,
    pub s_nlnno: crate::endian::U16<crate::endian::BigEndian>,
    pub s_flags: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:234-255`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L234-L255)*

Section header.

#### Fields

- **`s_name`**: `[u8; 8]`

  Section name.

- **`s_paddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Physical address.

- **`s_vaddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Virtual address (same as physical address).

- **`s_size`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section size.

- **`s_scnptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to raw data for section.

- **`s_relptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to relocation entries for section.

- **`s_lnnoptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to line number entries for section.

- **`s_nreloc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of relocation entries.

- **`s_nlnno`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of line number entries.

- **`s_flags`**: `crate::endian::U32<crate::endian::BigEndian>`

  Flags to define the section type.

#### Trait Implementations

##### `impl Clone for SectionHeader32`

- <span id="sectionheader32-clone"></span>`fn clone(&self) -> SectionHeader32` — [`SectionHeader32`](#sectionheader32)

##### `impl Copy for SectionHeader32`

##### `impl Debug for SectionHeader32`

- <span id="sectionheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for SectionHeader32`

##### `impl SectionHeader for xcoff::SectionHeader32`

- <span id="xcoffsectionheader32-type-word"></span>`type Word = u32`

- <span id="xcoffsectionheader32-type-halfword"></span>`type HalfWord = u16`

- <span id="xcoffsectionheader32-type-xcoff"></span>`type Xcoff = FileHeader32`

- <span id="xcoffsectionheader32-type-rel"></span>`type Rel = Rel32`

- <span id="xcoffsectionheader32-s-name"></span>`fn s_name(&self) -> &[u8; 8]`

- <span id="xcoffsectionheader32-s-paddr"></span>`fn s_paddr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-vaddr"></span>`fn s_vaddr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-size"></span>`fn s_size(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-scnptr"></span>`fn s_scnptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-relptr"></span>`fn s_relptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-lnnoptr"></span>`fn s_lnnoptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-nreloc"></span>`fn s_nreloc(&self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-nlnno"></span>`fn s_nlnno(&self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader32-s-flags"></span>`fn s_flags(&self) -> u32`

- <span id="xcoffsectionheader32-relocations"></span>`fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>` — [`Result`](../index.md), [`SectionHeader`](../read/xcoff/index.md)

### `SectionHeader64`

```rust
struct SectionHeader64 {
    pub s_name: [u8; 8],
    pub s_paddr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_vaddr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_size: crate::endian::U64<crate::endian::BigEndian>,
    pub s_scnptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_relptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_lnnoptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_nreloc: crate::endian::U32<crate::endian::BigEndian>,
    pub s_nlnno: crate::endian::U32<crate::endian::BigEndian>,
    pub s_flags: crate::endian::U32<crate::endian::BigEndian>,
    pub s_reserve: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:260-283`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L260-L283)*

Section header.

#### Fields

- **`s_name`**: `[u8; 8]`

  Section name.

- **`s_paddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Physical address.

- **`s_vaddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Virtual address (same as physical address).

- **`s_size`**: `crate::endian::U64<crate::endian::BigEndian>`

  Section size.

- **`s_scnptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to raw data for section.

- **`s_relptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to relocation entries for section.

- **`s_lnnoptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to line number entries for section.

- **`s_nreloc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of relocation entries.

- **`s_nlnno`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of line number entries.

- **`s_flags`**: `crate::endian::U32<crate::endian::BigEndian>`

  Flags to define the section type.

- **`s_reserve`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved.

#### Trait Implementations

##### `impl Clone for SectionHeader64`

- <span id="sectionheader64-clone"></span>`fn clone(&self) -> SectionHeader64` — [`SectionHeader64`](#sectionheader64)

##### `impl Copy for SectionHeader64`

##### `impl Debug for SectionHeader64`

- <span id="sectionheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for SectionHeader64`

##### `impl SectionHeader for xcoff::SectionHeader64`

- <span id="xcoffsectionheader64-type-word"></span>`type Word = u64`

- <span id="xcoffsectionheader64-type-halfword"></span>`type HalfWord = u32`

- <span id="xcoffsectionheader64-type-xcoff"></span>`type Xcoff = FileHeader64`

- <span id="xcoffsectionheader64-type-rel"></span>`type Rel = Rel64`

- <span id="xcoffsectionheader64-s-name"></span>`fn s_name(&self) -> &[u8; 8]`

- <span id="xcoffsectionheader64-s-paddr"></span>`fn s_paddr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-vaddr"></span>`fn s_vaddr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-size"></span>`fn s_size(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-scnptr"></span>`fn s_scnptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-relptr"></span>`fn s_relptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-lnnoptr"></span>`fn s_lnnoptr(&self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-nreloc"></span>`fn s_nreloc(&self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-nlnno"></span>`fn s_nlnno(&self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- <span id="xcoffsectionheader64-s-flags"></span>`fn s_flags(&self) -> u32`

- <span id="xcoffsectionheader64-relocations"></span>`fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>` — [`Result`](../index.md), [`SectionHeader`](../read/xcoff/index.md)

### `SymbolBytes`

```rust
struct SymbolBytes([u8; 18]);
```

*Defined in [`object-0.37.3/src/xcoff.rs:350`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L350)*

#### Trait Implementations

##### `impl Clone for SymbolBytes`

- <span id="symbolbytes-clone"></span>`fn clone(&self) -> SymbolBytes` — [`SymbolBytes`](#symbolbytes)

##### `impl Copy for SymbolBytes`

##### `impl Debug for SymbolBytes`

- <span id="symbolbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for SymbolBytes`

### `Symbol32`

```rust
struct Symbol32 {
    pub n_name: [u8; 8],
    pub n_value: crate::endian::U32<crate::endian::BigEndian>,
    pub n_scnum: crate::endian::I16<crate::endian::BigEndian>,
    pub n_type: crate::endian::U16<crate::endian::BigEndian>,
    pub n_sclass: u8,
    pub n_numaux: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:355-370`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L355-L370)*

Symbol table entry.

#### Fields

- **`n_name`**: `[u8; 8]`

  Symbol name.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`n_value`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol value; storage class-dependent.

- **`n_scnum`**: `crate::endian::I16<crate::endian::BigEndian>`

  Section number of symbol.

- **`n_type`**: `crate::endian::U16<crate::endian::BigEndian>`

  Basic and derived type specification.

- **`n_sclass`**: `u8`

  Storage class of symbol.

- **`n_numaux`**: `u8`

  Number of auxiliary entries.

#### Trait Implementations

##### `impl Clone for Symbol32`

- <span id="symbol32-clone"></span>`fn clone(&self) -> Symbol32` — [`Symbol32`](#symbol32)

##### `impl Copy for Symbol32`

##### `impl Debug for Symbol32`

- <span id="symbol32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for Symbol32`

##### `impl Symbol for xcoff::Symbol32`

- <span id="xcoffsymbol32-type-word"></span>`type Word = u32`

- <span id="xcoffsymbol32-n-value"></span>`fn n_value(&self) -> <Self as >::Word` — [`Symbol`](../read/xcoff/index.md)

- <span id="xcoffsymbol32-n-scnum"></span>`fn n_scnum(&self) -> i16`

- <span id="xcoffsymbol32-n-type"></span>`fn n_type(&self) -> u16`

- <span id="xcoffsymbol32-n-sclass"></span>`fn n_sclass(&self) -> u8`

- <span id="xcoffsymbol32-n-numaux"></span>`fn n_numaux(&self) -> u8`

- <span id="xcoffsymbol32-name-offset"></span>`fn name_offset(&self) -> Option<u32>`

- <span id="xcoffsymbol32-name"></span>`fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

### `Symbol64`

```rust
struct Symbol64 {
    pub n_value: crate::endian::U64<crate::endian::BigEndian>,
    pub n_offset: crate::endian::U32<crate::endian::BigEndian>,
    pub n_scnum: crate::endian::I16<crate::endian::BigEndian>,
    pub n_type: crate::endian::U16<crate::endian::BigEndian>,
    pub n_sclass: u8,
    pub n_numaux: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:375-388`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L375-L388)*

Symbol table entry.

#### Fields

- **`n_value`**: `crate::endian::U64<crate::endian::BigEndian>`

  Symbol value; storage class-dependent.

- **`n_offset`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of the name in string table or .debug section.

- **`n_scnum`**: `crate::endian::I16<crate::endian::BigEndian>`

  Section number of symbol.

- **`n_type`**: `crate::endian::U16<crate::endian::BigEndian>`

  Basic and derived type specification.

- **`n_sclass`**: `u8`

  Storage class of symbol.

- **`n_numaux`**: `u8`

  Number of auxiliary entries.

#### Trait Implementations

##### `impl Clone for Symbol64`

- <span id="symbol64-clone"></span>`fn clone(&self) -> Symbol64` — [`Symbol64`](#symbol64)

##### `impl Copy for Symbol64`

##### `impl Debug for Symbol64`

- <span id="symbol64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for Symbol64`

##### `impl Symbol for xcoff::Symbol64`

- <span id="xcoffsymbol64-type-word"></span>`type Word = u64`

- <span id="xcoffsymbol64-n-value"></span>`fn n_value(&self) -> <Self as >::Word` — [`Symbol`](../read/xcoff/index.md)

- <span id="xcoffsymbol64-n-scnum"></span>`fn n_scnum(&self) -> i16`

- <span id="xcoffsymbol64-n-type"></span>`fn n_type(&self) -> u16`

- <span id="xcoffsymbol64-n-sclass"></span>`fn n_sclass(&self) -> u8`

- <span id="xcoffsymbol64-n-numaux"></span>`fn n_numaux(&self) -> u8`

- <span id="xcoffsymbol64-name-offset"></span>`fn name_offset(&self) -> Option<u32>`

- <span id="xcoffsymbol64-name"></span>`fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

### `FileAux32`

```rust
struct FileAux32 {
    pub x_fname: [u8; 8],
    pub x_fpad: [u8; 6],
    pub x_ftype: u8,
    pub x_freserve: [u8; 3],
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:528-539`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L528-L539)*

File Auxiliary Entry for C_FILE Symbols.

#### Fields

- **`x_fname`**: `[u8; 8]`

  The source file name or compiler-related string.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`x_fpad`**: `[u8; 6]`

  Pad size for file name.

- **`x_ftype`**: `u8`

  The source-file string type.

- **`x_freserve`**: `[u8; 3]`

  Reserved.

#### Trait Implementations

##### `impl Clone for FileAux32`

- <span id="fileaux32-clone"></span>`fn clone(&self) -> FileAux32` — [`FileAux32`](#fileaux32)

##### `impl Copy for FileAux32`

##### `impl Debug for FileAux32`

- <span id="fileaux32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileAux for xcoff::FileAux32`

- <span id="xcofffileaux32-x-fname"></span>`fn x_fname(&self) -> &[u8; 8]`

- <span id="xcofffileaux32-x-ftype"></span>`fn x_ftype(&self) -> u8`

- <span id="xcofffileaux32-x-auxtype"></span>`fn x_auxtype(&self) -> Option<u8>`

##### `impl Pod for FileAux32`

### `FileAux64`

```rust
struct FileAux64 {
    pub x_fname: [u8; 8],
    pub x_fpad: [u8; 6],
    pub x_ftype: u8,
    pub x_freserve: [u8; 2],
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:544-557`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L544-L557)*

File Auxiliary Entry for C_FILE Symbols.

#### Fields

- **`x_fname`**: `[u8; 8]`

  The source file name or compiler-related string.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`x_fpad`**: `[u8; 6]`

  Pad size for file name.

- **`x_ftype`**: `u8`

  The source-file string type.

- **`x_freserve`**: `[u8; 2]`

  Reserved.

- **`x_auxtype`**: `u8`

  Specifies the type of auxiliary entry. Contains _AUX_FILE for this auxiliary entry.

#### Trait Implementations

##### `impl Clone for FileAux64`

- <span id="fileaux64-clone"></span>`fn clone(&self) -> FileAux64` — [`FileAux64`](#fileaux64)

##### `impl Copy for FileAux64`

##### `impl Debug for FileAux64`

- <span id="fileaux64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileAux for xcoff::FileAux64`

- <span id="xcofffileaux64-x-fname"></span>`fn x_fname(&self) -> &[u8; 8]`

- <span id="xcofffileaux64-x-ftype"></span>`fn x_ftype(&self) -> u8`

- <span id="xcofffileaux64-x-auxtype"></span>`fn x_auxtype(&self) -> Option<u8>`

##### `impl Pod for FileAux64`

### `CsectAux32`

```rust
struct CsectAux32 {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub x_parmhash: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snhash: crate::endian::U16<crate::endian::BigEndian>,
    pub x_smtyp: u8,
    pub x_smclas: u8,
    pub x_stab: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snstab: crate::endian::U16<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:573-588`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L573-L588)*

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section length.

- **`x_parmhash`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of parameter type-check hash in .typchk section.

- **`x_snhash`**: `crate::endian::U16<crate::endian::BigEndian>`

  .typchk section number.

- **`x_smtyp`**: `u8`

  Symbol alignment and type.

- **`x_smclas`**: `u8`

  Storage mapping class.

- **`x_stab`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved.

- **`x_snstab`**: `crate::endian::U16<crate::endian::BigEndian>`

  x_snstab.

#### Trait Implementations

##### `impl Clone for CsectAux32`

- <span id="csectaux32-clone"></span>`fn clone(&self) -> CsectAux32` — [`CsectAux32`](#csectaux32)

##### `impl Copy for CsectAux32`

##### `impl CsectAux for xcoff::CsectAux32`

- <span id="xcoffcsectaux32-x-scnlen"></span>`fn x_scnlen(&self) -> u64`

- <span id="xcoffcsectaux32-x-parmhash"></span>`fn x_parmhash(&self) -> u32`

- <span id="xcoffcsectaux32-x-snhash"></span>`fn x_snhash(&self) -> u16`

- <span id="xcoffcsectaux32-x-smtyp"></span>`fn x_smtyp(&self) -> u8`

- <span id="xcoffcsectaux32-x-smclas"></span>`fn x_smclas(&self) -> u8`

- <span id="xcoffcsectaux32-x-stab"></span>`fn x_stab(&self) -> Option<u32>`

- <span id="xcoffcsectaux32-x-snstab"></span>`fn x_snstab(&self) -> Option<u16>`

- <span id="xcoffcsectaux32-x-auxtype"></span>`fn x_auxtype(&self) -> Option<u8>`

##### `impl Debug for CsectAux32`

- <span id="csectaux32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for CsectAux32`

### `CsectAux64`

```rust
struct CsectAux64 {
    pub x_scnlen_lo: crate::endian::U32<crate::endian::BigEndian>,
    pub x_parmhash: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snhash: crate::endian::U16<crate::endian::BigEndian>,
    pub x_smtyp: u8,
    pub x_smclas: u8,
    pub x_scnlen_hi: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:593-610`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L593-L610)*

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

#### Fields

- **`x_scnlen_lo`**: `crate::endian::U32<crate::endian::BigEndian>`

  Low 4 bytes of section length.

- **`x_parmhash`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of parameter type-check hash in .typchk section.

- **`x_snhash`**: `crate::endian::U16<crate::endian::BigEndian>`

  .typchk section number.

- **`x_smtyp`**: `u8`

  Symbol alignment and type.

- **`x_smclas`**: `u8`

  Storage mapping class.

- **`x_scnlen_hi`**: `crate::endian::U32<crate::endian::BigEndian>`

  High 4 bytes of section length.

- **`pad`**: `u8`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_CSECT; indicates type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for CsectAux64`

- <span id="csectaux64-clone"></span>`fn clone(&self) -> CsectAux64` — [`CsectAux64`](#csectaux64)

##### `impl Copy for CsectAux64`

##### `impl CsectAux for xcoff::CsectAux64`

- <span id="xcoffcsectaux64-x-scnlen"></span>`fn x_scnlen(&self) -> u64`

- <span id="xcoffcsectaux64-x-parmhash"></span>`fn x_parmhash(&self) -> u32`

- <span id="xcoffcsectaux64-x-snhash"></span>`fn x_snhash(&self) -> u16`

- <span id="xcoffcsectaux64-x-smtyp"></span>`fn x_smtyp(&self) -> u8`

- <span id="xcoffcsectaux64-x-smclas"></span>`fn x_smclas(&self) -> u8`

- <span id="xcoffcsectaux64-x-stab"></span>`fn x_stab(&self) -> Option<u32>`

- <span id="xcoffcsectaux64-x-snstab"></span>`fn x_snstab(&self) -> Option<u16>`

- <span id="xcoffcsectaux64-x-auxtype"></span>`fn x_auxtype(&self) -> Option<u8>`

##### `impl Debug for CsectAux64`

- <span id="csectaux64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for CsectAux64`

### `FunAux32`

```rust
struct FunAux32 {
    pub x_exptr: crate::endian::U32<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_lnnoptr: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: crate::endian::U16<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:676-687`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L676-L687)*

Function auxiliary entry.

#### Fields

- **`x_exptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  File offset to exception table entry.

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_lnnoptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  File pointer to line number

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `crate::endian::U16<crate::endian::BigEndian>`

  Pad

#### Trait Implementations

##### `impl Clone for FunAux32`

- <span id="funaux32-clone"></span>`fn clone(&self) -> FunAux32` — [`FunAux32`](#funaux32)

##### `impl Copy for FunAux32`

##### `impl Debug for FunAux32`

- <span id="funaux32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for FunAux32`

### `FunAux64`

```rust
struct FunAux64 {
    pub x_lnnoptr: crate::endian::U64<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:692-703`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L692-L703)*

Function auxiliary entry.

#### Fields

- **`x_lnnoptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  File pointer to line number

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `u8`

  Pad

- **`x_auxtype`**: `u8`

  Contains _AUX_FCN; Type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for FunAux64`

- <span id="funaux64-clone"></span>`fn clone(&self) -> FunAux64` — [`FunAux64`](#funaux64)

##### `impl Copy for FunAux64`

##### `impl Debug for FunAux64`

- <span id="funaux64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for FunAux64`

### `ExpAux`

```rust
struct ExpAux {
    pub x_exptr: crate::endian::U64<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:708-719`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L708-L719)*

Exception auxiliary entry. (XCOFF64 only)

#### Fields

- **`x_exptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  File offset to exception table entry.

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `u8`

  Pad

- **`x_auxtype`**: `u8`

  Contains _AUX_EXCEPT; Type of auxiliary entry

#### Trait Implementations

##### `impl Clone for ExpAux`

- <span id="expaux-clone"></span>`fn clone(&self) -> ExpAux` — [`ExpAux`](#expaux)

##### `impl Copy for ExpAux`

##### `impl Debug for ExpAux`

- <span id="expaux-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for ExpAux`

### `BlockAux32`

```rust
struct BlockAux32 {
    pub pad: [u8; 2],
    pub x_lnnohi: crate::endian::U16<crate::endian::BigEndian>,
    pub x_lnnolo: crate::endian::U16<crate::endian::BigEndian>,
    pub pad2: [u8; 12],
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:724-733`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L724-L733)*

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

#### Fields

- **`pad`**: `[u8; 2]`

  Reserved.

- **`x_lnnohi`**: `crate::endian::U16<crate::endian::BigEndian>`

  High-order 2 bytes of the source line number.

- **`x_lnnolo`**: `crate::endian::U16<crate::endian::BigEndian>`

  Low-order 2 bytes of the source line number.

- **`pad2`**: `[u8; 12]`

  Reserved.

#### Trait Implementations

##### `impl Clone for BlockAux32`

- <span id="blockaux32-clone"></span>`fn clone(&self) -> BlockAux32` — [`BlockAux32`](#blockaux32)

##### `impl Copy for BlockAux32`

##### `impl Debug for BlockAux32`

- <span id="blockaux32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for BlockAux32`

### `BlockAux64`

```rust
struct BlockAux64 {
    pub x_lnno: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: [u8; 13],
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:738-745`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L738-L745)*

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

#### Fields

- **`x_lnno`**: `crate::endian::U32<crate::endian::BigEndian>`

  Source line number.

- **`pad`**: `[u8; 13]`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_SYM; Type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for BlockAux64`

- <span id="blockaux64-clone"></span>`fn clone(&self) -> BlockAux64` — [`BlockAux64`](#blockaux64)

##### `impl Copy for BlockAux64`

##### `impl Debug for BlockAux64`

- <span id="blockaux64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for BlockAux64`

### `StatAux`

```rust
struct StatAux {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub x_nreloc: crate::endian::U16<crate::endian::BigEndian>,
    pub x_nlinno: crate::endian::U16<crate::endian::BigEndian>,
    pub pad: [u8; 10],
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:750-759`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L750-L759)*

Section auxiliary entry for the C_STAT Symbol. (XCOFF32 Only)

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section length.

- **`x_nreloc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of relocation entries.

- **`x_nlinno`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of line numbers.

- **`pad`**: `[u8; 10]`

  Reserved.

#### Trait Implementations

##### `impl Clone for StatAux`

- <span id="stataux-clone"></span>`fn clone(&self) -> StatAux` — [`StatAux`](#stataux)

##### `impl Copy for StatAux`

##### `impl Debug for StatAux`

- <span id="stataux-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for StatAux`

### `DwarfAux32`

```rust
struct DwarfAux32 {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: [u8; 4],
    pub x_nreloc: crate::endian::U32<crate::endian::BigEndian>,
    pub pad2: [u8; 6],
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:764-773`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L764-L773)*

Section auxiliary entry Format for C_DWARF symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Length of portion of section represented by symbol.

- **`pad`**: `[u8; 4]`

  Reserved.

- **`x_nreloc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of relocation entries in section.

- **`pad2`**: `[u8; 6]`

  Reserved.

#### Trait Implementations

##### `impl Clone for DwarfAux32`

- <span id="dwarfaux32-clone"></span>`fn clone(&self) -> DwarfAux32` — [`DwarfAux32`](#dwarfaux32)

##### `impl Copy for DwarfAux32`

##### `impl Debug for DwarfAux32`

- <span id="dwarfaux32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for DwarfAux32`

### `DwarfAux64`

```rust
struct DwarfAux64 {
    pub x_scnlen: crate::endian::U64<crate::endian::BigEndian>,
    pub x_nreloc: crate::endian::U64<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:778-787`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L778-L787)*

Section auxiliary entry Format for C_DWARF symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U64<crate::endian::BigEndian>`

  Length of portion of section represented by symbol.

- **`x_nreloc`**: `crate::endian::U64<crate::endian::BigEndian>`

  Number of relocation entries in section.

- **`pad`**: `u8`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_SECT; Type of Auxiliary entry.

#### Trait Implementations

##### `impl Clone for DwarfAux64`

- <span id="dwarfaux64-clone"></span>`fn clone(&self) -> DwarfAux64` — [`DwarfAux64`](#dwarfaux64)

##### `impl Copy for DwarfAux64`

##### `impl Debug for DwarfAux64`

- <span id="dwarfaux64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for DwarfAux64`

### `Rel32`

```rust
struct Rel32 {
    pub r_vaddr: crate::endian::U32<crate::endian::BigEndian>,
    pub r_symndx: crate::endian::U32<crate::endian::BigEndian>,
    pub r_rsize: u8,
    pub r_rtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:807-816`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L807-L816)*

Relocation table entry

#### Fields

- **`r_vaddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Virtual address (position) in section to be relocated.

- **`r_symndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of item that is referenced.

- **`r_rsize`**: `u8`

  Relocation size and information.

- **`r_rtype`**: `u8`

  Relocation type.

#### Trait Implementations

##### `impl Clone for Rel32`

- <span id="rel32-clone"></span>`fn clone(&self) -> Rel32` — [`Rel32`](#rel32)

##### `impl Copy for Rel32`

##### `impl Debug for Rel32`

- <span id="rel32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for Rel32`

##### `impl Rel for xcoff::Rel32`

- <span id="xcoffrel32-type-word"></span>`type Word = u32`

- <span id="xcoffrel32-r-vaddr"></span>`fn r_vaddr(&self) -> <Self as >::Word` — [`Rel`](../read/xcoff/index.md)

- <span id="xcoffrel32-r-symndx"></span>`fn r_symndx(&self) -> u32`

- <span id="xcoffrel32-r-rsize"></span>`fn r_rsize(&self) -> u8`

- <span id="xcoffrel32-r-rtype"></span>`fn r_rtype(&self) -> u8`

### `Rel64`

```rust
struct Rel64 {
    pub r_vaddr: crate::endian::U64<crate::endian::BigEndian>,
    pub r_symndx: crate::endian::U32<crate::endian::BigEndian>,
    pub r_rsize: u8,
    pub r_rtype: u8,
}
```

*Defined in [`object-0.37.3/src/xcoff.rs:821-830`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L821-L830)*

Relocation table entry

#### Fields

- **`r_vaddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Virtual address (position) in section to be relocated.

- **`r_symndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of item that is referenced.

- **`r_rsize`**: `u8`

  Relocation size and information.

- **`r_rtype`**: `u8`

  Relocation type.

#### Trait Implementations

##### `impl Clone for Rel64`

- <span id="rel64-clone"></span>`fn clone(&self) -> Rel64` — [`Rel64`](#rel64)

##### `impl Copy for Rel64`

##### `impl Debug for Rel64`

- <span id="rel64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for Rel64`

##### `impl Rel for xcoff::Rel64`

- <span id="xcoffrel64-type-word"></span>`type Word = u64`

- <span id="xcoffrel64-r-vaddr"></span>`fn r_vaddr(&self) -> <Self as >::Word` — [`Rel`](../read/xcoff/index.md)

- <span id="xcoffrel64-r-symndx"></span>`fn r_symndx(&self) -> u32`

- <span id="xcoffrel64-r-rsize"></span>`fn r_rsize(&self) -> u8`

- <span id="xcoffrel64-r-rtype"></span>`fn r_rtype(&self) -> u8`

## Constants

### `MAGIC_64`
```rust
const MAGIC_64: u16 = 503u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:56`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L56)*

the 64-bit mach magic number

### `MAGIC_32`
```rust
const MAGIC_32: u16 = 479u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:58`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L58)*

the 32-bit mach magic number

### `F_RELFLG`
```rust
const F_RELFLG: u16 = 1u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:64`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L64)*

Indicates that the relocation information for binding has been removed from
the file.

### `F_EXEC`
```rust
const F_EXEC: u16 = 2u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:66`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L66)*

Indicates that the file is executable. No unresolved external references exist.

### `F_LNNO`
```rust
const F_LNNO: u16 = 4u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:68`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L68)*

Indicates that line numbers have been stripped from the file by a utility program.

### `F_FDPR_PROF`
```rust
const F_FDPR_PROF: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:70`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L70)*

Indicates that the file was profiled with the fdpr command.

### `F_FDPR_OPTI`
```rust
const F_FDPR_OPTI: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:72`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L72)*

Indicates that the file was reordered with the fdpr command.

### `F_DSA`
```rust
const F_DSA: u16 = 64u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:74`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L74)*

Indicates that the file uses Very Large Program Support.

### `F_VARPG`
```rust
const F_VARPG: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:77`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L77)*

Indicates that one of the members of the auxiliary header specifying the
medium page sizes is non-zero.

### `F_DYNLOAD`
```rust
const F_DYNLOAD: u16 = 4_096u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:81`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L81)*

Indicates the file is dynamically loadable and executable. External references
are resolved by way of imports, and the file might contain exports and loader
relocation.

### `F_SHROBJ`
```rust
const F_SHROBJ: u16 = 8_192u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:85`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L85)*

Indicates the file is a shared object (shared library). The file is separately
loadable. That is, it is not normally bound with other objects, and its loader
exports symbols are used as automatic import symbols for other object files.

### `F_LOADONLY`
```rust
const F_LOADONLY: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:89`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L89)*

If the object file is a member of an archive, it can be loaded by the system
loader, but the member is ignored by the binder. If the object file is not in
an archive, this flag has no effect.

### `AOUTHSZ_SHORT`
```rust
const AOUTHSZ_SHORT: u16 = 28u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:229`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L229)*

Some AIX programs generate auxiliary headers for 32-bit object files that
end after the data_start field.

### `STYP_REG`
```rust
const STYP_REG: u16 = 0u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:288`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L288)*

"regular" section

### `STYP_PAD`
```rust
const STYP_PAD: u16 = 8u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:293`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L293)*

Specifies a pad section. A section of this type is used to provide alignment
padding between sections within an XCOFF executable object file. This section
header type is obsolete since padding is allowed in an XCOFF file without a
corresponding pad section header.

### `STYP_DWARF`
```rust
const STYP_DWARF: u16 = 16u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:296`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L296)*

Specifies a DWARF debugging section, which provide source file and symbol
information for the symbolic debugger.

### `STYP_TEXT`
```rust
const STYP_TEXT: u16 = 32u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:299`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L299)*

Specifies an executable text (code) section. A section of this type contains
the executable instructions of a program.

### `STYP_DATA`
```rust
const STYP_DATA: u16 = 64u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:302`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L302)*

Specifies an initialized data section. A section of this type contains the
initialized data and the TOC of a program.

### `STYP_BSS`
```rust
const STYP_BSS: u16 = 128u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:305`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L305)*

Specifies an uninitialized data section. A section header of this type
defines the uninitialized data of a program.

### `STYP_EXCEPT`
```rust
const STYP_EXCEPT: u16 = 256u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:309`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L309)*

Specifies an exception section. A section of this type provides information
to identify the reason that a trap or exception occurred within an executable
object program.

### `STYP_INFO`
```rust
const STYP_INFO: u16 = 512u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:312`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L312)*

Specifies a comment section. A section of this type provides comments or data
to special processing utility programs.

### `STYP_TDATA`
```rust
const STYP_TDATA: u16 = 1_024u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:314`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L314)*

Specifies an initialized thread-local data section.

### `STYP_TBSS`
```rust
const STYP_TBSS: u16 = 2_048u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:316`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L316)*

Specifies an uninitialized thread-local data section.

### `STYP_LOADER`
```rust
const STYP_LOADER: u16 = 4_096u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:321`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L321)*

Specifies a loader section. A section of this type contains object file
information for the system loader to load an XCOFF executable. The information
includes imported symbols, exported symbols, relocation data, type-check
information, and shared object names.

### `STYP_DEBUG`
```rust
const STYP_DEBUG: u16 = 8_192u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:324`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L324)*

Specifies a debug section. A section of this type contains stabstring
information used by the symbolic debugger.

### `STYP_TYPCHK`
```rust
const STYP_TYPCHK: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:327`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L327)*

Specifies a type-check section. A section of this type contains
parameter/argument type-check strings used by the binder.

### `STYP_OVRFLO`
```rust
const STYP_OVRFLO: u16 = 32_768u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:332`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L332)*

Specifies a relocation or line-number field overflow section. A section
header of this type contains the count of relocation entries and line
number entries for some other section. This section header is required
when either of the counts exceeds 65,534.

### `SSUBTYP_DWINFO`
```rust
const SSUBTYP_DWINFO: u32 = 65_536u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:334`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L334)*

### `SSUBTYP_DWLINE`
```rust
const SSUBTYP_DWLINE: u32 = 131_072u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:335`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L335)*

### `SSUBTYP_DWPBNMS`
```rust
const SSUBTYP_DWPBNMS: u32 = 196_608u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:336`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L336)*

### `SSUBTYP_DWPBTYP`
```rust
const SSUBTYP_DWPBTYP: u32 = 262_144u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:337`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L337)*

### `SSUBTYP_DWARNGE`
```rust
const SSUBTYP_DWARNGE: u32 = 327_680u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:338`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L338)*

### `SSUBTYP_DWABREV`
```rust
const SSUBTYP_DWABREV: u32 = 393_216u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:339`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L339)*

### `SSUBTYP_DWSTR`
```rust
const SSUBTYP_DWSTR: u32 = 458_752u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:340`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L340)*

### `SSUBTYP_DWRNGES`
```rust
const SSUBTYP_DWRNGES: u32 = 524_288u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:341`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L341)*

### `SSUBTYP_DWLOC`
```rust
const SSUBTYP_DWLOC: u32 = 589_824u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:342`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L342)*

### `SSUBTYP_DWFRAME`
```rust
const SSUBTYP_DWFRAME: u32 = 655_360u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:343`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L343)*

### `SSUBTYP_DWMAC`
```rust
const SSUBTYP_DWMAC: u32 = 720_896u32;
```

*Defined in [`object-0.37.3/src/xcoff.rs:344`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L344)*

### `SIZEOF_SYMBOL`
```rust
const SIZEOF_SYMBOL: usize = 18usize;
```

*Defined in [`object-0.37.3/src/xcoff.rs:346`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L346)*

### `N_DEBUG`
```rust
const N_DEBUG: i16 = -2i16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:393`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L393)*

A special symbolic debugging symbol.

### `N_ABS`
```rust
const N_ABS: i16 = -1i16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:395`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L395)*

An absolute symbol. The symbol has a value but is not relocatable.

### `N_UNDEF`
```rust
const N_UNDEF: i16 = 0i16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:397`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L397)*

An undefined external symbol.

### `SYM_V_MASK`
```rust
const SYM_V_MASK: u16 = 61_440u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:404`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L404)*

Values for visibility as they would appear when encoded in the high 4 bits
of the 16-bit unsigned n_type field of symbol table entries. Valid for
32-bit XCOFF only when the o_vstamp in the auxiliary header is greater than 1.

### `SYM_V_INTERNAL`
```rust
const SYM_V_INTERNAL: u16 = 4_096u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:405`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L405)*

### `SYM_V_HIDDEN`
```rust
const SYM_V_HIDDEN: u16 = 8_192u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:406`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L406)*

### `SYM_V_PROTECTED`
```rust
const SYM_V_PROTECTED: u16 = 12_288u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:407`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L407)*

### `SYM_V_EXPORTED`
```rust
const SYM_V_EXPORTED: u16 = 16_384u16;
```

*Defined in [`object-0.37.3/src/xcoff.rs:408`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L408)*

### `C_FILE`
```rust
const C_FILE: u8 = 103u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:415`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L415)*

Source file name and compiler information.

### `C_BINCL`
```rust
const C_BINCL: u8 = 108u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:417`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L417)*

Beginning of include file.

### `C_EINCL`
```rust
const C_EINCL: u8 = 109u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:419`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L419)*

Ending of include file.

### `C_GSYM`
```rust
const C_GSYM: u8 = 128u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:421`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L421)*

Global variable.

### `C_STSYM`
```rust
const C_STSYM: u8 = 133u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:423`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L423)*

Statically allocated symbol.

### `C_BCOMM`
```rust
const C_BCOMM: u8 = 135u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:425`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L425)*

Beginning of common block.

### `C_ECOMM`
```rust
const C_ECOMM: u8 = 137u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:427`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L427)*

End of common block.

### `C_ENTRY`
```rust
const C_ENTRY: u8 = 141u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:429`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L429)*

Alternate entry.

### `C_BSTAT`
```rust
const C_BSTAT: u8 = 143u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:431`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L431)*

Beginning of static block.

### `C_ESTAT`
```rust
const C_ESTAT: u8 = 144u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:433`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L433)*

End of static block.

### `C_GTLS`
```rust
const C_GTLS: u8 = 145u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:435`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L435)*

Global thread-local variable.

### `C_STTLS`
```rust
const C_STTLS: u8 = 146u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:437`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L437)*

Static thread-local variable.

### `C_DWARF`
```rust
const C_DWARF: u8 = 112u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:439`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L439)*

DWARF section symbol.

### `C_LSYM`
```rust
const C_LSYM: u8 = 129u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:444`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L444)*

Automatic variable allocated on stack.

### `C_PSYM`
```rust
const C_PSYM: u8 = 130u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:446`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L446)*

Argument to subroutine allocated on stack.

### `C_RSYM`
```rust
const C_RSYM: u8 = 131u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:448`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L448)*

Register variable.

### `C_RPSYM`
```rust
const C_RPSYM: u8 = 132u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:450`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L450)*

Argument to function or procedure stored in register.

### `C_ECOML`
```rust
const C_ECOML: u8 = 136u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:452`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L452)*

Local member of common block.

### `C_FUN`
```rust
const C_FUN: u8 = 142u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:454`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L454)*

Function or procedure.

### `C_EXT`
```rust
const C_EXT: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:459`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L459)*

External symbol.

### `C_WEAKEXT`
```rust
const C_WEAKEXT: u8 = 111u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:461`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L461)*

Weak external symbol.

### `C_NULL`
```rust
const C_NULL: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:466`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L466)*

Symbol table entry marked for deletion.

### `C_STAT`
```rust
const C_STAT: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:468`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L468)*

Static.

### `C_BLOCK`
```rust
const C_BLOCK: u8 = 100u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:470`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L470)*

Beginning or end of inner block.

### `C_FCN`
```rust
const C_FCN: u8 = 101u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:472`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L472)*

Beginning or end of function.

### `C_HIDEXT`
```rust
const C_HIDEXT: u8 = 107u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:474`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L474)*

Un-named external symbol.

### `C_INFO`
```rust
const C_INFO: u8 = 110u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:476`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L476)*

Comment string in .info section.

### `C_DECL`
```rust
const C_DECL: u8 = 140u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:478`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L478)*

Declaration of object (type).

### `C_AUTO`
```rust
const C_AUTO: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:483`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L483)*

Automatic variable.

### `C_REG`
```rust
const C_REG: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:485`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L485)*

Register variable.

### `C_EXTDEF`
```rust
const C_EXTDEF: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:487`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L487)*

External definition.

### `C_LABEL`
```rust
const C_LABEL: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:489`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L489)*

Label.

### `C_ULABEL`
```rust
const C_ULABEL: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:491`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L491)*

Undefined label.

### `C_MOS`
```rust
const C_MOS: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:493`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L493)*

Member of structure.

### `C_ARG`
```rust
const C_ARG: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:495`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L495)*

Function argument.

### `C_STRTAG`
```rust
const C_STRTAG: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:497`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L497)*

Structure tag.

### `C_MOU`
```rust
const C_MOU: u8 = 11u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:499`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L499)*

Member of union.

### `C_UNTAG`
```rust
const C_UNTAG: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:501`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L501)*

Union tag.

### `C_TPDEF`
```rust
const C_TPDEF: u8 = 13u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:503`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L503)*

Type definition.

### `C_USTATIC`
```rust
const C_USTATIC: u8 = 14u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:505`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L505)*

Undefined static.

### `C_ENTAG`
```rust
const C_ENTAG: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:507`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L507)*

Enumeration tag.

### `C_MOE`
```rust
const C_MOE: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:509`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L509)*

Member of enumeration.

### `C_REGPARM`
```rust
const C_REGPARM: u8 = 17u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:511`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L511)*

Register parameter.

### `C_FIELD`
```rust
const C_FIELD: u8 = 18u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:513`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L513)*

Bit field.

### `C_EOS`
```rust
const C_EOS: u8 = 102u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:515`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L515)*

End of structure.

### `C_ALIAS`
```rust
const C_ALIAS: u8 = 105u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:517`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L517)*

Duplicate tag.

### `C_HIDDEN`
```rust
const C_HIDDEN: u8 = 106u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:519`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L519)*

Special storage class for external.

### `C_EFCN`
```rust
const C_EFCN: u8 = 255u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:521`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L521)*

Physical end of function.

### `C_TCSYM`
```rust
const C_TCSYM: u8 = 134u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:523`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L523)*

Reserved.

### `XFT_FN`
```rust
const XFT_FN: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:562`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L562)*

Specifies the source-file name.

### `XFT_CT`
```rust
const XFT_CT: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:564`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L564)*

Specifies the compiler time stamp.

### `XFT_CV`
```rust
const XFT_CV: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:566`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L566)*

Specifies the compiler version number.

### `XFT_CD`
```rust
const XFT_CD: u8 = 128u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:568`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L568)*

Specifies compiler-defined information.

### `XTY_ER`
```rust
const XTY_ER: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:615`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L615)*

External reference.

### `XTY_SD`
```rust
const XTY_SD: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:617`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L617)*

Csect definition for initialized storage.

### `XTY_LD`
```rust
const XTY_LD: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:619`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L619)*

Defines an entry point to an initialized csect.

### `XTY_CM`
```rust
const XTY_CM: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:621`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L621)*

Common csect definition. For uninitialized storage.

### `XMC_PR`
```rust
const XMC_PR: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:628`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L628)*

Program Code

### `XMC_RO`
```rust
const XMC_RO: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:630`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L630)*

Read Only Constant

### `XMC_DB`
```rust
const XMC_DB: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:632`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L632)*

Debug Dictionary Table

### `XMC_GL`
```rust
const XMC_GL: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:634`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L634)*

Global Linkage (Interfile Interface Code)

### `XMC_XO`
```rust
const XMC_XO: u8 = 7u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:636`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L636)*

Extended Operation (Pseudo Machine Instruction)

### `XMC_SV`
```rust
const XMC_SV: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:638`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L638)*

Supervisor Call (32-bit process only)

### `XMC_SV64`
```rust
const XMC_SV64: u8 = 17u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:640`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L640)*

Supervisor Call for 64-bit process

### `XMC_SV3264`
```rust
const XMC_SV3264: u8 = 18u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:642`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L642)*

Supervisor Call for both 32- and 64-bit processes

### `XMC_TI`
```rust
const XMC_TI: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:644`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L644)*

Traceback Index csect

### `XMC_TB`
```rust
const XMC_TB: u8 = 13u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:646`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L646)*

Traceback Table csect

### `XMC_RW`
```rust
const XMC_RW: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:651`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L651)*

Read Write Data

### `XMC_TC0`
```rust
const XMC_TC0: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:653`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L653)*

TOC Anchor for TOC Addressability

### `XMC_TC`
```rust
const XMC_TC: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:655`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L655)*

General TOC item

### `XMC_TD`
```rust
const XMC_TD: u8 = 16u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:657`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L657)*

Scalar data item in the TOC

### `XMC_DS`
```rust
const XMC_DS: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:659`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L659)*

Descriptor csect

### `XMC_UA`
```rust
const XMC_UA: u8 = 4u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:661`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L661)*

Unclassified - Treated as Read Write

### `XMC_BS`
```rust
const XMC_BS: u8 = 9u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:663`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L663)*

BSS class (uninitialized static internal)

### `XMC_UC`
```rust
const XMC_UC: u8 = 11u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:665`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L665)*

Un-named Fortran Common

### `XMC_TL`
```rust
const XMC_TL: u8 = 20u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:667`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L667)*

Initialized thread-local variable

### `XMC_UL`
```rust
const XMC_UL: u8 = 21u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:669`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L669)*

Uninitialized thread-local variable

### `XMC_TE`
```rust
const XMC_TE: u8 = 22u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:671`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L671)*

Symbol mapped at the end of TOC

### `AUX_EXCEPT`
```rust
const AUX_EXCEPT: u8 = 255u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:792`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L792)*

Identifies an exception auxiliary entry.

### `AUX_FCN`
```rust
const AUX_FCN: u8 = 254u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:794`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L794)*

Identifies a function auxiliary entry.

### `AUX_SYM`
```rust
const AUX_SYM: u8 = 253u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:796`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L796)*

Identifies a symbol auxiliary entry.

### `AUX_FILE`
```rust
const AUX_FILE: u8 = 252u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:798`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L798)*

Identifies a file auxiliary entry.

### `AUX_CSECT`
```rust
const AUX_CSECT: u8 = 251u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:800`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L800)*

Identifies a csect auxiliary entry.

### `AUX_SECT`
```rust
const AUX_SECT: u8 = 250u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:802`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L802)*

Identifies a SECT auxiliary entry.

### `R_POS`
```rust
const R_POS: u8 = 0u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:835`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L835)*

Positive relocation.

### `R_RL`
```rust
const R_RL: u8 = 12u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:837`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L837)*

Positive indirect load relocation.

### `R_RLA`
```rust
const R_RLA: u8 = 13u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:839`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L839)*

Positive load address relocation. Modifiable instruction.

### `R_NEG`
```rust
const R_NEG: u8 = 1u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:841`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L841)*

Negative relocation.

### `R_REL`
```rust
const R_REL: u8 = 2u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:843`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L843)*

Relative to self relocation.

### `R_TOC`
```rust
const R_TOC: u8 = 3u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:845`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L845)*

Relative to the TOC relocation.

### `R_TRL`
```rust
const R_TRL: u8 = 18u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:847`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L847)*

TOC relative indirect load relocation.

### `R_TRLA`
```rust
const R_TRLA: u8 = 19u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:849`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L849)*

Relative to the TOC or to the thread-local storage base relocation.

### `R_GL`
```rust
const R_GL: u8 = 5u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:851`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L851)*

Global linkage-external TOC address relocation.

### `R_TCL`
```rust
const R_TCL: u8 = 6u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:853`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L853)*

Local object TOC address relocation.

### `R_REF`
```rust
const R_REF: u8 = 15u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:855`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L855)*

A non-relocating relocation.

### `R_BA`
```rust
const R_BA: u8 = 8u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:857`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L857)*

Branch absolute relocation. References a non-modifiable instruction.

### `R_BR`
```rust
const R_BR: u8 = 10u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:859`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L859)*

Branch relative to self relocation. References a non-modifiable instruction.

### `R_RBA`
```rust
const R_RBA: u8 = 24u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:861`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L861)*

Branch absolute relocation. References a modifiable instruction.

### `R_RBR`
```rust
const R_RBR: u8 = 26u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:863`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L863)*

Branch relative to self relocation. References a modifiable instruction.

### `R_TLS`
```rust
const R_TLS: u8 = 32u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:865`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L865)*

General-dynamic reference to TLS symbol.

### `R_TLS_IE`
```rust
const R_TLS_IE: u8 = 33u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:867`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L867)*

Initial-exec reference to TLS symbol.

### `R_TLS_LD`
```rust
const R_TLS_LD: u8 = 34u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:869`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L869)*

Local-dynamic reference to TLS symbol.

### `R_TLS_LE`
```rust
const R_TLS_LE: u8 = 35u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:871`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L871)*

Local-exec reference to TLS symbol.

### `R_TLSM`
```rust
const R_TLSM: u8 = 36u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:873`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L873)*

Module reference to TLS.

### `R_TLSML`
```rust
const R_TLSML: u8 = 37u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:875`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L875)*

Module reference to the local TLS storage.

### `R_TOCU`
```rust
const R_TOCU: u8 = 48u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:877`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L877)*

Relative to TOC upper.

### `R_TOCL`
```rust
const R_TOCL: u8 = 49u8;
```

*Defined in [`object-0.37.3/src/xcoff.rs:879`](../../../.source_1765210505/object-0.37.3/src/xcoff.rs#L879)*

Relative to TOC lower.

