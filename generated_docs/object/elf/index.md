*[object](../index.md) / [elf](index.md)*

---

# Module `elf`

ELF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/elf.h, and is based heavily on it.

## Contents

- [Structs](#structs)
  - [`FileHeader32`](#fileheader32)
  - [`FileHeader64`](#fileheader64)
  - [`Ident`](#ident)
  - [`SectionHeader32`](#sectionheader32)
  - [`SectionHeader64`](#sectionheader64)
  - [`CompressionHeader32`](#compressionheader32)
  - [`CompressionHeader64`](#compressionheader64)
  - [`Sym32`](#sym32)
  - [`Sym64`](#sym64)
  - [`Syminfo32`](#syminfo32)
  - [`Syminfo64`](#syminfo64)
  - [`Rel32`](#rel32)
  - [`Rela32`](#rela32)
  - [`Rel64`](#rel64)
  - [`Rela64`](#rela64)
  - [`Relr32`](#relr32)
  - [`Relr64`](#relr64)
  - [`ProgramHeader32`](#programheader32)
  - [`ProgramHeader64`](#programheader64)
  - [`Dyn32`](#dyn32)
  - [`Dyn64`](#dyn64)
  - [`Versym`](#versym)
  - [`Verdef`](#verdef)
  - [`Verdaux`](#verdaux)
  - [`Verneed`](#verneed)
  - [`Vernaux`](#vernaux)
  - [`NoteHeader32`](#noteheader32)
  - [`NoteHeader64`](#noteheader64)
  - [`HashHeader`](#hashheader)
  - [`GnuHashHeader`](#gnuhashheader)
- [Functions](#functions)
  - [`hash`](#hash)
  - [`gnu_hash`](#gnu_hash)
  - [`ef_e2k_mach_to_flag`](#ef_e2k_mach_to_flag)
  - [`ef_e2k_flag_to_mach`](#ef_e2k_flag_to_mach)
- [Constants](#constants)
  - [`ELFMAG`](#elfmag)
  - [`ELFCLASSNONE`](#elfclassnone)
  - [`ELFCLASS32`](#elfclass32)
  - [`ELFCLASS64`](#elfclass64)
  - [`ELFDATANONE`](#elfdatanone)
  - [`ELFDATA2LSB`](#elfdata2lsb)
  - [`ELFDATA2MSB`](#elfdata2msb)
  - [`ELFOSABI_NONE`](#elfosabi_none)
  - [`ELFOSABI_SYSV`](#elfosabi_sysv)
  - [`ELFOSABI_HPUX`](#elfosabi_hpux)
  - [`ELFOSABI_NETBSD`](#elfosabi_netbsd)
  - [`ELFOSABI_GNU`](#elfosabi_gnu)
  - [`ELFOSABI_LINUX`](#elfosabi_linux)
  - [`ELFOSABI_HURD`](#elfosabi_hurd)
  - [`ELFOSABI_SOLARIS`](#elfosabi_solaris)
  - [`ELFOSABI_AIX`](#elfosabi_aix)
  - [`ELFOSABI_IRIX`](#elfosabi_irix)
  - [`ELFOSABI_FREEBSD`](#elfosabi_freebsd)
  - [`ELFOSABI_TRU64`](#elfosabi_tru64)
  - [`ELFOSABI_MODESTO`](#elfosabi_modesto)
  - [`ELFOSABI_OPENBSD`](#elfosabi_openbsd)
  - [`ELFOSABI_OPENVMS`](#elfosabi_openvms)
  - [`ELFOSABI_NSK`](#elfosabi_nsk)
  - [`ELFOSABI_AROS`](#elfosabi_aros)
  - [`ELFOSABI_FENIXOS`](#elfosabi_fenixos)
  - [`ELFOSABI_CLOUDABI`](#elfosabi_cloudabi)
  - [`ELFOSABI_ARM_AEABI`](#elfosabi_arm_aeabi)
  - [`ELFOSABI_ARM`](#elfosabi_arm)
  - [`ELFOSABI_STANDALONE`](#elfosabi_standalone)
  - [`ET_NONE`](#et_none)
  - [`ET_REL`](#et_rel)
  - [`ET_EXEC`](#et_exec)
  - [`ET_DYN`](#et_dyn)
  - [`ET_CORE`](#et_core)
  - [`ET_LOOS`](#et_loos)
  - [`ET_HIOS`](#et_hios)
  - [`ET_LOPROC`](#et_loproc)
  - [`ET_HIPROC`](#et_hiproc)
  - [`EM_NONE`](#em_none)
  - [`EM_M32`](#em_m32)
  - [`EM_SPARC`](#em_sparc)
  - [`EM_386`](#em_386)
  - [`EM_68K`](#em_68k)
  - [`EM_88K`](#em_88k)
  - [`EM_IAMCU`](#em_iamcu)
  - [`EM_860`](#em_860)
  - [`EM_MIPS`](#em_mips)
  - [`EM_S370`](#em_s370)
  - [`EM_MIPS_RS3_LE`](#em_mips_rs3_le)
  - [`EM_PARISC`](#em_parisc)
  - [`EM_VPP500`](#em_vpp500)
  - [`EM_SPARC32PLUS`](#em_sparc32plus)
  - [`EM_960`](#em_960)
  - [`EM_PPC`](#em_ppc)
  - [`EM_PPC64`](#em_ppc64)
  - [`EM_S390`](#em_s390)
  - [`EM_SPU`](#em_spu)
  - [`EM_V800`](#em_v800)
  - [`EM_FR20`](#em_fr20)
  - [`EM_RH32`](#em_rh32)
  - [`EM_RCE`](#em_rce)
  - [`EM_ARM`](#em_arm)
  - [`EM_FAKE_ALPHA`](#em_fake_alpha)
  - [`EM_SH`](#em_sh)
  - [`EM_SPARCV9`](#em_sparcv9)
  - [`EM_TRICORE`](#em_tricore)
  - [`EM_ARC`](#em_arc)
  - [`EM_H8_300`](#em_h8_300)
  - [`EM_H8_300H`](#em_h8_300h)
  - [`EM_H8S`](#em_h8s)
  - [`EM_H8_500`](#em_h8_500)
  - [`EM_IA_64`](#em_ia_64)
  - [`EM_MIPS_X`](#em_mips_x)
  - [`EM_COLDFIRE`](#em_coldfire)
  - [`EM_68HC12`](#em_68hc12)
  - [`EM_MMA`](#em_mma)
  - [`EM_PCP`](#em_pcp)
  - [`EM_NCPU`](#em_ncpu)
  - [`EM_NDR1`](#em_ndr1)
  - [`EM_STARCORE`](#em_starcore)
  - [`EM_ME16`](#em_me16)
  - [`EM_ST100`](#em_st100)
  - [`EM_TINYJ`](#em_tinyj)
  - [`EM_X86_64`](#em_x86_64)
  - [`EM_PDSP`](#em_pdsp)
  - [`EM_PDP10`](#em_pdp10)
  - [`EM_PDP11`](#em_pdp11)
  - [`EM_FX66`](#em_fx66)
  - [`EM_ST9PLUS`](#em_st9plus)
  - [`EM_ST7`](#em_st7)
  - [`EM_68HC16`](#em_68hc16)
  - [`EM_68HC11`](#em_68hc11)
  - [`EM_68HC08`](#em_68hc08)
  - [`EM_68HC05`](#em_68hc05)
  - [`EM_SVX`](#em_svx)
  - [`EM_ST19`](#em_st19)
  - [`EM_VAX`](#em_vax)
  - [`EM_CRIS`](#em_cris)
  - [`EM_JAVELIN`](#em_javelin)
  - [`EM_FIREPATH`](#em_firepath)
  - [`EM_ZSP`](#em_zsp)
  - [`EM_MMIX`](#em_mmix)
  - [`EM_HUANY`](#em_huany)
  - [`EM_PRISM`](#em_prism)
  - [`EM_AVR`](#em_avr)
  - [`EM_FR30`](#em_fr30)
  - [`EM_D10V`](#em_d10v)
  - [`EM_D30V`](#em_d30v)
  - [`EM_V850`](#em_v850)
  - [`EM_M32R`](#em_m32r)
  - [`EM_MN10300`](#em_mn10300)
  - [`EM_MN10200`](#em_mn10200)
  - [`EM_PJ`](#em_pj)
  - [`EM_OPENRISC`](#em_openrisc)
  - [`EM_ARC_COMPACT`](#em_arc_compact)
  - [`EM_XTENSA`](#em_xtensa)
  - [`EM_VIDEOCORE`](#em_videocore)
  - [`EM_TMM_GPP`](#em_tmm_gpp)
  - [`EM_NS32K`](#em_ns32k)
  - [`EM_TPC`](#em_tpc)
  - [`EM_SNP1K`](#em_snp1k)
  - [`EM_ST200`](#em_st200)
  - [`EM_IP2K`](#em_ip2k)
  - [`EM_MAX`](#em_max)
  - [`EM_CR`](#em_cr)
  - [`EM_F2MC16`](#em_f2mc16)
  - [`EM_MSP430`](#em_msp430)
  - [`EM_BLACKFIN`](#em_blackfin)
  - [`EM_SE_C33`](#em_se_c33)
  - [`EM_SEP`](#em_sep)
  - [`EM_ARCA`](#em_arca)
  - [`EM_UNICORE`](#em_unicore)
  - [`EM_EXCESS`](#em_excess)
  - [`EM_DXP`](#em_dxp)
  - [`EM_ALTERA_NIOS2`](#em_altera_nios2)
  - [`EM_CRX`](#em_crx)
  - [`EM_XGATE`](#em_xgate)
  - [`EM_C166`](#em_c166)
  - [`EM_M16C`](#em_m16c)
  - [`EM_DSPIC30F`](#em_dspic30f)
  - [`EM_CE`](#em_ce)
  - [`EM_M32C`](#em_m32c)
  - [`EM_TSK3000`](#em_tsk3000)
  - [`EM_RS08`](#em_rs08)
  - [`EM_SHARC`](#em_sharc)
  - [`EM_ECOG2`](#em_ecog2)
  - [`EM_SCORE7`](#em_score7)
  - [`EM_DSP24`](#em_dsp24)
  - [`EM_VIDEOCORE3`](#em_videocore3)
  - [`EM_LATTICEMICO32`](#em_latticemico32)
  - [`EM_SE_C17`](#em_se_c17)
  - [`EM_TI_C6000`](#em_ti_c6000)
  - [`EM_TI_C2000`](#em_ti_c2000)
  - [`EM_TI_C5500`](#em_ti_c5500)
  - [`EM_TI_ARP32`](#em_ti_arp32)
  - [`EM_TI_PRU`](#em_ti_pru)
  - [`EM_MMDSP_PLUS`](#em_mmdsp_plus)
  - [`EM_CYPRESS_M8C`](#em_cypress_m8c)
  - [`EM_R32C`](#em_r32c)
  - [`EM_TRIMEDIA`](#em_trimedia)
  - [`EM_HEXAGON`](#em_hexagon)
  - [`EM_8051`](#em_8051)
  - [`EM_STXP7X`](#em_stxp7x)
  - [`EM_NDS32`](#em_nds32)
  - [`EM_ECOG1X`](#em_ecog1x)
  - [`EM_MAXQ30`](#em_maxq30)
  - [`EM_XIMO16`](#em_ximo16)
  - [`EM_MANIK`](#em_manik)
  - [`EM_CRAYNV2`](#em_craynv2)
  - [`EM_RX`](#em_rx)
  - [`EM_METAG`](#em_metag)
  - [`EM_MCST_ELBRUS`](#em_mcst_elbrus)
  - [`EM_ECOG16`](#em_ecog16)
  - [`EM_CR16`](#em_cr16)
  - [`EM_ETPU`](#em_etpu)
  - [`EM_SLE9X`](#em_sle9x)
  - [`EM_L10M`](#em_l10m)
  - [`EM_K10M`](#em_k10m)
  - [`EM_AARCH64`](#em_aarch64)
  - [`EM_AVR32`](#em_avr32)
  - [`EM_STM8`](#em_stm8)
  - [`EM_TILE64`](#em_tile64)
  - [`EM_TILEPRO`](#em_tilepro)
  - [`EM_MICROBLAZE`](#em_microblaze)
  - [`EM_CUDA`](#em_cuda)
  - [`EM_TILEGX`](#em_tilegx)
  - [`EM_CLOUDSHIELD`](#em_cloudshield)
  - [`EM_COREA_1ST`](#em_corea_1st)
  - [`EM_COREA_2ND`](#em_corea_2nd)
  - [`EM_ARC_COMPACT2`](#em_arc_compact2)
  - [`EM_OPEN8`](#em_open8)
  - [`EM_RL78`](#em_rl78)
  - [`EM_VIDEOCORE5`](#em_videocore5)
  - [`EM_78KOR`](#em_78kor)
  - [`EM_56800EX`](#em_56800ex)
  - [`EM_BA1`](#em_ba1)
  - [`EM_BA2`](#em_ba2)
  - [`EM_XCORE`](#em_xcore)
  - [`EM_MCHP_PIC`](#em_mchp_pic)
  - [`EM_KM32`](#em_km32)
  - [`EM_KMX32`](#em_kmx32)
  - [`EM_EMX16`](#em_emx16)
  - [`EM_EMX8`](#em_emx8)
  - [`EM_KVARC`](#em_kvarc)
  - [`EM_CDP`](#em_cdp)
  - [`EM_COGE`](#em_coge)
  - [`EM_COOL`](#em_cool)
  - [`EM_NORC`](#em_norc)
  - [`EM_CSR_KALIMBA`](#em_csr_kalimba)
  - [`EM_Z80`](#em_z80)
  - [`EM_VISIUM`](#em_visium)
  - [`EM_FT32`](#em_ft32)
  - [`EM_MOXIE`](#em_moxie)
  - [`EM_AMDGPU`](#em_amdgpu)
  - [`EM_RISCV`](#em_riscv)
  - [`EM_BPF`](#em_bpf)
  - [`EM_CSKY`](#em_csky)
  - [`EM_LOONGARCH`](#em_loongarch)
  - [`EM_SBF`](#em_sbf)
  - [`EM_ALPHA`](#em_alpha)
  - [`EV_NONE`](#ev_none)
  - [`EV_CURRENT`](#ev_current)
  - [`SHN_UNDEF`](#shn_undef)
  - [`SHN_LORESERVE`](#shn_loreserve)
  - [`SHN_LOPROC`](#shn_loproc)
  - [`SHN_HIPROC`](#shn_hiproc)
  - [`SHN_LOOS`](#shn_loos)
  - [`SHN_HIOS`](#shn_hios)
  - [`SHN_ABS`](#shn_abs)
  - [`SHN_COMMON`](#shn_common)
  - [`SHN_XINDEX`](#shn_xindex)
  - [`SHN_HIRESERVE`](#shn_hireserve)
  - [`SHT_NULL`](#sht_null)
  - [`SHT_PROGBITS`](#sht_progbits)
  - [`SHT_SYMTAB`](#sht_symtab)
  - [`SHT_STRTAB`](#sht_strtab)
  - [`SHT_RELA`](#sht_rela)
  - [`SHT_HASH`](#sht_hash)
  - [`SHT_DYNAMIC`](#sht_dynamic)
  - [`SHT_NOTE`](#sht_note)
  - [`SHT_NOBITS`](#sht_nobits)
  - [`SHT_REL`](#sht_rel)
  - [`SHT_SHLIB`](#sht_shlib)
  - [`SHT_DYNSYM`](#sht_dynsym)
  - [`SHT_INIT_ARRAY`](#sht_init_array)
  - [`SHT_FINI_ARRAY`](#sht_fini_array)
  - [`SHT_PREINIT_ARRAY`](#sht_preinit_array)
  - [`SHT_GROUP`](#sht_group)
  - [`SHT_SYMTAB_SHNDX`](#sht_symtab_shndx)
  - [`SHT_RELR`](#sht_relr)
  - [`SHT_CREL`](#sht_crel)
  - [`SHT_LOOS`](#sht_loos)
  - [`SHT_LLVM_DEPENDENT_LIBRARIES`](#sht_llvm_dependent_libraries)
  - [`SHT_GNU_SFRAME`](#sht_gnu_sframe)
  - [`SHT_GNU_ATTRIBUTES`](#sht_gnu_attributes)
  - [`SHT_GNU_HASH`](#sht_gnu_hash)
  - [`SHT_GNU_LIBLIST`](#sht_gnu_liblist)
  - [`SHT_CHECKSUM`](#sht_checksum)
  - [`SHT_LOSUNW`](#sht_losunw)
  - [`SHT_SUNW_move`](#sht_sunw_move)
  - [`SHT_SUNW_COMDAT`](#sht_sunw_comdat)
  - [`SHT_SUNW_syminfo`](#sht_sunw_syminfo)
  - [`SHT_GNU_VERDEF`](#sht_gnu_verdef)
  - [`SHT_GNU_VERNEED`](#sht_gnu_verneed)
  - [`SHT_GNU_VERSYM`](#sht_gnu_versym)
  - [`SHT_HISUNW`](#sht_hisunw)
  - [`SHT_HIOS`](#sht_hios)
  - [`SHT_LOPROC`](#sht_loproc)
  - [`SHT_HIPROC`](#sht_hiproc)
  - [`SHT_LOUSER`](#sht_louser)
  - [`SHT_HIUSER`](#sht_hiuser)
  - [`SHF_WRITE`](#shf_write)
  - [`SHF_ALLOC`](#shf_alloc)
  - [`SHF_EXECINSTR`](#shf_execinstr)
  - [`SHF_MERGE`](#shf_merge)
  - [`SHF_STRINGS`](#shf_strings)
  - [`SHF_INFO_LINK`](#shf_info_link)
  - [`SHF_LINK_ORDER`](#shf_link_order)
  - [`SHF_OS_NONCONFORMING`](#shf_os_nonconforming)
  - [`SHF_GROUP`](#shf_group)
  - [`SHF_TLS`](#shf_tls)
  - [`SHF_COMPRESSED`](#shf_compressed)
  - [`SHF_MASKOS`](#shf_maskos)
  - [`SHF_GNU_RETAIN`](#shf_gnu_retain)
  - [`SHF_GNU_MBIND`](#shf_gnu_mbind)
  - [`SHF_MASKPROC`](#shf_maskproc)
  - [`SHF_EXCLUDE`](#shf_exclude)
  - [`ELFCOMPRESS_ZLIB`](#elfcompress_zlib)
  - [`ELFCOMPRESS_ZSTD`](#elfcompress_zstd)
  - [`ELFCOMPRESS_LOOS`](#elfcompress_loos)
  - [`ELFCOMPRESS_HIOS`](#elfcompress_hios)
  - [`ELFCOMPRESS_LOPROC`](#elfcompress_loproc)
  - [`ELFCOMPRESS_HIPROC`](#elfcompress_hiproc)
  - [`GRP_COMDAT`](#grp_comdat)
  - [`SYMINFO_BT_SELF`](#syminfo_bt_self)
  - [`SYMINFO_BT_PARENT`](#syminfo_bt_parent)
  - [`SYMINFO_BT_LOWRESERVE`](#syminfo_bt_lowreserve)
  - [`SYMINFO_FLG_DIRECT`](#syminfo_flg_direct)
  - [`SYMINFO_FLG_PASSTHRU`](#syminfo_flg_passthru)
  - [`SYMINFO_FLG_COPY`](#syminfo_flg_copy)
  - [`SYMINFO_FLG_LAZYLOAD`](#syminfo_flg_lazyload)
  - [`SYMINFO_NONE`](#syminfo_none)
  - [`SYMINFO_CURRENT`](#syminfo_current)
  - [`SYMINFO_NUM`](#syminfo_num)
  - [`STB_LOCAL`](#stb_local)
  - [`STB_GLOBAL`](#stb_global)
  - [`STB_WEAK`](#stb_weak)
  - [`STB_LOOS`](#stb_loos)
  - [`STB_GNU_UNIQUE`](#stb_gnu_unique)
  - [`STB_HIOS`](#stb_hios)
  - [`STB_LOPROC`](#stb_loproc)
  - [`STB_HIPROC`](#stb_hiproc)
  - [`STT_NOTYPE`](#stt_notype)
  - [`STT_OBJECT`](#stt_object)
  - [`STT_FUNC`](#stt_func)
  - [`STT_SECTION`](#stt_section)
  - [`STT_FILE`](#stt_file)
  - [`STT_COMMON`](#stt_common)
  - [`STT_TLS`](#stt_tls)
  - [`STT_LOOS`](#stt_loos)
  - [`STT_GNU_IFUNC`](#stt_gnu_ifunc)
  - [`STT_HIOS`](#stt_hios)
  - [`STT_LOPROC`](#stt_loproc)
  - [`STT_HIPROC`](#stt_hiproc)
  - [`STV_DEFAULT`](#stv_default)
  - [`STV_INTERNAL`](#stv_internal)
  - [`STV_HIDDEN`](#stv_hidden)
  - [`STV_PROTECTED`](#stv_protected)
  - [`PN_XNUM`](#pn_xnum)
  - [`PT_NULL`](#pt_null)
  - [`PT_LOAD`](#pt_load)
  - [`PT_DYNAMIC`](#pt_dynamic)
  - [`PT_INTERP`](#pt_interp)
  - [`PT_NOTE`](#pt_note)
  - [`PT_SHLIB`](#pt_shlib)
  - [`PT_PHDR`](#pt_phdr)
  - [`PT_TLS`](#pt_tls)
  - [`PT_LOOS`](#pt_loos)
  - [`PT_GNU_EH_FRAME`](#pt_gnu_eh_frame)
  - [`PT_GNU_STACK`](#pt_gnu_stack)
  - [`PT_GNU_RELRO`](#pt_gnu_relro)
  - [`PT_GNU_PROPERTY`](#pt_gnu_property)
  - [`PT_GNU_SFRAME`](#pt_gnu_sframe)
  - [`PT_HIOS`](#pt_hios)
  - [`PT_LOPROC`](#pt_loproc)
  - [`PT_HIPROC`](#pt_hiproc)
  - [`PF_X`](#pf_x)
  - [`PF_W`](#pf_w)
  - [`PF_R`](#pf_r)
  - [`PF_MASKOS`](#pf_maskos)
  - [`PF_MASKPROC`](#pf_maskproc)
  - [`ELF_NOTE_CORE`](#elf_note_core)
  - [`ELF_NOTE_LINUX`](#elf_note_linux)
  - [`NT_PRSTATUS`](#nt_prstatus)
  - [`NT_PRFPREG`](#nt_prfpreg)
  - [`NT_FPREGSET`](#nt_fpregset)
  - [`NT_PRPSINFO`](#nt_prpsinfo)
  - [`NT_PRXREG`](#nt_prxreg)
  - [`NT_TASKSTRUCT`](#nt_taskstruct)
  - [`NT_PLATFORM`](#nt_platform)
  - [`NT_AUXV`](#nt_auxv)
  - [`NT_GWINDOWS`](#nt_gwindows)
  - [`NT_ASRS`](#nt_asrs)
  - [`NT_PSTATUS`](#nt_pstatus)
  - [`NT_PSINFO`](#nt_psinfo)
  - [`NT_PRCRED`](#nt_prcred)
  - [`NT_UTSNAME`](#nt_utsname)
  - [`NT_LWPSTATUS`](#nt_lwpstatus)
  - [`NT_LWPSINFO`](#nt_lwpsinfo)
  - [`NT_PRFPXREG`](#nt_prfpxreg)
  - [`NT_SIGINFO`](#nt_siginfo)
  - [`NT_FILE`](#nt_file)
  - [`NT_PRXFPREG`](#nt_prxfpreg)
  - [`NT_PPC_VMX`](#nt_ppc_vmx)
  - [`NT_PPC_SPE`](#nt_ppc_spe)
  - [`NT_PPC_VSX`](#nt_ppc_vsx)
  - [`NT_PPC_TAR`](#nt_ppc_tar)
  - [`NT_PPC_PPR`](#nt_ppc_ppr)
  - [`NT_PPC_DSCR`](#nt_ppc_dscr)
  - [`NT_PPC_EBB`](#nt_ppc_ebb)
  - [`NT_PPC_PMU`](#nt_ppc_pmu)
  - [`NT_PPC_TM_CGPR`](#nt_ppc_tm_cgpr)
  - [`NT_PPC_TM_CFPR`](#nt_ppc_tm_cfpr)
  - [`NT_PPC_TM_CVMX`](#nt_ppc_tm_cvmx)
  - [`NT_PPC_TM_CVSX`](#nt_ppc_tm_cvsx)
  - [`NT_PPC_TM_SPR`](#nt_ppc_tm_spr)
  - [`NT_PPC_TM_CTAR`](#nt_ppc_tm_ctar)
  - [`NT_PPC_TM_CPPR`](#nt_ppc_tm_cppr)
  - [`NT_PPC_TM_CDSCR`](#nt_ppc_tm_cdscr)
  - [`NT_PPC_PKEY`](#nt_ppc_pkey)
  - [`NT_386_TLS`](#nt_386_tls)
  - [`NT_386_IOPERM`](#nt_386_ioperm)
  - [`NT_X86_XSTATE`](#nt_x86_xstate)
  - [`NT_S390_HIGH_GPRS`](#nt_s390_high_gprs)
  - [`NT_S390_TIMER`](#nt_s390_timer)
  - [`NT_S390_TODCMP`](#nt_s390_todcmp)
  - [`NT_S390_TODPREG`](#nt_s390_todpreg)
  - [`NT_S390_CTRS`](#nt_s390_ctrs)
  - [`NT_S390_PREFIX`](#nt_s390_prefix)
  - [`NT_S390_LAST_BREAK`](#nt_s390_last_break)
  - [`NT_S390_SYSTEM_CALL`](#nt_s390_system_call)
  - [`NT_S390_TDB`](#nt_s390_tdb)
  - [`NT_S390_VXRS_LOW`](#nt_s390_vxrs_low)
  - [`NT_S390_VXRS_HIGH`](#nt_s390_vxrs_high)
  - [`NT_S390_GS_CB`](#nt_s390_gs_cb)
  - [`NT_S390_GS_BC`](#nt_s390_gs_bc)
  - [`NT_S390_RI_CB`](#nt_s390_ri_cb)
  - [`NT_ARM_VFP`](#nt_arm_vfp)
  - [`NT_ARM_TLS`](#nt_arm_tls)
  - [`NT_ARM_HW_BREAK`](#nt_arm_hw_break)
  - [`NT_ARM_HW_WATCH`](#nt_arm_hw_watch)
  - [`NT_ARM_SYSTEM_CALL`](#nt_arm_system_call)
  - [`NT_ARM_SVE`](#nt_arm_sve)
  - [`NT_VMCOREDD`](#nt_vmcoredd)
  - [`NT_MIPS_DSP`](#nt_mips_dsp)
  - [`NT_MIPS_FP_MODE`](#nt_mips_fp_mode)
  - [`NT_VERSION`](#nt_version)
  - [`DT_NULL`](#dt_null)
  - [`DT_NEEDED`](#dt_needed)
  - [`DT_PLTRELSZ`](#dt_pltrelsz)
  - [`DT_PLTGOT`](#dt_pltgot)
  - [`DT_HASH`](#dt_hash)
  - [`DT_STRTAB`](#dt_strtab)
  - [`DT_SYMTAB`](#dt_symtab)
  - [`DT_RELA`](#dt_rela)
  - [`DT_RELASZ`](#dt_relasz)
  - [`DT_RELAENT`](#dt_relaent)
  - [`DT_STRSZ`](#dt_strsz)
  - [`DT_SYMENT`](#dt_syment)
  - [`DT_INIT`](#dt_init)
  - [`DT_FINI`](#dt_fini)
  - [`DT_SONAME`](#dt_soname)
  - [`DT_RPATH`](#dt_rpath)
  - [`DT_SYMBOLIC`](#dt_symbolic)
  - [`DT_REL`](#dt_rel)
  - [`DT_RELSZ`](#dt_relsz)
  - [`DT_RELENT`](#dt_relent)
  - [`DT_PLTREL`](#dt_pltrel)
  - [`DT_DEBUG`](#dt_debug)
  - [`DT_TEXTREL`](#dt_textrel)
  - [`DT_JMPREL`](#dt_jmprel)
  - [`DT_BIND_NOW`](#dt_bind_now)
  - [`DT_INIT_ARRAY`](#dt_init_array)
  - [`DT_FINI_ARRAY`](#dt_fini_array)
  - [`DT_INIT_ARRAYSZ`](#dt_init_arraysz)
  - [`DT_FINI_ARRAYSZ`](#dt_fini_arraysz)
  - [`DT_RUNPATH`](#dt_runpath)
  - [`DT_FLAGS`](#dt_flags)
  - [`DT_ENCODING`](#dt_encoding)
  - [`DT_PREINIT_ARRAY`](#dt_preinit_array)
  - [`DT_PREINIT_ARRAYSZ`](#dt_preinit_arraysz)
  - [`DT_SYMTAB_SHNDX`](#dt_symtab_shndx)
  - [`DT_LOOS`](#dt_loos)
  - [`DT_HIOS`](#dt_hios)
  - [`DT_LOPROC`](#dt_loproc)
  - [`DT_HIPROC`](#dt_hiproc)
  - [`DT_VALRNGLO`](#dt_valrnglo)
  - [`DT_GNU_PRELINKED`](#dt_gnu_prelinked)
  - [`DT_GNU_CONFLICTSZ`](#dt_gnu_conflictsz)
  - [`DT_GNU_LIBLISTSZ`](#dt_gnu_liblistsz)
  - [`DT_CHECKSUM`](#dt_checksum)
  - [`DT_PLTPADSZ`](#dt_pltpadsz)
  - [`DT_MOVEENT`](#dt_moveent)
  - [`DT_MOVESZ`](#dt_movesz)
  - [`DT_FEATURE_1`](#dt_feature_1)
  - [`DT_POSFLAG_1`](#dt_posflag_1)
  - [`DT_SYMINSZ`](#dt_syminsz)
  - [`DT_SYMINENT`](#dt_syminent)
  - [`DT_VALRNGHI`](#dt_valrnghi)
  - [`DT_ADDRRNGLO`](#dt_addrrnglo)
  - [`DT_GNU_HASH`](#dt_gnu_hash)
  - [`DT_TLSDESC_PLT`](#dt_tlsdesc_plt)
  - [`DT_TLSDESC_GOT`](#dt_tlsdesc_got)
  - [`DT_GNU_CONFLICT`](#dt_gnu_conflict)
  - [`DT_GNU_LIBLIST`](#dt_gnu_liblist)
  - [`DT_CONFIG`](#dt_config)
  - [`DT_DEPAUDIT`](#dt_depaudit)
  - [`DT_AUDIT`](#dt_audit)
  - [`DT_PLTPAD`](#dt_pltpad)
  - [`DT_MOVETAB`](#dt_movetab)
  - [`DT_SYMINFO`](#dt_syminfo)
  - [`DT_ADDRRNGHI`](#dt_addrrnghi)
  - [`DT_VERSYM`](#dt_versym)
  - [`DT_RELACOUNT`](#dt_relacount)
  - [`DT_RELCOUNT`](#dt_relcount)
  - [`DT_FLAGS_1`](#dt_flags_1)
  - [`DT_VERDEF`](#dt_verdef)
  - [`DT_VERDEFNUM`](#dt_verdefnum)
  - [`DT_VERNEED`](#dt_verneed)
  - [`DT_VERNEEDNUM`](#dt_verneednum)
  - [`DT_AUXILIARY`](#dt_auxiliary)
  - [`DT_FILTER`](#dt_filter)
  - [`DF_ORIGIN`](#df_origin)
  - [`DF_SYMBOLIC`](#df_symbolic)
  - [`DF_TEXTREL`](#df_textrel)
  - [`DF_BIND_NOW`](#df_bind_now)
  - [`DF_STATIC_TLS`](#df_static_tls)
  - [`DF_1_NOW`](#df_1_now)
  - [`DF_1_GLOBAL`](#df_1_global)
  - [`DF_1_GROUP`](#df_1_group)
  - [`DF_1_NODELETE`](#df_1_nodelete)
  - [`DF_1_LOADFLTR`](#df_1_loadfltr)
  - [`DF_1_INITFIRST`](#df_1_initfirst)
  - [`DF_1_NOOPEN`](#df_1_noopen)
  - [`DF_1_ORIGIN`](#df_1_origin)
  - [`DF_1_DIRECT`](#df_1_direct)
  - [`DF_1_TRANS`](#df_1_trans)
  - [`DF_1_INTERPOSE`](#df_1_interpose)
  - [`DF_1_NODEFLIB`](#df_1_nodeflib)
  - [`DF_1_NODUMP`](#df_1_nodump)
  - [`DF_1_CONFALT`](#df_1_confalt)
  - [`DF_1_ENDFILTEE`](#df_1_endfiltee)
  - [`DF_1_DISPRELDNE`](#df_1_dispreldne)
  - [`DF_1_DISPRELPND`](#df_1_disprelpnd)
  - [`DF_1_NODIRECT`](#df_1_nodirect)
  - [`DF_1_IGNMULDEF`](#df_1_ignmuldef)
  - [`DF_1_NOKSYMS`](#df_1_noksyms)
  - [`DF_1_NOHDR`](#df_1_nohdr)
  - [`DF_1_EDITED`](#df_1_edited)
  - [`DF_1_NORELOC`](#df_1_noreloc)
  - [`DF_1_SYMINTPOSE`](#df_1_symintpose)
  - [`DF_1_GLOBAUDIT`](#df_1_globaudit)
  - [`DF_1_SINGLETON`](#df_1_singleton)
  - [`DF_1_STUB`](#df_1_stub)
  - [`DF_1_PIE`](#df_1_pie)
  - [`VERSYM_HIDDEN`](#versym_hidden)
  - [`VERSYM_VERSION`](#versym_version)
  - [`VER_DEF_NONE`](#ver_def_none)
  - [`VER_DEF_CURRENT`](#ver_def_current)
  - [`VER_FLG_BASE`](#ver_flg_base)
  - [`VER_FLG_WEAK`](#ver_flg_weak)
  - [`VER_NDX_LOCAL`](#ver_ndx_local)
  - [`VER_NDX_GLOBAL`](#ver_ndx_global)
  - [`VER_NEED_NONE`](#ver_need_none)
  - [`VER_NEED_CURRENT`](#ver_need_current)
  - [`ELF_NOTE_SOLARIS`](#elf_note_solaris)
  - [`NT_SOLARIS_PAGESIZE_HINT`](#nt_solaris_pagesize_hint)
  - [`ELF_NOTE_GNU`](#elf_note_gnu)
  - [`ELF_NOTE_GO`](#elf_note_go)
  - [`NT_GNU_ABI_TAG`](#nt_gnu_abi_tag)
  - [`ELF_NOTE_OS_LINUX`](#elf_note_os_linux)
  - [`ELF_NOTE_OS_GNU`](#elf_note_os_gnu)
  - [`ELF_NOTE_OS_SOLARIS2`](#elf_note_os_solaris2)
  - [`ELF_NOTE_OS_FREEBSD`](#elf_note_os_freebsd)
  - [`NT_GNU_HWCAP`](#nt_gnu_hwcap)
  - [`NT_GNU_BUILD_ID`](#nt_gnu_build_id)
  - [`NT_GO_BUILD_ID`](#nt_go_build_id)
  - [`NT_GNU_GOLD_VERSION`](#nt_gnu_gold_version)
  - [`NT_GNU_PROPERTY_TYPE_0`](#nt_gnu_property_type_0)
  - [`GNU_PROPERTY_STACK_SIZE`](#gnu_property_stack_size)
  - [`GNU_PROPERTY_NO_COPY_ON_PROTECTED`](#gnu_property_no_copy_on_protected)
  - [`GNU_PROPERTY_UINT32_AND_LO`](#gnu_property_uint32_and_lo)
  - [`GNU_PROPERTY_UINT32_AND_HI`](#gnu_property_uint32_and_hi)
  - [`GNU_PROPERTY_UINT32_OR_LO`](#gnu_property_uint32_or_lo)
  - [`GNU_PROPERTY_UINT32_OR_HI`](#gnu_property_uint32_or_hi)
  - [`GNU_PROPERTY_1_NEEDED`](#gnu_property_1_needed)
  - [`GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS`](#gnu_property_1_needed_indirect_extern_access)
  - [`GNU_PROPERTY_LOPROC`](#gnu_property_loproc)
  - [`GNU_PROPERTY_HIPROC`](#gnu_property_hiproc)
  - [`GNU_PROPERTY_LOUSER`](#gnu_property_louser)
  - [`GNU_PROPERTY_HIUSER`](#gnu_property_hiuser)
  - [`GNU_PROPERTY_AARCH64_FEATURE_1_AND`](#gnu_property_aarch64_feature_1_and)
  - [`GNU_PROPERTY_AARCH64_FEATURE_PAUTH`](#gnu_property_aarch64_feature_pauth)
  - [`GNU_PROPERTY_AARCH64_FEATURE_1_BTI`](#gnu_property_aarch64_feature_1_bti)
  - [`GNU_PROPERTY_AARCH64_FEATURE_1_PAC`](#gnu_property_aarch64_feature_1_pac)
  - [`GNU_PROPERTY_X86_UINT32_AND_LO`](#gnu_property_x86_uint32_and_lo)
  - [`GNU_PROPERTY_X86_UINT32_AND_HI`](#gnu_property_x86_uint32_and_hi)
  - [`GNU_PROPERTY_X86_UINT32_OR_LO`](#gnu_property_x86_uint32_or_lo)
  - [`GNU_PROPERTY_X86_UINT32_OR_HI`](#gnu_property_x86_uint32_or_hi)
  - [`GNU_PROPERTY_X86_UINT32_OR_AND_LO`](#gnu_property_x86_uint32_or_and_lo)
  - [`GNU_PROPERTY_X86_UINT32_OR_AND_HI`](#gnu_property_x86_uint32_or_and_hi)
  - [`GNU_PROPERTY_X86_ISA_1_USED`](#gnu_property_x86_isa_1_used)
  - [`GNU_PROPERTY_X86_ISA_1_NEEDED`](#gnu_property_x86_isa_1_needed)
  - [`GNU_PROPERTY_X86_FEATURE_1_AND`](#gnu_property_x86_feature_1_and)
  - [`GNU_PROPERTY_X86_ISA_1_BASELINE`](#gnu_property_x86_isa_1_baseline)
  - [`GNU_PROPERTY_X86_ISA_1_V2`](#gnu_property_x86_isa_1_v2)
  - [`GNU_PROPERTY_X86_ISA_1_V3`](#gnu_property_x86_isa_1_v3)
  - [`GNU_PROPERTY_X86_ISA_1_V4`](#gnu_property_x86_isa_1_v4)
  - [`GNU_PROPERTY_X86_FEATURE_1_IBT`](#gnu_property_x86_feature_1_ibt)
  - [`GNU_PROPERTY_X86_FEATURE_1_SHSTK`](#gnu_property_x86_feature_1_shstk)
  - [`R_68K_NONE`](#r_68k_none)
  - [`R_68K_32`](#r_68k_32)
  - [`R_68K_16`](#r_68k_16)
  - [`R_68K_8`](#r_68k_8)
  - [`R_68K_PC32`](#r_68k_pc32)
  - [`R_68K_PC16`](#r_68k_pc16)
  - [`R_68K_PC8`](#r_68k_pc8)
  - [`R_68K_GOT32`](#r_68k_got32)
  - [`R_68K_GOT16`](#r_68k_got16)
  - [`R_68K_GOT8`](#r_68k_got8)
  - [`R_68K_GOT32O`](#r_68k_got32o)
  - [`R_68K_GOT16O`](#r_68k_got16o)
  - [`R_68K_GOT8O`](#r_68k_got8o)
  - [`R_68K_PLT32`](#r_68k_plt32)
  - [`R_68K_PLT16`](#r_68k_plt16)
  - [`R_68K_PLT8`](#r_68k_plt8)
  - [`R_68K_PLT32O`](#r_68k_plt32o)
  - [`R_68K_PLT16O`](#r_68k_plt16o)
  - [`R_68K_PLT8O`](#r_68k_plt8o)
  - [`R_68K_COPY`](#r_68k_copy)
  - [`R_68K_GLOB_DAT`](#r_68k_glob_dat)
  - [`R_68K_JMP_SLOT`](#r_68k_jmp_slot)
  - [`R_68K_RELATIVE`](#r_68k_relative)
  - [`R_68K_TLS_GD32`](#r_68k_tls_gd32)
  - [`R_68K_TLS_GD16`](#r_68k_tls_gd16)
  - [`R_68K_TLS_GD8`](#r_68k_tls_gd8)
  - [`R_68K_TLS_LDM32`](#r_68k_tls_ldm32)
  - [`R_68K_TLS_LDM16`](#r_68k_tls_ldm16)
  - [`R_68K_TLS_LDM8`](#r_68k_tls_ldm8)
  - [`R_68K_TLS_LDO32`](#r_68k_tls_ldo32)
  - [`R_68K_TLS_LDO16`](#r_68k_tls_ldo16)
  - [`R_68K_TLS_LDO8`](#r_68k_tls_ldo8)
  - [`R_68K_TLS_IE32`](#r_68k_tls_ie32)
  - [`R_68K_TLS_IE16`](#r_68k_tls_ie16)
  - [`R_68K_TLS_IE8`](#r_68k_tls_ie8)
  - [`R_68K_TLS_LE32`](#r_68k_tls_le32)
  - [`R_68K_TLS_LE16`](#r_68k_tls_le16)
  - [`R_68K_TLS_LE8`](#r_68k_tls_le8)
  - [`R_68K_TLS_DTPMOD32`](#r_68k_tls_dtpmod32)
  - [`R_68K_TLS_DTPREL32`](#r_68k_tls_dtprel32)
  - [`R_68K_TLS_TPREL32`](#r_68k_tls_tprel32)
  - [`R_386_NONE`](#r_386_none)
  - [`R_386_32`](#r_386_32)
  - [`R_386_PC32`](#r_386_pc32)
  - [`R_386_GOT32`](#r_386_got32)
  - [`R_386_PLT32`](#r_386_plt32)
  - [`R_386_COPY`](#r_386_copy)
  - [`R_386_GLOB_DAT`](#r_386_glob_dat)
  - [`R_386_JMP_SLOT`](#r_386_jmp_slot)
  - [`R_386_RELATIVE`](#r_386_relative)
  - [`R_386_GOTOFF`](#r_386_gotoff)
  - [`R_386_GOTPC`](#r_386_gotpc)
  - [`R_386_32PLT`](#r_386_32plt)
  - [`R_386_TLS_TPOFF`](#r_386_tls_tpoff)
  - [`R_386_TLS_IE`](#r_386_tls_ie)
  - [`R_386_TLS_GOTIE`](#r_386_tls_gotie)
  - [`R_386_TLS_LE`](#r_386_tls_le)
  - [`R_386_TLS_GD`](#r_386_tls_gd)
  - [`R_386_TLS_LDM`](#r_386_tls_ldm)
  - [`R_386_16`](#r_386_16)
  - [`R_386_PC16`](#r_386_pc16)
  - [`R_386_8`](#r_386_8)
  - [`R_386_PC8`](#r_386_pc8)
  - [`R_386_TLS_GD_32`](#r_386_tls_gd_32)
  - [`R_386_TLS_GD_PUSH`](#r_386_tls_gd_push)
  - [`R_386_TLS_GD_CALL`](#r_386_tls_gd_call)
  - [`R_386_TLS_GD_POP`](#r_386_tls_gd_pop)
  - [`R_386_TLS_LDM_32`](#r_386_tls_ldm_32)
  - [`R_386_TLS_LDM_PUSH`](#r_386_tls_ldm_push)
  - [`R_386_TLS_LDM_CALL`](#r_386_tls_ldm_call)
  - [`R_386_TLS_LDM_POP`](#r_386_tls_ldm_pop)
  - [`R_386_TLS_LDO_32`](#r_386_tls_ldo_32)
  - [`R_386_TLS_IE_32`](#r_386_tls_ie_32)
  - [`R_386_TLS_LE_32`](#r_386_tls_le_32)
  - [`R_386_TLS_DTPMOD32`](#r_386_tls_dtpmod32)
  - [`R_386_TLS_DTPOFF32`](#r_386_tls_dtpoff32)
  - [`R_386_TLS_TPOFF32`](#r_386_tls_tpoff32)
  - [`R_386_SIZE32`](#r_386_size32)
  - [`R_386_TLS_GOTDESC`](#r_386_tls_gotdesc)
  - [`R_386_TLS_DESC_CALL`](#r_386_tls_desc_call)
  - [`R_386_TLS_DESC`](#r_386_tls_desc)
  - [`R_386_IRELATIVE`](#r_386_irelative)
  - [`R_386_GOT32X`](#r_386_got32x)
  - [`R_SHARC_ADDR24_V3`](#r_sharc_addr24_v3)
  - [`R_SHARC_ADDR32_V3`](#r_sharc_addr32_v3)
  - [`R_SHARC_ADDR_VAR_V3`](#r_sharc_addr_var_v3)
  - [`R_SHARC_PCRSHORT_V3`](#r_sharc_pcrshort_v3)
  - [`R_SHARC_PCRLONG_V3`](#r_sharc_pcrlong_v3)
  - [`R_SHARC_DATA6_V3`](#r_sharc_data6_v3)
  - [`R_SHARC_DATA16_V3`](#r_sharc_data16_v3)
  - [`R_SHARC_DATA6_VISA_V3`](#r_sharc_data6_visa_v3)
  - [`R_SHARC_DATA7_VISA_V3`](#r_sharc_data7_visa_v3)
  - [`R_SHARC_DATA16_VISA_V3`](#r_sharc_data16_visa_v3)
  - [`R_SHARC_PCR6_VISA_V3`](#r_sharc_pcr6_visa_v3)
  - [`R_SHARC_ADDR_VAR16_V3`](#r_sharc_addr_var16_v3)
  - [`R_SHARC_CALC_PUSH_ADDR`](#r_sharc_calc_push_addr)
  - [`R_SHARC_CALC_PUSH_ADDEND`](#r_sharc_calc_push_addend)
  - [`R_SHARC_CALC_ADD`](#r_sharc_calc_add)
  - [`R_SHARC_CALC_SUB`](#r_sharc_calc_sub)
  - [`R_SHARC_CALC_MUL`](#r_sharc_calc_mul)
  - [`R_SHARC_CALC_DIV`](#r_sharc_calc_div)
  - [`R_SHARC_CALC_MOD`](#r_sharc_calc_mod)
  - [`R_SHARC_CALC_LSHIFT`](#r_sharc_calc_lshift)
  - [`R_SHARC_CALC_RSHIFT`](#r_sharc_calc_rshift)
  - [`R_SHARC_CALC_AND`](#r_sharc_calc_and)
  - [`R_SHARC_CALC_OR`](#r_sharc_calc_or)
  - [`R_SHARC_CALC_XOR`](#r_sharc_calc_xor)
  - [`R_SHARC_CALC_PUSH_LEN`](#r_sharc_calc_push_len)
  - [`R_SHARC_CALC_NOT`](#r_sharc_calc_not)
  - [`SHT_SHARC_ADI_ATTRIBUTES`](#sht_sharc_adi_attributes)
  - [`STT_SPARC_REGISTER`](#stt_sparc_register)
  - [`EF_SPARCV9_MM`](#ef_sparcv9_mm)
  - [`EF_SPARCV9_TSO`](#ef_sparcv9_tso)
  - [`EF_SPARCV9_PSO`](#ef_sparcv9_pso)
  - [`EF_SPARCV9_RMO`](#ef_sparcv9_rmo)
  - [`EF_SPARC_LEDATA`](#ef_sparc_ledata)
  - [`EF_SPARC_EXT_MASK`](#ef_sparc_ext_mask)
  - [`EF_SPARC_32PLUS`](#ef_sparc_32plus)
  - [`EF_SPARC_SUN_US1`](#ef_sparc_sun_us1)
  - [`EF_SPARC_HAL_R1`](#ef_sparc_hal_r1)
  - [`EF_SPARC_SUN_US3`](#ef_sparc_sun_us3)
  - [`R_SPARC_NONE`](#r_sparc_none)
  - [`R_SPARC_8`](#r_sparc_8)
  - [`R_SPARC_16`](#r_sparc_16)
  - [`R_SPARC_32`](#r_sparc_32)
  - [`R_SPARC_DISP8`](#r_sparc_disp8)
  - [`R_SPARC_DISP16`](#r_sparc_disp16)
  - [`R_SPARC_DISP32`](#r_sparc_disp32)
  - [`R_SPARC_WDISP30`](#r_sparc_wdisp30)
  - [`R_SPARC_WDISP22`](#r_sparc_wdisp22)
  - [`R_SPARC_HI22`](#r_sparc_hi22)
  - [`R_SPARC_22`](#r_sparc_22)
  - [`R_SPARC_13`](#r_sparc_13)
  - [`R_SPARC_LO10`](#r_sparc_lo10)
  - [`R_SPARC_GOT10`](#r_sparc_got10)
  - [`R_SPARC_GOT13`](#r_sparc_got13)
  - [`R_SPARC_GOT22`](#r_sparc_got22)
  - [`R_SPARC_PC10`](#r_sparc_pc10)
  - [`R_SPARC_PC22`](#r_sparc_pc22)
  - [`R_SPARC_WPLT30`](#r_sparc_wplt30)
  - [`R_SPARC_COPY`](#r_sparc_copy)
  - [`R_SPARC_GLOB_DAT`](#r_sparc_glob_dat)
  - [`R_SPARC_JMP_SLOT`](#r_sparc_jmp_slot)
  - [`R_SPARC_RELATIVE`](#r_sparc_relative)
  - [`R_SPARC_UA32`](#r_sparc_ua32)
  - [`R_SPARC_PLT32`](#r_sparc_plt32)
  - [`R_SPARC_HIPLT22`](#r_sparc_hiplt22)
  - [`R_SPARC_LOPLT10`](#r_sparc_loplt10)
  - [`R_SPARC_PCPLT32`](#r_sparc_pcplt32)
  - [`R_SPARC_PCPLT22`](#r_sparc_pcplt22)
  - [`R_SPARC_PCPLT10`](#r_sparc_pcplt10)
  - [`R_SPARC_10`](#r_sparc_10)
  - [`R_SPARC_11`](#r_sparc_11)
  - [`R_SPARC_64`](#r_sparc_64)
  - [`R_SPARC_OLO10`](#r_sparc_olo10)
  - [`R_SPARC_HH22`](#r_sparc_hh22)
  - [`R_SPARC_HM10`](#r_sparc_hm10)
  - [`R_SPARC_LM22`](#r_sparc_lm22)
  - [`R_SPARC_PC_HH22`](#r_sparc_pc_hh22)
  - [`R_SPARC_PC_HM10`](#r_sparc_pc_hm10)
  - [`R_SPARC_PC_LM22`](#r_sparc_pc_lm22)
  - [`R_SPARC_WDISP16`](#r_sparc_wdisp16)
  - [`R_SPARC_WDISP19`](#r_sparc_wdisp19)
  - [`R_SPARC_GLOB_JMP`](#r_sparc_glob_jmp)
  - [`R_SPARC_7`](#r_sparc_7)
  - [`R_SPARC_5`](#r_sparc_5)
  - [`R_SPARC_6`](#r_sparc_6)
  - [`R_SPARC_DISP64`](#r_sparc_disp64)
  - [`R_SPARC_PLT64`](#r_sparc_plt64)
  - [`R_SPARC_HIX22`](#r_sparc_hix22)
  - [`R_SPARC_LOX10`](#r_sparc_lox10)
  - [`R_SPARC_H44`](#r_sparc_h44)
  - [`R_SPARC_M44`](#r_sparc_m44)
  - [`R_SPARC_L44`](#r_sparc_l44)
  - [`R_SPARC_REGISTER`](#r_sparc_register)
  - [`R_SPARC_UA64`](#r_sparc_ua64)
  - [`R_SPARC_UA16`](#r_sparc_ua16)
  - [`R_SPARC_TLS_GD_HI22`](#r_sparc_tls_gd_hi22)
  - [`R_SPARC_TLS_GD_LO10`](#r_sparc_tls_gd_lo10)
  - [`R_SPARC_TLS_GD_ADD`](#r_sparc_tls_gd_add)
  - [`R_SPARC_TLS_GD_CALL`](#r_sparc_tls_gd_call)
  - [`R_SPARC_TLS_LDM_HI22`](#r_sparc_tls_ldm_hi22)
  - [`R_SPARC_TLS_LDM_LO10`](#r_sparc_tls_ldm_lo10)
  - [`R_SPARC_TLS_LDM_ADD`](#r_sparc_tls_ldm_add)
  - [`R_SPARC_TLS_LDM_CALL`](#r_sparc_tls_ldm_call)
  - [`R_SPARC_TLS_LDO_HIX22`](#r_sparc_tls_ldo_hix22)
  - [`R_SPARC_TLS_LDO_LOX10`](#r_sparc_tls_ldo_lox10)
  - [`R_SPARC_TLS_LDO_ADD`](#r_sparc_tls_ldo_add)
  - [`R_SPARC_TLS_IE_HI22`](#r_sparc_tls_ie_hi22)
  - [`R_SPARC_TLS_IE_LO10`](#r_sparc_tls_ie_lo10)
  - [`R_SPARC_TLS_IE_LD`](#r_sparc_tls_ie_ld)
  - [`R_SPARC_TLS_IE_LDX`](#r_sparc_tls_ie_ldx)
  - [`R_SPARC_TLS_IE_ADD`](#r_sparc_tls_ie_add)
  - [`R_SPARC_TLS_LE_HIX22`](#r_sparc_tls_le_hix22)
  - [`R_SPARC_TLS_LE_LOX10`](#r_sparc_tls_le_lox10)
  - [`R_SPARC_TLS_DTPMOD32`](#r_sparc_tls_dtpmod32)
  - [`R_SPARC_TLS_DTPMOD64`](#r_sparc_tls_dtpmod64)
  - [`R_SPARC_TLS_DTPOFF32`](#r_sparc_tls_dtpoff32)
  - [`R_SPARC_TLS_DTPOFF64`](#r_sparc_tls_dtpoff64)
  - [`R_SPARC_TLS_TPOFF32`](#r_sparc_tls_tpoff32)
  - [`R_SPARC_TLS_TPOFF64`](#r_sparc_tls_tpoff64)
  - [`R_SPARC_GOTDATA_HIX22`](#r_sparc_gotdata_hix22)
  - [`R_SPARC_GOTDATA_LOX10`](#r_sparc_gotdata_lox10)
  - [`R_SPARC_GOTDATA_OP_HIX22`](#r_sparc_gotdata_op_hix22)
  - [`R_SPARC_GOTDATA_OP_LOX10`](#r_sparc_gotdata_op_lox10)
  - [`R_SPARC_GOTDATA_OP`](#r_sparc_gotdata_op)
  - [`R_SPARC_H34`](#r_sparc_h34)
  - [`R_SPARC_SIZE32`](#r_sparc_size32)
  - [`R_SPARC_SIZE64`](#r_sparc_size64)
  - [`R_SPARC_WDISP10`](#r_sparc_wdisp10)
  - [`R_SPARC_JMP_IREL`](#r_sparc_jmp_irel)
  - [`R_SPARC_IRELATIVE`](#r_sparc_irelative)
  - [`R_SPARC_GNU_VTINHERIT`](#r_sparc_gnu_vtinherit)
  - [`R_SPARC_GNU_VTENTRY`](#r_sparc_gnu_vtentry)
  - [`R_SPARC_REV32`](#r_sparc_rev32)
  - [`DT_SPARC_REGISTER`](#dt_sparc_register)
  - [`EF_MIPS_NOREORDER`](#ef_mips_noreorder)
  - [`EF_MIPS_PIC`](#ef_mips_pic)
  - [`EF_MIPS_CPIC`](#ef_mips_cpic)
  - [`EF_MIPS_XGOT`](#ef_mips_xgot)
  - [`EF_MIPS_64BIT_WHIRL`](#ef_mips_64bit_whirl)
  - [`EF_MIPS_ABI2`](#ef_mips_abi2)
  - [`EF_MIPS_ABI_ON32`](#ef_mips_abi_on32)
  - [`EF_MIPS_FP64`](#ef_mips_fp64)
  - [`EF_MIPS_NAN2008`](#ef_mips_nan2008)
  - [`EF_MIPS_ARCH`](#ef_mips_arch)
  - [`EF_MIPS_ABI_O32`](#ef_mips_abi_o32)
  - [`EF_MIPS_ABI_O64`](#ef_mips_abi_o64)
  - [`EF_MIPS_ABI_EABI32`](#ef_mips_abi_eabi32)
  - [`EF_MIPS_ABI_EABI64`](#ef_mips_abi_eabi64)
  - [`EF_MIPS_ABI`](#ef_mips_abi)
  - [`EF_MIPS_ARCH_1`](#ef_mips_arch_1)
  - [`EF_MIPS_ARCH_2`](#ef_mips_arch_2)
  - [`EF_MIPS_ARCH_3`](#ef_mips_arch_3)
  - [`EF_MIPS_ARCH_4`](#ef_mips_arch_4)
  - [`EF_MIPS_ARCH_5`](#ef_mips_arch_5)
  - [`EF_MIPS_ARCH_32`](#ef_mips_arch_32)
  - [`EF_MIPS_ARCH_64`](#ef_mips_arch_64)
  - [`EF_MIPS_ARCH_32R2`](#ef_mips_arch_32r2)
  - [`EF_MIPS_ARCH_64R2`](#ef_mips_arch_64r2)
  - [`EF_MIPS_ARCH_32R6`](#ef_mips_arch_32r6)
  - [`EF_MIPS_ARCH_64R6`](#ef_mips_arch_64r6)
  - [`SHN_MIPS_ACOMMON`](#shn_mips_acommon)
  - [`SHN_MIPS_TEXT`](#shn_mips_text)
  - [`SHN_MIPS_DATA`](#shn_mips_data)
  - [`SHN_MIPS_SCOMMON`](#shn_mips_scommon)
  - [`SHN_MIPS_SUNDEFINED`](#shn_mips_sundefined)
  - [`SHT_MIPS_LIBLIST`](#sht_mips_liblist)
  - [`SHT_MIPS_MSYM`](#sht_mips_msym)
  - [`SHT_MIPS_CONFLICT`](#sht_mips_conflict)
  - [`SHT_MIPS_GPTAB`](#sht_mips_gptab)
  - [`SHT_MIPS_UCODE`](#sht_mips_ucode)
  - [`SHT_MIPS_DEBUG`](#sht_mips_debug)
  - [`SHT_MIPS_REGINFO`](#sht_mips_reginfo)
  - [`SHT_MIPS_PACKAGE`](#sht_mips_package)
  - [`SHT_MIPS_PACKSYM`](#sht_mips_packsym)
  - [`SHT_MIPS_RELD`](#sht_mips_reld)
  - [`SHT_MIPS_IFACE`](#sht_mips_iface)
  - [`SHT_MIPS_CONTENT`](#sht_mips_content)
  - [`SHT_MIPS_OPTIONS`](#sht_mips_options)
  - [`SHT_MIPS_SHDR`](#sht_mips_shdr)
  - [`SHT_MIPS_FDESC`](#sht_mips_fdesc)
  - [`SHT_MIPS_EXTSYM`](#sht_mips_extsym)
  - [`SHT_MIPS_DENSE`](#sht_mips_dense)
  - [`SHT_MIPS_PDESC`](#sht_mips_pdesc)
  - [`SHT_MIPS_LOCSYM`](#sht_mips_locsym)
  - [`SHT_MIPS_AUXSYM`](#sht_mips_auxsym)
  - [`SHT_MIPS_OPTSYM`](#sht_mips_optsym)
  - [`SHT_MIPS_LOCSTR`](#sht_mips_locstr)
  - [`SHT_MIPS_LINE`](#sht_mips_line)
  - [`SHT_MIPS_RFDESC`](#sht_mips_rfdesc)
  - [`SHT_MIPS_DELTASYM`](#sht_mips_deltasym)
  - [`SHT_MIPS_DELTAINST`](#sht_mips_deltainst)
  - [`SHT_MIPS_DELTACLASS`](#sht_mips_deltaclass)
  - [`SHT_MIPS_DWARF`](#sht_mips_dwarf)
  - [`SHT_MIPS_DELTADECL`](#sht_mips_deltadecl)
  - [`SHT_MIPS_SYMBOL_LIB`](#sht_mips_symbol_lib)
  - [`SHT_MIPS_EVENTS`](#sht_mips_events)
  - [`SHT_MIPS_TRANSLATE`](#sht_mips_translate)
  - [`SHT_MIPS_PIXIE`](#sht_mips_pixie)
  - [`SHT_MIPS_XLATE`](#sht_mips_xlate)
  - [`SHT_MIPS_XLATE_DEBUG`](#sht_mips_xlate_debug)
  - [`SHT_MIPS_WHIRL`](#sht_mips_whirl)
  - [`SHT_MIPS_EH_REGION`](#sht_mips_eh_region)
  - [`SHT_MIPS_XLATE_OLD`](#sht_mips_xlate_old)
  - [`SHT_MIPS_PDR_EXCEPTION`](#sht_mips_pdr_exception)
  - [`SHF_MIPS_GPREL`](#shf_mips_gprel)
  - [`SHF_MIPS_MERGE`](#shf_mips_merge)
  - [`SHF_MIPS_ADDR`](#shf_mips_addr)
  - [`SHF_MIPS_STRINGS`](#shf_mips_strings)
  - [`SHF_MIPS_NOSTRIP`](#shf_mips_nostrip)
  - [`SHF_MIPS_LOCAL`](#shf_mips_local)
  - [`SHF_MIPS_NAMES`](#shf_mips_names)
  - [`SHF_MIPS_NODUPE`](#shf_mips_nodupe)
  - [`STO_MIPS_PLT`](#sto_mips_plt)
  - [`STO_MIPS_SC_ALIGN_UNUSED`](#sto_mips_sc_align_unused)
  - [`STB_MIPS_SPLIT_COMMON`](#stb_mips_split_common)
  - [`ODK_NULL`](#odk_null)
  - [`ODK_REGINFO`](#odk_reginfo)
  - [`ODK_EXCEPTIONS`](#odk_exceptions)
  - [`ODK_PAD`](#odk_pad)
  - [`ODK_HWPATCH`](#odk_hwpatch)
  - [`ODK_FILL`](#odk_fill)
  - [`ODK_TAGS`](#odk_tags)
  - [`ODK_HWAND`](#odk_hwand)
  - [`ODK_HWOR`](#odk_hwor)
  - [`OEX_FPU_MIN`](#oex_fpu_min)
  - [`OEX_FPU_MAX`](#oex_fpu_max)
  - [`OEX_PAGE0`](#oex_page0)
  - [`OEX_SMM`](#oex_smm)
  - [`OEX_FPDBUG`](#oex_fpdbug)
  - [`OEX_PRECISEFP`](#oex_precisefp)
  - [`OEX_DISMISS`](#oex_dismiss)
  - [`OEX_FPU_INVAL`](#oex_fpu_inval)
  - [`OEX_FPU_DIV0`](#oex_fpu_div0)
  - [`OEX_FPU_OFLO`](#oex_fpu_oflo)
  - [`OEX_FPU_UFLO`](#oex_fpu_uflo)
  - [`OEX_FPU_INEX`](#oex_fpu_inex)
  - [`OHW_R4KEOP`](#ohw_r4keop)
  - [`OHW_R8KPFETCH`](#ohw_r8kpfetch)
  - [`OHW_R5KEOP`](#ohw_r5keop)
  - [`OHW_R5KCVTL`](#ohw_r5kcvtl)
  - [`OPAD_PREFIX`](#opad_prefix)
  - [`OPAD_POSTFIX`](#opad_postfix)
  - [`OPAD_SYMBOL`](#opad_symbol)
  - [`OHWA0_R4KEOP_CHECKED`](#ohwa0_r4keop_checked)
  - [`OHWA1_R4KEOP_CLEAN`](#ohwa1_r4keop_clean)
  - [`R_MIPS_NONE`](#r_mips_none)
  - [`R_MIPS_16`](#r_mips_16)
  - [`R_MIPS_32`](#r_mips_32)
  - [`R_MIPS_REL32`](#r_mips_rel32)
  - [`R_MIPS_26`](#r_mips_26)
  - [`R_MIPS_HI16`](#r_mips_hi16)
  - [`R_MIPS_LO16`](#r_mips_lo16)
  - [`R_MIPS_GPREL16`](#r_mips_gprel16)
  - [`R_MIPS_LITERAL`](#r_mips_literal)
  - [`R_MIPS_GOT16`](#r_mips_got16)
  - [`R_MIPS_PC16`](#r_mips_pc16)
  - [`R_MIPS_CALL16`](#r_mips_call16)
  - [`R_MIPS_GPREL32`](#r_mips_gprel32)
  - [`R_MIPS_SHIFT5`](#r_mips_shift5)
  - [`R_MIPS_SHIFT6`](#r_mips_shift6)
  - [`R_MIPS_64`](#r_mips_64)
  - [`R_MIPS_GOT_DISP`](#r_mips_got_disp)
  - [`R_MIPS_GOT_PAGE`](#r_mips_got_page)
  - [`R_MIPS_GOT_OFST`](#r_mips_got_ofst)
  - [`R_MIPS_GOT_HI16`](#r_mips_got_hi16)
  - [`R_MIPS_GOT_LO16`](#r_mips_got_lo16)
  - [`R_MIPS_SUB`](#r_mips_sub)
  - [`R_MIPS_INSERT_A`](#r_mips_insert_a)
  - [`R_MIPS_INSERT_B`](#r_mips_insert_b)
  - [`R_MIPS_DELETE`](#r_mips_delete)
  - [`R_MIPS_HIGHER`](#r_mips_higher)
  - [`R_MIPS_HIGHEST`](#r_mips_highest)
  - [`R_MIPS_CALL_HI16`](#r_mips_call_hi16)
  - [`R_MIPS_CALL_LO16`](#r_mips_call_lo16)
  - [`R_MIPS_SCN_DISP`](#r_mips_scn_disp)
  - [`R_MIPS_REL16`](#r_mips_rel16)
  - [`R_MIPS_ADD_IMMEDIATE`](#r_mips_add_immediate)
  - [`R_MIPS_PJUMP`](#r_mips_pjump)
  - [`R_MIPS_RELGOT`](#r_mips_relgot)
  - [`R_MIPS_JALR`](#r_mips_jalr)
  - [`R_MIPS_TLS_DTPMOD32`](#r_mips_tls_dtpmod32)
  - [`R_MIPS_TLS_DTPREL32`](#r_mips_tls_dtprel32)
  - [`R_MIPS_TLS_DTPMOD64`](#r_mips_tls_dtpmod64)
  - [`R_MIPS_TLS_DTPREL64`](#r_mips_tls_dtprel64)
  - [`R_MIPS_TLS_GD`](#r_mips_tls_gd)
  - [`R_MIPS_TLS_LDM`](#r_mips_tls_ldm)
  - [`R_MIPS_TLS_DTPREL_HI16`](#r_mips_tls_dtprel_hi16)
  - [`R_MIPS_TLS_DTPREL_LO16`](#r_mips_tls_dtprel_lo16)
  - [`R_MIPS_TLS_GOTTPREL`](#r_mips_tls_gottprel)
  - [`R_MIPS_TLS_TPREL32`](#r_mips_tls_tprel32)
  - [`R_MIPS_TLS_TPREL64`](#r_mips_tls_tprel64)
  - [`R_MIPS_TLS_TPREL_HI16`](#r_mips_tls_tprel_hi16)
  - [`R_MIPS_TLS_TPREL_LO16`](#r_mips_tls_tprel_lo16)
  - [`R_MIPS_GLOB_DAT`](#r_mips_glob_dat)
  - [`R_MIPS_COPY`](#r_mips_copy)
  - [`R_MIPS_JUMP_SLOT`](#r_mips_jump_slot)
  - [`PT_MIPS_REGINFO`](#pt_mips_reginfo)
  - [`PT_MIPS_RTPROC`](#pt_mips_rtproc)
  - [`PT_MIPS_OPTIONS`](#pt_mips_options)
  - [`PT_MIPS_ABIFLAGS`](#pt_mips_abiflags)
  - [`PF_MIPS_LOCAL`](#pf_mips_local)
  - [`DT_MIPS_RLD_VERSION`](#dt_mips_rld_version)
  - [`DT_MIPS_TIME_STAMP`](#dt_mips_time_stamp)
  - [`DT_MIPS_ICHECKSUM`](#dt_mips_ichecksum)
  - [`DT_MIPS_IVERSION`](#dt_mips_iversion)
  - [`DT_MIPS_FLAGS`](#dt_mips_flags)
  - [`DT_MIPS_BASE_ADDRESS`](#dt_mips_base_address)
  - [`DT_MIPS_MSYM`](#dt_mips_msym)
  - [`DT_MIPS_CONFLICT`](#dt_mips_conflict)
  - [`DT_MIPS_LIBLIST`](#dt_mips_liblist)
  - [`DT_MIPS_LOCAL_GOTNO`](#dt_mips_local_gotno)
  - [`DT_MIPS_CONFLICTNO`](#dt_mips_conflictno)
  - [`DT_MIPS_LIBLISTNO`](#dt_mips_liblistno)
  - [`DT_MIPS_SYMTABNO`](#dt_mips_symtabno)
  - [`DT_MIPS_UNREFEXTNO`](#dt_mips_unrefextno)
  - [`DT_MIPS_GOTSYM`](#dt_mips_gotsym)
  - [`DT_MIPS_HIPAGENO`](#dt_mips_hipageno)
  - [`DT_MIPS_RLD_MAP`](#dt_mips_rld_map)
  - [`DT_MIPS_DELTA_CLASS`](#dt_mips_delta_class)
  - [`DT_MIPS_DELTA_CLASS_NO`](#dt_mips_delta_class_no)
  - [`DT_MIPS_DELTA_INSTANCE`](#dt_mips_delta_instance)
  - [`DT_MIPS_DELTA_INSTANCE_NO`](#dt_mips_delta_instance_no)
  - [`DT_MIPS_DELTA_RELOC`](#dt_mips_delta_reloc)
  - [`DT_MIPS_DELTA_RELOC_NO`](#dt_mips_delta_reloc_no)
  - [`DT_MIPS_DELTA_SYM`](#dt_mips_delta_sym)
  - [`DT_MIPS_DELTA_SYM_NO`](#dt_mips_delta_sym_no)
  - [`DT_MIPS_DELTA_CLASSSYM`](#dt_mips_delta_classsym)
  - [`DT_MIPS_DELTA_CLASSSYM_NO`](#dt_mips_delta_classsym_no)
  - [`DT_MIPS_CXX_FLAGS`](#dt_mips_cxx_flags)
  - [`DT_MIPS_PIXIE_INIT`](#dt_mips_pixie_init)
  - [`DT_MIPS_SYMBOL_LIB`](#dt_mips_symbol_lib)
  - [`DT_MIPS_LOCALPAGE_GOTIDX`](#dt_mips_localpage_gotidx)
  - [`DT_MIPS_LOCAL_GOTIDX`](#dt_mips_local_gotidx)
  - [`DT_MIPS_HIDDEN_GOTIDX`](#dt_mips_hidden_gotidx)
  - [`DT_MIPS_PROTECTED_GOTIDX`](#dt_mips_protected_gotidx)
  - [`DT_MIPS_OPTIONS`](#dt_mips_options)
  - [`DT_MIPS_INTERFACE`](#dt_mips_interface)
  - [`DT_MIPS_DYNSTR_ALIGN`](#dt_mips_dynstr_align)
  - [`DT_MIPS_INTERFACE_SIZE`](#dt_mips_interface_size)
  - [`DT_MIPS_RLD_TEXT_RESOLVE_ADDR`](#dt_mips_rld_text_resolve_addr)
  - [`DT_MIPS_PERF_SUFFIX`](#dt_mips_perf_suffix)
  - [`DT_MIPS_COMPACT_SIZE`](#dt_mips_compact_size)
  - [`DT_MIPS_GP_VALUE`](#dt_mips_gp_value)
  - [`DT_MIPS_AUX_DYNAMIC`](#dt_mips_aux_dynamic)
  - [`DT_MIPS_PLTGOT`](#dt_mips_pltgot)
  - [`DT_MIPS_RWPLT`](#dt_mips_rwplt)
  - [`DT_MIPS_RLD_MAP_REL`](#dt_mips_rld_map_rel)
  - [`RHF_NONE`](#rhf_none)
  - [`RHF_QUICKSTART`](#rhf_quickstart)
  - [`RHF_NOTPOT`](#rhf_notpot)
  - [`RHF_NO_LIBRARY_REPLACEMENT`](#rhf_no_library_replacement)
  - [`RHF_NO_MOVE`](#rhf_no_move)
  - [`RHF_SGI_ONLY`](#rhf_sgi_only)
  - [`RHF_GUARANTEE_INIT`](#rhf_guarantee_init)
  - [`RHF_DELTA_C_PLUS_PLUS`](#rhf_delta_c_plus_plus)
  - [`RHF_GUARANTEE_START_INIT`](#rhf_guarantee_start_init)
  - [`RHF_PIXIE`](#rhf_pixie)
  - [`RHF_DEFAULT_DELAY_LOAD`](#rhf_default_delay_load)
  - [`RHF_REQUICKSTART`](#rhf_requickstart)
  - [`RHF_REQUICKSTARTED`](#rhf_requickstarted)
  - [`RHF_CORD`](#rhf_cord)
  - [`RHF_NO_UNRES_UNDEF`](#rhf_no_unres_undef)
  - [`RHF_RLD_ORDER_SAFE`](#rhf_rld_order_safe)
  - [`LL_NONE`](#ll_none)
  - [`LL_EXACT_MATCH`](#ll_exact_match)
  - [`LL_IGNORE_INT_VER`](#ll_ignore_int_ver)
  - [`LL_REQUIRE_MINOR`](#ll_require_minor)
  - [`LL_EXPORTS`](#ll_exports)
  - [`LL_DELAY_LOAD`](#ll_delay_load)
  - [`LL_DELTA`](#ll_delta)
  - [`EF_PARISC_TRAPNIL`](#ef_parisc_trapnil)
  - [`EF_PARISC_EXT`](#ef_parisc_ext)
  - [`EF_PARISC_LSB`](#ef_parisc_lsb)
  - [`EF_PARISC_WIDE`](#ef_parisc_wide)
  - [`EF_PARISC_NO_KABP`](#ef_parisc_no_kabp)
  - [`EF_PARISC_LAZYSWAP`](#ef_parisc_lazyswap)
  - [`EF_PARISC_ARCH`](#ef_parisc_arch)
  - [`EFA_PARISC_1_0`](#efa_parisc_1_0)
  - [`EFA_PARISC_1_1`](#efa_parisc_1_1)
  - [`EFA_PARISC_2_0`](#efa_parisc_2_0)
  - [`SHN_PARISC_ANSI_COMMON`](#shn_parisc_ansi_common)
  - [`SHN_PARISC_HUGE_COMMON`](#shn_parisc_huge_common)
  - [`SHT_PARISC_EXT`](#sht_parisc_ext)
  - [`SHT_PARISC_UNWIND`](#sht_parisc_unwind)
  - [`SHT_PARISC_DOC`](#sht_parisc_doc)
  - [`SHF_PARISC_SHORT`](#shf_parisc_short)
  - [`SHF_PARISC_HUGE`](#shf_parisc_huge)
  - [`SHF_PARISC_SBP`](#shf_parisc_sbp)
  - [`STT_PARISC_MILLICODE`](#stt_parisc_millicode)
  - [`STT_HP_OPAQUE`](#stt_hp_opaque)
  - [`STT_HP_STUB`](#stt_hp_stub)
  - [`R_PARISC_NONE`](#r_parisc_none)
  - [`R_PARISC_DIR32`](#r_parisc_dir32)
  - [`R_PARISC_DIR21L`](#r_parisc_dir21l)
  - [`R_PARISC_DIR17R`](#r_parisc_dir17r)
  - [`R_PARISC_DIR17F`](#r_parisc_dir17f)
  - [`R_PARISC_DIR14R`](#r_parisc_dir14r)
  - [`R_PARISC_PCREL32`](#r_parisc_pcrel32)
  - [`R_PARISC_PCREL21L`](#r_parisc_pcrel21l)
  - [`R_PARISC_PCREL17R`](#r_parisc_pcrel17r)
  - [`R_PARISC_PCREL17F`](#r_parisc_pcrel17f)
  - [`R_PARISC_PCREL14R`](#r_parisc_pcrel14r)
  - [`R_PARISC_DPREL21L`](#r_parisc_dprel21l)
  - [`R_PARISC_DPREL14R`](#r_parisc_dprel14r)
  - [`R_PARISC_GPREL21L`](#r_parisc_gprel21l)
  - [`R_PARISC_GPREL14R`](#r_parisc_gprel14r)
  - [`R_PARISC_LTOFF21L`](#r_parisc_ltoff21l)
  - [`R_PARISC_LTOFF14R`](#r_parisc_ltoff14r)
  - [`R_PARISC_SECREL32`](#r_parisc_secrel32)
  - [`R_PARISC_SEGBASE`](#r_parisc_segbase)
  - [`R_PARISC_SEGREL32`](#r_parisc_segrel32)
  - [`R_PARISC_PLTOFF21L`](#r_parisc_pltoff21l)
  - [`R_PARISC_PLTOFF14R`](#r_parisc_pltoff14r)
  - [`R_PARISC_LTOFF_FPTR32`](#r_parisc_ltoff_fptr32)
  - [`R_PARISC_LTOFF_FPTR21L`](#r_parisc_ltoff_fptr21l)
  - [`R_PARISC_LTOFF_FPTR14R`](#r_parisc_ltoff_fptr14r)
  - [`R_PARISC_FPTR64`](#r_parisc_fptr64)
  - [`R_PARISC_PLABEL32`](#r_parisc_plabel32)
  - [`R_PARISC_PLABEL21L`](#r_parisc_plabel21l)
  - [`R_PARISC_PLABEL14R`](#r_parisc_plabel14r)
  - [`R_PARISC_PCREL64`](#r_parisc_pcrel64)
  - [`R_PARISC_PCREL22F`](#r_parisc_pcrel22f)
  - [`R_PARISC_PCREL14WR`](#r_parisc_pcrel14wr)
  - [`R_PARISC_PCREL14DR`](#r_parisc_pcrel14dr)
  - [`R_PARISC_PCREL16F`](#r_parisc_pcrel16f)
  - [`R_PARISC_PCREL16WF`](#r_parisc_pcrel16wf)
  - [`R_PARISC_PCREL16DF`](#r_parisc_pcrel16df)
  - [`R_PARISC_DIR64`](#r_parisc_dir64)
  - [`R_PARISC_DIR14WR`](#r_parisc_dir14wr)
  - [`R_PARISC_DIR14DR`](#r_parisc_dir14dr)
  - [`R_PARISC_DIR16F`](#r_parisc_dir16f)
  - [`R_PARISC_DIR16WF`](#r_parisc_dir16wf)
  - [`R_PARISC_DIR16DF`](#r_parisc_dir16df)
  - [`R_PARISC_GPREL64`](#r_parisc_gprel64)
  - [`R_PARISC_GPREL14WR`](#r_parisc_gprel14wr)
  - [`R_PARISC_GPREL14DR`](#r_parisc_gprel14dr)
  - [`R_PARISC_GPREL16F`](#r_parisc_gprel16f)
  - [`R_PARISC_GPREL16WF`](#r_parisc_gprel16wf)
  - [`R_PARISC_GPREL16DF`](#r_parisc_gprel16df)
  - [`R_PARISC_LTOFF64`](#r_parisc_ltoff64)
  - [`R_PARISC_LTOFF14WR`](#r_parisc_ltoff14wr)
  - [`R_PARISC_LTOFF14DR`](#r_parisc_ltoff14dr)
  - [`R_PARISC_LTOFF16F`](#r_parisc_ltoff16f)
  - [`R_PARISC_LTOFF16WF`](#r_parisc_ltoff16wf)
  - [`R_PARISC_LTOFF16DF`](#r_parisc_ltoff16df)
  - [`R_PARISC_SECREL64`](#r_parisc_secrel64)
  - [`R_PARISC_SEGREL64`](#r_parisc_segrel64)
  - [`R_PARISC_PLTOFF14WR`](#r_parisc_pltoff14wr)
  - [`R_PARISC_PLTOFF14DR`](#r_parisc_pltoff14dr)
  - [`R_PARISC_PLTOFF16F`](#r_parisc_pltoff16f)
  - [`R_PARISC_PLTOFF16WF`](#r_parisc_pltoff16wf)
  - [`R_PARISC_PLTOFF16DF`](#r_parisc_pltoff16df)
  - [`R_PARISC_LTOFF_FPTR64`](#r_parisc_ltoff_fptr64)
  - [`R_PARISC_LTOFF_FPTR14WR`](#r_parisc_ltoff_fptr14wr)
  - [`R_PARISC_LTOFF_FPTR14DR`](#r_parisc_ltoff_fptr14dr)
  - [`R_PARISC_LTOFF_FPTR16F`](#r_parisc_ltoff_fptr16f)
  - [`R_PARISC_LTOFF_FPTR16WF`](#r_parisc_ltoff_fptr16wf)
  - [`R_PARISC_LTOFF_FPTR16DF`](#r_parisc_ltoff_fptr16df)
  - [`R_PARISC_LORESERVE`](#r_parisc_loreserve)
  - [`R_PARISC_COPY`](#r_parisc_copy)
  - [`R_PARISC_IPLT`](#r_parisc_iplt)
  - [`R_PARISC_EPLT`](#r_parisc_eplt)
  - [`R_PARISC_TPREL32`](#r_parisc_tprel32)
  - [`R_PARISC_TPREL21L`](#r_parisc_tprel21l)
  - [`R_PARISC_TPREL14R`](#r_parisc_tprel14r)
  - [`R_PARISC_LTOFF_TP21L`](#r_parisc_ltoff_tp21l)
  - [`R_PARISC_LTOFF_TP14R`](#r_parisc_ltoff_tp14r)
  - [`R_PARISC_LTOFF_TP14F`](#r_parisc_ltoff_tp14f)
  - [`R_PARISC_TPREL64`](#r_parisc_tprel64)
  - [`R_PARISC_TPREL14WR`](#r_parisc_tprel14wr)
  - [`R_PARISC_TPREL14DR`](#r_parisc_tprel14dr)
  - [`R_PARISC_TPREL16F`](#r_parisc_tprel16f)
  - [`R_PARISC_TPREL16WF`](#r_parisc_tprel16wf)
  - [`R_PARISC_TPREL16DF`](#r_parisc_tprel16df)
  - [`R_PARISC_LTOFF_TP64`](#r_parisc_ltoff_tp64)
  - [`R_PARISC_LTOFF_TP14WR`](#r_parisc_ltoff_tp14wr)
  - [`R_PARISC_LTOFF_TP14DR`](#r_parisc_ltoff_tp14dr)
  - [`R_PARISC_LTOFF_TP16F`](#r_parisc_ltoff_tp16f)
  - [`R_PARISC_LTOFF_TP16WF`](#r_parisc_ltoff_tp16wf)
  - [`R_PARISC_LTOFF_TP16DF`](#r_parisc_ltoff_tp16df)
  - [`R_PARISC_GNU_VTENTRY`](#r_parisc_gnu_vtentry)
  - [`R_PARISC_GNU_VTINHERIT`](#r_parisc_gnu_vtinherit)
  - [`R_PARISC_TLS_GD21L`](#r_parisc_tls_gd21l)
  - [`R_PARISC_TLS_GD14R`](#r_parisc_tls_gd14r)
  - [`R_PARISC_TLS_GDCALL`](#r_parisc_tls_gdcall)
  - [`R_PARISC_TLS_LDM21L`](#r_parisc_tls_ldm21l)
  - [`R_PARISC_TLS_LDM14R`](#r_parisc_tls_ldm14r)
  - [`R_PARISC_TLS_LDMCALL`](#r_parisc_tls_ldmcall)
  - [`R_PARISC_TLS_LDO21L`](#r_parisc_tls_ldo21l)
  - [`R_PARISC_TLS_LDO14R`](#r_parisc_tls_ldo14r)
  - [`R_PARISC_TLS_DTPMOD32`](#r_parisc_tls_dtpmod32)
  - [`R_PARISC_TLS_DTPMOD64`](#r_parisc_tls_dtpmod64)
  - [`R_PARISC_TLS_DTPOFF32`](#r_parisc_tls_dtpoff32)
  - [`R_PARISC_TLS_DTPOFF64`](#r_parisc_tls_dtpoff64)
  - [`R_PARISC_TLS_LE21L`](#r_parisc_tls_le21l)
  - [`R_PARISC_TLS_LE14R`](#r_parisc_tls_le14r)
  - [`R_PARISC_TLS_IE21L`](#r_parisc_tls_ie21l)
  - [`R_PARISC_TLS_IE14R`](#r_parisc_tls_ie14r)
  - [`R_PARISC_TLS_TPREL32`](#r_parisc_tls_tprel32)
  - [`R_PARISC_TLS_TPREL64`](#r_parisc_tls_tprel64)
  - [`R_PARISC_HIRESERVE`](#r_parisc_hireserve)
  - [`PT_HP_TLS`](#pt_hp_tls)
  - [`PT_HP_CORE_NONE`](#pt_hp_core_none)
  - [`PT_HP_CORE_VERSION`](#pt_hp_core_version)
  - [`PT_HP_CORE_KERNEL`](#pt_hp_core_kernel)
  - [`PT_HP_CORE_COMM`](#pt_hp_core_comm)
  - [`PT_HP_CORE_PROC`](#pt_hp_core_proc)
  - [`PT_HP_CORE_LOADABLE`](#pt_hp_core_loadable)
  - [`PT_HP_CORE_STACK`](#pt_hp_core_stack)
  - [`PT_HP_CORE_SHM`](#pt_hp_core_shm)
  - [`PT_HP_CORE_MMF`](#pt_hp_core_mmf)
  - [`PT_HP_PARALLEL`](#pt_hp_parallel)
  - [`PT_HP_FASTBIND`](#pt_hp_fastbind)
  - [`PT_HP_OPT_ANNOT`](#pt_hp_opt_annot)
  - [`PT_HP_HSL_ANNOT`](#pt_hp_hsl_annot)
  - [`PT_HP_STACK`](#pt_hp_stack)
  - [`PT_PARISC_ARCHEXT`](#pt_parisc_archext)
  - [`PT_PARISC_UNWIND`](#pt_parisc_unwind)
  - [`PF_PARISC_SBP`](#pf_parisc_sbp)
  - [`PF_HP_PAGE_SIZE`](#pf_hp_page_size)
  - [`PF_HP_FAR_SHARED`](#pf_hp_far_shared)
  - [`PF_HP_NEAR_SHARED`](#pf_hp_near_shared)
  - [`PF_HP_CODE`](#pf_hp_code)
  - [`PF_HP_MODIFY`](#pf_hp_modify)
  - [`PF_HP_LAZYSWAP`](#pf_hp_lazyswap)
  - [`PF_HP_SBP`](#pf_hp_sbp)
  - [`EF_ALPHA_32BIT`](#ef_alpha_32bit)
  - [`EF_ALPHA_CANRELAX`](#ef_alpha_canrelax)
  - [`SHT_ALPHA_DEBUG`](#sht_alpha_debug)
  - [`SHT_ALPHA_REGINFO`](#sht_alpha_reginfo)
  - [`SHF_ALPHA_GPREL`](#shf_alpha_gprel)
  - [`STO_ALPHA_NOPV`](#sto_alpha_nopv)
  - [`STO_ALPHA_STD_GPLOAD`](#sto_alpha_std_gpload)
  - [`R_ALPHA_NONE`](#r_alpha_none)
  - [`R_ALPHA_REFLONG`](#r_alpha_reflong)
  - [`R_ALPHA_REFQUAD`](#r_alpha_refquad)
  - [`R_ALPHA_GPREL32`](#r_alpha_gprel32)
  - [`R_ALPHA_LITERAL`](#r_alpha_literal)
  - [`R_ALPHA_LITUSE`](#r_alpha_lituse)
  - [`R_ALPHA_GPDISP`](#r_alpha_gpdisp)
  - [`R_ALPHA_BRADDR`](#r_alpha_braddr)
  - [`R_ALPHA_HINT`](#r_alpha_hint)
  - [`R_ALPHA_SREL16`](#r_alpha_srel16)
  - [`R_ALPHA_SREL32`](#r_alpha_srel32)
  - [`R_ALPHA_SREL64`](#r_alpha_srel64)
  - [`R_ALPHA_GPRELHIGH`](#r_alpha_gprelhigh)
  - [`R_ALPHA_GPRELLOW`](#r_alpha_gprellow)
  - [`R_ALPHA_GPREL16`](#r_alpha_gprel16)
  - [`R_ALPHA_COPY`](#r_alpha_copy)
  - [`R_ALPHA_GLOB_DAT`](#r_alpha_glob_dat)
  - [`R_ALPHA_JMP_SLOT`](#r_alpha_jmp_slot)
  - [`R_ALPHA_RELATIVE`](#r_alpha_relative)
  - [`R_ALPHA_TLS_GD_HI`](#r_alpha_tls_gd_hi)
  - [`R_ALPHA_TLSGD`](#r_alpha_tlsgd)
  - [`R_ALPHA_TLS_LDM`](#r_alpha_tls_ldm)
  - [`R_ALPHA_DTPMOD64`](#r_alpha_dtpmod64)
  - [`R_ALPHA_GOTDTPREL`](#r_alpha_gotdtprel)
  - [`R_ALPHA_DTPREL64`](#r_alpha_dtprel64)
  - [`R_ALPHA_DTPRELHI`](#r_alpha_dtprelhi)
  - [`R_ALPHA_DTPRELLO`](#r_alpha_dtprello)
  - [`R_ALPHA_DTPREL16`](#r_alpha_dtprel16)
  - [`R_ALPHA_GOTTPREL`](#r_alpha_gottprel)
  - [`R_ALPHA_TPREL64`](#r_alpha_tprel64)
  - [`R_ALPHA_TPRELHI`](#r_alpha_tprelhi)
  - [`R_ALPHA_TPRELLO`](#r_alpha_tprello)
  - [`R_ALPHA_TPREL16`](#r_alpha_tprel16)
  - [`LITUSE_ALPHA_ADDR`](#lituse_alpha_addr)
  - [`LITUSE_ALPHA_BASE`](#lituse_alpha_base)
  - [`LITUSE_ALPHA_BYTOFF`](#lituse_alpha_bytoff)
  - [`LITUSE_ALPHA_JSR`](#lituse_alpha_jsr)
  - [`LITUSE_ALPHA_TLS_GD`](#lituse_alpha_tls_gd)
  - [`LITUSE_ALPHA_TLS_LDM`](#lituse_alpha_tls_ldm)
  - [`DT_ALPHA_PLTRO`](#dt_alpha_pltro)
  - [`EF_PPC_EMB`](#ef_ppc_emb)
  - [`EF_PPC_RELOCATABLE`](#ef_ppc_relocatable)
  - [`EF_PPC_RELOCATABLE_LIB`](#ef_ppc_relocatable_lib)
  - [`R_PPC_NONE`](#r_ppc_none)
  - [`R_PPC_ADDR32`](#r_ppc_addr32)
  - [`R_PPC_ADDR24`](#r_ppc_addr24)
  - [`R_PPC_ADDR16`](#r_ppc_addr16)
  - [`R_PPC_ADDR16_LO`](#r_ppc_addr16_lo)
  - [`R_PPC_ADDR16_HI`](#r_ppc_addr16_hi)
  - [`R_PPC_ADDR16_HA`](#r_ppc_addr16_ha)
  - [`R_PPC_ADDR14`](#r_ppc_addr14)
  - [`R_PPC_ADDR14_BRTAKEN`](#r_ppc_addr14_brtaken)
  - [`R_PPC_ADDR14_BRNTAKEN`](#r_ppc_addr14_brntaken)
  - [`R_PPC_REL24`](#r_ppc_rel24)
  - [`R_PPC_REL14`](#r_ppc_rel14)
  - [`R_PPC_REL14_BRTAKEN`](#r_ppc_rel14_brtaken)
  - [`R_PPC_REL14_BRNTAKEN`](#r_ppc_rel14_brntaken)
  - [`R_PPC_GOT16`](#r_ppc_got16)
  - [`R_PPC_GOT16_LO`](#r_ppc_got16_lo)
  - [`R_PPC_GOT16_HI`](#r_ppc_got16_hi)
  - [`R_PPC_GOT16_HA`](#r_ppc_got16_ha)
  - [`R_PPC_PLTREL24`](#r_ppc_pltrel24)
  - [`R_PPC_COPY`](#r_ppc_copy)
  - [`R_PPC_GLOB_DAT`](#r_ppc_glob_dat)
  - [`R_PPC_JMP_SLOT`](#r_ppc_jmp_slot)
  - [`R_PPC_RELATIVE`](#r_ppc_relative)
  - [`R_PPC_LOCAL24PC`](#r_ppc_local24pc)
  - [`R_PPC_UADDR32`](#r_ppc_uaddr32)
  - [`R_PPC_UADDR16`](#r_ppc_uaddr16)
  - [`R_PPC_REL32`](#r_ppc_rel32)
  - [`R_PPC_PLT32`](#r_ppc_plt32)
  - [`R_PPC_PLTREL32`](#r_ppc_pltrel32)
  - [`R_PPC_PLT16_LO`](#r_ppc_plt16_lo)
  - [`R_PPC_PLT16_HI`](#r_ppc_plt16_hi)
  - [`R_PPC_PLT16_HA`](#r_ppc_plt16_ha)
  - [`R_PPC_SDAREL16`](#r_ppc_sdarel16)
  - [`R_PPC_SECTOFF`](#r_ppc_sectoff)
  - [`R_PPC_SECTOFF_LO`](#r_ppc_sectoff_lo)
  - [`R_PPC_SECTOFF_HI`](#r_ppc_sectoff_hi)
  - [`R_PPC_SECTOFF_HA`](#r_ppc_sectoff_ha)
  - [`R_PPC_TLS`](#r_ppc_tls)
  - [`R_PPC_DTPMOD32`](#r_ppc_dtpmod32)
  - [`R_PPC_TPREL16`](#r_ppc_tprel16)
  - [`R_PPC_TPREL16_LO`](#r_ppc_tprel16_lo)
  - [`R_PPC_TPREL16_HI`](#r_ppc_tprel16_hi)
  - [`R_PPC_TPREL16_HA`](#r_ppc_tprel16_ha)
  - [`R_PPC_TPREL32`](#r_ppc_tprel32)
  - [`R_PPC_DTPREL16`](#r_ppc_dtprel16)
  - [`R_PPC_DTPREL16_LO`](#r_ppc_dtprel16_lo)
  - [`R_PPC_DTPREL16_HI`](#r_ppc_dtprel16_hi)
  - [`R_PPC_DTPREL16_HA`](#r_ppc_dtprel16_ha)
  - [`R_PPC_DTPREL32`](#r_ppc_dtprel32)
  - [`R_PPC_GOT_TLSGD16`](#r_ppc_got_tlsgd16)
  - [`R_PPC_GOT_TLSGD16_LO`](#r_ppc_got_tlsgd16_lo)
  - [`R_PPC_GOT_TLSGD16_HI`](#r_ppc_got_tlsgd16_hi)
  - [`R_PPC_GOT_TLSGD16_HA`](#r_ppc_got_tlsgd16_ha)
  - [`R_PPC_GOT_TLSLD16`](#r_ppc_got_tlsld16)
  - [`R_PPC_GOT_TLSLD16_LO`](#r_ppc_got_tlsld16_lo)
  - [`R_PPC_GOT_TLSLD16_HI`](#r_ppc_got_tlsld16_hi)
  - [`R_PPC_GOT_TLSLD16_HA`](#r_ppc_got_tlsld16_ha)
  - [`R_PPC_GOT_TPREL16`](#r_ppc_got_tprel16)
  - [`R_PPC_GOT_TPREL16_LO`](#r_ppc_got_tprel16_lo)
  - [`R_PPC_GOT_TPREL16_HI`](#r_ppc_got_tprel16_hi)
  - [`R_PPC_GOT_TPREL16_HA`](#r_ppc_got_tprel16_ha)
  - [`R_PPC_GOT_DTPREL16`](#r_ppc_got_dtprel16)
  - [`R_PPC_GOT_DTPREL16_LO`](#r_ppc_got_dtprel16_lo)
  - [`R_PPC_GOT_DTPREL16_HI`](#r_ppc_got_dtprel16_hi)
  - [`R_PPC_GOT_DTPREL16_HA`](#r_ppc_got_dtprel16_ha)
  - [`R_PPC_TLSGD`](#r_ppc_tlsgd)
  - [`R_PPC_TLSLD`](#r_ppc_tlsld)
  - [`R_PPC_EMB_NADDR32`](#r_ppc_emb_naddr32)
  - [`R_PPC_EMB_NADDR16`](#r_ppc_emb_naddr16)
  - [`R_PPC_EMB_NADDR16_LO`](#r_ppc_emb_naddr16_lo)
  - [`R_PPC_EMB_NADDR16_HI`](#r_ppc_emb_naddr16_hi)
  - [`R_PPC_EMB_NADDR16_HA`](#r_ppc_emb_naddr16_ha)
  - [`R_PPC_EMB_SDAI16`](#r_ppc_emb_sdai16)
  - [`R_PPC_EMB_SDA2I16`](#r_ppc_emb_sda2i16)
  - [`R_PPC_EMB_SDA2REL`](#r_ppc_emb_sda2rel)
  - [`R_PPC_EMB_SDA21`](#r_ppc_emb_sda21)
  - [`R_PPC_EMB_MRKREF`](#r_ppc_emb_mrkref)
  - [`R_PPC_EMB_RELSEC16`](#r_ppc_emb_relsec16)
  - [`R_PPC_EMB_RELST_LO`](#r_ppc_emb_relst_lo)
  - [`R_PPC_EMB_RELST_HI`](#r_ppc_emb_relst_hi)
  - [`R_PPC_EMB_RELST_HA`](#r_ppc_emb_relst_ha)
  - [`R_PPC_EMB_BIT_FLD`](#r_ppc_emb_bit_fld)
  - [`R_PPC_EMB_RELSDA`](#r_ppc_emb_relsda)
  - [`R_PPC_DIAB_SDA21_LO`](#r_ppc_diab_sda21_lo)
  - [`R_PPC_DIAB_SDA21_HI`](#r_ppc_diab_sda21_hi)
  - [`R_PPC_DIAB_SDA21_HA`](#r_ppc_diab_sda21_ha)
  - [`R_PPC_DIAB_RELSDA_LO`](#r_ppc_diab_relsda_lo)
  - [`R_PPC_DIAB_RELSDA_HI`](#r_ppc_diab_relsda_hi)
  - [`R_PPC_DIAB_RELSDA_HA`](#r_ppc_diab_relsda_ha)
  - [`R_PPC_IRELATIVE`](#r_ppc_irelative)
  - [`R_PPC_REL16`](#r_ppc_rel16)
  - [`R_PPC_REL16_LO`](#r_ppc_rel16_lo)
  - [`R_PPC_REL16_HI`](#r_ppc_rel16_hi)
  - [`R_PPC_REL16_HA`](#r_ppc_rel16_ha)
  - [`R_PPC_TOC16`](#r_ppc_toc16)
  - [`DT_PPC_GOT`](#dt_ppc_got)
  - [`DT_PPC_OPT`](#dt_ppc_opt)
  - [`PPC_OPT_TLS`](#ppc_opt_tls)
  - [`R_PPC64_NONE`](#r_ppc64_none)
  - [`R_PPC64_ADDR32`](#r_ppc64_addr32)
  - [`R_PPC64_ADDR24`](#r_ppc64_addr24)
  - [`R_PPC64_ADDR16`](#r_ppc64_addr16)
  - [`R_PPC64_ADDR16_LO`](#r_ppc64_addr16_lo)
  - [`R_PPC64_ADDR16_HI`](#r_ppc64_addr16_hi)
  - [`R_PPC64_ADDR16_HA`](#r_ppc64_addr16_ha)
  - [`R_PPC64_ADDR14`](#r_ppc64_addr14)
  - [`R_PPC64_ADDR14_BRTAKEN`](#r_ppc64_addr14_brtaken)
  - [`R_PPC64_ADDR14_BRNTAKEN`](#r_ppc64_addr14_brntaken)
  - [`R_PPC64_REL24`](#r_ppc64_rel24)
  - [`R_PPC64_REL14`](#r_ppc64_rel14)
  - [`R_PPC64_REL14_BRTAKEN`](#r_ppc64_rel14_brtaken)
  - [`R_PPC64_REL14_BRNTAKEN`](#r_ppc64_rel14_brntaken)
  - [`R_PPC64_GOT16`](#r_ppc64_got16)
  - [`R_PPC64_GOT16_LO`](#r_ppc64_got16_lo)
  - [`R_PPC64_GOT16_HI`](#r_ppc64_got16_hi)
  - [`R_PPC64_GOT16_HA`](#r_ppc64_got16_ha)
  - [`R_PPC64_COPY`](#r_ppc64_copy)
  - [`R_PPC64_GLOB_DAT`](#r_ppc64_glob_dat)
  - [`R_PPC64_JMP_SLOT`](#r_ppc64_jmp_slot)
  - [`R_PPC64_RELATIVE`](#r_ppc64_relative)
  - [`R_PPC64_UADDR32`](#r_ppc64_uaddr32)
  - [`R_PPC64_UADDR16`](#r_ppc64_uaddr16)
  - [`R_PPC64_REL32`](#r_ppc64_rel32)
  - [`R_PPC64_PLT32`](#r_ppc64_plt32)
  - [`R_PPC64_PLTREL32`](#r_ppc64_pltrel32)
  - [`R_PPC64_PLT16_LO`](#r_ppc64_plt16_lo)
  - [`R_PPC64_PLT16_HI`](#r_ppc64_plt16_hi)
  - [`R_PPC64_PLT16_HA`](#r_ppc64_plt16_ha)
  - [`R_PPC64_SECTOFF`](#r_ppc64_sectoff)
  - [`R_PPC64_SECTOFF_LO`](#r_ppc64_sectoff_lo)
  - [`R_PPC64_SECTOFF_HI`](#r_ppc64_sectoff_hi)
  - [`R_PPC64_SECTOFF_HA`](#r_ppc64_sectoff_ha)
  - [`R_PPC64_ADDR30`](#r_ppc64_addr30)
  - [`R_PPC64_ADDR64`](#r_ppc64_addr64)
  - [`R_PPC64_ADDR16_HIGHER`](#r_ppc64_addr16_higher)
  - [`R_PPC64_ADDR16_HIGHERA`](#r_ppc64_addr16_highera)
  - [`R_PPC64_ADDR16_HIGHEST`](#r_ppc64_addr16_highest)
  - [`R_PPC64_ADDR16_HIGHESTA`](#r_ppc64_addr16_highesta)
  - [`R_PPC64_UADDR64`](#r_ppc64_uaddr64)
  - [`R_PPC64_REL64`](#r_ppc64_rel64)
  - [`R_PPC64_PLT64`](#r_ppc64_plt64)
  - [`R_PPC64_PLTREL64`](#r_ppc64_pltrel64)
  - [`R_PPC64_TOC16`](#r_ppc64_toc16)
  - [`R_PPC64_TOC16_LO`](#r_ppc64_toc16_lo)
  - [`R_PPC64_TOC16_HI`](#r_ppc64_toc16_hi)
  - [`R_PPC64_TOC16_HA`](#r_ppc64_toc16_ha)
  - [`R_PPC64_TOC`](#r_ppc64_toc)
  - [`R_PPC64_PLTGOT16`](#r_ppc64_pltgot16)
  - [`R_PPC64_PLTGOT16_LO`](#r_ppc64_pltgot16_lo)
  - [`R_PPC64_PLTGOT16_HI`](#r_ppc64_pltgot16_hi)
  - [`R_PPC64_PLTGOT16_HA`](#r_ppc64_pltgot16_ha)
  - [`R_PPC64_ADDR16_DS`](#r_ppc64_addr16_ds)
  - [`R_PPC64_ADDR16_LO_DS`](#r_ppc64_addr16_lo_ds)
  - [`R_PPC64_GOT16_DS`](#r_ppc64_got16_ds)
  - [`R_PPC64_GOT16_LO_DS`](#r_ppc64_got16_lo_ds)
  - [`R_PPC64_PLT16_LO_DS`](#r_ppc64_plt16_lo_ds)
  - [`R_PPC64_SECTOFF_DS`](#r_ppc64_sectoff_ds)
  - [`R_PPC64_SECTOFF_LO_DS`](#r_ppc64_sectoff_lo_ds)
  - [`R_PPC64_TOC16_DS`](#r_ppc64_toc16_ds)
  - [`R_PPC64_TOC16_LO_DS`](#r_ppc64_toc16_lo_ds)
  - [`R_PPC64_PLTGOT16_DS`](#r_ppc64_pltgot16_ds)
  - [`R_PPC64_PLTGOT16_LO_DS`](#r_ppc64_pltgot16_lo_ds)
  - [`R_PPC64_TLS`](#r_ppc64_tls)
  - [`R_PPC64_DTPMOD64`](#r_ppc64_dtpmod64)
  - [`R_PPC64_TPREL16`](#r_ppc64_tprel16)
  - [`R_PPC64_TPREL16_LO`](#r_ppc64_tprel16_lo)
  - [`R_PPC64_TPREL16_HI`](#r_ppc64_tprel16_hi)
  - [`R_PPC64_TPREL16_HA`](#r_ppc64_tprel16_ha)
  - [`R_PPC64_TPREL64`](#r_ppc64_tprel64)
  - [`R_PPC64_DTPREL16`](#r_ppc64_dtprel16)
  - [`R_PPC64_DTPREL16_LO`](#r_ppc64_dtprel16_lo)
  - [`R_PPC64_DTPREL16_HI`](#r_ppc64_dtprel16_hi)
  - [`R_PPC64_DTPREL16_HA`](#r_ppc64_dtprel16_ha)
  - [`R_PPC64_DTPREL64`](#r_ppc64_dtprel64)
  - [`R_PPC64_GOT_TLSGD16`](#r_ppc64_got_tlsgd16)
  - [`R_PPC64_GOT_TLSGD16_LO`](#r_ppc64_got_tlsgd16_lo)
  - [`R_PPC64_GOT_TLSGD16_HI`](#r_ppc64_got_tlsgd16_hi)
  - [`R_PPC64_GOT_TLSGD16_HA`](#r_ppc64_got_tlsgd16_ha)
  - [`R_PPC64_GOT_TLSLD16`](#r_ppc64_got_tlsld16)
  - [`R_PPC64_GOT_TLSLD16_LO`](#r_ppc64_got_tlsld16_lo)
  - [`R_PPC64_GOT_TLSLD16_HI`](#r_ppc64_got_tlsld16_hi)
  - [`R_PPC64_GOT_TLSLD16_HA`](#r_ppc64_got_tlsld16_ha)
  - [`R_PPC64_GOT_TPREL16_DS`](#r_ppc64_got_tprel16_ds)
  - [`R_PPC64_GOT_TPREL16_LO_DS`](#r_ppc64_got_tprel16_lo_ds)
  - [`R_PPC64_GOT_TPREL16_HI`](#r_ppc64_got_tprel16_hi)
  - [`R_PPC64_GOT_TPREL16_HA`](#r_ppc64_got_tprel16_ha)
  - [`R_PPC64_GOT_DTPREL16_DS`](#r_ppc64_got_dtprel16_ds)
  - [`R_PPC64_GOT_DTPREL16_LO_DS`](#r_ppc64_got_dtprel16_lo_ds)
  - [`R_PPC64_GOT_DTPREL16_HI`](#r_ppc64_got_dtprel16_hi)
  - [`R_PPC64_GOT_DTPREL16_HA`](#r_ppc64_got_dtprel16_ha)
  - [`R_PPC64_TPREL16_DS`](#r_ppc64_tprel16_ds)
  - [`R_PPC64_TPREL16_LO_DS`](#r_ppc64_tprel16_lo_ds)
  - [`R_PPC64_TPREL16_HIGHER`](#r_ppc64_tprel16_higher)
  - [`R_PPC64_TPREL16_HIGHERA`](#r_ppc64_tprel16_highera)
  - [`R_PPC64_TPREL16_HIGHEST`](#r_ppc64_tprel16_highest)
  - [`R_PPC64_TPREL16_HIGHESTA`](#r_ppc64_tprel16_highesta)
  - [`R_PPC64_DTPREL16_DS`](#r_ppc64_dtprel16_ds)
  - [`R_PPC64_DTPREL16_LO_DS`](#r_ppc64_dtprel16_lo_ds)
  - [`R_PPC64_DTPREL16_HIGHER`](#r_ppc64_dtprel16_higher)
  - [`R_PPC64_DTPREL16_HIGHERA`](#r_ppc64_dtprel16_highera)
  - [`R_PPC64_DTPREL16_HIGHEST`](#r_ppc64_dtprel16_highest)
  - [`R_PPC64_DTPREL16_HIGHESTA`](#r_ppc64_dtprel16_highesta)
  - [`R_PPC64_TLSGD`](#r_ppc64_tlsgd)
  - [`R_PPC64_TLSLD`](#r_ppc64_tlsld)
  - [`R_PPC64_TOCSAVE`](#r_ppc64_tocsave)
  - [`R_PPC64_ADDR16_HIGH`](#r_ppc64_addr16_high)
  - [`R_PPC64_ADDR16_HIGHA`](#r_ppc64_addr16_higha)
  - [`R_PPC64_TPREL16_HIGH`](#r_ppc64_tprel16_high)
  - [`R_PPC64_TPREL16_HIGHA`](#r_ppc64_tprel16_higha)
  - [`R_PPC64_DTPREL16_HIGH`](#r_ppc64_dtprel16_high)
  - [`R_PPC64_DTPREL16_HIGHA`](#r_ppc64_dtprel16_higha)
  - [`R_PPC64_JMP_IREL`](#r_ppc64_jmp_irel)
  - [`R_PPC64_IRELATIVE`](#r_ppc64_irelative)
  - [`R_PPC64_REL16`](#r_ppc64_rel16)
  - [`R_PPC64_REL16_LO`](#r_ppc64_rel16_lo)
  - [`R_PPC64_REL16_HI`](#r_ppc64_rel16_hi)
  - [`R_PPC64_REL16_HA`](#r_ppc64_rel16_ha)
  - [`EF_PPC64_ABI`](#ef_ppc64_abi)
  - [`DT_PPC64_GLINK`](#dt_ppc64_glink)
  - [`DT_PPC64_OPD`](#dt_ppc64_opd)
  - [`DT_PPC64_OPDSZ`](#dt_ppc64_opdsz)
  - [`DT_PPC64_OPT`](#dt_ppc64_opt)
  - [`PPC64_OPT_TLS`](#ppc64_opt_tls)
  - [`PPC64_OPT_MULTI_TOC`](#ppc64_opt_multi_toc)
  - [`PPC64_OPT_LOCALENTRY`](#ppc64_opt_localentry)
  - [`STO_PPC64_LOCAL_BIT`](#sto_ppc64_local_bit)
  - [`STO_PPC64_LOCAL_MASK`](#sto_ppc64_local_mask)
  - [`EF_ARM_RELEXEC`](#ef_arm_relexec)
  - [`EF_ARM_HASENTRY`](#ef_arm_hasentry)
  - [`EF_ARM_INTERWORK`](#ef_arm_interwork)
  - [`EF_ARM_APCS_26`](#ef_arm_apcs_26)
  - [`EF_ARM_APCS_FLOAT`](#ef_arm_apcs_float)
  - [`EF_ARM_PIC`](#ef_arm_pic)
  - [`EF_ARM_ALIGN8`](#ef_arm_align8)
  - [`EF_ARM_NEW_ABI`](#ef_arm_new_abi)
  - [`EF_ARM_OLD_ABI`](#ef_arm_old_abi)
  - [`EF_ARM_SOFT_FLOAT`](#ef_arm_soft_float)
  - [`EF_ARM_VFP_FLOAT`](#ef_arm_vfp_float)
  - [`EF_ARM_MAVERICK_FLOAT`](#ef_arm_maverick_float)
  - [`EF_ARM_ABI_FLOAT_SOFT`](#ef_arm_abi_float_soft)
  - [`EF_ARM_ABI_FLOAT_HARD`](#ef_arm_abi_float_hard)
  - [`EF_ARM_SYMSARESORTED`](#ef_arm_symsaresorted)
  - [`EF_ARM_DYNSYMSUSESEGIDX`](#ef_arm_dynsymsusesegidx)
  - [`EF_ARM_MAPSYMSFIRST`](#ef_arm_mapsymsfirst)
  - [`EF_ARM_BE8`](#ef_arm_be8)
  - [`EF_ARM_LE8`](#ef_arm_le8)
  - [`EF_ARM_EABIMASK`](#ef_arm_eabimask)
  - [`EF_ARM_EABI_UNKNOWN`](#ef_arm_eabi_unknown)
  - [`EF_ARM_EABI_VER1`](#ef_arm_eabi_ver1)
  - [`EF_ARM_EABI_VER2`](#ef_arm_eabi_ver2)
  - [`EF_ARM_EABI_VER3`](#ef_arm_eabi_ver3)
  - [`EF_ARM_EABI_VER4`](#ef_arm_eabi_ver4)
  - [`EF_ARM_EABI_VER5`](#ef_arm_eabi_ver5)
  - [`STT_ARM_TFUNC`](#stt_arm_tfunc)
  - [`STT_ARM_16BIT`](#stt_arm_16bit)
  - [`SHF_ARM_ENTRYSECT`](#shf_arm_entrysect)
  - [`SHF_ARM_COMDEF`](#shf_arm_comdef)
  - [`PF_ARM_SB`](#pf_arm_sb)
  - [`PF_ARM_PI`](#pf_arm_pi)
  - [`PF_ARM_ABS`](#pf_arm_abs)
  - [`PT_ARM_EXIDX`](#pt_arm_exidx)
  - [`SHT_ARM_EXIDX`](#sht_arm_exidx)
  - [`SHT_ARM_PREEMPTMAP`](#sht_arm_preemptmap)
  - [`SHT_ARM_ATTRIBUTES`](#sht_arm_attributes)
  - [`SHT_AARCH64_ATTRIBUTES`](#sht_aarch64_attributes)
  - [`R_AARCH64_NONE`](#r_aarch64_none)
  - [`R_AARCH64_P32_ABS32`](#r_aarch64_p32_abs32)
  - [`R_AARCH64_P32_COPY`](#r_aarch64_p32_copy)
  - [`R_AARCH64_P32_GLOB_DAT`](#r_aarch64_p32_glob_dat)
  - [`R_AARCH64_P32_JUMP_SLOT`](#r_aarch64_p32_jump_slot)
  - [`R_AARCH64_P32_RELATIVE`](#r_aarch64_p32_relative)
  - [`R_AARCH64_P32_TLS_DTPMOD`](#r_aarch64_p32_tls_dtpmod)
  - [`R_AARCH64_P32_TLS_DTPREL`](#r_aarch64_p32_tls_dtprel)
  - [`R_AARCH64_P32_TLS_TPREL`](#r_aarch64_p32_tls_tprel)
  - [`R_AARCH64_P32_TLSDESC`](#r_aarch64_p32_tlsdesc)
  - [`R_AARCH64_P32_IRELATIVE`](#r_aarch64_p32_irelative)
  - [`R_AARCH64_ABS64`](#r_aarch64_abs64)
  - [`R_AARCH64_ABS32`](#r_aarch64_abs32)
  - [`R_AARCH64_ABS16`](#r_aarch64_abs16)
  - [`R_AARCH64_PREL64`](#r_aarch64_prel64)
  - [`R_AARCH64_PREL32`](#r_aarch64_prel32)
  - [`R_AARCH64_PREL16`](#r_aarch64_prel16)
  - [`R_AARCH64_MOVW_UABS_G0`](#r_aarch64_movw_uabs_g0)
  - [`R_AARCH64_MOVW_UABS_G0_NC`](#r_aarch64_movw_uabs_g0_nc)
  - [`R_AARCH64_MOVW_UABS_G1`](#r_aarch64_movw_uabs_g1)
  - [`R_AARCH64_MOVW_UABS_G1_NC`](#r_aarch64_movw_uabs_g1_nc)
  - [`R_AARCH64_MOVW_UABS_G2`](#r_aarch64_movw_uabs_g2)
  - [`R_AARCH64_MOVW_UABS_G2_NC`](#r_aarch64_movw_uabs_g2_nc)
  - [`R_AARCH64_MOVW_UABS_G3`](#r_aarch64_movw_uabs_g3)
  - [`R_AARCH64_MOVW_SABS_G0`](#r_aarch64_movw_sabs_g0)
  - [`R_AARCH64_MOVW_SABS_G1`](#r_aarch64_movw_sabs_g1)
  - [`R_AARCH64_MOVW_SABS_G2`](#r_aarch64_movw_sabs_g2)
  - [`R_AARCH64_LD_PREL_LO19`](#r_aarch64_ld_prel_lo19)
  - [`R_AARCH64_ADR_PREL_LO21`](#r_aarch64_adr_prel_lo21)
  - [`R_AARCH64_ADR_PREL_PG_HI21`](#r_aarch64_adr_prel_pg_hi21)
  - [`R_AARCH64_ADR_PREL_PG_HI21_NC`](#r_aarch64_adr_prel_pg_hi21_nc)
  - [`R_AARCH64_ADD_ABS_LO12_NC`](#r_aarch64_add_abs_lo12_nc)
  - [`R_AARCH64_LDST8_ABS_LO12_NC`](#r_aarch64_ldst8_abs_lo12_nc)
  - [`R_AARCH64_TSTBR14`](#r_aarch64_tstbr14)
  - [`R_AARCH64_CONDBR19`](#r_aarch64_condbr19)
  - [`R_AARCH64_JUMP26`](#r_aarch64_jump26)
  - [`R_AARCH64_CALL26`](#r_aarch64_call26)
  - [`R_AARCH64_LDST16_ABS_LO12_NC`](#r_aarch64_ldst16_abs_lo12_nc)
  - [`R_AARCH64_LDST32_ABS_LO12_NC`](#r_aarch64_ldst32_abs_lo12_nc)
  - [`R_AARCH64_LDST64_ABS_LO12_NC`](#r_aarch64_ldst64_abs_lo12_nc)
  - [`R_AARCH64_MOVW_PREL_G0`](#r_aarch64_movw_prel_g0)
  - [`R_AARCH64_MOVW_PREL_G0_NC`](#r_aarch64_movw_prel_g0_nc)
  - [`R_AARCH64_MOVW_PREL_G1`](#r_aarch64_movw_prel_g1)
  - [`R_AARCH64_MOVW_PREL_G1_NC`](#r_aarch64_movw_prel_g1_nc)
  - [`R_AARCH64_MOVW_PREL_G2`](#r_aarch64_movw_prel_g2)
  - [`R_AARCH64_MOVW_PREL_G2_NC`](#r_aarch64_movw_prel_g2_nc)
  - [`R_AARCH64_MOVW_PREL_G3`](#r_aarch64_movw_prel_g3)
  - [`R_AARCH64_LDST128_ABS_LO12_NC`](#r_aarch64_ldst128_abs_lo12_nc)
  - [`R_AARCH64_MOVW_GOTOFF_G0`](#r_aarch64_movw_gotoff_g0)
  - [`R_AARCH64_MOVW_GOTOFF_G0_NC`](#r_aarch64_movw_gotoff_g0_nc)
  - [`R_AARCH64_MOVW_GOTOFF_G1`](#r_aarch64_movw_gotoff_g1)
  - [`R_AARCH64_MOVW_GOTOFF_G1_NC`](#r_aarch64_movw_gotoff_g1_nc)
  - [`R_AARCH64_MOVW_GOTOFF_G2`](#r_aarch64_movw_gotoff_g2)
  - [`R_AARCH64_MOVW_GOTOFF_G2_NC`](#r_aarch64_movw_gotoff_g2_nc)
  - [`R_AARCH64_MOVW_GOTOFF_G3`](#r_aarch64_movw_gotoff_g3)
  - [`R_AARCH64_GOTREL64`](#r_aarch64_gotrel64)
  - [`R_AARCH64_GOTREL32`](#r_aarch64_gotrel32)
  - [`R_AARCH64_GOT_LD_PREL19`](#r_aarch64_got_ld_prel19)
  - [`R_AARCH64_LD64_GOTOFF_LO15`](#r_aarch64_ld64_gotoff_lo15)
  - [`R_AARCH64_ADR_GOT_PAGE`](#r_aarch64_adr_got_page)
  - [`R_AARCH64_LD64_GOT_LO12_NC`](#r_aarch64_ld64_got_lo12_nc)
  - [`R_AARCH64_LD64_GOTPAGE_LO15`](#r_aarch64_ld64_gotpage_lo15)
  - [`R_AARCH64_TLSGD_ADR_PREL21`](#r_aarch64_tlsgd_adr_prel21)
  - [`R_AARCH64_TLSGD_ADR_PAGE21`](#r_aarch64_tlsgd_adr_page21)
  - [`R_AARCH64_TLSGD_ADD_LO12_NC`](#r_aarch64_tlsgd_add_lo12_nc)
  - [`R_AARCH64_TLSGD_MOVW_G1`](#r_aarch64_tlsgd_movw_g1)
  - [`R_AARCH64_TLSGD_MOVW_G0_NC`](#r_aarch64_tlsgd_movw_g0_nc)
  - [`R_AARCH64_TLSLD_ADR_PREL21`](#r_aarch64_tlsld_adr_prel21)
  - [`R_AARCH64_TLSLD_ADR_PAGE21`](#r_aarch64_tlsld_adr_page21)
  - [`R_AARCH64_TLSLD_ADD_LO12_NC`](#r_aarch64_tlsld_add_lo12_nc)
  - [`R_AARCH64_TLSLD_MOVW_G1`](#r_aarch64_tlsld_movw_g1)
  - [`R_AARCH64_TLSLD_MOVW_G0_NC`](#r_aarch64_tlsld_movw_g0_nc)
  - [`R_AARCH64_TLSLD_LD_PREL19`](#r_aarch64_tlsld_ld_prel19)
  - [`R_AARCH64_TLSLD_MOVW_DTPREL_G2`](#r_aarch64_tlsld_movw_dtprel_g2)
  - [`R_AARCH64_TLSLD_MOVW_DTPREL_G1`](#r_aarch64_tlsld_movw_dtprel_g1)
  - [`R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC`](#r_aarch64_tlsld_movw_dtprel_g1_nc)
  - [`R_AARCH64_TLSLD_MOVW_DTPREL_G0`](#r_aarch64_tlsld_movw_dtprel_g0)
  - [`R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC`](#r_aarch64_tlsld_movw_dtprel_g0_nc)
  - [`R_AARCH64_TLSLD_ADD_DTPREL_HI12`](#r_aarch64_tlsld_add_dtprel_hi12)
  - [`R_AARCH64_TLSLD_ADD_DTPREL_LO12`](#r_aarch64_tlsld_add_dtprel_lo12)
  - [`R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC`](#r_aarch64_tlsld_add_dtprel_lo12_nc)
  - [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12`](#r_aarch64_tlsld_ldst8_dtprel_lo12)
  - [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst8_dtprel_lo12_nc)
  - [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12`](#r_aarch64_tlsld_ldst16_dtprel_lo12)
  - [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst16_dtprel_lo12_nc)
  - [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12`](#r_aarch64_tlsld_ldst32_dtprel_lo12)
  - [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst32_dtprel_lo12_nc)
  - [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12`](#r_aarch64_tlsld_ldst64_dtprel_lo12)
  - [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst64_dtprel_lo12_nc)
  - [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G1`](#r_aarch64_tlsie_movw_gottprel_g1)
  - [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC`](#r_aarch64_tlsie_movw_gottprel_g0_nc)
  - [`R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21`](#r_aarch64_tlsie_adr_gottprel_page21)
  - [`R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC`](#r_aarch64_tlsie_ld64_gottprel_lo12_nc)
  - [`R_AARCH64_TLSIE_LD_GOTTPREL_PREL19`](#r_aarch64_tlsie_ld_gottprel_prel19)
  - [`R_AARCH64_TLSLE_MOVW_TPREL_G2`](#r_aarch64_tlsle_movw_tprel_g2)
  - [`R_AARCH64_TLSLE_MOVW_TPREL_G1`](#r_aarch64_tlsle_movw_tprel_g1)
  - [`R_AARCH64_TLSLE_MOVW_TPREL_G1_NC`](#r_aarch64_tlsle_movw_tprel_g1_nc)
  - [`R_AARCH64_TLSLE_MOVW_TPREL_G0`](#r_aarch64_tlsle_movw_tprel_g0)
  - [`R_AARCH64_TLSLE_MOVW_TPREL_G0_NC`](#r_aarch64_tlsle_movw_tprel_g0_nc)
  - [`R_AARCH64_TLSLE_ADD_TPREL_HI12`](#r_aarch64_tlsle_add_tprel_hi12)
  - [`R_AARCH64_TLSLE_ADD_TPREL_LO12`](#r_aarch64_tlsle_add_tprel_lo12)
  - [`R_AARCH64_TLSLE_ADD_TPREL_LO12_NC`](#r_aarch64_tlsle_add_tprel_lo12_nc)
  - [`R_AARCH64_TLSLE_LDST8_TPREL_LO12`](#r_aarch64_tlsle_ldst8_tprel_lo12)
  - [`R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst8_tprel_lo12_nc)
  - [`R_AARCH64_TLSLE_LDST16_TPREL_LO12`](#r_aarch64_tlsle_ldst16_tprel_lo12)
  - [`R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst16_tprel_lo12_nc)
  - [`R_AARCH64_TLSLE_LDST32_TPREL_LO12`](#r_aarch64_tlsle_ldst32_tprel_lo12)
  - [`R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst32_tprel_lo12_nc)
  - [`R_AARCH64_TLSLE_LDST64_TPREL_LO12`](#r_aarch64_tlsle_ldst64_tprel_lo12)
  - [`R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst64_tprel_lo12_nc)
  - [`R_AARCH64_TLSDESC_LD_PREL19`](#r_aarch64_tlsdesc_ld_prel19)
  - [`R_AARCH64_TLSDESC_ADR_PREL21`](#r_aarch64_tlsdesc_adr_prel21)
  - [`R_AARCH64_TLSDESC_ADR_PAGE21`](#r_aarch64_tlsdesc_adr_page21)
  - [`R_AARCH64_TLSDESC_LD64_LO12`](#r_aarch64_tlsdesc_ld64_lo12)
  - [`R_AARCH64_TLSDESC_ADD_LO12`](#r_aarch64_tlsdesc_add_lo12)
  - [`R_AARCH64_TLSDESC_OFF_G1`](#r_aarch64_tlsdesc_off_g1)
  - [`R_AARCH64_TLSDESC_OFF_G0_NC`](#r_aarch64_tlsdesc_off_g0_nc)
  - [`R_AARCH64_TLSDESC_LDR`](#r_aarch64_tlsdesc_ldr)
  - [`R_AARCH64_TLSDESC_ADD`](#r_aarch64_tlsdesc_add)
  - [`R_AARCH64_TLSDESC_CALL`](#r_aarch64_tlsdesc_call)
  - [`R_AARCH64_TLSLE_LDST128_TPREL_LO12`](#r_aarch64_tlsle_ldst128_tprel_lo12)
  - [`R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst128_tprel_lo12_nc)
  - [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12`](#r_aarch64_tlsld_ldst128_dtprel_lo12)
  - [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst128_dtprel_lo12_nc)
  - [`R_AARCH64_COPY`](#r_aarch64_copy)
  - [`R_AARCH64_GLOB_DAT`](#r_aarch64_glob_dat)
  - [`R_AARCH64_JUMP_SLOT`](#r_aarch64_jump_slot)
  - [`R_AARCH64_RELATIVE`](#r_aarch64_relative)
  - [`R_AARCH64_TLS_DTPMOD`](#r_aarch64_tls_dtpmod)
  - [`R_AARCH64_TLS_DTPREL`](#r_aarch64_tls_dtprel)
  - [`R_AARCH64_TLS_TPREL`](#r_aarch64_tls_tprel)
  - [`R_AARCH64_TLSDESC`](#r_aarch64_tlsdesc)
  - [`R_AARCH64_IRELATIVE`](#r_aarch64_irelative)
  - [`EF_AVR_ARCH`](#ef_avr_arch)
  - [`EF_AVR_LINKRELAX_PREPARED`](#ef_avr_linkrelax_prepared)
  - [`EF_AVR_ARCH_AVR1`](#ef_avr_arch_avr1)
  - [`EF_AVR_ARCH_AVR2`](#ef_avr_arch_avr2)
  - [`EF_AVR_ARCH_AVR25`](#ef_avr_arch_avr25)
  - [`EF_AVR_ARCH_AVR3`](#ef_avr_arch_avr3)
  - [`EF_AVR_ARCH_AVR31`](#ef_avr_arch_avr31)
  - [`EF_AVR_ARCH_AVR35`](#ef_avr_arch_avr35)
  - [`EF_AVR_ARCH_AVR4`](#ef_avr_arch_avr4)
  - [`EF_AVR_ARCH_AVR5`](#ef_avr_arch_avr5)
  - [`EF_AVR_ARCH_AVR51`](#ef_avr_arch_avr51)
  - [`EF_AVR_ARCH_AVR6`](#ef_avr_arch_avr6)
  - [`EF_AVR_ARCH_AVRTINY`](#ef_avr_arch_avrtiny)
  - [`EF_AVR_ARCH_XMEGA1`](#ef_avr_arch_xmega1)
  - [`EF_AVR_ARCH_XMEGA2`](#ef_avr_arch_xmega2)
  - [`EF_AVR_ARCH_XMEGA3`](#ef_avr_arch_xmega3)
  - [`EF_AVR_ARCH_XMEGA4`](#ef_avr_arch_xmega4)
  - [`EF_AVR_ARCH_XMEGA5`](#ef_avr_arch_xmega5)
  - [`EF_AVR_ARCH_XMEGA6`](#ef_avr_arch_xmega6)
  - [`EF_AVR_ARCH_XMEGA7`](#ef_avr_arch_xmega7)
  - [`R_AVR_NONE`](#r_avr_none)
  - [`R_AVR_32`](#r_avr_32)
  - [`R_AVR_7_PCREL`](#r_avr_7_pcrel)
  - [`R_AVR_13_PCREL`](#r_avr_13_pcrel)
  - [`R_AVR_16`](#r_avr_16)
  - [`R_AVR_16_PM`](#r_avr_16_pm)
  - [`R_AVR_LO8_LDI`](#r_avr_lo8_ldi)
  - [`R_AVR_HI8_LDI`](#r_avr_hi8_ldi)
  - [`R_AVR_HH8_LDI`](#r_avr_hh8_ldi)
  - [`R_AVR_LO8_LDI_NEG`](#r_avr_lo8_ldi_neg)
  - [`R_AVR_HI8_LDI_NEG`](#r_avr_hi8_ldi_neg)
  - [`R_AVR_HH8_LDI_NEG`](#r_avr_hh8_ldi_neg)
  - [`R_AVR_LO8_LDI_PM`](#r_avr_lo8_ldi_pm)
  - [`R_AVR_HI8_LDI_PM`](#r_avr_hi8_ldi_pm)
  - [`R_AVR_HH8_LDI_PM`](#r_avr_hh8_ldi_pm)
  - [`R_AVR_LO8_LDI_PM_NEG`](#r_avr_lo8_ldi_pm_neg)
  - [`R_AVR_HI8_LDI_PM_NEG`](#r_avr_hi8_ldi_pm_neg)
  - [`R_AVR_HH8_LDI_PM_NEG`](#r_avr_hh8_ldi_pm_neg)
  - [`R_AVR_CALL`](#r_avr_call)
  - [`R_AVR_LDI`](#r_avr_ldi)
  - [`R_AVR_6`](#r_avr_6)
  - [`R_AVR_6_ADIW`](#r_avr_6_adiw)
  - [`R_AVR_MS8_LDI`](#r_avr_ms8_ldi)
  - [`R_AVR_MS8_LDI_NEG`](#r_avr_ms8_ldi_neg)
  - [`R_AVR_LO8_LDI_GS`](#r_avr_lo8_ldi_gs)
  - [`R_AVR_HI8_LDI_GS`](#r_avr_hi8_ldi_gs)
  - [`R_AVR_8`](#r_avr_8)
  - [`R_AVR_8_LO8`](#r_avr_8_lo8)
  - [`R_AVR_8_HI8`](#r_avr_8_hi8)
  - [`R_AVR_8_HLO8`](#r_avr_8_hlo8)
  - [`R_AVR_DIFF8`](#r_avr_diff8)
  - [`R_AVR_DIFF16`](#r_avr_diff16)
  - [`R_AVR_DIFF32`](#r_avr_diff32)
  - [`R_AVR_LDS_STS_16`](#r_avr_lds_sts_16)
  - [`R_AVR_PORT6`](#r_avr_port6)
  - [`R_AVR_PORT5`](#r_avr_port5)
  - [`R_AVR_32_PCREL`](#r_avr_32_pcrel)
  - [`R_MSP430_32`](#r_msp430_32)
  - [`R_MSP430_16_BYTE`](#r_msp430_16_byte)
  - [`R_HEX_32`](#r_hex_32)
  - [`R_ARM_NONE`](#r_arm_none)
  - [`R_ARM_PC24`](#r_arm_pc24)
  - [`R_ARM_ABS32`](#r_arm_abs32)
  - [`R_ARM_REL32`](#r_arm_rel32)
  - [`R_ARM_PC13`](#r_arm_pc13)
  - [`R_ARM_ABS16`](#r_arm_abs16)
  - [`R_ARM_ABS12`](#r_arm_abs12)
  - [`R_ARM_THM_ABS5`](#r_arm_thm_abs5)
  - [`R_ARM_ABS8`](#r_arm_abs8)
  - [`R_ARM_SBREL32`](#r_arm_sbrel32)
  - [`R_ARM_THM_PC22`](#r_arm_thm_pc22)
  - [`R_ARM_THM_PC8`](#r_arm_thm_pc8)
  - [`R_ARM_AMP_VCALL9`](#r_arm_amp_vcall9)
  - [`R_ARM_SWI24`](#r_arm_swi24)
  - [`R_ARM_TLS_DESC`](#r_arm_tls_desc)
  - [`R_ARM_THM_SWI8`](#r_arm_thm_swi8)
  - [`R_ARM_XPC25`](#r_arm_xpc25)
  - [`R_ARM_THM_XPC22`](#r_arm_thm_xpc22)
  - [`R_ARM_TLS_DTPMOD32`](#r_arm_tls_dtpmod32)
  - [`R_ARM_TLS_DTPOFF32`](#r_arm_tls_dtpoff32)
  - [`R_ARM_TLS_TPOFF32`](#r_arm_tls_tpoff32)
  - [`R_ARM_COPY`](#r_arm_copy)
  - [`R_ARM_GLOB_DAT`](#r_arm_glob_dat)
  - [`R_ARM_JUMP_SLOT`](#r_arm_jump_slot)
  - [`R_ARM_RELATIVE`](#r_arm_relative)
  - [`R_ARM_GOTOFF`](#r_arm_gotoff)
  - [`R_ARM_GOTPC`](#r_arm_gotpc)
  - [`R_ARM_GOT32`](#r_arm_got32)
  - [`R_ARM_PLT32`](#r_arm_plt32)
  - [`R_ARM_CALL`](#r_arm_call)
  - [`R_ARM_JUMP24`](#r_arm_jump24)
  - [`R_ARM_THM_JUMP24`](#r_arm_thm_jump24)
  - [`R_ARM_BASE_ABS`](#r_arm_base_abs)
  - [`R_ARM_ALU_PCREL_7_0`](#r_arm_alu_pcrel_7_0)
  - [`R_ARM_ALU_PCREL_15_8`](#r_arm_alu_pcrel_15_8)
  - [`R_ARM_ALU_PCREL_23_15`](#r_arm_alu_pcrel_23_15)
  - [`R_ARM_LDR_SBREL_11_0`](#r_arm_ldr_sbrel_11_0)
  - [`R_ARM_ALU_SBREL_19_12`](#r_arm_alu_sbrel_19_12)
  - [`R_ARM_ALU_SBREL_27_20`](#r_arm_alu_sbrel_27_20)
  - [`R_ARM_TARGET1`](#r_arm_target1)
  - [`R_ARM_SBREL31`](#r_arm_sbrel31)
  - [`R_ARM_V4BX`](#r_arm_v4bx)
  - [`R_ARM_TARGET2`](#r_arm_target2)
  - [`R_ARM_PREL31`](#r_arm_prel31)
  - [`R_ARM_MOVW_ABS_NC`](#r_arm_movw_abs_nc)
  - [`R_ARM_MOVT_ABS`](#r_arm_movt_abs)
  - [`R_ARM_MOVW_PREL_NC`](#r_arm_movw_prel_nc)
  - [`R_ARM_MOVT_PREL`](#r_arm_movt_prel)
  - [`R_ARM_THM_MOVW_ABS_NC`](#r_arm_thm_movw_abs_nc)
  - [`R_ARM_THM_MOVT_ABS`](#r_arm_thm_movt_abs)
  - [`R_ARM_THM_MOVW_PREL_NC`](#r_arm_thm_movw_prel_nc)
  - [`R_ARM_THM_MOVT_PREL`](#r_arm_thm_movt_prel)
  - [`R_ARM_THM_JUMP19`](#r_arm_thm_jump19)
  - [`R_ARM_THM_JUMP6`](#r_arm_thm_jump6)
  - [`R_ARM_THM_ALU_PREL_11_0`](#r_arm_thm_alu_prel_11_0)
  - [`R_ARM_THM_PC12`](#r_arm_thm_pc12)
  - [`R_ARM_ABS32_NOI`](#r_arm_abs32_noi)
  - [`R_ARM_REL32_NOI`](#r_arm_rel32_noi)
  - [`R_ARM_ALU_PC_G0_NC`](#r_arm_alu_pc_g0_nc)
  - [`R_ARM_ALU_PC_G0`](#r_arm_alu_pc_g0)
  - [`R_ARM_ALU_PC_G1_NC`](#r_arm_alu_pc_g1_nc)
  - [`R_ARM_ALU_PC_G1`](#r_arm_alu_pc_g1)
  - [`R_ARM_ALU_PC_G2`](#r_arm_alu_pc_g2)
  - [`R_ARM_LDR_PC_G1`](#r_arm_ldr_pc_g1)
  - [`R_ARM_LDR_PC_G2`](#r_arm_ldr_pc_g2)
  - [`R_ARM_LDRS_PC_G0`](#r_arm_ldrs_pc_g0)
  - [`R_ARM_LDRS_PC_G1`](#r_arm_ldrs_pc_g1)
  - [`R_ARM_LDRS_PC_G2`](#r_arm_ldrs_pc_g2)
  - [`R_ARM_LDC_PC_G0`](#r_arm_ldc_pc_g0)
  - [`R_ARM_LDC_PC_G1`](#r_arm_ldc_pc_g1)
  - [`R_ARM_LDC_PC_G2`](#r_arm_ldc_pc_g2)
  - [`R_ARM_ALU_SB_G0_NC`](#r_arm_alu_sb_g0_nc)
  - [`R_ARM_ALU_SB_G0`](#r_arm_alu_sb_g0)
  - [`R_ARM_ALU_SB_G1_NC`](#r_arm_alu_sb_g1_nc)
  - [`R_ARM_ALU_SB_G1`](#r_arm_alu_sb_g1)
  - [`R_ARM_ALU_SB_G2`](#r_arm_alu_sb_g2)
  - [`R_ARM_LDR_SB_G0`](#r_arm_ldr_sb_g0)
  - [`R_ARM_LDR_SB_G1`](#r_arm_ldr_sb_g1)
  - [`R_ARM_LDR_SB_G2`](#r_arm_ldr_sb_g2)
  - [`R_ARM_LDRS_SB_G0`](#r_arm_ldrs_sb_g0)
  - [`R_ARM_LDRS_SB_G1`](#r_arm_ldrs_sb_g1)
  - [`R_ARM_LDRS_SB_G2`](#r_arm_ldrs_sb_g2)
  - [`R_ARM_LDC_SB_G0`](#r_arm_ldc_sb_g0)
  - [`R_ARM_LDC_SB_G1`](#r_arm_ldc_sb_g1)
  - [`R_ARM_LDC_SB_G2`](#r_arm_ldc_sb_g2)
  - [`R_ARM_MOVW_BREL_NC`](#r_arm_movw_brel_nc)
  - [`R_ARM_MOVT_BREL`](#r_arm_movt_brel)
  - [`R_ARM_MOVW_BREL`](#r_arm_movw_brel)
  - [`R_ARM_THM_MOVW_BREL_NC`](#r_arm_thm_movw_brel_nc)
  - [`R_ARM_THM_MOVT_BREL`](#r_arm_thm_movt_brel)
  - [`R_ARM_THM_MOVW_BREL`](#r_arm_thm_movw_brel)
  - [`R_ARM_TLS_GOTDESC`](#r_arm_tls_gotdesc)
  - [`R_ARM_TLS_CALL`](#r_arm_tls_call)
  - [`R_ARM_TLS_DESCSEQ`](#r_arm_tls_descseq)
  - [`R_ARM_THM_TLS_CALL`](#r_arm_thm_tls_call)
  - [`R_ARM_PLT32_ABS`](#r_arm_plt32_abs)
  - [`R_ARM_GOT_ABS`](#r_arm_got_abs)
  - [`R_ARM_GOT_PREL`](#r_arm_got_prel)
  - [`R_ARM_GOT_BREL12`](#r_arm_got_brel12)
  - [`R_ARM_GOTOFF12`](#r_arm_gotoff12)
  - [`R_ARM_GOTRELAX`](#r_arm_gotrelax)
  - [`R_ARM_GNU_VTENTRY`](#r_arm_gnu_vtentry)
  - [`R_ARM_GNU_VTINHERIT`](#r_arm_gnu_vtinherit)
  - [`R_ARM_THM_PC11`](#r_arm_thm_pc11)
  - [`R_ARM_THM_PC9`](#r_arm_thm_pc9)
  - [`R_ARM_TLS_GD32`](#r_arm_tls_gd32)
  - [`R_ARM_TLS_LDM32`](#r_arm_tls_ldm32)
  - [`R_ARM_TLS_LDO32`](#r_arm_tls_ldo32)
  - [`R_ARM_TLS_IE32`](#r_arm_tls_ie32)
  - [`R_ARM_TLS_LE32`](#r_arm_tls_le32)
  - [`R_ARM_TLS_LDO12`](#r_arm_tls_ldo12)
  - [`R_ARM_TLS_LE12`](#r_arm_tls_le12)
  - [`R_ARM_TLS_IE12GP`](#r_arm_tls_ie12gp)
  - [`R_ARM_ME_TOO`](#r_arm_me_too)
  - [`R_ARM_THM_TLS_DESCSEQ`](#r_arm_thm_tls_descseq)
  - [`R_ARM_THM_TLS_DESCSEQ16`](#r_arm_thm_tls_descseq16)
  - [`R_ARM_THM_TLS_DESCSEQ32`](#r_arm_thm_tls_descseq32)
  - [`R_ARM_THM_GOT_BREL12`](#r_arm_thm_got_brel12)
  - [`R_ARM_IRELATIVE`](#r_arm_irelative)
  - [`R_ARM_RXPC25`](#r_arm_rxpc25)
  - [`R_ARM_RSBREL32`](#r_arm_rsbrel32)
  - [`R_ARM_THM_RPC22`](#r_arm_thm_rpc22)
  - [`R_ARM_RREL32`](#r_arm_rrel32)
  - [`R_ARM_RABS22`](#r_arm_rabs22)
  - [`R_ARM_RPC24`](#r_arm_rpc24)
  - [`R_ARM_RBASE`](#r_arm_rbase)
  - [`R_CKCORE_NONE`](#r_ckcore_none)
  - [`R_CKCORE_ADDR32`](#r_ckcore_addr32)
  - [`R_CKCORE_PCRELIMM8BY4`](#r_ckcore_pcrelimm8by4)
  - [`R_CKCORE_PCRELIMM11BY2`](#r_ckcore_pcrelimm11by2)
  - [`R_CKCORE_PCREL32`](#r_ckcore_pcrel32)
  - [`R_CKCORE_PCRELJSR_IMM11BY2`](#r_ckcore_pcreljsr_imm11by2)
  - [`R_CKCORE_RELATIVE`](#r_ckcore_relative)
  - [`R_CKCORE_COPY`](#r_ckcore_copy)
  - [`R_CKCORE_GLOB_DAT`](#r_ckcore_glob_dat)
  - [`R_CKCORE_JUMP_SLOT`](#r_ckcore_jump_slot)
  - [`R_CKCORE_GOTOFF`](#r_ckcore_gotoff)
  - [`R_CKCORE_GOTPC`](#r_ckcore_gotpc)
  - [`R_CKCORE_GOT32`](#r_ckcore_got32)
  - [`R_CKCORE_PLT32`](#r_ckcore_plt32)
  - [`R_CKCORE_ADDRGOT`](#r_ckcore_addrgot)
  - [`R_CKCORE_ADDRPLT`](#r_ckcore_addrplt)
  - [`R_CKCORE_PCREL_IMM26BY2`](#r_ckcore_pcrel_imm26by2)
  - [`R_CKCORE_PCREL_IMM16BY2`](#r_ckcore_pcrel_imm16by2)
  - [`R_CKCORE_PCREL_IMM16BY4`](#r_ckcore_pcrel_imm16by4)
  - [`R_CKCORE_PCREL_IMM10BY2`](#r_ckcore_pcrel_imm10by2)
  - [`R_CKCORE_PCREL_IMM10BY4`](#r_ckcore_pcrel_imm10by4)
  - [`R_CKCORE_ADDR_HI16`](#r_ckcore_addr_hi16)
  - [`R_CKCORE_ADDR_LO16`](#r_ckcore_addr_lo16)
  - [`R_CKCORE_GOTPC_HI16`](#r_ckcore_gotpc_hi16)
  - [`R_CKCORE_GOTPC_LO16`](#r_ckcore_gotpc_lo16)
  - [`R_CKCORE_GOTOFF_HI16`](#r_ckcore_gotoff_hi16)
  - [`R_CKCORE_GOTOFF_LO16`](#r_ckcore_gotoff_lo16)
  - [`R_CKCORE_GOT12`](#r_ckcore_got12)
  - [`R_CKCORE_GOT_HI16`](#r_ckcore_got_hi16)
  - [`R_CKCORE_GOT_LO16`](#r_ckcore_got_lo16)
  - [`R_CKCORE_PLT12`](#r_ckcore_plt12)
  - [`R_CKCORE_PLT_HI16`](#r_ckcore_plt_hi16)
  - [`R_CKCORE_PLT_LO16`](#r_ckcore_plt_lo16)
  - [`R_CKCORE_ADDRGOT_HI16`](#r_ckcore_addrgot_hi16)
  - [`R_CKCORE_ADDRGOT_LO16`](#r_ckcore_addrgot_lo16)
  - [`R_CKCORE_ADDRPLT_HI16`](#r_ckcore_addrplt_hi16)
  - [`R_CKCORE_ADDRPLT_LO16`](#r_ckcore_addrplt_lo16)
  - [`R_CKCORE_PCREL_JSR_IMM26BY2`](#r_ckcore_pcrel_jsr_imm26by2)
  - [`R_CKCORE_TOFFSET_LO16`](#r_ckcore_toffset_lo16)
  - [`R_CKCORE_DOFFSET_LO16`](#r_ckcore_doffset_lo16)
  - [`R_CKCORE_PCREL_IMM18BY2`](#r_ckcore_pcrel_imm18by2)
  - [`R_CKCORE_DOFFSET_IMM18`](#r_ckcore_doffset_imm18)
  - [`R_CKCORE_DOFFSET_IMM18BY2`](#r_ckcore_doffset_imm18by2)
  - [`R_CKCORE_DOFFSET_IMM18BY4`](#r_ckcore_doffset_imm18by4)
  - [`R_CKCORE_GOT_IMM18BY4`](#r_ckcore_got_imm18by4)
  - [`R_CKCORE_PLT_IMM18BY4`](#r_ckcore_plt_imm18by4)
  - [`R_CKCORE_PCREL_IMM7BY4`](#r_ckcore_pcrel_imm7by4)
  - [`R_CKCORE_TLS_LE32`](#r_ckcore_tls_le32)
  - [`R_CKCORE_TLS_IE32`](#r_ckcore_tls_ie32)
  - [`R_CKCORE_TLS_GD32`](#r_ckcore_tls_gd32)
  - [`R_CKCORE_TLS_LDM32`](#r_ckcore_tls_ldm32)
  - [`R_CKCORE_TLS_LDO32`](#r_ckcore_tls_ldo32)
  - [`R_CKCORE_TLS_DTPMOD32`](#r_ckcore_tls_dtpmod32)
  - [`R_CKCORE_TLS_DTPOFF32`](#r_ckcore_tls_dtpoff32)
  - [`R_CKCORE_TLS_TPOFF32`](#r_ckcore_tls_tpoff32)
  - [`EF_CSKY_ABIMASK`](#ef_csky_abimask)
  - [`EF_CSKY_OTHER`](#ef_csky_other)
  - [`EF_CSKY_PROCESSOR`](#ef_csky_processor)
  - [`EF_CSKY_ABIV1`](#ef_csky_abiv1)
  - [`EF_CSKY_ABIV2`](#ef_csky_abiv2)
  - [`SHT_CSKY_ATTRIBUTES`](#sht_csky_attributes)
  - [`EF_IA_64_MASKOS`](#ef_ia_64_maskos)
  - [`EF_IA_64_ABI64`](#ef_ia_64_abi64)
  - [`EF_IA_64_ARCH`](#ef_ia_64_arch)
  - [`PT_IA_64_ARCHEXT`](#pt_ia_64_archext)
  - [`PT_IA_64_UNWIND`](#pt_ia_64_unwind)
  - [`PT_IA_64_HP_OPT_ANOT`](#pt_ia_64_hp_opt_anot)
  - [`PT_IA_64_HP_HSL_ANOT`](#pt_ia_64_hp_hsl_anot)
  - [`PT_IA_64_HP_STACK`](#pt_ia_64_hp_stack)
  - [`PF_IA_64_NORECOV`](#pf_ia_64_norecov)
  - [`SHT_IA_64_EXT`](#sht_ia_64_ext)
  - [`SHT_IA_64_UNWIND`](#sht_ia_64_unwind)
  - [`SHF_IA_64_SHORT`](#shf_ia_64_short)
  - [`SHF_IA_64_NORECOV`](#shf_ia_64_norecov)
  - [`DT_IA_64_PLT_RESERVE`](#dt_ia_64_plt_reserve)
  - [`R_IA64_NONE`](#r_ia64_none)
  - [`R_IA64_IMM14`](#r_ia64_imm14)
  - [`R_IA64_IMM22`](#r_ia64_imm22)
  - [`R_IA64_IMM64`](#r_ia64_imm64)
  - [`R_IA64_DIR32MSB`](#r_ia64_dir32msb)
  - [`R_IA64_DIR32LSB`](#r_ia64_dir32lsb)
  - [`R_IA64_DIR64MSB`](#r_ia64_dir64msb)
  - [`R_IA64_DIR64LSB`](#r_ia64_dir64lsb)
  - [`R_IA64_GPREL22`](#r_ia64_gprel22)
  - [`R_IA64_GPREL64I`](#r_ia64_gprel64i)
  - [`R_IA64_GPREL32MSB`](#r_ia64_gprel32msb)
  - [`R_IA64_GPREL32LSB`](#r_ia64_gprel32lsb)
  - [`R_IA64_GPREL64MSB`](#r_ia64_gprel64msb)
  - [`R_IA64_GPREL64LSB`](#r_ia64_gprel64lsb)
  - [`R_IA64_LTOFF22`](#r_ia64_ltoff22)
  - [`R_IA64_LTOFF64I`](#r_ia64_ltoff64i)
  - [`R_IA64_PLTOFF22`](#r_ia64_pltoff22)
  - [`R_IA64_PLTOFF64I`](#r_ia64_pltoff64i)
  - [`R_IA64_PLTOFF64MSB`](#r_ia64_pltoff64msb)
  - [`R_IA64_PLTOFF64LSB`](#r_ia64_pltoff64lsb)
  - [`R_IA64_FPTR64I`](#r_ia64_fptr64i)
  - [`R_IA64_FPTR32MSB`](#r_ia64_fptr32msb)
  - [`R_IA64_FPTR32LSB`](#r_ia64_fptr32lsb)
  - [`R_IA64_FPTR64MSB`](#r_ia64_fptr64msb)
  - [`R_IA64_FPTR64LSB`](#r_ia64_fptr64lsb)
  - [`R_IA64_PCREL60B`](#r_ia64_pcrel60b)
  - [`R_IA64_PCREL21B`](#r_ia64_pcrel21b)
  - [`R_IA64_PCREL21M`](#r_ia64_pcrel21m)
  - [`R_IA64_PCREL21F`](#r_ia64_pcrel21f)
  - [`R_IA64_PCREL32MSB`](#r_ia64_pcrel32msb)
  - [`R_IA64_PCREL32LSB`](#r_ia64_pcrel32lsb)
  - [`R_IA64_PCREL64MSB`](#r_ia64_pcrel64msb)
  - [`R_IA64_PCREL64LSB`](#r_ia64_pcrel64lsb)
  - [`R_IA64_LTOFF_FPTR22`](#r_ia64_ltoff_fptr22)
  - [`R_IA64_LTOFF_FPTR64I`](#r_ia64_ltoff_fptr64i)
  - [`R_IA64_LTOFF_FPTR32MSB`](#r_ia64_ltoff_fptr32msb)
  - [`R_IA64_LTOFF_FPTR32LSB`](#r_ia64_ltoff_fptr32lsb)
  - [`R_IA64_LTOFF_FPTR64MSB`](#r_ia64_ltoff_fptr64msb)
  - [`R_IA64_LTOFF_FPTR64LSB`](#r_ia64_ltoff_fptr64lsb)
  - [`R_IA64_SEGREL32MSB`](#r_ia64_segrel32msb)
  - [`R_IA64_SEGREL32LSB`](#r_ia64_segrel32lsb)
  - [`R_IA64_SEGREL64MSB`](#r_ia64_segrel64msb)
  - [`R_IA64_SEGREL64LSB`](#r_ia64_segrel64lsb)
  - [`R_IA64_SECREL32MSB`](#r_ia64_secrel32msb)
  - [`R_IA64_SECREL32LSB`](#r_ia64_secrel32lsb)
  - [`R_IA64_SECREL64MSB`](#r_ia64_secrel64msb)
  - [`R_IA64_SECREL64LSB`](#r_ia64_secrel64lsb)
  - [`R_IA64_REL32MSB`](#r_ia64_rel32msb)
  - [`R_IA64_REL32LSB`](#r_ia64_rel32lsb)
  - [`R_IA64_REL64MSB`](#r_ia64_rel64msb)
  - [`R_IA64_REL64LSB`](#r_ia64_rel64lsb)
  - [`R_IA64_LTV32MSB`](#r_ia64_ltv32msb)
  - [`R_IA64_LTV32LSB`](#r_ia64_ltv32lsb)
  - [`R_IA64_LTV64MSB`](#r_ia64_ltv64msb)
  - [`R_IA64_LTV64LSB`](#r_ia64_ltv64lsb)
  - [`R_IA64_PCREL21BI`](#r_ia64_pcrel21bi)
  - [`R_IA64_PCREL22`](#r_ia64_pcrel22)
  - [`R_IA64_PCREL64I`](#r_ia64_pcrel64i)
  - [`R_IA64_IPLTMSB`](#r_ia64_ipltmsb)
  - [`R_IA64_IPLTLSB`](#r_ia64_ipltlsb)
  - [`R_IA64_COPY`](#r_ia64_copy)
  - [`R_IA64_SUB`](#r_ia64_sub)
  - [`R_IA64_LTOFF22X`](#r_ia64_ltoff22x)
  - [`R_IA64_LDXMOV`](#r_ia64_ldxmov)
  - [`R_IA64_TPREL14`](#r_ia64_tprel14)
  - [`R_IA64_TPREL22`](#r_ia64_tprel22)
  - [`R_IA64_TPREL64I`](#r_ia64_tprel64i)
  - [`R_IA64_TPREL64MSB`](#r_ia64_tprel64msb)
  - [`R_IA64_TPREL64LSB`](#r_ia64_tprel64lsb)
  - [`R_IA64_LTOFF_TPREL22`](#r_ia64_ltoff_tprel22)
  - [`R_IA64_DTPMOD64MSB`](#r_ia64_dtpmod64msb)
  - [`R_IA64_DTPMOD64LSB`](#r_ia64_dtpmod64lsb)
  - [`R_IA64_LTOFF_DTPMOD22`](#r_ia64_ltoff_dtpmod22)
  - [`R_IA64_DTPREL14`](#r_ia64_dtprel14)
  - [`R_IA64_DTPREL22`](#r_ia64_dtprel22)
  - [`R_IA64_DTPREL64I`](#r_ia64_dtprel64i)
  - [`R_IA64_DTPREL32MSB`](#r_ia64_dtprel32msb)
  - [`R_IA64_DTPREL32LSB`](#r_ia64_dtprel32lsb)
  - [`R_IA64_DTPREL64MSB`](#r_ia64_dtprel64msb)
  - [`R_IA64_DTPREL64LSB`](#r_ia64_dtprel64lsb)
  - [`R_IA64_LTOFF_DTPREL22`](#r_ia64_ltoff_dtprel22)
  - [`EF_SH_MACH_MASK`](#ef_sh_mach_mask)
  - [`EF_SH_UNKNOWN`](#ef_sh_unknown)
  - [`EF_SH1`](#ef_sh1)
  - [`EF_SH2`](#ef_sh2)
  - [`EF_SH3`](#ef_sh3)
  - [`EF_SH_DSP`](#ef_sh_dsp)
  - [`EF_SH3_DSP`](#ef_sh3_dsp)
  - [`EF_SH4AL_DSP`](#ef_sh4al_dsp)
  - [`EF_SH3E`](#ef_sh3e)
  - [`EF_SH4`](#ef_sh4)
  - [`EF_SH2E`](#ef_sh2e)
  - [`EF_SH4A`](#ef_sh4a)
  - [`EF_SH2A`](#ef_sh2a)
  - [`EF_SH4_NOFPU`](#ef_sh4_nofpu)
  - [`EF_SH4A_NOFPU`](#ef_sh4a_nofpu)
  - [`EF_SH4_NOMMU_NOFPU`](#ef_sh4_nommu_nofpu)
  - [`EF_SH2A_NOFPU`](#ef_sh2a_nofpu)
  - [`EF_SH3_NOMMU`](#ef_sh3_nommu)
  - [`EF_SH2A_SH4_NOFPU`](#ef_sh2a_sh4_nofpu)
  - [`EF_SH2A_SH3_NOFPU`](#ef_sh2a_sh3_nofpu)
  - [`EF_SH2A_SH4`](#ef_sh2a_sh4)
  - [`EF_SH2A_SH3E`](#ef_sh2a_sh3e)
  - [`R_SH_NONE`](#r_sh_none)
  - [`R_SH_DIR32`](#r_sh_dir32)
  - [`R_SH_REL32`](#r_sh_rel32)
  - [`R_SH_DIR8WPN`](#r_sh_dir8wpn)
  - [`R_SH_IND12W`](#r_sh_ind12w)
  - [`R_SH_DIR8WPL`](#r_sh_dir8wpl)
  - [`R_SH_DIR8WPZ`](#r_sh_dir8wpz)
  - [`R_SH_DIR8BP`](#r_sh_dir8bp)
  - [`R_SH_DIR8W`](#r_sh_dir8w)
  - [`R_SH_DIR8L`](#r_sh_dir8l)
  - [`R_SH_SWITCH16`](#r_sh_switch16)
  - [`R_SH_SWITCH32`](#r_sh_switch32)
  - [`R_SH_USES`](#r_sh_uses)
  - [`R_SH_COUNT`](#r_sh_count)
  - [`R_SH_ALIGN`](#r_sh_align)
  - [`R_SH_CODE`](#r_sh_code)
  - [`R_SH_DATA`](#r_sh_data)
  - [`R_SH_LABEL`](#r_sh_label)
  - [`R_SH_SWITCH8`](#r_sh_switch8)
  - [`R_SH_GNU_VTINHERIT`](#r_sh_gnu_vtinherit)
  - [`R_SH_GNU_VTENTRY`](#r_sh_gnu_vtentry)
  - [`R_SH_TLS_GD_32`](#r_sh_tls_gd_32)
  - [`R_SH_TLS_LD_32`](#r_sh_tls_ld_32)
  - [`R_SH_TLS_LDO_32`](#r_sh_tls_ldo_32)
  - [`R_SH_TLS_IE_32`](#r_sh_tls_ie_32)
  - [`R_SH_TLS_LE_32`](#r_sh_tls_le_32)
  - [`R_SH_TLS_DTPMOD32`](#r_sh_tls_dtpmod32)
  - [`R_SH_TLS_DTPOFF32`](#r_sh_tls_dtpoff32)
  - [`R_SH_TLS_TPOFF32`](#r_sh_tls_tpoff32)
  - [`R_SH_GOT32`](#r_sh_got32)
  - [`R_SH_PLT32`](#r_sh_plt32)
  - [`R_SH_COPY`](#r_sh_copy)
  - [`R_SH_GLOB_DAT`](#r_sh_glob_dat)
  - [`R_SH_JMP_SLOT`](#r_sh_jmp_slot)
  - [`R_SH_RELATIVE`](#r_sh_relative)
  - [`R_SH_GOTOFF`](#r_sh_gotoff)
  - [`R_SH_GOTPC`](#r_sh_gotpc)
  - [`EF_S390_HIGH_GPRS`](#ef_s390_high_gprs)
  - [`R_390_NONE`](#r_390_none)
  - [`R_390_8`](#r_390_8)
  - [`R_390_12`](#r_390_12)
  - [`R_390_16`](#r_390_16)
  - [`R_390_32`](#r_390_32)
  - [`R_390_PC32`](#r_390_pc32)
  - [`R_390_GOT12`](#r_390_got12)
  - [`R_390_GOT32`](#r_390_got32)
  - [`R_390_PLT32`](#r_390_plt32)
  - [`R_390_COPY`](#r_390_copy)
  - [`R_390_GLOB_DAT`](#r_390_glob_dat)
  - [`R_390_JMP_SLOT`](#r_390_jmp_slot)
  - [`R_390_RELATIVE`](#r_390_relative)
  - [`R_390_GOTOFF32`](#r_390_gotoff32)
  - [`R_390_GOTPC`](#r_390_gotpc)
  - [`R_390_GOT16`](#r_390_got16)
  - [`R_390_PC16`](#r_390_pc16)
  - [`R_390_PC16DBL`](#r_390_pc16dbl)
  - [`R_390_PLT16DBL`](#r_390_plt16dbl)
  - [`R_390_PC32DBL`](#r_390_pc32dbl)
  - [`R_390_PLT32DBL`](#r_390_plt32dbl)
  - [`R_390_GOTPCDBL`](#r_390_gotpcdbl)
  - [`R_390_64`](#r_390_64)
  - [`R_390_PC64`](#r_390_pc64)
  - [`R_390_GOT64`](#r_390_got64)
  - [`R_390_PLT64`](#r_390_plt64)
  - [`R_390_GOTENT`](#r_390_gotent)
  - [`R_390_GOTOFF16`](#r_390_gotoff16)
  - [`R_390_GOTOFF64`](#r_390_gotoff64)
  - [`R_390_GOTPLT12`](#r_390_gotplt12)
  - [`R_390_GOTPLT16`](#r_390_gotplt16)
  - [`R_390_GOTPLT32`](#r_390_gotplt32)
  - [`R_390_GOTPLT64`](#r_390_gotplt64)
  - [`R_390_GOTPLTENT`](#r_390_gotpltent)
  - [`R_390_PLTOFF16`](#r_390_pltoff16)
  - [`R_390_PLTOFF32`](#r_390_pltoff32)
  - [`R_390_PLTOFF64`](#r_390_pltoff64)
  - [`R_390_TLS_LOAD`](#r_390_tls_load)
  - [`R_390_TLS_GDCALL`](#r_390_tls_gdcall)
  - [`R_390_TLS_LDCALL`](#r_390_tls_ldcall)
  - [`R_390_TLS_GD32`](#r_390_tls_gd32)
  - [`R_390_TLS_GD64`](#r_390_tls_gd64)
  - [`R_390_TLS_GOTIE12`](#r_390_tls_gotie12)
  - [`R_390_TLS_GOTIE32`](#r_390_tls_gotie32)
  - [`R_390_TLS_GOTIE64`](#r_390_tls_gotie64)
  - [`R_390_TLS_LDM32`](#r_390_tls_ldm32)
  - [`R_390_TLS_LDM64`](#r_390_tls_ldm64)
  - [`R_390_TLS_IE32`](#r_390_tls_ie32)
  - [`R_390_TLS_IE64`](#r_390_tls_ie64)
  - [`R_390_TLS_IEENT`](#r_390_tls_ieent)
  - [`R_390_TLS_LE32`](#r_390_tls_le32)
  - [`R_390_TLS_LE64`](#r_390_tls_le64)
  - [`R_390_TLS_LDO32`](#r_390_tls_ldo32)
  - [`R_390_TLS_LDO64`](#r_390_tls_ldo64)
  - [`R_390_TLS_DTPMOD`](#r_390_tls_dtpmod)
  - [`R_390_TLS_DTPOFF`](#r_390_tls_dtpoff)
  - [`R_390_TLS_TPOFF`](#r_390_tls_tpoff)
  - [`R_390_20`](#r_390_20)
  - [`R_390_GOT20`](#r_390_got20)
  - [`R_390_GOTPLT20`](#r_390_gotplt20)
  - [`R_390_TLS_GOTIE20`](#r_390_tls_gotie20)
  - [`R_390_IRELATIVE`](#r_390_irelative)
  - [`R_CRIS_NONE`](#r_cris_none)
  - [`R_CRIS_8`](#r_cris_8)
  - [`R_CRIS_16`](#r_cris_16)
  - [`R_CRIS_32`](#r_cris_32)
  - [`R_CRIS_8_PCREL`](#r_cris_8_pcrel)
  - [`R_CRIS_16_PCREL`](#r_cris_16_pcrel)
  - [`R_CRIS_32_PCREL`](#r_cris_32_pcrel)
  - [`R_CRIS_GNU_VTINHERIT`](#r_cris_gnu_vtinherit)
  - [`R_CRIS_GNU_VTENTRY`](#r_cris_gnu_vtentry)
  - [`R_CRIS_COPY`](#r_cris_copy)
  - [`R_CRIS_GLOB_DAT`](#r_cris_glob_dat)
  - [`R_CRIS_JUMP_SLOT`](#r_cris_jump_slot)
  - [`R_CRIS_RELATIVE`](#r_cris_relative)
  - [`R_CRIS_16_GOT`](#r_cris_16_got)
  - [`R_CRIS_32_GOT`](#r_cris_32_got)
  - [`R_CRIS_16_GOTPLT`](#r_cris_16_gotplt)
  - [`R_CRIS_32_GOTPLT`](#r_cris_32_gotplt)
  - [`R_CRIS_32_GOTREL`](#r_cris_32_gotrel)
  - [`R_CRIS_32_PLT_GOTREL`](#r_cris_32_plt_gotrel)
  - [`R_CRIS_32_PLT_PCREL`](#r_cris_32_plt_pcrel)
  - [`R_X86_64_NONE`](#r_x86_64_none)
  - [`R_X86_64_64`](#r_x86_64_64)
  - [`R_X86_64_PC32`](#r_x86_64_pc32)
  - [`R_X86_64_GOT32`](#r_x86_64_got32)
  - [`R_X86_64_PLT32`](#r_x86_64_plt32)
  - [`R_X86_64_COPY`](#r_x86_64_copy)
  - [`R_X86_64_GLOB_DAT`](#r_x86_64_glob_dat)
  - [`R_X86_64_JUMP_SLOT`](#r_x86_64_jump_slot)
  - [`R_X86_64_RELATIVE`](#r_x86_64_relative)
  - [`R_X86_64_GOTPCREL`](#r_x86_64_gotpcrel)
  - [`R_X86_64_32`](#r_x86_64_32)
  - [`R_X86_64_32S`](#r_x86_64_32s)
  - [`R_X86_64_16`](#r_x86_64_16)
  - [`R_X86_64_PC16`](#r_x86_64_pc16)
  - [`R_X86_64_8`](#r_x86_64_8)
  - [`R_X86_64_PC8`](#r_x86_64_pc8)
  - [`R_X86_64_DTPMOD64`](#r_x86_64_dtpmod64)
  - [`R_X86_64_DTPOFF64`](#r_x86_64_dtpoff64)
  - [`R_X86_64_TPOFF64`](#r_x86_64_tpoff64)
  - [`R_X86_64_TLSGD`](#r_x86_64_tlsgd)
  - [`R_X86_64_TLSLD`](#r_x86_64_tlsld)
  - [`R_X86_64_DTPOFF32`](#r_x86_64_dtpoff32)
  - [`R_X86_64_GOTTPOFF`](#r_x86_64_gottpoff)
  - [`R_X86_64_TPOFF32`](#r_x86_64_tpoff32)
  - [`R_X86_64_PC64`](#r_x86_64_pc64)
  - [`R_X86_64_GOTOFF64`](#r_x86_64_gotoff64)
  - [`R_X86_64_GOTPC32`](#r_x86_64_gotpc32)
  - [`R_X86_64_GOT64`](#r_x86_64_got64)
  - [`R_X86_64_GOTPCREL64`](#r_x86_64_gotpcrel64)
  - [`R_X86_64_GOTPC64`](#r_x86_64_gotpc64)
  - [`R_X86_64_GOTPLT64`](#r_x86_64_gotplt64)
  - [`R_X86_64_PLTOFF64`](#r_x86_64_pltoff64)
  - [`R_X86_64_SIZE32`](#r_x86_64_size32)
  - [`R_X86_64_SIZE64`](#r_x86_64_size64)
  - [`R_X86_64_GOTPC32_TLSDESC`](#r_x86_64_gotpc32_tlsdesc)
  - [`R_X86_64_TLSDESC_CALL`](#r_x86_64_tlsdesc_call)
  - [`R_X86_64_TLSDESC`](#r_x86_64_tlsdesc)
  - [`R_X86_64_IRELATIVE`](#r_x86_64_irelative)
  - [`R_X86_64_RELATIVE64`](#r_x86_64_relative64)
  - [`R_X86_64_GOTPCRELX`](#r_x86_64_gotpcrelx)
  - [`R_X86_64_REX_GOTPCRELX`](#r_x86_64_rex_gotpcrelx)
  - [`SHT_X86_64_UNWIND`](#sht_x86_64_unwind)
  - [`R_MN10300_NONE`](#r_mn10300_none)
  - [`R_MN10300_32`](#r_mn10300_32)
  - [`R_MN10300_16`](#r_mn10300_16)
  - [`R_MN10300_8`](#r_mn10300_8)
  - [`R_MN10300_PCREL32`](#r_mn10300_pcrel32)
  - [`R_MN10300_PCREL16`](#r_mn10300_pcrel16)
  - [`R_MN10300_PCREL8`](#r_mn10300_pcrel8)
  - [`R_MN10300_GNU_VTINHERIT`](#r_mn10300_gnu_vtinherit)
  - [`R_MN10300_GNU_VTENTRY`](#r_mn10300_gnu_vtentry)
  - [`R_MN10300_24`](#r_mn10300_24)
  - [`R_MN10300_GOTPC32`](#r_mn10300_gotpc32)
  - [`R_MN10300_GOTPC16`](#r_mn10300_gotpc16)
  - [`R_MN10300_GOTOFF32`](#r_mn10300_gotoff32)
  - [`R_MN10300_GOTOFF24`](#r_mn10300_gotoff24)
  - [`R_MN10300_GOTOFF16`](#r_mn10300_gotoff16)
  - [`R_MN10300_PLT32`](#r_mn10300_plt32)
  - [`R_MN10300_PLT16`](#r_mn10300_plt16)
  - [`R_MN10300_GOT32`](#r_mn10300_got32)
  - [`R_MN10300_GOT24`](#r_mn10300_got24)
  - [`R_MN10300_GOT16`](#r_mn10300_got16)
  - [`R_MN10300_COPY`](#r_mn10300_copy)
  - [`R_MN10300_GLOB_DAT`](#r_mn10300_glob_dat)
  - [`R_MN10300_JMP_SLOT`](#r_mn10300_jmp_slot)
  - [`R_MN10300_RELATIVE`](#r_mn10300_relative)
  - [`R_MN10300_TLS_GD`](#r_mn10300_tls_gd)
  - [`R_MN10300_TLS_LD`](#r_mn10300_tls_ld)
  - [`R_MN10300_TLS_LDO`](#r_mn10300_tls_ldo)
  - [`R_MN10300_TLS_GOTIE`](#r_mn10300_tls_gotie)
  - [`R_MN10300_TLS_IE`](#r_mn10300_tls_ie)
  - [`R_MN10300_TLS_LE`](#r_mn10300_tls_le)
  - [`R_MN10300_TLS_DTPMOD`](#r_mn10300_tls_dtpmod)
  - [`R_MN10300_TLS_DTPOFF`](#r_mn10300_tls_dtpoff)
  - [`R_MN10300_TLS_TPOFF`](#r_mn10300_tls_tpoff)
  - [`R_MN10300_SYM_DIFF`](#r_mn10300_sym_diff)
  - [`R_MN10300_ALIGN`](#r_mn10300_align)
  - [`R_M32R_NONE`](#r_m32r_none)
  - [`R_M32R_16`](#r_m32r_16)
  - [`R_M32R_32`](#r_m32r_32)
  - [`R_M32R_24`](#r_m32r_24)
  - [`R_M32R_10_PCREL`](#r_m32r_10_pcrel)
  - [`R_M32R_18_PCREL`](#r_m32r_18_pcrel)
  - [`R_M32R_26_PCREL`](#r_m32r_26_pcrel)
  - [`R_M32R_HI16_ULO`](#r_m32r_hi16_ulo)
  - [`R_M32R_HI16_SLO`](#r_m32r_hi16_slo)
  - [`R_M32R_LO16`](#r_m32r_lo16)
  - [`R_M32R_SDA16`](#r_m32r_sda16)
  - [`R_M32R_GNU_VTINHERIT`](#r_m32r_gnu_vtinherit)
  - [`R_M32R_GNU_VTENTRY`](#r_m32r_gnu_vtentry)
  - [`R_M32R_16_RELA`](#r_m32r_16_rela)
  - [`R_M32R_32_RELA`](#r_m32r_32_rela)
  - [`R_M32R_24_RELA`](#r_m32r_24_rela)
  - [`R_M32R_10_PCREL_RELA`](#r_m32r_10_pcrel_rela)
  - [`R_M32R_18_PCREL_RELA`](#r_m32r_18_pcrel_rela)
  - [`R_M32R_26_PCREL_RELA`](#r_m32r_26_pcrel_rela)
  - [`R_M32R_HI16_ULO_RELA`](#r_m32r_hi16_ulo_rela)
  - [`R_M32R_HI16_SLO_RELA`](#r_m32r_hi16_slo_rela)
  - [`R_M32R_LO16_RELA`](#r_m32r_lo16_rela)
  - [`R_M32R_SDA16_RELA`](#r_m32r_sda16_rela)
  - [`R_M32R_RELA_GNU_VTINHERIT`](#r_m32r_rela_gnu_vtinherit)
  - [`R_M32R_RELA_GNU_VTENTRY`](#r_m32r_rela_gnu_vtentry)
  - [`R_M32R_REL32`](#r_m32r_rel32)
  - [`R_M32R_GOT24`](#r_m32r_got24)
  - [`R_M32R_26_PLTREL`](#r_m32r_26_pltrel)
  - [`R_M32R_COPY`](#r_m32r_copy)
  - [`R_M32R_GLOB_DAT`](#r_m32r_glob_dat)
  - [`R_M32R_JMP_SLOT`](#r_m32r_jmp_slot)
  - [`R_M32R_RELATIVE`](#r_m32r_relative)
  - [`R_M32R_GOTOFF`](#r_m32r_gotoff)
  - [`R_M32R_GOTPC24`](#r_m32r_gotpc24)
  - [`R_M32R_GOT16_HI_ULO`](#r_m32r_got16_hi_ulo)
  - [`R_M32R_GOT16_HI_SLO`](#r_m32r_got16_hi_slo)
  - [`R_M32R_GOT16_LO`](#r_m32r_got16_lo)
  - [`R_M32R_GOTPC_HI_ULO`](#r_m32r_gotpc_hi_ulo)
  - [`R_M32R_GOTPC_HI_SLO`](#r_m32r_gotpc_hi_slo)
  - [`R_M32R_GOTPC_LO`](#r_m32r_gotpc_lo)
  - [`R_M32R_GOTOFF_HI_ULO`](#r_m32r_gotoff_hi_ulo)
  - [`R_M32R_GOTOFF_HI_SLO`](#r_m32r_gotoff_hi_slo)
  - [`R_M32R_GOTOFF_LO`](#r_m32r_gotoff_lo)
  - [`R_M32R_NUM`](#r_m32r_num)
  - [`R_MICROBLAZE_NONE`](#r_microblaze_none)
  - [`R_MICROBLAZE_32`](#r_microblaze_32)
  - [`R_MICROBLAZE_32_PCREL`](#r_microblaze_32_pcrel)
  - [`R_MICROBLAZE_64_PCREL`](#r_microblaze_64_pcrel)
  - [`R_MICROBLAZE_32_PCREL_LO`](#r_microblaze_32_pcrel_lo)
  - [`R_MICROBLAZE_64`](#r_microblaze_64)
  - [`R_MICROBLAZE_32_LO`](#r_microblaze_32_lo)
  - [`R_MICROBLAZE_SRO32`](#r_microblaze_sro32)
  - [`R_MICROBLAZE_SRW32`](#r_microblaze_srw32)
  - [`R_MICROBLAZE_64_NONE`](#r_microblaze_64_none)
  - [`R_MICROBLAZE_32_SYM_OP_SYM`](#r_microblaze_32_sym_op_sym)
  - [`R_MICROBLAZE_GNU_VTINHERIT`](#r_microblaze_gnu_vtinherit)
  - [`R_MICROBLAZE_GNU_VTENTRY`](#r_microblaze_gnu_vtentry)
  - [`R_MICROBLAZE_GOTPC_64`](#r_microblaze_gotpc_64)
  - [`R_MICROBLAZE_GOT_64`](#r_microblaze_got_64)
  - [`R_MICROBLAZE_PLT_64`](#r_microblaze_plt_64)
  - [`R_MICROBLAZE_REL`](#r_microblaze_rel)
  - [`R_MICROBLAZE_JUMP_SLOT`](#r_microblaze_jump_slot)
  - [`R_MICROBLAZE_GLOB_DAT`](#r_microblaze_glob_dat)
  - [`R_MICROBLAZE_GOTOFF_64`](#r_microblaze_gotoff_64)
  - [`R_MICROBLAZE_GOTOFF_32`](#r_microblaze_gotoff_32)
  - [`R_MICROBLAZE_COPY`](#r_microblaze_copy)
  - [`R_MICROBLAZE_TLS`](#r_microblaze_tls)
  - [`R_MICROBLAZE_TLSGD`](#r_microblaze_tlsgd)
  - [`R_MICROBLAZE_TLSLD`](#r_microblaze_tlsld)
  - [`R_MICROBLAZE_TLSDTPMOD32`](#r_microblaze_tlsdtpmod32)
  - [`R_MICROBLAZE_TLSDTPREL32`](#r_microblaze_tlsdtprel32)
  - [`R_MICROBLAZE_TLSDTPREL64`](#r_microblaze_tlsdtprel64)
  - [`R_MICROBLAZE_TLSGOTTPREL32`](#r_microblaze_tlsgottprel32)
  - [`R_MICROBLAZE_TLSTPREL32`](#r_microblaze_tlstprel32)
  - [`DT_NIOS2_GP`](#dt_nios2_gp)
  - [`R_NIOS2_NONE`](#r_nios2_none)
  - [`R_NIOS2_S16`](#r_nios2_s16)
  - [`R_NIOS2_U16`](#r_nios2_u16)
  - [`R_NIOS2_PCREL16`](#r_nios2_pcrel16)
  - [`R_NIOS2_CALL26`](#r_nios2_call26)
  - [`R_NIOS2_IMM5`](#r_nios2_imm5)
  - [`R_NIOS2_CACHE_OPX`](#r_nios2_cache_opx)
  - [`R_NIOS2_IMM6`](#r_nios2_imm6)
  - [`R_NIOS2_IMM8`](#r_nios2_imm8)
  - [`R_NIOS2_HI16`](#r_nios2_hi16)
  - [`R_NIOS2_LO16`](#r_nios2_lo16)
  - [`R_NIOS2_HIADJ16`](#r_nios2_hiadj16)
  - [`R_NIOS2_BFD_RELOC_32`](#r_nios2_bfd_reloc_32)
  - [`R_NIOS2_BFD_RELOC_16`](#r_nios2_bfd_reloc_16)
  - [`R_NIOS2_BFD_RELOC_8`](#r_nios2_bfd_reloc_8)
  - [`R_NIOS2_GPREL`](#r_nios2_gprel)
  - [`R_NIOS2_GNU_VTINHERIT`](#r_nios2_gnu_vtinherit)
  - [`R_NIOS2_GNU_VTENTRY`](#r_nios2_gnu_vtentry)
  - [`R_NIOS2_UJMP`](#r_nios2_ujmp)
  - [`R_NIOS2_CJMP`](#r_nios2_cjmp)
  - [`R_NIOS2_CALLR`](#r_nios2_callr)
  - [`R_NIOS2_ALIGN`](#r_nios2_align)
  - [`R_NIOS2_GOT16`](#r_nios2_got16)
  - [`R_NIOS2_CALL16`](#r_nios2_call16)
  - [`R_NIOS2_GOTOFF_LO`](#r_nios2_gotoff_lo)
  - [`R_NIOS2_GOTOFF_HA`](#r_nios2_gotoff_ha)
  - [`R_NIOS2_PCREL_LO`](#r_nios2_pcrel_lo)
  - [`R_NIOS2_PCREL_HA`](#r_nios2_pcrel_ha)
  - [`R_NIOS2_TLS_GD16`](#r_nios2_tls_gd16)
  - [`R_NIOS2_TLS_LDM16`](#r_nios2_tls_ldm16)
  - [`R_NIOS2_TLS_LDO16`](#r_nios2_tls_ldo16)
  - [`R_NIOS2_TLS_IE16`](#r_nios2_tls_ie16)
  - [`R_NIOS2_TLS_LE16`](#r_nios2_tls_le16)
  - [`R_NIOS2_TLS_DTPMOD`](#r_nios2_tls_dtpmod)
  - [`R_NIOS2_TLS_DTPREL`](#r_nios2_tls_dtprel)
  - [`R_NIOS2_TLS_TPREL`](#r_nios2_tls_tprel)
  - [`R_NIOS2_COPY`](#r_nios2_copy)
  - [`R_NIOS2_GLOB_DAT`](#r_nios2_glob_dat)
  - [`R_NIOS2_JUMP_SLOT`](#r_nios2_jump_slot)
  - [`R_NIOS2_RELATIVE`](#r_nios2_relative)
  - [`R_NIOS2_GOTOFF`](#r_nios2_gotoff)
  - [`R_NIOS2_CALL26_NOAT`](#r_nios2_call26_noat)
  - [`R_NIOS2_GOT_LO`](#r_nios2_got_lo)
  - [`R_NIOS2_GOT_HA`](#r_nios2_got_ha)
  - [`R_NIOS2_CALL_LO`](#r_nios2_call_lo)
  - [`R_NIOS2_CALL_HA`](#r_nios2_call_ha)
  - [`R_TILEPRO_NONE`](#r_tilepro_none)
  - [`R_TILEPRO_32`](#r_tilepro_32)
  - [`R_TILEPRO_16`](#r_tilepro_16)
  - [`R_TILEPRO_8`](#r_tilepro_8)
  - [`R_TILEPRO_32_PCREL`](#r_tilepro_32_pcrel)
  - [`R_TILEPRO_16_PCREL`](#r_tilepro_16_pcrel)
  - [`R_TILEPRO_8_PCREL`](#r_tilepro_8_pcrel)
  - [`R_TILEPRO_LO16`](#r_tilepro_lo16)
  - [`R_TILEPRO_HI16`](#r_tilepro_hi16)
  - [`R_TILEPRO_HA16`](#r_tilepro_ha16)
  - [`R_TILEPRO_COPY`](#r_tilepro_copy)
  - [`R_TILEPRO_GLOB_DAT`](#r_tilepro_glob_dat)
  - [`R_TILEPRO_JMP_SLOT`](#r_tilepro_jmp_slot)
  - [`R_TILEPRO_RELATIVE`](#r_tilepro_relative)
  - [`R_TILEPRO_BROFF_X1`](#r_tilepro_broff_x1)
  - [`R_TILEPRO_JOFFLONG_X1`](#r_tilepro_jofflong_x1)
  - [`R_TILEPRO_JOFFLONG_X1_PLT`](#r_tilepro_jofflong_x1_plt)
  - [`R_TILEPRO_IMM8_X0`](#r_tilepro_imm8_x0)
  - [`R_TILEPRO_IMM8_Y0`](#r_tilepro_imm8_y0)
  - [`R_TILEPRO_IMM8_X1`](#r_tilepro_imm8_x1)
  - [`R_TILEPRO_IMM8_Y1`](#r_tilepro_imm8_y1)
  - [`R_TILEPRO_MT_IMM15_X1`](#r_tilepro_mt_imm15_x1)
  - [`R_TILEPRO_MF_IMM15_X1`](#r_tilepro_mf_imm15_x1)
  - [`R_TILEPRO_IMM16_X0`](#r_tilepro_imm16_x0)
  - [`R_TILEPRO_IMM16_X1`](#r_tilepro_imm16_x1)
  - [`R_TILEPRO_IMM16_X0_LO`](#r_tilepro_imm16_x0_lo)
  - [`R_TILEPRO_IMM16_X1_LO`](#r_tilepro_imm16_x1_lo)
  - [`R_TILEPRO_IMM16_X0_HI`](#r_tilepro_imm16_x0_hi)
  - [`R_TILEPRO_IMM16_X1_HI`](#r_tilepro_imm16_x1_hi)
  - [`R_TILEPRO_IMM16_X0_HA`](#r_tilepro_imm16_x0_ha)
  - [`R_TILEPRO_IMM16_X1_HA`](#r_tilepro_imm16_x1_ha)
  - [`R_TILEPRO_IMM16_X0_PCREL`](#r_tilepro_imm16_x0_pcrel)
  - [`R_TILEPRO_IMM16_X1_PCREL`](#r_tilepro_imm16_x1_pcrel)
  - [`R_TILEPRO_IMM16_X0_LO_PCREL`](#r_tilepro_imm16_x0_lo_pcrel)
  - [`R_TILEPRO_IMM16_X1_LO_PCREL`](#r_tilepro_imm16_x1_lo_pcrel)
  - [`R_TILEPRO_IMM16_X0_HI_PCREL`](#r_tilepro_imm16_x0_hi_pcrel)
  - [`R_TILEPRO_IMM16_X1_HI_PCREL`](#r_tilepro_imm16_x1_hi_pcrel)
  - [`R_TILEPRO_IMM16_X0_HA_PCREL`](#r_tilepro_imm16_x0_ha_pcrel)
  - [`R_TILEPRO_IMM16_X1_HA_PCREL`](#r_tilepro_imm16_x1_ha_pcrel)
  - [`R_TILEPRO_IMM16_X0_GOT`](#r_tilepro_imm16_x0_got)
  - [`R_TILEPRO_IMM16_X1_GOT`](#r_tilepro_imm16_x1_got)
  - [`R_TILEPRO_IMM16_X0_GOT_LO`](#r_tilepro_imm16_x0_got_lo)
  - [`R_TILEPRO_IMM16_X1_GOT_LO`](#r_tilepro_imm16_x1_got_lo)
  - [`R_TILEPRO_IMM16_X0_GOT_HI`](#r_tilepro_imm16_x0_got_hi)
  - [`R_TILEPRO_IMM16_X1_GOT_HI`](#r_tilepro_imm16_x1_got_hi)
  - [`R_TILEPRO_IMM16_X0_GOT_HA`](#r_tilepro_imm16_x0_got_ha)
  - [`R_TILEPRO_IMM16_X1_GOT_HA`](#r_tilepro_imm16_x1_got_ha)
  - [`R_TILEPRO_MMSTART_X0`](#r_tilepro_mmstart_x0)
  - [`R_TILEPRO_MMEND_X0`](#r_tilepro_mmend_x0)
  - [`R_TILEPRO_MMSTART_X1`](#r_tilepro_mmstart_x1)
  - [`R_TILEPRO_MMEND_X1`](#r_tilepro_mmend_x1)
  - [`R_TILEPRO_SHAMT_X0`](#r_tilepro_shamt_x0)
  - [`R_TILEPRO_SHAMT_X1`](#r_tilepro_shamt_x1)
  - [`R_TILEPRO_SHAMT_Y0`](#r_tilepro_shamt_y0)
  - [`R_TILEPRO_SHAMT_Y1`](#r_tilepro_shamt_y1)
  - [`R_TILEPRO_DEST_IMM8_X1`](#r_tilepro_dest_imm8_x1)
  - [`R_TILEPRO_TLS_GD_CALL`](#r_tilepro_tls_gd_call)
  - [`R_TILEPRO_IMM8_X0_TLS_GD_ADD`](#r_tilepro_imm8_x0_tls_gd_add)
  - [`R_TILEPRO_IMM8_X1_TLS_GD_ADD`](#r_tilepro_imm8_x1_tls_gd_add)
  - [`R_TILEPRO_IMM8_Y0_TLS_GD_ADD`](#r_tilepro_imm8_y0_tls_gd_add)
  - [`R_TILEPRO_IMM8_Y1_TLS_GD_ADD`](#r_tilepro_imm8_y1_tls_gd_add)
  - [`R_TILEPRO_TLS_IE_LOAD`](#r_tilepro_tls_ie_load)
  - [`R_TILEPRO_IMM16_X0_TLS_GD`](#r_tilepro_imm16_x0_tls_gd)
  - [`R_TILEPRO_IMM16_X1_TLS_GD`](#r_tilepro_imm16_x1_tls_gd)
  - [`R_TILEPRO_IMM16_X0_TLS_GD_LO`](#r_tilepro_imm16_x0_tls_gd_lo)
  - [`R_TILEPRO_IMM16_X1_TLS_GD_LO`](#r_tilepro_imm16_x1_tls_gd_lo)
  - [`R_TILEPRO_IMM16_X0_TLS_GD_HI`](#r_tilepro_imm16_x0_tls_gd_hi)
  - [`R_TILEPRO_IMM16_X1_TLS_GD_HI`](#r_tilepro_imm16_x1_tls_gd_hi)
  - [`R_TILEPRO_IMM16_X0_TLS_GD_HA`](#r_tilepro_imm16_x0_tls_gd_ha)
  - [`R_TILEPRO_IMM16_X1_TLS_GD_HA`](#r_tilepro_imm16_x1_tls_gd_ha)
  - [`R_TILEPRO_IMM16_X0_TLS_IE`](#r_tilepro_imm16_x0_tls_ie)
  - [`R_TILEPRO_IMM16_X1_TLS_IE`](#r_tilepro_imm16_x1_tls_ie)
  - [`R_TILEPRO_IMM16_X0_TLS_IE_LO`](#r_tilepro_imm16_x0_tls_ie_lo)
  - [`R_TILEPRO_IMM16_X1_TLS_IE_LO`](#r_tilepro_imm16_x1_tls_ie_lo)
  - [`R_TILEPRO_IMM16_X0_TLS_IE_HI`](#r_tilepro_imm16_x0_tls_ie_hi)
  - [`R_TILEPRO_IMM16_X1_TLS_IE_HI`](#r_tilepro_imm16_x1_tls_ie_hi)
  - [`R_TILEPRO_IMM16_X0_TLS_IE_HA`](#r_tilepro_imm16_x0_tls_ie_ha)
  - [`R_TILEPRO_IMM16_X1_TLS_IE_HA`](#r_tilepro_imm16_x1_tls_ie_ha)
  - [`R_TILEPRO_TLS_DTPMOD32`](#r_tilepro_tls_dtpmod32)
  - [`R_TILEPRO_TLS_DTPOFF32`](#r_tilepro_tls_dtpoff32)
  - [`R_TILEPRO_TLS_TPOFF32`](#r_tilepro_tls_tpoff32)
  - [`R_TILEPRO_IMM16_X0_TLS_LE`](#r_tilepro_imm16_x0_tls_le)
  - [`R_TILEPRO_IMM16_X1_TLS_LE`](#r_tilepro_imm16_x1_tls_le)
  - [`R_TILEPRO_IMM16_X0_TLS_LE_LO`](#r_tilepro_imm16_x0_tls_le_lo)
  - [`R_TILEPRO_IMM16_X1_TLS_LE_LO`](#r_tilepro_imm16_x1_tls_le_lo)
  - [`R_TILEPRO_IMM16_X0_TLS_LE_HI`](#r_tilepro_imm16_x0_tls_le_hi)
  - [`R_TILEPRO_IMM16_X1_TLS_LE_HI`](#r_tilepro_imm16_x1_tls_le_hi)
  - [`R_TILEPRO_IMM16_X0_TLS_LE_HA`](#r_tilepro_imm16_x0_tls_le_ha)
  - [`R_TILEPRO_IMM16_X1_TLS_LE_HA`](#r_tilepro_imm16_x1_tls_le_ha)
  - [`R_TILEPRO_GNU_VTINHERIT`](#r_tilepro_gnu_vtinherit)
  - [`R_TILEPRO_GNU_VTENTRY`](#r_tilepro_gnu_vtentry)
  - [`R_TILEGX_NONE`](#r_tilegx_none)
  - [`R_TILEGX_64`](#r_tilegx_64)
  - [`R_TILEGX_32`](#r_tilegx_32)
  - [`R_TILEGX_16`](#r_tilegx_16)
  - [`R_TILEGX_8`](#r_tilegx_8)
  - [`R_TILEGX_64_PCREL`](#r_tilegx_64_pcrel)
  - [`R_TILEGX_32_PCREL`](#r_tilegx_32_pcrel)
  - [`R_TILEGX_16_PCREL`](#r_tilegx_16_pcrel)
  - [`R_TILEGX_8_PCREL`](#r_tilegx_8_pcrel)
  - [`R_TILEGX_HW0`](#r_tilegx_hw0)
  - [`R_TILEGX_HW1`](#r_tilegx_hw1)
  - [`R_TILEGX_HW2`](#r_tilegx_hw2)
  - [`R_TILEGX_HW3`](#r_tilegx_hw3)
  - [`R_TILEGX_HW0_LAST`](#r_tilegx_hw0_last)
  - [`R_TILEGX_HW1_LAST`](#r_tilegx_hw1_last)
  - [`R_TILEGX_HW2_LAST`](#r_tilegx_hw2_last)
  - [`R_TILEGX_COPY`](#r_tilegx_copy)
  - [`R_TILEGX_GLOB_DAT`](#r_tilegx_glob_dat)
  - [`R_TILEGX_JMP_SLOT`](#r_tilegx_jmp_slot)
  - [`R_TILEGX_RELATIVE`](#r_tilegx_relative)
  - [`R_TILEGX_BROFF_X1`](#r_tilegx_broff_x1)
  - [`R_TILEGX_JUMPOFF_X1`](#r_tilegx_jumpoff_x1)
  - [`R_TILEGX_JUMPOFF_X1_PLT`](#r_tilegx_jumpoff_x1_plt)
  - [`R_TILEGX_IMM8_X0`](#r_tilegx_imm8_x0)
  - [`R_TILEGX_IMM8_Y0`](#r_tilegx_imm8_y0)
  - [`R_TILEGX_IMM8_X1`](#r_tilegx_imm8_x1)
  - [`R_TILEGX_IMM8_Y1`](#r_tilegx_imm8_y1)
  - [`R_TILEGX_DEST_IMM8_X1`](#r_tilegx_dest_imm8_x1)
  - [`R_TILEGX_MT_IMM14_X1`](#r_tilegx_mt_imm14_x1)
  - [`R_TILEGX_MF_IMM14_X1`](#r_tilegx_mf_imm14_x1)
  - [`R_TILEGX_MMSTART_X0`](#r_tilegx_mmstart_x0)
  - [`R_TILEGX_MMEND_X0`](#r_tilegx_mmend_x0)
  - [`R_TILEGX_SHAMT_X0`](#r_tilegx_shamt_x0)
  - [`R_TILEGX_SHAMT_X1`](#r_tilegx_shamt_x1)
  - [`R_TILEGX_SHAMT_Y0`](#r_tilegx_shamt_y0)
  - [`R_TILEGX_SHAMT_Y1`](#r_tilegx_shamt_y1)
  - [`R_TILEGX_IMM16_X0_HW0`](#r_tilegx_imm16_x0_hw0)
  - [`R_TILEGX_IMM16_X1_HW0`](#r_tilegx_imm16_x1_hw0)
  - [`R_TILEGX_IMM16_X0_HW1`](#r_tilegx_imm16_x0_hw1)
  - [`R_TILEGX_IMM16_X1_HW1`](#r_tilegx_imm16_x1_hw1)
  - [`R_TILEGX_IMM16_X0_HW2`](#r_tilegx_imm16_x0_hw2)
  - [`R_TILEGX_IMM16_X1_HW2`](#r_tilegx_imm16_x1_hw2)
  - [`R_TILEGX_IMM16_X0_HW3`](#r_tilegx_imm16_x0_hw3)
  - [`R_TILEGX_IMM16_X1_HW3`](#r_tilegx_imm16_x1_hw3)
  - [`R_TILEGX_IMM16_X0_HW0_LAST`](#r_tilegx_imm16_x0_hw0_last)
  - [`R_TILEGX_IMM16_X1_HW0_LAST`](#r_tilegx_imm16_x1_hw0_last)
  - [`R_TILEGX_IMM16_X0_HW1_LAST`](#r_tilegx_imm16_x0_hw1_last)
  - [`R_TILEGX_IMM16_X1_HW1_LAST`](#r_tilegx_imm16_x1_hw1_last)
  - [`R_TILEGX_IMM16_X0_HW2_LAST`](#r_tilegx_imm16_x0_hw2_last)
  - [`R_TILEGX_IMM16_X1_HW2_LAST`](#r_tilegx_imm16_x1_hw2_last)
  - [`R_TILEGX_IMM16_X0_HW0_PCREL`](#r_tilegx_imm16_x0_hw0_pcrel)
  - [`R_TILEGX_IMM16_X1_HW0_PCREL`](#r_tilegx_imm16_x1_hw0_pcrel)
  - [`R_TILEGX_IMM16_X0_HW1_PCREL`](#r_tilegx_imm16_x0_hw1_pcrel)
  - [`R_TILEGX_IMM16_X1_HW1_PCREL`](#r_tilegx_imm16_x1_hw1_pcrel)
  - [`R_TILEGX_IMM16_X0_HW2_PCREL`](#r_tilegx_imm16_x0_hw2_pcrel)
  - [`R_TILEGX_IMM16_X1_HW2_PCREL`](#r_tilegx_imm16_x1_hw2_pcrel)
  - [`R_TILEGX_IMM16_X0_HW3_PCREL`](#r_tilegx_imm16_x0_hw3_pcrel)
  - [`R_TILEGX_IMM16_X1_HW3_PCREL`](#r_tilegx_imm16_x1_hw3_pcrel)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_PCREL`](#r_tilegx_imm16_x0_hw0_last_pcrel)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_PCREL`](#r_tilegx_imm16_x1_hw0_last_pcrel)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_PCREL`](#r_tilegx_imm16_x0_hw1_last_pcrel)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_PCREL`](#r_tilegx_imm16_x1_hw1_last_pcrel)
  - [`R_TILEGX_IMM16_X0_HW2_LAST_PCREL`](#r_tilegx_imm16_x0_hw2_last_pcrel)
  - [`R_TILEGX_IMM16_X1_HW2_LAST_PCREL`](#r_tilegx_imm16_x1_hw2_last_pcrel)
  - [`R_TILEGX_IMM16_X0_HW0_GOT`](#r_tilegx_imm16_x0_hw0_got)
  - [`R_TILEGX_IMM16_X1_HW0_GOT`](#r_tilegx_imm16_x1_hw0_got)
  - [`R_TILEGX_IMM16_X0_HW0_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW0_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW1_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW1_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW2_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW2_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_GOT`](#r_tilegx_imm16_x0_hw0_last_got)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_GOT`](#r_tilegx_imm16_x1_hw0_last_got)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_GOT`](#r_tilegx_imm16_x0_hw1_last_got)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_GOT`](#r_tilegx_imm16_x1_hw1_last_got)
  - [`R_TILEGX_IMM16_X0_HW3_PLT_PCREL`](#r_tilegx_imm16_x0_hw3_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW3_PLT_PCREL`](#r_tilegx_imm16_x1_hw3_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW0_TLS_GD`](#r_tilegx_imm16_x0_hw0_tls_gd)
  - [`R_TILEGX_IMM16_X1_HW0_TLS_GD`](#r_tilegx_imm16_x1_hw0_tls_gd)
  - [`R_TILEGX_IMM16_X0_HW0_TLS_LE`](#r_tilegx_imm16_x0_hw0_tls_le)
  - [`R_TILEGX_IMM16_X1_HW0_TLS_LE`](#r_tilegx_imm16_x1_hw0_tls_le)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw0_last_tls_le)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw0_last_tls_le)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw1_last_tls_le)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw1_last_tls_le)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw0_last_tls_gd)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw0_last_tls_gd)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw1_last_tls_gd)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw1_last_tls_gd)
  - [`R_TILEGX_IMM16_X0_HW0_TLS_IE`](#r_tilegx_imm16_x0_hw0_tls_ie)
  - [`R_TILEGX_IMM16_X1_HW0_TLS_IE`](#r_tilegx_imm16_x1_hw0_tls_ie)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_last_plt_pcrel)
  - [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw0_last_tls_ie)
  - [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw0_last_tls_ie)
  - [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw1_last_tls_ie)
  - [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw1_last_tls_ie)
  - [`R_TILEGX_TLS_DTPMOD64`](#r_tilegx_tls_dtpmod64)
  - [`R_TILEGX_TLS_DTPOFF64`](#r_tilegx_tls_dtpoff64)
  - [`R_TILEGX_TLS_TPOFF64`](#r_tilegx_tls_tpoff64)
  - [`R_TILEGX_TLS_DTPMOD32`](#r_tilegx_tls_dtpmod32)
  - [`R_TILEGX_TLS_DTPOFF32`](#r_tilegx_tls_dtpoff32)
  - [`R_TILEGX_TLS_TPOFF32`](#r_tilegx_tls_tpoff32)
  - [`R_TILEGX_TLS_GD_CALL`](#r_tilegx_tls_gd_call)
  - [`R_TILEGX_IMM8_X0_TLS_GD_ADD`](#r_tilegx_imm8_x0_tls_gd_add)
  - [`R_TILEGX_IMM8_X1_TLS_GD_ADD`](#r_tilegx_imm8_x1_tls_gd_add)
  - [`R_TILEGX_IMM8_Y0_TLS_GD_ADD`](#r_tilegx_imm8_y0_tls_gd_add)
  - [`R_TILEGX_IMM8_Y1_TLS_GD_ADD`](#r_tilegx_imm8_y1_tls_gd_add)
  - [`R_TILEGX_TLS_IE_LOAD`](#r_tilegx_tls_ie_load)
  - [`R_TILEGX_IMM8_X0_TLS_ADD`](#r_tilegx_imm8_x0_tls_add)
  - [`R_TILEGX_IMM8_X1_TLS_ADD`](#r_tilegx_imm8_x1_tls_add)
  - [`R_TILEGX_IMM8_Y0_TLS_ADD`](#r_tilegx_imm8_y0_tls_add)
  - [`R_TILEGX_IMM8_Y1_TLS_ADD`](#r_tilegx_imm8_y1_tls_add)
  - [`R_TILEGX_GNU_VTINHERIT`](#r_tilegx_gnu_vtinherit)
  - [`R_TILEGX_GNU_VTENTRY`](#r_tilegx_gnu_vtentry)
  - [`EF_RISCV_RVC`](#ef_riscv_rvc)
  - [`EF_RISCV_FLOAT_ABI`](#ef_riscv_float_abi)
  - [`EF_RISCV_FLOAT_ABI_SOFT`](#ef_riscv_float_abi_soft)
  - [`EF_RISCV_FLOAT_ABI_SINGLE`](#ef_riscv_float_abi_single)
  - [`EF_RISCV_FLOAT_ABI_DOUBLE`](#ef_riscv_float_abi_double)
  - [`EF_RISCV_FLOAT_ABI_QUAD`](#ef_riscv_float_abi_quad)
  - [`EF_RISCV_RVE`](#ef_riscv_rve)
  - [`EF_RISCV_TSO`](#ef_riscv_tso)
  - [`EF_RISCV_RV64ILP32`](#ef_riscv_rv64ilp32)
  - [`SHT_RISCV_ATTRIBUTES`](#sht_riscv_attributes)
  - [`R_RISCV_NONE`](#r_riscv_none)
  - [`R_RISCV_32`](#r_riscv_32)
  - [`R_RISCV_64`](#r_riscv_64)
  - [`R_RISCV_RELATIVE`](#r_riscv_relative)
  - [`R_RISCV_COPY`](#r_riscv_copy)
  - [`R_RISCV_JUMP_SLOT`](#r_riscv_jump_slot)
  - [`R_RISCV_TLS_DTPMOD32`](#r_riscv_tls_dtpmod32)
  - [`R_RISCV_TLS_DTPMOD64`](#r_riscv_tls_dtpmod64)
  - [`R_RISCV_TLS_DTPREL32`](#r_riscv_tls_dtprel32)
  - [`R_RISCV_TLS_DTPREL64`](#r_riscv_tls_dtprel64)
  - [`R_RISCV_TLS_TPREL32`](#r_riscv_tls_tprel32)
  - [`R_RISCV_TLS_TPREL64`](#r_riscv_tls_tprel64)
  - [`R_RISCV_TLSDESC`](#r_riscv_tlsdesc)
  - [`R_RISCV_BRANCH`](#r_riscv_branch)
  - [`R_RISCV_JAL`](#r_riscv_jal)
  - [`R_RISCV_CALL`](#r_riscv_call)
  - [`R_RISCV_CALL_PLT`](#r_riscv_call_plt)
  - [`R_RISCV_GOT_HI20`](#r_riscv_got_hi20)
  - [`R_RISCV_TLS_GOT_HI20`](#r_riscv_tls_got_hi20)
  - [`R_RISCV_TLS_GD_HI20`](#r_riscv_tls_gd_hi20)
  - [`R_RISCV_PCREL_HI20`](#r_riscv_pcrel_hi20)
  - [`R_RISCV_PCREL_LO12_I`](#r_riscv_pcrel_lo12_i)
  - [`R_RISCV_PCREL_LO12_S`](#r_riscv_pcrel_lo12_s)
  - [`R_RISCV_HI20`](#r_riscv_hi20)
  - [`R_RISCV_LO12_I`](#r_riscv_lo12_i)
  - [`R_RISCV_LO12_S`](#r_riscv_lo12_s)
  - [`R_RISCV_TPREL_HI20`](#r_riscv_tprel_hi20)
  - [`R_RISCV_TPREL_LO12_I`](#r_riscv_tprel_lo12_i)
  - [`R_RISCV_TPREL_LO12_S`](#r_riscv_tprel_lo12_s)
  - [`R_RISCV_TPREL_ADD`](#r_riscv_tprel_add)
  - [`R_RISCV_ADD8`](#r_riscv_add8)
  - [`R_RISCV_ADD16`](#r_riscv_add16)
  - [`R_RISCV_ADD32`](#r_riscv_add32)
  - [`R_RISCV_ADD64`](#r_riscv_add64)
  - [`R_RISCV_SUB8`](#r_riscv_sub8)
  - [`R_RISCV_SUB16`](#r_riscv_sub16)
  - [`R_RISCV_SUB32`](#r_riscv_sub32)
  - [`R_RISCV_SUB64`](#r_riscv_sub64)
  - [`R_RISCV_GOT32_PCREL`](#r_riscv_got32_pcrel)
  - [`R_RISCV_ALIGN`](#r_riscv_align)
  - [`R_RISCV_RVC_BRANCH`](#r_riscv_rvc_branch)
  - [`R_RISCV_RVC_JUMP`](#r_riscv_rvc_jump)
  - [`R_RISCV_RVC_LUI`](#r_riscv_rvc_lui)
  - [`R_RISCV_GPREL_I`](#r_riscv_gprel_i)
  - [`R_RISCV_GPREL_S`](#r_riscv_gprel_s)
  - [`R_RISCV_TPREL_I`](#r_riscv_tprel_i)
  - [`R_RISCV_TPREL_S`](#r_riscv_tprel_s)
  - [`R_RISCV_RELAX`](#r_riscv_relax)
  - [`R_RISCV_SUB6`](#r_riscv_sub6)
  - [`R_RISCV_SET6`](#r_riscv_set6)
  - [`R_RISCV_SET8`](#r_riscv_set8)
  - [`R_RISCV_SET16`](#r_riscv_set16)
  - [`R_RISCV_SET32`](#r_riscv_set32)
  - [`R_RISCV_32_PCREL`](#r_riscv_32_pcrel)
  - [`R_RISCV_IRELATIVE`](#r_riscv_irelative)
  - [`R_RISCV_PLT32`](#r_riscv_plt32)
  - [`R_RISCV_SET_ULEB128`](#r_riscv_set_uleb128)
  - [`R_RISCV_SUB_ULEB128`](#r_riscv_sub_uleb128)
  - [`R_RISCV_TLSDESC_HI20`](#r_riscv_tlsdesc_hi20)
  - [`R_RISCV_TLSDESC_LOAD_LO12`](#r_riscv_tlsdesc_load_lo12)
  - [`R_RISCV_TLSDESC_ADD_LO12`](#r_riscv_tlsdesc_add_lo12)
  - [`R_RISCV_TLSDESC_CALL`](#r_riscv_tlsdesc_call)
  - [`R_BPF_NONE`](#r_bpf_none)
  - [`R_BPF_64_64`](#r_bpf_64_64)
  - [`R_BPF_64_32`](#r_bpf_64_32)
  - [`R_SBF_NONE`](#r_sbf_none)
  - [`R_SBF_64_64`](#r_sbf_64_64)
  - [`R_SBF_64_32`](#r_sbf_64_32)
  - [`R_METAG_HIADDR16`](#r_metag_hiaddr16)
  - [`R_METAG_LOADDR16`](#r_metag_loaddr16)
  - [`R_METAG_ADDR32`](#r_metag_addr32)
  - [`R_METAG_NONE`](#r_metag_none)
  - [`R_METAG_RELBRANCH`](#r_metag_relbranch)
  - [`R_METAG_GETSETOFF`](#r_metag_getsetoff)
  - [`R_METAG_REG32OP1`](#r_metag_reg32op1)
  - [`R_METAG_REG32OP2`](#r_metag_reg32op2)
  - [`R_METAG_REG32OP3`](#r_metag_reg32op3)
  - [`R_METAG_REG16OP1`](#r_metag_reg16op1)
  - [`R_METAG_REG16OP2`](#r_metag_reg16op2)
  - [`R_METAG_REG16OP3`](#r_metag_reg16op3)
  - [`R_METAG_REG32OP4`](#r_metag_reg32op4)
  - [`R_METAG_HIOG`](#r_metag_hiog)
  - [`R_METAG_LOOG`](#r_metag_loog)
  - [`R_METAG_REL8`](#r_metag_rel8)
  - [`R_METAG_REL16`](#r_metag_rel16)
  - [`R_METAG_GNU_VTINHERIT`](#r_metag_gnu_vtinherit)
  - [`R_METAG_GNU_VTENTRY`](#r_metag_gnu_vtentry)
  - [`R_METAG_HI16_GOTOFF`](#r_metag_hi16_gotoff)
  - [`R_METAG_LO16_GOTOFF`](#r_metag_lo16_gotoff)
  - [`R_METAG_GETSET_GOTOFF`](#r_metag_getset_gotoff)
  - [`R_METAG_GETSET_GOT`](#r_metag_getset_got)
  - [`R_METAG_HI16_GOTPC`](#r_metag_hi16_gotpc)
  - [`R_METAG_LO16_GOTPC`](#r_metag_lo16_gotpc)
  - [`R_METAG_HI16_PLT`](#r_metag_hi16_plt)
  - [`R_METAG_LO16_PLT`](#r_metag_lo16_plt)
  - [`R_METAG_RELBRANCH_PLT`](#r_metag_relbranch_plt)
  - [`R_METAG_GOTOFF`](#r_metag_gotoff)
  - [`R_METAG_PLT`](#r_metag_plt)
  - [`R_METAG_COPY`](#r_metag_copy)
  - [`R_METAG_JMP_SLOT`](#r_metag_jmp_slot)
  - [`R_METAG_RELATIVE`](#r_metag_relative)
  - [`R_METAG_GLOB_DAT`](#r_metag_glob_dat)
  - [`R_METAG_TLS_GD`](#r_metag_tls_gd)
  - [`R_METAG_TLS_LDM`](#r_metag_tls_ldm)
  - [`R_METAG_TLS_LDO_HI16`](#r_metag_tls_ldo_hi16)
  - [`R_METAG_TLS_LDO_LO16`](#r_metag_tls_ldo_lo16)
  - [`R_METAG_TLS_LDO`](#r_metag_tls_ldo)
  - [`R_METAG_TLS_IE`](#r_metag_tls_ie)
  - [`R_METAG_TLS_IENONPIC`](#r_metag_tls_ienonpic)
  - [`R_METAG_TLS_IENONPIC_HI16`](#r_metag_tls_ienonpic_hi16)
  - [`R_METAG_TLS_IENONPIC_LO16`](#r_metag_tls_ienonpic_lo16)
  - [`R_METAG_TLS_TPOFF`](#r_metag_tls_tpoff)
  - [`R_METAG_TLS_DTPMOD`](#r_metag_tls_dtpmod)
  - [`R_METAG_TLS_DTPOFF`](#r_metag_tls_dtpoff)
  - [`R_METAG_TLS_LE`](#r_metag_tls_le)
  - [`R_METAG_TLS_LE_HI16`](#r_metag_tls_le_hi16)
  - [`R_METAG_TLS_LE_LO16`](#r_metag_tls_le_lo16)
  - [`R_NDS32_NONE`](#r_nds32_none)
  - [`R_NDS32_32_RELA`](#r_nds32_32_rela)
  - [`R_NDS32_COPY`](#r_nds32_copy)
  - [`R_NDS32_GLOB_DAT`](#r_nds32_glob_dat)
  - [`R_NDS32_JMP_SLOT`](#r_nds32_jmp_slot)
  - [`R_NDS32_RELATIVE`](#r_nds32_relative)
  - [`R_NDS32_TLS_TPOFF`](#r_nds32_tls_tpoff)
  - [`R_NDS32_TLS_DESC`](#r_nds32_tls_desc)
  - [`EF_LARCH_ABI_MODIFIER_MASK`](#ef_larch_abi_modifier_mask)
  - [`EF_LARCH_ABI_SOFT_FLOAT`](#ef_larch_abi_soft_float)
  - [`EF_LARCH_ABI_SINGLE_FLOAT`](#ef_larch_abi_single_float)
  - [`EF_LARCH_ABI_DOUBLE_FLOAT`](#ef_larch_abi_double_float)
  - [`EF_LARCH_OBJABI_V1`](#ef_larch_objabi_v1)
  - [`R_LARCH_NONE`](#r_larch_none)
  - [`R_LARCH_32`](#r_larch_32)
  - [`R_LARCH_64`](#r_larch_64)
  - [`R_LARCH_RELATIVE`](#r_larch_relative)
  - [`R_LARCH_COPY`](#r_larch_copy)
  - [`R_LARCH_JUMP_SLOT`](#r_larch_jump_slot)
  - [`R_LARCH_TLS_DTPMOD32`](#r_larch_tls_dtpmod32)
  - [`R_LARCH_TLS_DTPMOD64`](#r_larch_tls_dtpmod64)
  - [`R_LARCH_TLS_DTPREL32`](#r_larch_tls_dtprel32)
  - [`R_LARCH_TLS_DTPREL64`](#r_larch_tls_dtprel64)
  - [`R_LARCH_TLS_TPREL32`](#r_larch_tls_tprel32)
  - [`R_LARCH_TLS_TPREL64`](#r_larch_tls_tprel64)
  - [`R_LARCH_IRELATIVE`](#r_larch_irelative)
  - [`R_LARCH_MARK_LA`](#r_larch_mark_la)
  - [`R_LARCH_MARK_PCREL`](#r_larch_mark_pcrel)
  - [`R_LARCH_SOP_PUSH_PCREL`](#r_larch_sop_push_pcrel)
  - [`R_LARCH_SOP_PUSH_ABSOLUTE`](#r_larch_sop_push_absolute)
  - [`R_LARCH_SOP_PUSH_DUP`](#r_larch_sop_push_dup)
  - [`R_LARCH_SOP_PUSH_GPREL`](#r_larch_sop_push_gprel)
  - [`R_LARCH_SOP_PUSH_TLS_TPREL`](#r_larch_sop_push_tls_tprel)
  - [`R_LARCH_SOP_PUSH_TLS_GOT`](#r_larch_sop_push_tls_got)
  - [`R_LARCH_SOP_PUSH_TLS_GD`](#r_larch_sop_push_tls_gd)
  - [`R_LARCH_SOP_PUSH_PLT_PCREL`](#r_larch_sop_push_plt_pcrel)
  - [`R_LARCH_SOP_ASSERT`](#r_larch_sop_assert)
  - [`R_LARCH_SOP_NOT`](#r_larch_sop_not)
  - [`R_LARCH_SOP_SUB`](#r_larch_sop_sub)
  - [`R_LARCH_SOP_SL`](#r_larch_sop_sl)
  - [`R_LARCH_SOP_SR`](#r_larch_sop_sr)
  - [`R_LARCH_SOP_ADD`](#r_larch_sop_add)
  - [`R_LARCH_SOP_AND`](#r_larch_sop_and)
  - [`R_LARCH_SOP_IF_ELSE`](#r_larch_sop_if_else)
  - [`R_LARCH_SOP_POP_32_S_10_5`](#r_larch_sop_pop_32_s_10_5)
  - [`R_LARCH_SOP_POP_32_U_10_12`](#r_larch_sop_pop_32_u_10_12)
  - [`R_LARCH_SOP_POP_32_S_10_12`](#r_larch_sop_pop_32_s_10_12)
  - [`R_LARCH_SOP_POP_32_S_10_16`](#r_larch_sop_pop_32_s_10_16)
  - [`R_LARCH_SOP_POP_32_S_10_16_S2`](#r_larch_sop_pop_32_s_10_16_s2)
  - [`R_LARCH_SOP_POP_32_S_5_20`](#r_larch_sop_pop_32_s_5_20)
  - [`R_LARCH_SOP_POP_32_S_0_5_10_16_S2`](#r_larch_sop_pop_32_s_0_5_10_16_s2)
  - [`R_LARCH_SOP_POP_32_S_0_10_10_16_S2`](#r_larch_sop_pop_32_s_0_10_10_16_s2)
  - [`R_LARCH_SOP_POP_32_U`](#r_larch_sop_pop_32_u)
  - [`R_LARCH_ADD8`](#r_larch_add8)
  - [`R_LARCH_ADD16`](#r_larch_add16)
  - [`R_LARCH_ADD24`](#r_larch_add24)
  - [`R_LARCH_ADD32`](#r_larch_add32)
  - [`R_LARCH_ADD64`](#r_larch_add64)
  - [`R_LARCH_SUB8`](#r_larch_sub8)
  - [`R_LARCH_SUB16`](#r_larch_sub16)
  - [`R_LARCH_SUB24`](#r_larch_sub24)
  - [`R_LARCH_SUB32`](#r_larch_sub32)
  - [`R_LARCH_SUB64`](#r_larch_sub64)
  - [`R_LARCH_GNU_VTINHERIT`](#r_larch_gnu_vtinherit)
  - [`R_LARCH_GNU_VTENTRY`](#r_larch_gnu_vtentry)
  - [`R_LARCH_B16`](#r_larch_b16)
  - [`R_LARCH_B21`](#r_larch_b21)
  - [`R_LARCH_B26`](#r_larch_b26)
  - [`R_LARCH_ABS_HI20`](#r_larch_abs_hi20)
  - [`R_LARCH_ABS_LO12`](#r_larch_abs_lo12)
  - [`R_LARCH_ABS64_LO20`](#r_larch_abs64_lo20)
  - [`R_LARCH_ABS64_HI12`](#r_larch_abs64_hi12)
  - [`R_LARCH_PCALA_HI20`](#r_larch_pcala_hi20)
  - [`R_LARCH_PCALA_LO12`](#r_larch_pcala_lo12)
  - [`R_LARCH_PCALA64_LO20`](#r_larch_pcala64_lo20)
  - [`R_LARCH_PCALA64_HI12`](#r_larch_pcala64_hi12)
  - [`R_LARCH_GOT_PC_HI20`](#r_larch_got_pc_hi20)
  - [`R_LARCH_GOT_PC_LO12`](#r_larch_got_pc_lo12)
  - [`R_LARCH_GOT64_PC_LO20`](#r_larch_got64_pc_lo20)
  - [`R_LARCH_GOT64_PC_HI12`](#r_larch_got64_pc_hi12)
  - [`R_LARCH_GOT_HI20`](#r_larch_got_hi20)
  - [`R_LARCH_GOT_LO12`](#r_larch_got_lo12)
  - [`R_LARCH_GOT64_LO20`](#r_larch_got64_lo20)
  - [`R_LARCH_GOT64_HI12`](#r_larch_got64_hi12)
  - [`R_LARCH_TLS_LE_HI20`](#r_larch_tls_le_hi20)
  - [`R_LARCH_TLS_LE_LO12`](#r_larch_tls_le_lo12)
  - [`R_LARCH_TLS_LE64_LO20`](#r_larch_tls_le64_lo20)
  - [`R_LARCH_TLS_LE64_HI12`](#r_larch_tls_le64_hi12)
  - [`R_LARCH_TLS_IE_PC_HI20`](#r_larch_tls_ie_pc_hi20)
  - [`R_LARCH_TLS_IE_PC_LO12`](#r_larch_tls_ie_pc_lo12)
  - [`R_LARCH_TLS_IE64_PC_LO20`](#r_larch_tls_ie64_pc_lo20)
  - [`R_LARCH_TLS_IE64_PC_HI12`](#r_larch_tls_ie64_pc_hi12)
  - [`R_LARCH_TLS_IE_HI20`](#r_larch_tls_ie_hi20)
  - [`R_LARCH_TLS_IE_LO12`](#r_larch_tls_ie_lo12)
  - [`R_LARCH_TLS_IE64_LO20`](#r_larch_tls_ie64_lo20)
  - [`R_LARCH_TLS_IE64_HI12`](#r_larch_tls_ie64_hi12)
  - [`R_LARCH_TLS_LD_PC_HI20`](#r_larch_tls_ld_pc_hi20)
  - [`R_LARCH_TLS_LD_HI20`](#r_larch_tls_ld_hi20)
  - [`R_LARCH_TLS_GD_PC_HI20`](#r_larch_tls_gd_pc_hi20)
  - [`R_LARCH_TLS_GD_HI20`](#r_larch_tls_gd_hi20)
  - [`R_LARCH_32_PCREL`](#r_larch_32_pcrel)
  - [`R_LARCH_RELAX`](#r_larch_relax)
  - [`R_LARCH_DELETE`](#r_larch_delete)
  - [`R_LARCH_ALIGN`](#r_larch_align)
  - [`R_LARCH_PCREL20_S2`](#r_larch_pcrel20_s2)
  - [`R_LARCH_CFA`](#r_larch_cfa)
  - [`R_LARCH_ADD6`](#r_larch_add6)
  - [`R_LARCH_SUB6`](#r_larch_sub6)
  - [`R_LARCH_ADD_ULEB128`](#r_larch_add_uleb128)
  - [`R_LARCH_SUB_ULEB128`](#r_larch_sub_uleb128)
  - [`R_LARCH_64_PCREL`](#r_larch_64_pcrel)
  - [`R_LARCH_CALL36`](#r_larch_call36)
  - [`R_LARCH_TLS_DESC_PC_HI20`](#r_larch_tls_desc_pc_hi20)
  - [`R_LARCH_TLS_DESC_PC_LO12`](#r_larch_tls_desc_pc_lo12)
  - [`R_LARCH_TLS_DESC64_PC_LO20`](#r_larch_tls_desc64_pc_lo20)
  - [`R_LARCH_TLS_DESC64_PC_HI12`](#r_larch_tls_desc64_pc_hi12)
  - [`R_LARCH_TLS_DESC_HI20`](#r_larch_tls_desc_hi20)
  - [`R_LARCH_TLS_DESC_LO12`](#r_larch_tls_desc_lo12)
  - [`R_LARCH_TLS_DESC64_LO20`](#r_larch_tls_desc64_lo20)
  - [`R_LARCH_TLS_DESC64_HI12`](#r_larch_tls_desc64_hi12)
  - [`R_LARCH_TLS_DESC_LD`](#r_larch_tls_desc_ld)
  - [`R_LARCH_TLS_DESC_CALL`](#r_larch_tls_desc_call)
  - [`R_LARCH_TLS_LE_HI20_R`](#r_larch_tls_le_hi20_r)
  - [`R_LARCH_TLS_LE_ADD_R`](#r_larch_tls_le_add_r)
  - [`R_LARCH_TLS_LE_LO12_R`](#r_larch_tls_le_lo12_r)
  - [`R_LARCH_TLS_LD_PCREL20_S2`](#r_larch_tls_ld_pcrel20_s2)
  - [`R_LARCH_TLS_GD_PCREL20_S2`](#r_larch_tls_gd_pcrel20_s2)
  - [`R_LARCH_TLS_DESC_PCREL20_S2`](#r_larch_tls_desc_pcrel20_s2)
  - [`R_XTENSA_NONE`](#r_xtensa_none)
  - [`R_XTENSA_32`](#r_xtensa_32)
  - [`R_XTENSA_RTLD`](#r_xtensa_rtld)
  - [`R_XTENSA_GLOB_DAT`](#r_xtensa_glob_dat)
  - [`R_XTENSA_JMP_SLOT`](#r_xtensa_jmp_slot)
  - [`R_XTENSA_RELATIVE`](#r_xtensa_relative)
  - [`R_XTENSA_PLT`](#r_xtensa_plt)
  - [`R_XTENSA_OP0`](#r_xtensa_op0)
  - [`R_XTENSA_OP1`](#r_xtensa_op1)
  - [`R_XTENSA_OP2`](#r_xtensa_op2)
  - [`R_XTENSA_ASM_EXPAND`](#r_xtensa_asm_expand)
  - [`R_XTENSA_ASM_SIMPLIFY`](#r_xtensa_asm_simplify)
  - [`R_XTENSA_32_PCREL`](#r_xtensa_32_pcrel)
  - [`R_XTENSA_GNU_VTINHERIT`](#r_xtensa_gnu_vtinherit)
  - [`R_XTENSA_GNU_VTENTRY`](#r_xtensa_gnu_vtentry)
  - [`R_XTENSA_DIFF8`](#r_xtensa_diff8)
  - [`R_XTENSA_DIFF16`](#r_xtensa_diff16)
  - [`R_XTENSA_DIFF32`](#r_xtensa_diff32)
  - [`R_XTENSA_SLOT0_OP`](#r_xtensa_slot0_op)
  - [`R_XTENSA_SLOT1_OP`](#r_xtensa_slot1_op)
  - [`R_XTENSA_SLOT2_OP`](#r_xtensa_slot2_op)
  - [`R_XTENSA_SLOT3_OP`](#r_xtensa_slot3_op)
  - [`R_XTENSA_SLOT4_OP`](#r_xtensa_slot4_op)
  - [`R_XTENSA_SLOT5_OP`](#r_xtensa_slot5_op)
  - [`R_XTENSA_SLOT6_OP`](#r_xtensa_slot6_op)
  - [`R_XTENSA_SLOT7_OP`](#r_xtensa_slot7_op)
  - [`R_XTENSA_SLOT8_OP`](#r_xtensa_slot8_op)
  - [`R_XTENSA_SLOT9_OP`](#r_xtensa_slot9_op)
  - [`R_XTENSA_SLOT10_OP`](#r_xtensa_slot10_op)
  - [`R_XTENSA_SLOT11_OP`](#r_xtensa_slot11_op)
  - [`R_XTENSA_SLOT12_OP`](#r_xtensa_slot12_op)
  - [`R_XTENSA_SLOT13_OP`](#r_xtensa_slot13_op)
  - [`R_XTENSA_SLOT14_OP`](#r_xtensa_slot14_op)
  - [`R_XTENSA_SLOT0_ALT`](#r_xtensa_slot0_alt)
  - [`R_XTENSA_SLOT1_ALT`](#r_xtensa_slot1_alt)
  - [`R_XTENSA_SLOT2_ALT`](#r_xtensa_slot2_alt)
  - [`R_XTENSA_SLOT3_ALT`](#r_xtensa_slot3_alt)
  - [`R_XTENSA_SLOT4_ALT`](#r_xtensa_slot4_alt)
  - [`R_XTENSA_SLOT5_ALT`](#r_xtensa_slot5_alt)
  - [`R_XTENSA_SLOT6_ALT`](#r_xtensa_slot6_alt)
  - [`R_XTENSA_SLOT7_ALT`](#r_xtensa_slot7_alt)
  - [`R_XTENSA_SLOT8_ALT`](#r_xtensa_slot8_alt)
  - [`R_XTENSA_SLOT9_ALT`](#r_xtensa_slot9_alt)
  - [`R_XTENSA_SLOT10_ALT`](#r_xtensa_slot10_alt)
  - [`R_XTENSA_SLOT11_ALT`](#r_xtensa_slot11_alt)
  - [`R_XTENSA_SLOT12_ALT`](#r_xtensa_slot12_alt)
  - [`R_XTENSA_SLOT13_ALT`](#r_xtensa_slot13_alt)
  - [`R_XTENSA_SLOT14_ALT`](#r_xtensa_slot14_alt)
  - [`R_XTENSA_TLSDESC_FN`](#r_xtensa_tlsdesc_fn)
  - [`R_XTENSA_TLSDESC_ARG`](#r_xtensa_tlsdesc_arg)
  - [`R_XTENSA_TLS_DTPOFF`](#r_xtensa_tls_dtpoff)
  - [`R_XTENSA_TLS_TPOFF`](#r_xtensa_tls_tpoff)
  - [`R_XTENSA_TLS_FUNC`](#r_xtensa_tls_func)
  - [`R_XTENSA_TLS_ARG`](#r_xtensa_tls_arg)
  - [`R_XTENSA_TLS_CALL`](#r_xtensa_tls_call)
  - [`R_XTENSA_PDIFF8`](#r_xtensa_pdiff8)
  - [`R_XTENSA_PDIFF16`](#r_xtensa_pdiff16)
  - [`R_XTENSA_PDIFF32`](#r_xtensa_pdiff32)
  - [`R_XTENSA_NDIFF8`](#r_xtensa_ndiff8)
  - [`R_XTENSA_NDIFF16`](#r_xtensa_ndiff16)
  - [`R_XTENSA_NDIFF32`](#r_xtensa_ndiff32)
  - [`EF_E2K_IPD`](#ef_e2k_ipd)
  - [`EF_E2K_X86APP`](#ef_e2k_x86app)
  - [`EF_E2K_4MB_PAGES`](#ef_e2k_4mb_pages)
  - [`EF_E2K_INCOMPAT`](#ef_e2k_incompat)
  - [`EF_E2K_PM`](#ef_e2k_pm)
  - [`EF_E2K_PACK_SEGMENTS`](#ef_e2k_pack_segments)
  - [`E_E2K_MACH_BASE`](#e_e2k_mach_base)
  - [`E_E2K_MACH_EV1`](#e_e2k_mach_ev1)
  - [`E_E2K_MACH_EV2`](#e_e2k_mach_ev2)
  - [`E_E2K_MACH_EV3`](#e_e2k_mach_ev3)
  - [`E_E2K_MACH_EV4`](#e_e2k_mach_ev4)
  - [`E_E2K_MACH_EV5`](#e_e2k_mach_ev5)
  - [`E_E2K_MACH_EV6`](#e_e2k_mach_ev6)
  - [`E_E2K_MACH_EV7`](#e_e2k_mach_ev7)
  - [`E_E2K_MACH_8C`](#e_e2k_mach_8c)
  - [`E_E2K_MACH_1CPLUS`](#e_e2k_mach_1cplus)
  - [`E_E2K_MACH_12C`](#e_e2k_mach_12c)
  - [`E_E2K_MACH_16C`](#e_e2k_mach_16c)
  - [`E_E2K_MACH_2C3`](#e_e2k_mach_2c3)
  - [`E_E2K_MACH_48C`](#e_e2k_mach_48c)
  - [`E_E2K_MACH_8V7`](#e_e2k_mach_8v7)
  - [`R_E2K_32_ABS`](#r_e2k_32_abs)
  - [`R_E2K_32_PC`](#r_e2k_32_pc)
  - [`R_E2K_AP_GOT`](#r_e2k_ap_got)
  - [`R_E2K_PL_GOT`](#r_e2k_pl_got)
  - [`R_E2K_32_JMP_SLOT`](#r_e2k_32_jmp_slot)
  - [`R_E2K_32_COPY`](#r_e2k_32_copy)
  - [`R_E2K_32_RELATIVE`](#r_e2k_32_relative)
  - [`R_E2K_32_IRELATIVE`](#r_e2k_32_irelative)
  - [`R_E2K_32_SIZE`](#r_e2k_32_size)
  - [`R_E2K_32_DYNOPT`](#r_e2k_32_dynopt)
  - [`R_E2K_64_ABS`](#r_e2k_64_abs)
  - [`R_E2K_64_ABS_LIT`](#r_e2k_64_abs_lit)
  - [`R_E2K_64_PC_LIT`](#r_e2k_64_pc_lit)
  - [`R_E2K_64_JMP_SLOT`](#r_e2k_64_jmp_slot)
  - [`R_E2K_64_COPY`](#r_e2k_64_copy)
  - [`R_E2K_64_RELATIVE`](#r_e2k_64_relative)
  - [`R_E2K_64_RELATIVE_LIT`](#r_e2k_64_relative_lit)
  - [`R_E2K_64_IRELATIVE`](#r_e2k_64_irelative)
  - [`R_E2K_64_SIZE`](#r_e2k_64_size)
  - [`R_E2K_64_GOTOFF`](#r_e2k_64_gotoff)
  - [`R_E2K_TLS_GDMOD`](#r_e2k_tls_gdmod)
  - [`R_E2K_TLS_GDREL`](#r_e2k_tls_gdrel)
  - [`R_E2K_TLS_IE`](#r_e2k_tls_ie)
  - [`R_E2K_32_TLS_LE`](#r_e2k_32_tls_le)
  - [`R_E2K_64_TLS_LE`](#r_e2k_64_tls_le)
  - [`R_E2K_TLS_32_DTPMOD`](#r_e2k_tls_32_dtpmod)
  - [`R_E2K_TLS_32_DTPREL`](#r_e2k_tls_32_dtprel)
  - [`R_E2K_TLS_64_DTPMOD`](#r_e2k_tls_64_dtpmod)
  - [`R_E2K_TLS_64_DTPREL`](#r_e2k_tls_64_dtprel)
  - [`R_E2K_TLS_32_TPREL`](#r_e2k_tls_32_tprel)
  - [`R_E2K_TLS_64_TPREL`](#r_e2k_tls_64_tprel)
  - [`R_E2K_AP`](#r_e2k_ap)
  - [`R_E2K_PL`](#r_e2k_pl)
  - [`R_E2K_GOT`](#r_e2k_got)
  - [`R_E2K_GOTOFF`](#r_e2k_gotoff)
  - [`R_E2K_DISP`](#r_e2k_disp)
  - [`R_E2K_PREF`](#r_e2k_pref)
  - [`R_E2K_NONE`](#r_e2k_none)
  - [`R_E2K_GOTPLT`](#r_e2k_gotplt)
  - [`R_E2K_ISLOCAL`](#r_e2k_islocal)
  - [`R_E2K_ISLOCAL32`](#r_e2k_islocal32)
  - [`R_E2K_64_GOTOFF_LIT`](#r_e2k_64_gotoff_lit)
  - [`R_E2K_64_DYNOPT`](#r_e2k_64_dynopt)
  - [`R_E2K_64_PC`](#r_e2k_64_pc)
  - [`DT_E2K_LAZY`](#dt_e2k_lazy)
  - [`DT_E2K_LAZY_GOT`](#dt_e2k_lazy_got)
  - [`DT_E2K_INIT_GOT`](#dt_e2k_init_got)
  - [`DT_E2K_EXPORT_PL`](#dt_e2k_export_pl)
  - [`DT_E2K_EXPORT_PLSZ`](#dt_e2k_export_plsz)
  - [`DT_E2K_REAL_PLTGOT`](#dt_e2k_real_pltgot)
  - [`DT_E2K_NO_SELFINIT`](#dt_e2k_no_selfinit)
  - [`DT_E2K_NUM`](#dt_e2k_num)
  - [`Tag_File`](#tag_file)
  - [`Tag_Section`](#tag_section)
  - [`Tag_Symbol`](#tag_symbol)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FileHeader32`](#fileheader32) | struct | The header at the start of every 32-bit ELF file. |
| [`FileHeader64`](#fileheader64) | struct | The header at the start of every 64-bit ELF file. |
| [`Ident`](#ident) | struct | Magic number and other information. |
| [`SectionHeader32`](#sectionheader32) | struct | Section header. |
| [`SectionHeader64`](#sectionheader64) | struct | Section header. |
| [`CompressionHeader32`](#compressionheader32) | struct | Section compression header. |
| [`CompressionHeader64`](#compressionheader64) | struct | Section compression header. |
| [`Sym32`](#sym32) | struct | Symbol table entry. |
| [`Sym64`](#sym64) | struct | Symbol table entry. |
| [`Syminfo32`](#syminfo32) | struct | Additional information about a `Sym32`. |
| [`Syminfo64`](#syminfo64) | struct | Additional information about a `Sym64`. |
| [`Rel32`](#rel32) | struct | Relocation table entry without explicit addend. |
| [`Rela32`](#rela32) | struct | Relocation table entry with explicit addend. |
| [`Rel64`](#rel64) | struct | Relocation table entry without explicit addend. |
| [`Rela64`](#rela64) | struct | Relocation table entry with explicit addend. |
| [`Relr32`](#relr32) | struct | 32-bit relative relocation table entry. |
| [`Relr64`](#relr64) | struct | 64-bit relative relocation table entry. |
| [`ProgramHeader32`](#programheader32) | struct | Program segment header. |
| [`ProgramHeader64`](#programheader64) | struct | Program segment header. |
| [`Dyn32`](#dyn32) | struct | Dynamic section entry. |
| [`Dyn64`](#dyn64) | struct | Dynamic section entry. |
| [`Versym`](#versym) | struct | Version symbol information |
| [`Verdef`](#verdef) | struct | Version definition sections |
| [`Verdaux`](#verdaux) | struct | Auxiliary version information. |
| [`Verneed`](#verneed) | struct | Version dependency. |
| [`Vernaux`](#vernaux) | struct | Auxiliary needed version information. |
| [`NoteHeader32`](#noteheader32) | struct | Note section entry header. |
| [`NoteHeader64`](#noteheader64) | struct | Note section entry header. |
| [`HashHeader`](#hashheader) | struct | Header of `SHT_HASH` section. |
| [`GnuHashHeader`](#gnuhashheader) | struct | Header of `SHT_GNU_HASH` section. |
| [`hash`](#hash) | fn | Calculate the SysV hash for a symbol name. |
| [`gnu_hash`](#gnu_hash) | fn | Calculate the GNU hash for a symbol name. |
| [`ef_e2k_mach_to_flag`](#ef_e2k_mach_to_flag) | fn | Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`. |
| [`ef_e2k_flag_to_mach`](#ef_e2k_flag_to_mach) | fn | Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`. |
| [`ELFMAG`](#elfmag) | const | File identification bytes stored in `Ident::magic`. |
| [`ELFCLASSNONE`](#elfclassnone) | const | Invalid class. |
| [`ELFCLASS32`](#elfclass32) | const | 32-bit object. |
| [`ELFCLASS64`](#elfclass64) | const | 64-bit object. |
| [`ELFDATANONE`](#elfdatanone) | const | Invalid data encoding. |
| [`ELFDATA2LSB`](#elfdata2lsb) | const | 2's complement, little endian. |
| [`ELFDATA2MSB`](#elfdata2msb) | const | 2's complement, big endian. |
| [`ELFOSABI_NONE`](#elfosabi_none) | const | UNIX System V ABI. |
| [`ELFOSABI_SYSV`](#elfosabi_sysv) | const | UNIX System V ABI. |
| [`ELFOSABI_HPUX`](#elfosabi_hpux) | const | HP-UX. |
| [`ELFOSABI_NETBSD`](#elfosabi_netbsd) | const | NetBSD. |
| [`ELFOSABI_GNU`](#elfosabi_gnu) | const | Object uses GNU ELF extensions. |
| [`ELFOSABI_LINUX`](#elfosabi_linux) | const | Object uses GNU ELF extensions. |
| [`ELFOSABI_HURD`](#elfosabi_hurd) | const | GNU/Hurd. |
| [`ELFOSABI_SOLARIS`](#elfosabi_solaris) | const | Sun Solaris. |
| [`ELFOSABI_AIX`](#elfosabi_aix) | const | IBM AIX. |
| [`ELFOSABI_IRIX`](#elfosabi_irix) | const | SGI Irix. |
| [`ELFOSABI_FREEBSD`](#elfosabi_freebsd) | const | FreeBSD. |
| [`ELFOSABI_TRU64`](#elfosabi_tru64) | const | Compaq TRU64 UNIX. |
| [`ELFOSABI_MODESTO`](#elfosabi_modesto) | const | Novell Modesto. |
| [`ELFOSABI_OPENBSD`](#elfosabi_openbsd) | const | OpenBSD. |
| [`ELFOSABI_OPENVMS`](#elfosabi_openvms) | const | OpenVMS. |
| [`ELFOSABI_NSK`](#elfosabi_nsk) | const | Hewlett-Packard Non-Stop Kernel. |
| [`ELFOSABI_AROS`](#elfosabi_aros) | const | AROS |
| [`ELFOSABI_FENIXOS`](#elfosabi_fenixos) | const | FenixOS |
| [`ELFOSABI_CLOUDABI`](#elfosabi_cloudabi) | const | Nuxi CloudABI |
| [`ELFOSABI_ARM_AEABI`](#elfosabi_arm_aeabi) | const | ARM EABI. |
| [`ELFOSABI_ARM`](#elfosabi_arm) | const | ARM. |
| [`ELFOSABI_STANDALONE`](#elfosabi_standalone) | const | Standalone (embedded) application. |
| [`ET_NONE`](#et_none) | const | No file type. |
| [`ET_REL`](#et_rel) | const | Relocatable file. |
| [`ET_EXEC`](#et_exec) | const | Executable file. |
| [`ET_DYN`](#et_dyn) | const | Shared object file. |
| [`ET_CORE`](#et_core) | const | Core file. |
| [`ET_LOOS`](#et_loos) | const | OS-specific range start. |
| [`ET_HIOS`](#et_hios) | const | OS-specific range end. |
| [`ET_LOPROC`](#et_loproc) | const | Processor-specific range start. |
| [`ET_HIPROC`](#et_hiproc) | const | Processor-specific range end. |
| [`EM_NONE`](#em_none) | const | No machine |
| [`EM_M32`](#em_m32) | const | AT&T WE 32100 |
| [`EM_SPARC`](#em_sparc) | const | SUN SPARC |
| [`EM_386`](#em_386) | const | Intel 80386 |
| [`EM_68K`](#em_68k) | const | Motorola m68k family |
| [`EM_88K`](#em_88k) | const | Motorola m88k family |
| [`EM_IAMCU`](#em_iamcu) | const | Intel MCU |
| [`EM_860`](#em_860) | const | Intel 80860 |
| [`EM_MIPS`](#em_mips) | const | MIPS R3000 big-endian |
| [`EM_S370`](#em_s370) | const | IBM System/370 |
| [`EM_MIPS_RS3_LE`](#em_mips_rs3_le) | const | MIPS R3000 little-endian |
| [`EM_PARISC`](#em_parisc) | const | HPPA |
| [`EM_VPP500`](#em_vpp500) | const | Fujitsu VPP500 |
| [`EM_SPARC32PLUS`](#em_sparc32plus) | const | Sun's "v8plus" |
| [`EM_960`](#em_960) | const | Intel 80960 |
| [`EM_PPC`](#em_ppc) | const | PowerPC |
| [`EM_PPC64`](#em_ppc64) | const | PowerPC 64-bit |
| [`EM_S390`](#em_s390) | const | IBM S390 |
| [`EM_SPU`](#em_spu) | const | IBM SPU/SPC |
| [`EM_V800`](#em_v800) | const | NEC V800 series |
| [`EM_FR20`](#em_fr20) | const | Fujitsu FR20 |
| [`EM_RH32`](#em_rh32) | const | TRW RH-32 |
| [`EM_RCE`](#em_rce) | const | Motorola RCE |
| [`EM_ARM`](#em_arm) | const | ARM |
| [`EM_FAKE_ALPHA`](#em_fake_alpha) | const | Digital Alpha |
| [`EM_SH`](#em_sh) | const | Hitachi SH |
| [`EM_SPARCV9`](#em_sparcv9) | const | SPARC v9 64-bit |
| [`EM_TRICORE`](#em_tricore) | const | Siemens Tricore |
| [`EM_ARC`](#em_arc) | const | Argonaut RISC Core |
| [`EM_H8_300`](#em_h8_300) | const | Hitachi H8/300 |
| [`EM_H8_300H`](#em_h8_300h) | const | Hitachi H8/300H |
| [`EM_H8S`](#em_h8s) | const | Hitachi H8S |
| [`EM_H8_500`](#em_h8_500) | const | Hitachi H8/500 |
| [`EM_IA_64`](#em_ia_64) | const | Intel Merced |
| [`EM_MIPS_X`](#em_mips_x) | const | Stanford MIPS-X |
| [`EM_COLDFIRE`](#em_coldfire) | const | Motorola Coldfire |
| [`EM_68HC12`](#em_68hc12) | const | Motorola M68HC12 |
| [`EM_MMA`](#em_mma) | const | Fujitsu MMA Multimedia Accelerator |
| [`EM_PCP`](#em_pcp) | const | Siemens PCP |
| [`EM_NCPU`](#em_ncpu) | const | Sony nCPU embeeded RISC |
| [`EM_NDR1`](#em_ndr1) | const | Denso NDR1 microprocessor |
| [`EM_STARCORE`](#em_starcore) | const | Motorola Start*Core processor |
| [`EM_ME16`](#em_me16) | const | Toyota ME16 processor |
| [`EM_ST100`](#em_st100) | const | STMicroelectronic ST100 processor |
| [`EM_TINYJ`](#em_tinyj) | const | Advanced Logic Corp. Tinyj emb.fam |
| [`EM_X86_64`](#em_x86_64) | const | AMD x86-64 architecture |
| [`EM_PDSP`](#em_pdsp) | const | Sony DSP Processor |
| [`EM_PDP10`](#em_pdp10) | const | Digital PDP-10 |
| [`EM_PDP11`](#em_pdp11) | const | Digital PDP-11 |
| [`EM_FX66`](#em_fx66) | const | Siemens FX66 microcontroller |
| [`EM_ST9PLUS`](#em_st9plus) | const | STMicroelectronics ST9+ 8/16 mc |
| [`EM_ST7`](#em_st7) | const | STmicroelectronics ST7 8 bit mc |
| [`EM_68HC16`](#em_68hc16) | const | Motorola MC68HC16 microcontroller |
| [`EM_68HC11`](#em_68hc11) | const | Motorola MC68HC11 microcontroller |
| [`EM_68HC08`](#em_68hc08) | const | Motorola MC68HC08 microcontroller |
| [`EM_68HC05`](#em_68hc05) | const | Motorola MC68HC05 microcontroller |
| [`EM_SVX`](#em_svx) | const | Silicon Graphics SVx |
| [`EM_ST19`](#em_st19) | const | STMicroelectronics ST19 8 bit mc |
| [`EM_VAX`](#em_vax) | const | Digital VAX |
| [`EM_CRIS`](#em_cris) | const | Axis Communications 32-bit emb.proc |
| [`EM_JAVELIN`](#em_javelin) | const | Infineon Technologies 32-bit emb.proc |
| [`EM_FIREPATH`](#em_firepath) | const | Element 14 64-bit DSP Processor |
| [`EM_ZSP`](#em_zsp) | const | LSI Logic 16-bit DSP Processor |
| [`EM_MMIX`](#em_mmix) | const | Donald Knuth's educational 64-bit proc |
| [`EM_HUANY`](#em_huany) | const | Harvard University machine-independent object files |
| [`EM_PRISM`](#em_prism) | const | SiTera Prism |
| [`EM_AVR`](#em_avr) | const | Atmel AVR 8-bit microcontroller |
| [`EM_FR30`](#em_fr30) | const | Fujitsu FR30 |
| [`EM_D10V`](#em_d10v) | const | Mitsubishi D10V |
| [`EM_D30V`](#em_d30v) | const | Mitsubishi D30V |
| [`EM_V850`](#em_v850) | const | NEC v850 |
| [`EM_M32R`](#em_m32r) | const | Mitsubishi M32R |
| [`EM_MN10300`](#em_mn10300) | const | Matsushita MN10300 |
| [`EM_MN10200`](#em_mn10200) | const | Matsushita MN10200 |
| [`EM_PJ`](#em_pj) | const | picoJava |
| [`EM_OPENRISC`](#em_openrisc) | const | OpenRISC 32-bit embedded processor |
| [`EM_ARC_COMPACT`](#em_arc_compact) | const | ARC International ARCompact |
| [`EM_XTENSA`](#em_xtensa) | const | Tensilica Xtensa Architecture |
| [`EM_VIDEOCORE`](#em_videocore) | const | Alphamosaic VideoCore |
| [`EM_TMM_GPP`](#em_tmm_gpp) | const | Thompson Multimedia General Purpose Proc |
| [`EM_NS32K`](#em_ns32k) | const | National Semi. |
| [`EM_TPC`](#em_tpc) | const | Tenor Network TPC |
| [`EM_SNP1K`](#em_snp1k) | const | Trebia SNP 1000 |
| [`EM_ST200`](#em_st200) | const | STMicroelectronics ST200 |
| [`EM_IP2K`](#em_ip2k) | const | Ubicom IP2xxx |
| [`EM_MAX`](#em_max) | const | MAX processor |
| [`EM_CR`](#em_cr) | const | National Semi. |
| [`EM_F2MC16`](#em_f2mc16) | const | Fujitsu F2MC16 |
| [`EM_MSP430`](#em_msp430) | const | Texas Instruments msp430 |
| [`EM_BLACKFIN`](#em_blackfin) | const | Analog Devices Blackfin DSP |
| [`EM_SE_C33`](#em_se_c33) | const | Seiko Epson S1C33 family |
| [`EM_SEP`](#em_sep) | const | Sharp embedded microprocessor |
| [`EM_ARCA`](#em_arca) | const | Arca RISC |
| [`EM_UNICORE`](#em_unicore) | const | PKU-Unity & MPRC Peking Uni. |
| [`EM_EXCESS`](#em_excess) | const | eXcess configurable cpu |
| [`EM_DXP`](#em_dxp) | const | Icera Semi. |
| [`EM_ALTERA_NIOS2`](#em_altera_nios2) | const | Altera Nios II |
| [`EM_CRX`](#em_crx) | const | National Semi. |
| [`EM_XGATE`](#em_xgate) | const | Motorola XGATE |
| [`EM_C166`](#em_c166) | const | Infineon C16x/XC16x |
| [`EM_M16C`](#em_m16c) | const | Renesas M16C |
| [`EM_DSPIC30F`](#em_dspic30f) | const | Microchip Technology dsPIC30F |
| [`EM_CE`](#em_ce) | const | Freescale Communication Engine RISC |
| [`EM_M32C`](#em_m32c) | const | Renesas M32C |
| [`EM_TSK3000`](#em_tsk3000) | const | Altium TSK3000 |
| [`EM_RS08`](#em_rs08) | const | Freescale RS08 |
| [`EM_SHARC`](#em_sharc) | const | Analog Devices SHARC family |
| [`EM_ECOG2`](#em_ecog2) | const | Cyan Technology eCOG2 |
| [`EM_SCORE7`](#em_score7) | const | Sunplus S+core7 RISC |
| [`EM_DSP24`](#em_dsp24) | const | New Japan Radio (NJR) 24-bit DSP |
| [`EM_VIDEOCORE3`](#em_videocore3) | const | Broadcom VideoCore III |
| [`EM_LATTICEMICO32`](#em_latticemico32) | const | RISC for Lattice FPGA |
| [`EM_SE_C17`](#em_se_c17) | const | Seiko Epson C17 |
| [`EM_TI_C6000`](#em_ti_c6000) | const | Texas Instruments TMS320C6000 DSP |
| [`EM_TI_C2000`](#em_ti_c2000) | const | Texas Instruments TMS320C2000 DSP |
| [`EM_TI_C5500`](#em_ti_c5500) | const | Texas Instruments TMS320C55x DSP |
| [`EM_TI_ARP32`](#em_ti_arp32) | const | Texas Instruments App. |
| [`EM_TI_PRU`](#em_ti_pru) | const | Texas Instruments Prog. |
| [`EM_MMDSP_PLUS`](#em_mmdsp_plus) | const | STMicroelectronics 64bit VLIW DSP |
| [`EM_CYPRESS_M8C`](#em_cypress_m8c) | const | Cypress M8C |
| [`EM_R32C`](#em_r32c) | const | Renesas R32C |
| [`EM_TRIMEDIA`](#em_trimedia) | const | NXP Semi. |
| [`EM_HEXAGON`](#em_hexagon) | const | QUALCOMM Hexagon |
| [`EM_8051`](#em_8051) | const | Intel 8051 and variants |
| [`EM_STXP7X`](#em_stxp7x) | const | STMicroelectronics STxP7x |
| [`EM_NDS32`](#em_nds32) | const | Andes Tech. |
| [`EM_ECOG1X`](#em_ecog1x) | const | Cyan Technology eCOG1X |
| [`EM_MAXQ30`](#em_maxq30) | const | Dallas Semi. |
| [`EM_XIMO16`](#em_ximo16) | const | New Japan Radio (NJR) 16-bit DSP |
| [`EM_MANIK`](#em_manik) | const | M2000 Reconfigurable RISC |
| [`EM_CRAYNV2`](#em_craynv2) | const | Cray NV2 vector architecture |
| [`EM_RX`](#em_rx) | const | Renesas RX |
| [`EM_METAG`](#em_metag) | const | Imagination Tech. |
| [`EM_MCST_ELBRUS`](#em_mcst_elbrus) | const | MCST Elbrus |
| [`EM_ECOG16`](#em_ecog16) | const | Cyan Technology eCOG16 |
| [`EM_CR16`](#em_cr16) | const | National Semi. |
| [`EM_ETPU`](#em_etpu) | const | Freescale Extended Time Processing Unit |
| [`EM_SLE9X`](#em_sle9x) | const | Infineon Tech. |
| [`EM_L10M`](#em_l10m) | const | Intel L10M |
| [`EM_K10M`](#em_k10m) | const | Intel K10M |
| [`EM_AARCH64`](#em_aarch64) | const | ARM AARCH64 |
| [`EM_AVR32`](#em_avr32) | const | Amtel 32-bit microprocessor |
| [`EM_STM8`](#em_stm8) | const | STMicroelectronics STM8 |
| [`EM_TILE64`](#em_tile64) | const | Tileta TILE64 |
| [`EM_TILEPRO`](#em_tilepro) | const | Tilera TILEPro |
| [`EM_MICROBLAZE`](#em_microblaze) | const | Xilinx MicroBlaze |
| [`EM_CUDA`](#em_cuda) | const | NVIDIA CUDA |
| [`EM_TILEGX`](#em_tilegx) | const | Tilera TILE-Gx |
| [`EM_CLOUDSHIELD`](#em_cloudshield) | const | CloudShield |
| [`EM_COREA_1ST`](#em_corea_1st) | const | KIPO-KAIST Core-A 1st gen. |
| [`EM_COREA_2ND`](#em_corea_2nd) | const | KIPO-KAIST Core-A 2nd gen. |
| [`EM_ARC_COMPACT2`](#em_arc_compact2) | const | Synopsys ARCompact V2 |
| [`EM_OPEN8`](#em_open8) | const | Open8 RISC |
| [`EM_RL78`](#em_rl78) | const | Renesas RL78 |
| [`EM_VIDEOCORE5`](#em_videocore5) | const | Broadcom VideoCore V |
| [`EM_78KOR`](#em_78kor) | const | Renesas 78KOR |
| [`EM_56800EX`](#em_56800ex) | const | Freescale 56800EX DSC |
| [`EM_BA1`](#em_ba1) | const | Beyond BA1 |
| [`EM_BA2`](#em_ba2) | const | Beyond BA2 |
| [`EM_XCORE`](#em_xcore) | const | XMOS xCORE |
| [`EM_MCHP_PIC`](#em_mchp_pic) | const | Microchip 8-bit PIC(r) |
| [`EM_KM32`](#em_km32) | const | KM211 KM32 |
| [`EM_KMX32`](#em_kmx32) | const | KM211 KMX32 |
| [`EM_EMX16`](#em_emx16) | const | KM211 KMX16 |
| [`EM_EMX8`](#em_emx8) | const | KM211 KMX8 |
| [`EM_KVARC`](#em_kvarc) | const | KM211 KVARC |
| [`EM_CDP`](#em_cdp) | const | Paneve CDP |
| [`EM_COGE`](#em_coge) | const | Cognitive Smart Memory Processor |
| [`EM_COOL`](#em_cool) | const | Bluechip CoolEngine |
| [`EM_NORC`](#em_norc) | const | Nanoradio Optimized RISC |
| [`EM_CSR_KALIMBA`](#em_csr_kalimba) | const | CSR Kalimba |
| [`EM_Z80`](#em_z80) | const | Zilog Z80 |
| [`EM_VISIUM`](#em_visium) | const | Controls and Data Services VISIUMcore |
| [`EM_FT32`](#em_ft32) | const | FTDI Chip FT32 |
| [`EM_MOXIE`](#em_moxie) | const | Moxie processor |
| [`EM_AMDGPU`](#em_amdgpu) | const | AMD GPU |
| [`EM_RISCV`](#em_riscv) | const | RISC-V |
| [`EM_BPF`](#em_bpf) | const | Linux BPF -- in-kernel virtual machine |
| [`EM_CSKY`](#em_csky) | const | C-SKY |
| [`EM_LOONGARCH`](#em_loongarch) | const | Loongson LoongArch |
| [`EM_SBF`](#em_sbf) | const | Solana Binary Format |
| [`EM_ALPHA`](#em_alpha) | const | Digital Alpha |
| [`EV_NONE`](#ev_none) | const | Invalid ELF version. |
| [`EV_CURRENT`](#ev_current) | const | Current ELF version. |
| [`SHN_UNDEF`](#shn_undef) | const | Undefined section. |
| [`SHN_LORESERVE`](#shn_loreserve) | const | OS-specific range start. |
| [`SHN_LOPROC`](#shn_loproc) | const | Start of processor-specific section indices. |
| [`SHN_HIPROC`](#shn_hiproc) | const | End of processor-specific section indices. |
| [`SHN_LOOS`](#shn_loos) | const | Start of OS-specific section indices. |
| [`SHN_HIOS`](#shn_hios) | const | End of OS-specific section indices. |
| [`SHN_ABS`](#shn_abs) | const | Associated symbol is absolute. |
| [`SHN_COMMON`](#shn_common) | const | Associated symbol is common. |
| [`SHN_XINDEX`](#shn_xindex) | const | Section index is in the `SHT_SYMTAB_SHNDX` section. |
| [`SHN_HIRESERVE`](#shn_hireserve) | const | End of reserved section indices. |
| [`SHT_NULL`](#sht_null) | const | Section header table entry is unused. |
| [`SHT_PROGBITS`](#sht_progbits) | const | Program data. |
| [`SHT_SYMTAB`](#sht_symtab) | const | Symbol table. |
| [`SHT_STRTAB`](#sht_strtab) | const | String table. |
| [`SHT_RELA`](#sht_rela) | const | Relocation entries with explicit addends. |
| [`SHT_HASH`](#sht_hash) | const | Symbol hash table. |
| [`SHT_DYNAMIC`](#sht_dynamic) | const | Dynamic linking information. |
| [`SHT_NOTE`](#sht_note) | const | Notes. |
| [`SHT_NOBITS`](#sht_nobits) | const | Program space with no data (bss). |
| [`SHT_REL`](#sht_rel) | const | Relocation entries without explicit addends. |
| [`SHT_SHLIB`](#sht_shlib) | const | Reserved section type. |
| [`SHT_DYNSYM`](#sht_dynsym) | const | Dynamic linker symbol table. |
| [`SHT_INIT_ARRAY`](#sht_init_array) | const | Array of constructors. |
| [`SHT_FINI_ARRAY`](#sht_fini_array) | const | Array of destructors. |
| [`SHT_PREINIT_ARRAY`](#sht_preinit_array) | const | Array of pre-constructors. |
| [`SHT_GROUP`](#sht_group) | const | Section group. |
| [`SHT_SYMTAB_SHNDX`](#sht_symtab_shndx) | const | Extended section indices for a symbol table. |
| [`SHT_RELR`](#sht_relr) | const | Relocation entries; only offsets. |
| [`SHT_CREL`](#sht_crel) | const | Experimental CREL relocations. |
| [`SHT_LOOS`](#sht_loos) | const | Start of OS-specific section types. |
| [`SHT_LLVM_DEPENDENT_LIBRARIES`](#sht_llvm_dependent_libraries) | const | LLVM-style dependent libraries. |
| [`SHT_GNU_SFRAME`](#sht_gnu_sframe) | const | GNU SFrame stack trace format. |
| [`SHT_GNU_ATTRIBUTES`](#sht_gnu_attributes) | const | Object attributes. |
| [`SHT_GNU_HASH`](#sht_gnu_hash) | const | GNU-style hash table. |
| [`SHT_GNU_LIBLIST`](#sht_gnu_liblist) | const | Prelink library list |
| [`SHT_CHECKSUM`](#sht_checksum) | const | Checksum for DSO content. |
| [`SHT_LOSUNW`](#sht_losunw) | const | Sun-specific low bound. |
| [`SHT_SUNW_move`](#sht_sunw_move) | const |  |
| [`SHT_SUNW_COMDAT`](#sht_sunw_comdat) | const |  |
| [`SHT_SUNW_syminfo`](#sht_sunw_syminfo) | const |  |
| [`SHT_GNU_VERDEF`](#sht_gnu_verdef) | const | Version definition section. |
| [`SHT_GNU_VERNEED`](#sht_gnu_verneed) | const | Version needs section. |
| [`SHT_GNU_VERSYM`](#sht_gnu_versym) | const | Version symbol table. |
| [`SHT_HISUNW`](#sht_hisunw) | const | Sun-specific high bound. |
| [`SHT_HIOS`](#sht_hios) | const | End of OS-specific section types. |
| [`SHT_LOPROC`](#sht_loproc) | const | Start of processor-specific section types. |
| [`SHT_HIPROC`](#sht_hiproc) | const | End of processor-specific section types. |
| [`SHT_LOUSER`](#sht_louser) | const | Start of application-specific section types. |
| [`SHT_HIUSER`](#sht_hiuser) | const | End of application-specific section types. |
| [`SHF_WRITE`](#shf_write) | const | Section is writable. |
| [`SHF_ALLOC`](#shf_alloc) | const | Section occupies memory during execution. |
| [`SHF_EXECINSTR`](#shf_execinstr) | const | Section is executable. |
| [`SHF_MERGE`](#shf_merge) | const | Section may be be merged to eliminate duplication. |
| [`SHF_STRINGS`](#shf_strings) | const | Section contains nul-terminated strings. |
| [`SHF_INFO_LINK`](#shf_info_link) | const | The `sh_info` field contains a section header table index. |
| [`SHF_LINK_ORDER`](#shf_link_order) | const | Section has special ordering requirements when combining sections. |
| [`SHF_OS_NONCONFORMING`](#shf_os_nonconforming) | const | Section requires special OS-specific handling. |
| [`SHF_GROUP`](#shf_group) | const | Section is a member of a group. |
| [`SHF_TLS`](#shf_tls) | const | Section holds thread-local storage. |
| [`SHF_COMPRESSED`](#shf_compressed) | const | Section is compressed. |
| [`SHF_MASKOS`](#shf_maskos) | const | OS-specific section flags. |
| [`SHF_GNU_RETAIN`](#shf_gnu_retain) | const | Section should not be garbage collected by the linker. |
| [`SHF_GNU_MBIND`](#shf_gnu_mbind) | const | Mbind section. |
| [`SHF_MASKPROC`](#shf_maskproc) | const | Processor-specific section flags. |
| [`SHF_EXCLUDE`](#shf_exclude) | const | This section is excluded from the final executable or shared library. |
| [`ELFCOMPRESS_ZLIB`](#elfcompress_zlib) | const | ZLIB/DEFLATE algorithm. |
| [`ELFCOMPRESS_ZSTD`](#elfcompress_zstd) | const | Zstandard algorithm. |
| [`ELFCOMPRESS_LOOS`](#elfcompress_loos) | const | Start of OS-specific compression types. |
| [`ELFCOMPRESS_HIOS`](#elfcompress_hios) | const | End of OS-specific compression types. |
| [`ELFCOMPRESS_LOPROC`](#elfcompress_loproc) | const | Start of processor-specific compression types. |
| [`ELFCOMPRESS_HIPROC`](#elfcompress_hiproc) | const | End of processor-specific compression types. |
| [`GRP_COMDAT`](#grp_comdat) | const | Mark group as COMDAT. |
| [`SYMINFO_BT_SELF`](#syminfo_bt_self) | const | Symbol bound to self |
| [`SYMINFO_BT_PARENT`](#syminfo_bt_parent) | const | Symbol bound to parent |
| [`SYMINFO_BT_LOWRESERVE`](#syminfo_bt_lowreserve) | const | Beginning of reserved entries |
| [`SYMINFO_FLG_DIRECT`](#syminfo_flg_direct) | const | Direct bound symbol |
| [`SYMINFO_FLG_PASSTHRU`](#syminfo_flg_passthru) | const | Pass-thru symbol for translator |
| [`SYMINFO_FLG_COPY`](#syminfo_flg_copy) | const | Symbol is a copy-reloc |
| [`SYMINFO_FLG_LAZYLOAD`](#syminfo_flg_lazyload) | const | Symbol bound to object to be lazy loaded |
| [`SYMINFO_NONE`](#syminfo_none) | const |  |
| [`SYMINFO_CURRENT`](#syminfo_current) | const |  |
| [`SYMINFO_NUM`](#syminfo_num) | const |  |
| [`STB_LOCAL`](#stb_local) | const | Local symbol. |
| [`STB_GLOBAL`](#stb_global) | const | Global symbol. |
| [`STB_WEAK`](#stb_weak) | const | Weak symbol. |
| [`STB_LOOS`](#stb_loos) | const | Start of OS-specific symbol binding. |
| [`STB_GNU_UNIQUE`](#stb_gnu_unique) | const | Unique symbol. |
| [`STB_HIOS`](#stb_hios) | const | End of OS-specific symbol binding. |
| [`STB_LOPROC`](#stb_loproc) | const | Start of processor-specific symbol binding. |
| [`STB_HIPROC`](#stb_hiproc) | const | End of processor-specific symbol binding. |
| [`STT_NOTYPE`](#stt_notype) | const | Symbol type is unspecified. |
| [`STT_OBJECT`](#stt_object) | const | Symbol is a data object. |
| [`STT_FUNC`](#stt_func) | const | Symbol is a code object. |
| [`STT_SECTION`](#stt_section) | const | Symbol is associated with a section. |
| [`STT_FILE`](#stt_file) | const | Symbol's name is a file name. |
| [`STT_COMMON`](#stt_common) | const | Symbol is a common data object. |
| [`STT_TLS`](#stt_tls) | const | Symbol is a thread-local storage object. |
| [`STT_LOOS`](#stt_loos) | const | Start of OS-specific symbol types. |
| [`STT_GNU_IFUNC`](#stt_gnu_ifunc) | const | Symbol is an indirect code object. |
| [`STT_HIOS`](#stt_hios) | const | End of OS-specific symbol types. |
| [`STT_LOPROC`](#stt_loproc) | const | Start of processor-specific symbol types. |
| [`STT_HIPROC`](#stt_hiproc) | const | End of processor-specific symbol types. |
| [`STV_DEFAULT`](#stv_default) | const | Default symbol visibility rules. |
| [`STV_INTERNAL`](#stv_internal) | const | Processor specific hidden class. |
| [`STV_HIDDEN`](#stv_hidden) | const | Symbol is not visible to other components. |
| [`STV_PROTECTED`](#stv_protected) | const | Symbol is visible to other components, but is not preemptible. |
| [`PN_XNUM`](#pn_xnum) | const | Special value for `FileHeader*::e_phnum`. |
| [`PT_NULL`](#pt_null) | const | Program header table entry is unused. |
| [`PT_LOAD`](#pt_load) | const | Loadable program segment. |
| [`PT_DYNAMIC`](#pt_dynamic) | const | Dynamic linking information. |
| [`PT_INTERP`](#pt_interp) | const | Program interpreter. |
| [`PT_NOTE`](#pt_note) | const | Auxiliary information. |
| [`PT_SHLIB`](#pt_shlib) | const | Reserved. |
| [`PT_PHDR`](#pt_phdr) | const | Segment contains the program header table. |
| [`PT_TLS`](#pt_tls) | const | Thread-local storage segment. |
| [`PT_LOOS`](#pt_loos) | const | Start of OS-specific segment types. |
| [`PT_GNU_EH_FRAME`](#pt_gnu_eh_frame) | const | GCC `.eh_frame_hdr` segment. |
| [`PT_GNU_STACK`](#pt_gnu_stack) | const | Indicates stack executability. |
| [`PT_GNU_RELRO`](#pt_gnu_relro) | const | Read-only after relocation. |
| [`PT_GNU_PROPERTY`](#pt_gnu_property) | const | Segment containing `.note.gnu.property` section. |
| [`PT_GNU_SFRAME`](#pt_gnu_sframe) | const | GNU SFrame stack trace format. |
| [`PT_HIOS`](#pt_hios) | const | End of OS-specific segment types. |
| [`PT_LOPROC`](#pt_loproc) | const | Start of processor-specific segment types. |
| [`PT_HIPROC`](#pt_hiproc) | const | End of processor-specific segment types. |
| [`PF_X`](#pf_x) | const | Segment is executable. |
| [`PF_W`](#pf_w) | const | Segment is writable. |
| [`PF_R`](#pf_r) | const | Segment is readable. |
| [`PF_MASKOS`](#pf_maskos) | const | OS-specific segment flags. |
| [`PF_MASKPROC`](#pf_maskproc) | const | Processor-specific segment flags. |
| [`ELF_NOTE_CORE`](#elf_note_core) | const | Note name for core files. |
| [`ELF_NOTE_LINUX`](#elf_note_linux) | const | Note name for linux core files. |
| [`NT_PRSTATUS`](#nt_prstatus) | const | Contains copy of prstatus struct. |
| [`NT_PRFPREG`](#nt_prfpreg) | const | Contains copy of fpregset struct. |
| [`NT_FPREGSET`](#nt_fpregset) | const | Contains copy of fpregset struct. |
| [`NT_PRPSINFO`](#nt_prpsinfo) | const | Contains copy of prpsinfo struct. |
| [`NT_PRXREG`](#nt_prxreg) | const | Contains copy of prxregset struct. |
| [`NT_TASKSTRUCT`](#nt_taskstruct) | const | Contains copy of task structure. |
| [`NT_PLATFORM`](#nt_platform) | const | String from sysinfo(SI_PLATFORM). |
| [`NT_AUXV`](#nt_auxv) | const | Contains copy of auxv array. |
| [`NT_GWINDOWS`](#nt_gwindows) | const | Contains copy of gwindows struct. |
| [`NT_ASRS`](#nt_asrs) | const | Contains copy of asrset struct. |
| [`NT_PSTATUS`](#nt_pstatus) | const | Contains copy of pstatus struct. |
| [`NT_PSINFO`](#nt_psinfo) | const | Contains copy of psinfo struct. |
| [`NT_PRCRED`](#nt_prcred) | const | Contains copy of prcred struct. |
| [`NT_UTSNAME`](#nt_utsname) | const | Contains copy of utsname struct. |
| [`NT_LWPSTATUS`](#nt_lwpstatus) | const | Contains copy of lwpstatus struct. |
| [`NT_LWPSINFO`](#nt_lwpsinfo) | const | Contains copy of lwpinfo struct. |
| [`NT_PRFPXREG`](#nt_prfpxreg) | const | Contains copy of fprxregset struct. |
| [`NT_SIGINFO`](#nt_siginfo) | const | Contains copy of siginfo_t, size might increase. |
| [`NT_FILE`](#nt_file) | const | Contains information about mapped files. |
| [`NT_PRXFPREG`](#nt_prxfpreg) | const | Contains copy of user_fxsr_struct. |
| [`NT_PPC_VMX`](#nt_ppc_vmx) | const | PowerPC Altivec/VMX registers. |
| [`NT_PPC_SPE`](#nt_ppc_spe) | const | PowerPC SPE/EVR registers. |
| [`NT_PPC_VSX`](#nt_ppc_vsx) | const | PowerPC VSX registers. |
| [`NT_PPC_TAR`](#nt_ppc_tar) | const | Target Address Register. |
| [`NT_PPC_PPR`](#nt_ppc_ppr) | const | Program Priority Register. |
| [`NT_PPC_DSCR`](#nt_ppc_dscr) | const | Data Stream Control Register. |
| [`NT_PPC_EBB`](#nt_ppc_ebb) | const | Event Based Branch Registers. |
| [`NT_PPC_PMU`](#nt_ppc_pmu) | const | Performance Monitor Registers. |
| [`NT_PPC_TM_CGPR`](#nt_ppc_tm_cgpr) | const | TM checkpointed GPR Registers. |
| [`NT_PPC_TM_CFPR`](#nt_ppc_tm_cfpr) | const | TM checkpointed FPR Registers. |
| [`NT_PPC_TM_CVMX`](#nt_ppc_tm_cvmx) | const | TM checkpointed VMX Registers. |
| [`NT_PPC_TM_CVSX`](#nt_ppc_tm_cvsx) | const | TM checkpointed VSX Registers. |
| [`NT_PPC_TM_SPR`](#nt_ppc_tm_spr) | const | TM Special Purpose Registers. |
| [`NT_PPC_TM_CTAR`](#nt_ppc_tm_ctar) | const | TM checkpointed Target Address Register. |
| [`NT_PPC_TM_CPPR`](#nt_ppc_tm_cppr) | const | TM checkpointed Program Priority Register. |
| [`NT_PPC_TM_CDSCR`](#nt_ppc_tm_cdscr) | const | TM checkpointed Data Stream Control Register. |
| [`NT_PPC_PKEY`](#nt_ppc_pkey) | const | Memory Protection Keys registers. |
| [`NT_386_TLS`](#nt_386_tls) | const | i386 TLS slots (struct user_desc). |
| [`NT_386_IOPERM`](#nt_386_ioperm) | const | x86 io permission bitmap (1=deny). |
| [`NT_X86_XSTATE`](#nt_x86_xstate) | const | x86 extended state using xsave. |
| [`NT_S390_HIGH_GPRS`](#nt_s390_high_gprs) | const | s390 upper register halves. |
| [`NT_S390_TIMER`](#nt_s390_timer) | const | s390 timer register. |
| [`NT_S390_TODCMP`](#nt_s390_todcmp) | const | s390 TOD clock comparator register. |
| [`NT_S390_TODPREG`](#nt_s390_todpreg) | const | s390 TOD programmable register. |
| [`NT_S390_CTRS`](#nt_s390_ctrs) | const | s390 control registers. |
| [`NT_S390_PREFIX`](#nt_s390_prefix) | const | s390 prefix register. |
| [`NT_S390_LAST_BREAK`](#nt_s390_last_break) | const | s390 breaking event address. |
| [`NT_S390_SYSTEM_CALL`](#nt_s390_system_call) | const | s390 system call restart data. |
| [`NT_S390_TDB`](#nt_s390_tdb) | const | s390 transaction diagnostic block. |
| [`NT_S390_VXRS_LOW`](#nt_s390_vxrs_low) | const | s390 vector registers 0-15 upper half. |
| [`NT_S390_VXRS_HIGH`](#nt_s390_vxrs_high) | const | s390 vector registers 16-31. |
| [`NT_S390_GS_CB`](#nt_s390_gs_cb) | const | s390 guarded storage registers. |
| [`NT_S390_GS_BC`](#nt_s390_gs_bc) | const | s390 guarded storage broadcast control block. |
| [`NT_S390_RI_CB`](#nt_s390_ri_cb) | const | s390 runtime instrumentation. |
| [`NT_ARM_VFP`](#nt_arm_vfp) | const | ARM VFP/NEON registers. |
| [`NT_ARM_TLS`](#nt_arm_tls) | const | ARM TLS register. |
| [`NT_ARM_HW_BREAK`](#nt_arm_hw_break) | const | ARM hardware breakpoint registers. |
| [`NT_ARM_HW_WATCH`](#nt_arm_hw_watch) | const | ARM hardware watchpoint registers. |
| [`NT_ARM_SYSTEM_CALL`](#nt_arm_system_call) | const | ARM system call number. |
| [`NT_ARM_SVE`](#nt_arm_sve) | const | ARM Scalable Vector Extension registers. |
| [`NT_VMCOREDD`](#nt_vmcoredd) | const | Vmcore Device Dump Note. |
| [`NT_MIPS_DSP`](#nt_mips_dsp) | const | MIPS DSP ASE registers. |
| [`NT_MIPS_FP_MODE`](#nt_mips_fp_mode) | const | MIPS floating-point mode. |
| [`NT_VERSION`](#nt_version) | const | Note type for version string. |
| [`DT_NULL`](#dt_null) | const | Marks end of dynamic section |
| [`DT_NEEDED`](#dt_needed) | const | Name of needed library |
| [`DT_PLTRELSZ`](#dt_pltrelsz) | const | Size in bytes of PLT relocs |
| [`DT_PLTGOT`](#dt_pltgot) | const | Processor defined value |
| [`DT_HASH`](#dt_hash) | const | Address of symbol hash table |
| [`DT_STRTAB`](#dt_strtab) | const | Address of string table |
| [`DT_SYMTAB`](#dt_symtab) | const | Address of symbol table |
| [`DT_RELA`](#dt_rela) | const | Address of Rela relocs |
| [`DT_RELASZ`](#dt_relasz) | const | Total size of Rela relocs |
| [`DT_RELAENT`](#dt_relaent) | const | Size of one Rela reloc |
| [`DT_STRSZ`](#dt_strsz) | const | Size of string table |
| [`DT_SYMENT`](#dt_syment) | const | Size of one symbol table entry |
| [`DT_INIT`](#dt_init) | const | Address of init function |
| [`DT_FINI`](#dt_fini) | const | Address of termination function |
| [`DT_SONAME`](#dt_soname) | const | Name of shared object |
| [`DT_RPATH`](#dt_rpath) | const | Library search path (deprecated) |
| [`DT_SYMBOLIC`](#dt_symbolic) | const | Start symbol search here |
| [`DT_REL`](#dt_rel) | const | Address of Rel relocs |
| [`DT_RELSZ`](#dt_relsz) | const | Total size of Rel relocs |
| [`DT_RELENT`](#dt_relent) | const | Size of one Rel reloc |
| [`DT_PLTREL`](#dt_pltrel) | const | Type of reloc in PLT |
| [`DT_DEBUG`](#dt_debug) | const | For debugging; unspecified |
| [`DT_TEXTREL`](#dt_textrel) | const | Reloc might modify .text |
| [`DT_JMPREL`](#dt_jmprel) | const | Address of PLT relocs |
| [`DT_BIND_NOW`](#dt_bind_now) | const | Process relocations of object |
| [`DT_INIT_ARRAY`](#dt_init_array) | const | Array with addresses of init fct |
| [`DT_FINI_ARRAY`](#dt_fini_array) | const | Array with addresses of fini fct |
| [`DT_INIT_ARRAYSZ`](#dt_init_arraysz) | const | Size in bytes of DT_INIT_ARRAY |
| [`DT_FINI_ARRAYSZ`](#dt_fini_arraysz) | const | Size in bytes of DT_FINI_ARRAY |
| [`DT_RUNPATH`](#dt_runpath) | const | Library search path |
| [`DT_FLAGS`](#dt_flags) | const | Flags for the object being loaded |
| [`DT_ENCODING`](#dt_encoding) | const | Start of encoded range |
| [`DT_PREINIT_ARRAY`](#dt_preinit_array) | const | Array with addresses of preinit fct |
| [`DT_PREINIT_ARRAYSZ`](#dt_preinit_arraysz) | const | size in bytes of DT_PREINIT_ARRAY |
| [`DT_SYMTAB_SHNDX`](#dt_symtab_shndx) | const | Address of SYMTAB_SHNDX section |
| [`DT_LOOS`](#dt_loos) | const | Start of OS-specific |
| [`DT_HIOS`](#dt_hios) | const | End of OS-specific |
| [`DT_LOPROC`](#dt_loproc) | const | Start of processor-specific |
| [`DT_HIPROC`](#dt_hiproc) | const | End of processor-specific |
| [`DT_VALRNGLO`](#dt_valrnglo) | const |  |
| [`DT_GNU_PRELINKED`](#dt_gnu_prelinked) | const | Prelinking timestamp |
| [`DT_GNU_CONFLICTSZ`](#dt_gnu_conflictsz) | const | Size of conflict section |
| [`DT_GNU_LIBLISTSZ`](#dt_gnu_liblistsz) | const | Size of library list |
| [`DT_CHECKSUM`](#dt_checksum) | const |  |
| [`DT_PLTPADSZ`](#dt_pltpadsz) | const |  |
| [`DT_MOVEENT`](#dt_moveent) | const |  |
| [`DT_MOVESZ`](#dt_movesz) | const |  |
| [`DT_FEATURE_1`](#dt_feature_1) | const | Feature selection (DTF_*). |
| [`DT_POSFLAG_1`](#dt_posflag_1) | const | Flags for DT_* entries, affecting the following DT_* entry. |
| [`DT_SYMINSZ`](#dt_syminsz) | const | Size of syminfo table (in bytes) |
| [`DT_SYMINENT`](#dt_syminent) | const | Entry size of syminfo |
| [`DT_VALRNGHI`](#dt_valrnghi) | const |  |
| [`DT_ADDRRNGLO`](#dt_addrrnglo) | const |  |
| [`DT_GNU_HASH`](#dt_gnu_hash) | const | GNU-style hash table. |
| [`DT_TLSDESC_PLT`](#dt_tlsdesc_plt) | const |  |
| [`DT_TLSDESC_GOT`](#dt_tlsdesc_got) | const |  |
| [`DT_GNU_CONFLICT`](#dt_gnu_conflict) | const | Start of conflict section |
| [`DT_GNU_LIBLIST`](#dt_gnu_liblist) | const | Library list |
| [`DT_CONFIG`](#dt_config) | const | Configuration information. |
| [`DT_DEPAUDIT`](#dt_depaudit) | const | Dependency auditing. |
| [`DT_AUDIT`](#dt_audit) | const | Object auditing. |
| [`DT_PLTPAD`](#dt_pltpad) | const | PLT padding. |
| [`DT_MOVETAB`](#dt_movetab) | const | Move table. |
| [`DT_SYMINFO`](#dt_syminfo) | const | Syminfo table. |
| [`DT_ADDRRNGHI`](#dt_addrrnghi) | const |  |
| [`DT_VERSYM`](#dt_versym) | const |  |
| [`DT_RELACOUNT`](#dt_relacount) | const |  |
| [`DT_RELCOUNT`](#dt_relcount) | const |  |
| [`DT_FLAGS_1`](#dt_flags_1) | const | State flags, see DF_1_* below. |
| [`DT_VERDEF`](#dt_verdef) | const | Address of version definition table |
| [`DT_VERDEFNUM`](#dt_verdefnum) | const | Number of version definitions |
| [`DT_VERNEED`](#dt_verneed) | const | Address of table with needed versions |
| [`DT_VERNEEDNUM`](#dt_verneednum) | const | Number of needed versions |
| [`DT_AUXILIARY`](#dt_auxiliary) | const | Shared object to load before self |
| [`DT_FILTER`](#dt_filter) | const | Shared object to get values from |
| [`DF_ORIGIN`](#df_origin) | const | Object may use DF_ORIGIN |
| [`DF_SYMBOLIC`](#df_symbolic) | const | Symbol resolutions starts here |
| [`DF_TEXTREL`](#df_textrel) | const | Object contains text relocations |
| [`DF_BIND_NOW`](#df_bind_now) | const | No lazy binding for this object |
| [`DF_STATIC_TLS`](#df_static_tls) | const | Module uses the static TLS model |
| [`DF_1_NOW`](#df_1_now) | const | Set RTLD_NOW for this object. |
| [`DF_1_GLOBAL`](#df_1_global) | const | Set RTLD_GLOBAL for this object. |
| [`DF_1_GROUP`](#df_1_group) | const | Set RTLD_GROUP for this object. |
| [`DF_1_NODELETE`](#df_1_nodelete) | const | Set RTLD_NODELETE for this object. |
| [`DF_1_LOADFLTR`](#df_1_loadfltr) | const | Trigger filtee loading at runtime. |
| [`DF_1_INITFIRST`](#df_1_initfirst) | const | Set RTLD_INITFIRST for this object. |
| [`DF_1_NOOPEN`](#df_1_noopen) | const | Set RTLD_NOOPEN for this object. |
| [`DF_1_ORIGIN`](#df_1_origin) | const | $ORIGIN must be handled. |
| [`DF_1_DIRECT`](#df_1_direct) | const | Direct binding enabled. |
| [`DF_1_TRANS`](#df_1_trans) | const |  |
| [`DF_1_INTERPOSE`](#df_1_interpose) | const | Object is used to interpose. |
| [`DF_1_NODEFLIB`](#df_1_nodeflib) | const | Ignore default lib search path. |
| [`DF_1_NODUMP`](#df_1_nodump) | const | Object can't be dldump'ed. |
| [`DF_1_CONFALT`](#df_1_confalt) | const | Configuration alternative created. |
| [`DF_1_ENDFILTEE`](#df_1_endfiltee) | const | Filtee terminates filters search. |
| [`DF_1_DISPRELDNE`](#df_1_dispreldne) | const | Disp reloc applied at build time. |
| [`DF_1_DISPRELPND`](#df_1_disprelpnd) | const | Disp reloc applied at run-time. |
| [`DF_1_NODIRECT`](#df_1_nodirect) | const | Object has no-direct binding. |
| [`DF_1_IGNMULDEF`](#df_1_ignmuldef) | const |  |
| [`DF_1_NOKSYMS`](#df_1_noksyms) | const |  |
| [`DF_1_NOHDR`](#df_1_nohdr) | const |  |
| [`DF_1_EDITED`](#df_1_edited) | const | Object is modified after built. |
| [`DF_1_NORELOC`](#df_1_noreloc) | const |  |
| [`DF_1_SYMINTPOSE`](#df_1_symintpose) | const | Object has individual interposers. |
| [`DF_1_GLOBAUDIT`](#df_1_globaudit) | const | Global auditing required. |
| [`DF_1_SINGLETON`](#df_1_singleton) | const | Singleton symbols are used. |
| [`DF_1_STUB`](#df_1_stub) | const |  |
| [`DF_1_PIE`](#df_1_pie) | const |  |
| [`VERSYM_HIDDEN`](#versym_hidden) | const | Symbol is hidden. |
| [`VERSYM_VERSION`](#versym_version) | const | Symbol version index. |
| [`VER_DEF_NONE`](#ver_def_none) | const | No version |
| [`VER_DEF_CURRENT`](#ver_def_current) | const | Current version |
| [`VER_FLG_BASE`](#ver_flg_base) | const | Version definition of file itself |
| [`VER_FLG_WEAK`](#ver_flg_weak) | const | Weak version identifier |
| [`VER_NDX_LOCAL`](#ver_ndx_local) | const | Symbol is local. |
| [`VER_NDX_GLOBAL`](#ver_ndx_global) | const | Symbol is global. |
| [`VER_NEED_NONE`](#ver_need_none) | const | No version |
| [`VER_NEED_CURRENT`](#ver_need_current) | const | Current version |
| [`ELF_NOTE_SOLARIS`](#elf_note_solaris) | const | Solaris entries in the note section have this name. |
| [`NT_SOLARIS_PAGESIZE_HINT`](#nt_solaris_pagesize_hint) | const | Desired pagesize for the binary. |
| [`ELF_NOTE_GNU`](#elf_note_gnu) | const | GNU entries in the note section have this name. |
| [`ELF_NOTE_GO`](#elf_note_go) | const | Go entries in the note section have this name. |
| [`NT_GNU_ABI_TAG`](#nt_gnu_abi_tag) | const | ABI information. |
| [`ELF_NOTE_OS_LINUX`](#elf_note_os_linux) | const | OS descriptor for `NT_GNU_ABI_TAG`. |
| [`ELF_NOTE_OS_GNU`](#elf_note_os_gnu) | const | OS descriptor for `NT_GNU_ABI_TAG`. |
| [`ELF_NOTE_OS_SOLARIS2`](#elf_note_os_solaris2) | const | OS descriptor for `NT_GNU_ABI_TAG`. |
| [`ELF_NOTE_OS_FREEBSD`](#elf_note_os_freebsd) | const | OS descriptor for `NT_GNU_ABI_TAG`. |
| [`NT_GNU_HWCAP`](#nt_gnu_hwcap) | const | Synthetic hwcap information. |
| [`NT_GNU_BUILD_ID`](#nt_gnu_build_id) | const | Build ID bits as generated by `ld --build-id`. |
| [`NT_GO_BUILD_ID`](#nt_go_build_id) | const | Build ID bits as generated by Go's gc compiler. |
| [`NT_GNU_GOLD_VERSION`](#nt_gnu_gold_version) | const | Version note generated by GNU gold containing a version string. |
| [`NT_GNU_PROPERTY_TYPE_0`](#nt_gnu_property_type_0) | const | Program property. |
| [`GNU_PROPERTY_STACK_SIZE`](#gnu_property_stack_size) | const | Stack size. |
| [`GNU_PROPERTY_NO_COPY_ON_PROTECTED`](#gnu_property_no_copy_on_protected) | const | No copy relocation on protected data symbol. |
| [`GNU_PROPERTY_UINT32_AND_LO`](#gnu_property_uint32_and_lo) | const |  |
| [`GNU_PROPERTY_UINT32_AND_HI`](#gnu_property_uint32_and_hi) | const |  |
| [`GNU_PROPERTY_UINT32_OR_LO`](#gnu_property_uint32_or_lo) | const |  |
| [`GNU_PROPERTY_UINT32_OR_HI`](#gnu_property_uint32_or_hi) | const |  |
| [`GNU_PROPERTY_1_NEEDED`](#gnu_property_1_needed) | const | The needed properties by the object file. |
| [`GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS`](#gnu_property_1_needed_indirect_extern_access) | const | Set if the object file requires canonical function pointers and |
| [`GNU_PROPERTY_LOPROC`](#gnu_property_loproc) | const | Processor-specific semantics, lo |
| [`GNU_PROPERTY_HIPROC`](#gnu_property_hiproc) | const | Processor-specific semantics, hi |
| [`GNU_PROPERTY_LOUSER`](#gnu_property_louser) | const | Application-specific semantics, lo |
| [`GNU_PROPERTY_HIUSER`](#gnu_property_hiuser) | const | Application-specific semantics, hi |
| [`GNU_PROPERTY_AARCH64_FEATURE_1_AND`](#gnu_property_aarch64_feature_1_and) | const | AArch64 specific GNU properties. |
| [`GNU_PROPERTY_AARCH64_FEATURE_PAUTH`](#gnu_property_aarch64_feature_pauth) | const |  |
| [`GNU_PROPERTY_AARCH64_FEATURE_1_BTI`](#gnu_property_aarch64_feature_1_bti) | const |  |
| [`GNU_PROPERTY_AARCH64_FEATURE_1_PAC`](#gnu_property_aarch64_feature_1_pac) | const |  |
| [`GNU_PROPERTY_X86_UINT32_AND_LO`](#gnu_property_x86_uint32_and_lo) | const |  |
| [`GNU_PROPERTY_X86_UINT32_AND_HI`](#gnu_property_x86_uint32_and_hi) | const |  |
| [`GNU_PROPERTY_X86_UINT32_OR_LO`](#gnu_property_x86_uint32_or_lo) | const |  |
| [`GNU_PROPERTY_X86_UINT32_OR_HI`](#gnu_property_x86_uint32_or_hi) | const |  |
| [`GNU_PROPERTY_X86_UINT32_OR_AND_LO`](#gnu_property_x86_uint32_or_and_lo) | const |  |
| [`GNU_PROPERTY_X86_UINT32_OR_AND_HI`](#gnu_property_x86_uint32_or_and_hi) | const |  |
| [`GNU_PROPERTY_X86_ISA_1_USED`](#gnu_property_x86_isa_1_used) | const | The x86 instruction sets indicated by the corresponding bits are |
| [`GNU_PROPERTY_X86_ISA_1_NEEDED`](#gnu_property_x86_isa_1_needed) | const | The x86 instruction sets indicated by the corresponding bits are |
| [`GNU_PROPERTY_X86_FEATURE_1_AND`](#gnu_property_x86_feature_1_and) | const | X86 processor-specific features used in program. |
| [`GNU_PROPERTY_X86_ISA_1_BASELINE`](#gnu_property_x86_isa_1_baseline) | const | GNU_PROPERTY_X86_ISA_1_BASELINE: CMOV, CX8 (cmpxchg8b), FPU (fld) |
| [`GNU_PROPERTY_X86_ISA_1_V2`](#gnu_property_x86_isa_1_v2) | const | GNU_PROPERTY_X86_ISA_1_V2: GNU_PROPERTY_X86_ISA_1_BASELINE |
| [`GNU_PROPERTY_X86_ISA_1_V3`](#gnu_property_x86_isa_1_v3) | const | GNU_PROPERTY_X86_ISA_1_V3: GNU_PROPERTY_X86_ISA_1_V2, AVX, AVX2, BMI1 |
| [`GNU_PROPERTY_X86_ISA_1_V4`](#gnu_property_x86_isa_1_v4) | const | GNU_PROPERTY_X86_ISA_1_V4: GNU_PROPERTY_X86_ISA_1_V3, AVX512F |
| [`GNU_PROPERTY_X86_FEATURE_1_IBT`](#gnu_property_x86_feature_1_ibt) | const | This indicates that all executable sections are compatible with IBT. |
| [`GNU_PROPERTY_X86_FEATURE_1_SHSTK`](#gnu_property_x86_feature_1_shstk) | const | This indicates that all executable sections are compatible with SHSTK. |
| [`R_68K_NONE`](#r_68k_none) | const | No reloc |
| [`R_68K_32`](#r_68k_32) | const | Direct 32 bit |
| [`R_68K_16`](#r_68k_16) | const | Direct 16 bit |
| [`R_68K_8`](#r_68k_8) | const | Direct 8 bit |
| [`R_68K_PC32`](#r_68k_pc32) | const | PC relative 32 bit |
| [`R_68K_PC16`](#r_68k_pc16) | const | PC relative 16 bit |
| [`R_68K_PC8`](#r_68k_pc8) | const | PC relative 8 bit |
| [`R_68K_GOT32`](#r_68k_got32) | const | 32 bit PC relative GOT entry |
| [`R_68K_GOT16`](#r_68k_got16) | const | 16 bit PC relative GOT entry |
| [`R_68K_GOT8`](#r_68k_got8) | const | 8 bit PC relative GOT entry |
| [`R_68K_GOT32O`](#r_68k_got32o) | const | 32 bit GOT offset |
| [`R_68K_GOT16O`](#r_68k_got16o) | const | 16 bit GOT offset |
| [`R_68K_GOT8O`](#r_68k_got8o) | const | 8 bit GOT offset |
| [`R_68K_PLT32`](#r_68k_plt32) | const | 32 bit PC relative PLT address |
| [`R_68K_PLT16`](#r_68k_plt16) | const | 16 bit PC relative PLT address |
| [`R_68K_PLT8`](#r_68k_plt8) | const | 8 bit PC relative PLT address |
| [`R_68K_PLT32O`](#r_68k_plt32o) | const | 32 bit PLT offset |
| [`R_68K_PLT16O`](#r_68k_plt16o) | const | 16 bit PLT offset |
| [`R_68K_PLT8O`](#r_68k_plt8o) | const | 8 bit PLT offset |
| [`R_68K_COPY`](#r_68k_copy) | const | Copy symbol at runtime |
| [`R_68K_GLOB_DAT`](#r_68k_glob_dat) | const | Create GOT entry |
| [`R_68K_JMP_SLOT`](#r_68k_jmp_slot) | const | Create PLT entry |
| [`R_68K_RELATIVE`](#r_68k_relative) | const | Adjust by program base |
| [`R_68K_TLS_GD32`](#r_68k_tls_gd32) | const | 32 bit GOT offset for GD |
| [`R_68K_TLS_GD16`](#r_68k_tls_gd16) | const | 16 bit GOT offset for GD |
| [`R_68K_TLS_GD8`](#r_68k_tls_gd8) | const | 8 bit GOT offset for GD |
| [`R_68K_TLS_LDM32`](#r_68k_tls_ldm32) | const | 32 bit GOT offset for LDM |
| [`R_68K_TLS_LDM16`](#r_68k_tls_ldm16) | const | 16 bit GOT offset for LDM |
| [`R_68K_TLS_LDM8`](#r_68k_tls_ldm8) | const | 8 bit GOT offset for LDM |
| [`R_68K_TLS_LDO32`](#r_68k_tls_ldo32) | const | 32 bit module-relative offset |
| [`R_68K_TLS_LDO16`](#r_68k_tls_ldo16) | const | 16 bit module-relative offset |
| [`R_68K_TLS_LDO8`](#r_68k_tls_ldo8) | const | 8 bit module-relative offset |
| [`R_68K_TLS_IE32`](#r_68k_tls_ie32) | const | 32 bit GOT offset for IE |
| [`R_68K_TLS_IE16`](#r_68k_tls_ie16) | const | 16 bit GOT offset for IE |
| [`R_68K_TLS_IE8`](#r_68k_tls_ie8) | const | 8 bit GOT offset for IE |
| [`R_68K_TLS_LE32`](#r_68k_tls_le32) | const | 32 bit offset relative to static TLS block |
| [`R_68K_TLS_LE16`](#r_68k_tls_le16) | const | 16 bit offset relative to static TLS block |
| [`R_68K_TLS_LE8`](#r_68k_tls_le8) | const | 8 bit offset relative to static TLS block |
| [`R_68K_TLS_DTPMOD32`](#r_68k_tls_dtpmod32) | const | 32 bit module number |
| [`R_68K_TLS_DTPREL32`](#r_68k_tls_dtprel32) | const | 32 bit module-relative offset |
| [`R_68K_TLS_TPREL32`](#r_68k_tls_tprel32) | const | 32 bit TP-relative offset |
| [`R_386_NONE`](#r_386_none) | const | No reloc |
| [`R_386_32`](#r_386_32) | const | Direct 32 bit |
| [`R_386_PC32`](#r_386_pc32) | const | PC relative 32 bit |
| [`R_386_GOT32`](#r_386_got32) | const | 32 bit GOT entry |
| [`R_386_PLT32`](#r_386_plt32) | const | 32 bit PLT address |
| [`R_386_COPY`](#r_386_copy) | const | Copy symbol at runtime |
| [`R_386_GLOB_DAT`](#r_386_glob_dat) | const | Create GOT entry |
| [`R_386_JMP_SLOT`](#r_386_jmp_slot) | const | Create PLT entry |
| [`R_386_RELATIVE`](#r_386_relative) | const | Adjust by program base |
| [`R_386_GOTOFF`](#r_386_gotoff) | const | 32 bit offset to GOT |
| [`R_386_GOTPC`](#r_386_gotpc) | const | 32 bit PC relative offset to GOT |
| [`R_386_32PLT`](#r_386_32plt) | const | Direct 32 bit PLT address |
| [`R_386_TLS_TPOFF`](#r_386_tls_tpoff) | const | Offset in static TLS block |
| [`R_386_TLS_IE`](#r_386_tls_ie) | const | Address of GOT entry for static TLS block offset |
| [`R_386_TLS_GOTIE`](#r_386_tls_gotie) | const | GOT entry for static TLS block offset |
| [`R_386_TLS_LE`](#r_386_tls_le) | const | Offset relative to static TLS block |
| [`R_386_TLS_GD`](#r_386_tls_gd) | const | Direct 32 bit for GNU version of general dynamic thread local data |
| [`R_386_TLS_LDM`](#r_386_tls_ldm) | const | Direct 32 bit for GNU version of local dynamic thread local data in LE code |
| [`R_386_16`](#r_386_16) | const | Direct 16 bit |
| [`R_386_PC16`](#r_386_pc16) | const | PC relative 16 bit |
| [`R_386_8`](#r_386_8) | const | Direct 8 bit |
| [`R_386_PC8`](#r_386_pc8) | const | PC relative 8 bit |
| [`R_386_TLS_GD_32`](#r_386_tls_gd_32) | const | Direct 32 bit for general dynamic thread local data |
| [`R_386_TLS_GD_PUSH`](#r_386_tls_gd_push) | const | Tag for pushl in GD TLS code |
| [`R_386_TLS_GD_CALL`](#r_386_tls_gd_call) | const | Relocation for call to __tls_get_addr() |
| [`R_386_TLS_GD_POP`](#r_386_tls_gd_pop) | const | Tag for popl in GD TLS code |
| [`R_386_TLS_LDM_32`](#r_386_tls_ldm_32) | const | Direct 32 bit for local dynamic thread local data in LE code |
| [`R_386_TLS_LDM_PUSH`](#r_386_tls_ldm_push) | const | Tag for pushl in LDM TLS code |
| [`R_386_TLS_LDM_CALL`](#r_386_tls_ldm_call) | const | Relocation for call to __tls_get_addr() in LDM code |
| [`R_386_TLS_LDM_POP`](#r_386_tls_ldm_pop) | const | Tag for popl in LDM TLS code |
| [`R_386_TLS_LDO_32`](#r_386_tls_ldo_32) | const | Offset relative to TLS block |
| [`R_386_TLS_IE_32`](#r_386_tls_ie_32) | const | GOT entry for negated static TLS block offset |
| [`R_386_TLS_LE_32`](#r_386_tls_le_32) | const | Negated offset relative to static TLS block |
| [`R_386_TLS_DTPMOD32`](#r_386_tls_dtpmod32) | const | ID of module containing symbol |
| [`R_386_TLS_DTPOFF32`](#r_386_tls_dtpoff32) | const | Offset in TLS block |
| [`R_386_TLS_TPOFF32`](#r_386_tls_tpoff32) | const | Negated offset in static TLS block |
| [`R_386_SIZE32`](#r_386_size32) | const | 32-bit symbol size |
| [`R_386_TLS_GOTDESC`](#r_386_tls_gotdesc) | const | GOT offset for TLS descriptor. |
| [`R_386_TLS_DESC_CALL`](#r_386_tls_desc_call) | const | Marker of call through TLS descriptor for relaxation. |
| [`R_386_TLS_DESC`](#r_386_tls_desc) | const | TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol. |
| [`R_386_IRELATIVE`](#r_386_irelative) | const | Adjust indirectly by program base |
| [`R_386_GOT32X`](#r_386_got32x) | const | Load from 32 bit GOT entry, relaxable. |
| [`R_SHARC_ADDR24_V3`](#r_sharc_addr24_v3) | const | 24-bit absolute address in bits 23:0 of a 48-bit instr |
| [`R_SHARC_ADDR32_V3`](#r_sharc_addr32_v3) | const | 32-bit absolute address in bits 31:0 of a 48-bit instr |
| [`R_SHARC_ADDR_VAR_V3`](#r_sharc_addr_var_v3) | const | 32-bit absolute address in bits 31:0 of a 32-bit data location |
| [`R_SHARC_PCRSHORT_V3`](#r_sharc_pcrshort_v3) | const | 6-bit PC-relative address in bits 32:27 of a 48-bit instr |
| [`R_SHARC_PCRLONG_V3`](#r_sharc_pcrlong_v3) | const | 24-bit PC-relative address in bits 23:0 of a 48-bit instr |
| [`R_SHARC_DATA6_V3`](#r_sharc_data6_v3) | const | 6-bit absolute address in bits 32:27 of a 48-bit instr |
| [`R_SHARC_DATA16_V3`](#r_sharc_data16_v3) | const | 16-bit absolute address in bits 39:24 of a 48-bit instr |
| [`R_SHARC_DATA6_VISA_V3`](#r_sharc_data6_visa_v3) | const | 6-bit absolute address into bits 16:11 of a 32-bit instr |
| [`R_SHARC_DATA7_VISA_V3`](#r_sharc_data7_visa_v3) | const | 7-bit absolute address into bits 6:0 of a 32-bit instr |
| [`R_SHARC_DATA16_VISA_V3`](#r_sharc_data16_visa_v3) | const | 16-bit absolute address into bits 15:0 of a 32-bit instr |
| [`R_SHARC_PCR6_VISA_V3`](#r_sharc_pcr6_visa_v3) | const | 6-bit PC-relative address into bits 16:11 of a Type B |
| [`R_SHARC_ADDR_VAR16_V3`](#r_sharc_addr_var16_v3) | const | 16-bit absolute address into bits 15:0 of a 16-bit location. |
| [`R_SHARC_CALC_PUSH_ADDR`](#r_sharc_calc_push_addr) | const |  |
| [`R_SHARC_CALC_PUSH_ADDEND`](#r_sharc_calc_push_addend) | const |  |
| [`R_SHARC_CALC_ADD`](#r_sharc_calc_add) | const |  |
| [`R_SHARC_CALC_SUB`](#r_sharc_calc_sub) | const |  |
| [`R_SHARC_CALC_MUL`](#r_sharc_calc_mul) | const |  |
| [`R_SHARC_CALC_DIV`](#r_sharc_calc_div) | const |  |
| [`R_SHARC_CALC_MOD`](#r_sharc_calc_mod) | const |  |
| [`R_SHARC_CALC_LSHIFT`](#r_sharc_calc_lshift) | const |  |
| [`R_SHARC_CALC_RSHIFT`](#r_sharc_calc_rshift) | const |  |
| [`R_SHARC_CALC_AND`](#r_sharc_calc_and) | const |  |
| [`R_SHARC_CALC_OR`](#r_sharc_calc_or) | const |  |
| [`R_SHARC_CALC_XOR`](#r_sharc_calc_xor) | const |  |
| [`R_SHARC_CALC_PUSH_LEN`](#r_sharc_calc_push_len) | const |  |
| [`R_SHARC_CALC_NOT`](#r_sharc_calc_not) | const |  |
| [`SHT_SHARC_ADI_ATTRIBUTES`](#sht_sharc_adi_attributes) | const | .adi.attributes |
| [`STT_SPARC_REGISTER`](#stt_sparc_register) | const | Global register reserved to app. |
| [`EF_SPARCV9_MM`](#ef_sparcv9_mm) | const |  |
| [`EF_SPARCV9_TSO`](#ef_sparcv9_tso) | const |  |
| [`EF_SPARCV9_PSO`](#ef_sparcv9_pso) | const |  |
| [`EF_SPARCV9_RMO`](#ef_sparcv9_rmo) | const |  |
| [`EF_SPARC_LEDATA`](#ef_sparc_ledata) | const | little endian data |
| [`EF_SPARC_EXT_MASK`](#ef_sparc_ext_mask) | const |  |
| [`EF_SPARC_32PLUS`](#ef_sparc_32plus) | const | generic V8+ features |
| [`EF_SPARC_SUN_US1`](#ef_sparc_sun_us1) | const | Sun UltraSPARC1 extensions |
| [`EF_SPARC_HAL_R1`](#ef_sparc_hal_r1) | const | HAL R1 extensions |
| [`EF_SPARC_SUN_US3`](#ef_sparc_sun_us3) | const | Sun UltraSPARCIII extensions |
| [`R_SPARC_NONE`](#r_sparc_none) | const | No reloc |
| [`R_SPARC_8`](#r_sparc_8) | const | Direct 8 bit |
| [`R_SPARC_16`](#r_sparc_16) | const | Direct 16 bit |
| [`R_SPARC_32`](#r_sparc_32) | const | Direct 32 bit |
| [`R_SPARC_DISP8`](#r_sparc_disp8) | const | PC relative 8 bit |
| [`R_SPARC_DISP16`](#r_sparc_disp16) | const | PC relative 16 bit |
| [`R_SPARC_DISP32`](#r_sparc_disp32) | const | PC relative 32 bit |
| [`R_SPARC_WDISP30`](#r_sparc_wdisp30) | const | PC relative 30 bit shifted |
| [`R_SPARC_WDISP22`](#r_sparc_wdisp22) | const | PC relative 22 bit shifted |
| [`R_SPARC_HI22`](#r_sparc_hi22) | const | High 22 bit |
| [`R_SPARC_22`](#r_sparc_22) | const | Direct 22 bit |
| [`R_SPARC_13`](#r_sparc_13) | const | Direct 13 bit |
| [`R_SPARC_LO10`](#r_sparc_lo10) | const | Truncated 10 bit |
| [`R_SPARC_GOT10`](#r_sparc_got10) | const | Truncated 10 bit GOT entry |
| [`R_SPARC_GOT13`](#r_sparc_got13) | const | 13 bit GOT entry |
| [`R_SPARC_GOT22`](#r_sparc_got22) | const | 22 bit GOT entry shifted |
| [`R_SPARC_PC10`](#r_sparc_pc10) | const | PC relative 10 bit truncated |
| [`R_SPARC_PC22`](#r_sparc_pc22) | const | PC relative 22 bit shifted |
| [`R_SPARC_WPLT30`](#r_sparc_wplt30) | const | 30 bit PC relative PLT address |
| [`R_SPARC_COPY`](#r_sparc_copy) | const | Copy symbol at runtime |
| [`R_SPARC_GLOB_DAT`](#r_sparc_glob_dat) | const | Create GOT entry |
| [`R_SPARC_JMP_SLOT`](#r_sparc_jmp_slot) | const | Create PLT entry |
| [`R_SPARC_RELATIVE`](#r_sparc_relative) | const | Adjust by program base |
| [`R_SPARC_UA32`](#r_sparc_ua32) | const | Direct 32 bit unaligned |
| [`R_SPARC_PLT32`](#r_sparc_plt32) | const | Direct 32 bit ref to PLT entry |
| [`R_SPARC_HIPLT22`](#r_sparc_hiplt22) | const | High 22 bit PLT entry |
| [`R_SPARC_LOPLT10`](#r_sparc_loplt10) | const | Truncated 10 bit PLT entry |
| [`R_SPARC_PCPLT32`](#r_sparc_pcplt32) | const | PC rel 32 bit ref to PLT entry |
| [`R_SPARC_PCPLT22`](#r_sparc_pcplt22) | const | PC rel high 22 bit PLT entry |
| [`R_SPARC_PCPLT10`](#r_sparc_pcplt10) | const | PC rel trunc 10 bit PLT entry |
| [`R_SPARC_10`](#r_sparc_10) | const | Direct 10 bit |
| [`R_SPARC_11`](#r_sparc_11) | const | Direct 11 bit |
| [`R_SPARC_64`](#r_sparc_64) | const | Direct 64 bit |
| [`R_SPARC_OLO10`](#r_sparc_olo10) | const | 10bit with secondary 13bit addend |
| [`R_SPARC_HH22`](#r_sparc_hh22) | const | Top 22 bits of direct 64 bit |
| [`R_SPARC_HM10`](#r_sparc_hm10) | const | High middle 10 bits of ... |
| [`R_SPARC_LM22`](#r_sparc_lm22) | const | Low middle 22 bits of ... |
| [`R_SPARC_PC_HH22`](#r_sparc_pc_hh22) | const | Top 22 bits of pc rel 64 bit |
| [`R_SPARC_PC_HM10`](#r_sparc_pc_hm10) | const | High middle 10 bit of ... |
| [`R_SPARC_PC_LM22`](#r_sparc_pc_lm22) | const | Low miggle 22 bits of ... |
| [`R_SPARC_WDISP16`](#r_sparc_wdisp16) | const | PC relative 16 bit shifted |
| [`R_SPARC_WDISP19`](#r_sparc_wdisp19) | const | PC relative 19 bit shifted |
| [`R_SPARC_GLOB_JMP`](#r_sparc_glob_jmp) | const | was part of v9 ABI but was removed |
| [`R_SPARC_7`](#r_sparc_7) | const | Direct 7 bit |
| [`R_SPARC_5`](#r_sparc_5) | const | Direct 5 bit |
| [`R_SPARC_6`](#r_sparc_6) | const | Direct 6 bit |
| [`R_SPARC_DISP64`](#r_sparc_disp64) | const | PC relative 64 bit |
| [`R_SPARC_PLT64`](#r_sparc_plt64) | const | Direct 64 bit ref to PLT entry |
| [`R_SPARC_HIX22`](#r_sparc_hix22) | const | High 22 bit complemented |
| [`R_SPARC_LOX10`](#r_sparc_lox10) | const | Truncated 11 bit complemented |
| [`R_SPARC_H44`](#r_sparc_h44) | const | Direct high 12 of 44 bit |
| [`R_SPARC_M44`](#r_sparc_m44) | const | Direct mid 22 of 44 bit |
| [`R_SPARC_L44`](#r_sparc_l44) | const | Direct low 10 of 44 bit |
| [`R_SPARC_REGISTER`](#r_sparc_register) | const | Global register usage |
| [`R_SPARC_UA64`](#r_sparc_ua64) | const | Direct 64 bit unaligned |
| [`R_SPARC_UA16`](#r_sparc_ua16) | const | Direct 16 bit unaligned |
| [`R_SPARC_TLS_GD_HI22`](#r_sparc_tls_gd_hi22) | const |  |
| [`R_SPARC_TLS_GD_LO10`](#r_sparc_tls_gd_lo10) | const |  |
| [`R_SPARC_TLS_GD_ADD`](#r_sparc_tls_gd_add) | const |  |
| [`R_SPARC_TLS_GD_CALL`](#r_sparc_tls_gd_call) | const |  |
| [`R_SPARC_TLS_LDM_HI22`](#r_sparc_tls_ldm_hi22) | const |  |
| [`R_SPARC_TLS_LDM_LO10`](#r_sparc_tls_ldm_lo10) | const |  |
| [`R_SPARC_TLS_LDM_ADD`](#r_sparc_tls_ldm_add) | const |  |
| [`R_SPARC_TLS_LDM_CALL`](#r_sparc_tls_ldm_call) | const |  |
| [`R_SPARC_TLS_LDO_HIX22`](#r_sparc_tls_ldo_hix22) | const |  |
| [`R_SPARC_TLS_LDO_LOX10`](#r_sparc_tls_ldo_lox10) | const |  |
| [`R_SPARC_TLS_LDO_ADD`](#r_sparc_tls_ldo_add) | const |  |
| [`R_SPARC_TLS_IE_HI22`](#r_sparc_tls_ie_hi22) | const |  |
| [`R_SPARC_TLS_IE_LO10`](#r_sparc_tls_ie_lo10) | const |  |
| [`R_SPARC_TLS_IE_LD`](#r_sparc_tls_ie_ld) | const |  |
| [`R_SPARC_TLS_IE_LDX`](#r_sparc_tls_ie_ldx) | const |  |
| [`R_SPARC_TLS_IE_ADD`](#r_sparc_tls_ie_add) | const |  |
| [`R_SPARC_TLS_LE_HIX22`](#r_sparc_tls_le_hix22) | const |  |
| [`R_SPARC_TLS_LE_LOX10`](#r_sparc_tls_le_lox10) | const |  |
| [`R_SPARC_TLS_DTPMOD32`](#r_sparc_tls_dtpmod32) | const |  |
| [`R_SPARC_TLS_DTPMOD64`](#r_sparc_tls_dtpmod64) | const |  |
| [`R_SPARC_TLS_DTPOFF32`](#r_sparc_tls_dtpoff32) | const |  |
| [`R_SPARC_TLS_DTPOFF64`](#r_sparc_tls_dtpoff64) | const |  |
| [`R_SPARC_TLS_TPOFF32`](#r_sparc_tls_tpoff32) | const |  |
| [`R_SPARC_TLS_TPOFF64`](#r_sparc_tls_tpoff64) | const |  |
| [`R_SPARC_GOTDATA_HIX22`](#r_sparc_gotdata_hix22) | const |  |
| [`R_SPARC_GOTDATA_LOX10`](#r_sparc_gotdata_lox10) | const |  |
| [`R_SPARC_GOTDATA_OP_HIX22`](#r_sparc_gotdata_op_hix22) | const |  |
| [`R_SPARC_GOTDATA_OP_LOX10`](#r_sparc_gotdata_op_lox10) | const |  |
| [`R_SPARC_GOTDATA_OP`](#r_sparc_gotdata_op) | const |  |
| [`R_SPARC_H34`](#r_sparc_h34) | const |  |
| [`R_SPARC_SIZE32`](#r_sparc_size32) | const |  |
| [`R_SPARC_SIZE64`](#r_sparc_size64) | const |  |
| [`R_SPARC_WDISP10`](#r_sparc_wdisp10) | const |  |
| [`R_SPARC_JMP_IREL`](#r_sparc_jmp_irel) | const |  |
| [`R_SPARC_IRELATIVE`](#r_sparc_irelative) | const |  |
| [`R_SPARC_GNU_VTINHERIT`](#r_sparc_gnu_vtinherit) | const |  |
| [`R_SPARC_GNU_VTENTRY`](#r_sparc_gnu_vtentry) | const |  |
| [`R_SPARC_REV32`](#r_sparc_rev32) | const |  |
| [`DT_SPARC_REGISTER`](#dt_sparc_register) | const |  |
| [`EF_MIPS_NOREORDER`](#ef_mips_noreorder) | const | A .noreorder directive was used. |
| [`EF_MIPS_PIC`](#ef_mips_pic) | const | Contains PIC code. |
| [`EF_MIPS_CPIC`](#ef_mips_cpic) | const | Uses PIC calling sequence. |
| [`EF_MIPS_XGOT`](#ef_mips_xgot) | const |  |
| [`EF_MIPS_64BIT_WHIRL`](#ef_mips_64bit_whirl) | const |  |
| [`EF_MIPS_ABI2`](#ef_mips_abi2) | const |  |
| [`EF_MIPS_ABI_ON32`](#ef_mips_abi_on32) | const |  |
| [`EF_MIPS_FP64`](#ef_mips_fp64) | const | Uses FP64 (12 callee-saved). |
| [`EF_MIPS_NAN2008`](#ef_mips_nan2008) | const | Uses IEEE 754-2008 NaN encoding. |
| [`EF_MIPS_ARCH`](#ef_mips_arch) | const | MIPS architecture level. |
| [`EF_MIPS_ABI_O32`](#ef_mips_abi_o32) | const | The first MIPS 32 bit ABI |
| [`EF_MIPS_ABI_O64`](#ef_mips_abi_o64) | const | O32 ABI extended for 64-bit architectures |
| [`EF_MIPS_ABI_EABI32`](#ef_mips_abi_eabi32) | const | EABI in 32-bit mode |
| [`EF_MIPS_ABI_EABI64`](#ef_mips_abi_eabi64) | const | EABI in 64-bit mode |
| [`EF_MIPS_ABI`](#ef_mips_abi) | const | Mask for selecting EF_MIPS_ABI_ variant |
| [`EF_MIPS_ARCH_1`](#ef_mips_arch_1) | const | -mips1 code. |
| [`EF_MIPS_ARCH_2`](#ef_mips_arch_2) | const | -mips2 code. |
| [`EF_MIPS_ARCH_3`](#ef_mips_arch_3) | const | -mips3 code. |
| [`EF_MIPS_ARCH_4`](#ef_mips_arch_4) | const | -mips4 code. |
| [`EF_MIPS_ARCH_5`](#ef_mips_arch_5) | const | -mips5 code. |
| [`EF_MIPS_ARCH_32`](#ef_mips_arch_32) | const | MIPS32 code. |
| [`EF_MIPS_ARCH_64`](#ef_mips_arch_64) | const | MIPS64 code. |
| [`EF_MIPS_ARCH_32R2`](#ef_mips_arch_32r2) | const | MIPS32r2 code. |
| [`EF_MIPS_ARCH_64R2`](#ef_mips_arch_64r2) | const | MIPS64r2 code. |
| [`EF_MIPS_ARCH_32R6`](#ef_mips_arch_32r6) | const | MIPS32r6 code |
| [`EF_MIPS_ARCH_64R6`](#ef_mips_arch_64r6) | const | MIPS64r6 code |
| [`SHN_MIPS_ACOMMON`](#shn_mips_acommon) | const | Allocated common symbols. |
| [`SHN_MIPS_TEXT`](#shn_mips_text) | const | Allocated test symbols. |
| [`SHN_MIPS_DATA`](#shn_mips_data) | const | Allocated data symbols. |
| [`SHN_MIPS_SCOMMON`](#shn_mips_scommon) | const | Small common symbols. |
| [`SHN_MIPS_SUNDEFINED`](#shn_mips_sundefined) | const | Small undefined symbols. |
| [`SHT_MIPS_LIBLIST`](#sht_mips_liblist) | const | Shared objects used in link. |
| [`SHT_MIPS_MSYM`](#sht_mips_msym) | const |  |
| [`SHT_MIPS_CONFLICT`](#sht_mips_conflict) | const | Conflicting symbols. |
| [`SHT_MIPS_GPTAB`](#sht_mips_gptab) | const | Global data area sizes. |
| [`SHT_MIPS_UCODE`](#sht_mips_ucode) | const | Reserved for SGI/MIPS compilers |
| [`SHT_MIPS_DEBUG`](#sht_mips_debug) | const | MIPS ECOFF debugging info. |
| [`SHT_MIPS_REGINFO`](#sht_mips_reginfo) | const | Register usage information. |
| [`SHT_MIPS_PACKAGE`](#sht_mips_package) | const |  |
| [`SHT_MIPS_PACKSYM`](#sht_mips_packsym) | const |  |
| [`SHT_MIPS_RELD`](#sht_mips_reld) | const |  |
| [`SHT_MIPS_IFACE`](#sht_mips_iface) | const |  |
| [`SHT_MIPS_CONTENT`](#sht_mips_content) | const |  |
| [`SHT_MIPS_OPTIONS`](#sht_mips_options) | const | Miscellaneous options. |
| [`SHT_MIPS_SHDR`](#sht_mips_shdr) | const |  |
| [`SHT_MIPS_FDESC`](#sht_mips_fdesc) | const |  |
| [`SHT_MIPS_EXTSYM`](#sht_mips_extsym) | const |  |
| [`SHT_MIPS_DENSE`](#sht_mips_dense) | const |  |
| [`SHT_MIPS_PDESC`](#sht_mips_pdesc) | const |  |
| [`SHT_MIPS_LOCSYM`](#sht_mips_locsym) | const |  |
| [`SHT_MIPS_AUXSYM`](#sht_mips_auxsym) | const |  |
| [`SHT_MIPS_OPTSYM`](#sht_mips_optsym) | const |  |
| [`SHT_MIPS_LOCSTR`](#sht_mips_locstr) | const |  |
| [`SHT_MIPS_LINE`](#sht_mips_line) | const |  |
| [`SHT_MIPS_RFDESC`](#sht_mips_rfdesc) | const |  |
| [`SHT_MIPS_DELTASYM`](#sht_mips_deltasym) | const |  |
| [`SHT_MIPS_DELTAINST`](#sht_mips_deltainst) | const |  |
| [`SHT_MIPS_DELTACLASS`](#sht_mips_deltaclass) | const |  |
| [`SHT_MIPS_DWARF`](#sht_mips_dwarf) | const | DWARF debugging information. |
| [`SHT_MIPS_DELTADECL`](#sht_mips_deltadecl) | const |  |
| [`SHT_MIPS_SYMBOL_LIB`](#sht_mips_symbol_lib) | const |  |
| [`SHT_MIPS_EVENTS`](#sht_mips_events) | const | Event section. |
| [`SHT_MIPS_TRANSLATE`](#sht_mips_translate) | const |  |
| [`SHT_MIPS_PIXIE`](#sht_mips_pixie) | const |  |
| [`SHT_MIPS_XLATE`](#sht_mips_xlate) | const |  |
| [`SHT_MIPS_XLATE_DEBUG`](#sht_mips_xlate_debug) | const |  |
| [`SHT_MIPS_WHIRL`](#sht_mips_whirl) | const |  |
| [`SHT_MIPS_EH_REGION`](#sht_mips_eh_region) | const |  |
| [`SHT_MIPS_XLATE_OLD`](#sht_mips_xlate_old) | const |  |
| [`SHT_MIPS_PDR_EXCEPTION`](#sht_mips_pdr_exception) | const |  |
| [`SHF_MIPS_GPREL`](#shf_mips_gprel) | const | Must be in global data area. |
| [`SHF_MIPS_MERGE`](#shf_mips_merge) | const |  |
| [`SHF_MIPS_ADDR`](#shf_mips_addr) | const |  |
| [`SHF_MIPS_STRINGS`](#shf_mips_strings) | const |  |
| [`SHF_MIPS_NOSTRIP`](#shf_mips_nostrip) | const |  |
| [`SHF_MIPS_LOCAL`](#shf_mips_local) | const |  |
| [`SHF_MIPS_NAMES`](#shf_mips_names) | const |  |
| [`SHF_MIPS_NODUPE`](#shf_mips_nodupe) | const |  |
| [`STO_MIPS_PLT`](#sto_mips_plt) | const |  |
| [`STO_MIPS_SC_ALIGN_UNUSED`](#sto_mips_sc_align_unused) | const | Only valid for `STB_MIPS_SPLIT_COMMON`. |
| [`STB_MIPS_SPLIT_COMMON`](#stb_mips_split_common) | const |  |
| [`ODK_NULL`](#odk_null) | const | Undefined. |
| [`ODK_REGINFO`](#odk_reginfo) | const | Register usage information. |
| [`ODK_EXCEPTIONS`](#odk_exceptions) | const | Exception processing options. |
| [`ODK_PAD`](#odk_pad) | const | Section padding options. |
| [`ODK_HWPATCH`](#odk_hwpatch) | const | Hardware workarounds performed |
| [`ODK_FILL`](#odk_fill) | const | record the fill value used by the linker. |
| [`ODK_TAGS`](#odk_tags) | const | reserve space for desktop tools to write. |
| [`ODK_HWAND`](#odk_hwand) | const | HW workarounds. |
| [`ODK_HWOR`](#odk_hwor) | const | HW workarounds. |
| [`OEX_FPU_MIN`](#oex_fpu_min) | const | FPE's which MUST be enabled. |
| [`OEX_FPU_MAX`](#oex_fpu_max) | const | FPE's which MAY be enabled. |
| [`OEX_PAGE0`](#oex_page0) | const | page zero must be mapped. |
| [`OEX_SMM`](#oex_smm) | const | Force sequential memory mode? |
| [`OEX_FPDBUG`](#oex_fpdbug) | const | Force floating point debug mode? |
| [`OEX_PRECISEFP`](#oex_precisefp) | const |  |
| [`OEX_DISMISS`](#oex_dismiss) | const | Dismiss invalid address faults? |
| [`OEX_FPU_INVAL`](#oex_fpu_inval) | const |  |
| [`OEX_FPU_DIV0`](#oex_fpu_div0) | const |  |
| [`OEX_FPU_OFLO`](#oex_fpu_oflo) | const |  |
| [`OEX_FPU_UFLO`](#oex_fpu_uflo) | const |  |
| [`OEX_FPU_INEX`](#oex_fpu_inex) | const |  |
| [`OHW_R4KEOP`](#ohw_r4keop) | const | R4000 end-of-page patch. |
| [`OHW_R8KPFETCH`](#ohw_r8kpfetch) | const | may need R8000 prefetch patch. |
| [`OHW_R5KEOP`](#ohw_r5keop) | const | R5000 end-of-page patch. |
| [`OHW_R5KCVTL`](#ohw_r5kcvtl) | const | R5000 cvt.\[ds\].l bug. |
| [`OPAD_PREFIX`](#opad_prefix) | const |  |
| [`OPAD_POSTFIX`](#opad_postfix) | const |  |
| [`OPAD_SYMBOL`](#opad_symbol) | const |  |
| [`OHWA0_R4KEOP_CHECKED`](#ohwa0_r4keop_checked) | const |  |
| [`OHWA1_R4KEOP_CLEAN`](#ohwa1_r4keop_clean) | const |  |
| [`R_MIPS_NONE`](#r_mips_none) | const | No reloc |
| [`R_MIPS_16`](#r_mips_16) | const | Direct 16 bit |
| [`R_MIPS_32`](#r_mips_32) | const | Direct 32 bit |
| [`R_MIPS_REL32`](#r_mips_rel32) | const | PC relative 32 bit |
| [`R_MIPS_26`](#r_mips_26) | const | Direct 26 bit shifted |
| [`R_MIPS_HI16`](#r_mips_hi16) | const | High 16 bit |
| [`R_MIPS_LO16`](#r_mips_lo16) | const | Low 16 bit |
| [`R_MIPS_GPREL16`](#r_mips_gprel16) | const | GP relative 16 bit |
| [`R_MIPS_LITERAL`](#r_mips_literal) | const | 16 bit literal entry |
| [`R_MIPS_GOT16`](#r_mips_got16) | const | 16 bit GOT entry |
| [`R_MIPS_PC16`](#r_mips_pc16) | const | PC relative 16 bit |
| [`R_MIPS_CALL16`](#r_mips_call16) | const | 16 bit GOT entry for function |
| [`R_MIPS_GPREL32`](#r_mips_gprel32) | const | GP relative 32 bit |
| [`R_MIPS_SHIFT5`](#r_mips_shift5) | const |  |
| [`R_MIPS_SHIFT6`](#r_mips_shift6) | const |  |
| [`R_MIPS_64`](#r_mips_64) | const |  |
| [`R_MIPS_GOT_DISP`](#r_mips_got_disp) | const |  |
| [`R_MIPS_GOT_PAGE`](#r_mips_got_page) | const |  |
| [`R_MIPS_GOT_OFST`](#r_mips_got_ofst) | const |  |
| [`R_MIPS_GOT_HI16`](#r_mips_got_hi16) | const |  |
| [`R_MIPS_GOT_LO16`](#r_mips_got_lo16) | const |  |
| [`R_MIPS_SUB`](#r_mips_sub) | const |  |
| [`R_MIPS_INSERT_A`](#r_mips_insert_a) | const |  |
| [`R_MIPS_INSERT_B`](#r_mips_insert_b) | const |  |
| [`R_MIPS_DELETE`](#r_mips_delete) | const |  |
| [`R_MIPS_HIGHER`](#r_mips_higher) | const |  |
| [`R_MIPS_HIGHEST`](#r_mips_highest) | const |  |
| [`R_MIPS_CALL_HI16`](#r_mips_call_hi16) | const |  |
| [`R_MIPS_CALL_LO16`](#r_mips_call_lo16) | const |  |
| [`R_MIPS_SCN_DISP`](#r_mips_scn_disp) | const |  |
| [`R_MIPS_REL16`](#r_mips_rel16) | const |  |
| [`R_MIPS_ADD_IMMEDIATE`](#r_mips_add_immediate) | const |  |
| [`R_MIPS_PJUMP`](#r_mips_pjump) | const |  |
| [`R_MIPS_RELGOT`](#r_mips_relgot) | const |  |
| [`R_MIPS_JALR`](#r_mips_jalr) | const |  |
| [`R_MIPS_TLS_DTPMOD32`](#r_mips_tls_dtpmod32) | const | Module number 32 bit |
| [`R_MIPS_TLS_DTPREL32`](#r_mips_tls_dtprel32) | const | Module-relative offset 32 bit |
| [`R_MIPS_TLS_DTPMOD64`](#r_mips_tls_dtpmod64) | const | Module number 64 bit |
| [`R_MIPS_TLS_DTPREL64`](#r_mips_tls_dtprel64) | const | Module-relative offset 64 bit |
| [`R_MIPS_TLS_GD`](#r_mips_tls_gd) | const | 16 bit GOT offset for GD |
| [`R_MIPS_TLS_LDM`](#r_mips_tls_ldm) | const | 16 bit GOT offset for LDM |
| [`R_MIPS_TLS_DTPREL_HI16`](#r_mips_tls_dtprel_hi16) | const | Module-relative offset, high 16 bits |
| [`R_MIPS_TLS_DTPREL_LO16`](#r_mips_tls_dtprel_lo16) | const | Module-relative offset, low 16 bits |
| [`R_MIPS_TLS_GOTTPREL`](#r_mips_tls_gottprel) | const | 16 bit GOT offset for IE |
| [`R_MIPS_TLS_TPREL32`](#r_mips_tls_tprel32) | const | TP-relative offset, 32 bit |
| [`R_MIPS_TLS_TPREL64`](#r_mips_tls_tprel64) | const | TP-relative offset, 64 bit |
| [`R_MIPS_TLS_TPREL_HI16`](#r_mips_tls_tprel_hi16) | const | TP-relative offset, high 16 bits |
| [`R_MIPS_TLS_TPREL_LO16`](#r_mips_tls_tprel_lo16) | const | TP-relative offset, low 16 bits |
| [`R_MIPS_GLOB_DAT`](#r_mips_glob_dat) | const |  |
| [`R_MIPS_COPY`](#r_mips_copy) | const |  |
| [`R_MIPS_JUMP_SLOT`](#r_mips_jump_slot) | const |  |
| [`PT_MIPS_REGINFO`](#pt_mips_reginfo) | const | Register usage information. |
| [`PT_MIPS_RTPROC`](#pt_mips_rtproc) | const | Runtime procedure table. |
| [`PT_MIPS_OPTIONS`](#pt_mips_options) | const |  |
| [`PT_MIPS_ABIFLAGS`](#pt_mips_abiflags) | const | FP mode requirement. |
| [`PF_MIPS_LOCAL`](#pf_mips_local) | const |  |
| [`DT_MIPS_RLD_VERSION`](#dt_mips_rld_version) | const | Runtime linker interface version |
| [`DT_MIPS_TIME_STAMP`](#dt_mips_time_stamp) | const | Timestamp |
| [`DT_MIPS_ICHECKSUM`](#dt_mips_ichecksum) | const | Checksum |
| [`DT_MIPS_IVERSION`](#dt_mips_iversion) | const | Version string (string tbl index) |
| [`DT_MIPS_FLAGS`](#dt_mips_flags) | const | Flags |
| [`DT_MIPS_BASE_ADDRESS`](#dt_mips_base_address) | const | Base address |
| [`DT_MIPS_MSYM`](#dt_mips_msym) | const |  |
| [`DT_MIPS_CONFLICT`](#dt_mips_conflict) | const | Address of CONFLICT section |
| [`DT_MIPS_LIBLIST`](#dt_mips_liblist) | const | Address of LIBLIST section |
| [`DT_MIPS_LOCAL_GOTNO`](#dt_mips_local_gotno) | const | Number of local GOT entries |
| [`DT_MIPS_CONFLICTNO`](#dt_mips_conflictno) | const | Number of CONFLICT entries |
| [`DT_MIPS_LIBLISTNO`](#dt_mips_liblistno) | const | Number of LIBLIST entries |
| [`DT_MIPS_SYMTABNO`](#dt_mips_symtabno) | const | Number of DYNSYM entries |
| [`DT_MIPS_UNREFEXTNO`](#dt_mips_unrefextno) | const | First external DYNSYM |
| [`DT_MIPS_GOTSYM`](#dt_mips_gotsym) | const | First GOT entry in DYNSYM |
| [`DT_MIPS_HIPAGENO`](#dt_mips_hipageno) | const | Number of GOT page table entries |
| [`DT_MIPS_RLD_MAP`](#dt_mips_rld_map) | const | Address of run time loader map. |
| [`DT_MIPS_DELTA_CLASS`](#dt_mips_delta_class) | const | Delta C++ class definition. |
| [`DT_MIPS_DELTA_CLASS_NO`](#dt_mips_delta_class_no) | const | Number of entries in DT_MIPS_DELTA_CLASS. |
| [`DT_MIPS_DELTA_INSTANCE`](#dt_mips_delta_instance) | const | Delta C++ class instances. |
| [`DT_MIPS_DELTA_INSTANCE_NO`](#dt_mips_delta_instance_no) | const | Number of entries in DT_MIPS_DELTA_INSTANCE. |
| [`DT_MIPS_DELTA_RELOC`](#dt_mips_delta_reloc) | const | Delta relocations. |
| [`DT_MIPS_DELTA_RELOC_NO`](#dt_mips_delta_reloc_no) | const | Number of entries in DT_MIPS_DELTA_RELOC. |
| [`DT_MIPS_DELTA_SYM`](#dt_mips_delta_sym) | const | Delta symbols that Delta relocations refer to. |
| [`DT_MIPS_DELTA_SYM_NO`](#dt_mips_delta_sym_no) | const | Number of entries in DT_MIPS_DELTA_SYM. |
| [`DT_MIPS_DELTA_CLASSSYM`](#dt_mips_delta_classsym) | const | Delta symbols that hold the class declaration. |
| [`DT_MIPS_DELTA_CLASSSYM_NO`](#dt_mips_delta_classsym_no) | const | Number of entries in DT_MIPS_DELTA_CLASSSYM. |
| [`DT_MIPS_CXX_FLAGS`](#dt_mips_cxx_flags) | const | Flags indicating for C++ flavor. |
| [`DT_MIPS_PIXIE_INIT`](#dt_mips_pixie_init) | const |  |
| [`DT_MIPS_SYMBOL_LIB`](#dt_mips_symbol_lib) | const |  |
| [`DT_MIPS_LOCALPAGE_GOTIDX`](#dt_mips_localpage_gotidx) | const |  |
| [`DT_MIPS_LOCAL_GOTIDX`](#dt_mips_local_gotidx) | const |  |
| [`DT_MIPS_HIDDEN_GOTIDX`](#dt_mips_hidden_gotidx) | const |  |
| [`DT_MIPS_PROTECTED_GOTIDX`](#dt_mips_protected_gotidx) | const |  |
| [`DT_MIPS_OPTIONS`](#dt_mips_options) | const | Address of .options. |
| [`DT_MIPS_INTERFACE`](#dt_mips_interface) | const | Address of .interface. |
| [`DT_MIPS_DYNSTR_ALIGN`](#dt_mips_dynstr_align) | const |  |
| [`DT_MIPS_INTERFACE_SIZE`](#dt_mips_interface_size) | const | Size of the .interface section. |
| [`DT_MIPS_RLD_TEXT_RESOLVE_ADDR`](#dt_mips_rld_text_resolve_addr) | const | Address of rld_text_rsolve function stored in GOT. |
| [`DT_MIPS_PERF_SUFFIX`](#dt_mips_perf_suffix) | const | Default suffix of dso to be added by rld on dlopen() calls. |
| [`DT_MIPS_COMPACT_SIZE`](#dt_mips_compact_size) | const | (O32)Size of compact rel section. |
| [`DT_MIPS_GP_VALUE`](#dt_mips_gp_value) | const | GP value for aux GOTs. |
| [`DT_MIPS_AUX_DYNAMIC`](#dt_mips_aux_dynamic) | const | Address of aux .dynamic. |
| [`DT_MIPS_PLTGOT`](#dt_mips_pltgot) | const | The address of .got.plt in an executable using the new non-PIC ABI. |
| [`DT_MIPS_RWPLT`](#dt_mips_rwplt) | const | The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable. |
| [`DT_MIPS_RLD_MAP_REL`](#dt_mips_rld_map_rel) | const | An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address. |
| [`RHF_NONE`](#rhf_none) | const | No flags |
| [`RHF_QUICKSTART`](#rhf_quickstart) | const | Use quickstart |
| [`RHF_NOTPOT`](#rhf_notpot) | const | Hash size not power of 2 |
| [`RHF_NO_LIBRARY_REPLACEMENT`](#rhf_no_library_replacement) | const | Ignore LD_LIBRARY_PATH |
| [`RHF_NO_MOVE`](#rhf_no_move) | const |  |
| [`RHF_SGI_ONLY`](#rhf_sgi_only) | const |  |
| [`RHF_GUARANTEE_INIT`](#rhf_guarantee_init) | const |  |
| [`RHF_DELTA_C_PLUS_PLUS`](#rhf_delta_c_plus_plus) | const |  |
| [`RHF_GUARANTEE_START_INIT`](#rhf_guarantee_start_init) | const |  |
| [`RHF_PIXIE`](#rhf_pixie) | const |  |
| [`RHF_DEFAULT_DELAY_LOAD`](#rhf_default_delay_load) | const |  |
| [`RHF_REQUICKSTART`](#rhf_requickstart) | const |  |
| [`RHF_REQUICKSTARTED`](#rhf_requickstarted) | const |  |
| [`RHF_CORD`](#rhf_cord) | const |  |
| [`RHF_NO_UNRES_UNDEF`](#rhf_no_unres_undef) | const |  |
| [`RHF_RLD_ORDER_SAFE`](#rhf_rld_order_safe) | const |  |
| [`LL_NONE`](#ll_none) | const |  |
| [`LL_EXACT_MATCH`](#ll_exact_match) | const | Require exact match |
| [`LL_IGNORE_INT_VER`](#ll_ignore_int_ver) | const | Ignore interface version |
| [`LL_REQUIRE_MINOR`](#ll_require_minor) | const |  |
| [`LL_EXPORTS`](#ll_exports) | const |  |
| [`LL_DELAY_LOAD`](#ll_delay_load) | const |  |
| [`LL_DELTA`](#ll_delta) | const |  |
| [`EF_PARISC_TRAPNIL`](#ef_parisc_trapnil) | const | Trap nil pointer dereference. |
| [`EF_PARISC_EXT`](#ef_parisc_ext) | const | Program uses arch. |
| [`EF_PARISC_LSB`](#ef_parisc_lsb) | const | Program expects little endian. |
| [`EF_PARISC_WIDE`](#ef_parisc_wide) | const | Program expects wide mode. |
| [`EF_PARISC_NO_KABP`](#ef_parisc_no_kabp) | const | No kernel assisted branch prediction. |
| [`EF_PARISC_LAZYSWAP`](#ef_parisc_lazyswap) | const | Allow lazy swapping. |
| [`EF_PARISC_ARCH`](#ef_parisc_arch) | const | Architecture version. |
| [`EFA_PARISC_1_0`](#efa_parisc_1_0) | const | PA-RISC 1.0 big-endian. |
| [`EFA_PARISC_1_1`](#efa_parisc_1_1) | const | PA-RISC 1.1 big-endian. |
| [`EFA_PARISC_2_0`](#efa_parisc_2_0) | const | PA-RISC 2.0 big-endian. |
| [`SHN_PARISC_ANSI_COMMON`](#shn_parisc_ansi_common) | const | Section for tentatively declared symbols in ANSI C. |
| [`SHN_PARISC_HUGE_COMMON`](#shn_parisc_huge_common) | const | Common blocks in huge model. |
| [`SHT_PARISC_EXT`](#sht_parisc_ext) | const | Contains product specific ext. |
| [`SHT_PARISC_UNWIND`](#sht_parisc_unwind) | const | Unwind information. |
| [`SHT_PARISC_DOC`](#sht_parisc_doc) | const | Debug info for optimized code. |
| [`SHF_PARISC_SHORT`](#shf_parisc_short) | const | Section with short addressing. |
| [`SHF_PARISC_HUGE`](#shf_parisc_huge) | const | Section far from gp. |
| [`SHF_PARISC_SBP`](#shf_parisc_sbp) | const | Static branch prediction code. |
| [`STT_PARISC_MILLICODE`](#stt_parisc_millicode) | const | Millicode function entry point. |
| [`STT_HP_OPAQUE`](#stt_hp_opaque) | const |  |
| [`STT_HP_STUB`](#stt_hp_stub) | const |  |
| [`R_PARISC_NONE`](#r_parisc_none) | const | No reloc. |
| [`R_PARISC_DIR32`](#r_parisc_dir32) | const | Direct 32-bit reference. |
| [`R_PARISC_DIR21L`](#r_parisc_dir21l) | const | Left 21 bits of eff. |
| [`R_PARISC_DIR17R`](#r_parisc_dir17r) | const | Right 17 bits of eff. |
| [`R_PARISC_DIR17F`](#r_parisc_dir17f) | const | 17 bits of eff. |
| [`R_PARISC_DIR14R`](#r_parisc_dir14r) | const | Right 14 bits of eff. |
| [`R_PARISC_PCREL32`](#r_parisc_pcrel32) | const | 32-bit rel. |
| [`R_PARISC_PCREL21L`](#r_parisc_pcrel21l) | const | Left 21 bits of rel. |
| [`R_PARISC_PCREL17R`](#r_parisc_pcrel17r) | const | Right 17 bits of rel. |
| [`R_PARISC_PCREL17F`](#r_parisc_pcrel17f) | const | 17 bits of rel. |
| [`R_PARISC_PCREL14R`](#r_parisc_pcrel14r) | const | Right 14 bits of rel. |
| [`R_PARISC_DPREL21L`](#r_parisc_dprel21l) | const | Left 21 bits of rel. |
| [`R_PARISC_DPREL14R`](#r_parisc_dprel14r) | const | Right 14 bits of rel. |
| [`R_PARISC_GPREL21L`](#r_parisc_gprel21l) | const | GP-relative, left 21 bits. |
| [`R_PARISC_GPREL14R`](#r_parisc_gprel14r) | const | GP-relative, right 14 bits. |
| [`R_PARISC_LTOFF21L`](#r_parisc_ltoff21l) | const | LT-relative, left 21 bits. |
| [`R_PARISC_LTOFF14R`](#r_parisc_ltoff14r) | const | LT-relative, right 14 bits. |
| [`R_PARISC_SECREL32`](#r_parisc_secrel32) | const | 32 bits section rel. |
| [`R_PARISC_SEGBASE`](#r_parisc_segbase) | const | No relocation, set segment base. |
| [`R_PARISC_SEGREL32`](#r_parisc_segrel32) | const | 32 bits segment rel. |
| [`R_PARISC_PLTOFF21L`](#r_parisc_pltoff21l) | const | PLT rel. |
| [`R_PARISC_PLTOFF14R`](#r_parisc_pltoff14r) | const | PLT rel. |
| [`R_PARISC_LTOFF_FPTR32`](#r_parisc_ltoff_fptr32) | const | 32 bits LT-rel. |
| [`R_PARISC_LTOFF_FPTR21L`](#r_parisc_ltoff_fptr21l) | const | LT-rel. |
| [`R_PARISC_LTOFF_FPTR14R`](#r_parisc_ltoff_fptr14r) | const | LT-rel. |
| [`R_PARISC_FPTR64`](#r_parisc_fptr64) | const | 64 bits function address. |
| [`R_PARISC_PLABEL32`](#r_parisc_plabel32) | const | 32 bits function address. |
| [`R_PARISC_PLABEL21L`](#r_parisc_plabel21l) | const | Left 21 bits of fdesc address. |
| [`R_PARISC_PLABEL14R`](#r_parisc_plabel14r) | const | Right 14 bits of fdesc address. |
| [`R_PARISC_PCREL64`](#r_parisc_pcrel64) | const | 64 bits PC-rel. |
| [`R_PARISC_PCREL22F`](#r_parisc_pcrel22f) | const | 22 bits PC-rel. |
| [`R_PARISC_PCREL14WR`](#r_parisc_pcrel14wr) | const | PC-rel. |
| [`R_PARISC_PCREL14DR`](#r_parisc_pcrel14dr) | const | PC rel. |
| [`R_PARISC_PCREL16F`](#r_parisc_pcrel16f) | const | 16 bits PC-rel. |
| [`R_PARISC_PCREL16WF`](#r_parisc_pcrel16wf) | const | 16 bits PC-rel. |
| [`R_PARISC_PCREL16DF`](#r_parisc_pcrel16df) | const | 16 bits PC-rel. |
| [`R_PARISC_DIR64`](#r_parisc_dir64) | const | 64 bits of eff. |
| [`R_PARISC_DIR14WR`](#r_parisc_dir14wr) | const | 14 bits of eff. |
| [`R_PARISC_DIR14DR`](#r_parisc_dir14dr) | const | 14 bits of eff. |
| [`R_PARISC_DIR16F`](#r_parisc_dir16f) | const | 16 bits of eff. |
| [`R_PARISC_DIR16WF`](#r_parisc_dir16wf) | const | 16 bits of eff. |
| [`R_PARISC_DIR16DF`](#r_parisc_dir16df) | const | 16 bits of eff. |
| [`R_PARISC_GPREL64`](#r_parisc_gprel64) | const | 64 bits of GP-rel. |
| [`R_PARISC_GPREL14WR`](#r_parisc_gprel14wr) | const | GP-rel. |
| [`R_PARISC_GPREL14DR`](#r_parisc_gprel14dr) | const | GP-rel. |
| [`R_PARISC_GPREL16F`](#r_parisc_gprel16f) | const | 16 bits GP-rel. |
| [`R_PARISC_GPREL16WF`](#r_parisc_gprel16wf) | const | 16 bits GP-rel. |
| [`R_PARISC_GPREL16DF`](#r_parisc_gprel16df) | const | 16 bits GP-rel. |
| [`R_PARISC_LTOFF64`](#r_parisc_ltoff64) | const | 64 bits LT-rel. |
| [`R_PARISC_LTOFF14WR`](#r_parisc_ltoff14wr) | const | LT-rel. |
| [`R_PARISC_LTOFF14DR`](#r_parisc_ltoff14dr) | const | LT-rel. |
| [`R_PARISC_LTOFF16F`](#r_parisc_ltoff16f) | const | 16 bits LT-rel. |
| [`R_PARISC_LTOFF16WF`](#r_parisc_ltoff16wf) | const | 16 bits LT-rel. |
| [`R_PARISC_LTOFF16DF`](#r_parisc_ltoff16df) | const | 16 bits LT-rel. |
| [`R_PARISC_SECREL64`](#r_parisc_secrel64) | const | 64 bits section rel. |
| [`R_PARISC_SEGREL64`](#r_parisc_segrel64) | const | 64 bits segment rel. |
| [`R_PARISC_PLTOFF14WR`](#r_parisc_pltoff14wr) | const | PLT-rel. |
| [`R_PARISC_PLTOFF14DR`](#r_parisc_pltoff14dr) | const | PLT-rel. |
| [`R_PARISC_PLTOFF16F`](#r_parisc_pltoff16f) | const | 16 bits LT-rel. |
| [`R_PARISC_PLTOFF16WF`](#r_parisc_pltoff16wf) | const | 16 bits PLT-rel. |
| [`R_PARISC_PLTOFF16DF`](#r_parisc_pltoff16df) | const | 16 bits PLT-rel. |
| [`R_PARISC_LTOFF_FPTR64`](#r_parisc_ltoff_fptr64) | const | 64 bits LT-rel. |
| [`R_PARISC_LTOFF_FPTR14WR`](#r_parisc_ltoff_fptr14wr) | const | LT-rel. |
| [`R_PARISC_LTOFF_FPTR14DR`](#r_parisc_ltoff_fptr14dr) | const | LT-rel. |
| [`R_PARISC_LTOFF_FPTR16F`](#r_parisc_ltoff_fptr16f) | const | 16 bits LT-rel. |
| [`R_PARISC_LTOFF_FPTR16WF`](#r_parisc_ltoff_fptr16wf) | const | 16 bits LT-rel. |
| [`R_PARISC_LTOFF_FPTR16DF`](#r_parisc_ltoff_fptr16df) | const | 16 bits LT-rel. |
| [`R_PARISC_LORESERVE`](#r_parisc_loreserve) | const |  |
| [`R_PARISC_COPY`](#r_parisc_copy) | const | Copy relocation. |
| [`R_PARISC_IPLT`](#r_parisc_iplt) | const | Dynamic reloc, imported PLT |
| [`R_PARISC_EPLT`](#r_parisc_eplt) | const | Dynamic reloc, exported PLT |
| [`R_PARISC_TPREL32`](#r_parisc_tprel32) | const | 32 bits TP-rel. |
| [`R_PARISC_TPREL21L`](#r_parisc_tprel21l) | const | TP-rel. |
| [`R_PARISC_TPREL14R`](#r_parisc_tprel14r) | const | TP-rel. |
| [`R_PARISC_LTOFF_TP21L`](#r_parisc_ltoff_tp21l) | const | LT-TP-rel. |
| [`R_PARISC_LTOFF_TP14R`](#r_parisc_ltoff_tp14r) | const | LT-TP-rel. |
| [`R_PARISC_LTOFF_TP14F`](#r_parisc_ltoff_tp14f) | const | 14 bits LT-TP-rel. |
| [`R_PARISC_TPREL64`](#r_parisc_tprel64) | const | 64 bits TP-rel. |
| [`R_PARISC_TPREL14WR`](#r_parisc_tprel14wr) | const | TP-rel. |
| [`R_PARISC_TPREL14DR`](#r_parisc_tprel14dr) | const | TP-rel. |
| [`R_PARISC_TPREL16F`](#r_parisc_tprel16f) | const | 16 bits TP-rel. |
| [`R_PARISC_TPREL16WF`](#r_parisc_tprel16wf) | const | 16 bits TP-rel. |
| [`R_PARISC_TPREL16DF`](#r_parisc_tprel16df) | const | 16 bits TP-rel. |
| [`R_PARISC_LTOFF_TP64`](#r_parisc_ltoff_tp64) | const | 64 bits LT-TP-rel. |
| [`R_PARISC_LTOFF_TP14WR`](#r_parisc_ltoff_tp14wr) | const | LT-TP-rel. |
| [`R_PARISC_LTOFF_TP14DR`](#r_parisc_ltoff_tp14dr) | const | LT-TP-rel. |
| [`R_PARISC_LTOFF_TP16F`](#r_parisc_ltoff_tp16f) | const | 16 bits LT-TP-rel. |
| [`R_PARISC_LTOFF_TP16WF`](#r_parisc_ltoff_tp16wf) | const | 16 bits LT-TP-rel. |
| [`R_PARISC_LTOFF_TP16DF`](#r_parisc_ltoff_tp16df) | const | 16 bits LT-TP-rel. |
| [`R_PARISC_GNU_VTENTRY`](#r_parisc_gnu_vtentry) | const |  |
| [`R_PARISC_GNU_VTINHERIT`](#r_parisc_gnu_vtinherit) | const |  |
| [`R_PARISC_TLS_GD21L`](#r_parisc_tls_gd21l) | const | GD 21-bit left. |
| [`R_PARISC_TLS_GD14R`](#r_parisc_tls_gd14r) | const | GD 14-bit right. |
| [`R_PARISC_TLS_GDCALL`](#r_parisc_tls_gdcall) | const | GD call to __t_g_a. |
| [`R_PARISC_TLS_LDM21L`](#r_parisc_tls_ldm21l) | const | LD module 21-bit left. |
| [`R_PARISC_TLS_LDM14R`](#r_parisc_tls_ldm14r) | const | LD module 14-bit right. |
| [`R_PARISC_TLS_LDMCALL`](#r_parisc_tls_ldmcall) | const | LD module call to __t_g_a. |
| [`R_PARISC_TLS_LDO21L`](#r_parisc_tls_ldo21l) | const | LD offset 21-bit left. |
| [`R_PARISC_TLS_LDO14R`](#r_parisc_tls_ldo14r) | const | LD offset 14-bit right. |
| [`R_PARISC_TLS_DTPMOD32`](#r_parisc_tls_dtpmod32) | const | DTP module 32-bit. |
| [`R_PARISC_TLS_DTPMOD64`](#r_parisc_tls_dtpmod64) | const | DTP module 64-bit. |
| [`R_PARISC_TLS_DTPOFF32`](#r_parisc_tls_dtpoff32) | const | DTP offset 32-bit. |
| [`R_PARISC_TLS_DTPOFF64`](#r_parisc_tls_dtpoff64) | const | DTP offset 32-bit. |
| [`R_PARISC_TLS_LE21L`](#r_parisc_tls_le21l) | const |  |
| [`R_PARISC_TLS_LE14R`](#r_parisc_tls_le14r) | const |  |
| [`R_PARISC_TLS_IE21L`](#r_parisc_tls_ie21l) | const |  |
| [`R_PARISC_TLS_IE14R`](#r_parisc_tls_ie14r) | const |  |
| [`R_PARISC_TLS_TPREL32`](#r_parisc_tls_tprel32) | const |  |
| [`R_PARISC_TLS_TPREL64`](#r_parisc_tls_tprel64) | const |  |
| [`R_PARISC_HIRESERVE`](#r_parisc_hireserve) | const |  |
| [`PT_HP_TLS`](#pt_hp_tls) | const |  |
| [`PT_HP_CORE_NONE`](#pt_hp_core_none) | const |  |
| [`PT_HP_CORE_VERSION`](#pt_hp_core_version) | const |  |
| [`PT_HP_CORE_KERNEL`](#pt_hp_core_kernel) | const |  |
| [`PT_HP_CORE_COMM`](#pt_hp_core_comm) | const |  |
| [`PT_HP_CORE_PROC`](#pt_hp_core_proc) | const |  |
| [`PT_HP_CORE_LOADABLE`](#pt_hp_core_loadable) | const |  |
| [`PT_HP_CORE_STACK`](#pt_hp_core_stack) | const |  |
| [`PT_HP_CORE_SHM`](#pt_hp_core_shm) | const |  |
| [`PT_HP_CORE_MMF`](#pt_hp_core_mmf) | const |  |
| [`PT_HP_PARALLEL`](#pt_hp_parallel) | const |  |
| [`PT_HP_FASTBIND`](#pt_hp_fastbind) | const |  |
| [`PT_HP_OPT_ANNOT`](#pt_hp_opt_annot) | const |  |
| [`PT_HP_HSL_ANNOT`](#pt_hp_hsl_annot) | const |  |
| [`PT_HP_STACK`](#pt_hp_stack) | const |  |
| [`PT_PARISC_ARCHEXT`](#pt_parisc_archext) | const |  |
| [`PT_PARISC_UNWIND`](#pt_parisc_unwind) | const |  |
| [`PF_PARISC_SBP`](#pf_parisc_sbp) | const |  |
| [`PF_HP_PAGE_SIZE`](#pf_hp_page_size) | const |  |
| [`PF_HP_FAR_SHARED`](#pf_hp_far_shared) | const |  |
| [`PF_HP_NEAR_SHARED`](#pf_hp_near_shared) | const |  |
| [`PF_HP_CODE`](#pf_hp_code) | const |  |
| [`PF_HP_MODIFY`](#pf_hp_modify) | const |  |
| [`PF_HP_LAZYSWAP`](#pf_hp_lazyswap) | const |  |
| [`PF_HP_SBP`](#pf_hp_sbp) | const |  |
| [`EF_ALPHA_32BIT`](#ef_alpha_32bit) | const | All addresses must be < 2GB. |
| [`EF_ALPHA_CANRELAX`](#ef_alpha_canrelax) | const | Relocations for relaxing exist. |
| [`SHT_ALPHA_DEBUG`](#sht_alpha_debug) | const |  |
| [`SHT_ALPHA_REGINFO`](#sht_alpha_reginfo) | const |  |
| [`SHF_ALPHA_GPREL`](#shf_alpha_gprel) | const |  |
| [`STO_ALPHA_NOPV`](#sto_alpha_nopv) | const | No PV required. |
| [`STO_ALPHA_STD_GPLOAD`](#sto_alpha_std_gpload) | const | PV only used for initial ldgp. |
| [`R_ALPHA_NONE`](#r_alpha_none) | const | No reloc |
| [`R_ALPHA_REFLONG`](#r_alpha_reflong) | const | Direct 32 bit |
| [`R_ALPHA_REFQUAD`](#r_alpha_refquad) | const | Direct 64 bit |
| [`R_ALPHA_GPREL32`](#r_alpha_gprel32) | const | GP relative 32 bit |
| [`R_ALPHA_LITERAL`](#r_alpha_literal) | const | GP relative 16 bit w/optimization |
| [`R_ALPHA_LITUSE`](#r_alpha_lituse) | const | Optimization hint for LITERAL |
| [`R_ALPHA_GPDISP`](#r_alpha_gpdisp) | const | Add displacement to GP |
| [`R_ALPHA_BRADDR`](#r_alpha_braddr) | const | PC+4 relative 23 bit shifted |
| [`R_ALPHA_HINT`](#r_alpha_hint) | const | PC+4 relative 16 bit shifted |
| [`R_ALPHA_SREL16`](#r_alpha_srel16) | const | PC relative 16 bit |
| [`R_ALPHA_SREL32`](#r_alpha_srel32) | const | PC relative 32 bit |
| [`R_ALPHA_SREL64`](#r_alpha_srel64) | const | PC relative 64 bit |
| [`R_ALPHA_GPRELHIGH`](#r_alpha_gprelhigh) | const | GP relative 32 bit, high 16 bits |
| [`R_ALPHA_GPRELLOW`](#r_alpha_gprellow) | const | GP relative 32 bit, low 16 bits |
| [`R_ALPHA_GPREL16`](#r_alpha_gprel16) | const | GP relative 16 bit |
| [`R_ALPHA_COPY`](#r_alpha_copy) | const | Copy symbol at runtime |
| [`R_ALPHA_GLOB_DAT`](#r_alpha_glob_dat) | const | Create GOT entry |
| [`R_ALPHA_JMP_SLOT`](#r_alpha_jmp_slot) | const | Create PLT entry |
| [`R_ALPHA_RELATIVE`](#r_alpha_relative) | const | Adjust by program base |
| [`R_ALPHA_TLS_GD_HI`](#r_alpha_tls_gd_hi) | const |  |
| [`R_ALPHA_TLSGD`](#r_alpha_tlsgd) | const |  |
| [`R_ALPHA_TLS_LDM`](#r_alpha_tls_ldm) | const |  |
| [`R_ALPHA_DTPMOD64`](#r_alpha_dtpmod64) | const |  |
| [`R_ALPHA_GOTDTPREL`](#r_alpha_gotdtprel) | const |  |
| [`R_ALPHA_DTPREL64`](#r_alpha_dtprel64) | const |  |
| [`R_ALPHA_DTPRELHI`](#r_alpha_dtprelhi) | const |  |
| [`R_ALPHA_DTPRELLO`](#r_alpha_dtprello) | const |  |
| [`R_ALPHA_DTPREL16`](#r_alpha_dtprel16) | const |  |
| [`R_ALPHA_GOTTPREL`](#r_alpha_gottprel) | const |  |
| [`R_ALPHA_TPREL64`](#r_alpha_tprel64) | const |  |
| [`R_ALPHA_TPRELHI`](#r_alpha_tprelhi) | const |  |
| [`R_ALPHA_TPRELLO`](#r_alpha_tprello) | const |  |
| [`R_ALPHA_TPREL16`](#r_alpha_tprel16) | const |  |
| [`LITUSE_ALPHA_ADDR`](#lituse_alpha_addr) | const |  |
| [`LITUSE_ALPHA_BASE`](#lituse_alpha_base) | const |  |
| [`LITUSE_ALPHA_BYTOFF`](#lituse_alpha_bytoff) | const |  |
| [`LITUSE_ALPHA_JSR`](#lituse_alpha_jsr) | const |  |
| [`LITUSE_ALPHA_TLS_GD`](#lituse_alpha_tls_gd) | const |  |
| [`LITUSE_ALPHA_TLS_LDM`](#lituse_alpha_tls_ldm) | const |  |
| [`DT_ALPHA_PLTRO`](#dt_alpha_pltro) | const |  |
| [`EF_PPC_EMB`](#ef_ppc_emb) | const | PowerPC embedded flag |
| [`EF_PPC_RELOCATABLE`](#ef_ppc_relocatable) | const | PowerPC -mrelocatable flag |
| [`EF_PPC_RELOCATABLE_LIB`](#ef_ppc_relocatable_lib) | const | PowerPC -mrelocatable-lib flag |
| [`R_PPC_NONE`](#r_ppc_none) | const |  |
| [`R_PPC_ADDR32`](#r_ppc_addr32) | const | 32bit absolute address |
| [`R_PPC_ADDR24`](#r_ppc_addr24) | const | 26bit address, 2 bits ignored. |
| [`R_PPC_ADDR16`](#r_ppc_addr16) | const | 16bit absolute address |
| [`R_PPC_ADDR16_LO`](#r_ppc_addr16_lo) | const | lower 16bit of absolute address |
| [`R_PPC_ADDR16_HI`](#r_ppc_addr16_hi) | const | high 16bit of absolute address |
| [`R_PPC_ADDR16_HA`](#r_ppc_addr16_ha) | const | adjusted high 16bit |
| [`R_PPC_ADDR14`](#r_ppc_addr14) | const | 16bit address, 2 bits ignored |
| [`R_PPC_ADDR14_BRTAKEN`](#r_ppc_addr14_brtaken) | const |  |
| [`R_PPC_ADDR14_BRNTAKEN`](#r_ppc_addr14_brntaken) | const |  |
| [`R_PPC_REL24`](#r_ppc_rel24) | const | PC relative 26 bit |
| [`R_PPC_REL14`](#r_ppc_rel14) | const | PC relative 16 bit |
| [`R_PPC_REL14_BRTAKEN`](#r_ppc_rel14_brtaken) | const |  |
| [`R_PPC_REL14_BRNTAKEN`](#r_ppc_rel14_brntaken) | const |  |
| [`R_PPC_GOT16`](#r_ppc_got16) | const |  |
| [`R_PPC_GOT16_LO`](#r_ppc_got16_lo) | const |  |
| [`R_PPC_GOT16_HI`](#r_ppc_got16_hi) | const |  |
| [`R_PPC_GOT16_HA`](#r_ppc_got16_ha) | const |  |
| [`R_PPC_PLTREL24`](#r_ppc_pltrel24) | const |  |
| [`R_PPC_COPY`](#r_ppc_copy) | const |  |
| [`R_PPC_GLOB_DAT`](#r_ppc_glob_dat) | const |  |
| [`R_PPC_JMP_SLOT`](#r_ppc_jmp_slot) | const |  |
| [`R_PPC_RELATIVE`](#r_ppc_relative) | const |  |
| [`R_PPC_LOCAL24PC`](#r_ppc_local24pc) | const |  |
| [`R_PPC_UADDR32`](#r_ppc_uaddr32) | const |  |
| [`R_PPC_UADDR16`](#r_ppc_uaddr16) | const |  |
| [`R_PPC_REL32`](#r_ppc_rel32) | const |  |
| [`R_PPC_PLT32`](#r_ppc_plt32) | const |  |
| [`R_PPC_PLTREL32`](#r_ppc_pltrel32) | const |  |
| [`R_PPC_PLT16_LO`](#r_ppc_plt16_lo) | const |  |
| [`R_PPC_PLT16_HI`](#r_ppc_plt16_hi) | const |  |
| [`R_PPC_PLT16_HA`](#r_ppc_plt16_ha) | const |  |
| [`R_PPC_SDAREL16`](#r_ppc_sdarel16) | const |  |
| [`R_PPC_SECTOFF`](#r_ppc_sectoff) | const |  |
| [`R_PPC_SECTOFF_LO`](#r_ppc_sectoff_lo) | const |  |
| [`R_PPC_SECTOFF_HI`](#r_ppc_sectoff_hi) | const |  |
| [`R_PPC_SECTOFF_HA`](#r_ppc_sectoff_ha) | const |  |
| [`R_PPC_TLS`](#r_ppc_tls) | const | none    (sym+add)@tls |
| [`R_PPC_DTPMOD32`](#r_ppc_dtpmod32) | const | word32  (sym+add)@dtpmod |
| [`R_PPC_TPREL16`](#r_ppc_tprel16) | const | half16* (sym+add)@tprel |
| [`R_PPC_TPREL16_LO`](#r_ppc_tprel16_lo) | const | half16  (sym+add)@tprel@l |
| [`R_PPC_TPREL16_HI`](#r_ppc_tprel16_hi) | const | half16  (sym+add)@tprel@h |
| [`R_PPC_TPREL16_HA`](#r_ppc_tprel16_ha) | const | half16  (sym+add)@tprel@ha |
| [`R_PPC_TPREL32`](#r_ppc_tprel32) | const | word32  (sym+add)@tprel |
| [`R_PPC_DTPREL16`](#r_ppc_dtprel16) | const | half16*(sym+add)@dtprel |
| [`R_PPC_DTPREL16_LO`](#r_ppc_dtprel16_lo) | const | half16  (sym+add)@dtprel@l |
| [`R_PPC_DTPREL16_HI`](#r_ppc_dtprel16_hi) | const | half16  (sym+add)@dtprel@h |
| [`R_PPC_DTPREL16_HA`](#r_ppc_dtprel16_ha) | const | half16  (sym+add)@dtprel@ha |
| [`R_PPC_DTPREL32`](#r_ppc_dtprel32) | const | word32  (sym+add)@dtprel |
| [`R_PPC_GOT_TLSGD16`](#r_ppc_got_tlsgd16) | const | half16* (sym+add)@got@tlsgd |
| [`R_PPC_GOT_TLSGD16_LO`](#r_ppc_got_tlsgd16_lo) | const | half16  (sym+add)@got@tlsgd@l |
| [`R_PPC_GOT_TLSGD16_HI`](#r_ppc_got_tlsgd16_hi) | const | half16  (sym+add)@got@tlsgd@h |
| [`R_PPC_GOT_TLSGD16_HA`](#r_ppc_got_tlsgd16_ha) | const | half16  (sym+add)@got@tlsgd@ha |
| [`R_PPC_GOT_TLSLD16`](#r_ppc_got_tlsld16) | const | half16* (sym+add)@got@tlsld |
| [`R_PPC_GOT_TLSLD16_LO`](#r_ppc_got_tlsld16_lo) | const | half16  (sym+add)@got@tlsld@l |
| [`R_PPC_GOT_TLSLD16_HI`](#r_ppc_got_tlsld16_hi) | const | half16  (sym+add)@got@tlsld@h |
| [`R_PPC_GOT_TLSLD16_HA`](#r_ppc_got_tlsld16_ha) | const | half16  (sym+add)@got@tlsld@ha |
| [`R_PPC_GOT_TPREL16`](#r_ppc_got_tprel16) | const | half16* (sym+add)@got@tprel |
| [`R_PPC_GOT_TPREL16_LO`](#r_ppc_got_tprel16_lo) | const | half16  (sym+add)@got@tprel@l |
| [`R_PPC_GOT_TPREL16_HI`](#r_ppc_got_tprel16_hi) | const | half16  (sym+add)@got@tprel@h |
| [`R_PPC_GOT_TPREL16_HA`](#r_ppc_got_tprel16_ha) | const | half16  (sym+add)@got@tprel@ha |
| [`R_PPC_GOT_DTPREL16`](#r_ppc_got_dtprel16) | const | half16* (sym+add)@got@dtprel |
| [`R_PPC_GOT_DTPREL16_LO`](#r_ppc_got_dtprel16_lo) | const | half16* (sym+add)@got@dtprel@l |
| [`R_PPC_GOT_DTPREL16_HI`](#r_ppc_got_dtprel16_hi) | const | half16* (sym+add)@got@dtprel@h |
| [`R_PPC_GOT_DTPREL16_HA`](#r_ppc_got_dtprel16_ha) | const | half16* (sym+add)@got@dtprel@ha |
| [`R_PPC_TLSGD`](#r_ppc_tlsgd) | const | none    (sym+add)@tlsgd |
| [`R_PPC_TLSLD`](#r_ppc_tlsld) | const | none    (sym+add)@tlsld |
| [`R_PPC_EMB_NADDR32`](#r_ppc_emb_naddr32) | const |  |
| [`R_PPC_EMB_NADDR16`](#r_ppc_emb_naddr16) | const |  |
| [`R_PPC_EMB_NADDR16_LO`](#r_ppc_emb_naddr16_lo) | const |  |
| [`R_PPC_EMB_NADDR16_HI`](#r_ppc_emb_naddr16_hi) | const |  |
| [`R_PPC_EMB_NADDR16_HA`](#r_ppc_emb_naddr16_ha) | const |  |
| [`R_PPC_EMB_SDAI16`](#r_ppc_emb_sdai16) | const |  |
| [`R_PPC_EMB_SDA2I16`](#r_ppc_emb_sda2i16) | const |  |
| [`R_PPC_EMB_SDA2REL`](#r_ppc_emb_sda2rel) | const |  |
| [`R_PPC_EMB_SDA21`](#r_ppc_emb_sda21) | const | 16 bit offset in SDA |
| [`R_PPC_EMB_MRKREF`](#r_ppc_emb_mrkref) | const |  |
| [`R_PPC_EMB_RELSEC16`](#r_ppc_emb_relsec16) | const |  |
| [`R_PPC_EMB_RELST_LO`](#r_ppc_emb_relst_lo) | const |  |
| [`R_PPC_EMB_RELST_HI`](#r_ppc_emb_relst_hi) | const |  |
| [`R_PPC_EMB_RELST_HA`](#r_ppc_emb_relst_ha) | const |  |
| [`R_PPC_EMB_BIT_FLD`](#r_ppc_emb_bit_fld) | const |  |
| [`R_PPC_EMB_RELSDA`](#r_ppc_emb_relsda) | const | 16 bit relative offset in SDA |
| [`R_PPC_DIAB_SDA21_LO`](#r_ppc_diab_sda21_lo) | const | like EMB_SDA21, but lower 16 bit |
| [`R_PPC_DIAB_SDA21_HI`](#r_ppc_diab_sda21_hi) | const | like EMB_SDA21, but high 16 bit |
| [`R_PPC_DIAB_SDA21_HA`](#r_ppc_diab_sda21_ha) | const | like EMB_SDA21, adjusted high 16 |
| [`R_PPC_DIAB_RELSDA_LO`](#r_ppc_diab_relsda_lo) | const | like EMB_RELSDA, but lower 16 bit |
| [`R_PPC_DIAB_RELSDA_HI`](#r_ppc_diab_relsda_hi) | const | like EMB_RELSDA, but high 16 bit |
| [`R_PPC_DIAB_RELSDA_HA`](#r_ppc_diab_relsda_ha) | const | like EMB_RELSDA, adjusted high 16 |
| [`R_PPC_IRELATIVE`](#r_ppc_irelative) | const | GNU extension to support local ifunc. |
| [`R_PPC_REL16`](#r_ppc_rel16) | const | half16   (sym+add-.) |
| [`R_PPC_REL16_LO`](#r_ppc_rel16_lo) | const | half16   (sym+add-.)@l |
| [`R_PPC_REL16_HI`](#r_ppc_rel16_hi) | const | half16   (sym+add-.)@h |
| [`R_PPC_REL16_HA`](#r_ppc_rel16_ha) | const | half16   (sym+add-.)@ha |
| [`R_PPC_TOC16`](#r_ppc_toc16) | const | This is a phony reloc to handle any old fashioned TOC16 references that may |
| [`DT_PPC_GOT`](#dt_ppc_got) | const |  |
| [`DT_PPC_OPT`](#dt_ppc_opt) | const |  |
| [`PPC_OPT_TLS`](#ppc_opt_tls) | const |  |
| [`R_PPC64_NONE`](#r_ppc64_none) | const |  |
| [`R_PPC64_ADDR32`](#r_ppc64_addr32) | const | 32bit absolute address |
| [`R_PPC64_ADDR24`](#r_ppc64_addr24) | const | 26bit address, word aligned |
| [`R_PPC64_ADDR16`](#r_ppc64_addr16) | const | 16bit absolute address |
| [`R_PPC64_ADDR16_LO`](#r_ppc64_addr16_lo) | const | lower 16bits of address |
| [`R_PPC64_ADDR16_HI`](#r_ppc64_addr16_hi) | const | high 16bits of address. |
| [`R_PPC64_ADDR16_HA`](#r_ppc64_addr16_ha) | const | adjusted high 16bits. |
| [`R_PPC64_ADDR14`](#r_ppc64_addr14) | const | 16bit address, word aligned |
| [`R_PPC64_ADDR14_BRTAKEN`](#r_ppc64_addr14_brtaken) | const |  |
| [`R_PPC64_ADDR14_BRNTAKEN`](#r_ppc64_addr14_brntaken) | const |  |
| [`R_PPC64_REL24`](#r_ppc64_rel24) | const | PC-rel. |
| [`R_PPC64_REL14`](#r_ppc64_rel14) | const | PC relative 16 bit |
| [`R_PPC64_REL14_BRTAKEN`](#r_ppc64_rel14_brtaken) | const |  |
| [`R_PPC64_REL14_BRNTAKEN`](#r_ppc64_rel14_brntaken) | const |  |
| [`R_PPC64_GOT16`](#r_ppc64_got16) | const |  |
| [`R_PPC64_GOT16_LO`](#r_ppc64_got16_lo) | const |  |
| [`R_PPC64_GOT16_HI`](#r_ppc64_got16_hi) | const |  |
| [`R_PPC64_GOT16_HA`](#r_ppc64_got16_ha) | const |  |
| [`R_PPC64_COPY`](#r_ppc64_copy) | const |  |
| [`R_PPC64_GLOB_DAT`](#r_ppc64_glob_dat) | const |  |
| [`R_PPC64_JMP_SLOT`](#r_ppc64_jmp_slot) | const |  |
| [`R_PPC64_RELATIVE`](#r_ppc64_relative) | const |  |
| [`R_PPC64_UADDR32`](#r_ppc64_uaddr32) | const |  |
| [`R_PPC64_UADDR16`](#r_ppc64_uaddr16) | const |  |
| [`R_PPC64_REL32`](#r_ppc64_rel32) | const |  |
| [`R_PPC64_PLT32`](#r_ppc64_plt32) | const |  |
| [`R_PPC64_PLTREL32`](#r_ppc64_pltrel32) | const |  |
| [`R_PPC64_PLT16_LO`](#r_ppc64_plt16_lo) | const |  |
| [`R_PPC64_PLT16_HI`](#r_ppc64_plt16_hi) | const |  |
| [`R_PPC64_PLT16_HA`](#r_ppc64_plt16_ha) | const |  |
| [`R_PPC64_SECTOFF`](#r_ppc64_sectoff) | const |  |
| [`R_PPC64_SECTOFF_LO`](#r_ppc64_sectoff_lo) | const |  |
| [`R_PPC64_SECTOFF_HI`](#r_ppc64_sectoff_hi) | const |  |
| [`R_PPC64_SECTOFF_HA`](#r_ppc64_sectoff_ha) | const |  |
| [`R_PPC64_ADDR30`](#r_ppc64_addr30) | const | word30 (S + A - P) >> 2 |
| [`R_PPC64_ADDR64`](#r_ppc64_addr64) | const | doubleword64 S + A |
| [`R_PPC64_ADDR16_HIGHER`](#r_ppc64_addr16_higher) | const | half16 #higher(S + A) |
| [`R_PPC64_ADDR16_HIGHERA`](#r_ppc64_addr16_highera) | const | half16 #highera(S + A) |
| [`R_PPC64_ADDR16_HIGHEST`](#r_ppc64_addr16_highest) | const | half16 #highest(S + A) |
| [`R_PPC64_ADDR16_HIGHESTA`](#r_ppc64_addr16_highesta) | const | half16 #highesta(S + A) |
| [`R_PPC64_UADDR64`](#r_ppc64_uaddr64) | const | doubleword64 S + A |
| [`R_PPC64_REL64`](#r_ppc64_rel64) | const | doubleword64 S + A - P |
| [`R_PPC64_PLT64`](#r_ppc64_plt64) | const | doubleword64 L + A |
| [`R_PPC64_PLTREL64`](#r_ppc64_pltrel64) | const | doubleword64 L + A - P |
| [`R_PPC64_TOC16`](#r_ppc64_toc16) | const | half16* S + A - .TOC |
| [`R_PPC64_TOC16_LO`](#r_ppc64_toc16_lo) | const | half16 #lo(S + A - .TOC.) |
| [`R_PPC64_TOC16_HI`](#r_ppc64_toc16_hi) | const | half16 #hi(S + A - .TOC.) |
| [`R_PPC64_TOC16_HA`](#r_ppc64_toc16_ha) | const | half16 #ha(S + A - .TOC.) |
| [`R_PPC64_TOC`](#r_ppc64_toc) | const | doubleword64 .TOC |
| [`R_PPC64_PLTGOT16`](#r_ppc64_pltgot16) | const | half16* M + A |
| [`R_PPC64_PLTGOT16_LO`](#r_ppc64_pltgot16_lo) | const | half16 #lo(M + A) |
| [`R_PPC64_PLTGOT16_HI`](#r_ppc64_pltgot16_hi) | const | half16 #hi(M + A) |
| [`R_PPC64_PLTGOT16_HA`](#r_ppc64_pltgot16_ha) | const | half16 #ha(M + A) |
| [`R_PPC64_ADDR16_DS`](#r_ppc64_addr16_ds) | const | half16ds* (S + A) >> 2 |
| [`R_PPC64_ADDR16_LO_DS`](#r_ppc64_addr16_lo_ds) | const | half16ds  #lo(S + A) >> 2 |
| [`R_PPC64_GOT16_DS`](#r_ppc64_got16_ds) | const | half16ds* (G + A) >> 2 |
| [`R_PPC64_GOT16_LO_DS`](#r_ppc64_got16_lo_ds) | const | half16ds  #lo(G + A) >> 2 |
| [`R_PPC64_PLT16_LO_DS`](#r_ppc64_plt16_lo_ds) | const | half16ds  #lo(L + A) >> 2 |
| [`R_PPC64_SECTOFF_DS`](#r_ppc64_sectoff_ds) | const | half16ds* (R + A) >> 2 |
| [`R_PPC64_SECTOFF_LO_DS`](#r_ppc64_sectoff_lo_ds) | const | half16ds  #lo(R + A) >> 2 |
| [`R_PPC64_TOC16_DS`](#r_ppc64_toc16_ds) | const | half16ds* (S + A - .TOC.) >> 2 |
| [`R_PPC64_TOC16_LO_DS`](#r_ppc64_toc16_lo_ds) | const | half16ds  #lo(S + A - .TOC.) >> 2 |
| [`R_PPC64_PLTGOT16_DS`](#r_ppc64_pltgot16_ds) | const | half16ds* (M + A) >> 2 |
| [`R_PPC64_PLTGOT16_LO_DS`](#r_ppc64_pltgot16_lo_ds) | const | half16ds  #lo(M + A) >> 2 |
| [`R_PPC64_TLS`](#r_ppc64_tls) | const | none    (sym+add)@tls |
| [`R_PPC64_DTPMOD64`](#r_ppc64_dtpmod64) | const | doubleword64 (sym+add)@dtpmod |
| [`R_PPC64_TPREL16`](#r_ppc64_tprel16) | const | half16* (sym+add)@tprel |
| [`R_PPC64_TPREL16_LO`](#r_ppc64_tprel16_lo) | const | half16  (sym+add)@tprel@l |
| [`R_PPC64_TPREL16_HI`](#r_ppc64_tprel16_hi) | const | half16  (sym+add)@tprel@h |
| [`R_PPC64_TPREL16_HA`](#r_ppc64_tprel16_ha) | const | half16  (sym+add)@tprel@ha |
| [`R_PPC64_TPREL64`](#r_ppc64_tprel64) | const | doubleword64 (sym+add)@tprel |
| [`R_PPC64_DTPREL16`](#r_ppc64_dtprel16) | const | half16* (sym+add)@dtprel |
| [`R_PPC64_DTPREL16_LO`](#r_ppc64_dtprel16_lo) | const | half16  (sym+add)@dtprel@l |
| [`R_PPC64_DTPREL16_HI`](#r_ppc64_dtprel16_hi) | const | half16  (sym+add)@dtprel@h |
| [`R_PPC64_DTPREL16_HA`](#r_ppc64_dtprel16_ha) | const | half16  (sym+add)@dtprel@ha |
| [`R_PPC64_DTPREL64`](#r_ppc64_dtprel64) | const | doubleword64 (sym+add)@dtprel |
| [`R_PPC64_GOT_TLSGD16`](#r_ppc64_got_tlsgd16) | const | half16* (sym+add)@got@tlsgd |
| [`R_PPC64_GOT_TLSGD16_LO`](#r_ppc64_got_tlsgd16_lo) | const | half16  (sym+add)@got@tlsgd@l |
| [`R_PPC64_GOT_TLSGD16_HI`](#r_ppc64_got_tlsgd16_hi) | const | half16  (sym+add)@got@tlsgd@h |
| [`R_PPC64_GOT_TLSGD16_HA`](#r_ppc64_got_tlsgd16_ha) | const | half16  (sym+add)@got@tlsgd@ha |
| [`R_PPC64_GOT_TLSLD16`](#r_ppc64_got_tlsld16) | const | half16* (sym+add)@got@tlsld |
| [`R_PPC64_GOT_TLSLD16_LO`](#r_ppc64_got_tlsld16_lo) | const | half16  (sym+add)@got@tlsld@l |
| [`R_PPC64_GOT_TLSLD16_HI`](#r_ppc64_got_tlsld16_hi) | const | half16  (sym+add)@got@tlsld@h |
| [`R_PPC64_GOT_TLSLD16_HA`](#r_ppc64_got_tlsld16_ha) | const | half16  (sym+add)@got@tlsld@ha |
| [`R_PPC64_GOT_TPREL16_DS`](#r_ppc64_got_tprel16_ds) | const | half16ds* (sym+add)@got@tprel |
| [`R_PPC64_GOT_TPREL16_LO_DS`](#r_ppc64_got_tprel16_lo_ds) | const | half16ds (sym+add)@got@tprel@l |
| [`R_PPC64_GOT_TPREL16_HI`](#r_ppc64_got_tprel16_hi) | const | half16  (sym+add)@got@tprel@h |
| [`R_PPC64_GOT_TPREL16_HA`](#r_ppc64_got_tprel16_ha) | const | half16  (sym+add)@got@tprel@ha |
| [`R_PPC64_GOT_DTPREL16_DS`](#r_ppc64_got_dtprel16_ds) | const | half16ds* (sym+add)@got@dtprel |
| [`R_PPC64_GOT_DTPREL16_LO_DS`](#r_ppc64_got_dtprel16_lo_ds) | const | half16ds (sym+add)@got@dtprel@l |
| [`R_PPC64_GOT_DTPREL16_HI`](#r_ppc64_got_dtprel16_hi) | const | half16  (sym+add)@got@dtprel@h |
| [`R_PPC64_GOT_DTPREL16_HA`](#r_ppc64_got_dtprel16_ha) | const | half16  (sym+add)@got@dtprel@ha |
| [`R_PPC64_TPREL16_DS`](#r_ppc64_tprel16_ds) | const | half16ds* (sym+add)@tprel |
| [`R_PPC64_TPREL16_LO_DS`](#r_ppc64_tprel16_lo_ds) | const | half16ds (sym+add)@tprel@l |
| [`R_PPC64_TPREL16_HIGHER`](#r_ppc64_tprel16_higher) | const | half16  (sym+add)@tprel@higher |
| [`R_PPC64_TPREL16_HIGHERA`](#r_ppc64_tprel16_highera) | const | half16  (sym+add)@tprel@highera |
| [`R_PPC64_TPREL16_HIGHEST`](#r_ppc64_tprel16_highest) | const | half16  (sym+add)@tprel@highest |
| [`R_PPC64_TPREL16_HIGHESTA`](#r_ppc64_tprel16_highesta) | const | half16  (sym+add)@tprel@highesta |
| [`R_PPC64_DTPREL16_DS`](#r_ppc64_dtprel16_ds) | const | half16ds* (sym+add)@dtprel |
| [`R_PPC64_DTPREL16_LO_DS`](#r_ppc64_dtprel16_lo_ds) | const | half16ds (sym+add)@dtprel@l |
| [`R_PPC64_DTPREL16_HIGHER`](#r_ppc64_dtprel16_higher) | const | half16  (sym+add)@dtprel@higher |
| [`R_PPC64_DTPREL16_HIGHERA`](#r_ppc64_dtprel16_highera) | const | half16  (sym+add)@dtprel@highera |
| [`R_PPC64_DTPREL16_HIGHEST`](#r_ppc64_dtprel16_highest) | const | half16  (sym+add)@dtprel@highest |
| [`R_PPC64_DTPREL16_HIGHESTA`](#r_ppc64_dtprel16_highesta) | const | half16  (sym+add)@dtprel@highesta |
| [`R_PPC64_TLSGD`](#r_ppc64_tlsgd) | const | none    (sym+add)@tlsgd |
| [`R_PPC64_TLSLD`](#r_ppc64_tlsld) | const | none    (sym+add)@tlsld |
| [`R_PPC64_TOCSAVE`](#r_ppc64_tocsave) | const | none |
| [`R_PPC64_ADDR16_HIGH`](#r_ppc64_addr16_high) | const |  |
| [`R_PPC64_ADDR16_HIGHA`](#r_ppc64_addr16_higha) | const |  |
| [`R_PPC64_TPREL16_HIGH`](#r_ppc64_tprel16_high) | const |  |
| [`R_PPC64_TPREL16_HIGHA`](#r_ppc64_tprel16_higha) | const |  |
| [`R_PPC64_DTPREL16_HIGH`](#r_ppc64_dtprel16_high) | const |  |
| [`R_PPC64_DTPREL16_HIGHA`](#r_ppc64_dtprel16_higha) | const |  |
| [`R_PPC64_JMP_IREL`](#r_ppc64_jmp_irel) | const | GNU extension to support local ifunc. |
| [`R_PPC64_IRELATIVE`](#r_ppc64_irelative) | const | GNU extension to support local ifunc. |
| [`R_PPC64_REL16`](#r_ppc64_rel16) | const | half16   (sym+add-.) |
| [`R_PPC64_REL16_LO`](#r_ppc64_rel16_lo) | const | half16   (sym+add-.)@l |
| [`R_PPC64_REL16_HI`](#r_ppc64_rel16_hi) | const | half16   (sym+add-.)@h |
| [`R_PPC64_REL16_HA`](#r_ppc64_rel16_ha) | const | half16   (sym+add-.)@ha |
| [`EF_PPC64_ABI`](#ef_ppc64_abi) | const | PowerPC64 bits specifying ABI. |
| [`DT_PPC64_GLINK`](#dt_ppc64_glink) | const |  |
| [`DT_PPC64_OPD`](#dt_ppc64_opd) | const |  |
| [`DT_PPC64_OPDSZ`](#dt_ppc64_opdsz) | const |  |
| [`DT_PPC64_OPT`](#dt_ppc64_opt) | const |  |
| [`PPC64_OPT_TLS`](#ppc64_opt_tls) | const |  |
| [`PPC64_OPT_MULTI_TOC`](#ppc64_opt_multi_toc) | const |  |
| [`PPC64_OPT_LOCALENTRY`](#ppc64_opt_localentry) | const |  |
| [`STO_PPC64_LOCAL_BIT`](#sto_ppc64_local_bit) | const |  |
| [`STO_PPC64_LOCAL_MASK`](#sto_ppc64_local_mask) | const |  |
| [`EF_ARM_RELEXEC`](#ef_arm_relexec) | const |  |
| [`EF_ARM_HASENTRY`](#ef_arm_hasentry) | const |  |
| [`EF_ARM_INTERWORK`](#ef_arm_interwork) | const |  |
| [`EF_ARM_APCS_26`](#ef_arm_apcs_26) | const |  |
| [`EF_ARM_APCS_FLOAT`](#ef_arm_apcs_float) | const |  |
| [`EF_ARM_PIC`](#ef_arm_pic) | const |  |
| [`EF_ARM_ALIGN8`](#ef_arm_align8) | const | 8-bit structure alignment is in use |
| [`EF_ARM_NEW_ABI`](#ef_arm_new_abi) | const |  |
| [`EF_ARM_OLD_ABI`](#ef_arm_old_abi) | const |  |
| [`EF_ARM_SOFT_FLOAT`](#ef_arm_soft_float) | const |  |
| [`EF_ARM_VFP_FLOAT`](#ef_arm_vfp_float) | const |  |
| [`EF_ARM_MAVERICK_FLOAT`](#ef_arm_maverick_float) | const |  |
| [`EF_ARM_ABI_FLOAT_SOFT`](#ef_arm_abi_float_soft) | const | NB conflicts with EF_ARM_SOFT_FLOAT |
| [`EF_ARM_ABI_FLOAT_HARD`](#ef_arm_abi_float_hard) | const | NB conflicts with EF_ARM_VFP_FLOAT |
| [`EF_ARM_SYMSARESORTED`](#ef_arm_symsaresorted) | const |  |
| [`EF_ARM_DYNSYMSUSESEGIDX`](#ef_arm_dynsymsusesegidx) | const |  |
| [`EF_ARM_MAPSYMSFIRST`](#ef_arm_mapsymsfirst) | const |  |
| [`EF_ARM_BE8`](#ef_arm_be8) | const |  |
| [`EF_ARM_LE8`](#ef_arm_le8) | const |  |
| [`EF_ARM_EABIMASK`](#ef_arm_eabimask) | const |  |
| [`EF_ARM_EABI_UNKNOWN`](#ef_arm_eabi_unknown) | const |  |
| [`EF_ARM_EABI_VER1`](#ef_arm_eabi_ver1) | const |  |
| [`EF_ARM_EABI_VER2`](#ef_arm_eabi_ver2) | const |  |
| [`EF_ARM_EABI_VER3`](#ef_arm_eabi_ver3) | const |  |
| [`EF_ARM_EABI_VER4`](#ef_arm_eabi_ver4) | const |  |
| [`EF_ARM_EABI_VER5`](#ef_arm_eabi_ver5) | const |  |
| [`STT_ARM_TFUNC`](#stt_arm_tfunc) | const | A Thumb function. |
| [`STT_ARM_16BIT`](#stt_arm_16bit) | const | A Thumb label. |
| [`SHF_ARM_ENTRYSECT`](#shf_arm_entrysect) | const | Section contains an entry point |
| [`SHF_ARM_COMDEF`](#shf_arm_comdef) | const | Section may be multiply defined in the input to a link step. |
| [`PF_ARM_SB`](#pf_arm_sb) | const | Segment contains the location addressed by the static base. |
| [`PF_ARM_PI`](#pf_arm_pi) | const | Position-independent segment. |
| [`PF_ARM_ABS`](#pf_arm_abs) | const | Absolute segment. |
| [`PT_ARM_EXIDX`](#pt_arm_exidx) | const | ARM unwind segment. |
| [`SHT_ARM_EXIDX`](#sht_arm_exidx) | const | ARM unwind section. |
| [`SHT_ARM_PREEMPTMAP`](#sht_arm_preemptmap) | const | Preemption details. |
| [`SHT_ARM_ATTRIBUTES`](#sht_arm_attributes) | const | ARM attributes section. |
| [`SHT_AARCH64_ATTRIBUTES`](#sht_aarch64_attributes) | const | AArch64 attributes section. |
| [`R_AARCH64_NONE`](#r_aarch64_none) | const | No relocation. |
| [`R_AARCH64_P32_ABS32`](#r_aarch64_p32_abs32) | const | Direct 32 bit. |
| [`R_AARCH64_P32_COPY`](#r_aarch64_p32_copy) | const | Copy symbol at runtime. |
| [`R_AARCH64_P32_GLOB_DAT`](#r_aarch64_p32_glob_dat) | const | Create GOT entry. |
| [`R_AARCH64_P32_JUMP_SLOT`](#r_aarch64_p32_jump_slot) | const | Create PLT entry. |
| [`R_AARCH64_P32_RELATIVE`](#r_aarch64_p32_relative) | const | Adjust by program base. |
| [`R_AARCH64_P32_TLS_DTPMOD`](#r_aarch64_p32_tls_dtpmod) | const | Module number, 32 bit. |
| [`R_AARCH64_P32_TLS_DTPREL`](#r_aarch64_p32_tls_dtprel) | const | Module-relative offset, 32 bit. |
| [`R_AARCH64_P32_TLS_TPREL`](#r_aarch64_p32_tls_tprel) | const | TP-relative offset, 32 bit. |
| [`R_AARCH64_P32_TLSDESC`](#r_aarch64_p32_tlsdesc) | const | TLS Descriptor. |
| [`R_AARCH64_P32_IRELATIVE`](#r_aarch64_p32_irelative) | const | STT_GNU_IFUNC relocation. |
| [`R_AARCH64_ABS64`](#r_aarch64_abs64) | const | Direct 64 bit. |
| [`R_AARCH64_ABS32`](#r_aarch64_abs32) | const | Direct 32 bit. |
| [`R_AARCH64_ABS16`](#r_aarch64_abs16) | const | Direct 16-bit. |
| [`R_AARCH64_PREL64`](#r_aarch64_prel64) | const | PC-relative 64-bit. |
| [`R_AARCH64_PREL32`](#r_aarch64_prel32) | const | PC-relative 32-bit. |
| [`R_AARCH64_PREL16`](#r_aarch64_prel16) | const | PC-relative 16-bit. |
| [`R_AARCH64_MOVW_UABS_G0`](#r_aarch64_movw_uabs_g0) | const | Dir. |
| [`R_AARCH64_MOVW_UABS_G0_NC`](#r_aarch64_movw_uabs_g0_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_UABS_G1`](#r_aarch64_movw_uabs_g1) | const | Dir. |
| [`R_AARCH64_MOVW_UABS_G1_NC`](#r_aarch64_movw_uabs_g1_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_UABS_G2`](#r_aarch64_movw_uabs_g2) | const | Dir. |
| [`R_AARCH64_MOVW_UABS_G2_NC`](#r_aarch64_movw_uabs_g2_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_UABS_G3`](#r_aarch64_movw_uabs_g3) | const | Dir. |
| [`R_AARCH64_MOVW_SABS_G0`](#r_aarch64_movw_sabs_g0) | const | Dir. |
| [`R_AARCH64_MOVW_SABS_G1`](#r_aarch64_movw_sabs_g1) | const | Dir. |
| [`R_AARCH64_MOVW_SABS_G2`](#r_aarch64_movw_sabs_g2) | const | Dir. |
| [`R_AARCH64_LD_PREL_LO19`](#r_aarch64_ld_prel_lo19) | const | PC-rel. |
| [`R_AARCH64_ADR_PREL_LO21`](#r_aarch64_adr_prel_lo21) | const | PC-rel. |
| [`R_AARCH64_ADR_PREL_PG_HI21`](#r_aarch64_adr_prel_pg_hi21) | const | Page-rel. |
| [`R_AARCH64_ADR_PREL_PG_HI21_NC`](#r_aarch64_adr_prel_pg_hi21_nc) | const | Likewise; no overflow check. |
| [`R_AARCH64_ADD_ABS_LO12_NC`](#r_aarch64_add_abs_lo12_nc) | const | Dir. |
| [`R_AARCH64_LDST8_ABS_LO12_NC`](#r_aarch64_ldst8_abs_lo12_nc) | const | Likewise for LD/ST; no check. |
| [`R_AARCH64_TSTBR14`](#r_aarch64_tstbr14) | const | PC-rel. |
| [`R_AARCH64_CONDBR19`](#r_aarch64_condbr19) | const | PC-rel. |
| [`R_AARCH64_JUMP26`](#r_aarch64_jump26) | const | PC-rel. |
| [`R_AARCH64_CALL26`](#r_aarch64_call26) | const | Likewise for CALL. |
| [`R_AARCH64_LDST16_ABS_LO12_NC`](#r_aarch64_ldst16_abs_lo12_nc) | const | Dir. |
| [`R_AARCH64_LDST32_ABS_LO12_NC`](#r_aarch64_ldst32_abs_lo12_nc) | const | Likewise for bits 11:2. |
| [`R_AARCH64_LDST64_ABS_LO12_NC`](#r_aarch64_ldst64_abs_lo12_nc) | const | Likewise for bits 11:3. |
| [`R_AARCH64_MOVW_PREL_G0`](#r_aarch64_movw_prel_g0) | const | PC-rel. |
| [`R_AARCH64_MOVW_PREL_G0_NC`](#r_aarch64_movw_prel_g0_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_PREL_G1`](#r_aarch64_movw_prel_g1) | const | PC-rel. |
| [`R_AARCH64_MOVW_PREL_G1_NC`](#r_aarch64_movw_prel_g1_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_PREL_G2`](#r_aarch64_movw_prel_g2) | const | PC-rel. |
| [`R_AARCH64_MOVW_PREL_G2_NC`](#r_aarch64_movw_prel_g2_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_PREL_G3`](#r_aarch64_movw_prel_g3) | const | PC-rel. |
| [`R_AARCH64_LDST128_ABS_LO12_NC`](#r_aarch64_ldst128_abs_lo12_nc) | const | Dir. |
| [`R_AARCH64_MOVW_GOTOFF_G0`](#r_aarch64_movw_gotoff_g0) | const | GOT-rel. |
| [`R_AARCH64_MOVW_GOTOFF_G0_NC`](#r_aarch64_movw_gotoff_g0_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_GOTOFF_G1`](#r_aarch64_movw_gotoff_g1) | const | GOT-rel. |
| [`R_AARCH64_MOVW_GOTOFF_G1_NC`](#r_aarch64_movw_gotoff_g1_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_GOTOFF_G2`](#r_aarch64_movw_gotoff_g2) | const | GOT-rel. |
| [`R_AARCH64_MOVW_GOTOFF_G2_NC`](#r_aarch64_movw_gotoff_g2_nc) | const | Likewise for MOVK; no check. |
| [`R_AARCH64_MOVW_GOTOFF_G3`](#r_aarch64_movw_gotoff_g3) | const | GOT-rel. |
| [`R_AARCH64_GOTREL64`](#r_aarch64_gotrel64) | const | GOT-relative 64-bit. |
| [`R_AARCH64_GOTREL32`](#r_aarch64_gotrel32) | const | GOT-relative 32-bit. |
| [`R_AARCH64_GOT_LD_PREL19`](#r_aarch64_got_ld_prel19) | const | PC-rel. |
| [`R_AARCH64_LD64_GOTOFF_LO15`](#r_aarch64_ld64_gotoff_lo15) | const | GOT-rel. |
| [`R_AARCH64_ADR_GOT_PAGE`](#r_aarch64_adr_got_page) | const | P-page-rel. |
| [`R_AARCH64_LD64_GOT_LO12_NC`](#r_aarch64_ld64_got_lo12_nc) | const | Dir. |
| [`R_AARCH64_LD64_GOTPAGE_LO15`](#r_aarch64_ld64_gotpage_lo15) | const | GOT-page-rel. |
| [`R_AARCH64_TLSGD_ADR_PREL21`](#r_aarch64_tlsgd_adr_prel21) | const | PC-relative ADR imm. |
| [`R_AARCH64_TLSGD_ADR_PAGE21`](#r_aarch64_tlsgd_adr_page21) | const | page-rel. |
| [`R_AARCH64_TLSGD_ADD_LO12_NC`](#r_aarch64_tlsgd_add_lo12_nc) | const | direct ADD imm. |
| [`R_AARCH64_TLSGD_MOVW_G1`](#r_aarch64_tlsgd_movw_g1) | const | GOT-rel. |
| [`R_AARCH64_TLSGD_MOVW_G0_NC`](#r_aarch64_tlsgd_movw_g0_nc) | const | GOT-rel. |
| [`R_AARCH64_TLSLD_ADR_PREL21`](#r_aarch64_tlsld_adr_prel21) | const | Like 512; local dynamic model. |
| [`R_AARCH64_TLSLD_ADR_PAGE21`](#r_aarch64_tlsld_adr_page21) | const | Like 513; local dynamic model. |
| [`R_AARCH64_TLSLD_ADD_LO12_NC`](#r_aarch64_tlsld_add_lo12_nc) | const | Like 514; local dynamic model. |
| [`R_AARCH64_TLSLD_MOVW_G1`](#r_aarch64_tlsld_movw_g1) | const | Like 515; local dynamic model. |
| [`R_AARCH64_TLSLD_MOVW_G0_NC`](#r_aarch64_tlsld_movw_g0_nc) | const | Like 516; local dynamic model. |
| [`R_AARCH64_TLSLD_LD_PREL19`](#r_aarch64_tlsld_ld_prel19) | const | TLS PC-rel. |
| [`R_AARCH64_TLSLD_MOVW_DTPREL_G2`](#r_aarch64_tlsld_movw_dtprel_g2) | const | TLS DTP-rel. |
| [`R_AARCH64_TLSLD_MOVW_DTPREL_G1`](#r_aarch64_tlsld_movw_dtprel_g1) | const | TLS DTP-rel. |
| [`R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC`](#r_aarch64_tlsld_movw_dtprel_g1_nc) | const | Likewise; MOVK; no check. |
| [`R_AARCH64_TLSLD_MOVW_DTPREL_G0`](#r_aarch64_tlsld_movw_dtprel_g0) | const | TLS DTP-rel. |
| [`R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC`](#r_aarch64_tlsld_movw_dtprel_g0_nc) | const | Likewise; MOVK; no check. |
| [`R_AARCH64_TLSLD_ADD_DTPREL_HI12`](#r_aarch64_tlsld_add_dtprel_hi12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_ADD_DTPREL_LO12`](#r_aarch64_tlsld_add_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC`](#r_aarch64_tlsld_add_dtprel_lo12_nc) | const | Likewise; no ovfl. |
| [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12`](#r_aarch64_tlsld_ldst8_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst8_dtprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12`](#r_aarch64_tlsld_ldst16_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst16_dtprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12`](#r_aarch64_tlsld_ldst32_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst32_dtprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12`](#r_aarch64_tlsld_ldst64_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst64_dtprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G1`](#r_aarch64_tlsie_movw_gottprel_g1) | const | GOT-rel. |
| [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC`](#r_aarch64_tlsie_movw_gottprel_g0_nc) | const | GOT-rel. |
| [`R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21`](#r_aarch64_tlsie_adr_gottprel_page21) | const | Page-rel. |
| [`R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC`](#r_aarch64_tlsie_ld64_gottprel_lo12_nc) | const | Direct LD off. |
| [`R_AARCH64_TLSIE_LD_GOTTPREL_PREL19`](#r_aarch64_tlsie_ld_gottprel_prel19) | const | PC-rel. |
| [`R_AARCH64_TLSLE_MOVW_TPREL_G2`](#r_aarch64_tlsle_movw_tprel_g2) | const | TLS TP-rel. |
| [`R_AARCH64_TLSLE_MOVW_TPREL_G1`](#r_aarch64_tlsle_movw_tprel_g1) | const | TLS TP-rel. |
| [`R_AARCH64_TLSLE_MOVW_TPREL_G1_NC`](#r_aarch64_tlsle_movw_tprel_g1_nc) | const | Likewise; MOVK; no check. |
| [`R_AARCH64_TLSLE_MOVW_TPREL_G0`](#r_aarch64_tlsle_movw_tprel_g0) | const | TLS TP-rel. |
| [`R_AARCH64_TLSLE_MOVW_TPREL_G0_NC`](#r_aarch64_tlsle_movw_tprel_g0_nc) | const | Likewise; MOVK; no check. |
| [`R_AARCH64_TLSLE_ADD_TPREL_HI12`](#r_aarch64_tlsle_add_tprel_hi12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_ADD_TPREL_LO12`](#r_aarch64_tlsle_add_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_ADD_TPREL_LO12_NC`](#r_aarch64_tlsle_add_tprel_lo12_nc) | const | Likewise; no ovfl. |
| [`R_AARCH64_TLSLE_LDST8_TPREL_LO12`](#r_aarch64_tlsle_ldst8_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst8_tprel_lo12_nc) | const | Likewise; no ovfl. |
| [`R_AARCH64_TLSLE_LDST16_TPREL_LO12`](#r_aarch64_tlsle_ldst16_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst16_tprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLE_LDST32_TPREL_LO12`](#r_aarch64_tlsle_ldst32_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst32_tprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLE_LDST64_TPREL_LO12`](#r_aarch64_tlsle_ldst64_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst64_tprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSDESC_LD_PREL19`](#r_aarch64_tlsdesc_ld_prel19) | const | PC-rel. |
| [`R_AARCH64_TLSDESC_ADR_PREL21`](#r_aarch64_tlsdesc_adr_prel21) | const | PC-rel. |
| [`R_AARCH64_TLSDESC_ADR_PAGE21`](#r_aarch64_tlsdesc_adr_page21) | const | Page-rel. |
| [`R_AARCH64_TLSDESC_LD64_LO12`](#r_aarch64_tlsdesc_ld64_lo12) | const | Direct LD off. |
| [`R_AARCH64_TLSDESC_ADD_LO12`](#r_aarch64_tlsdesc_add_lo12) | const | Direct ADD imm. |
| [`R_AARCH64_TLSDESC_OFF_G1`](#r_aarch64_tlsdesc_off_g1) | const | GOT-rel. |
| [`R_AARCH64_TLSDESC_OFF_G0_NC`](#r_aarch64_tlsdesc_off_g0_nc) | const | GOT-rel. |
| [`R_AARCH64_TLSDESC_LDR`](#r_aarch64_tlsdesc_ldr) | const | Relax LDR. |
| [`R_AARCH64_TLSDESC_ADD`](#r_aarch64_tlsdesc_add) | const | Relax ADD. |
| [`R_AARCH64_TLSDESC_CALL`](#r_aarch64_tlsdesc_call) | const | Relax BLR. |
| [`R_AARCH64_TLSLE_LDST128_TPREL_LO12`](#r_aarch64_tlsle_ldst128_tprel_lo12) | const | TP-rel. |
| [`R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst128_tprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12`](#r_aarch64_tlsld_ldst128_dtprel_lo12) | const | DTP-rel. |
| [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst128_dtprel_lo12_nc) | const | Likewise; no check. |
| [`R_AARCH64_COPY`](#r_aarch64_copy) | const | Copy symbol at runtime. |
| [`R_AARCH64_GLOB_DAT`](#r_aarch64_glob_dat) | const | Create GOT entry. |
| [`R_AARCH64_JUMP_SLOT`](#r_aarch64_jump_slot) | const | Create PLT entry. |
| [`R_AARCH64_RELATIVE`](#r_aarch64_relative) | const | Adjust by program base. |
| [`R_AARCH64_TLS_DTPMOD`](#r_aarch64_tls_dtpmod) | const | Module number, 64 bit. |
| [`R_AARCH64_TLS_DTPREL`](#r_aarch64_tls_dtprel) | const | Module-relative offset, 64 bit. |
| [`R_AARCH64_TLS_TPREL`](#r_aarch64_tls_tprel) | const | TP-relative offset, 64 bit. |
| [`R_AARCH64_TLSDESC`](#r_aarch64_tlsdesc) | const | TLS Descriptor. |
| [`R_AARCH64_IRELATIVE`](#r_aarch64_irelative) | const | STT_GNU_IFUNC relocation. |
| [`EF_AVR_ARCH`](#ef_avr_arch) | const | Bitmask for `EF_AVR_ARCH_*`. |
| [`EF_AVR_LINKRELAX_PREPARED`](#ef_avr_linkrelax_prepared) | const | If set, it is assumed that the elf file uses local symbols as reference |
| [`EF_AVR_ARCH_AVR1`](#ef_avr_arch_avr1) | const |  |
| [`EF_AVR_ARCH_AVR2`](#ef_avr_arch_avr2) | const |  |
| [`EF_AVR_ARCH_AVR25`](#ef_avr_arch_avr25) | const |  |
| [`EF_AVR_ARCH_AVR3`](#ef_avr_arch_avr3) | const |  |
| [`EF_AVR_ARCH_AVR31`](#ef_avr_arch_avr31) | const |  |
| [`EF_AVR_ARCH_AVR35`](#ef_avr_arch_avr35) | const |  |
| [`EF_AVR_ARCH_AVR4`](#ef_avr_arch_avr4) | const |  |
| [`EF_AVR_ARCH_AVR5`](#ef_avr_arch_avr5) | const |  |
| [`EF_AVR_ARCH_AVR51`](#ef_avr_arch_avr51) | const |  |
| [`EF_AVR_ARCH_AVR6`](#ef_avr_arch_avr6) | const |  |
| [`EF_AVR_ARCH_AVRTINY`](#ef_avr_arch_avrtiny) | const |  |
| [`EF_AVR_ARCH_XMEGA1`](#ef_avr_arch_xmega1) | const |  |
| [`EF_AVR_ARCH_XMEGA2`](#ef_avr_arch_xmega2) | const |  |
| [`EF_AVR_ARCH_XMEGA3`](#ef_avr_arch_xmega3) | const |  |
| [`EF_AVR_ARCH_XMEGA4`](#ef_avr_arch_xmega4) | const |  |
| [`EF_AVR_ARCH_XMEGA5`](#ef_avr_arch_xmega5) | const |  |
| [`EF_AVR_ARCH_XMEGA6`](#ef_avr_arch_xmega6) | const |  |
| [`EF_AVR_ARCH_XMEGA7`](#ef_avr_arch_xmega7) | const |  |
| [`R_AVR_NONE`](#r_avr_none) | const |  |
| [`R_AVR_32`](#r_avr_32) | const | Direct 32 bit |
| [`R_AVR_7_PCREL`](#r_avr_7_pcrel) | const |  |
| [`R_AVR_13_PCREL`](#r_avr_13_pcrel) | const |  |
| [`R_AVR_16`](#r_avr_16) | const | Direct 16 bit |
| [`R_AVR_16_PM`](#r_avr_16_pm) | const |  |
| [`R_AVR_LO8_LDI`](#r_avr_lo8_ldi) | const |  |
| [`R_AVR_HI8_LDI`](#r_avr_hi8_ldi) | const |  |
| [`R_AVR_HH8_LDI`](#r_avr_hh8_ldi) | const |  |
| [`R_AVR_LO8_LDI_NEG`](#r_avr_lo8_ldi_neg) | const |  |
| [`R_AVR_HI8_LDI_NEG`](#r_avr_hi8_ldi_neg) | const |  |
| [`R_AVR_HH8_LDI_NEG`](#r_avr_hh8_ldi_neg) | const |  |
| [`R_AVR_LO8_LDI_PM`](#r_avr_lo8_ldi_pm) | const |  |
| [`R_AVR_HI8_LDI_PM`](#r_avr_hi8_ldi_pm) | const |  |
| [`R_AVR_HH8_LDI_PM`](#r_avr_hh8_ldi_pm) | const |  |
| [`R_AVR_LO8_LDI_PM_NEG`](#r_avr_lo8_ldi_pm_neg) | const |  |
| [`R_AVR_HI8_LDI_PM_NEG`](#r_avr_hi8_ldi_pm_neg) | const |  |
| [`R_AVR_HH8_LDI_PM_NEG`](#r_avr_hh8_ldi_pm_neg) | const |  |
| [`R_AVR_CALL`](#r_avr_call) | const |  |
| [`R_AVR_LDI`](#r_avr_ldi) | const |  |
| [`R_AVR_6`](#r_avr_6) | const |  |
| [`R_AVR_6_ADIW`](#r_avr_6_adiw) | const |  |
| [`R_AVR_MS8_LDI`](#r_avr_ms8_ldi) | const |  |
| [`R_AVR_MS8_LDI_NEG`](#r_avr_ms8_ldi_neg) | const |  |
| [`R_AVR_LO8_LDI_GS`](#r_avr_lo8_ldi_gs) | const |  |
| [`R_AVR_HI8_LDI_GS`](#r_avr_hi8_ldi_gs) | const |  |
| [`R_AVR_8`](#r_avr_8) | const |  |
| [`R_AVR_8_LO8`](#r_avr_8_lo8) | const |  |
| [`R_AVR_8_HI8`](#r_avr_8_hi8) | const |  |
| [`R_AVR_8_HLO8`](#r_avr_8_hlo8) | const |  |
| [`R_AVR_DIFF8`](#r_avr_diff8) | const |  |
| [`R_AVR_DIFF16`](#r_avr_diff16) | const |  |
| [`R_AVR_DIFF32`](#r_avr_diff32) | const |  |
| [`R_AVR_LDS_STS_16`](#r_avr_lds_sts_16) | const |  |
| [`R_AVR_PORT6`](#r_avr_port6) | const |  |
| [`R_AVR_PORT5`](#r_avr_port5) | const |  |
| [`R_AVR_32_PCREL`](#r_avr_32_pcrel) | const |  |
| [`R_MSP430_32`](#r_msp430_32) | const | Direct 32 bit |
| [`R_MSP430_16_BYTE`](#r_msp430_16_byte) | const | Direct 16 bit |
| [`R_HEX_32`](#r_hex_32) | const | Direct 32 bit |
| [`R_ARM_NONE`](#r_arm_none) | const | No reloc |
| [`R_ARM_PC24`](#r_arm_pc24) | const | Deprecated PC relative 26 bit branch. |
| [`R_ARM_ABS32`](#r_arm_abs32) | const | Direct 32 bit |
| [`R_ARM_REL32`](#r_arm_rel32) | const | PC relative 32 bit |
| [`R_ARM_PC13`](#r_arm_pc13) | const |  |
| [`R_ARM_ABS16`](#r_arm_abs16) | const | Direct 16 bit |
| [`R_ARM_ABS12`](#r_arm_abs12) | const | Direct 12 bit |
| [`R_ARM_THM_ABS5`](#r_arm_thm_abs5) | const | Direct & 0x7C (`LDR`, `STR`). |
| [`R_ARM_ABS8`](#r_arm_abs8) | const | Direct 8 bit |
| [`R_ARM_SBREL32`](#r_arm_sbrel32) | const |  |
| [`R_ARM_THM_PC22`](#r_arm_thm_pc22) | const | PC relative 24 bit (Thumb32 `BL`). |
| [`R_ARM_THM_PC8`](#r_arm_thm_pc8) | const | PC relative & 0x3FC (Thumb16 `LDR`, `ADD`, `ADR`). |
| [`R_ARM_AMP_VCALL9`](#r_arm_amp_vcall9) | const |  |
| [`R_ARM_SWI24`](#r_arm_swi24) | const | Obsolete static relocation. |
| [`R_ARM_TLS_DESC`](#r_arm_tls_desc) | const | Dynamic relocation. |
| [`R_ARM_THM_SWI8`](#r_arm_thm_swi8) | const | Reserved. |
| [`R_ARM_XPC25`](#r_arm_xpc25) | const | Reserved. |
| [`R_ARM_THM_XPC22`](#r_arm_thm_xpc22) | const | Reserved. |
| [`R_ARM_TLS_DTPMOD32`](#r_arm_tls_dtpmod32) | const | ID of module containing symbol |
| [`R_ARM_TLS_DTPOFF32`](#r_arm_tls_dtpoff32) | const | Offset in TLS block |
| [`R_ARM_TLS_TPOFF32`](#r_arm_tls_tpoff32) | const | Offset in static TLS block |
| [`R_ARM_COPY`](#r_arm_copy) | const | Copy symbol at runtime |
| [`R_ARM_GLOB_DAT`](#r_arm_glob_dat) | const | Create GOT entry |
| [`R_ARM_JUMP_SLOT`](#r_arm_jump_slot) | const | Create PLT entry |
| [`R_ARM_RELATIVE`](#r_arm_relative) | const | Adjust by program base |
| [`R_ARM_GOTOFF`](#r_arm_gotoff) | const | 32 bit offset to GOT |
| [`R_ARM_GOTPC`](#r_arm_gotpc) | const | 32 bit PC relative offset to GOT |
| [`R_ARM_GOT32`](#r_arm_got32) | const | 32 bit GOT entry |
| [`R_ARM_PLT32`](#r_arm_plt32) | const | Deprecated, 32 bit PLT address. |
| [`R_ARM_CALL`](#r_arm_call) | const | PC relative 24 bit (`BL`, `BLX`). |
| [`R_ARM_JUMP24`](#r_arm_jump24) | const | PC relative 24 bit (`B`, `BL<cond>`). |
| [`R_ARM_THM_JUMP24`](#r_arm_thm_jump24) | const | PC relative 24 bit (Thumb32 `B.W`). |
| [`R_ARM_BASE_ABS`](#r_arm_base_abs) | const | Adjust by program base. |
| [`R_ARM_ALU_PCREL_7_0`](#r_arm_alu_pcrel_7_0) | const | Obsolete. |
| [`R_ARM_ALU_PCREL_15_8`](#r_arm_alu_pcrel_15_8) | const | Obsolete. |
| [`R_ARM_ALU_PCREL_23_15`](#r_arm_alu_pcrel_23_15) | const | Obsolete. |
| [`R_ARM_LDR_SBREL_11_0`](#r_arm_ldr_sbrel_11_0) | const | Deprecated, prog. |
| [`R_ARM_ALU_SBREL_19_12`](#r_arm_alu_sbrel_19_12) | const | Deprecated, prog. |
| [`R_ARM_ALU_SBREL_27_20`](#r_arm_alu_sbrel_27_20) | const | Deprecated, prog. |
| [`R_ARM_TARGET1`](#r_arm_target1) | const |  |
| [`R_ARM_SBREL31`](#r_arm_sbrel31) | const | Program base relative. |
| [`R_ARM_V4BX`](#r_arm_v4bx) | const |  |
| [`R_ARM_TARGET2`](#r_arm_target2) | const |  |
| [`R_ARM_PREL31`](#r_arm_prel31) | const | 32 bit PC relative. |
| [`R_ARM_MOVW_ABS_NC`](#r_arm_movw_abs_nc) | const | Direct 16-bit (`MOVW`). |
| [`R_ARM_MOVT_ABS`](#r_arm_movt_abs) | const | Direct high 16-bit (`MOVT`). |
| [`R_ARM_MOVW_PREL_NC`](#r_arm_movw_prel_nc) | const | PC relative 16-bit (`MOVW`). |
| [`R_ARM_MOVT_PREL`](#r_arm_movt_prel) | const | PC relative (MOVT). |
| [`R_ARM_THM_MOVW_ABS_NC`](#r_arm_thm_movw_abs_nc) | const | Direct 16 bit (Thumb32 `MOVW`). |
| [`R_ARM_THM_MOVT_ABS`](#r_arm_thm_movt_abs) | const | Direct high 16 bit (Thumb32 `MOVT`). |
| [`R_ARM_THM_MOVW_PREL_NC`](#r_arm_thm_movw_prel_nc) | const | PC relative 16 bit (Thumb32 `MOVW`). |
| [`R_ARM_THM_MOVT_PREL`](#r_arm_thm_movt_prel) | const | PC relative high 16 bit (Thumb32 `MOVT`). |
| [`R_ARM_THM_JUMP19`](#r_arm_thm_jump19) | const | PC relative 20 bit (Thumb32 `B<cond>.W`). |
| [`R_ARM_THM_JUMP6`](#r_arm_thm_jump6) | const | PC relative X & 0x7E (Thumb16 `CBZ`, `CBNZ`). |
| [`R_ARM_THM_ALU_PREL_11_0`](#r_arm_thm_alu_prel_11_0) | const | PC relative 12 bit (Thumb32 `ADR.W`). |
| [`R_ARM_THM_PC12`](#r_arm_thm_pc12) | const | PC relative 12 bit (Thumb32 `LDR{D,SB,H,SH}`). |
| [`R_ARM_ABS32_NOI`](#r_arm_abs32_noi) | const | Direct 32-bit. |
| [`R_ARM_REL32_NOI`](#r_arm_rel32_noi) | const | PC relative 32-bit. |
| [`R_ARM_ALU_PC_G0_NC`](#r_arm_alu_pc_g0_nc) | const | PC relative (`ADD`, `SUB`). |
| [`R_ARM_ALU_PC_G0`](#r_arm_alu_pc_g0) | const | PC relative (`ADD`, `SUB`). |
| [`R_ARM_ALU_PC_G1_NC`](#r_arm_alu_pc_g1_nc) | const | PC relative (`ADD`, `SUB`). |
| [`R_ARM_ALU_PC_G1`](#r_arm_alu_pc_g1) | const | PC relative (`ADD`, `SUB`). |
| [`R_ARM_ALU_PC_G2`](#r_arm_alu_pc_g2) | const | PC relative (`ADD`, `SUB`). |
| [`R_ARM_LDR_PC_G1`](#r_arm_ldr_pc_g1) | const | PC relative (`LDR`,`STR`,`LDRB`,`STRB`). |
| [`R_ARM_LDR_PC_G2`](#r_arm_ldr_pc_g2) | const | PC relative (`LDR`,`STR`,`LDRB`,`STRB`). |
| [`R_ARM_LDRS_PC_G0`](#r_arm_ldrs_pc_g0) | const | PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`). |
| [`R_ARM_LDRS_PC_G1`](#r_arm_ldrs_pc_g1) | const | PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`). |
| [`R_ARM_LDRS_PC_G2`](#r_arm_ldrs_pc_g2) | const | PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`). |
| [`R_ARM_LDC_PC_G0`](#r_arm_ldc_pc_g0) | const | PC relative (`LDC`, `STC`). |
| [`R_ARM_LDC_PC_G1`](#r_arm_ldc_pc_g1) | const | PC relative (`LDC`, `STC`). |
| [`R_ARM_LDC_PC_G2`](#r_arm_ldc_pc_g2) | const | PC relative (`LDC`, `STC`). |
| [`R_ARM_ALU_SB_G0_NC`](#r_arm_alu_sb_g0_nc) | const | Program base relative (`ADD`,`SUB`). |
| [`R_ARM_ALU_SB_G0`](#r_arm_alu_sb_g0) | const | Program base relative (`ADD`,`SUB`). |
| [`R_ARM_ALU_SB_G1_NC`](#r_arm_alu_sb_g1_nc) | const | Program base relative (`ADD`,`SUB`). |
| [`R_ARM_ALU_SB_G1`](#r_arm_alu_sb_g1) | const | Program base relative (`ADD`,`SUB`). |
| [`R_ARM_ALU_SB_G2`](#r_arm_alu_sb_g2) | const | Program base relative (`ADD`,`SUB`). |
| [`R_ARM_LDR_SB_G0`](#r_arm_ldr_sb_g0) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDR_SB_G1`](#r_arm_ldr_sb_g1) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDR_SB_G2`](#r_arm_ldr_sb_g2) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDRS_SB_G0`](#r_arm_ldrs_sb_g0) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDRS_SB_G1`](#r_arm_ldrs_sb_g1) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDRS_SB_G2`](#r_arm_ldrs_sb_g2) | const | Program base relative (`LDR`, `STR`, `LDRB`, `STRB`). |
| [`R_ARM_LDC_SB_G0`](#r_arm_ldc_sb_g0) | const | Program base relative (`LDC`,`STC`). |
| [`R_ARM_LDC_SB_G1`](#r_arm_ldc_sb_g1) | const | Program base relative (`LDC`,`STC`). |
| [`R_ARM_LDC_SB_G2`](#r_arm_ldc_sb_g2) | const | Program base relative (`LDC`,`STC`). |
| [`R_ARM_MOVW_BREL_NC`](#r_arm_movw_brel_nc) | const | Program base relative 16 bit (`MOVW`). |
| [`R_ARM_MOVT_BREL`](#r_arm_movt_brel) | const | Program base relative high 16 bit (`MOVT`). |
| [`R_ARM_MOVW_BREL`](#r_arm_movw_brel) | const | Program base relative 16 bit (`MOVW`). |
| [`R_ARM_THM_MOVW_BREL_NC`](#r_arm_thm_movw_brel_nc) | const | Program base relative 16 bit (Thumb32 `MOVW`). |
| [`R_ARM_THM_MOVT_BREL`](#r_arm_thm_movt_brel) | const | Program base relative high 16 bit (Thumb32 `MOVT`). |
| [`R_ARM_THM_MOVW_BREL`](#r_arm_thm_movw_brel) | const | Program base relative 16 bit (Thumb32 `MOVW`). |
| [`R_ARM_TLS_GOTDESC`](#r_arm_tls_gotdesc) | const |  |
| [`R_ARM_TLS_CALL`](#r_arm_tls_call) | const |  |
| [`R_ARM_TLS_DESCSEQ`](#r_arm_tls_descseq) | const | TLS relaxation. |
| [`R_ARM_THM_TLS_CALL`](#r_arm_thm_tls_call) | const |  |
| [`R_ARM_PLT32_ABS`](#r_arm_plt32_abs) | const |  |
| [`R_ARM_GOT_ABS`](#r_arm_got_abs) | const | GOT entry. |
| [`R_ARM_GOT_PREL`](#r_arm_got_prel) | const | PC relative GOT entry. |
| [`R_ARM_GOT_BREL12`](#r_arm_got_brel12) | const | GOT entry relative to GOT origin (`LDR`). |
| [`R_ARM_GOTOFF12`](#r_arm_gotoff12) | const | 12 bit, GOT entry relative to GOT origin (`LDR`, `STR`). |
| [`R_ARM_GOTRELAX`](#r_arm_gotrelax) | const |  |
| [`R_ARM_GNU_VTENTRY`](#r_arm_gnu_vtentry) | const |  |
| [`R_ARM_GNU_VTINHERIT`](#r_arm_gnu_vtinherit) | const |  |
| [`R_ARM_THM_PC11`](#r_arm_thm_pc11) | const | PC relative & 0xFFE (Thumb16 `B`). |
| [`R_ARM_THM_PC9`](#r_arm_thm_pc9) | const | PC relative & 0x1FE (Thumb16 `B`/`B<cond>`). |
| [`R_ARM_TLS_GD32`](#r_arm_tls_gd32) | const | PC-rel 32 bit for global dynamic thread local data |
| [`R_ARM_TLS_LDM32`](#r_arm_tls_ldm32) | const | PC-rel 32 bit for local dynamic thread local data |
| [`R_ARM_TLS_LDO32`](#r_arm_tls_ldo32) | const | 32 bit offset relative to TLS block |
| [`R_ARM_TLS_IE32`](#r_arm_tls_ie32) | const | PC-rel 32 bit for GOT entry of static TLS block offset |
| [`R_ARM_TLS_LE32`](#r_arm_tls_le32) | const | 32 bit offset relative to static TLS block |
| [`R_ARM_TLS_LDO12`](#r_arm_tls_ldo12) | const | 12 bit relative to TLS block (`LDR`, `STR`). |
| [`R_ARM_TLS_LE12`](#r_arm_tls_le12) | const | 12 bit relative to static TLS block (`LDR`, `STR`). |
| [`R_ARM_TLS_IE12GP`](#r_arm_tls_ie12gp) | const | 12 bit GOT entry relative to GOT origin (`LDR`). |
| [`R_ARM_ME_TOO`](#r_arm_me_too) | const | Obsolete. |
| [`R_ARM_THM_TLS_DESCSEQ`](#r_arm_thm_tls_descseq) | const |  |
| [`R_ARM_THM_TLS_DESCSEQ16`](#r_arm_thm_tls_descseq16) | const |  |
| [`R_ARM_THM_TLS_DESCSEQ32`](#r_arm_thm_tls_descseq32) | const |  |
| [`R_ARM_THM_GOT_BREL12`](#r_arm_thm_got_brel12) | const | GOT entry relative to GOT origin, 12 bit (Thumb32 `LDR`). |
| [`R_ARM_IRELATIVE`](#r_arm_irelative) | const |  |
| [`R_ARM_RXPC25`](#r_arm_rxpc25) | const |  |
| [`R_ARM_RSBREL32`](#r_arm_rsbrel32) | const |  |
| [`R_ARM_THM_RPC22`](#r_arm_thm_rpc22) | const |  |
| [`R_ARM_RREL32`](#r_arm_rrel32) | const |  |
| [`R_ARM_RABS22`](#r_arm_rabs22) | const |  |
| [`R_ARM_RPC24`](#r_arm_rpc24) | const |  |
| [`R_ARM_RBASE`](#r_arm_rbase) | const |  |
| [`R_CKCORE_NONE`](#r_ckcore_none) | const | no reloc |
| [`R_CKCORE_ADDR32`](#r_ckcore_addr32) | const | direct 32 bit (S + A) |
| [`R_CKCORE_PCRELIMM8BY4`](#r_ckcore_pcrelimm8by4) | const | disp ((S + A - P) >> 2) & 0xff |
| [`R_CKCORE_PCRELIMM11BY2`](#r_ckcore_pcrelimm11by2) | const | disp ((S + A - P) >> 1) & 0x7ff |
| [`R_CKCORE_PCREL32`](#r_ckcore_pcrel32) | const | 32-bit rel (S + A - P) |
| [`R_CKCORE_PCRELJSR_IMM11BY2`](#r_ckcore_pcreljsr_imm11by2) | const | disp ((S + A - P) >>1) & 0x7ff |
| [`R_CKCORE_RELATIVE`](#r_ckcore_relative) | const | 32 bit adjust program base(B + A) |
| [`R_CKCORE_COPY`](#r_ckcore_copy) | const | 32 bit adjust by program base |
| [`R_CKCORE_GLOB_DAT`](#r_ckcore_glob_dat) | const | off between got and sym (S) |
| [`R_CKCORE_JUMP_SLOT`](#r_ckcore_jump_slot) | const | PLT entry (S) |
| [`R_CKCORE_GOTOFF`](#r_ckcore_gotoff) | const | offset to GOT (S + A - GOT) |
| [`R_CKCORE_GOTPC`](#r_ckcore_gotpc) | const | PC offset to GOT (GOT + A - P) |
| [`R_CKCORE_GOT32`](#r_ckcore_got32) | const | 32 bit GOT entry (G) |
| [`R_CKCORE_PLT32`](#r_ckcore_plt32) | const | 32 bit PLT entry (G) |
| [`R_CKCORE_ADDRGOT`](#r_ckcore_addrgot) | const | GOT entry in GLOB_DAT (GOT + G) |
| [`R_CKCORE_ADDRPLT`](#r_ckcore_addrplt) | const | PLT entry in GLOB_DAT (GOT + G) |
| [`R_CKCORE_PCREL_IMM26BY2`](#r_ckcore_pcrel_imm26by2) | const | ((S + A - P) >> 1) & 0x3ff_ffff |
| [`R_CKCORE_PCREL_IMM16BY2`](#r_ckcore_pcrel_imm16by2) | const | disp ((S + A - P) >> 1) & 0xffff |
| [`R_CKCORE_PCREL_IMM16BY4`](#r_ckcore_pcrel_imm16by4) | const | disp ((S + A - P) >> 2) & 0xffff |
| [`R_CKCORE_PCREL_IMM10BY2`](#r_ckcore_pcrel_imm10by2) | const | disp ((S + A - P) >> 1) & 0x3ff |
| [`R_CKCORE_PCREL_IMM10BY4`](#r_ckcore_pcrel_imm10by4) | const | disp ((S + A - P) >> 2) & 0x3ff |
| [`R_CKCORE_ADDR_HI16`](#r_ckcore_addr_hi16) | const | high & low 16 bit ADDR, ((S + A) >> 16) & 0xffff |
| [`R_CKCORE_ADDR_LO16`](#r_ckcore_addr_lo16) | const | (S + A) & 0xffff |
| [`R_CKCORE_GOTPC_HI16`](#r_ckcore_gotpc_hi16) | const | high & low 16 bit GOTPC, ((GOT + A - P) >> 16) & 0xffff |
| [`R_CKCORE_GOTPC_LO16`](#r_ckcore_gotpc_lo16) | const | (GOT + A - P) & 0xffff |
| [`R_CKCORE_GOTOFF_HI16`](#r_ckcore_gotoff_hi16) | const | high & low 16 bit GOTOFF, ((S + A - GOT) >> 16) & 0xffff |
| [`R_CKCORE_GOTOFF_LO16`](#r_ckcore_gotoff_lo16) | const | (S + A - GOT) & 0xffff |
| [`R_CKCORE_GOT12`](#r_ckcore_got12) | const | 12 bit disp GOT entry (G) |
| [`R_CKCORE_GOT_HI16`](#r_ckcore_got_hi16) | const | high & low 16 bit GOT, (G >> 16) & 0xffff |
| [`R_CKCORE_GOT_LO16`](#r_ckcore_got_lo16) | const | (G & 0xffff) |
| [`R_CKCORE_PLT12`](#r_ckcore_plt12) | const | 12 bit disp PLT entry (G) |
| [`R_CKCORE_PLT_HI16`](#r_ckcore_plt_hi16) | const | high & low 16 bit PLT, (G >> 16) & 0xffff |
| [`R_CKCORE_PLT_LO16`](#r_ckcore_plt_lo16) | const | G & 0xffff |
| [`R_CKCORE_ADDRGOT_HI16`](#r_ckcore_addrgot_hi16) | const | high & low 16 bit ADDRGOT, (GOT + G * 4) & 0xffff |
| [`R_CKCORE_ADDRGOT_LO16`](#r_ckcore_addrgot_lo16) | const | (GOT + G * 4) & 0xffff |
| [`R_CKCORE_ADDRPLT_HI16`](#r_ckcore_addrplt_hi16) | const | high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF |
| [`R_CKCORE_ADDRPLT_LO16`](#r_ckcore_addrplt_lo16) | const | (GOT+G*4) & 0xffff |
| [`R_CKCORE_PCREL_JSR_IMM26BY2`](#r_ckcore_pcrel_jsr_imm26by2) | const | disp ((S+A-P) >>1) & x3ff_ffff |
| [`R_CKCORE_TOFFSET_LO16`](#r_ckcore_toffset_lo16) | const | (S+A-BTEXT) & 0xffff |
| [`R_CKCORE_DOFFSET_LO16`](#r_ckcore_doffset_lo16) | const | (S+A-BTEXT) & 0xffff |
| [`R_CKCORE_PCREL_IMM18BY2`](#r_ckcore_pcrel_imm18by2) | const | disp ((S+A-P) >>1) & 0x3ffff |
| [`R_CKCORE_DOFFSET_IMM18`](#r_ckcore_doffset_imm18) | const | disp (S+A-BDATA) & 0x3ffff |
| [`R_CKCORE_DOFFSET_IMM18BY2`](#r_ckcore_doffset_imm18by2) | const | disp ((S+A-BDATA)>>1) & 0x3ffff |
| [`R_CKCORE_DOFFSET_IMM18BY4`](#r_ckcore_doffset_imm18by4) | const | disp ((S+A-BDATA)>>2) & 0x3ffff |
| [`R_CKCORE_GOT_IMM18BY4`](#r_ckcore_got_imm18by4) | const | disp (G >> 2) |
| [`R_CKCORE_PLT_IMM18BY4`](#r_ckcore_plt_imm18by4) | const | disp (G >> 2) |
| [`R_CKCORE_PCREL_IMM7BY4`](#r_ckcore_pcrel_imm7by4) | const | disp ((S+A-P) >>2) & 0x7f |
| [`R_CKCORE_TLS_LE32`](#r_ckcore_tls_le32) | const | 32 bit offset to TLS block |
| [`R_CKCORE_TLS_IE32`](#r_ckcore_tls_ie32) | const |  |
| [`R_CKCORE_TLS_GD32`](#r_ckcore_tls_gd32) | const |  |
| [`R_CKCORE_TLS_LDM32`](#r_ckcore_tls_ldm32) | const |  |
| [`R_CKCORE_TLS_LDO32`](#r_ckcore_tls_ldo32) | const |  |
| [`R_CKCORE_TLS_DTPMOD32`](#r_ckcore_tls_dtpmod32) | const |  |
| [`R_CKCORE_TLS_DTPOFF32`](#r_ckcore_tls_dtpoff32) | const |  |
| [`R_CKCORE_TLS_TPOFF32`](#r_ckcore_tls_tpoff32) | const |  |
| [`EF_CSKY_ABIMASK`](#ef_csky_abimask) | const |  |
| [`EF_CSKY_OTHER`](#ef_csky_other) | const |  |
| [`EF_CSKY_PROCESSOR`](#ef_csky_processor) | const |  |
| [`EF_CSKY_ABIV1`](#ef_csky_abiv1) | const |  |
| [`EF_CSKY_ABIV2`](#ef_csky_abiv2) | const |  |
| [`SHT_CSKY_ATTRIBUTES`](#sht_csky_attributes) | const | C-SKY attributes section. |
| [`EF_IA_64_MASKOS`](#ef_ia_64_maskos) | const | os-specific flags |
| [`EF_IA_64_ABI64`](#ef_ia_64_abi64) | const | 64-bit ABI |
| [`EF_IA_64_ARCH`](#ef_ia_64_arch) | const | arch. |
| [`PT_IA_64_ARCHEXT`](#pt_ia_64_archext) | const | arch extension bits |
| [`PT_IA_64_UNWIND`](#pt_ia_64_unwind) | const | ia64 unwind bits |
| [`PT_IA_64_HP_OPT_ANOT`](#pt_ia_64_hp_opt_anot) | const |  |
| [`PT_IA_64_HP_HSL_ANOT`](#pt_ia_64_hp_hsl_anot) | const |  |
| [`PT_IA_64_HP_STACK`](#pt_ia_64_hp_stack) | const |  |
| [`PF_IA_64_NORECOV`](#pf_ia_64_norecov) | const | spec insns w/o recovery |
| [`SHT_IA_64_EXT`](#sht_ia_64_ext) | const | extension bits |
| [`SHT_IA_64_UNWIND`](#sht_ia_64_unwind) | const | unwind bits |
| [`SHF_IA_64_SHORT`](#shf_ia_64_short) | const | section near gp |
| [`SHF_IA_64_NORECOV`](#shf_ia_64_norecov) | const | spec insns w/o recovery |
| [`DT_IA_64_PLT_RESERVE`](#dt_ia_64_plt_reserve) | const |  |
| [`R_IA64_NONE`](#r_ia64_none) | const | none |
| [`R_IA64_IMM14`](#r_ia64_imm14) | const | symbol + addend, add imm14 |
| [`R_IA64_IMM22`](#r_ia64_imm22) | const | symbol + addend, add imm22 |
| [`R_IA64_IMM64`](#r_ia64_imm64) | const | symbol + addend, mov imm64 |
| [`R_IA64_DIR32MSB`](#r_ia64_dir32msb) | const | symbol + addend, data4 MSB |
| [`R_IA64_DIR32LSB`](#r_ia64_dir32lsb) | const | symbol + addend, data4 LSB |
| [`R_IA64_DIR64MSB`](#r_ia64_dir64msb) | const | symbol + addend, data8 MSB |
| [`R_IA64_DIR64LSB`](#r_ia64_dir64lsb) | const | symbol + addend, data8 LSB |
| [`R_IA64_GPREL22`](#r_ia64_gprel22) | const | @gprel(sym + add), add imm22 |
| [`R_IA64_GPREL64I`](#r_ia64_gprel64i) | const | @gprel(sym + add), mov imm64 |
| [`R_IA64_GPREL32MSB`](#r_ia64_gprel32msb) | const | @gprel(sym + add), data4 MSB |
| [`R_IA64_GPREL32LSB`](#r_ia64_gprel32lsb) | const | @gprel(sym + add), data4 LSB |
| [`R_IA64_GPREL64MSB`](#r_ia64_gprel64msb) | const | @gprel(sym + add), data8 MSB |
| [`R_IA64_GPREL64LSB`](#r_ia64_gprel64lsb) | const | @gprel(sym + add), data8 LSB |
| [`R_IA64_LTOFF22`](#r_ia64_ltoff22) | const | @ltoff(sym + add), add imm22 |
| [`R_IA64_LTOFF64I`](#r_ia64_ltoff64i) | const | @ltoff(sym + add), mov imm64 |
| [`R_IA64_PLTOFF22`](#r_ia64_pltoff22) | const | @pltoff(sym + add), add imm22 |
| [`R_IA64_PLTOFF64I`](#r_ia64_pltoff64i) | const | @pltoff(sym + add), mov imm64 |
| [`R_IA64_PLTOFF64MSB`](#r_ia64_pltoff64msb) | const | @pltoff(sym + add), data8 MSB |
| [`R_IA64_PLTOFF64LSB`](#r_ia64_pltoff64lsb) | const | @pltoff(sym + add), data8 LSB |
| [`R_IA64_FPTR64I`](#r_ia64_fptr64i) | const | @fptr(sym + add), mov imm64 |
| [`R_IA64_FPTR32MSB`](#r_ia64_fptr32msb) | const | @fptr(sym + add), data4 MSB |
| [`R_IA64_FPTR32LSB`](#r_ia64_fptr32lsb) | const | @fptr(sym + add), data4 LSB |
| [`R_IA64_FPTR64MSB`](#r_ia64_fptr64msb) | const | @fptr(sym + add), data8 MSB |
| [`R_IA64_FPTR64LSB`](#r_ia64_fptr64lsb) | const | @fptr(sym + add), data8 LSB |
| [`R_IA64_PCREL60B`](#r_ia64_pcrel60b) | const | @pcrel(sym + add), brl |
| [`R_IA64_PCREL21B`](#r_ia64_pcrel21b) | const | @pcrel(sym + add), ptb, call |
| [`R_IA64_PCREL21M`](#r_ia64_pcrel21m) | const | @pcrel(sym + add), chk.s |
| [`R_IA64_PCREL21F`](#r_ia64_pcrel21f) | const | @pcrel(sym + add), fchkf |
| [`R_IA64_PCREL32MSB`](#r_ia64_pcrel32msb) | const | @pcrel(sym + add), data4 MSB |
| [`R_IA64_PCREL32LSB`](#r_ia64_pcrel32lsb) | const | @pcrel(sym + add), data4 LSB |
| [`R_IA64_PCREL64MSB`](#r_ia64_pcrel64msb) | const | @pcrel(sym + add), data8 MSB |
| [`R_IA64_PCREL64LSB`](#r_ia64_pcrel64lsb) | const | @pcrel(sym + add), data8 LSB |
| [`R_IA64_LTOFF_FPTR22`](#r_ia64_ltoff_fptr22) | const | @ltoff(@fptr(s+a)), imm22 |
| [`R_IA64_LTOFF_FPTR64I`](#r_ia64_ltoff_fptr64i) | const | @ltoff(@fptr(s+a)), imm64 |
| [`R_IA64_LTOFF_FPTR32MSB`](#r_ia64_ltoff_fptr32msb) | const | @ltoff(@fptr(s+a)), data4 MSB |
| [`R_IA64_LTOFF_FPTR32LSB`](#r_ia64_ltoff_fptr32lsb) | const | @ltoff(@fptr(s+a)), data4 LSB |
| [`R_IA64_LTOFF_FPTR64MSB`](#r_ia64_ltoff_fptr64msb) | const | @ltoff(@fptr(s+a)), data8 MSB |
| [`R_IA64_LTOFF_FPTR64LSB`](#r_ia64_ltoff_fptr64lsb) | const | @ltoff(@fptr(s+a)), data8 LSB |
| [`R_IA64_SEGREL32MSB`](#r_ia64_segrel32msb) | const | @segrel(sym + add), data4 MSB |
| [`R_IA64_SEGREL32LSB`](#r_ia64_segrel32lsb) | const | @segrel(sym + add), data4 LSB |
| [`R_IA64_SEGREL64MSB`](#r_ia64_segrel64msb) | const | @segrel(sym + add), data8 MSB |
| [`R_IA64_SEGREL64LSB`](#r_ia64_segrel64lsb) | const | @segrel(sym + add), data8 LSB |
| [`R_IA64_SECREL32MSB`](#r_ia64_secrel32msb) | const | @secrel(sym + add), data4 MSB |
| [`R_IA64_SECREL32LSB`](#r_ia64_secrel32lsb) | const | @secrel(sym + add), data4 LSB |
| [`R_IA64_SECREL64MSB`](#r_ia64_secrel64msb) | const | @secrel(sym + add), data8 MSB |
| [`R_IA64_SECREL64LSB`](#r_ia64_secrel64lsb) | const | @secrel(sym + add), data8 LSB |
| [`R_IA64_REL32MSB`](#r_ia64_rel32msb) | const | data 4 + REL |
| [`R_IA64_REL32LSB`](#r_ia64_rel32lsb) | const | data 4 + REL |
| [`R_IA64_REL64MSB`](#r_ia64_rel64msb) | const | data 8 + REL |
| [`R_IA64_REL64LSB`](#r_ia64_rel64lsb) | const | data 8 + REL |
| [`R_IA64_LTV32MSB`](#r_ia64_ltv32msb) | const | symbol + addend, data4 MSB |
| [`R_IA64_LTV32LSB`](#r_ia64_ltv32lsb) | const | symbol + addend, data4 LSB |
| [`R_IA64_LTV64MSB`](#r_ia64_ltv64msb) | const | symbol + addend, data8 MSB |
| [`R_IA64_LTV64LSB`](#r_ia64_ltv64lsb) | const | symbol + addend, data8 LSB |
| [`R_IA64_PCREL21BI`](#r_ia64_pcrel21bi) | const | @pcrel(sym + add), 21bit inst |
| [`R_IA64_PCREL22`](#r_ia64_pcrel22) | const | @pcrel(sym + add), 22bit inst |
| [`R_IA64_PCREL64I`](#r_ia64_pcrel64i) | const | @pcrel(sym + add), 64bit inst |
| [`R_IA64_IPLTMSB`](#r_ia64_ipltmsb) | const | dynamic reloc, imported PLT, MSB |
| [`R_IA64_IPLTLSB`](#r_ia64_ipltlsb) | const | dynamic reloc, imported PLT, LSB |
| [`R_IA64_COPY`](#r_ia64_copy) | const | copy relocation |
| [`R_IA64_SUB`](#r_ia64_sub) | const | Addend and symbol difference |
| [`R_IA64_LTOFF22X`](#r_ia64_ltoff22x) | const | LTOFF22, relaxable. |
| [`R_IA64_LDXMOV`](#r_ia64_ldxmov) | const | Use of LTOFF22X. |
| [`R_IA64_TPREL14`](#r_ia64_tprel14) | const | @tprel(sym + add), imm14 |
| [`R_IA64_TPREL22`](#r_ia64_tprel22) | const | @tprel(sym + add), imm22 |
| [`R_IA64_TPREL64I`](#r_ia64_tprel64i) | const | @tprel(sym + add), imm64 |
| [`R_IA64_TPREL64MSB`](#r_ia64_tprel64msb) | const | @tprel(sym + add), data8 MSB |
| [`R_IA64_TPREL64LSB`](#r_ia64_tprel64lsb) | const | @tprel(sym + add), data8 LSB |
| [`R_IA64_LTOFF_TPREL22`](#r_ia64_ltoff_tprel22) | const | @ltoff(@tprel(s+a)), imm2 |
| [`R_IA64_DTPMOD64MSB`](#r_ia64_dtpmod64msb) | const | @dtpmod(sym + add), data8 MSB |
| [`R_IA64_DTPMOD64LSB`](#r_ia64_dtpmod64lsb) | const | @dtpmod(sym + add), data8 LSB |
| [`R_IA64_LTOFF_DTPMOD22`](#r_ia64_ltoff_dtpmod22) | const | @ltoff(@dtpmod(sym + add)), imm22 |
| [`R_IA64_DTPREL14`](#r_ia64_dtprel14) | const | @dtprel(sym + add), imm14 |
| [`R_IA64_DTPREL22`](#r_ia64_dtprel22) | const | @dtprel(sym + add), imm22 |
| [`R_IA64_DTPREL64I`](#r_ia64_dtprel64i) | const | @dtprel(sym + add), imm64 |
| [`R_IA64_DTPREL32MSB`](#r_ia64_dtprel32msb) | const | @dtprel(sym + add), data4 MSB |
| [`R_IA64_DTPREL32LSB`](#r_ia64_dtprel32lsb) | const | @dtprel(sym + add), data4 LSB |
| [`R_IA64_DTPREL64MSB`](#r_ia64_dtprel64msb) | const | @dtprel(sym + add), data8 MSB |
| [`R_IA64_DTPREL64LSB`](#r_ia64_dtprel64lsb) | const | @dtprel(sym + add), data8 LSB |
| [`R_IA64_LTOFF_DTPREL22`](#r_ia64_ltoff_dtprel22) | const | @ltoff(@dtprel(s+a)), imm22 |
| [`EF_SH_MACH_MASK`](#ef_sh_mach_mask) | const |  |
| [`EF_SH_UNKNOWN`](#ef_sh_unknown) | const |  |
| [`EF_SH1`](#ef_sh1) | const |  |
| [`EF_SH2`](#ef_sh2) | const |  |
| [`EF_SH3`](#ef_sh3) | const |  |
| [`EF_SH_DSP`](#ef_sh_dsp) | const |  |
| [`EF_SH3_DSP`](#ef_sh3_dsp) | const |  |
| [`EF_SH4AL_DSP`](#ef_sh4al_dsp) | const |  |
| [`EF_SH3E`](#ef_sh3e) | const |  |
| [`EF_SH4`](#ef_sh4) | const |  |
| [`EF_SH2E`](#ef_sh2e) | const |  |
| [`EF_SH4A`](#ef_sh4a) | const |  |
| [`EF_SH2A`](#ef_sh2a) | const |  |
| [`EF_SH4_NOFPU`](#ef_sh4_nofpu) | const |  |
| [`EF_SH4A_NOFPU`](#ef_sh4a_nofpu) | const |  |
| [`EF_SH4_NOMMU_NOFPU`](#ef_sh4_nommu_nofpu) | const |  |
| [`EF_SH2A_NOFPU`](#ef_sh2a_nofpu) | const |  |
| [`EF_SH3_NOMMU`](#ef_sh3_nommu) | const |  |
| [`EF_SH2A_SH4_NOFPU`](#ef_sh2a_sh4_nofpu) | const |  |
| [`EF_SH2A_SH3_NOFPU`](#ef_sh2a_sh3_nofpu) | const |  |
| [`EF_SH2A_SH4`](#ef_sh2a_sh4) | const |  |
| [`EF_SH2A_SH3E`](#ef_sh2a_sh3e) | const |  |
| [`R_SH_NONE`](#r_sh_none) | const |  |
| [`R_SH_DIR32`](#r_sh_dir32) | const |  |
| [`R_SH_REL32`](#r_sh_rel32) | const |  |
| [`R_SH_DIR8WPN`](#r_sh_dir8wpn) | const |  |
| [`R_SH_IND12W`](#r_sh_ind12w) | const |  |
| [`R_SH_DIR8WPL`](#r_sh_dir8wpl) | const |  |
| [`R_SH_DIR8WPZ`](#r_sh_dir8wpz) | const |  |
| [`R_SH_DIR8BP`](#r_sh_dir8bp) | const |  |
| [`R_SH_DIR8W`](#r_sh_dir8w) | const |  |
| [`R_SH_DIR8L`](#r_sh_dir8l) | const |  |
| [`R_SH_SWITCH16`](#r_sh_switch16) | const |  |
| [`R_SH_SWITCH32`](#r_sh_switch32) | const |  |
| [`R_SH_USES`](#r_sh_uses) | const |  |
| [`R_SH_COUNT`](#r_sh_count) | const |  |
| [`R_SH_ALIGN`](#r_sh_align) | const |  |
| [`R_SH_CODE`](#r_sh_code) | const |  |
| [`R_SH_DATA`](#r_sh_data) | const |  |
| [`R_SH_LABEL`](#r_sh_label) | const |  |
| [`R_SH_SWITCH8`](#r_sh_switch8) | const |  |
| [`R_SH_GNU_VTINHERIT`](#r_sh_gnu_vtinherit) | const |  |
| [`R_SH_GNU_VTENTRY`](#r_sh_gnu_vtentry) | const |  |
| [`R_SH_TLS_GD_32`](#r_sh_tls_gd_32) | const |  |
| [`R_SH_TLS_LD_32`](#r_sh_tls_ld_32) | const |  |
| [`R_SH_TLS_LDO_32`](#r_sh_tls_ldo_32) | const |  |
| [`R_SH_TLS_IE_32`](#r_sh_tls_ie_32) | const |  |
| [`R_SH_TLS_LE_32`](#r_sh_tls_le_32) | const |  |
| [`R_SH_TLS_DTPMOD32`](#r_sh_tls_dtpmod32) | const |  |
| [`R_SH_TLS_DTPOFF32`](#r_sh_tls_dtpoff32) | const |  |
| [`R_SH_TLS_TPOFF32`](#r_sh_tls_tpoff32) | const |  |
| [`R_SH_GOT32`](#r_sh_got32) | const |  |
| [`R_SH_PLT32`](#r_sh_plt32) | const |  |
| [`R_SH_COPY`](#r_sh_copy) | const |  |
| [`R_SH_GLOB_DAT`](#r_sh_glob_dat) | const |  |
| [`R_SH_JMP_SLOT`](#r_sh_jmp_slot) | const |  |
| [`R_SH_RELATIVE`](#r_sh_relative) | const |  |
| [`R_SH_GOTOFF`](#r_sh_gotoff) | const |  |
| [`R_SH_GOTPC`](#r_sh_gotpc) | const |  |
| [`EF_S390_HIGH_GPRS`](#ef_s390_high_gprs) | const | High GPRs kernel facility needed. |
| [`R_390_NONE`](#r_390_none) | const | No reloc. |
| [`R_390_8`](#r_390_8) | const | Direct 8 bit. |
| [`R_390_12`](#r_390_12) | const | Direct 12 bit. |
| [`R_390_16`](#r_390_16) | const | Direct 16 bit. |
| [`R_390_32`](#r_390_32) | const | Direct 32 bit. |
| [`R_390_PC32`](#r_390_pc32) | const | PC relative 32 bit. |
| [`R_390_GOT12`](#r_390_got12) | const | 12 bit GOT offset. |
| [`R_390_GOT32`](#r_390_got32) | const | 32 bit GOT offset. |
| [`R_390_PLT32`](#r_390_plt32) | const | 32 bit PC relative PLT address. |
| [`R_390_COPY`](#r_390_copy) | const | Copy symbol at runtime. |
| [`R_390_GLOB_DAT`](#r_390_glob_dat) | const | Create GOT entry. |
| [`R_390_JMP_SLOT`](#r_390_jmp_slot) | const | Create PLT entry. |
| [`R_390_RELATIVE`](#r_390_relative) | const | Adjust by program base. |
| [`R_390_GOTOFF32`](#r_390_gotoff32) | const | 32 bit offset to GOT. |
| [`R_390_GOTPC`](#r_390_gotpc) | const | 32 bit PC relative offset to GOT. |
| [`R_390_GOT16`](#r_390_got16) | const | 16 bit GOT offset. |
| [`R_390_PC16`](#r_390_pc16) | const | PC relative 16 bit. |
| [`R_390_PC16DBL`](#r_390_pc16dbl) | const | PC relative 16 bit shifted by 1. |
| [`R_390_PLT16DBL`](#r_390_plt16dbl) | const | 16 bit PC rel. |
| [`R_390_PC32DBL`](#r_390_pc32dbl) | const | PC relative 32 bit shifted by 1. |
| [`R_390_PLT32DBL`](#r_390_plt32dbl) | const | 32 bit PC rel. |
| [`R_390_GOTPCDBL`](#r_390_gotpcdbl) | const | 32 bit PC rel. |
| [`R_390_64`](#r_390_64) | const | Direct 64 bit. |
| [`R_390_PC64`](#r_390_pc64) | const | PC relative 64 bit. |
| [`R_390_GOT64`](#r_390_got64) | const | 64 bit GOT offset. |
| [`R_390_PLT64`](#r_390_plt64) | const | 64 bit PC relative PLT address. |
| [`R_390_GOTENT`](#r_390_gotent) | const | 32 bit PC rel. |
| [`R_390_GOTOFF16`](#r_390_gotoff16) | const | 16 bit offset to GOT. |
| [`R_390_GOTOFF64`](#r_390_gotoff64) | const | 64 bit offset to GOT. |
| [`R_390_GOTPLT12`](#r_390_gotplt12) | const | 12 bit offset to jump slot. |
| [`R_390_GOTPLT16`](#r_390_gotplt16) | const | 16 bit offset to jump slot. |
| [`R_390_GOTPLT32`](#r_390_gotplt32) | const | 32 bit offset to jump slot. |
| [`R_390_GOTPLT64`](#r_390_gotplt64) | const | 64 bit offset to jump slot. |
| [`R_390_GOTPLTENT`](#r_390_gotpltent) | const | 32 bit rel. |
| [`R_390_PLTOFF16`](#r_390_pltoff16) | const | 16 bit offset from GOT to PLT. |
| [`R_390_PLTOFF32`](#r_390_pltoff32) | const | 32 bit offset from GOT to PLT. |
| [`R_390_PLTOFF64`](#r_390_pltoff64) | const | 16 bit offset from GOT to PLT. |
| [`R_390_TLS_LOAD`](#r_390_tls_load) | const | Tag for load insn in TLS code. |
| [`R_390_TLS_GDCALL`](#r_390_tls_gdcall) | const | Tag for function call in general dynamic TLS code. |
| [`R_390_TLS_LDCALL`](#r_390_tls_ldcall) | const | Tag for function call in local dynamic TLS code. |
| [`R_390_TLS_GD32`](#r_390_tls_gd32) | const | Direct 32 bit for general dynamic thread local data. |
| [`R_390_TLS_GD64`](#r_390_tls_gd64) | const | Direct 64 bit for general dynamic thread local data. |
| [`R_390_TLS_GOTIE12`](#r_390_tls_gotie12) | const | 12 bit GOT offset for static TLS block offset. |
| [`R_390_TLS_GOTIE32`](#r_390_tls_gotie32) | const | 32 bit GOT offset for static TLS block offset. |
| [`R_390_TLS_GOTIE64`](#r_390_tls_gotie64) | const | 64 bit GOT offset for static TLS block offset. |
| [`R_390_TLS_LDM32`](#r_390_tls_ldm32) | const | Direct 32 bit for local dynamic thread local data in LE code. |
| [`R_390_TLS_LDM64`](#r_390_tls_ldm64) | const | Direct 64 bit for local dynamic thread local data in LE code. |
| [`R_390_TLS_IE32`](#r_390_tls_ie32) | const | 32 bit address of GOT entry for negated static TLS block offset. |
| [`R_390_TLS_IE64`](#r_390_tls_ie64) | const | 64 bit address of GOT entry for negated static TLS block offset. |
| [`R_390_TLS_IEENT`](#r_390_tls_ieent) | const | 32 bit rel. |
| [`R_390_TLS_LE32`](#r_390_tls_le32) | const | 32 bit negated offset relative to static TLS block. |
| [`R_390_TLS_LE64`](#r_390_tls_le64) | const | 64 bit negated offset relative to static TLS block. |
| [`R_390_TLS_LDO32`](#r_390_tls_ldo32) | const | 32 bit offset relative to TLS block. |
| [`R_390_TLS_LDO64`](#r_390_tls_ldo64) | const | 64 bit offset relative to TLS block. |
| [`R_390_TLS_DTPMOD`](#r_390_tls_dtpmod) | const | ID of module containing symbol. |
| [`R_390_TLS_DTPOFF`](#r_390_tls_dtpoff) | const | Offset in TLS block. |
| [`R_390_TLS_TPOFF`](#r_390_tls_tpoff) | const | Negated offset in static TLS block. |
| [`R_390_20`](#r_390_20) | const | Direct 20 bit. |
| [`R_390_GOT20`](#r_390_got20) | const | 20 bit GOT offset. |
| [`R_390_GOTPLT20`](#r_390_gotplt20) | const | 20 bit offset to jump slot. |
| [`R_390_TLS_GOTIE20`](#r_390_tls_gotie20) | const | 20 bit GOT offset for static TLS block offset. |
| [`R_390_IRELATIVE`](#r_390_irelative) | const | STT_GNU_IFUNC relocation. |
| [`R_CRIS_NONE`](#r_cris_none) | const |  |
| [`R_CRIS_8`](#r_cris_8) | const |  |
| [`R_CRIS_16`](#r_cris_16) | const |  |
| [`R_CRIS_32`](#r_cris_32) | const |  |
| [`R_CRIS_8_PCREL`](#r_cris_8_pcrel) | const |  |
| [`R_CRIS_16_PCREL`](#r_cris_16_pcrel) | const |  |
| [`R_CRIS_32_PCREL`](#r_cris_32_pcrel) | const |  |
| [`R_CRIS_GNU_VTINHERIT`](#r_cris_gnu_vtinherit) | const |  |
| [`R_CRIS_GNU_VTENTRY`](#r_cris_gnu_vtentry) | const |  |
| [`R_CRIS_COPY`](#r_cris_copy) | const |  |
| [`R_CRIS_GLOB_DAT`](#r_cris_glob_dat) | const |  |
| [`R_CRIS_JUMP_SLOT`](#r_cris_jump_slot) | const |  |
| [`R_CRIS_RELATIVE`](#r_cris_relative) | const |  |
| [`R_CRIS_16_GOT`](#r_cris_16_got) | const |  |
| [`R_CRIS_32_GOT`](#r_cris_32_got) | const |  |
| [`R_CRIS_16_GOTPLT`](#r_cris_16_gotplt) | const |  |
| [`R_CRIS_32_GOTPLT`](#r_cris_32_gotplt) | const |  |
| [`R_CRIS_32_GOTREL`](#r_cris_32_gotrel) | const |  |
| [`R_CRIS_32_PLT_GOTREL`](#r_cris_32_plt_gotrel) | const |  |
| [`R_CRIS_32_PLT_PCREL`](#r_cris_32_plt_pcrel) | const |  |
| [`R_X86_64_NONE`](#r_x86_64_none) | const | No reloc |
| [`R_X86_64_64`](#r_x86_64_64) | const | Direct 64 bit |
| [`R_X86_64_PC32`](#r_x86_64_pc32) | const | PC relative 32 bit signed |
| [`R_X86_64_GOT32`](#r_x86_64_got32) | const | 32 bit GOT entry |
| [`R_X86_64_PLT32`](#r_x86_64_plt32) | const | 32 bit PLT address |
| [`R_X86_64_COPY`](#r_x86_64_copy) | const | Copy symbol at runtime |
| [`R_X86_64_GLOB_DAT`](#r_x86_64_glob_dat) | const | Create GOT entry |
| [`R_X86_64_JUMP_SLOT`](#r_x86_64_jump_slot) | const | Create PLT entry |
| [`R_X86_64_RELATIVE`](#r_x86_64_relative) | const | Adjust by program base |
| [`R_X86_64_GOTPCREL`](#r_x86_64_gotpcrel) | const | 32 bit signed PC relative offset to GOT |
| [`R_X86_64_32`](#r_x86_64_32) | const | Direct 32 bit zero extended |
| [`R_X86_64_32S`](#r_x86_64_32s) | const | Direct 32 bit sign extended |
| [`R_X86_64_16`](#r_x86_64_16) | const | Direct 16 bit zero extended |
| [`R_X86_64_PC16`](#r_x86_64_pc16) | const | 16 bit sign extended pc relative |
| [`R_X86_64_8`](#r_x86_64_8) | const | Direct 8 bit sign extended |
| [`R_X86_64_PC8`](#r_x86_64_pc8) | const | 8 bit sign extended pc relative |
| [`R_X86_64_DTPMOD64`](#r_x86_64_dtpmod64) | const | ID of module containing symbol |
| [`R_X86_64_DTPOFF64`](#r_x86_64_dtpoff64) | const | Offset in module's TLS block |
| [`R_X86_64_TPOFF64`](#r_x86_64_tpoff64) | const | Offset in initial TLS block |
| [`R_X86_64_TLSGD`](#r_x86_64_tlsgd) | const | 32 bit signed PC relative offset to two GOT entries for GD symbol |
| [`R_X86_64_TLSLD`](#r_x86_64_tlsld) | const | 32 bit signed PC relative offset to two GOT entries for LD symbol |
| [`R_X86_64_DTPOFF32`](#r_x86_64_dtpoff32) | const | Offset in TLS block |
| [`R_X86_64_GOTTPOFF`](#r_x86_64_gottpoff) | const | 32 bit signed PC relative offset to GOT entry for IE symbol |
| [`R_X86_64_TPOFF32`](#r_x86_64_tpoff32) | const | Offset in initial TLS block |
| [`R_X86_64_PC64`](#r_x86_64_pc64) | const | PC relative 64 bit |
| [`R_X86_64_GOTOFF64`](#r_x86_64_gotoff64) | const | 64 bit offset to GOT |
| [`R_X86_64_GOTPC32`](#r_x86_64_gotpc32) | const | 32 bit signed pc relative offset to GOT |
| [`R_X86_64_GOT64`](#r_x86_64_got64) | const | 64-bit GOT entry offset |
| [`R_X86_64_GOTPCREL64`](#r_x86_64_gotpcrel64) | const | 64-bit PC relative offset to GOT entry |
| [`R_X86_64_GOTPC64`](#r_x86_64_gotpc64) | const | 64-bit PC relative offset to GOT |
| [`R_X86_64_GOTPLT64`](#r_x86_64_gotplt64) | const | like GOT64, says PLT entry needed |
| [`R_X86_64_PLTOFF64`](#r_x86_64_pltoff64) | const | 64-bit GOT relative offset to PLT entry |
| [`R_X86_64_SIZE32`](#r_x86_64_size32) | const | Size of symbol plus 32-bit addend |
| [`R_X86_64_SIZE64`](#r_x86_64_size64) | const | Size of symbol plus 64-bit addend |
| [`R_X86_64_GOTPC32_TLSDESC`](#r_x86_64_gotpc32_tlsdesc) | const | GOT offset for TLS descriptor. |
| [`R_X86_64_TLSDESC_CALL`](#r_x86_64_tlsdesc_call) | const | Marker for call through TLS descriptor. |
| [`R_X86_64_TLSDESC`](#r_x86_64_tlsdesc) | const | TLS descriptor. |
| [`R_X86_64_IRELATIVE`](#r_x86_64_irelative) | const | Adjust indirectly by program base |
| [`R_X86_64_RELATIVE64`](#r_x86_64_relative64) | const | 64-bit adjust by program base |
| [`R_X86_64_GOTPCRELX`](#r_x86_64_gotpcrelx) | const | Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable. |
| [`R_X86_64_REX_GOTPCRELX`](#r_x86_64_rex_gotpcrelx) | const | Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable. |
| [`SHT_X86_64_UNWIND`](#sht_x86_64_unwind) | const | Unwind information. |
| [`R_MN10300_NONE`](#r_mn10300_none) | const | No reloc. |
| [`R_MN10300_32`](#r_mn10300_32) | const | Direct 32 bit. |
| [`R_MN10300_16`](#r_mn10300_16) | const | Direct 16 bit. |
| [`R_MN10300_8`](#r_mn10300_8) | const | Direct 8 bit. |
| [`R_MN10300_PCREL32`](#r_mn10300_pcrel32) | const | PC-relative 32-bit. |
| [`R_MN10300_PCREL16`](#r_mn10300_pcrel16) | const | PC-relative 16-bit signed. |
| [`R_MN10300_PCREL8`](#r_mn10300_pcrel8) | const | PC-relative 8-bit signed. |
| [`R_MN10300_GNU_VTINHERIT`](#r_mn10300_gnu_vtinherit) | const | Ancient C++ vtable garbage... |
| [`R_MN10300_GNU_VTENTRY`](#r_mn10300_gnu_vtentry) | const | ... |
| [`R_MN10300_24`](#r_mn10300_24) | const | Direct 24 bit. |
| [`R_MN10300_GOTPC32`](#r_mn10300_gotpc32) | const | 32-bit PCrel offset to GOT. |
| [`R_MN10300_GOTPC16`](#r_mn10300_gotpc16) | const | 16-bit PCrel offset to GOT. |
| [`R_MN10300_GOTOFF32`](#r_mn10300_gotoff32) | const | 32-bit offset from GOT. |
| [`R_MN10300_GOTOFF24`](#r_mn10300_gotoff24) | const | 24-bit offset from GOT. |
| [`R_MN10300_GOTOFF16`](#r_mn10300_gotoff16) | const | 16-bit offset from GOT. |
| [`R_MN10300_PLT32`](#r_mn10300_plt32) | const | 32-bit PCrel to PLT entry. |
| [`R_MN10300_PLT16`](#r_mn10300_plt16) | const | 16-bit PCrel to PLT entry. |
| [`R_MN10300_GOT32`](#r_mn10300_got32) | const | 32-bit offset to GOT entry. |
| [`R_MN10300_GOT24`](#r_mn10300_got24) | const | 24-bit offset to GOT entry. |
| [`R_MN10300_GOT16`](#r_mn10300_got16) | const | 16-bit offset to GOT entry. |
| [`R_MN10300_COPY`](#r_mn10300_copy) | const | Copy symbol at runtime. |
| [`R_MN10300_GLOB_DAT`](#r_mn10300_glob_dat) | const | Create GOT entry. |
| [`R_MN10300_JMP_SLOT`](#r_mn10300_jmp_slot) | const | Create PLT entry. |
| [`R_MN10300_RELATIVE`](#r_mn10300_relative) | const | Adjust by program base. |
| [`R_MN10300_TLS_GD`](#r_mn10300_tls_gd) | const | 32-bit offset for global dynamic. |
| [`R_MN10300_TLS_LD`](#r_mn10300_tls_ld) | const | 32-bit offset for local dynamic. |
| [`R_MN10300_TLS_LDO`](#r_mn10300_tls_ldo) | const | Module-relative offset. |
| [`R_MN10300_TLS_GOTIE`](#r_mn10300_tls_gotie) | const | GOT offset for static TLS block offset. |
| [`R_MN10300_TLS_IE`](#r_mn10300_tls_ie) | const | GOT address for static TLS block offset. |
| [`R_MN10300_TLS_LE`](#r_mn10300_tls_le) | const | Offset relative to static TLS block. |
| [`R_MN10300_TLS_DTPMOD`](#r_mn10300_tls_dtpmod) | const | ID of module containing symbol. |
| [`R_MN10300_TLS_DTPOFF`](#r_mn10300_tls_dtpoff) | const | Offset in module TLS block. |
| [`R_MN10300_TLS_TPOFF`](#r_mn10300_tls_tpoff) | const | Offset in static TLS block. |
| [`R_MN10300_SYM_DIFF`](#r_mn10300_sym_diff) | const | Adjustment for next reloc as needed by linker relaxation. |
| [`R_MN10300_ALIGN`](#r_mn10300_align) | const | Alignment requirement for linker relaxation. |
| [`R_M32R_NONE`](#r_m32r_none) | const | No reloc. |
| [`R_M32R_16`](#r_m32r_16) | const | Direct 16 bit. |
| [`R_M32R_32`](#r_m32r_32) | const | Direct 32 bit. |
| [`R_M32R_24`](#r_m32r_24) | const | Direct 24 bit. |
| [`R_M32R_10_PCREL`](#r_m32r_10_pcrel) | const | PC relative 10 bit shifted. |
| [`R_M32R_18_PCREL`](#r_m32r_18_pcrel) | const | PC relative 18 bit shifted. |
| [`R_M32R_26_PCREL`](#r_m32r_26_pcrel) | const | PC relative 26 bit shifted. |
| [`R_M32R_HI16_ULO`](#r_m32r_hi16_ulo) | const | High 16 bit with unsigned low. |
| [`R_M32R_HI16_SLO`](#r_m32r_hi16_slo) | const | High 16 bit with signed low. |
| [`R_M32R_LO16`](#r_m32r_lo16) | const | Low 16 bit. |
| [`R_M32R_SDA16`](#r_m32r_sda16) | const | 16 bit offset in SDA. |
| [`R_M32R_GNU_VTINHERIT`](#r_m32r_gnu_vtinherit) | const |  |
| [`R_M32R_GNU_VTENTRY`](#r_m32r_gnu_vtentry) | const |  |
| [`R_M32R_16_RELA`](#r_m32r_16_rela) | const | Direct 16 bit. |
| [`R_M32R_32_RELA`](#r_m32r_32_rela) | const | Direct 32 bit. |
| [`R_M32R_24_RELA`](#r_m32r_24_rela) | const | Direct 24 bit. |
| [`R_M32R_10_PCREL_RELA`](#r_m32r_10_pcrel_rela) | const | PC relative 10 bit shifted. |
| [`R_M32R_18_PCREL_RELA`](#r_m32r_18_pcrel_rela) | const | PC relative 18 bit shifted. |
| [`R_M32R_26_PCREL_RELA`](#r_m32r_26_pcrel_rela) | const | PC relative 26 bit shifted. |
| [`R_M32R_HI16_ULO_RELA`](#r_m32r_hi16_ulo_rela) | const | High 16 bit with unsigned low |
| [`R_M32R_HI16_SLO_RELA`](#r_m32r_hi16_slo_rela) | const | High 16 bit with signed low |
| [`R_M32R_LO16_RELA`](#r_m32r_lo16_rela) | const | Low 16 bit |
| [`R_M32R_SDA16_RELA`](#r_m32r_sda16_rela) | const | 16 bit offset in SDA |
| [`R_M32R_RELA_GNU_VTINHERIT`](#r_m32r_rela_gnu_vtinherit) | const |  |
| [`R_M32R_RELA_GNU_VTENTRY`](#r_m32r_rela_gnu_vtentry) | const |  |
| [`R_M32R_REL32`](#r_m32r_rel32) | const | PC relative 32 bit. |
| [`R_M32R_GOT24`](#r_m32r_got24) | const | 24 bit GOT entry |
| [`R_M32R_26_PLTREL`](#r_m32r_26_pltrel) | const | 26 bit PC relative to PLT shifted |
| [`R_M32R_COPY`](#r_m32r_copy) | const | Copy symbol at runtime |
| [`R_M32R_GLOB_DAT`](#r_m32r_glob_dat) | const | Create GOT entry |
| [`R_M32R_JMP_SLOT`](#r_m32r_jmp_slot) | const | Create PLT entry |
| [`R_M32R_RELATIVE`](#r_m32r_relative) | const | Adjust by program base |
| [`R_M32R_GOTOFF`](#r_m32r_gotoff) | const | 24 bit offset to GOT |
| [`R_M32R_GOTPC24`](#r_m32r_gotpc24) | const | 24 bit PC relative offset to GOT |
| [`R_M32R_GOT16_HI_ULO`](#r_m32r_got16_hi_ulo) | const | High 16 bit GOT entry with unsigned low |
| [`R_M32R_GOT16_HI_SLO`](#r_m32r_got16_hi_slo) | const | High 16 bit GOT entry with signed low |
| [`R_M32R_GOT16_LO`](#r_m32r_got16_lo) | const | Low 16 bit GOT entry |
| [`R_M32R_GOTPC_HI_ULO`](#r_m32r_gotpc_hi_ulo) | const | High 16 bit PC relative offset to GOT with unsigned low |
| [`R_M32R_GOTPC_HI_SLO`](#r_m32r_gotpc_hi_slo) | const | High 16 bit PC relative offset to GOT with signed low |
| [`R_M32R_GOTPC_LO`](#r_m32r_gotpc_lo) | const | Low 16 bit PC relative offset to GOT |
| [`R_M32R_GOTOFF_HI_ULO`](#r_m32r_gotoff_hi_ulo) | const | High 16 bit offset to GOT with unsigned low |
| [`R_M32R_GOTOFF_HI_SLO`](#r_m32r_gotoff_hi_slo) | const | High 16 bit offset to GOT with signed low |
| [`R_M32R_GOTOFF_LO`](#r_m32r_gotoff_lo) | const | Low 16 bit offset to GOT |
| [`R_M32R_NUM`](#r_m32r_num) | const | Keep this the last entry. |
| [`R_MICROBLAZE_NONE`](#r_microblaze_none) | const | No reloc. |
| [`R_MICROBLAZE_32`](#r_microblaze_32) | const | Direct 32 bit. |
| [`R_MICROBLAZE_32_PCREL`](#r_microblaze_32_pcrel) | const | PC relative 32 bit. |
| [`R_MICROBLAZE_64_PCREL`](#r_microblaze_64_pcrel) | const | PC relative 64 bit. |
| [`R_MICROBLAZE_32_PCREL_LO`](#r_microblaze_32_pcrel_lo) | const | Low 16 bits of PCREL32. |
| [`R_MICROBLAZE_64`](#r_microblaze_64) | const | Direct 64 bit. |
| [`R_MICROBLAZE_32_LO`](#r_microblaze_32_lo) | const | Low 16 bit. |
| [`R_MICROBLAZE_SRO32`](#r_microblaze_sro32) | const | Read-only small data area. |
| [`R_MICROBLAZE_SRW32`](#r_microblaze_srw32) | const | Read-write small data area. |
| [`R_MICROBLAZE_64_NONE`](#r_microblaze_64_none) | const | No reloc. |
| [`R_MICROBLAZE_32_SYM_OP_SYM`](#r_microblaze_32_sym_op_sym) | const | Symbol Op Symbol relocation. |
| [`R_MICROBLAZE_GNU_VTINHERIT`](#r_microblaze_gnu_vtinherit) | const | GNU C++ vtable hierarchy. |
| [`R_MICROBLAZE_GNU_VTENTRY`](#r_microblaze_gnu_vtentry) | const | GNU C++ vtable member usage. |
| [`R_MICROBLAZE_GOTPC_64`](#r_microblaze_gotpc_64) | const | PC-relative GOT offset. |
| [`R_MICROBLAZE_GOT_64`](#r_microblaze_got_64) | const | GOT entry offset. |
| [`R_MICROBLAZE_PLT_64`](#r_microblaze_plt_64) | const | PLT offset (PC-relative). |
| [`R_MICROBLAZE_REL`](#r_microblaze_rel) | const | Adjust by program base. |
| [`R_MICROBLAZE_JUMP_SLOT`](#r_microblaze_jump_slot) | const | Create PLT entry. |
| [`R_MICROBLAZE_GLOB_DAT`](#r_microblaze_glob_dat) | const | Create GOT entry. |
| [`R_MICROBLAZE_GOTOFF_64`](#r_microblaze_gotoff_64) | const | 64 bit offset to GOT. |
| [`R_MICROBLAZE_GOTOFF_32`](#r_microblaze_gotoff_32) | const | 32 bit offset to GOT. |
| [`R_MICROBLAZE_COPY`](#r_microblaze_copy) | const | Runtime copy. |
| [`R_MICROBLAZE_TLS`](#r_microblaze_tls) | const | TLS Reloc. |
| [`R_MICROBLAZE_TLSGD`](#r_microblaze_tlsgd) | const | TLS General Dynamic. |
| [`R_MICROBLAZE_TLSLD`](#r_microblaze_tlsld) | const | TLS Local Dynamic. |
| [`R_MICROBLAZE_TLSDTPMOD32`](#r_microblaze_tlsdtpmod32) | const | TLS Module ID. |
| [`R_MICROBLAZE_TLSDTPREL32`](#r_microblaze_tlsdtprel32) | const | TLS Offset Within TLS Block. |
| [`R_MICROBLAZE_TLSDTPREL64`](#r_microblaze_tlsdtprel64) | const | TLS Offset Within TLS Block. |
| [`R_MICROBLAZE_TLSGOTTPREL32`](#r_microblaze_tlsgottprel32) | const | TLS Offset From Thread Pointer. |
| [`R_MICROBLAZE_TLSTPREL32`](#r_microblaze_tlstprel32) | const | TLS Offset From Thread Pointer. |
| [`DT_NIOS2_GP`](#dt_nios2_gp) | const | Address of _gp. |
| [`R_NIOS2_NONE`](#r_nios2_none) | const | No reloc. |
| [`R_NIOS2_S16`](#r_nios2_s16) | const | Direct signed 16 bit. |
| [`R_NIOS2_U16`](#r_nios2_u16) | const | Direct unsigned 16 bit. |
| [`R_NIOS2_PCREL16`](#r_nios2_pcrel16) | const | PC relative 16 bit. |
| [`R_NIOS2_CALL26`](#r_nios2_call26) | const | Direct call. |
| [`R_NIOS2_IMM5`](#r_nios2_imm5) | const | 5 bit constant expression. |
| [`R_NIOS2_CACHE_OPX`](#r_nios2_cache_opx) | const | 5 bit expression, shift 22. |
| [`R_NIOS2_IMM6`](#r_nios2_imm6) | const | 6 bit constant expression. |
| [`R_NIOS2_IMM8`](#r_nios2_imm8) | const | 8 bit constant expression. |
| [`R_NIOS2_HI16`](#r_nios2_hi16) | const | High 16 bit. |
| [`R_NIOS2_LO16`](#r_nios2_lo16) | const | Low 16 bit. |
| [`R_NIOS2_HIADJ16`](#r_nios2_hiadj16) | const | High 16 bit, adjusted. |
| [`R_NIOS2_BFD_RELOC_32`](#r_nios2_bfd_reloc_32) | const | 32 bit symbol value + addend. |
| [`R_NIOS2_BFD_RELOC_16`](#r_nios2_bfd_reloc_16) | const | 16 bit symbol value + addend. |
| [`R_NIOS2_BFD_RELOC_8`](#r_nios2_bfd_reloc_8) | const | 8 bit symbol value + addend. |
| [`R_NIOS2_GPREL`](#r_nios2_gprel) | const | 16 bit GP pointer offset. |
| [`R_NIOS2_GNU_VTINHERIT`](#r_nios2_gnu_vtinherit) | const | GNU C++ vtable hierarchy. |
| [`R_NIOS2_GNU_VTENTRY`](#r_nios2_gnu_vtentry) | const | GNU C++ vtable member usage. |
| [`R_NIOS2_UJMP`](#r_nios2_ujmp) | const | Unconditional branch. |
| [`R_NIOS2_CJMP`](#r_nios2_cjmp) | const | Conditional branch. |
| [`R_NIOS2_CALLR`](#r_nios2_callr) | const | Indirect call through register. |
| [`R_NIOS2_ALIGN`](#r_nios2_align) | const | Alignment requirement for linker relaxation. |
| [`R_NIOS2_GOT16`](#r_nios2_got16) | const | 16 bit GOT entry. |
| [`R_NIOS2_CALL16`](#r_nios2_call16) | const | 16 bit GOT entry for function. |
| [`R_NIOS2_GOTOFF_LO`](#r_nios2_gotoff_lo) | const | %lo of offset to GOT pointer. |
| [`R_NIOS2_GOTOFF_HA`](#r_nios2_gotoff_ha) | const | %hiadj of offset to GOT pointer. |
| [`R_NIOS2_PCREL_LO`](#r_nios2_pcrel_lo) | const | %lo of PC relative offset. |
| [`R_NIOS2_PCREL_HA`](#r_nios2_pcrel_ha) | const | %hiadj of PC relative offset. |
| [`R_NIOS2_TLS_GD16`](#r_nios2_tls_gd16) | const | 16 bit GOT offset for TLS GD. |
| [`R_NIOS2_TLS_LDM16`](#r_nios2_tls_ldm16) | const | 16 bit GOT offset for TLS LDM. |
| [`R_NIOS2_TLS_LDO16`](#r_nios2_tls_ldo16) | const | 16 bit module relative offset. |
| [`R_NIOS2_TLS_IE16`](#r_nios2_tls_ie16) | const | 16 bit GOT offset for TLS IE. |
| [`R_NIOS2_TLS_LE16`](#r_nios2_tls_le16) | const | 16 bit LE TP-relative offset. |
| [`R_NIOS2_TLS_DTPMOD`](#r_nios2_tls_dtpmod) | const | Module number. |
| [`R_NIOS2_TLS_DTPREL`](#r_nios2_tls_dtprel) | const | Module-relative offset. |
| [`R_NIOS2_TLS_TPREL`](#r_nios2_tls_tprel) | const | TP-relative offset. |
| [`R_NIOS2_COPY`](#r_nios2_copy) | const | Copy symbol at runtime. |
| [`R_NIOS2_GLOB_DAT`](#r_nios2_glob_dat) | const | Create GOT entry. |
| [`R_NIOS2_JUMP_SLOT`](#r_nios2_jump_slot) | const | Create PLT entry. |
| [`R_NIOS2_RELATIVE`](#r_nios2_relative) | const | Adjust by program base. |
| [`R_NIOS2_GOTOFF`](#r_nios2_gotoff) | const | 16 bit offset to GOT pointer. |
| [`R_NIOS2_CALL26_NOAT`](#r_nios2_call26_noat) | const | Direct call in .noat section. |
| [`R_NIOS2_GOT_LO`](#r_nios2_got_lo) | const | %lo() of GOT entry. |
| [`R_NIOS2_GOT_HA`](#r_nios2_got_ha) | const | %hiadj() of GOT entry. |
| [`R_NIOS2_CALL_LO`](#r_nios2_call_lo) | const | %lo() of function GOT entry. |
| [`R_NIOS2_CALL_HA`](#r_nios2_call_ha) | const | %hiadj() of function GOT entry. |
| [`R_TILEPRO_NONE`](#r_tilepro_none) | const | No reloc |
| [`R_TILEPRO_32`](#r_tilepro_32) | const | Direct 32 bit |
| [`R_TILEPRO_16`](#r_tilepro_16) | const | Direct 16 bit |
| [`R_TILEPRO_8`](#r_tilepro_8) | const | Direct 8 bit |
| [`R_TILEPRO_32_PCREL`](#r_tilepro_32_pcrel) | const | PC relative 32 bit |
| [`R_TILEPRO_16_PCREL`](#r_tilepro_16_pcrel) | const | PC relative 16 bit |
| [`R_TILEPRO_8_PCREL`](#r_tilepro_8_pcrel) | const | PC relative 8 bit |
| [`R_TILEPRO_LO16`](#r_tilepro_lo16) | const | Low 16 bit |
| [`R_TILEPRO_HI16`](#r_tilepro_hi16) | const | High 16 bit |
| [`R_TILEPRO_HA16`](#r_tilepro_ha16) | const | High 16 bit, adjusted |
| [`R_TILEPRO_COPY`](#r_tilepro_copy) | const | Copy relocation |
| [`R_TILEPRO_GLOB_DAT`](#r_tilepro_glob_dat) | const | Create GOT entry |
| [`R_TILEPRO_JMP_SLOT`](#r_tilepro_jmp_slot) | const | Create PLT entry |
| [`R_TILEPRO_RELATIVE`](#r_tilepro_relative) | const | Adjust by program base |
| [`R_TILEPRO_BROFF_X1`](#r_tilepro_broff_x1) | const | X1 pipe branch offset |
| [`R_TILEPRO_JOFFLONG_X1`](#r_tilepro_jofflong_x1) | const | X1 pipe jump offset |
| [`R_TILEPRO_JOFFLONG_X1_PLT`](#r_tilepro_jofflong_x1_plt) | const | X1 pipe jump offset to PLT |
| [`R_TILEPRO_IMM8_X0`](#r_tilepro_imm8_x0) | const | X0 pipe 8-bit |
| [`R_TILEPRO_IMM8_Y0`](#r_tilepro_imm8_y0) | const | Y0 pipe 8-bit |
| [`R_TILEPRO_IMM8_X1`](#r_tilepro_imm8_x1) | const | X1 pipe 8-bit |
| [`R_TILEPRO_IMM8_Y1`](#r_tilepro_imm8_y1) | const | Y1 pipe 8-bit |
| [`R_TILEPRO_MT_IMM15_X1`](#r_tilepro_mt_imm15_x1) | const | X1 pipe mtspr |
| [`R_TILEPRO_MF_IMM15_X1`](#r_tilepro_mf_imm15_x1) | const | X1 pipe mfspr |
| [`R_TILEPRO_IMM16_X0`](#r_tilepro_imm16_x0) | const | X0 pipe 16-bit |
| [`R_TILEPRO_IMM16_X1`](#r_tilepro_imm16_x1) | const | X1 pipe 16-bit |
| [`R_TILEPRO_IMM16_X0_LO`](#r_tilepro_imm16_x0_lo) | const | X0 pipe low 16-bit |
| [`R_TILEPRO_IMM16_X1_LO`](#r_tilepro_imm16_x1_lo) | const | X1 pipe low 16-bit |
| [`R_TILEPRO_IMM16_X0_HI`](#r_tilepro_imm16_x0_hi) | const | X0 pipe high 16-bit |
| [`R_TILEPRO_IMM16_X1_HI`](#r_tilepro_imm16_x1_hi) | const | X1 pipe high 16-bit |
| [`R_TILEPRO_IMM16_X0_HA`](#r_tilepro_imm16_x0_ha) | const | X0 pipe high 16-bit, adjusted |
| [`R_TILEPRO_IMM16_X1_HA`](#r_tilepro_imm16_x1_ha) | const | X1 pipe high 16-bit, adjusted |
| [`R_TILEPRO_IMM16_X0_PCREL`](#r_tilepro_imm16_x0_pcrel) | const | X0 pipe PC relative 16 bit |
| [`R_TILEPRO_IMM16_X1_PCREL`](#r_tilepro_imm16_x1_pcrel) | const | X1 pipe PC relative 16 bit |
| [`R_TILEPRO_IMM16_X0_LO_PCREL`](#r_tilepro_imm16_x0_lo_pcrel) | const | X0 pipe PC relative low 16 bit |
| [`R_TILEPRO_IMM16_X1_LO_PCREL`](#r_tilepro_imm16_x1_lo_pcrel) | const | X1 pipe PC relative low 16 bit |
| [`R_TILEPRO_IMM16_X0_HI_PCREL`](#r_tilepro_imm16_x0_hi_pcrel) | const | X0 pipe PC relative high 16 bit |
| [`R_TILEPRO_IMM16_X1_HI_PCREL`](#r_tilepro_imm16_x1_hi_pcrel) | const | X1 pipe PC relative high 16 bit |
| [`R_TILEPRO_IMM16_X0_HA_PCREL`](#r_tilepro_imm16_x0_ha_pcrel) | const | X0 pipe PC relative ha() 16 bit |
| [`R_TILEPRO_IMM16_X1_HA_PCREL`](#r_tilepro_imm16_x1_ha_pcrel) | const | X1 pipe PC relative ha() 16 bit |
| [`R_TILEPRO_IMM16_X0_GOT`](#r_tilepro_imm16_x0_got) | const | X0 pipe 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X1_GOT`](#r_tilepro_imm16_x1_got) | const | X1 pipe 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X0_GOT_LO`](#r_tilepro_imm16_x0_got_lo) | const | X0 pipe low 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X1_GOT_LO`](#r_tilepro_imm16_x1_got_lo) | const | X1 pipe low 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X0_GOT_HI`](#r_tilepro_imm16_x0_got_hi) | const | X0 pipe high 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X1_GOT_HI`](#r_tilepro_imm16_x1_got_hi) | const | X1 pipe high 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X0_GOT_HA`](#r_tilepro_imm16_x0_got_ha) | const | X0 pipe ha() 16-bit GOT offset |
| [`R_TILEPRO_IMM16_X1_GOT_HA`](#r_tilepro_imm16_x1_got_ha) | const | X1 pipe ha() 16-bit GOT offset |
| [`R_TILEPRO_MMSTART_X0`](#r_tilepro_mmstart_x0) | const | X0 pipe mm "start" |
| [`R_TILEPRO_MMEND_X0`](#r_tilepro_mmend_x0) | const | X0 pipe mm "end" |
| [`R_TILEPRO_MMSTART_X1`](#r_tilepro_mmstart_x1) | const | X1 pipe mm "start" |
| [`R_TILEPRO_MMEND_X1`](#r_tilepro_mmend_x1) | const | X1 pipe mm "end" |
| [`R_TILEPRO_SHAMT_X0`](#r_tilepro_shamt_x0) | const | X0 pipe shift amount |
| [`R_TILEPRO_SHAMT_X1`](#r_tilepro_shamt_x1) | const | X1 pipe shift amount |
| [`R_TILEPRO_SHAMT_Y0`](#r_tilepro_shamt_y0) | const | Y0 pipe shift amount |
| [`R_TILEPRO_SHAMT_Y1`](#r_tilepro_shamt_y1) | const | Y1 pipe shift amount |
| [`R_TILEPRO_DEST_IMM8_X1`](#r_tilepro_dest_imm8_x1) | const | X1 pipe destination 8-bit |
| [`R_TILEPRO_TLS_GD_CALL`](#r_tilepro_tls_gd_call) | const | "jal" for TLS GD |
| [`R_TILEPRO_IMM8_X0_TLS_GD_ADD`](#r_tilepro_imm8_x0_tls_gd_add) | const | X0 pipe "addi" for TLS GD |
| [`R_TILEPRO_IMM8_X1_TLS_GD_ADD`](#r_tilepro_imm8_x1_tls_gd_add) | const | X1 pipe "addi" for TLS GD |
| [`R_TILEPRO_IMM8_Y0_TLS_GD_ADD`](#r_tilepro_imm8_y0_tls_gd_add) | const | Y0 pipe "addi" for TLS GD |
| [`R_TILEPRO_IMM8_Y1_TLS_GD_ADD`](#r_tilepro_imm8_y1_tls_gd_add) | const | Y1 pipe "addi" for TLS GD |
| [`R_TILEPRO_TLS_IE_LOAD`](#r_tilepro_tls_ie_load) | const | "lw_tls" for TLS IE |
| [`R_TILEPRO_IMM16_X0_TLS_GD`](#r_tilepro_imm16_x0_tls_gd) | const | X0 pipe 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X1_TLS_GD`](#r_tilepro_imm16_x1_tls_gd) | const | X1 pipe 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X0_TLS_GD_LO`](#r_tilepro_imm16_x0_tls_gd_lo) | const | X0 pipe low 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X1_TLS_GD_LO`](#r_tilepro_imm16_x1_tls_gd_lo) | const | X1 pipe low 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X0_TLS_GD_HI`](#r_tilepro_imm16_x0_tls_gd_hi) | const | X0 pipe high 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X1_TLS_GD_HI`](#r_tilepro_imm16_x1_tls_gd_hi) | const | X1 pipe high 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X0_TLS_GD_HA`](#r_tilepro_imm16_x0_tls_gd_ha) | const | X0 pipe ha() 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X1_TLS_GD_HA`](#r_tilepro_imm16_x1_tls_gd_ha) | const | X1 pipe ha() 16-bit TLS GD offset |
| [`R_TILEPRO_IMM16_X0_TLS_IE`](#r_tilepro_imm16_x0_tls_ie) | const | X0 pipe 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X1_TLS_IE`](#r_tilepro_imm16_x1_tls_ie) | const | X1 pipe 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X0_TLS_IE_LO`](#r_tilepro_imm16_x0_tls_ie_lo) | const | X0 pipe low 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X1_TLS_IE_LO`](#r_tilepro_imm16_x1_tls_ie_lo) | const | X1 pipe low 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X0_TLS_IE_HI`](#r_tilepro_imm16_x0_tls_ie_hi) | const | X0 pipe high 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X1_TLS_IE_HI`](#r_tilepro_imm16_x1_tls_ie_hi) | const | X1 pipe high 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X0_TLS_IE_HA`](#r_tilepro_imm16_x0_tls_ie_ha) | const | X0 pipe ha() 16-bit TLS IE offset |
| [`R_TILEPRO_IMM16_X1_TLS_IE_HA`](#r_tilepro_imm16_x1_tls_ie_ha) | const | X1 pipe ha() 16-bit TLS IE offset |
| [`R_TILEPRO_TLS_DTPMOD32`](#r_tilepro_tls_dtpmod32) | const | ID of module containing symbol |
| [`R_TILEPRO_TLS_DTPOFF32`](#r_tilepro_tls_dtpoff32) | const | Offset in TLS block |
| [`R_TILEPRO_TLS_TPOFF32`](#r_tilepro_tls_tpoff32) | const | Offset in static TLS block |
| [`R_TILEPRO_IMM16_X0_TLS_LE`](#r_tilepro_imm16_x0_tls_le) | const | X0 pipe 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X1_TLS_LE`](#r_tilepro_imm16_x1_tls_le) | const | X1 pipe 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X0_TLS_LE_LO`](#r_tilepro_imm16_x0_tls_le_lo) | const | X0 pipe low 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X1_TLS_LE_LO`](#r_tilepro_imm16_x1_tls_le_lo) | const | X1 pipe low 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X0_TLS_LE_HI`](#r_tilepro_imm16_x0_tls_le_hi) | const | X0 pipe high 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X1_TLS_LE_HI`](#r_tilepro_imm16_x1_tls_le_hi) | const | X1 pipe high 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X0_TLS_LE_HA`](#r_tilepro_imm16_x0_tls_le_ha) | const | X0 pipe ha() 16-bit TLS LE offset |
| [`R_TILEPRO_IMM16_X1_TLS_LE_HA`](#r_tilepro_imm16_x1_tls_le_ha) | const | X1 pipe ha() 16-bit TLS LE offset |
| [`R_TILEPRO_GNU_VTINHERIT`](#r_tilepro_gnu_vtinherit) | const | GNU C++ vtable hierarchy |
| [`R_TILEPRO_GNU_VTENTRY`](#r_tilepro_gnu_vtentry) | const | GNU C++ vtable member usage |
| [`R_TILEGX_NONE`](#r_tilegx_none) | const | No reloc |
| [`R_TILEGX_64`](#r_tilegx_64) | const | Direct 64 bit |
| [`R_TILEGX_32`](#r_tilegx_32) | const | Direct 32 bit |
| [`R_TILEGX_16`](#r_tilegx_16) | const | Direct 16 bit |
| [`R_TILEGX_8`](#r_tilegx_8) | const | Direct 8 bit |
| [`R_TILEGX_64_PCREL`](#r_tilegx_64_pcrel) | const | PC relative 64 bit |
| [`R_TILEGX_32_PCREL`](#r_tilegx_32_pcrel) | const | PC relative 32 bit |
| [`R_TILEGX_16_PCREL`](#r_tilegx_16_pcrel) | const | PC relative 16 bit |
| [`R_TILEGX_8_PCREL`](#r_tilegx_8_pcrel) | const | PC relative 8 bit |
| [`R_TILEGX_HW0`](#r_tilegx_hw0) | const | hword 0 16-bit |
| [`R_TILEGX_HW1`](#r_tilegx_hw1) | const | hword 1 16-bit |
| [`R_TILEGX_HW2`](#r_tilegx_hw2) | const | hword 2 16-bit |
| [`R_TILEGX_HW3`](#r_tilegx_hw3) | const | hword 3 16-bit |
| [`R_TILEGX_HW0_LAST`](#r_tilegx_hw0_last) | const | last hword 0 16-bit |
| [`R_TILEGX_HW1_LAST`](#r_tilegx_hw1_last) | const | last hword 1 16-bit |
| [`R_TILEGX_HW2_LAST`](#r_tilegx_hw2_last) | const | last hword 2 16-bit |
| [`R_TILEGX_COPY`](#r_tilegx_copy) | const | Copy relocation |
| [`R_TILEGX_GLOB_DAT`](#r_tilegx_glob_dat) | const | Create GOT entry |
| [`R_TILEGX_JMP_SLOT`](#r_tilegx_jmp_slot) | const | Create PLT entry |
| [`R_TILEGX_RELATIVE`](#r_tilegx_relative) | const | Adjust by program base |
| [`R_TILEGX_BROFF_X1`](#r_tilegx_broff_x1) | const | X1 pipe branch offset |
| [`R_TILEGX_JUMPOFF_X1`](#r_tilegx_jumpoff_x1) | const | X1 pipe jump offset |
| [`R_TILEGX_JUMPOFF_X1_PLT`](#r_tilegx_jumpoff_x1_plt) | const | X1 pipe jump offset to PLT |
| [`R_TILEGX_IMM8_X0`](#r_tilegx_imm8_x0) | const | X0 pipe 8-bit |
| [`R_TILEGX_IMM8_Y0`](#r_tilegx_imm8_y0) | const | Y0 pipe 8-bit |
| [`R_TILEGX_IMM8_X1`](#r_tilegx_imm8_x1) | const | X1 pipe 8-bit |
| [`R_TILEGX_IMM8_Y1`](#r_tilegx_imm8_y1) | const | Y1 pipe 8-bit |
| [`R_TILEGX_DEST_IMM8_X1`](#r_tilegx_dest_imm8_x1) | const | X1 pipe destination 8-bit |
| [`R_TILEGX_MT_IMM14_X1`](#r_tilegx_mt_imm14_x1) | const | X1 pipe mtspr |
| [`R_TILEGX_MF_IMM14_X1`](#r_tilegx_mf_imm14_x1) | const | X1 pipe mfspr |
| [`R_TILEGX_MMSTART_X0`](#r_tilegx_mmstart_x0) | const | X0 pipe mm "start" |
| [`R_TILEGX_MMEND_X0`](#r_tilegx_mmend_x0) | const | X0 pipe mm "end" |
| [`R_TILEGX_SHAMT_X0`](#r_tilegx_shamt_x0) | const | X0 pipe shift amount |
| [`R_TILEGX_SHAMT_X1`](#r_tilegx_shamt_x1) | const | X1 pipe shift amount |
| [`R_TILEGX_SHAMT_Y0`](#r_tilegx_shamt_y0) | const | Y0 pipe shift amount |
| [`R_TILEGX_SHAMT_Y1`](#r_tilegx_shamt_y1) | const | Y1 pipe shift amount |
| [`R_TILEGX_IMM16_X0_HW0`](#r_tilegx_imm16_x0_hw0) | const | X0 pipe hword 0 |
| [`R_TILEGX_IMM16_X1_HW0`](#r_tilegx_imm16_x1_hw0) | const | X1 pipe hword 0 |
| [`R_TILEGX_IMM16_X0_HW1`](#r_tilegx_imm16_x0_hw1) | const | X0 pipe hword 1 |
| [`R_TILEGX_IMM16_X1_HW1`](#r_tilegx_imm16_x1_hw1) | const | X1 pipe hword 1 |
| [`R_TILEGX_IMM16_X0_HW2`](#r_tilegx_imm16_x0_hw2) | const | X0 pipe hword 2 |
| [`R_TILEGX_IMM16_X1_HW2`](#r_tilegx_imm16_x1_hw2) | const | X1 pipe hword 2 |
| [`R_TILEGX_IMM16_X0_HW3`](#r_tilegx_imm16_x0_hw3) | const | X0 pipe hword 3 |
| [`R_TILEGX_IMM16_X1_HW3`](#r_tilegx_imm16_x1_hw3) | const | X1 pipe hword 3 |
| [`R_TILEGX_IMM16_X0_HW0_LAST`](#r_tilegx_imm16_x0_hw0_last) | const | X0 pipe last hword 0 |
| [`R_TILEGX_IMM16_X1_HW0_LAST`](#r_tilegx_imm16_x1_hw0_last) | const | X1 pipe last hword 0 |
| [`R_TILEGX_IMM16_X0_HW1_LAST`](#r_tilegx_imm16_x0_hw1_last) | const | X0 pipe last hword 1 |
| [`R_TILEGX_IMM16_X1_HW1_LAST`](#r_tilegx_imm16_x1_hw1_last) | const | X1 pipe last hword 1 |
| [`R_TILEGX_IMM16_X0_HW2_LAST`](#r_tilegx_imm16_x0_hw2_last) | const | X0 pipe last hword 2 |
| [`R_TILEGX_IMM16_X1_HW2_LAST`](#r_tilegx_imm16_x1_hw2_last) | const | X1 pipe last hword 2 |
| [`R_TILEGX_IMM16_X0_HW0_PCREL`](#r_tilegx_imm16_x0_hw0_pcrel) | const | X0 pipe PC relative hword 0 |
| [`R_TILEGX_IMM16_X1_HW0_PCREL`](#r_tilegx_imm16_x1_hw0_pcrel) | const | X1 pipe PC relative hword 0 |
| [`R_TILEGX_IMM16_X0_HW1_PCREL`](#r_tilegx_imm16_x0_hw1_pcrel) | const | X0 pipe PC relative hword 1 |
| [`R_TILEGX_IMM16_X1_HW1_PCREL`](#r_tilegx_imm16_x1_hw1_pcrel) | const | X1 pipe PC relative hword 1 |
| [`R_TILEGX_IMM16_X0_HW2_PCREL`](#r_tilegx_imm16_x0_hw2_pcrel) | const | X0 pipe PC relative hword 2 |
| [`R_TILEGX_IMM16_X1_HW2_PCREL`](#r_tilegx_imm16_x1_hw2_pcrel) | const | X1 pipe PC relative hword 2 |
| [`R_TILEGX_IMM16_X0_HW3_PCREL`](#r_tilegx_imm16_x0_hw3_pcrel) | const | X0 pipe PC relative hword 3 |
| [`R_TILEGX_IMM16_X1_HW3_PCREL`](#r_tilegx_imm16_x1_hw3_pcrel) | const | X1 pipe PC relative hword 3 |
| [`R_TILEGX_IMM16_X0_HW0_LAST_PCREL`](#r_tilegx_imm16_x0_hw0_last_pcrel) | const | X0 pipe PC-rel last hword 0 |
| [`R_TILEGX_IMM16_X1_HW0_LAST_PCREL`](#r_tilegx_imm16_x1_hw0_last_pcrel) | const | X1 pipe PC-rel last hword 0 |
| [`R_TILEGX_IMM16_X0_HW1_LAST_PCREL`](#r_tilegx_imm16_x0_hw1_last_pcrel) | const | X0 pipe PC-rel last hword 1 |
| [`R_TILEGX_IMM16_X1_HW1_LAST_PCREL`](#r_tilegx_imm16_x1_hw1_last_pcrel) | const | X1 pipe PC-rel last hword 1 |
| [`R_TILEGX_IMM16_X0_HW2_LAST_PCREL`](#r_tilegx_imm16_x0_hw2_last_pcrel) | const | X0 pipe PC-rel last hword 2 |
| [`R_TILEGX_IMM16_X1_HW2_LAST_PCREL`](#r_tilegx_imm16_x1_hw2_last_pcrel) | const | X1 pipe PC-rel last hword 2 |
| [`R_TILEGX_IMM16_X0_HW0_GOT`](#r_tilegx_imm16_x0_hw0_got) | const | X0 pipe hword 0 GOT offset |
| [`R_TILEGX_IMM16_X1_HW0_GOT`](#r_tilegx_imm16_x1_hw0_got) | const | X1 pipe hword 0 GOT offset |
| [`R_TILEGX_IMM16_X0_HW0_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_plt_pcrel) | const | X0 pipe PC-rel PLT hword 0 |
| [`R_TILEGX_IMM16_X1_HW0_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_plt_pcrel) | const | X1 pipe PC-rel PLT hword 0 |
| [`R_TILEGX_IMM16_X0_HW1_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_plt_pcrel) | const | X0 pipe PC-rel PLT hword 1 |
| [`R_TILEGX_IMM16_X1_HW1_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_plt_pcrel) | const | X1 pipe PC-rel PLT hword 1 |
| [`R_TILEGX_IMM16_X0_HW2_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_plt_pcrel) | const | X0 pipe PC-rel PLT hword 2 |
| [`R_TILEGX_IMM16_X1_HW2_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_plt_pcrel) | const | X1 pipe PC-rel PLT hword 2 |
| [`R_TILEGX_IMM16_X0_HW0_LAST_GOT`](#r_tilegx_imm16_x0_hw0_last_got) | const | X0 pipe last hword 0 GOT offset |
| [`R_TILEGX_IMM16_X1_HW0_LAST_GOT`](#r_tilegx_imm16_x1_hw0_last_got) | const | X1 pipe last hword 0 GOT offset |
| [`R_TILEGX_IMM16_X0_HW1_LAST_GOT`](#r_tilegx_imm16_x0_hw1_last_got) | const | X0 pipe last hword 1 GOT offset |
| [`R_TILEGX_IMM16_X1_HW1_LAST_GOT`](#r_tilegx_imm16_x1_hw1_last_got) | const | X1 pipe last hword 1 GOT offset |
| [`R_TILEGX_IMM16_X0_HW3_PLT_PCREL`](#r_tilegx_imm16_x0_hw3_plt_pcrel) | const | X0 pipe PC-rel PLT hword 3 |
| [`R_TILEGX_IMM16_X1_HW3_PLT_PCREL`](#r_tilegx_imm16_x1_hw3_plt_pcrel) | const | X1 pipe PC-rel PLT hword 3 |
| [`R_TILEGX_IMM16_X0_HW0_TLS_GD`](#r_tilegx_imm16_x0_hw0_tls_gd) | const | X0 pipe hword 0 TLS GD offset |
| [`R_TILEGX_IMM16_X1_HW0_TLS_GD`](#r_tilegx_imm16_x1_hw0_tls_gd) | const | X1 pipe hword 0 TLS GD offset |
| [`R_TILEGX_IMM16_X0_HW0_TLS_LE`](#r_tilegx_imm16_x0_hw0_tls_le) | const | X0 pipe hword 0 TLS LE offset |
| [`R_TILEGX_IMM16_X1_HW0_TLS_LE`](#r_tilegx_imm16_x1_hw0_tls_le) | const | X1 pipe hword 0 TLS LE offset |
| [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw0_last_tls_le) | const | X0 pipe last hword 0 LE off |
| [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw0_last_tls_le) | const | X1 pipe last hword 0 LE off |
| [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw1_last_tls_le) | const | X0 pipe last hword 1 LE off |
| [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw1_last_tls_le) | const | X1 pipe last hword 1 LE off |
| [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw0_last_tls_gd) | const | X0 pipe last hword 0 GD off |
| [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw0_last_tls_gd) | const | X1 pipe last hword 0 GD off |
| [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw1_last_tls_gd) | const | X0 pipe last hword 1 GD off |
| [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw1_last_tls_gd) | const | X1 pipe last hword 1 GD off |
| [`R_TILEGX_IMM16_X0_HW0_TLS_IE`](#r_tilegx_imm16_x0_hw0_tls_ie) | const | X0 pipe hword 0 TLS IE offset |
| [`R_TILEGX_IMM16_X1_HW0_TLS_IE`](#r_tilegx_imm16_x1_hw0_tls_ie) | const | X1 pipe hword 0 TLS IE offset |
| [`R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_last_plt_pcrel) | const | X0 pipe PC-rel PLT last hword 0 |
| [`R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_last_plt_pcrel) | const | X1 pipe PC-rel PLT last hword 0 |
| [`R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_last_plt_pcrel) | const | X0 pipe PC-rel PLT last hword 1 |
| [`R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_last_plt_pcrel) | const | X1 pipe PC-rel PLT last hword 1 |
| [`R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_last_plt_pcrel) | const | X0 pipe PC-rel PLT last hword 2 |
| [`R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_last_plt_pcrel) | const | X1 pipe PC-rel PLT last hword 2 |
| [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw0_last_tls_ie) | const | X0 pipe last hword 0 IE off |
| [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw0_last_tls_ie) | const | X1 pipe last hword 0 IE off |
| [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw1_last_tls_ie) | const | X0 pipe last hword 1 IE off |
| [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw1_last_tls_ie) | const | X1 pipe last hword 1 IE off |
| [`R_TILEGX_TLS_DTPMOD64`](#r_tilegx_tls_dtpmod64) | const | 64-bit ID of symbol's module |
| [`R_TILEGX_TLS_DTPOFF64`](#r_tilegx_tls_dtpoff64) | const | 64-bit offset in TLS block |
| [`R_TILEGX_TLS_TPOFF64`](#r_tilegx_tls_tpoff64) | const | 64-bit offset in static TLS block |
| [`R_TILEGX_TLS_DTPMOD32`](#r_tilegx_tls_dtpmod32) | const | 32-bit ID of symbol's module |
| [`R_TILEGX_TLS_DTPOFF32`](#r_tilegx_tls_dtpoff32) | const | 32-bit offset in TLS block |
| [`R_TILEGX_TLS_TPOFF32`](#r_tilegx_tls_tpoff32) | const | 32-bit offset in static TLS block |
| [`R_TILEGX_TLS_GD_CALL`](#r_tilegx_tls_gd_call) | const | "jal" for TLS GD |
| [`R_TILEGX_IMM8_X0_TLS_GD_ADD`](#r_tilegx_imm8_x0_tls_gd_add) | const | X0 pipe "addi" for TLS GD |
| [`R_TILEGX_IMM8_X1_TLS_GD_ADD`](#r_tilegx_imm8_x1_tls_gd_add) | const | X1 pipe "addi" for TLS GD |
| [`R_TILEGX_IMM8_Y0_TLS_GD_ADD`](#r_tilegx_imm8_y0_tls_gd_add) | const | Y0 pipe "addi" for TLS GD |
| [`R_TILEGX_IMM8_Y1_TLS_GD_ADD`](#r_tilegx_imm8_y1_tls_gd_add) | const | Y1 pipe "addi" for TLS GD |
| [`R_TILEGX_TLS_IE_LOAD`](#r_tilegx_tls_ie_load) | const | "ld_tls" for TLS IE |
| [`R_TILEGX_IMM8_X0_TLS_ADD`](#r_tilegx_imm8_x0_tls_add) | const | X0 pipe "addi" for TLS GD/IE |
| [`R_TILEGX_IMM8_X1_TLS_ADD`](#r_tilegx_imm8_x1_tls_add) | const | X1 pipe "addi" for TLS GD/IE |
| [`R_TILEGX_IMM8_Y0_TLS_ADD`](#r_tilegx_imm8_y0_tls_add) | const | Y0 pipe "addi" for TLS GD/IE |
| [`R_TILEGX_IMM8_Y1_TLS_ADD`](#r_tilegx_imm8_y1_tls_add) | const | Y1 pipe "addi" for TLS GD/IE |
| [`R_TILEGX_GNU_VTINHERIT`](#r_tilegx_gnu_vtinherit) | const | GNU C++ vtable hierarchy |
| [`R_TILEGX_GNU_VTENTRY`](#r_tilegx_gnu_vtentry) | const | GNU C++ vtable member usage |
| [`EF_RISCV_RVC`](#ef_riscv_rvc) | const |  |
| [`EF_RISCV_FLOAT_ABI`](#ef_riscv_float_abi) | const |  |
| [`EF_RISCV_FLOAT_ABI_SOFT`](#ef_riscv_float_abi_soft) | const |  |
| [`EF_RISCV_FLOAT_ABI_SINGLE`](#ef_riscv_float_abi_single) | const |  |
| [`EF_RISCV_FLOAT_ABI_DOUBLE`](#ef_riscv_float_abi_double) | const |  |
| [`EF_RISCV_FLOAT_ABI_QUAD`](#ef_riscv_float_abi_quad) | const |  |
| [`EF_RISCV_RVE`](#ef_riscv_rve) | const |  |
| [`EF_RISCV_TSO`](#ef_riscv_tso) | const |  |
| [`EF_RISCV_RV64ILP32`](#ef_riscv_rv64ilp32) | const |  |
| [`SHT_RISCV_ATTRIBUTES`](#sht_riscv_attributes) | const | RISC-V attributes section. |
| [`R_RISCV_NONE`](#r_riscv_none) | const |  |
| [`R_RISCV_32`](#r_riscv_32) | const |  |
| [`R_RISCV_64`](#r_riscv_64) | const |  |
| [`R_RISCV_RELATIVE`](#r_riscv_relative) | const |  |
| [`R_RISCV_COPY`](#r_riscv_copy) | const |  |
| [`R_RISCV_JUMP_SLOT`](#r_riscv_jump_slot) | const |  |
| [`R_RISCV_TLS_DTPMOD32`](#r_riscv_tls_dtpmod32) | const |  |
| [`R_RISCV_TLS_DTPMOD64`](#r_riscv_tls_dtpmod64) | const |  |
| [`R_RISCV_TLS_DTPREL32`](#r_riscv_tls_dtprel32) | const |  |
| [`R_RISCV_TLS_DTPREL64`](#r_riscv_tls_dtprel64) | const |  |
| [`R_RISCV_TLS_TPREL32`](#r_riscv_tls_tprel32) | const |  |
| [`R_RISCV_TLS_TPREL64`](#r_riscv_tls_tprel64) | const |  |
| [`R_RISCV_TLSDESC`](#r_riscv_tlsdesc) | const |  |
| [`R_RISCV_BRANCH`](#r_riscv_branch) | const |  |
| [`R_RISCV_JAL`](#r_riscv_jal) | const |  |
| [`R_RISCV_CALL`](#r_riscv_call) | const |  |
| [`R_RISCV_CALL_PLT`](#r_riscv_call_plt) | const |  |
| [`R_RISCV_GOT_HI20`](#r_riscv_got_hi20) | const |  |
| [`R_RISCV_TLS_GOT_HI20`](#r_riscv_tls_got_hi20) | const |  |
| [`R_RISCV_TLS_GD_HI20`](#r_riscv_tls_gd_hi20) | const |  |
| [`R_RISCV_PCREL_HI20`](#r_riscv_pcrel_hi20) | const |  |
| [`R_RISCV_PCREL_LO12_I`](#r_riscv_pcrel_lo12_i) | const |  |
| [`R_RISCV_PCREL_LO12_S`](#r_riscv_pcrel_lo12_s) | const |  |
| [`R_RISCV_HI20`](#r_riscv_hi20) | const |  |
| [`R_RISCV_LO12_I`](#r_riscv_lo12_i) | const |  |
| [`R_RISCV_LO12_S`](#r_riscv_lo12_s) | const |  |
| [`R_RISCV_TPREL_HI20`](#r_riscv_tprel_hi20) | const |  |
| [`R_RISCV_TPREL_LO12_I`](#r_riscv_tprel_lo12_i) | const |  |
| [`R_RISCV_TPREL_LO12_S`](#r_riscv_tprel_lo12_s) | const |  |
| [`R_RISCV_TPREL_ADD`](#r_riscv_tprel_add) | const |  |
| [`R_RISCV_ADD8`](#r_riscv_add8) | const |  |
| [`R_RISCV_ADD16`](#r_riscv_add16) | const |  |
| [`R_RISCV_ADD32`](#r_riscv_add32) | const |  |
| [`R_RISCV_ADD64`](#r_riscv_add64) | const |  |
| [`R_RISCV_SUB8`](#r_riscv_sub8) | const |  |
| [`R_RISCV_SUB16`](#r_riscv_sub16) | const |  |
| [`R_RISCV_SUB32`](#r_riscv_sub32) | const |  |
| [`R_RISCV_SUB64`](#r_riscv_sub64) | const |  |
| [`R_RISCV_GOT32_PCREL`](#r_riscv_got32_pcrel) | const |  |
| [`R_RISCV_ALIGN`](#r_riscv_align) | const |  |
| [`R_RISCV_RVC_BRANCH`](#r_riscv_rvc_branch) | const |  |
| [`R_RISCV_RVC_JUMP`](#r_riscv_rvc_jump) | const |  |
| [`R_RISCV_RVC_LUI`](#r_riscv_rvc_lui) | const |  |
| [`R_RISCV_GPREL_I`](#r_riscv_gprel_i) | const |  |
| [`R_RISCV_GPREL_S`](#r_riscv_gprel_s) | const |  |
| [`R_RISCV_TPREL_I`](#r_riscv_tprel_i) | const |  |
| [`R_RISCV_TPREL_S`](#r_riscv_tprel_s) | const |  |
| [`R_RISCV_RELAX`](#r_riscv_relax) | const |  |
| [`R_RISCV_SUB6`](#r_riscv_sub6) | const |  |
| [`R_RISCV_SET6`](#r_riscv_set6) | const |  |
| [`R_RISCV_SET8`](#r_riscv_set8) | const |  |
| [`R_RISCV_SET16`](#r_riscv_set16) | const |  |
| [`R_RISCV_SET32`](#r_riscv_set32) | const |  |
| [`R_RISCV_32_PCREL`](#r_riscv_32_pcrel) | const |  |
| [`R_RISCV_IRELATIVE`](#r_riscv_irelative) | const |  |
| [`R_RISCV_PLT32`](#r_riscv_plt32) | const |  |
| [`R_RISCV_SET_ULEB128`](#r_riscv_set_uleb128) | const |  |
| [`R_RISCV_SUB_ULEB128`](#r_riscv_sub_uleb128) | const |  |
| [`R_RISCV_TLSDESC_HI20`](#r_riscv_tlsdesc_hi20) | const |  |
| [`R_RISCV_TLSDESC_LOAD_LO12`](#r_riscv_tlsdesc_load_lo12) | const |  |
| [`R_RISCV_TLSDESC_ADD_LO12`](#r_riscv_tlsdesc_add_lo12) | const |  |
| [`R_RISCV_TLSDESC_CALL`](#r_riscv_tlsdesc_call) | const |  |
| [`R_BPF_NONE`](#r_bpf_none) | const | No reloc |
| [`R_BPF_64_64`](#r_bpf_64_64) | const |  |
| [`R_BPF_64_32`](#r_bpf_64_32) | const |  |
| [`R_SBF_NONE`](#r_sbf_none) | const | No reloc |
| [`R_SBF_64_64`](#r_sbf_64_64) | const |  |
| [`R_SBF_64_32`](#r_sbf_64_32) | const |  |
| [`R_METAG_HIADDR16`](#r_metag_hiaddr16) | const |  |
| [`R_METAG_LOADDR16`](#r_metag_loaddr16) | const |  |
| [`R_METAG_ADDR32`](#r_metag_addr32) | const | 32bit absolute address |
| [`R_METAG_NONE`](#r_metag_none) | const | No reloc |
| [`R_METAG_RELBRANCH`](#r_metag_relbranch) | const |  |
| [`R_METAG_GETSETOFF`](#r_metag_getsetoff) | const |  |
| [`R_METAG_REG32OP1`](#r_metag_reg32op1) | const |  |
| [`R_METAG_REG32OP2`](#r_metag_reg32op2) | const |  |
| [`R_METAG_REG32OP3`](#r_metag_reg32op3) | const |  |
| [`R_METAG_REG16OP1`](#r_metag_reg16op1) | const |  |
| [`R_METAG_REG16OP2`](#r_metag_reg16op2) | const |  |
| [`R_METAG_REG16OP3`](#r_metag_reg16op3) | const |  |
| [`R_METAG_REG32OP4`](#r_metag_reg32op4) | const |  |
| [`R_METAG_HIOG`](#r_metag_hiog) | const |  |
| [`R_METAG_LOOG`](#r_metag_loog) | const |  |
| [`R_METAG_REL8`](#r_metag_rel8) | const |  |
| [`R_METAG_REL16`](#r_metag_rel16) | const |  |
| [`R_METAG_GNU_VTINHERIT`](#r_metag_gnu_vtinherit) | const |  |
| [`R_METAG_GNU_VTENTRY`](#r_metag_gnu_vtentry) | const |  |
| [`R_METAG_HI16_GOTOFF`](#r_metag_hi16_gotoff) | const |  |
| [`R_METAG_LO16_GOTOFF`](#r_metag_lo16_gotoff) | const |  |
| [`R_METAG_GETSET_GOTOFF`](#r_metag_getset_gotoff) | const |  |
| [`R_METAG_GETSET_GOT`](#r_metag_getset_got) | const |  |
| [`R_METAG_HI16_GOTPC`](#r_metag_hi16_gotpc) | const |  |
| [`R_METAG_LO16_GOTPC`](#r_metag_lo16_gotpc) | const |  |
| [`R_METAG_HI16_PLT`](#r_metag_hi16_plt) | const |  |
| [`R_METAG_LO16_PLT`](#r_metag_lo16_plt) | const |  |
| [`R_METAG_RELBRANCH_PLT`](#r_metag_relbranch_plt) | const |  |
| [`R_METAG_GOTOFF`](#r_metag_gotoff) | const |  |
| [`R_METAG_PLT`](#r_metag_plt) | const |  |
| [`R_METAG_COPY`](#r_metag_copy) | const |  |
| [`R_METAG_JMP_SLOT`](#r_metag_jmp_slot) | const |  |
| [`R_METAG_RELATIVE`](#r_metag_relative) | const |  |
| [`R_METAG_GLOB_DAT`](#r_metag_glob_dat) | const |  |
| [`R_METAG_TLS_GD`](#r_metag_tls_gd) | const |  |
| [`R_METAG_TLS_LDM`](#r_metag_tls_ldm) | const |  |
| [`R_METAG_TLS_LDO_HI16`](#r_metag_tls_ldo_hi16) | const |  |
| [`R_METAG_TLS_LDO_LO16`](#r_metag_tls_ldo_lo16) | const |  |
| [`R_METAG_TLS_LDO`](#r_metag_tls_ldo) | const |  |
| [`R_METAG_TLS_IE`](#r_metag_tls_ie) | const |  |
| [`R_METAG_TLS_IENONPIC`](#r_metag_tls_ienonpic) | const |  |
| [`R_METAG_TLS_IENONPIC_HI16`](#r_metag_tls_ienonpic_hi16) | const |  |
| [`R_METAG_TLS_IENONPIC_LO16`](#r_metag_tls_ienonpic_lo16) | const |  |
| [`R_METAG_TLS_TPOFF`](#r_metag_tls_tpoff) | const |  |
| [`R_METAG_TLS_DTPMOD`](#r_metag_tls_dtpmod) | const |  |
| [`R_METAG_TLS_DTPOFF`](#r_metag_tls_dtpoff) | const |  |
| [`R_METAG_TLS_LE`](#r_metag_tls_le) | const |  |
| [`R_METAG_TLS_LE_HI16`](#r_metag_tls_le_hi16) | const |  |
| [`R_METAG_TLS_LE_LO16`](#r_metag_tls_le_lo16) | const |  |
| [`R_NDS32_NONE`](#r_nds32_none) | const |  |
| [`R_NDS32_32_RELA`](#r_nds32_32_rela) | const |  |
| [`R_NDS32_COPY`](#r_nds32_copy) | const |  |
| [`R_NDS32_GLOB_DAT`](#r_nds32_glob_dat) | const |  |
| [`R_NDS32_JMP_SLOT`](#r_nds32_jmp_slot) | const |  |
| [`R_NDS32_RELATIVE`](#r_nds32_relative) | const |  |
| [`R_NDS32_TLS_TPOFF`](#r_nds32_tls_tpoff) | const |  |
| [`R_NDS32_TLS_DESC`](#r_nds32_tls_desc) | const |  |
| [`EF_LARCH_ABI_MODIFIER_MASK`](#ef_larch_abi_modifier_mask) | const | Additional properties of the base ABI type, including the FP calling |
| [`EF_LARCH_ABI_SOFT_FLOAT`](#ef_larch_abi_soft_float) | const | Uses GPRs and the stack for parameter passing |
| [`EF_LARCH_ABI_SINGLE_FLOAT`](#ef_larch_abi_single_float) | const | Uses GPRs, 32-bit FPRs and the stack for parameter passing |
| [`EF_LARCH_ABI_DOUBLE_FLOAT`](#ef_larch_abi_double_float) | const | Uses GPRs, 64-bit FPRs and the stack for parameter passing |
| [`EF_LARCH_OBJABI_V1`](#ef_larch_objabi_v1) | const | Uses relocation types directly writing to immediate slots |
| [`R_LARCH_NONE`](#r_larch_none) | const | No reloc |
| [`R_LARCH_32`](#r_larch_32) | const | Runtime address resolving |
| [`R_LARCH_64`](#r_larch_64) | const | Runtime address resolving |
| [`R_LARCH_RELATIVE`](#r_larch_relative) | const | Runtime fixup for load-address |
| [`R_LARCH_COPY`](#r_larch_copy) | const | Runtime memory copy in executable |
| [`R_LARCH_JUMP_SLOT`](#r_larch_jump_slot) | const | Runtime PLT supporting |
| [`R_LARCH_TLS_DTPMOD32`](#r_larch_tls_dtpmod32) | const | Runtime relocation for TLS-GD |
| [`R_LARCH_TLS_DTPMOD64`](#r_larch_tls_dtpmod64) | const | Runtime relocation for TLS-GD |
| [`R_LARCH_TLS_DTPREL32`](#r_larch_tls_dtprel32) | const | Runtime relocation for TLS-GD |
| [`R_LARCH_TLS_DTPREL64`](#r_larch_tls_dtprel64) | const | Runtime relocation for TLS-GD |
| [`R_LARCH_TLS_TPREL32`](#r_larch_tls_tprel32) | const | Runtime relocation for TLE-IE |
| [`R_LARCH_TLS_TPREL64`](#r_larch_tls_tprel64) | const | Runtime relocation for TLE-IE |
| [`R_LARCH_IRELATIVE`](#r_larch_irelative) | const | Runtime local indirect function resolving |
| [`R_LARCH_MARK_LA`](#r_larch_mark_la) | const | Mark la.abs: load absolute address for static link. |
| [`R_LARCH_MARK_PCREL`](#r_larch_mark_pcrel) | const | Mark external label branch: access PC relative address for static link. |
| [`R_LARCH_SOP_PUSH_PCREL`](#r_larch_sop_push_pcrel) | const | Push PC-relative offset |
| [`R_LARCH_SOP_PUSH_ABSOLUTE`](#r_larch_sop_push_absolute) | const | Push constant or absolute address |
| [`R_LARCH_SOP_PUSH_DUP`](#r_larch_sop_push_dup) | const | Duplicate stack top |
| [`R_LARCH_SOP_PUSH_GPREL`](#r_larch_sop_push_gprel) | const | Push for access GOT entry |
| [`R_LARCH_SOP_PUSH_TLS_TPREL`](#r_larch_sop_push_tls_tprel) | const | Push for TLS-LE |
| [`R_LARCH_SOP_PUSH_TLS_GOT`](#r_larch_sop_push_tls_got) | const | Push for TLS-IE |
| [`R_LARCH_SOP_PUSH_TLS_GD`](#r_larch_sop_push_tls_gd) | const | Push for TLS-GD |
| [`R_LARCH_SOP_PUSH_PLT_PCREL`](#r_larch_sop_push_plt_pcrel) | const | Push for external function calling |
| [`R_LARCH_SOP_ASSERT`](#r_larch_sop_assert) | const | Assert stack top |
| [`R_LARCH_SOP_NOT`](#r_larch_sop_not) | const | Stack top logical not (unary) |
| [`R_LARCH_SOP_SUB`](#r_larch_sop_sub) | const | Stack top subtraction (binary) |
| [`R_LARCH_SOP_SL`](#r_larch_sop_sl) | const | Stack top left shift (binary) |
| [`R_LARCH_SOP_SR`](#r_larch_sop_sr) | const | Stack top right shift (binary) |
| [`R_LARCH_SOP_ADD`](#r_larch_sop_add) | const | Stack top addition (binary) |
| [`R_LARCH_SOP_AND`](#r_larch_sop_and) | const | Stack top bitwise and (binary) |
| [`R_LARCH_SOP_IF_ELSE`](#r_larch_sop_if_else) | const | Stack top selection (tertiary) |
| [`R_LARCH_SOP_POP_32_S_10_5`](#r_larch_sop_pop_32_s_10_5) | const | Pop stack top to fill 5-bit signed immediate operand |
| [`R_LARCH_SOP_POP_32_U_10_12`](#r_larch_sop_pop_32_u_10_12) | const | Pop stack top to fill 12-bit unsigned immediate operand |
| [`R_LARCH_SOP_POP_32_S_10_12`](#r_larch_sop_pop_32_s_10_12) | const | Pop stack top to fill 12-bit signed immediate operand |
| [`R_LARCH_SOP_POP_32_S_10_16`](#r_larch_sop_pop_32_s_10_16) | const | Pop stack top to fill 16-bit signed immediate operand |
| [`R_LARCH_SOP_POP_32_S_10_16_S2`](#r_larch_sop_pop_32_s_10_16_s2) | const | Pop stack top to fill 18-bit signed immediate operand with two trailing |
| [`R_LARCH_SOP_POP_32_S_5_20`](#r_larch_sop_pop_32_s_5_20) | const | Pop stack top to fill 20-bit signed immediate operand |
| [`R_LARCH_SOP_POP_32_S_0_5_10_16_S2`](#r_larch_sop_pop_32_s_0_5_10_16_s2) | const | Pop stack top to fill 23-bit signed immediate operand with two trailing |
| [`R_LARCH_SOP_POP_32_S_0_10_10_16_S2`](#r_larch_sop_pop_32_s_0_10_10_16_s2) | const | Pop stack top to fill 28-bit signed immediate operand with two trailing |
| [`R_LARCH_SOP_POP_32_U`](#r_larch_sop_pop_32_u) | const | Pop stack top to fill an instruction |
| [`R_LARCH_ADD8`](#r_larch_add8) | const | 8-bit in-place addition |
| [`R_LARCH_ADD16`](#r_larch_add16) | const | 16-bit in-place addition |
| [`R_LARCH_ADD24`](#r_larch_add24) | const | 24-bit in-place addition |
| [`R_LARCH_ADD32`](#r_larch_add32) | const | 32-bit in-place addition |
| [`R_LARCH_ADD64`](#r_larch_add64) | const | 64-bit in-place addition |
| [`R_LARCH_SUB8`](#r_larch_sub8) | const | 8-bit in-place subtraction |
| [`R_LARCH_SUB16`](#r_larch_sub16) | const | 16-bit in-place subtraction |
| [`R_LARCH_SUB24`](#r_larch_sub24) | const | 24-bit in-place subtraction |
| [`R_LARCH_SUB32`](#r_larch_sub32) | const | 32-bit in-place subtraction |
| [`R_LARCH_SUB64`](#r_larch_sub64) | const | 64-bit in-place subtraction |
| [`R_LARCH_GNU_VTINHERIT`](#r_larch_gnu_vtinherit) | const | GNU C++ vtable hierarchy |
| [`R_LARCH_GNU_VTENTRY`](#r_larch_gnu_vtentry) | const | GNU C++ vtable member usage |
| [`R_LARCH_B16`](#r_larch_b16) | const | 18-bit PC-relative jump offset with two trailing zeros |
| [`R_LARCH_B21`](#r_larch_b21) | const | 23-bit PC-relative jump offset with two trailing zeros |
| [`R_LARCH_B26`](#r_larch_b26) | const | 28-bit PC-relative jump offset with two trailing zeros |
| [`R_LARCH_ABS_HI20`](#r_larch_abs_hi20) | const | 12..=31 bits of 32/64-bit absolute address |
| [`R_LARCH_ABS_LO12`](#r_larch_abs_lo12) | const | 0..=11 bits of 32/64-bit absolute address |
| [`R_LARCH_ABS64_LO20`](#r_larch_abs64_lo20) | const | 32..=51 bits of 64-bit absolute address |
| [`R_LARCH_ABS64_HI12`](#r_larch_abs64_hi12) | const | 52..=63 bits of 64-bit absolute address |
| [`R_LARCH_PCALA_HI20`](#r_larch_pcala_hi20) | const | The signed 32-bit offset `offs` from `PC & 0xfffff000` to |
| [`R_LARCH_PCALA_LO12`](#r_larch_pcala_lo12) | const | Same as R_LARCH_ABS_LO12. |
| [`R_LARCH_PCALA64_LO20`](#r_larch_pcala64_lo20) | const | 32..=51 bits of the 64-bit offset from the |
| [`R_LARCH_PCALA64_HI12`](#r_larch_pcala64_hi12) | const | 52..=63 bits of the 64-bit offset from the |
| [`R_LARCH_GOT_PC_HI20`](#r_larch_got_pc_hi20) | const | The signed 32-bit offset `offs` from `PC & 0xfffff000` to |
| [`R_LARCH_GOT_PC_LO12`](#r_larch_got_pc_lo12) | const | 0..=11 bits of the 32/64-bit offset from the |
| [`R_LARCH_GOT64_PC_LO20`](#r_larch_got64_pc_lo20) | const | 32..=51 bits of the 64-bit offset from the |
| [`R_LARCH_GOT64_PC_HI12`](#r_larch_got64_pc_hi12) | const | 52..=63 bits of the 64-bit offset from the |
| [`R_LARCH_GOT_HI20`](#r_larch_got_hi20) | const | 12..=31 bits of 32/64-bit GOT entry absolute address |
| [`R_LARCH_GOT_LO12`](#r_larch_got_lo12) | const | 0..=11 bits of 32/64-bit GOT entry absolute address |
| [`R_LARCH_GOT64_LO20`](#r_larch_got64_lo20) | const | 32..=51 bits of 64-bit GOT entry absolute address |
| [`R_LARCH_GOT64_HI12`](#r_larch_got64_hi12) | const | 52..=63 bits of 64-bit GOT entry absolute address |
| [`R_LARCH_TLS_LE_HI20`](#r_larch_tls_le_hi20) | const | 12..=31 bits of TLS LE 32/64-bit offset from thread pointer |
| [`R_LARCH_TLS_LE_LO12`](#r_larch_tls_le_lo12) | const | 0..=11 bits of TLS LE 32/64-bit offset from thread pointer |
| [`R_LARCH_TLS_LE64_LO20`](#r_larch_tls_le64_lo20) | const | 32..=51 bits of TLS LE 64-bit offset from thread pointer |
| [`R_LARCH_TLS_LE64_HI12`](#r_larch_tls_le64_hi12) | const | 52..=63 bits of TLS LE 64-bit offset from thread pointer |
| [`R_LARCH_TLS_IE_PC_HI20`](#r_larch_tls_ie_pc_hi20) | const | The signed 32-bit offset `offs` from `PC & 0xfffff000` to |
| [`R_LARCH_TLS_IE_PC_LO12`](#r_larch_tls_ie_pc_lo12) | const | 0..=12 bits of the 32/64-bit offset from the |
| [`R_LARCH_TLS_IE64_PC_LO20`](#r_larch_tls_ie64_pc_lo20) | const | 32..=51 bits of the 64-bit offset from the |
| [`R_LARCH_TLS_IE64_PC_HI12`](#r_larch_tls_ie64_pc_hi12) | const | 52..=63 bits of the 64-bit offset from the |
| [`R_LARCH_TLS_IE_HI20`](#r_larch_tls_ie_hi20) | const | 12..=31 bits of TLS IE GOT entry 32/64-bit absolute address |
| [`R_LARCH_TLS_IE_LO12`](#r_larch_tls_ie_lo12) | const | 0..=11 bits of TLS IE GOT entry 32/64-bit absolute address |
| [`R_LARCH_TLS_IE64_LO20`](#r_larch_tls_ie64_lo20) | const | 32..=51 bits of TLS IE GOT entry 64-bit absolute address |
| [`R_LARCH_TLS_IE64_HI12`](#r_larch_tls_ie64_hi12) | const | 51..=63 bits of TLS IE GOT entry 64-bit absolute address |
| [`R_LARCH_TLS_LD_PC_HI20`](#r_larch_tls_ld_pc_hi20) | const | 12..=31 bits of the offset from `PC` to `GP + GD + 0x800`, where |
| [`R_LARCH_TLS_LD_HI20`](#r_larch_tls_ld_hi20) | const | 12..=31 bits of TLS LD GOT entry 32/64-bit absolute address |
| [`R_LARCH_TLS_GD_PC_HI20`](#r_larch_tls_gd_pc_hi20) | const | 12..=31 bits of the 32/64-bit PC-relative offset to the PC-relative |
| [`R_LARCH_TLS_GD_HI20`](#r_larch_tls_gd_hi20) | const | 12..=31 bits of TLS GD GOT entry 32/64-bit absolute address |
| [`R_LARCH_32_PCREL`](#r_larch_32_pcrel) | const | 32-bit PC relative |
| [`R_LARCH_RELAX`](#r_larch_relax) | const | Paired with a normal relocation at the same address to indicate the |
| [`R_LARCH_DELETE`](#r_larch_delete) | const | Reserved |
| [`R_LARCH_ALIGN`](#r_larch_align) | const | Delete some bytes to ensure the instruction at PC + A aligned to |
| [`R_LARCH_PCREL20_S2`](#r_larch_pcrel20_s2) | const | 22-bit PC-relative offset with two trailing zeros |
| [`R_LARCH_CFA`](#r_larch_cfa) | const | Reserved |
| [`R_LARCH_ADD6`](#r_larch_add6) | const | 6-bit in-place addition |
| [`R_LARCH_SUB6`](#r_larch_sub6) | const | 6-bit in-place subtraction |
| [`R_LARCH_ADD_ULEB128`](#r_larch_add_uleb128) | const | LEB128 in-place addition |
| [`R_LARCH_SUB_ULEB128`](#r_larch_sub_uleb128) | const | LEB128 in-place subtraction |
| [`R_LARCH_64_PCREL`](#r_larch_64_pcrel) | const | 64-bit PC relative |
| [`R_LARCH_CALL36`](#r_larch_call36) | const | 18..=37 bits of `S + A - PC` into the `pcaddu18i` instruction at `PC` |
| [`R_LARCH_TLS_DESC_PC_HI20`](#r_larch_tls_desc_pc_hi20) | const | 12..=31 bits of 32/64-bit PC-relative offset to TLS DESC GOT entry |
| [`R_LARCH_TLS_DESC_PC_LO12`](#r_larch_tls_desc_pc_lo12) | const | 0..=11 bits of 32/64-bit TLS DESC GOT entry address |
| [`R_LARCH_TLS_DESC64_PC_LO20`](#r_larch_tls_desc64_pc_lo20) | const | 32..=51 bits of 64-bit PC-relative offset to TLS DESC GOT entry |
| [`R_LARCH_TLS_DESC64_PC_HI12`](#r_larch_tls_desc64_pc_hi12) | const | 52..=63 bits of 64-bit PC-relative offset to TLS DESC GOT entry |
| [`R_LARCH_TLS_DESC_HI20`](#r_larch_tls_desc_hi20) | const | 12..=31 bits of 32/64-bit TLS DESC GOT entry absolute address |
| [`R_LARCH_TLS_DESC_LO12`](#r_larch_tls_desc_lo12) | const | 0..=11 bits of 32/64-bit TLS DESC GOT entry absolute address |
| [`R_LARCH_TLS_DESC64_LO20`](#r_larch_tls_desc64_lo20) | const | 32..=51 bits of 64-bit TLS DESC GOT entry absolute address |
| [`R_LARCH_TLS_DESC64_HI12`](#r_larch_tls_desc64_hi12) | const | 52..=63 bits of 64-bit TLS DESC GOT entry absolute address |
| [`R_LARCH_TLS_DESC_LD`](#r_larch_tls_desc_ld) | const | Used on ld.{w,d} for TLS DESC to get the resolve function address |
| [`R_LARCH_TLS_DESC_CALL`](#r_larch_tls_desc_call) | const | Used on jirl for TLS DESC to call the resolve function |
| [`R_LARCH_TLS_LE_HI20_R`](#r_larch_tls_le_hi20_r) | const | 12..=31 bits of TLS LE 32/64-bit offset from TP register, can be relaxed |
| [`R_LARCH_TLS_LE_ADD_R`](#r_larch_tls_le_add_r) | const | TLS LE thread pointer usage, can be relaxed |
| [`R_LARCH_TLS_LE_LO12_R`](#r_larch_tls_le_lo12_r) | const | 0..=11 bits of TLS LE 32/64-bit offset from TP register, sign-extended |
| [`R_LARCH_TLS_LD_PCREL20_S2`](#r_larch_tls_ld_pcrel20_s2) | const | 22-bit PC-relative offset to TLS LD GOT entry |
| [`R_LARCH_TLS_GD_PCREL20_S2`](#r_larch_tls_gd_pcrel20_s2) | const | 22-bit PC-relative offset to TLS GD GOT entry |
| [`R_LARCH_TLS_DESC_PCREL20_S2`](#r_larch_tls_desc_pcrel20_s2) | const | 22-bit PC-relative offset to TLS DESC GOT entry |
| [`R_XTENSA_NONE`](#r_xtensa_none) | const |  |
| [`R_XTENSA_32`](#r_xtensa_32) | const |  |
| [`R_XTENSA_RTLD`](#r_xtensa_rtld) | const |  |
| [`R_XTENSA_GLOB_DAT`](#r_xtensa_glob_dat) | const |  |
| [`R_XTENSA_JMP_SLOT`](#r_xtensa_jmp_slot) | const |  |
| [`R_XTENSA_RELATIVE`](#r_xtensa_relative) | const |  |
| [`R_XTENSA_PLT`](#r_xtensa_plt) | const |  |
| [`R_XTENSA_OP0`](#r_xtensa_op0) | const |  |
| [`R_XTENSA_OP1`](#r_xtensa_op1) | const |  |
| [`R_XTENSA_OP2`](#r_xtensa_op2) | const |  |
| [`R_XTENSA_ASM_EXPAND`](#r_xtensa_asm_expand) | const |  |
| [`R_XTENSA_ASM_SIMPLIFY`](#r_xtensa_asm_simplify) | const |  |
| [`R_XTENSA_32_PCREL`](#r_xtensa_32_pcrel) | const |  |
| [`R_XTENSA_GNU_VTINHERIT`](#r_xtensa_gnu_vtinherit) | const |  |
| [`R_XTENSA_GNU_VTENTRY`](#r_xtensa_gnu_vtentry) | const |  |
| [`R_XTENSA_DIFF8`](#r_xtensa_diff8) | const |  |
| [`R_XTENSA_DIFF16`](#r_xtensa_diff16) | const |  |
| [`R_XTENSA_DIFF32`](#r_xtensa_diff32) | const |  |
| [`R_XTENSA_SLOT0_OP`](#r_xtensa_slot0_op) | const |  |
| [`R_XTENSA_SLOT1_OP`](#r_xtensa_slot1_op) | const |  |
| [`R_XTENSA_SLOT2_OP`](#r_xtensa_slot2_op) | const |  |
| [`R_XTENSA_SLOT3_OP`](#r_xtensa_slot3_op) | const |  |
| [`R_XTENSA_SLOT4_OP`](#r_xtensa_slot4_op) | const |  |
| [`R_XTENSA_SLOT5_OP`](#r_xtensa_slot5_op) | const |  |
| [`R_XTENSA_SLOT6_OP`](#r_xtensa_slot6_op) | const |  |
| [`R_XTENSA_SLOT7_OP`](#r_xtensa_slot7_op) | const |  |
| [`R_XTENSA_SLOT8_OP`](#r_xtensa_slot8_op) | const |  |
| [`R_XTENSA_SLOT9_OP`](#r_xtensa_slot9_op) | const |  |
| [`R_XTENSA_SLOT10_OP`](#r_xtensa_slot10_op) | const |  |
| [`R_XTENSA_SLOT11_OP`](#r_xtensa_slot11_op) | const |  |
| [`R_XTENSA_SLOT12_OP`](#r_xtensa_slot12_op) | const |  |
| [`R_XTENSA_SLOT13_OP`](#r_xtensa_slot13_op) | const |  |
| [`R_XTENSA_SLOT14_OP`](#r_xtensa_slot14_op) | const |  |
| [`R_XTENSA_SLOT0_ALT`](#r_xtensa_slot0_alt) | const |  |
| [`R_XTENSA_SLOT1_ALT`](#r_xtensa_slot1_alt) | const |  |
| [`R_XTENSA_SLOT2_ALT`](#r_xtensa_slot2_alt) | const |  |
| [`R_XTENSA_SLOT3_ALT`](#r_xtensa_slot3_alt) | const |  |
| [`R_XTENSA_SLOT4_ALT`](#r_xtensa_slot4_alt) | const |  |
| [`R_XTENSA_SLOT5_ALT`](#r_xtensa_slot5_alt) | const |  |
| [`R_XTENSA_SLOT6_ALT`](#r_xtensa_slot6_alt) | const |  |
| [`R_XTENSA_SLOT7_ALT`](#r_xtensa_slot7_alt) | const |  |
| [`R_XTENSA_SLOT8_ALT`](#r_xtensa_slot8_alt) | const |  |
| [`R_XTENSA_SLOT9_ALT`](#r_xtensa_slot9_alt) | const |  |
| [`R_XTENSA_SLOT10_ALT`](#r_xtensa_slot10_alt) | const |  |
| [`R_XTENSA_SLOT11_ALT`](#r_xtensa_slot11_alt) | const |  |
| [`R_XTENSA_SLOT12_ALT`](#r_xtensa_slot12_alt) | const |  |
| [`R_XTENSA_SLOT13_ALT`](#r_xtensa_slot13_alt) | const |  |
| [`R_XTENSA_SLOT14_ALT`](#r_xtensa_slot14_alt) | const |  |
| [`R_XTENSA_TLSDESC_FN`](#r_xtensa_tlsdesc_fn) | const |  |
| [`R_XTENSA_TLSDESC_ARG`](#r_xtensa_tlsdesc_arg) | const |  |
| [`R_XTENSA_TLS_DTPOFF`](#r_xtensa_tls_dtpoff) | const |  |
| [`R_XTENSA_TLS_TPOFF`](#r_xtensa_tls_tpoff) | const |  |
| [`R_XTENSA_TLS_FUNC`](#r_xtensa_tls_func) | const |  |
| [`R_XTENSA_TLS_ARG`](#r_xtensa_tls_arg) | const |  |
| [`R_XTENSA_TLS_CALL`](#r_xtensa_tls_call) | const |  |
| [`R_XTENSA_PDIFF8`](#r_xtensa_pdiff8) | const |  |
| [`R_XTENSA_PDIFF16`](#r_xtensa_pdiff16) | const |  |
| [`R_XTENSA_PDIFF32`](#r_xtensa_pdiff32) | const |  |
| [`R_XTENSA_NDIFF8`](#r_xtensa_ndiff8) | const |  |
| [`R_XTENSA_NDIFF16`](#r_xtensa_ndiff16) | const |  |
| [`R_XTENSA_NDIFF32`](#r_xtensa_ndiff32) | const |  |
| [`EF_E2K_IPD`](#ef_e2k_ipd) | const |  |
| [`EF_E2K_X86APP`](#ef_e2k_x86app) | const |  |
| [`EF_E2K_4MB_PAGES`](#ef_e2k_4mb_pages) | const |  |
| [`EF_E2K_INCOMPAT`](#ef_e2k_incompat) | const |  |
| [`EF_E2K_PM`](#ef_e2k_pm) | const |  |
| [`EF_E2K_PACK_SEGMENTS`](#ef_e2k_pack_segments) | const |  |
| [`E_E2K_MACH_BASE`](#e_e2k_mach_base) | const | -march=generic code. |
| [`E_E2K_MACH_EV1`](#e_e2k_mach_ev1) | const | -march=elbrus-v1 code. |
| [`E_E2K_MACH_EV2`](#e_e2k_mach_ev2) | const | -march=elbrus-v2 code. |
| [`E_E2K_MACH_EV3`](#e_e2k_mach_ev3) | const | -march=elbrus-v3 code. |
| [`E_E2K_MACH_EV4`](#e_e2k_mach_ev4) | const | -march=elbrus-v4 code. |
| [`E_E2K_MACH_EV5`](#e_e2k_mach_ev5) | const | -march=elbrus-v5 code. |
| [`E_E2K_MACH_EV6`](#e_e2k_mach_ev6) | const | -march=elbrus-v6 code. |
| [`E_E2K_MACH_EV7`](#e_e2k_mach_ev7) | const | -march=elbrus-v7 code. |
| [`E_E2K_MACH_8C`](#e_e2k_mach_8c) | const | -mtune=elbrus-8c code. |
| [`E_E2K_MACH_1CPLUS`](#e_e2k_mach_1cplus) | const | -mtune=elbrus-1c+ code. |
| [`E_E2K_MACH_12C`](#e_e2k_mach_12c) | const | -mtune=elbrus-12c code. |
| [`E_E2K_MACH_16C`](#e_e2k_mach_16c) | const | -mtune=elbrus-16c code. |
| [`E_E2K_MACH_2C3`](#e_e2k_mach_2c3) | const | -mtune=elbrus-2c3 code. |
| [`E_E2K_MACH_48C`](#e_e2k_mach_48c) | const | -mtune=elbrus-48c code. |
| [`E_E2K_MACH_8V7`](#e_e2k_mach_8v7) | const | -mtune=elbrus-8v7 code. |
| [`R_E2K_32_ABS`](#r_e2k_32_abs) | const | Direct 32 bit. |
| [`R_E2K_32_PC`](#r_e2k_32_pc) | const | PC relative 32 bit. |
| [`R_E2K_AP_GOT`](#r_e2k_ap_got) | const | 32-bit offset of AP GOT entry. |
| [`R_E2K_PL_GOT`](#r_e2k_pl_got) | const | 32-bit offset of PL GOT entry. |
| [`R_E2K_32_JMP_SLOT`](#r_e2k_32_jmp_slot) | const | Create PLT entry. |
| [`R_E2K_32_COPY`](#r_e2k_32_copy) | const | Copy relocation, 32-bit case. |
| [`R_E2K_32_RELATIVE`](#r_e2k_32_relative) | const | Adjust by program base, 32-bit case. |
| [`R_E2K_32_IRELATIVE`](#r_e2k_32_irelative) | const | Adjust indirectly by program base, 32-bit case. |
| [`R_E2K_32_SIZE`](#r_e2k_32_size) | const | Size of symbol plus 32-bit addend. |
| [`R_E2K_32_DYNOPT`](#r_e2k_32_dynopt) | const | Symbol value if resolved by the definition in the same |
| [`R_E2K_64_ABS`](#r_e2k_64_abs) | const | Direct 64 bit. |
| [`R_E2K_64_ABS_LIT`](#r_e2k_64_abs_lit) | const | Direct 64 bit for literal. |
| [`R_E2K_64_PC_LIT`](#r_e2k_64_pc_lit) | const | PC relative 64 bit for literal. |
| [`R_E2K_64_JMP_SLOT`](#r_e2k_64_jmp_slot) | const | Create PLT entry, 64-bit case. |
| [`R_E2K_64_COPY`](#r_e2k_64_copy) | const | Copy relocation, 64-bit case. |
| [`R_E2K_64_RELATIVE`](#r_e2k_64_relative) | const | Adjust by program base, 64-bit case. |
| [`R_E2K_64_RELATIVE_LIT`](#r_e2k_64_relative_lit) | const | Adjust by program base for literal, 64-bit case. |
| [`R_E2K_64_IRELATIVE`](#r_e2k_64_irelative) | const | Adjust indirectly by program base, 64-bit case. |
| [`R_E2K_64_SIZE`](#r_e2k_64_size) | const | Size of symbol plus 64-bit addend. |
| [`R_E2K_64_GOTOFF`](#r_e2k_64_gotoff) | const | 64-bit offset of the symbol from GOT. |
| [`R_E2K_TLS_GDMOD`](#r_e2k_tls_gdmod) | const | GOT entry for ID of module containing symbol. |
| [`R_E2K_TLS_GDREL`](#r_e2k_tls_gdrel) | const | GOT entry for offset in module TLS block. |
| [`R_E2K_TLS_IE`](#r_e2k_tls_ie) | const | Static TLS block offset GOT entry. |
| [`R_E2K_32_TLS_LE`](#r_e2k_32_tls_le) | const | Offset relative to static TLS block, 32-bit case. |
| [`R_E2K_64_TLS_LE`](#r_e2k_64_tls_le) | const | Offset relative to static TLS block, 64-bit case. |
| [`R_E2K_TLS_32_DTPMOD`](#r_e2k_tls_32_dtpmod) | const | ID of module containing symbol, 32-bit case. |
| [`R_E2K_TLS_32_DTPREL`](#r_e2k_tls_32_dtprel) | const | Offset in module TLS block, 32-bit case. |
| [`R_E2K_TLS_64_DTPMOD`](#r_e2k_tls_64_dtpmod) | const | ID of module containing symbol, 64-bit case. |
| [`R_E2K_TLS_64_DTPREL`](#r_e2k_tls_64_dtprel) | const | Offset in module TLS block, 64-bit case. |
| [`R_E2K_TLS_32_TPREL`](#r_e2k_tls_32_tprel) | const | Offset in static TLS block, 32-bit case. |
| [`R_E2K_TLS_64_TPREL`](#r_e2k_tls_64_tprel) | const | Offset in static TLS block, 64-bit case. |
| [`R_E2K_AP`](#r_e2k_ap) | const | Direct AP. |
| [`R_E2K_PL`](#r_e2k_pl) | const | Direct PL. |
| [`R_E2K_GOT`](#r_e2k_got) | const | 32-bit offset of the symbol's entry in GOT. |
| [`R_E2K_GOTOFF`](#r_e2k_gotoff) | const | 32-bit offset of the symbol from GOT. |
| [`R_E2K_DISP`](#r_e2k_disp) | const | PC relative 28 bit for DISP. |
| [`R_E2K_PREF`](#r_e2k_pref) | const | Prefetch insn line containing the label (symbol). |
| [`R_E2K_NONE`](#r_e2k_none) | const | No reloc. |
| [`R_E2K_GOTPLT`](#r_e2k_gotplt) | const | 32-bit offset of the symbol's entry in .got.plt. |
| [`R_E2K_ISLOCAL`](#r_e2k_islocal) | const | Is symbol resolved locally during the link. |
| [`R_E2K_ISLOCAL32`](#r_e2k_islocal32) | const | Is symbol resloved locally during the link. |
| [`R_E2K_64_GOTOFF_LIT`](#r_e2k_64_gotoff_lit) | const | The symbol's offset from GOT encoded within a 64-bit literal. |
| [`R_E2K_64_DYNOPT`](#r_e2k_64_dynopt) | const | Symbol value if resolved by the definition in the same |
| [`R_E2K_64_PC`](#r_e2k_64_pc) | const | PC relative 64 bit in data. |
| [`DT_E2K_LAZY`](#dt_e2k_lazy) | const |  |
| [`DT_E2K_LAZY_GOT`](#dt_e2k_lazy_got) | const |  |
| [`DT_E2K_INIT_GOT`](#dt_e2k_init_got) | const |  |
| [`DT_E2K_EXPORT_PL`](#dt_e2k_export_pl) | const |  |
| [`DT_E2K_EXPORT_PLSZ`](#dt_e2k_export_plsz) | const |  |
| [`DT_E2K_REAL_PLTGOT`](#dt_e2k_real_pltgot) | const |  |
| [`DT_E2K_NO_SELFINIT`](#dt_e2k_no_selfinit) | const |  |
| [`DT_E2K_NUM`](#dt_e2k_num) | const |  |
| [`Tag_File`](#tag_file) | const |  |
| [`Tag_Section`](#tag_section) | const |  |
| [`Tag_Symbol`](#tag_symbol) | const |  |

## Structs

### `FileHeader32<E: Endian>`

```rust
struct FileHeader32<E: Endian> {
    pub e_ident: Ident,
    pub e_type: crate::endian::U16<E>,
    pub e_machine: crate::endian::U16<E>,
    pub e_version: crate::endian::U32<E>,
    pub e_entry: crate::endian::U32<E>,
    pub e_phoff: crate::endian::U32<E>,
    pub e_shoff: crate::endian::U32<E>,
    pub e_flags: crate::endian::U32<E>,
    pub e_ehsize: crate::endian::U16<E>,
    pub e_phentsize: crate::endian::U16<E>,
    pub e_phnum: crate::endian::U16<E>,
    pub e_shentsize: crate::endian::U16<E>,
    pub e_shnum: crate::endian::U16<E>,
    pub e_shstrndx: crate::endian::U16<E>,
}
```

The header at the start of every 32-bit ELF file.

#### Fields

- **`e_ident`**: `Ident`

  Magic number and other information.

- **`e_type`**: `crate::endian::U16<E>`

  Object file type. One of the `ET_*` constants.

- **`e_machine`**: `crate::endian::U16<E>`

  Architecture. One of the `EM_*` constants.

- **`e_version`**: `crate::endian::U32<E>`

  Object file version. Must be `EV_CURRENT`.

- **`e_entry`**: `crate::endian::U32<E>`

  Entry point virtual address.

- **`e_phoff`**: `crate::endian::U32<E>`

  Program header table file offset.

- **`e_shoff`**: `crate::endian::U32<E>`

  Section header table file offset.

- **`e_flags`**: `crate::endian::U32<E>`

  Processor-specific flags.
  
  A combination of the `EF_*` constants.

- **`e_ehsize`**: `crate::endian::U16<E>`

  Size in bytes of this header.

- **`e_phentsize`**: `crate::endian::U16<E>`

  Program header table entry size.

- **`e_phnum`**: `crate::endian::U16<E>`

  Program header table entry count.
  
  If the count is greater than or equal to `PN_XNUM` then this field is set to
  `PN_XNUM` and the count is stored in the `sh_info` field of section 0.

- **`e_shentsize`**: `crate::endian::U16<E>`

  Section header table entry size.

- **`e_shnum`**: `crate::endian::U16<E>`

  Section header table entry count.
  
  If the count is greater than or equal to `SHN_LORESERVE` then this field is set to
  `0` and the count is stored in the `sh_size` field of section 0.
  first section header.

- **`e_shstrndx`**: `crate::endian::U16<E>`

  Section header string table index.
  
  If the index is greater than or equal to `SHN_LORESERVE` then this field is set to
  `SHN_XINDEX` and the index is stored in the `sh_link` field of section 0.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for FileHeader32<E>`

- <span id="fileheader32-clone"></span>`fn clone(&self) -> FileHeader32<E>` — [`FileHeader32`](#fileheader32)

##### `impl<E: marker::Copy + Endian> Copy for FileHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for FileHeader32<E>`

- <span id="fileheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> FileHeader for elf::FileHeader32<Endian>`

- <span id="elffileheader32-word"></span>`type Word = u32`

- <span id="elffileheader32-sword"></span>`type Sword = i32`

- <span id="elffileheader32-endian"></span>`type Endian = Endian`

- <span id="elffileheader32-programheader"></span>`type ProgramHeader = ProgramHeader32<Endian>`

- <span id="elffileheader32-sectionheader"></span>`type SectionHeader = SectionHeader32<Endian>`

- <span id="elffileheader32-compressionheader"></span>`type CompressionHeader = CompressionHeader32<Endian>`

- <span id="elffileheader32-noteheader"></span>`type NoteHeader = NoteHeader32<Endian>`

- <span id="elffileheader32-dyn"></span>`type Dyn = Dyn32<Endian>`

- <span id="elffileheader32-sym"></span>`type Sym = Sym32<Endian>`

- <span id="elffileheader32-rel"></span>`type Rel = Rel32<Endian>`

- <span id="elffileheader32-rela"></span>`type Rela = Rela32<Endian>`

- <span id="elffileheader32-relr"></span>`type Relr = Relr32<Endian>`

- <span id="elffileheader32-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="elffileheader32-is-type-64-sized"></span>`fn is_type_64_sized() -> bool`

- <span id="elffileheader32-e-ident"></span>`fn e_ident(&self) -> &elf::Ident` — [`Ident`](#ident)

- <span id="elffileheader32-e-type"></span>`fn e_type(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-machine"></span>`fn e_machine(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-version"></span>`fn e_version(&self, endian: <Self as >::Endian) -> u32` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-entry"></span>`fn e_entry(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-phoff"></span>`fn e_phoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-shoff"></span>`fn e_shoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-flags"></span>`fn e_flags(&self, endian: <Self as >::Endian) -> u32` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-ehsize"></span>`fn e_ehsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-phentsize"></span>`fn e_phentsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-phnum"></span>`fn e_phnum(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-shentsize"></span>`fn e_shentsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-shnum"></span>`fn e_shnum(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader32-e-shstrndx"></span>`fn e_shstrndx(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

##### `impl<E: Endian> Pod for FileHeader32<E>`

### `FileHeader64<E: Endian>`

```rust
struct FileHeader64<E: Endian> {
    pub e_ident: Ident,
    pub e_type: crate::endian::U16<E>,
    pub e_machine: crate::endian::U16<E>,
    pub e_version: crate::endian::U32<E>,
    pub e_entry: crate::endian::U64<E>,
    pub e_phoff: crate::endian::U64<E>,
    pub e_shoff: crate::endian::U64<E>,
    pub e_flags: crate::endian::U32<E>,
    pub e_ehsize: crate::endian::U16<E>,
    pub e_phentsize: crate::endian::U16<E>,
    pub e_phnum: crate::endian::U16<E>,
    pub e_shentsize: crate::endian::U16<E>,
    pub e_shnum: crate::endian::U16<E>,
    pub e_shstrndx: crate::endian::U16<E>,
}
```

The header at the start of every 64-bit ELF file.

#### Fields

- **`e_ident`**: `Ident`

  Magic number and other information.

- **`e_type`**: `crate::endian::U16<E>`

  Object file type. One of the `ET_*` constants.

- **`e_machine`**: `crate::endian::U16<E>`

  Architecture. One of the `EM_*` constants.

- **`e_version`**: `crate::endian::U32<E>`

  Object file version. Must be `EV_CURRENT`.

- **`e_entry`**: `crate::endian::U64<E>`

  Entry point virtual address.

- **`e_phoff`**: `crate::endian::U64<E>`

  Program header table file offset.

- **`e_shoff`**: `crate::endian::U64<E>`

  Section header table file offset.

- **`e_flags`**: `crate::endian::U32<E>`

  Processor-specific flags.
  
  A combination of the `EF_*` constants.

- **`e_ehsize`**: `crate::endian::U16<E>`

  Size in bytes of this header.

- **`e_phentsize`**: `crate::endian::U16<E>`

  Program header table entry size.

- **`e_phnum`**: `crate::endian::U16<E>`

  Program header table entry count.
  
  If the count is greater than or equal to `PN_XNUM` then this field is set to
  `PN_XNUM` and the count is stored in the `sh_info` field of section 0.

- **`e_shentsize`**: `crate::endian::U16<E>`

  Section header table entry size.

- **`e_shnum`**: `crate::endian::U16<E>`

  Section header table entry count.
  
  If the count is greater than or equal to `SHN_LORESERVE` then this field is set to
  `0` and the count is stored in the `sh_size` field of section 0.
  first section header.

- **`e_shstrndx`**: `crate::endian::U16<E>`

  Section header string table index.
  
  If the index is greater than or equal to `SHN_LORESERVE` then this field is set to
  `SHN_XINDEX` and the index is stored in the `sh_link` field of section 0.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for FileHeader64<E>`

- <span id="fileheader64-clone"></span>`fn clone(&self) -> FileHeader64<E>` — [`FileHeader64`](#fileheader64)

##### `impl<E: marker::Copy + Endian> Copy for FileHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for FileHeader64<E>`

- <span id="fileheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> FileHeader for elf::FileHeader64<Endian>`

- <span id="elffileheader64-word"></span>`type Word = u64`

- <span id="elffileheader64-sword"></span>`type Sword = i64`

- <span id="elffileheader64-endian"></span>`type Endian = Endian`

- <span id="elffileheader64-programheader"></span>`type ProgramHeader = ProgramHeader64<Endian>`

- <span id="elffileheader64-sectionheader"></span>`type SectionHeader = SectionHeader64<Endian>`

- <span id="elffileheader64-compressionheader"></span>`type CompressionHeader = CompressionHeader64<Endian>`

- <span id="elffileheader64-noteheader"></span>`type NoteHeader = NoteHeader32<Endian>`

- <span id="elffileheader64-dyn"></span>`type Dyn = Dyn64<Endian>`

- <span id="elffileheader64-sym"></span>`type Sym = Sym64<Endian>`

- <span id="elffileheader64-rel"></span>`type Rel = Rel64<Endian>`

- <span id="elffileheader64-rela"></span>`type Rela = Rela64<Endian>`

- <span id="elffileheader64-relr"></span>`type Relr = Relr64<Endian>`

- <span id="elffileheader64-is-type-64"></span>`fn is_type_64(&self) -> bool`

- <span id="elffileheader64-is-type-64-sized"></span>`fn is_type_64_sized() -> bool`

- <span id="elffileheader64-e-ident"></span>`fn e_ident(&self) -> &elf::Ident` — [`Ident`](#ident)

- <span id="elffileheader64-e-type"></span>`fn e_type(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-machine"></span>`fn e_machine(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-version"></span>`fn e_version(&self, endian: <Self as >::Endian) -> u32` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-entry"></span>`fn e_entry(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-phoff"></span>`fn e_phoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-shoff"></span>`fn e_shoff(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-flags"></span>`fn e_flags(&self, endian: <Self as >::Endian) -> u32` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-ehsize"></span>`fn e_ehsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-phentsize"></span>`fn e_phentsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-phnum"></span>`fn e_phnum(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-shentsize"></span>`fn e_shentsize(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-shnum"></span>`fn e_shnum(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

- <span id="elffileheader64-e-shstrndx"></span>`fn e_shstrndx(&self, endian: <Self as >::Endian) -> u16` — [`FileHeader`](../read/elf/index.md)

##### `impl<E: Endian> Pod for FileHeader64<E>`

### `Ident`

```rust
struct Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}
```

Magic number and other information.

Contained in the file header.

#### Fields

- **`magic`**: `[u8; 4]`

  Magic number. Must be `ELFMAG`.

- **`class`**: `u8`

  File class. One of the `ELFCLASS*` constants.

- **`data`**: `u8`

  Data encoding. One of the `ELFDATA*` constants.

- **`version`**: `u8`

  ELF version. Must be `EV_CURRENT`.

- **`os_abi`**: `u8`

  OS ABI identification. One of the `ELFOSABI*` constants.

- **`abi_version`**: `u8`

  ABI version.
  
  The meaning of this field depends on the `os_abi` value.

- **`padding`**: `[u8; 7]`

  Padding bytes.

#### Trait Implementations

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

##### `impl Copy for Ident`

##### `impl Debug for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionHeader32<E: Endian>`

```rust
struct SectionHeader32<E: Endian> {
    pub sh_name: crate::endian::U32<E>,
    pub sh_type: crate::endian::U32<E>,
    pub sh_flags: crate::endian::U32<E>,
    pub sh_addr: crate::endian::U32<E>,
    pub sh_offset: crate::endian::U32<E>,
    pub sh_size: crate::endian::U32<E>,
    pub sh_link: crate::endian::U32<E>,
    pub sh_info: crate::endian::U32<E>,
    pub sh_addralign: crate::endian::U32<E>,
    pub sh_entsize: crate::endian::U32<E>,
}
```

Section header.

#### Fields

- **`sh_name`**: `crate::endian::U32<E>`

  Section name.
  
  This is an offset into the section header string table.

- **`sh_type`**: `crate::endian::U32<E>`

  Section type. One of the `SHT_*` constants.

- **`sh_flags`**: `crate::endian::U32<E>`

  Section flags. A combination of the `SHF_*` constants.

- **`sh_addr`**: `crate::endian::U32<E>`

  Section virtual address at execution.

- **`sh_offset`**: `crate::endian::U32<E>`

  Section file offset.

- **`sh_size`**: `crate::endian::U32<E>`

  Section size in bytes.

- **`sh_link`**: `crate::endian::U32<E>`

  Link to another section.
  
  The section relationship depends on the `sh_type` value.

- **`sh_info`**: `crate::endian::U32<E>`

  Additional section information.
  
  The meaning of this field depends on the `sh_type` value.

- **`sh_addralign`**: `crate::endian::U32<E>`

  Section alignment.

- **`sh_entsize`**: `crate::endian::U32<E>`

  Entry size if the section holds a table.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for SectionHeader32<E>`

- <span id="sectionheader32-clone"></span>`fn clone(&self) -> SectionHeader32<E>` — [`SectionHeader32`](#sectionheader32)

##### `impl<E: marker::Copy + Endian> Copy for SectionHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SectionHeader32<E>`

- <span id="sectionheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for SectionHeader32<E>`

##### `impl<Endian: endian::Endian> SectionHeader for elf::SectionHeader32<Endian>`

- <span id="elfsectionheader32-elf"></span>`type Elf = FileHeader32<Endian>`

- <span id="elfsectionheader32-word"></span>`type Word = u32`

- <span id="elfsectionheader32-endian"></span>`type Endian = Endian`

- <span id="elfsectionheader32-sh-name"></span>`fn sh_name(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-type"></span>`fn sh_type(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-flags"></span>`fn sh_flags(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-addr"></span>`fn sh_addr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-offset"></span>`fn sh_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-size"></span>`fn sh_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-link"></span>`fn sh_link(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-info"></span>`fn sh_info(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-addralign"></span>`fn sh_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader32-sh-entsize"></span>`fn sh_entsize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

### `SectionHeader64<E: Endian>`

```rust
struct SectionHeader64<E: Endian> {
    pub sh_name: crate::endian::U32<E>,
    pub sh_type: crate::endian::U32<E>,
    pub sh_flags: crate::endian::U64<E>,
    pub sh_addr: crate::endian::U64<E>,
    pub sh_offset: crate::endian::U64<E>,
    pub sh_size: crate::endian::U64<E>,
    pub sh_link: crate::endian::U32<E>,
    pub sh_info: crate::endian::U32<E>,
    pub sh_addralign: crate::endian::U64<E>,
    pub sh_entsize: crate::endian::U64<E>,
}
```

Section header.

#### Fields

- **`sh_name`**: `crate::endian::U32<E>`

  Section name.
  
  This is an offset into the section header string table.

- **`sh_type`**: `crate::endian::U32<E>`

  Section type. One of the `SHT_*` constants.

- **`sh_flags`**: `crate::endian::U64<E>`

  Section flags. A combination of the `SHF_*` constants.

- **`sh_addr`**: `crate::endian::U64<E>`

  Section virtual address at execution.

- **`sh_offset`**: `crate::endian::U64<E>`

  Section file offset.

- **`sh_size`**: `crate::endian::U64<E>`

  Section size in bytes.

- **`sh_link`**: `crate::endian::U32<E>`

  Link to another section.
  
  The section relationship depends on the `sh_type` value.

- **`sh_info`**: `crate::endian::U32<E>`

  Additional section information.
  
  The meaning of this field depends on the `sh_type` value.

- **`sh_addralign`**: `crate::endian::U64<E>`

  Section alignment.

- **`sh_entsize`**: `crate::endian::U64<E>`

  Entry size if the section holds a table.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for SectionHeader64<E>`

- <span id="sectionheader64-clone"></span>`fn clone(&self) -> SectionHeader64<E>` — [`SectionHeader64`](#sectionheader64)

##### `impl<E: marker::Copy + Endian> Copy for SectionHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for SectionHeader64<E>`

- <span id="sectionheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for SectionHeader64<E>`

##### `impl<Endian: endian::Endian> SectionHeader for elf::SectionHeader64<Endian>`

- <span id="elfsectionheader64-word"></span>`type Word = u64`

- <span id="elfsectionheader64-endian"></span>`type Endian = Endian`

- <span id="elfsectionheader64-elf"></span>`type Elf = FileHeader64<Endian>`

- <span id="elfsectionheader64-sh-name"></span>`fn sh_name(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-type"></span>`fn sh_type(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-flags"></span>`fn sh_flags(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-addr"></span>`fn sh_addr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-offset"></span>`fn sh_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-size"></span>`fn sh_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-link"></span>`fn sh_link(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-info"></span>`fn sh_info(&self, endian: <Self as >::Endian) -> u32` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-addralign"></span>`fn sh_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

- <span id="elfsectionheader64-sh-entsize"></span>`fn sh_entsize(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`SectionHeader`](../read/elf/index.md)

### `CompressionHeader32<E: Endian>`

```rust
struct CompressionHeader32<E: Endian> {
    pub ch_type: crate::endian::U32Bytes<E>,
    pub ch_size: crate::endian::U32Bytes<E>,
    pub ch_addralign: crate::endian::U32Bytes<E>,
}
```

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

#### Fields

- **`ch_type`**: `crate::endian::U32Bytes<E>`

  Compression format. One of the `ELFCOMPRESS_*` values.

- **`ch_size`**: `crate::endian::U32Bytes<E>`

  Uncompressed data size.

- **`ch_addralign`**: `crate::endian::U32Bytes<E>`

  Uncompressed data alignment.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for CompressionHeader32<E>`

- <span id="compressionheader32-clone"></span>`fn clone(&self) -> CompressionHeader32<E>` — [`CompressionHeader32`](#compressionheader32)

##### `impl<Endian: endian::Endian> CompressionHeader for elf::CompressionHeader32<Endian>`

- <span id="elfcompressionheader32-word"></span>`type Word = u32`

- <span id="elfcompressionheader32-endian"></span>`type Endian = Endian`

- <span id="elfcompressionheader32-ch-type"></span>`fn ch_type(&self, endian: <Self as >::Endian) -> u32` — [`CompressionHeader`](../read/elf/index.md)

- <span id="elfcompressionheader32-ch-size"></span>`fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`CompressionHeader`](../read/elf/index.md)

- <span id="elfcompressionheader32-ch-addralign"></span>`fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`CompressionHeader`](../read/elf/index.md)

##### `impl<E: marker::Copy + Endian> Copy for CompressionHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for CompressionHeader32<E>`

- <span id="compressionheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for CompressionHeader32<E>`

- <span id="compressionheader32-default"></span>`fn default() -> CompressionHeader32<E>` — [`CompressionHeader32`](#compressionheader32)

##### `impl<E: Endian> Pod for CompressionHeader32<E>`

### `CompressionHeader64<E: Endian>`

```rust
struct CompressionHeader64<E: Endian> {
    pub ch_type: crate::endian::U32Bytes<E>,
    pub ch_reserved: crate::endian::U32Bytes<E>,
    pub ch_size: crate::endian::U64Bytes<E>,
    pub ch_addralign: crate::endian::U64Bytes<E>,
}
```

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

#### Fields

- **`ch_type`**: `crate::endian::U32Bytes<E>`

  Compression format. One of the `ELFCOMPRESS_*` values.

- **`ch_reserved`**: `crate::endian::U32Bytes<E>`

  Reserved.

- **`ch_size`**: `crate::endian::U64Bytes<E>`

  Uncompressed data size.

- **`ch_addralign`**: `crate::endian::U64Bytes<E>`

  Uncompressed data alignment.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for CompressionHeader64<E>`

- <span id="compressionheader64-clone"></span>`fn clone(&self) -> CompressionHeader64<E>` — [`CompressionHeader64`](#compressionheader64)

##### `impl<Endian: endian::Endian> CompressionHeader for elf::CompressionHeader64<Endian>`

- <span id="elfcompressionheader64-word"></span>`type Word = u64`

- <span id="elfcompressionheader64-endian"></span>`type Endian = Endian`

- <span id="elfcompressionheader64-ch-type"></span>`fn ch_type(&self, endian: <Self as >::Endian) -> u32` — [`CompressionHeader`](../read/elf/index.md)

- <span id="elfcompressionheader64-ch-size"></span>`fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`CompressionHeader`](../read/elf/index.md)

- <span id="elfcompressionheader64-ch-addralign"></span>`fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`CompressionHeader`](../read/elf/index.md)

##### `impl<E: marker::Copy + Endian> Copy for CompressionHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for CompressionHeader64<E>`

- <span id="compressionheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for CompressionHeader64<E>`

- <span id="compressionheader64-default"></span>`fn default() -> CompressionHeader64<E>` — [`CompressionHeader64`](#compressionheader64)

##### `impl<E: Endian> Pod for CompressionHeader64<E>`

### `Sym32<E: Endian>`

```rust
struct Sym32<E: Endian> {
    pub st_name: crate::endian::U32<E>,
    pub st_value: crate::endian::U32<E>,
    pub st_size: crate::endian::U32<E>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: crate::endian::U16<E>,
}
```

Symbol table entry.

#### Fields

- **`st_name`**: `crate::endian::U32<E>`

  Symbol name.
  
  This is an offset into the symbol string table.

- **`st_value`**: `crate::endian::U32<E>`

  Symbol value.

- **`st_size`**: `crate::endian::U32<E>`

  Symbol size.

- **`st_info`**: `u8`

  Symbol type and binding.
  
  Use the `st_type` and `st_bind` methods to access this value.

- **`st_other`**: `u8`

  Symbol visibility.
  
  Use the `st_visibility` method to access this value.

- **`st_shndx`**: `crate::endian::U16<E>`

  Section index or one of the `SHN_*` values.

#### Implementations

- <span id="sym32-st-bind"></span>`fn st_bind(&self) -> u8`

- <span id="sym32-st-type"></span>`fn st_type(&self) -> u8`

- <span id="sym32-set-st-info"></span>`fn set_st_info(&mut self, st_bind: u8, st_type: u8)`

- <span id="sym32-st-visibility"></span>`fn st_visibility(&self) -> u8`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Sym32<E>`

- <span id="sym32-clone"></span>`fn clone(&self) -> Sym32<E>` — [`Sym32`](#sym32)

##### `impl<E: marker::Copy + Endian> Copy for Sym32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Sym32<E>`

- <span id="sym32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for Sym32<E>`

- <span id="sym32-default"></span>`fn default() -> Sym32<E>` — [`Sym32`](#sym32)

##### `impl<E: Endian> Pod for Sym32<E>`

##### `impl<Endian: endian::Endian> Sym for elf::Sym32<Endian>`

- <span id="elfsym32-word"></span>`type Word = u32`

- <span id="elfsym32-endian"></span>`type Endian = Endian`

- <span id="elfsym32-st-name"></span>`fn st_name(&self, endian: <Self as >::Endian) -> u32` — [`Sym`](../read/elf/index.md)

- <span id="elfsym32-st-info"></span>`fn st_info(&self) -> u8`

- <span id="elfsym32-st-bind"></span>`fn st_bind(&self) -> u8`

- <span id="elfsym32-st-type"></span>`fn st_type(&self) -> u8`

- <span id="elfsym32-st-other"></span>`fn st_other(&self) -> u8`

- <span id="elfsym32-st-visibility"></span>`fn st_visibility(&self) -> u8`

- <span id="elfsym32-st-shndx"></span>`fn st_shndx(&self, endian: <Self as >::Endian) -> u16` — [`Sym`](../read/elf/index.md)

- <span id="elfsym32-st-value"></span>`fn st_value(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Sym`](../read/elf/index.md)

- <span id="elfsym32-st-size"></span>`fn st_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Sym`](../read/elf/index.md)

### `Sym64<E: Endian>`

```rust
struct Sym64<E: Endian> {
    pub st_name: crate::endian::U32<E>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: crate::endian::U16<E>,
    pub st_value: crate::endian::U64<E>,
    pub st_size: crate::endian::U64<E>,
}
```

Symbol table entry.

#### Fields

- **`st_name`**: `crate::endian::U32<E>`

  Symbol name.
  
  This is an offset into the symbol string table.

- **`st_info`**: `u8`

  Symbol type and binding.
  
  Use the `st_bind` and `st_type` methods to access this value.

- **`st_other`**: `u8`

  Symbol visibility.
  
  Use the `st_visibility` method to access this value.

- **`st_shndx`**: `crate::endian::U16<E>`

  Section index or one of the `SHN_*` values.

- **`st_value`**: `crate::endian::U64<E>`

  Symbol value.

- **`st_size`**: `crate::endian::U64<E>`

  Symbol size.

#### Implementations

- <span id="sym64-st-bind"></span>`fn st_bind(&self) -> u8`

- <span id="sym64-st-type"></span>`fn st_type(&self) -> u8`

- <span id="sym64-set-st-info"></span>`fn set_st_info(&mut self, st_bind: u8, st_type: u8)`

- <span id="sym64-st-visibility"></span>`fn st_visibility(&self) -> u8`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Sym64<E>`

- <span id="sym64-clone"></span>`fn clone(&self) -> Sym64<E>` — [`Sym64`](#sym64)

##### `impl<E: marker::Copy + Endian> Copy for Sym64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Sym64<E>`

- <span id="sym64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for Sym64<E>`

- <span id="sym64-default"></span>`fn default() -> Sym64<E>` — [`Sym64`](#sym64)

##### `impl<E: Endian> Pod for Sym64<E>`

##### `impl<Endian: endian::Endian> Sym for elf::Sym64<Endian>`

- <span id="elfsym64-word"></span>`type Word = u64`

- <span id="elfsym64-endian"></span>`type Endian = Endian`

- <span id="elfsym64-st-name"></span>`fn st_name(&self, endian: <Self as >::Endian) -> u32` — [`Sym`](../read/elf/index.md)

- <span id="elfsym64-st-info"></span>`fn st_info(&self) -> u8`

- <span id="elfsym64-st-bind"></span>`fn st_bind(&self) -> u8`

- <span id="elfsym64-st-type"></span>`fn st_type(&self) -> u8`

- <span id="elfsym64-st-other"></span>`fn st_other(&self) -> u8`

- <span id="elfsym64-st-visibility"></span>`fn st_visibility(&self) -> u8`

- <span id="elfsym64-st-shndx"></span>`fn st_shndx(&self, endian: <Self as >::Endian) -> u16` — [`Sym`](../read/elf/index.md)

- <span id="elfsym64-st-value"></span>`fn st_value(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Sym`](../read/elf/index.md)

- <span id="elfsym64-st-size"></span>`fn st_size(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Sym`](../read/elf/index.md)

### `Syminfo32<E: Endian>`

```rust
struct Syminfo32<E: Endian> {
    pub si_boundto: crate::endian::U16<E>,
    pub si_flags: crate::endian::U16<E>,
}
```

Additional information about a `Sym32`.

#### Fields

- **`si_boundto`**: `crate::endian::U16<E>`

  Direct bindings, symbol bound to.

- **`si_flags`**: `crate::endian::U16<E>`

  Per symbol flags.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Syminfo32<E>`

- <span id="syminfo32-clone"></span>`fn clone(&self) -> Syminfo32<E>` — [`Syminfo32`](#syminfo32)

##### `impl<E: marker::Copy + Endian> Copy for Syminfo32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Syminfo32<E>`

- <span id="syminfo32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Syminfo32<E>`

### `Syminfo64<E: Endian>`

```rust
struct Syminfo64<E: Endian> {
    pub si_boundto: crate::endian::U16<E>,
    pub si_flags: crate::endian::U16<E>,
}
```

Additional information about a `Sym64`.

#### Fields

- **`si_boundto`**: `crate::endian::U16<E>`

  Direct bindings, symbol bound to.

- **`si_flags`**: `crate::endian::U16<E>`

  Per symbol flags.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Syminfo64<E>`

- <span id="syminfo64-clone"></span>`fn clone(&self) -> Syminfo64<E>` — [`Syminfo64`](#syminfo64)

##### `impl<E: marker::Copy + Endian> Copy for Syminfo64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Syminfo64<E>`

- <span id="syminfo64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Syminfo64<E>`

### `Rel32<E: Endian>`

```rust
struct Rel32<E: Endian> {
    pub r_offset: crate::endian::U32<E>,
    pub r_info: crate::endian::U32<E>,
}
```

Relocation table entry without explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U32<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U32<E>`

  Relocation type and symbol index.

#### Implementations

- <span id="rel32-r-sym"></span>`fn r_sym(&self, endian: E) -> u32`

- <span id="rel32-r-type"></span>`fn r_type(&self, endian: E) -> u32`

- <span id="rel32-r-info"></span>`fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>` — [`U32`](../index.md)

- <span id="rel32-set-r-info"></span>`fn set_r_info(&mut self, endian: E, r_sym: u32, r_type: u8)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Rel32<E>`

- <span id="rel32-clone"></span>`fn clone(&self) -> Rel32<E>` — [`Rel32`](#rel32)

##### `impl<E: marker::Copy + Endian> Copy for Rel32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Rel32<E>`

- <span id="rel32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Rel32<E>`

##### `impl<Endian: endian::Endian> Rel for elf::Rel32<Endian>`

- <span id="elfrel32-word"></span>`type Word = u32`

- <span id="elfrel32-sword"></span>`type Sword = i32`

- <span id="elfrel32-endian"></span>`type Endian = Endian`

- <span id="elfrel32-r-offset"></span>`fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rel`](../read/elf/index.md)

- <span id="elfrel32-r-info"></span>`fn r_info(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rel`](../read/elf/index.md)

- <span id="elfrel32-r-sym"></span>`fn r_sym(&self, endian: <Self as >::Endian) -> u32` — [`Rel`](../read/elf/index.md)

- <span id="elfrel32-r-type"></span>`fn r_type(&self, endian: <Self as >::Endian) -> u32` — [`Rel`](../read/elf/index.md)

### `Rela32<E: Endian>`

```rust
struct Rela32<E: Endian> {
    pub r_offset: crate::endian::U32<E>,
    pub r_info: crate::endian::U32<E>,
    pub r_addend: crate::endian::I32<E>,
}
```

Relocation table entry with explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U32<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U32<E>`

  Relocation type and symbol index.

- **`r_addend`**: `crate::endian::I32<E>`

  Explicit addend.

#### Implementations

- <span id="rela32-r-sym"></span>`fn r_sym(&self, endian: E) -> u32`

- <span id="rela32-r-type"></span>`fn r_type(&self, endian: E) -> u32`

- <span id="rela32-r-info"></span>`fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>` — [`U32`](../index.md)

- <span id="rela32-set-r-info"></span>`fn set_r_info(&mut self, endian: E, r_sym: u32, r_type: u8)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Rela32<E>`

- <span id="rela32-clone"></span>`fn clone(&self) -> Rela32<E>` — [`Rela32`](#rela32)

##### `impl<E: marker::Copy + Endian> Copy for Rela32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Rela32<E>`

- <span id="rela32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Rela32<E>`

##### `impl<Endian: endian::Endian> Rela for elf::Rela32<Endian>`

- <span id="elfrela32-word"></span>`type Word = u32`

- <span id="elfrela32-sword"></span>`type Sword = i32`

- <span id="elfrela32-endian"></span>`type Endian = Endian`

- <span id="elfrela32-r-offset"></span>`fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rela`](../read/elf/index.md)

- <span id="elfrela32-r-info"></span>`fn r_info(&self, endian: <Self as >::Endian, _is_mips64el: bool) -> <Self as >::Word` — [`Rela`](../read/elf/index.md)

- <span id="elfrela32-r-addend"></span>`fn r_addend(&self, endian: <Self as >::Endian) -> <Self as >::Sword` — [`Rela`](../read/elf/index.md)

- <span id="elfrela32-r-sym"></span>`fn r_sym(&self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32` — [`Rela`](../read/elf/index.md)

- <span id="elfrela32-r-type"></span>`fn r_type(&self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32` — [`Rela`](../read/elf/index.md)

### `Rel64<E: Endian>`

```rust
struct Rel64<E: Endian> {
    pub r_offset: crate::endian::U64<E>,
    pub r_info: crate::endian::U64<E>,
}
```

Relocation table entry without explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U64<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U64<E>`

  Relocation type and symbol index.

#### Implementations

- <span id="rel64-r-sym"></span>`fn r_sym(&self, endian: E) -> u32`

- <span id="rel64-r-type"></span>`fn r_type(&self, endian: E) -> u32`

- <span id="rel64-r-info"></span>`fn r_info(endian: E, r_sym: u32, r_type: u32) -> U64<E>` — [`U64`](../index.md)

- <span id="rel64-set-r-info"></span>`fn set_r_info(&mut self, endian: E, r_sym: u32, r_type: u32)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Rel64<E>`

- <span id="rel64-clone"></span>`fn clone(&self) -> Rel64<E>` — [`Rel64`](#rel64)

##### `impl<E: marker::Copy + Endian> Copy for Rel64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Rel64<E>`

- <span id="rel64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Rel64<E>`

##### `impl<Endian: endian::Endian> Rel for elf::Rel64<Endian>`

- <span id="elfrel64-word"></span>`type Word = u64`

- <span id="elfrel64-sword"></span>`type Sword = i64`

- <span id="elfrel64-endian"></span>`type Endian = Endian`

- <span id="elfrel64-r-offset"></span>`fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rel`](../read/elf/index.md)

- <span id="elfrel64-r-info"></span>`fn r_info(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rel`](../read/elf/index.md)

- <span id="elfrel64-r-sym"></span>`fn r_sym(&self, endian: <Self as >::Endian) -> u32` — [`Rel`](../read/elf/index.md)

- <span id="elfrel64-r-type"></span>`fn r_type(&self, endian: <Self as >::Endian) -> u32` — [`Rel`](../read/elf/index.md)

### `Rela64<E: Endian>`

```rust
struct Rela64<E: Endian> {
    pub r_offset: crate::endian::U64<E>,
    pub r_info: crate::endian::U64<E>,
    pub r_addend: crate::endian::I64<E>,
}
```

Relocation table entry with explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U64<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U64<E>`

  Relocation type and symbol index.

- **`r_addend`**: `crate::endian::I64<E>`

  Explicit addend.

#### Implementations

- <span id="rela64-get-r-info"></span>`fn get_r_info(&self, endian: E, is_mips64el: bool) -> u64`

- <span id="rela64-r-sym"></span>`fn r_sym(&self, endian: E, is_mips64el: bool) -> u32`

- <span id="rela64-r-type"></span>`fn r_type(&self, endian: E, is_mips64el: bool) -> u32`

- <span id="rela64-r-info"></span>`fn r_info(endian: E, is_mips64el: bool, r_sym: u32, r_type: u32) -> U64<E>` — [`U64`](../index.md)

- <span id="rela64-set-r-info"></span>`fn set_r_info(&mut self, endian: E, is_mips64el: bool, r_sym: u32, r_type: u32)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Rela64<E>`

- <span id="rela64-clone"></span>`fn clone(&self) -> Rela64<E>` — [`Rela64`](#rela64)

##### `impl<E: marker::Copy + Endian> Copy for Rela64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Rela64<E>`

- <span id="rela64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Rela64<E>`

##### `impl<Endian: endian::Endian> Rela for elf::Rela64<Endian>`

- <span id="elfrela64-word"></span>`type Word = u64`

- <span id="elfrela64-sword"></span>`type Sword = i64`

- <span id="elfrela64-endian"></span>`type Endian = Endian`

- <span id="elfrela64-r-offset"></span>`fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Rela`](../read/elf/index.md)

- <span id="elfrela64-r-info"></span>`fn r_info(&self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word` — [`Rela`](../read/elf/index.md)

- <span id="elfrela64-r-addend"></span>`fn r_addend(&self, endian: <Self as >::Endian) -> <Self as >::Sword` — [`Rela`](../read/elf/index.md)

- <span id="elfrela64-r-sym"></span>`fn r_sym(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32` — [`Rela`](../read/elf/index.md)

- <span id="elfrela64-r-type"></span>`fn r_type(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32` — [`Rela`](../read/elf/index.md)

### `Relr32<E: Endian>`

```rust
struct Relr32<E: Endian>(crate::endian::U32<E>);
```

32-bit relative relocation table entry.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Relr32<E>`

- <span id="relr32-clone"></span>`fn clone(&self) -> Relr32<E>` — [`Relr32`](#relr32)

##### `impl<E: marker::Copy + Endian> Copy for Relr32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Relr32<E>`

- <span id="relr32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Relr32<E>`

##### `impl<Endian: endian::Endian> Relr for elf::Relr32<Endian>`

- <span id="elfrelr32-word"></span>`type Word = u32`

- <span id="elfrelr32-endian"></span>`type Endian = Endian`

- <span id="elfrelr32-count"></span>`const COUNT: u8`

- <span id="elfrelr32-get"></span>`fn get(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Relr`](../read/elf/index.md)

- <span id="elfrelr32-next"></span>`fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>` — [`Relr`](../read/elf/index.md)

### `Relr64<E: Endian>`

```rust
struct Relr64<E: Endian>(crate::endian::U64<E>);
```

64-bit relative relocation table entry.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Relr64<E>`

- <span id="relr64-clone"></span>`fn clone(&self) -> Relr64<E>` — [`Relr64`](#relr64)

##### `impl<E: marker::Copy + Endian> Copy for Relr64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Relr64<E>`

- <span id="relr64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Relr64<E>`

##### `impl<Endian: endian::Endian> Relr for elf::Relr64<Endian>`

- <span id="elfrelr64-word"></span>`type Word = u64`

- <span id="elfrelr64-endian"></span>`type Endian = Endian`

- <span id="elfrelr64-count"></span>`const COUNT: u8`

- <span id="elfrelr64-get"></span>`fn get(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Relr`](../read/elf/index.md)

- <span id="elfrelr64-next"></span>`fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>` — [`Relr`](../read/elf/index.md)

### `ProgramHeader32<E: Endian>`

```rust
struct ProgramHeader32<E: Endian> {
    pub p_type: crate::endian::U32<E>,
    pub p_offset: crate::endian::U32<E>,
    pub p_vaddr: crate::endian::U32<E>,
    pub p_paddr: crate::endian::U32<E>,
    pub p_filesz: crate::endian::U32<E>,
    pub p_memsz: crate::endian::U32<E>,
    pub p_flags: crate::endian::U32<E>,
    pub p_align: crate::endian::U32<E>,
}
```

Program segment header.

#### Fields

- **`p_type`**: `crate::endian::U32<E>`

  Segment type. One of the `PT_*` constants.

- **`p_offset`**: `crate::endian::U32<E>`

  Segment file offset.

- **`p_vaddr`**: `crate::endian::U32<E>`

  Segment virtual address.

- **`p_paddr`**: `crate::endian::U32<E>`

  Segment physical address.

- **`p_filesz`**: `crate::endian::U32<E>`

  Segment size in the file.

- **`p_memsz`**: `crate::endian::U32<E>`

  Segment size in memory.

- **`p_flags`**: `crate::endian::U32<E>`

  Segment flags. A combination of the `PF_*` constants.

- **`p_align`**: `crate::endian::U32<E>`

  Segment alignment.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for ProgramHeader32<E>`

- <span id="programheader32-clone"></span>`fn clone(&self) -> ProgramHeader32<E>` — [`ProgramHeader32`](#programheader32)

##### `impl<E: marker::Copy + Endian> Copy for ProgramHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for ProgramHeader32<E>`

- <span id="programheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for ProgramHeader32<E>`

##### `impl<Endian: endian::Endian> ProgramHeader for elf::ProgramHeader32<Endian>`

- <span id="elfprogramheader32-word"></span>`type Word = u32`

- <span id="elfprogramheader32-endian"></span>`type Endian = Endian`

- <span id="elfprogramheader32-elf"></span>`type Elf = FileHeader32<Endian>`

- <span id="elfprogramheader32-p-type"></span>`fn p_type(&self, endian: <Self as >::Endian) -> u32` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-flags"></span>`fn p_flags(&self, endian: <Self as >::Endian) -> u32` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-offset"></span>`fn p_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-vaddr"></span>`fn p_vaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-paddr"></span>`fn p_paddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-filesz"></span>`fn p_filesz(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-memsz"></span>`fn p_memsz(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader32-p-align"></span>`fn p_align(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

### `ProgramHeader64<E: Endian>`

```rust
struct ProgramHeader64<E: Endian> {
    pub p_type: crate::endian::U32<E>,
    pub p_flags: crate::endian::U32<E>,
    pub p_offset: crate::endian::U64<E>,
    pub p_vaddr: crate::endian::U64<E>,
    pub p_paddr: crate::endian::U64<E>,
    pub p_filesz: crate::endian::U64<E>,
    pub p_memsz: crate::endian::U64<E>,
    pub p_align: crate::endian::U64<E>,
}
```

Program segment header.

#### Fields

- **`p_type`**: `crate::endian::U32<E>`

  Segment type. One of the `PT_*` constants.

- **`p_flags`**: `crate::endian::U32<E>`

  Segment flags. A combination of the `PF_*` constants.

- **`p_offset`**: `crate::endian::U64<E>`

  Segment file offset.

- **`p_vaddr`**: `crate::endian::U64<E>`

  Segment virtual address.

- **`p_paddr`**: `crate::endian::U64<E>`

  Segment physical address.

- **`p_filesz`**: `crate::endian::U64<E>`

  Segment size in the file.

- **`p_memsz`**: `crate::endian::U64<E>`

  Segment size in memory.

- **`p_align`**: `crate::endian::U64<E>`

  Segment alignment.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for ProgramHeader64<E>`

- <span id="programheader64-clone"></span>`fn clone(&self) -> ProgramHeader64<E>` — [`ProgramHeader64`](#programheader64)

##### `impl<E: marker::Copy + Endian> Copy for ProgramHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for ProgramHeader64<E>`

- <span id="programheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for ProgramHeader64<E>`

##### `impl<Endian: endian::Endian> ProgramHeader for elf::ProgramHeader64<Endian>`

- <span id="elfprogramheader64-word"></span>`type Word = u64`

- <span id="elfprogramheader64-endian"></span>`type Endian = Endian`

- <span id="elfprogramheader64-elf"></span>`type Elf = FileHeader64<Endian>`

- <span id="elfprogramheader64-p-type"></span>`fn p_type(&self, endian: <Self as >::Endian) -> u32` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-flags"></span>`fn p_flags(&self, endian: <Self as >::Endian) -> u32` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-offset"></span>`fn p_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-vaddr"></span>`fn p_vaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-paddr"></span>`fn p_paddr(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-filesz"></span>`fn p_filesz(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-memsz"></span>`fn p_memsz(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

- <span id="elfprogramheader64-p-align"></span>`fn p_align(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`ProgramHeader`](../read/elf/index.md)

### `Dyn32<E: Endian>`

```rust
struct Dyn32<E: Endian> {
    pub d_tag: crate::endian::U32<E>,
    pub d_val: crate::endian::U32<E>,
}
```

Dynamic section entry.

#### Fields

- **`d_tag`**: `crate::endian::U32<E>`

  Dynamic entry type.

- **`d_val`**: `crate::endian::U32<E>`

  Value (integer or address).

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Dyn32<E>`

- <span id="dyn32-clone"></span>`fn clone(&self) -> Dyn32<E>` — [`Dyn32`](#dyn32)

##### `impl<E: marker::Copy + Endian> Copy for Dyn32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Dyn32<E>`

- <span id="dyn32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> Dyn for elf::Dyn32<Endian>`

- <span id="elfdyn32-word"></span>`type Word = u32`

- <span id="elfdyn32-endian"></span>`type Endian = Endian`

- <span id="elfdyn32-d-tag"></span>`fn d_tag(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Dyn`](../read/elf/index.md)

- <span id="elfdyn32-d-val"></span>`fn d_val(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Dyn`](../read/elf/index.md)

##### `impl<E: Endian> Pod for Dyn32<E>`

### `Dyn64<E: Endian>`

```rust
struct Dyn64<E: Endian> {
    pub d_tag: crate::endian::U64<E>,
    pub d_val: crate::endian::U64<E>,
}
```

Dynamic section entry.

#### Fields

- **`d_tag`**: `crate::endian::U64<E>`

  Dynamic entry type.

- **`d_val`**: `crate::endian::U64<E>`

  Value (integer or address).

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Dyn64<E>`

- <span id="dyn64-clone"></span>`fn clone(&self) -> Dyn64<E>` — [`Dyn64`](#dyn64)

##### `impl<E: marker::Copy + Endian> Copy for Dyn64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Dyn64<E>`

- <span id="dyn64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> Dyn for elf::Dyn64<Endian>`

- <span id="elfdyn64-word"></span>`type Word = u64`

- <span id="elfdyn64-endian"></span>`type Endian = Endian`

- <span id="elfdyn64-d-tag"></span>`fn d_tag(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Dyn`](../read/elf/index.md)

- <span id="elfdyn64-d-val"></span>`fn d_val(&self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Dyn`](../read/elf/index.md)

##### `impl<E: Endian> Pod for Dyn64<E>`

### `Versym<E: Endian>`

```rust
struct Versym<E: Endian>(crate::endian::U16<E>);
```

Version symbol information

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Versym<E>`

- <span id="versym-clone"></span>`fn clone(&self) -> Versym<E>` — [`Versym`](#versym)

##### `impl<E: marker::Copy + Endian> Copy for Versym<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Versym<E>`

- <span id="versym-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Versym<E>`

### `Verdef<E: Endian>`

```rust
struct Verdef<E: Endian> {
    pub vd_version: crate::endian::U16<E>,
    pub vd_flags: crate::endian::U16<E>,
    pub vd_ndx: crate::endian::U16<E>,
    pub vd_cnt: crate::endian::U16<E>,
    pub vd_hash: crate::endian::U32<E>,
    pub vd_aux: crate::endian::U32<E>,
    pub vd_next: crate::endian::U32<E>,
}
```

Version definition sections

#### Fields

- **`vd_version`**: `crate::endian::U16<E>`

  Version revision

- **`vd_flags`**: `crate::endian::U16<E>`

  Version information

- **`vd_ndx`**: `crate::endian::U16<E>`

  Version Index

- **`vd_cnt`**: `crate::endian::U16<E>`

  Number of associated aux entries

- **`vd_hash`**: `crate::endian::U32<E>`

  Version name hash value

- **`vd_aux`**: `crate::endian::U32<E>`

  Offset in bytes to verdaux array

- **`vd_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verdef entry

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Verdef<E>`

- <span id="verdef-clone"></span>`fn clone(&self) -> Verdef<E>` — [`Verdef`](#verdef)

##### `impl<E: marker::Copy + Endian> Copy for Verdef<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Verdef<E>`

- <span id="verdef-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Verdef<E>`

### `Verdaux<E: Endian>`

```rust
struct Verdaux<E: Endian> {
    pub vda_name: crate::endian::U32<E>,
    pub vda_next: crate::endian::U32<E>,
}
```

Auxiliary version information.

#### Fields

- **`vda_name`**: `crate::endian::U32<E>`

  Version or dependency names

- **`vda_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verdaux

#### Implementations

- <span id="elfverdaux-name"></span>`fn name<'data, R: ReadRef<'data>>(&self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Verdaux<E>`

- <span id="verdaux-clone"></span>`fn clone(&self) -> Verdaux<E>` — [`Verdaux`](#verdaux)

##### `impl<E: marker::Copy + Endian> Copy for Verdaux<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Verdaux<E>`

- <span id="verdaux-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Verdaux<E>`

### `Verneed<E: Endian>`

```rust
struct Verneed<E: Endian> {
    pub vn_version: crate::endian::U16<E>,
    pub vn_cnt: crate::endian::U16<E>,
    pub vn_file: crate::endian::U32<E>,
    pub vn_aux: crate::endian::U32<E>,
    pub vn_next: crate::endian::U32<E>,
}
```

Version dependency.

#### Fields

- **`vn_version`**: `crate::endian::U16<E>`

  Version of structure

- **`vn_cnt`**: `crate::endian::U16<E>`

  Number of associated aux entries

- **`vn_file`**: `crate::endian::U32<E>`

  Offset of filename for this dependency

- **`vn_aux`**: `crate::endian::U32<E>`

  Offset in bytes to vernaux array

- **`vn_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verneed entry

#### Implementations

- <span id="elfverneed-file"></span>`fn file<'data, R: ReadRef<'data>>(&self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Verneed<E>`

- <span id="verneed-clone"></span>`fn clone(&self) -> Verneed<E>` — [`Verneed`](#verneed)

##### `impl<E: marker::Copy + Endian> Copy for Verneed<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Verneed<E>`

- <span id="verneed-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Verneed<E>`

### `Vernaux<E: Endian>`

```rust
struct Vernaux<E: Endian> {
    pub vna_hash: crate::endian::U32<E>,
    pub vna_flags: crate::endian::U16<E>,
    pub vna_other: crate::endian::U16<E>,
    pub vna_name: crate::endian::U32<E>,
    pub vna_next: crate::endian::U32<E>,
}
```

Auxiliary needed version information.

#### Fields

- **`vna_hash`**: `crate::endian::U32<E>`

  Hash value of dependency name

- **`vna_flags`**: `crate::endian::U16<E>`

  Dependency specific information

- **`vna_other`**: `crate::endian::U16<E>`

  Version Index

- **`vna_name`**: `crate::endian::U32<E>`

  Dependency name string offset

- **`vna_next`**: `crate::endian::U32<E>`

  Offset in bytes to next vernaux entry

#### Implementations

- <span id="elfvernaux-name"></span>`fn name<'data, R: ReadRef<'data>>(&self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for Vernaux<E>`

- <span id="vernaux-clone"></span>`fn clone(&self) -> Vernaux<E>` — [`Vernaux`](#vernaux)

##### `impl<E: marker::Copy + Endian> Copy for Vernaux<E>`

##### `impl<E: fmt::Debug + Endian> Debug for Vernaux<E>`

- <span id="vernaux-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for Vernaux<E>`

### `NoteHeader32<E: Endian>`

```rust
struct NoteHeader32<E: Endian> {
    pub n_namesz: crate::endian::U32<E>,
    pub n_descsz: crate::endian::U32<E>,
    pub n_type: crate::endian::U32<E>,
}
```

Note section entry header.

A note consists of a header followed by a variable length name and descriptor.

#### Fields

- **`n_namesz`**: `crate::endian::U32<E>`

  Length of the note's name.
  
  Some known names are defined by the `ELF_NOTE_*` constants.

- **`n_descsz`**: `crate::endian::U32<E>`

  Length of the note's descriptor.
  
  The content of the descriptor depends on the note name and type.

- **`n_type`**: `crate::endian::U32<E>`

  Type of the note.
  
  One of the `NT_*` constants. The note name determines which
  `NT_*` constants are valid.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for NoteHeader32<E>`

- <span id="noteheader32-clone"></span>`fn clone(&self) -> NoteHeader32<E>` — [`NoteHeader32`](#noteheader32)

##### `impl<E: marker::Copy + Endian> Copy for NoteHeader32<E>`

##### `impl<E: fmt::Debug + Endian> Debug for NoteHeader32<E>`

- <span id="noteheader32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> NoteHeader for elf::NoteHeader32<Endian>`

- <span id="elfnoteheader32-endian"></span>`type Endian = Endian`

- <span id="elfnoteheader32-n-namesz"></span>`fn n_namesz(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

- <span id="elfnoteheader32-n-descsz"></span>`fn n_descsz(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

- <span id="elfnoteheader32-n-type"></span>`fn n_type(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

##### `impl<E: Endian> Pod for NoteHeader32<E>`

### `NoteHeader64<E: Endian>`

```rust
struct NoteHeader64<E: Endian> {
    pub n_namesz: crate::endian::U32<E>,
    pub n_descsz: crate::endian::U32<E>,
    pub n_type: crate::endian::U32<E>,
}
```

Note section entry header.

#### Fields

- **`n_namesz`**: `crate::endian::U32<E>`

  Length of the note's name.
  
  Some known names are defined by the `ELF_NOTE_*` constants.

- **`n_descsz`**: `crate::endian::U32<E>`

  Length of the note's descriptor.
  
  The content of the descriptor depends on the note name and type.

- **`n_type`**: `crate::endian::U32<E>`

  Type of the note.
  
  One of the `NT_*` constants. The note name determines which
  `NT_*` constants are valid.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for NoteHeader64<E>`

- <span id="noteheader64-clone"></span>`fn clone(&self) -> NoteHeader64<E>` — [`NoteHeader64`](#noteheader64)

##### `impl<E: marker::Copy + Endian> Copy for NoteHeader64<E>`

##### `impl<E: fmt::Debug + Endian> Debug for NoteHeader64<E>`

- <span id="noteheader64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian: endian::Endian> NoteHeader for elf::NoteHeader64<Endian>`

- <span id="elfnoteheader64-endian"></span>`type Endian = Endian`

- <span id="elfnoteheader64-n-namesz"></span>`fn n_namesz(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

- <span id="elfnoteheader64-n-descsz"></span>`fn n_descsz(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

- <span id="elfnoteheader64-n-type"></span>`fn n_type(&self, endian: <Self as >::Endian) -> u32` — [`NoteHeader`](../read/elf/index.md)

##### `impl<E: Endian> Pod for NoteHeader64<E>`

### `HashHeader<E: Endian>`

```rust
struct HashHeader<E: Endian> {
    pub bucket_count: crate::endian::U32<E>,
    pub chain_count: crate::endian::U32<E>,
}
```

Header of `SHT_HASH` section.

#### Fields

- **`bucket_count`**: `crate::endian::U32<E>`

  The number of hash buckets.

- **`chain_count`**: `crate::endian::U32<E>`

  The number of chain values.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for HashHeader<E>`

- <span id="hashheader-clone"></span>`fn clone(&self) -> HashHeader<E>` — [`HashHeader`](#hashheader)

##### `impl<E: marker::Copy + Endian> Copy for HashHeader<E>`

##### `impl<E: fmt::Debug + Endian> Debug for HashHeader<E>`

- <span id="hashheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for HashHeader<E>`

### `GnuHashHeader<E: Endian>`

```rust
struct GnuHashHeader<E: Endian> {
    pub bucket_count: crate::endian::U32<E>,
    pub symbol_base: crate::endian::U32<E>,
    pub bloom_count: crate::endian::U32<E>,
    pub bloom_shift: crate::endian::U32<E>,
}
```

Header of `SHT_GNU_HASH` section.

#### Fields

- **`bucket_count`**: `crate::endian::U32<E>`

  The number of hash buckets.

- **`symbol_base`**: `crate::endian::U32<E>`

  The symbol table index of the first symbol in the hash.

- **`bloom_count`**: `crate::endian::U32<E>`

  The number of words in the bloom filter.
  
  Must be a non-zero power of 2.

- **`bloom_shift`**: `crate::endian::U32<E>`

  The bit shift count for the bloom filter.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for GnuHashHeader<E>`

- <span id="gnuhashheader-clone"></span>`fn clone(&self) -> GnuHashHeader<E>` — [`GnuHashHeader`](#gnuhashheader)

##### `impl<E: marker::Copy + Endian> Copy for GnuHashHeader<E>`

##### `impl<E: fmt::Debug + Endian> Debug for GnuHashHeader<E>`

- <span id="gnuhashheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: Endian> Pod for GnuHashHeader<E>`

## Functions

### `hash`

```rust
fn hash(name: &[u8]) -> u32
```

Calculate the SysV hash for a symbol name.

Used for `SHT_HASH`.

### `gnu_hash`

```rust
fn gnu_hash(name: &[u8]) -> u32
```

Calculate the GNU hash for a symbol name.

Used for `SHT_GNU_HASH`.

### `ef_e2k_mach_to_flag`

```rust
const fn ef_e2k_mach_to_flag(e_flags: u32, x: u32) -> u32
```

Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`.

### `ef_e2k_flag_to_mach`

```rust
const fn ef_e2k_flag_to_mach(e_flags: u32) -> u32
```

Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`.

## Constants

### `ELFMAG`

```rust
const ELFMAG: [u8; 4];
```

File identification bytes stored in `Ident::magic`.

### `ELFCLASSNONE`

```rust
const ELFCLASSNONE: u8 = 0u8;
```

Invalid class.

### `ELFCLASS32`

```rust
const ELFCLASS32: u8 = 1u8;
```

32-bit object.

### `ELFCLASS64`

```rust
const ELFCLASS64: u8 = 2u8;
```

64-bit object.

### `ELFDATANONE`

```rust
const ELFDATANONE: u8 = 0u8;
```

Invalid data encoding.

### `ELFDATA2LSB`

```rust
const ELFDATA2LSB: u8 = 1u8;
```

2's complement, little endian.

### `ELFDATA2MSB`

```rust
const ELFDATA2MSB: u8 = 2u8;
```

2's complement, big endian.

### `ELFOSABI_NONE`

```rust
const ELFOSABI_NONE: u8 = 0u8;
```

UNIX System V ABI.

### `ELFOSABI_SYSV`

```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

UNIX System V ABI.

Alias.

### `ELFOSABI_HPUX`

```rust
const ELFOSABI_HPUX: u8 = 1u8;
```

HP-UX.

### `ELFOSABI_NETBSD`

```rust
const ELFOSABI_NETBSD: u8 = 2u8;
```

NetBSD.

### `ELFOSABI_GNU`

```rust
const ELFOSABI_GNU: u8 = 3u8;
```

Object uses GNU ELF extensions.

### `ELFOSABI_LINUX`

```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

Object uses GNU ELF extensions.

Compatibility alias.

### `ELFOSABI_HURD`

```rust
const ELFOSABI_HURD: u8 = 4u8;
```

GNU/Hurd.

### `ELFOSABI_SOLARIS`

```rust
const ELFOSABI_SOLARIS: u8 = 6u8;
```

Sun Solaris.

### `ELFOSABI_AIX`

```rust
const ELFOSABI_AIX: u8 = 7u8;
```

IBM AIX.

### `ELFOSABI_IRIX`

```rust
const ELFOSABI_IRIX: u8 = 8u8;
```

SGI Irix.

### `ELFOSABI_FREEBSD`

```rust
const ELFOSABI_FREEBSD: u8 = 9u8;
```

FreeBSD.

### `ELFOSABI_TRU64`

```rust
const ELFOSABI_TRU64: u8 = 10u8;
```

Compaq TRU64 UNIX.

### `ELFOSABI_MODESTO`

```rust
const ELFOSABI_MODESTO: u8 = 11u8;
```

Novell Modesto.

### `ELFOSABI_OPENBSD`

```rust
const ELFOSABI_OPENBSD: u8 = 12u8;
```

OpenBSD.

### `ELFOSABI_OPENVMS`

```rust
const ELFOSABI_OPENVMS: u8 = 13u8;
```

OpenVMS.

### `ELFOSABI_NSK`

```rust
const ELFOSABI_NSK: u8 = 14u8;
```

Hewlett-Packard Non-Stop Kernel.

### `ELFOSABI_AROS`

```rust
const ELFOSABI_AROS: u8 = 15u8;
```

AROS

### `ELFOSABI_FENIXOS`

```rust
const ELFOSABI_FENIXOS: u8 = 16u8;
```

FenixOS

### `ELFOSABI_CLOUDABI`

```rust
const ELFOSABI_CLOUDABI: u8 = 17u8;
```

Nuxi CloudABI

### `ELFOSABI_ARM_AEABI`

```rust
const ELFOSABI_ARM_AEABI: u8 = 64u8;
```

ARM EABI.

### `ELFOSABI_ARM`

```rust
const ELFOSABI_ARM: u8 = 97u8;
```

ARM.

### `ELFOSABI_STANDALONE`

```rust
const ELFOSABI_STANDALONE: u8 = 255u8;
```

Standalone (embedded) application.

### `ET_NONE`

```rust
const ET_NONE: u16 = 0u16;
```

No file type.

### `ET_REL`

```rust
const ET_REL: u16 = 1u16;
```

Relocatable file.

### `ET_EXEC`

```rust
const ET_EXEC: u16 = 2u16;
```

Executable file.

### `ET_DYN`

```rust
const ET_DYN: u16 = 3u16;
```

Shared object file.

### `ET_CORE`

```rust
const ET_CORE: u16 = 4u16;
```

Core file.

### `ET_LOOS`

```rust
const ET_LOOS: u16 = 65_024u16;
```

OS-specific range start.

### `ET_HIOS`

```rust
const ET_HIOS: u16 = 65_279u16;
```

OS-specific range end.

### `ET_LOPROC`

```rust
const ET_LOPROC: u16 = 65_280u16;
```

Processor-specific range start.

### `ET_HIPROC`

```rust
const ET_HIPROC: u16 = 65_535u16;
```

Processor-specific range end.

### `EM_NONE`

```rust
const EM_NONE: u16 = 0u16;
```

No machine

### `EM_M32`

```rust
const EM_M32: u16 = 1u16;
```

AT&T WE 32100

### `EM_SPARC`

```rust
const EM_SPARC: u16 = 2u16;
```

SUN SPARC

### `EM_386`

```rust
const EM_386: u16 = 3u16;
```

Intel 80386

### `EM_68K`

```rust
const EM_68K: u16 = 4u16;
```

Motorola m68k family

### `EM_88K`

```rust
const EM_88K: u16 = 5u16;
```

Motorola m88k family

### `EM_IAMCU`

```rust
const EM_IAMCU: u16 = 6u16;
```

Intel MCU

### `EM_860`

```rust
const EM_860: u16 = 7u16;
```

Intel 80860

### `EM_MIPS`

```rust
const EM_MIPS: u16 = 8u16;
```

MIPS R3000 big-endian

### `EM_S370`

```rust
const EM_S370: u16 = 9u16;
```

IBM System/370

### `EM_MIPS_RS3_LE`

```rust
const EM_MIPS_RS3_LE: u16 = 10u16;
```

MIPS R3000 little-endian

### `EM_PARISC`

```rust
const EM_PARISC: u16 = 15u16;
```

HPPA

### `EM_VPP500`

```rust
const EM_VPP500: u16 = 17u16;
```

Fujitsu VPP500

### `EM_SPARC32PLUS`

```rust
const EM_SPARC32PLUS: u16 = 18u16;
```

Sun's "v8plus"

### `EM_960`

```rust
const EM_960: u16 = 19u16;
```

Intel 80960

### `EM_PPC`

```rust
const EM_PPC: u16 = 20u16;
```

PowerPC

### `EM_PPC64`

```rust
const EM_PPC64: u16 = 21u16;
```

PowerPC 64-bit

### `EM_S390`

```rust
const EM_S390: u16 = 22u16;
```

IBM S390

### `EM_SPU`

```rust
const EM_SPU: u16 = 23u16;
```

IBM SPU/SPC

### `EM_V800`

```rust
const EM_V800: u16 = 36u16;
```

NEC V800 series

### `EM_FR20`

```rust
const EM_FR20: u16 = 37u16;
```

Fujitsu FR20

### `EM_RH32`

```rust
const EM_RH32: u16 = 38u16;
```

TRW RH-32

### `EM_RCE`

```rust
const EM_RCE: u16 = 39u16;
```

Motorola RCE

### `EM_ARM`

```rust
const EM_ARM: u16 = 40u16;
```

ARM

### `EM_FAKE_ALPHA`

```rust
const EM_FAKE_ALPHA: u16 = 41u16;
```

Digital Alpha

### `EM_SH`

```rust
const EM_SH: u16 = 42u16;
```

Hitachi SH

### `EM_SPARCV9`

```rust
const EM_SPARCV9: u16 = 43u16;
```

SPARC v9 64-bit

### `EM_TRICORE`

```rust
const EM_TRICORE: u16 = 44u16;
```

Siemens Tricore

### `EM_ARC`

```rust
const EM_ARC: u16 = 45u16;
```

Argonaut RISC Core

### `EM_H8_300`

```rust
const EM_H8_300: u16 = 46u16;
```

Hitachi H8/300

### `EM_H8_300H`

```rust
const EM_H8_300H: u16 = 47u16;
```

Hitachi H8/300H

### `EM_H8S`

```rust
const EM_H8S: u16 = 48u16;
```

Hitachi H8S

### `EM_H8_500`

```rust
const EM_H8_500: u16 = 49u16;
```

Hitachi H8/500

### `EM_IA_64`

```rust
const EM_IA_64: u16 = 50u16;
```

Intel Merced

### `EM_MIPS_X`

```rust
const EM_MIPS_X: u16 = 51u16;
```

Stanford MIPS-X

### `EM_COLDFIRE`

```rust
const EM_COLDFIRE: u16 = 52u16;
```

Motorola Coldfire

### `EM_68HC12`

```rust
const EM_68HC12: u16 = 53u16;
```

Motorola M68HC12

### `EM_MMA`

```rust
const EM_MMA: u16 = 54u16;
```

Fujitsu MMA Multimedia Accelerator

### `EM_PCP`

```rust
const EM_PCP: u16 = 55u16;
```

Siemens PCP

### `EM_NCPU`

```rust
const EM_NCPU: u16 = 56u16;
```

Sony nCPU embeeded RISC

### `EM_NDR1`

```rust
const EM_NDR1: u16 = 57u16;
```

Denso NDR1 microprocessor

### `EM_STARCORE`

```rust
const EM_STARCORE: u16 = 58u16;
```

Motorola Start*Core processor

### `EM_ME16`

```rust
const EM_ME16: u16 = 59u16;
```

Toyota ME16 processor

### `EM_ST100`

```rust
const EM_ST100: u16 = 60u16;
```

STMicroelectronic ST100 processor

### `EM_TINYJ`

```rust
const EM_TINYJ: u16 = 61u16;
```

Advanced Logic Corp. Tinyj emb.fam

### `EM_X86_64`

```rust
const EM_X86_64: u16 = 62u16;
```

AMD x86-64 architecture

### `EM_PDSP`

```rust
const EM_PDSP: u16 = 63u16;
```

Sony DSP Processor

### `EM_PDP10`

```rust
const EM_PDP10: u16 = 64u16;
```

Digital PDP-10

### `EM_PDP11`

```rust
const EM_PDP11: u16 = 65u16;
```

Digital PDP-11

### `EM_FX66`

```rust
const EM_FX66: u16 = 66u16;
```

Siemens FX66 microcontroller

### `EM_ST9PLUS`

```rust
const EM_ST9PLUS: u16 = 67u16;
```

STMicroelectronics ST9+ 8/16 mc

### `EM_ST7`

```rust
const EM_ST7: u16 = 68u16;
```

STmicroelectronics ST7 8 bit mc

### `EM_68HC16`

```rust
const EM_68HC16: u16 = 69u16;
```

Motorola MC68HC16 microcontroller

### `EM_68HC11`

```rust
const EM_68HC11: u16 = 70u16;
```

Motorola MC68HC11 microcontroller

### `EM_68HC08`

```rust
const EM_68HC08: u16 = 71u16;
```

Motorola MC68HC08 microcontroller

### `EM_68HC05`

```rust
const EM_68HC05: u16 = 72u16;
```

Motorola MC68HC05 microcontroller

### `EM_SVX`

```rust
const EM_SVX: u16 = 73u16;
```

Silicon Graphics SVx

### `EM_ST19`

```rust
const EM_ST19: u16 = 74u16;
```

STMicroelectronics ST19 8 bit mc

### `EM_VAX`

```rust
const EM_VAX: u16 = 75u16;
```

Digital VAX

### `EM_CRIS`

```rust
const EM_CRIS: u16 = 76u16;
```

Axis Communications 32-bit emb.proc

### `EM_JAVELIN`

```rust
const EM_JAVELIN: u16 = 77u16;
```

Infineon Technologies 32-bit emb.proc

### `EM_FIREPATH`

```rust
const EM_FIREPATH: u16 = 78u16;
```

Element 14 64-bit DSP Processor

### `EM_ZSP`

```rust
const EM_ZSP: u16 = 79u16;
```

LSI Logic 16-bit DSP Processor

### `EM_MMIX`

```rust
const EM_MMIX: u16 = 80u16;
```

Donald Knuth's educational 64-bit proc

### `EM_HUANY`

```rust
const EM_HUANY: u16 = 81u16;
```

Harvard University machine-independent object files

### `EM_PRISM`

```rust
const EM_PRISM: u16 = 82u16;
```

SiTera Prism

### `EM_AVR`

```rust
const EM_AVR: u16 = 83u16;
```

Atmel AVR 8-bit microcontroller

### `EM_FR30`

```rust
const EM_FR30: u16 = 84u16;
```

Fujitsu FR30

### `EM_D10V`

```rust
const EM_D10V: u16 = 85u16;
```

Mitsubishi D10V

### `EM_D30V`

```rust
const EM_D30V: u16 = 86u16;
```

Mitsubishi D30V

### `EM_V850`

```rust
const EM_V850: u16 = 87u16;
```

NEC v850

### `EM_M32R`

```rust
const EM_M32R: u16 = 88u16;
```

Mitsubishi M32R

### `EM_MN10300`

```rust
const EM_MN10300: u16 = 89u16;
```

Matsushita MN10300

### `EM_MN10200`

```rust
const EM_MN10200: u16 = 90u16;
```

Matsushita MN10200

### `EM_PJ`

```rust
const EM_PJ: u16 = 91u16;
```

picoJava

### `EM_OPENRISC`

```rust
const EM_OPENRISC: u16 = 92u16;
```

OpenRISC 32-bit embedded processor

### `EM_ARC_COMPACT`

```rust
const EM_ARC_COMPACT: u16 = 93u16;
```

ARC International ARCompact

### `EM_XTENSA`

```rust
const EM_XTENSA: u16 = 94u16;
```

Tensilica Xtensa Architecture

### `EM_VIDEOCORE`

```rust
const EM_VIDEOCORE: u16 = 95u16;
```

Alphamosaic VideoCore

### `EM_TMM_GPP`

```rust
const EM_TMM_GPP: u16 = 96u16;
```

Thompson Multimedia General Purpose Proc

### `EM_NS32K`

```rust
const EM_NS32K: u16 = 97u16;
```

National Semi. 32000

### `EM_TPC`

```rust
const EM_TPC: u16 = 98u16;
```

Tenor Network TPC

### `EM_SNP1K`

```rust
const EM_SNP1K: u16 = 99u16;
```

Trebia SNP 1000

### `EM_ST200`

```rust
const EM_ST200: u16 = 100u16;
```

STMicroelectronics ST200

### `EM_IP2K`

```rust
const EM_IP2K: u16 = 101u16;
```

Ubicom IP2xxx

### `EM_MAX`

```rust
const EM_MAX: u16 = 102u16;
```

MAX processor

### `EM_CR`

```rust
const EM_CR: u16 = 103u16;
```

National Semi. CompactRISC

### `EM_F2MC16`

```rust
const EM_F2MC16: u16 = 104u16;
```

Fujitsu F2MC16

### `EM_MSP430`

```rust
const EM_MSP430: u16 = 105u16;
```

Texas Instruments msp430

### `EM_BLACKFIN`

```rust
const EM_BLACKFIN: u16 = 106u16;
```

Analog Devices Blackfin DSP

### `EM_SE_C33`

```rust
const EM_SE_C33: u16 = 107u16;
```

Seiko Epson S1C33 family

### `EM_SEP`

```rust
const EM_SEP: u16 = 108u16;
```

Sharp embedded microprocessor

### `EM_ARCA`

```rust
const EM_ARCA: u16 = 109u16;
```

Arca RISC

### `EM_UNICORE`

```rust
const EM_UNICORE: u16 = 110u16;
```

PKU-Unity & MPRC Peking Uni. mc series

### `EM_EXCESS`

```rust
const EM_EXCESS: u16 = 111u16;
```

eXcess configurable cpu

### `EM_DXP`

```rust
const EM_DXP: u16 = 112u16;
```

Icera Semi. Deep Execution Processor

### `EM_ALTERA_NIOS2`

```rust
const EM_ALTERA_NIOS2: u16 = 113u16;
```

Altera Nios II

### `EM_CRX`

```rust
const EM_CRX: u16 = 114u16;
```

National Semi. CompactRISC CRX

### `EM_XGATE`

```rust
const EM_XGATE: u16 = 115u16;
```

Motorola XGATE

### `EM_C166`

```rust
const EM_C166: u16 = 116u16;
```

Infineon C16x/XC16x

### `EM_M16C`

```rust
const EM_M16C: u16 = 117u16;
```

Renesas M16C

### `EM_DSPIC30F`

```rust
const EM_DSPIC30F: u16 = 118u16;
```

Microchip Technology dsPIC30F

### `EM_CE`

```rust
const EM_CE: u16 = 119u16;
```

Freescale Communication Engine RISC

### `EM_M32C`

```rust
const EM_M32C: u16 = 120u16;
```

Renesas M32C

### `EM_TSK3000`

```rust
const EM_TSK3000: u16 = 131u16;
```

Altium TSK3000

### `EM_RS08`

```rust
const EM_RS08: u16 = 132u16;
```

Freescale RS08

### `EM_SHARC`

```rust
const EM_SHARC: u16 = 133u16;
```

Analog Devices SHARC family

### `EM_ECOG2`

```rust
const EM_ECOG2: u16 = 134u16;
```

Cyan Technology eCOG2

### `EM_SCORE7`

```rust
const EM_SCORE7: u16 = 135u16;
```

Sunplus S+core7 RISC

### `EM_DSP24`

```rust
const EM_DSP24: u16 = 136u16;
```

New Japan Radio (NJR) 24-bit DSP

### `EM_VIDEOCORE3`

```rust
const EM_VIDEOCORE3: u16 = 137u16;
```

Broadcom VideoCore III

### `EM_LATTICEMICO32`

```rust
const EM_LATTICEMICO32: u16 = 138u16;
```

RISC for Lattice FPGA

### `EM_SE_C17`

```rust
const EM_SE_C17: u16 = 139u16;
```

Seiko Epson C17

### `EM_TI_C6000`

```rust
const EM_TI_C6000: u16 = 140u16;
```

Texas Instruments TMS320C6000 DSP

### `EM_TI_C2000`

```rust
const EM_TI_C2000: u16 = 141u16;
```

Texas Instruments TMS320C2000 DSP

### `EM_TI_C5500`

```rust
const EM_TI_C5500: u16 = 142u16;
```

Texas Instruments TMS320C55x DSP

### `EM_TI_ARP32`

```rust
const EM_TI_ARP32: u16 = 143u16;
```

Texas Instruments App. Specific RISC

### `EM_TI_PRU`

```rust
const EM_TI_PRU: u16 = 144u16;
```

Texas Instruments Prog. Realtime Unit

### `EM_MMDSP_PLUS`

```rust
const EM_MMDSP_PLUS: u16 = 160u16;
```

STMicroelectronics 64bit VLIW DSP

### `EM_CYPRESS_M8C`

```rust
const EM_CYPRESS_M8C: u16 = 161u16;
```

Cypress M8C

### `EM_R32C`

```rust
const EM_R32C: u16 = 162u16;
```

Renesas R32C

### `EM_TRIMEDIA`

```rust
const EM_TRIMEDIA: u16 = 163u16;
```

NXP Semi. TriMedia

### `EM_HEXAGON`

```rust
const EM_HEXAGON: u16 = 164u16;
```

QUALCOMM Hexagon

### `EM_8051`

```rust
const EM_8051: u16 = 165u16;
```

Intel 8051 and variants

### `EM_STXP7X`

```rust
const EM_STXP7X: u16 = 166u16;
```

STMicroelectronics STxP7x

### `EM_NDS32`

```rust
const EM_NDS32: u16 = 167u16;
```

Andes Tech. compact code emb. RISC

### `EM_ECOG1X`

```rust
const EM_ECOG1X: u16 = 168u16;
```

Cyan Technology eCOG1X

### `EM_MAXQ30`

```rust
const EM_MAXQ30: u16 = 169u16;
```

Dallas Semi. MAXQ30 mc

### `EM_XIMO16`

```rust
const EM_XIMO16: u16 = 170u16;
```

New Japan Radio (NJR) 16-bit DSP

### `EM_MANIK`

```rust
const EM_MANIK: u16 = 171u16;
```

M2000 Reconfigurable RISC

### `EM_CRAYNV2`

```rust
const EM_CRAYNV2: u16 = 172u16;
```

Cray NV2 vector architecture

### `EM_RX`

```rust
const EM_RX: u16 = 173u16;
```

Renesas RX

### `EM_METAG`

```rust
const EM_METAG: u16 = 174u16;
```

Imagination Tech. META

### `EM_MCST_ELBRUS`

```rust
const EM_MCST_ELBRUS: u16 = 175u16;
```

MCST Elbrus

### `EM_ECOG16`

```rust
const EM_ECOG16: u16 = 176u16;
```

Cyan Technology eCOG16

### `EM_CR16`

```rust
const EM_CR16: u16 = 177u16;
```

National Semi. CompactRISC CR16

### `EM_ETPU`

```rust
const EM_ETPU: u16 = 178u16;
```

Freescale Extended Time Processing Unit

### `EM_SLE9X`

```rust
const EM_SLE9X: u16 = 179u16;
```

Infineon Tech. SLE9X

### `EM_L10M`

```rust
const EM_L10M: u16 = 180u16;
```

Intel L10M

### `EM_K10M`

```rust
const EM_K10M: u16 = 181u16;
```

Intel K10M

### `EM_AARCH64`

```rust
const EM_AARCH64: u16 = 183u16;
```

ARM AARCH64

### `EM_AVR32`

```rust
const EM_AVR32: u16 = 185u16;
```

Amtel 32-bit microprocessor

### `EM_STM8`

```rust
const EM_STM8: u16 = 186u16;
```

STMicroelectronics STM8

### `EM_TILE64`

```rust
const EM_TILE64: u16 = 187u16;
```

Tileta TILE64

### `EM_TILEPRO`

```rust
const EM_TILEPRO: u16 = 188u16;
```

Tilera TILEPro

### `EM_MICROBLAZE`

```rust
const EM_MICROBLAZE: u16 = 189u16;
```

Xilinx MicroBlaze

### `EM_CUDA`

```rust
const EM_CUDA: u16 = 190u16;
```

NVIDIA CUDA

### `EM_TILEGX`

```rust
const EM_TILEGX: u16 = 191u16;
```

Tilera TILE-Gx

### `EM_CLOUDSHIELD`

```rust
const EM_CLOUDSHIELD: u16 = 192u16;
```

CloudShield

### `EM_COREA_1ST`

```rust
const EM_COREA_1ST: u16 = 193u16;
```

KIPO-KAIST Core-A 1st gen.

### `EM_COREA_2ND`

```rust
const EM_COREA_2ND: u16 = 194u16;
```

KIPO-KAIST Core-A 2nd gen.

### `EM_ARC_COMPACT2`

```rust
const EM_ARC_COMPACT2: u16 = 195u16;
```

Synopsys ARCompact V2

### `EM_OPEN8`

```rust
const EM_OPEN8: u16 = 196u16;
```

Open8 RISC

### `EM_RL78`

```rust
const EM_RL78: u16 = 197u16;
```

Renesas RL78

### `EM_VIDEOCORE5`

```rust
const EM_VIDEOCORE5: u16 = 198u16;
```

Broadcom VideoCore V

### `EM_78KOR`

```rust
const EM_78KOR: u16 = 199u16;
```

Renesas 78KOR

### `EM_56800EX`

```rust
const EM_56800EX: u16 = 200u16;
```

Freescale 56800EX DSC

### `EM_BA1`

```rust
const EM_BA1: u16 = 201u16;
```

Beyond BA1

### `EM_BA2`

```rust
const EM_BA2: u16 = 202u16;
```

Beyond BA2

### `EM_XCORE`

```rust
const EM_XCORE: u16 = 203u16;
```

XMOS xCORE

### `EM_MCHP_PIC`

```rust
const EM_MCHP_PIC: u16 = 204u16;
```

Microchip 8-bit PIC(r)

### `EM_KM32`

```rust
const EM_KM32: u16 = 210u16;
```

KM211 KM32

### `EM_KMX32`

```rust
const EM_KMX32: u16 = 211u16;
```

KM211 KMX32

### `EM_EMX16`

```rust
const EM_EMX16: u16 = 212u16;
```

KM211 KMX16

### `EM_EMX8`

```rust
const EM_EMX8: u16 = 213u16;
```

KM211 KMX8

### `EM_KVARC`

```rust
const EM_KVARC: u16 = 214u16;
```

KM211 KVARC

### `EM_CDP`

```rust
const EM_CDP: u16 = 215u16;
```

Paneve CDP

### `EM_COGE`

```rust
const EM_COGE: u16 = 216u16;
```

Cognitive Smart Memory Processor

### `EM_COOL`

```rust
const EM_COOL: u16 = 217u16;
```

Bluechip CoolEngine

### `EM_NORC`

```rust
const EM_NORC: u16 = 218u16;
```

Nanoradio Optimized RISC

### `EM_CSR_KALIMBA`

```rust
const EM_CSR_KALIMBA: u16 = 219u16;
```

CSR Kalimba

### `EM_Z80`

```rust
const EM_Z80: u16 = 220u16;
```

Zilog Z80

### `EM_VISIUM`

```rust
const EM_VISIUM: u16 = 221u16;
```

Controls and Data Services VISIUMcore

### `EM_FT32`

```rust
const EM_FT32: u16 = 222u16;
```

FTDI Chip FT32

### `EM_MOXIE`

```rust
const EM_MOXIE: u16 = 223u16;
```

Moxie processor

### `EM_AMDGPU`

```rust
const EM_AMDGPU: u16 = 224u16;
```

AMD GPU

### `EM_RISCV`

```rust
const EM_RISCV: u16 = 243u16;
```

RISC-V

### `EM_BPF`

```rust
const EM_BPF: u16 = 247u16;
```

Linux BPF -- in-kernel virtual machine

### `EM_CSKY`

```rust
const EM_CSKY: u16 = 252u16;
```

C-SKY

### `EM_LOONGARCH`

```rust
const EM_LOONGARCH: u16 = 258u16;
```

Loongson LoongArch

### `EM_SBF`

```rust
const EM_SBF: u16 = 263u16;
```

Solana Binary Format

### `EM_ALPHA`

```rust
const EM_ALPHA: u16 = 36_902u16;
```

Digital Alpha

### `EV_NONE`

```rust
const EV_NONE: u8 = 0u8;
```

Invalid ELF version.

### `EV_CURRENT`

```rust
const EV_CURRENT: u8 = 1u8;
```

Current ELF version.

### `SHN_UNDEF`

```rust
const SHN_UNDEF: u16 = 0u16;
```

Undefined section.

### `SHN_LORESERVE`

```rust
const SHN_LORESERVE: u16 = 65_280u16;
```

OS-specific range start.
Start of reserved section indices.

### `SHN_LOPROC`

```rust
const SHN_LOPROC: u16 = 65_280u16;
```

Start of processor-specific section indices.

### `SHN_HIPROC`

```rust
const SHN_HIPROC: u16 = 65_311u16;
```

End of processor-specific section indices.

### `SHN_LOOS`

```rust
const SHN_LOOS: u16 = 65_312u16;
```

Start of OS-specific section indices.

### `SHN_HIOS`

```rust
const SHN_HIOS: u16 = 65_343u16;
```

End of OS-specific section indices.

### `SHN_ABS`

```rust
const SHN_ABS: u16 = 65_521u16;
```

Associated symbol is absolute.

### `SHN_COMMON`

```rust
const SHN_COMMON: u16 = 65_522u16;
```

Associated symbol is common.

### `SHN_XINDEX`

```rust
const SHN_XINDEX: u16 = 65_535u16;
```

Section index is in the `SHT_SYMTAB_SHNDX` section.

### `SHN_HIRESERVE`

```rust
const SHN_HIRESERVE: u16 = 65_535u16;
```

End of reserved section indices.

### `SHT_NULL`

```rust
const SHT_NULL: u32 = 0u32;
```

Section header table entry is unused.

### `SHT_PROGBITS`

```rust
const SHT_PROGBITS: u32 = 1u32;
```

Program data.

### `SHT_SYMTAB`

```rust
const SHT_SYMTAB: u32 = 2u32;
```

Symbol table.

### `SHT_STRTAB`

```rust
const SHT_STRTAB: u32 = 3u32;
```

String table.

### `SHT_RELA`

```rust
const SHT_RELA: u32 = 4u32;
```

Relocation entries with explicit addends.

### `SHT_HASH`

```rust
const SHT_HASH: u32 = 5u32;
```

Symbol hash table.

### `SHT_DYNAMIC`

```rust
const SHT_DYNAMIC: u32 = 6u32;
```

Dynamic linking information.

### `SHT_NOTE`

```rust
const SHT_NOTE: u32 = 7u32;
```

Notes.

### `SHT_NOBITS`

```rust
const SHT_NOBITS: u32 = 8u32;
```

Program space with no data (bss).

### `SHT_REL`

```rust
const SHT_REL: u32 = 9u32;
```

Relocation entries without explicit addends.

### `SHT_SHLIB`

```rust
const SHT_SHLIB: u32 = 10u32;
```

Reserved section type.

### `SHT_DYNSYM`

```rust
const SHT_DYNSYM: u32 = 11u32;
```

Dynamic linker symbol table.

### `SHT_INIT_ARRAY`

```rust
const SHT_INIT_ARRAY: u32 = 14u32;
```

Array of constructors.

### `SHT_FINI_ARRAY`

```rust
const SHT_FINI_ARRAY: u32 = 15u32;
```

Array of destructors.

### `SHT_PREINIT_ARRAY`

```rust
const SHT_PREINIT_ARRAY: u32 = 16u32;
```

Array of pre-constructors.

### `SHT_GROUP`

```rust
const SHT_GROUP: u32 = 17u32;
```

Section group.

### `SHT_SYMTAB_SHNDX`

```rust
const SHT_SYMTAB_SHNDX: u32 = 18u32;
```

Extended section indices for a symbol table.

### `SHT_RELR`

```rust
const SHT_RELR: u32 = 19u32;
```

Relocation entries; only offsets.

### `SHT_CREL`

```rust
const SHT_CREL: u32 = 1_073_741_844u32;
```

Experimental CREL relocations. LLVM will change the value and
break compatibility in the future.

### `SHT_LOOS`

```rust
const SHT_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific section types.

### `SHT_LLVM_DEPENDENT_LIBRARIES`

```rust
const SHT_LLVM_DEPENDENT_LIBRARIES: u32 = 1_879_002_116u32;
```

LLVM-style dependent libraries.

### `SHT_GNU_SFRAME`

```rust
const SHT_GNU_SFRAME: u32 = 1_879_048_180u32;
```

GNU SFrame stack trace format.

### `SHT_GNU_ATTRIBUTES`

```rust
const SHT_GNU_ATTRIBUTES: u32 = 1_879_048_181u32;
```

Object attributes.

### `SHT_GNU_HASH`

```rust
const SHT_GNU_HASH: u32 = 1_879_048_182u32;
```

GNU-style hash table.

### `SHT_GNU_LIBLIST`

```rust
const SHT_GNU_LIBLIST: u32 = 1_879_048_183u32;
```

Prelink library list

### `SHT_CHECKSUM`

```rust
const SHT_CHECKSUM: u32 = 1_879_048_184u32;
```

Checksum for DSO content.

### `SHT_LOSUNW`

```rust
const SHT_LOSUNW: u32 = 1_879_048_186u32;
```

Sun-specific low bound.

### `SHT_SUNW_move`

```rust
const SHT_SUNW_move: u32 = 1_879_048_186u32;
```

### `SHT_SUNW_COMDAT`

```rust
const SHT_SUNW_COMDAT: u32 = 1_879_048_187u32;
```

### `SHT_SUNW_syminfo`

```rust
const SHT_SUNW_syminfo: u32 = 1_879_048_188u32;
```

### `SHT_GNU_VERDEF`

```rust
const SHT_GNU_VERDEF: u32 = 1_879_048_189u32;
```

Version definition section.

### `SHT_GNU_VERNEED`

```rust
const SHT_GNU_VERNEED: u32 = 1_879_048_190u32;
```

Version needs section.

### `SHT_GNU_VERSYM`

```rust
const SHT_GNU_VERSYM: u32 = 1_879_048_191u32;
```

Version symbol table.

### `SHT_HISUNW`

```rust
const SHT_HISUNW: u32 = 1_879_048_191u32;
```

Sun-specific high bound.

### `SHT_HIOS`

```rust
const SHT_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific section types.

### `SHT_LOPROC`

```rust
const SHT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific section types.

### `SHT_HIPROC`

```rust
const SHT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific section types.

### `SHT_LOUSER`

```rust
const SHT_LOUSER: u32 = 2_147_483_648u32;
```

Start of application-specific section types.

### `SHT_HIUSER`

```rust
const SHT_HIUSER: u32 = 2_415_919_103u32;
```

End of application-specific section types.

### `SHF_WRITE`

```rust
const SHF_WRITE: u32 = 1u32;
```

Section is writable.

### `SHF_ALLOC`

```rust
const SHF_ALLOC: u32 = 2u32;
```

Section occupies memory during execution.

### `SHF_EXECINSTR`

```rust
const SHF_EXECINSTR: u32 = 4u32;
```

Section is executable.

### `SHF_MERGE`

```rust
const SHF_MERGE: u32 = 16u32;
```

Section may be be merged to eliminate duplication.

### `SHF_STRINGS`

```rust
const SHF_STRINGS: u32 = 32u32;
```

Section contains nul-terminated strings.

### `SHF_INFO_LINK`

```rust
const SHF_INFO_LINK: u32 = 64u32;
```

The `sh_info` field contains a section header table index.

### `SHF_LINK_ORDER`

```rust
const SHF_LINK_ORDER: u32 = 128u32;
```

Section has special ordering requirements when combining sections.

### `SHF_OS_NONCONFORMING`

```rust
const SHF_OS_NONCONFORMING: u32 = 256u32;
```

Section requires special OS-specific handling.

### `SHF_GROUP`

```rust
const SHF_GROUP: u32 = 512u32;
```

Section is a member of a group.

### `SHF_TLS`

```rust
const SHF_TLS: u32 = 1_024u32;
```

Section holds thread-local storage.

### `SHF_COMPRESSED`

```rust
const SHF_COMPRESSED: u32 = 2_048u32;
```

Section is compressed.

Compressed sections begin with one of the `CompressionHeader*` headers.

### `SHF_MASKOS`

```rust
const SHF_MASKOS: u32 = 267_386_880u32;
```

OS-specific section flags.

### `SHF_GNU_RETAIN`

```rust
const SHF_GNU_RETAIN: u32 = 2_097_152u32;
```

Section should not be garbage collected by the linker.

### `SHF_GNU_MBIND`

```rust
const SHF_GNU_MBIND: u32 = 16_777_216u32;
```

Mbind section.

### `SHF_MASKPROC`

```rust
const SHF_MASKPROC: u32 = 4_026_531_840u32;
```

Processor-specific section flags.

### `SHF_EXCLUDE`

```rust
const SHF_EXCLUDE: u32 = 2_147_483_648u32;
```

This section is excluded from the final executable or shared library.

### `ELFCOMPRESS_ZLIB`

```rust
const ELFCOMPRESS_ZLIB: u32 = 1u32;
```

ZLIB/DEFLATE algorithm.

### `ELFCOMPRESS_ZSTD`

```rust
const ELFCOMPRESS_ZSTD: u32 = 2u32;
```

Zstandard algorithm.

### `ELFCOMPRESS_LOOS`

```rust
const ELFCOMPRESS_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific compression types.

### `ELFCOMPRESS_HIOS`

```rust
const ELFCOMPRESS_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific compression types.

### `ELFCOMPRESS_LOPROC`

```rust
const ELFCOMPRESS_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific compression types.

### `ELFCOMPRESS_HIPROC`

```rust
const ELFCOMPRESS_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific compression types.

### `GRP_COMDAT`

```rust
const GRP_COMDAT: u32 = 1u32;
```

Mark group as COMDAT.

### `SYMINFO_BT_SELF`

```rust
const SYMINFO_BT_SELF: u16 = 65_535u16;
```

Symbol bound to self

### `SYMINFO_BT_PARENT`

```rust
const SYMINFO_BT_PARENT: u16 = 65_534u16;
```

Symbol bound to parent

### `SYMINFO_BT_LOWRESERVE`

```rust
const SYMINFO_BT_LOWRESERVE: u16 = 65_280u16;
```

Beginning of reserved entries

### `SYMINFO_FLG_DIRECT`

```rust
const SYMINFO_FLG_DIRECT: u16 = 1u16;
```

Direct bound symbol

### `SYMINFO_FLG_PASSTHRU`

```rust
const SYMINFO_FLG_PASSTHRU: u16 = 2u16;
```

Pass-thru symbol for translator

### `SYMINFO_FLG_COPY`

```rust
const SYMINFO_FLG_COPY: u16 = 4u16;
```

Symbol is a copy-reloc

### `SYMINFO_FLG_LAZYLOAD`

```rust
const SYMINFO_FLG_LAZYLOAD: u16 = 8u16;
```

Symbol bound to object to be lazy loaded

### `SYMINFO_NONE`

```rust
const SYMINFO_NONE: u16 = 0u16;
```

### `SYMINFO_CURRENT`

```rust
const SYMINFO_CURRENT: u16 = 1u16;
```

### `SYMINFO_NUM`

```rust
const SYMINFO_NUM: u16 = 2u16;
```

### `STB_LOCAL`

```rust
const STB_LOCAL: u8 = 0u8;
```

Local symbol.

### `STB_GLOBAL`

```rust
const STB_GLOBAL: u8 = 1u8;
```

Global symbol.

### `STB_WEAK`

```rust
const STB_WEAK: u8 = 2u8;
```

Weak symbol.

### `STB_LOOS`

```rust
const STB_LOOS: u8 = 10u8;
```

Start of OS-specific symbol binding.

### `STB_GNU_UNIQUE`

```rust
const STB_GNU_UNIQUE: u8 = 10u8;
```

Unique symbol.

### `STB_HIOS`

```rust
const STB_HIOS: u8 = 12u8;
```

End of OS-specific symbol binding.

### `STB_LOPROC`

```rust
const STB_LOPROC: u8 = 13u8;
```

Start of processor-specific symbol binding.

### `STB_HIPROC`

```rust
const STB_HIPROC: u8 = 15u8;
```

End of processor-specific symbol binding.

### `STT_NOTYPE`

```rust
const STT_NOTYPE: u8 = 0u8;
```

Symbol type is unspecified.

### `STT_OBJECT`

```rust
const STT_OBJECT: u8 = 1u8;
```

Symbol is a data object.

### `STT_FUNC`

```rust
const STT_FUNC: u8 = 2u8;
```

Symbol is a code object.

### `STT_SECTION`

```rust
const STT_SECTION: u8 = 3u8;
```

Symbol is associated with a section.

### `STT_FILE`

```rust
const STT_FILE: u8 = 4u8;
```

Symbol's name is a file name.

### `STT_COMMON`

```rust
const STT_COMMON: u8 = 5u8;
```

Symbol is a common data object.

### `STT_TLS`

```rust
const STT_TLS: u8 = 6u8;
```

Symbol is a thread-local storage object.

### `STT_LOOS`

```rust
const STT_LOOS: u8 = 10u8;
```

Start of OS-specific symbol types.

### `STT_GNU_IFUNC`

```rust
const STT_GNU_IFUNC: u8 = 10u8;
```

Symbol is an indirect code object.

### `STT_HIOS`

```rust
const STT_HIOS: u8 = 12u8;
```

End of OS-specific symbol types.

### `STT_LOPROC`

```rust
const STT_LOPROC: u8 = 13u8;
```

Start of processor-specific symbol types.

### `STT_HIPROC`

```rust
const STT_HIPROC: u8 = 15u8;
```

End of processor-specific symbol types.

### `STV_DEFAULT`

```rust
const STV_DEFAULT: u8 = 0u8;
```

Default symbol visibility rules.

### `STV_INTERNAL`

```rust
const STV_INTERNAL: u8 = 1u8;
```

Processor specific hidden class.

### `STV_HIDDEN`

```rust
const STV_HIDDEN: u8 = 2u8;
```

Symbol is not visible to other components.

### `STV_PROTECTED`

```rust
const STV_PROTECTED: u8 = 3u8;
```

Symbol is visible to other components, but is not preemptible.

### `PN_XNUM`

```rust
const PN_XNUM: u16 = 65_535u16;
```

Special value for `FileHeader*::e_phnum`.

This indicates that the real number of program headers is too large to fit into e_phnum.
Instead the real value is in the field `sh_info` of section 0.

### `PT_NULL`

```rust
const PT_NULL: u32 = 0u32;
```

Program header table entry is unused.

### `PT_LOAD`

```rust
const PT_LOAD: u32 = 1u32;
```

Loadable program segment.

### `PT_DYNAMIC`

```rust
const PT_DYNAMIC: u32 = 2u32;
```

Dynamic linking information.

### `PT_INTERP`

```rust
const PT_INTERP: u32 = 3u32;
```

Program interpreter.

### `PT_NOTE`

```rust
const PT_NOTE: u32 = 4u32;
```

Auxiliary information.

### `PT_SHLIB`

```rust
const PT_SHLIB: u32 = 5u32;
```

Reserved.

### `PT_PHDR`

```rust
const PT_PHDR: u32 = 6u32;
```

Segment contains the program header table.

### `PT_TLS`

```rust
const PT_TLS: u32 = 7u32;
```

Thread-local storage segment.

### `PT_LOOS`

```rust
const PT_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific segment types.

### `PT_GNU_EH_FRAME`

```rust
const PT_GNU_EH_FRAME: u32 = 1_685_382_480u32;
```

GCC `.eh_frame_hdr` segment.

### `PT_GNU_STACK`

```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

Indicates stack executability.

### `PT_GNU_RELRO`

```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

Read-only after relocation.

### `PT_GNU_PROPERTY`

```rust
const PT_GNU_PROPERTY: u32 = 1_685_382_483u32;
```

Segment containing `.note.gnu.property` section.

### `PT_GNU_SFRAME`

```rust
const PT_GNU_SFRAME: u32 = 1_685_382_484u32;
```

GNU SFrame stack trace format.

### `PT_HIOS`

```rust
const PT_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific segment types.

### `PT_LOPROC`

```rust
const PT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific segment types.

### `PT_HIPROC`

```rust
const PT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific segment types.

### `PF_X`

```rust
const PF_X: u32 = 1u32;
```

Segment is executable.

### `PF_W`

```rust
const PF_W: u32 = 2u32;
```

Segment is writable.

### `PF_R`

```rust
const PF_R: u32 = 4u32;
```

Segment is readable.

### `PF_MASKOS`

```rust
const PF_MASKOS: u32 = 267_386_880u32;
```

OS-specific segment flags.

### `PF_MASKPROC`

```rust
const PF_MASKPROC: u32 = 4_026_531_840u32;
```

Processor-specific segment flags.

### `ELF_NOTE_CORE`

```rust
const ELF_NOTE_CORE: &[u8];
```

Note name for core files.

### `ELF_NOTE_LINUX`

```rust
const ELF_NOTE_LINUX: &[u8];
```

Note name for linux core files.

Notes in linux core files may also use `ELF_NOTE_CORE`.

### `NT_PRSTATUS`

```rust
const NT_PRSTATUS: u32 = 1u32;
```

Contains copy of prstatus struct.

### `NT_PRFPREG`

```rust
const NT_PRFPREG: u32 = 2u32;
```

Contains copy of fpregset struct.

### `NT_FPREGSET`

```rust
const NT_FPREGSET: u32 = 2u32;
```

Contains copy of fpregset struct.

### `NT_PRPSINFO`

```rust
const NT_PRPSINFO: u32 = 3u32;
```

Contains copy of prpsinfo struct.

### `NT_PRXREG`

```rust
const NT_PRXREG: u32 = 4u32;
```

Contains copy of prxregset struct.

### `NT_TASKSTRUCT`

```rust
const NT_TASKSTRUCT: u32 = 4u32;
```

Contains copy of task structure.

### `NT_PLATFORM`

```rust
const NT_PLATFORM: u32 = 5u32;
```

String from sysinfo(SI_PLATFORM).

### `NT_AUXV`

```rust
const NT_AUXV: u32 = 6u32;
```

Contains copy of auxv array.

### `NT_GWINDOWS`

```rust
const NT_GWINDOWS: u32 = 7u32;
```

Contains copy of gwindows struct.

### `NT_ASRS`

```rust
const NT_ASRS: u32 = 8u32;
```

Contains copy of asrset struct.

### `NT_PSTATUS`

```rust
const NT_PSTATUS: u32 = 10u32;
```

Contains copy of pstatus struct.

### `NT_PSINFO`

```rust
const NT_PSINFO: u32 = 13u32;
```

Contains copy of psinfo struct.

### `NT_PRCRED`

```rust
const NT_PRCRED: u32 = 14u32;
```

Contains copy of prcred struct.

### `NT_UTSNAME`

```rust
const NT_UTSNAME: u32 = 15u32;
```

Contains copy of utsname struct.

### `NT_LWPSTATUS`

```rust
const NT_LWPSTATUS: u32 = 16u32;
```

Contains copy of lwpstatus struct.

### `NT_LWPSINFO`

```rust
const NT_LWPSINFO: u32 = 17u32;
```

Contains copy of lwpinfo struct.

### `NT_PRFPXREG`

```rust
const NT_PRFPXREG: u32 = 20u32;
```

Contains copy of fprxregset struct.

### `NT_SIGINFO`

```rust
const NT_SIGINFO: u32 = 1_397_311_305u32;
```

Contains copy of siginfo_t, size might increase.

### `NT_FILE`

```rust
const NT_FILE: u32 = 1_179_208_773u32;
```

Contains information about mapped files.

### `NT_PRXFPREG`

```rust
const NT_PRXFPREG: u32 = 1_189_489_535u32;
```

Contains copy of user_fxsr_struct.

### `NT_PPC_VMX`

```rust
const NT_PPC_VMX: u32 = 256u32;
```

PowerPC Altivec/VMX registers.

### `NT_PPC_SPE`

```rust
const NT_PPC_SPE: u32 = 257u32;
```

PowerPC SPE/EVR registers.

### `NT_PPC_VSX`

```rust
const NT_PPC_VSX: u32 = 258u32;
```

PowerPC VSX registers.

### `NT_PPC_TAR`

```rust
const NT_PPC_TAR: u32 = 259u32;
```

Target Address Register.

### `NT_PPC_PPR`

```rust
const NT_PPC_PPR: u32 = 260u32;
```

Program Priority Register.

### `NT_PPC_DSCR`

```rust
const NT_PPC_DSCR: u32 = 261u32;
```

Data Stream Control Register.

### `NT_PPC_EBB`

```rust
const NT_PPC_EBB: u32 = 262u32;
```

Event Based Branch Registers.

### `NT_PPC_PMU`

```rust
const NT_PPC_PMU: u32 = 263u32;
```

Performance Monitor Registers.

### `NT_PPC_TM_CGPR`

```rust
const NT_PPC_TM_CGPR: u32 = 264u32;
```

TM checkpointed GPR Registers.

### `NT_PPC_TM_CFPR`

```rust
const NT_PPC_TM_CFPR: u32 = 265u32;
```

TM checkpointed FPR Registers.

### `NT_PPC_TM_CVMX`

```rust
const NT_PPC_TM_CVMX: u32 = 266u32;
```

TM checkpointed VMX Registers.

### `NT_PPC_TM_CVSX`

```rust
const NT_PPC_TM_CVSX: u32 = 267u32;
```

TM checkpointed VSX Registers.

### `NT_PPC_TM_SPR`

```rust
const NT_PPC_TM_SPR: u32 = 268u32;
```

TM Special Purpose Registers.

### `NT_PPC_TM_CTAR`

```rust
const NT_PPC_TM_CTAR: u32 = 269u32;
```

TM checkpointed Target Address Register.

### `NT_PPC_TM_CPPR`

```rust
const NT_PPC_TM_CPPR: u32 = 270u32;
```

TM checkpointed Program Priority Register.

### `NT_PPC_TM_CDSCR`

```rust
const NT_PPC_TM_CDSCR: u32 = 271u32;
```

TM checkpointed Data Stream Control Register.

### `NT_PPC_PKEY`

```rust
const NT_PPC_PKEY: u32 = 272u32;
```

Memory Protection Keys registers.

### `NT_386_TLS`

```rust
const NT_386_TLS: u32 = 512u32;
```

i386 TLS slots (struct user_desc).

### `NT_386_IOPERM`

```rust
const NT_386_IOPERM: u32 = 513u32;
```

x86 io permission bitmap (1=deny).

### `NT_X86_XSTATE`

```rust
const NT_X86_XSTATE: u32 = 514u32;
```

x86 extended state using xsave.

### `NT_S390_HIGH_GPRS`

```rust
const NT_S390_HIGH_GPRS: u32 = 768u32;
```

s390 upper register halves.

### `NT_S390_TIMER`

```rust
const NT_S390_TIMER: u32 = 769u32;
```

s390 timer register.

### `NT_S390_TODCMP`

```rust
const NT_S390_TODCMP: u32 = 770u32;
```

s390 TOD clock comparator register.

### `NT_S390_TODPREG`

```rust
const NT_S390_TODPREG: u32 = 771u32;
```

s390 TOD programmable register.

### `NT_S390_CTRS`

```rust
const NT_S390_CTRS: u32 = 772u32;
```

s390 control registers.

### `NT_S390_PREFIX`

```rust
const NT_S390_PREFIX: u32 = 773u32;
```

s390 prefix register.

### `NT_S390_LAST_BREAK`

```rust
const NT_S390_LAST_BREAK: u32 = 774u32;
```

s390 breaking event address.

### `NT_S390_SYSTEM_CALL`

```rust
const NT_S390_SYSTEM_CALL: u32 = 775u32;
```

s390 system call restart data.

### `NT_S390_TDB`

```rust
const NT_S390_TDB: u32 = 776u32;
```

s390 transaction diagnostic block.

### `NT_S390_VXRS_LOW`

```rust
const NT_S390_VXRS_LOW: u32 = 777u32;
```

s390 vector registers 0-15 upper half.

### `NT_S390_VXRS_HIGH`

```rust
const NT_S390_VXRS_HIGH: u32 = 778u32;
```

s390 vector registers 16-31.

### `NT_S390_GS_CB`

```rust
const NT_S390_GS_CB: u32 = 779u32;
```

s390 guarded storage registers.

### `NT_S390_GS_BC`

```rust
const NT_S390_GS_BC: u32 = 780u32;
```

s390 guarded storage broadcast control block.

### `NT_S390_RI_CB`

```rust
const NT_S390_RI_CB: u32 = 781u32;
```

s390 runtime instrumentation.

### `NT_ARM_VFP`

```rust
const NT_ARM_VFP: u32 = 1_024u32;
```

ARM VFP/NEON registers.

### `NT_ARM_TLS`

```rust
const NT_ARM_TLS: u32 = 1_025u32;
```

ARM TLS register.

### `NT_ARM_HW_BREAK`

```rust
const NT_ARM_HW_BREAK: u32 = 1_026u32;
```

ARM hardware breakpoint registers.

### `NT_ARM_HW_WATCH`

```rust
const NT_ARM_HW_WATCH: u32 = 1_027u32;
```

ARM hardware watchpoint registers.

### `NT_ARM_SYSTEM_CALL`

```rust
const NT_ARM_SYSTEM_CALL: u32 = 1_028u32;
```

ARM system call number.

### `NT_ARM_SVE`

```rust
const NT_ARM_SVE: u32 = 1_029u32;
```

ARM Scalable Vector Extension registers.

### `NT_VMCOREDD`

```rust
const NT_VMCOREDD: u32 = 1_792u32;
```

Vmcore Device Dump Note.

### `NT_MIPS_DSP`

```rust
const NT_MIPS_DSP: u32 = 2_048u32;
```

MIPS DSP ASE registers.

### `NT_MIPS_FP_MODE`

```rust
const NT_MIPS_FP_MODE: u32 = 2_049u32;
```

MIPS floating-point mode.

### `NT_VERSION`

```rust
const NT_VERSION: u32 = 1u32;
```

Note type for version string.

This note may appear in object files.

It must be handled as a special case because it has no descriptor, and instead
uses the note name as the version string.

### `DT_NULL`

```rust
const DT_NULL: u32 = 0u32;
```

Marks end of dynamic section

### `DT_NEEDED`

```rust
const DT_NEEDED: u32 = 1u32;
```

Name of needed library

### `DT_PLTRELSZ`

```rust
const DT_PLTRELSZ: u32 = 2u32;
```

Size in bytes of PLT relocs

### `DT_PLTGOT`

```rust
const DT_PLTGOT: u32 = 3u32;
```

Processor defined value

### `DT_HASH`

```rust
const DT_HASH: u32 = 4u32;
```

Address of symbol hash table

### `DT_STRTAB`

```rust
const DT_STRTAB: u32 = 5u32;
```

Address of string table

### `DT_SYMTAB`

```rust
const DT_SYMTAB: u32 = 6u32;
```

Address of symbol table

### `DT_RELA`

```rust
const DT_RELA: u32 = 7u32;
```

Address of Rela relocs

### `DT_RELASZ`

```rust
const DT_RELASZ: u32 = 8u32;
```

Total size of Rela relocs

### `DT_RELAENT`

```rust
const DT_RELAENT: u32 = 9u32;
```

Size of one Rela reloc

### `DT_STRSZ`

```rust
const DT_STRSZ: u32 = 10u32;
```

Size of string table

### `DT_SYMENT`

```rust
const DT_SYMENT: u32 = 11u32;
```

Size of one symbol table entry

### `DT_INIT`

```rust
const DT_INIT: u32 = 12u32;
```

Address of init function

### `DT_FINI`

```rust
const DT_FINI: u32 = 13u32;
```

Address of termination function

### `DT_SONAME`

```rust
const DT_SONAME: u32 = 14u32;
```

Name of shared object

### `DT_RPATH`

```rust
const DT_RPATH: u32 = 15u32;
```

Library search path (deprecated)

### `DT_SYMBOLIC`

```rust
const DT_SYMBOLIC: u32 = 16u32;
```

Start symbol search here

### `DT_REL`

```rust
const DT_REL: u32 = 17u32;
```

Address of Rel relocs

### `DT_RELSZ`

```rust
const DT_RELSZ: u32 = 18u32;
```

Total size of Rel relocs

### `DT_RELENT`

```rust
const DT_RELENT: u32 = 19u32;
```

Size of one Rel reloc

### `DT_PLTREL`

```rust
const DT_PLTREL: u32 = 20u32;
```

Type of reloc in PLT

### `DT_DEBUG`

```rust
const DT_DEBUG: u32 = 21u32;
```

For debugging; unspecified

### `DT_TEXTREL`

```rust
const DT_TEXTREL: u32 = 22u32;
```

Reloc might modify .text

### `DT_JMPREL`

```rust
const DT_JMPREL: u32 = 23u32;
```

Address of PLT relocs

### `DT_BIND_NOW`

```rust
const DT_BIND_NOW: u32 = 24u32;
```

Process relocations of object

### `DT_INIT_ARRAY`

```rust
const DT_INIT_ARRAY: u32 = 25u32;
```

Array with addresses of init fct

### `DT_FINI_ARRAY`

```rust
const DT_FINI_ARRAY: u32 = 26u32;
```

Array with addresses of fini fct

### `DT_INIT_ARRAYSZ`

```rust
const DT_INIT_ARRAYSZ: u32 = 27u32;
```

Size in bytes of DT_INIT_ARRAY

### `DT_FINI_ARRAYSZ`

```rust
const DT_FINI_ARRAYSZ: u32 = 28u32;
```

Size in bytes of DT_FINI_ARRAY

### `DT_RUNPATH`

```rust
const DT_RUNPATH: u32 = 29u32;
```

Library search path

### `DT_FLAGS`

```rust
const DT_FLAGS: u32 = 30u32;
```

Flags for the object being loaded

### `DT_ENCODING`

```rust
const DT_ENCODING: u32 = 32u32;
```

Start of encoded range

### `DT_PREINIT_ARRAY`

```rust
const DT_PREINIT_ARRAY: u32 = 32u32;
```

Array with addresses of preinit fct

### `DT_PREINIT_ARRAYSZ`

```rust
const DT_PREINIT_ARRAYSZ: u32 = 33u32;
```

size in bytes of DT_PREINIT_ARRAY

### `DT_SYMTAB_SHNDX`

```rust
const DT_SYMTAB_SHNDX: u32 = 34u32;
```

Address of SYMTAB_SHNDX section

### `DT_LOOS`

```rust
const DT_LOOS: u32 = 1_610_612_749u32;
```

Start of OS-specific

### `DT_HIOS`

```rust
const DT_HIOS: u32 = 1_879_044_096u32;
```

End of OS-specific

### `DT_LOPROC`

```rust
const DT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific

### `DT_HIPROC`

```rust
const DT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific

### `DT_VALRNGLO`

```rust
const DT_VALRNGLO: u32 = 1_879_047_424u32;
```

### `DT_GNU_PRELINKED`

```rust
const DT_GNU_PRELINKED: u32 = 1_879_047_669u32;
```

Prelinking timestamp

### `DT_GNU_CONFLICTSZ`

```rust
const DT_GNU_CONFLICTSZ: u32 = 1_879_047_670u32;
```

Size of conflict section

### `DT_GNU_LIBLISTSZ`

```rust
const DT_GNU_LIBLISTSZ: u32 = 1_879_047_671u32;
```

Size of library list

### `DT_CHECKSUM`

```rust
const DT_CHECKSUM: u32 = 1_879_047_672u32;
```

### `DT_PLTPADSZ`

```rust
const DT_PLTPADSZ: u32 = 1_879_047_673u32;
```

### `DT_MOVEENT`

```rust
const DT_MOVEENT: u32 = 1_879_047_674u32;
```

### `DT_MOVESZ`

```rust
const DT_MOVESZ: u32 = 1_879_047_675u32;
```

### `DT_FEATURE_1`

```rust
const DT_FEATURE_1: u32 = 1_879_047_676u32;
```

Feature selection (DTF_*).

### `DT_POSFLAG_1`

```rust
const DT_POSFLAG_1: u32 = 1_879_047_677u32;
```

Flags for DT_* entries, affecting the following DT_* entry.

### `DT_SYMINSZ`

```rust
const DT_SYMINSZ: u32 = 1_879_047_678u32;
```

Size of syminfo table (in bytes)

### `DT_SYMINENT`

```rust
const DT_SYMINENT: u32 = 1_879_047_679u32;
```

Entry size of syminfo

### `DT_VALRNGHI`

```rust
const DT_VALRNGHI: u32 = 1_879_047_679u32;
```

### `DT_ADDRRNGLO`

```rust
const DT_ADDRRNGLO: u32 = 1_879_047_680u32;
```

### `DT_GNU_HASH`

```rust
const DT_GNU_HASH: u32 = 1_879_047_925u32;
```

GNU-style hash table.

### `DT_TLSDESC_PLT`

```rust
const DT_TLSDESC_PLT: u32 = 1_879_047_926u32;
```

### `DT_TLSDESC_GOT`

```rust
const DT_TLSDESC_GOT: u32 = 1_879_047_927u32;
```

### `DT_GNU_CONFLICT`

```rust
const DT_GNU_CONFLICT: u32 = 1_879_047_928u32;
```

Start of conflict section

### `DT_GNU_LIBLIST`

```rust
const DT_GNU_LIBLIST: u32 = 1_879_047_929u32;
```

Library list

### `DT_CONFIG`

```rust
const DT_CONFIG: u32 = 1_879_047_930u32;
```

Configuration information.

### `DT_DEPAUDIT`

```rust
const DT_DEPAUDIT: u32 = 1_879_047_931u32;
```

Dependency auditing.

### `DT_AUDIT`

```rust
const DT_AUDIT: u32 = 1_879_047_932u32;
```

Object auditing.

### `DT_PLTPAD`

```rust
const DT_PLTPAD: u32 = 1_879_047_933u32;
```

PLT padding.

### `DT_MOVETAB`

```rust
const DT_MOVETAB: u32 = 1_879_047_934u32;
```

Move table.

### `DT_SYMINFO`

```rust
const DT_SYMINFO: u32 = 1_879_047_935u32;
```

Syminfo table.

### `DT_ADDRRNGHI`

```rust
const DT_ADDRRNGHI: u32 = 1_879_047_935u32;
```

### `DT_VERSYM`

```rust
const DT_VERSYM: u32 = 1_879_048_176u32;
```

### `DT_RELACOUNT`

```rust
const DT_RELACOUNT: u32 = 1_879_048_185u32;
```

### `DT_RELCOUNT`

```rust
const DT_RELCOUNT: u32 = 1_879_048_186u32;
```

### `DT_FLAGS_1`

```rust
const DT_FLAGS_1: u32 = 1_879_048_187u32;
```

State flags, see DF_1_* below.

### `DT_VERDEF`

```rust
const DT_VERDEF: u32 = 1_879_048_188u32;
```

Address of version definition table

### `DT_VERDEFNUM`

```rust
const DT_VERDEFNUM: u32 = 1_879_048_189u32;
```

Number of version definitions

### `DT_VERNEED`

```rust
const DT_VERNEED: u32 = 1_879_048_190u32;
```

Address of table with needed versions

### `DT_VERNEEDNUM`

```rust
const DT_VERNEEDNUM: u32 = 1_879_048_191u32;
```

Number of needed versions

### `DT_AUXILIARY`

```rust
const DT_AUXILIARY: u32 = 2_147_483_645u32;
```

Shared object to load before self

### `DT_FILTER`

```rust
const DT_FILTER: u32 = 2_147_483_647u32;
```

Shared object to get values from

### `DF_ORIGIN`

```rust
const DF_ORIGIN: u32 = 1u32;
```

Object may use DF_ORIGIN

### `DF_SYMBOLIC`

```rust
const DF_SYMBOLIC: u32 = 2u32;
```

Symbol resolutions starts here

### `DF_TEXTREL`

```rust
const DF_TEXTREL: u32 = 4u32;
```

Object contains text relocations

### `DF_BIND_NOW`

```rust
const DF_BIND_NOW: u32 = 8u32;
```

No lazy binding for this object

### `DF_STATIC_TLS`

```rust
const DF_STATIC_TLS: u32 = 16u32;
```

Module uses the static TLS model

### `DF_1_NOW`

```rust
const DF_1_NOW: u32 = 1u32;
```

Set RTLD_NOW for this object.

### `DF_1_GLOBAL`

```rust
const DF_1_GLOBAL: u32 = 2u32;
```

Set RTLD_GLOBAL for this object.

### `DF_1_GROUP`

```rust
const DF_1_GROUP: u32 = 4u32;
```

Set RTLD_GROUP for this object.

### `DF_1_NODELETE`

```rust
const DF_1_NODELETE: u32 = 8u32;
```

Set RTLD_NODELETE for this object.

### `DF_1_LOADFLTR`

```rust
const DF_1_LOADFLTR: u32 = 16u32;
```

Trigger filtee loading at runtime.

### `DF_1_INITFIRST`

```rust
const DF_1_INITFIRST: u32 = 32u32;
```

Set RTLD_INITFIRST for this object.

### `DF_1_NOOPEN`

```rust
const DF_1_NOOPEN: u32 = 64u32;
```

Set RTLD_NOOPEN for this object.

### `DF_1_ORIGIN`

```rust
const DF_1_ORIGIN: u32 = 128u32;
```

$ORIGIN must be handled.

### `DF_1_DIRECT`

```rust
const DF_1_DIRECT: u32 = 256u32;
```

Direct binding enabled.

### `DF_1_TRANS`

```rust
const DF_1_TRANS: u32 = 512u32;
```

### `DF_1_INTERPOSE`

```rust
const DF_1_INTERPOSE: u32 = 1_024u32;
```

Object is used to interpose.

### `DF_1_NODEFLIB`

```rust
const DF_1_NODEFLIB: u32 = 2_048u32;
```

Ignore default lib search path.

### `DF_1_NODUMP`

```rust
const DF_1_NODUMP: u32 = 4_096u32;
```

Object can't be dldump'ed.

### `DF_1_CONFALT`

```rust
const DF_1_CONFALT: u32 = 8_192u32;
```

Configuration alternative created.

### `DF_1_ENDFILTEE`

```rust
const DF_1_ENDFILTEE: u32 = 16_384u32;
```

Filtee terminates filters search.

### `DF_1_DISPRELDNE`

```rust
const DF_1_DISPRELDNE: u32 = 32_768u32;
```

Disp reloc applied at build time.

### `DF_1_DISPRELPND`

```rust
const DF_1_DISPRELPND: u32 = 65_536u32;
```

Disp reloc applied at run-time.

### `DF_1_NODIRECT`

```rust
const DF_1_NODIRECT: u32 = 131_072u32;
```

Object has no-direct binding.

### `DF_1_IGNMULDEF`

```rust
const DF_1_IGNMULDEF: u32 = 262_144u32;
```

### `DF_1_NOKSYMS`

```rust
const DF_1_NOKSYMS: u32 = 524_288u32;
```

### `DF_1_NOHDR`

```rust
const DF_1_NOHDR: u32 = 1_048_576u32;
```

### `DF_1_EDITED`

```rust
const DF_1_EDITED: u32 = 2_097_152u32;
```

Object is modified after built.

### `DF_1_NORELOC`

```rust
const DF_1_NORELOC: u32 = 4_194_304u32;
```

### `DF_1_SYMINTPOSE`

```rust
const DF_1_SYMINTPOSE: u32 = 8_388_608u32;
```

Object has individual interposers.

### `DF_1_GLOBAUDIT`

```rust
const DF_1_GLOBAUDIT: u32 = 16_777_216u32;
```

Global auditing required.

### `DF_1_SINGLETON`

```rust
const DF_1_SINGLETON: u32 = 33_554_432u32;
```

Singleton symbols are used.

### `DF_1_STUB`

```rust
const DF_1_STUB: u32 = 67_108_864u32;
```

### `DF_1_PIE`

```rust
const DF_1_PIE: u32 = 134_217_728u32;
```

### `VERSYM_HIDDEN`

```rust
const VERSYM_HIDDEN: u16 = 32_768u16;
```

Symbol is hidden.

### `VERSYM_VERSION`

```rust
const VERSYM_VERSION: u16 = 32_767u16;
```

Symbol version index.

### `VER_DEF_NONE`

```rust
const VER_DEF_NONE: u16 = 0u16;
```

No version

### `VER_DEF_CURRENT`

```rust
const VER_DEF_CURRENT: u16 = 1u16;
```

Current version

### `VER_FLG_BASE`

```rust
const VER_FLG_BASE: u16 = 1u16;
```

Version definition of file itself

### `VER_FLG_WEAK`

```rust
const VER_FLG_WEAK: u16 = 2u16;
```

Weak version identifier

### `VER_NDX_LOCAL`

```rust
const VER_NDX_LOCAL: u16 = 0u16;
```

Symbol is local.

### `VER_NDX_GLOBAL`

```rust
const VER_NDX_GLOBAL: u16 = 1u16;
```

Symbol is global.

### `VER_NEED_NONE`

```rust
const VER_NEED_NONE: u16 = 0u16;
```

No version

### `VER_NEED_CURRENT`

```rust
const VER_NEED_CURRENT: u16 = 1u16;
```

Current version

### `ELF_NOTE_SOLARIS`

```rust
const ELF_NOTE_SOLARIS: &[u8];
```

Solaris entries in the note section have this name.

### `NT_SOLARIS_PAGESIZE_HINT`

```rust
const NT_SOLARIS_PAGESIZE_HINT: u32 = 1u32;
```

Desired pagesize for the binary.

### `ELF_NOTE_GNU`

```rust
const ELF_NOTE_GNU: &[u8];
```

GNU entries in the note section have this name.

### `ELF_NOTE_GO`

```rust
const ELF_NOTE_GO: &[u8];
```

Go entries in the note section have this name.

### `NT_GNU_ABI_TAG`

```rust
const NT_GNU_ABI_TAG: u32 = 1u32;
```

ABI information.

The descriptor consists of words:
- word 0: OS descriptor
- word 1: major version of the ABI
- word 2: minor version of the ABI
- word 3: subminor version of the ABI

### `ELF_NOTE_OS_LINUX`

```rust
const ELF_NOTE_OS_LINUX: u32 = 0u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_GNU`

```rust
const ELF_NOTE_OS_GNU: u32 = 1u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_SOLARIS2`

```rust
const ELF_NOTE_OS_SOLARIS2: u32 = 2u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_FREEBSD`

```rust
const ELF_NOTE_OS_FREEBSD: u32 = 3u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `NT_GNU_HWCAP`

```rust
const NT_GNU_HWCAP: u32 = 2u32;
```

Synthetic hwcap information.

The descriptor begins with two words:
- word 0: number of entries
- word 1: bitmask of enabled entries

Then follow variable-length entries, one byte followed by a
'\0'-terminated hwcap name string.  The byte gives the bit
number to test if enabled, (1U << bit) & bitmask.  */

### `NT_GNU_BUILD_ID`

```rust
const NT_GNU_BUILD_ID: u32 = 3u32;
```

Build ID bits as generated by `ld --build-id`.

The descriptor consists of any nonzero number of bytes.

### `NT_GO_BUILD_ID`

```rust
const NT_GO_BUILD_ID: u32 = 4u32;
```

Build ID bits as generated by Go's gc compiler.

The descriptor consists of any nonzero number of bytes.

### `NT_GNU_GOLD_VERSION`

```rust
const NT_GNU_GOLD_VERSION: u32 = 4u32;
```

Version note generated by GNU gold containing a version string.

### `NT_GNU_PROPERTY_TYPE_0`

```rust
const NT_GNU_PROPERTY_TYPE_0: u32 = 5u32;
```

Program property.

### `GNU_PROPERTY_STACK_SIZE`

```rust
const GNU_PROPERTY_STACK_SIZE: u32 = 1u32;
```

Stack size.

### `GNU_PROPERTY_NO_COPY_ON_PROTECTED`

```rust
const GNU_PROPERTY_NO_COPY_ON_PROTECTED: u32 = 2u32;
```

No copy relocation on protected data symbol.

### `GNU_PROPERTY_UINT32_AND_LO`

```rust
const GNU_PROPERTY_UINT32_AND_LO: u32 = 2_952_790_016u32;
```

### `GNU_PROPERTY_UINT32_AND_HI`

```rust
const GNU_PROPERTY_UINT32_AND_HI: u32 = 2_952_822_783u32;
```

### `GNU_PROPERTY_UINT32_OR_LO`

```rust
const GNU_PROPERTY_UINT32_OR_LO: u32 = 2_952_822_784u32;
```

### `GNU_PROPERTY_UINT32_OR_HI`

```rust
const GNU_PROPERTY_UINT32_OR_HI: u32 = 2_952_855_551u32;
```

### `GNU_PROPERTY_1_NEEDED`

```rust
const GNU_PROPERTY_1_NEEDED: u32 = 2_952_822_784u32;
```

The needed properties by the object file.  */

### `GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS`

```rust
const GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS: u32 = 1u32;
```

Set if the object file requires canonical function pointers and
cannot be used with copy relocation.

### `GNU_PROPERTY_LOPROC`

```rust
const GNU_PROPERTY_LOPROC: u32 = 3_221_225_472u32;
```

Processor-specific semantics, lo

### `GNU_PROPERTY_HIPROC`

```rust
const GNU_PROPERTY_HIPROC: u32 = 3_758_096_383u32;
```

Processor-specific semantics, hi

### `GNU_PROPERTY_LOUSER`

```rust
const GNU_PROPERTY_LOUSER: u32 = 3_758_096_384u32;
```

Application-specific semantics, lo

### `GNU_PROPERTY_HIUSER`

```rust
const GNU_PROPERTY_HIUSER: u32 = 4_294_967_295u32;
```

Application-specific semantics, hi

### `GNU_PROPERTY_AARCH64_FEATURE_1_AND`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 3_221_225_472u32;
```

AArch64 specific GNU properties.

### `GNU_PROPERTY_AARCH64_FEATURE_PAUTH`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_PAUTH: u32 = 3_221_225_473u32;
```

### `GNU_PROPERTY_AARCH64_FEATURE_1_BTI`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1u32;
```

### `GNU_PROPERTY_AARCH64_FEATURE_1_PAC`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_PAC: u32 = 2u32;
```

### `GNU_PROPERTY_X86_UINT32_AND_LO`

```rust
const GNU_PROPERTY_X86_UINT32_AND_LO: u32 = 3_221_225_474u32;
```

### `GNU_PROPERTY_X86_UINT32_AND_HI`

```rust
const GNU_PROPERTY_X86_UINT32_AND_HI: u32 = 3_221_258_239u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_LO`

```rust
const GNU_PROPERTY_X86_UINT32_OR_LO: u32 = 3_221_258_240u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_HI`

```rust
const GNU_PROPERTY_X86_UINT32_OR_HI: u32 = 3_221_291_007u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_AND_LO`

```rust
const GNU_PROPERTY_X86_UINT32_OR_AND_LO: u32 = 3_221_291_008u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_AND_HI`

```rust
const GNU_PROPERTY_X86_UINT32_OR_AND_HI: u32 = 3_221_323_775u32;
```

### `GNU_PROPERTY_X86_ISA_1_USED`

```rust
const GNU_PROPERTY_X86_ISA_1_USED: u32 = 3_221_291_010u32;
```

The x86 instruction sets indicated by the corresponding bits are
used in program.  Their support in the hardware is optional.

### `GNU_PROPERTY_X86_ISA_1_NEEDED`

```rust
const GNU_PROPERTY_X86_ISA_1_NEEDED: u32 = 3_221_258_242u32;
```

The x86 instruction sets indicated by the corresponding bits are
used in program and they must be supported by the hardware.

### `GNU_PROPERTY_X86_FEATURE_1_AND`

```rust
const GNU_PROPERTY_X86_FEATURE_1_AND: u32 = 3_221_225_474u32;
```

X86 processor-specific features used in program.

### `GNU_PROPERTY_X86_ISA_1_BASELINE`

```rust
const GNU_PROPERTY_X86_ISA_1_BASELINE: u32 = 1u32;
```

GNU_PROPERTY_X86_ISA_1_BASELINE: CMOV, CX8 (cmpxchg8b), FPU (fld),
MMX, OSFXSR (fxsave), SCE (syscall), SSE and SSE2.

### `GNU_PROPERTY_X86_ISA_1_V2`

```rust
const GNU_PROPERTY_X86_ISA_1_V2: u32 = 2u32;
```

GNU_PROPERTY_X86_ISA_1_V2: GNU_PROPERTY_X86_ISA_1_BASELINE,
CMPXCHG16B (cmpxchg16b), LAHF-SAHF (lahf), POPCNT (popcnt), SSE3,
SSSE3, SSE4.1 and SSE4.2.

### `GNU_PROPERTY_X86_ISA_1_V3`

```rust
const GNU_PROPERTY_X86_ISA_1_V3: u32 = 4u32;
```

GNU_PROPERTY_X86_ISA_1_V3: GNU_PROPERTY_X86_ISA_1_V2, AVX, AVX2, BMI1,
BMI2, F16C, FMA, LZCNT, MOVBE, XSAVE.

### `GNU_PROPERTY_X86_ISA_1_V4`

```rust
const GNU_PROPERTY_X86_ISA_1_V4: u32 = 8u32;
```

GNU_PROPERTY_X86_ISA_1_V4: GNU_PROPERTY_X86_ISA_1_V3, AVX512F,
AVX512BW, AVX512CD, AVX512DQ and AVX512VL.

### `GNU_PROPERTY_X86_FEATURE_1_IBT`

```rust
const GNU_PROPERTY_X86_FEATURE_1_IBT: u32 = 1u32;
```

This indicates that all executable sections are compatible with IBT.

### `GNU_PROPERTY_X86_FEATURE_1_SHSTK`

```rust
const GNU_PROPERTY_X86_FEATURE_1_SHSTK: u32 = 2u32;
```

This indicates that all executable sections are compatible with SHSTK.

### `R_68K_NONE`

```rust
const R_68K_NONE: u32 = 0u32;
```

No reloc

### `R_68K_32`

```rust
const R_68K_32: u32 = 1u32;
```

Direct 32 bit

### `R_68K_16`

```rust
const R_68K_16: u32 = 2u32;
```

Direct 16 bit

### `R_68K_8`

```rust
const R_68K_8: u32 = 3u32;
```

Direct 8 bit

### `R_68K_PC32`

```rust
const R_68K_PC32: u32 = 4u32;
```

PC relative 32 bit

### `R_68K_PC16`

```rust
const R_68K_PC16: u32 = 5u32;
```

PC relative 16 bit

### `R_68K_PC8`

```rust
const R_68K_PC8: u32 = 6u32;
```

PC relative 8 bit

### `R_68K_GOT32`

```rust
const R_68K_GOT32: u32 = 7u32;
```

32 bit PC relative GOT entry

### `R_68K_GOT16`

```rust
const R_68K_GOT16: u32 = 8u32;
```

16 bit PC relative GOT entry

### `R_68K_GOT8`

```rust
const R_68K_GOT8: u32 = 9u32;
```

8 bit PC relative GOT entry

### `R_68K_GOT32O`

```rust
const R_68K_GOT32O: u32 = 10u32;
```

32 bit GOT offset

### `R_68K_GOT16O`

```rust
const R_68K_GOT16O: u32 = 11u32;
```

16 bit GOT offset

### `R_68K_GOT8O`

```rust
const R_68K_GOT8O: u32 = 12u32;
```

8 bit GOT offset

### `R_68K_PLT32`

```rust
const R_68K_PLT32: u32 = 13u32;
```

32 bit PC relative PLT address

### `R_68K_PLT16`

```rust
const R_68K_PLT16: u32 = 14u32;
```

16 bit PC relative PLT address

### `R_68K_PLT8`

```rust
const R_68K_PLT8: u32 = 15u32;
```

8 bit PC relative PLT address

### `R_68K_PLT32O`

```rust
const R_68K_PLT32O: u32 = 16u32;
```

32 bit PLT offset

### `R_68K_PLT16O`

```rust
const R_68K_PLT16O: u32 = 17u32;
```

16 bit PLT offset

### `R_68K_PLT8O`

```rust
const R_68K_PLT8O: u32 = 18u32;
```

8 bit PLT offset

### `R_68K_COPY`

```rust
const R_68K_COPY: u32 = 19u32;
```

Copy symbol at runtime

### `R_68K_GLOB_DAT`

```rust
const R_68K_GLOB_DAT: u32 = 20u32;
```

Create GOT entry

### `R_68K_JMP_SLOT`

```rust
const R_68K_JMP_SLOT: u32 = 21u32;
```

Create PLT entry

### `R_68K_RELATIVE`

```rust
const R_68K_RELATIVE: u32 = 22u32;
```

Adjust by program base

### `R_68K_TLS_GD32`

```rust
const R_68K_TLS_GD32: u32 = 25u32;
```

32 bit GOT offset for GD

### `R_68K_TLS_GD16`

```rust
const R_68K_TLS_GD16: u32 = 26u32;
```

16 bit GOT offset for GD

### `R_68K_TLS_GD8`

```rust
const R_68K_TLS_GD8: u32 = 27u32;
```

8 bit GOT offset for GD

### `R_68K_TLS_LDM32`

```rust
const R_68K_TLS_LDM32: u32 = 28u32;
```

32 bit GOT offset for LDM

### `R_68K_TLS_LDM16`

```rust
const R_68K_TLS_LDM16: u32 = 29u32;
```

16 bit GOT offset for LDM

### `R_68K_TLS_LDM8`

```rust
const R_68K_TLS_LDM8: u32 = 30u32;
```

8 bit GOT offset for LDM

### `R_68K_TLS_LDO32`

```rust
const R_68K_TLS_LDO32: u32 = 31u32;
```

32 bit module-relative offset

### `R_68K_TLS_LDO16`

```rust
const R_68K_TLS_LDO16: u32 = 32u32;
```

16 bit module-relative offset

### `R_68K_TLS_LDO8`

```rust
const R_68K_TLS_LDO8: u32 = 33u32;
```

8 bit module-relative offset

### `R_68K_TLS_IE32`

```rust
const R_68K_TLS_IE32: u32 = 34u32;
```

32 bit GOT offset for IE

### `R_68K_TLS_IE16`

```rust
const R_68K_TLS_IE16: u32 = 35u32;
```

16 bit GOT offset for IE

### `R_68K_TLS_IE8`

```rust
const R_68K_TLS_IE8: u32 = 36u32;
```

8 bit GOT offset for IE

### `R_68K_TLS_LE32`

```rust
const R_68K_TLS_LE32: u32 = 37u32;
```

32 bit offset relative to static TLS block

### `R_68K_TLS_LE16`

```rust
const R_68K_TLS_LE16: u32 = 38u32;
```

16 bit offset relative to static TLS block

### `R_68K_TLS_LE8`

```rust
const R_68K_TLS_LE8: u32 = 39u32;
```

8 bit offset relative to static TLS block

### `R_68K_TLS_DTPMOD32`

```rust
const R_68K_TLS_DTPMOD32: u32 = 40u32;
```

32 bit module number

### `R_68K_TLS_DTPREL32`

```rust
const R_68K_TLS_DTPREL32: u32 = 41u32;
```

32 bit module-relative offset

### `R_68K_TLS_TPREL32`

```rust
const R_68K_TLS_TPREL32: u32 = 42u32;
```

32 bit TP-relative offset

### `R_386_NONE`

```rust
const R_386_NONE: u32 = 0u32;
```

No reloc

### `R_386_32`

```rust
const R_386_32: u32 = 1u32;
```

Direct 32 bit

### `R_386_PC32`

```rust
const R_386_PC32: u32 = 2u32;
```

PC relative 32 bit

### `R_386_GOT32`

```rust
const R_386_GOT32: u32 = 3u32;
```

32 bit GOT entry

### `R_386_PLT32`

```rust
const R_386_PLT32: u32 = 4u32;
```

32 bit PLT address

### `R_386_COPY`

```rust
const R_386_COPY: u32 = 5u32;
```

Copy symbol at runtime

### `R_386_GLOB_DAT`

```rust
const R_386_GLOB_DAT: u32 = 6u32;
```

Create GOT entry

### `R_386_JMP_SLOT`

```rust
const R_386_JMP_SLOT: u32 = 7u32;
```

Create PLT entry

### `R_386_RELATIVE`

```rust
const R_386_RELATIVE: u32 = 8u32;
```

Adjust by program base

### `R_386_GOTOFF`

```rust
const R_386_GOTOFF: u32 = 9u32;
```

32 bit offset to GOT

### `R_386_GOTPC`

```rust
const R_386_GOTPC: u32 = 10u32;
```

32 bit PC relative offset to GOT

### `R_386_32PLT`

```rust
const R_386_32PLT: u32 = 11u32;
```

Direct 32 bit PLT address

### `R_386_TLS_TPOFF`

```rust
const R_386_TLS_TPOFF: u32 = 14u32;
```

Offset in static TLS block

### `R_386_TLS_IE`

```rust
const R_386_TLS_IE: u32 = 15u32;
```

Address of GOT entry for static TLS block offset

### `R_386_TLS_GOTIE`

```rust
const R_386_TLS_GOTIE: u32 = 16u32;
```

GOT entry for static TLS block offset

### `R_386_TLS_LE`

```rust
const R_386_TLS_LE: u32 = 17u32;
```

Offset relative to static TLS block

### `R_386_TLS_GD`

```rust
const R_386_TLS_GD: u32 = 18u32;
```

Direct 32 bit for GNU version of general dynamic thread local data

### `R_386_TLS_LDM`

```rust
const R_386_TLS_LDM: u32 = 19u32;
```

Direct 32 bit for GNU version of local dynamic thread local data in LE code

### `R_386_16`

```rust
const R_386_16: u32 = 20u32;
```

Direct 16 bit

### `R_386_PC16`

```rust
const R_386_PC16: u32 = 21u32;
```

PC relative 16 bit

### `R_386_8`

```rust
const R_386_8: u32 = 22u32;
```

Direct 8 bit

### `R_386_PC8`

```rust
const R_386_PC8: u32 = 23u32;
```

PC relative 8 bit

### `R_386_TLS_GD_32`

```rust
const R_386_TLS_GD_32: u32 = 24u32;
```

Direct 32 bit for general dynamic thread local data

### `R_386_TLS_GD_PUSH`

```rust
const R_386_TLS_GD_PUSH: u32 = 25u32;
```

Tag for pushl in GD TLS code

### `R_386_TLS_GD_CALL`

```rust
const R_386_TLS_GD_CALL: u32 = 26u32;
```

Relocation for call to __tls_get_addr()

### `R_386_TLS_GD_POP`

```rust
const R_386_TLS_GD_POP: u32 = 27u32;
```

Tag for popl in GD TLS code

### `R_386_TLS_LDM_32`

```rust
const R_386_TLS_LDM_32: u32 = 28u32;
```

Direct 32 bit for local dynamic thread local data in LE code

### `R_386_TLS_LDM_PUSH`

```rust
const R_386_TLS_LDM_PUSH: u32 = 29u32;
```

Tag for pushl in LDM TLS code

### `R_386_TLS_LDM_CALL`

```rust
const R_386_TLS_LDM_CALL: u32 = 30u32;
```

Relocation for call to __tls_get_addr() in LDM code

### `R_386_TLS_LDM_POP`

```rust
const R_386_TLS_LDM_POP: u32 = 31u32;
```

Tag for popl in LDM TLS code

### `R_386_TLS_LDO_32`

```rust
const R_386_TLS_LDO_32: u32 = 32u32;
```

Offset relative to TLS block

### `R_386_TLS_IE_32`

```rust
const R_386_TLS_IE_32: u32 = 33u32;
```

GOT entry for negated static TLS block offset

### `R_386_TLS_LE_32`

```rust
const R_386_TLS_LE_32: u32 = 34u32;
```

Negated offset relative to static TLS block

### `R_386_TLS_DTPMOD32`

```rust
const R_386_TLS_DTPMOD32: u32 = 35u32;
```

ID of module containing symbol

### `R_386_TLS_DTPOFF32`

```rust
const R_386_TLS_DTPOFF32: u32 = 36u32;
```

Offset in TLS block

### `R_386_TLS_TPOFF32`

```rust
const R_386_TLS_TPOFF32: u32 = 37u32;
```

Negated offset in static TLS block

### `R_386_SIZE32`

```rust
const R_386_SIZE32: u32 = 38u32;
```

32-bit symbol size

### `R_386_TLS_GOTDESC`

```rust
const R_386_TLS_GOTDESC: u32 = 39u32;
```

GOT offset for TLS descriptor.

### `R_386_TLS_DESC_CALL`

```rust
const R_386_TLS_DESC_CALL: u32 = 40u32;
```

Marker of call through TLS descriptor for relaxation.

### `R_386_TLS_DESC`

```rust
const R_386_TLS_DESC: u32 = 41u32;
```

TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol.

### `R_386_IRELATIVE`

```rust
const R_386_IRELATIVE: u32 = 42u32;
```

Adjust indirectly by program base

### `R_386_GOT32X`

```rust
const R_386_GOT32X: u32 = 43u32;
```

Load from 32 bit GOT entry, relaxable.

### `R_SHARC_ADDR24_V3`

```rust
const R_SHARC_ADDR24_V3: u32 = 11u32;
```

24-bit absolute address in bits 23:0 of a 48-bit instr

Targets:

* Type 25a (PC_DIRECT)

### `R_SHARC_ADDR32_V3`

```rust
const R_SHARC_ADDR32_V3: u32 = 12u32;
```

32-bit absolute address in bits 31:0 of a 48-bit instr

Targets:

* Type 14a
* Type 14d
* Type 15a
* Type 16a
* Type 17a
* Type 18a
* Type 19a

### `R_SHARC_ADDR_VAR_V3`

```rust
const R_SHARC_ADDR_VAR_V3: u32 = 13u32;
```

32-bit absolute address in bits 31:0 of a 32-bit data location

Represented with `RelocationEncoding::Generic`

### `R_SHARC_PCRSHORT_V3`

```rust
const R_SHARC_PCRSHORT_V3: u32 = 14u32;
```

6-bit PC-relative address in bits 32:27 of a 48-bit instr

Targets:

* Type 9a
* Type 10a

### `R_SHARC_PCRLONG_V3`

```rust
const R_SHARC_PCRLONG_V3: u32 = 15u32;
```

24-bit PC-relative address in bits 23:0 of a 48-bit instr

Targets:

* Type 8a
* Type 12a (truncated to 23 bits after relocation)
* Type 13a (truncated to 23 bits after relocation)
* Type 25a (PC Relative)

### `R_SHARC_DATA6_V3`

```rust
const R_SHARC_DATA6_V3: u32 = 16u32;
```

6-bit absolute address in bits 32:27 of a 48-bit instr

Targets:

* Type 4a
* Type 4b
* Type 4d

### `R_SHARC_DATA16_V3`

```rust
const R_SHARC_DATA16_V3: u32 = 17u32;
```

16-bit absolute address in bits 39:24 of a 48-bit instr

Targets:

* Type 12a

### `R_SHARC_DATA6_VISA_V3`

```rust
const R_SHARC_DATA6_VISA_V3: u32 = 18u32;
```

6-bit absolute address into bits 16:11 of a 32-bit instr

Targets:

* Type 4b

### `R_SHARC_DATA7_VISA_V3`

```rust
const R_SHARC_DATA7_VISA_V3: u32 = 19u32;
```

7-bit absolute address into bits 6:0 of a 32-bit instr

### `R_SHARC_DATA16_VISA_V3`

```rust
const R_SHARC_DATA16_VISA_V3: u32 = 20u32;
```

16-bit absolute address into bits 15:0 of a 32-bit instr

### `R_SHARC_PCR6_VISA_V3`

```rust
const R_SHARC_PCR6_VISA_V3: u32 = 23u32;
```

6-bit PC-relative address into bits 16:11 of a Type B

Targets:

* Type 9b

### `R_SHARC_ADDR_VAR16_V3`

```rust
const R_SHARC_ADDR_VAR16_V3: u32 = 25u32;
```

16-bit absolute address into bits 15:0 of a 16-bit location.

Represented with `RelocationEncoding::Generic`

### `R_SHARC_CALC_PUSH_ADDR`

```rust
const R_SHARC_CALC_PUSH_ADDR: u32 = 224u32;
```

### `R_SHARC_CALC_PUSH_ADDEND`

```rust
const R_SHARC_CALC_PUSH_ADDEND: u32 = 225u32;
```

### `R_SHARC_CALC_ADD`

```rust
const R_SHARC_CALC_ADD: u32 = 226u32;
```

### `R_SHARC_CALC_SUB`

```rust
const R_SHARC_CALC_SUB: u32 = 227u32;
```

### `R_SHARC_CALC_MUL`

```rust
const R_SHARC_CALC_MUL: u32 = 228u32;
```

### `R_SHARC_CALC_DIV`

```rust
const R_SHARC_CALC_DIV: u32 = 229u32;
```

### `R_SHARC_CALC_MOD`

```rust
const R_SHARC_CALC_MOD: u32 = 230u32;
```

### `R_SHARC_CALC_LSHIFT`

```rust
const R_SHARC_CALC_LSHIFT: u32 = 231u32;
```

### `R_SHARC_CALC_RSHIFT`

```rust
const R_SHARC_CALC_RSHIFT: u32 = 232u32;
```

### `R_SHARC_CALC_AND`

```rust
const R_SHARC_CALC_AND: u32 = 233u32;
```

### `R_SHARC_CALC_OR`

```rust
const R_SHARC_CALC_OR: u32 = 234u32;
```

### `R_SHARC_CALC_XOR`

```rust
const R_SHARC_CALC_XOR: u32 = 235u32;
```

### `R_SHARC_CALC_PUSH_LEN`

```rust
const R_SHARC_CALC_PUSH_LEN: u32 = 236u32;
```

### `R_SHARC_CALC_NOT`

```rust
const R_SHARC_CALC_NOT: u32 = 246u32;
```

### `SHT_SHARC_ADI_ATTRIBUTES`

```rust
const SHT_SHARC_ADI_ATTRIBUTES: u32 = 1_879_048_194u32;
```

.adi.attributes

### `STT_SPARC_REGISTER`

```rust
const STT_SPARC_REGISTER: u8 = 13u8;
```

Global register reserved to app.

### `EF_SPARCV9_MM`

```rust
const EF_SPARCV9_MM: u32 = 3u32;
```

### `EF_SPARCV9_TSO`

```rust
const EF_SPARCV9_TSO: u32 = 0u32;
```

### `EF_SPARCV9_PSO`

```rust
const EF_SPARCV9_PSO: u32 = 1u32;
```

### `EF_SPARCV9_RMO`

```rust
const EF_SPARCV9_RMO: u32 = 2u32;
```

### `EF_SPARC_LEDATA`

```rust
const EF_SPARC_LEDATA: u32 = 8_388_608u32;
```

little endian data

### `EF_SPARC_EXT_MASK`

```rust
const EF_SPARC_EXT_MASK: u32 = 16_776_960u32;
```

### `EF_SPARC_32PLUS`

```rust
const EF_SPARC_32PLUS: u32 = 256u32;
```

generic V8+ features

### `EF_SPARC_SUN_US1`

```rust
const EF_SPARC_SUN_US1: u32 = 512u32;
```

Sun UltraSPARC1 extensions

### `EF_SPARC_HAL_R1`

```rust
const EF_SPARC_HAL_R1: u32 = 1_024u32;
```

HAL R1 extensions

### `EF_SPARC_SUN_US3`

```rust
const EF_SPARC_SUN_US3: u32 = 2_048u32;
```

Sun UltraSPARCIII extensions

### `R_SPARC_NONE`

```rust
const R_SPARC_NONE: u32 = 0u32;
```

No reloc

### `R_SPARC_8`

```rust
const R_SPARC_8: u32 = 1u32;
```

Direct 8 bit

### `R_SPARC_16`

```rust
const R_SPARC_16: u32 = 2u32;
```

Direct 16 bit

### `R_SPARC_32`

```rust
const R_SPARC_32: u32 = 3u32;
```

Direct 32 bit

### `R_SPARC_DISP8`

```rust
const R_SPARC_DISP8: u32 = 4u32;
```

PC relative 8 bit

### `R_SPARC_DISP16`

```rust
const R_SPARC_DISP16: u32 = 5u32;
```

PC relative 16 bit

### `R_SPARC_DISP32`

```rust
const R_SPARC_DISP32: u32 = 6u32;
```

PC relative 32 bit

### `R_SPARC_WDISP30`

```rust
const R_SPARC_WDISP30: u32 = 7u32;
```

PC relative 30 bit shifted

### `R_SPARC_WDISP22`

```rust
const R_SPARC_WDISP22: u32 = 8u32;
```

PC relative 22 bit shifted

### `R_SPARC_HI22`

```rust
const R_SPARC_HI22: u32 = 9u32;
```

High 22 bit

### `R_SPARC_22`

```rust
const R_SPARC_22: u32 = 10u32;
```

Direct 22 bit

### `R_SPARC_13`

```rust
const R_SPARC_13: u32 = 11u32;
```

Direct 13 bit

### `R_SPARC_LO10`

```rust
const R_SPARC_LO10: u32 = 12u32;
```

Truncated 10 bit

### `R_SPARC_GOT10`

```rust
const R_SPARC_GOT10: u32 = 13u32;
```

Truncated 10 bit GOT entry

### `R_SPARC_GOT13`

```rust
const R_SPARC_GOT13: u32 = 14u32;
```

13 bit GOT entry

### `R_SPARC_GOT22`

```rust
const R_SPARC_GOT22: u32 = 15u32;
```

22 bit GOT entry shifted

### `R_SPARC_PC10`

```rust
const R_SPARC_PC10: u32 = 16u32;
```

PC relative 10 bit truncated

### `R_SPARC_PC22`

```rust
const R_SPARC_PC22: u32 = 17u32;
```

PC relative 22 bit shifted

### `R_SPARC_WPLT30`

```rust
const R_SPARC_WPLT30: u32 = 18u32;
```

30 bit PC relative PLT address

### `R_SPARC_COPY`

```rust
const R_SPARC_COPY: u32 = 19u32;
```

Copy symbol at runtime

### `R_SPARC_GLOB_DAT`

```rust
const R_SPARC_GLOB_DAT: u32 = 20u32;
```

Create GOT entry

### `R_SPARC_JMP_SLOT`

```rust
const R_SPARC_JMP_SLOT: u32 = 21u32;
```

Create PLT entry

### `R_SPARC_RELATIVE`

```rust
const R_SPARC_RELATIVE: u32 = 22u32;
```

Adjust by program base

### `R_SPARC_UA32`

```rust
const R_SPARC_UA32: u32 = 23u32;
```

Direct 32 bit unaligned

### `R_SPARC_PLT32`

```rust
const R_SPARC_PLT32: u32 = 24u32;
```

Direct 32 bit ref to PLT entry

### `R_SPARC_HIPLT22`

```rust
const R_SPARC_HIPLT22: u32 = 25u32;
```

High 22 bit PLT entry

### `R_SPARC_LOPLT10`

```rust
const R_SPARC_LOPLT10: u32 = 26u32;
```

Truncated 10 bit PLT entry

### `R_SPARC_PCPLT32`

```rust
const R_SPARC_PCPLT32: u32 = 27u32;
```

PC rel 32 bit ref to PLT entry

### `R_SPARC_PCPLT22`

```rust
const R_SPARC_PCPLT22: u32 = 28u32;
```

PC rel high 22 bit PLT entry

### `R_SPARC_PCPLT10`

```rust
const R_SPARC_PCPLT10: u32 = 29u32;
```

PC rel trunc 10 bit PLT entry

### `R_SPARC_10`

```rust
const R_SPARC_10: u32 = 30u32;
```

Direct 10 bit

### `R_SPARC_11`

```rust
const R_SPARC_11: u32 = 31u32;
```

Direct 11 bit

### `R_SPARC_64`

```rust
const R_SPARC_64: u32 = 32u32;
```

Direct 64 bit

### `R_SPARC_OLO10`

```rust
const R_SPARC_OLO10: u32 = 33u32;
```

10bit with secondary 13bit addend

### `R_SPARC_HH22`

```rust
const R_SPARC_HH22: u32 = 34u32;
```

Top 22 bits of direct 64 bit

### `R_SPARC_HM10`

```rust
const R_SPARC_HM10: u32 = 35u32;
```

High middle 10 bits of ...

### `R_SPARC_LM22`

```rust
const R_SPARC_LM22: u32 = 36u32;
```

Low middle 22 bits of ...

### `R_SPARC_PC_HH22`

```rust
const R_SPARC_PC_HH22: u32 = 37u32;
```

Top 22 bits of pc rel 64 bit

### `R_SPARC_PC_HM10`

```rust
const R_SPARC_PC_HM10: u32 = 38u32;
```

High middle 10 bit of ...

### `R_SPARC_PC_LM22`

```rust
const R_SPARC_PC_LM22: u32 = 39u32;
```

Low miggle 22 bits of ...

### `R_SPARC_WDISP16`

```rust
const R_SPARC_WDISP16: u32 = 40u32;
```

PC relative 16 bit shifted

### `R_SPARC_WDISP19`

```rust
const R_SPARC_WDISP19: u32 = 41u32;
```

PC relative 19 bit shifted

### `R_SPARC_GLOB_JMP`

```rust
const R_SPARC_GLOB_JMP: u32 = 42u32;
```

was part of v9 ABI but was removed

### `R_SPARC_7`

```rust
const R_SPARC_7: u32 = 43u32;
```

Direct 7 bit

### `R_SPARC_5`

```rust
const R_SPARC_5: u32 = 44u32;
```

Direct 5 bit

### `R_SPARC_6`

```rust
const R_SPARC_6: u32 = 45u32;
```

Direct 6 bit

### `R_SPARC_DISP64`

```rust
const R_SPARC_DISP64: u32 = 46u32;
```

PC relative 64 bit

### `R_SPARC_PLT64`

```rust
const R_SPARC_PLT64: u32 = 47u32;
```

Direct 64 bit ref to PLT entry

### `R_SPARC_HIX22`

```rust
const R_SPARC_HIX22: u32 = 48u32;
```

High 22 bit complemented

### `R_SPARC_LOX10`

```rust
const R_SPARC_LOX10: u32 = 49u32;
```

Truncated 11 bit complemented

### `R_SPARC_H44`

```rust
const R_SPARC_H44: u32 = 50u32;
```

Direct high 12 of 44 bit

### `R_SPARC_M44`

```rust
const R_SPARC_M44: u32 = 51u32;
```

Direct mid 22 of 44 bit

### `R_SPARC_L44`

```rust
const R_SPARC_L44: u32 = 52u32;
```

Direct low 10 of 44 bit

### `R_SPARC_REGISTER`

```rust
const R_SPARC_REGISTER: u32 = 53u32;
```

Global register usage

### `R_SPARC_UA64`

```rust
const R_SPARC_UA64: u32 = 54u32;
```

Direct 64 bit unaligned

### `R_SPARC_UA16`

```rust
const R_SPARC_UA16: u32 = 55u32;
```

Direct 16 bit unaligned

### `R_SPARC_TLS_GD_HI22`

```rust
const R_SPARC_TLS_GD_HI22: u32 = 56u32;
```

### `R_SPARC_TLS_GD_LO10`

```rust
const R_SPARC_TLS_GD_LO10: u32 = 57u32;
```

### `R_SPARC_TLS_GD_ADD`

```rust
const R_SPARC_TLS_GD_ADD: u32 = 58u32;
```

### `R_SPARC_TLS_GD_CALL`

```rust
const R_SPARC_TLS_GD_CALL: u32 = 59u32;
```

### `R_SPARC_TLS_LDM_HI22`

```rust
const R_SPARC_TLS_LDM_HI22: u32 = 60u32;
```

### `R_SPARC_TLS_LDM_LO10`

```rust
const R_SPARC_TLS_LDM_LO10: u32 = 61u32;
```

### `R_SPARC_TLS_LDM_ADD`

```rust
const R_SPARC_TLS_LDM_ADD: u32 = 62u32;
```

### `R_SPARC_TLS_LDM_CALL`

```rust
const R_SPARC_TLS_LDM_CALL: u32 = 63u32;
```

### `R_SPARC_TLS_LDO_HIX22`

```rust
const R_SPARC_TLS_LDO_HIX22: u32 = 64u32;
```

### `R_SPARC_TLS_LDO_LOX10`

```rust
const R_SPARC_TLS_LDO_LOX10: u32 = 65u32;
```

### `R_SPARC_TLS_LDO_ADD`

```rust
const R_SPARC_TLS_LDO_ADD: u32 = 66u32;
```

### `R_SPARC_TLS_IE_HI22`

```rust
const R_SPARC_TLS_IE_HI22: u32 = 67u32;
```

### `R_SPARC_TLS_IE_LO10`

```rust
const R_SPARC_TLS_IE_LO10: u32 = 68u32;
```

### `R_SPARC_TLS_IE_LD`

```rust
const R_SPARC_TLS_IE_LD: u32 = 69u32;
```

### `R_SPARC_TLS_IE_LDX`

```rust
const R_SPARC_TLS_IE_LDX: u32 = 70u32;
```

### `R_SPARC_TLS_IE_ADD`

```rust
const R_SPARC_TLS_IE_ADD: u32 = 71u32;
```

### `R_SPARC_TLS_LE_HIX22`

```rust
const R_SPARC_TLS_LE_HIX22: u32 = 72u32;
```

### `R_SPARC_TLS_LE_LOX10`

```rust
const R_SPARC_TLS_LE_LOX10: u32 = 73u32;
```

### `R_SPARC_TLS_DTPMOD32`

```rust
const R_SPARC_TLS_DTPMOD32: u32 = 74u32;
```

### `R_SPARC_TLS_DTPMOD64`

```rust
const R_SPARC_TLS_DTPMOD64: u32 = 75u32;
```

### `R_SPARC_TLS_DTPOFF32`

```rust
const R_SPARC_TLS_DTPOFF32: u32 = 76u32;
```

### `R_SPARC_TLS_DTPOFF64`

```rust
const R_SPARC_TLS_DTPOFF64: u32 = 77u32;
```

### `R_SPARC_TLS_TPOFF32`

```rust
const R_SPARC_TLS_TPOFF32: u32 = 78u32;
```

### `R_SPARC_TLS_TPOFF64`

```rust
const R_SPARC_TLS_TPOFF64: u32 = 79u32;
```

### `R_SPARC_GOTDATA_HIX22`

```rust
const R_SPARC_GOTDATA_HIX22: u32 = 80u32;
```

### `R_SPARC_GOTDATA_LOX10`

```rust
const R_SPARC_GOTDATA_LOX10: u32 = 81u32;
```

### `R_SPARC_GOTDATA_OP_HIX22`

```rust
const R_SPARC_GOTDATA_OP_HIX22: u32 = 82u32;
```

### `R_SPARC_GOTDATA_OP_LOX10`

```rust
const R_SPARC_GOTDATA_OP_LOX10: u32 = 83u32;
```

### `R_SPARC_GOTDATA_OP`

```rust
const R_SPARC_GOTDATA_OP: u32 = 84u32;
```

### `R_SPARC_H34`

```rust
const R_SPARC_H34: u32 = 85u32;
```

### `R_SPARC_SIZE32`

```rust
const R_SPARC_SIZE32: u32 = 86u32;
```

### `R_SPARC_SIZE64`

```rust
const R_SPARC_SIZE64: u32 = 87u32;
```

### `R_SPARC_WDISP10`

```rust
const R_SPARC_WDISP10: u32 = 88u32;
```

### `R_SPARC_JMP_IREL`

```rust
const R_SPARC_JMP_IREL: u32 = 248u32;
```

### `R_SPARC_IRELATIVE`

```rust
const R_SPARC_IRELATIVE: u32 = 249u32;
```

### `R_SPARC_GNU_VTINHERIT`

```rust
const R_SPARC_GNU_VTINHERIT: u32 = 250u32;
```

### `R_SPARC_GNU_VTENTRY`

```rust
const R_SPARC_GNU_VTENTRY: u32 = 251u32;
```

### `R_SPARC_REV32`

```rust
const R_SPARC_REV32: u32 = 252u32;
```

### `DT_SPARC_REGISTER`

```rust
const DT_SPARC_REGISTER: u32 = 1_879_048_193u32;
```

### `EF_MIPS_NOREORDER`

```rust
const EF_MIPS_NOREORDER: u32 = 1u32;
```

A .noreorder directive was used.

### `EF_MIPS_PIC`

```rust
const EF_MIPS_PIC: u32 = 2u32;
```

Contains PIC code.

### `EF_MIPS_CPIC`

```rust
const EF_MIPS_CPIC: u32 = 4u32;
```

Uses PIC calling sequence.

### `EF_MIPS_XGOT`

```rust
const EF_MIPS_XGOT: u32 = 8u32;
```

### `EF_MIPS_64BIT_WHIRL`

```rust
const EF_MIPS_64BIT_WHIRL: u32 = 16u32;
```

### `EF_MIPS_ABI2`

```rust
const EF_MIPS_ABI2: u32 = 32u32;
```

### `EF_MIPS_ABI_ON32`

```rust
const EF_MIPS_ABI_ON32: u32 = 64u32;
```

### `EF_MIPS_FP64`

```rust
const EF_MIPS_FP64: u32 = 512u32;
```

Uses FP64 (12 callee-saved).

### `EF_MIPS_NAN2008`

```rust
const EF_MIPS_NAN2008: u32 = 1_024u32;
```

Uses IEEE 754-2008 NaN encoding.

### `EF_MIPS_ARCH`

```rust
const EF_MIPS_ARCH: u32 = 4_026_531_840u32;
```

MIPS architecture level.

### `EF_MIPS_ABI_O32`

```rust
const EF_MIPS_ABI_O32: u32 = 4_096u32;
```

The first MIPS 32 bit ABI

### `EF_MIPS_ABI_O64`

```rust
const EF_MIPS_ABI_O64: u32 = 8_192u32;
```

O32 ABI extended for 64-bit architectures

### `EF_MIPS_ABI_EABI32`

```rust
const EF_MIPS_ABI_EABI32: u32 = 12_288u32;
```

EABI in 32-bit mode

### `EF_MIPS_ABI_EABI64`

```rust
const EF_MIPS_ABI_EABI64: u32 = 16_384u32;
```

EABI in 64-bit mode

### `EF_MIPS_ABI`

```rust
const EF_MIPS_ABI: u32 = 61_440u32;
```

Mask for selecting EF_MIPS_ABI_ variant

### `EF_MIPS_ARCH_1`

```rust
const EF_MIPS_ARCH_1: u32 = 0u32;
```

-mips1 code.

### `EF_MIPS_ARCH_2`

```rust
const EF_MIPS_ARCH_2: u32 = 268_435_456u32;
```

-mips2 code.

### `EF_MIPS_ARCH_3`

```rust
const EF_MIPS_ARCH_3: u32 = 536_870_912u32;
```

-mips3 code.

### `EF_MIPS_ARCH_4`

```rust
const EF_MIPS_ARCH_4: u32 = 805_306_368u32;
```

-mips4 code.

### `EF_MIPS_ARCH_5`

```rust
const EF_MIPS_ARCH_5: u32 = 1_073_741_824u32;
```

-mips5 code.

### `EF_MIPS_ARCH_32`

```rust
const EF_MIPS_ARCH_32: u32 = 1_342_177_280u32;
```

MIPS32 code.

### `EF_MIPS_ARCH_64`

```rust
const EF_MIPS_ARCH_64: u32 = 1_610_612_736u32;
```

MIPS64 code.

### `EF_MIPS_ARCH_32R2`

```rust
const EF_MIPS_ARCH_32R2: u32 = 1_879_048_192u32;
```

MIPS32r2 code.

### `EF_MIPS_ARCH_64R2`

```rust
const EF_MIPS_ARCH_64R2: u32 = 2_147_483_648u32;
```

MIPS64r2 code.

### `EF_MIPS_ARCH_32R6`

```rust
const EF_MIPS_ARCH_32R6: u32 = 2_415_919_104u32;
```

MIPS32r6 code

### `EF_MIPS_ARCH_64R6`

```rust
const EF_MIPS_ARCH_64R6: u32 = 2_684_354_560u32;
```

MIPS64r6 code

### `SHN_MIPS_ACOMMON`

```rust
const SHN_MIPS_ACOMMON: u16 = 65_280u16;
```

Allocated common symbols.

### `SHN_MIPS_TEXT`

```rust
const SHN_MIPS_TEXT: u16 = 65_281u16;
```

Allocated test symbols.

### `SHN_MIPS_DATA`

```rust
const SHN_MIPS_DATA: u16 = 65_282u16;
```

Allocated data symbols.

### `SHN_MIPS_SCOMMON`

```rust
const SHN_MIPS_SCOMMON: u16 = 65_283u16;
```

Small common symbols.

### `SHN_MIPS_SUNDEFINED`

```rust
const SHN_MIPS_SUNDEFINED: u16 = 65_284u16;
```

Small undefined symbols.

### `SHT_MIPS_LIBLIST`

```rust
const SHT_MIPS_LIBLIST: u32 = 1_879_048_192u32;
```

Shared objects used in link.

### `SHT_MIPS_MSYM`

```rust
const SHT_MIPS_MSYM: u32 = 1_879_048_193u32;
```

### `SHT_MIPS_CONFLICT`

```rust
const SHT_MIPS_CONFLICT: u32 = 1_879_048_194u32;
```

Conflicting symbols.

### `SHT_MIPS_GPTAB`

```rust
const SHT_MIPS_GPTAB: u32 = 1_879_048_195u32;
```

Global data area sizes.

### `SHT_MIPS_UCODE`

```rust
const SHT_MIPS_UCODE: u32 = 1_879_048_196u32;
```

Reserved for SGI/MIPS compilers

### `SHT_MIPS_DEBUG`

```rust
const SHT_MIPS_DEBUG: u32 = 1_879_048_197u32;
```

MIPS ECOFF debugging info.

### `SHT_MIPS_REGINFO`

```rust
const SHT_MIPS_REGINFO: u32 = 1_879_048_198u32;
```

Register usage information.

### `SHT_MIPS_PACKAGE`

```rust
const SHT_MIPS_PACKAGE: u32 = 1_879_048_199u32;
```

### `SHT_MIPS_PACKSYM`

```rust
const SHT_MIPS_PACKSYM: u32 = 1_879_048_200u32;
```

### `SHT_MIPS_RELD`

```rust
const SHT_MIPS_RELD: u32 = 1_879_048_201u32;
```

### `SHT_MIPS_IFACE`

```rust
const SHT_MIPS_IFACE: u32 = 1_879_048_203u32;
```

### `SHT_MIPS_CONTENT`

```rust
const SHT_MIPS_CONTENT: u32 = 1_879_048_204u32;
```

### `SHT_MIPS_OPTIONS`

```rust
const SHT_MIPS_OPTIONS: u32 = 1_879_048_205u32;
```

Miscellaneous options.

### `SHT_MIPS_SHDR`

```rust
const SHT_MIPS_SHDR: u32 = 1_879_048_208u32;
```

### `SHT_MIPS_FDESC`

```rust
const SHT_MIPS_FDESC: u32 = 1_879_048_209u32;
```

### `SHT_MIPS_EXTSYM`

```rust
const SHT_MIPS_EXTSYM: u32 = 1_879_048_210u32;
```

### `SHT_MIPS_DENSE`

```rust
const SHT_MIPS_DENSE: u32 = 1_879_048_211u32;
```

### `SHT_MIPS_PDESC`

```rust
const SHT_MIPS_PDESC: u32 = 1_879_048_212u32;
```

### `SHT_MIPS_LOCSYM`

```rust
const SHT_MIPS_LOCSYM: u32 = 1_879_048_213u32;
```

### `SHT_MIPS_AUXSYM`

```rust
const SHT_MIPS_AUXSYM: u32 = 1_879_048_214u32;
```

### `SHT_MIPS_OPTSYM`

```rust
const SHT_MIPS_OPTSYM: u32 = 1_879_048_215u32;
```

### `SHT_MIPS_LOCSTR`

```rust
const SHT_MIPS_LOCSTR: u32 = 1_879_048_216u32;
```

### `SHT_MIPS_LINE`

```rust
const SHT_MIPS_LINE: u32 = 1_879_048_217u32;
```

### `SHT_MIPS_RFDESC`

```rust
const SHT_MIPS_RFDESC: u32 = 1_879_048_218u32;
```

### `SHT_MIPS_DELTASYM`

```rust
const SHT_MIPS_DELTASYM: u32 = 1_879_048_219u32;
```

### `SHT_MIPS_DELTAINST`

```rust
const SHT_MIPS_DELTAINST: u32 = 1_879_048_220u32;
```

### `SHT_MIPS_DELTACLASS`

```rust
const SHT_MIPS_DELTACLASS: u32 = 1_879_048_221u32;
```

### `SHT_MIPS_DWARF`

```rust
const SHT_MIPS_DWARF: u32 = 1_879_048_222u32;
```

DWARF debugging information.

### `SHT_MIPS_DELTADECL`

```rust
const SHT_MIPS_DELTADECL: u32 = 1_879_048_223u32;
```

### `SHT_MIPS_SYMBOL_LIB`

```rust
const SHT_MIPS_SYMBOL_LIB: u32 = 1_879_048_224u32;
```

### `SHT_MIPS_EVENTS`

```rust
const SHT_MIPS_EVENTS: u32 = 1_879_048_225u32;
```

Event section.

### `SHT_MIPS_TRANSLATE`

```rust
const SHT_MIPS_TRANSLATE: u32 = 1_879_048_226u32;
```

### `SHT_MIPS_PIXIE`

```rust
const SHT_MIPS_PIXIE: u32 = 1_879_048_227u32;
```

### `SHT_MIPS_XLATE`

```rust
const SHT_MIPS_XLATE: u32 = 1_879_048_228u32;
```

### `SHT_MIPS_XLATE_DEBUG`

```rust
const SHT_MIPS_XLATE_DEBUG: u32 = 1_879_048_229u32;
```

### `SHT_MIPS_WHIRL`

```rust
const SHT_MIPS_WHIRL: u32 = 1_879_048_230u32;
```

### `SHT_MIPS_EH_REGION`

```rust
const SHT_MIPS_EH_REGION: u32 = 1_879_048_231u32;
```

### `SHT_MIPS_XLATE_OLD`

```rust
const SHT_MIPS_XLATE_OLD: u32 = 1_879_048_232u32;
```

### `SHT_MIPS_PDR_EXCEPTION`

```rust
const SHT_MIPS_PDR_EXCEPTION: u32 = 1_879_048_233u32;
```

### `SHF_MIPS_GPREL`

```rust
const SHF_MIPS_GPREL: u32 = 268_435_456u32;
```

Must be in global data area.

### `SHF_MIPS_MERGE`

```rust
const SHF_MIPS_MERGE: u32 = 536_870_912u32;
```

### `SHF_MIPS_ADDR`

```rust
const SHF_MIPS_ADDR: u32 = 1_073_741_824u32;
```

### `SHF_MIPS_STRINGS`

```rust
const SHF_MIPS_STRINGS: u32 = 2_147_483_648u32;
```

### `SHF_MIPS_NOSTRIP`

```rust
const SHF_MIPS_NOSTRIP: u32 = 134_217_728u32;
```

### `SHF_MIPS_LOCAL`

```rust
const SHF_MIPS_LOCAL: u32 = 67_108_864u32;
```

### `SHF_MIPS_NAMES`

```rust
const SHF_MIPS_NAMES: u32 = 33_554_432u32;
```

### `SHF_MIPS_NODUPE`

```rust
const SHF_MIPS_NODUPE: u32 = 16_777_216u32;
```

### `STO_MIPS_PLT`

```rust
const STO_MIPS_PLT: u8 = 8u8;
```

### `STO_MIPS_SC_ALIGN_UNUSED`

```rust
const STO_MIPS_SC_ALIGN_UNUSED: u8 = 255u8;
```

Only valid for `STB_MIPS_SPLIT_COMMON`.

### `STB_MIPS_SPLIT_COMMON`

```rust
const STB_MIPS_SPLIT_COMMON: u8 = 13u8;
```

### `ODK_NULL`

```rust
const ODK_NULL: u32 = 0u32;
```

Undefined.

### `ODK_REGINFO`

```rust
const ODK_REGINFO: u32 = 1u32;
```

Register usage information.

### `ODK_EXCEPTIONS`

```rust
const ODK_EXCEPTIONS: u32 = 2u32;
```

Exception processing options.

### `ODK_PAD`

```rust
const ODK_PAD: u32 = 3u32;
```

Section padding options.

### `ODK_HWPATCH`

```rust
const ODK_HWPATCH: u32 = 4u32;
```

Hardware workarounds performed

### `ODK_FILL`

```rust
const ODK_FILL: u32 = 5u32;
```

record the fill value used by the linker.

### `ODK_TAGS`

```rust
const ODK_TAGS: u32 = 6u32;
```

reserve space for desktop tools to write.

### `ODK_HWAND`

```rust
const ODK_HWAND: u32 = 7u32;
```

HW workarounds.  'AND' bits when merging.

### `ODK_HWOR`

```rust
const ODK_HWOR: u32 = 8u32;
```

HW workarounds.  'OR' bits when merging.

### `OEX_FPU_MIN`

```rust
const OEX_FPU_MIN: u32 = 31u32;
```

FPE's which MUST be enabled.

### `OEX_FPU_MAX`

```rust
const OEX_FPU_MAX: u32 = 7_936u32;
```

FPE's which MAY be enabled.

### `OEX_PAGE0`

```rust
const OEX_PAGE0: u32 = 65_536u32;
```

page zero must be mapped.

### `OEX_SMM`

```rust
const OEX_SMM: u32 = 131_072u32;
```

Force sequential memory mode?

### `OEX_FPDBUG`

```rust
const OEX_FPDBUG: u32 = 262_144u32;
```

Force floating point debug mode?

### `OEX_PRECISEFP`

```rust
const OEX_PRECISEFP: u32 = 262_144u32;
```

### `OEX_DISMISS`

```rust
const OEX_DISMISS: u32 = 524_288u32;
```

Dismiss invalid address faults?

### `OEX_FPU_INVAL`

```rust
const OEX_FPU_INVAL: u32 = 16u32;
```

### `OEX_FPU_DIV0`

```rust
const OEX_FPU_DIV0: u32 = 8u32;
```

### `OEX_FPU_OFLO`

```rust
const OEX_FPU_OFLO: u32 = 4u32;
```

### `OEX_FPU_UFLO`

```rust
const OEX_FPU_UFLO: u32 = 2u32;
```

### `OEX_FPU_INEX`

```rust
const OEX_FPU_INEX: u32 = 1u32;
```

### `OHW_R4KEOP`

```rust
const OHW_R4KEOP: u32 = 1u32;
```

R4000 end-of-page patch.

### `OHW_R8KPFETCH`

```rust
const OHW_R8KPFETCH: u32 = 2u32;
```

may need R8000 prefetch patch.

### `OHW_R5KEOP`

```rust
const OHW_R5KEOP: u32 = 4u32;
```

R5000 end-of-page patch.

### `OHW_R5KCVTL`

```rust
const OHW_R5KCVTL: u32 = 8u32;
```

R5000 cvt.\[ds\].l bug.  clean=1.

### `OPAD_PREFIX`

```rust
const OPAD_PREFIX: u32 = 1u32;
```

### `OPAD_POSTFIX`

```rust
const OPAD_POSTFIX: u32 = 2u32;
```

### `OPAD_SYMBOL`

```rust
const OPAD_SYMBOL: u32 = 4u32;
```

### `OHWA0_R4KEOP_CHECKED`

```rust
const OHWA0_R4KEOP_CHECKED: u32 = 1u32;
```

### `OHWA1_R4KEOP_CLEAN`

```rust
const OHWA1_R4KEOP_CLEAN: u32 = 2u32;
```

### `R_MIPS_NONE`

```rust
const R_MIPS_NONE: u32 = 0u32;
```

No reloc

### `R_MIPS_16`

```rust
const R_MIPS_16: u32 = 1u32;
```

Direct 16 bit

### `R_MIPS_32`

```rust
const R_MIPS_32: u32 = 2u32;
```

Direct 32 bit

### `R_MIPS_REL32`

```rust
const R_MIPS_REL32: u32 = 3u32;
```

PC relative 32 bit

### `R_MIPS_26`

```rust
const R_MIPS_26: u32 = 4u32;
```

Direct 26 bit shifted

### `R_MIPS_HI16`

```rust
const R_MIPS_HI16: u32 = 5u32;
```

High 16 bit

### `R_MIPS_LO16`

```rust
const R_MIPS_LO16: u32 = 6u32;
```

Low 16 bit

### `R_MIPS_GPREL16`

```rust
const R_MIPS_GPREL16: u32 = 7u32;
```

GP relative 16 bit

### `R_MIPS_LITERAL`

```rust
const R_MIPS_LITERAL: u32 = 8u32;
```

16 bit literal entry

### `R_MIPS_GOT16`

```rust
const R_MIPS_GOT16: u32 = 9u32;
```

16 bit GOT entry

### `R_MIPS_PC16`

```rust
const R_MIPS_PC16: u32 = 10u32;
```

PC relative 16 bit

### `R_MIPS_CALL16`

```rust
const R_MIPS_CALL16: u32 = 11u32;
```

16 bit GOT entry for function

### `R_MIPS_GPREL32`

```rust
const R_MIPS_GPREL32: u32 = 12u32;
```

GP relative 32 bit

### `R_MIPS_SHIFT5`

```rust
const R_MIPS_SHIFT5: u32 = 16u32;
```

### `R_MIPS_SHIFT6`

```rust
const R_MIPS_SHIFT6: u32 = 17u32;
```

### `R_MIPS_64`

```rust
const R_MIPS_64: u32 = 18u32;
```

### `R_MIPS_GOT_DISP`

```rust
const R_MIPS_GOT_DISP: u32 = 19u32;
```

### `R_MIPS_GOT_PAGE`

```rust
const R_MIPS_GOT_PAGE: u32 = 20u32;
```

### `R_MIPS_GOT_OFST`

```rust
const R_MIPS_GOT_OFST: u32 = 21u32;
```

### `R_MIPS_GOT_HI16`

```rust
const R_MIPS_GOT_HI16: u32 = 22u32;
```

### `R_MIPS_GOT_LO16`

```rust
const R_MIPS_GOT_LO16: u32 = 23u32;
```

### `R_MIPS_SUB`

```rust
const R_MIPS_SUB: u32 = 24u32;
```

### `R_MIPS_INSERT_A`

```rust
const R_MIPS_INSERT_A: u32 = 25u32;
```

### `R_MIPS_INSERT_B`

```rust
const R_MIPS_INSERT_B: u32 = 26u32;
```

### `R_MIPS_DELETE`

```rust
const R_MIPS_DELETE: u32 = 27u32;
```

### `R_MIPS_HIGHER`

```rust
const R_MIPS_HIGHER: u32 = 28u32;
```

### `R_MIPS_HIGHEST`

```rust
const R_MIPS_HIGHEST: u32 = 29u32;
```

### `R_MIPS_CALL_HI16`

```rust
const R_MIPS_CALL_HI16: u32 = 30u32;
```

### `R_MIPS_CALL_LO16`

```rust
const R_MIPS_CALL_LO16: u32 = 31u32;
```

### `R_MIPS_SCN_DISP`

```rust
const R_MIPS_SCN_DISP: u32 = 32u32;
```

### `R_MIPS_REL16`

```rust
const R_MIPS_REL16: u32 = 33u32;
```

### `R_MIPS_ADD_IMMEDIATE`

```rust
const R_MIPS_ADD_IMMEDIATE: u32 = 34u32;
```

### `R_MIPS_PJUMP`

```rust
const R_MIPS_PJUMP: u32 = 35u32;
```

### `R_MIPS_RELGOT`

```rust
const R_MIPS_RELGOT: u32 = 36u32;
```

### `R_MIPS_JALR`

```rust
const R_MIPS_JALR: u32 = 37u32;
```

### `R_MIPS_TLS_DTPMOD32`

```rust
const R_MIPS_TLS_DTPMOD32: u32 = 38u32;
```

Module number 32 bit

### `R_MIPS_TLS_DTPREL32`

```rust
const R_MIPS_TLS_DTPREL32: u32 = 39u32;
```

Module-relative offset 32 bit

### `R_MIPS_TLS_DTPMOD64`

```rust
const R_MIPS_TLS_DTPMOD64: u32 = 40u32;
```

Module number 64 bit

### `R_MIPS_TLS_DTPREL64`

```rust
const R_MIPS_TLS_DTPREL64: u32 = 41u32;
```

Module-relative offset 64 bit

### `R_MIPS_TLS_GD`

```rust
const R_MIPS_TLS_GD: u32 = 42u32;
```

16 bit GOT offset for GD

### `R_MIPS_TLS_LDM`

```rust
const R_MIPS_TLS_LDM: u32 = 43u32;
```

16 bit GOT offset for LDM

### `R_MIPS_TLS_DTPREL_HI16`

```rust
const R_MIPS_TLS_DTPREL_HI16: u32 = 44u32;
```

Module-relative offset, high 16 bits

### `R_MIPS_TLS_DTPREL_LO16`

```rust
const R_MIPS_TLS_DTPREL_LO16: u32 = 45u32;
```

Module-relative offset, low 16 bits

### `R_MIPS_TLS_GOTTPREL`

```rust
const R_MIPS_TLS_GOTTPREL: u32 = 46u32;
```

16 bit GOT offset for IE

### `R_MIPS_TLS_TPREL32`

```rust
const R_MIPS_TLS_TPREL32: u32 = 47u32;
```

TP-relative offset, 32 bit

### `R_MIPS_TLS_TPREL64`

```rust
const R_MIPS_TLS_TPREL64: u32 = 48u32;
```

TP-relative offset, 64 bit

### `R_MIPS_TLS_TPREL_HI16`

```rust
const R_MIPS_TLS_TPREL_HI16: u32 = 49u32;
```

TP-relative offset, high 16 bits

### `R_MIPS_TLS_TPREL_LO16`

```rust
const R_MIPS_TLS_TPREL_LO16: u32 = 50u32;
```

TP-relative offset, low 16 bits

### `R_MIPS_GLOB_DAT`

```rust
const R_MIPS_GLOB_DAT: u32 = 51u32;
```

### `R_MIPS_COPY`

```rust
const R_MIPS_COPY: u32 = 126u32;
```

### `R_MIPS_JUMP_SLOT`

```rust
const R_MIPS_JUMP_SLOT: u32 = 127u32;
```

### `PT_MIPS_REGINFO`

```rust
const PT_MIPS_REGINFO: u32 = 1_879_048_192u32;
```

Register usage information.

### `PT_MIPS_RTPROC`

```rust
const PT_MIPS_RTPROC: u32 = 1_879_048_193u32;
```

Runtime procedure table.

### `PT_MIPS_OPTIONS`

```rust
const PT_MIPS_OPTIONS: u32 = 1_879_048_194u32;
```

### `PT_MIPS_ABIFLAGS`

```rust
const PT_MIPS_ABIFLAGS: u32 = 1_879_048_195u32;
```

FP mode requirement.

### `PF_MIPS_LOCAL`

```rust
const PF_MIPS_LOCAL: u32 = 268_435_456u32;
```

### `DT_MIPS_RLD_VERSION`

```rust
const DT_MIPS_RLD_VERSION: u32 = 1_879_048_193u32;
```

Runtime linker interface version

### `DT_MIPS_TIME_STAMP`

```rust
const DT_MIPS_TIME_STAMP: u32 = 1_879_048_194u32;
```

Timestamp

### `DT_MIPS_ICHECKSUM`

```rust
const DT_MIPS_ICHECKSUM: u32 = 1_879_048_195u32;
```

Checksum

### `DT_MIPS_IVERSION`

```rust
const DT_MIPS_IVERSION: u32 = 1_879_048_196u32;
```

Version string (string tbl index)

### `DT_MIPS_FLAGS`

```rust
const DT_MIPS_FLAGS: u32 = 1_879_048_197u32;
```

Flags

### `DT_MIPS_BASE_ADDRESS`

```rust
const DT_MIPS_BASE_ADDRESS: u32 = 1_879_048_198u32;
```

Base address

### `DT_MIPS_MSYM`

```rust
const DT_MIPS_MSYM: u32 = 1_879_048_199u32;
```

### `DT_MIPS_CONFLICT`

```rust
const DT_MIPS_CONFLICT: u32 = 1_879_048_200u32;
```

Address of CONFLICT section

### `DT_MIPS_LIBLIST`

```rust
const DT_MIPS_LIBLIST: u32 = 1_879_048_201u32;
```

Address of LIBLIST section

### `DT_MIPS_LOCAL_GOTNO`

```rust
const DT_MIPS_LOCAL_GOTNO: u32 = 1_879_048_202u32;
```

Number of local GOT entries

### `DT_MIPS_CONFLICTNO`

```rust
const DT_MIPS_CONFLICTNO: u32 = 1_879_048_203u32;
```

Number of CONFLICT entries

### `DT_MIPS_LIBLISTNO`

```rust
const DT_MIPS_LIBLISTNO: u32 = 1_879_048_208u32;
```

Number of LIBLIST entries

### `DT_MIPS_SYMTABNO`

```rust
const DT_MIPS_SYMTABNO: u32 = 1_879_048_209u32;
```

Number of DYNSYM entries

### `DT_MIPS_UNREFEXTNO`

```rust
const DT_MIPS_UNREFEXTNO: u32 = 1_879_048_210u32;
```

First external DYNSYM

### `DT_MIPS_GOTSYM`

```rust
const DT_MIPS_GOTSYM: u32 = 1_879_048_211u32;
```

First GOT entry in DYNSYM

### `DT_MIPS_HIPAGENO`

```rust
const DT_MIPS_HIPAGENO: u32 = 1_879_048_212u32;
```

Number of GOT page table entries

### `DT_MIPS_RLD_MAP`

```rust
const DT_MIPS_RLD_MAP: u32 = 1_879_048_214u32;
```

Address of run time loader map.

### `DT_MIPS_DELTA_CLASS`

```rust
const DT_MIPS_DELTA_CLASS: u32 = 1_879_048_215u32;
```

Delta C++ class definition.

### `DT_MIPS_DELTA_CLASS_NO`

```rust
const DT_MIPS_DELTA_CLASS_NO: u32 = 1_879_048_216u32;
```

Number of entries in DT_MIPS_DELTA_CLASS.

### `DT_MIPS_DELTA_INSTANCE`

```rust
const DT_MIPS_DELTA_INSTANCE: u32 = 1_879_048_217u32;
```

Delta C++ class instances.

### `DT_MIPS_DELTA_INSTANCE_NO`

```rust
const DT_MIPS_DELTA_INSTANCE_NO: u32 = 1_879_048_218u32;
```

Number of entries in DT_MIPS_DELTA_INSTANCE.

### `DT_MIPS_DELTA_RELOC`

```rust
const DT_MIPS_DELTA_RELOC: u32 = 1_879_048_219u32;
```

Delta relocations.

### `DT_MIPS_DELTA_RELOC_NO`

```rust
const DT_MIPS_DELTA_RELOC_NO: u32 = 1_879_048_220u32;
```

Number of entries in DT_MIPS_DELTA_RELOC.

### `DT_MIPS_DELTA_SYM`

```rust
const DT_MIPS_DELTA_SYM: u32 = 1_879_048_221u32;
```

Delta symbols that Delta relocations refer to.

### `DT_MIPS_DELTA_SYM_NO`

```rust
const DT_MIPS_DELTA_SYM_NO: u32 = 1_879_048_222u32;
```

Number of entries in DT_MIPS_DELTA_SYM.

### `DT_MIPS_DELTA_CLASSSYM`

```rust
const DT_MIPS_DELTA_CLASSSYM: u32 = 1_879_048_224u32;
```

Delta symbols that hold the class declaration.

### `DT_MIPS_DELTA_CLASSSYM_NO`

```rust
const DT_MIPS_DELTA_CLASSSYM_NO: u32 = 1_879_048_225u32;
```

Number of entries in DT_MIPS_DELTA_CLASSSYM.

### `DT_MIPS_CXX_FLAGS`

```rust
const DT_MIPS_CXX_FLAGS: u32 = 1_879_048_226u32;
```

Flags indicating for C++ flavor.

### `DT_MIPS_PIXIE_INIT`

```rust
const DT_MIPS_PIXIE_INIT: u32 = 1_879_048_227u32;
```

### `DT_MIPS_SYMBOL_LIB`

```rust
const DT_MIPS_SYMBOL_LIB: u32 = 1_879_048_228u32;
```

### `DT_MIPS_LOCALPAGE_GOTIDX`

```rust
const DT_MIPS_LOCALPAGE_GOTIDX: u32 = 1_879_048_229u32;
```

### `DT_MIPS_LOCAL_GOTIDX`

```rust
const DT_MIPS_LOCAL_GOTIDX: u32 = 1_879_048_230u32;
```

### `DT_MIPS_HIDDEN_GOTIDX`

```rust
const DT_MIPS_HIDDEN_GOTIDX: u32 = 1_879_048_231u32;
```

### `DT_MIPS_PROTECTED_GOTIDX`

```rust
const DT_MIPS_PROTECTED_GOTIDX: u32 = 1_879_048_232u32;
```

### `DT_MIPS_OPTIONS`

```rust
const DT_MIPS_OPTIONS: u32 = 1_879_048_233u32;
```

Address of .options.

### `DT_MIPS_INTERFACE`

```rust
const DT_MIPS_INTERFACE: u32 = 1_879_048_234u32;
```

Address of .interface.

### `DT_MIPS_DYNSTR_ALIGN`

```rust
const DT_MIPS_DYNSTR_ALIGN: u32 = 1_879_048_235u32;
```

### `DT_MIPS_INTERFACE_SIZE`

```rust
const DT_MIPS_INTERFACE_SIZE: u32 = 1_879_048_236u32;
```

Size of the .interface section.

### `DT_MIPS_RLD_TEXT_RESOLVE_ADDR`

```rust
const DT_MIPS_RLD_TEXT_RESOLVE_ADDR: u32 = 1_879_048_237u32;
```

Address of rld_text_rsolve function stored in GOT.

### `DT_MIPS_PERF_SUFFIX`

```rust
const DT_MIPS_PERF_SUFFIX: u32 = 1_879_048_238u32;
```

Default suffix of dso to be added by rld on dlopen() calls.

### `DT_MIPS_COMPACT_SIZE`

```rust
const DT_MIPS_COMPACT_SIZE: u32 = 1_879_048_239u32;
```

(O32)Size of compact rel section.

### `DT_MIPS_GP_VALUE`

```rust
const DT_MIPS_GP_VALUE: u32 = 1_879_048_240u32;
```

GP value for aux GOTs.

### `DT_MIPS_AUX_DYNAMIC`

```rust
const DT_MIPS_AUX_DYNAMIC: u32 = 1_879_048_241u32;
```

Address of aux .dynamic.

### `DT_MIPS_PLTGOT`

```rust
const DT_MIPS_PLTGOT: u32 = 1_879_048_242u32;
```

The address of .got.plt in an executable using the new non-PIC ABI.

### `DT_MIPS_RWPLT`

```rust
const DT_MIPS_RWPLT: u32 = 1_879_048_244u32;
```

The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable.  For a non-writable PLT, this is omitted or has a zero value.

### `DT_MIPS_RLD_MAP_REL`

```rust
const DT_MIPS_RLD_MAP_REL: u32 = 1_879_048_245u32;
```

An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address.

### `RHF_NONE`

```rust
const RHF_NONE: u32 = 0u32;
```

No flags

### `RHF_QUICKSTART`

```rust
const RHF_QUICKSTART: u32 = 1u32;
```

Use quickstart

### `RHF_NOTPOT`

```rust
const RHF_NOTPOT: u32 = 2u32;
```

Hash size not power of 2

### `RHF_NO_LIBRARY_REPLACEMENT`

```rust
const RHF_NO_LIBRARY_REPLACEMENT: u32 = 4u32;
```

Ignore LD_LIBRARY_PATH

### `RHF_NO_MOVE`

```rust
const RHF_NO_MOVE: u32 = 8u32;
```

### `RHF_SGI_ONLY`

```rust
const RHF_SGI_ONLY: u32 = 16u32;
```

### `RHF_GUARANTEE_INIT`

```rust
const RHF_GUARANTEE_INIT: u32 = 32u32;
```

### `RHF_DELTA_C_PLUS_PLUS`

```rust
const RHF_DELTA_C_PLUS_PLUS: u32 = 64u32;
```

### `RHF_GUARANTEE_START_INIT`

```rust
const RHF_GUARANTEE_START_INIT: u32 = 128u32;
```

### `RHF_PIXIE`

```rust
const RHF_PIXIE: u32 = 256u32;
```

### `RHF_DEFAULT_DELAY_LOAD`

```rust
const RHF_DEFAULT_DELAY_LOAD: u32 = 512u32;
```

### `RHF_REQUICKSTART`

```rust
const RHF_REQUICKSTART: u32 = 1_024u32;
```

### `RHF_REQUICKSTARTED`

```rust
const RHF_REQUICKSTARTED: u32 = 2_048u32;
```

### `RHF_CORD`

```rust
const RHF_CORD: u32 = 4_096u32;
```

### `RHF_NO_UNRES_UNDEF`

```rust
const RHF_NO_UNRES_UNDEF: u32 = 8_192u32;
```

### `RHF_RLD_ORDER_SAFE`

```rust
const RHF_RLD_ORDER_SAFE: u32 = 16_384u32;
```

### `LL_NONE`

```rust
const LL_NONE: u32 = 0u32;
```

### `LL_EXACT_MATCH`

```rust
const LL_EXACT_MATCH: u32 = 1u32;
```

Require exact match

### `LL_IGNORE_INT_VER`

```rust
const LL_IGNORE_INT_VER: u32 = 2u32;
```

Ignore interface version

### `LL_REQUIRE_MINOR`

```rust
const LL_REQUIRE_MINOR: u32 = 4u32;
```

### `LL_EXPORTS`

```rust
const LL_EXPORTS: u32 = 8u32;
```

### `LL_DELAY_LOAD`

```rust
const LL_DELAY_LOAD: u32 = 16u32;
```

### `LL_DELTA`

```rust
const LL_DELTA: u32 = 32u32;
```

### `EF_PARISC_TRAPNIL`

```rust
const EF_PARISC_TRAPNIL: u32 = 65_536u32;
```

Trap nil pointer dereference.

### `EF_PARISC_EXT`

```rust
const EF_PARISC_EXT: u32 = 131_072u32;
```

Program uses arch. extensions.

### `EF_PARISC_LSB`

```rust
const EF_PARISC_LSB: u32 = 262_144u32;
```

Program expects little endian.

### `EF_PARISC_WIDE`

```rust
const EF_PARISC_WIDE: u32 = 524_288u32;
```

Program expects wide mode.

### `EF_PARISC_NO_KABP`

```rust
const EF_PARISC_NO_KABP: u32 = 1_048_576u32;
```

No kernel assisted branch prediction.

### `EF_PARISC_LAZYSWAP`

```rust
const EF_PARISC_LAZYSWAP: u32 = 4_194_304u32;
```

Allow lazy swapping.

### `EF_PARISC_ARCH`

```rust
const EF_PARISC_ARCH: u32 = 65_535u32;
```

Architecture version.

### `EFA_PARISC_1_0`

```rust
const EFA_PARISC_1_0: u32 = 523u32;
```

PA-RISC 1.0 big-endian.

### `EFA_PARISC_1_1`

```rust
const EFA_PARISC_1_1: u32 = 528u32;
```

PA-RISC 1.1 big-endian.

### `EFA_PARISC_2_0`

```rust
const EFA_PARISC_2_0: u32 = 532u32;
```

PA-RISC 2.0 big-endian.

### `SHN_PARISC_ANSI_COMMON`

```rust
const SHN_PARISC_ANSI_COMMON: u16 = 65_280u16;
```

Section for tentatively declared symbols in ANSI C.

### `SHN_PARISC_HUGE_COMMON`

```rust
const SHN_PARISC_HUGE_COMMON: u16 = 65_281u16;
```

Common blocks in huge model.

### `SHT_PARISC_EXT`

```rust
const SHT_PARISC_EXT: u32 = 1_879_048_192u32;
```

Contains product specific ext.

### `SHT_PARISC_UNWIND`

```rust
const SHT_PARISC_UNWIND: u32 = 1_879_048_193u32;
```

Unwind information.

### `SHT_PARISC_DOC`

```rust
const SHT_PARISC_DOC: u32 = 1_879_048_194u32;
```

Debug info for optimized code.

### `SHF_PARISC_SHORT`

```rust
const SHF_PARISC_SHORT: u32 = 536_870_912u32;
```

Section with short addressing.

### `SHF_PARISC_HUGE`

```rust
const SHF_PARISC_HUGE: u32 = 1_073_741_824u32;
```

Section far from gp.

### `SHF_PARISC_SBP`

```rust
const SHF_PARISC_SBP: u32 = 2_147_483_648u32;
```

Static branch prediction code.

### `STT_PARISC_MILLICODE`

```rust
const STT_PARISC_MILLICODE: u8 = 13u8;
```

Millicode function entry point.

### `STT_HP_OPAQUE`

```rust
const STT_HP_OPAQUE: u8 = 11u8;
```

### `STT_HP_STUB`

```rust
const STT_HP_STUB: u8 = 12u8;
```

### `R_PARISC_NONE`

```rust
const R_PARISC_NONE: u32 = 0u32;
```

No reloc.

### `R_PARISC_DIR32`

```rust
const R_PARISC_DIR32: u32 = 1u32;
```

Direct 32-bit reference.

### `R_PARISC_DIR21L`

```rust
const R_PARISC_DIR21L: u32 = 2u32;
```

Left 21 bits of eff. address.

### `R_PARISC_DIR17R`

```rust
const R_PARISC_DIR17R: u32 = 3u32;
```

Right 17 bits of eff. address.

### `R_PARISC_DIR17F`

```rust
const R_PARISC_DIR17F: u32 = 4u32;
```

17 bits of eff. address.

### `R_PARISC_DIR14R`

```rust
const R_PARISC_DIR14R: u32 = 6u32;
```

Right 14 bits of eff. address.

### `R_PARISC_PCREL32`

```rust
const R_PARISC_PCREL32: u32 = 9u32;
```

32-bit rel. address.

### `R_PARISC_PCREL21L`

```rust
const R_PARISC_PCREL21L: u32 = 10u32;
```

Left 21 bits of rel. address.

### `R_PARISC_PCREL17R`

```rust
const R_PARISC_PCREL17R: u32 = 11u32;
```

Right 17 bits of rel. address.

### `R_PARISC_PCREL17F`

```rust
const R_PARISC_PCREL17F: u32 = 12u32;
```

17 bits of rel. address.

### `R_PARISC_PCREL14R`

```rust
const R_PARISC_PCREL14R: u32 = 14u32;
```

Right 14 bits of rel. address.

### `R_PARISC_DPREL21L`

```rust
const R_PARISC_DPREL21L: u32 = 18u32;
```

Left 21 bits of rel. address.

### `R_PARISC_DPREL14R`

```rust
const R_PARISC_DPREL14R: u32 = 22u32;
```

Right 14 bits of rel. address.

### `R_PARISC_GPREL21L`

```rust
const R_PARISC_GPREL21L: u32 = 26u32;
```

GP-relative, left 21 bits.

### `R_PARISC_GPREL14R`

```rust
const R_PARISC_GPREL14R: u32 = 30u32;
```

GP-relative, right 14 bits.

### `R_PARISC_LTOFF21L`

```rust
const R_PARISC_LTOFF21L: u32 = 34u32;
```

LT-relative, left 21 bits.

### `R_PARISC_LTOFF14R`

```rust
const R_PARISC_LTOFF14R: u32 = 38u32;
```

LT-relative, right 14 bits.

### `R_PARISC_SECREL32`

```rust
const R_PARISC_SECREL32: u32 = 41u32;
```

32 bits section rel. address.

### `R_PARISC_SEGBASE`

```rust
const R_PARISC_SEGBASE: u32 = 48u32;
```

No relocation, set segment base.

### `R_PARISC_SEGREL32`

```rust
const R_PARISC_SEGREL32: u32 = 49u32;
```

32 bits segment rel. address.

### `R_PARISC_PLTOFF21L`

```rust
const R_PARISC_PLTOFF21L: u32 = 50u32;
```

PLT rel. address, left 21 bits.

### `R_PARISC_PLTOFF14R`

```rust
const R_PARISC_PLTOFF14R: u32 = 54u32;
```

PLT rel. address, right 14 bits.

### `R_PARISC_LTOFF_FPTR32`

```rust
const R_PARISC_LTOFF_FPTR32: u32 = 57u32;
```

32 bits LT-rel. function pointer.

### `R_PARISC_LTOFF_FPTR21L`

```rust
const R_PARISC_LTOFF_FPTR21L: u32 = 58u32;
```

LT-rel. fct ptr, left 21 bits.

### `R_PARISC_LTOFF_FPTR14R`

```rust
const R_PARISC_LTOFF_FPTR14R: u32 = 62u32;
```

LT-rel. fct ptr, right 14 bits.

### `R_PARISC_FPTR64`

```rust
const R_PARISC_FPTR64: u32 = 64u32;
```

64 bits function address.

### `R_PARISC_PLABEL32`

```rust
const R_PARISC_PLABEL32: u32 = 65u32;
```

32 bits function address.

### `R_PARISC_PLABEL21L`

```rust
const R_PARISC_PLABEL21L: u32 = 66u32;
```

Left 21 bits of fdesc address.

### `R_PARISC_PLABEL14R`

```rust
const R_PARISC_PLABEL14R: u32 = 70u32;
```

Right 14 bits of fdesc address.

### `R_PARISC_PCREL64`

```rust
const R_PARISC_PCREL64: u32 = 72u32;
```

64 bits PC-rel. address.

### `R_PARISC_PCREL22F`

```rust
const R_PARISC_PCREL22F: u32 = 74u32;
```

22 bits PC-rel. address.

### `R_PARISC_PCREL14WR`

```rust
const R_PARISC_PCREL14WR: u32 = 75u32;
```

PC-rel. address, right 14 bits.

### `R_PARISC_PCREL14DR`

```rust
const R_PARISC_PCREL14DR: u32 = 76u32;
```

PC rel. address, right 14 bits.

### `R_PARISC_PCREL16F`

```rust
const R_PARISC_PCREL16F: u32 = 77u32;
```

16 bits PC-rel. address.

### `R_PARISC_PCREL16WF`

```rust
const R_PARISC_PCREL16WF: u32 = 78u32;
```

16 bits PC-rel. address.

### `R_PARISC_PCREL16DF`

```rust
const R_PARISC_PCREL16DF: u32 = 79u32;
```

16 bits PC-rel. address.

### `R_PARISC_DIR64`

```rust
const R_PARISC_DIR64: u32 = 80u32;
```

64 bits of eff. address.

### `R_PARISC_DIR14WR`

```rust
const R_PARISC_DIR14WR: u32 = 83u32;
```

14 bits of eff. address.

### `R_PARISC_DIR14DR`

```rust
const R_PARISC_DIR14DR: u32 = 84u32;
```

14 bits of eff. address.

### `R_PARISC_DIR16F`

```rust
const R_PARISC_DIR16F: u32 = 85u32;
```

16 bits of eff. address.

### `R_PARISC_DIR16WF`

```rust
const R_PARISC_DIR16WF: u32 = 86u32;
```

16 bits of eff. address.

### `R_PARISC_DIR16DF`

```rust
const R_PARISC_DIR16DF: u32 = 87u32;
```

16 bits of eff. address.

### `R_PARISC_GPREL64`

```rust
const R_PARISC_GPREL64: u32 = 88u32;
```

64 bits of GP-rel. address.

### `R_PARISC_GPREL14WR`

```rust
const R_PARISC_GPREL14WR: u32 = 91u32;
```

GP-rel. address, right 14 bits.

### `R_PARISC_GPREL14DR`

```rust
const R_PARISC_GPREL14DR: u32 = 92u32;
```

GP-rel. address, right 14 bits.

### `R_PARISC_GPREL16F`

```rust
const R_PARISC_GPREL16F: u32 = 93u32;
```

16 bits GP-rel. address.

### `R_PARISC_GPREL16WF`

```rust
const R_PARISC_GPREL16WF: u32 = 94u32;
```

16 bits GP-rel. address.

### `R_PARISC_GPREL16DF`

```rust
const R_PARISC_GPREL16DF: u32 = 95u32;
```

16 bits GP-rel. address.

### `R_PARISC_LTOFF64`

```rust
const R_PARISC_LTOFF64: u32 = 96u32;
```

64 bits LT-rel. address.

### `R_PARISC_LTOFF14WR`

```rust
const R_PARISC_LTOFF14WR: u32 = 99u32;
```

LT-rel. address, right 14 bits.

### `R_PARISC_LTOFF14DR`

```rust
const R_PARISC_LTOFF14DR: u32 = 100u32;
```

LT-rel. address, right 14 bits.

### `R_PARISC_LTOFF16F`

```rust
const R_PARISC_LTOFF16F: u32 = 101u32;
```

16 bits LT-rel. address.

### `R_PARISC_LTOFF16WF`

```rust
const R_PARISC_LTOFF16WF: u32 = 102u32;
```

16 bits LT-rel. address.

### `R_PARISC_LTOFF16DF`

```rust
const R_PARISC_LTOFF16DF: u32 = 103u32;
```

16 bits LT-rel. address.

### `R_PARISC_SECREL64`

```rust
const R_PARISC_SECREL64: u32 = 104u32;
```

64 bits section rel. address.

### `R_PARISC_SEGREL64`

```rust
const R_PARISC_SEGREL64: u32 = 112u32;
```

64 bits segment rel. address.

### `R_PARISC_PLTOFF14WR`

```rust
const R_PARISC_PLTOFF14WR: u32 = 115u32;
```

PLT-rel. address, right 14 bits.

### `R_PARISC_PLTOFF14DR`

```rust
const R_PARISC_PLTOFF14DR: u32 = 116u32;
```

PLT-rel. address, right 14 bits.

### `R_PARISC_PLTOFF16F`

```rust
const R_PARISC_PLTOFF16F: u32 = 117u32;
```

16 bits LT-rel. address.

### `R_PARISC_PLTOFF16WF`

```rust
const R_PARISC_PLTOFF16WF: u32 = 118u32;
```

16 bits PLT-rel. address.

### `R_PARISC_PLTOFF16DF`

```rust
const R_PARISC_PLTOFF16DF: u32 = 119u32;
```

16 bits PLT-rel. address.

### `R_PARISC_LTOFF_FPTR64`

```rust
const R_PARISC_LTOFF_FPTR64: u32 = 120u32;
```

64 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR14WR`

```rust
const R_PARISC_LTOFF_FPTR14WR: u32 = 123u32;
```

LT-rel. fct. ptr., right 14 bits.

### `R_PARISC_LTOFF_FPTR14DR`

```rust
const R_PARISC_LTOFF_FPTR14DR: u32 = 124u32;
```

LT-rel. fct. ptr., right 14 bits.

### `R_PARISC_LTOFF_FPTR16F`

```rust
const R_PARISC_LTOFF_FPTR16F: u32 = 125u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR16WF`

```rust
const R_PARISC_LTOFF_FPTR16WF: u32 = 126u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR16DF`

```rust
const R_PARISC_LTOFF_FPTR16DF: u32 = 127u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LORESERVE`

```rust
const R_PARISC_LORESERVE: u32 = 128u32;
```

### `R_PARISC_COPY`

```rust
const R_PARISC_COPY: u32 = 128u32;
```

Copy relocation.

### `R_PARISC_IPLT`

```rust
const R_PARISC_IPLT: u32 = 129u32;
```

Dynamic reloc, imported PLT

### `R_PARISC_EPLT`

```rust
const R_PARISC_EPLT: u32 = 130u32;
```

Dynamic reloc, exported PLT

### `R_PARISC_TPREL32`

```rust
const R_PARISC_TPREL32: u32 = 153u32;
```

32 bits TP-rel. address.

### `R_PARISC_TPREL21L`

```rust
const R_PARISC_TPREL21L: u32 = 154u32;
```

TP-rel. address, left 21 bits.

### `R_PARISC_TPREL14R`

```rust
const R_PARISC_TPREL14R: u32 = 158u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP21L`

```rust
const R_PARISC_LTOFF_TP21L: u32 = 162u32;
```

LT-TP-rel. address, left 21 bits.

### `R_PARISC_LTOFF_TP14R`

```rust
const R_PARISC_LTOFF_TP14R: u32 = 166u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP14F`

```rust
const R_PARISC_LTOFF_TP14F: u32 = 167u32;
```

14 bits LT-TP-rel. address.

### `R_PARISC_TPREL64`

```rust
const R_PARISC_TPREL64: u32 = 216u32;
```

64 bits TP-rel. address.

### `R_PARISC_TPREL14WR`

```rust
const R_PARISC_TPREL14WR: u32 = 219u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_TPREL14DR`

```rust
const R_PARISC_TPREL14DR: u32 = 220u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_TPREL16F`

```rust
const R_PARISC_TPREL16F: u32 = 221u32;
```

16 bits TP-rel. address.

### `R_PARISC_TPREL16WF`

```rust
const R_PARISC_TPREL16WF: u32 = 222u32;
```

16 bits TP-rel. address.

### `R_PARISC_TPREL16DF`

```rust
const R_PARISC_TPREL16DF: u32 = 223u32;
```

16 bits TP-rel. address.

### `R_PARISC_LTOFF_TP64`

```rust
const R_PARISC_LTOFF_TP64: u32 = 224u32;
```

64 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP14WR`

```rust
const R_PARISC_LTOFF_TP14WR: u32 = 227u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP14DR`

```rust
const R_PARISC_LTOFF_TP14DR: u32 = 228u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP16F`

```rust
const R_PARISC_LTOFF_TP16F: u32 = 229u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP16WF`

```rust
const R_PARISC_LTOFF_TP16WF: u32 = 230u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP16DF`

```rust
const R_PARISC_LTOFF_TP16DF: u32 = 231u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_GNU_VTENTRY`

```rust
const R_PARISC_GNU_VTENTRY: u32 = 232u32;
```

### `R_PARISC_GNU_VTINHERIT`

```rust
const R_PARISC_GNU_VTINHERIT: u32 = 233u32;
```

### `R_PARISC_TLS_GD21L`

```rust
const R_PARISC_TLS_GD21L: u32 = 234u32;
```

GD 21-bit left.

### `R_PARISC_TLS_GD14R`

```rust
const R_PARISC_TLS_GD14R: u32 = 235u32;
```

GD 14-bit right.

### `R_PARISC_TLS_GDCALL`

```rust
const R_PARISC_TLS_GDCALL: u32 = 236u32;
```

GD call to __t_g_a.

### `R_PARISC_TLS_LDM21L`

```rust
const R_PARISC_TLS_LDM21L: u32 = 237u32;
```

LD module 21-bit left.

### `R_PARISC_TLS_LDM14R`

```rust
const R_PARISC_TLS_LDM14R: u32 = 238u32;
```

LD module 14-bit right.

### `R_PARISC_TLS_LDMCALL`

```rust
const R_PARISC_TLS_LDMCALL: u32 = 239u32;
```

LD module call to __t_g_a.

### `R_PARISC_TLS_LDO21L`

```rust
const R_PARISC_TLS_LDO21L: u32 = 240u32;
```

LD offset 21-bit left.

### `R_PARISC_TLS_LDO14R`

```rust
const R_PARISC_TLS_LDO14R: u32 = 241u32;
```

LD offset 14-bit right.

### `R_PARISC_TLS_DTPMOD32`

```rust
const R_PARISC_TLS_DTPMOD32: u32 = 242u32;
```

DTP module 32-bit.

### `R_PARISC_TLS_DTPMOD64`

```rust
const R_PARISC_TLS_DTPMOD64: u32 = 243u32;
```

DTP module 64-bit.

### `R_PARISC_TLS_DTPOFF32`

```rust
const R_PARISC_TLS_DTPOFF32: u32 = 244u32;
```

DTP offset 32-bit.

### `R_PARISC_TLS_DTPOFF64`

```rust
const R_PARISC_TLS_DTPOFF64: u32 = 245u32;
```

DTP offset 32-bit.

### `R_PARISC_TLS_LE21L`

```rust
const R_PARISC_TLS_LE21L: u32 = 154u32;
```

### `R_PARISC_TLS_LE14R`

```rust
const R_PARISC_TLS_LE14R: u32 = 158u32;
```

### `R_PARISC_TLS_IE21L`

```rust
const R_PARISC_TLS_IE21L: u32 = 162u32;
```

### `R_PARISC_TLS_IE14R`

```rust
const R_PARISC_TLS_IE14R: u32 = 166u32;
```

### `R_PARISC_TLS_TPREL32`

```rust
const R_PARISC_TLS_TPREL32: u32 = 153u32;
```

### `R_PARISC_TLS_TPREL64`

```rust
const R_PARISC_TLS_TPREL64: u32 = 216u32;
```

### `R_PARISC_HIRESERVE`

```rust
const R_PARISC_HIRESERVE: u32 = 255u32;
```

### `PT_HP_TLS`

```rust
const PT_HP_TLS: u32 = 1_610_612_736u32;
```

### `PT_HP_CORE_NONE`

```rust
const PT_HP_CORE_NONE: u32 = 1_610_612_737u32;
```

### `PT_HP_CORE_VERSION`

```rust
const PT_HP_CORE_VERSION: u32 = 1_610_612_738u32;
```

### `PT_HP_CORE_KERNEL`

```rust
const PT_HP_CORE_KERNEL: u32 = 1_610_612_739u32;
```

### `PT_HP_CORE_COMM`

```rust
const PT_HP_CORE_COMM: u32 = 1_610_612_740u32;
```

### `PT_HP_CORE_PROC`

```rust
const PT_HP_CORE_PROC: u32 = 1_610_612_741u32;
```

### `PT_HP_CORE_LOADABLE`

```rust
const PT_HP_CORE_LOADABLE: u32 = 1_610_612_742u32;
```

### `PT_HP_CORE_STACK`

```rust
const PT_HP_CORE_STACK: u32 = 1_610_612_743u32;
```

### `PT_HP_CORE_SHM`

```rust
const PT_HP_CORE_SHM: u32 = 1_610_612_744u32;
```

### `PT_HP_CORE_MMF`

```rust
const PT_HP_CORE_MMF: u32 = 1_610_612_745u32;
```

### `PT_HP_PARALLEL`

```rust
const PT_HP_PARALLEL: u32 = 1_610_612_752u32;
```

### `PT_HP_FASTBIND`

```rust
const PT_HP_FASTBIND: u32 = 1_610_612_753u32;
```

### `PT_HP_OPT_ANNOT`

```rust
const PT_HP_OPT_ANNOT: u32 = 1_610_612_754u32;
```

### `PT_HP_HSL_ANNOT`

```rust
const PT_HP_HSL_ANNOT: u32 = 1_610_612_755u32;
```

### `PT_HP_STACK`

```rust
const PT_HP_STACK: u32 = 1_610_612_756u32;
```

### `PT_PARISC_ARCHEXT`

```rust
const PT_PARISC_ARCHEXT: u32 = 1_879_048_192u32;
```

### `PT_PARISC_UNWIND`

```rust
const PT_PARISC_UNWIND: u32 = 1_879_048_193u32;
```

### `PF_PARISC_SBP`

```rust
const PF_PARISC_SBP: u32 = 134_217_728u32;
```

### `PF_HP_PAGE_SIZE`

```rust
const PF_HP_PAGE_SIZE: u32 = 1_048_576u32;
```

### `PF_HP_FAR_SHARED`

```rust
const PF_HP_FAR_SHARED: u32 = 2_097_152u32;
```

### `PF_HP_NEAR_SHARED`

```rust
const PF_HP_NEAR_SHARED: u32 = 4_194_304u32;
```

### `PF_HP_CODE`

```rust
const PF_HP_CODE: u32 = 16_777_216u32;
```

### `PF_HP_MODIFY`

```rust
const PF_HP_MODIFY: u32 = 33_554_432u32;
```

### `PF_HP_LAZYSWAP`

```rust
const PF_HP_LAZYSWAP: u32 = 67_108_864u32;
```

### `PF_HP_SBP`

```rust
const PF_HP_SBP: u32 = 134_217_728u32;
```

### `EF_ALPHA_32BIT`

```rust
const EF_ALPHA_32BIT: u32 = 1u32;
```

All addresses must be < 2GB.

### `EF_ALPHA_CANRELAX`

```rust
const EF_ALPHA_CANRELAX: u32 = 2u32;
```

Relocations for relaxing exist.

### `SHT_ALPHA_DEBUG`

```rust
const SHT_ALPHA_DEBUG: u32 = 1_879_048_193u32;
```

### `SHT_ALPHA_REGINFO`

```rust
const SHT_ALPHA_REGINFO: u32 = 1_879_048_194u32;
```

### `SHF_ALPHA_GPREL`

```rust
const SHF_ALPHA_GPREL: u32 = 268_435_456u32;
```

### `STO_ALPHA_NOPV`

```rust
const STO_ALPHA_NOPV: u8 = 128u8;
```

No PV required.

### `STO_ALPHA_STD_GPLOAD`

```rust
const STO_ALPHA_STD_GPLOAD: u8 = 136u8;
```

PV only used for initial ldgp.

### `R_ALPHA_NONE`

```rust
const R_ALPHA_NONE: u32 = 0u32;
```

No reloc

### `R_ALPHA_REFLONG`

```rust
const R_ALPHA_REFLONG: u32 = 1u32;
```

Direct 32 bit

### `R_ALPHA_REFQUAD`

```rust
const R_ALPHA_REFQUAD: u32 = 2u32;
```

Direct 64 bit

### `R_ALPHA_GPREL32`

```rust
const R_ALPHA_GPREL32: u32 = 3u32;
```

GP relative 32 bit

### `R_ALPHA_LITERAL`

```rust
const R_ALPHA_LITERAL: u32 = 4u32;
```

GP relative 16 bit w/optimization

### `R_ALPHA_LITUSE`

```rust
const R_ALPHA_LITUSE: u32 = 5u32;
```

Optimization hint for LITERAL

### `R_ALPHA_GPDISP`

```rust
const R_ALPHA_GPDISP: u32 = 6u32;
```

Add displacement to GP

### `R_ALPHA_BRADDR`

```rust
const R_ALPHA_BRADDR: u32 = 7u32;
```

PC+4 relative 23 bit shifted

### `R_ALPHA_HINT`

```rust
const R_ALPHA_HINT: u32 = 8u32;
```

PC+4 relative 16 bit shifted

### `R_ALPHA_SREL16`

```rust
const R_ALPHA_SREL16: u32 = 9u32;
```

PC relative 16 bit

### `R_ALPHA_SREL32`

```rust
const R_ALPHA_SREL32: u32 = 10u32;
```

PC relative 32 bit

### `R_ALPHA_SREL64`

```rust
const R_ALPHA_SREL64: u32 = 11u32;
```

PC relative 64 bit

### `R_ALPHA_GPRELHIGH`

```rust
const R_ALPHA_GPRELHIGH: u32 = 17u32;
```

GP relative 32 bit, high 16 bits

### `R_ALPHA_GPRELLOW`

```rust
const R_ALPHA_GPRELLOW: u32 = 18u32;
```

GP relative 32 bit, low 16 bits

### `R_ALPHA_GPREL16`

```rust
const R_ALPHA_GPREL16: u32 = 19u32;
```

GP relative 16 bit

### `R_ALPHA_COPY`

```rust
const R_ALPHA_COPY: u32 = 24u32;
```

Copy symbol at runtime

### `R_ALPHA_GLOB_DAT`

```rust
const R_ALPHA_GLOB_DAT: u32 = 25u32;
```

Create GOT entry

### `R_ALPHA_JMP_SLOT`

```rust
const R_ALPHA_JMP_SLOT: u32 = 26u32;
```

Create PLT entry

### `R_ALPHA_RELATIVE`

```rust
const R_ALPHA_RELATIVE: u32 = 27u32;
```

Adjust by program base

### `R_ALPHA_TLS_GD_HI`

```rust
const R_ALPHA_TLS_GD_HI: u32 = 28u32;
```

### `R_ALPHA_TLSGD`

```rust
const R_ALPHA_TLSGD: u32 = 29u32;
```

### `R_ALPHA_TLS_LDM`

```rust
const R_ALPHA_TLS_LDM: u32 = 30u32;
```

### `R_ALPHA_DTPMOD64`

```rust
const R_ALPHA_DTPMOD64: u32 = 31u32;
```

### `R_ALPHA_GOTDTPREL`

```rust
const R_ALPHA_GOTDTPREL: u32 = 32u32;
```

### `R_ALPHA_DTPREL64`

```rust
const R_ALPHA_DTPREL64: u32 = 33u32;
```

### `R_ALPHA_DTPRELHI`

```rust
const R_ALPHA_DTPRELHI: u32 = 34u32;
```

### `R_ALPHA_DTPRELLO`

```rust
const R_ALPHA_DTPRELLO: u32 = 35u32;
```

### `R_ALPHA_DTPREL16`

```rust
const R_ALPHA_DTPREL16: u32 = 36u32;
```

### `R_ALPHA_GOTTPREL`

```rust
const R_ALPHA_GOTTPREL: u32 = 37u32;
```

### `R_ALPHA_TPREL64`

```rust
const R_ALPHA_TPREL64: u32 = 38u32;
```

### `R_ALPHA_TPRELHI`

```rust
const R_ALPHA_TPRELHI: u32 = 39u32;
```

### `R_ALPHA_TPRELLO`

```rust
const R_ALPHA_TPRELLO: u32 = 40u32;
```

### `R_ALPHA_TPREL16`

```rust
const R_ALPHA_TPREL16: u32 = 41u32;
```

### `LITUSE_ALPHA_ADDR`

```rust
const LITUSE_ALPHA_ADDR: u32 = 0u32;
```

### `LITUSE_ALPHA_BASE`

```rust
const LITUSE_ALPHA_BASE: u32 = 1u32;
```

### `LITUSE_ALPHA_BYTOFF`

```rust
const LITUSE_ALPHA_BYTOFF: u32 = 2u32;
```

### `LITUSE_ALPHA_JSR`

```rust
const LITUSE_ALPHA_JSR: u32 = 3u32;
```

### `LITUSE_ALPHA_TLS_GD`

```rust
const LITUSE_ALPHA_TLS_GD: u32 = 4u32;
```

### `LITUSE_ALPHA_TLS_LDM`

```rust
const LITUSE_ALPHA_TLS_LDM: u32 = 5u32;
```

### `DT_ALPHA_PLTRO`

```rust
const DT_ALPHA_PLTRO: u32 = 1_879_048_192u32;
```

### `EF_PPC_EMB`

```rust
const EF_PPC_EMB: u32 = 2_147_483_648u32;
```

PowerPC embedded flag

### `EF_PPC_RELOCATABLE`

```rust
const EF_PPC_RELOCATABLE: u32 = 65_536u32;
```

PowerPC -mrelocatable flag

### `EF_PPC_RELOCATABLE_LIB`

```rust
const EF_PPC_RELOCATABLE_LIB: u32 = 32_768u32;
```

PowerPC -mrelocatable-lib flag

### `R_PPC_NONE`

```rust
const R_PPC_NONE: u32 = 0u32;
```

### `R_PPC_ADDR32`

```rust
const R_PPC_ADDR32: u32 = 1u32;
```

32bit absolute address

### `R_PPC_ADDR24`

```rust
const R_PPC_ADDR24: u32 = 2u32;
```

26bit address, 2 bits ignored.

### `R_PPC_ADDR16`

```rust
const R_PPC_ADDR16: u32 = 3u32;
```

16bit absolute address

### `R_PPC_ADDR16_LO`

```rust
const R_PPC_ADDR16_LO: u32 = 4u32;
```

lower 16bit of absolute address

### `R_PPC_ADDR16_HI`

```rust
const R_PPC_ADDR16_HI: u32 = 5u32;
```

high 16bit of absolute address

### `R_PPC_ADDR16_HA`

```rust
const R_PPC_ADDR16_HA: u32 = 6u32;
```

adjusted high 16bit

### `R_PPC_ADDR14`

```rust
const R_PPC_ADDR14: u32 = 7u32;
```

16bit address, 2 bits ignored

### `R_PPC_ADDR14_BRTAKEN`

```rust
const R_PPC_ADDR14_BRTAKEN: u32 = 8u32;
```

### `R_PPC_ADDR14_BRNTAKEN`

```rust
const R_PPC_ADDR14_BRNTAKEN: u32 = 9u32;
```

### `R_PPC_REL24`

```rust
const R_PPC_REL24: u32 = 10u32;
```

PC relative 26 bit

### `R_PPC_REL14`

```rust
const R_PPC_REL14: u32 = 11u32;
```

PC relative 16 bit

### `R_PPC_REL14_BRTAKEN`

```rust
const R_PPC_REL14_BRTAKEN: u32 = 12u32;
```

### `R_PPC_REL14_BRNTAKEN`

```rust
const R_PPC_REL14_BRNTAKEN: u32 = 13u32;
```

### `R_PPC_GOT16`

```rust
const R_PPC_GOT16: u32 = 14u32;
```

### `R_PPC_GOT16_LO`

```rust
const R_PPC_GOT16_LO: u32 = 15u32;
```

### `R_PPC_GOT16_HI`

```rust
const R_PPC_GOT16_HI: u32 = 16u32;
```

### `R_PPC_GOT16_HA`

```rust
const R_PPC_GOT16_HA: u32 = 17u32;
```

### `R_PPC_PLTREL24`

```rust
const R_PPC_PLTREL24: u32 = 18u32;
```

### `R_PPC_COPY`

```rust
const R_PPC_COPY: u32 = 19u32;
```

### `R_PPC_GLOB_DAT`

```rust
const R_PPC_GLOB_DAT: u32 = 20u32;
```

### `R_PPC_JMP_SLOT`

```rust
const R_PPC_JMP_SLOT: u32 = 21u32;
```

### `R_PPC_RELATIVE`

```rust
const R_PPC_RELATIVE: u32 = 22u32;
```

### `R_PPC_LOCAL24PC`

```rust
const R_PPC_LOCAL24PC: u32 = 23u32;
```

### `R_PPC_UADDR32`

```rust
const R_PPC_UADDR32: u32 = 24u32;
```

### `R_PPC_UADDR16`

```rust
const R_PPC_UADDR16: u32 = 25u32;
```

### `R_PPC_REL32`

```rust
const R_PPC_REL32: u32 = 26u32;
```

### `R_PPC_PLT32`

```rust
const R_PPC_PLT32: u32 = 27u32;
```

### `R_PPC_PLTREL32`

```rust
const R_PPC_PLTREL32: u32 = 28u32;
```

### `R_PPC_PLT16_LO`

```rust
const R_PPC_PLT16_LO: u32 = 29u32;
```

### `R_PPC_PLT16_HI`

```rust
const R_PPC_PLT16_HI: u32 = 30u32;
```

### `R_PPC_PLT16_HA`

```rust
const R_PPC_PLT16_HA: u32 = 31u32;
```

### `R_PPC_SDAREL16`

```rust
const R_PPC_SDAREL16: u32 = 32u32;
```

### `R_PPC_SECTOFF`

```rust
const R_PPC_SECTOFF: u32 = 33u32;
```

### `R_PPC_SECTOFF_LO`

```rust
const R_PPC_SECTOFF_LO: u32 = 34u32;
```

### `R_PPC_SECTOFF_HI`

```rust
const R_PPC_SECTOFF_HI: u32 = 35u32;
```

### `R_PPC_SECTOFF_HA`

```rust
const R_PPC_SECTOFF_HA: u32 = 36u32;
```

### `R_PPC_TLS`

```rust
const R_PPC_TLS: u32 = 67u32;
```

none    (sym+add)@tls

### `R_PPC_DTPMOD32`

```rust
const R_PPC_DTPMOD32: u32 = 68u32;
```

word32  (sym+add)@dtpmod

### `R_PPC_TPREL16`

```rust
const R_PPC_TPREL16: u32 = 69u32;
```

half16* (sym+add)@tprel

### `R_PPC_TPREL16_LO`

```rust
const R_PPC_TPREL16_LO: u32 = 70u32;
```

half16  (sym+add)@tprel@l

### `R_PPC_TPREL16_HI`

```rust
const R_PPC_TPREL16_HI: u32 = 71u32;
```

half16  (sym+add)@tprel@h

### `R_PPC_TPREL16_HA`

```rust
const R_PPC_TPREL16_HA: u32 = 72u32;
```

half16  (sym+add)@tprel@ha

### `R_PPC_TPREL32`

```rust
const R_PPC_TPREL32: u32 = 73u32;
```

word32  (sym+add)@tprel

### `R_PPC_DTPREL16`

```rust
const R_PPC_DTPREL16: u32 = 74u32;
```

half16*(sym+add)@dtprel

### `R_PPC_DTPREL16_LO`

```rust
const R_PPC_DTPREL16_LO: u32 = 75u32;
```

half16  (sym+add)@dtprel@l

### `R_PPC_DTPREL16_HI`

```rust
const R_PPC_DTPREL16_HI: u32 = 76u32;
```

half16  (sym+add)@dtprel@h

### `R_PPC_DTPREL16_HA`

```rust
const R_PPC_DTPREL16_HA: u32 = 77u32;
```

half16  (sym+add)@dtprel@ha

### `R_PPC_DTPREL32`

```rust
const R_PPC_DTPREL32: u32 = 78u32;
```

word32  (sym+add)@dtprel

### `R_PPC_GOT_TLSGD16`

```rust
const R_PPC_GOT_TLSGD16: u32 = 79u32;
```

half16* (sym+add)@got@tlsgd

### `R_PPC_GOT_TLSGD16_LO`

```rust
const R_PPC_GOT_TLSGD16_LO: u32 = 80u32;
```

half16  (sym+add)@got@tlsgd@l

### `R_PPC_GOT_TLSGD16_HI`

```rust
const R_PPC_GOT_TLSGD16_HI: u32 = 81u32;
```

half16  (sym+add)@got@tlsgd@h

### `R_PPC_GOT_TLSGD16_HA`

```rust
const R_PPC_GOT_TLSGD16_HA: u32 = 82u32;
```

half16  (sym+add)@got@tlsgd@ha

### `R_PPC_GOT_TLSLD16`

```rust
const R_PPC_GOT_TLSLD16: u32 = 83u32;
```

half16* (sym+add)@got@tlsld

### `R_PPC_GOT_TLSLD16_LO`

```rust
const R_PPC_GOT_TLSLD16_LO: u32 = 84u32;
```

half16  (sym+add)@got@tlsld@l

### `R_PPC_GOT_TLSLD16_HI`

```rust
const R_PPC_GOT_TLSLD16_HI: u32 = 85u32;
```

half16  (sym+add)@got@tlsld@h

### `R_PPC_GOT_TLSLD16_HA`

```rust
const R_PPC_GOT_TLSLD16_HA: u32 = 86u32;
```

half16  (sym+add)@got@tlsld@ha

### `R_PPC_GOT_TPREL16`

```rust
const R_PPC_GOT_TPREL16: u32 = 87u32;
```

half16* (sym+add)@got@tprel

### `R_PPC_GOT_TPREL16_LO`

```rust
const R_PPC_GOT_TPREL16_LO: u32 = 88u32;
```

half16  (sym+add)@got@tprel@l

### `R_PPC_GOT_TPREL16_HI`

```rust
const R_PPC_GOT_TPREL16_HI: u32 = 89u32;
```

half16  (sym+add)@got@tprel@h

### `R_PPC_GOT_TPREL16_HA`

```rust
const R_PPC_GOT_TPREL16_HA: u32 = 90u32;
```

half16  (sym+add)@got@tprel@ha

### `R_PPC_GOT_DTPREL16`

```rust
const R_PPC_GOT_DTPREL16: u32 = 91u32;
```

half16* (sym+add)@got@dtprel

### `R_PPC_GOT_DTPREL16_LO`

```rust
const R_PPC_GOT_DTPREL16_LO: u32 = 92u32;
```

half16* (sym+add)@got@dtprel@l

### `R_PPC_GOT_DTPREL16_HI`

```rust
const R_PPC_GOT_DTPREL16_HI: u32 = 93u32;
```

half16* (sym+add)@got@dtprel@h

### `R_PPC_GOT_DTPREL16_HA`

```rust
const R_PPC_GOT_DTPREL16_HA: u32 = 94u32;
```

half16* (sym+add)@got@dtprel@ha

### `R_PPC_TLSGD`

```rust
const R_PPC_TLSGD: u32 = 95u32;
```

none    (sym+add)@tlsgd

### `R_PPC_TLSLD`

```rust
const R_PPC_TLSLD: u32 = 96u32;
```

none    (sym+add)@tlsld

### `R_PPC_EMB_NADDR32`

```rust
const R_PPC_EMB_NADDR32: u32 = 101u32;
```

### `R_PPC_EMB_NADDR16`

```rust
const R_PPC_EMB_NADDR16: u32 = 102u32;
```

### `R_PPC_EMB_NADDR16_LO`

```rust
const R_PPC_EMB_NADDR16_LO: u32 = 103u32;
```

### `R_PPC_EMB_NADDR16_HI`

```rust
const R_PPC_EMB_NADDR16_HI: u32 = 104u32;
```

### `R_PPC_EMB_NADDR16_HA`

```rust
const R_PPC_EMB_NADDR16_HA: u32 = 105u32;
```

### `R_PPC_EMB_SDAI16`

```rust
const R_PPC_EMB_SDAI16: u32 = 106u32;
```

### `R_PPC_EMB_SDA2I16`

```rust
const R_PPC_EMB_SDA2I16: u32 = 107u32;
```

### `R_PPC_EMB_SDA2REL`

```rust
const R_PPC_EMB_SDA2REL: u32 = 108u32;
```

### `R_PPC_EMB_SDA21`

```rust
const R_PPC_EMB_SDA21: u32 = 109u32;
```

16 bit offset in SDA

### `R_PPC_EMB_MRKREF`

```rust
const R_PPC_EMB_MRKREF: u32 = 110u32;
```

### `R_PPC_EMB_RELSEC16`

```rust
const R_PPC_EMB_RELSEC16: u32 = 111u32;
```

### `R_PPC_EMB_RELST_LO`

```rust
const R_PPC_EMB_RELST_LO: u32 = 112u32;
```

### `R_PPC_EMB_RELST_HI`

```rust
const R_PPC_EMB_RELST_HI: u32 = 113u32;
```

### `R_PPC_EMB_RELST_HA`

```rust
const R_PPC_EMB_RELST_HA: u32 = 114u32;
```

### `R_PPC_EMB_BIT_FLD`

```rust
const R_PPC_EMB_BIT_FLD: u32 = 115u32;
```

### `R_PPC_EMB_RELSDA`

```rust
const R_PPC_EMB_RELSDA: u32 = 116u32;
```

16 bit relative offset in SDA

### `R_PPC_DIAB_SDA21_LO`

```rust
const R_PPC_DIAB_SDA21_LO: u32 = 180u32;
```

like EMB_SDA21, but lower 16 bit

### `R_PPC_DIAB_SDA21_HI`

```rust
const R_PPC_DIAB_SDA21_HI: u32 = 181u32;
```

like EMB_SDA21, but high 16 bit

### `R_PPC_DIAB_SDA21_HA`

```rust
const R_PPC_DIAB_SDA21_HA: u32 = 182u32;
```

like EMB_SDA21, adjusted high 16

### `R_PPC_DIAB_RELSDA_LO`

```rust
const R_PPC_DIAB_RELSDA_LO: u32 = 183u32;
```

like EMB_RELSDA, but lower 16 bit

### `R_PPC_DIAB_RELSDA_HI`

```rust
const R_PPC_DIAB_RELSDA_HI: u32 = 184u32;
```

like EMB_RELSDA, but high 16 bit

### `R_PPC_DIAB_RELSDA_HA`

```rust
const R_PPC_DIAB_RELSDA_HA: u32 = 185u32;
```

like EMB_RELSDA, adjusted high 16

### `R_PPC_IRELATIVE`

```rust
const R_PPC_IRELATIVE: u32 = 248u32;
```

GNU extension to support local ifunc.

### `R_PPC_REL16`

```rust
const R_PPC_REL16: u32 = 249u32;
```

half16   (sym+add-.)

### `R_PPC_REL16_LO`

```rust
const R_PPC_REL16_LO: u32 = 250u32;
```

half16   (sym+add-.)@l

### `R_PPC_REL16_HI`

```rust
const R_PPC_REL16_HI: u32 = 251u32;
```

half16   (sym+add-.)@h

### `R_PPC_REL16_HA`

```rust
const R_PPC_REL16_HA: u32 = 252u32;
```

half16   (sym+add-.)@ha

### `R_PPC_TOC16`

```rust
const R_PPC_TOC16: u32 = 255u32;
```

This is a phony reloc to handle any old fashioned TOC16 references that may
still be in object files.

### `DT_PPC_GOT`

```rust
const DT_PPC_GOT: u32 = 1_879_048_192u32;
```

### `DT_PPC_OPT`

```rust
const DT_PPC_OPT: u32 = 1_879_048_193u32;
```

### `PPC_OPT_TLS`

```rust
const PPC_OPT_TLS: u32 = 1u32;
```

### `R_PPC64_NONE`

```rust
const R_PPC64_NONE: u32 = 0u32;
```

### `R_PPC64_ADDR32`

```rust
const R_PPC64_ADDR32: u32 = 1u32;
```

32bit absolute address

### `R_PPC64_ADDR24`

```rust
const R_PPC64_ADDR24: u32 = 2u32;
```

26bit address, word aligned

### `R_PPC64_ADDR16`

```rust
const R_PPC64_ADDR16: u32 = 3u32;
```

16bit absolute address

### `R_PPC64_ADDR16_LO`

```rust
const R_PPC64_ADDR16_LO: u32 = 4u32;
```

lower 16bits of address

### `R_PPC64_ADDR16_HI`

```rust
const R_PPC64_ADDR16_HI: u32 = 5u32;
```

high 16bits of address.

### `R_PPC64_ADDR16_HA`

```rust
const R_PPC64_ADDR16_HA: u32 = 6u32;
```

adjusted high 16bits.

### `R_PPC64_ADDR14`

```rust
const R_PPC64_ADDR14: u32 = 7u32;
```

16bit address, word aligned

### `R_PPC64_ADDR14_BRTAKEN`

```rust
const R_PPC64_ADDR14_BRTAKEN: u32 = 8u32;
```

### `R_PPC64_ADDR14_BRNTAKEN`

```rust
const R_PPC64_ADDR14_BRNTAKEN: u32 = 9u32;
```

### `R_PPC64_REL24`

```rust
const R_PPC64_REL24: u32 = 10u32;
```

PC-rel. 26 bit, word aligned

### `R_PPC64_REL14`

```rust
const R_PPC64_REL14: u32 = 11u32;
```

PC relative 16 bit

### `R_PPC64_REL14_BRTAKEN`

```rust
const R_PPC64_REL14_BRTAKEN: u32 = 12u32;
```

### `R_PPC64_REL14_BRNTAKEN`

```rust
const R_PPC64_REL14_BRNTAKEN: u32 = 13u32;
```

### `R_PPC64_GOT16`

```rust
const R_PPC64_GOT16: u32 = 14u32;
```

### `R_PPC64_GOT16_LO`

```rust
const R_PPC64_GOT16_LO: u32 = 15u32;
```

### `R_PPC64_GOT16_HI`

```rust
const R_PPC64_GOT16_HI: u32 = 16u32;
```

### `R_PPC64_GOT16_HA`

```rust
const R_PPC64_GOT16_HA: u32 = 17u32;
```

### `R_PPC64_COPY`

```rust
const R_PPC64_COPY: u32 = 19u32;
```

### `R_PPC64_GLOB_DAT`

```rust
const R_PPC64_GLOB_DAT: u32 = 20u32;
```

### `R_PPC64_JMP_SLOT`

```rust
const R_PPC64_JMP_SLOT: u32 = 21u32;
```

### `R_PPC64_RELATIVE`

```rust
const R_PPC64_RELATIVE: u32 = 22u32;
```

### `R_PPC64_UADDR32`

```rust
const R_PPC64_UADDR32: u32 = 24u32;
```

### `R_PPC64_UADDR16`

```rust
const R_PPC64_UADDR16: u32 = 25u32;
```

### `R_PPC64_REL32`

```rust
const R_PPC64_REL32: u32 = 26u32;
```

### `R_PPC64_PLT32`

```rust
const R_PPC64_PLT32: u32 = 27u32;
```

### `R_PPC64_PLTREL32`

```rust
const R_PPC64_PLTREL32: u32 = 28u32;
```

### `R_PPC64_PLT16_LO`

```rust
const R_PPC64_PLT16_LO: u32 = 29u32;
```

### `R_PPC64_PLT16_HI`

```rust
const R_PPC64_PLT16_HI: u32 = 30u32;
```

### `R_PPC64_PLT16_HA`

```rust
const R_PPC64_PLT16_HA: u32 = 31u32;
```

### `R_PPC64_SECTOFF`

```rust
const R_PPC64_SECTOFF: u32 = 33u32;
```

### `R_PPC64_SECTOFF_LO`

```rust
const R_PPC64_SECTOFF_LO: u32 = 34u32;
```

### `R_PPC64_SECTOFF_HI`

```rust
const R_PPC64_SECTOFF_HI: u32 = 35u32;
```

### `R_PPC64_SECTOFF_HA`

```rust
const R_PPC64_SECTOFF_HA: u32 = 36u32;
```

### `R_PPC64_ADDR30`

```rust
const R_PPC64_ADDR30: u32 = 37u32;
```

word30 (S + A - P) >> 2

### `R_PPC64_ADDR64`

```rust
const R_PPC64_ADDR64: u32 = 38u32;
```

doubleword64 S + A

### `R_PPC64_ADDR16_HIGHER`

```rust
const R_PPC64_ADDR16_HIGHER: u32 = 39u32;
```

half16 #higher(S + A)

### `R_PPC64_ADDR16_HIGHERA`

```rust
const R_PPC64_ADDR16_HIGHERA: u32 = 40u32;
```

half16 #highera(S + A)

### `R_PPC64_ADDR16_HIGHEST`

```rust
const R_PPC64_ADDR16_HIGHEST: u32 = 41u32;
```

half16 #highest(S + A)

### `R_PPC64_ADDR16_HIGHESTA`

```rust
const R_PPC64_ADDR16_HIGHESTA: u32 = 42u32;
```

half16 #highesta(S + A)

### `R_PPC64_UADDR64`

```rust
const R_PPC64_UADDR64: u32 = 43u32;
```

doubleword64 S + A

### `R_PPC64_REL64`

```rust
const R_PPC64_REL64: u32 = 44u32;
```

doubleword64 S + A - P

### `R_PPC64_PLT64`

```rust
const R_PPC64_PLT64: u32 = 45u32;
```

doubleword64 L + A

### `R_PPC64_PLTREL64`

```rust
const R_PPC64_PLTREL64: u32 = 46u32;
```

doubleword64 L + A - P

### `R_PPC64_TOC16`

```rust
const R_PPC64_TOC16: u32 = 47u32;
```

half16* S + A - .TOC

### `R_PPC64_TOC16_LO`

```rust
const R_PPC64_TOC16_LO: u32 = 48u32;
```

half16 #lo(S + A - .TOC.)

### `R_PPC64_TOC16_HI`

```rust
const R_PPC64_TOC16_HI: u32 = 49u32;
```

half16 #hi(S + A - .TOC.)

### `R_PPC64_TOC16_HA`

```rust
const R_PPC64_TOC16_HA: u32 = 50u32;
```

half16 #ha(S + A - .TOC.)

### `R_PPC64_TOC`

```rust
const R_PPC64_TOC: u32 = 51u32;
```

doubleword64 .TOC

### `R_PPC64_PLTGOT16`

```rust
const R_PPC64_PLTGOT16: u32 = 52u32;
```

half16* M + A

### `R_PPC64_PLTGOT16_LO`

```rust
const R_PPC64_PLTGOT16_LO: u32 = 53u32;
```

half16 #lo(M + A)

### `R_PPC64_PLTGOT16_HI`

```rust
const R_PPC64_PLTGOT16_HI: u32 = 54u32;
```

half16 #hi(M + A)

### `R_PPC64_PLTGOT16_HA`

```rust
const R_PPC64_PLTGOT16_HA: u32 = 55u32;
```

half16 #ha(M + A)

### `R_PPC64_ADDR16_DS`

```rust
const R_PPC64_ADDR16_DS: u32 = 56u32;
```

half16ds* (S + A) >> 2

### `R_PPC64_ADDR16_LO_DS`

```rust
const R_PPC64_ADDR16_LO_DS: u32 = 57u32;
```

half16ds  #lo(S + A) >> 2

### `R_PPC64_GOT16_DS`

```rust
const R_PPC64_GOT16_DS: u32 = 58u32;
```

half16ds* (G + A) >> 2

### `R_PPC64_GOT16_LO_DS`

```rust
const R_PPC64_GOT16_LO_DS: u32 = 59u32;
```

half16ds  #lo(G + A) >> 2

### `R_PPC64_PLT16_LO_DS`

```rust
const R_PPC64_PLT16_LO_DS: u32 = 60u32;
```

half16ds  #lo(L + A) >> 2

### `R_PPC64_SECTOFF_DS`

```rust
const R_PPC64_SECTOFF_DS: u32 = 61u32;
```

half16ds* (R + A) >> 2

### `R_PPC64_SECTOFF_LO_DS`

```rust
const R_PPC64_SECTOFF_LO_DS: u32 = 62u32;
```

half16ds  #lo(R + A) >> 2

### `R_PPC64_TOC16_DS`

```rust
const R_PPC64_TOC16_DS: u32 = 63u32;
```

half16ds* (S + A - .TOC.) >> 2

### `R_PPC64_TOC16_LO_DS`

```rust
const R_PPC64_TOC16_LO_DS: u32 = 64u32;
```

half16ds  #lo(S + A - .TOC.) >> 2

### `R_PPC64_PLTGOT16_DS`

```rust
const R_PPC64_PLTGOT16_DS: u32 = 65u32;
```

half16ds* (M + A) >> 2

### `R_PPC64_PLTGOT16_LO_DS`

```rust
const R_PPC64_PLTGOT16_LO_DS: u32 = 66u32;
```

half16ds  #lo(M + A) >> 2

### `R_PPC64_TLS`

```rust
const R_PPC64_TLS: u32 = 67u32;
```

none    (sym+add)@tls

### `R_PPC64_DTPMOD64`

```rust
const R_PPC64_DTPMOD64: u32 = 68u32;
```

doubleword64 (sym+add)@dtpmod

### `R_PPC64_TPREL16`

```rust
const R_PPC64_TPREL16: u32 = 69u32;
```

half16* (sym+add)@tprel

### `R_PPC64_TPREL16_LO`

```rust
const R_PPC64_TPREL16_LO: u32 = 70u32;
```

half16  (sym+add)@tprel@l

### `R_PPC64_TPREL16_HI`

```rust
const R_PPC64_TPREL16_HI: u32 = 71u32;
```

half16  (sym+add)@tprel@h

### `R_PPC64_TPREL16_HA`

```rust
const R_PPC64_TPREL16_HA: u32 = 72u32;
```

half16  (sym+add)@tprel@ha

### `R_PPC64_TPREL64`

```rust
const R_PPC64_TPREL64: u32 = 73u32;
```

doubleword64 (sym+add)@tprel

### `R_PPC64_DTPREL16`

```rust
const R_PPC64_DTPREL16: u32 = 74u32;
```

half16* (sym+add)@dtprel

### `R_PPC64_DTPREL16_LO`

```rust
const R_PPC64_DTPREL16_LO: u32 = 75u32;
```

half16  (sym+add)@dtprel@l

### `R_PPC64_DTPREL16_HI`

```rust
const R_PPC64_DTPREL16_HI: u32 = 76u32;
```

half16  (sym+add)@dtprel@h

### `R_PPC64_DTPREL16_HA`

```rust
const R_PPC64_DTPREL16_HA: u32 = 77u32;
```

half16  (sym+add)@dtprel@ha

### `R_PPC64_DTPREL64`

```rust
const R_PPC64_DTPREL64: u32 = 78u32;
```

doubleword64 (sym+add)@dtprel

### `R_PPC64_GOT_TLSGD16`

```rust
const R_PPC64_GOT_TLSGD16: u32 = 79u32;
```

half16* (sym+add)@got@tlsgd

### `R_PPC64_GOT_TLSGD16_LO`

```rust
const R_PPC64_GOT_TLSGD16_LO: u32 = 80u32;
```

half16  (sym+add)@got@tlsgd@l

### `R_PPC64_GOT_TLSGD16_HI`

```rust
const R_PPC64_GOT_TLSGD16_HI: u32 = 81u32;
```

half16  (sym+add)@got@tlsgd@h

### `R_PPC64_GOT_TLSGD16_HA`

```rust
const R_PPC64_GOT_TLSGD16_HA: u32 = 82u32;
```

half16  (sym+add)@got@tlsgd@ha

### `R_PPC64_GOT_TLSLD16`

```rust
const R_PPC64_GOT_TLSLD16: u32 = 83u32;
```

half16* (sym+add)@got@tlsld

### `R_PPC64_GOT_TLSLD16_LO`

```rust
const R_PPC64_GOT_TLSLD16_LO: u32 = 84u32;
```

half16  (sym+add)@got@tlsld@l

### `R_PPC64_GOT_TLSLD16_HI`

```rust
const R_PPC64_GOT_TLSLD16_HI: u32 = 85u32;
```

half16  (sym+add)@got@tlsld@h

### `R_PPC64_GOT_TLSLD16_HA`

```rust
const R_PPC64_GOT_TLSLD16_HA: u32 = 86u32;
```

half16  (sym+add)@got@tlsld@ha

### `R_PPC64_GOT_TPREL16_DS`

```rust
const R_PPC64_GOT_TPREL16_DS: u32 = 87u32;
```

half16ds* (sym+add)@got@tprel

### `R_PPC64_GOT_TPREL16_LO_DS`

```rust
const R_PPC64_GOT_TPREL16_LO_DS: u32 = 88u32;
```

half16ds (sym+add)@got@tprel@l

### `R_PPC64_GOT_TPREL16_HI`

```rust
const R_PPC64_GOT_TPREL16_HI: u32 = 89u32;
```

half16  (sym+add)@got@tprel@h

### `R_PPC64_GOT_TPREL16_HA`

```rust
const R_PPC64_GOT_TPREL16_HA: u32 = 90u32;
```

half16  (sym+add)@got@tprel@ha

### `R_PPC64_GOT_DTPREL16_DS`

```rust
const R_PPC64_GOT_DTPREL16_DS: u32 = 91u32;
```

half16ds* (sym+add)@got@dtprel

### `R_PPC64_GOT_DTPREL16_LO_DS`

```rust
const R_PPC64_GOT_DTPREL16_LO_DS: u32 = 92u32;
```

half16ds (sym+add)@got@dtprel@l

### `R_PPC64_GOT_DTPREL16_HI`

```rust
const R_PPC64_GOT_DTPREL16_HI: u32 = 93u32;
```

half16  (sym+add)@got@dtprel@h

### `R_PPC64_GOT_DTPREL16_HA`

```rust
const R_PPC64_GOT_DTPREL16_HA: u32 = 94u32;
```

half16  (sym+add)@got@dtprel@ha

### `R_PPC64_TPREL16_DS`

```rust
const R_PPC64_TPREL16_DS: u32 = 95u32;
```

half16ds* (sym+add)@tprel

### `R_PPC64_TPREL16_LO_DS`

```rust
const R_PPC64_TPREL16_LO_DS: u32 = 96u32;
```

half16ds (sym+add)@tprel@l

### `R_PPC64_TPREL16_HIGHER`

```rust
const R_PPC64_TPREL16_HIGHER: u32 = 97u32;
```

half16  (sym+add)@tprel@higher

### `R_PPC64_TPREL16_HIGHERA`

```rust
const R_PPC64_TPREL16_HIGHERA: u32 = 98u32;
```

half16  (sym+add)@tprel@highera

### `R_PPC64_TPREL16_HIGHEST`

```rust
const R_PPC64_TPREL16_HIGHEST: u32 = 99u32;
```

half16  (sym+add)@tprel@highest

### `R_PPC64_TPREL16_HIGHESTA`

```rust
const R_PPC64_TPREL16_HIGHESTA: u32 = 100u32;
```

half16  (sym+add)@tprel@highesta

### `R_PPC64_DTPREL16_DS`

```rust
const R_PPC64_DTPREL16_DS: u32 = 101u32;
```

half16ds* (sym+add)@dtprel

### `R_PPC64_DTPREL16_LO_DS`

```rust
const R_PPC64_DTPREL16_LO_DS: u32 = 102u32;
```

half16ds (sym+add)@dtprel@l

### `R_PPC64_DTPREL16_HIGHER`

```rust
const R_PPC64_DTPREL16_HIGHER: u32 = 103u32;
```

half16  (sym+add)@dtprel@higher

### `R_PPC64_DTPREL16_HIGHERA`

```rust
const R_PPC64_DTPREL16_HIGHERA: u32 = 104u32;
```

half16  (sym+add)@dtprel@highera

### `R_PPC64_DTPREL16_HIGHEST`

```rust
const R_PPC64_DTPREL16_HIGHEST: u32 = 105u32;
```

half16  (sym+add)@dtprel@highest

### `R_PPC64_DTPREL16_HIGHESTA`

```rust
const R_PPC64_DTPREL16_HIGHESTA: u32 = 106u32;
```

half16  (sym+add)@dtprel@highesta

### `R_PPC64_TLSGD`

```rust
const R_PPC64_TLSGD: u32 = 107u32;
```

none    (sym+add)@tlsgd

### `R_PPC64_TLSLD`

```rust
const R_PPC64_TLSLD: u32 = 108u32;
```

none    (sym+add)@tlsld

### `R_PPC64_TOCSAVE`

```rust
const R_PPC64_TOCSAVE: u32 = 109u32;
```

none

### `R_PPC64_ADDR16_HIGH`

```rust
const R_PPC64_ADDR16_HIGH: u32 = 110u32;
```

### `R_PPC64_ADDR16_HIGHA`

```rust
const R_PPC64_ADDR16_HIGHA: u32 = 111u32;
```

### `R_PPC64_TPREL16_HIGH`

```rust
const R_PPC64_TPREL16_HIGH: u32 = 112u32;
```

### `R_PPC64_TPREL16_HIGHA`

```rust
const R_PPC64_TPREL16_HIGHA: u32 = 113u32;
```

### `R_PPC64_DTPREL16_HIGH`

```rust
const R_PPC64_DTPREL16_HIGH: u32 = 114u32;
```

### `R_PPC64_DTPREL16_HIGHA`

```rust
const R_PPC64_DTPREL16_HIGHA: u32 = 115u32;
```

### `R_PPC64_JMP_IREL`

```rust
const R_PPC64_JMP_IREL: u32 = 247u32;
```

GNU extension to support local ifunc.

### `R_PPC64_IRELATIVE`

```rust
const R_PPC64_IRELATIVE: u32 = 248u32;
```

GNU extension to support local ifunc.

### `R_PPC64_REL16`

```rust
const R_PPC64_REL16: u32 = 249u32;
```

half16   (sym+add-.)

### `R_PPC64_REL16_LO`

```rust
const R_PPC64_REL16_LO: u32 = 250u32;
```

half16   (sym+add-.)@l

### `R_PPC64_REL16_HI`

```rust
const R_PPC64_REL16_HI: u32 = 251u32;
```

half16   (sym+add-.)@h

### `R_PPC64_REL16_HA`

```rust
const R_PPC64_REL16_HA: u32 = 252u32;
```

half16   (sym+add-.)@ha

### `EF_PPC64_ABI`

```rust
const EF_PPC64_ABI: u32 = 3u32;
```

PowerPC64 bits specifying ABI.

1 for original function descriptor using ABI,
2 for revised ABI without function descriptors,
0 for unspecified or not using any features affected by the differences.

### `DT_PPC64_GLINK`

```rust
const DT_PPC64_GLINK: u32 = 1_879_048_192u32;
```

### `DT_PPC64_OPD`

```rust
const DT_PPC64_OPD: u32 = 1_879_048_193u32;
```

### `DT_PPC64_OPDSZ`

```rust
const DT_PPC64_OPDSZ: u32 = 1_879_048_194u32;
```

### `DT_PPC64_OPT`

```rust
const DT_PPC64_OPT: u32 = 1_879_048_195u32;
```

### `PPC64_OPT_TLS`

```rust
const PPC64_OPT_TLS: u32 = 1u32;
```

### `PPC64_OPT_MULTI_TOC`

```rust
const PPC64_OPT_MULTI_TOC: u32 = 2u32;
```

### `PPC64_OPT_LOCALENTRY`

```rust
const PPC64_OPT_LOCALENTRY: u32 = 4u32;
```

### `STO_PPC64_LOCAL_BIT`

```rust
const STO_PPC64_LOCAL_BIT: u8 = 5u8;
```

### `STO_PPC64_LOCAL_MASK`

```rust
const STO_PPC64_LOCAL_MASK: u8 = 224u8;
```

### `EF_ARM_RELEXEC`

```rust
const EF_ARM_RELEXEC: u32 = 1u32;
```

### `EF_ARM_HASENTRY`

```rust
const EF_ARM_HASENTRY: u32 = 2u32;
```

### `EF_ARM_INTERWORK`

```rust
const EF_ARM_INTERWORK: u32 = 4u32;
```

### `EF_ARM_APCS_26`

```rust
const EF_ARM_APCS_26: u32 = 8u32;
```

### `EF_ARM_APCS_FLOAT`

```rust
const EF_ARM_APCS_FLOAT: u32 = 16u32;
```

### `EF_ARM_PIC`

```rust
const EF_ARM_PIC: u32 = 32u32;
```

### `EF_ARM_ALIGN8`

```rust
const EF_ARM_ALIGN8: u32 = 64u32;
```

8-bit structure alignment is in use

### `EF_ARM_NEW_ABI`

```rust
const EF_ARM_NEW_ABI: u32 = 128u32;
```

### `EF_ARM_OLD_ABI`

```rust
const EF_ARM_OLD_ABI: u32 = 256u32;
```

### `EF_ARM_SOFT_FLOAT`

```rust
const EF_ARM_SOFT_FLOAT: u32 = 512u32;
```

### `EF_ARM_VFP_FLOAT`

```rust
const EF_ARM_VFP_FLOAT: u32 = 1_024u32;
```

### `EF_ARM_MAVERICK_FLOAT`

```rust
const EF_ARM_MAVERICK_FLOAT: u32 = 2_048u32;
```

### `EF_ARM_ABI_FLOAT_SOFT`

```rust
const EF_ARM_ABI_FLOAT_SOFT: u32 = 512u32;
```

NB conflicts with EF_ARM_SOFT_FLOAT

### `EF_ARM_ABI_FLOAT_HARD`

```rust
const EF_ARM_ABI_FLOAT_HARD: u32 = 1_024u32;
```

NB conflicts with EF_ARM_VFP_FLOAT

### `EF_ARM_SYMSARESORTED`

```rust
const EF_ARM_SYMSARESORTED: u32 = 4u32;
```

### `EF_ARM_DYNSYMSUSESEGIDX`

```rust
const EF_ARM_DYNSYMSUSESEGIDX: u32 = 8u32;
```

### `EF_ARM_MAPSYMSFIRST`

```rust
const EF_ARM_MAPSYMSFIRST: u32 = 16u32;
```

### `EF_ARM_BE8`

```rust
const EF_ARM_BE8: u32 = 8_388_608u32;
```

### `EF_ARM_LE8`

```rust
const EF_ARM_LE8: u32 = 4_194_304u32;
```

### `EF_ARM_EABIMASK`

```rust
const EF_ARM_EABIMASK: u32 = 4_278_190_080u32;
```

### `EF_ARM_EABI_UNKNOWN`

```rust
const EF_ARM_EABI_UNKNOWN: u32 = 0u32;
```

### `EF_ARM_EABI_VER1`

```rust
const EF_ARM_EABI_VER1: u32 = 16_777_216u32;
```

### `EF_ARM_EABI_VER2`

```rust
const EF_ARM_EABI_VER2: u32 = 33_554_432u32;
```

### `EF_ARM_EABI_VER3`

```rust
const EF_ARM_EABI_VER3: u32 = 50_331_648u32;
```

### `EF_ARM_EABI_VER4`

```rust
const EF_ARM_EABI_VER4: u32 = 67_108_864u32;
```

### `EF_ARM_EABI_VER5`

```rust
const EF_ARM_EABI_VER5: u32 = 83_886_080u32;
```

### `STT_ARM_TFUNC`

```rust
const STT_ARM_TFUNC: u8 = 13u8;
```

A Thumb function.

### `STT_ARM_16BIT`

```rust
const STT_ARM_16BIT: u8 = 15u8;
```

A Thumb label.

### `SHF_ARM_ENTRYSECT`

```rust
const SHF_ARM_ENTRYSECT: u32 = 268_435_456u32;
```

Section contains an entry point

### `SHF_ARM_COMDEF`

```rust
const SHF_ARM_COMDEF: u32 = 2_147_483_648u32;
```

Section may be multiply defined in the input to a link step.

### `PF_ARM_SB`

```rust
const PF_ARM_SB: u32 = 268_435_456u32;
```

Segment contains the location addressed by the static base.

### `PF_ARM_PI`

```rust
const PF_ARM_PI: u32 = 536_870_912u32;
```

Position-independent segment.

### `PF_ARM_ABS`

```rust
const PF_ARM_ABS: u32 = 1_073_741_824u32;
```

Absolute segment.

### `PT_ARM_EXIDX`

```rust
const PT_ARM_EXIDX: u32 = 1_879_048_193u32;
```

ARM unwind segment.

### `SHT_ARM_EXIDX`

```rust
const SHT_ARM_EXIDX: u32 = 1_879_048_193u32;
```

ARM unwind section.

### `SHT_ARM_PREEMPTMAP`

```rust
const SHT_ARM_PREEMPTMAP: u32 = 1_879_048_194u32;
```

Preemption details.

### `SHT_ARM_ATTRIBUTES`

```rust
const SHT_ARM_ATTRIBUTES: u32 = 1_879_048_195u32;
```

ARM attributes section.

### `SHT_AARCH64_ATTRIBUTES`

```rust
const SHT_AARCH64_ATTRIBUTES: u32 = 1_879_048_195u32;
```

AArch64 attributes section.

### `R_AARCH64_NONE`

```rust
const R_AARCH64_NONE: u32 = 0u32;
```

No relocation.

### `R_AARCH64_P32_ABS32`

```rust
const R_AARCH64_P32_ABS32: u32 = 1u32;
```

Direct 32 bit.

### `R_AARCH64_P32_COPY`

```rust
const R_AARCH64_P32_COPY: u32 = 180u32;
```

Copy symbol at runtime.

### `R_AARCH64_P32_GLOB_DAT`

```rust
const R_AARCH64_P32_GLOB_DAT: u32 = 181u32;
```

Create GOT entry.

### `R_AARCH64_P32_JUMP_SLOT`

```rust
const R_AARCH64_P32_JUMP_SLOT: u32 = 182u32;
```

Create PLT entry.

### `R_AARCH64_P32_RELATIVE`

```rust
const R_AARCH64_P32_RELATIVE: u32 = 183u32;
```

Adjust by program base.

### `R_AARCH64_P32_TLS_DTPMOD`

```rust
const R_AARCH64_P32_TLS_DTPMOD: u32 = 184u32;
```

Module number, 32 bit.

### `R_AARCH64_P32_TLS_DTPREL`

```rust
const R_AARCH64_P32_TLS_DTPREL: u32 = 185u32;
```

Module-relative offset, 32 bit.

### `R_AARCH64_P32_TLS_TPREL`

```rust
const R_AARCH64_P32_TLS_TPREL: u32 = 186u32;
```

TP-relative offset, 32 bit.

### `R_AARCH64_P32_TLSDESC`

```rust
const R_AARCH64_P32_TLSDESC: u32 = 187u32;
```

TLS Descriptor.

### `R_AARCH64_P32_IRELATIVE`

```rust
const R_AARCH64_P32_IRELATIVE: u32 = 188u32;
```

STT_GNU_IFUNC relocation.

### `R_AARCH64_ABS64`

```rust
const R_AARCH64_ABS64: u32 = 257u32;
```

Direct 64 bit.

### `R_AARCH64_ABS32`

```rust
const R_AARCH64_ABS32: u32 = 258u32;
```

Direct 32 bit.

### `R_AARCH64_ABS16`

```rust
const R_AARCH64_ABS16: u32 = 259u32;
```

Direct 16-bit.

### `R_AARCH64_PREL64`

```rust
const R_AARCH64_PREL64: u32 = 260u32;
```

PC-relative 64-bit.

### `R_AARCH64_PREL32`

```rust
const R_AARCH64_PREL32: u32 = 261u32;
```

PC-relative 32-bit.

### `R_AARCH64_PREL16`

```rust
const R_AARCH64_PREL16: u32 = 262u32;
```

PC-relative 16-bit.

### `R_AARCH64_MOVW_UABS_G0`

```rust
const R_AARCH64_MOVW_UABS_G0: u32 = 263u32;
```

Dir. MOVZ imm. from bits 15:0.

### `R_AARCH64_MOVW_UABS_G0_NC`

```rust
const R_AARCH64_MOVW_UABS_G0_NC: u32 = 264u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G1`

```rust
const R_AARCH64_MOVW_UABS_G1: u32 = 265u32;
```

Dir. MOVZ imm. from bits 31:16.

### `R_AARCH64_MOVW_UABS_G1_NC`

```rust
const R_AARCH64_MOVW_UABS_G1_NC: u32 = 266u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G2`

```rust
const R_AARCH64_MOVW_UABS_G2: u32 = 267u32;
```

Dir. MOVZ imm. from bits 47:32.

### `R_AARCH64_MOVW_UABS_G2_NC`

```rust
const R_AARCH64_MOVW_UABS_G2_NC: u32 = 268u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G3`

```rust
const R_AARCH64_MOVW_UABS_G3: u32 = 269u32;
```

Dir. MOV{K,Z} imm. from 63:48.

### `R_AARCH64_MOVW_SABS_G0`

```rust
const R_AARCH64_MOVW_SABS_G0: u32 = 270u32;
```

Dir. MOV{N,Z} imm. from 15:0.

### `R_AARCH64_MOVW_SABS_G1`

```rust
const R_AARCH64_MOVW_SABS_G1: u32 = 271u32;
```

Dir. MOV{N,Z} imm. from 31:16.

### `R_AARCH64_MOVW_SABS_G2`

```rust
const R_AARCH64_MOVW_SABS_G2: u32 = 272u32;
```

Dir. MOV{N,Z} imm. from 47:32.

### `R_AARCH64_LD_PREL_LO19`

```rust
const R_AARCH64_LD_PREL_LO19: u32 = 273u32;
```

PC-rel. LD imm. from bits 20:2.

### `R_AARCH64_ADR_PREL_LO21`

```rust
const R_AARCH64_ADR_PREL_LO21: u32 = 274u32;
```

PC-rel. ADR imm. from bits 20:0.

### `R_AARCH64_ADR_PREL_PG_HI21`

```rust
const R_AARCH64_ADR_PREL_PG_HI21: u32 = 275u32;
```

Page-rel. ADRP imm. from 32:12.

### `R_AARCH64_ADR_PREL_PG_HI21_NC`

```rust
const R_AARCH64_ADR_PREL_PG_HI21_NC: u32 = 276u32;
```

Likewise; no overflow check.

### `R_AARCH64_ADD_ABS_LO12_NC`

```rust
const R_AARCH64_ADD_ABS_LO12_NC: u32 = 277u32;
```

Dir. ADD imm. from bits 11:0.

### `R_AARCH64_LDST8_ABS_LO12_NC`

```rust
const R_AARCH64_LDST8_ABS_LO12_NC: u32 = 278u32;
```

Likewise for LD/ST; no check.

### `R_AARCH64_TSTBR14`

```rust
const R_AARCH64_TSTBR14: u32 = 279u32;
```

PC-rel. TBZ/TBNZ imm. from 15:2.

### `R_AARCH64_CONDBR19`

```rust
const R_AARCH64_CONDBR19: u32 = 280u32;
```

PC-rel. cond. br. imm. from 20:2.

### `R_AARCH64_JUMP26`

```rust
const R_AARCH64_JUMP26: u32 = 282u32;
```

PC-rel. B imm. from bits 27:2.

### `R_AARCH64_CALL26`

```rust
const R_AARCH64_CALL26: u32 = 283u32;
```

Likewise for CALL.

### `R_AARCH64_LDST16_ABS_LO12_NC`

```rust
const R_AARCH64_LDST16_ABS_LO12_NC: u32 = 284u32;
```

Dir. ADD imm. from bits 11:1.

### `R_AARCH64_LDST32_ABS_LO12_NC`

```rust
const R_AARCH64_LDST32_ABS_LO12_NC: u32 = 285u32;
```

Likewise for bits 11:2.

### `R_AARCH64_LDST64_ABS_LO12_NC`

```rust
const R_AARCH64_LDST64_ABS_LO12_NC: u32 = 286u32;
```

Likewise for bits 11:3.

### `R_AARCH64_MOVW_PREL_G0`

```rust
const R_AARCH64_MOVW_PREL_G0: u32 = 287u32;
```

PC-rel. MOV{N,Z} imm. from 15:0.

### `R_AARCH64_MOVW_PREL_G0_NC`

```rust
const R_AARCH64_MOVW_PREL_G0_NC: u32 = 288u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G1`

```rust
const R_AARCH64_MOVW_PREL_G1: u32 = 289u32;
```

PC-rel. MOV{N,Z} imm. from 31:16.

### `R_AARCH64_MOVW_PREL_G1_NC`

```rust
const R_AARCH64_MOVW_PREL_G1_NC: u32 = 290u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G2`

```rust
const R_AARCH64_MOVW_PREL_G2: u32 = 291u32;
```

PC-rel. MOV{N,Z} imm. from 47:32.

### `R_AARCH64_MOVW_PREL_G2_NC`

```rust
const R_AARCH64_MOVW_PREL_G2_NC: u32 = 292u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G3`

```rust
const R_AARCH64_MOVW_PREL_G3: u32 = 293u32;
```

PC-rel. MOV{N,Z} imm. from 63:48.

### `R_AARCH64_LDST128_ABS_LO12_NC`

```rust
const R_AARCH64_LDST128_ABS_LO12_NC: u32 = 299u32;
```

Dir. ADD imm. from bits 11:4.

### `R_AARCH64_MOVW_GOTOFF_G0`

```rust
const R_AARCH64_MOVW_GOTOFF_G0: u32 = 300u32;
```

GOT-rel. off. MOV{N,Z} imm. 15:0.

### `R_AARCH64_MOVW_GOTOFF_G0_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G0_NC: u32 = 301u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G1`

```rust
const R_AARCH64_MOVW_GOTOFF_G1: u32 = 302u32;
```

GOT-rel. o. MOV{N,Z} imm. 31:16.

### `R_AARCH64_MOVW_GOTOFF_G1_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G1_NC: u32 = 303u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G2`

```rust
const R_AARCH64_MOVW_GOTOFF_G2: u32 = 304u32;
```

GOT-rel. o. MOV{N,Z} imm. 47:32.

### `R_AARCH64_MOVW_GOTOFF_G2_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G2_NC: u32 = 305u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G3`

```rust
const R_AARCH64_MOVW_GOTOFF_G3: u32 = 306u32;
```

GOT-rel. o. MOV{N,Z} imm. 63:48.

### `R_AARCH64_GOTREL64`

```rust
const R_AARCH64_GOTREL64: u32 = 307u32;
```

GOT-relative 64-bit.

### `R_AARCH64_GOTREL32`

```rust
const R_AARCH64_GOTREL32: u32 = 308u32;
```

GOT-relative 32-bit.

### `R_AARCH64_GOT_LD_PREL19`

```rust
const R_AARCH64_GOT_LD_PREL19: u32 = 309u32;
```

PC-rel. GOT off. load imm. 20:2.

### `R_AARCH64_LD64_GOTOFF_LO15`

```rust
const R_AARCH64_LD64_GOTOFF_LO15: u32 = 310u32;
```

GOT-rel. off. LD/ST imm. 14:3.

### `R_AARCH64_ADR_GOT_PAGE`

```rust
const R_AARCH64_ADR_GOT_PAGE: u32 = 311u32;
```

P-page-rel. GOT off. ADRP 32:12.

### `R_AARCH64_LD64_GOT_LO12_NC`

```rust
const R_AARCH64_LD64_GOT_LO12_NC: u32 = 312u32;
```

Dir. GOT off. LD/ST imm. 11:3.

### `R_AARCH64_LD64_GOTPAGE_LO15`

```rust
const R_AARCH64_LD64_GOTPAGE_LO15: u32 = 313u32;
```

GOT-page-rel. GOT off. LD/ST 14:3

### `R_AARCH64_TLSGD_ADR_PREL21`

```rust
const R_AARCH64_TLSGD_ADR_PREL21: u32 = 512u32;
```

PC-relative ADR imm. 20:0.

### `R_AARCH64_TLSGD_ADR_PAGE21`

```rust
const R_AARCH64_TLSGD_ADR_PAGE21: u32 = 513u32;
```

page-rel. ADRP imm. 32:12.

### `R_AARCH64_TLSGD_ADD_LO12_NC`

```rust
const R_AARCH64_TLSGD_ADD_LO12_NC: u32 = 514u32;
```

direct ADD imm. from 11:0.

### `R_AARCH64_TLSGD_MOVW_G1`

```rust
const R_AARCH64_TLSGD_MOVW_G1: u32 = 515u32;
```

GOT-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSGD_MOVW_G0_NC`

```rust
const R_AARCH64_TLSGD_MOVW_G0_NC: u32 = 516u32;
```

GOT-rel. MOVK imm. 15:0.

### `R_AARCH64_TLSLD_ADR_PREL21`

```rust
const R_AARCH64_TLSLD_ADR_PREL21: u32 = 517u32;
```

Like 512; local dynamic model.

### `R_AARCH64_TLSLD_ADR_PAGE21`

```rust
const R_AARCH64_TLSLD_ADR_PAGE21: u32 = 518u32;
```

Like 513; local dynamic model.

### `R_AARCH64_TLSLD_ADD_LO12_NC`

```rust
const R_AARCH64_TLSLD_ADD_LO12_NC: u32 = 519u32;
```

Like 514; local dynamic model.

### `R_AARCH64_TLSLD_MOVW_G1`

```rust
const R_AARCH64_TLSLD_MOVW_G1: u32 = 520u32;
```

Like 515; local dynamic model.

### `R_AARCH64_TLSLD_MOVW_G0_NC`

```rust
const R_AARCH64_TLSLD_MOVW_G0_NC: u32 = 521u32;
```

Like 516; local dynamic model.

### `R_AARCH64_TLSLD_LD_PREL19`

```rust
const R_AARCH64_TLSLD_LD_PREL19: u32 = 522u32;
```

TLS PC-rel. load imm. 20:2.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G2`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G2: u32 = 523u32;
```

TLS DTP-rel. MOV{N,Z} 47:32.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G1`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G1: u32 = 524u32;
```

TLS DTP-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: u32 = 525u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G0`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G0: u32 = 526u32;
```

TLS DTP-rel. MOV{N,Z} 15:0.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: u32 = 527u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLD_ADD_DTPREL_HI12`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_HI12: u32 = 528u32;
```

DTP-rel. ADD imm. from 23:12.

### `R_AARCH64_TLSLD_ADD_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_LO12: u32 = 529u32;
```

DTP-rel. ADD imm. from 11:0.

### `R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: u32 = 530u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLD_LDST8_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST8_DTPREL_LO12: u32 = 531u32;
```

DTP-rel. LD/ST imm. 11:0.

### `R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: u32 = 532u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST16_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST16_DTPREL_LO12: u32 = 533u32;
```

DTP-rel. LD/ST imm. 11:1.

### `R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: u32 = 534u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST32_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST32_DTPREL_LO12: u32 = 535u32;
```

DTP-rel. LD/ST imm. 11:2.

### `R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: u32 = 536u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST64_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST64_DTPREL_LO12: u32 = 537u32;
```

DTP-rel. LD/ST imm. 11:3.

### `R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: u32 = 538u32;
```

Likewise; no check.

### `R_AARCH64_TLSIE_MOVW_GOTTPREL_G1`

```rust
const R_AARCH64_TLSIE_MOVW_GOTTPREL_G1: u32 = 539u32;
```

GOT-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC`

```rust
const R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: u32 = 540u32;
```

GOT-rel. MOVK 15:0.

### `R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21`

```rust
const R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: u32 = 541u32;
```

Page-rel. ADRP 32:12.

### `R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC`

```rust
const R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: u32 = 542u32;
```

Direct LD off. 11:3.

### `R_AARCH64_TLSIE_LD_GOTTPREL_PREL19`

```rust
const R_AARCH64_TLSIE_LD_GOTTPREL_PREL19: u32 = 543u32;
```

PC-rel. load imm. 20:2.

### `R_AARCH64_TLSLE_MOVW_TPREL_G2`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G2: u32 = 544u32;
```

TLS TP-rel. MOV{N,Z} 47:32.

### `R_AARCH64_TLSLE_MOVW_TPREL_G1`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G1: u32 = 545u32;
```

TLS TP-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSLE_MOVW_TPREL_G1_NC`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G1_NC: u32 = 546u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLE_MOVW_TPREL_G0`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G0: u32 = 547u32;
```

TLS TP-rel. MOV{N,Z} 15:0.

### `R_AARCH64_TLSLE_MOVW_TPREL_G0_NC`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G0_NC: u32 = 548u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLE_ADD_TPREL_HI12`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_HI12: u32 = 549u32;
```

TP-rel. ADD imm. 23:12.

### `R_AARCH64_TLSLE_ADD_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_LO12: u32 = 550u32;
```

TP-rel. ADD imm. 11:0.

### `R_AARCH64_TLSLE_ADD_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_LO12_NC: u32 = 551u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLE_LDST8_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST8_TPREL_LO12: u32 = 552u32;
```

TP-rel. LD/ST off. 11:0.

### `R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: u32 = 553u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLE_LDST16_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST16_TPREL_LO12: u32 = 554u32;
```

TP-rel. LD/ST off. 11:1.

### `R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: u32 = 555u32;
```

Likewise; no check.

### `R_AARCH64_TLSLE_LDST32_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST32_TPREL_LO12: u32 = 556u32;
```

TP-rel. LD/ST off. 11:2.

### `R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: u32 = 557u32;
```

Likewise; no check.

### `R_AARCH64_TLSLE_LDST64_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST64_TPREL_LO12: u32 = 558u32;
```

TP-rel. LD/ST off. 11:3.

### `R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: u32 = 559u32;
```

Likewise; no check.

### `R_AARCH64_TLSDESC_LD_PREL19`

```rust
const R_AARCH64_TLSDESC_LD_PREL19: u32 = 560u32;
```

PC-rel. load immediate 20:2.

### `R_AARCH64_TLSDESC_ADR_PREL21`

```rust
const R_AARCH64_TLSDESC_ADR_PREL21: u32 = 561u32;
```

PC-rel. ADR immediate 20:0.

### `R_AARCH64_TLSDESC_ADR_PAGE21`

```rust
const R_AARCH64_TLSDESC_ADR_PAGE21: u32 = 562u32;
```

Page-rel. ADRP imm. 32:12.

### `R_AARCH64_TLSDESC_LD64_LO12`

```rust
const R_AARCH64_TLSDESC_LD64_LO12: u32 = 563u32;
```

Direct LD off. from 11:3.

### `R_AARCH64_TLSDESC_ADD_LO12`

```rust
const R_AARCH64_TLSDESC_ADD_LO12: u32 = 564u32;
```

Direct ADD imm. from 11:0.

### `R_AARCH64_TLSDESC_OFF_G1`

```rust
const R_AARCH64_TLSDESC_OFF_G1: u32 = 565u32;
```

GOT-rel. MOV{N,Z} imm. 31:16.

### `R_AARCH64_TLSDESC_OFF_G0_NC`

```rust
const R_AARCH64_TLSDESC_OFF_G0_NC: u32 = 566u32;
```

GOT-rel. MOVK imm. 15:0; no ck.

### `R_AARCH64_TLSDESC_LDR`

```rust
const R_AARCH64_TLSDESC_LDR: u32 = 567u32;
```

Relax LDR.

### `R_AARCH64_TLSDESC_ADD`

```rust
const R_AARCH64_TLSDESC_ADD: u32 = 568u32;
```

Relax ADD.

### `R_AARCH64_TLSDESC_CALL`

```rust
const R_AARCH64_TLSDESC_CALL: u32 = 569u32;
```

Relax BLR.

### `R_AARCH64_TLSLE_LDST128_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST128_TPREL_LO12: u32 = 570u32;
```

TP-rel. LD/ST off. 11:4.

### `R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC: u32 = 571u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST128_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST128_DTPREL_LO12: u32 = 572u32;
```

DTP-rel. LD/ST imm. 11:4.

### `R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC: u32 = 573u32;
```

Likewise; no check.

### `R_AARCH64_COPY`

```rust
const R_AARCH64_COPY: u32 = 1_024u32;
```

Copy symbol at runtime.

### `R_AARCH64_GLOB_DAT`

```rust
const R_AARCH64_GLOB_DAT: u32 = 1_025u32;
```

Create GOT entry.

### `R_AARCH64_JUMP_SLOT`

```rust
const R_AARCH64_JUMP_SLOT: u32 = 1_026u32;
```

Create PLT entry.

### `R_AARCH64_RELATIVE`

```rust
const R_AARCH64_RELATIVE: u32 = 1_027u32;
```

Adjust by program base.

### `R_AARCH64_TLS_DTPMOD`

```rust
const R_AARCH64_TLS_DTPMOD: u32 = 1_028u32;
```

Module number, 64 bit.

### `R_AARCH64_TLS_DTPREL`

```rust
const R_AARCH64_TLS_DTPREL: u32 = 1_029u32;
```

Module-relative offset, 64 bit.

### `R_AARCH64_TLS_TPREL`

```rust
const R_AARCH64_TLS_TPREL: u32 = 1_030u32;
```

TP-relative offset, 64 bit.

### `R_AARCH64_TLSDESC`

```rust
const R_AARCH64_TLSDESC: u32 = 1_031u32;
```

TLS Descriptor.

### `R_AARCH64_IRELATIVE`

```rust
const R_AARCH64_IRELATIVE: u32 = 1_032u32;
```

STT_GNU_IFUNC relocation.

### `EF_AVR_ARCH`

```rust
const EF_AVR_ARCH: u32 = 127u32;
```

Bitmask for `EF_AVR_ARCH_*`.

### `EF_AVR_LINKRELAX_PREPARED`

```rust
const EF_AVR_LINKRELAX_PREPARED: u32 = 128u32;
```

If set, it is assumed that the elf file uses local symbols as reference
for the relocations so that linker relaxation is possible.

### `EF_AVR_ARCH_AVR1`

```rust
const EF_AVR_ARCH_AVR1: u32 = 1u32;
```

### `EF_AVR_ARCH_AVR2`

```rust
const EF_AVR_ARCH_AVR2: u32 = 2u32;
```

### `EF_AVR_ARCH_AVR25`

```rust
const EF_AVR_ARCH_AVR25: u32 = 25u32;
```

### `EF_AVR_ARCH_AVR3`

```rust
const EF_AVR_ARCH_AVR3: u32 = 3u32;
```

### `EF_AVR_ARCH_AVR31`

```rust
const EF_AVR_ARCH_AVR31: u32 = 31u32;
```

### `EF_AVR_ARCH_AVR35`

```rust
const EF_AVR_ARCH_AVR35: u32 = 35u32;
```

### `EF_AVR_ARCH_AVR4`

```rust
const EF_AVR_ARCH_AVR4: u32 = 4u32;
```

### `EF_AVR_ARCH_AVR5`

```rust
const EF_AVR_ARCH_AVR5: u32 = 5u32;
```

### `EF_AVR_ARCH_AVR51`

```rust
const EF_AVR_ARCH_AVR51: u32 = 51u32;
```

### `EF_AVR_ARCH_AVR6`

```rust
const EF_AVR_ARCH_AVR6: u32 = 6u32;
```

### `EF_AVR_ARCH_AVRTINY`

```rust
const EF_AVR_ARCH_AVRTINY: u32 = 100u32;
```

### `EF_AVR_ARCH_XMEGA1`

```rust
const EF_AVR_ARCH_XMEGA1: u32 = 101u32;
```

### `EF_AVR_ARCH_XMEGA2`

```rust
const EF_AVR_ARCH_XMEGA2: u32 = 102u32;
```

### `EF_AVR_ARCH_XMEGA3`

```rust
const EF_AVR_ARCH_XMEGA3: u32 = 103u32;
```

### `EF_AVR_ARCH_XMEGA4`

```rust
const EF_AVR_ARCH_XMEGA4: u32 = 104u32;
```

### `EF_AVR_ARCH_XMEGA5`

```rust
const EF_AVR_ARCH_XMEGA5: u32 = 105u32;
```

### `EF_AVR_ARCH_XMEGA6`

```rust
const EF_AVR_ARCH_XMEGA6: u32 = 106u32;
```

### `EF_AVR_ARCH_XMEGA7`

```rust
const EF_AVR_ARCH_XMEGA7: u32 = 107u32;
```

### `R_AVR_NONE`

```rust
const R_AVR_NONE: u32 = 0u32;
```

### `R_AVR_32`

```rust
const R_AVR_32: u32 = 1u32;
```

Direct 32 bit

### `R_AVR_7_PCREL`

```rust
const R_AVR_7_PCREL: u32 = 2u32;
```

### `R_AVR_13_PCREL`

```rust
const R_AVR_13_PCREL: u32 = 3u32;
```

### `R_AVR_16`

```rust
const R_AVR_16: u32 = 4u32;
```

Direct 16 bit

### `R_AVR_16_PM`

```rust
const R_AVR_16_PM: u32 = 5u32;
```

### `R_AVR_LO8_LDI`

```rust
const R_AVR_LO8_LDI: u32 = 6u32;
```

### `R_AVR_HI8_LDI`

```rust
const R_AVR_HI8_LDI: u32 = 7u32;
```

### `R_AVR_HH8_LDI`

```rust
const R_AVR_HH8_LDI: u32 = 8u32;
```

### `R_AVR_LO8_LDI_NEG`

```rust
const R_AVR_LO8_LDI_NEG: u32 = 9u32;
```

### `R_AVR_HI8_LDI_NEG`

```rust
const R_AVR_HI8_LDI_NEG: u32 = 10u32;
```

### `R_AVR_HH8_LDI_NEG`

```rust
const R_AVR_HH8_LDI_NEG: u32 = 11u32;
```

### `R_AVR_LO8_LDI_PM`

```rust
const R_AVR_LO8_LDI_PM: u32 = 12u32;
```

### `R_AVR_HI8_LDI_PM`

```rust
const R_AVR_HI8_LDI_PM: u32 = 13u32;
```

### `R_AVR_HH8_LDI_PM`

```rust
const R_AVR_HH8_LDI_PM: u32 = 14u32;
```

### `R_AVR_LO8_LDI_PM_NEG`

```rust
const R_AVR_LO8_LDI_PM_NEG: u32 = 15u32;
```

### `R_AVR_HI8_LDI_PM_NEG`

```rust
const R_AVR_HI8_LDI_PM_NEG: u32 = 16u32;
```

### `R_AVR_HH8_LDI_PM_NEG`

```rust
const R_AVR_HH8_LDI_PM_NEG: u32 = 17u32;
```

### `R_AVR_CALL`

```rust
const R_AVR_CALL: u32 = 18u32;
```

### `R_AVR_LDI`

```rust
const R_AVR_LDI: u32 = 19u32;
```

### `R_AVR_6`

```rust
const R_AVR_6: u32 = 20u32;
```

### `R_AVR_6_ADIW`

```rust
const R_AVR_6_ADIW: u32 = 21u32;
```

### `R_AVR_MS8_LDI`

```rust
const R_AVR_MS8_LDI: u32 = 22u32;
```

### `R_AVR_MS8_LDI_NEG`

```rust
const R_AVR_MS8_LDI_NEG: u32 = 23u32;
```

### `R_AVR_LO8_LDI_GS`

```rust
const R_AVR_LO8_LDI_GS: u32 = 24u32;
```

### `R_AVR_HI8_LDI_GS`

```rust
const R_AVR_HI8_LDI_GS: u32 = 25u32;
```

### `R_AVR_8`

```rust
const R_AVR_8: u32 = 26u32;
```

### `R_AVR_8_LO8`

```rust
const R_AVR_8_LO8: u32 = 27u32;
```

### `R_AVR_8_HI8`

```rust
const R_AVR_8_HI8: u32 = 28u32;
```

### `R_AVR_8_HLO8`

```rust
const R_AVR_8_HLO8: u32 = 29u32;
```

### `R_AVR_DIFF8`

```rust
const R_AVR_DIFF8: u32 = 30u32;
```

### `R_AVR_DIFF16`

```rust
const R_AVR_DIFF16: u32 = 31u32;
```

### `R_AVR_DIFF32`

```rust
const R_AVR_DIFF32: u32 = 32u32;
```

### `R_AVR_LDS_STS_16`

```rust
const R_AVR_LDS_STS_16: u32 = 33u32;
```

### `R_AVR_PORT6`

```rust
const R_AVR_PORT6: u32 = 34u32;
```

### `R_AVR_PORT5`

```rust
const R_AVR_PORT5: u32 = 35u32;
```

### `R_AVR_32_PCREL`

```rust
const R_AVR_32_PCREL: u32 = 36u32;
```

### `R_MSP430_32`

```rust
const R_MSP430_32: u32 = 1u32;
```

Direct 32 bit

### `R_MSP430_16_BYTE`

```rust
const R_MSP430_16_BYTE: u32 = 5u32;
```

Direct 16 bit

### `R_HEX_32`

```rust
const R_HEX_32: u32 = 6u32;
```

Direct 32 bit

### `R_ARM_NONE`

```rust
const R_ARM_NONE: u32 = 0u32;
```

No reloc

### `R_ARM_PC24`

```rust
const R_ARM_PC24: u32 = 1u32;
```

Deprecated PC relative 26 bit branch.

### `R_ARM_ABS32`

```rust
const R_ARM_ABS32: u32 = 2u32;
```

Direct 32 bit

### `R_ARM_REL32`

```rust
const R_ARM_REL32: u32 = 3u32;
```

PC relative 32 bit

### `R_ARM_PC13`

```rust
const R_ARM_PC13: u32 = 4u32;
```

### `R_ARM_ABS16`

```rust
const R_ARM_ABS16: u32 = 5u32;
```

Direct 16 bit

### `R_ARM_ABS12`

```rust
const R_ARM_ABS12: u32 = 6u32;
```

Direct 12 bit

### `R_ARM_THM_ABS5`

```rust
const R_ARM_THM_ABS5: u32 = 7u32;
```

Direct & 0x7C (`LDR`, `STR`).

### `R_ARM_ABS8`

```rust
const R_ARM_ABS8: u32 = 8u32;
```

Direct 8 bit

### `R_ARM_SBREL32`

```rust
const R_ARM_SBREL32: u32 = 9u32;
```

### `R_ARM_THM_PC22`

```rust
const R_ARM_THM_PC22: u32 = 10u32;
```

PC relative 24 bit (Thumb32 `BL`).

### `R_ARM_THM_PC8`

```rust
const R_ARM_THM_PC8: u32 = 11u32;
```

PC relative & 0x3FC (Thumb16 `LDR`, `ADD`, `ADR`).

### `R_ARM_AMP_VCALL9`

```rust
const R_ARM_AMP_VCALL9: u32 = 12u32;
```

### `R_ARM_SWI24`

```rust
const R_ARM_SWI24: u32 = 13u32;
```

Obsolete static relocation.

### `R_ARM_TLS_DESC`

```rust
const R_ARM_TLS_DESC: u32 = 13u32;
```

Dynamic relocation.

### `R_ARM_THM_SWI8`

```rust
const R_ARM_THM_SWI8: u32 = 14u32;
```

Reserved.

### `R_ARM_XPC25`

```rust
const R_ARM_XPC25: u32 = 15u32;
```

Reserved.

### `R_ARM_THM_XPC22`

```rust
const R_ARM_THM_XPC22: u32 = 16u32;
```

Reserved.

### `R_ARM_TLS_DTPMOD32`

```rust
const R_ARM_TLS_DTPMOD32: u32 = 17u32;
```

ID of module containing symbol

### `R_ARM_TLS_DTPOFF32`

```rust
const R_ARM_TLS_DTPOFF32: u32 = 18u32;
```

Offset in TLS block

### `R_ARM_TLS_TPOFF32`

```rust
const R_ARM_TLS_TPOFF32: u32 = 19u32;
```

Offset in static TLS block

### `R_ARM_COPY`

```rust
const R_ARM_COPY: u32 = 20u32;
```

Copy symbol at runtime

### `R_ARM_GLOB_DAT`

```rust
const R_ARM_GLOB_DAT: u32 = 21u32;
```

Create GOT entry

### `R_ARM_JUMP_SLOT`

```rust
const R_ARM_JUMP_SLOT: u32 = 22u32;
```

Create PLT entry

### `R_ARM_RELATIVE`

```rust
const R_ARM_RELATIVE: u32 = 23u32;
```

Adjust by program base

### `R_ARM_GOTOFF`

```rust
const R_ARM_GOTOFF: u32 = 24u32;
```

32 bit offset to GOT

### `R_ARM_GOTPC`

```rust
const R_ARM_GOTPC: u32 = 25u32;
```

32 bit PC relative offset to GOT

### `R_ARM_GOT32`

```rust
const R_ARM_GOT32: u32 = 26u32;
```

32 bit GOT entry

### `R_ARM_PLT32`

```rust
const R_ARM_PLT32: u32 = 27u32;
```

Deprecated, 32 bit PLT address.

### `R_ARM_CALL`

```rust
const R_ARM_CALL: u32 = 28u32;
```

PC relative 24 bit (`BL`, `BLX`).

### `R_ARM_JUMP24`

```rust
const R_ARM_JUMP24: u32 = 29u32;
```

PC relative 24 bit (`B`, `BL<cond>`).

### `R_ARM_THM_JUMP24`

```rust
const R_ARM_THM_JUMP24: u32 = 30u32;
```

PC relative 24 bit (Thumb32 `B.W`).

### `R_ARM_BASE_ABS`

```rust
const R_ARM_BASE_ABS: u32 = 31u32;
```

Adjust by program base.

### `R_ARM_ALU_PCREL_7_0`

```rust
const R_ARM_ALU_PCREL_7_0: u32 = 32u32;
```

Obsolete.

### `R_ARM_ALU_PCREL_15_8`

```rust
const R_ARM_ALU_PCREL_15_8: u32 = 33u32;
```

Obsolete.

### `R_ARM_ALU_PCREL_23_15`

```rust
const R_ARM_ALU_PCREL_23_15: u32 = 34u32;
```

Obsolete.

### `R_ARM_LDR_SBREL_11_0`

```rust
const R_ARM_LDR_SBREL_11_0: u32 = 35u32;
```

Deprecated, prog. base relative.

### `R_ARM_ALU_SBREL_19_12`

```rust
const R_ARM_ALU_SBREL_19_12: u32 = 36u32;
```

Deprecated, prog. base relative.

### `R_ARM_ALU_SBREL_27_20`

```rust
const R_ARM_ALU_SBREL_27_20: u32 = 37u32;
```

Deprecated, prog. base relative.

### `R_ARM_TARGET1`

```rust
const R_ARM_TARGET1: u32 = 38u32;
```

### `R_ARM_SBREL31`

```rust
const R_ARM_SBREL31: u32 = 39u32;
```

Program base relative.

### `R_ARM_V4BX`

```rust
const R_ARM_V4BX: u32 = 40u32;
```

### `R_ARM_TARGET2`

```rust
const R_ARM_TARGET2: u32 = 41u32;
```

### `R_ARM_PREL31`

```rust
const R_ARM_PREL31: u32 = 42u32;
```

32 bit PC relative.

### `R_ARM_MOVW_ABS_NC`

```rust
const R_ARM_MOVW_ABS_NC: u32 = 43u32;
```

Direct 16-bit (`MOVW`).

### `R_ARM_MOVT_ABS`

```rust
const R_ARM_MOVT_ABS: u32 = 44u32;
```

Direct high 16-bit (`MOVT`).

### `R_ARM_MOVW_PREL_NC`

```rust
const R_ARM_MOVW_PREL_NC: u32 = 45u32;
```

PC relative 16-bit (`MOVW`).

### `R_ARM_MOVT_PREL`

```rust
const R_ARM_MOVT_PREL: u32 = 46u32;
```

PC relative (MOVT).

### `R_ARM_THM_MOVW_ABS_NC`

```rust
const R_ARM_THM_MOVW_ABS_NC: u32 = 47u32;
```

Direct 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_ABS`

```rust
const R_ARM_THM_MOVT_ABS: u32 = 48u32;
```

Direct high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_MOVW_PREL_NC`

```rust
const R_ARM_THM_MOVW_PREL_NC: u32 = 49u32;
```

PC relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_PREL`

```rust
const R_ARM_THM_MOVT_PREL: u32 = 50u32;
```

PC relative high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_JUMP19`

```rust
const R_ARM_THM_JUMP19: u32 = 51u32;
```

PC relative 20 bit (Thumb32 `B<cond>.W`).

### `R_ARM_THM_JUMP6`

```rust
const R_ARM_THM_JUMP6: u32 = 52u32;
```

PC relative X & 0x7E (Thumb16 `CBZ`, `CBNZ`).

### `R_ARM_THM_ALU_PREL_11_0`

```rust
const R_ARM_THM_ALU_PREL_11_0: u32 = 53u32;
```

PC relative 12 bit (Thumb32 `ADR.W`).

### `R_ARM_THM_PC12`

```rust
const R_ARM_THM_PC12: u32 = 54u32;
```

PC relative 12 bit (Thumb32 `LDR{D,SB,H,SH}`).

### `R_ARM_ABS32_NOI`

```rust
const R_ARM_ABS32_NOI: u32 = 55u32;
```

Direct 32-bit.

### `R_ARM_REL32_NOI`

```rust
const R_ARM_REL32_NOI: u32 = 56u32;
```

PC relative 32-bit.

### `R_ARM_ALU_PC_G0_NC`

```rust
const R_ARM_ALU_PC_G0_NC: u32 = 57u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G0`

```rust
const R_ARM_ALU_PC_G0: u32 = 58u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G1_NC`

```rust
const R_ARM_ALU_PC_G1_NC: u32 = 59u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G1`

```rust
const R_ARM_ALU_PC_G1: u32 = 60u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G2`

```rust
const R_ARM_ALU_PC_G2: u32 = 61u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_LDR_PC_G1`

```rust
const R_ARM_LDR_PC_G1: u32 = 62u32;
```

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).

### `R_ARM_LDR_PC_G2`

```rust
const R_ARM_LDR_PC_G2: u32 = 63u32;
```

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).

### `R_ARM_LDRS_PC_G0`

```rust
const R_ARM_LDRS_PC_G0: u32 = 64u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDRS_PC_G1`

```rust
const R_ARM_LDRS_PC_G1: u32 = 65u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDRS_PC_G2`

```rust
const R_ARM_LDRS_PC_G2: u32 = 66u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDC_PC_G0`

```rust
const R_ARM_LDC_PC_G0: u32 = 67u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_LDC_PC_G1`

```rust
const R_ARM_LDC_PC_G1: u32 = 68u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_LDC_PC_G2`

```rust
const R_ARM_LDC_PC_G2: u32 = 69u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_ALU_SB_G0_NC`

```rust
const R_ARM_ALU_SB_G0_NC: u32 = 70u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G0`

```rust
const R_ARM_ALU_SB_G0: u32 = 71u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G1_NC`

```rust
const R_ARM_ALU_SB_G1_NC: u32 = 72u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G1`

```rust
const R_ARM_ALU_SB_G1: u32 = 73u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G2`

```rust
const R_ARM_ALU_SB_G2: u32 = 74u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_LDR_SB_G0`

```rust
const R_ARM_LDR_SB_G0: u32 = 75u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDR_SB_G1`

```rust
const R_ARM_LDR_SB_G1: u32 = 76u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDR_SB_G2`

```rust
const R_ARM_LDR_SB_G2: u32 = 77u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G0`

```rust
const R_ARM_LDRS_SB_G0: u32 = 78u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G1`

```rust
const R_ARM_LDRS_SB_G1: u32 = 79u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G2`

```rust
const R_ARM_LDRS_SB_G2: u32 = 80u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDC_SB_G0`

```rust
const R_ARM_LDC_SB_G0: u32 = 81u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_LDC_SB_G1`

```rust
const R_ARM_LDC_SB_G1: u32 = 82u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_LDC_SB_G2`

```rust
const R_ARM_LDC_SB_G2: u32 = 83u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_MOVW_BREL_NC`

```rust
const R_ARM_MOVW_BREL_NC: u32 = 84u32;
```

Program base relative 16 bit (`MOVW`).

### `R_ARM_MOVT_BREL`

```rust
const R_ARM_MOVT_BREL: u32 = 85u32;
```

Program base relative high 16 bit (`MOVT`).

### `R_ARM_MOVW_BREL`

```rust
const R_ARM_MOVW_BREL: u32 = 86u32;
```

Program base relative 16 bit (`MOVW`).

### `R_ARM_THM_MOVW_BREL_NC`

```rust
const R_ARM_THM_MOVW_BREL_NC: u32 = 87u32;
```

Program base relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_BREL`

```rust
const R_ARM_THM_MOVT_BREL: u32 = 88u32;
```

Program base relative high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_MOVW_BREL`

```rust
const R_ARM_THM_MOVW_BREL: u32 = 89u32;
```

Program base relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_TLS_GOTDESC`

```rust
const R_ARM_TLS_GOTDESC: u32 = 90u32;
```

### `R_ARM_TLS_CALL`

```rust
const R_ARM_TLS_CALL: u32 = 91u32;
```

### `R_ARM_TLS_DESCSEQ`

```rust
const R_ARM_TLS_DESCSEQ: u32 = 92u32;
```

TLS relaxation.

### `R_ARM_THM_TLS_CALL`

```rust
const R_ARM_THM_TLS_CALL: u32 = 93u32;
```

### `R_ARM_PLT32_ABS`

```rust
const R_ARM_PLT32_ABS: u32 = 94u32;
```

### `R_ARM_GOT_ABS`

```rust
const R_ARM_GOT_ABS: u32 = 95u32;
```

GOT entry.

### `R_ARM_GOT_PREL`

```rust
const R_ARM_GOT_PREL: u32 = 96u32;
```

PC relative GOT entry.

### `R_ARM_GOT_BREL12`

```rust
const R_ARM_GOT_BREL12: u32 = 97u32;
```

GOT entry relative to GOT origin (`LDR`).

### `R_ARM_GOTOFF12`

```rust
const R_ARM_GOTOFF12: u32 = 98u32;
```

12 bit, GOT entry relative to GOT origin (`LDR`, `STR`).

### `R_ARM_GOTRELAX`

```rust
const R_ARM_GOTRELAX: u32 = 99u32;
```

### `R_ARM_GNU_VTENTRY`

```rust
const R_ARM_GNU_VTENTRY: u32 = 100u32;
```

### `R_ARM_GNU_VTINHERIT`

```rust
const R_ARM_GNU_VTINHERIT: u32 = 101u32;
```

### `R_ARM_THM_PC11`

```rust
const R_ARM_THM_PC11: u32 = 102u32;
```

PC relative & 0xFFE (Thumb16 `B`).

### `R_ARM_THM_PC9`

```rust
const R_ARM_THM_PC9: u32 = 103u32;
```

PC relative & 0x1FE (Thumb16 `B`/`B<cond>`).

### `R_ARM_TLS_GD32`

```rust
const R_ARM_TLS_GD32: u32 = 104u32;
```

PC-rel 32 bit for global dynamic thread local data

### `R_ARM_TLS_LDM32`

```rust
const R_ARM_TLS_LDM32: u32 = 105u32;
```

PC-rel 32 bit for local dynamic thread local data

### `R_ARM_TLS_LDO32`

```rust
const R_ARM_TLS_LDO32: u32 = 106u32;
```

32 bit offset relative to TLS block

### `R_ARM_TLS_IE32`

```rust
const R_ARM_TLS_IE32: u32 = 107u32;
```

PC-rel 32 bit for GOT entry of static TLS block offset

### `R_ARM_TLS_LE32`

```rust
const R_ARM_TLS_LE32: u32 = 108u32;
```

32 bit offset relative to static TLS block

### `R_ARM_TLS_LDO12`

```rust
const R_ARM_TLS_LDO12: u32 = 109u32;
```

12 bit relative to TLS block (`LDR`, `STR`).

### `R_ARM_TLS_LE12`

```rust
const R_ARM_TLS_LE12: u32 = 110u32;
```

12 bit relative to static TLS block (`LDR`, `STR`).

### `R_ARM_TLS_IE12GP`

```rust
const R_ARM_TLS_IE12GP: u32 = 111u32;
```

12 bit GOT entry relative to GOT origin (`LDR`).

### `R_ARM_ME_TOO`

```rust
const R_ARM_ME_TOO: u32 = 128u32;
```

Obsolete.

### `R_ARM_THM_TLS_DESCSEQ`

```rust
const R_ARM_THM_TLS_DESCSEQ: u32 = 129u32;
```

### `R_ARM_THM_TLS_DESCSEQ16`

```rust
const R_ARM_THM_TLS_DESCSEQ16: u32 = 129u32;
```

### `R_ARM_THM_TLS_DESCSEQ32`

```rust
const R_ARM_THM_TLS_DESCSEQ32: u32 = 130u32;
```

### `R_ARM_THM_GOT_BREL12`

```rust
const R_ARM_THM_GOT_BREL12: u32 = 131u32;
```

GOT entry relative to GOT origin, 12 bit (Thumb32 `LDR`).

### `R_ARM_IRELATIVE`

```rust
const R_ARM_IRELATIVE: u32 = 160u32;
```

### `R_ARM_RXPC25`

```rust
const R_ARM_RXPC25: u32 = 249u32;
```

### `R_ARM_RSBREL32`

```rust
const R_ARM_RSBREL32: u32 = 250u32;
```

### `R_ARM_THM_RPC22`

```rust
const R_ARM_THM_RPC22: u32 = 251u32;
```

### `R_ARM_RREL32`

```rust
const R_ARM_RREL32: u32 = 252u32;
```

### `R_ARM_RABS22`

```rust
const R_ARM_RABS22: u32 = 253u32;
```

### `R_ARM_RPC24`

```rust
const R_ARM_RPC24: u32 = 254u32;
```

### `R_ARM_RBASE`

```rust
const R_ARM_RBASE: u32 = 255u32;
```

### `R_CKCORE_NONE`

```rust
const R_CKCORE_NONE: u32 = 0u32;
```

no reloc

### `R_CKCORE_ADDR32`

```rust
const R_CKCORE_ADDR32: u32 = 1u32;
```

direct 32 bit (S + A)

### `R_CKCORE_PCRELIMM8BY4`

```rust
const R_CKCORE_PCRELIMM8BY4: u32 = 2u32;
```

disp ((S + A - P) >> 2) & 0xff

### `R_CKCORE_PCRELIMM11BY2`

```rust
const R_CKCORE_PCRELIMM11BY2: u32 = 3u32;
```

disp ((S + A - P) >> 1) & 0x7ff

### `R_CKCORE_PCREL32`

```rust
const R_CKCORE_PCREL32: u32 = 5u32;
```

32-bit rel (S + A - P)

### `R_CKCORE_PCRELJSR_IMM11BY2`

```rust
const R_CKCORE_PCRELJSR_IMM11BY2: u32 = 6u32;
```

disp ((S + A - P) >>1) & 0x7ff

### `R_CKCORE_RELATIVE`

```rust
const R_CKCORE_RELATIVE: u32 = 9u32;
```

32 bit adjust program base(B + A)

### `R_CKCORE_COPY`

```rust
const R_CKCORE_COPY: u32 = 10u32;
```

32 bit adjust by program base

### `R_CKCORE_GLOB_DAT`

```rust
const R_CKCORE_GLOB_DAT: u32 = 11u32;
```

off between got and sym (S)

### `R_CKCORE_JUMP_SLOT`

```rust
const R_CKCORE_JUMP_SLOT: u32 = 12u32;
```

PLT entry (S)

### `R_CKCORE_GOTOFF`

```rust
const R_CKCORE_GOTOFF: u32 = 13u32;
```

offset to GOT (S + A - GOT)

### `R_CKCORE_GOTPC`

```rust
const R_CKCORE_GOTPC: u32 = 14u32;
```

PC offset to GOT (GOT + A - P)

### `R_CKCORE_GOT32`

```rust
const R_CKCORE_GOT32: u32 = 15u32;
```

32 bit GOT entry (G)

### `R_CKCORE_PLT32`

```rust
const R_CKCORE_PLT32: u32 = 16u32;
```

32 bit PLT entry (G)

### `R_CKCORE_ADDRGOT`

```rust
const R_CKCORE_ADDRGOT: u32 = 17u32;
```

GOT entry in GLOB_DAT (GOT + G)

### `R_CKCORE_ADDRPLT`

```rust
const R_CKCORE_ADDRPLT: u32 = 18u32;
```

PLT entry in GLOB_DAT (GOT + G)

### `R_CKCORE_PCREL_IMM26BY2`

```rust
const R_CKCORE_PCREL_IMM26BY2: u32 = 19u32;
```

((S + A - P) >> 1) & 0x3ff_ffff

### `R_CKCORE_PCREL_IMM16BY2`

```rust
const R_CKCORE_PCREL_IMM16BY2: u32 = 20u32;
```

disp ((S + A - P) >> 1) & 0xffff

### `R_CKCORE_PCREL_IMM16BY4`

```rust
const R_CKCORE_PCREL_IMM16BY4: u32 = 21u32;
```

disp ((S + A - P) >> 2) & 0xffff

### `R_CKCORE_PCREL_IMM10BY2`

```rust
const R_CKCORE_PCREL_IMM10BY2: u32 = 22u32;
```

disp ((S + A - P) >> 1) & 0x3ff

### `R_CKCORE_PCREL_IMM10BY4`

```rust
const R_CKCORE_PCREL_IMM10BY4: u32 = 23u32;
```

disp ((S + A - P) >> 2) & 0x3ff

### `R_CKCORE_ADDR_HI16`

```rust
const R_CKCORE_ADDR_HI16: u32 = 24u32;
```

high & low 16 bit ADDR, ((S + A) >> 16) & 0xffff

### `R_CKCORE_ADDR_LO16`

```rust
const R_CKCORE_ADDR_LO16: u32 = 25u32;
```

(S + A) & 0xffff

### `R_CKCORE_GOTPC_HI16`

```rust
const R_CKCORE_GOTPC_HI16: u32 = 26u32;
```

high & low 16 bit GOTPC, ((GOT + A - P) >> 16) & 0xffff

### `R_CKCORE_GOTPC_LO16`

```rust
const R_CKCORE_GOTPC_LO16: u32 = 27u32;
```

(GOT + A - P) & 0xffff

### `R_CKCORE_GOTOFF_HI16`

```rust
const R_CKCORE_GOTOFF_HI16: u32 = 28u32;
```

high & low 16 bit GOTOFF, ((S + A - GOT) >> 16) & 0xffff

### `R_CKCORE_GOTOFF_LO16`

```rust
const R_CKCORE_GOTOFF_LO16: u32 = 29u32;
```

(S + A - GOT) & 0xffff

### `R_CKCORE_GOT12`

```rust
const R_CKCORE_GOT12: u32 = 30u32;
```

12 bit disp GOT entry (G)

### `R_CKCORE_GOT_HI16`

```rust
const R_CKCORE_GOT_HI16: u32 = 31u32;
```

high & low 16 bit GOT, (G >> 16) & 0xffff

### `R_CKCORE_GOT_LO16`

```rust
const R_CKCORE_GOT_LO16: u32 = 32u32;
```

(G & 0xffff)

### `R_CKCORE_PLT12`

```rust
const R_CKCORE_PLT12: u32 = 33u32;
```

12 bit disp PLT entry (G)

### `R_CKCORE_PLT_HI16`

```rust
const R_CKCORE_PLT_HI16: u32 = 34u32;
```

high & low 16 bit PLT, (G >> 16) & 0xffff

### `R_CKCORE_PLT_LO16`

```rust
const R_CKCORE_PLT_LO16: u32 = 35u32;
```

G & 0xffff

### `R_CKCORE_ADDRGOT_HI16`

```rust
const R_CKCORE_ADDRGOT_HI16: u32 = 36u32;
```

high & low 16 bit ADDRGOT, (GOT + G * 4) & 0xffff

### `R_CKCORE_ADDRGOT_LO16`

```rust
const R_CKCORE_ADDRGOT_LO16: u32 = 37u32;
```

(GOT + G * 4) & 0xffff

### `R_CKCORE_ADDRPLT_HI16`

```rust
const R_CKCORE_ADDRPLT_HI16: u32 = 38u32;
```

high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF

### `R_CKCORE_ADDRPLT_LO16`

```rust
const R_CKCORE_ADDRPLT_LO16: u32 = 39u32;
```

(GOT+G*4) & 0xffff

### `R_CKCORE_PCREL_JSR_IMM26BY2`

```rust
const R_CKCORE_PCREL_JSR_IMM26BY2: u32 = 40u32;
```

disp ((S+A-P) >>1) & x3ff_ffff

### `R_CKCORE_TOFFSET_LO16`

```rust
const R_CKCORE_TOFFSET_LO16: u32 = 41u32;
```

(S+A-BTEXT) & 0xffff

### `R_CKCORE_DOFFSET_LO16`

```rust
const R_CKCORE_DOFFSET_LO16: u32 = 42u32;
```

(S+A-BTEXT) & 0xffff

### `R_CKCORE_PCREL_IMM18BY2`

```rust
const R_CKCORE_PCREL_IMM18BY2: u32 = 43u32;
```

disp ((S+A-P) >>1) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18`

```rust
const R_CKCORE_DOFFSET_IMM18: u32 = 44u32;
```

disp (S+A-BDATA) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18BY2`

```rust
const R_CKCORE_DOFFSET_IMM18BY2: u32 = 45u32;
```

disp ((S+A-BDATA)>>1) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18BY4`

```rust
const R_CKCORE_DOFFSET_IMM18BY4: u32 = 46u32;
```

disp ((S+A-BDATA)>>2) & 0x3ffff

### `R_CKCORE_GOT_IMM18BY4`

```rust
const R_CKCORE_GOT_IMM18BY4: u32 = 48u32;
```

disp (G >> 2)

### `R_CKCORE_PLT_IMM18BY4`

```rust
const R_CKCORE_PLT_IMM18BY4: u32 = 49u32;
```

disp (G >> 2)

### `R_CKCORE_PCREL_IMM7BY4`

```rust
const R_CKCORE_PCREL_IMM7BY4: u32 = 50u32;
```

disp ((S+A-P) >>2) & 0x7f

### `R_CKCORE_TLS_LE32`

```rust
const R_CKCORE_TLS_LE32: u32 = 51u32;
```

32 bit offset to TLS block

### `R_CKCORE_TLS_IE32`

```rust
const R_CKCORE_TLS_IE32: u32 = 52u32;
```

### `R_CKCORE_TLS_GD32`

```rust
const R_CKCORE_TLS_GD32: u32 = 53u32;
```

### `R_CKCORE_TLS_LDM32`

```rust
const R_CKCORE_TLS_LDM32: u32 = 54u32;
```

### `R_CKCORE_TLS_LDO32`

```rust
const R_CKCORE_TLS_LDO32: u32 = 55u32;
```

### `R_CKCORE_TLS_DTPMOD32`

```rust
const R_CKCORE_TLS_DTPMOD32: u32 = 56u32;
```

### `R_CKCORE_TLS_DTPOFF32`

```rust
const R_CKCORE_TLS_DTPOFF32: u32 = 57u32;
```

### `R_CKCORE_TLS_TPOFF32`

```rust
const R_CKCORE_TLS_TPOFF32: u32 = 58u32;
```

### `EF_CSKY_ABIMASK`

```rust
const EF_CSKY_ABIMASK: u32 = 4_026_531_840u32;
```

### `EF_CSKY_OTHER`

```rust
const EF_CSKY_OTHER: u32 = 268_369_920u32;
```

### `EF_CSKY_PROCESSOR`

```rust
const EF_CSKY_PROCESSOR: u32 = 65_535u32;
```

### `EF_CSKY_ABIV1`

```rust
const EF_CSKY_ABIV1: u32 = 268_435_456u32;
```

### `EF_CSKY_ABIV2`

```rust
const EF_CSKY_ABIV2: u32 = 536_870_912u32;
```

### `SHT_CSKY_ATTRIBUTES`

```rust
const SHT_CSKY_ATTRIBUTES: u32 = 1_879_048_193u32;
```

C-SKY attributes section.

### `EF_IA_64_MASKOS`

```rust
const EF_IA_64_MASKOS: u32 = 15u32;
```

os-specific flags

### `EF_IA_64_ABI64`

```rust
const EF_IA_64_ABI64: u32 = 16u32;
```

64-bit ABI

### `EF_IA_64_ARCH`

```rust
const EF_IA_64_ARCH: u32 = 4_278_190_080u32;
```

arch. version mask

### `PT_IA_64_ARCHEXT`

```rust
const PT_IA_64_ARCHEXT: u32 = 1_879_048_192u32;
```

arch extension bits

### `PT_IA_64_UNWIND`

```rust
const PT_IA_64_UNWIND: u32 = 1_879_048_193u32;
```

ia64 unwind bits

### `PT_IA_64_HP_OPT_ANOT`

```rust
const PT_IA_64_HP_OPT_ANOT: u32 = 1_610_612_754u32;
```

### `PT_IA_64_HP_HSL_ANOT`

```rust
const PT_IA_64_HP_HSL_ANOT: u32 = 1_610_612_755u32;
```

### `PT_IA_64_HP_STACK`

```rust
const PT_IA_64_HP_STACK: u32 = 1_610_612_756u32;
```

### `PF_IA_64_NORECOV`

```rust
const PF_IA_64_NORECOV: u32 = 2_147_483_648u32;
```

spec insns w/o recovery

### `SHT_IA_64_EXT`

```rust
const SHT_IA_64_EXT: u32 = 1_879_048_192u32;
```

extension bits

### `SHT_IA_64_UNWIND`

```rust
const SHT_IA_64_UNWIND: u32 = 1_879_048_193u32;
```

unwind bits

### `SHF_IA_64_SHORT`

```rust
const SHF_IA_64_SHORT: u32 = 268_435_456u32;
```

section near gp

### `SHF_IA_64_NORECOV`

```rust
const SHF_IA_64_NORECOV: u32 = 536_870_912u32;
```

spec insns w/o recovery

### `DT_IA_64_PLT_RESERVE`

```rust
const DT_IA_64_PLT_RESERVE: u32 = 1_879_048_192u32;
```

### `R_IA64_NONE`

```rust
const R_IA64_NONE: u32 = 0u32;
```

none

### `R_IA64_IMM14`

```rust
const R_IA64_IMM14: u32 = 33u32;
```

symbol + addend, add imm14

### `R_IA64_IMM22`

```rust
const R_IA64_IMM22: u32 = 34u32;
```

symbol + addend, add imm22

### `R_IA64_IMM64`

```rust
const R_IA64_IMM64: u32 = 35u32;
```

symbol + addend, mov imm64

### `R_IA64_DIR32MSB`

```rust
const R_IA64_DIR32MSB: u32 = 36u32;
```

symbol + addend, data4 MSB

### `R_IA64_DIR32LSB`

```rust
const R_IA64_DIR32LSB: u32 = 37u32;
```

symbol + addend, data4 LSB

### `R_IA64_DIR64MSB`

```rust
const R_IA64_DIR64MSB: u32 = 38u32;
```

symbol + addend, data8 MSB

### `R_IA64_DIR64LSB`

```rust
const R_IA64_DIR64LSB: u32 = 39u32;
```

symbol + addend, data8 LSB

### `R_IA64_GPREL22`

```rust
const R_IA64_GPREL22: u32 = 42u32;
```

@gprel(sym + add), add imm22

### `R_IA64_GPREL64I`

```rust
const R_IA64_GPREL64I: u32 = 43u32;
```

@gprel(sym + add), mov imm64

### `R_IA64_GPREL32MSB`

```rust
const R_IA64_GPREL32MSB: u32 = 44u32;
```

@gprel(sym + add), data4 MSB

### `R_IA64_GPREL32LSB`

```rust
const R_IA64_GPREL32LSB: u32 = 45u32;
```

@gprel(sym + add), data4 LSB

### `R_IA64_GPREL64MSB`

```rust
const R_IA64_GPREL64MSB: u32 = 46u32;
```

@gprel(sym + add), data8 MSB

### `R_IA64_GPREL64LSB`

```rust
const R_IA64_GPREL64LSB: u32 = 47u32;
```

@gprel(sym + add), data8 LSB

### `R_IA64_LTOFF22`

```rust
const R_IA64_LTOFF22: u32 = 50u32;
```

@ltoff(sym + add), add imm22

### `R_IA64_LTOFF64I`

```rust
const R_IA64_LTOFF64I: u32 = 51u32;
```

@ltoff(sym + add), mov imm64

### `R_IA64_PLTOFF22`

```rust
const R_IA64_PLTOFF22: u32 = 58u32;
```

@pltoff(sym + add), add imm22

### `R_IA64_PLTOFF64I`

```rust
const R_IA64_PLTOFF64I: u32 = 59u32;
```

@pltoff(sym + add), mov imm64

### `R_IA64_PLTOFF64MSB`

```rust
const R_IA64_PLTOFF64MSB: u32 = 62u32;
```

@pltoff(sym + add), data8 MSB

### `R_IA64_PLTOFF64LSB`

```rust
const R_IA64_PLTOFF64LSB: u32 = 63u32;
```

@pltoff(sym + add), data8 LSB

### `R_IA64_FPTR64I`

```rust
const R_IA64_FPTR64I: u32 = 67u32;
```

@fptr(sym + add), mov imm64

### `R_IA64_FPTR32MSB`

```rust
const R_IA64_FPTR32MSB: u32 = 68u32;
```

@fptr(sym + add), data4 MSB

### `R_IA64_FPTR32LSB`

```rust
const R_IA64_FPTR32LSB: u32 = 69u32;
```

@fptr(sym + add), data4 LSB

### `R_IA64_FPTR64MSB`

```rust
const R_IA64_FPTR64MSB: u32 = 70u32;
```

@fptr(sym + add), data8 MSB

### `R_IA64_FPTR64LSB`

```rust
const R_IA64_FPTR64LSB: u32 = 71u32;
```

@fptr(sym + add), data8 LSB

### `R_IA64_PCREL60B`

```rust
const R_IA64_PCREL60B: u32 = 72u32;
```

@pcrel(sym + add), brl

### `R_IA64_PCREL21B`

```rust
const R_IA64_PCREL21B: u32 = 73u32;
```

@pcrel(sym + add), ptb, call

### `R_IA64_PCREL21M`

```rust
const R_IA64_PCREL21M: u32 = 74u32;
```

@pcrel(sym + add), chk.s

### `R_IA64_PCREL21F`

```rust
const R_IA64_PCREL21F: u32 = 75u32;
```

@pcrel(sym + add), fchkf

### `R_IA64_PCREL32MSB`

```rust
const R_IA64_PCREL32MSB: u32 = 76u32;
```

@pcrel(sym + add), data4 MSB

### `R_IA64_PCREL32LSB`

```rust
const R_IA64_PCREL32LSB: u32 = 77u32;
```

@pcrel(sym + add), data4 LSB

### `R_IA64_PCREL64MSB`

```rust
const R_IA64_PCREL64MSB: u32 = 78u32;
```

@pcrel(sym + add), data8 MSB

### `R_IA64_PCREL64LSB`

```rust
const R_IA64_PCREL64LSB: u32 = 79u32;
```

@pcrel(sym + add), data8 LSB

### `R_IA64_LTOFF_FPTR22`

```rust
const R_IA64_LTOFF_FPTR22: u32 = 82u32;
```

@ltoff(@fptr(s+a)), imm22

### `R_IA64_LTOFF_FPTR64I`

```rust
const R_IA64_LTOFF_FPTR64I: u32 = 83u32;
```

@ltoff(@fptr(s+a)), imm64

### `R_IA64_LTOFF_FPTR32MSB`

```rust
const R_IA64_LTOFF_FPTR32MSB: u32 = 84u32;
```

@ltoff(@fptr(s+a)), data4 MSB

### `R_IA64_LTOFF_FPTR32LSB`

```rust
const R_IA64_LTOFF_FPTR32LSB: u32 = 85u32;
```

@ltoff(@fptr(s+a)), data4 LSB

### `R_IA64_LTOFF_FPTR64MSB`

```rust
const R_IA64_LTOFF_FPTR64MSB: u32 = 86u32;
```

@ltoff(@fptr(s+a)), data8 MSB

### `R_IA64_LTOFF_FPTR64LSB`

```rust
const R_IA64_LTOFF_FPTR64LSB: u32 = 87u32;
```

@ltoff(@fptr(s+a)), data8 LSB

### `R_IA64_SEGREL32MSB`

```rust
const R_IA64_SEGREL32MSB: u32 = 92u32;
```

@segrel(sym + add), data4 MSB

### `R_IA64_SEGREL32LSB`

```rust
const R_IA64_SEGREL32LSB: u32 = 93u32;
```

@segrel(sym + add), data4 LSB

### `R_IA64_SEGREL64MSB`

```rust
const R_IA64_SEGREL64MSB: u32 = 94u32;
```

@segrel(sym + add), data8 MSB

### `R_IA64_SEGREL64LSB`

```rust
const R_IA64_SEGREL64LSB: u32 = 95u32;
```

@segrel(sym + add), data8 LSB

### `R_IA64_SECREL32MSB`

```rust
const R_IA64_SECREL32MSB: u32 = 100u32;
```

@secrel(sym + add), data4 MSB

### `R_IA64_SECREL32LSB`

```rust
const R_IA64_SECREL32LSB: u32 = 101u32;
```

@secrel(sym + add), data4 LSB

### `R_IA64_SECREL64MSB`

```rust
const R_IA64_SECREL64MSB: u32 = 102u32;
```

@secrel(sym + add), data8 MSB

### `R_IA64_SECREL64LSB`

```rust
const R_IA64_SECREL64LSB: u32 = 103u32;
```

@secrel(sym + add), data8 LSB

### `R_IA64_REL32MSB`

```rust
const R_IA64_REL32MSB: u32 = 108u32;
```

data 4 + REL

### `R_IA64_REL32LSB`

```rust
const R_IA64_REL32LSB: u32 = 109u32;
```

data 4 + REL

### `R_IA64_REL64MSB`

```rust
const R_IA64_REL64MSB: u32 = 110u32;
```

data 8 + REL

### `R_IA64_REL64LSB`

```rust
const R_IA64_REL64LSB: u32 = 111u32;
```

data 8 + REL

### `R_IA64_LTV32MSB`

```rust
const R_IA64_LTV32MSB: u32 = 116u32;
```

symbol + addend, data4 MSB

### `R_IA64_LTV32LSB`

```rust
const R_IA64_LTV32LSB: u32 = 117u32;
```

symbol + addend, data4 LSB

### `R_IA64_LTV64MSB`

```rust
const R_IA64_LTV64MSB: u32 = 118u32;
```

symbol + addend, data8 MSB

### `R_IA64_LTV64LSB`

```rust
const R_IA64_LTV64LSB: u32 = 119u32;
```

symbol + addend, data8 LSB

### `R_IA64_PCREL21BI`

```rust
const R_IA64_PCREL21BI: u32 = 121u32;
```

@pcrel(sym + add), 21bit inst

### `R_IA64_PCREL22`

```rust
const R_IA64_PCREL22: u32 = 122u32;
```

@pcrel(sym + add), 22bit inst

### `R_IA64_PCREL64I`

```rust
const R_IA64_PCREL64I: u32 = 123u32;
```

@pcrel(sym + add), 64bit inst

### `R_IA64_IPLTMSB`

```rust
const R_IA64_IPLTMSB: u32 = 128u32;
```

dynamic reloc, imported PLT, MSB

### `R_IA64_IPLTLSB`

```rust
const R_IA64_IPLTLSB: u32 = 129u32;
```

dynamic reloc, imported PLT, LSB

### `R_IA64_COPY`

```rust
const R_IA64_COPY: u32 = 132u32;
```

copy relocation

### `R_IA64_SUB`

```rust
const R_IA64_SUB: u32 = 133u32;
```

Addend and symbol difference

### `R_IA64_LTOFF22X`

```rust
const R_IA64_LTOFF22X: u32 = 134u32;
```

LTOFF22, relaxable.

### `R_IA64_LDXMOV`

```rust
const R_IA64_LDXMOV: u32 = 135u32;
```

Use of LTOFF22X.

### `R_IA64_TPREL14`

```rust
const R_IA64_TPREL14: u32 = 145u32;
```

@tprel(sym + add), imm14

### `R_IA64_TPREL22`

```rust
const R_IA64_TPREL22: u32 = 146u32;
```

@tprel(sym + add), imm22

### `R_IA64_TPREL64I`

```rust
const R_IA64_TPREL64I: u32 = 147u32;
```

@tprel(sym + add), imm64

### `R_IA64_TPREL64MSB`

```rust
const R_IA64_TPREL64MSB: u32 = 150u32;
```

@tprel(sym + add), data8 MSB

### `R_IA64_TPREL64LSB`

```rust
const R_IA64_TPREL64LSB: u32 = 151u32;
```

@tprel(sym + add), data8 LSB

### `R_IA64_LTOFF_TPREL22`

```rust
const R_IA64_LTOFF_TPREL22: u32 = 154u32;
```

@ltoff(@tprel(s+a)), imm2

### `R_IA64_DTPMOD64MSB`

```rust
const R_IA64_DTPMOD64MSB: u32 = 166u32;
```

@dtpmod(sym + add), data8 MSB

### `R_IA64_DTPMOD64LSB`

```rust
const R_IA64_DTPMOD64LSB: u32 = 167u32;
```

@dtpmod(sym + add), data8 LSB

### `R_IA64_LTOFF_DTPMOD22`

```rust
const R_IA64_LTOFF_DTPMOD22: u32 = 170u32;
```

@ltoff(@dtpmod(sym + add)), imm22

### `R_IA64_DTPREL14`

```rust
const R_IA64_DTPREL14: u32 = 177u32;
```

@dtprel(sym + add), imm14

### `R_IA64_DTPREL22`

```rust
const R_IA64_DTPREL22: u32 = 178u32;
```

@dtprel(sym + add), imm22

### `R_IA64_DTPREL64I`

```rust
const R_IA64_DTPREL64I: u32 = 179u32;
```

@dtprel(sym + add), imm64

### `R_IA64_DTPREL32MSB`

```rust
const R_IA64_DTPREL32MSB: u32 = 180u32;
```

@dtprel(sym + add), data4 MSB

### `R_IA64_DTPREL32LSB`

```rust
const R_IA64_DTPREL32LSB: u32 = 181u32;
```

@dtprel(sym + add), data4 LSB

### `R_IA64_DTPREL64MSB`

```rust
const R_IA64_DTPREL64MSB: u32 = 182u32;
```

@dtprel(sym + add), data8 MSB

### `R_IA64_DTPREL64LSB`

```rust
const R_IA64_DTPREL64LSB: u32 = 183u32;
```

@dtprel(sym + add), data8 LSB

### `R_IA64_LTOFF_DTPREL22`

```rust
const R_IA64_LTOFF_DTPREL22: u32 = 186u32;
```

@ltoff(@dtprel(s+a)), imm22

### `EF_SH_MACH_MASK`

```rust
const EF_SH_MACH_MASK: u32 = 31u32;
```

### `EF_SH_UNKNOWN`

```rust
const EF_SH_UNKNOWN: u32 = 0u32;
```

### `EF_SH1`

```rust
const EF_SH1: u32 = 1u32;
```

### `EF_SH2`

```rust
const EF_SH2: u32 = 2u32;
```

### `EF_SH3`

```rust
const EF_SH3: u32 = 3u32;
```

### `EF_SH_DSP`

```rust
const EF_SH_DSP: u32 = 4u32;
```

### `EF_SH3_DSP`

```rust
const EF_SH3_DSP: u32 = 5u32;
```

### `EF_SH4AL_DSP`

```rust
const EF_SH4AL_DSP: u32 = 6u32;
```

### `EF_SH3E`

```rust
const EF_SH3E: u32 = 8u32;
```

### `EF_SH4`

```rust
const EF_SH4: u32 = 9u32;
```

### `EF_SH2E`

```rust
const EF_SH2E: u32 = 11u32;
```

### `EF_SH4A`

```rust
const EF_SH4A: u32 = 12u32;
```

### `EF_SH2A`

```rust
const EF_SH2A: u32 = 13u32;
```

### `EF_SH4_NOFPU`

```rust
const EF_SH4_NOFPU: u32 = 16u32;
```

### `EF_SH4A_NOFPU`

```rust
const EF_SH4A_NOFPU: u32 = 17u32;
```

### `EF_SH4_NOMMU_NOFPU`

```rust
const EF_SH4_NOMMU_NOFPU: u32 = 18u32;
```

### `EF_SH2A_NOFPU`

```rust
const EF_SH2A_NOFPU: u32 = 19u32;
```

### `EF_SH3_NOMMU`

```rust
const EF_SH3_NOMMU: u32 = 20u32;
```

### `EF_SH2A_SH4_NOFPU`

```rust
const EF_SH2A_SH4_NOFPU: u32 = 21u32;
```

### `EF_SH2A_SH3_NOFPU`

```rust
const EF_SH2A_SH3_NOFPU: u32 = 22u32;
```

### `EF_SH2A_SH4`

```rust
const EF_SH2A_SH4: u32 = 23u32;
```

### `EF_SH2A_SH3E`

```rust
const EF_SH2A_SH3E: u32 = 24u32;
```

### `R_SH_NONE`

```rust
const R_SH_NONE: u32 = 0u32;
```

### `R_SH_DIR32`

```rust
const R_SH_DIR32: u32 = 1u32;
```

### `R_SH_REL32`

```rust
const R_SH_REL32: u32 = 2u32;
```

### `R_SH_DIR8WPN`

```rust
const R_SH_DIR8WPN: u32 = 3u32;
```

### `R_SH_IND12W`

```rust
const R_SH_IND12W: u32 = 4u32;
```

### `R_SH_DIR8WPL`

```rust
const R_SH_DIR8WPL: u32 = 5u32;
```

### `R_SH_DIR8WPZ`

```rust
const R_SH_DIR8WPZ: u32 = 6u32;
```

### `R_SH_DIR8BP`

```rust
const R_SH_DIR8BP: u32 = 7u32;
```

### `R_SH_DIR8W`

```rust
const R_SH_DIR8W: u32 = 8u32;
```

### `R_SH_DIR8L`

```rust
const R_SH_DIR8L: u32 = 9u32;
```

### `R_SH_SWITCH16`

```rust
const R_SH_SWITCH16: u32 = 25u32;
```

### `R_SH_SWITCH32`

```rust
const R_SH_SWITCH32: u32 = 26u32;
```

### `R_SH_USES`

```rust
const R_SH_USES: u32 = 27u32;
```

### `R_SH_COUNT`

```rust
const R_SH_COUNT: u32 = 28u32;
```

### `R_SH_ALIGN`

```rust
const R_SH_ALIGN: u32 = 29u32;
```

### `R_SH_CODE`

```rust
const R_SH_CODE: u32 = 30u32;
```

### `R_SH_DATA`

```rust
const R_SH_DATA: u32 = 31u32;
```

### `R_SH_LABEL`

```rust
const R_SH_LABEL: u32 = 32u32;
```

### `R_SH_SWITCH8`

```rust
const R_SH_SWITCH8: u32 = 33u32;
```

### `R_SH_GNU_VTINHERIT`

```rust
const R_SH_GNU_VTINHERIT: u32 = 34u32;
```

### `R_SH_GNU_VTENTRY`

```rust
const R_SH_GNU_VTENTRY: u32 = 35u32;
```

### `R_SH_TLS_GD_32`

```rust
const R_SH_TLS_GD_32: u32 = 144u32;
```

### `R_SH_TLS_LD_32`

```rust
const R_SH_TLS_LD_32: u32 = 145u32;
```

### `R_SH_TLS_LDO_32`

```rust
const R_SH_TLS_LDO_32: u32 = 146u32;
```

### `R_SH_TLS_IE_32`

```rust
const R_SH_TLS_IE_32: u32 = 147u32;
```

### `R_SH_TLS_LE_32`

```rust
const R_SH_TLS_LE_32: u32 = 148u32;
```

### `R_SH_TLS_DTPMOD32`

```rust
const R_SH_TLS_DTPMOD32: u32 = 149u32;
```

### `R_SH_TLS_DTPOFF32`

```rust
const R_SH_TLS_DTPOFF32: u32 = 150u32;
```

### `R_SH_TLS_TPOFF32`

```rust
const R_SH_TLS_TPOFF32: u32 = 151u32;
```

### `R_SH_GOT32`

```rust
const R_SH_GOT32: u32 = 160u32;
```

### `R_SH_PLT32`

```rust
const R_SH_PLT32: u32 = 161u32;
```

### `R_SH_COPY`

```rust
const R_SH_COPY: u32 = 162u32;
```

### `R_SH_GLOB_DAT`

```rust
const R_SH_GLOB_DAT: u32 = 163u32;
```

### `R_SH_JMP_SLOT`

```rust
const R_SH_JMP_SLOT: u32 = 164u32;
```

### `R_SH_RELATIVE`

```rust
const R_SH_RELATIVE: u32 = 165u32;
```

### `R_SH_GOTOFF`

```rust
const R_SH_GOTOFF: u32 = 166u32;
```

### `R_SH_GOTPC`

```rust
const R_SH_GOTPC: u32 = 167u32;
```

### `EF_S390_HIGH_GPRS`

```rust
const EF_S390_HIGH_GPRS: u32 = 1u32;
```

High GPRs kernel facility needed.

### `R_390_NONE`

```rust
const R_390_NONE: u32 = 0u32;
```

No reloc.

### `R_390_8`

```rust
const R_390_8: u32 = 1u32;
```

Direct 8 bit.

### `R_390_12`

```rust
const R_390_12: u32 = 2u32;
```

Direct 12 bit.

### `R_390_16`

```rust
const R_390_16: u32 = 3u32;
```

Direct 16 bit.

### `R_390_32`

```rust
const R_390_32: u32 = 4u32;
```

Direct 32 bit.

### `R_390_PC32`

```rust
const R_390_PC32: u32 = 5u32;
```

PC relative 32 bit.

### `R_390_GOT12`

```rust
const R_390_GOT12: u32 = 6u32;
```

12 bit GOT offset.

### `R_390_GOT32`

```rust
const R_390_GOT32: u32 = 7u32;
```

32 bit GOT offset.

### `R_390_PLT32`

```rust
const R_390_PLT32: u32 = 8u32;
```

32 bit PC relative PLT address.

### `R_390_COPY`

```rust
const R_390_COPY: u32 = 9u32;
```

Copy symbol at runtime.

### `R_390_GLOB_DAT`

```rust
const R_390_GLOB_DAT: u32 = 10u32;
```

Create GOT entry.

### `R_390_JMP_SLOT`

```rust
const R_390_JMP_SLOT: u32 = 11u32;
```

Create PLT entry.

### `R_390_RELATIVE`

```rust
const R_390_RELATIVE: u32 = 12u32;
```

Adjust by program base.

### `R_390_GOTOFF32`

```rust
const R_390_GOTOFF32: u32 = 13u32;
```

32 bit offset to GOT.

### `R_390_GOTPC`

```rust
const R_390_GOTPC: u32 = 14u32;
```

32 bit PC relative offset to GOT.

### `R_390_GOT16`

```rust
const R_390_GOT16: u32 = 15u32;
```

16 bit GOT offset.

### `R_390_PC16`

```rust
const R_390_PC16: u32 = 16u32;
```

PC relative 16 bit.

### `R_390_PC16DBL`

```rust
const R_390_PC16DBL: u32 = 17u32;
```

PC relative 16 bit shifted by 1.

### `R_390_PLT16DBL`

```rust
const R_390_PLT16DBL: u32 = 18u32;
```

16 bit PC rel. PLT shifted by 1.

### `R_390_PC32DBL`

```rust
const R_390_PC32DBL: u32 = 19u32;
```

PC relative 32 bit shifted by 1.

### `R_390_PLT32DBL`

```rust
const R_390_PLT32DBL: u32 = 20u32;
```

32 bit PC rel. PLT shifted by 1.

### `R_390_GOTPCDBL`

```rust
const R_390_GOTPCDBL: u32 = 21u32;
```

32 bit PC rel. GOT shifted by 1.

### `R_390_64`

```rust
const R_390_64: u32 = 22u32;
```

Direct 64 bit.

### `R_390_PC64`

```rust
const R_390_PC64: u32 = 23u32;
```

PC relative 64 bit.

### `R_390_GOT64`

```rust
const R_390_GOT64: u32 = 24u32;
```

64 bit GOT offset.

### `R_390_PLT64`

```rust
const R_390_PLT64: u32 = 25u32;
```

64 bit PC relative PLT address.

### `R_390_GOTENT`

```rust
const R_390_GOTENT: u32 = 26u32;
```

32 bit PC rel. to GOT entry >> 1.

### `R_390_GOTOFF16`

```rust
const R_390_GOTOFF16: u32 = 27u32;
```

16 bit offset to GOT.

### `R_390_GOTOFF64`

```rust
const R_390_GOTOFF64: u32 = 28u32;
```

64 bit offset to GOT.

### `R_390_GOTPLT12`

```rust
const R_390_GOTPLT12: u32 = 29u32;
```

12 bit offset to jump slot.

### `R_390_GOTPLT16`

```rust
const R_390_GOTPLT16: u32 = 30u32;
```

16 bit offset to jump slot.

### `R_390_GOTPLT32`

```rust
const R_390_GOTPLT32: u32 = 31u32;
```

32 bit offset to jump slot.

### `R_390_GOTPLT64`

```rust
const R_390_GOTPLT64: u32 = 32u32;
```

64 bit offset to jump slot.

### `R_390_GOTPLTENT`

```rust
const R_390_GOTPLTENT: u32 = 33u32;
```

32 bit rel. offset to jump slot.

### `R_390_PLTOFF16`

```rust
const R_390_PLTOFF16: u32 = 34u32;
```

16 bit offset from GOT to PLT.

### `R_390_PLTOFF32`

```rust
const R_390_PLTOFF32: u32 = 35u32;
```

32 bit offset from GOT to PLT.

### `R_390_PLTOFF64`

```rust
const R_390_PLTOFF64: u32 = 36u32;
```

16 bit offset from GOT to PLT.

### `R_390_TLS_LOAD`

```rust
const R_390_TLS_LOAD: u32 = 37u32;
```

Tag for load insn in TLS code.

### `R_390_TLS_GDCALL`

```rust
const R_390_TLS_GDCALL: u32 = 38u32;
```

Tag for function call in general dynamic TLS code.

### `R_390_TLS_LDCALL`

```rust
const R_390_TLS_LDCALL: u32 = 39u32;
```

Tag for function call in local dynamic TLS code.

### `R_390_TLS_GD32`

```rust
const R_390_TLS_GD32: u32 = 40u32;
```

Direct 32 bit for general dynamic thread local data.

### `R_390_TLS_GD64`

```rust
const R_390_TLS_GD64: u32 = 41u32;
```

Direct 64 bit for general dynamic thread local data.

### `R_390_TLS_GOTIE12`

```rust
const R_390_TLS_GOTIE12: u32 = 42u32;
```

12 bit GOT offset for static TLS block offset.

### `R_390_TLS_GOTIE32`

```rust
const R_390_TLS_GOTIE32: u32 = 43u32;
```

32 bit GOT offset for static TLS block offset.

### `R_390_TLS_GOTIE64`

```rust
const R_390_TLS_GOTIE64: u32 = 44u32;
```

64 bit GOT offset for static TLS block offset.

### `R_390_TLS_LDM32`

```rust
const R_390_TLS_LDM32: u32 = 45u32;
```

Direct 32 bit for local dynamic thread local data in LE code.

### `R_390_TLS_LDM64`

```rust
const R_390_TLS_LDM64: u32 = 46u32;
```

Direct 64 bit for local dynamic thread local data in LE code.

### `R_390_TLS_IE32`

```rust
const R_390_TLS_IE32: u32 = 47u32;
```

32 bit address of GOT entry for negated static TLS block offset.

### `R_390_TLS_IE64`

```rust
const R_390_TLS_IE64: u32 = 48u32;
```

64 bit address of GOT entry for negated static TLS block offset.

### `R_390_TLS_IEENT`

```rust
const R_390_TLS_IEENT: u32 = 49u32;
```

32 bit rel. offset to GOT entry for negated static TLS block offset.

### `R_390_TLS_LE32`

```rust
const R_390_TLS_LE32: u32 = 50u32;
```

32 bit negated offset relative to static TLS block.

### `R_390_TLS_LE64`

```rust
const R_390_TLS_LE64: u32 = 51u32;
```

64 bit negated offset relative to static TLS block.

### `R_390_TLS_LDO32`

```rust
const R_390_TLS_LDO32: u32 = 52u32;
```

32 bit offset relative to TLS block.

### `R_390_TLS_LDO64`

```rust
const R_390_TLS_LDO64: u32 = 53u32;
```

64 bit offset relative to TLS block.

### `R_390_TLS_DTPMOD`

```rust
const R_390_TLS_DTPMOD: u32 = 54u32;
```

ID of module containing symbol.

### `R_390_TLS_DTPOFF`

```rust
const R_390_TLS_DTPOFF: u32 = 55u32;
```

Offset in TLS block.

### `R_390_TLS_TPOFF`

```rust
const R_390_TLS_TPOFF: u32 = 56u32;
```

Negated offset in static TLS block.

### `R_390_20`

```rust
const R_390_20: u32 = 57u32;
```

Direct 20 bit.

### `R_390_GOT20`

```rust
const R_390_GOT20: u32 = 58u32;
```

20 bit GOT offset.

### `R_390_GOTPLT20`

```rust
const R_390_GOTPLT20: u32 = 59u32;
```

20 bit offset to jump slot.

### `R_390_TLS_GOTIE20`

```rust
const R_390_TLS_GOTIE20: u32 = 60u32;
```

20 bit GOT offset for static TLS block offset.

### `R_390_IRELATIVE`

```rust
const R_390_IRELATIVE: u32 = 61u32;
```

STT_GNU_IFUNC relocation.

### `R_CRIS_NONE`

```rust
const R_CRIS_NONE: u32 = 0u32;
```

### `R_CRIS_8`

```rust
const R_CRIS_8: u32 = 1u32;
```

### `R_CRIS_16`

```rust
const R_CRIS_16: u32 = 2u32;
```

### `R_CRIS_32`

```rust
const R_CRIS_32: u32 = 3u32;
```

### `R_CRIS_8_PCREL`

```rust
const R_CRIS_8_PCREL: u32 = 4u32;
```

### `R_CRIS_16_PCREL`

```rust
const R_CRIS_16_PCREL: u32 = 5u32;
```

### `R_CRIS_32_PCREL`

```rust
const R_CRIS_32_PCREL: u32 = 6u32;
```

### `R_CRIS_GNU_VTINHERIT`

```rust
const R_CRIS_GNU_VTINHERIT: u32 = 7u32;
```

### `R_CRIS_GNU_VTENTRY`

```rust
const R_CRIS_GNU_VTENTRY: u32 = 8u32;
```

### `R_CRIS_COPY`

```rust
const R_CRIS_COPY: u32 = 9u32;
```

### `R_CRIS_GLOB_DAT`

```rust
const R_CRIS_GLOB_DAT: u32 = 10u32;
```

### `R_CRIS_JUMP_SLOT`

```rust
const R_CRIS_JUMP_SLOT: u32 = 11u32;
```

### `R_CRIS_RELATIVE`

```rust
const R_CRIS_RELATIVE: u32 = 12u32;
```

### `R_CRIS_16_GOT`

```rust
const R_CRIS_16_GOT: u32 = 13u32;
```

### `R_CRIS_32_GOT`

```rust
const R_CRIS_32_GOT: u32 = 14u32;
```

### `R_CRIS_16_GOTPLT`

```rust
const R_CRIS_16_GOTPLT: u32 = 15u32;
```

### `R_CRIS_32_GOTPLT`

```rust
const R_CRIS_32_GOTPLT: u32 = 16u32;
```

### `R_CRIS_32_GOTREL`

```rust
const R_CRIS_32_GOTREL: u32 = 17u32;
```

### `R_CRIS_32_PLT_GOTREL`

```rust
const R_CRIS_32_PLT_GOTREL: u32 = 18u32;
```

### `R_CRIS_32_PLT_PCREL`

```rust
const R_CRIS_32_PLT_PCREL: u32 = 19u32;
```

### `R_X86_64_NONE`

```rust
const R_X86_64_NONE: u32 = 0u32;
```

No reloc

### `R_X86_64_64`

```rust
const R_X86_64_64: u32 = 1u32;
```

Direct 64 bit

### `R_X86_64_PC32`

```rust
const R_X86_64_PC32: u32 = 2u32;
```

PC relative 32 bit signed

### `R_X86_64_GOT32`

```rust
const R_X86_64_GOT32: u32 = 3u32;
```

32 bit GOT entry

### `R_X86_64_PLT32`

```rust
const R_X86_64_PLT32: u32 = 4u32;
```

32 bit PLT address

### `R_X86_64_COPY`

```rust
const R_X86_64_COPY: u32 = 5u32;
```

Copy symbol at runtime

### `R_X86_64_GLOB_DAT`

```rust
const R_X86_64_GLOB_DAT: u32 = 6u32;
```

Create GOT entry

### `R_X86_64_JUMP_SLOT`

```rust
const R_X86_64_JUMP_SLOT: u32 = 7u32;
```

Create PLT entry

### `R_X86_64_RELATIVE`

```rust
const R_X86_64_RELATIVE: u32 = 8u32;
```

Adjust by program base

### `R_X86_64_GOTPCREL`

```rust
const R_X86_64_GOTPCREL: u32 = 9u32;
```

32 bit signed PC relative offset to GOT

### `R_X86_64_32`

```rust
const R_X86_64_32: u32 = 10u32;
```

Direct 32 bit zero extended

### `R_X86_64_32S`

```rust
const R_X86_64_32S: u32 = 11u32;
```

Direct 32 bit sign extended

### `R_X86_64_16`

```rust
const R_X86_64_16: u32 = 12u32;
```

Direct 16 bit zero extended

### `R_X86_64_PC16`

```rust
const R_X86_64_PC16: u32 = 13u32;
```

16 bit sign extended pc relative

### `R_X86_64_8`

```rust
const R_X86_64_8: u32 = 14u32;
```

Direct 8 bit sign extended

### `R_X86_64_PC8`

```rust
const R_X86_64_PC8: u32 = 15u32;
```

8 bit sign extended pc relative

### `R_X86_64_DTPMOD64`

```rust
const R_X86_64_DTPMOD64: u32 = 16u32;
```

ID of module containing symbol

### `R_X86_64_DTPOFF64`

```rust
const R_X86_64_DTPOFF64: u32 = 17u32;
```

Offset in module's TLS block

### `R_X86_64_TPOFF64`

```rust
const R_X86_64_TPOFF64: u32 = 18u32;
```

Offset in initial TLS block

### `R_X86_64_TLSGD`

```rust
const R_X86_64_TLSGD: u32 = 19u32;
```

32 bit signed PC relative offset to two GOT entries for GD symbol

### `R_X86_64_TLSLD`

```rust
const R_X86_64_TLSLD: u32 = 20u32;
```

32 bit signed PC relative offset to two GOT entries for LD symbol

### `R_X86_64_DTPOFF32`

```rust
const R_X86_64_DTPOFF32: u32 = 21u32;
```

Offset in TLS block

### `R_X86_64_GOTTPOFF`

```rust
const R_X86_64_GOTTPOFF: u32 = 22u32;
```

32 bit signed PC relative offset to GOT entry for IE symbol

### `R_X86_64_TPOFF32`

```rust
const R_X86_64_TPOFF32: u32 = 23u32;
```

Offset in initial TLS block

### `R_X86_64_PC64`

```rust
const R_X86_64_PC64: u32 = 24u32;
```

PC relative 64 bit

### `R_X86_64_GOTOFF64`

```rust
const R_X86_64_GOTOFF64: u32 = 25u32;
```

64 bit offset to GOT

### `R_X86_64_GOTPC32`

```rust
const R_X86_64_GOTPC32: u32 = 26u32;
```

32 bit signed pc relative offset to GOT

### `R_X86_64_GOT64`

```rust
const R_X86_64_GOT64: u32 = 27u32;
```

64-bit GOT entry offset

### `R_X86_64_GOTPCREL64`

```rust
const R_X86_64_GOTPCREL64: u32 = 28u32;
```

64-bit PC relative offset to GOT entry

### `R_X86_64_GOTPC64`

```rust
const R_X86_64_GOTPC64: u32 = 29u32;
```

64-bit PC relative offset to GOT

### `R_X86_64_GOTPLT64`

```rust
const R_X86_64_GOTPLT64: u32 = 30u32;
```

like GOT64, says PLT entry needed

### `R_X86_64_PLTOFF64`

```rust
const R_X86_64_PLTOFF64: u32 = 31u32;
```

64-bit GOT relative offset to PLT entry

### `R_X86_64_SIZE32`

```rust
const R_X86_64_SIZE32: u32 = 32u32;
```

Size of symbol plus 32-bit addend

### `R_X86_64_SIZE64`

```rust
const R_X86_64_SIZE64: u32 = 33u32;
```

Size of symbol plus 64-bit addend

### `R_X86_64_GOTPC32_TLSDESC`

```rust
const R_X86_64_GOTPC32_TLSDESC: u32 = 34u32;
```

GOT offset for TLS descriptor.

### `R_X86_64_TLSDESC_CALL`

```rust
const R_X86_64_TLSDESC_CALL: u32 = 35u32;
```

Marker for call through TLS descriptor.

### `R_X86_64_TLSDESC`

```rust
const R_X86_64_TLSDESC: u32 = 36u32;
```

TLS descriptor.

### `R_X86_64_IRELATIVE`

```rust
const R_X86_64_IRELATIVE: u32 = 37u32;
```

Adjust indirectly by program base

### `R_X86_64_RELATIVE64`

```rust
const R_X86_64_RELATIVE64: u32 = 38u32;
```

64-bit adjust by program base

### `R_X86_64_GOTPCRELX`

```rust
const R_X86_64_GOTPCRELX: u32 = 41u32;
```

Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.

### `R_X86_64_REX_GOTPCRELX`

```rust
const R_X86_64_REX_GOTPCRELX: u32 = 42u32;
```

Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.

### `SHT_X86_64_UNWIND`

```rust
const SHT_X86_64_UNWIND: u32 = 1_879_048_193u32;
```

Unwind information.

### `R_MN10300_NONE`

```rust
const R_MN10300_NONE: u32 = 0u32;
```

No reloc.

### `R_MN10300_32`

```rust
const R_MN10300_32: u32 = 1u32;
```

Direct 32 bit.

### `R_MN10300_16`

```rust
const R_MN10300_16: u32 = 2u32;
```

Direct 16 bit.

### `R_MN10300_8`

```rust
const R_MN10300_8: u32 = 3u32;
```

Direct 8 bit.

### `R_MN10300_PCREL32`

```rust
const R_MN10300_PCREL32: u32 = 4u32;
```

PC-relative 32-bit.

### `R_MN10300_PCREL16`

```rust
const R_MN10300_PCREL16: u32 = 5u32;
```

PC-relative 16-bit signed.

### `R_MN10300_PCREL8`

```rust
const R_MN10300_PCREL8: u32 = 6u32;
```

PC-relative 8-bit signed.

### `R_MN10300_GNU_VTINHERIT`

```rust
const R_MN10300_GNU_VTINHERIT: u32 = 7u32;
```

Ancient C++ vtable garbage...

### `R_MN10300_GNU_VTENTRY`

```rust
const R_MN10300_GNU_VTENTRY: u32 = 8u32;
```

... collection annotation.

### `R_MN10300_24`

```rust
const R_MN10300_24: u32 = 9u32;
```

Direct 24 bit.

### `R_MN10300_GOTPC32`

```rust
const R_MN10300_GOTPC32: u32 = 10u32;
```

32-bit PCrel offset to GOT.

### `R_MN10300_GOTPC16`

```rust
const R_MN10300_GOTPC16: u32 = 11u32;
```

16-bit PCrel offset to GOT.

### `R_MN10300_GOTOFF32`

```rust
const R_MN10300_GOTOFF32: u32 = 12u32;
```

32-bit offset from GOT.

### `R_MN10300_GOTOFF24`

```rust
const R_MN10300_GOTOFF24: u32 = 13u32;
```

24-bit offset from GOT.

### `R_MN10300_GOTOFF16`

```rust
const R_MN10300_GOTOFF16: u32 = 14u32;
```

16-bit offset from GOT.

### `R_MN10300_PLT32`

```rust
const R_MN10300_PLT32: u32 = 15u32;
```

32-bit PCrel to PLT entry.

### `R_MN10300_PLT16`

```rust
const R_MN10300_PLT16: u32 = 16u32;
```

16-bit PCrel to PLT entry.

### `R_MN10300_GOT32`

```rust
const R_MN10300_GOT32: u32 = 17u32;
```

32-bit offset to GOT entry.

### `R_MN10300_GOT24`

```rust
const R_MN10300_GOT24: u32 = 18u32;
```

24-bit offset to GOT entry.

### `R_MN10300_GOT16`

```rust
const R_MN10300_GOT16: u32 = 19u32;
```

16-bit offset to GOT entry.

### `R_MN10300_COPY`

```rust
const R_MN10300_COPY: u32 = 20u32;
```

Copy symbol at runtime.

### `R_MN10300_GLOB_DAT`

```rust
const R_MN10300_GLOB_DAT: u32 = 21u32;
```

Create GOT entry.

### `R_MN10300_JMP_SLOT`

```rust
const R_MN10300_JMP_SLOT: u32 = 22u32;
```

Create PLT entry.

### `R_MN10300_RELATIVE`

```rust
const R_MN10300_RELATIVE: u32 = 23u32;
```

Adjust by program base.

### `R_MN10300_TLS_GD`

```rust
const R_MN10300_TLS_GD: u32 = 24u32;
```

32-bit offset for global dynamic.

### `R_MN10300_TLS_LD`

```rust
const R_MN10300_TLS_LD: u32 = 25u32;
```

32-bit offset for local dynamic.

### `R_MN10300_TLS_LDO`

```rust
const R_MN10300_TLS_LDO: u32 = 26u32;
```

Module-relative offset.

### `R_MN10300_TLS_GOTIE`

```rust
const R_MN10300_TLS_GOTIE: u32 = 27u32;
```

GOT offset for static TLS block offset.

### `R_MN10300_TLS_IE`

```rust
const R_MN10300_TLS_IE: u32 = 28u32;
```

GOT address for static TLS block offset.

### `R_MN10300_TLS_LE`

```rust
const R_MN10300_TLS_LE: u32 = 29u32;
```

Offset relative to static TLS block.

### `R_MN10300_TLS_DTPMOD`

```rust
const R_MN10300_TLS_DTPMOD: u32 = 30u32;
```

ID of module containing symbol.

### `R_MN10300_TLS_DTPOFF`

```rust
const R_MN10300_TLS_DTPOFF: u32 = 31u32;
```

Offset in module TLS block.

### `R_MN10300_TLS_TPOFF`

```rust
const R_MN10300_TLS_TPOFF: u32 = 32u32;
```

Offset in static TLS block.

### `R_MN10300_SYM_DIFF`

```rust
const R_MN10300_SYM_DIFF: u32 = 33u32;
```

Adjustment for next reloc as needed by linker relaxation.

### `R_MN10300_ALIGN`

```rust
const R_MN10300_ALIGN: u32 = 34u32;
```

Alignment requirement for linker relaxation.

### `R_M32R_NONE`

```rust
const R_M32R_NONE: u32 = 0u32;
```

No reloc.

### `R_M32R_16`

```rust
const R_M32R_16: u32 = 1u32;
```

Direct 16 bit.

### `R_M32R_32`

```rust
const R_M32R_32: u32 = 2u32;
```

Direct 32 bit.

### `R_M32R_24`

```rust
const R_M32R_24: u32 = 3u32;
```

Direct 24 bit.

### `R_M32R_10_PCREL`

```rust
const R_M32R_10_PCREL: u32 = 4u32;
```

PC relative 10 bit shifted.

### `R_M32R_18_PCREL`

```rust
const R_M32R_18_PCREL: u32 = 5u32;
```

PC relative 18 bit shifted.

### `R_M32R_26_PCREL`

```rust
const R_M32R_26_PCREL: u32 = 6u32;
```

PC relative 26 bit shifted.

### `R_M32R_HI16_ULO`

```rust
const R_M32R_HI16_ULO: u32 = 7u32;
```

High 16 bit with unsigned low.

### `R_M32R_HI16_SLO`

```rust
const R_M32R_HI16_SLO: u32 = 8u32;
```

High 16 bit with signed low.

### `R_M32R_LO16`

```rust
const R_M32R_LO16: u32 = 9u32;
```

Low 16 bit.

### `R_M32R_SDA16`

```rust
const R_M32R_SDA16: u32 = 10u32;
```

16 bit offset in SDA.

### `R_M32R_GNU_VTINHERIT`

```rust
const R_M32R_GNU_VTINHERIT: u32 = 11u32;
```

### `R_M32R_GNU_VTENTRY`

```rust
const R_M32R_GNU_VTENTRY: u32 = 12u32;
```

### `R_M32R_16_RELA`

```rust
const R_M32R_16_RELA: u32 = 33u32;
```

Direct 16 bit.

### `R_M32R_32_RELA`

```rust
const R_M32R_32_RELA: u32 = 34u32;
```

Direct 32 bit.

### `R_M32R_24_RELA`

```rust
const R_M32R_24_RELA: u32 = 35u32;
```

Direct 24 bit.

### `R_M32R_10_PCREL_RELA`

```rust
const R_M32R_10_PCREL_RELA: u32 = 36u32;
```

PC relative 10 bit shifted.

### `R_M32R_18_PCREL_RELA`

```rust
const R_M32R_18_PCREL_RELA: u32 = 37u32;
```

PC relative 18 bit shifted.

### `R_M32R_26_PCREL_RELA`

```rust
const R_M32R_26_PCREL_RELA: u32 = 38u32;
```

PC relative 26 bit shifted.

### `R_M32R_HI16_ULO_RELA`

```rust
const R_M32R_HI16_ULO_RELA: u32 = 39u32;
```

High 16 bit with unsigned low

### `R_M32R_HI16_SLO_RELA`

```rust
const R_M32R_HI16_SLO_RELA: u32 = 40u32;
```

High 16 bit with signed low

### `R_M32R_LO16_RELA`

```rust
const R_M32R_LO16_RELA: u32 = 41u32;
```

Low 16 bit

### `R_M32R_SDA16_RELA`

```rust
const R_M32R_SDA16_RELA: u32 = 42u32;
```

16 bit offset in SDA

### `R_M32R_RELA_GNU_VTINHERIT`

```rust
const R_M32R_RELA_GNU_VTINHERIT: u32 = 43u32;
```

### `R_M32R_RELA_GNU_VTENTRY`

```rust
const R_M32R_RELA_GNU_VTENTRY: u32 = 44u32;
```

### `R_M32R_REL32`

```rust
const R_M32R_REL32: u32 = 45u32;
```

PC relative 32 bit.

### `R_M32R_GOT24`

```rust
const R_M32R_GOT24: u32 = 48u32;
```

24 bit GOT entry

### `R_M32R_26_PLTREL`

```rust
const R_M32R_26_PLTREL: u32 = 49u32;
```

26 bit PC relative to PLT shifted

### `R_M32R_COPY`

```rust
const R_M32R_COPY: u32 = 50u32;
```

Copy symbol at runtime

### `R_M32R_GLOB_DAT`

```rust
const R_M32R_GLOB_DAT: u32 = 51u32;
```

Create GOT entry

### `R_M32R_JMP_SLOT`

```rust
const R_M32R_JMP_SLOT: u32 = 52u32;
```

Create PLT entry

### `R_M32R_RELATIVE`

```rust
const R_M32R_RELATIVE: u32 = 53u32;
```

Adjust by program base

### `R_M32R_GOTOFF`

```rust
const R_M32R_GOTOFF: u32 = 54u32;
```

24 bit offset to GOT

### `R_M32R_GOTPC24`

```rust
const R_M32R_GOTPC24: u32 = 55u32;
```

24 bit PC relative offset to GOT

### `R_M32R_GOT16_HI_ULO`

```rust
const R_M32R_GOT16_HI_ULO: u32 = 56u32;
```

High 16 bit GOT entry with unsigned low

### `R_M32R_GOT16_HI_SLO`

```rust
const R_M32R_GOT16_HI_SLO: u32 = 57u32;
```

High 16 bit GOT entry with signed low

### `R_M32R_GOT16_LO`

```rust
const R_M32R_GOT16_LO: u32 = 58u32;
```

Low 16 bit GOT entry

### `R_M32R_GOTPC_HI_ULO`

```rust
const R_M32R_GOTPC_HI_ULO: u32 = 59u32;
```

High 16 bit PC relative offset to GOT with unsigned low

### `R_M32R_GOTPC_HI_SLO`

```rust
const R_M32R_GOTPC_HI_SLO: u32 = 60u32;
```

High 16 bit PC relative offset to GOT with signed low

### `R_M32R_GOTPC_LO`

```rust
const R_M32R_GOTPC_LO: u32 = 61u32;
```

Low 16 bit PC relative offset to GOT

### `R_M32R_GOTOFF_HI_ULO`

```rust
const R_M32R_GOTOFF_HI_ULO: u32 = 62u32;
```

High 16 bit offset to GOT with unsigned low

### `R_M32R_GOTOFF_HI_SLO`

```rust
const R_M32R_GOTOFF_HI_SLO: u32 = 63u32;
```

High 16 bit offset to GOT with signed low

### `R_M32R_GOTOFF_LO`

```rust
const R_M32R_GOTOFF_LO: u32 = 64u32;
```

Low 16 bit offset to GOT

### `R_M32R_NUM`

```rust
const R_M32R_NUM: u32 = 256u32;
```

Keep this the last entry.

### `R_MICROBLAZE_NONE`

```rust
const R_MICROBLAZE_NONE: u32 = 0u32;
```

No reloc.

### `R_MICROBLAZE_32`

```rust
const R_MICROBLAZE_32: u32 = 1u32;
```

Direct 32 bit.

### `R_MICROBLAZE_32_PCREL`

```rust
const R_MICROBLAZE_32_PCREL: u32 = 2u32;
```

PC relative 32 bit.

### `R_MICROBLAZE_64_PCREL`

```rust
const R_MICROBLAZE_64_PCREL: u32 = 3u32;
```

PC relative 64 bit.

### `R_MICROBLAZE_32_PCREL_LO`

```rust
const R_MICROBLAZE_32_PCREL_LO: u32 = 4u32;
```

Low 16 bits of PCREL32.

### `R_MICROBLAZE_64`

```rust
const R_MICROBLAZE_64: u32 = 5u32;
```

Direct 64 bit.

### `R_MICROBLAZE_32_LO`

```rust
const R_MICROBLAZE_32_LO: u32 = 6u32;
```

Low 16 bit.

### `R_MICROBLAZE_SRO32`

```rust
const R_MICROBLAZE_SRO32: u32 = 7u32;
```

Read-only small data area.

### `R_MICROBLAZE_SRW32`

```rust
const R_MICROBLAZE_SRW32: u32 = 8u32;
```

Read-write small data area.

### `R_MICROBLAZE_64_NONE`

```rust
const R_MICROBLAZE_64_NONE: u32 = 9u32;
```

No reloc.

### `R_MICROBLAZE_32_SYM_OP_SYM`

```rust
const R_MICROBLAZE_32_SYM_OP_SYM: u32 = 10u32;
```

Symbol Op Symbol relocation.

### `R_MICROBLAZE_GNU_VTINHERIT`

```rust
const R_MICROBLAZE_GNU_VTINHERIT: u32 = 11u32;
```

GNU C++ vtable hierarchy.

### `R_MICROBLAZE_GNU_VTENTRY`

```rust
const R_MICROBLAZE_GNU_VTENTRY: u32 = 12u32;
```

GNU C++ vtable member usage.

### `R_MICROBLAZE_GOTPC_64`

```rust
const R_MICROBLAZE_GOTPC_64: u32 = 13u32;
```

PC-relative GOT offset.

### `R_MICROBLAZE_GOT_64`

```rust
const R_MICROBLAZE_GOT_64: u32 = 14u32;
```

GOT entry offset.

### `R_MICROBLAZE_PLT_64`

```rust
const R_MICROBLAZE_PLT_64: u32 = 15u32;
```

PLT offset (PC-relative).

### `R_MICROBLAZE_REL`

```rust
const R_MICROBLAZE_REL: u32 = 16u32;
```

Adjust by program base.

### `R_MICROBLAZE_JUMP_SLOT`

```rust
const R_MICROBLAZE_JUMP_SLOT: u32 = 17u32;
```

Create PLT entry.

### `R_MICROBLAZE_GLOB_DAT`

```rust
const R_MICROBLAZE_GLOB_DAT: u32 = 18u32;
```

Create GOT entry.

### `R_MICROBLAZE_GOTOFF_64`

```rust
const R_MICROBLAZE_GOTOFF_64: u32 = 19u32;
```

64 bit offset to GOT.

### `R_MICROBLAZE_GOTOFF_32`

```rust
const R_MICROBLAZE_GOTOFF_32: u32 = 20u32;
```

32 bit offset to GOT.

### `R_MICROBLAZE_COPY`

```rust
const R_MICROBLAZE_COPY: u32 = 21u32;
```

Runtime copy.

### `R_MICROBLAZE_TLS`

```rust
const R_MICROBLAZE_TLS: u32 = 22u32;
```

TLS Reloc.

### `R_MICROBLAZE_TLSGD`

```rust
const R_MICROBLAZE_TLSGD: u32 = 23u32;
```

TLS General Dynamic.

### `R_MICROBLAZE_TLSLD`

```rust
const R_MICROBLAZE_TLSLD: u32 = 24u32;
```

TLS Local Dynamic.

### `R_MICROBLAZE_TLSDTPMOD32`

```rust
const R_MICROBLAZE_TLSDTPMOD32: u32 = 25u32;
```

TLS Module ID.

### `R_MICROBLAZE_TLSDTPREL32`

```rust
const R_MICROBLAZE_TLSDTPREL32: u32 = 26u32;
```

TLS Offset Within TLS Block.

### `R_MICROBLAZE_TLSDTPREL64`

```rust
const R_MICROBLAZE_TLSDTPREL64: u32 = 27u32;
```

TLS Offset Within TLS Block.

### `R_MICROBLAZE_TLSGOTTPREL32`

```rust
const R_MICROBLAZE_TLSGOTTPREL32: u32 = 28u32;
```

TLS Offset From Thread Pointer.

### `R_MICROBLAZE_TLSTPREL32`

```rust
const R_MICROBLAZE_TLSTPREL32: u32 = 29u32;
```

TLS Offset From Thread Pointer.

### `DT_NIOS2_GP`

```rust
const DT_NIOS2_GP: u32 = 1_879_048_194u32;
```

Address of _gp.

### `R_NIOS2_NONE`

```rust
const R_NIOS2_NONE: u32 = 0u32;
```

No reloc.

### `R_NIOS2_S16`

```rust
const R_NIOS2_S16: u32 = 1u32;
```

Direct signed 16 bit.

### `R_NIOS2_U16`

```rust
const R_NIOS2_U16: u32 = 2u32;
```

Direct unsigned 16 bit.

### `R_NIOS2_PCREL16`

```rust
const R_NIOS2_PCREL16: u32 = 3u32;
```

PC relative 16 bit.

### `R_NIOS2_CALL26`

```rust
const R_NIOS2_CALL26: u32 = 4u32;
```

Direct call.

### `R_NIOS2_IMM5`

```rust
const R_NIOS2_IMM5: u32 = 5u32;
```

5 bit constant expression.

### `R_NIOS2_CACHE_OPX`

```rust
const R_NIOS2_CACHE_OPX: u32 = 6u32;
```

5 bit expression, shift 22.

### `R_NIOS2_IMM6`

```rust
const R_NIOS2_IMM6: u32 = 7u32;
```

6 bit constant expression.

### `R_NIOS2_IMM8`

```rust
const R_NIOS2_IMM8: u32 = 8u32;
```

8 bit constant expression.

### `R_NIOS2_HI16`

```rust
const R_NIOS2_HI16: u32 = 9u32;
```

High 16 bit.

### `R_NIOS2_LO16`

```rust
const R_NIOS2_LO16: u32 = 10u32;
```

Low 16 bit.

### `R_NIOS2_HIADJ16`

```rust
const R_NIOS2_HIADJ16: u32 = 11u32;
```

High 16 bit, adjusted.

### `R_NIOS2_BFD_RELOC_32`

```rust
const R_NIOS2_BFD_RELOC_32: u32 = 12u32;
```

32 bit symbol value + addend.

### `R_NIOS2_BFD_RELOC_16`

```rust
const R_NIOS2_BFD_RELOC_16: u32 = 13u32;
```

16 bit symbol value + addend.

### `R_NIOS2_BFD_RELOC_8`

```rust
const R_NIOS2_BFD_RELOC_8: u32 = 14u32;
```

8 bit symbol value + addend.

### `R_NIOS2_GPREL`

```rust
const R_NIOS2_GPREL: u32 = 15u32;
```

16 bit GP pointer offset.

### `R_NIOS2_GNU_VTINHERIT`

```rust
const R_NIOS2_GNU_VTINHERIT: u32 = 16u32;
```

GNU C++ vtable hierarchy.

### `R_NIOS2_GNU_VTENTRY`

```rust
const R_NIOS2_GNU_VTENTRY: u32 = 17u32;
```

GNU C++ vtable member usage.

### `R_NIOS2_UJMP`

```rust
const R_NIOS2_UJMP: u32 = 18u32;
```

Unconditional branch.

### `R_NIOS2_CJMP`

```rust
const R_NIOS2_CJMP: u32 = 19u32;
```

Conditional branch.

### `R_NIOS2_CALLR`

```rust
const R_NIOS2_CALLR: u32 = 20u32;
```

Indirect call through register.

### `R_NIOS2_ALIGN`

```rust
const R_NIOS2_ALIGN: u32 = 21u32;
```

Alignment requirement for linker relaxation.

### `R_NIOS2_GOT16`

```rust
const R_NIOS2_GOT16: u32 = 22u32;
```

16 bit GOT entry.

### `R_NIOS2_CALL16`

```rust
const R_NIOS2_CALL16: u32 = 23u32;
```

16 bit GOT entry for function.

### `R_NIOS2_GOTOFF_LO`

```rust
const R_NIOS2_GOTOFF_LO: u32 = 24u32;
```

%lo of offset to GOT pointer.

### `R_NIOS2_GOTOFF_HA`

```rust
const R_NIOS2_GOTOFF_HA: u32 = 25u32;
```

%hiadj of offset to GOT pointer.

### `R_NIOS2_PCREL_LO`

```rust
const R_NIOS2_PCREL_LO: u32 = 26u32;
```

%lo of PC relative offset.

### `R_NIOS2_PCREL_HA`

```rust
const R_NIOS2_PCREL_HA: u32 = 27u32;
```

%hiadj of PC relative offset.

### `R_NIOS2_TLS_GD16`

```rust
const R_NIOS2_TLS_GD16: u32 = 28u32;
```

16 bit GOT offset for TLS GD.

### `R_NIOS2_TLS_LDM16`

```rust
const R_NIOS2_TLS_LDM16: u32 = 29u32;
```

16 bit GOT offset for TLS LDM.

### `R_NIOS2_TLS_LDO16`

```rust
const R_NIOS2_TLS_LDO16: u32 = 30u32;
```

16 bit module relative offset.

### `R_NIOS2_TLS_IE16`

```rust
const R_NIOS2_TLS_IE16: u32 = 31u32;
```

16 bit GOT offset for TLS IE.

### `R_NIOS2_TLS_LE16`

```rust
const R_NIOS2_TLS_LE16: u32 = 32u32;
```

16 bit LE TP-relative offset.

### `R_NIOS2_TLS_DTPMOD`

```rust
const R_NIOS2_TLS_DTPMOD: u32 = 33u32;
```

Module number.

### `R_NIOS2_TLS_DTPREL`

```rust
const R_NIOS2_TLS_DTPREL: u32 = 34u32;
```

Module-relative offset.

### `R_NIOS2_TLS_TPREL`

```rust
const R_NIOS2_TLS_TPREL: u32 = 35u32;
```

TP-relative offset.

### `R_NIOS2_COPY`

```rust
const R_NIOS2_COPY: u32 = 36u32;
```

Copy symbol at runtime.

### `R_NIOS2_GLOB_DAT`

```rust
const R_NIOS2_GLOB_DAT: u32 = 37u32;
```

Create GOT entry.

### `R_NIOS2_JUMP_SLOT`

```rust
const R_NIOS2_JUMP_SLOT: u32 = 38u32;
```

Create PLT entry.

### `R_NIOS2_RELATIVE`

```rust
const R_NIOS2_RELATIVE: u32 = 39u32;
```

Adjust by program base.

### `R_NIOS2_GOTOFF`

```rust
const R_NIOS2_GOTOFF: u32 = 40u32;
```

16 bit offset to GOT pointer.

### `R_NIOS2_CALL26_NOAT`

```rust
const R_NIOS2_CALL26_NOAT: u32 = 41u32;
```

Direct call in .noat section.

### `R_NIOS2_GOT_LO`

```rust
const R_NIOS2_GOT_LO: u32 = 42u32;
```

%lo() of GOT entry.

### `R_NIOS2_GOT_HA`

```rust
const R_NIOS2_GOT_HA: u32 = 43u32;
```

%hiadj() of GOT entry.

### `R_NIOS2_CALL_LO`

```rust
const R_NIOS2_CALL_LO: u32 = 44u32;
```

%lo() of function GOT entry.

### `R_NIOS2_CALL_HA`

```rust
const R_NIOS2_CALL_HA: u32 = 45u32;
```

%hiadj() of function GOT entry.

### `R_TILEPRO_NONE`

```rust
const R_TILEPRO_NONE: u32 = 0u32;
```

No reloc

### `R_TILEPRO_32`

```rust
const R_TILEPRO_32: u32 = 1u32;
```

Direct 32 bit

### `R_TILEPRO_16`

```rust
const R_TILEPRO_16: u32 = 2u32;
```

Direct 16 bit

### `R_TILEPRO_8`

```rust
const R_TILEPRO_8: u32 = 3u32;
```

Direct 8 bit

### `R_TILEPRO_32_PCREL`

```rust
const R_TILEPRO_32_PCREL: u32 = 4u32;
```

PC relative 32 bit

### `R_TILEPRO_16_PCREL`

```rust
const R_TILEPRO_16_PCREL: u32 = 5u32;
```

PC relative 16 bit

### `R_TILEPRO_8_PCREL`

```rust
const R_TILEPRO_8_PCREL: u32 = 6u32;
```

PC relative 8 bit

### `R_TILEPRO_LO16`

```rust
const R_TILEPRO_LO16: u32 = 7u32;
```

Low 16 bit

### `R_TILEPRO_HI16`

```rust
const R_TILEPRO_HI16: u32 = 8u32;
```

High 16 bit

### `R_TILEPRO_HA16`

```rust
const R_TILEPRO_HA16: u32 = 9u32;
```

High 16 bit, adjusted

### `R_TILEPRO_COPY`

```rust
const R_TILEPRO_COPY: u32 = 10u32;
```

Copy relocation

### `R_TILEPRO_GLOB_DAT`

```rust
const R_TILEPRO_GLOB_DAT: u32 = 11u32;
```

Create GOT entry

### `R_TILEPRO_JMP_SLOT`

```rust
const R_TILEPRO_JMP_SLOT: u32 = 12u32;
```

Create PLT entry

### `R_TILEPRO_RELATIVE`

```rust
const R_TILEPRO_RELATIVE: u32 = 13u32;
```

Adjust by program base

### `R_TILEPRO_BROFF_X1`

```rust
const R_TILEPRO_BROFF_X1: u32 = 14u32;
```

X1 pipe branch offset

### `R_TILEPRO_JOFFLONG_X1`

```rust
const R_TILEPRO_JOFFLONG_X1: u32 = 15u32;
```

X1 pipe jump offset

### `R_TILEPRO_JOFFLONG_X1_PLT`

```rust
const R_TILEPRO_JOFFLONG_X1_PLT: u32 = 16u32;
```

X1 pipe jump offset to PLT

### `R_TILEPRO_IMM8_X0`

```rust
const R_TILEPRO_IMM8_X0: u32 = 17u32;
```

X0 pipe 8-bit

### `R_TILEPRO_IMM8_Y0`

```rust
const R_TILEPRO_IMM8_Y0: u32 = 18u32;
```

Y0 pipe 8-bit

### `R_TILEPRO_IMM8_X1`

```rust
const R_TILEPRO_IMM8_X1: u32 = 19u32;
```

X1 pipe 8-bit

### `R_TILEPRO_IMM8_Y1`

```rust
const R_TILEPRO_IMM8_Y1: u32 = 20u32;
```

Y1 pipe 8-bit

### `R_TILEPRO_MT_IMM15_X1`

```rust
const R_TILEPRO_MT_IMM15_X1: u32 = 21u32;
```

X1 pipe mtspr

### `R_TILEPRO_MF_IMM15_X1`

```rust
const R_TILEPRO_MF_IMM15_X1: u32 = 22u32;
```

X1 pipe mfspr

### `R_TILEPRO_IMM16_X0`

```rust
const R_TILEPRO_IMM16_X0: u32 = 23u32;
```

X0 pipe 16-bit

### `R_TILEPRO_IMM16_X1`

```rust
const R_TILEPRO_IMM16_X1: u32 = 24u32;
```

X1 pipe 16-bit

### `R_TILEPRO_IMM16_X0_LO`

```rust
const R_TILEPRO_IMM16_X0_LO: u32 = 25u32;
```

X0 pipe low 16-bit

### `R_TILEPRO_IMM16_X1_LO`

```rust
const R_TILEPRO_IMM16_X1_LO: u32 = 26u32;
```

X1 pipe low 16-bit

### `R_TILEPRO_IMM16_X0_HI`

```rust
const R_TILEPRO_IMM16_X0_HI: u32 = 27u32;
```

X0 pipe high 16-bit

### `R_TILEPRO_IMM16_X1_HI`

```rust
const R_TILEPRO_IMM16_X1_HI: u32 = 28u32;
```

X1 pipe high 16-bit

### `R_TILEPRO_IMM16_X0_HA`

```rust
const R_TILEPRO_IMM16_X0_HA: u32 = 29u32;
```

X0 pipe high 16-bit, adjusted

### `R_TILEPRO_IMM16_X1_HA`

```rust
const R_TILEPRO_IMM16_X1_HA: u32 = 30u32;
```

X1 pipe high 16-bit, adjusted

### `R_TILEPRO_IMM16_X0_PCREL`

```rust
const R_TILEPRO_IMM16_X0_PCREL: u32 = 31u32;
```

X0 pipe PC relative 16 bit

### `R_TILEPRO_IMM16_X1_PCREL`

```rust
const R_TILEPRO_IMM16_X1_PCREL: u32 = 32u32;
```

X1 pipe PC relative 16 bit

### `R_TILEPRO_IMM16_X0_LO_PCREL`

```rust
const R_TILEPRO_IMM16_X0_LO_PCREL: u32 = 33u32;
```

X0 pipe PC relative low 16 bit

### `R_TILEPRO_IMM16_X1_LO_PCREL`

```rust
const R_TILEPRO_IMM16_X1_LO_PCREL: u32 = 34u32;
```

X1 pipe PC relative low 16 bit

### `R_TILEPRO_IMM16_X0_HI_PCREL`

```rust
const R_TILEPRO_IMM16_X0_HI_PCREL: u32 = 35u32;
```

X0 pipe PC relative high 16 bit

### `R_TILEPRO_IMM16_X1_HI_PCREL`

```rust
const R_TILEPRO_IMM16_X1_HI_PCREL: u32 = 36u32;
```

X1 pipe PC relative high 16 bit

### `R_TILEPRO_IMM16_X0_HA_PCREL`

```rust
const R_TILEPRO_IMM16_X0_HA_PCREL: u32 = 37u32;
```

X0 pipe PC relative ha() 16 bit

### `R_TILEPRO_IMM16_X1_HA_PCREL`

```rust
const R_TILEPRO_IMM16_X1_HA_PCREL: u32 = 38u32;
```

X1 pipe PC relative ha() 16 bit

### `R_TILEPRO_IMM16_X0_GOT`

```rust
const R_TILEPRO_IMM16_X0_GOT: u32 = 39u32;
```

X0 pipe 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT`

```rust
const R_TILEPRO_IMM16_X1_GOT: u32 = 40u32;
```

X1 pipe 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_LO`

```rust
const R_TILEPRO_IMM16_X0_GOT_LO: u32 = 41u32;
```

X0 pipe low 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_LO`

```rust
const R_TILEPRO_IMM16_X1_GOT_LO: u32 = 42u32;
```

X1 pipe low 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_HI`

```rust
const R_TILEPRO_IMM16_X0_GOT_HI: u32 = 43u32;
```

X0 pipe high 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_HI`

```rust
const R_TILEPRO_IMM16_X1_GOT_HI: u32 = 44u32;
```

X1 pipe high 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_HA`

```rust
const R_TILEPRO_IMM16_X0_GOT_HA: u32 = 45u32;
```

X0 pipe ha() 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_HA`

```rust
const R_TILEPRO_IMM16_X1_GOT_HA: u32 = 46u32;
```

X1 pipe ha() 16-bit GOT offset

### `R_TILEPRO_MMSTART_X0`

```rust
const R_TILEPRO_MMSTART_X0: u32 = 47u32;
```

X0 pipe mm "start"

### `R_TILEPRO_MMEND_X0`

```rust
const R_TILEPRO_MMEND_X0: u32 = 48u32;
```

X0 pipe mm "end"

### `R_TILEPRO_MMSTART_X1`

```rust
const R_TILEPRO_MMSTART_X1: u32 = 49u32;
```

X1 pipe mm "start"

### `R_TILEPRO_MMEND_X1`

```rust
const R_TILEPRO_MMEND_X1: u32 = 50u32;
```

X1 pipe mm "end"

### `R_TILEPRO_SHAMT_X0`

```rust
const R_TILEPRO_SHAMT_X0: u32 = 51u32;
```

X0 pipe shift amount

### `R_TILEPRO_SHAMT_X1`

```rust
const R_TILEPRO_SHAMT_X1: u32 = 52u32;
```

X1 pipe shift amount

### `R_TILEPRO_SHAMT_Y0`

```rust
const R_TILEPRO_SHAMT_Y0: u32 = 53u32;
```

Y0 pipe shift amount

### `R_TILEPRO_SHAMT_Y1`

```rust
const R_TILEPRO_SHAMT_Y1: u32 = 54u32;
```

Y1 pipe shift amount

### `R_TILEPRO_DEST_IMM8_X1`

```rust
const R_TILEPRO_DEST_IMM8_X1: u32 = 55u32;
```

X1 pipe destination 8-bit

### `R_TILEPRO_TLS_GD_CALL`

```rust
const R_TILEPRO_TLS_GD_CALL: u32 = 60u32;
```

"jal" for TLS GD

### `R_TILEPRO_IMM8_X0_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_X0_TLS_GD_ADD: u32 = 61u32;
```

X0 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_X1_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_X1_TLS_GD_ADD: u32 = 62u32;
```

X1 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_Y0_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_Y0_TLS_GD_ADD: u32 = 63u32;
```

Y0 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_Y1_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_Y1_TLS_GD_ADD: u32 = 64u32;
```

Y1 pipe "addi" for TLS GD

### `R_TILEPRO_TLS_IE_LOAD`

```rust
const R_TILEPRO_TLS_IE_LOAD: u32 = 65u32;
```

"lw_tls" for TLS IE

### `R_TILEPRO_IMM16_X0_TLS_GD`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD: u32 = 66u32;
```

X0 pipe 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD: u32 = 67u32;
```

X1 pipe 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_LO: u32 = 68u32;
```

X0 pipe low 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_LO: u32 = 69u32;
```

X1 pipe low 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_HI: u32 = 70u32;
```

X0 pipe high 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_HI: u32 = 71u32;
```

X1 pipe high 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_HA: u32 = 72u32;
```

X0 pipe ha() 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_HA: u32 = 73u32;
```

X1 pipe ha() 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_IE`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE: u32 = 74u32;
```

X0 pipe 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE: u32 = 75u32;
```

X1 pipe 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_LO: u32 = 76u32;
```

X0 pipe low 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_LO: u32 = 77u32;
```

X1 pipe low 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_HI: u32 = 78u32;
```

X0 pipe high 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_HI: u32 = 79u32;
```

X1 pipe high 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_HA: u32 = 80u32;
```

X0 pipe ha() 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_HA: u32 = 81u32;
```

X1 pipe ha() 16-bit TLS IE offset

### `R_TILEPRO_TLS_DTPMOD32`

```rust
const R_TILEPRO_TLS_DTPMOD32: u32 = 82u32;
```

ID of module containing symbol

### `R_TILEPRO_TLS_DTPOFF32`

```rust
const R_TILEPRO_TLS_DTPOFF32: u32 = 83u32;
```

Offset in TLS block

### `R_TILEPRO_TLS_TPOFF32`

```rust
const R_TILEPRO_TLS_TPOFF32: u32 = 84u32;
```

Offset in static TLS block

### `R_TILEPRO_IMM16_X0_TLS_LE`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE: u32 = 85u32;
```

X0 pipe 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE: u32 = 86u32;
```

X1 pipe 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_LO: u32 = 87u32;
```

X0 pipe low 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_LO: u32 = 88u32;
```

X1 pipe low 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_HI: u32 = 89u32;
```

X0 pipe high 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_HI: u32 = 90u32;
```

X1 pipe high 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_HA: u32 = 91u32;
```

X0 pipe ha() 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_HA: u32 = 92u32;
```

X1 pipe ha() 16-bit TLS LE offset

### `R_TILEPRO_GNU_VTINHERIT`

```rust
const R_TILEPRO_GNU_VTINHERIT: u32 = 128u32;
```

GNU C++ vtable hierarchy

### `R_TILEPRO_GNU_VTENTRY`

```rust
const R_TILEPRO_GNU_VTENTRY: u32 = 129u32;
```

GNU C++ vtable member usage

### `R_TILEGX_NONE`

```rust
const R_TILEGX_NONE: u32 = 0u32;
```

No reloc

### `R_TILEGX_64`

```rust
const R_TILEGX_64: u32 = 1u32;
```

Direct 64 bit

### `R_TILEGX_32`

```rust
const R_TILEGX_32: u32 = 2u32;
```

Direct 32 bit

### `R_TILEGX_16`

```rust
const R_TILEGX_16: u32 = 3u32;
```

Direct 16 bit

### `R_TILEGX_8`

```rust
const R_TILEGX_8: u32 = 4u32;
```

Direct 8 bit

### `R_TILEGX_64_PCREL`

```rust
const R_TILEGX_64_PCREL: u32 = 5u32;
```

PC relative 64 bit

### `R_TILEGX_32_PCREL`

```rust
const R_TILEGX_32_PCREL: u32 = 6u32;
```

PC relative 32 bit

### `R_TILEGX_16_PCREL`

```rust
const R_TILEGX_16_PCREL: u32 = 7u32;
```

PC relative 16 bit

### `R_TILEGX_8_PCREL`

```rust
const R_TILEGX_8_PCREL: u32 = 8u32;
```

PC relative 8 bit

### `R_TILEGX_HW0`

```rust
const R_TILEGX_HW0: u32 = 9u32;
```

hword 0 16-bit

### `R_TILEGX_HW1`

```rust
const R_TILEGX_HW1: u32 = 10u32;
```

hword 1 16-bit

### `R_TILEGX_HW2`

```rust
const R_TILEGX_HW2: u32 = 11u32;
```

hword 2 16-bit

### `R_TILEGX_HW3`

```rust
const R_TILEGX_HW3: u32 = 12u32;
```

hword 3 16-bit

### `R_TILEGX_HW0_LAST`

```rust
const R_TILEGX_HW0_LAST: u32 = 13u32;
```

last hword 0 16-bit

### `R_TILEGX_HW1_LAST`

```rust
const R_TILEGX_HW1_LAST: u32 = 14u32;
```

last hword 1 16-bit

### `R_TILEGX_HW2_LAST`

```rust
const R_TILEGX_HW2_LAST: u32 = 15u32;
```

last hword 2 16-bit

### `R_TILEGX_COPY`

```rust
const R_TILEGX_COPY: u32 = 16u32;
```

Copy relocation

### `R_TILEGX_GLOB_DAT`

```rust
const R_TILEGX_GLOB_DAT: u32 = 17u32;
```

Create GOT entry

### `R_TILEGX_JMP_SLOT`

```rust
const R_TILEGX_JMP_SLOT: u32 = 18u32;
```

Create PLT entry

### `R_TILEGX_RELATIVE`

```rust
const R_TILEGX_RELATIVE: u32 = 19u32;
```

Adjust by program base

### `R_TILEGX_BROFF_X1`

```rust
const R_TILEGX_BROFF_X1: u32 = 20u32;
```

X1 pipe branch offset

### `R_TILEGX_JUMPOFF_X1`

```rust
const R_TILEGX_JUMPOFF_X1: u32 = 21u32;
```

X1 pipe jump offset

### `R_TILEGX_JUMPOFF_X1_PLT`

```rust
const R_TILEGX_JUMPOFF_X1_PLT: u32 = 22u32;
```

X1 pipe jump offset to PLT

### `R_TILEGX_IMM8_X0`

```rust
const R_TILEGX_IMM8_X0: u32 = 23u32;
```

X0 pipe 8-bit

### `R_TILEGX_IMM8_Y0`

```rust
const R_TILEGX_IMM8_Y0: u32 = 24u32;
```

Y0 pipe 8-bit

### `R_TILEGX_IMM8_X1`

```rust
const R_TILEGX_IMM8_X1: u32 = 25u32;
```

X1 pipe 8-bit

### `R_TILEGX_IMM8_Y1`

```rust
const R_TILEGX_IMM8_Y1: u32 = 26u32;
```

Y1 pipe 8-bit

### `R_TILEGX_DEST_IMM8_X1`

```rust
const R_TILEGX_DEST_IMM8_X1: u32 = 27u32;
```

X1 pipe destination 8-bit

### `R_TILEGX_MT_IMM14_X1`

```rust
const R_TILEGX_MT_IMM14_X1: u32 = 28u32;
```

X1 pipe mtspr

### `R_TILEGX_MF_IMM14_X1`

```rust
const R_TILEGX_MF_IMM14_X1: u32 = 29u32;
```

X1 pipe mfspr

### `R_TILEGX_MMSTART_X0`

```rust
const R_TILEGX_MMSTART_X0: u32 = 30u32;
```

X0 pipe mm "start"

### `R_TILEGX_MMEND_X0`

```rust
const R_TILEGX_MMEND_X0: u32 = 31u32;
```

X0 pipe mm "end"

### `R_TILEGX_SHAMT_X0`

```rust
const R_TILEGX_SHAMT_X0: u32 = 32u32;
```

X0 pipe shift amount

### `R_TILEGX_SHAMT_X1`

```rust
const R_TILEGX_SHAMT_X1: u32 = 33u32;
```

X1 pipe shift amount

### `R_TILEGX_SHAMT_Y0`

```rust
const R_TILEGX_SHAMT_Y0: u32 = 34u32;
```

Y0 pipe shift amount

### `R_TILEGX_SHAMT_Y1`

```rust
const R_TILEGX_SHAMT_Y1: u32 = 35u32;
```

Y1 pipe shift amount

### `R_TILEGX_IMM16_X0_HW0`

```rust
const R_TILEGX_IMM16_X0_HW0: u32 = 36u32;
```

X0 pipe hword 0

### `R_TILEGX_IMM16_X1_HW0`

```rust
const R_TILEGX_IMM16_X1_HW0: u32 = 37u32;
```

X1 pipe hword 0

### `R_TILEGX_IMM16_X0_HW1`

```rust
const R_TILEGX_IMM16_X0_HW1: u32 = 38u32;
```

X0 pipe hword 1

### `R_TILEGX_IMM16_X1_HW1`

```rust
const R_TILEGX_IMM16_X1_HW1: u32 = 39u32;
```

X1 pipe hword 1

### `R_TILEGX_IMM16_X0_HW2`

```rust
const R_TILEGX_IMM16_X0_HW2: u32 = 40u32;
```

X0 pipe hword 2

### `R_TILEGX_IMM16_X1_HW2`

```rust
const R_TILEGX_IMM16_X1_HW2: u32 = 41u32;
```

X1 pipe hword 2

### `R_TILEGX_IMM16_X0_HW3`

```rust
const R_TILEGX_IMM16_X0_HW3: u32 = 42u32;
```

X0 pipe hword 3

### `R_TILEGX_IMM16_X1_HW3`

```rust
const R_TILEGX_IMM16_X1_HW3: u32 = 43u32;
```

X1 pipe hword 3

### `R_TILEGX_IMM16_X0_HW0_LAST`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST: u32 = 44u32;
```

X0 pipe last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST: u32 = 45u32;
```

X1 pipe last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST: u32 = 46u32;
```

X0 pipe last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST: u32 = 47u32;
```

X1 pipe last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST: u32 = 48u32;
```

X0 pipe last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST: u32 = 49u32;
```

X1 pipe last hword 2

### `R_TILEGX_IMM16_X0_HW0_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_PCREL: u32 = 50u32;
```

X0 pipe PC relative hword 0

### `R_TILEGX_IMM16_X1_HW0_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_PCREL: u32 = 51u32;
```

X1 pipe PC relative hword 0

### `R_TILEGX_IMM16_X0_HW1_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_PCREL: u32 = 52u32;
```

X0 pipe PC relative hword 1

### `R_TILEGX_IMM16_X1_HW1_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_PCREL: u32 = 53u32;
```

X1 pipe PC relative hword 1

### `R_TILEGX_IMM16_X0_HW2_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_PCREL: u32 = 54u32;
```

X0 pipe PC relative hword 2

### `R_TILEGX_IMM16_X1_HW2_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_PCREL: u32 = 55u32;
```

X1 pipe PC relative hword 2

### `R_TILEGX_IMM16_X0_HW3_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW3_PCREL: u32 = 56u32;
```

X0 pipe PC relative hword 3

### `R_TILEGX_IMM16_X1_HW3_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW3_PCREL: u32 = 57u32;
```

X1 pipe PC relative hword 3

### `R_TILEGX_IMM16_X0_HW0_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_PCREL: u32 = 58u32;
```

X0 pipe PC-rel last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_PCREL: u32 = 59u32;
```

X1 pipe PC-rel last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_PCREL: u32 = 60u32;
```

X0 pipe PC-rel last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_PCREL: u32 = 61u32;
```

X1 pipe PC-rel last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST_PCREL: u32 = 62u32;
```

X0 pipe PC-rel last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST_PCREL: u32 = 63u32;
```

X1 pipe PC-rel last hword 2

### `R_TILEGX_IMM16_X0_HW0_GOT`

```rust
const R_TILEGX_IMM16_X0_HW0_GOT: u32 = 64u32;
```

X0 pipe hword 0 GOT offset

### `R_TILEGX_IMM16_X1_HW0_GOT`

```rust
const R_TILEGX_IMM16_X1_HW0_GOT: u32 = 65u32;
```

X1 pipe hword 0 GOT offset

### `R_TILEGX_IMM16_X0_HW0_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_PLT_PCREL: u32 = 66u32;
```

X0 pipe PC-rel PLT hword 0

### `R_TILEGX_IMM16_X1_HW0_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_PLT_PCREL: u32 = 67u32;
```

X1 pipe PC-rel PLT hword 0

### `R_TILEGX_IMM16_X0_HW1_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_PLT_PCREL: u32 = 68u32;
```

X0 pipe PC-rel PLT hword 1

### `R_TILEGX_IMM16_X1_HW1_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_PLT_PCREL: u32 = 69u32;
```

X1 pipe PC-rel PLT hword 1

### `R_TILEGX_IMM16_X0_HW2_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_PLT_PCREL: u32 = 70u32;
```

X0 pipe PC-rel PLT hword 2

### `R_TILEGX_IMM16_X1_HW2_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_PLT_PCREL: u32 = 71u32;
```

X1 pipe PC-rel PLT hword 2

### `R_TILEGX_IMM16_X0_HW0_LAST_GOT`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_GOT: u32 = 72u32;
```

X0 pipe last hword 0 GOT offset

### `R_TILEGX_IMM16_X1_HW0_LAST_GOT`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_GOT: u32 = 73u32;
```

X1 pipe last hword 0 GOT offset

### `R_TILEGX_IMM16_X0_HW1_LAST_GOT`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_GOT: u32 = 74u32;
```

X0 pipe last hword 1 GOT offset

### `R_TILEGX_IMM16_X1_HW1_LAST_GOT`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_GOT: u32 = 75u32;
```

X1 pipe last hword 1 GOT offset

### `R_TILEGX_IMM16_X0_HW3_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW3_PLT_PCREL: u32 = 76u32;
```

X0 pipe PC-rel PLT hword 3

### `R_TILEGX_IMM16_X1_HW3_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW3_PLT_PCREL: u32 = 77u32;
```

X1 pipe PC-rel PLT hword 3

### `R_TILEGX_IMM16_X0_HW0_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_GD: u32 = 78u32;
```

X0 pipe hword 0 TLS GD offset

### `R_TILEGX_IMM16_X1_HW0_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_GD: u32 = 79u32;
```

X1 pipe hword 0 TLS GD offset

### `R_TILEGX_IMM16_X0_HW0_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_LE: u32 = 80u32;
```

X0 pipe hword 0 TLS LE offset

### `R_TILEGX_IMM16_X1_HW0_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_LE: u32 = 81u32;
```

X1 pipe hword 0 TLS LE offset

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: u32 = 82u32;
```

X0 pipe last hword 0 LE off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: u32 = 83u32;
```

X1 pipe last hword 0 LE off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: u32 = 84u32;
```

X0 pipe last hword 1 LE off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: u32 = 85u32;
```

X1 pipe last hword 1 LE off

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: u32 = 86u32;
```

X0 pipe last hword 0 GD off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: u32 = 87u32;
```

X1 pipe last hword 0 GD off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: u32 = 88u32;
```

X0 pipe last hword 1 GD off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: u32 = 89u32;
```

X1 pipe last hword 1 GD off

### `R_TILEGX_IMM16_X0_HW0_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_IE: u32 = 92u32;
```

X0 pipe hword 0 TLS IE offset

### `R_TILEGX_IMM16_X1_HW0_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_IE: u32 = 93u32;
```

X1 pipe hword 0 TLS IE offset

### `R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL: u32 = 94u32;
```

X0 pipe PC-rel PLT last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL: u32 = 95u32;
```

X1 pipe PC-rel PLT last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL: u32 = 96u32;
```

X0 pipe PC-rel PLT last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL: u32 = 97u32;
```

X1 pipe PC-rel PLT last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL: u32 = 98u32;
```

X0 pipe PC-rel PLT last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL: u32 = 99u32;
```

X1 pipe PC-rel PLT last hword 2

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: u32 = 100u32;
```

X0 pipe last hword 0 IE off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: u32 = 101u32;
```

X1 pipe last hword 0 IE off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: u32 = 102u32;
```

X0 pipe last hword 1 IE off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: u32 = 103u32;
```

X1 pipe last hword 1 IE off

### `R_TILEGX_TLS_DTPMOD64`

```rust
const R_TILEGX_TLS_DTPMOD64: u32 = 106u32;
```

64-bit ID of symbol's module

### `R_TILEGX_TLS_DTPOFF64`

```rust
const R_TILEGX_TLS_DTPOFF64: u32 = 107u32;
```

64-bit offset in TLS block

### `R_TILEGX_TLS_TPOFF64`

```rust
const R_TILEGX_TLS_TPOFF64: u32 = 108u32;
```

64-bit offset in static TLS block

### `R_TILEGX_TLS_DTPMOD32`

```rust
const R_TILEGX_TLS_DTPMOD32: u32 = 109u32;
```

32-bit ID of symbol's module

### `R_TILEGX_TLS_DTPOFF32`

```rust
const R_TILEGX_TLS_DTPOFF32: u32 = 110u32;
```

32-bit offset in TLS block

### `R_TILEGX_TLS_TPOFF32`

```rust
const R_TILEGX_TLS_TPOFF32: u32 = 111u32;
```

32-bit offset in static TLS block

### `R_TILEGX_TLS_GD_CALL`

```rust
const R_TILEGX_TLS_GD_CALL: u32 = 112u32;
```

"jal" for TLS GD

### `R_TILEGX_IMM8_X0_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_X0_TLS_GD_ADD: u32 = 113u32;
```

X0 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_X1_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_X1_TLS_GD_ADD: u32 = 114u32;
```

X1 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_Y0_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_Y0_TLS_GD_ADD: u32 = 115u32;
```

Y0 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_Y1_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_Y1_TLS_GD_ADD: u32 = 116u32;
```

Y1 pipe "addi" for TLS GD

### `R_TILEGX_TLS_IE_LOAD`

```rust
const R_TILEGX_TLS_IE_LOAD: u32 = 117u32;
```

"ld_tls" for TLS IE

### `R_TILEGX_IMM8_X0_TLS_ADD`

```rust
const R_TILEGX_IMM8_X0_TLS_ADD: u32 = 118u32;
```

X0 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_X1_TLS_ADD`

```rust
const R_TILEGX_IMM8_X1_TLS_ADD: u32 = 119u32;
```

X1 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_Y0_TLS_ADD`

```rust
const R_TILEGX_IMM8_Y0_TLS_ADD: u32 = 120u32;
```

Y0 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_Y1_TLS_ADD`

```rust
const R_TILEGX_IMM8_Y1_TLS_ADD: u32 = 121u32;
```

Y1 pipe "addi" for TLS GD/IE

### `R_TILEGX_GNU_VTINHERIT`

```rust
const R_TILEGX_GNU_VTINHERIT: u32 = 128u32;
```

GNU C++ vtable hierarchy

### `R_TILEGX_GNU_VTENTRY`

```rust
const R_TILEGX_GNU_VTENTRY: u32 = 129u32;
```

GNU C++ vtable member usage

### `EF_RISCV_RVC`

```rust
const EF_RISCV_RVC: u32 = 1u32;
```

### `EF_RISCV_FLOAT_ABI`

```rust
const EF_RISCV_FLOAT_ABI: u32 = 6u32;
```

### `EF_RISCV_FLOAT_ABI_SOFT`

```rust
const EF_RISCV_FLOAT_ABI_SOFT: u32 = 0u32;
```

### `EF_RISCV_FLOAT_ABI_SINGLE`

```rust
const EF_RISCV_FLOAT_ABI_SINGLE: u32 = 2u32;
```

### `EF_RISCV_FLOAT_ABI_DOUBLE`

```rust
const EF_RISCV_FLOAT_ABI_DOUBLE: u32 = 4u32;
```

### `EF_RISCV_FLOAT_ABI_QUAD`

```rust
const EF_RISCV_FLOAT_ABI_QUAD: u32 = 6u32;
```

### `EF_RISCV_RVE`

```rust
const EF_RISCV_RVE: u32 = 8u32;
```

### `EF_RISCV_TSO`

```rust
const EF_RISCV_TSO: u32 = 16u32;
```

### `EF_RISCV_RV64ILP32`

```rust
const EF_RISCV_RV64ILP32: u32 = 32u32;
```

### `SHT_RISCV_ATTRIBUTES`

```rust
const SHT_RISCV_ATTRIBUTES: u32 = 1_879_048_195u32;
```

RISC-V attributes section.

### `R_RISCV_NONE`

```rust
const R_RISCV_NONE: u32 = 0u32;
```

### `R_RISCV_32`

```rust
const R_RISCV_32: u32 = 1u32;
```

### `R_RISCV_64`

```rust
const R_RISCV_64: u32 = 2u32;
```

### `R_RISCV_RELATIVE`

```rust
const R_RISCV_RELATIVE: u32 = 3u32;
```

### `R_RISCV_COPY`

```rust
const R_RISCV_COPY: u32 = 4u32;
```

### `R_RISCV_JUMP_SLOT`

```rust
const R_RISCV_JUMP_SLOT: u32 = 5u32;
```

### `R_RISCV_TLS_DTPMOD32`

```rust
const R_RISCV_TLS_DTPMOD32: u32 = 6u32;
```

### `R_RISCV_TLS_DTPMOD64`

```rust
const R_RISCV_TLS_DTPMOD64: u32 = 7u32;
```

### `R_RISCV_TLS_DTPREL32`

```rust
const R_RISCV_TLS_DTPREL32: u32 = 8u32;
```

### `R_RISCV_TLS_DTPREL64`

```rust
const R_RISCV_TLS_DTPREL64: u32 = 9u32;
```

### `R_RISCV_TLS_TPREL32`

```rust
const R_RISCV_TLS_TPREL32: u32 = 10u32;
```

### `R_RISCV_TLS_TPREL64`

```rust
const R_RISCV_TLS_TPREL64: u32 = 11u32;
```

### `R_RISCV_TLSDESC`

```rust
const R_RISCV_TLSDESC: u32 = 12u32;
```

### `R_RISCV_BRANCH`

```rust
const R_RISCV_BRANCH: u32 = 16u32;
```

### `R_RISCV_JAL`

```rust
const R_RISCV_JAL: u32 = 17u32;
```

### `R_RISCV_CALL`

```rust
const R_RISCV_CALL: u32 = 18u32;
```

### `R_RISCV_CALL_PLT`

```rust
const R_RISCV_CALL_PLT: u32 = 19u32;
```

### `R_RISCV_GOT_HI20`

```rust
const R_RISCV_GOT_HI20: u32 = 20u32;
```

### `R_RISCV_TLS_GOT_HI20`

```rust
const R_RISCV_TLS_GOT_HI20: u32 = 21u32;
```

### `R_RISCV_TLS_GD_HI20`

```rust
const R_RISCV_TLS_GD_HI20: u32 = 22u32;
```

### `R_RISCV_PCREL_HI20`

```rust
const R_RISCV_PCREL_HI20: u32 = 23u32;
```

### `R_RISCV_PCREL_LO12_I`

```rust
const R_RISCV_PCREL_LO12_I: u32 = 24u32;
```

### `R_RISCV_PCREL_LO12_S`

```rust
const R_RISCV_PCREL_LO12_S: u32 = 25u32;
```

### `R_RISCV_HI20`

```rust
const R_RISCV_HI20: u32 = 26u32;
```

### `R_RISCV_LO12_I`

```rust
const R_RISCV_LO12_I: u32 = 27u32;
```

### `R_RISCV_LO12_S`

```rust
const R_RISCV_LO12_S: u32 = 28u32;
```

### `R_RISCV_TPREL_HI20`

```rust
const R_RISCV_TPREL_HI20: u32 = 29u32;
```

### `R_RISCV_TPREL_LO12_I`

```rust
const R_RISCV_TPREL_LO12_I: u32 = 30u32;
```

### `R_RISCV_TPREL_LO12_S`

```rust
const R_RISCV_TPREL_LO12_S: u32 = 31u32;
```

### `R_RISCV_TPREL_ADD`

```rust
const R_RISCV_TPREL_ADD: u32 = 32u32;
```

### `R_RISCV_ADD8`

```rust
const R_RISCV_ADD8: u32 = 33u32;
```

### `R_RISCV_ADD16`

```rust
const R_RISCV_ADD16: u32 = 34u32;
```

### `R_RISCV_ADD32`

```rust
const R_RISCV_ADD32: u32 = 35u32;
```

### `R_RISCV_ADD64`

```rust
const R_RISCV_ADD64: u32 = 36u32;
```

### `R_RISCV_SUB8`

```rust
const R_RISCV_SUB8: u32 = 37u32;
```

### `R_RISCV_SUB16`

```rust
const R_RISCV_SUB16: u32 = 38u32;
```

### `R_RISCV_SUB32`

```rust
const R_RISCV_SUB32: u32 = 39u32;
```

### `R_RISCV_SUB64`

```rust
const R_RISCV_SUB64: u32 = 40u32;
```

### `R_RISCV_GOT32_PCREL`

```rust
const R_RISCV_GOT32_PCREL: u32 = 41u32;
```

### `R_RISCV_ALIGN`

```rust
const R_RISCV_ALIGN: u32 = 43u32;
```

### `R_RISCV_RVC_BRANCH`

```rust
const R_RISCV_RVC_BRANCH: u32 = 44u32;
```

### `R_RISCV_RVC_JUMP`

```rust
const R_RISCV_RVC_JUMP: u32 = 45u32;
```

### `R_RISCV_RVC_LUI`

```rust
const R_RISCV_RVC_LUI: u32 = 46u32;
```

### `R_RISCV_GPREL_I`

```rust
const R_RISCV_GPREL_I: u32 = 47u32;
```

### `R_RISCV_GPREL_S`

```rust
const R_RISCV_GPREL_S: u32 = 48u32;
```

### `R_RISCV_TPREL_I`

```rust
const R_RISCV_TPREL_I: u32 = 49u32;
```

### `R_RISCV_TPREL_S`

```rust
const R_RISCV_TPREL_S: u32 = 50u32;
```

### `R_RISCV_RELAX`

```rust
const R_RISCV_RELAX: u32 = 51u32;
```

### `R_RISCV_SUB6`

```rust
const R_RISCV_SUB6: u32 = 52u32;
```

### `R_RISCV_SET6`

```rust
const R_RISCV_SET6: u32 = 53u32;
```

### `R_RISCV_SET8`

```rust
const R_RISCV_SET8: u32 = 54u32;
```

### `R_RISCV_SET16`

```rust
const R_RISCV_SET16: u32 = 55u32;
```

### `R_RISCV_SET32`

```rust
const R_RISCV_SET32: u32 = 56u32;
```

### `R_RISCV_32_PCREL`

```rust
const R_RISCV_32_PCREL: u32 = 57u32;
```

### `R_RISCV_IRELATIVE`

```rust
const R_RISCV_IRELATIVE: u32 = 58u32;
```

### `R_RISCV_PLT32`

```rust
const R_RISCV_PLT32: u32 = 59u32;
```

### `R_RISCV_SET_ULEB128`

```rust
const R_RISCV_SET_ULEB128: u32 = 60u32;
```

### `R_RISCV_SUB_ULEB128`

```rust
const R_RISCV_SUB_ULEB128: u32 = 61u32;
```

### `R_RISCV_TLSDESC_HI20`

```rust
const R_RISCV_TLSDESC_HI20: u32 = 62u32;
```

### `R_RISCV_TLSDESC_LOAD_LO12`

```rust
const R_RISCV_TLSDESC_LOAD_LO12: u32 = 63u32;
```

### `R_RISCV_TLSDESC_ADD_LO12`

```rust
const R_RISCV_TLSDESC_ADD_LO12: u32 = 64u32;
```

### `R_RISCV_TLSDESC_CALL`

```rust
const R_RISCV_TLSDESC_CALL: u32 = 65u32;
```

### `R_BPF_NONE`

```rust
const R_BPF_NONE: u32 = 0u32;
```

No reloc

### `R_BPF_64_64`

```rust
const R_BPF_64_64: u32 = 1u32;
```

### `R_BPF_64_32`

```rust
const R_BPF_64_32: u32 = 10u32;
```

### `R_SBF_NONE`

```rust
const R_SBF_NONE: u32 = 0u32;
```

No reloc

### `R_SBF_64_64`

```rust
const R_SBF_64_64: u32 = 1u32;
```

### `R_SBF_64_32`

```rust
const R_SBF_64_32: u32 = 10u32;
```

### `R_METAG_HIADDR16`

```rust
const R_METAG_HIADDR16: u32 = 0u32;
```

### `R_METAG_LOADDR16`

```rust
const R_METAG_LOADDR16: u32 = 1u32;
```

### `R_METAG_ADDR32`

```rust
const R_METAG_ADDR32: u32 = 2u32;
```

32bit absolute address

### `R_METAG_NONE`

```rust
const R_METAG_NONE: u32 = 3u32;
```

No reloc

### `R_METAG_RELBRANCH`

```rust
const R_METAG_RELBRANCH: u32 = 4u32;
```

### `R_METAG_GETSETOFF`

```rust
const R_METAG_GETSETOFF: u32 = 5u32;
```

### `R_METAG_REG32OP1`

```rust
const R_METAG_REG32OP1: u32 = 6u32;
```

### `R_METAG_REG32OP2`

```rust
const R_METAG_REG32OP2: u32 = 7u32;
```

### `R_METAG_REG32OP3`

```rust
const R_METAG_REG32OP3: u32 = 8u32;
```

### `R_METAG_REG16OP1`

```rust
const R_METAG_REG16OP1: u32 = 9u32;
```

### `R_METAG_REG16OP2`

```rust
const R_METAG_REG16OP2: u32 = 10u32;
```

### `R_METAG_REG16OP3`

```rust
const R_METAG_REG16OP3: u32 = 11u32;
```

### `R_METAG_REG32OP4`

```rust
const R_METAG_REG32OP4: u32 = 12u32;
```

### `R_METAG_HIOG`

```rust
const R_METAG_HIOG: u32 = 13u32;
```

### `R_METAG_LOOG`

```rust
const R_METAG_LOOG: u32 = 14u32;
```

### `R_METAG_REL8`

```rust
const R_METAG_REL8: u32 = 15u32;
```

### `R_METAG_REL16`

```rust
const R_METAG_REL16: u32 = 16u32;
```

### `R_METAG_GNU_VTINHERIT`

```rust
const R_METAG_GNU_VTINHERIT: u32 = 30u32;
```

### `R_METAG_GNU_VTENTRY`

```rust
const R_METAG_GNU_VTENTRY: u32 = 31u32;
```

### `R_METAG_HI16_GOTOFF`

```rust
const R_METAG_HI16_GOTOFF: u32 = 32u32;
```

### `R_METAG_LO16_GOTOFF`

```rust
const R_METAG_LO16_GOTOFF: u32 = 33u32;
```

### `R_METAG_GETSET_GOTOFF`

```rust
const R_METAG_GETSET_GOTOFF: u32 = 34u32;
```

### `R_METAG_GETSET_GOT`

```rust
const R_METAG_GETSET_GOT: u32 = 35u32;
```

### `R_METAG_HI16_GOTPC`

```rust
const R_METAG_HI16_GOTPC: u32 = 36u32;
```

### `R_METAG_LO16_GOTPC`

```rust
const R_METAG_LO16_GOTPC: u32 = 37u32;
```

### `R_METAG_HI16_PLT`

```rust
const R_METAG_HI16_PLT: u32 = 38u32;
```

### `R_METAG_LO16_PLT`

```rust
const R_METAG_LO16_PLT: u32 = 39u32;
```

### `R_METAG_RELBRANCH_PLT`

```rust
const R_METAG_RELBRANCH_PLT: u32 = 40u32;
```

### `R_METAG_GOTOFF`

```rust
const R_METAG_GOTOFF: u32 = 41u32;
```

### `R_METAG_PLT`

```rust
const R_METAG_PLT: u32 = 42u32;
```

### `R_METAG_COPY`

```rust
const R_METAG_COPY: u32 = 43u32;
```

### `R_METAG_JMP_SLOT`

```rust
const R_METAG_JMP_SLOT: u32 = 44u32;
```

### `R_METAG_RELATIVE`

```rust
const R_METAG_RELATIVE: u32 = 45u32;
```

### `R_METAG_GLOB_DAT`

```rust
const R_METAG_GLOB_DAT: u32 = 46u32;
```

### `R_METAG_TLS_GD`

```rust
const R_METAG_TLS_GD: u32 = 47u32;
```

### `R_METAG_TLS_LDM`

```rust
const R_METAG_TLS_LDM: u32 = 48u32;
```

### `R_METAG_TLS_LDO_HI16`

```rust
const R_METAG_TLS_LDO_HI16: u32 = 49u32;
```

### `R_METAG_TLS_LDO_LO16`

```rust
const R_METAG_TLS_LDO_LO16: u32 = 50u32;
```

### `R_METAG_TLS_LDO`

```rust
const R_METAG_TLS_LDO: u32 = 51u32;
```

### `R_METAG_TLS_IE`

```rust
const R_METAG_TLS_IE: u32 = 52u32;
```

### `R_METAG_TLS_IENONPIC`

```rust
const R_METAG_TLS_IENONPIC: u32 = 53u32;
```

### `R_METAG_TLS_IENONPIC_HI16`

```rust
const R_METAG_TLS_IENONPIC_HI16: u32 = 54u32;
```

### `R_METAG_TLS_IENONPIC_LO16`

```rust
const R_METAG_TLS_IENONPIC_LO16: u32 = 55u32;
```

### `R_METAG_TLS_TPOFF`

```rust
const R_METAG_TLS_TPOFF: u32 = 56u32;
```

### `R_METAG_TLS_DTPMOD`

```rust
const R_METAG_TLS_DTPMOD: u32 = 57u32;
```

### `R_METAG_TLS_DTPOFF`

```rust
const R_METAG_TLS_DTPOFF: u32 = 58u32;
```

### `R_METAG_TLS_LE`

```rust
const R_METAG_TLS_LE: u32 = 59u32;
```

### `R_METAG_TLS_LE_HI16`

```rust
const R_METAG_TLS_LE_HI16: u32 = 60u32;
```

### `R_METAG_TLS_LE_LO16`

```rust
const R_METAG_TLS_LE_LO16: u32 = 61u32;
```

### `R_NDS32_NONE`

```rust
const R_NDS32_NONE: u32 = 0u32;
```

### `R_NDS32_32_RELA`

```rust
const R_NDS32_32_RELA: u32 = 20u32;
```

### `R_NDS32_COPY`

```rust
const R_NDS32_COPY: u32 = 39u32;
```

### `R_NDS32_GLOB_DAT`

```rust
const R_NDS32_GLOB_DAT: u32 = 40u32;
```

### `R_NDS32_JMP_SLOT`

```rust
const R_NDS32_JMP_SLOT: u32 = 41u32;
```

### `R_NDS32_RELATIVE`

```rust
const R_NDS32_RELATIVE: u32 = 42u32;
```

### `R_NDS32_TLS_TPOFF`

```rust
const R_NDS32_TLS_TPOFF: u32 = 102u32;
```

### `R_NDS32_TLS_DESC`

```rust
const R_NDS32_TLS_DESC: u32 = 119u32;
```

### `EF_LARCH_ABI_MODIFIER_MASK`

```rust
const EF_LARCH_ABI_MODIFIER_MASK: u32 = 7u32;
```

Additional properties of the base ABI type, including the FP calling
convention.

### `EF_LARCH_ABI_SOFT_FLOAT`

```rust
const EF_LARCH_ABI_SOFT_FLOAT: u32 = 1u32;
```

Uses GPRs and the stack for parameter passing

### `EF_LARCH_ABI_SINGLE_FLOAT`

```rust
const EF_LARCH_ABI_SINGLE_FLOAT: u32 = 2u32;
```

Uses GPRs, 32-bit FPRs and the stack for parameter passing

### `EF_LARCH_ABI_DOUBLE_FLOAT`

```rust
const EF_LARCH_ABI_DOUBLE_FLOAT: u32 = 3u32;
```

Uses GPRs, 64-bit FPRs and the stack for parameter passing

### `EF_LARCH_OBJABI_V1`

```rust
const EF_LARCH_OBJABI_V1: u32 = 64u32;
```

Uses relocation types directly writing to immediate slots

### `R_LARCH_NONE`

```rust
const R_LARCH_NONE: u32 = 0u32;
```

No reloc

### `R_LARCH_32`

```rust
const R_LARCH_32: u32 = 1u32;
```

Runtime address resolving

### `R_LARCH_64`

```rust
const R_LARCH_64: u32 = 2u32;
```

Runtime address resolving

### `R_LARCH_RELATIVE`

```rust
const R_LARCH_RELATIVE: u32 = 3u32;
```

Runtime fixup for load-address

### `R_LARCH_COPY`

```rust
const R_LARCH_COPY: u32 = 4u32;
```

Runtime memory copy in executable

### `R_LARCH_JUMP_SLOT`

```rust
const R_LARCH_JUMP_SLOT: u32 = 5u32;
```

Runtime PLT supporting

### `R_LARCH_TLS_DTPMOD32`

```rust
const R_LARCH_TLS_DTPMOD32: u32 = 6u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPMOD64`

```rust
const R_LARCH_TLS_DTPMOD64: u32 = 7u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPREL32`

```rust
const R_LARCH_TLS_DTPREL32: u32 = 8u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPREL64`

```rust
const R_LARCH_TLS_DTPREL64: u32 = 9u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_TPREL32`

```rust
const R_LARCH_TLS_TPREL32: u32 = 10u32;
```

Runtime relocation for TLE-IE

### `R_LARCH_TLS_TPREL64`

```rust
const R_LARCH_TLS_TPREL64: u32 = 11u32;
```

Runtime relocation for TLE-IE

### `R_LARCH_IRELATIVE`

```rust
const R_LARCH_IRELATIVE: u32 = 12u32;
```

Runtime local indirect function resolving

### `R_LARCH_MARK_LA`

```rust
const R_LARCH_MARK_LA: u32 = 20u32;
```

Mark la.abs: load absolute address for static link.

### `R_LARCH_MARK_PCREL`

```rust
const R_LARCH_MARK_PCREL: u32 = 21u32;
```

Mark external label branch: access PC relative address for static link.

### `R_LARCH_SOP_PUSH_PCREL`

```rust
const R_LARCH_SOP_PUSH_PCREL: u32 = 22u32;
```

Push PC-relative offset

### `R_LARCH_SOP_PUSH_ABSOLUTE`

```rust
const R_LARCH_SOP_PUSH_ABSOLUTE: u32 = 23u32;
```

Push constant or absolute address

### `R_LARCH_SOP_PUSH_DUP`

```rust
const R_LARCH_SOP_PUSH_DUP: u32 = 24u32;
```

Duplicate stack top

### `R_LARCH_SOP_PUSH_GPREL`

```rust
const R_LARCH_SOP_PUSH_GPREL: u32 = 25u32;
```

Push for access GOT entry

### `R_LARCH_SOP_PUSH_TLS_TPREL`

```rust
const R_LARCH_SOP_PUSH_TLS_TPREL: u32 = 26u32;
```

Push for TLS-LE

### `R_LARCH_SOP_PUSH_TLS_GOT`

```rust
const R_LARCH_SOP_PUSH_TLS_GOT: u32 = 27u32;
```

Push for TLS-IE

### `R_LARCH_SOP_PUSH_TLS_GD`

```rust
const R_LARCH_SOP_PUSH_TLS_GD: u32 = 28u32;
```

Push for TLS-GD

### `R_LARCH_SOP_PUSH_PLT_PCREL`

```rust
const R_LARCH_SOP_PUSH_PLT_PCREL: u32 = 29u32;
```

Push for external function calling

### `R_LARCH_SOP_ASSERT`

```rust
const R_LARCH_SOP_ASSERT: u32 = 30u32;
```

Assert stack top

### `R_LARCH_SOP_NOT`

```rust
const R_LARCH_SOP_NOT: u32 = 31u32;
```

Stack top logical not (unary)

### `R_LARCH_SOP_SUB`

```rust
const R_LARCH_SOP_SUB: u32 = 32u32;
```

Stack top subtraction (binary)

### `R_LARCH_SOP_SL`

```rust
const R_LARCH_SOP_SL: u32 = 33u32;
```

Stack top left shift (binary)

### `R_LARCH_SOP_SR`

```rust
const R_LARCH_SOP_SR: u32 = 34u32;
```

Stack top right shift (binary)

### `R_LARCH_SOP_ADD`

```rust
const R_LARCH_SOP_ADD: u32 = 35u32;
```

Stack top addition (binary)

### `R_LARCH_SOP_AND`

```rust
const R_LARCH_SOP_AND: u32 = 36u32;
```

Stack top bitwise and (binary)

### `R_LARCH_SOP_IF_ELSE`

```rust
const R_LARCH_SOP_IF_ELSE: u32 = 37u32;
```

Stack top selection (tertiary)

### `R_LARCH_SOP_POP_32_S_10_5`

```rust
const R_LARCH_SOP_POP_32_S_10_5: u32 = 38u32;
```

Pop stack top to fill 5-bit signed immediate operand

### `R_LARCH_SOP_POP_32_U_10_12`

```rust
const R_LARCH_SOP_POP_32_U_10_12: u32 = 39u32;
```

Pop stack top to fill 12-bit unsigned immediate operand

### `R_LARCH_SOP_POP_32_S_10_12`

```rust
const R_LARCH_SOP_POP_32_S_10_12: u32 = 40u32;
```

Pop stack top to fill 12-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_10_16`

```rust
const R_LARCH_SOP_POP_32_S_10_16: u32 = 41u32;
```

Pop stack top to fill 16-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_10_16_S2: u32 = 42u32;
```

Pop stack top to fill 18-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_S_5_20`

```rust
const R_LARCH_SOP_POP_32_S_5_20: u32 = 43u32;
```

Pop stack top to fill 20-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_0_5_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_0_5_10_16_S2: u32 = 44u32;
```

Pop stack top to fill 23-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_S_0_10_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_0_10_10_16_S2: u32 = 45u32;
```

Pop stack top to fill 28-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_U`

```rust
const R_LARCH_SOP_POP_32_U: u32 = 46u32;
```

Pop stack top to fill an instruction

### `R_LARCH_ADD8`

```rust
const R_LARCH_ADD8: u32 = 47u32;
```

8-bit in-place addition

### `R_LARCH_ADD16`

```rust
const R_LARCH_ADD16: u32 = 48u32;
```

16-bit in-place addition

### `R_LARCH_ADD24`

```rust
const R_LARCH_ADD24: u32 = 49u32;
```

24-bit in-place addition

### `R_LARCH_ADD32`

```rust
const R_LARCH_ADD32: u32 = 50u32;
```

32-bit in-place addition

### `R_LARCH_ADD64`

```rust
const R_LARCH_ADD64: u32 = 51u32;
```

64-bit in-place addition

### `R_LARCH_SUB8`

```rust
const R_LARCH_SUB8: u32 = 52u32;
```

8-bit in-place subtraction

### `R_LARCH_SUB16`

```rust
const R_LARCH_SUB16: u32 = 53u32;
```

16-bit in-place subtraction

### `R_LARCH_SUB24`

```rust
const R_LARCH_SUB24: u32 = 54u32;
```

24-bit in-place subtraction

### `R_LARCH_SUB32`

```rust
const R_LARCH_SUB32: u32 = 55u32;
```

32-bit in-place subtraction

### `R_LARCH_SUB64`

```rust
const R_LARCH_SUB64: u32 = 56u32;
```

64-bit in-place subtraction

### `R_LARCH_GNU_VTINHERIT`

```rust
const R_LARCH_GNU_VTINHERIT: u32 = 57u32;
```

GNU C++ vtable hierarchy

### `R_LARCH_GNU_VTENTRY`

```rust
const R_LARCH_GNU_VTENTRY: u32 = 58u32;
```

GNU C++ vtable member usage

### `R_LARCH_B16`

```rust
const R_LARCH_B16: u32 = 64u32;
```

18-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_B21`

```rust
const R_LARCH_B21: u32 = 65u32;
```

23-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_B26`

```rust
const R_LARCH_B26: u32 = 66u32;
```

28-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_ABS_HI20`

```rust
const R_LARCH_ABS_HI20: u32 = 67u32;
```

12..=31 bits of 32/64-bit absolute address

### `R_LARCH_ABS_LO12`

```rust
const R_LARCH_ABS_LO12: u32 = 68u32;
```

0..=11 bits of 32/64-bit absolute address

### `R_LARCH_ABS64_LO20`

```rust
const R_LARCH_ABS64_LO20: u32 = 69u32;
```

32..=51 bits of 64-bit absolute address

### `R_LARCH_ABS64_HI12`

```rust
const R_LARCH_ABS64_HI12: u32 = 70u32;
```

52..=63 bits of 64-bit absolute address

### `R_LARCH_PCALA_HI20`

```rust
const R_LARCH_PCALA_HI20: u32 = 71u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(S + A + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for `S + A` as `PC + offs` (`offs`
is sign-extended to VA bits).

### `R_LARCH_PCALA_LO12`

```rust
const R_LARCH_PCALA_LO12: u32 = 72u32;
```

Same as R_LARCH_ABS_LO12.  0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_PCALA64_LO20`

```rust
const R_LARCH_PCALA64_LO20: u32 = 73u32;
```

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_PCALA64_HI12`

```rust
const R_LARCH_PCALA64_HI12: u32 = 74u32;
```

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_GOT_PC_HI20`

```rust
const R_LARCH_GOT_PC_HI20: u32 = 75u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + G + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the GOT entry at `GP + G` as
`PC + offs` (`offs` is sign-extended to VA bits).

### `R_LARCH_GOT_PC_LO12`

```rust
const R_LARCH_GOT_PC_LO12: u32 = 76u32;
```

0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT64_PC_LO20`

```rust
const R_LARCH_GOT64_PC_LO20: u32 = 77u32;
```

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT64_PC_HI12`

```rust
const R_LARCH_GOT64_PC_HI12: u32 = 78u32;
```

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT_HI20`

```rust
const R_LARCH_GOT_HI20: u32 = 79u32;
```

12..=31 bits of 32/64-bit GOT entry absolute address

### `R_LARCH_GOT_LO12`

```rust
const R_LARCH_GOT_LO12: u32 = 80u32;
```

0..=11 bits of 32/64-bit GOT entry absolute address

### `R_LARCH_GOT64_LO20`

```rust
const R_LARCH_GOT64_LO20: u32 = 81u32;
```

32..=51 bits of 64-bit GOT entry absolute address

### `R_LARCH_GOT64_HI12`

```rust
const R_LARCH_GOT64_HI12: u32 = 82u32;
```

52..=63 bits of 64-bit GOT entry absolute address

### `R_LARCH_TLS_LE_HI20`

```rust
const R_LARCH_TLS_LE_HI20: u32 = 83u32;
```

12..=31 bits of TLS LE 32/64-bit offset from thread pointer

### `R_LARCH_TLS_LE_LO12`

```rust
const R_LARCH_TLS_LE_LO12: u32 = 84u32;
```

0..=11 bits of TLS LE 32/64-bit offset from thread pointer

### `R_LARCH_TLS_LE64_LO20`

```rust
const R_LARCH_TLS_LE64_LO20: u32 = 85u32;
```

32..=51 bits of TLS LE 64-bit offset from thread pointer

### `R_LARCH_TLS_LE64_HI12`

```rust
const R_LARCH_TLS_LE64_HI12: u32 = 86u32;
```

52..=63 bits of TLS LE 64-bit offset from thread pointer

### `R_LARCH_TLS_IE_PC_HI20`

```rust
const R_LARCH_TLS_IE_PC_HI20: u32 = 87u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + IE + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the TLS IE GOT entry at
`GP + IE` as `PC + offs` (`offs` is sign-extended to VA bits).

### `R_LARCH_TLS_IE_PC_LO12`

```rust
const R_LARCH_TLS_IE_PC_LO12: u32 = 88u32;
```

0..=12 bits of the 32/64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE64_PC_LO20`

```rust
const R_LARCH_TLS_IE64_PC_LO20: u32 = 89u32;
```

32..=51 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE64_PC_HI12`

```rust
const R_LARCH_TLS_IE64_PC_HI12: u32 = 90u32;
```

52..=63 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE_HI20`

```rust
const R_LARCH_TLS_IE_HI20: u32 = 91u32;
```

12..=31 bits of TLS IE GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_IE_LO12`

```rust
const R_LARCH_TLS_IE_LO12: u32 = 92u32;
```

0..=11 bits of TLS IE GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_IE64_LO20`

```rust
const R_LARCH_TLS_IE64_LO20: u32 = 93u32;
```

32..=51 bits of TLS IE GOT entry 64-bit absolute address

### `R_LARCH_TLS_IE64_HI12`

```rust
const R_LARCH_TLS_IE64_HI12: u32 = 94u32;
```

51..=63 bits of TLS IE GOT entry 64-bit absolute address

### `R_LARCH_TLS_LD_PC_HI20`

```rust
const R_LARCH_TLS_LD_PC_HI20: u32 = 95u32;
```

12..=31 bits of the offset from `PC` to `GP + GD + 0x800`, where
`GP + GD` is a TLS LD GOT entry

### `R_LARCH_TLS_LD_HI20`

```rust
const R_LARCH_TLS_LD_HI20: u32 = 96u32;
```

12..=31 bits of TLS LD GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_GD_PC_HI20`

```rust
const R_LARCH_TLS_GD_PC_HI20: u32 = 97u32;
```

12..=31 bits of the 32/64-bit PC-relative offset to the PC-relative
anchor for the TLE GD GOT entry.

### `R_LARCH_TLS_GD_HI20`

```rust
const R_LARCH_TLS_GD_HI20: u32 = 98u32;
```

12..=31 bits of TLS GD GOT entry 32/64-bit absolute address

### `R_LARCH_32_PCREL`

```rust
const R_LARCH_32_PCREL: u32 = 99u32;
```

32-bit PC relative

### `R_LARCH_RELAX`

```rust
const R_LARCH_RELAX: u32 = 100u32;
```

Paired with a normal relocation at the same address to indicate the
instruction can be relaxed

### `R_LARCH_DELETE`

```rust
const R_LARCH_DELETE: u32 = 101u32;
```

Reserved

### `R_LARCH_ALIGN`

```rust
const R_LARCH_ALIGN: u32 = 102u32;
```

Delete some bytes to ensure the instruction at PC + A aligned to
`A.next_power_of_two()`-byte boundary

### `R_LARCH_PCREL20_S2`

```rust
const R_LARCH_PCREL20_S2: u32 = 103u32;
```

22-bit PC-relative offset with two trailing zeros

### `R_LARCH_CFA`

```rust
const R_LARCH_CFA: u32 = 104u32;
```

Reserved

### `R_LARCH_ADD6`

```rust
const R_LARCH_ADD6: u32 = 105u32;
```

6-bit in-place addition

### `R_LARCH_SUB6`

```rust
const R_LARCH_SUB6: u32 = 106u32;
```

6-bit in-place subtraction

### `R_LARCH_ADD_ULEB128`

```rust
const R_LARCH_ADD_ULEB128: u32 = 107u32;
```

LEB128 in-place addition

### `R_LARCH_SUB_ULEB128`

```rust
const R_LARCH_SUB_ULEB128: u32 = 108u32;
```

LEB128 in-place subtraction

### `R_LARCH_64_PCREL`

```rust
const R_LARCH_64_PCREL: u32 = 109u32;
```

64-bit PC relative

### `R_LARCH_CALL36`

```rust
const R_LARCH_CALL36: u32 = 110u32;
```

18..=37 bits of `S + A - PC` into the `pcaddu18i` instruction at `PC`,
and 2..=17 bits of `S + A - PC` into the `jirl` instruction at `PC + 4`

### `R_LARCH_TLS_DESC_PC_HI20`

```rust
const R_LARCH_TLS_DESC_PC_HI20: u32 = 111u32;
```

12..=31 bits of 32/64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC_PC_LO12`

```rust
const R_LARCH_TLS_DESC_PC_LO12: u32 = 112u32;
```

0..=11 bits of 32/64-bit TLS DESC GOT entry address

### `R_LARCH_TLS_DESC64_PC_LO20`

```rust
const R_LARCH_TLS_DESC64_PC_LO20: u32 = 113u32;
```

32..=51 bits of 64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC64_PC_HI12`

```rust
const R_LARCH_TLS_DESC64_PC_HI12: u32 = 114u32;
```

52..=63 bits of 64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC_HI20`

```rust
const R_LARCH_TLS_DESC_HI20: u32 = 115u32;
```

12..=31 bits of 32/64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC_LO12`

```rust
const R_LARCH_TLS_DESC_LO12: u32 = 116u32;
```

0..=11 bits of 32/64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC64_LO20`

```rust
const R_LARCH_TLS_DESC64_LO20: u32 = 117u32;
```

32..=51 bits of 64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC64_HI12`

```rust
const R_LARCH_TLS_DESC64_HI12: u32 = 118u32;
```

52..=63 bits of 64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC_LD`

```rust
const R_LARCH_TLS_DESC_LD: u32 = 119u32;
```

Used on ld.{w,d} for TLS DESC to get the resolve function address
from GOT entry

### `R_LARCH_TLS_DESC_CALL`

```rust
const R_LARCH_TLS_DESC_CALL: u32 = 120u32;
```

Used on jirl for TLS DESC to call the resolve function

### `R_LARCH_TLS_LE_HI20_R`

```rust
const R_LARCH_TLS_LE_HI20_R: u32 = 121u32;
```

12..=31 bits of TLS LE 32/64-bit offset from TP register, can be relaxed

### `R_LARCH_TLS_LE_ADD_R`

```rust
const R_LARCH_TLS_LE_ADD_R: u32 = 122u32;
```

TLS LE thread pointer usage, can be relaxed

### `R_LARCH_TLS_LE_LO12_R`

```rust
const R_LARCH_TLS_LE_LO12_R: u32 = 123u32;
```

0..=11 bits of TLS LE 32/64-bit offset from TP register, sign-extended,
can be relaxed.

### `R_LARCH_TLS_LD_PCREL20_S2`

```rust
const R_LARCH_TLS_LD_PCREL20_S2: u32 = 124u32;
```

22-bit PC-relative offset to TLS LD GOT entry

### `R_LARCH_TLS_GD_PCREL20_S2`

```rust
const R_LARCH_TLS_GD_PCREL20_S2: u32 = 125u32;
```

22-bit PC-relative offset to TLS GD GOT entry

### `R_LARCH_TLS_DESC_PCREL20_S2`

```rust
const R_LARCH_TLS_DESC_PCREL20_S2: u32 = 126u32;
```

22-bit PC-relative offset to TLS DESC GOT entry

### `R_XTENSA_NONE`

```rust
const R_XTENSA_NONE: u32 = 0u32;
```

### `R_XTENSA_32`

```rust
const R_XTENSA_32: u32 = 1u32;
```

### `R_XTENSA_RTLD`

```rust
const R_XTENSA_RTLD: u32 = 2u32;
```

### `R_XTENSA_GLOB_DAT`

```rust
const R_XTENSA_GLOB_DAT: u32 = 3u32;
```

### `R_XTENSA_JMP_SLOT`

```rust
const R_XTENSA_JMP_SLOT: u32 = 4u32;
```

### `R_XTENSA_RELATIVE`

```rust
const R_XTENSA_RELATIVE: u32 = 5u32;
```

### `R_XTENSA_PLT`

```rust
const R_XTENSA_PLT: u32 = 6u32;
```

### `R_XTENSA_OP0`

```rust
const R_XTENSA_OP0: u32 = 8u32;
```

### `R_XTENSA_OP1`

```rust
const R_XTENSA_OP1: u32 = 9u32;
```

### `R_XTENSA_OP2`

```rust
const R_XTENSA_OP2: u32 = 10u32;
```

### `R_XTENSA_ASM_EXPAND`

```rust
const R_XTENSA_ASM_EXPAND: u32 = 11u32;
```

### `R_XTENSA_ASM_SIMPLIFY`

```rust
const R_XTENSA_ASM_SIMPLIFY: u32 = 12u32;
```

### `R_XTENSA_32_PCREL`

```rust
const R_XTENSA_32_PCREL: u32 = 14u32;
```

### `R_XTENSA_GNU_VTINHERIT`

```rust
const R_XTENSA_GNU_VTINHERIT: u32 = 15u32;
```

### `R_XTENSA_GNU_VTENTRY`

```rust
const R_XTENSA_GNU_VTENTRY: u32 = 16u32;
```

### `R_XTENSA_DIFF8`

```rust
const R_XTENSA_DIFF8: u32 = 17u32;
```

### `R_XTENSA_DIFF16`

```rust
const R_XTENSA_DIFF16: u32 = 18u32;
```

### `R_XTENSA_DIFF32`

```rust
const R_XTENSA_DIFF32: u32 = 19u32;
```

### `R_XTENSA_SLOT0_OP`

```rust
const R_XTENSA_SLOT0_OP: u32 = 20u32;
```

### `R_XTENSA_SLOT1_OP`

```rust
const R_XTENSA_SLOT1_OP: u32 = 21u32;
```

### `R_XTENSA_SLOT2_OP`

```rust
const R_XTENSA_SLOT2_OP: u32 = 22u32;
```

### `R_XTENSA_SLOT3_OP`

```rust
const R_XTENSA_SLOT3_OP: u32 = 23u32;
```

### `R_XTENSA_SLOT4_OP`

```rust
const R_XTENSA_SLOT4_OP: u32 = 24u32;
```

### `R_XTENSA_SLOT5_OP`

```rust
const R_XTENSA_SLOT5_OP: u32 = 25u32;
```

### `R_XTENSA_SLOT6_OP`

```rust
const R_XTENSA_SLOT6_OP: u32 = 26u32;
```

### `R_XTENSA_SLOT7_OP`

```rust
const R_XTENSA_SLOT7_OP: u32 = 27u32;
```

### `R_XTENSA_SLOT8_OP`

```rust
const R_XTENSA_SLOT8_OP: u32 = 28u32;
```

### `R_XTENSA_SLOT9_OP`

```rust
const R_XTENSA_SLOT9_OP: u32 = 29u32;
```

### `R_XTENSA_SLOT10_OP`

```rust
const R_XTENSA_SLOT10_OP: u32 = 30u32;
```

### `R_XTENSA_SLOT11_OP`

```rust
const R_XTENSA_SLOT11_OP: u32 = 31u32;
```

### `R_XTENSA_SLOT12_OP`

```rust
const R_XTENSA_SLOT12_OP: u32 = 32u32;
```

### `R_XTENSA_SLOT13_OP`

```rust
const R_XTENSA_SLOT13_OP: u32 = 33u32;
```

### `R_XTENSA_SLOT14_OP`

```rust
const R_XTENSA_SLOT14_OP: u32 = 34u32;
```

### `R_XTENSA_SLOT0_ALT`

```rust
const R_XTENSA_SLOT0_ALT: u32 = 35u32;
```

### `R_XTENSA_SLOT1_ALT`

```rust
const R_XTENSA_SLOT1_ALT: u32 = 36u32;
```

### `R_XTENSA_SLOT2_ALT`

```rust
const R_XTENSA_SLOT2_ALT: u32 = 37u32;
```

### `R_XTENSA_SLOT3_ALT`

```rust
const R_XTENSA_SLOT3_ALT: u32 = 38u32;
```

### `R_XTENSA_SLOT4_ALT`

```rust
const R_XTENSA_SLOT4_ALT: u32 = 39u32;
```

### `R_XTENSA_SLOT5_ALT`

```rust
const R_XTENSA_SLOT5_ALT: u32 = 40u32;
```

### `R_XTENSA_SLOT6_ALT`

```rust
const R_XTENSA_SLOT6_ALT: u32 = 41u32;
```

### `R_XTENSA_SLOT7_ALT`

```rust
const R_XTENSA_SLOT7_ALT: u32 = 42u32;
```

### `R_XTENSA_SLOT8_ALT`

```rust
const R_XTENSA_SLOT8_ALT: u32 = 43u32;
```

### `R_XTENSA_SLOT9_ALT`

```rust
const R_XTENSA_SLOT9_ALT: u32 = 44u32;
```

### `R_XTENSA_SLOT10_ALT`

```rust
const R_XTENSA_SLOT10_ALT: u32 = 45u32;
```

### `R_XTENSA_SLOT11_ALT`

```rust
const R_XTENSA_SLOT11_ALT: u32 = 46u32;
```

### `R_XTENSA_SLOT12_ALT`

```rust
const R_XTENSA_SLOT12_ALT: u32 = 47u32;
```

### `R_XTENSA_SLOT13_ALT`

```rust
const R_XTENSA_SLOT13_ALT: u32 = 48u32;
```

### `R_XTENSA_SLOT14_ALT`

```rust
const R_XTENSA_SLOT14_ALT: u32 = 49u32;
```

### `R_XTENSA_TLSDESC_FN`

```rust
const R_XTENSA_TLSDESC_FN: u32 = 50u32;
```

### `R_XTENSA_TLSDESC_ARG`

```rust
const R_XTENSA_TLSDESC_ARG: u32 = 51u32;
```

### `R_XTENSA_TLS_DTPOFF`

```rust
const R_XTENSA_TLS_DTPOFF: u32 = 52u32;
```

### `R_XTENSA_TLS_TPOFF`

```rust
const R_XTENSA_TLS_TPOFF: u32 = 53u32;
```

### `R_XTENSA_TLS_FUNC`

```rust
const R_XTENSA_TLS_FUNC: u32 = 54u32;
```

### `R_XTENSA_TLS_ARG`

```rust
const R_XTENSA_TLS_ARG: u32 = 55u32;
```

### `R_XTENSA_TLS_CALL`

```rust
const R_XTENSA_TLS_CALL: u32 = 56u32;
```

### `R_XTENSA_PDIFF8`

```rust
const R_XTENSA_PDIFF8: u32 = 57u32;
```

### `R_XTENSA_PDIFF16`

```rust
const R_XTENSA_PDIFF16: u32 = 58u32;
```

### `R_XTENSA_PDIFF32`

```rust
const R_XTENSA_PDIFF32: u32 = 59u32;
```

### `R_XTENSA_NDIFF8`

```rust
const R_XTENSA_NDIFF8: u32 = 60u32;
```

### `R_XTENSA_NDIFF16`

```rust
const R_XTENSA_NDIFF16: u32 = 61u32;
```

### `R_XTENSA_NDIFF32`

```rust
const R_XTENSA_NDIFF32: u32 = 62u32;
```

### `EF_E2K_IPD`

```rust
const EF_E2K_IPD: u32 = 3u32;
```

### `EF_E2K_X86APP`

```rust
const EF_E2K_X86APP: u32 = 4u32;
```

### `EF_E2K_4MB_PAGES`

```rust
const EF_E2K_4MB_PAGES: u32 = 8u32;
```

### `EF_E2K_INCOMPAT`

```rust
const EF_E2K_INCOMPAT: u32 = 16u32;
```

### `EF_E2K_PM`

```rust
const EF_E2K_PM: u32 = 32u32;
```

### `EF_E2K_PACK_SEGMENTS`

```rust
const EF_E2K_PACK_SEGMENTS: u32 = 64u32;
```

### `E_E2K_MACH_BASE`

```rust
const E_E2K_MACH_BASE: u32 = 0u32;
```

-march=generic code.

Legacy. Shouldn't be created nowadays.

### `E_E2K_MACH_EV1`

```rust
const E_E2K_MACH_EV1: u32 = 1u32;
```

-march=elbrus-v1 code.

Legacy. Shouldn't be created nowadays.

### `E_E2K_MACH_EV2`

```rust
const E_E2K_MACH_EV2: u32 = 2u32;
```

-march=elbrus-v2 code.

### `E_E2K_MACH_EV3`

```rust
const E_E2K_MACH_EV3: u32 = 3u32;
```

-march=elbrus-v3 code.

### `E_E2K_MACH_EV4`

```rust
const E_E2K_MACH_EV4: u32 = 4u32;
```

-march=elbrus-v4 code.

### `E_E2K_MACH_EV5`

```rust
const E_E2K_MACH_EV5: u32 = 5u32;
```

-march=elbrus-v5 code.

### `E_E2K_MACH_EV6`

```rust
const E_E2K_MACH_EV6: u32 = 6u32;
```

-march=elbrus-v6 code.

### `E_E2K_MACH_EV7`

```rust
const E_E2K_MACH_EV7: u32 = 7u32;
```

-march=elbrus-v7 code.

### `E_E2K_MACH_8C`

```rust
const E_E2K_MACH_8C: u32 = 19u32;
```

-mtune=elbrus-8c code.

### `E_E2K_MACH_1CPLUS`

```rust
const E_E2K_MACH_1CPLUS: u32 = 20u32;
```

-mtune=elbrus-1c+ code.

### `E_E2K_MACH_12C`

```rust
const E_E2K_MACH_12C: u32 = 21u32;
```

-mtune=elbrus-12c code.

### `E_E2K_MACH_16C`

```rust
const E_E2K_MACH_16C: u32 = 22u32;
```

-mtune=elbrus-16c code.

### `E_E2K_MACH_2C3`

```rust
const E_E2K_MACH_2C3: u32 = 23u32;
```

-mtune=elbrus-2c3 code.

### `E_E2K_MACH_48C`

```rust
const E_E2K_MACH_48C: u32 = 24u32;
```

-mtune=elbrus-48c code.

### `E_E2K_MACH_8V7`

```rust
const E_E2K_MACH_8V7: u32 = 25u32;
```

-mtune=elbrus-8v7 code.

### `R_E2K_32_ABS`

```rust
const R_E2K_32_ABS: u32 = 0u32;
```

Direct 32 bit.

### `R_E2K_32_PC`

```rust
const R_E2K_32_PC: u32 = 2u32;
```

PC relative 32 bit.

### `R_E2K_AP_GOT`

```rust
const R_E2K_AP_GOT: u32 = 3u32;
```

32-bit offset of AP GOT entry.

### `R_E2K_PL_GOT`

```rust
const R_E2K_PL_GOT: u32 = 4u32;
```

32-bit offset of PL GOT entry.

### `R_E2K_32_JMP_SLOT`

```rust
const R_E2K_32_JMP_SLOT: u32 = 8u32;
```

Create PLT entry.

### `R_E2K_32_COPY`

```rust
const R_E2K_32_COPY: u32 = 9u32;
```

Copy relocation, 32-bit case.

### `R_E2K_32_RELATIVE`

```rust
const R_E2K_32_RELATIVE: u32 = 10u32;
```

Adjust by program base, 32-bit case.

### `R_E2K_32_IRELATIVE`

```rust
const R_E2K_32_IRELATIVE: u32 = 11u32;
```

Adjust indirectly by program base, 32-bit case.

### `R_E2K_32_SIZE`

```rust
const R_E2K_32_SIZE: u32 = 12u32;
```

Size of symbol plus 32-bit addend.

### `R_E2K_32_DYNOPT`

```rust
const R_E2K_32_DYNOPT: u32 = 13u32;
```

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 32-bit case.

### `R_E2K_64_ABS`

```rust
const R_E2K_64_ABS: u32 = 50u32;
```

Direct 64 bit.

### `R_E2K_64_ABS_LIT`

```rust
const R_E2K_64_ABS_LIT: u32 = 51u32;
```

Direct 64 bit for literal.

### `R_E2K_64_PC_LIT`

```rust
const R_E2K_64_PC_LIT: u32 = 54u32;
```

PC relative 64 bit for literal.

### `R_E2K_64_JMP_SLOT`

```rust
const R_E2K_64_JMP_SLOT: u32 = 63u32;
```

Create PLT entry, 64-bit case.

### `R_E2K_64_COPY`

```rust
const R_E2K_64_COPY: u32 = 64u32;
```

Copy relocation, 64-bit case.

### `R_E2K_64_RELATIVE`

```rust
const R_E2K_64_RELATIVE: u32 = 65u32;
```

Adjust by program base, 64-bit case.

### `R_E2K_64_RELATIVE_LIT`

```rust
const R_E2K_64_RELATIVE_LIT: u32 = 66u32;
```

Adjust by program base for literal, 64-bit case.

### `R_E2K_64_IRELATIVE`

```rust
const R_E2K_64_IRELATIVE: u32 = 67u32;
```

Adjust indirectly by program base, 64-bit case.

### `R_E2K_64_SIZE`

```rust
const R_E2K_64_SIZE: u32 = 68u32;
```

Size of symbol plus 64-bit addend.

### `R_E2K_64_GOTOFF`

```rust
const R_E2K_64_GOTOFF: u32 = 69u32;
```

64-bit offset of the symbol from GOT.

### `R_E2K_TLS_GDMOD`

```rust
const R_E2K_TLS_GDMOD: u32 = 70u32;
```

GOT entry for ID of module containing symbol.

### `R_E2K_TLS_GDREL`

```rust
const R_E2K_TLS_GDREL: u32 = 71u32;
```

GOT entry for offset in module TLS block.

### `R_E2K_TLS_IE`

```rust
const R_E2K_TLS_IE: u32 = 74u32;
```

Static TLS block offset GOT entry.

### `R_E2K_32_TLS_LE`

```rust
const R_E2K_32_TLS_LE: u32 = 75u32;
```

Offset relative to static TLS block, 32-bit case.

### `R_E2K_64_TLS_LE`

```rust
const R_E2K_64_TLS_LE: u32 = 76u32;
```

Offset relative to static TLS block, 64-bit case.

### `R_E2K_TLS_32_DTPMOD`

```rust
const R_E2K_TLS_32_DTPMOD: u32 = 80u32;
```

ID of module containing symbol, 32-bit case.

### `R_E2K_TLS_32_DTPREL`

```rust
const R_E2K_TLS_32_DTPREL: u32 = 81u32;
```

Offset in module TLS block, 32-bit case.

### `R_E2K_TLS_64_DTPMOD`

```rust
const R_E2K_TLS_64_DTPMOD: u32 = 82u32;
```

ID of module containing symbol, 64-bit case.

### `R_E2K_TLS_64_DTPREL`

```rust
const R_E2K_TLS_64_DTPREL: u32 = 83u32;
```

Offset in module TLS block, 64-bit case.

### `R_E2K_TLS_32_TPREL`

```rust
const R_E2K_TLS_32_TPREL: u32 = 84u32;
```

Offset in static TLS block, 32-bit case.

### `R_E2K_TLS_64_TPREL`

```rust
const R_E2K_TLS_64_TPREL: u32 = 85u32;
```

Offset in static TLS block, 64-bit case.

### `R_E2K_AP`

```rust
const R_E2K_AP: u32 = 100u32;
```

Direct AP.

### `R_E2K_PL`

```rust
const R_E2K_PL: u32 = 101u32;
```

Direct PL.

### `R_E2K_GOT`

```rust
const R_E2K_GOT: u32 = 108u32;
```

32-bit offset of the symbol's entry in GOT.

### `R_E2K_GOTOFF`

```rust
const R_E2K_GOTOFF: u32 = 109u32;
```

32-bit offset of the symbol from GOT.

### `R_E2K_DISP`

```rust
const R_E2K_DISP: u32 = 110u32;
```

PC relative 28 bit for DISP.

### `R_E2K_PREF`

```rust
const R_E2K_PREF: u32 = 111u32;
```

Prefetch insn line containing the label (symbol).

### `R_E2K_NONE`

```rust
const R_E2K_NONE: u32 = 112u32;
```

No reloc.

### `R_E2K_GOTPLT`

```rust
const R_E2K_GOTPLT: u32 = 114u32;
```

32-bit offset of the symbol's entry in .got.plt.

### `R_E2K_ISLOCAL`

```rust
const R_E2K_ISLOCAL: u32 = 115u32;
```

Is symbol resolved locally during the link.
The result is encoded in 5-bit ALS.src1.

### `R_E2K_ISLOCAL32`

```rust
const R_E2K_ISLOCAL32: u32 = 118u32;
```

Is symbol resloved locally during the link.
The result is encoded in a long 32-bit LTS.

### `R_E2K_64_GOTOFF_LIT`

```rust
const R_E2K_64_GOTOFF_LIT: u32 = 256u32;
```

The symbol's offset from GOT encoded within a 64-bit literal.

### `R_E2K_64_DYNOPT`

```rust
const R_E2K_64_DYNOPT: u32 = 257u32;
```

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 64-bit case.

### `R_E2K_64_PC`

```rust
const R_E2K_64_PC: u32 = 258u32;
```

PC relative 64 bit in data.

### `DT_E2K_LAZY`

```rust
const DT_E2K_LAZY: u32 = 1_879_048_193u32;
```

### `DT_E2K_LAZY_GOT`

```rust
const DT_E2K_LAZY_GOT: u32 = 1_879_048_195u32;
```

### `DT_E2K_INIT_GOT`

```rust
const DT_E2K_INIT_GOT: u32 = 1_879_052_316u32;
```

### `DT_E2K_EXPORT_PL`

```rust
const DT_E2K_EXPORT_PL: u32 = 1_879_052_317u32;
```

### `DT_E2K_EXPORT_PLSZ`

```rust
const DT_E2K_EXPORT_PLSZ: u32 = 1_879_052_318u32;
```

### `DT_E2K_REAL_PLTGOT`

```rust
const DT_E2K_REAL_PLTGOT: u32 = 1_879_052_319u32;
```

### `DT_E2K_NO_SELFINIT`

```rust
const DT_E2K_NO_SELFINIT: u32 = 1_879_052_320u32;
```

### `DT_E2K_NUM`

```rust
const DT_E2K_NUM: u32 = 4_129u32;
```

### `Tag_File`

```rust
const Tag_File: u8 = 1u8;
```

### `Tag_Section`

```rust
const Tag_Section: u8 = 2u8;
```

### `Tag_Symbol`

```rust
const Tag_Symbol: u8 = 3u8;
```

