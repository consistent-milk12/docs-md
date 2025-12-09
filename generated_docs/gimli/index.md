# Crate `gimli`

`gimli` is a library for reading and writing the
[DWARF debugging format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html) modules
for examples and API documentation.

## Cargo Features

Cargo features that can be enabled with `gimli`:

* `std`: Enabled by default. Use the `std` library. Disabling this feature
  allows using `gimli` in embedded environments that do not have access to
  `std`. Note that even when `std` is disabled, `gimli` still requires an
  implementation of the `alloc` crate.

* `read`: Enabled by default. Enables the `read` module. Use of `std` is
  optional.

* `write`: Enabled by default. Enables the `write` module. Always uses
  the `std` library.

## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`arch`](#arch)
  - [`constants`](#constants)
  - [`endianity`](#endianity)
  - [`leb128`](#leb128)
  - [`read`](#read)
  - [`util`](#util)
  - [`addr`](#addr)
  - [`cfi`](#cfi)
  - [`dwarf`](#dwarf)
  - [`endian_slice`](#endian_slice)
  - [`reader`](#reader)
  - [`relocate`](#relocate)
  - [`abbrev`](#abbrev)
  - [`aranges`](#aranges)
  - [`index`](#index)
  - [`line`](#line)
  - [`lists`](#lists)
  - [`loclists`](#loclists)
  - [`lookup`](#lookup)
  - [`macros`](#macros)
  - [`op`](#op)
  - [`pubnames`](#pubnames)
  - [`pubtypes`](#pubtypes)
  - [`rnglists`](#rnglists)
  - [`str`](#str)
  - [`unit`](#unit)
  - [`value`](#value)
- [Structs](#structs)
  - [`Encoding`](#encoding)
  - [`LineEncoding`](#lineencoding)
  - [`Register`](#register)
  - [`DebugAbbrevOffset`](#debugabbrevoffset)
  - [`DebugAddrOffset`](#debugaddroffset)
  - [`DebugAddrBase`](#debugaddrbase)
  - [`DebugAddrIndex`](#debugaddrindex)
  - [`DebugArangesOffset`](#debugarangesoffset)
  - [`DebugInfoOffset`](#debuginfooffset)
  - [`DebugLineOffset`](#debuglineoffset)
  - [`DebugLineStrOffset`](#debuglinestroffset)
  - [`LocationListsOffset`](#locationlistsoffset)
  - [`DebugLocListsBase`](#debugloclistsbase)
  - [`DebugLocListsIndex`](#debugloclistsindex)
  - [`DebugMacinfoOffset`](#debugmacinfooffset)
  - [`DebugMacroOffset`](#debugmacrooffset)
  - [`RawRangeListsOffset`](#rawrangelistsoffset)
  - [`RangeListsOffset`](#rangelistsoffset)
  - [`DebugRngListsBase`](#debugrnglistsbase)
  - [`DebugRngListsIndex`](#debugrnglistsindex)
  - [`DebugStrOffset`](#debugstroffset)
  - [`DebugStrOffsetsBase`](#debugstroffsetsbase)
  - [`DebugStrOffsetsIndex`](#debugstroffsetsindex)
  - [`DebugTypesOffset`](#debugtypesoffset)
  - [`DebugTypeSignature`](#debugtypesignature)
  - [`DebugFrameOffset`](#debugframeoffset)
  - [`EhFrameOffset`](#ehframeoffset)
  - [`DwoId`](#dwoid)
  - [`Arm`](#arm)
  - [`AArch64`](#aarch64)
  - [`LoongArch`](#loongarch)
  - [`MIPS`](#mips)
  - [`RiscV`](#riscv)
  - [`X86`](#x86)
  - [`X86_64`](#x86_64)
  - [`PowerPc64`](#powerpc64)
  - [`DwSect`](#dwsect)
  - [`DwSectV2`](#dwsectv2)
  - [`DwUt`](#dwut)
  - [`DwCfa`](#dwcfa)
  - [`DwChildren`](#dwchildren)
  - [`DwTag`](#dwtag)
  - [`DwAt`](#dwat)
  - [`DwForm`](#dwform)
  - [`DwAte`](#dwate)
  - [`DwLle`](#dwlle)
  - [`DwDs`](#dwds)
  - [`DwEnd`](#dwend)
  - [`DwAccess`](#dwaccess)
  - [`DwVis`](#dwvis)
  - [`DwVirtuality`](#dwvirtuality)
  - [`DwLang`](#dwlang)
  - [`DwAddr`](#dwaddr)
  - [`DwId`](#dwid)
  - [`DwCc`](#dwcc)
  - [`DwInl`](#dwinl)
  - [`DwOrd`](#dword)
  - [`DwDsc`](#dwdsc)
  - [`DwIdx`](#dwidx)
  - [`DwDefaulted`](#dwdefaulted)
  - [`DwLns`](#dwlns)
  - [`DwLne`](#dwlne)
  - [`DwLnct`](#dwlnct)
  - [`DwMacinfo`](#dwmacinfo)
  - [`DwMacro`](#dwmacro)
  - [`DwRle`](#dwrle)
  - [`DwOp`](#dwop)
  - [`DwEhPe`](#dwehpe)
  - [`LittleEndian`](#littleendian)
  - [`BigEndian`](#bigendian)
  - [`UnitOffset`](#unitoffset)
  - [`StoreOnHeap`](#storeonheap)
- [Enums](#enums)
  - [`Format`](#format)
  - [`Vendor`](#vendor)
  - [`UnitSectionOffset`](#unitsectionoffset)
  - [`SectionId`](#sectionid)
  - [`DwarfFileType`](#dwarffiletype)
  - [`RunTimeEndian`](#runtimeendian)
  - [`Error`](#error)
- [Traits](#traits)
  - [`Endianity`](#endianity)
  - [`Section`](#section)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`EndianBuf`](#endianbuf)
  - [`Result`](#result)
- [Constants](#constants)
  - [`DW_SECT_INFO`](#dw_sect_info)
  - [`DW_SECT_ABBREV`](#dw_sect_abbrev)
  - [`DW_SECT_LINE`](#dw_sect_line)
  - [`DW_SECT_LOCLISTS`](#dw_sect_loclists)
  - [`DW_SECT_STR_OFFSETS`](#dw_sect_str_offsets)
  - [`DW_SECT_MACRO`](#dw_sect_macro)
  - [`DW_SECT_RNGLISTS`](#dw_sect_rnglists)
  - [`DW_SECT_V2_INFO`](#dw_sect_v2_info)
  - [`DW_SECT_V2_TYPES`](#dw_sect_v2_types)
  - [`DW_SECT_V2_ABBREV`](#dw_sect_v2_abbrev)
  - [`DW_SECT_V2_LINE`](#dw_sect_v2_line)
  - [`DW_SECT_V2_LOC`](#dw_sect_v2_loc)
  - [`DW_SECT_V2_STR_OFFSETS`](#dw_sect_v2_str_offsets)
  - [`DW_SECT_V2_MACINFO`](#dw_sect_v2_macinfo)
  - [`DW_SECT_V2_MACRO`](#dw_sect_v2_macro)
  - [`DW_UT_compile`](#dw_ut_compile)
  - [`DW_UT_type`](#dw_ut_type)
  - [`DW_UT_partial`](#dw_ut_partial)
  - [`DW_UT_skeleton`](#dw_ut_skeleton)
  - [`DW_UT_split_compile`](#dw_ut_split_compile)
  - [`DW_UT_split_type`](#dw_ut_split_type)
  - [`DW_UT_lo_user`](#dw_ut_lo_user)
  - [`DW_UT_hi_user`](#dw_ut_hi_user)
  - [`DW_CFA_advance_loc`](#dw_cfa_advance_loc)
  - [`DW_CFA_offset`](#dw_cfa_offset)
  - [`DW_CFA_restore`](#dw_cfa_restore)
  - [`DW_CFA_nop`](#dw_cfa_nop)
  - [`DW_CFA_set_loc`](#dw_cfa_set_loc)
  - [`DW_CFA_advance_loc1`](#dw_cfa_advance_loc1)
  - [`DW_CFA_advance_loc2`](#dw_cfa_advance_loc2)
  - [`DW_CFA_advance_loc4`](#dw_cfa_advance_loc4)
  - [`DW_CFA_offset_extended`](#dw_cfa_offset_extended)
  - [`DW_CFA_restore_extended`](#dw_cfa_restore_extended)
  - [`DW_CFA_undefined`](#dw_cfa_undefined)
  - [`DW_CFA_same_value`](#dw_cfa_same_value)
  - [`DW_CFA_register`](#dw_cfa_register)
  - [`DW_CFA_remember_state`](#dw_cfa_remember_state)
  - [`DW_CFA_restore_state`](#dw_cfa_restore_state)
  - [`DW_CFA_def_cfa`](#dw_cfa_def_cfa)
  - [`DW_CFA_def_cfa_register`](#dw_cfa_def_cfa_register)
  - [`DW_CFA_def_cfa_offset`](#dw_cfa_def_cfa_offset)
  - [`DW_CFA_def_cfa_expression`](#dw_cfa_def_cfa_expression)
  - [`DW_CFA_expression`](#dw_cfa_expression)
  - [`DW_CFA_offset_extended_sf`](#dw_cfa_offset_extended_sf)
  - [`DW_CFA_def_cfa_sf`](#dw_cfa_def_cfa_sf)
  - [`DW_CFA_def_cfa_offset_sf`](#dw_cfa_def_cfa_offset_sf)
  - [`DW_CFA_val_offset`](#dw_cfa_val_offset)
  - [`DW_CFA_val_offset_sf`](#dw_cfa_val_offset_sf)
  - [`DW_CFA_val_expression`](#dw_cfa_val_expression)
  - [`DW_CFA_lo_user`](#dw_cfa_lo_user)
  - [`DW_CFA_hi_user`](#dw_cfa_hi_user)
  - [`DW_CFA_MIPS_advance_loc8`](#dw_cfa_mips_advance_loc8)
  - [`DW_CFA_GNU_window_save`](#dw_cfa_gnu_window_save)
  - [`DW_CFA_GNU_args_size`](#dw_cfa_gnu_args_size)
  - [`DW_CFA_GNU_negative_offset_extended`](#dw_cfa_gnu_negative_offset_extended)
  - [`DW_CFA_AARCH64_negate_ra_state`](#dw_cfa_aarch64_negate_ra_state)
  - [`DW_CHILDREN_no`](#dw_children_no)
  - [`DW_CHILDREN_yes`](#dw_children_yes)
  - [`DW_TAG_null`](#dw_tag_null)
  - [`DW_TAG_global_subroutine`](#dw_tag_global_subroutine)
  - [`DW_TAG_global_variable`](#dw_tag_global_variable)
  - [`DW_TAG_local_variable`](#dw_tag_local_variable)
  - [`DW_TAG_subroutine`](#dw_tag_subroutine)
  - [`DW_TAG_array_type`](#dw_tag_array_type)
  - [`DW_TAG_class_type`](#dw_tag_class_type)
  - [`DW_TAG_entry_point`](#dw_tag_entry_point)
  - [`DW_TAG_enumeration_type`](#dw_tag_enumeration_type)
  - [`DW_TAG_formal_parameter`](#dw_tag_formal_parameter)
  - [`DW_TAG_imported_declaration`](#dw_tag_imported_declaration)
  - [`DW_TAG_label`](#dw_tag_label)
  - [`DW_TAG_lexical_block`](#dw_tag_lexical_block)
  - [`DW_TAG_member`](#dw_tag_member)
  - [`DW_TAG_pointer_type`](#dw_tag_pointer_type)
  - [`DW_TAG_reference_type`](#dw_tag_reference_type)
  - [`DW_TAG_compile_unit`](#dw_tag_compile_unit)
  - [`DW_TAG_string_type`](#dw_tag_string_type)
  - [`DW_TAG_structure_type`](#dw_tag_structure_type)
  - [`DW_TAG_subroutine_type`](#dw_tag_subroutine_type)
  - [`DW_TAG_typedef`](#dw_tag_typedef)
  - [`DW_TAG_union_type`](#dw_tag_union_type)
  - [`DW_TAG_unspecified_parameters`](#dw_tag_unspecified_parameters)
  - [`DW_TAG_variant`](#dw_tag_variant)
  - [`DW_TAG_common_block`](#dw_tag_common_block)
  - [`DW_TAG_common_inclusion`](#dw_tag_common_inclusion)
  - [`DW_TAG_inheritance`](#dw_tag_inheritance)
  - [`DW_TAG_inlined_subroutine`](#dw_tag_inlined_subroutine)
  - [`DW_TAG_module`](#dw_tag_module)
  - [`DW_TAG_ptr_to_member_type`](#dw_tag_ptr_to_member_type)
  - [`DW_TAG_set_type`](#dw_tag_set_type)
  - [`DW_TAG_subrange_type`](#dw_tag_subrange_type)
  - [`DW_TAG_with_stmt`](#dw_tag_with_stmt)
  - [`DW_TAG_access_declaration`](#dw_tag_access_declaration)
  - [`DW_TAG_base_type`](#dw_tag_base_type)
  - [`DW_TAG_catch_block`](#dw_tag_catch_block)
  - [`DW_TAG_const_type`](#dw_tag_const_type)
  - [`DW_TAG_constant`](#dw_tag_constant)
  - [`DW_TAG_enumerator`](#dw_tag_enumerator)
  - [`DW_TAG_file_type`](#dw_tag_file_type)
  - [`DW_TAG_friend`](#dw_tag_friend)
  - [`DW_TAG_namelist`](#dw_tag_namelist)
  - [`DW_TAG_namelist_item`](#dw_tag_namelist_item)
  - [`DW_TAG_packed_type`](#dw_tag_packed_type)
  - [`DW_TAG_subprogram`](#dw_tag_subprogram)
  - [`DW_TAG_template_type_parameter`](#dw_tag_template_type_parameter)
  - [`DW_TAG_template_value_parameter`](#dw_tag_template_value_parameter)
  - [`DW_TAG_thrown_type`](#dw_tag_thrown_type)
  - [`DW_TAG_try_block`](#dw_tag_try_block)
  - [`DW_TAG_variant_part`](#dw_tag_variant_part)
  - [`DW_TAG_variable`](#dw_tag_variable)
  - [`DW_TAG_volatile_type`](#dw_tag_volatile_type)
  - [`DW_TAG_dwarf_procedure`](#dw_tag_dwarf_procedure)
  - [`DW_TAG_restrict_type`](#dw_tag_restrict_type)
  - [`DW_TAG_interface_type`](#dw_tag_interface_type)
  - [`DW_TAG_namespace`](#dw_tag_namespace)
  - [`DW_TAG_imported_module`](#dw_tag_imported_module)
  - [`DW_TAG_unspecified_type`](#dw_tag_unspecified_type)
  - [`DW_TAG_partial_unit`](#dw_tag_partial_unit)
  - [`DW_TAG_imported_unit`](#dw_tag_imported_unit)
  - [`DW_TAG_condition`](#dw_tag_condition)
  - [`DW_TAG_shared_type`](#dw_tag_shared_type)
  - [`DW_TAG_type_unit`](#dw_tag_type_unit)
  - [`DW_TAG_rvalue_reference_type`](#dw_tag_rvalue_reference_type)
  - [`DW_TAG_template_alias`](#dw_tag_template_alias)
  - [`DW_TAG_coarray_type`](#dw_tag_coarray_type)
  - [`DW_TAG_generic_subrange`](#dw_tag_generic_subrange)
  - [`DW_TAG_dynamic_type`](#dw_tag_dynamic_type)
  - [`DW_TAG_atomic_type`](#dw_tag_atomic_type)
  - [`DW_TAG_call_site`](#dw_tag_call_site)
  - [`DW_TAG_call_site_parameter`](#dw_tag_call_site_parameter)
  - [`DW_TAG_skeleton_unit`](#dw_tag_skeleton_unit)
  - [`DW_TAG_immutable_type`](#dw_tag_immutable_type)
  - [`DW_TAG_lo_user`](#dw_tag_lo_user)
  - [`DW_TAG_hi_user`](#dw_tag_hi_user)
  - [`DW_TAG_MIPS_loop`](#dw_tag_mips_loop)
  - [`DW_TAG_HP_array_descriptor`](#dw_tag_hp_array_descriptor)
  - [`DW_TAG_HP_Bliss_field`](#dw_tag_hp_bliss_field)
  - [`DW_TAG_HP_Bliss_field_set`](#dw_tag_hp_bliss_field_set)
  - [`DW_TAG_format_label`](#dw_tag_format_label)
  - [`DW_TAG_function_template`](#dw_tag_function_template)
  - [`DW_TAG_class_template`](#dw_tag_class_template)
  - [`DW_TAG_GNU_BINCL`](#dw_tag_gnu_bincl)
  - [`DW_TAG_GNU_EINCL`](#dw_tag_gnu_eincl)
  - [`DW_TAG_GNU_template_template_param`](#dw_tag_gnu_template_template_param)
  - [`DW_TAG_GNU_template_parameter_pack`](#dw_tag_gnu_template_parameter_pack)
  - [`DW_TAG_GNU_formal_parameter_pack`](#dw_tag_gnu_formal_parameter_pack)
  - [`DW_TAG_GNU_call_site`](#dw_tag_gnu_call_site)
  - [`DW_TAG_GNU_call_site_parameter`](#dw_tag_gnu_call_site_parameter)
  - [`DW_TAG_APPLE_property`](#dw_tag_apple_property)
  - [`DW_TAG_SUN_function_template`](#dw_tag_sun_function_template)
  - [`DW_TAG_SUN_class_template`](#dw_tag_sun_class_template)
  - [`DW_TAG_SUN_struct_template`](#dw_tag_sun_struct_template)
  - [`DW_TAG_SUN_union_template`](#dw_tag_sun_union_template)
  - [`DW_TAG_SUN_indirect_inheritance`](#dw_tag_sun_indirect_inheritance)
  - [`DW_TAG_SUN_codeflags`](#dw_tag_sun_codeflags)
  - [`DW_TAG_SUN_memop_info`](#dw_tag_sun_memop_info)
  - [`DW_TAG_SUN_omp_child_func`](#dw_tag_sun_omp_child_func)
  - [`DW_TAG_SUN_rtti_descriptor`](#dw_tag_sun_rtti_descriptor)
  - [`DW_TAG_SUN_dtor_info`](#dw_tag_sun_dtor_info)
  - [`DW_TAG_SUN_dtor`](#dw_tag_sun_dtor)
  - [`DW_TAG_SUN_f90_interface`](#dw_tag_sun_f90_interface)
  - [`DW_TAG_SUN_fortran_vax_structure`](#dw_tag_sun_fortran_vax_structure)
  - [`DW_TAG_ALTIUM_circ_type`](#dw_tag_altium_circ_type)
  - [`DW_TAG_ALTIUM_mwa_circ_type`](#dw_tag_altium_mwa_circ_type)
  - [`DW_TAG_ALTIUM_rev_carry_type`](#dw_tag_altium_rev_carry_type)
  - [`DW_TAG_ALTIUM_rom`](#dw_tag_altium_rom)
  - [`DW_TAG_upc_shared_type`](#dw_tag_upc_shared_type)
  - [`DW_TAG_upc_strict_type`](#dw_tag_upc_strict_type)
  - [`DW_TAG_upc_relaxed_type`](#dw_tag_upc_relaxed_type)
  - [`DW_TAG_PGI_kanji_type`](#dw_tag_pgi_kanji_type)
  - [`DW_TAG_PGI_interface_block`](#dw_tag_pgi_interface_block)
  - [`DW_TAG_BORLAND_property`](#dw_tag_borland_property)
  - [`DW_TAG_BORLAND_Delphi_string`](#dw_tag_borland_delphi_string)
  - [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw_tag_borland_delphi_dynamic_array)
  - [`DW_TAG_BORLAND_Delphi_set`](#dw_tag_borland_delphi_set)
  - [`DW_TAG_BORLAND_Delphi_variant`](#dw_tag_borland_delphi_variant)
  - [`DW_AT_null`](#dw_at_null)
  - [`DW_AT_fund_type`](#dw_at_fund_type)
  - [`DW_AT_mod_fund_type`](#dw_at_mod_fund_type)
  - [`DW_AT_user_def_type`](#dw_at_user_def_type)
  - [`DW_AT_mod_u_d_type`](#dw_at_mod_u_d_type)
  - [`DW_AT_subscr_data`](#dw_at_subscr_data)
  - [`DW_AT_element_list`](#dw_at_element_list)
  - [`DW_AT_member`](#dw_at_member)
  - [`DW_AT_friends`](#dw_at_friends)
  - [`DW_AT_program`](#dw_at_program)
  - [`DW_AT_private`](#dw_at_private)
  - [`DW_AT_protected`](#dw_at_protected)
  - [`DW_AT_public`](#dw_at_public)
  - [`DW_AT_pure_virtual`](#dw_at_pure_virtual)
  - [`DW_AT_virtual`](#dw_at_virtual)
  - [`DW_AT_specification_v1`](#dw_at_specification_v1)
  - [`DW_AT_sibling`](#dw_at_sibling)
  - [`DW_AT_location`](#dw_at_location)
  - [`DW_AT_name`](#dw_at_name)
  - [`DW_AT_ordering`](#dw_at_ordering)
  - [`DW_AT_byte_size`](#dw_at_byte_size)
  - [`DW_AT_bit_offset`](#dw_at_bit_offset)
  - [`DW_AT_bit_size`](#dw_at_bit_size)
  - [`DW_AT_stmt_list`](#dw_at_stmt_list)
  - [`DW_AT_low_pc`](#dw_at_low_pc)
  - [`DW_AT_high_pc`](#dw_at_high_pc)
  - [`DW_AT_language`](#dw_at_language)
  - [`DW_AT_discr`](#dw_at_discr)
  - [`DW_AT_discr_value`](#dw_at_discr_value)
  - [`DW_AT_visibility`](#dw_at_visibility)
  - [`DW_AT_import`](#dw_at_import)
  - [`DW_AT_string_length`](#dw_at_string_length)
  - [`DW_AT_common_reference`](#dw_at_common_reference)
  - [`DW_AT_comp_dir`](#dw_at_comp_dir)
  - [`DW_AT_const_value`](#dw_at_const_value)
  - [`DW_AT_containing_type`](#dw_at_containing_type)
  - [`DW_AT_default_value`](#dw_at_default_value)
  - [`DW_AT_inline`](#dw_at_inline)
  - [`DW_AT_is_optional`](#dw_at_is_optional)
  - [`DW_AT_lower_bound`](#dw_at_lower_bound)
  - [`DW_AT_producer`](#dw_at_producer)
  - [`DW_AT_prototyped`](#dw_at_prototyped)
  - [`DW_AT_return_addr`](#dw_at_return_addr)
  - [`DW_AT_start_scope`](#dw_at_start_scope)
  - [`DW_AT_bit_stride`](#dw_at_bit_stride)
  - [`DW_AT_upper_bound`](#dw_at_upper_bound)
  - [`DW_AT_abstract_origin`](#dw_at_abstract_origin)
  - [`DW_AT_accessibility`](#dw_at_accessibility)
  - [`DW_AT_address_class`](#dw_at_address_class)
  - [`DW_AT_artificial`](#dw_at_artificial)
  - [`DW_AT_base_types`](#dw_at_base_types)
  - [`DW_AT_calling_convention`](#dw_at_calling_convention)
  - [`DW_AT_count`](#dw_at_count)
  - [`DW_AT_data_member_location`](#dw_at_data_member_location)
  - [`DW_AT_decl_column`](#dw_at_decl_column)
  - [`DW_AT_decl_file`](#dw_at_decl_file)
  - [`DW_AT_decl_line`](#dw_at_decl_line)
  - [`DW_AT_declaration`](#dw_at_declaration)
  - [`DW_AT_discr_list`](#dw_at_discr_list)
  - [`DW_AT_encoding`](#dw_at_encoding)
  - [`DW_AT_external`](#dw_at_external)
  - [`DW_AT_frame_base`](#dw_at_frame_base)
  - [`DW_AT_friend`](#dw_at_friend)
  - [`DW_AT_identifier_case`](#dw_at_identifier_case)
  - [`DW_AT_macro_info`](#dw_at_macro_info)
  - [`DW_AT_namelist_item`](#dw_at_namelist_item)
  - [`DW_AT_priority`](#dw_at_priority)
  - [`DW_AT_segment`](#dw_at_segment)
  - [`DW_AT_specification`](#dw_at_specification)
  - [`DW_AT_static_link`](#dw_at_static_link)
  - [`DW_AT_type`](#dw_at_type)
  - [`DW_AT_use_location`](#dw_at_use_location)
  - [`DW_AT_variable_parameter`](#dw_at_variable_parameter)
  - [`DW_AT_virtuality`](#dw_at_virtuality)
  - [`DW_AT_vtable_elem_location`](#dw_at_vtable_elem_location)
  - [`DW_AT_allocated`](#dw_at_allocated)
  - [`DW_AT_associated`](#dw_at_associated)
  - [`DW_AT_data_location`](#dw_at_data_location)
  - [`DW_AT_byte_stride`](#dw_at_byte_stride)
  - [`DW_AT_entry_pc`](#dw_at_entry_pc)
  - [`DW_AT_use_UTF8`](#dw_at_use_utf8)
  - [`DW_AT_extension`](#dw_at_extension)
  - [`DW_AT_ranges`](#dw_at_ranges)
  - [`DW_AT_trampoline`](#dw_at_trampoline)
  - [`DW_AT_call_column`](#dw_at_call_column)
  - [`DW_AT_call_file`](#dw_at_call_file)
  - [`DW_AT_call_line`](#dw_at_call_line)
  - [`DW_AT_description`](#dw_at_description)
  - [`DW_AT_binary_scale`](#dw_at_binary_scale)
  - [`DW_AT_decimal_scale`](#dw_at_decimal_scale)
  - [`DW_AT_small`](#dw_at_small)
  - [`DW_AT_decimal_sign`](#dw_at_decimal_sign)
  - [`DW_AT_digit_count`](#dw_at_digit_count)
  - [`DW_AT_picture_string`](#dw_at_picture_string)
  - [`DW_AT_mutable`](#dw_at_mutable)
  - [`DW_AT_threads_scaled`](#dw_at_threads_scaled)
  - [`DW_AT_explicit`](#dw_at_explicit)
  - [`DW_AT_object_pointer`](#dw_at_object_pointer)
  - [`DW_AT_endianity`](#dw_at_endianity)
  - [`DW_AT_elemental`](#dw_at_elemental)
  - [`DW_AT_pure`](#dw_at_pure)
  - [`DW_AT_recursive`](#dw_at_recursive)
  - [`DW_AT_signature`](#dw_at_signature)
  - [`DW_AT_main_subprogram`](#dw_at_main_subprogram)
  - [`DW_AT_data_bit_offset`](#dw_at_data_bit_offset)
  - [`DW_AT_const_expr`](#dw_at_const_expr)
  - [`DW_AT_enum_class`](#dw_at_enum_class)
  - [`DW_AT_linkage_name`](#dw_at_linkage_name)
  - [`DW_AT_string_length_bit_size`](#dw_at_string_length_bit_size)
  - [`DW_AT_string_length_byte_size`](#dw_at_string_length_byte_size)
  - [`DW_AT_rank`](#dw_at_rank)
  - [`DW_AT_str_offsets_base`](#dw_at_str_offsets_base)
  - [`DW_AT_addr_base`](#dw_at_addr_base)
  - [`DW_AT_rnglists_base`](#dw_at_rnglists_base)
  - [`DW_AT_dwo_name`](#dw_at_dwo_name)
  - [`DW_AT_reference`](#dw_at_reference)
  - [`DW_AT_rvalue_reference`](#dw_at_rvalue_reference)
  - [`DW_AT_macros`](#dw_at_macros)
  - [`DW_AT_call_all_calls`](#dw_at_call_all_calls)
  - [`DW_AT_call_all_source_calls`](#dw_at_call_all_source_calls)
  - [`DW_AT_call_all_tail_calls`](#dw_at_call_all_tail_calls)
  - [`DW_AT_call_return_pc`](#dw_at_call_return_pc)
  - [`DW_AT_call_value`](#dw_at_call_value)
  - [`DW_AT_call_origin`](#dw_at_call_origin)
  - [`DW_AT_call_parameter`](#dw_at_call_parameter)
  - [`DW_AT_call_pc`](#dw_at_call_pc)
  - [`DW_AT_call_tail_call`](#dw_at_call_tail_call)
  - [`DW_AT_call_target`](#dw_at_call_target)
  - [`DW_AT_call_target_clobbered`](#dw_at_call_target_clobbered)
  - [`DW_AT_call_data_location`](#dw_at_call_data_location)
  - [`DW_AT_call_data_value`](#dw_at_call_data_value)
  - [`DW_AT_noreturn`](#dw_at_noreturn)
  - [`DW_AT_alignment`](#dw_at_alignment)
  - [`DW_AT_export_symbols`](#dw_at_export_symbols)
  - [`DW_AT_deleted`](#dw_at_deleted)
  - [`DW_AT_defaulted`](#dw_at_defaulted)
  - [`DW_AT_loclists_base`](#dw_at_loclists_base)
  - [`DW_AT_lo_user`](#dw_at_lo_user)
  - [`DW_AT_hi_user`](#dw_at_hi_user)
  - [`DW_AT_MIPS_fde`](#dw_at_mips_fde)
  - [`DW_AT_MIPS_loop_begin`](#dw_at_mips_loop_begin)
  - [`DW_AT_MIPS_tail_loop_begin`](#dw_at_mips_tail_loop_begin)
  - [`DW_AT_MIPS_epilog_begin`](#dw_at_mips_epilog_begin)
  - [`DW_AT_MIPS_loop_unroll_factor`](#dw_at_mips_loop_unroll_factor)
  - [`DW_AT_MIPS_software_pipeline_depth`](#dw_at_mips_software_pipeline_depth)
  - [`DW_AT_MIPS_linkage_name`](#dw_at_mips_linkage_name)
  - [`DW_AT_MIPS_stride`](#dw_at_mips_stride)
  - [`DW_AT_MIPS_abstract_name`](#dw_at_mips_abstract_name)
  - [`DW_AT_MIPS_clone_origin`](#dw_at_mips_clone_origin)
  - [`DW_AT_MIPS_has_inlines`](#dw_at_mips_has_inlines)
  - [`DW_AT_MIPS_stride_byte`](#dw_at_mips_stride_byte)
  - [`DW_AT_MIPS_stride_elem`](#dw_at_mips_stride_elem)
  - [`DW_AT_MIPS_ptr_dopetype`](#dw_at_mips_ptr_dopetype)
  - [`DW_AT_MIPS_allocatable_dopetype`](#dw_at_mips_allocatable_dopetype)
  - [`DW_AT_MIPS_assumed_shape_dopetype`](#dw_at_mips_assumed_shape_dopetype)
  - [`DW_AT_MIPS_assumed_size`](#dw_at_mips_assumed_size)
  - [`DW_AT_INTEL_other_endian`](#dw_at_intel_other_endian)
  - [`DW_AT_sf_names`](#dw_at_sf_names)
  - [`DW_AT_src_info`](#dw_at_src_info)
  - [`DW_AT_mac_info`](#dw_at_mac_info)
  - [`DW_AT_src_coords`](#dw_at_src_coords)
  - [`DW_AT_body_begin`](#dw_at_body_begin)
  - [`DW_AT_body_end`](#dw_at_body_end)
  - [`DW_AT_GNU_vector`](#dw_at_gnu_vector)
  - [`DW_AT_GNU_guarded_by`](#dw_at_gnu_guarded_by)
  - [`DW_AT_GNU_pt_guarded_by`](#dw_at_gnu_pt_guarded_by)
  - [`DW_AT_GNU_guarded`](#dw_at_gnu_guarded)
  - [`DW_AT_GNU_pt_guarded`](#dw_at_gnu_pt_guarded)
  - [`DW_AT_GNU_locks_excluded`](#dw_at_gnu_locks_excluded)
  - [`DW_AT_GNU_exclusive_locks_required`](#dw_at_gnu_exclusive_locks_required)
  - [`DW_AT_GNU_shared_locks_required`](#dw_at_gnu_shared_locks_required)
  - [`DW_AT_GNU_odr_signature`](#dw_at_gnu_odr_signature)
  - [`DW_AT_GNU_template_name`](#dw_at_gnu_template_name)
  - [`DW_AT_GNU_call_site_value`](#dw_at_gnu_call_site_value)
  - [`DW_AT_GNU_call_site_data_value`](#dw_at_gnu_call_site_data_value)
  - [`DW_AT_GNU_call_site_target`](#dw_at_gnu_call_site_target)
  - [`DW_AT_GNU_call_site_target_clobbered`](#dw_at_gnu_call_site_target_clobbered)
  - [`DW_AT_GNU_tail_call`](#dw_at_gnu_tail_call)
  - [`DW_AT_GNU_all_tail_call_sites`](#dw_at_gnu_all_tail_call_sites)
  - [`DW_AT_GNU_all_call_sites`](#dw_at_gnu_all_call_sites)
  - [`DW_AT_GNU_all_source_call_sites`](#dw_at_gnu_all_source_call_sites)
  - [`DW_AT_GNU_macros`](#dw_at_gnu_macros)
  - [`DW_AT_GNU_deleted`](#dw_at_gnu_deleted)
  - [`DW_AT_GNU_dwo_name`](#dw_at_gnu_dwo_name)
  - [`DW_AT_GNU_dwo_id`](#dw_at_gnu_dwo_id)
  - [`DW_AT_GNU_ranges_base`](#dw_at_gnu_ranges_base)
  - [`DW_AT_GNU_addr_base`](#dw_at_gnu_addr_base)
  - [`DW_AT_GNU_pubnames`](#dw_at_gnu_pubnames)
  - [`DW_AT_GNU_pubtypes`](#dw_at_gnu_pubtypes)
  - [`DW_AT_GNU_discriminator`](#dw_at_gnu_discriminator)
  - [`DW_AT_GNU_locviews`](#dw_at_gnu_locviews)
  - [`DW_AT_GNU_entry_view`](#dw_at_gnu_entry_view)
  - [`DW_AT_SUN_template`](#dw_at_sun_template)
  - [`DW_AT_SUN_alignment`](#dw_at_sun_alignment)
  - [`DW_AT_SUN_vtable`](#dw_at_sun_vtable)
  - [`DW_AT_SUN_count_guarantee`](#dw_at_sun_count_guarantee)
  - [`DW_AT_SUN_command_line`](#dw_at_sun_command_line)
  - [`DW_AT_SUN_vbase`](#dw_at_sun_vbase)
  - [`DW_AT_SUN_compile_options`](#dw_at_sun_compile_options)
  - [`DW_AT_SUN_language`](#dw_at_sun_language)
  - [`DW_AT_SUN_browser_file`](#dw_at_sun_browser_file)
  - [`DW_AT_SUN_vtable_abi`](#dw_at_sun_vtable_abi)
  - [`DW_AT_SUN_func_offsets`](#dw_at_sun_func_offsets)
  - [`DW_AT_SUN_cf_kind`](#dw_at_sun_cf_kind)
  - [`DW_AT_SUN_vtable_index`](#dw_at_sun_vtable_index)
  - [`DW_AT_SUN_omp_tpriv_addr`](#dw_at_sun_omp_tpriv_addr)
  - [`DW_AT_SUN_omp_child_func`](#dw_at_sun_omp_child_func)
  - [`DW_AT_SUN_func_offset`](#dw_at_sun_func_offset)
  - [`DW_AT_SUN_memop_type_ref`](#dw_at_sun_memop_type_ref)
  - [`DW_AT_SUN_profile_id`](#dw_at_sun_profile_id)
  - [`DW_AT_SUN_memop_signature`](#dw_at_sun_memop_signature)
  - [`DW_AT_SUN_obj_dir`](#dw_at_sun_obj_dir)
  - [`DW_AT_SUN_obj_file`](#dw_at_sun_obj_file)
  - [`DW_AT_SUN_original_name`](#dw_at_sun_original_name)
  - [`DW_AT_SUN_hwcprof_signature`](#dw_at_sun_hwcprof_signature)
  - [`DW_AT_SUN_amd64_parmdump`](#dw_at_sun_amd64_parmdump)
  - [`DW_AT_SUN_part_link_name`](#dw_at_sun_part_link_name)
  - [`DW_AT_SUN_link_name`](#dw_at_sun_link_name)
  - [`DW_AT_SUN_pass_with_const`](#dw_at_sun_pass_with_const)
  - [`DW_AT_SUN_return_with_const`](#dw_at_sun_return_with_const)
  - [`DW_AT_SUN_import_by_name`](#dw_at_sun_import_by_name)
  - [`DW_AT_SUN_f90_pointer`](#dw_at_sun_f90_pointer)
  - [`DW_AT_SUN_pass_by_ref`](#dw_at_sun_pass_by_ref)
  - [`DW_AT_SUN_f90_allocatable`](#dw_at_sun_f90_allocatable)
  - [`DW_AT_SUN_f90_assumed_shape_array`](#dw_at_sun_f90_assumed_shape_array)
  - [`DW_AT_SUN_c_vla`](#dw_at_sun_c_vla)
  - [`DW_AT_SUN_return_value_ptr`](#dw_at_sun_return_value_ptr)
  - [`DW_AT_SUN_dtor_start`](#dw_at_sun_dtor_start)
  - [`DW_AT_SUN_dtor_length`](#dw_at_sun_dtor_length)
  - [`DW_AT_SUN_dtor_state_initial`](#dw_at_sun_dtor_state_initial)
  - [`DW_AT_SUN_dtor_state_final`](#dw_at_sun_dtor_state_final)
  - [`DW_AT_SUN_dtor_state_deltas`](#dw_at_sun_dtor_state_deltas)
  - [`DW_AT_SUN_import_by_lname`](#dw_at_sun_import_by_lname)
  - [`DW_AT_SUN_f90_use_only`](#dw_at_sun_f90_use_only)
  - [`DW_AT_SUN_namelist_spec`](#dw_at_sun_namelist_spec)
  - [`DW_AT_SUN_is_omp_child_func`](#dw_at_sun_is_omp_child_func)
  - [`DW_AT_SUN_fortran_main_alias`](#dw_at_sun_fortran_main_alias)
  - [`DW_AT_SUN_fortran_based`](#dw_at_sun_fortran_based)
  - [`DW_AT_ALTIUM_loclist`](#dw_at_altium_loclist)
  - [`DW_AT_use_GNAT_descriptive_type`](#dw_at_use_gnat_descriptive_type)
  - [`DW_AT_GNAT_descriptive_type`](#dw_at_gnat_descriptive_type)
  - [`DW_AT_GNU_numerator`](#dw_at_gnu_numerator)
  - [`DW_AT_GNU_denominator`](#dw_at_gnu_denominator)
  - [`DW_AT_GNU_bias`](#dw_at_gnu_bias)
  - [`DW_AT_upc_threads_scaled`](#dw_at_upc_threads_scaled)
  - [`DW_AT_PGI_lbase`](#dw_at_pgi_lbase)
  - [`DW_AT_PGI_soffset`](#dw_at_pgi_soffset)
  - [`DW_AT_PGI_lstride`](#dw_at_pgi_lstride)
  - [`DW_AT_BORLAND_property_read`](#dw_at_borland_property_read)
  - [`DW_AT_BORLAND_property_write`](#dw_at_borland_property_write)
  - [`DW_AT_BORLAND_property_implements`](#dw_at_borland_property_implements)
  - [`DW_AT_BORLAND_property_index`](#dw_at_borland_property_index)
  - [`DW_AT_BORLAND_property_default`](#dw_at_borland_property_default)
  - [`DW_AT_BORLAND_Delphi_unit`](#dw_at_borland_delphi_unit)
  - [`DW_AT_BORLAND_Delphi_class`](#dw_at_borland_delphi_class)
  - [`DW_AT_BORLAND_Delphi_record`](#dw_at_borland_delphi_record)
  - [`DW_AT_BORLAND_Delphi_metaclass`](#dw_at_borland_delphi_metaclass)
  - [`DW_AT_BORLAND_Delphi_constructor`](#dw_at_borland_delphi_constructor)
  - [`DW_AT_BORLAND_Delphi_destructor`](#dw_at_borland_delphi_destructor)
  - [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw_at_borland_delphi_anonymous_method)
  - [`DW_AT_BORLAND_Delphi_interface`](#dw_at_borland_delphi_interface)
  - [`DW_AT_BORLAND_Delphi_ABI`](#dw_at_borland_delphi_abi)
  - [`DW_AT_BORLAND_Delphi_return`](#dw_at_borland_delphi_return)
  - [`DW_AT_BORLAND_Delphi_frameptr`](#dw_at_borland_delphi_frameptr)
  - [`DW_AT_BORLAND_closure`](#dw_at_borland_closure)
  - [`DW_AT_LLVM_include_path`](#dw_at_llvm_include_path)
  - [`DW_AT_LLVM_config_macros`](#dw_at_llvm_config_macros)
  - [`DW_AT_LLVM_isysroot`](#dw_at_llvm_isysroot)
  - [`DW_AT_APPLE_optimized`](#dw_at_apple_optimized)
  - [`DW_AT_APPLE_flags`](#dw_at_apple_flags)
  - [`DW_AT_APPLE_isa`](#dw_at_apple_isa)
  - [`DW_AT_APPLE_block`](#dw_at_apple_block)
  - [`DW_AT_APPLE_major_runtime_vers`](#dw_at_apple_major_runtime_vers)
  - [`DW_AT_APPLE_runtime_class`](#dw_at_apple_runtime_class)
  - [`DW_AT_APPLE_omit_frame_ptr`](#dw_at_apple_omit_frame_ptr)
  - [`DW_AT_APPLE_property_name`](#dw_at_apple_property_name)
  - [`DW_AT_APPLE_property_getter`](#dw_at_apple_property_getter)
  - [`DW_AT_APPLE_property_setter`](#dw_at_apple_property_setter)
  - [`DW_AT_APPLE_property_attribute`](#dw_at_apple_property_attribute)
  - [`DW_AT_APPLE_objc_complete_type`](#dw_at_apple_objc_complete_type)
  - [`DW_AT_APPLE_property`](#dw_at_apple_property)
  - [`DW_FORM_null`](#dw_form_null)
  - [`DW_FORM_ref`](#dw_form_ref)
  - [`DW_FORM_addr`](#dw_form_addr)
  - [`DW_FORM_block2`](#dw_form_block2)
  - [`DW_FORM_block4`](#dw_form_block4)
  - [`DW_FORM_data2`](#dw_form_data2)
  - [`DW_FORM_data4`](#dw_form_data4)
  - [`DW_FORM_data8`](#dw_form_data8)
  - [`DW_FORM_string`](#dw_form_string)
  - [`DW_FORM_block`](#dw_form_block)
  - [`DW_FORM_block1`](#dw_form_block1)
  - [`DW_FORM_data1`](#dw_form_data1)
  - [`DW_FORM_flag`](#dw_form_flag)
  - [`DW_FORM_sdata`](#dw_form_sdata)
  - [`DW_FORM_strp`](#dw_form_strp)
  - [`DW_FORM_udata`](#dw_form_udata)
  - [`DW_FORM_ref_addr`](#dw_form_ref_addr)
  - [`DW_FORM_ref1`](#dw_form_ref1)
  - [`DW_FORM_ref2`](#dw_form_ref2)
  - [`DW_FORM_ref4`](#dw_form_ref4)
  - [`DW_FORM_ref8`](#dw_form_ref8)
  - [`DW_FORM_ref_udata`](#dw_form_ref_udata)
  - [`DW_FORM_indirect`](#dw_form_indirect)
  - [`DW_FORM_sec_offset`](#dw_form_sec_offset)
  - [`DW_FORM_exprloc`](#dw_form_exprloc)
  - [`DW_FORM_flag_present`](#dw_form_flag_present)
  - [`DW_FORM_ref_sig8`](#dw_form_ref_sig8)
  - [`DW_FORM_strx`](#dw_form_strx)
  - [`DW_FORM_addrx`](#dw_form_addrx)
  - [`DW_FORM_ref_sup4`](#dw_form_ref_sup4)
  - [`DW_FORM_strp_sup`](#dw_form_strp_sup)
  - [`DW_FORM_data16`](#dw_form_data16)
  - [`DW_FORM_line_strp`](#dw_form_line_strp)
  - [`DW_FORM_implicit_const`](#dw_form_implicit_const)
  - [`DW_FORM_loclistx`](#dw_form_loclistx)
  - [`DW_FORM_rnglistx`](#dw_form_rnglistx)
  - [`DW_FORM_ref_sup8`](#dw_form_ref_sup8)
  - [`DW_FORM_strx1`](#dw_form_strx1)
  - [`DW_FORM_strx2`](#dw_form_strx2)
  - [`DW_FORM_strx3`](#dw_form_strx3)
  - [`DW_FORM_strx4`](#dw_form_strx4)
  - [`DW_FORM_addrx1`](#dw_form_addrx1)
  - [`DW_FORM_addrx2`](#dw_form_addrx2)
  - [`DW_FORM_addrx3`](#dw_form_addrx3)
  - [`DW_FORM_addrx4`](#dw_form_addrx4)
  - [`DW_FORM_GNU_addr_index`](#dw_form_gnu_addr_index)
  - [`DW_FORM_GNU_str_index`](#dw_form_gnu_str_index)
  - [`DW_FORM_GNU_ref_alt`](#dw_form_gnu_ref_alt)
  - [`DW_FORM_GNU_strp_alt`](#dw_form_gnu_strp_alt)
  - [`DW_ATE_address`](#dw_ate_address)
  - [`DW_ATE_boolean`](#dw_ate_boolean)
  - [`DW_ATE_complex_float`](#dw_ate_complex_float)
  - [`DW_ATE_float`](#dw_ate_float)
  - [`DW_ATE_signed`](#dw_ate_signed)
  - [`DW_ATE_signed_char`](#dw_ate_signed_char)
  - [`DW_ATE_unsigned`](#dw_ate_unsigned)
  - [`DW_ATE_unsigned_char`](#dw_ate_unsigned_char)
  - [`DW_ATE_imaginary_float`](#dw_ate_imaginary_float)
  - [`DW_ATE_packed_decimal`](#dw_ate_packed_decimal)
  - [`DW_ATE_numeric_string`](#dw_ate_numeric_string)
  - [`DW_ATE_edited`](#dw_ate_edited)
  - [`DW_ATE_signed_fixed`](#dw_ate_signed_fixed)
  - [`DW_ATE_unsigned_fixed`](#dw_ate_unsigned_fixed)
  - [`DW_ATE_decimal_float`](#dw_ate_decimal_float)
  - [`DW_ATE_UTF`](#dw_ate_utf)
  - [`DW_ATE_UCS`](#dw_ate_ucs)
  - [`DW_ATE_ASCII`](#dw_ate_ascii)
  - [`DW_ATE_lo_user`](#dw_ate_lo_user)
  - [`DW_ATE_hi_user`](#dw_ate_hi_user)
  - [`DW_LLE_end_of_list`](#dw_lle_end_of_list)
  - [`DW_LLE_base_addressx`](#dw_lle_base_addressx)
  - [`DW_LLE_startx_endx`](#dw_lle_startx_endx)
  - [`DW_LLE_startx_length`](#dw_lle_startx_length)
  - [`DW_LLE_offset_pair`](#dw_lle_offset_pair)
  - [`DW_LLE_default_location`](#dw_lle_default_location)
  - [`DW_LLE_base_address`](#dw_lle_base_address)
  - [`DW_LLE_start_end`](#dw_lle_start_end)
  - [`DW_LLE_start_length`](#dw_lle_start_length)
  - [`DW_LLE_GNU_view_pair`](#dw_lle_gnu_view_pair)
  - [`DW_DS_unsigned`](#dw_ds_unsigned)
  - [`DW_DS_leading_overpunch`](#dw_ds_leading_overpunch)
  - [`DW_DS_trailing_overpunch`](#dw_ds_trailing_overpunch)
  - [`DW_DS_leading_separate`](#dw_ds_leading_separate)
  - [`DW_DS_trailing_separate`](#dw_ds_trailing_separate)
  - [`DW_END_default`](#dw_end_default)
  - [`DW_END_big`](#dw_end_big)
  - [`DW_END_little`](#dw_end_little)
  - [`DW_END_lo_user`](#dw_end_lo_user)
  - [`DW_END_hi_user`](#dw_end_hi_user)
  - [`DW_ACCESS_public`](#dw_access_public)
  - [`DW_ACCESS_protected`](#dw_access_protected)
  - [`DW_ACCESS_private`](#dw_access_private)
  - [`DW_VIS_local`](#dw_vis_local)
  - [`DW_VIS_exported`](#dw_vis_exported)
  - [`DW_VIS_qualified`](#dw_vis_qualified)
  - [`DW_VIRTUALITY_none`](#dw_virtuality_none)
  - [`DW_VIRTUALITY_virtual`](#dw_virtuality_virtual)
  - [`DW_VIRTUALITY_pure_virtual`](#dw_virtuality_pure_virtual)
  - [`DW_LANG_C89`](#dw_lang_c89)
  - [`DW_LANG_C`](#dw_lang_c)
  - [`DW_LANG_Ada83`](#dw_lang_ada83)
  - [`DW_LANG_C_plus_plus`](#dw_lang_c_plus_plus)
  - [`DW_LANG_Cobol74`](#dw_lang_cobol74)
  - [`DW_LANG_Cobol85`](#dw_lang_cobol85)
  - [`DW_LANG_Fortran77`](#dw_lang_fortran77)
  - [`DW_LANG_Fortran90`](#dw_lang_fortran90)
  - [`DW_LANG_Pascal83`](#dw_lang_pascal83)
  - [`DW_LANG_Modula2`](#dw_lang_modula2)
  - [`DW_LANG_Java`](#dw_lang_java)
  - [`DW_LANG_C99`](#dw_lang_c99)
  - [`DW_LANG_Ada95`](#dw_lang_ada95)
  - [`DW_LANG_Fortran95`](#dw_lang_fortran95)
  - [`DW_LANG_PLI`](#dw_lang_pli)
  - [`DW_LANG_ObjC`](#dw_lang_objc)
  - [`DW_LANG_ObjC_plus_plus`](#dw_lang_objc_plus_plus)
  - [`DW_LANG_UPC`](#dw_lang_upc)
  - [`DW_LANG_D`](#dw_lang_d)
  - [`DW_LANG_Python`](#dw_lang_python)
  - [`DW_LANG_OpenCL`](#dw_lang_opencl)
  - [`DW_LANG_Go`](#dw_lang_go)
  - [`DW_LANG_Modula3`](#dw_lang_modula3)
  - [`DW_LANG_Haskell`](#dw_lang_haskell)
  - [`DW_LANG_C_plus_plus_03`](#dw_lang_c_plus_plus_03)
  - [`DW_LANG_C_plus_plus_11`](#dw_lang_c_plus_plus_11)
  - [`DW_LANG_OCaml`](#dw_lang_ocaml)
  - [`DW_LANG_Rust`](#dw_lang_rust)
  - [`DW_LANG_C11`](#dw_lang_c11)
  - [`DW_LANG_Swift`](#dw_lang_swift)
  - [`DW_LANG_Julia`](#dw_lang_julia)
  - [`DW_LANG_Dylan`](#dw_lang_dylan)
  - [`DW_LANG_C_plus_plus_14`](#dw_lang_c_plus_plus_14)
  - [`DW_LANG_Fortran03`](#dw_lang_fortran03)
  - [`DW_LANG_Fortran08`](#dw_lang_fortran08)
  - [`DW_LANG_RenderScript`](#dw_lang_renderscript)
  - [`DW_LANG_BLISS`](#dw_lang_bliss)
  - [`DW_LANG_Kotlin`](#dw_lang_kotlin)
  - [`DW_LANG_Zig`](#dw_lang_zig)
  - [`DW_LANG_Crystal`](#dw_lang_crystal)
  - [`DW_LANG_C_plus_plus_17`](#dw_lang_c_plus_plus_17)
  - [`DW_LANG_C_plus_plus_20`](#dw_lang_c_plus_plus_20)
  - [`DW_LANG_C17`](#dw_lang_c17)
  - [`DW_LANG_Fortran18`](#dw_lang_fortran18)
  - [`DW_LANG_Ada2005`](#dw_lang_ada2005)
  - [`DW_LANG_Ada2012`](#dw_lang_ada2012)
  - [`DW_LANG_lo_user`](#dw_lang_lo_user)
  - [`DW_LANG_hi_user`](#dw_lang_hi_user)
  - [`DW_LANG_Mips_Assembler`](#dw_lang_mips_assembler)
  - [`DW_LANG_GOOGLE_RenderScript`](#dw_lang_google_renderscript)
  - [`DW_LANG_SUN_Assembler`](#dw_lang_sun_assembler)
  - [`DW_LANG_ALTIUM_Assembler`](#dw_lang_altium_assembler)
  - [`DW_LANG_BORLAND_Delphi`](#dw_lang_borland_delphi)
  - [`DW_ADDR_none`](#dw_addr_none)
  - [`DW_ID_case_sensitive`](#dw_id_case_sensitive)
  - [`DW_ID_up_case`](#dw_id_up_case)
  - [`DW_ID_down_case`](#dw_id_down_case)
  - [`DW_ID_case_insensitive`](#dw_id_case_insensitive)
  - [`DW_CC_normal`](#dw_cc_normal)
  - [`DW_CC_program`](#dw_cc_program)
  - [`DW_CC_nocall`](#dw_cc_nocall)
  - [`DW_CC_pass_by_reference`](#dw_cc_pass_by_reference)
  - [`DW_CC_pass_by_value`](#dw_cc_pass_by_value)
  - [`DW_CC_lo_user`](#dw_cc_lo_user)
  - [`DW_CC_hi_user`](#dw_cc_hi_user)
  - [`DW_INL_not_inlined`](#dw_inl_not_inlined)
  - [`DW_INL_inlined`](#dw_inl_inlined)
  - [`DW_INL_declared_not_inlined`](#dw_inl_declared_not_inlined)
  - [`DW_INL_declared_inlined`](#dw_inl_declared_inlined)
  - [`DW_ORD_row_major`](#dw_ord_row_major)
  - [`DW_ORD_col_major`](#dw_ord_col_major)
  - [`DW_DSC_label`](#dw_dsc_label)
  - [`DW_DSC_range`](#dw_dsc_range)
  - [`DW_IDX_compile_unit`](#dw_idx_compile_unit)
  - [`DW_IDX_type_unit`](#dw_idx_type_unit)
  - [`DW_IDX_die_offset`](#dw_idx_die_offset)
  - [`DW_IDX_parent`](#dw_idx_parent)
  - [`DW_IDX_type_hash`](#dw_idx_type_hash)
  - [`DW_IDX_lo_user`](#dw_idx_lo_user)
  - [`DW_IDX_hi_user`](#dw_idx_hi_user)
  - [`DW_DEFAULTED_no`](#dw_defaulted_no)
  - [`DW_DEFAULTED_in_class`](#dw_defaulted_in_class)
  - [`DW_DEFAULTED_out_of_class`](#dw_defaulted_out_of_class)
  - [`DW_LNS_copy`](#dw_lns_copy)
  - [`DW_LNS_advance_pc`](#dw_lns_advance_pc)
  - [`DW_LNS_advance_line`](#dw_lns_advance_line)
  - [`DW_LNS_set_file`](#dw_lns_set_file)
  - [`DW_LNS_set_column`](#dw_lns_set_column)
  - [`DW_LNS_negate_stmt`](#dw_lns_negate_stmt)
  - [`DW_LNS_set_basic_block`](#dw_lns_set_basic_block)
  - [`DW_LNS_const_add_pc`](#dw_lns_const_add_pc)
  - [`DW_LNS_fixed_advance_pc`](#dw_lns_fixed_advance_pc)
  - [`DW_LNS_set_prologue_end`](#dw_lns_set_prologue_end)
  - [`DW_LNS_set_epilogue_begin`](#dw_lns_set_epilogue_begin)
  - [`DW_LNS_set_isa`](#dw_lns_set_isa)
  - [`DW_LNE_end_sequence`](#dw_lne_end_sequence)
  - [`DW_LNE_set_address`](#dw_lne_set_address)
  - [`DW_LNE_define_file`](#dw_lne_define_file)
  - [`DW_LNE_set_discriminator`](#dw_lne_set_discriminator)
  - [`DW_LNE_lo_user`](#dw_lne_lo_user)
  - [`DW_LNE_hi_user`](#dw_lne_hi_user)
  - [`DW_LNCT_path`](#dw_lnct_path)
  - [`DW_LNCT_directory_index`](#dw_lnct_directory_index)
  - [`DW_LNCT_timestamp`](#dw_lnct_timestamp)
  - [`DW_LNCT_size`](#dw_lnct_size)
  - [`DW_LNCT_MD5`](#dw_lnct_md5)
  - [`DW_LNCT_lo_user`](#dw_lnct_lo_user)
  - [`DW_LNCT_LLVM_source`](#dw_lnct_llvm_source)
  - [`DW_LNCT_hi_user`](#dw_lnct_hi_user)
  - [`DW_MACINFO_define`](#dw_macinfo_define)
  - [`DW_MACINFO_undef`](#dw_macinfo_undef)
  - [`DW_MACINFO_start_file`](#dw_macinfo_start_file)
  - [`DW_MACINFO_end_file`](#dw_macinfo_end_file)
  - [`DW_MACINFO_vendor_ext`](#dw_macinfo_vendor_ext)
  - [`DW_MACRO_define`](#dw_macro_define)
  - [`DW_MACRO_undef`](#dw_macro_undef)
  - [`DW_MACRO_start_file`](#dw_macro_start_file)
  - [`DW_MACRO_end_file`](#dw_macro_end_file)
  - [`DW_MACRO_define_strp`](#dw_macro_define_strp)
  - [`DW_MACRO_undef_strp`](#dw_macro_undef_strp)
  - [`DW_MACRO_import`](#dw_macro_import)
  - [`DW_MACRO_define_sup`](#dw_macro_define_sup)
  - [`DW_MACRO_undef_sup`](#dw_macro_undef_sup)
  - [`DW_MACRO_import_sup`](#dw_macro_import_sup)
  - [`DW_MACRO_define_strx`](#dw_macro_define_strx)
  - [`DW_MACRO_undef_strx`](#dw_macro_undef_strx)
  - [`DW_MACRO_lo_user`](#dw_macro_lo_user)
  - [`DW_MACRO_hi_user`](#dw_macro_hi_user)
  - [`DW_RLE_end_of_list`](#dw_rle_end_of_list)
  - [`DW_RLE_base_addressx`](#dw_rle_base_addressx)
  - [`DW_RLE_startx_endx`](#dw_rle_startx_endx)
  - [`DW_RLE_startx_length`](#dw_rle_startx_length)
  - [`DW_RLE_offset_pair`](#dw_rle_offset_pair)
  - [`DW_RLE_base_address`](#dw_rle_base_address)
  - [`DW_RLE_start_end`](#dw_rle_start_end)
  - [`DW_RLE_start_length`](#dw_rle_start_length)
  - [`DW_OP_addr`](#dw_op_addr)
  - [`DW_OP_deref`](#dw_op_deref)
  - [`DW_OP_const1u`](#dw_op_const1u)
  - [`DW_OP_const1s`](#dw_op_const1s)
  - [`DW_OP_const2u`](#dw_op_const2u)
  - [`DW_OP_const2s`](#dw_op_const2s)
  - [`DW_OP_const4u`](#dw_op_const4u)
  - [`DW_OP_const4s`](#dw_op_const4s)
  - [`DW_OP_const8u`](#dw_op_const8u)
  - [`DW_OP_const8s`](#dw_op_const8s)
  - [`DW_OP_constu`](#dw_op_constu)
  - [`DW_OP_consts`](#dw_op_consts)
  - [`DW_OP_dup`](#dw_op_dup)
  - [`DW_OP_drop`](#dw_op_drop)
  - [`DW_OP_over`](#dw_op_over)
  - [`DW_OP_pick`](#dw_op_pick)
  - [`DW_OP_swap`](#dw_op_swap)
  - [`DW_OP_rot`](#dw_op_rot)
  - [`DW_OP_xderef`](#dw_op_xderef)
  - [`DW_OP_abs`](#dw_op_abs)
  - [`DW_OP_and`](#dw_op_and)
  - [`DW_OP_div`](#dw_op_div)
  - [`DW_OP_minus`](#dw_op_minus)
  - [`DW_OP_mod`](#dw_op_mod)
  - [`DW_OP_mul`](#dw_op_mul)
  - [`DW_OP_neg`](#dw_op_neg)
  - [`DW_OP_not`](#dw_op_not)
  - [`DW_OP_or`](#dw_op_or)
  - [`DW_OP_plus`](#dw_op_plus)
  - [`DW_OP_plus_uconst`](#dw_op_plus_uconst)
  - [`DW_OP_shl`](#dw_op_shl)
  - [`DW_OP_shr`](#dw_op_shr)
  - [`DW_OP_shra`](#dw_op_shra)
  - [`DW_OP_xor`](#dw_op_xor)
  - [`DW_OP_bra`](#dw_op_bra)
  - [`DW_OP_eq`](#dw_op_eq)
  - [`DW_OP_ge`](#dw_op_ge)
  - [`DW_OP_gt`](#dw_op_gt)
  - [`DW_OP_le`](#dw_op_le)
  - [`DW_OP_lt`](#dw_op_lt)
  - [`DW_OP_ne`](#dw_op_ne)
  - [`DW_OP_skip`](#dw_op_skip)
  - [`DW_OP_lit0`](#dw_op_lit0)
  - [`DW_OP_lit1`](#dw_op_lit1)
  - [`DW_OP_lit2`](#dw_op_lit2)
  - [`DW_OP_lit3`](#dw_op_lit3)
  - [`DW_OP_lit4`](#dw_op_lit4)
  - [`DW_OP_lit5`](#dw_op_lit5)
  - [`DW_OP_lit6`](#dw_op_lit6)
  - [`DW_OP_lit7`](#dw_op_lit7)
  - [`DW_OP_lit8`](#dw_op_lit8)
  - [`DW_OP_lit9`](#dw_op_lit9)
  - [`DW_OP_lit10`](#dw_op_lit10)
  - [`DW_OP_lit11`](#dw_op_lit11)
  - [`DW_OP_lit12`](#dw_op_lit12)
  - [`DW_OP_lit13`](#dw_op_lit13)
  - [`DW_OP_lit14`](#dw_op_lit14)
  - [`DW_OP_lit15`](#dw_op_lit15)
  - [`DW_OP_lit16`](#dw_op_lit16)
  - [`DW_OP_lit17`](#dw_op_lit17)
  - [`DW_OP_lit18`](#dw_op_lit18)
  - [`DW_OP_lit19`](#dw_op_lit19)
  - [`DW_OP_lit20`](#dw_op_lit20)
  - [`DW_OP_lit21`](#dw_op_lit21)
  - [`DW_OP_lit22`](#dw_op_lit22)
  - [`DW_OP_lit23`](#dw_op_lit23)
  - [`DW_OP_lit24`](#dw_op_lit24)
  - [`DW_OP_lit25`](#dw_op_lit25)
  - [`DW_OP_lit26`](#dw_op_lit26)
  - [`DW_OP_lit27`](#dw_op_lit27)
  - [`DW_OP_lit28`](#dw_op_lit28)
  - [`DW_OP_lit29`](#dw_op_lit29)
  - [`DW_OP_lit30`](#dw_op_lit30)
  - [`DW_OP_lit31`](#dw_op_lit31)
  - [`DW_OP_reg0`](#dw_op_reg0)
  - [`DW_OP_reg1`](#dw_op_reg1)
  - [`DW_OP_reg2`](#dw_op_reg2)
  - [`DW_OP_reg3`](#dw_op_reg3)
  - [`DW_OP_reg4`](#dw_op_reg4)
  - [`DW_OP_reg5`](#dw_op_reg5)
  - [`DW_OP_reg6`](#dw_op_reg6)
  - [`DW_OP_reg7`](#dw_op_reg7)
  - [`DW_OP_reg8`](#dw_op_reg8)
  - [`DW_OP_reg9`](#dw_op_reg9)
  - [`DW_OP_reg10`](#dw_op_reg10)
  - [`DW_OP_reg11`](#dw_op_reg11)
  - [`DW_OP_reg12`](#dw_op_reg12)
  - [`DW_OP_reg13`](#dw_op_reg13)
  - [`DW_OP_reg14`](#dw_op_reg14)
  - [`DW_OP_reg15`](#dw_op_reg15)
  - [`DW_OP_reg16`](#dw_op_reg16)
  - [`DW_OP_reg17`](#dw_op_reg17)
  - [`DW_OP_reg18`](#dw_op_reg18)
  - [`DW_OP_reg19`](#dw_op_reg19)
  - [`DW_OP_reg20`](#dw_op_reg20)
  - [`DW_OP_reg21`](#dw_op_reg21)
  - [`DW_OP_reg22`](#dw_op_reg22)
  - [`DW_OP_reg23`](#dw_op_reg23)
  - [`DW_OP_reg24`](#dw_op_reg24)
  - [`DW_OP_reg25`](#dw_op_reg25)
  - [`DW_OP_reg26`](#dw_op_reg26)
  - [`DW_OP_reg27`](#dw_op_reg27)
  - [`DW_OP_reg28`](#dw_op_reg28)
  - [`DW_OP_reg29`](#dw_op_reg29)
  - [`DW_OP_reg30`](#dw_op_reg30)
  - [`DW_OP_reg31`](#dw_op_reg31)
  - [`DW_OP_breg0`](#dw_op_breg0)
  - [`DW_OP_breg1`](#dw_op_breg1)
  - [`DW_OP_breg2`](#dw_op_breg2)
  - [`DW_OP_breg3`](#dw_op_breg3)
  - [`DW_OP_breg4`](#dw_op_breg4)
  - [`DW_OP_breg5`](#dw_op_breg5)
  - [`DW_OP_breg6`](#dw_op_breg6)
  - [`DW_OP_breg7`](#dw_op_breg7)
  - [`DW_OP_breg8`](#dw_op_breg8)
  - [`DW_OP_breg9`](#dw_op_breg9)
  - [`DW_OP_breg10`](#dw_op_breg10)
  - [`DW_OP_breg11`](#dw_op_breg11)
  - [`DW_OP_breg12`](#dw_op_breg12)
  - [`DW_OP_breg13`](#dw_op_breg13)
  - [`DW_OP_breg14`](#dw_op_breg14)
  - [`DW_OP_breg15`](#dw_op_breg15)
  - [`DW_OP_breg16`](#dw_op_breg16)
  - [`DW_OP_breg17`](#dw_op_breg17)
  - [`DW_OP_breg18`](#dw_op_breg18)
  - [`DW_OP_breg19`](#dw_op_breg19)
  - [`DW_OP_breg20`](#dw_op_breg20)
  - [`DW_OP_breg21`](#dw_op_breg21)
  - [`DW_OP_breg22`](#dw_op_breg22)
  - [`DW_OP_breg23`](#dw_op_breg23)
  - [`DW_OP_breg24`](#dw_op_breg24)
  - [`DW_OP_breg25`](#dw_op_breg25)
  - [`DW_OP_breg26`](#dw_op_breg26)
  - [`DW_OP_breg27`](#dw_op_breg27)
  - [`DW_OP_breg28`](#dw_op_breg28)
  - [`DW_OP_breg29`](#dw_op_breg29)
  - [`DW_OP_breg30`](#dw_op_breg30)
  - [`DW_OP_breg31`](#dw_op_breg31)
  - [`DW_OP_regx`](#dw_op_regx)
  - [`DW_OP_fbreg`](#dw_op_fbreg)
  - [`DW_OP_bregx`](#dw_op_bregx)
  - [`DW_OP_piece`](#dw_op_piece)
  - [`DW_OP_deref_size`](#dw_op_deref_size)
  - [`DW_OP_xderef_size`](#dw_op_xderef_size)
  - [`DW_OP_nop`](#dw_op_nop)
  - [`DW_OP_push_object_address`](#dw_op_push_object_address)
  - [`DW_OP_call2`](#dw_op_call2)
  - [`DW_OP_call4`](#dw_op_call4)
  - [`DW_OP_call_ref`](#dw_op_call_ref)
  - [`DW_OP_form_tls_address`](#dw_op_form_tls_address)
  - [`DW_OP_call_frame_cfa`](#dw_op_call_frame_cfa)
  - [`DW_OP_bit_piece`](#dw_op_bit_piece)
  - [`DW_OP_implicit_value`](#dw_op_implicit_value)
  - [`DW_OP_stack_value`](#dw_op_stack_value)
  - [`DW_OP_implicit_pointer`](#dw_op_implicit_pointer)
  - [`DW_OP_addrx`](#dw_op_addrx)
  - [`DW_OP_constx`](#dw_op_constx)
  - [`DW_OP_entry_value`](#dw_op_entry_value)
  - [`DW_OP_const_type`](#dw_op_const_type)
  - [`DW_OP_regval_type`](#dw_op_regval_type)
  - [`DW_OP_deref_type`](#dw_op_deref_type)
  - [`DW_OP_xderef_type`](#dw_op_xderef_type)
  - [`DW_OP_convert`](#dw_op_convert)
  - [`DW_OP_reinterpret`](#dw_op_reinterpret)
  - [`DW_OP_GNU_push_tls_address`](#dw_op_gnu_push_tls_address)
  - [`DW_OP_GNU_implicit_pointer`](#dw_op_gnu_implicit_pointer)
  - [`DW_OP_GNU_entry_value`](#dw_op_gnu_entry_value)
  - [`DW_OP_GNU_const_type`](#dw_op_gnu_const_type)
  - [`DW_OP_GNU_regval_type`](#dw_op_gnu_regval_type)
  - [`DW_OP_GNU_deref_type`](#dw_op_gnu_deref_type)
  - [`DW_OP_GNU_convert`](#dw_op_gnu_convert)
  - [`DW_OP_GNU_reinterpret`](#dw_op_gnu_reinterpret)
  - [`DW_OP_GNU_parameter_ref`](#dw_op_gnu_parameter_ref)
  - [`DW_OP_GNU_addr_index`](#dw_op_gnu_addr_index)
  - [`DW_OP_GNU_const_index`](#dw_op_gnu_const_index)
  - [`DW_OP_WASM_location`](#dw_op_wasm_location)
  - [`DW_EH_PE_uleb128`](#dw_eh_pe_uleb128)
  - [`DW_EH_PE_udata2`](#dw_eh_pe_udata2)
  - [`DW_EH_PE_udata4`](#dw_eh_pe_udata4)
  - [`DW_EH_PE_udata8`](#dw_eh_pe_udata8)
  - [`DW_EH_PE_sleb128`](#dw_eh_pe_sleb128)
  - [`DW_EH_PE_sdata2`](#dw_eh_pe_sdata2)
  - [`DW_EH_PE_sdata4`](#dw_eh_pe_sdata4)
  - [`DW_EH_PE_sdata8`](#dw_eh_pe_sdata8)
  - [`DW_EH_PE_pcrel`](#dw_eh_pe_pcrel)
  - [`DW_EH_PE_textrel`](#dw_eh_pe_textrel)
  - [`DW_EH_PE_datarel`](#dw_eh_pe_datarel)
  - [`DW_EH_PE_funcrel`](#dw_eh_pe_funcrel)
  - [`DW_EH_PE_aligned`](#dw_eh_pe_aligned)
  - [`DW_EH_PE_indirect`](#dw_eh_pe_indirect)
  - [`DW_EH_PE_absptr`](#dw_eh_pe_absptr)
  - [`DW_EH_PE_omit`](#dw_eh_pe_omit)
  - [`DW_EH_PE_FORMAT_MASK`](#dw_eh_pe_format_mask)
  - [`DW_EH_PE_APPLICATION_MASK`](#dw_eh_pe_application_mask)
- [Macros](#macros)
  - [`registers!`](#registers)
  - [`dw!`](#dw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod |  |
| [`arch`](#arch) | mod |  |
| [`constants`](#constants) | mod | Constant definitions. |
| [`endianity`](#endianity) | mod | Types for compile-time and run-time endianity. |
| [`leb128`](#leb128) | mod | Read and write DWARF's "Little Endian Base 128" (LEB128) variable length |
| [`read`](#read) | mod | Read DWARF debugging information. |
| [`util`](#util) | mod |  |
| [`addr`](#addr) | mod |  |
| [`cfi`](#cfi) | mod |  |
| [`dwarf`](#dwarf) | mod |  |
| [`endian_slice`](#endian_slice) | mod | Working with byte slices that have an associated endianity. |
| [`reader`](#reader) | mod |  |
| [`relocate`](#relocate) | mod |  |
| [`abbrev`](#abbrev) | mod | Functions for parsing DWARF debugging abbreviations. |
| [`aranges`](#aranges) | mod |  |
| [`index`](#index) | mod |  |
| [`line`](#line) | mod |  |
| [`lists`](#lists) | mod |  |
| [`loclists`](#loclists) | mod |  |
| [`lookup`](#lookup) | mod |  |
| [`macros`](#macros) | mod |  |
| [`op`](#op) | mod | Functions for parsing and evaluating DWARF expressions. |
| [`pubnames`](#pubnames) | mod |  |
| [`pubtypes`](#pubtypes) | mod |  |
| [`rnglists`](#rnglists) | mod |  |
| [`str`](#str) | mod |  |
| [`unit`](#unit) | mod | Functions for parsing DWARF `.debug_info` and `.debug_types` sections. |
| [`value`](#value) | mod | Definitions for values used in DWARF expressions. |
| [`Encoding`](#encoding) | struct | Encoding parameters that are commonly used for multiple DWARF sections. |
| [`LineEncoding`](#lineencoding) | struct | Encoding parameters for a line number program. |
| [`Register`](#register) | struct | A DWARF register number. |
| [`DebugAbbrevOffset`](#debugabbrevoffset) | struct | An offset into the `.debug_abbrev` section. |
| [`DebugAddrOffset`](#debugaddroffset) | struct | An offset into the `.debug_addr` section. |
| [`DebugAddrBase`](#debugaddrbase) | struct | An offset to a set of entries in the `.debug_addr` section. |
| [`DebugAddrIndex`](#debugaddrindex) | struct | An index into a set of addresses in the `.debug_addr` section. |
| [`DebugArangesOffset`](#debugarangesoffset) | struct | An offset into the `.debug_aranges` section. |
| [`DebugInfoOffset`](#debuginfooffset) | struct | An offset into the `.debug_info` section. |
| [`DebugLineOffset`](#debuglineoffset) | struct | An offset into the `.debug_line` section. |
| [`DebugLineStrOffset`](#debuglinestroffset) | struct | An offset into the `.debug_line_str` section. |
| [`LocationListsOffset`](#locationlistsoffset) | struct | An offset into either the `.debug_loc` section or the `.debug_loclists` section |
| [`DebugLocListsBase`](#debugloclistsbase) | struct | An offset to a set of location list offsets in the `.debug_loclists` section. |
| [`DebugLocListsIndex`](#debugloclistsindex) | struct | An index into a set of location list offsets in the `.debug_loclists` section. |
| [`DebugMacinfoOffset`](#debugmacinfooffset) | struct | An offset into the `.debug_macinfo` section. |
| [`DebugMacroOffset`](#debugmacrooffset) | struct | An offset into the `.debug_macro` section. |
| [`RawRangeListsOffset`](#rawrangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section |
| [`RangeListsOffset`](#rangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section |
| [`DebugRngListsBase`](#debugrnglistsbase) | struct | An offset to a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugRngListsIndex`](#debugrnglistsindex) | struct | An index into a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugStrOffset`](#debugstroffset) | struct | An offset into the `.debug_str` section. |
| [`DebugStrOffsetsBase`](#debugstroffsetsbase) | struct | An offset to a set of entries in the `.debug_str_offsets` section. |
| [`DebugStrOffsetsIndex`](#debugstroffsetsindex) | struct | An index into a set of entries in the `.debug_str_offsets` section. |
| [`DebugTypesOffset`](#debugtypesoffset) | struct | An offset into the `.debug_types` section. |
| [`DebugTypeSignature`](#debugtypesignature) | struct | A type signature as used in the `.debug_types` section. |
| [`DebugFrameOffset`](#debugframeoffset) | struct | An offset into the `.debug_frame` section. |
| [`EhFrameOffset`](#ehframeoffset) | struct | An offset into the `.eh_frame` section. |
| [`DwoId`](#dwoid) | struct | An optionally-provided implementation-defined compilation unit ID to enable |
| [`Arm`](#arm) | struct | ARM architecture specific definitions. |
| [`AArch64`](#aarch64) | struct | ARM 64-bit (AArch64) architecture specific definitions. |
| [`LoongArch`](#loongarch) | struct | LoongArch architecture specific definitions. |
| [`MIPS`](#mips) | struct | MIPS architecture specific definitions. |
| [`RiscV`](#riscv) | struct | RISC-V architecture specific definitions. |
| [`X86`](#x86) | struct | Intel i386 architecture specific definitions. |
| [`X86_64`](#x86_64) | struct | AMD64 architecture specific definitions. |
| [`PowerPc64`](#powerpc64) | struct | PowerPC 64bit |
| [`DwSect`](#dwsect) | struct | The section type field in a `.dwp` unit index. |
| [`DwSectV2`](#dwsectv2) | struct | The section type field in a `.dwp` unit index with version 2. |
| [`DwUt`](#dwut) | struct | The unit type field in a unit header. |
| [`DwCfa`](#dwcfa) | struct | The opcode for a call frame instruction. |
| [`DwChildren`](#dwchildren) | struct | The child determination encodings for DIE attributes. |
| [`DwTag`](#dwtag) | struct | The tag encodings for DIE attributes. |
| [`DwAt`](#dwat) | struct | The attribute encodings for DIE attributes. |
| [`DwForm`](#dwform) | struct | The attribute form encodings for DIE attributes. |
| [`DwAte`](#dwate) | struct | The encodings of the constants used in the `DW_AT_encoding` attribute. |
| [`DwLle`](#dwlle) | struct | The encodings of the constants used in location list entries. |
| [`DwDs`](#dwds) | struct | The encodings of the constants used in the `DW_AT_decimal_sign` attribute. |
| [`DwEnd`](#dwend) | struct | The encodings of the constants used in the `DW_AT_endianity` attribute. |
| [`DwAccess`](#dwaccess) | struct | The encodings of the constants used in the `DW_AT_accessibility` attribute. |
| [`DwVis`](#dwvis) | struct | The encodings of the constants used in the `DW_AT_visibility` attribute. |
| [`DwVirtuality`](#dwvirtuality) | struct | The encodings of the constants used in the `DW_AT_virtuality` attribute. |
| [`DwLang`](#dwlang) | struct | The encodings of the constants used in the `DW_AT_language` attribute. |
| [`DwAddr`](#dwaddr) | struct | The encodings of the constants used in the `DW_AT_address_class` attribute. |
| [`DwId`](#dwid) | struct | The encodings of the constants used in the `DW_AT_identifier_case` attribute. |
| [`DwCc`](#dwcc) | struct | The encodings of the constants used in the `DW_AT_calling_convention` attribute. |
| [`DwInl`](#dwinl) | struct | The encodings of the constants used in the `DW_AT_inline` attribute. |
| [`DwOrd`](#dword) | struct | The encodings of the constants used in the `DW_AT_ordering` attribute. |
| [`DwDsc`](#dwdsc) | struct | The encodings of the constants used in the `DW_AT_discr_list` attribute. |
| [`DwIdx`](#dwidx) | struct | Name index attribute encodings. |
| [`DwDefaulted`](#dwdefaulted) | struct | The encodings of the constants used in the `DW_AT_defaulted` attribute. |
| [`DwLns`](#dwlns) | struct | The encodings for the standard opcodes for line number information. |
| [`DwLne`](#dwlne) | struct | The encodings for the extended opcodes for line number information. |
| [`DwLnct`](#dwlnct) | struct | The encodings for the line number header entry formats. |
| [`DwMacinfo`](#dwmacinfo) | struct | Type codes for macro definitions in the `.debug_macinfo` section. |
| [`DwMacro`](#dwmacro) | struct | The encodings for macro information entry types. |
| [`DwRle`](#dwrle) | struct | Range list entry encoding values. |
| [`DwOp`](#dwop) | struct | The encodings for DWARF expression operations. |
| [`DwEhPe`](#dwehpe) | struct | Pointer encoding used by `.eh_frame`. |
| [`LittleEndian`](#littleendian) | struct | Little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Big endian byte order. |
| [`UnitOffset`](#unitoffset) | struct | An offset into the current compilation or type unit. |
| [`StoreOnHeap`](#storeonheap) | struct | Indicates that storage should be allocated on heap. |
| [`Format`](#format) | enum | Whether the format of a compilation unit is 32- or 64-bit. |
| [`Vendor`](#vendor) | enum | Which vendor extensions to support. |
| [`UnitSectionOffset`](#unitsectionoffset) | enum | An offset into the `.debug_info` or `.debug_types` sections. |
| [`SectionId`](#sectionid) | enum | An identifier for a DWARF section. |
| [`DwarfFileType`](#dwarffiletype) | enum | The "type" of file with DWARF debugging information. |
| [`RunTimeEndian`](#runtimeendian) | enum | Byte order that is selectable at runtime. |
| [`Error`](#error) | enum | An error that occurred when parsing. |
| [`Endianity`](#endianity) | trait | A trait describing the endianity of some buffer. |
| [`Section`](#section) | trait | A convenience trait for loading DWARF sections from object files. |
| [`NativeEndian`](#nativeendian) | type | The native endianity for the target platform. |
| [`EndianBuf`](#endianbuf) | type | `EndianBuf` has been renamed to `EndianSlice`. |
| [`Result`](#result) | type | The result of a parse. |
| [`DW_SECT_INFO`](#dw_sect_info) | const |  |
| [`DW_SECT_ABBREV`](#dw_sect_abbrev) | const |  |
| [`DW_SECT_LINE`](#dw_sect_line) | const |  |
| [`DW_SECT_LOCLISTS`](#dw_sect_loclists) | const |  |
| [`DW_SECT_STR_OFFSETS`](#dw_sect_str_offsets) | const |  |
| [`DW_SECT_MACRO`](#dw_sect_macro) | const |  |
| [`DW_SECT_RNGLISTS`](#dw_sect_rnglists) | const |  |
| [`DW_SECT_V2_INFO`](#dw_sect_v2_info) | const |  |
| [`DW_SECT_V2_TYPES`](#dw_sect_v2_types) | const |  |
| [`DW_SECT_V2_ABBREV`](#dw_sect_v2_abbrev) | const |  |
| [`DW_SECT_V2_LINE`](#dw_sect_v2_line) | const |  |
| [`DW_SECT_V2_LOC`](#dw_sect_v2_loc) | const |  |
| [`DW_SECT_V2_STR_OFFSETS`](#dw_sect_v2_str_offsets) | const |  |
| [`DW_SECT_V2_MACINFO`](#dw_sect_v2_macinfo) | const |  |
| [`DW_SECT_V2_MACRO`](#dw_sect_v2_macro) | const |  |
| [`DW_UT_compile`](#dw_ut_compile) | const |  |
| [`DW_UT_type`](#dw_ut_type) | const |  |
| [`DW_UT_partial`](#dw_ut_partial) | const |  |
| [`DW_UT_skeleton`](#dw_ut_skeleton) | const |  |
| [`DW_UT_split_compile`](#dw_ut_split_compile) | const |  |
| [`DW_UT_split_type`](#dw_ut_split_type) | const |  |
| [`DW_UT_lo_user`](#dw_ut_lo_user) | const |  |
| [`DW_UT_hi_user`](#dw_ut_hi_user) | const |  |
| [`DW_CFA_advance_loc`](#dw_cfa_advance_loc) | const |  |
| [`DW_CFA_offset`](#dw_cfa_offset) | const |  |
| [`DW_CFA_restore`](#dw_cfa_restore) | const |  |
| [`DW_CFA_nop`](#dw_cfa_nop) | const |  |
| [`DW_CFA_set_loc`](#dw_cfa_set_loc) | const |  |
| [`DW_CFA_advance_loc1`](#dw_cfa_advance_loc1) | const |  |
| [`DW_CFA_advance_loc2`](#dw_cfa_advance_loc2) | const |  |
| [`DW_CFA_advance_loc4`](#dw_cfa_advance_loc4) | const |  |
| [`DW_CFA_offset_extended`](#dw_cfa_offset_extended) | const |  |
| [`DW_CFA_restore_extended`](#dw_cfa_restore_extended) | const |  |
| [`DW_CFA_undefined`](#dw_cfa_undefined) | const |  |
| [`DW_CFA_same_value`](#dw_cfa_same_value) | const |  |
| [`DW_CFA_register`](#dw_cfa_register) | const |  |
| [`DW_CFA_remember_state`](#dw_cfa_remember_state) | const |  |
| [`DW_CFA_restore_state`](#dw_cfa_restore_state) | const |  |
| [`DW_CFA_def_cfa`](#dw_cfa_def_cfa) | const |  |
| [`DW_CFA_def_cfa_register`](#dw_cfa_def_cfa_register) | const |  |
| [`DW_CFA_def_cfa_offset`](#dw_cfa_def_cfa_offset) | const |  |
| [`DW_CFA_def_cfa_expression`](#dw_cfa_def_cfa_expression) | const |  |
| [`DW_CFA_expression`](#dw_cfa_expression) | const |  |
| [`DW_CFA_offset_extended_sf`](#dw_cfa_offset_extended_sf) | const |  |
| [`DW_CFA_def_cfa_sf`](#dw_cfa_def_cfa_sf) | const |  |
| [`DW_CFA_def_cfa_offset_sf`](#dw_cfa_def_cfa_offset_sf) | const |  |
| [`DW_CFA_val_offset`](#dw_cfa_val_offset) | const |  |
| [`DW_CFA_val_offset_sf`](#dw_cfa_val_offset_sf) | const |  |
| [`DW_CFA_val_expression`](#dw_cfa_val_expression) | const |  |
| [`DW_CFA_lo_user`](#dw_cfa_lo_user) | const |  |
| [`DW_CFA_hi_user`](#dw_cfa_hi_user) | const |  |
| [`DW_CFA_MIPS_advance_loc8`](#dw_cfa_mips_advance_loc8) | const |  |
| [`DW_CFA_GNU_window_save`](#dw_cfa_gnu_window_save) | const |  |
| [`DW_CFA_GNU_args_size`](#dw_cfa_gnu_args_size) | const |  |
| [`DW_CFA_GNU_negative_offset_extended`](#dw_cfa_gnu_negative_offset_extended) | const |  |
| [`DW_CFA_AARCH64_negate_ra_state`](#dw_cfa_aarch64_negate_ra_state) | const |  |
| [`DW_CHILDREN_no`](#dw_children_no) | const |  |
| [`DW_CHILDREN_yes`](#dw_children_yes) | const |  |
| [`DW_TAG_null`](#dw_tag_null) | const |  |
| [`DW_TAG_global_subroutine`](#dw_tag_global_subroutine) | const |  |
| [`DW_TAG_global_variable`](#dw_tag_global_variable) | const |  |
| [`DW_TAG_local_variable`](#dw_tag_local_variable) | const |  |
| [`DW_TAG_subroutine`](#dw_tag_subroutine) | const |  |
| [`DW_TAG_array_type`](#dw_tag_array_type) | const |  |
| [`DW_TAG_class_type`](#dw_tag_class_type) | const |  |
| [`DW_TAG_entry_point`](#dw_tag_entry_point) | const |  |
| [`DW_TAG_enumeration_type`](#dw_tag_enumeration_type) | const |  |
| [`DW_TAG_formal_parameter`](#dw_tag_formal_parameter) | const |  |
| [`DW_TAG_imported_declaration`](#dw_tag_imported_declaration) | const |  |
| [`DW_TAG_label`](#dw_tag_label) | const |  |
| [`DW_TAG_lexical_block`](#dw_tag_lexical_block) | const |  |
| [`DW_TAG_member`](#dw_tag_member) | const |  |
| [`DW_TAG_pointer_type`](#dw_tag_pointer_type) | const |  |
| [`DW_TAG_reference_type`](#dw_tag_reference_type) | const |  |
| [`DW_TAG_compile_unit`](#dw_tag_compile_unit) | const |  |
| [`DW_TAG_string_type`](#dw_tag_string_type) | const |  |
| [`DW_TAG_structure_type`](#dw_tag_structure_type) | const |  |
| [`DW_TAG_subroutine_type`](#dw_tag_subroutine_type) | const |  |
| [`DW_TAG_typedef`](#dw_tag_typedef) | const |  |
| [`DW_TAG_union_type`](#dw_tag_union_type) | const |  |
| [`DW_TAG_unspecified_parameters`](#dw_tag_unspecified_parameters) | const |  |
| [`DW_TAG_variant`](#dw_tag_variant) | const |  |
| [`DW_TAG_common_block`](#dw_tag_common_block) | const |  |
| [`DW_TAG_common_inclusion`](#dw_tag_common_inclusion) | const |  |
| [`DW_TAG_inheritance`](#dw_tag_inheritance) | const |  |
| [`DW_TAG_inlined_subroutine`](#dw_tag_inlined_subroutine) | const |  |
| [`DW_TAG_module`](#dw_tag_module) | const |  |
| [`DW_TAG_ptr_to_member_type`](#dw_tag_ptr_to_member_type) | const |  |
| [`DW_TAG_set_type`](#dw_tag_set_type) | const |  |
| [`DW_TAG_subrange_type`](#dw_tag_subrange_type) | const |  |
| [`DW_TAG_with_stmt`](#dw_tag_with_stmt) | const |  |
| [`DW_TAG_access_declaration`](#dw_tag_access_declaration) | const |  |
| [`DW_TAG_base_type`](#dw_tag_base_type) | const |  |
| [`DW_TAG_catch_block`](#dw_tag_catch_block) | const |  |
| [`DW_TAG_const_type`](#dw_tag_const_type) | const |  |
| [`DW_TAG_constant`](#dw_tag_constant) | const |  |
| [`DW_TAG_enumerator`](#dw_tag_enumerator) | const |  |
| [`DW_TAG_file_type`](#dw_tag_file_type) | const |  |
| [`DW_TAG_friend`](#dw_tag_friend) | const |  |
| [`DW_TAG_namelist`](#dw_tag_namelist) | const |  |
| [`DW_TAG_namelist_item`](#dw_tag_namelist_item) | const |  |
| [`DW_TAG_packed_type`](#dw_tag_packed_type) | const |  |
| [`DW_TAG_subprogram`](#dw_tag_subprogram) | const |  |
| [`DW_TAG_template_type_parameter`](#dw_tag_template_type_parameter) | const |  |
| [`DW_TAG_template_value_parameter`](#dw_tag_template_value_parameter) | const |  |
| [`DW_TAG_thrown_type`](#dw_tag_thrown_type) | const |  |
| [`DW_TAG_try_block`](#dw_tag_try_block) | const |  |
| [`DW_TAG_variant_part`](#dw_tag_variant_part) | const |  |
| [`DW_TAG_variable`](#dw_tag_variable) | const |  |
| [`DW_TAG_volatile_type`](#dw_tag_volatile_type) | const |  |
| [`DW_TAG_dwarf_procedure`](#dw_tag_dwarf_procedure) | const |  |
| [`DW_TAG_restrict_type`](#dw_tag_restrict_type) | const |  |
| [`DW_TAG_interface_type`](#dw_tag_interface_type) | const |  |
| [`DW_TAG_namespace`](#dw_tag_namespace) | const |  |
| [`DW_TAG_imported_module`](#dw_tag_imported_module) | const |  |
| [`DW_TAG_unspecified_type`](#dw_tag_unspecified_type) | const |  |
| [`DW_TAG_partial_unit`](#dw_tag_partial_unit) | const |  |
| [`DW_TAG_imported_unit`](#dw_tag_imported_unit) | const |  |
| [`DW_TAG_condition`](#dw_tag_condition) | const |  |
| [`DW_TAG_shared_type`](#dw_tag_shared_type) | const |  |
| [`DW_TAG_type_unit`](#dw_tag_type_unit) | const |  |
| [`DW_TAG_rvalue_reference_type`](#dw_tag_rvalue_reference_type) | const |  |
| [`DW_TAG_template_alias`](#dw_tag_template_alias) | const |  |
| [`DW_TAG_coarray_type`](#dw_tag_coarray_type) | const |  |
| [`DW_TAG_generic_subrange`](#dw_tag_generic_subrange) | const |  |
| [`DW_TAG_dynamic_type`](#dw_tag_dynamic_type) | const |  |
| [`DW_TAG_atomic_type`](#dw_tag_atomic_type) | const |  |
| [`DW_TAG_call_site`](#dw_tag_call_site) | const |  |
| [`DW_TAG_call_site_parameter`](#dw_tag_call_site_parameter) | const |  |
| [`DW_TAG_skeleton_unit`](#dw_tag_skeleton_unit) | const |  |
| [`DW_TAG_immutable_type`](#dw_tag_immutable_type) | const |  |
| [`DW_TAG_lo_user`](#dw_tag_lo_user) | const |  |
| [`DW_TAG_hi_user`](#dw_tag_hi_user) | const |  |
| [`DW_TAG_MIPS_loop`](#dw_tag_mips_loop) | const |  |
| [`DW_TAG_HP_array_descriptor`](#dw_tag_hp_array_descriptor) | const |  |
| [`DW_TAG_HP_Bliss_field`](#dw_tag_hp_bliss_field) | const |  |
| [`DW_TAG_HP_Bliss_field_set`](#dw_tag_hp_bliss_field_set) | const |  |
| [`DW_TAG_format_label`](#dw_tag_format_label) | const |  |
| [`DW_TAG_function_template`](#dw_tag_function_template) | const |  |
| [`DW_TAG_class_template`](#dw_tag_class_template) | const |  |
| [`DW_TAG_GNU_BINCL`](#dw_tag_gnu_bincl) | const |  |
| [`DW_TAG_GNU_EINCL`](#dw_tag_gnu_eincl) | const |  |
| [`DW_TAG_GNU_template_template_param`](#dw_tag_gnu_template_template_param) | const |  |
| [`DW_TAG_GNU_template_parameter_pack`](#dw_tag_gnu_template_parameter_pack) | const |  |
| [`DW_TAG_GNU_formal_parameter_pack`](#dw_tag_gnu_formal_parameter_pack) | const |  |
| [`DW_TAG_GNU_call_site`](#dw_tag_gnu_call_site) | const |  |
| [`DW_TAG_GNU_call_site_parameter`](#dw_tag_gnu_call_site_parameter) | const |  |
| [`DW_TAG_APPLE_property`](#dw_tag_apple_property) | const |  |
| [`DW_TAG_SUN_function_template`](#dw_tag_sun_function_template) | const |  |
| [`DW_TAG_SUN_class_template`](#dw_tag_sun_class_template) | const |  |
| [`DW_TAG_SUN_struct_template`](#dw_tag_sun_struct_template) | const |  |
| [`DW_TAG_SUN_union_template`](#dw_tag_sun_union_template) | const |  |
| [`DW_TAG_SUN_indirect_inheritance`](#dw_tag_sun_indirect_inheritance) | const |  |
| [`DW_TAG_SUN_codeflags`](#dw_tag_sun_codeflags) | const |  |
| [`DW_TAG_SUN_memop_info`](#dw_tag_sun_memop_info) | const |  |
| [`DW_TAG_SUN_omp_child_func`](#dw_tag_sun_omp_child_func) | const |  |
| [`DW_TAG_SUN_rtti_descriptor`](#dw_tag_sun_rtti_descriptor) | const |  |
| [`DW_TAG_SUN_dtor_info`](#dw_tag_sun_dtor_info) | const |  |
| [`DW_TAG_SUN_dtor`](#dw_tag_sun_dtor) | const |  |
| [`DW_TAG_SUN_f90_interface`](#dw_tag_sun_f90_interface) | const |  |
| [`DW_TAG_SUN_fortran_vax_structure`](#dw_tag_sun_fortran_vax_structure) | const |  |
| [`DW_TAG_ALTIUM_circ_type`](#dw_tag_altium_circ_type) | const |  |
| [`DW_TAG_ALTIUM_mwa_circ_type`](#dw_tag_altium_mwa_circ_type) | const |  |
| [`DW_TAG_ALTIUM_rev_carry_type`](#dw_tag_altium_rev_carry_type) | const |  |
| [`DW_TAG_ALTIUM_rom`](#dw_tag_altium_rom) | const |  |
| [`DW_TAG_upc_shared_type`](#dw_tag_upc_shared_type) | const |  |
| [`DW_TAG_upc_strict_type`](#dw_tag_upc_strict_type) | const |  |
| [`DW_TAG_upc_relaxed_type`](#dw_tag_upc_relaxed_type) | const |  |
| [`DW_TAG_PGI_kanji_type`](#dw_tag_pgi_kanji_type) | const |  |
| [`DW_TAG_PGI_interface_block`](#dw_tag_pgi_interface_block) | const |  |
| [`DW_TAG_BORLAND_property`](#dw_tag_borland_property) | const |  |
| [`DW_TAG_BORLAND_Delphi_string`](#dw_tag_borland_delphi_string) | const |  |
| [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw_tag_borland_delphi_dynamic_array) | const |  |
| [`DW_TAG_BORLAND_Delphi_set`](#dw_tag_borland_delphi_set) | const |  |
| [`DW_TAG_BORLAND_Delphi_variant`](#dw_tag_borland_delphi_variant) | const |  |
| [`DW_AT_null`](#dw_at_null) | const |  |
| [`DW_AT_fund_type`](#dw_at_fund_type) | const |  |
| [`DW_AT_mod_fund_type`](#dw_at_mod_fund_type) | const |  |
| [`DW_AT_user_def_type`](#dw_at_user_def_type) | const |  |
| [`DW_AT_mod_u_d_type`](#dw_at_mod_u_d_type) | const |  |
| [`DW_AT_subscr_data`](#dw_at_subscr_data) | const |  |
| [`DW_AT_element_list`](#dw_at_element_list) | const |  |
| [`DW_AT_member`](#dw_at_member) | const |  |
| [`DW_AT_friends`](#dw_at_friends) | const |  |
| [`DW_AT_program`](#dw_at_program) | const |  |
| [`DW_AT_private`](#dw_at_private) | const |  |
| [`DW_AT_protected`](#dw_at_protected) | const |  |
| [`DW_AT_public`](#dw_at_public) | const |  |
| [`DW_AT_pure_virtual`](#dw_at_pure_virtual) | const |  |
| [`DW_AT_virtual`](#dw_at_virtual) | const |  |
| [`DW_AT_specification_v1`](#dw_at_specification_v1) | const |  |
| [`DW_AT_sibling`](#dw_at_sibling) | const |  |
| [`DW_AT_location`](#dw_at_location) | const |  |
| [`DW_AT_name`](#dw_at_name) | const |  |
| [`DW_AT_ordering`](#dw_at_ordering) | const |  |
| [`DW_AT_byte_size`](#dw_at_byte_size) | const |  |
| [`DW_AT_bit_offset`](#dw_at_bit_offset) | const |  |
| [`DW_AT_bit_size`](#dw_at_bit_size) | const |  |
| [`DW_AT_stmt_list`](#dw_at_stmt_list) | const |  |
| [`DW_AT_low_pc`](#dw_at_low_pc) | const |  |
| [`DW_AT_high_pc`](#dw_at_high_pc) | const |  |
| [`DW_AT_language`](#dw_at_language) | const |  |
| [`DW_AT_discr`](#dw_at_discr) | const |  |
| [`DW_AT_discr_value`](#dw_at_discr_value) | const |  |
| [`DW_AT_visibility`](#dw_at_visibility) | const |  |
| [`DW_AT_import`](#dw_at_import) | const |  |
| [`DW_AT_string_length`](#dw_at_string_length) | const |  |
| [`DW_AT_common_reference`](#dw_at_common_reference) | const |  |
| [`DW_AT_comp_dir`](#dw_at_comp_dir) | const |  |
| [`DW_AT_const_value`](#dw_at_const_value) | const |  |
| [`DW_AT_containing_type`](#dw_at_containing_type) | const |  |
| [`DW_AT_default_value`](#dw_at_default_value) | const |  |
| [`DW_AT_inline`](#dw_at_inline) | const |  |
| [`DW_AT_is_optional`](#dw_at_is_optional) | const |  |
| [`DW_AT_lower_bound`](#dw_at_lower_bound) | const |  |
| [`DW_AT_producer`](#dw_at_producer) | const |  |
| [`DW_AT_prototyped`](#dw_at_prototyped) | const |  |
| [`DW_AT_return_addr`](#dw_at_return_addr) | const |  |
| [`DW_AT_start_scope`](#dw_at_start_scope) | const |  |
| [`DW_AT_bit_stride`](#dw_at_bit_stride) | const |  |
| [`DW_AT_upper_bound`](#dw_at_upper_bound) | const |  |
| [`DW_AT_abstract_origin`](#dw_at_abstract_origin) | const |  |
| [`DW_AT_accessibility`](#dw_at_accessibility) | const |  |
| [`DW_AT_address_class`](#dw_at_address_class) | const |  |
| [`DW_AT_artificial`](#dw_at_artificial) | const |  |
| [`DW_AT_base_types`](#dw_at_base_types) | const |  |
| [`DW_AT_calling_convention`](#dw_at_calling_convention) | const |  |
| [`DW_AT_count`](#dw_at_count) | const |  |
| [`DW_AT_data_member_location`](#dw_at_data_member_location) | const |  |
| [`DW_AT_decl_column`](#dw_at_decl_column) | const |  |
| [`DW_AT_decl_file`](#dw_at_decl_file) | const |  |
| [`DW_AT_decl_line`](#dw_at_decl_line) | const |  |
| [`DW_AT_declaration`](#dw_at_declaration) | const |  |
| [`DW_AT_discr_list`](#dw_at_discr_list) | const |  |
| [`DW_AT_encoding`](#dw_at_encoding) | const |  |
| [`DW_AT_external`](#dw_at_external) | const |  |
| [`DW_AT_frame_base`](#dw_at_frame_base) | const |  |
| [`DW_AT_friend`](#dw_at_friend) | const |  |
| [`DW_AT_identifier_case`](#dw_at_identifier_case) | const |  |
| [`DW_AT_macro_info`](#dw_at_macro_info) | const |  |
| [`DW_AT_namelist_item`](#dw_at_namelist_item) | const |  |
| [`DW_AT_priority`](#dw_at_priority) | const |  |
| [`DW_AT_segment`](#dw_at_segment) | const |  |
| [`DW_AT_specification`](#dw_at_specification) | const |  |
| [`DW_AT_static_link`](#dw_at_static_link) | const |  |
| [`DW_AT_type`](#dw_at_type) | const |  |
| [`DW_AT_use_location`](#dw_at_use_location) | const |  |
| [`DW_AT_variable_parameter`](#dw_at_variable_parameter) | const |  |
| [`DW_AT_virtuality`](#dw_at_virtuality) | const |  |
| [`DW_AT_vtable_elem_location`](#dw_at_vtable_elem_location) | const |  |
| [`DW_AT_allocated`](#dw_at_allocated) | const |  |
| [`DW_AT_associated`](#dw_at_associated) | const |  |
| [`DW_AT_data_location`](#dw_at_data_location) | const |  |
| [`DW_AT_byte_stride`](#dw_at_byte_stride) | const |  |
| [`DW_AT_entry_pc`](#dw_at_entry_pc) | const |  |
| [`DW_AT_use_UTF8`](#dw_at_use_utf8) | const |  |
| [`DW_AT_extension`](#dw_at_extension) | const |  |
| [`DW_AT_ranges`](#dw_at_ranges) | const |  |
| [`DW_AT_trampoline`](#dw_at_trampoline) | const |  |
| [`DW_AT_call_column`](#dw_at_call_column) | const |  |
| [`DW_AT_call_file`](#dw_at_call_file) | const |  |
| [`DW_AT_call_line`](#dw_at_call_line) | const |  |
| [`DW_AT_description`](#dw_at_description) | const |  |
| [`DW_AT_binary_scale`](#dw_at_binary_scale) | const |  |
| [`DW_AT_decimal_scale`](#dw_at_decimal_scale) | const |  |
| [`DW_AT_small`](#dw_at_small) | const |  |
| [`DW_AT_decimal_sign`](#dw_at_decimal_sign) | const |  |
| [`DW_AT_digit_count`](#dw_at_digit_count) | const |  |
| [`DW_AT_picture_string`](#dw_at_picture_string) | const |  |
| [`DW_AT_mutable`](#dw_at_mutable) | const |  |
| [`DW_AT_threads_scaled`](#dw_at_threads_scaled) | const |  |
| [`DW_AT_explicit`](#dw_at_explicit) | const |  |
| [`DW_AT_object_pointer`](#dw_at_object_pointer) | const |  |
| [`DW_AT_endianity`](#dw_at_endianity) | const |  |
| [`DW_AT_elemental`](#dw_at_elemental) | const |  |
| [`DW_AT_pure`](#dw_at_pure) | const |  |
| [`DW_AT_recursive`](#dw_at_recursive) | const |  |
| [`DW_AT_signature`](#dw_at_signature) | const |  |
| [`DW_AT_main_subprogram`](#dw_at_main_subprogram) | const |  |
| [`DW_AT_data_bit_offset`](#dw_at_data_bit_offset) | const |  |
| [`DW_AT_const_expr`](#dw_at_const_expr) | const |  |
| [`DW_AT_enum_class`](#dw_at_enum_class) | const |  |
| [`DW_AT_linkage_name`](#dw_at_linkage_name) | const |  |
| [`DW_AT_string_length_bit_size`](#dw_at_string_length_bit_size) | const |  |
| [`DW_AT_string_length_byte_size`](#dw_at_string_length_byte_size) | const |  |
| [`DW_AT_rank`](#dw_at_rank) | const |  |
| [`DW_AT_str_offsets_base`](#dw_at_str_offsets_base) | const |  |
| [`DW_AT_addr_base`](#dw_at_addr_base) | const |  |
| [`DW_AT_rnglists_base`](#dw_at_rnglists_base) | const |  |
| [`DW_AT_dwo_name`](#dw_at_dwo_name) | const |  |
| [`DW_AT_reference`](#dw_at_reference) | const |  |
| [`DW_AT_rvalue_reference`](#dw_at_rvalue_reference) | const |  |
| [`DW_AT_macros`](#dw_at_macros) | const |  |
| [`DW_AT_call_all_calls`](#dw_at_call_all_calls) | const |  |
| [`DW_AT_call_all_source_calls`](#dw_at_call_all_source_calls) | const |  |
| [`DW_AT_call_all_tail_calls`](#dw_at_call_all_tail_calls) | const |  |
| [`DW_AT_call_return_pc`](#dw_at_call_return_pc) | const |  |
| [`DW_AT_call_value`](#dw_at_call_value) | const |  |
| [`DW_AT_call_origin`](#dw_at_call_origin) | const |  |
| [`DW_AT_call_parameter`](#dw_at_call_parameter) | const |  |
| [`DW_AT_call_pc`](#dw_at_call_pc) | const |  |
| [`DW_AT_call_tail_call`](#dw_at_call_tail_call) | const |  |
| [`DW_AT_call_target`](#dw_at_call_target) | const |  |
| [`DW_AT_call_target_clobbered`](#dw_at_call_target_clobbered) | const |  |
| [`DW_AT_call_data_location`](#dw_at_call_data_location) | const |  |
| [`DW_AT_call_data_value`](#dw_at_call_data_value) | const |  |
| [`DW_AT_noreturn`](#dw_at_noreturn) | const |  |
| [`DW_AT_alignment`](#dw_at_alignment) | const |  |
| [`DW_AT_export_symbols`](#dw_at_export_symbols) | const |  |
| [`DW_AT_deleted`](#dw_at_deleted) | const |  |
| [`DW_AT_defaulted`](#dw_at_defaulted) | const |  |
| [`DW_AT_loclists_base`](#dw_at_loclists_base) | const |  |
| [`DW_AT_lo_user`](#dw_at_lo_user) | const |  |
| [`DW_AT_hi_user`](#dw_at_hi_user) | const |  |
| [`DW_AT_MIPS_fde`](#dw_at_mips_fde) | const |  |
| [`DW_AT_MIPS_loop_begin`](#dw_at_mips_loop_begin) | const |  |
| [`DW_AT_MIPS_tail_loop_begin`](#dw_at_mips_tail_loop_begin) | const |  |
| [`DW_AT_MIPS_epilog_begin`](#dw_at_mips_epilog_begin) | const |  |
| [`DW_AT_MIPS_loop_unroll_factor`](#dw_at_mips_loop_unroll_factor) | const |  |
| [`DW_AT_MIPS_software_pipeline_depth`](#dw_at_mips_software_pipeline_depth) | const |  |
| [`DW_AT_MIPS_linkage_name`](#dw_at_mips_linkage_name) | const |  |
| [`DW_AT_MIPS_stride`](#dw_at_mips_stride) | const |  |
| [`DW_AT_MIPS_abstract_name`](#dw_at_mips_abstract_name) | const |  |
| [`DW_AT_MIPS_clone_origin`](#dw_at_mips_clone_origin) | const |  |
| [`DW_AT_MIPS_has_inlines`](#dw_at_mips_has_inlines) | const |  |
| [`DW_AT_MIPS_stride_byte`](#dw_at_mips_stride_byte) | const |  |
| [`DW_AT_MIPS_stride_elem`](#dw_at_mips_stride_elem) | const |  |
| [`DW_AT_MIPS_ptr_dopetype`](#dw_at_mips_ptr_dopetype) | const |  |
| [`DW_AT_MIPS_allocatable_dopetype`](#dw_at_mips_allocatable_dopetype) | const |  |
| [`DW_AT_MIPS_assumed_shape_dopetype`](#dw_at_mips_assumed_shape_dopetype) | const |  |
| [`DW_AT_MIPS_assumed_size`](#dw_at_mips_assumed_size) | const |  |
| [`DW_AT_INTEL_other_endian`](#dw_at_intel_other_endian) | const |  |
| [`DW_AT_sf_names`](#dw_at_sf_names) | const |  |
| [`DW_AT_src_info`](#dw_at_src_info) | const |  |
| [`DW_AT_mac_info`](#dw_at_mac_info) | const |  |
| [`DW_AT_src_coords`](#dw_at_src_coords) | const |  |
| [`DW_AT_body_begin`](#dw_at_body_begin) | const |  |
| [`DW_AT_body_end`](#dw_at_body_end) | const |  |
| [`DW_AT_GNU_vector`](#dw_at_gnu_vector) | const |  |
| [`DW_AT_GNU_guarded_by`](#dw_at_gnu_guarded_by) | const |  |
| [`DW_AT_GNU_pt_guarded_by`](#dw_at_gnu_pt_guarded_by) | const |  |
| [`DW_AT_GNU_guarded`](#dw_at_gnu_guarded) | const |  |
| [`DW_AT_GNU_pt_guarded`](#dw_at_gnu_pt_guarded) | const |  |
| [`DW_AT_GNU_locks_excluded`](#dw_at_gnu_locks_excluded) | const |  |
| [`DW_AT_GNU_exclusive_locks_required`](#dw_at_gnu_exclusive_locks_required) | const |  |
| [`DW_AT_GNU_shared_locks_required`](#dw_at_gnu_shared_locks_required) | const |  |
| [`DW_AT_GNU_odr_signature`](#dw_at_gnu_odr_signature) | const |  |
| [`DW_AT_GNU_template_name`](#dw_at_gnu_template_name) | const |  |
| [`DW_AT_GNU_call_site_value`](#dw_at_gnu_call_site_value) | const |  |
| [`DW_AT_GNU_call_site_data_value`](#dw_at_gnu_call_site_data_value) | const |  |
| [`DW_AT_GNU_call_site_target`](#dw_at_gnu_call_site_target) | const |  |
| [`DW_AT_GNU_call_site_target_clobbered`](#dw_at_gnu_call_site_target_clobbered) | const |  |
| [`DW_AT_GNU_tail_call`](#dw_at_gnu_tail_call) | const |  |
| [`DW_AT_GNU_all_tail_call_sites`](#dw_at_gnu_all_tail_call_sites) | const |  |
| [`DW_AT_GNU_all_call_sites`](#dw_at_gnu_all_call_sites) | const |  |
| [`DW_AT_GNU_all_source_call_sites`](#dw_at_gnu_all_source_call_sites) | const |  |
| [`DW_AT_GNU_macros`](#dw_at_gnu_macros) | const |  |
| [`DW_AT_GNU_deleted`](#dw_at_gnu_deleted) | const |  |
| [`DW_AT_GNU_dwo_name`](#dw_at_gnu_dwo_name) | const |  |
| [`DW_AT_GNU_dwo_id`](#dw_at_gnu_dwo_id) | const |  |
| [`DW_AT_GNU_ranges_base`](#dw_at_gnu_ranges_base) | const |  |
| [`DW_AT_GNU_addr_base`](#dw_at_gnu_addr_base) | const |  |
| [`DW_AT_GNU_pubnames`](#dw_at_gnu_pubnames) | const |  |
| [`DW_AT_GNU_pubtypes`](#dw_at_gnu_pubtypes) | const |  |
| [`DW_AT_GNU_discriminator`](#dw_at_gnu_discriminator) | const |  |
| [`DW_AT_GNU_locviews`](#dw_at_gnu_locviews) | const |  |
| [`DW_AT_GNU_entry_view`](#dw_at_gnu_entry_view) | const |  |
| [`DW_AT_SUN_template`](#dw_at_sun_template) | const |  |
| [`DW_AT_SUN_alignment`](#dw_at_sun_alignment) | const |  |
| [`DW_AT_SUN_vtable`](#dw_at_sun_vtable) | const |  |
| [`DW_AT_SUN_count_guarantee`](#dw_at_sun_count_guarantee) | const |  |
| [`DW_AT_SUN_command_line`](#dw_at_sun_command_line) | const |  |
| [`DW_AT_SUN_vbase`](#dw_at_sun_vbase) | const |  |
| [`DW_AT_SUN_compile_options`](#dw_at_sun_compile_options) | const |  |
| [`DW_AT_SUN_language`](#dw_at_sun_language) | const |  |
| [`DW_AT_SUN_browser_file`](#dw_at_sun_browser_file) | const |  |
| [`DW_AT_SUN_vtable_abi`](#dw_at_sun_vtable_abi) | const |  |
| [`DW_AT_SUN_func_offsets`](#dw_at_sun_func_offsets) | const |  |
| [`DW_AT_SUN_cf_kind`](#dw_at_sun_cf_kind) | const |  |
| [`DW_AT_SUN_vtable_index`](#dw_at_sun_vtable_index) | const |  |
| [`DW_AT_SUN_omp_tpriv_addr`](#dw_at_sun_omp_tpriv_addr) | const |  |
| [`DW_AT_SUN_omp_child_func`](#dw_at_sun_omp_child_func) | const |  |
| [`DW_AT_SUN_func_offset`](#dw_at_sun_func_offset) | const |  |
| [`DW_AT_SUN_memop_type_ref`](#dw_at_sun_memop_type_ref) | const |  |
| [`DW_AT_SUN_profile_id`](#dw_at_sun_profile_id) | const |  |
| [`DW_AT_SUN_memop_signature`](#dw_at_sun_memop_signature) | const |  |
| [`DW_AT_SUN_obj_dir`](#dw_at_sun_obj_dir) | const |  |
| [`DW_AT_SUN_obj_file`](#dw_at_sun_obj_file) | const |  |
| [`DW_AT_SUN_original_name`](#dw_at_sun_original_name) | const |  |
| [`DW_AT_SUN_hwcprof_signature`](#dw_at_sun_hwcprof_signature) | const |  |
| [`DW_AT_SUN_amd64_parmdump`](#dw_at_sun_amd64_parmdump) | const |  |
| [`DW_AT_SUN_part_link_name`](#dw_at_sun_part_link_name) | const |  |
| [`DW_AT_SUN_link_name`](#dw_at_sun_link_name) | const |  |
| [`DW_AT_SUN_pass_with_const`](#dw_at_sun_pass_with_const) | const |  |
| [`DW_AT_SUN_return_with_const`](#dw_at_sun_return_with_const) | const |  |
| [`DW_AT_SUN_import_by_name`](#dw_at_sun_import_by_name) | const |  |
| [`DW_AT_SUN_f90_pointer`](#dw_at_sun_f90_pointer) | const |  |
| [`DW_AT_SUN_pass_by_ref`](#dw_at_sun_pass_by_ref) | const |  |
| [`DW_AT_SUN_f90_allocatable`](#dw_at_sun_f90_allocatable) | const |  |
| [`DW_AT_SUN_f90_assumed_shape_array`](#dw_at_sun_f90_assumed_shape_array) | const |  |
| [`DW_AT_SUN_c_vla`](#dw_at_sun_c_vla) | const |  |
| [`DW_AT_SUN_return_value_ptr`](#dw_at_sun_return_value_ptr) | const |  |
| [`DW_AT_SUN_dtor_start`](#dw_at_sun_dtor_start) | const |  |
| [`DW_AT_SUN_dtor_length`](#dw_at_sun_dtor_length) | const |  |
| [`DW_AT_SUN_dtor_state_initial`](#dw_at_sun_dtor_state_initial) | const |  |
| [`DW_AT_SUN_dtor_state_final`](#dw_at_sun_dtor_state_final) | const |  |
| [`DW_AT_SUN_dtor_state_deltas`](#dw_at_sun_dtor_state_deltas) | const |  |
| [`DW_AT_SUN_import_by_lname`](#dw_at_sun_import_by_lname) | const |  |
| [`DW_AT_SUN_f90_use_only`](#dw_at_sun_f90_use_only) | const |  |
| [`DW_AT_SUN_namelist_spec`](#dw_at_sun_namelist_spec) | const |  |
| [`DW_AT_SUN_is_omp_child_func`](#dw_at_sun_is_omp_child_func) | const |  |
| [`DW_AT_SUN_fortran_main_alias`](#dw_at_sun_fortran_main_alias) | const |  |
| [`DW_AT_SUN_fortran_based`](#dw_at_sun_fortran_based) | const |  |
| [`DW_AT_ALTIUM_loclist`](#dw_at_altium_loclist) | const |  |
| [`DW_AT_use_GNAT_descriptive_type`](#dw_at_use_gnat_descriptive_type) | const |  |
| [`DW_AT_GNAT_descriptive_type`](#dw_at_gnat_descriptive_type) | const |  |
| [`DW_AT_GNU_numerator`](#dw_at_gnu_numerator) | const |  |
| [`DW_AT_GNU_denominator`](#dw_at_gnu_denominator) | const |  |
| [`DW_AT_GNU_bias`](#dw_at_gnu_bias) | const |  |
| [`DW_AT_upc_threads_scaled`](#dw_at_upc_threads_scaled) | const |  |
| [`DW_AT_PGI_lbase`](#dw_at_pgi_lbase) | const |  |
| [`DW_AT_PGI_soffset`](#dw_at_pgi_soffset) | const |  |
| [`DW_AT_PGI_lstride`](#dw_at_pgi_lstride) | const |  |
| [`DW_AT_BORLAND_property_read`](#dw_at_borland_property_read) | const |  |
| [`DW_AT_BORLAND_property_write`](#dw_at_borland_property_write) | const |  |
| [`DW_AT_BORLAND_property_implements`](#dw_at_borland_property_implements) | const |  |
| [`DW_AT_BORLAND_property_index`](#dw_at_borland_property_index) | const |  |
| [`DW_AT_BORLAND_property_default`](#dw_at_borland_property_default) | const |  |
| [`DW_AT_BORLAND_Delphi_unit`](#dw_at_borland_delphi_unit) | const |  |
| [`DW_AT_BORLAND_Delphi_class`](#dw_at_borland_delphi_class) | const |  |
| [`DW_AT_BORLAND_Delphi_record`](#dw_at_borland_delphi_record) | const |  |
| [`DW_AT_BORLAND_Delphi_metaclass`](#dw_at_borland_delphi_metaclass) | const |  |
| [`DW_AT_BORLAND_Delphi_constructor`](#dw_at_borland_delphi_constructor) | const |  |
| [`DW_AT_BORLAND_Delphi_destructor`](#dw_at_borland_delphi_destructor) | const |  |
| [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw_at_borland_delphi_anonymous_method) | const |  |
| [`DW_AT_BORLAND_Delphi_interface`](#dw_at_borland_delphi_interface) | const |  |
| [`DW_AT_BORLAND_Delphi_ABI`](#dw_at_borland_delphi_abi) | const |  |
| [`DW_AT_BORLAND_Delphi_return`](#dw_at_borland_delphi_return) | const |  |
| [`DW_AT_BORLAND_Delphi_frameptr`](#dw_at_borland_delphi_frameptr) | const |  |
| [`DW_AT_BORLAND_closure`](#dw_at_borland_closure) | const |  |
| [`DW_AT_LLVM_include_path`](#dw_at_llvm_include_path) | const |  |
| [`DW_AT_LLVM_config_macros`](#dw_at_llvm_config_macros) | const |  |
| [`DW_AT_LLVM_isysroot`](#dw_at_llvm_isysroot) | const |  |
| [`DW_AT_APPLE_optimized`](#dw_at_apple_optimized) | const |  |
| [`DW_AT_APPLE_flags`](#dw_at_apple_flags) | const |  |
| [`DW_AT_APPLE_isa`](#dw_at_apple_isa) | const |  |
| [`DW_AT_APPLE_block`](#dw_at_apple_block) | const |  |
| [`DW_AT_APPLE_major_runtime_vers`](#dw_at_apple_major_runtime_vers) | const |  |
| [`DW_AT_APPLE_runtime_class`](#dw_at_apple_runtime_class) | const |  |
| [`DW_AT_APPLE_omit_frame_ptr`](#dw_at_apple_omit_frame_ptr) | const |  |
| [`DW_AT_APPLE_property_name`](#dw_at_apple_property_name) | const |  |
| [`DW_AT_APPLE_property_getter`](#dw_at_apple_property_getter) | const |  |
| [`DW_AT_APPLE_property_setter`](#dw_at_apple_property_setter) | const |  |
| [`DW_AT_APPLE_property_attribute`](#dw_at_apple_property_attribute) | const |  |
| [`DW_AT_APPLE_objc_complete_type`](#dw_at_apple_objc_complete_type) | const |  |
| [`DW_AT_APPLE_property`](#dw_at_apple_property) | const |  |
| [`DW_FORM_null`](#dw_form_null) | const |  |
| [`DW_FORM_ref`](#dw_form_ref) | const |  |
| [`DW_FORM_addr`](#dw_form_addr) | const |  |
| [`DW_FORM_block2`](#dw_form_block2) | const |  |
| [`DW_FORM_block4`](#dw_form_block4) | const |  |
| [`DW_FORM_data2`](#dw_form_data2) | const |  |
| [`DW_FORM_data4`](#dw_form_data4) | const |  |
| [`DW_FORM_data8`](#dw_form_data8) | const |  |
| [`DW_FORM_string`](#dw_form_string) | const |  |
| [`DW_FORM_block`](#dw_form_block) | const |  |
| [`DW_FORM_block1`](#dw_form_block1) | const |  |
| [`DW_FORM_data1`](#dw_form_data1) | const |  |
| [`DW_FORM_flag`](#dw_form_flag) | const |  |
| [`DW_FORM_sdata`](#dw_form_sdata) | const |  |
| [`DW_FORM_strp`](#dw_form_strp) | const |  |
| [`DW_FORM_udata`](#dw_form_udata) | const |  |
| [`DW_FORM_ref_addr`](#dw_form_ref_addr) | const |  |
| [`DW_FORM_ref1`](#dw_form_ref1) | const |  |
| [`DW_FORM_ref2`](#dw_form_ref2) | const |  |
| [`DW_FORM_ref4`](#dw_form_ref4) | const |  |
| [`DW_FORM_ref8`](#dw_form_ref8) | const |  |
| [`DW_FORM_ref_udata`](#dw_form_ref_udata) | const |  |
| [`DW_FORM_indirect`](#dw_form_indirect) | const |  |
| [`DW_FORM_sec_offset`](#dw_form_sec_offset) | const |  |
| [`DW_FORM_exprloc`](#dw_form_exprloc) | const |  |
| [`DW_FORM_flag_present`](#dw_form_flag_present) | const |  |
| [`DW_FORM_ref_sig8`](#dw_form_ref_sig8) | const |  |
| [`DW_FORM_strx`](#dw_form_strx) | const |  |
| [`DW_FORM_addrx`](#dw_form_addrx) | const |  |
| [`DW_FORM_ref_sup4`](#dw_form_ref_sup4) | const |  |
| [`DW_FORM_strp_sup`](#dw_form_strp_sup) | const |  |
| [`DW_FORM_data16`](#dw_form_data16) | const |  |
| [`DW_FORM_line_strp`](#dw_form_line_strp) | const |  |
| [`DW_FORM_implicit_const`](#dw_form_implicit_const) | const |  |
| [`DW_FORM_loclistx`](#dw_form_loclistx) | const |  |
| [`DW_FORM_rnglistx`](#dw_form_rnglistx) | const |  |
| [`DW_FORM_ref_sup8`](#dw_form_ref_sup8) | const |  |
| [`DW_FORM_strx1`](#dw_form_strx1) | const |  |
| [`DW_FORM_strx2`](#dw_form_strx2) | const |  |
| [`DW_FORM_strx3`](#dw_form_strx3) | const |  |
| [`DW_FORM_strx4`](#dw_form_strx4) | const |  |
| [`DW_FORM_addrx1`](#dw_form_addrx1) | const |  |
| [`DW_FORM_addrx2`](#dw_form_addrx2) | const |  |
| [`DW_FORM_addrx3`](#dw_form_addrx3) | const |  |
| [`DW_FORM_addrx4`](#dw_form_addrx4) | const |  |
| [`DW_FORM_GNU_addr_index`](#dw_form_gnu_addr_index) | const |  |
| [`DW_FORM_GNU_str_index`](#dw_form_gnu_str_index) | const |  |
| [`DW_FORM_GNU_ref_alt`](#dw_form_gnu_ref_alt) | const |  |
| [`DW_FORM_GNU_strp_alt`](#dw_form_gnu_strp_alt) | const |  |
| [`DW_ATE_address`](#dw_ate_address) | const |  |
| [`DW_ATE_boolean`](#dw_ate_boolean) | const |  |
| [`DW_ATE_complex_float`](#dw_ate_complex_float) | const |  |
| [`DW_ATE_float`](#dw_ate_float) | const |  |
| [`DW_ATE_signed`](#dw_ate_signed) | const |  |
| [`DW_ATE_signed_char`](#dw_ate_signed_char) | const |  |
| [`DW_ATE_unsigned`](#dw_ate_unsigned) | const |  |
| [`DW_ATE_unsigned_char`](#dw_ate_unsigned_char) | const |  |
| [`DW_ATE_imaginary_float`](#dw_ate_imaginary_float) | const |  |
| [`DW_ATE_packed_decimal`](#dw_ate_packed_decimal) | const |  |
| [`DW_ATE_numeric_string`](#dw_ate_numeric_string) | const |  |
| [`DW_ATE_edited`](#dw_ate_edited) | const |  |
| [`DW_ATE_signed_fixed`](#dw_ate_signed_fixed) | const |  |
| [`DW_ATE_unsigned_fixed`](#dw_ate_unsigned_fixed) | const |  |
| [`DW_ATE_decimal_float`](#dw_ate_decimal_float) | const |  |
| [`DW_ATE_UTF`](#dw_ate_utf) | const |  |
| [`DW_ATE_UCS`](#dw_ate_ucs) | const |  |
| [`DW_ATE_ASCII`](#dw_ate_ascii) | const |  |
| [`DW_ATE_lo_user`](#dw_ate_lo_user) | const |  |
| [`DW_ATE_hi_user`](#dw_ate_hi_user) | const |  |
| [`DW_LLE_end_of_list`](#dw_lle_end_of_list) | const |  |
| [`DW_LLE_base_addressx`](#dw_lle_base_addressx) | const |  |
| [`DW_LLE_startx_endx`](#dw_lle_startx_endx) | const |  |
| [`DW_LLE_startx_length`](#dw_lle_startx_length) | const |  |
| [`DW_LLE_offset_pair`](#dw_lle_offset_pair) | const |  |
| [`DW_LLE_default_location`](#dw_lle_default_location) | const |  |
| [`DW_LLE_base_address`](#dw_lle_base_address) | const |  |
| [`DW_LLE_start_end`](#dw_lle_start_end) | const |  |
| [`DW_LLE_start_length`](#dw_lle_start_length) | const |  |
| [`DW_LLE_GNU_view_pair`](#dw_lle_gnu_view_pair) | const |  |
| [`DW_DS_unsigned`](#dw_ds_unsigned) | const |  |
| [`DW_DS_leading_overpunch`](#dw_ds_leading_overpunch) | const |  |
| [`DW_DS_trailing_overpunch`](#dw_ds_trailing_overpunch) | const |  |
| [`DW_DS_leading_separate`](#dw_ds_leading_separate) | const |  |
| [`DW_DS_trailing_separate`](#dw_ds_trailing_separate) | const |  |
| [`DW_END_default`](#dw_end_default) | const |  |
| [`DW_END_big`](#dw_end_big) | const |  |
| [`DW_END_little`](#dw_end_little) | const |  |
| [`DW_END_lo_user`](#dw_end_lo_user) | const |  |
| [`DW_END_hi_user`](#dw_end_hi_user) | const |  |
| [`DW_ACCESS_public`](#dw_access_public) | const |  |
| [`DW_ACCESS_protected`](#dw_access_protected) | const |  |
| [`DW_ACCESS_private`](#dw_access_private) | const |  |
| [`DW_VIS_local`](#dw_vis_local) | const |  |
| [`DW_VIS_exported`](#dw_vis_exported) | const |  |
| [`DW_VIS_qualified`](#dw_vis_qualified) | const |  |
| [`DW_VIRTUALITY_none`](#dw_virtuality_none) | const |  |
| [`DW_VIRTUALITY_virtual`](#dw_virtuality_virtual) | const |  |
| [`DW_VIRTUALITY_pure_virtual`](#dw_virtuality_pure_virtual) | const |  |
| [`DW_LANG_C89`](#dw_lang_c89) | const |  |
| [`DW_LANG_C`](#dw_lang_c) | const |  |
| [`DW_LANG_Ada83`](#dw_lang_ada83) | const |  |
| [`DW_LANG_C_plus_plus`](#dw_lang_c_plus_plus) | const |  |
| [`DW_LANG_Cobol74`](#dw_lang_cobol74) | const |  |
| [`DW_LANG_Cobol85`](#dw_lang_cobol85) | const |  |
| [`DW_LANG_Fortran77`](#dw_lang_fortran77) | const |  |
| [`DW_LANG_Fortran90`](#dw_lang_fortran90) | const |  |
| [`DW_LANG_Pascal83`](#dw_lang_pascal83) | const |  |
| [`DW_LANG_Modula2`](#dw_lang_modula2) | const |  |
| [`DW_LANG_Java`](#dw_lang_java) | const |  |
| [`DW_LANG_C99`](#dw_lang_c99) | const |  |
| [`DW_LANG_Ada95`](#dw_lang_ada95) | const |  |
| [`DW_LANG_Fortran95`](#dw_lang_fortran95) | const |  |
| [`DW_LANG_PLI`](#dw_lang_pli) | const |  |
| [`DW_LANG_ObjC`](#dw_lang_objc) | const |  |
| [`DW_LANG_ObjC_plus_plus`](#dw_lang_objc_plus_plus) | const |  |
| [`DW_LANG_UPC`](#dw_lang_upc) | const |  |
| [`DW_LANG_D`](#dw_lang_d) | const |  |
| [`DW_LANG_Python`](#dw_lang_python) | const |  |
| [`DW_LANG_OpenCL`](#dw_lang_opencl) | const |  |
| [`DW_LANG_Go`](#dw_lang_go) | const |  |
| [`DW_LANG_Modula3`](#dw_lang_modula3) | const |  |
| [`DW_LANG_Haskell`](#dw_lang_haskell) | const |  |
| [`DW_LANG_C_plus_plus_03`](#dw_lang_c_plus_plus_03) | const |  |
| [`DW_LANG_C_plus_plus_11`](#dw_lang_c_plus_plus_11) | const |  |
| [`DW_LANG_OCaml`](#dw_lang_ocaml) | const |  |
| [`DW_LANG_Rust`](#dw_lang_rust) | const |  |
| [`DW_LANG_C11`](#dw_lang_c11) | const |  |
| [`DW_LANG_Swift`](#dw_lang_swift) | const |  |
| [`DW_LANG_Julia`](#dw_lang_julia) | const |  |
| [`DW_LANG_Dylan`](#dw_lang_dylan) | const |  |
| [`DW_LANG_C_plus_plus_14`](#dw_lang_c_plus_plus_14) | const |  |
| [`DW_LANG_Fortran03`](#dw_lang_fortran03) | const |  |
| [`DW_LANG_Fortran08`](#dw_lang_fortran08) | const |  |
| [`DW_LANG_RenderScript`](#dw_lang_renderscript) | const |  |
| [`DW_LANG_BLISS`](#dw_lang_bliss) | const |  |
| [`DW_LANG_Kotlin`](#dw_lang_kotlin) | const |  |
| [`DW_LANG_Zig`](#dw_lang_zig) | const |  |
| [`DW_LANG_Crystal`](#dw_lang_crystal) | const |  |
| [`DW_LANG_C_plus_plus_17`](#dw_lang_c_plus_plus_17) | const |  |
| [`DW_LANG_C_plus_plus_20`](#dw_lang_c_plus_plus_20) | const |  |
| [`DW_LANG_C17`](#dw_lang_c17) | const |  |
| [`DW_LANG_Fortran18`](#dw_lang_fortran18) | const |  |
| [`DW_LANG_Ada2005`](#dw_lang_ada2005) | const |  |
| [`DW_LANG_Ada2012`](#dw_lang_ada2012) | const |  |
| [`DW_LANG_lo_user`](#dw_lang_lo_user) | const |  |
| [`DW_LANG_hi_user`](#dw_lang_hi_user) | const |  |
| [`DW_LANG_Mips_Assembler`](#dw_lang_mips_assembler) | const |  |
| [`DW_LANG_GOOGLE_RenderScript`](#dw_lang_google_renderscript) | const |  |
| [`DW_LANG_SUN_Assembler`](#dw_lang_sun_assembler) | const |  |
| [`DW_LANG_ALTIUM_Assembler`](#dw_lang_altium_assembler) | const |  |
| [`DW_LANG_BORLAND_Delphi`](#dw_lang_borland_delphi) | const |  |
| [`DW_ADDR_none`](#dw_addr_none) | const |  |
| [`DW_ID_case_sensitive`](#dw_id_case_sensitive) | const |  |
| [`DW_ID_up_case`](#dw_id_up_case) | const |  |
| [`DW_ID_down_case`](#dw_id_down_case) | const |  |
| [`DW_ID_case_insensitive`](#dw_id_case_insensitive) | const |  |
| [`DW_CC_normal`](#dw_cc_normal) | const |  |
| [`DW_CC_program`](#dw_cc_program) | const |  |
| [`DW_CC_nocall`](#dw_cc_nocall) | const |  |
| [`DW_CC_pass_by_reference`](#dw_cc_pass_by_reference) | const |  |
| [`DW_CC_pass_by_value`](#dw_cc_pass_by_value) | const |  |
| [`DW_CC_lo_user`](#dw_cc_lo_user) | const |  |
| [`DW_CC_hi_user`](#dw_cc_hi_user) | const |  |
| [`DW_INL_not_inlined`](#dw_inl_not_inlined) | const |  |
| [`DW_INL_inlined`](#dw_inl_inlined) | const |  |
| [`DW_INL_declared_not_inlined`](#dw_inl_declared_not_inlined) | const |  |
| [`DW_INL_declared_inlined`](#dw_inl_declared_inlined) | const |  |
| [`DW_ORD_row_major`](#dw_ord_row_major) | const |  |
| [`DW_ORD_col_major`](#dw_ord_col_major) | const |  |
| [`DW_DSC_label`](#dw_dsc_label) | const |  |
| [`DW_DSC_range`](#dw_dsc_range) | const |  |
| [`DW_IDX_compile_unit`](#dw_idx_compile_unit) | const |  |
| [`DW_IDX_type_unit`](#dw_idx_type_unit) | const |  |
| [`DW_IDX_die_offset`](#dw_idx_die_offset) | const |  |
| [`DW_IDX_parent`](#dw_idx_parent) | const |  |
| [`DW_IDX_type_hash`](#dw_idx_type_hash) | const |  |
| [`DW_IDX_lo_user`](#dw_idx_lo_user) | const |  |
| [`DW_IDX_hi_user`](#dw_idx_hi_user) | const |  |
| [`DW_DEFAULTED_no`](#dw_defaulted_no) | const |  |
| [`DW_DEFAULTED_in_class`](#dw_defaulted_in_class) | const |  |
| [`DW_DEFAULTED_out_of_class`](#dw_defaulted_out_of_class) | const |  |
| [`DW_LNS_copy`](#dw_lns_copy) | const |  |
| [`DW_LNS_advance_pc`](#dw_lns_advance_pc) | const |  |
| [`DW_LNS_advance_line`](#dw_lns_advance_line) | const |  |
| [`DW_LNS_set_file`](#dw_lns_set_file) | const |  |
| [`DW_LNS_set_column`](#dw_lns_set_column) | const |  |
| [`DW_LNS_negate_stmt`](#dw_lns_negate_stmt) | const |  |
| [`DW_LNS_set_basic_block`](#dw_lns_set_basic_block) | const |  |
| [`DW_LNS_const_add_pc`](#dw_lns_const_add_pc) | const |  |
| [`DW_LNS_fixed_advance_pc`](#dw_lns_fixed_advance_pc) | const |  |
| [`DW_LNS_set_prologue_end`](#dw_lns_set_prologue_end) | const |  |
| [`DW_LNS_set_epilogue_begin`](#dw_lns_set_epilogue_begin) | const |  |
| [`DW_LNS_set_isa`](#dw_lns_set_isa) | const |  |
| [`DW_LNE_end_sequence`](#dw_lne_end_sequence) | const |  |
| [`DW_LNE_set_address`](#dw_lne_set_address) | const |  |
| [`DW_LNE_define_file`](#dw_lne_define_file) | const |  |
| [`DW_LNE_set_discriminator`](#dw_lne_set_discriminator) | const |  |
| [`DW_LNE_lo_user`](#dw_lne_lo_user) | const |  |
| [`DW_LNE_hi_user`](#dw_lne_hi_user) | const |  |
| [`DW_LNCT_path`](#dw_lnct_path) | const |  |
| [`DW_LNCT_directory_index`](#dw_lnct_directory_index) | const |  |
| [`DW_LNCT_timestamp`](#dw_lnct_timestamp) | const |  |
| [`DW_LNCT_size`](#dw_lnct_size) | const |  |
| [`DW_LNCT_MD5`](#dw_lnct_md5) | const |  |
| [`DW_LNCT_lo_user`](#dw_lnct_lo_user) | const |  |
| [`DW_LNCT_LLVM_source`](#dw_lnct_llvm_source) | const |  |
| [`DW_LNCT_hi_user`](#dw_lnct_hi_user) | const |  |
| [`DW_MACINFO_define`](#dw_macinfo_define) | const |  |
| [`DW_MACINFO_undef`](#dw_macinfo_undef) | const |  |
| [`DW_MACINFO_start_file`](#dw_macinfo_start_file) | const |  |
| [`DW_MACINFO_end_file`](#dw_macinfo_end_file) | const |  |
| [`DW_MACINFO_vendor_ext`](#dw_macinfo_vendor_ext) | const |  |
| [`DW_MACRO_define`](#dw_macro_define) | const |  |
| [`DW_MACRO_undef`](#dw_macro_undef) | const |  |
| [`DW_MACRO_start_file`](#dw_macro_start_file) | const |  |
| [`DW_MACRO_end_file`](#dw_macro_end_file) | const |  |
| [`DW_MACRO_define_strp`](#dw_macro_define_strp) | const |  |
| [`DW_MACRO_undef_strp`](#dw_macro_undef_strp) | const |  |
| [`DW_MACRO_import`](#dw_macro_import) | const |  |
| [`DW_MACRO_define_sup`](#dw_macro_define_sup) | const |  |
| [`DW_MACRO_undef_sup`](#dw_macro_undef_sup) | const |  |
| [`DW_MACRO_import_sup`](#dw_macro_import_sup) | const |  |
| [`DW_MACRO_define_strx`](#dw_macro_define_strx) | const |  |
| [`DW_MACRO_undef_strx`](#dw_macro_undef_strx) | const |  |
| [`DW_MACRO_lo_user`](#dw_macro_lo_user) | const |  |
| [`DW_MACRO_hi_user`](#dw_macro_hi_user) | const |  |
| [`DW_RLE_end_of_list`](#dw_rle_end_of_list) | const |  |
| [`DW_RLE_base_addressx`](#dw_rle_base_addressx) | const |  |
| [`DW_RLE_startx_endx`](#dw_rle_startx_endx) | const |  |
| [`DW_RLE_startx_length`](#dw_rle_startx_length) | const |  |
| [`DW_RLE_offset_pair`](#dw_rle_offset_pair) | const |  |
| [`DW_RLE_base_address`](#dw_rle_base_address) | const |  |
| [`DW_RLE_start_end`](#dw_rle_start_end) | const |  |
| [`DW_RLE_start_length`](#dw_rle_start_length) | const |  |
| [`DW_OP_addr`](#dw_op_addr) | const |  |
| [`DW_OP_deref`](#dw_op_deref) | const |  |
| [`DW_OP_const1u`](#dw_op_const1u) | const |  |
| [`DW_OP_const1s`](#dw_op_const1s) | const |  |
| [`DW_OP_const2u`](#dw_op_const2u) | const |  |
| [`DW_OP_const2s`](#dw_op_const2s) | const |  |
| [`DW_OP_const4u`](#dw_op_const4u) | const |  |
| [`DW_OP_const4s`](#dw_op_const4s) | const |  |
| [`DW_OP_const8u`](#dw_op_const8u) | const |  |
| [`DW_OP_const8s`](#dw_op_const8s) | const |  |
| [`DW_OP_constu`](#dw_op_constu) | const |  |
| [`DW_OP_consts`](#dw_op_consts) | const |  |
| [`DW_OP_dup`](#dw_op_dup) | const |  |
| [`DW_OP_drop`](#dw_op_drop) | const |  |
| [`DW_OP_over`](#dw_op_over) | const |  |
| [`DW_OP_pick`](#dw_op_pick) | const |  |
| [`DW_OP_swap`](#dw_op_swap) | const |  |
| [`DW_OP_rot`](#dw_op_rot) | const |  |
| [`DW_OP_xderef`](#dw_op_xderef) | const |  |
| [`DW_OP_abs`](#dw_op_abs) | const |  |
| [`DW_OP_and`](#dw_op_and) | const |  |
| [`DW_OP_div`](#dw_op_div) | const |  |
| [`DW_OP_minus`](#dw_op_minus) | const |  |
| [`DW_OP_mod`](#dw_op_mod) | const |  |
| [`DW_OP_mul`](#dw_op_mul) | const |  |
| [`DW_OP_neg`](#dw_op_neg) | const |  |
| [`DW_OP_not`](#dw_op_not) | const |  |
| [`DW_OP_or`](#dw_op_or) | const |  |
| [`DW_OP_plus`](#dw_op_plus) | const |  |
| [`DW_OP_plus_uconst`](#dw_op_plus_uconst) | const |  |
| [`DW_OP_shl`](#dw_op_shl) | const |  |
| [`DW_OP_shr`](#dw_op_shr) | const |  |
| [`DW_OP_shra`](#dw_op_shra) | const |  |
| [`DW_OP_xor`](#dw_op_xor) | const |  |
| [`DW_OP_bra`](#dw_op_bra) | const |  |
| [`DW_OP_eq`](#dw_op_eq) | const |  |
| [`DW_OP_ge`](#dw_op_ge) | const |  |
| [`DW_OP_gt`](#dw_op_gt) | const |  |
| [`DW_OP_le`](#dw_op_le) | const |  |
| [`DW_OP_lt`](#dw_op_lt) | const |  |
| [`DW_OP_ne`](#dw_op_ne) | const |  |
| [`DW_OP_skip`](#dw_op_skip) | const |  |
| [`DW_OP_lit0`](#dw_op_lit0) | const |  |
| [`DW_OP_lit1`](#dw_op_lit1) | const |  |
| [`DW_OP_lit2`](#dw_op_lit2) | const |  |
| [`DW_OP_lit3`](#dw_op_lit3) | const |  |
| [`DW_OP_lit4`](#dw_op_lit4) | const |  |
| [`DW_OP_lit5`](#dw_op_lit5) | const |  |
| [`DW_OP_lit6`](#dw_op_lit6) | const |  |
| [`DW_OP_lit7`](#dw_op_lit7) | const |  |
| [`DW_OP_lit8`](#dw_op_lit8) | const |  |
| [`DW_OP_lit9`](#dw_op_lit9) | const |  |
| [`DW_OP_lit10`](#dw_op_lit10) | const |  |
| [`DW_OP_lit11`](#dw_op_lit11) | const |  |
| [`DW_OP_lit12`](#dw_op_lit12) | const |  |
| [`DW_OP_lit13`](#dw_op_lit13) | const |  |
| [`DW_OP_lit14`](#dw_op_lit14) | const |  |
| [`DW_OP_lit15`](#dw_op_lit15) | const |  |
| [`DW_OP_lit16`](#dw_op_lit16) | const |  |
| [`DW_OP_lit17`](#dw_op_lit17) | const |  |
| [`DW_OP_lit18`](#dw_op_lit18) | const |  |
| [`DW_OP_lit19`](#dw_op_lit19) | const |  |
| [`DW_OP_lit20`](#dw_op_lit20) | const |  |
| [`DW_OP_lit21`](#dw_op_lit21) | const |  |
| [`DW_OP_lit22`](#dw_op_lit22) | const |  |
| [`DW_OP_lit23`](#dw_op_lit23) | const |  |
| [`DW_OP_lit24`](#dw_op_lit24) | const |  |
| [`DW_OP_lit25`](#dw_op_lit25) | const |  |
| [`DW_OP_lit26`](#dw_op_lit26) | const |  |
| [`DW_OP_lit27`](#dw_op_lit27) | const |  |
| [`DW_OP_lit28`](#dw_op_lit28) | const |  |
| [`DW_OP_lit29`](#dw_op_lit29) | const |  |
| [`DW_OP_lit30`](#dw_op_lit30) | const |  |
| [`DW_OP_lit31`](#dw_op_lit31) | const |  |
| [`DW_OP_reg0`](#dw_op_reg0) | const |  |
| [`DW_OP_reg1`](#dw_op_reg1) | const |  |
| [`DW_OP_reg2`](#dw_op_reg2) | const |  |
| [`DW_OP_reg3`](#dw_op_reg3) | const |  |
| [`DW_OP_reg4`](#dw_op_reg4) | const |  |
| [`DW_OP_reg5`](#dw_op_reg5) | const |  |
| [`DW_OP_reg6`](#dw_op_reg6) | const |  |
| [`DW_OP_reg7`](#dw_op_reg7) | const |  |
| [`DW_OP_reg8`](#dw_op_reg8) | const |  |
| [`DW_OP_reg9`](#dw_op_reg9) | const |  |
| [`DW_OP_reg10`](#dw_op_reg10) | const |  |
| [`DW_OP_reg11`](#dw_op_reg11) | const |  |
| [`DW_OP_reg12`](#dw_op_reg12) | const |  |
| [`DW_OP_reg13`](#dw_op_reg13) | const |  |
| [`DW_OP_reg14`](#dw_op_reg14) | const |  |
| [`DW_OP_reg15`](#dw_op_reg15) | const |  |
| [`DW_OP_reg16`](#dw_op_reg16) | const |  |
| [`DW_OP_reg17`](#dw_op_reg17) | const |  |
| [`DW_OP_reg18`](#dw_op_reg18) | const |  |
| [`DW_OP_reg19`](#dw_op_reg19) | const |  |
| [`DW_OP_reg20`](#dw_op_reg20) | const |  |
| [`DW_OP_reg21`](#dw_op_reg21) | const |  |
| [`DW_OP_reg22`](#dw_op_reg22) | const |  |
| [`DW_OP_reg23`](#dw_op_reg23) | const |  |
| [`DW_OP_reg24`](#dw_op_reg24) | const |  |
| [`DW_OP_reg25`](#dw_op_reg25) | const |  |
| [`DW_OP_reg26`](#dw_op_reg26) | const |  |
| [`DW_OP_reg27`](#dw_op_reg27) | const |  |
| [`DW_OP_reg28`](#dw_op_reg28) | const |  |
| [`DW_OP_reg29`](#dw_op_reg29) | const |  |
| [`DW_OP_reg30`](#dw_op_reg30) | const |  |
| [`DW_OP_reg31`](#dw_op_reg31) | const |  |
| [`DW_OP_breg0`](#dw_op_breg0) | const |  |
| [`DW_OP_breg1`](#dw_op_breg1) | const |  |
| [`DW_OP_breg2`](#dw_op_breg2) | const |  |
| [`DW_OP_breg3`](#dw_op_breg3) | const |  |
| [`DW_OP_breg4`](#dw_op_breg4) | const |  |
| [`DW_OP_breg5`](#dw_op_breg5) | const |  |
| [`DW_OP_breg6`](#dw_op_breg6) | const |  |
| [`DW_OP_breg7`](#dw_op_breg7) | const |  |
| [`DW_OP_breg8`](#dw_op_breg8) | const |  |
| [`DW_OP_breg9`](#dw_op_breg9) | const |  |
| [`DW_OP_breg10`](#dw_op_breg10) | const |  |
| [`DW_OP_breg11`](#dw_op_breg11) | const |  |
| [`DW_OP_breg12`](#dw_op_breg12) | const |  |
| [`DW_OP_breg13`](#dw_op_breg13) | const |  |
| [`DW_OP_breg14`](#dw_op_breg14) | const |  |
| [`DW_OP_breg15`](#dw_op_breg15) | const |  |
| [`DW_OP_breg16`](#dw_op_breg16) | const |  |
| [`DW_OP_breg17`](#dw_op_breg17) | const |  |
| [`DW_OP_breg18`](#dw_op_breg18) | const |  |
| [`DW_OP_breg19`](#dw_op_breg19) | const |  |
| [`DW_OP_breg20`](#dw_op_breg20) | const |  |
| [`DW_OP_breg21`](#dw_op_breg21) | const |  |
| [`DW_OP_breg22`](#dw_op_breg22) | const |  |
| [`DW_OP_breg23`](#dw_op_breg23) | const |  |
| [`DW_OP_breg24`](#dw_op_breg24) | const |  |
| [`DW_OP_breg25`](#dw_op_breg25) | const |  |
| [`DW_OP_breg26`](#dw_op_breg26) | const |  |
| [`DW_OP_breg27`](#dw_op_breg27) | const |  |
| [`DW_OP_breg28`](#dw_op_breg28) | const |  |
| [`DW_OP_breg29`](#dw_op_breg29) | const |  |
| [`DW_OP_breg30`](#dw_op_breg30) | const |  |
| [`DW_OP_breg31`](#dw_op_breg31) | const |  |
| [`DW_OP_regx`](#dw_op_regx) | const |  |
| [`DW_OP_fbreg`](#dw_op_fbreg) | const |  |
| [`DW_OP_bregx`](#dw_op_bregx) | const |  |
| [`DW_OP_piece`](#dw_op_piece) | const |  |
| [`DW_OP_deref_size`](#dw_op_deref_size) | const |  |
| [`DW_OP_xderef_size`](#dw_op_xderef_size) | const |  |
| [`DW_OP_nop`](#dw_op_nop) | const |  |
| [`DW_OP_push_object_address`](#dw_op_push_object_address) | const |  |
| [`DW_OP_call2`](#dw_op_call2) | const |  |
| [`DW_OP_call4`](#dw_op_call4) | const |  |
| [`DW_OP_call_ref`](#dw_op_call_ref) | const |  |
| [`DW_OP_form_tls_address`](#dw_op_form_tls_address) | const |  |
| [`DW_OP_call_frame_cfa`](#dw_op_call_frame_cfa) | const |  |
| [`DW_OP_bit_piece`](#dw_op_bit_piece) | const |  |
| [`DW_OP_implicit_value`](#dw_op_implicit_value) | const |  |
| [`DW_OP_stack_value`](#dw_op_stack_value) | const |  |
| [`DW_OP_implicit_pointer`](#dw_op_implicit_pointer) | const |  |
| [`DW_OP_addrx`](#dw_op_addrx) | const |  |
| [`DW_OP_constx`](#dw_op_constx) | const |  |
| [`DW_OP_entry_value`](#dw_op_entry_value) | const |  |
| [`DW_OP_const_type`](#dw_op_const_type) | const |  |
| [`DW_OP_regval_type`](#dw_op_regval_type) | const |  |
| [`DW_OP_deref_type`](#dw_op_deref_type) | const |  |
| [`DW_OP_xderef_type`](#dw_op_xderef_type) | const |  |
| [`DW_OP_convert`](#dw_op_convert) | const |  |
| [`DW_OP_reinterpret`](#dw_op_reinterpret) | const |  |
| [`DW_OP_GNU_push_tls_address`](#dw_op_gnu_push_tls_address) | const |  |
| [`DW_OP_GNU_implicit_pointer`](#dw_op_gnu_implicit_pointer) | const |  |
| [`DW_OP_GNU_entry_value`](#dw_op_gnu_entry_value) | const |  |
| [`DW_OP_GNU_const_type`](#dw_op_gnu_const_type) | const |  |
| [`DW_OP_GNU_regval_type`](#dw_op_gnu_regval_type) | const |  |
| [`DW_OP_GNU_deref_type`](#dw_op_gnu_deref_type) | const |  |
| [`DW_OP_GNU_convert`](#dw_op_gnu_convert) | const |  |
| [`DW_OP_GNU_reinterpret`](#dw_op_gnu_reinterpret) | const |  |
| [`DW_OP_GNU_parameter_ref`](#dw_op_gnu_parameter_ref) | const |  |
| [`DW_OP_GNU_addr_index`](#dw_op_gnu_addr_index) | const |  |
| [`DW_OP_GNU_const_index`](#dw_op_gnu_const_index) | const |  |
| [`DW_OP_WASM_location`](#dw_op_wasm_location) | const |  |
| [`DW_EH_PE_uleb128`](#dw_eh_pe_uleb128) | const |  |
| [`DW_EH_PE_udata2`](#dw_eh_pe_udata2) | const |  |
| [`DW_EH_PE_udata4`](#dw_eh_pe_udata4) | const |  |
| [`DW_EH_PE_udata8`](#dw_eh_pe_udata8) | const |  |
| [`DW_EH_PE_sleb128`](#dw_eh_pe_sleb128) | const |  |
| [`DW_EH_PE_sdata2`](#dw_eh_pe_sdata2) | const |  |
| [`DW_EH_PE_sdata4`](#dw_eh_pe_sdata4) | const |  |
| [`DW_EH_PE_sdata8`](#dw_eh_pe_sdata8) | const |  |
| [`DW_EH_PE_pcrel`](#dw_eh_pe_pcrel) | const |  |
| [`DW_EH_PE_textrel`](#dw_eh_pe_textrel) | const |  |
| [`DW_EH_PE_datarel`](#dw_eh_pe_datarel) | const |  |
| [`DW_EH_PE_funcrel`](#dw_eh_pe_funcrel) | const |  |
| [`DW_EH_PE_aligned`](#dw_eh_pe_aligned) | const |  |
| [`DW_EH_PE_indirect`](#dw_eh_pe_indirect) | const |  |
| [`DW_EH_PE_absptr`](#dw_eh_pe_absptr) | const |  |
| [`DW_EH_PE_omit`](#dw_eh_pe_omit) | const |  |
| [`DW_EH_PE_FORMAT_MASK`](#dw_eh_pe_format_mask) | const |  |
| [`DW_EH_PE_APPLICATION_MASK`](#dw_eh_pe_application_mask) | const |  |
| [`registers!`](#registers) | macro |  |
| [`dw!`](#dw) | macro |  |

## Modules

- [`common`](common/index.md)
- [`arch`](arch/index.md)
- [`constants`](constants/index.md) — Constant definitions.
- [`endianity`](endianity/index.md) — Types for compile-time and run-time endianity.
- [`leb128`](leb128/index.md) — Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
- [`read`](read/index.md) — Read DWARF debugging information.
- [`util`](util/index.md)
- [`addr`](addr/index.md)
- [`cfi`](cfi/index.md)
- [`dwarf`](dwarf/index.md)
- [`endian_slice`](endian_slice/index.md) — Working with byte slices that have an associated endianity.
- [`reader`](reader/index.md)
- [`relocate`](relocate/index.md)
- [`abbrev`](abbrev/index.md) — Functions for parsing DWARF debugging abbreviations.
- [`aranges`](aranges/index.md)
- [`index`](index/index.md)
- [`line`](line/index.md)
- [`lists`](lists/index.md)
- [`loclists`](loclists/index.md)
- [`lookup`](lookup/index.md)
- [`macros`](macros/index.md)
- [`op`](op/index.md) — Functions for parsing and evaluating DWARF expressions.
- [`pubnames`](pubnames/index.md)
- [`pubtypes`](pubtypes/index.md)
- [`rnglists`](rnglists/index.md)
- [`str`](str/index.md)
- [`unit`](unit/index.md) — Functions for parsing DWARF `.debug_info` and `.debug_types` sections.
- [`value`](value/index.md) — Definitions for values used in DWARF expressions.

## Structs

### `Encoding`

```rust
struct Encoding {
    pub address_size: u8,
    pub format: Format,
    pub version: u16,
}
```

Encoding parameters that are commonly used for multiple DWARF sections.

This is intended to be small enough to pass by value.

#### Fields

- **`address_size`**: `u8`

  The size of an address.

- **`format`**: `Format`

  Whether the DWARF format is 32- or 64-bit.

- **`version`**: `u16`

  The DWARF version of the header.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](#encoding)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl Hash for Encoding`

- <span id="encoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Encoding`

- <span id="encoding-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](#encoding)

##### `impl StructuralPartialEq for Encoding`

### `LineEncoding`

```rust
struct LineEncoding {
    pub minimum_instruction_length: u8,
    pub maximum_operations_per_instruction: u8,
    pub default_is_stmt: bool,
    pub line_base: i8,
    pub line_range: u8,
}
```

Encoding parameters for a line number program.

#### Fields

- **`minimum_instruction_length`**: `u8`

  The size in bytes of the smallest target machine instruction.

- **`maximum_operations_per_instruction`**: `u8`

  The maximum number of individual operations that may be encoded in an
  instruction.

- **`default_is_stmt`**: `bool`

  The initial value of the `is_stmt` register.

- **`line_base`**: `i8`

  The minimum value which a special opcode can add to the line register.

- **`line_range`**: `u8`

  The range of values which a special opcode can add to the line register.

#### Trait Implementations

##### `impl Clone for LineEncoding`

- <span id="lineencoding-clone"></span>`fn clone(&self) -> LineEncoding` — [`LineEncoding`](#lineencoding)

##### `impl Copy for LineEncoding`

##### `impl Debug for LineEncoding`

- <span id="lineencoding-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEncoding`

- <span id="lineencoding-default"></span>`fn default() -> Self`

##### `impl Eq for LineEncoding`

##### `impl Hash for LineEncoding`

- <span id="lineencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LineEncoding`

- <span id="lineencoding-eq"></span>`fn eq(&self, other: &LineEncoding) -> bool` — [`LineEncoding`](#lineencoding)

##### `impl StructuralPartialEq for LineEncoding`

### `Register`

```rust
struct Register(u16);
```

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

#### Implementations

- <span id="cratecommonregister-from-u64"></span>`fn from_u64(x: u64) -> Result<Register>` — [`Result`](#result), [`Register`](#register)

#### Trait Implementations

##### `impl Clone for Register`

- <span id="register-clone"></span>`fn clone(&self) -> Register` — [`Register`](#register)

##### `impl Copy for Register`

##### `impl Debug for Register`

- <span id="register-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Register`

##### `impl Hash for Register`

- <span id="register-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Register`

- <span id="register-cmp"></span>`fn cmp(&self, other: &Register) -> cmp::Ordering` — [`Register`](#register)

##### `impl PartialEq for Register`

- <span id="register-eq"></span>`fn eq(&self, other: &Register) -> bool` — [`Register`](#register)

##### `impl PartialOrd for Register`

- <span id="register-partial-cmp"></span>`fn partial_cmp(&self, other: &Register) -> option::Option<cmp::Ordering>` — [`Register`](#register)

##### `impl StructuralPartialEq for Register`

### `DebugAbbrevOffset<T>`

```rust
struct DebugAbbrevOffset<T>(T);
```

An offset into the `.debug_abbrev` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-clone"></span>`fn clone(&self) -> DebugAbbrevOffset<T>` — [`DebugAbbrevOffset`](#debugabbrevoffset)

##### `impl<T: marker::Copy> Copy for DebugAbbrevOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAbbrevOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-eq"></span>`fn eq(&self, other: &DebugAbbrevOffset<T>) -> bool` — [`DebugAbbrevOffset`](#debugabbrevoffset)

##### `impl<T> StructuralPartialEq for DebugAbbrevOffset<T>`

### `DebugAddrOffset<T>`

```rust
struct DebugAddrOffset<T>(T);
```

An offset into the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrOffset<T>`

- <span id="debugaddroffset-clone"></span>`fn clone(&self) -> DebugAddrOffset<T>` — [`DebugAddrOffset`](#debugaddroffset)

##### `impl<T: marker::Copy> Copy for DebugAddrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrOffset<T>`

- <span id="debugaddroffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrOffset<T>`

- <span id="debugaddroffset-eq"></span>`fn eq(&self, other: &DebugAddrOffset<T>) -> bool` — [`DebugAddrOffset`](#debugaddroffset)

##### `impl<T> StructuralPartialEq for DebugAddrOffset<T>`

### `DebugAddrBase<T>`

```rust
struct DebugAddrBase<T>(T);
```

An offset to a set of entries in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrBase<T>`

- <span id="debugaddrbase-clone"></span>`fn clone(&self) -> DebugAddrBase<T>` — [`DebugAddrBase`](#debugaddrbase)

##### `impl<T: marker::Copy> Copy for DebugAddrBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrBase<T>`

- <span id="debugaddrbase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrBase<T>`

- <span id="debugaddrbase-eq"></span>`fn eq(&self, other: &DebugAddrBase<T>) -> bool` — [`DebugAddrBase`](#debugaddrbase)

##### `impl<T> StructuralPartialEq for DebugAddrBase<T>`

### `DebugAddrIndex<T>`

```rust
struct DebugAddrIndex<T>(T);
```

An index into a set of addresses in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrIndex<T>`

- <span id="debugaddrindex-clone"></span>`fn clone(&self) -> DebugAddrIndex<T>` — [`DebugAddrIndex`](#debugaddrindex)

##### `impl<T: marker::Copy> Copy for DebugAddrIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrIndex<T>`

- <span id="debugaddrindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrIndex<T>`

- <span id="debugaddrindex-eq"></span>`fn eq(&self, other: &DebugAddrIndex<T>) -> bool` — [`DebugAddrIndex`](#debugaddrindex)

##### `impl<T> StructuralPartialEq for DebugAddrIndex<T>`

### `DebugArangesOffset<T>`

```rust
struct DebugArangesOffset<T>(T);
```

An offset into the `.debug_aranges` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugArangesOffset<T>`

- <span id="debugarangesoffset-clone"></span>`fn clone(&self) -> DebugArangesOffset<T>` — [`DebugArangesOffset`](#debugarangesoffset)

##### `impl<T: marker::Copy> Copy for DebugArangesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugArangesOffset<T>`

- <span id="debugarangesoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugArangesOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugArangesOffset<T>`

- <span id="debugarangesoffset-eq"></span>`fn eq(&self, other: &DebugArangesOffset<T>) -> bool` — [`DebugArangesOffset`](#debugarangesoffset)

##### `impl<T> StructuralPartialEq for DebugArangesOffset<T>`

### `DebugInfoOffset<T>`

```rust
struct DebugInfoOffset<T>(T);
```

An offset into the `.debug_info` section.

#### Implementations

- <span id="cratecommondebuginfooffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](read/index.md), [`UnitOffset`](#unitoffset)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugInfoOffset<T>`

- <span id="debuginfooffset-clone"></span>`fn clone(&self) -> DebugInfoOffset<T>` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: marker::Copy> Copy for DebugInfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugInfoOffset<T>`

- <span id="debuginfooffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugInfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugInfoOffset<T>`

- <span id="debuginfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugInfoOffset<T>`

- <span id="debuginfooffset-cmp"></span>`fn cmp(&self, other: &DebugInfoOffset<T>) -> cmp::Ordering` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugInfoOffset<T>`

- <span id="debuginfooffset-eq"></span>`fn eq(&self, other: &DebugInfoOffset<T>) -> bool` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugInfoOffset<T>`

- <span id="debuginfooffset-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugInfoOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T> StructuralPartialEq for DebugInfoOffset<T>`

### `DebugLineOffset<T>`

```rust
struct DebugLineOffset<T>(T);
```

An offset into the `.debug_line` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineOffset<T>`

- <span id="debuglineoffset-clone"></span>`fn clone(&self) -> DebugLineOffset<T>` — [`DebugLineOffset`](#debuglineoffset)

##### `impl<T: marker::Copy> Copy for DebugLineOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineOffset<T>`

- <span id="debuglineoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineOffset<T>`

- <span id="debuglineoffset-eq"></span>`fn eq(&self, other: &DebugLineOffset<T>) -> bool` — [`DebugLineOffset`](#debuglineoffset)

##### `impl<T> StructuralPartialEq for DebugLineOffset<T>`

### `DebugLineStrOffset<T>`

```rust
struct DebugLineStrOffset<T>(T);
```

An offset into the `.debug_line_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-clone"></span>`fn clone(&self) -> DebugLineStrOffset<T>` — [`DebugLineStrOffset`](#debuglinestroffset)

##### `impl<T: marker::Copy> Copy for DebugLineStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-eq"></span>`fn eq(&self, other: &DebugLineStrOffset<T>) -> bool` — [`DebugLineStrOffset`](#debuglinestroffset)

##### `impl<T> StructuralPartialEq for DebugLineStrOffset<T>`

### `LocationListsOffset<T>`

```rust
struct LocationListsOffset<T>(T);
```

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for LocationListsOffset<T>`

- <span id="locationlistsoffset-clone"></span>`fn clone(&self) -> LocationListsOffset<T>` — [`LocationListsOffset`](#locationlistsoffset)

##### `impl<T: marker::Copy> Copy for LocationListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for LocationListsOffset<T>`

- <span id="locationlistsoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for LocationListsOffset<T>`

##### `impl<T: hash::Hash> Hash for LocationListsOffset<T>`

- <span id="locationlistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for LocationListsOffset<T>`

- <span id="locationlistsoffset-eq"></span>`fn eq(&self, other: &LocationListsOffset<T>) -> bool` — [`LocationListsOffset`](#locationlistsoffset)

##### `impl<T> StructuralPartialEq for LocationListsOffset<T>`

### `DebugLocListsBase<T>`

```rust
struct DebugLocListsBase<T>(T);
```

An offset to a set of location list offsets in the `.debug_loclists` section.

#### Implementations

- <span id="cratecommondebugloclistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugLocListsBase`](#debugloclistsbase)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsBase<T>`

- <span id="debugloclistsbase-clone"></span>`fn clone(&self) -> DebugLocListsBase<T>` — [`DebugLocListsBase`](#debugloclistsbase)

##### `impl<T: marker::Copy> Copy for DebugLocListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsBase<T>`

- <span id="debugloclistsbase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsBase<T>`

- <span id="debugloclistsbase-eq"></span>`fn eq(&self, other: &DebugLocListsBase<T>) -> bool` — [`DebugLocListsBase`](#debugloclistsbase)

##### `impl<T> StructuralPartialEq for DebugLocListsBase<T>`

### `DebugLocListsIndex<T>`

```rust
struct DebugLocListsIndex<T>(T);
```

An index into a set of location list offsets in the `.debug_loclists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-clone"></span>`fn clone(&self) -> DebugLocListsIndex<T>` — [`DebugLocListsIndex`](#debugloclistsindex)

##### `impl<T: marker::Copy> Copy for DebugLocListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-eq"></span>`fn eq(&self, other: &DebugLocListsIndex<T>) -> bool` — [`DebugLocListsIndex`](#debugloclistsindex)

##### `impl<T> StructuralPartialEq for DebugLocListsIndex<T>`

### `DebugMacinfoOffset<T>`

```rust
struct DebugMacinfoOffset<T>(T);
```

An offset into the `.debug_macinfo` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-clone"></span>`fn clone(&self) -> DebugMacinfoOffset<T>` — [`DebugMacinfoOffset`](#debugmacinfooffset)

##### `impl<T: marker::Copy> Copy for DebugMacinfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacinfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-eq"></span>`fn eq(&self, other: &DebugMacinfoOffset<T>) -> bool` — [`DebugMacinfoOffset`](#debugmacinfooffset)

##### `impl<T> StructuralPartialEq for DebugMacinfoOffset<T>`

### `DebugMacroOffset<T>`

```rust
struct DebugMacroOffset<T>(T);
```

An offset into the `.debug_macro` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacroOffset<T>`

- <span id="debugmacrooffset-clone"></span>`fn clone(&self) -> DebugMacroOffset<T>` — [`DebugMacroOffset`](#debugmacrooffset)

##### `impl<T: marker::Copy> Copy for DebugMacroOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacroOffset<T>`

- <span id="debugmacrooffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacroOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacroOffset<T>`

- <span id="debugmacrooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacroOffset<T>`

- <span id="debugmacrooffset-eq"></span>`fn eq(&self, other: &DebugMacroOffset<T>) -> bool` — [`DebugMacroOffset`](#debugmacrooffset)

##### `impl<T> StructuralPartialEq for DebugMacroOffset<T>`

### `RawRangeListsOffset<T>`

```rust
struct RawRangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-clone"></span>`fn clone(&self) -> RawRangeListsOffset<T>` — [`RawRangeListsOffset`](#rawrangelistsoffset)

##### `impl<T: marker::Copy> Copy for RawRangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RawRangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-eq"></span>`fn eq(&self, other: &RawRangeListsOffset<T>) -> bool` — [`RawRangeListsOffset`](#rawrangelistsoffset)

##### `impl<T> StructuralPartialEq for RawRangeListsOffset<T>`

### `RangeListsOffset<T>`

```rust
struct RangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RangeListsOffset<T>`

- <span id="rangelistsoffset-clone"></span>`fn clone(&self) -> RangeListsOffset<T>` — [`RangeListsOffset`](#rangelistsoffset)

##### `impl<T: marker::Copy> Copy for RangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RangeListsOffset<T>`

- <span id="rangelistsoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RangeListsOffset<T>`

- <span id="rangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RangeListsOffset<T>`

- <span id="rangelistsoffset-eq"></span>`fn eq(&self, other: &RangeListsOffset<T>) -> bool` — [`RangeListsOffset`](#rangelistsoffset)

##### `impl<T> StructuralPartialEq for RangeListsOffset<T>`

### `DebugRngListsBase<T>`

```rust
struct DebugRngListsBase<T>(T);
```

An offset to a set of range list offsets in the `.debug_rnglists` section.

#### Implementations

- <span id="cratecommondebugrnglistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugRngListsBase`](#debugrnglistsbase)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-clone"></span>`fn clone(&self) -> DebugRngListsBase<T>` — [`DebugRngListsBase`](#debugrnglistsbase)

##### `impl<T: marker::Copy> Copy for DebugRngListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-eq"></span>`fn eq(&self, other: &DebugRngListsBase<T>) -> bool` — [`DebugRngListsBase`](#debugrnglistsbase)

##### `impl<T> StructuralPartialEq for DebugRngListsBase<T>`

### `DebugRngListsIndex<T>`

```rust
struct DebugRngListsIndex<T>(T);
```

An index into a set of range list offsets in the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-clone"></span>`fn clone(&self) -> DebugRngListsIndex<T>` — [`DebugRngListsIndex`](#debugrnglistsindex)

##### `impl<T: marker::Copy> Copy for DebugRngListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-eq"></span>`fn eq(&self, other: &DebugRngListsIndex<T>) -> bool` — [`DebugRngListsIndex`](#debugrnglistsindex)

##### `impl<T> StructuralPartialEq for DebugRngListsIndex<T>`

### `DebugStrOffset<T>`

```rust
struct DebugStrOffset<T>(T);
```

An offset into the `.debug_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffset<T>`

- <span id="debugstroffset-clone"></span>`fn clone(&self) -> DebugStrOffset<T>` — [`DebugStrOffset`](#debugstroffset)

##### `impl<T: marker::Copy> Copy for DebugStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffset<T>`

- <span id="debugstroffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffset<T>`

- <span id="debugstroffset-eq"></span>`fn eq(&self, other: &DebugStrOffset<T>) -> bool` — [`DebugStrOffset`](#debugstroffset)

##### `impl<T> StructuralPartialEq for DebugStrOffset<T>`

### `DebugStrOffsetsBase<T>`

```rust
struct DebugStrOffsetsBase<T>(T);
```

An offset to a set of entries in the `.debug_str_offsets` section.

#### Implementations

- <span id="cratecommondebugstroffsetsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugStrOffsetsBase`](#debugstroffsetsbase)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-clone"></span>`fn clone(&self) -> DebugStrOffsetsBase<T>` — [`DebugStrOffsetsBase`](#debugstroffsetsbase)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-eq"></span>`fn eq(&self, other: &DebugStrOffsetsBase<T>) -> bool` — [`DebugStrOffsetsBase`](#debugstroffsetsbase)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsBase<T>`

### `DebugStrOffsetsIndex<T>`

```rust
struct DebugStrOffsetsIndex<T>(T);
```

An index into a set of entries in the `.debug_str_offsets` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-clone"></span>`fn clone(&self) -> DebugStrOffsetsIndex<T>` — [`DebugStrOffsetsIndex`](#debugstroffsetsindex)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-eq"></span>`fn eq(&self, other: &DebugStrOffsetsIndex<T>) -> bool` — [`DebugStrOffsetsIndex`](#debugstroffsetsindex)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsIndex<T>`

### `DebugTypesOffset<T>`

```rust
struct DebugTypesOffset<T>(T);
```

An offset into the `.debug_types` section.

#### Implementations

- <span id="cratecommondebugtypesoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](read/index.md), [`UnitOffset`](#unitoffset)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugTypesOffset<T>`

- <span id="debugtypesoffset-clone"></span>`fn clone(&self) -> DebugTypesOffset<T>` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: marker::Copy> Copy for DebugTypesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugTypesOffset<T>`

- <span id="debugtypesoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugTypesOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugTypesOffset<T>`

- <span id="debugtypesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugTypesOffset<T>`

- <span id="debugtypesoffset-cmp"></span>`fn cmp(&self, other: &DebugTypesOffset<T>) -> cmp::Ordering` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugTypesOffset<T>`

- <span id="debugtypesoffset-eq"></span>`fn eq(&self, other: &DebugTypesOffset<T>) -> bool` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugTypesOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T> StructuralPartialEq for DebugTypesOffset<T>`

### `DebugTypeSignature`

```rust
struct DebugTypeSignature(u64);
```

A type signature as used in the `.debug_types` section.

#### Trait Implementations

##### `impl Clone for DebugTypeSignature`

- <span id="debugtypesignature-clone"></span>`fn clone(&self) -> DebugTypeSignature` — [`DebugTypeSignature`](#debugtypesignature)

##### `impl Copy for DebugTypeSignature`

##### `impl Debug for DebugTypeSignature`

- <span id="debugtypesignature-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DebugTypeSignature`

##### `impl Hash for DebugTypeSignature`

- <span id="debugtypesignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DebugTypeSignature`

- <span id="debugtypesignature-eq"></span>`fn eq(&self, other: &DebugTypeSignature) -> bool` — [`DebugTypeSignature`](#debugtypesignature)

##### `impl StructuralPartialEq for DebugTypeSignature`

### `DebugFrameOffset<T>`

```rust
struct DebugFrameOffset<T>(T);
```

An offset into the `.debug_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugFrameOffset<T>`

- <span id="debugframeoffset-clone"></span>`fn clone(&self) -> DebugFrameOffset<T>` — [`DebugFrameOffset`](#debugframeoffset)

##### `impl<T: marker::Copy> Copy for DebugFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugFrameOffset<T>`

- <span id="debugframeoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugFrameOffset<T>`

- <span id="debugframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugFrameOffset<T>`

- <span id="debugframeoffset-eq"></span>`fn eq(&self, other: &DebugFrameOffset<T>) -> bool` — [`DebugFrameOffset`](#debugframeoffset)

##### `impl<T> StructuralPartialEq for DebugFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::DebugFrameOffset<T>`

- <span id="cratecommondebugframeoffset-into"></span>`fn into(self) -> T`

### `EhFrameOffset<T>`

```rust
struct EhFrameOffset<T>(T);
```

An offset into the `.eh_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for EhFrameOffset<T>`

- <span id="ehframeoffset-clone"></span>`fn clone(&self) -> EhFrameOffset<T>` — [`EhFrameOffset`](#ehframeoffset)

##### `impl<T: marker::Copy> Copy for EhFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for EhFrameOffset<T>`

- <span id="ehframeoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for EhFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for EhFrameOffset<T>`

- <span id="ehframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for EhFrameOffset<T>`

- <span id="ehframeoffset-eq"></span>`fn eq(&self, other: &EhFrameOffset<T>) -> bool` — [`EhFrameOffset`](#ehframeoffset)

##### `impl<T> StructuralPartialEq for EhFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::EhFrameOffset<T>`

- <span id="cratecommonehframeoffset-into"></span>`fn into(self) -> T`

### `DwoId`

```rust
struct DwoId(u64);
```

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

#### Trait Implementations

##### `impl Clone for DwoId`

- <span id="dwoid-clone"></span>`fn clone(&self) -> DwoId` — [`DwoId`](#dwoid)

##### `impl Copy for DwoId`

##### `impl Debug for DwoId`

- <span id="dwoid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DwoId`

##### `impl Hash for DwoId`

- <span id="dwoid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DwoId`

- <span id="dwoid-eq"></span>`fn eq(&self, other: &DwoId) -> bool` — [`DwoId`](#dwoid)

##### `impl StructuralPartialEq for DwoId`

### `Arm`

```rust
struct Arm;
```

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

#### Implementations

- <span id="arm-r0"></span>`const R0: Register`

- <span id="arm-r1"></span>`const R1: Register`

- <span id="arm-r2"></span>`const R2: Register`

- <span id="arm-r3"></span>`const R3: Register`

- <span id="arm-r4"></span>`const R4: Register`

- <span id="arm-r5"></span>`const R5: Register`

- <span id="arm-r6"></span>`const R6: Register`

- <span id="arm-r7"></span>`const R7: Register`

- <span id="arm-r8"></span>`const R8: Register`

- <span id="arm-r9"></span>`const R9: Register`

- <span id="arm-r10"></span>`const R10: Register`

- <span id="arm-r11"></span>`const R11: Register`

- <span id="arm-r12"></span>`const R12: Register`

- <span id="arm-r13"></span>`const R13: Register`

- <span id="arm-r14"></span>`const R14: Register`

- <span id="arm-r15"></span>`const R15: Register`

- <span id="arm-wcgr0"></span>`const WCGR0: Register`

- <span id="arm-wcgr1"></span>`const WCGR1: Register`

- <span id="arm-wcgr2"></span>`const WCGR2: Register`

- <span id="arm-wcgr3"></span>`const WCGR3: Register`

- <span id="arm-wcgr4"></span>`const WCGR4: Register`

- <span id="arm-wcgr5"></span>`const WCGR5: Register`

- <span id="arm-wcgr6"></span>`const WCGR6: Register`

- <span id="arm-wcgr7"></span>`const WCGR7: Register`

- <span id="arm-wr0"></span>`const WR0: Register`

- <span id="arm-wr1"></span>`const WR1: Register`

- <span id="arm-wr2"></span>`const WR2: Register`

- <span id="arm-wr3"></span>`const WR3: Register`

- <span id="arm-wr4"></span>`const WR4: Register`

- <span id="arm-wr5"></span>`const WR5: Register`

- <span id="arm-wr6"></span>`const WR6: Register`

- <span id="arm-wr7"></span>`const WR7: Register`

- <span id="arm-wr8"></span>`const WR8: Register`

- <span id="arm-wr9"></span>`const WR9: Register`

- <span id="arm-wr10"></span>`const WR10: Register`

- <span id="arm-wr11"></span>`const WR11: Register`

- <span id="arm-wr12"></span>`const WR12: Register`

- <span id="arm-wr13"></span>`const WR13: Register`

- <span id="arm-wr14"></span>`const WR14: Register`

- <span id="arm-wr15"></span>`const WR15: Register`

- <span id="arm-spsr"></span>`const SPSR: Register`

- <span id="arm-spsr-fiq"></span>`const SPSR_FIQ: Register`

- <span id="arm-spsr-irq"></span>`const SPSR_IRQ: Register`

- <span id="arm-spsr-abt"></span>`const SPSR_ABT: Register`

- <span id="arm-spsr-und"></span>`const SPSR_UND: Register`

- <span id="arm-spsr-svc"></span>`const SPSR_SVC: Register`

- <span id="arm-ra-auth-code"></span>`const RA_AUTH_CODE: Register`

- <span id="arm-r8-usr"></span>`const R8_USR: Register`

- <span id="arm-r9-usr"></span>`const R9_USR: Register`

- <span id="arm-r10-usr"></span>`const R10_USR: Register`

- <span id="arm-r11-usr"></span>`const R11_USR: Register`

- <span id="arm-r12-usr"></span>`const R12_USR: Register`

- <span id="arm-r13-usr"></span>`const R13_USR: Register`

- <span id="arm-r14-usr"></span>`const R14_USR: Register`

- <span id="arm-r8-fiq"></span>`const R8_FIQ: Register`

- <span id="arm-r9-fiq"></span>`const R9_FIQ: Register`

- <span id="arm-r10-fiq"></span>`const R10_FIQ: Register`

- <span id="arm-r11-fiq"></span>`const R11_FIQ: Register`

- <span id="arm-r12-fiq"></span>`const R12_FIQ: Register`

- <span id="arm-r13-fiq"></span>`const R13_FIQ: Register`

- <span id="arm-r14-fiq"></span>`const R14_FIQ: Register`

- <span id="arm-r13-irq"></span>`const R13_IRQ: Register`

- <span id="arm-r14-irq"></span>`const R14_IRQ: Register`

- <span id="arm-r13-abt"></span>`const R13_ABT: Register`

- <span id="arm-r14-abt"></span>`const R14_ABT: Register`

- <span id="arm-r13-und"></span>`const R13_UND: Register`

- <span id="arm-r14-und"></span>`const R14_UND: Register`

- <span id="arm-r13-svc"></span>`const R13_SVC: Register`

- <span id="arm-r14-svc"></span>`const R14_SVC: Register`

- <span id="arm-wc0"></span>`const WC0: Register`

- <span id="arm-wc1"></span>`const WC1: Register`

- <span id="arm-wc2"></span>`const WC2: Register`

- <span id="arm-wc3"></span>`const WC3: Register`

- <span id="arm-wc4"></span>`const WC4: Register`

- <span id="arm-wc5"></span>`const WC5: Register`

- <span id="arm-wc6"></span>`const WC6: Register`

- <span id="arm-wc7"></span>`const WC7: Register`

- <span id="arm-d0"></span>`const D0: Register`

- <span id="arm-d1"></span>`const D1: Register`

- <span id="arm-d2"></span>`const D2: Register`

- <span id="arm-d3"></span>`const D3: Register`

- <span id="arm-d4"></span>`const D4: Register`

- <span id="arm-d5"></span>`const D5: Register`

- <span id="arm-d6"></span>`const D6: Register`

- <span id="arm-d7"></span>`const D7: Register`

- <span id="arm-d8"></span>`const D8: Register`

- <span id="arm-d9"></span>`const D9: Register`

- <span id="arm-d10"></span>`const D10: Register`

- <span id="arm-d11"></span>`const D11: Register`

- <span id="arm-d12"></span>`const D12: Register`

- <span id="arm-d13"></span>`const D13: Register`

- <span id="arm-d14"></span>`const D14: Register`

- <span id="arm-d15"></span>`const D15: Register`

- <span id="arm-d16"></span>`const D16: Register`

- <span id="arm-d17"></span>`const D17: Register`

- <span id="arm-d18"></span>`const D18: Register`

- <span id="arm-d19"></span>`const D19: Register`

- <span id="arm-d20"></span>`const D20: Register`

- <span id="arm-d21"></span>`const D21: Register`

- <span id="arm-d22"></span>`const D22: Register`

- <span id="arm-d23"></span>`const D23: Register`

- <span id="arm-d24"></span>`const D24: Register`

- <span id="arm-d25"></span>`const D25: Register`

- <span id="arm-d26"></span>`const D26: Register`

- <span id="arm-d27"></span>`const D27: Register`

- <span id="arm-d28"></span>`const D28: Register`

- <span id="arm-d29"></span>`const D29: Register`

- <span id="arm-d30"></span>`const D30: Register`

- <span id="arm-d31"></span>`const D31: Register`

- <span id="arm-tpidruro"></span>`const TPIDRURO: Register`

- <span id="arm-tpidrurw"></span>`const TPIDRURW: Register`

- <span id="arm-tpidpr"></span>`const TPIDPR: Register`

- <span id="arm-htpidpr"></span>`const HTPIDPR: Register`

- <span id="arm-sp"></span>`const SP: Register`

- <span id="arm-lr"></span>`const LR: Register`

- <span id="arm-pc"></span>`const PC: Register`

- <span id="arm-acc0"></span>`const ACC0: Register`

- <span id="arm-acc1"></span>`const ACC1: Register`

- <span id="arm-acc2"></span>`const ACC2: Register`

- <span id="arm-acc3"></span>`const ACC3: Register`

- <span id="arm-acc4"></span>`const ACC4: Register`

- <span id="arm-acc5"></span>`const ACC5: Register`

- <span id="arm-acc6"></span>`const ACC6: Register`

- <span id="arm-acc7"></span>`const ACC7: Register`

- <span id="arm-s0"></span>`const S0: Register`

- <span id="arm-s1"></span>`const S1: Register`

- <span id="arm-s2"></span>`const S2: Register`

- <span id="arm-s3"></span>`const S3: Register`

- <span id="arm-s4"></span>`const S4: Register`

- <span id="arm-s5"></span>`const S5: Register`

- <span id="arm-s6"></span>`const S6: Register`

- <span id="arm-s7"></span>`const S7: Register`

- <span id="arm-s8"></span>`const S8: Register`

- <span id="arm-s9"></span>`const S9: Register`

- <span id="arm-s10"></span>`const S10: Register`

- <span id="arm-s11"></span>`const S11: Register`

- <span id="arm-s12"></span>`const S12: Register`

- <span id="arm-s13"></span>`const S13: Register`

- <span id="arm-s14"></span>`const S14: Register`

- <span id="arm-s15"></span>`const S15: Register`

- <span id="arm-s16"></span>`const S16: Register`

- <span id="arm-s17"></span>`const S17: Register`

- <span id="arm-s18"></span>`const S18: Register`

- <span id="arm-s19"></span>`const S19: Register`

- <span id="arm-s20"></span>`const S20: Register`

- <span id="arm-s21"></span>`const S21: Register`

- <span id="arm-s22"></span>`const S22: Register`

- <span id="arm-s23"></span>`const S23: Register`

- <span id="arm-s24"></span>`const S24: Register`

- <span id="arm-s25"></span>`const S25: Register`

- <span id="arm-s26"></span>`const S26: Register`

- <span id="arm-s27"></span>`const S27: Register`

- <span id="arm-s28"></span>`const S28: Register`

- <span id="arm-s29"></span>`const S29: Register`

- <span id="arm-s30"></span>`const S30: Register`

- <span id="arm-s31"></span>`const S31: Register`

#### Trait Implementations

##### `impl Clone for Arm`

- <span id="arm-clone"></span>`fn clone(&self) -> Arm` — [`Arm`](#arm)

##### `impl Copy for Arm`

##### `impl Debug for Arm`

- <span id="arm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AArch64`

```rust
struct AArch64;
```

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

#### Implementations

- <span id="aarch64-x0"></span>`const X0: Register`

- <span id="aarch64-x1"></span>`const X1: Register`

- <span id="aarch64-x2"></span>`const X2: Register`

- <span id="aarch64-x3"></span>`const X3: Register`

- <span id="aarch64-x4"></span>`const X4: Register`

- <span id="aarch64-x5"></span>`const X5: Register`

- <span id="aarch64-x6"></span>`const X6: Register`

- <span id="aarch64-x7"></span>`const X7: Register`

- <span id="aarch64-x8"></span>`const X8: Register`

- <span id="aarch64-x9"></span>`const X9: Register`

- <span id="aarch64-x10"></span>`const X10: Register`

- <span id="aarch64-x11"></span>`const X11: Register`

- <span id="aarch64-x12"></span>`const X12: Register`

- <span id="aarch64-x13"></span>`const X13: Register`

- <span id="aarch64-x14"></span>`const X14: Register`

- <span id="aarch64-x15"></span>`const X15: Register`

- <span id="aarch64-x16"></span>`const X16: Register`

- <span id="aarch64-x17"></span>`const X17: Register`

- <span id="aarch64-x18"></span>`const X18: Register`

- <span id="aarch64-x19"></span>`const X19: Register`

- <span id="aarch64-x20"></span>`const X20: Register`

- <span id="aarch64-x21"></span>`const X21: Register`

- <span id="aarch64-x22"></span>`const X22: Register`

- <span id="aarch64-x23"></span>`const X23: Register`

- <span id="aarch64-x24"></span>`const X24: Register`

- <span id="aarch64-x25"></span>`const X25: Register`

- <span id="aarch64-x26"></span>`const X26: Register`

- <span id="aarch64-x27"></span>`const X27: Register`

- <span id="aarch64-x28"></span>`const X28: Register`

- <span id="aarch64-x29"></span>`const X29: Register`

- <span id="aarch64-x30"></span>`const X30: Register`

- <span id="aarch64-sp"></span>`const SP: Register`

- <span id="aarch64-pc"></span>`const PC: Register`

- <span id="aarch64-elr-mode"></span>`const ELR_MODE: Register`

- <span id="aarch64-ra-sign-state"></span>`const RA_SIGN_STATE: Register`

- <span id="aarch64-tpidrro-el0"></span>`const TPIDRRO_EL0: Register`

- <span id="aarch64-tpidr-el0"></span>`const TPIDR_EL0: Register`

- <span id="aarch64-tpidr-el1"></span>`const TPIDR_EL1: Register`

- <span id="aarch64-tpidr-el2"></span>`const TPIDR_EL2: Register`

- <span id="aarch64-tpidr-el3"></span>`const TPIDR_EL3: Register`

- <span id="aarch64-vg"></span>`const VG: Register`

- <span id="aarch64-ffr"></span>`const FFR: Register`

- <span id="aarch64-p0"></span>`const P0: Register`

- <span id="aarch64-p1"></span>`const P1: Register`

- <span id="aarch64-p2"></span>`const P2: Register`

- <span id="aarch64-p3"></span>`const P3: Register`

- <span id="aarch64-p4"></span>`const P4: Register`

- <span id="aarch64-p5"></span>`const P5: Register`

- <span id="aarch64-p6"></span>`const P6: Register`

- <span id="aarch64-p7"></span>`const P7: Register`

- <span id="aarch64-p8"></span>`const P8: Register`

- <span id="aarch64-p9"></span>`const P9: Register`

- <span id="aarch64-p10"></span>`const P10: Register`

- <span id="aarch64-p11"></span>`const P11: Register`

- <span id="aarch64-p12"></span>`const P12: Register`

- <span id="aarch64-p13"></span>`const P13: Register`

- <span id="aarch64-p14"></span>`const P14: Register`

- <span id="aarch64-p15"></span>`const P15: Register`

- <span id="aarch64-v0"></span>`const V0: Register`

- <span id="aarch64-v1"></span>`const V1: Register`

- <span id="aarch64-v2"></span>`const V2: Register`

- <span id="aarch64-v3"></span>`const V3: Register`

- <span id="aarch64-v4"></span>`const V4: Register`

- <span id="aarch64-v5"></span>`const V5: Register`

- <span id="aarch64-v6"></span>`const V6: Register`

- <span id="aarch64-v7"></span>`const V7: Register`

- <span id="aarch64-v8"></span>`const V8: Register`

- <span id="aarch64-v9"></span>`const V9: Register`

- <span id="aarch64-v10"></span>`const V10: Register`

- <span id="aarch64-v11"></span>`const V11: Register`

- <span id="aarch64-v12"></span>`const V12: Register`

- <span id="aarch64-v13"></span>`const V13: Register`

- <span id="aarch64-v14"></span>`const V14: Register`

- <span id="aarch64-v15"></span>`const V15: Register`

- <span id="aarch64-v16"></span>`const V16: Register`

- <span id="aarch64-v17"></span>`const V17: Register`

- <span id="aarch64-v18"></span>`const V18: Register`

- <span id="aarch64-v19"></span>`const V19: Register`

- <span id="aarch64-v20"></span>`const V20: Register`

- <span id="aarch64-v21"></span>`const V21: Register`

- <span id="aarch64-v22"></span>`const V22: Register`

- <span id="aarch64-v23"></span>`const V23: Register`

- <span id="aarch64-v24"></span>`const V24: Register`

- <span id="aarch64-v25"></span>`const V25: Register`

- <span id="aarch64-v26"></span>`const V26: Register`

- <span id="aarch64-v27"></span>`const V27: Register`

- <span id="aarch64-v28"></span>`const V28: Register`

- <span id="aarch64-v29"></span>`const V29: Register`

- <span id="aarch64-v30"></span>`const V30: Register`

- <span id="aarch64-v31"></span>`const V31: Register`

- <span id="aarch64-z0"></span>`const Z0: Register`

- <span id="aarch64-z1"></span>`const Z1: Register`

- <span id="aarch64-z2"></span>`const Z2: Register`

- <span id="aarch64-z3"></span>`const Z3: Register`

- <span id="aarch64-z4"></span>`const Z4: Register`

- <span id="aarch64-z5"></span>`const Z5: Register`

- <span id="aarch64-z6"></span>`const Z6: Register`

- <span id="aarch64-z7"></span>`const Z7: Register`

- <span id="aarch64-z8"></span>`const Z8: Register`

- <span id="aarch64-z9"></span>`const Z9: Register`

- <span id="aarch64-z10"></span>`const Z10: Register`

- <span id="aarch64-z11"></span>`const Z11: Register`

- <span id="aarch64-z12"></span>`const Z12: Register`

- <span id="aarch64-z13"></span>`const Z13: Register`

- <span id="aarch64-z14"></span>`const Z14: Register`

- <span id="aarch64-z15"></span>`const Z15: Register`

- <span id="aarch64-z16"></span>`const Z16: Register`

- <span id="aarch64-z17"></span>`const Z17: Register`

- <span id="aarch64-z18"></span>`const Z18: Register`

- <span id="aarch64-z19"></span>`const Z19: Register`

- <span id="aarch64-z20"></span>`const Z20: Register`

- <span id="aarch64-z21"></span>`const Z21: Register`

- <span id="aarch64-z22"></span>`const Z22: Register`

- <span id="aarch64-z23"></span>`const Z23: Register`

- <span id="aarch64-z24"></span>`const Z24: Register`

- <span id="aarch64-z25"></span>`const Z25: Register`

- <span id="aarch64-z26"></span>`const Z26: Register`

- <span id="aarch64-z27"></span>`const Z27: Register`

- <span id="aarch64-z28"></span>`const Z28: Register`

- <span id="aarch64-z29"></span>`const Z29: Register`

- <span id="aarch64-z30"></span>`const Z30: Register`

- <span id="aarch64-z31"></span>`const Z31: Register`

#### Trait Implementations

##### `impl Clone for AArch64`

- <span id="aarch64-clone"></span>`fn clone(&self) -> AArch64` — [`AArch64`](#aarch64)

##### `impl Copy for AArch64`

##### `impl Debug for AArch64`

- <span id="aarch64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LoongArch`

```rust
struct LoongArch;
```

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

#### Implementations

- <span id="loongarch-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](#register)

- <span id="loongarch-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](#register)

#### Trait Implementations

##### `impl Clone for LoongArch`

- <span id="loongarch-clone"></span>`fn clone(&self) -> LoongArch` — [`LoongArch`](#loongarch)

##### `impl Copy for LoongArch`

##### `impl Debug for LoongArch`

- <span id="loongarch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MIPS`

```rust
struct MIPS;
```

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

#### Implementations

- <span id="mips-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](#register)

- <span id="mips-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](#register)

#### Trait Implementations

##### `impl Clone for MIPS`

- <span id="mips-clone"></span>`fn clone(&self) -> MIPS` — [`MIPS`](#mips)

##### `impl Copy for MIPS`

##### `impl Debug for MIPS`

- <span id="mips-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RiscV`

```rust
struct RiscV;
```

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

#### Implementations

- <span id="riscv-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](#register)

- <span id="riscv-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](#register)

#### Trait Implementations

##### `impl Clone for RiscV`

- <span id="riscv-clone"></span>`fn clone(&self) -> RiscV` — [`RiscV`](#riscv)

##### `impl Copy for RiscV`

##### `impl Debug for RiscV`

- <span id="riscv-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86`

```rust
struct X86;
```

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

#### Implementations

- <span id="x86-eax"></span>`const EAX: Register`

- <span id="x86-ecx"></span>`const ECX: Register`

- <span id="x86-edx"></span>`const EDX: Register`

- <span id="x86-ebx"></span>`const EBX: Register`

- <span id="x86-esp"></span>`const ESP: Register`

- <span id="x86-ebp"></span>`const EBP: Register`

- <span id="x86-esi"></span>`const ESI: Register`

- <span id="x86-edi"></span>`const EDI: Register`

- <span id="x86-ra"></span>`const RA: Register`

- <span id="x86-st0"></span>`const ST0: Register`

- <span id="x86-st1"></span>`const ST1: Register`

- <span id="x86-st2"></span>`const ST2: Register`

- <span id="x86-st3"></span>`const ST3: Register`

- <span id="x86-st4"></span>`const ST4: Register`

- <span id="x86-st5"></span>`const ST5: Register`

- <span id="x86-st6"></span>`const ST6: Register`

- <span id="x86-st7"></span>`const ST7: Register`

- <span id="x86-xmm0"></span>`const XMM0: Register`

- <span id="x86-xmm1"></span>`const XMM1: Register`

- <span id="x86-xmm2"></span>`const XMM2: Register`

- <span id="x86-xmm3"></span>`const XMM3: Register`

- <span id="x86-xmm4"></span>`const XMM4: Register`

- <span id="x86-xmm5"></span>`const XMM5: Register`

- <span id="x86-xmm6"></span>`const XMM6: Register`

- <span id="x86-xmm7"></span>`const XMM7: Register`

- <span id="x86-mm0"></span>`const MM0: Register`

- <span id="x86-mm1"></span>`const MM1: Register`

- <span id="x86-mm2"></span>`const MM2: Register`

- <span id="x86-mm3"></span>`const MM3: Register`

- <span id="x86-mm4"></span>`const MM4: Register`

- <span id="x86-mm5"></span>`const MM5: Register`

- <span id="x86-mm6"></span>`const MM6: Register`

- <span id="x86-mm7"></span>`const MM7: Register`

- <span id="x86-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-es"></span>`const ES: Register`

- <span id="x86-cs"></span>`const CS: Register`

- <span id="x86-ss"></span>`const SS: Register`

- <span id="x86-ds"></span>`const DS: Register`

- <span id="x86-fs"></span>`const FS: Register`

- <span id="x86-gs"></span>`const GS: Register`

- <span id="x86-tr"></span>`const TR: Register`

- <span id="x86-ldtr"></span>`const LDTR: Register`

- <span id="x86-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-gs-base"></span>`const GS_BASE: Register`

#### Trait Implementations

##### `impl Clone for X86`

- <span id="x86-clone"></span>`fn clone(&self) -> X86` — [`X86`](#x86)

##### `impl Copy for X86`

##### `impl Debug for X86`

- <span id="x86-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86_64`

```rust
struct X86_64;
```

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

#### Implementations

- <span id="x86-64-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](#register)

- <span id="x86-64-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](#register)

#### Trait Implementations

##### `impl Clone for X86_64`

- <span id="x86-64-clone"></span>`fn clone(&self) -> X86_64` — [`X86_64`](#x86-64)

##### `impl Copy for X86_64`

##### `impl Debug for X86_64`

- <span id="x86-64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PowerPc64`

```rust
struct PowerPc64;
```

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

#### Implementations

- <span id="powerpc64-r0"></span>`const R0: Register`

- <span id="powerpc64-r1"></span>`const R1: Register`

- <span id="powerpc64-r2"></span>`const R2: Register`

- <span id="powerpc64-r3"></span>`const R3: Register`

- <span id="powerpc64-r4"></span>`const R4: Register`

- <span id="powerpc64-r5"></span>`const R5: Register`

- <span id="powerpc64-r6"></span>`const R6: Register`

- <span id="powerpc64-r7"></span>`const R7: Register`

- <span id="powerpc64-r8"></span>`const R8: Register`

- <span id="powerpc64-r9"></span>`const R9: Register`

- <span id="powerpc64-r10"></span>`const R10: Register`

- <span id="powerpc64-r11"></span>`const R11: Register`

- <span id="powerpc64-r12"></span>`const R12: Register`

- <span id="powerpc64-r13"></span>`const R13: Register`

- <span id="powerpc64-r14"></span>`const R14: Register`

- <span id="powerpc64-r15"></span>`const R15: Register`

- <span id="powerpc64-r16"></span>`const R16: Register`

- <span id="powerpc64-r17"></span>`const R17: Register`

- <span id="powerpc64-r18"></span>`const R18: Register`

- <span id="powerpc64-r19"></span>`const R19: Register`

- <span id="powerpc64-r20"></span>`const R20: Register`

- <span id="powerpc64-r21"></span>`const R21: Register`

- <span id="powerpc64-r22"></span>`const R22: Register`

- <span id="powerpc64-r23"></span>`const R23: Register`

- <span id="powerpc64-r24"></span>`const R24: Register`

- <span id="powerpc64-r25"></span>`const R25: Register`

- <span id="powerpc64-r26"></span>`const R26: Register`

- <span id="powerpc64-r27"></span>`const R27: Register`

- <span id="powerpc64-r28"></span>`const R28: Register`

- <span id="powerpc64-r29"></span>`const R29: Register`

- <span id="powerpc64-r30"></span>`const R30: Register`

- <span id="powerpc64-r31"></span>`const R31: Register`

- <span id="powerpc64-f0"></span>`const F0: Register`

- <span id="powerpc64-f1"></span>`const F1: Register`

- <span id="powerpc64-f2"></span>`const F2: Register`

- <span id="powerpc64-f3"></span>`const F3: Register`

- <span id="powerpc64-f4"></span>`const F4: Register`

- <span id="powerpc64-f5"></span>`const F5: Register`

- <span id="powerpc64-f6"></span>`const F6: Register`

- <span id="powerpc64-f7"></span>`const F7: Register`

- <span id="powerpc64-f8"></span>`const F8: Register`

- <span id="powerpc64-f9"></span>`const F9: Register`

- <span id="powerpc64-f10"></span>`const F10: Register`

- <span id="powerpc64-f11"></span>`const F11: Register`

- <span id="powerpc64-f12"></span>`const F12: Register`

- <span id="powerpc64-f13"></span>`const F13: Register`

- <span id="powerpc64-f14"></span>`const F14: Register`

- <span id="powerpc64-f15"></span>`const F15: Register`

- <span id="powerpc64-f16"></span>`const F16: Register`

- <span id="powerpc64-f17"></span>`const F17: Register`

- <span id="powerpc64-f18"></span>`const F18: Register`

- <span id="powerpc64-f19"></span>`const F19: Register`

- <span id="powerpc64-f20"></span>`const F20: Register`

- <span id="powerpc64-f21"></span>`const F21: Register`

- <span id="powerpc64-f22"></span>`const F22: Register`

- <span id="powerpc64-f23"></span>`const F23: Register`

- <span id="powerpc64-f24"></span>`const F24: Register`

- <span id="powerpc64-f25"></span>`const F25: Register`

- <span id="powerpc64-f26"></span>`const F26: Register`

- <span id="powerpc64-f27"></span>`const F27: Register`

- <span id="powerpc64-f28"></span>`const F28: Register`

- <span id="powerpc64-f29"></span>`const F29: Register`

- <span id="powerpc64-f30"></span>`const F30: Register`

- <span id="powerpc64-f31"></span>`const F31: Register`

- <span id="powerpc64-lr"></span>`const LR: Register`

- <span id="powerpc64-ctr"></span>`const CTR: Register`

- <span id="powerpc64-cr0"></span>`const CR0: Register`

- <span id="powerpc64-cr1"></span>`const CR1: Register`

- <span id="powerpc64-cr2"></span>`const CR2: Register`

- <span id="powerpc64-cr3"></span>`const CR3: Register`

- <span id="powerpc64-cr4"></span>`const CR4: Register`

- <span id="powerpc64-cr5"></span>`const CR5: Register`

- <span id="powerpc64-cr6"></span>`const CR6: Register`

- <span id="powerpc64-cr7"></span>`const CR7: Register`

- <span id="powerpc64-xer"></span>`const XER: Register`

- <span id="powerpc64-vr0"></span>`const VR0: Register`

- <span id="powerpc64-vr1"></span>`const VR1: Register`

- <span id="powerpc64-vr2"></span>`const VR2: Register`

- <span id="powerpc64-vr3"></span>`const VR3: Register`

- <span id="powerpc64-vr4"></span>`const VR4: Register`

- <span id="powerpc64-vr5"></span>`const VR5: Register`

- <span id="powerpc64-vr6"></span>`const VR6: Register`

- <span id="powerpc64-vr7"></span>`const VR7: Register`

- <span id="powerpc64-vr8"></span>`const VR8: Register`

- <span id="powerpc64-vr9"></span>`const VR9: Register`

- <span id="powerpc64-vr10"></span>`const VR10: Register`

- <span id="powerpc64-vr11"></span>`const VR11: Register`

- <span id="powerpc64-vr12"></span>`const VR12: Register`

- <span id="powerpc64-vr13"></span>`const VR13: Register`

- <span id="powerpc64-vr14"></span>`const VR14: Register`

- <span id="powerpc64-vr15"></span>`const VR15: Register`

- <span id="powerpc64-vr16"></span>`const VR16: Register`

- <span id="powerpc64-vr17"></span>`const VR17: Register`

- <span id="powerpc64-vr18"></span>`const VR18: Register`

- <span id="powerpc64-vr19"></span>`const VR19: Register`

- <span id="powerpc64-vr20"></span>`const VR20: Register`

- <span id="powerpc64-vr21"></span>`const VR21: Register`

- <span id="powerpc64-vr22"></span>`const VR22: Register`

- <span id="powerpc64-vr23"></span>`const VR23: Register`

- <span id="powerpc64-vr24"></span>`const VR24: Register`

- <span id="powerpc64-vr25"></span>`const VR25: Register`

- <span id="powerpc64-vr26"></span>`const VR26: Register`

- <span id="powerpc64-vr27"></span>`const VR27: Register`

- <span id="powerpc64-vr28"></span>`const VR28: Register`

- <span id="powerpc64-vr29"></span>`const VR29: Register`

- <span id="powerpc64-vr30"></span>`const VR30: Register`

- <span id="powerpc64-vr31"></span>`const VR31: Register`

- <span id="powerpc64-vscr"></span>`const VSCR: Register`

- <span id="powerpc64-tfhar"></span>`const TFHAR: Register`

- <span id="powerpc64-tfiar"></span>`const TFIAR: Register`

- <span id="powerpc64-texasr"></span>`const TEXASR: Register`

#### Trait Implementations

##### `impl Clone for PowerPc64`

- <span id="powerpc64-clone"></span>`fn clone(&self) -> PowerPc64` — [`PowerPc64`](#powerpc64)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- <span id="powerpc64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DwSect`

```rust
struct DwSect(u32);
```

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

#### Implementations

- <span id="dwsect-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSect`

- <span id="dwsect-clone"></span>`fn clone(&self) -> DwSect` — [`DwSect`](#dwsect)

##### `impl Copy for DwSect`

##### `impl Debug for DwSect`

- <span id="dwsect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSect`

- <span id="dwsect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSect`

##### `impl Hash for DwSect`

- <span id="dwsect-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSect`

- <span id="dwsect-cmp"></span>`fn cmp(&self, other: &DwSect) -> cmp::Ordering` — [`DwSect`](#dwsect)

##### `impl PartialEq for DwSect`

- <span id="dwsect-eq"></span>`fn eq(&self, other: &DwSect) -> bool` — [`DwSect`](#dwsect)

##### `impl PartialOrd for DwSect`

- <span id="dwsect-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSect) -> option::Option<cmp::Ordering>` — [`DwSect`](#dwsect)

##### `impl StructuralPartialEq for DwSect`

##### `impl<T> ToString for DwSect`

- <span id="dwsect-to-string"></span>`fn to_string(&self) -> String`

### `DwSectV2`

```rust
struct DwSectV2(u32);
```

The section type field in a `.dwp` unit index with version 2.

#### Implementations

- <span id="dwsectv2-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSectV2`

- <span id="dwsectv2-clone"></span>`fn clone(&self) -> DwSectV2` — [`DwSectV2`](#dwsectv2)

##### `impl Copy for DwSectV2`

##### `impl Debug for DwSectV2`

- <span id="dwsectv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSectV2`

- <span id="dwsectv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSectV2`

##### `impl Hash for DwSectV2`

- <span id="dwsectv2-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSectV2`

- <span id="dwsectv2-cmp"></span>`fn cmp(&self, other: &DwSectV2) -> cmp::Ordering` — [`DwSectV2`](#dwsectv2)

##### `impl PartialEq for DwSectV2`

- <span id="dwsectv2-eq"></span>`fn eq(&self, other: &DwSectV2) -> bool` — [`DwSectV2`](#dwsectv2)

##### `impl PartialOrd for DwSectV2`

- <span id="dwsectv2-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSectV2) -> option::Option<cmp::Ordering>` — [`DwSectV2`](#dwsectv2)

##### `impl StructuralPartialEq for DwSectV2`

##### `impl<T> ToString for DwSectV2`

- <span id="dwsectv2-to-string"></span>`fn to_string(&self) -> String`

### `DwUt`

```rust
struct DwUt(u8);
```

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

#### Implementations

- <span id="dwut-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwUt`

- <span id="dwut-clone"></span>`fn clone(&self) -> DwUt` — [`DwUt`](#dwut)

##### `impl Copy for DwUt`

##### `impl Debug for DwUt`

- <span id="dwut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwUt`

- <span id="dwut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwUt`

##### `impl Hash for DwUt`

- <span id="dwut-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwUt`

- <span id="dwut-cmp"></span>`fn cmp(&self, other: &DwUt) -> cmp::Ordering` — [`DwUt`](#dwut)

##### `impl PartialEq for DwUt`

- <span id="dwut-eq"></span>`fn eq(&self, other: &DwUt) -> bool` — [`DwUt`](#dwut)

##### `impl PartialOrd for DwUt`

- <span id="dwut-partial-cmp"></span>`fn partial_cmp(&self, other: &DwUt) -> option::Option<cmp::Ordering>` — [`DwUt`](#dwut)

##### `impl StructuralPartialEq for DwUt`

##### `impl<T> ToString for DwUt`

- <span id="dwut-to-string"></span>`fn to_string(&self) -> String`

### `DwCfa`

```rust
struct DwCfa(u8);
```

The opcode for a call frame instruction.

Section 7.24:
> Call frame instructions are encoded in one or more bytes. The primary
> opcode is encoded in the high order two bits of the first byte (that is,
> opcode = byte >> 6). An operand or extended opcode may be encoded in the
> low order 6 bits. Additional operands are encoded in subsequent bytes.

#### Implementations

- <span id="dwcfa-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCfa`

- <span id="dwcfa-clone"></span>`fn clone(&self) -> DwCfa` — [`DwCfa`](#dwcfa)

##### `impl Copy for DwCfa`

##### `impl Debug for DwCfa`

- <span id="dwcfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCfa`

- <span id="dwcfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCfa`

##### `impl Hash for DwCfa`

- <span id="dwcfa-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCfa`

- <span id="dwcfa-cmp"></span>`fn cmp(&self, other: &DwCfa) -> cmp::Ordering` — [`DwCfa`](#dwcfa)

##### `impl PartialEq for DwCfa`

- <span id="dwcfa-eq"></span>`fn eq(&self, other: &DwCfa) -> bool` — [`DwCfa`](#dwcfa)

##### `impl PartialOrd for DwCfa`

- <span id="dwcfa-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCfa) -> option::Option<cmp::Ordering>` — [`DwCfa`](#dwcfa)

##### `impl StructuralPartialEq for DwCfa`

##### `impl<T> ToString for DwCfa`

- <span id="dwcfa-to-string"></span>`fn to_string(&self) -> String`

### `DwChildren`

```rust
struct DwChildren(u8);
```

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

#### Implementations

- <span id="dwchildren-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwChildren`

- <span id="dwchildren-clone"></span>`fn clone(&self) -> DwChildren` — [`DwChildren`](#dwchildren)

##### `impl Copy for DwChildren`

##### `impl Debug for DwChildren`

- <span id="dwchildren-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwChildren`

- <span id="dwchildren-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwChildren`

##### `impl Hash for DwChildren`

- <span id="dwchildren-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwChildren`

- <span id="dwchildren-cmp"></span>`fn cmp(&self, other: &DwChildren) -> cmp::Ordering` — [`DwChildren`](#dwchildren)

##### `impl PartialEq for DwChildren`

- <span id="dwchildren-eq"></span>`fn eq(&self, other: &DwChildren) -> bool` — [`DwChildren`](#dwchildren)

##### `impl PartialOrd for DwChildren`

- <span id="dwchildren-partial-cmp"></span>`fn partial_cmp(&self, other: &DwChildren) -> option::Option<cmp::Ordering>` — [`DwChildren`](#dwchildren)

##### `impl StructuralPartialEq for DwChildren`

##### `impl<T> ToString for DwChildren`

- <span id="dwchildren-to-string"></span>`fn to_string(&self) -> String`

### `DwTag`

```rust
struct DwTag(u16);
```

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

#### Implementations

- <span id="dwtag-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwTag`

- <span id="dwtag-clone"></span>`fn clone(&self) -> DwTag` — [`DwTag`](#dwtag)

##### `impl Copy for DwTag`

##### `impl Debug for DwTag`

- <span id="dwtag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwTag`

- <span id="dwtag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwTag`

##### `impl Hash for DwTag`

- <span id="dwtag-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwTag`

- <span id="dwtag-cmp"></span>`fn cmp(&self, other: &DwTag) -> cmp::Ordering` — [`DwTag`](#dwtag)

##### `impl PartialEq for DwTag`

- <span id="dwtag-eq"></span>`fn eq(&self, other: &DwTag) -> bool` — [`DwTag`](#dwtag)

##### `impl PartialOrd for DwTag`

- <span id="dwtag-partial-cmp"></span>`fn partial_cmp(&self, other: &DwTag) -> option::Option<cmp::Ordering>` — [`DwTag`](#dwtag)

##### `impl StructuralPartialEq for DwTag`

##### `impl<T> ToString for DwTag`

- <span id="dwtag-to-string"></span>`fn to_string(&self) -> String`

### `DwAt`

```rust
struct DwAt(u16);
```

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

#### Implementations

- <span id="dwat-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAt`

- <span id="dwat-clone"></span>`fn clone(&self) -> DwAt` — [`DwAt`](#dwat)

##### `impl Copy for DwAt`

##### `impl Debug for DwAt`

- <span id="dwat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAt`

- <span id="dwat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAt`

##### `impl Hash for DwAt`

- <span id="dwat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAt`

- <span id="dwat-cmp"></span>`fn cmp(&self, other: &DwAt) -> cmp::Ordering` — [`DwAt`](#dwat)

##### `impl PartialEq for DwAt`

- <span id="dwat-eq"></span>`fn eq(&self, other: &DwAt) -> bool` — [`DwAt`](#dwat)

##### `impl PartialOrd for DwAt`

- <span id="dwat-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAt) -> option::Option<cmp::Ordering>` — [`DwAt`](#dwat)

##### `impl StructuralPartialEq for DwAt`

##### `impl<T> ToString for DwAt`

- <span id="dwat-to-string"></span>`fn to_string(&self) -> String`

### `DwForm`

```rust
struct DwForm(u16);
```

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

#### Implementations

- <span id="dwform-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwForm`

- <span id="dwform-clone"></span>`fn clone(&self) -> DwForm` — [`DwForm`](#dwform)

##### `impl Copy for DwForm`

##### `impl Debug for DwForm`

- <span id="dwform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwForm`

- <span id="dwform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwForm`

##### `impl Hash for DwForm`

- <span id="dwform-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwForm`

- <span id="dwform-cmp"></span>`fn cmp(&self, other: &DwForm) -> cmp::Ordering` — [`DwForm`](#dwform)

##### `impl PartialEq for DwForm`

- <span id="dwform-eq"></span>`fn eq(&self, other: &DwForm) -> bool` — [`DwForm`](#dwform)

##### `impl PartialOrd for DwForm`

- <span id="dwform-partial-cmp"></span>`fn partial_cmp(&self, other: &DwForm) -> option::Option<cmp::Ordering>` — [`DwForm`](#dwform)

##### `impl StructuralPartialEq for DwForm`

##### `impl<T> ToString for DwForm`

- <span id="dwform-to-string"></span>`fn to_string(&self) -> String`

### `DwAte`

```rust
struct DwAte(u8);
```

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

#### Implementations

- <span id="dwate-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAte`

- <span id="dwate-clone"></span>`fn clone(&self) -> DwAte` — [`DwAte`](#dwate)

##### `impl Copy for DwAte`

##### `impl Debug for DwAte`

- <span id="dwate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAte`

- <span id="dwate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAte`

##### `impl Hash for DwAte`

- <span id="dwate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAte`

- <span id="dwate-cmp"></span>`fn cmp(&self, other: &DwAte) -> cmp::Ordering` — [`DwAte`](#dwate)

##### `impl PartialEq for DwAte`

- <span id="dwate-eq"></span>`fn eq(&self, other: &DwAte) -> bool` — [`DwAte`](#dwate)

##### `impl PartialOrd for DwAte`

- <span id="dwate-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAte) -> option::Option<cmp::Ordering>` — [`DwAte`](#dwate)

##### `impl StructuralPartialEq for DwAte`

##### `impl<T> ToString for DwAte`

- <span id="dwate-to-string"></span>`fn to_string(&self) -> String`

### `DwLle`

```rust
struct DwLle(u8);
```

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

#### Implementations

- <span id="dwlle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLle`

- <span id="dwlle-clone"></span>`fn clone(&self) -> DwLle` — [`DwLle`](#dwlle)

##### `impl Copy for DwLle`

##### `impl Debug for DwLle`

- <span id="dwlle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLle`

- <span id="dwlle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLle`

##### `impl Hash for DwLle`

- <span id="dwlle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLle`

- <span id="dwlle-cmp"></span>`fn cmp(&self, other: &DwLle) -> cmp::Ordering` — [`DwLle`](#dwlle)

##### `impl PartialEq for DwLle`

- <span id="dwlle-eq"></span>`fn eq(&self, other: &DwLle) -> bool` — [`DwLle`](#dwlle)

##### `impl PartialOrd for DwLle`

- <span id="dwlle-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLle) -> option::Option<cmp::Ordering>` — [`DwLle`](#dwlle)

##### `impl StructuralPartialEq for DwLle`

##### `impl<T> ToString for DwLle`

- <span id="dwlle-to-string"></span>`fn to_string(&self) -> String`

### `DwDs`

```rust
struct DwDs(u8);
```

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

#### Implementations

- <span id="dwds-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDs`

- <span id="dwds-clone"></span>`fn clone(&self) -> DwDs` — [`DwDs`](#dwds)

##### `impl Copy for DwDs`

##### `impl Debug for DwDs`

- <span id="dwds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDs`

- <span id="dwds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDs`

##### `impl Hash for DwDs`

- <span id="dwds-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDs`

- <span id="dwds-cmp"></span>`fn cmp(&self, other: &DwDs) -> cmp::Ordering` — [`DwDs`](#dwds)

##### `impl PartialEq for DwDs`

- <span id="dwds-eq"></span>`fn eq(&self, other: &DwDs) -> bool` — [`DwDs`](#dwds)

##### `impl PartialOrd for DwDs`

- <span id="dwds-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDs) -> option::Option<cmp::Ordering>` — [`DwDs`](#dwds)

##### `impl StructuralPartialEq for DwDs`

##### `impl<T> ToString for DwDs`

- <span id="dwds-to-string"></span>`fn to_string(&self) -> String`

### `DwEnd`

```rust
struct DwEnd(u8);
```

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

#### Implementations

- <span id="dwend-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwEnd`

- <span id="dwend-clone"></span>`fn clone(&self) -> DwEnd` — [`DwEnd`](#dwend)

##### `impl Copy for DwEnd`

##### `impl Debug for DwEnd`

- <span id="dwend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEnd`

- <span id="dwend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEnd`

##### `impl Hash for DwEnd`

- <span id="dwend-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEnd`

- <span id="dwend-cmp"></span>`fn cmp(&self, other: &DwEnd) -> cmp::Ordering` — [`DwEnd`](#dwend)

##### `impl PartialEq for DwEnd`

- <span id="dwend-eq"></span>`fn eq(&self, other: &DwEnd) -> bool` — [`DwEnd`](#dwend)

##### `impl PartialOrd for DwEnd`

- <span id="dwend-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEnd) -> option::Option<cmp::Ordering>` — [`DwEnd`](#dwend)

##### `impl StructuralPartialEq for DwEnd`

##### `impl<T> ToString for DwEnd`

- <span id="dwend-to-string"></span>`fn to_string(&self) -> String`

### `DwAccess`

```rust
struct DwAccess(u8);
```

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

#### Implementations

- <span id="dwaccess-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAccess`

- <span id="dwaccess-clone"></span>`fn clone(&self) -> DwAccess` — [`DwAccess`](#dwaccess)

##### `impl Copy for DwAccess`

##### `impl Debug for DwAccess`

- <span id="dwaccess-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAccess`

- <span id="dwaccess-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAccess`

##### `impl Hash for DwAccess`

- <span id="dwaccess-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAccess`

- <span id="dwaccess-cmp"></span>`fn cmp(&self, other: &DwAccess) -> cmp::Ordering` — [`DwAccess`](#dwaccess)

##### `impl PartialEq for DwAccess`

- <span id="dwaccess-eq"></span>`fn eq(&self, other: &DwAccess) -> bool` — [`DwAccess`](#dwaccess)

##### `impl PartialOrd for DwAccess`

- <span id="dwaccess-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAccess) -> option::Option<cmp::Ordering>` — [`DwAccess`](#dwaccess)

##### `impl StructuralPartialEq for DwAccess`

##### `impl<T> ToString for DwAccess`

- <span id="dwaccess-to-string"></span>`fn to_string(&self) -> String`

### `DwVis`

```rust
struct DwVis(u8);
```

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

#### Implementations

- <span id="dwvis-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVis`

- <span id="dwvis-clone"></span>`fn clone(&self) -> DwVis` — [`DwVis`](#dwvis)

##### `impl Copy for DwVis`

##### `impl Debug for DwVis`

- <span id="dwvis-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVis`

- <span id="dwvis-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVis`

##### `impl Hash for DwVis`

- <span id="dwvis-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVis`

- <span id="dwvis-cmp"></span>`fn cmp(&self, other: &DwVis) -> cmp::Ordering` — [`DwVis`](#dwvis)

##### `impl PartialEq for DwVis`

- <span id="dwvis-eq"></span>`fn eq(&self, other: &DwVis) -> bool` — [`DwVis`](#dwvis)

##### `impl PartialOrd for DwVis`

- <span id="dwvis-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVis) -> option::Option<cmp::Ordering>` — [`DwVis`](#dwvis)

##### `impl StructuralPartialEq for DwVis`

##### `impl<T> ToString for DwVis`

- <span id="dwvis-to-string"></span>`fn to_string(&self) -> String`

### `DwVirtuality`

```rust
struct DwVirtuality(u8);
```

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

#### Implementations

- <span id="dwvirtuality-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVirtuality`

- <span id="dwvirtuality-clone"></span>`fn clone(&self) -> DwVirtuality` — [`DwVirtuality`](#dwvirtuality)

##### `impl Copy for DwVirtuality`

##### `impl Debug for DwVirtuality`

- <span id="dwvirtuality-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVirtuality`

- <span id="dwvirtuality-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVirtuality`

##### `impl Hash for DwVirtuality`

- <span id="dwvirtuality-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVirtuality`

- <span id="dwvirtuality-cmp"></span>`fn cmp(&self, other: &DwVirtuality) -> cmp::Ordering` — [`DwVirtuality`](#dwvirtuality)

##### `impl PartialEq for DwVirtuality`

- <span id="dwvirtuality-eq"></span>`fn eq(&self, other: &DwVirtuality) -> bool` — [`DwVirtuality`](#dwvirtuality)

##### `impl PartialOrd for DwVirtuality`

- <span id="dwvirtuality-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVirtuality) -> option::Option<cmp::Ordering>` — [`DwVirtuality`](#dwvirtuality)

##### `impl StructuralPartialEq for DwVirtuality`

##### `impl<T> ToString for DwVirtuality`

- <span id="dwvirtuality-to-string"></span>`fn to_string(&self) -> String`

### `DwLang`

```rust
struct DwLang(u16);
```

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

#### Implementations

- <span id="dwlang-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLang`

- <span id="dwlang-clone"></span>`fn clone(&self) -> DwLang` — [`DwLang`](#dwlang)

##### `impl Copy for DwLang`

##### `impl Debug for DwLang`

- <span id="dwlang-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLang`

- <span id="dwlang-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLang`

##### `impl Hash for DwLang`

- <span id="dwlang-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLang`

- <span id="dwlang-cmp"></span>`fn cmp(&self, other: &DwLang) -> cmp::Ordering` — [`DwLang`](#dwlang)

##### `impl PartialEq for DwLang`

- <span id="dwlang-eq"></span>`fn eq(&self, other: &DwLang) -> bool` — [`DwLang`](#dwlang)

##### `impl PartialOrd for DwLang`

- <span id="dwlang-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLang) -> option::Option<cmp::Ordering>` — [`DwLang`](#dwlang)

##### `impl StructuralPartialEq for DwLang`

##### `impl<T> ToString for DwLang`

- <span id="dwlang-to-string"></span>`fn to_string(&self) -> String`

### `DwAddr`

```rust
struct DwAddr(u64);
```

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

#### Implementations

- <span id="dwaddr-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAddr`

- <span id="dwaddr-clone"></span>`fn clone(&self) -> DwAddr` — [`DwAddr`](#dwaddr)

##### `impl Copy for DwAddr`

##### `impl Debug for DwAddr`

- <span id="dwaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAddr`

- <span id="dwaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAddr`

##### `impl Hash for DwAddr`

- <span id="dwaddr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAddr`

- <span id="dwaddr-cmp"></span>`fn cmp(&self, other: &DwAddr) -> cmp::Ordering` — [`DwAddr`](#dwaddr)

##### `impl PartialEq for DwAddr`

- <span id="dwaddr-eq"></span>`fn eq(&self, other: &DwAddr) -> bool` — [`DwAddr`](#dwaddr)

##### `impl PartialOrd for DwAddr`

- <span id="dwaddr-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAddr) -> option::Option<cmp::Ordering>` — [`DwAddr`](#dwaddr)

##### `impl StructuralPartialEq for DwAddr`

##### `impl<T> ToString for DwAddr`

- <span id="dwaddr-to-string"></span>`fn to_string(&self) -> String`

### `DwId`

```rust
struct DwId(u8);
```

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

#### Implementations

- <span id="dwid-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwId`

- <span id="dwid-clone"></span>`fn clone(&self) -> DwId` — [`DwId`](#dwid)

##### `impl Copy for DwId`

##### `impl Debug for DwId`

- <span id="dwid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwId`

- <span id="dwid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwId`

##### `impl Hash for DwId`

- <span id="dwid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwId`

- <span id="dwid-cmp"></span>`fn cmp(&self, other: &DwId) -> cmp::Ordering` — [`DwId`](#dwid)

##### `impl PartialEq for DwId`

- <span id="dwid-eq"></span>`fn eq(&self, other: &DwId) -> bool` — [`DwId`](#dwid)

##### `impl PartialOrd for DwId`

- <span id="dwid-partial-cmp"></span>`fn partial_cmp(&self, other: &DwId) -> option::Option<cmp::Ordering>` — [`DwId`](#dwid)

##### `impl StructuralPartialEq for DwId`

##### `impl<T> ToString for DwId`

- <span id="dwid-to-string"></span>`fn to_string(&self) -> String`

### `DwCc`

```rust
struct DwCc(u8);
```

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

#### Implementations

- <span id="dwcc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCc`

- <span id="dwcc-clone"></span>`fn clone(&self) -> DwCc` — [`DwCc`](#dwcc)

##### `impl Copy for DwCc`

##### `impl Debug for DwCc`

- <span id="dwcc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCc`

- <span id="dwcc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCc`

##### `impl Hash for DwCc`

- <span id="dwcc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCc`

- <span id="dwcc-cmp"></span>`fn cmp(&self, other: &DwCc) -> cmp::Ordering` — [`DwCc`](#dwcc)

##### `impl PartialEq for DwCc`

- <span id="dwcc-eq"></span>`fn eq(&self, other: &DwCc) -> bool` — [`DwCc`](#dwcc)

##### `impl PartialOrd for DwCc`

- <span id="dwcc-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCc) -> option::Option<cmp::Ordering>` — [`DwCc`](#dwcc)

##### `impl StructuralPartialEq for DwCc`

##### `impl<T> ToString for DwCc`

- <span id="dwcc-to-string"></span>`fn to_string(&self) -> String`

### `DwInl`

```rust
struct DwInl(u8);
```

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

#### Implementations

- <span id="dwinl-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwInl`

- <span id="dwinl-clone"></span>`fn clone(&self) -> DwInl` — [`DwInl`](#dwinl)

##### `impl Copy for DwInl`

##### `impl Debug for DwInl`

- <span id="dwinl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwInl`

- <span id="dwinl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwInl`

##### `impl Hash for DwInl`

- <span id="dwinl-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwInl`

- <span id="dwinl-cmp"></span>`fn cmp(&self, other: &DwInl) -> cmp::Ordering` — [`DwInl`](#dwinl)

##### `impl PartialEq for DwInl`

- <span id="dwinl-eq"></span>`fn eq(&self, other: &DwInl) -> bool` — [`DwInl`](#dwinl)

##### `impl PartialOrd for DwInl`

- <span id="dwinl-partial-cmp"></span>`fn partial_cmp(&self, other: &DwInl) -> option::Option<cmp::Ordering>` — [`DwInl`](#dwinl)

##### `impl StructuralPartialEq for DwInl`

##### `impl<T> ToString for DwInl`

- <span id="dwinl-to-string"></span>`fn to_string(&self) -> String`

### `DwOrd`

```rust
struct DwOrd(u8);
```

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

#### Implementations

- <span id="dword-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOrd`

- <span id="dword-clone"></span>`fn clone(&self) -> DwOrd` — [`DwOrd`](#dword)

##### `impl Copy for DwOrd`

##### `impl Debug for DwOrd`

- <span id="dword-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOrd`

- <span id="dword-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOrd`

##### `impl Hash for DwOrd`

- <span id="dword-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOrd`

- <span id="dword-cmp"></span>`fn cmp(&self, other: &DwOrd) -> cmp::Ordering` — [`DwOrd`](#dword)

##### `impl PartialEq for DwOrd`

- <span id="dword-eq"></span>`fn eq(&self, other: &DwOrd) -> bool` — [`DwOrd`](#dword)

##### `impl PartialOrd for DwOrd`

- <span id="dword-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOrd) -> option::Option<cmp::Ordering>` — [`DwOrd`](#dword)

##### `impl StructuralPartialEq for DwOrd`

##### `impl<T> ToString for DwOrd`

- <span id="dword-to-string"></span>`fn to_string(&self) -> String`

### `DwDsc`

```rust
struct DwDsc(u8);
```

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

#### Implementations

- <span id="dwdsc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDsc`

- <span id="dwdsc-clone"></span>`fn clone(&self) -> DwDsc` — [`DwDsc`](#dwdsc)

##### `impl Copy for DwDsc`

##### `impl Debug for DwDsc`

- <span id="dwdsc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDsc`

- <span id="dwdsc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDsc`

##### `impl Hash for DwDsc`

- <span id="dwdsc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDsc`

- <span id="dwdsc-cmp"></span>`fn cmp(&self, other: &DwDsc) -> cmp::Ordering` — [`DwDsc`](#dwdsc)

##### `impl PartialEq for DwDsc`

- <span id="dwdsc-eq"></span>`fn eq(&self, other: &DwDsc) -> bool` — [`DwDsc`](#dwdsc)

##### `impl PartialOrd for DwDsc`

- <span id="dwdsc-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDsc) -> option::Option<cmp::Ordering>` — [`DwDsc`](#dwdsc)

##### `impl StructuralPartialEq for DwDsc`

##### `impl<T> ToString for DwDsc`

- <span id="dwdsc-to-string"></span>`fn to_string(&self) -> String`

### `DwIdx`

```rust
struct DwIdx(u16);
```

Name index attribute encodings.

See Section 7.19, Table 7.23.

#### Implementations

- <span id="dwidx-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwIdx`

- <span id="dwidx-clone"></span>`fn clone(&self) -> DwIdx` — [`DwIdx`](#dwidx)

##### `impl Copy for DwIdx`

##### `impl Debug for DwIdx`

- <span id="dwidx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwIdx`

- <span id="dwidx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwIdx`

##### `impl Hash for DwIdx`

- <span id="dwidx-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwIdx`

- <span id="dwidx-cmp"></span>`fn cmp(&self, other: &DwIdx) -> cmp::Ordering` — [`DwIdx`](#dwidx)

##### `impl PartialEq for DwIdx`

- <span id="dwidx-eq"></span>`fn eq(&self, other: &DwIdx) -> bool` — [`DwIdx`](#dwidx)

##### `impl PartialOrd for DwIdx`

- <span id="dwidx-partial-cmp"></span>`fn partial_cmp(&self, other: &DwIdx) -> option::Option<cmp::Ordering>` — [`DwIdx`](#dwidx)

##### `impl StructuralPartialEq for DwIdx`

##### `impl<T> ToString for DwIdx`

- <span id="dwidx-to-string"></span>`fn to_string(&self) -> String`

### `DwDefaulted`

```rust
struct DwDefaulted(u8);
```

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

#### Implementations

- <span id="dwdefaulted-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDefaulted`

- <span id="dwdefaulted-clone"></span>`fn clone(&self) -> DwDefaulted` — [`DwDefaulted`](#dwdefaulted)

##### `impl Copy for DwDefaulted`

##### `impl Debug for DwDefaulted`

- <span id="dwdefaulted-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDefaulted`

- <span id="dwdefaulted-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDefaulted`

##### `impl Hash for DwDefaulted`

- <span id="dwdefaulted-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDefaulted`

- <span id="dwdefaulted-cmp"></span>`fn cmp(&self, other: &DwDefaulted) -> cmp::Ordering` — [`DwDefaulted`](#dwdefaulted)

##### `impl PartialEq for DwDefaulted`

- <span id="dwdefaulted-eq"></span>`fn eq(&self, other: &DwDefaulted) -> bool` — [`DwDefaulted`](#dwdefaulted)

##### `impl PartialOrd for DwDefaulted`

- <span id="dwdefaulted-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDefaulted) -> option::Option<cmp::Ordering>` — [`DwDefaulted`](#dwdefaulted)

##### `impl StructuralPartialEq for DwDefaulted`

##### `impl<T> ToString for DwDefaulted`

- <span id="dwdefaulted-to-string"></span>`fn to_string(&self) -> String`

### `DwLns`

```rust
struct DwLns(u8);
```

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

#### Implementations

- <span id="dwlns-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLns`

- <span id="dwlns-clone"></span>`fn clone(&self) -> DwLns` — [`DwLns`](#dwlns)

##### `impl Copy for DwLns`

##### `impl Debug for DwLns`

- <span id="dwlns-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLns`

- <span id="dwlns-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLns`

##### `impl Hash for DwLns`

- <span id="dwlns-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLns`

- <span id="dwlns-cmp"></span>`fn cmp(&self, other: &DwLns) -> cmp::Ordering` — [`DwLns`](#dwlns)

##### `impl PartialEq for DwLns`

- <span id="dwlns-eq"></span>`fn eq(&self, other: &DwLns) -> bool` — [`DwLns`](#dwlns)

##### `impl PartialOrd for DwLns`

- <span id="dwlns-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLns) -> option::Option<cmp::Ordering>` — [`DwLns`](#dwlns)

##### `impl StructuralPartialEq for DwLns`

##### `impl<T> ToString for DwLns`

- <span id="dwlns-to-string"></span>`fn to_string(&self) -> String`

### `DwLne`

```rust
struct DwLne(u8);
```

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

#### Implementations

- <span id="dwlne-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLne`

- <span id="dwlne-clone"></span>`fn clone(&self) -> DwLne` — [`DwLne`](#dwlne)

##### `impl Copy for DwLne`

##### `impl Debug for DwLne`

- <span id="dwlne-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLne`

- <span id="dwlne-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLne`

##### `impl Hash for DwLne`

- <span id="dwlne-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLne`

- <span id="dwlne-cmp"></span>`fn cmp(&self, other: &DwLne) -> cmp::Ordering` — [`DwLne`](#dwlne)

##### `impl PartialEq for DwLne`

- <span id="dwlne-eq"></span>`fn eq(&self, other: &DwLne) -> bool` — [`DwLne`](#dwlne)

##### `impl PartialOrd for DwLne`

- <span id="dwlne-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLne) -> option::Option<cmp::Ordering>` — [`DwLne`](#dwlne)

##### `impl StructuralPartialEq for DwLne`

##### `impl<T> ToString for DwLne`

- <span id="dwlne-to-string"></span>`fn to_string(&self) -> String`

### `DwLnct`

```rust
struct DwLnct(u16);
```

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

#### Implementations

- <span id="dwlnct-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLnct`

- <span id="dwlnct-clone"></span>`fn clone(&self) -> DwLnct` — [`DwLnct`](#dwlnct)

##### `impl Copy for DwLnct`

##### `impl Debug for DwLnct`

- <span id="dwlnct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLnct`

- <span id="dwlnct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLnct`

##### `impl Hash for DwLnct`

- <span id="dwlnct-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLnct`

- <span id="dwlnct-cmp"></span>`fn cmp(&self, other: &DwLnct) -> cmp::Ordering` — [`DwLnct`](#dwlnct)

##### `impl PartialEq for DwLnct`

- <span id="dwlnct-eq"></span>`fn eq(&self, other: &DwLnct) -> bool` — [`DwLnct`](#dwlnct)

##### `impl PartialOrd for DwLnct`

- <span id="dwlnct-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLnct) -> option::Option<cmp::Ordering>` — [`DwLnct`](#dwlnct)

##### `impl StructuralPartialEq for DwLnct`

##### `impl<T> ToString for DwLnct`

- <span id="dwlnct-to-string"></span>`fn to_string(&self) -> String`

### `DwMacinfo`

```rust
struct DwMacinfo(u8);
```

Type codes for macro definitions in the `.debug_macinfo` section.

See Section 7.22, Figure 39 for DWARF 4.

#### Implementations

- <span id="dwmacinfo-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacinfo`

- <span id="dwmacinfo-clone"></span>`fn clone(&self) -> DwMacinfo` — [`DwMacinfo`](#dwmacinfo)

##### `impl Copy for DwMacinfo`

##### `impl Debug for DwMacinfo`

- <span id="dwmacinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacinfo`

- <span id="dwmacinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacinfo`

##### `impl Hash for DwMacinfo`

- <span id="dwmacinfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacinfo`

- <span id="dwmacinfo-cmp"></span>`fn cmp(&self, other: &DwMacinfo) -> cmp::Ordering` — [`DwMacinfo`](#dwmacinfo)

##### `impl PartialEq for DwMacinfo`

- <span id="dwmacinfo-eq"></span>`fn eq(&self, other: &DwMacinfo) -> bool` — [`DwMacinfo`](#dwmacinfo)

##### `impl PartialOrd for DwMacinfo`

- <span id="dwmacinfo-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacinfo) -> option::Option<cmp::Ordering>` — [`DwMacinfo`](#dwmacinfo)

##### `impl StructuralPartialEq for DwMacinfo`

##### `impl<T> ToString for DwMacinfo`

- <span id="dwmacinfo-to-string"></span>`fn to_string(&self) -> String`

### `DwMacro`

```rust
struct DwMacro(u8);
```

The encodings for macro information entry types.

See Section 7.23, Table 7.28 for DWARF 5.

#### Implementations

- <span id="dwmacro-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacro`

- <span id="dwmacro-clone"></span>`fn clone(&self) -> DwMacro` — [`DwMacro`](#dwmacro)

##### `impl Copy for DwMacro`

##### `impl Debug for DwMacro`

- <span id="dwmacro-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacro`

- <span id="dwmacro-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacro`

##### `impl Hash for DwMacro`

- <span id="dwmacro-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacro`

- <span id="dwmacro-cmp"></span>`fn cmp(&self, other: &DwMacro) -> cmp::Ordering` — [`DwMacro`](#dwmacro)

##### `impl PartialEq for DwMacro`

- <span id="dwmacro-eq"></span>`fn eq(&self, other: &DwMacro) -> bool` — [`DwMacro`](#dwmacro)

##### `impl PartialOrd for DwMacro`

- <span id="dwmacro-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacro) -> option::Option<cmp::Ordering>` — [`DwMacro`](#dwmacro)

##### `impl StructuralPartialEq for DwMacro`

##### `impl<T> ToString for DwMacro`

- <span id="dwmacro-to-string"></span>`fn to_string(&self) -> String`

### `DwRle`

```rust
struct DwRle(u8);
```

Range list entry encoding values.

See Section 7.25, Table 7.30.

#### Implementations

- <span id="dwrle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwRle`

- <span id="dwrle-clone"></span>`fn clone(&self) -> DwRle` — [`DwRle`](#dwrle)

##### `impl Copy for DwRle`

##### `impl Debug for DwRle`

- <span id="dwrle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwRle`

- <span id="dwrle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwRle`

##### `impl Hash for DwRle`

- <span id="dwrle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwRle`

- <span id="dwrle-cmp"></span>`fn cmp(&self, other: &DwRle) -> cmp::Ordering` — [`DwRle`](#dwrle)

##### `impl PartialEq for DwRle`

- <span id="dwrle-eq"></span>`fn eq(&self, other: &DwRle) -> bool` — [`DwRle`](#dwrle)

##### `impl PartialOrd for DwRle`

- <span id="dwrle-partial-cmp"></span>`fn partial_cmp(&self, other: &DwRle) -> option::Option<cmp::Ordering>` — [`DwRle`](#dwrle)

##### `impl StructuralPartialEq for DwRle`

##### `impl<T> ToString for DwRle`

- <span id="dwrle-to-string"></span>`fn to_string(&self) -> String`

### `DwOp`

```rust
struct DwOp(u8);
```

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

#### Implementations

- <span id="dwop-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOp`

- <span id="dwop-clone"></span>`fn clone(&self) -> DwOp` — [`DwOp`](#dwop)

##### `impl Copy for DwOp`

##### `impl Debug for DwOp`

- <span id="dwop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOp`

- <span id="dwop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOp`

##### `impl Hash for DwOp`

- <span id="dwop-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOp`

- <span id="dwop-cmp"></span>`fn cmp(&self, other: &DwOp) -> cmp::Ordering` — [`DwOp`](#dwop)

##### `impl PartialEq for DwOp`

- <span id="dwop-eq"></span>`fn eq(&self, other: &DwOp) -> bool` — [`DwOp`](#dwop)

##### `impl PartialOrd for DwOp`

- <span id="dwop-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOp) -> option::Option<cmp::Ordering>` — [`DwOp`](#dwop)

##### `impl StructuralPartialEq for DwOp`

##### `impl<T> ToString for DwOp`

- <span id="dwop-to-string"></span>`fn to_string(&self) -> String`

### `DwEhPe`

```rust
struct DwEhPe(u8);
```

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

#### Implementations

- <span id="dwehpe-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl BitOr for DwEhPe`

- <span id="dwehpe-output"></span>`type Output = DwEhPe`

- <span id="dwehpe-bitor"></span>`fn bitor(self, rhs: DwEhPe) -> DwEhPe` — [`DwEhPe`](#dwehpe)

##### `impl Clone for DwEhPe`

- <span id="dwehpe-clone"></span>`fn clone(&self) -> DwEhPe` — [`DwEhPe`](#dwehpe)

##### `impl Copy for DwEhPe`

##### `impl Debug for DwEhPe`

- <span id="dwehpe-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEhPe`

- <span id="dwehpe-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEhPe`

##### `impl Hash for DwEhPe`

- <span id="dwehpe-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEhPe`

- <span id="dwehpe-cmp"></span>`fn cmp(&self, other: &DwEhPe) -> cmp::Ordering` — [`DwEhPe`](#dwehpe)

##### `impl PartialEq for DwEhPe`

- <span id="dwehpe-eq"></span>`fn eq(&self, other: &DwEhPe) -> bool` — [`DwEhPe`](#dwehpe)

##### `impl PartialOrd for DwEhPe`

- <span id="dwehpe-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEhPe) -> option::Option<cmp::Ordering>` — [`DwEhPe`](#dwehpe)

##### `impl StructuralPartialEq for DwEhPe`

##### `impl<T> ToString for DwEhPe`

- <span id="dwehpe-to-string"></span>`fn to_string(&self) -> String`

### `LittleEndian`

```rust
struct LittleEndian;
```

Little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Endianity for LittleEndian`

- <span id="littleendian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Endianity for BigEndian`

- <span id="bigendian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- <span id="bigendian-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl StructuralPartialEq for BigEndian`

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

An offset into the current compilation or type unit.

#### Implementations

- <span id="cratereadunitoffset-to-debug-info-offset"></span>`fn to_debug_info_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](read/index.md), [`DebugInfoOffset`](#debuginfooffset)

- <span id="cratereadunitoffset-to-debug-types-offset"></span>`fn to_debug_types_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](read/index.md), [`DebugTypesOffset`](#debugtypesoffset)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitOffset<T>`

- <span id="unitoffset-clone"></span>`fn clone(&self) -> UnitOffset<T>` — [`UnitOffset`](#unitoffset)

##### `impl<T: marker::Copy> Copy for UnitOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitOffset<T>`

- <span id="unitoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitOffset<T>`

- <span id="unitoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitOffset<T>`

- <span id="unitoffset-cmp"></span>`fn cmp(&self, other: &UnitOffset<T>) -> cmp::Ordering` — [`UnitOffset`](#unitoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitOffset<T>`

- <span id="unitoffset-eq"></span>`fn eq(&self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](#unitoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitOffset<T>`

- <span id="unitoffset-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitOffset`](#unitoffset)

##### `impl<T> StructuralPartialEq for UnitOffset<T>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Clone for StoreOnHeap`

- <span id="storeonheap-clone"></span>`fn clone(&self) -> StoreOnHeap` — [`StoreOnHeap`](#storeonheap)

##### `impl Copy for StoreOnHeap`

##### `impl Debug for StoreOnHeap`

- <span id="storeonheap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StoreOnHeap`

##### `impl<R: Reader> EvaluationStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-stack"></span>`type Stack = Vec<Value>`

- <span id="cratereadstoreonheap-expressionstack"></span>`type ExpressionStack = Vec<(R, R)>`

- <span id="cratereadstoreonheap-result"></span>`type Result = Vec<Piece<R>>`

##### `impl PartialEq for StoreOnHeap`

- <span id="storeonheap-eq"></span>`fn eq(&self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](#storeonheap)

##### `impl StructuralPartialEq for StoreOnHeap`

##### `impl<T: ReaderOffset> UnwindContextStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-rules"></span>`type Rules = [(Register, RegisterRule<T>); 192]`

- <span id="cratereadstoreonheap-stack"></span>`type Stack = Box<[UnwindTableRow<T>; 4]>`

## Enums

### `Format`

```rust
enum Format {
    Dwarf64,
    Dwarf32,
}
```

Whether the format of a compilation unit is 32- or 64-bit.

#### Variants

- **`Dwarf64`**

  64-bit DWARF

- **`Dwarf32`**

  32-bit DWARF

#### Implementations

- <span id="format-initial-length-size"></span>`fn initial_length_size(self) -> u8`

- <span id="format-word-size"></span>`fn word_size(self) -> u8`

#### Trait Implementations

##### `impl Clone for Format`

- <span id="format-clone"></span>`fn clone(&self) -> Format` — [`Format`](#format)

##### `impl Copy for Format`

##### `impl Debug for Format`

- <span id="format-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Format`

##### `impl Hash for Format`

- <span id="format-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Format`

- <span id="format-eq"></span>`fn eq(&self, other: &Format) -> bool` — [`Format`](#format)

##### `impl StructuralPartialEq for Format`

### `Vendor`

```rust
enum Vendor {
    Default,
    AArch64,
}
```

Which vendor extensions to support.

#### Variants

- **`Default`**

  A default set of extensions, including some common GNU extensions.

- **`AArch64`**

  AAarch64 extensions.

#### Trait Implementations

##### `impl Clone for Vendor`

- <span id="vendor-clone"></span>`fn clone(&self) -> Vendor` — [`Vendor`](#vendor)

##### `impl Copy for Vendor`

##### `impl Debug for Vendor`

- <span id="vendor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Vendor`

##### `impl PartialEq for Vendor`

- <span id="vendor-eq"></span>`fn eq(&self, other: &Vendor) -> bool` — [`Vendor`](#vendor)

##### `impl StructuralPartialEq for Vendor`

### `UnitSectionOffset<T>`

```rust
enum UnitSectionOffset<T> {
    DebugInfoOffset(DebugInfoOffset<T>),
    DebugTypesOffset(DebugTypesOffset<T>),
}
```

An offset into the `.debug_info` or `.debug_types` sections.

#### Variants

- **`DebugInfoOffset`**

  An offset into the `.debug_info` section.

- **`DebugTypesOffset`**

  An offset into the `.debug_types` section.

#### Implementations

- <span id="unitsectionoffset-as-debug-info-offset"></span>`fn as_debug_info_offset(&self) -> Option<DebugInfoOffset<T>>` — [`DebugInfoOffset`](#debuginfooffset)

- <span id="unitsectionoffset-as-debug-types-offset"></span>`fn as_debug_types_offset(&self) -> Option<DebugTypesOffset<T>>` — [`DebugTypesOffset`](#debugtypesoffset)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitSectionOffset<T>`

- <span id="unitsectionoffset-clone"></span>`fn clone(&self) -> UnitSectionOffset<T>` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: marker::Copy> Copy for UnitSectionOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitSectionOffset<T>`

- <span id="unitsectionoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitSectionOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitSectionOffset<T>`

- <span id="unitsectionoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitSectionOffset<T>`

- <span id="unitsectionoffset-cmp"></span>`fn cmp(&self, other: &UnitSectionOffset<T>) -> cmp::Ordering` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitSectionOffset<T>`

- <span id="unitsectionoffset-eq"></span>`fn eq(&self, other: &UnitSectionOffset<T>) -> bool` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitSectionOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T> StructuralPartialEq for UnitSectionOffset<T>`

### `SectionId`

```rust
enum SectionId {
    DebugAbbrev,
    DebugAddr,
    DebugAranges,
    DebugCuIndex,
    DebugFrame,
    EhFrame,
    EhFrameHdr,
    DebugInfo,
    DebugLine,
    DebugLineStr,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugPubNames,
    DebugPubTypes,
    DebugRanges,
    DebugRngLists,
    DebugStr,
    DebugStrOffsets,
    DebugTuIndex,
    DebugTypes,
}
```

An identifier for a DWARF section.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev` section.

- **`DebugAddr`**

  The `.debug_addr` section.

- **`DebugAranges`**

  The `.debug_aranges` section.

- **`DebugCuIndex`**

  The `.debug_cu_index` section.

- **`DebugFrame`**

  The `.debug_frame` section.

- **`EhFrame`**

  The `.eh_frame` section.

- **`EhFrameHdr`**

  The `.eh_frame_hdr` section.

- **`DebugInfo`**

  The `.debug_info` section.

- **`DebugLine`**

  The `.debug_line` section.

- **`DebugLineStr`**

  The `.debug_line_str` section.

- **`DebugLoc`**

  The `.debug_loc` section.

- **`DebugLocLists`**

  The `.debug_loclists` section.

- **`DebugMacinfo`**

  The `.debug_macinfo` section.

- **`DebugMacro`**

  The `.debug_macro` section.

- **`DebugPubNames`**

  The `.debug_pubnames` section.

- **`DebugPubTypes`**

  The `.debug_pubtypes` section.

- **`DebugRanges`**

  The `.debug_ranges` section.

- **`DebugRngLists`**

  The `.debug_rnglists` section.

- **`DebugStr`**

  The `.debug_str` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets` section.

- **`DebugTuIndex`**

  The `.debug_tu_index` section.

- **`DebugTypes`**

  The `.debug_types` section.

#### Implementations

- <span id="sectionid-name"></span>`fn name(self) -> &'static str`

- <span id="sectionid-dwo-name"></span>`fn dwo_name(self) -> Option<&'static str>`

- <span id="sectionid-xcoff-name"></span>`fn xcoff_name(self) -> Option<&'static str>`

- <span id="sectionid-is-string"></span>`fn is_string(self) -> bool`

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](#sectionid)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl Hash for SectionId`

- <span id="sectionid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SectionId`

- <span id="sectionid-cmp"></span>`fn cmp(&self, other: &SectionId) -> cmp::Ordering` — [`SectionId`](#sectionid)

##### `impl PartialEq for SectionId`

- <span id="sectionid-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](#sectionid)

##### `impl PartialOrd for SectionId`

- <span id="sectionid-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionId) -> option::Option<cmp::Ordering>` — [`SectionId`](#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `DwarfFileType`

```rust
enum DwarfFileType {
    Main,
    Dwo,
}
```

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

#### Variants

- **`Main`**

  A normal executable or object file.

- **`Dwo`**

  A .dwo split DWARF file.

#### Trait Implementations

##### `impl Clone for DwarfFileType`

- <span id="dwarffiletype-clone"></span>`fn clone(&self) -> DwarfFileType` — [`DwarfFileType`](#dwarffiletype)

##### `impl Copy for DwarfFileType`

##### `impl Debug for DwarfFileType`

- <span id="dwarffiletype-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DwarfFileType`

- <span id="dwarffiletype-default"></span>`fn default() -> Self`

##### `impl Eq for DwarfFileType`

##### `impl PartialEq for DwarfFileType`

- <span id="dwarffiletype-eq"></span>`fn eq(&self, other: &DwarfFileType) -> bool` — [`DwarfFileType`](#dwarffiletype)

##### `impl StructuralPartialEq for DwarfFileType`

### `RunTimeEndian`

```rust
enum RunTimeEndian {
    Little,
    Big,
}
```

Byte order that is selectable at runtime.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for RunTimeEndian`

- <span id="runtimeendian-clone"></span>`fn clone(&self) -> RunTimeEndian` — [`RunTimeEndian`](#runtimeendian)

##### `impl Copy for RunTimeEndian`

##### `impl Debug for RunTimeEndian`

- <span id="runtimeendian-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RunTimeEndian`

- <span id="runtimeendian-default"></span>`fn default() -> RunTimeEndian` — [`RunTimeEndian`](#runtimeendian)

##### `impl Endianity for RunTimeEndian`

- <span id="runtimeendian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for RunTimeEndian`

##### `impl Hash for RunTimeEndian`

- <span id="runtimeendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RunTimeEndian`

- <span id="runtimeendian-eq"></span>`fn eq(&self, other: &RunTimeEndian) -> bool` — [`RunTimeEndian`](#runtimeendian)

##### `impl StructuralPartialEq for RunTimeEndian`

### `Error`

```rust
enum Error {
    Io,
    PcRelativePointerButSectionBaseIsUndefined,
    TextRelativePointerButTextBaseIsUndefined,
    DataRelativePointerButDataBaseIsUndefined,
    FuncRelativePointerInBadContext,
    CannotParseOmitPointerEncoding,
    BadUnsignedLeb128,
    BadSignedLeb128,
    AbbreviationTagZero,
    AttributeFormZero,
    BadHasChildren,
    BadLength,
    UnknownForm(constants::DwForm),
    ExpectedZero,
    DuplicateAbbreviationCode,
    DuplicateArange,
    UnknownReservedLength,
    UnknownVersion(u64),
    UnknownAbbreviation(u64),
    UnexpectedEof(ReaderOffsetId),
    UnexpectedNull,
    UnknownStandardOpcode(constants::DwLns),
    UnknownExtendedOpcode(constants::DwLne),
    UnknownLocListsEntry(constants::DwLle),
    UnknownRangeListsEntry(constants::DwRle),
    UnsupportedAddressSize(u8),
    UnsupportedOffsetSize(u8),
    UnsupportedFieldSize(u8),
    MinimumInstructionLengthZero,
    MaximumOperationsPerInstructionZero,
    LineRangeZero,
    OpcodeBaseZero,
    BadUtf8,
    NotCieId,
    NotCiePointer,
    NotFdePointer,
    BadBranchTarget(u64),
    InvalidPushObjectAddress,
    NotEnoughStackItems,
    TooManyIterations,
    InvalidExpression(constants::DwOp),
    UnsupportedEvaluation,
    InvalidPiece,
    InvalidExpressionTerminator(u64),
    DivisionByZero,
    TypeMismatch,
    IntegralTypeRequired,
    UnsupportedTypeOperation,
    InvalidShiftExpression,
    InvalidDerefSize(u8),
    UnknownCallFrameInstruction(constants::DwCfa),
    InvalidAddressRange,
    AddressOverflow,
    CfiInstructionInInvalidContext,
    PopWithEmptyStack,
    NoUnwindInfoForAddress,
    UnsupportedOffset,
    UnknownPointerEncoding(constants::DwEhPe),
    NoEntryAtGivenOffset,
    OffsetOutOfBounds,
    UnknownAugmentation,
    UnsupportedPointerEncoding,
    UnsupportedRegister(u64),
    TooManyRegisterRules,
    StackFull,
    VariableLengthSearchTable,
    UnsupportedUnitType,
    UnsupportedAddressIndex,
    UnsupportedSegmentSize,
    MissingUnitDie,
    UnsupportedAttributeForm,
    MissingFileEntryFormatPath,
    ExpectedStringAttributeValue,
    InvalidImplicitConst,
    InvalidIndexSectionCount,
    InvalidIndexSlotCount,
    InvalidIndexRow,
    UnknownIndexSection(constants::DwSect),
    UnknownIndexSectionV2(constants::DwSectV2),
    InvalidMacinfoType(constants::DwMacinfo),
    InvalidMacroType(constants::DwMacro),
    UnsupportedOpcodeOperandsTable,
}
```

An error that occurred when parsing.

#### Variants

- **`Io`**

  An I/O error occurred while reading.

- **`PcRelativePointerButSectionBaseIsUndefined`**

  Found a PC relative pointer, but the section base is undefined.

- **`TextRelativePointerButTextBaseIsUndefined`**

  Found a `.text` relative pointer, but the `.text` base is undefined.

- **`DataRelativePointerButDataBaseIsUndefined`**

  Found a data relative pointer, but the data base is undefined.

- **`FuncRelativePointerInBadContext`**

  Found a function relative pointer in a context that does not have a
  function base.

- **`CannotParseOmitPointerEncoding`**

  Cannot parse a pointer with a `DW_EH_PE_omit` encoding.

- **`BadUnsignedLeb128`**

  An error parsing an unsigned LEB128 value.

- **`BadSignedLeb128`**

  An error parsing a signed LEB128 value.

- **`AbbreviationTagZero`**

  An abbreviation declared that its tag is zero, but zero is reserved for
  null records.

- **`AttributeFormZero`**

  An attribute specification declared that its form is zero, but zero is
  reserved for null records.

- **`BadHasChildren`**

  The abbreviation's has-children byte was not one of
  `DW_CHILDREN_{yes,no}`.

- **`BadLength`**

  The specified length is impossible.

- **`UnknownForm`**

  Found an unknown `DW_FORM_*` type.

- **`ExpectedZero`**

  Expected a zero, found something else.

- **`DuplicateAbbreviationCode`**

  Found an abbreviation code that has already been used.

- **`DuplicateArange`**

  Found a duplicate arange.

- **`UnknownReservedLength`**

  Found an unknown reserved length value.

- **`UnknownVersion`**

  Found an unknown DWARF version.

- **`UnknownAbbreviation`**

  Found a record with an unknown abbreviation code.

- **`UnexpectedEof`**

  Hit the end of input before it was expected.

- **`UnexpectedNull`**

  Read a null entry before it was expected.

- **`UnknownStandardOpcode`**

  Found an unknown standard opcode.

- **`UnknownExtendedOpcode`**

  Found an unknown extended opcode.

- **`UnknownLocListsEntry`**

  Found an unknown location-lists format.

- **`UnknownRangeListsEntry`**

  Found an unknown range-lists format.

- **`UnsupportedAddressSize`**

  The specified address size is not supported.

- **`UnsupportedOffsetSize`**

  The specified offset size is not supported.

- **`UnsupportedFieldSize`**

  The specified field size is not supported.

- **`MinimumInstructionLengthZero`**

  The minimum instruction length must not be zero.

- **`MaximumOperationsPerInstructionZero`**

  The maximum operations per instruction must not be zero.

- **`LineRangeZero`**

  The line range must not be zero.

- **`OpcodeBaseZero`**

  The opcode base must not be zero.

- **`BadUtf8`**

  Found an invalid UTF-8 string.

- **`NotCieId`**

  Expected to find the CIE ID, but found something else.

- **`NotCiePointer`**

  Expected to find a pointer to a CIE, but found the CIE ID instead.

- **`NotFdePointer`**

  Expected to find a pointer to an FDE, but found a CIE instead.

- **`BadBranchTarget`**

  Invalid branch target for a DW_OP_bra or DW_OP_skip.

- **`InvalidPushObjectAddress`**

  DW_OP_push_object_address used but no address passed in.

- **`NotEnoughStackItems`**

  Not enough items on the stack when evaluating an expression.

- **`TooManyIterations`**

  Too many iterations to compute the expression.

- **`InvalidExpression`**

  An unrecognized operation was found while parsing a DWARF
  expression.

- **`UnsupportedEvaluation`**

  An unsupported operation was found while evaluating a DWARF expression.

- **`InvalidPiece`**

  The expression had a piece followed by an expression
  terminator without a piece.

- **`InvalidExpressionTerminator`**

  An expression-terminating operation was followed by something
  other than the end of the expression or a piece operation.

- **`DivisionByZero`**

  Division or modulus by zero when evaluating an expression.

- **`TypeMismatch`**

  An expression operation used mismatching types.

- **`IntegralTypeRequired`**

  An expression operation required an integral type but saw a
  floating point type.

- **`UnsupportedTypeOperation`**

  An expression operation used types that are not supported.

- **`InvalidShiftExpression`**

  The shift value in an expression must be a non-negative integer.

- **`InvalidDerefSize`**

  The size of a deref expression must not be larger than the size of an address.

- **`UnknownCallFrameInstruction`**

  An unknown DW_CFA_* instruction.

- **`InvalidAddressRange`**

  The end of an address range was before the beginning.

- **`AddressOverflow`**

  An address calculation overflowed.
  
  This is returned in cases where the address is expected to be
  larger than a previous address, but the calculation overflowed.

- **`CfiInstructionInInvalidContext`**

  Encountered a call frame instruction in a context in which it is not
  valid.

- **`PopWithEmptyStack`**

  When evaluating call frame instructions, found a `DW_CFA_restore_state`
  stack pop instruction, but the stack was empty, and had nothing to pop.

- **`NoUnwindInfoForAddress`**

  Do not have unwind info for the given address.

- **`UnsupportedOffset`**

  An offset value was larger than the maximum supported value.

- **`UnknownPointerEncoding`**

  The given pointer encoding is either unknown or invalid.

- **`NoEntryAtGivenOffset`**

  Did not find an entry at the given offset.

- **`OffsetOutOfBounds`**

  The given offset is out of bounds.

- **`UnknownAugmentation`**

  Found an unknown CFI augmentation.

- **`UnsupportedPointerEncoding`**

  We do not support the given pointer encoding yet.

- **`UnsupportedRegister`**

  Registers larger than `u16` are not supported.

- **`TooManyRegisterRules`**

  The CFI program defined more register rules than we have storage for.

- **`StackFull`**

  Attempted to push onto the CFI or evaluation stack, but it was already
  at full capacity.

- **`VariableLengthSearchTable`**

  The `.eh_frame_hdr` binary search table claims to be variable-length encoded,
  which makes binary search impossible.

- **`UnsupportedUnitType`**

  The `DW_UT_*` value for this unit is not supported yet.

- **`UnsupportedAddressIndex`**

  Ranges using AddressIndex are not supported yet.

- **`UnsupportedSegmentSize`**

  Nonzero segment selector sizes aren't supported yet.

- **`MissingUnitDie`**

  A compilation unit or type unit is missing its top level DIE.

- **`UnsupportedAttributeForm`**

  A DIE attribute used an unsupported form.

- **`MissingFileEntryFormatPath`**

  Missing DW_LNCT_path in file entry format.

- **`ExpectedStringAttributeValue`**

  Expected an attribute value to be a string form.

- **`InvalidImplicitConst`**

  `DW_FORM_implicit_const` used in an invalid context.

- **`InvalidIndexSectionCount`**

  Invalid section count in `.dwp` index.

- **`InvalidIndexSlotCount`**

  Invalid slot count in `.dwp` index.

- **`InvalidIndexRow`**

  Invalid hash row in `.dwp` index.

- **`UnknownIndexSection`**

  Unknown section type in `.dwp` index.

- **`UnknownIndexSectionV2`**

  Unknown section type in version 2 `.dwp` index.

- **`InvalidMacinfoType`**

  Invalid macinfo type in `.debug_macinfo`.

- **`InvalidMacroType`**

  Invalid macro type in `.debug_macro`.

- **`UnsupportedOpcodeOperandsTable`**

  The optional `opcode_operands_table` in `.debug_macro` is currently not supported.

#### Implementations

- <span id="error-description"></span>`fn description(&self) -> &str`

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `Endianity`

```rust
trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq { ... }
```

A trait describing the endianity of some buffer.

#### Required Methods

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(self, buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(self, buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_uint(&mut self, buf: &[u8]) -> u64`

  Read an unsigned n-bytes integer u64.

- `fn read_i16(self, buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i32(self, buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i64(self, buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_f32(self, buf: &[u8]) -> f32`

  Reads a 32 bit floating point number from `buf`.

- `fn read_f64(self, buf: &[u8]) -> f64`

  Reads a 32 bit floating point number from `buf`.

- `fn write_u16(self, buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(self, buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(self, buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

#### Implementors

- [`BigEndian`](#bigendian)
- [`LittleEndian`](#littleendian)
- [`RunTimeEndian`](#runtimeendian)

### `Section<R>`

```rust
trait Section<R>: From<R> { ... }
```

A convenience trait for loading DWARF sections from object files.  To be
used like:

```rust
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

#### Required Methods

- `fn id() -> SectionId`

  Returns the section id for this type.

- `fn reader(&self) -> &R`

  Returns the `Reader` for this section.

#### Provided Methods

- `fn section_name() -> &'static str`

  Returns the ELF section name for this type.

- `fn dwo_section_name() -> Option<&'static str>`

  Returns the ELF section name (if any) for this type when used in a dwo

- `fn xcoff_section_name() -> Option<&'static str>`

  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- `fn load<F, E>(f: F) -> core::result::Result<Self, E>`

  Try to load the section using the given loader function.

- `fn dwp_range(&self, offset: u32, size: u32) -> Result<Self>`

  Returns the subrange of the section that is the contribution of

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>`

  Returns the `Reader` for this section.

#### Implementors

- [`DebugAbbrev`](read/index.md)
- [`DebugAddr`](read/index.md)
- [`DebugAranges`](read/index.md)
- [`DebugCuIndex`](read/index.md)
- [`DebugFrame`](read/index.md)
- [`DebugInfo`](read/index.md)
- [`DebugLineStr`](read/index.md)
- [`DebugLine`](read/index.md)
- [`DebugLocLists`](read/index.md)
- [`DebugLoc`](read/index.md)
- [`DebugMacinfo`](read/index.md)
- [`DebugMacro`](read/index.md)
- [`DebugPubNames`](read/index.md)
- [`DebugPubTypes`](read/index.md)
- [`DebugRanges`](read/index.md)
- [`DebugRngLists`](read/index.md)
- [`DebugStrOffsets`](read/index.md)
- [`DebugStr`](read/index.md)
- [`DebugTuIndex`](read/index.md)
- [`DebugTypes`](read/index.md)
- [`EhFrameHdr`](read/index.md)
- [`EhFrame`](read/index.md)

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianity for the target platform.

### `EndianBuf<'input, Endian>`

```rust
type EndianBuf<'input, Endian> = EndianSlice<'input, Endian>;
```

`EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
`gimli` versions, we export this type alias.

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result of a parse.

## Constants

### `DW_SECT_INFO`

```rust
const DW_SECT_INFO: DwSect;
```

### `DW_SECT_ABBREV`

```rust
const DW_SECT_ABBREV: DwSect;
```

### `DW_SECT_LINE`

```rust
const DW_SECT_LINE: DwSect;
```

### `DW_SECT_LOCLISTS`

```rust
const DW_SECT_LOCLISTS: DwSect;
```

### `DW_SECT_STR_OFFSETS`

```rust
const DW_SECT_STR_OFFSETS: DwSect;
```

### `DW_SECT_MACRO`

```rust
const DW_SECT_MACRO: DwSect;
```

### `DW_SECT_RNGLISTS`

```rust
const DW_SECT_RNGLISTS: DwSect;
```

### `DW_SECT_V2_INFO`

```rust
const DW_SECT_V2_INFO: DwSectV2;
```

### `DW_SECT_V2_TYPES`

```rust
const DW_SECT_V2_TYPES: DwSectV2;
```

### `DW_SECT_V2_ABBREV`

```rust
const DW_SECT_V2_ABBREV: DwSectV2;
```

### `DW_SECT_V2_LINE`

```rust
const DW_SECT_V2_LINE: DwSectV2;
```

### `DW_SECT_V2_LOC`

```rust
const DW_SECT_V2_LOC: DwSectV2;
```

### `DW_SECT_V2_STR_OFFSETS`

```rust
const DW_SECT_V2_STR_OFFSETS: DwSectV2;
```

### `DW_SECT_V2_MACINFO`

```rust
const DW_SECT_V2_MACINFO: DwSectV2;
```

### `DW_SECT_V2_MACRO`

```rust
const DW_SECT_V2_MACRO: DwSectV2;
```

### `DW_UT_compile`

```rust
const DW_UT_compile: DwUt;
```

### `DW_UT_type`

```rust
const DW_UT_type: DwUt;
```

### `DW_UT_partial`

```rust
const DW_UT_partial: DwUt;
```

### `DW_UT_skeleton`

```rust
const DW_UT_skeleton: DwUt;
```

### `DW_UT_split_compile`

```rust
const DW_UT_split_compile: DwUt;
```

### `DW_UT_split_type`

```rust
const DW_UT_split_type: DwUt;
```

### `DW_UT_lo_user`

```rust
const DW_UT_lo_user: DwUt;
```

### `DW_UT_hi_user`

```rust
const DW_UT_hi_user: DwUt;
```

### `DW_CFA_advance_loc`

```rust
const DW_CFA_advance_loc: DwCfa;
```

### `DW_CFA_offset`

```rust
const DW_CFA_offset: DwCfa;
```

### `DW_CFA_restore`

```rust
const DW_CFA_restore: DwCfa;
```

### `DW_CFA_nop`

```rust
const DW_CFA_nop: DwCfa;
```

### `DW_CFA_set_loc`

```rust
const DW_CFA_set_loc: DwCfa;
```

### `DW_CFA_advance_loc1`

```rust
const DW_CFA_advance_loc1: DwCfa;
```

### `DW_CFA_advance_loc2`

```rust
const DW_CFA_advance_loc2: DwCfa;
```

### `DW_CFA_advance_loc4`

```rust
const DW_CFA_advance_loc4: DwCfa;
```

### `DW_CFA_offset_extended`

```rust
const DW_CFA_offset_extended: DwCfa;
```

### `DW_CFA_restore_extended`

```rust
const DW_CFA_restore_extended: DwCfa;
```

### `DW_CFA_undefined`

```rust
const DW_CFA_undefined: DwCfa;
```

### `DW_CFA_same_value`

```rust
const DW_CFA_same_value: DwCfa;
```

### `DW_CFA_register`

```rust
const DW_CFA_register: DwCfa;
```

### `DW_CFA_remember_state`

```rust
const DW_CFA_remember_state: DwCfa;
```

### `DW_CFA_restore_state`

```rust
const DW_CFA_restore_state: DwCfa;
```

### `DW_CFA_def_cfa`

```rust
const DW_CFA_def_cfa: DwCfa;
```

### `DW_CFA_def_cfa_register`

```rust
const DW_CFA_def_cfa_register: DwCfa;
```

### `DW_CFA_def_cfa_offset`

```rust
const DW_CFA_def_cfa_offset: DwCfa;
```

### `DW_CFA_def_cfa_expression`

```rust
const DW_CFA_def_cfa_expression: DwCfa;
```

### `DW_CFA_expression`

```rust
const DW_CFA_expression: DwCfa;
```

### `DW_CFA_offset_extended_sf`

```rust
const DW_CFA_offset_extended_sf: DwCfa;
```

### `DW_CFA_def_cfa_sf`

```rust
const DW_CFA_def_cfa_sf: DwCfa;
```

### `DW_CFA_def_cfa_offset_sf`

```rust
const DW_CFA_def_cfa_offset_sf: DwCfa;
```

### `DW_CFA_val_offset`

```rust
const DW_CFA_val_offset: DwCfa;
```

### `DW_CFA_val_offset_sf`

```rust
const DW_CFA_val_offset_sf: DwCfa;
```

### `DW_CFA_val_expression`

```rust
const DW_CFA_val_expression: DwCfa;
```

### `DW_CFA_lo_user`

```rust
const DW_CFA_lo_user: DwCfa;
```

### `DW_CFA_hi_user`

```rust
const DW_CFA_hi_user: DwCfa;
```

### `DW_CFA_MIPS_advance_loc8`

```rust
const DW_CFA_MIPS_advance_loc8: DwCfa;
```

### `DW_CFA_GNU_window_save`

```rust
const DW_CFA_GNU_window_save: DwCfa;
```

### `DW_CFA_GNU_args_size`

```rust
const DW_CFA_GNU_args_size: DwCfa;
```

### `DW_CFA_GNU_negative_offset_extended`

```rust
const DW_CFA_GNU_negative_offset_extended: DwCfa;
```

### `DW_CFA_AARCH64_negate_ra_state`

```rust
const DW_CFA_AARCH64_negate_ra_state: DwCfa;
```

### `DW_CHILDREN_no`

```rust
const DW_CHILDREN_no: DwChildren;
```

### `DW_CHILDREN_yes`

```rust
const DW_CHILDREN_yes: DwChildren;
```

### `DW_TAG_null`

```rust
const DW_TAG_null: DwTag;
```

### `DW_TAG_global_subroutine`

```rust
const DW_TAG_global_subroutine: DwTag;
```

### `DW_TAG_global_variable`

```rust
const DW_TAG_global_variable: DwTag;
```

### `DW_TAG_local_variable`

```rust
const DW_TAG_local_variable: DwTag;
```

### `DW_TAG_subroutine`

```rust
const DW_TAG_subroutine: DwTag;
```

### `DW_TAG_array_type`

```rust
const DW_TAG_array_type: DwTag;
```

### `DW_TAG_class_type`

```rust
const DW_TAG_class_type: DwTag;
```

### `DW_TAG_entry_point`

```rust
const DW_TAG_entry_point: DwTag;
```

### `DW_TAG_enumeration_type`

```rust
const DW_TAG_enumeration_type: DwTag;
```

### `DW_TAG_formal_parameter`

```rust
const DW_TAG_formal_parameter: DwTag;
```

### `DW_TAG_imported_declaration`

```rust
const DW_TAG_imported_declaration: DwTag;
```

### `DW_TAG_label`

```rust
const DW_TAG_label: DwTag;
```

### `DW_TAG_lexical_block`

```rust
const DW_TAG_lexical_block: DwTag;
```

### `DW_TAG_member`

```rust
const DW_TAG_member: DwTag;
```

### `DW_TAG_pointer_type`

```rust
const DW_TAG_pointer_type: DwTag;
```

### `DW_TAG_reference_type`

```rust
const DW_TAG_reference_type: DwTag;
```

### `DW_TAG_compile_unit`

```rust
const DW_TAG_compile_unit: DwTag;
```

### `DW_TAG_string_type`

```rust
const DW_TAG_string_type: DwTag;
```

### `DW_TAG_structure_type`

```rust
const DW_TAG_structure_type: DwTag;
```

### `DW_TAG_subroutine_type`

```rust
const DW_TAG_subroutine_type: DwTag;
```

### `DW_TAG_typedef`

```rust
const DW_TAG_typedef: DwTag;
```

### `DW_TAG_union_type`

```rust
const DW_TAG_union_type: DwTag;
```

### `DW_TAG_unspecified_parameters`

```rust
const DW_TAG_unspecified_parameters: DwTag;
```

### `DW_TAG_variant`

```rust
const DW_TAG_variant: DwTag;
```

### `DW_TAG_common_block`

```rust
const DW_TAG_common_block: DwTag;
```

### `DW_TAG_common_inclusion`

```rust
const DW_TAG_common_inclusion: DwTag;
```

### `DW_TAG_inheritance`

```rust
const DW_TAG_inheritance: DwTag;
```

### `DW_TAG_inlined_subroutine`

```rust
const DW_TAG_inlined_subroutine: DwTag;
```

### `DW_TAG_module`

```rust
const DW_TAG_module: DwTag;
```

### `DW_TAG_ptr_to_member_type`

```rust
const DW_TAG_ptr_to_member_type: DwTag;
```

### `DW_TAG_set_type`

```rust
const DW_TAG_set_type: DwTag;
```

### `DW_TAG_subrange_type`

```rust
const DW_TAG_subrange_type: DwTag;
```

### `DW_TAG_with_stmt`

```rust
const DW_TAG_with_stmt: DwTag;
```

### `DW_TAG_access_declaration`

```rust
const DW_TAG_access_declaration: DwTag;
```

### `DW_TAG_base_type`

```rust
const DW_TAG_base_type: DwTag;
```

### `DW_TAG_catch_block`

```rust
const DW_TAG_catch_block: DwTag;
```

### `DW_TAG_const_type`

```rust
const DW_TAG_const_type: DwTag;
```

### `DW_TAG_constant`

```rust
const DW_TAG_constant: DwTag;
```

### `DW_TAG_enumerator`

```rust
const DW_TAG_enumerator: DwTag;
```

### `DW_TAG_file_type`

```rust
const DW_TAG_file_type: DwTag;
```

### `DW_TAG_friend`

```rust
const DW_TAG_friend: DwTag;
```

### `DW_TAG_namelist`

```rust
const DW_TAG_namelist: DwTag;
```

### `DW_TAG_namelist_item`

```rust
const DW_TAG_namelist_item: DwTag;
```

### `DW_TAG_packed_type`

```rust
const DW_TAG_packed_type: DwTag;
```

### `DW_TAG_subprogram`

```rust
const DW_TAG_subprogram: DwTag;
```

### `DW_TAG_template_type_parameter`

```rust
const DW_TAG_template_type_parameter: DwTag;
```

### `DW_TAG_template_value_parameter`

```rust
const DW_TAG_template_value_parameter: DwTag;
```

### `DW_TAG_thrown_type`

```rust
const DW_TAG_thrown_type: DwTag;
```

### `DW_TAG_try_block`

```rust
const DW_TAG_try_block: DwTag;
```

### `DW_TAG_variant_part`

```rust
const DW_TAG_variant_part: DwTag;
```

### `DW_TAG_variable`

```rust
const DW_TAG_variable: DwTag;
```

### `DW_TAG_volatile_type`

```rust
const DW_TAG_volatile_type: DwTag;
```

### `DW_TAG_dwarf_procedure`

```rust
const DW_TAG_dwarf_procedure: DwTag;
```

### `DW_TAG_restrict_type`

```rust
const DW_TAG_restrict_type: DwTag;
```

### `DW_TAG_interface_type`

```rust
const DW_TAG_interface_type: DwTag;
```

### `DW_TAG_namespace`

```rust
const DW_TAG_namespace: DwTag;
```

### `DW_TAG_imported_module`

```rust
const DW_TAG_imported_module: DwTag;
```

### `DW_TAG_unspecified_type`

```rust
const DW_TAG_unspecified_type: DwTag;
```

### `DW_TAG_partial_unit`

```rust
const DW_TAG_partial_unit: DwTag;
```

### `DW_TAG_imported_unit`

```rust
const DW_TAG_imported_unit: DwTag;
```

### `DW_TAG_condition`

```rust
const DW_TAG_condition: DwTag;
```

### `DW_TAG_shared_type`

```rust
const DW_TAG_shared_type: DwTag;
```

### `DW_TAG_type_unit`

```rust
const DW_TAG_type_unit: DwTag;
```

### `DW_TAG_rvalue_reference_type`

```rust
const DW_TAG_rvalue_reference_type: DwTag;
```

### `DW_TAG_template_alias`

```rust
const DW_TAG_template_alias: DwTag;
```

### `DW_TAG_coarray_type`

```rust
const DW_TAG_coarray_type: DwTag;
```

### `DW_TAG_generic_subrange`

```rust
const DW_TAG_generic_subrange: DwTag;
```

### `DW_TAG_dynamic_type`

```rust
const DW_TAG_dynamic_type: DwTag;
```

### `DW_TAG_atomic_type`

```rust
const DW_TAG_atomic_type: DwTag;
```

### `DW_TAG_call_site`

```rust
const DW_TAG_call_site: DwTag;
```

### `DW_TAG_call_site_parameter`

```rust
const DW_TAG_call_site_parameter: DwTag;
```

### `DW_TAG_skeleton_unit`

```rust
const DW_TAG_skeleton_unit: DwTag;
```

### `DW_TAG_immutable_type`

```rust
const DW_TAG_immutable_type: DwTag;
```

### `DW_TAG_lo_user`

```rust
const DW_TAG_lo_user: DwTag;
```

### `DW_TAG_hi_user`

```rust
const DW_TAG_hi_user: DwTag;
```

### `DW_TAG_MIPS_loop`

```rust
const DW_TAG_MIPS_loop: DwTag;
```

### `DW_TAG_HP_array_descriptor`

```rust
const DW_TAG_HP_array_descriptor: DwTag;
```

### `DW_TAG_HP_Bliss_field`

```rust
const DW_TAG_HP_Bliss_field: DwTag;
```

### `DW_TAG_HP_Bliss_field_set`

```rust
const DW_TAG_HP_Bliss_field_set: DwTag;
```

### `DW_TAG_format_label`

```rust
const DW_TAG_format_label: DwTag;
```

### `DW_TAG_function_template`

```rust
const DW_TAG_function_template: DwTag;
```

### `DW_TAG_class_template`

```rust
const DW_TAG_class_template: DwTag;
```

### `DW_TAG_GNU_BINCL`

```rust
const DW_TAG_GNU_BINCL: DwTag;
```

### `DW_TAG_GNU_EINCL`

```rust
const DW_TAG_GNU_EINCL: DwTag;
```

### `DW_TAG_GNU_template_template_param`

```rust
const DW_TAG_GNU_template_template_param: DwTag;
```

### `DW_TAG_GNU_template_parameter_pack`

```rust
const DW_TAG_GNU_template_parameter_pack: DwTag;
```

### `DW_TAG_GNU_formal_parameter_pack`

```rust
const DW_TAG_GNU_formal_parameter_pack: DwTag;
```

### `DW_TAG_GNU_call_site`

```rust
const DW_TAG_GNU_call_site: DwTag;
```

### `DW_TAG_GNU_call_site_parameter`

```rust
const DW_TAG_GNU_call_site_parameter: DwTag;
```

### `DW_TAG_APPLE_property`

```rust
const DW_TAG_APPLE_property: DwTag;
```

### `DW_TAG_SUN_function_template`

```rust
const DW_TAG_SUN_function_template: DwTag;
```

### `DW_TAG_SUN_class_template`

```rust
const DW_TAG_SUN_class_template: DwTag;
```

### `DW_TAG_SUN_struct_template`

```rust
const DW_TAG_SUN_struct_template: DwTag;
```

### `DW_TAG_SUN_union_template`

```rust
const DW_TAG_SUN_union_template: DwTag;
```

### `DW_TAG_SUN_indirect_inheritance`

```rust
const DW_TAG_SUN_indirect_inheritance: DwTag;
```

### `DW_TAG_SUN_codeflags`

```rust
const DW_TAG_SUN_codeflags: DwTag;
```

### `DW_TAG_SUN_memop_info`

```rust
const DW_TAG_SUN_memop_info: DwTag;
```

### `DW_TAG_SUN_omp_child_func`

```rust
const DW_TAG_SUN_omp_child_func: DwTag;
```

### `DW_TAG_SUN_rtti_descriptor`

```rust
const DW_TAG_SUN_rtti_descriptor: DwTag;
```

### `DW_TAG_SUN_dtor_info`

```rust
const DW_TAG_SUN_dtor_info: DwTag;
```

### `DW_TAG_SUN_dtor`

```rust
const DW_TAG_SUN_dtor: DwTag;
```

### `DW_TAG_SUN_f90_interface`

```rust
const DW_TAG_SUN_f90_interface: DwTag;
```

### `DW_TAG_SUN_fortran_vax_structure`

```rust
const DW_TAG_SUN_fortran_vax_structure: DwTag;
```

### `DW_TAG_ALTIUM_circ_type`

```rust
const DW_TAG_ALTIUM_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_mwa_circ_type`

```rust
const DW_TAG_ALTIUM_mwa_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_rev_carry_type`

```rust
const DW_TAG_ALTIUM_rev_carry_type: DwTag;
```

### `DW_TAG_ALTIUM_rom`

```rust
const DW_TAG_ALTIUM_rom: DwTag;
```

### `DW_TAG_upc_shared_type`

```rust
const DW_TAG_upc_shared_type: DwTag;
```

### `DW_TAG_upc_strict_type`

```rust
const DW_TAG_upc_strict_type: DwTag;
```

### `DW_TAG_upc_relaxed_type`

```rust
const DW_TAG_upc_relaxed_type: DwTag;
```

### `DW_TAG_PGI_kanji_type`

```rust
const DW_TAG_PGI_kanji_type: DwTag;
```

### `DW_TAG_PGI_interface_block`

```rust
const DW_TAG_PGI_interface_block: DwTag;
```

### `DW_TAG_BORLAND_property`

```rust
const DW_TAG_BORLAND_property: DwTag;
```

### `DW_TAG_BORLAND_Delphi_string`

```rust
const DW_TAG_BORLAND_Delphi_string: DwTag;
```

### `DW_TAG_BORLAND_Delphi_dynamic_array`

```rust
const DW_TAG_BORLAND_Delphi_dynamic_array: DwTag;
```

### `DW_TAG_BORLAND_Delphi_set`

```rust
const DW_TAG_BORLAND_Delphi_set: DwTag;
```

### `DW_TAG_BORLAND_Delphi_variant`

```rust
const DW_TAG_BORLAND_Delphi_variant: DwTag;
```

### `DW_AT_null`

```rust
const DW_AT_null: DwAt;
```

### `DW_AT_fund_type`

```rust
const DW_AT_fund_type: DwAt;
```

### `DW_AT_mod_fund_type`

```rust
const DW_AT_mod_fund_type: DwAt;
```

### `DW_AT_user_def_type`

```rust
const DW_AT_user_def_type: DwAt;
```

### `DW_AT_mod_u_d_type`

```rust
const DW_AT_mod_u_d_type: DwAt;
```

### `DW_AT_subscr_data`

```rust
const DW_AT_subscr_data: DwAt;
```

### `DW_AT_element_list`

```rust
const DW_AT_element_list: DwAt;
```

### `DW_AT_member`

```rust
const DW_AT_member: DwAt;
```

### `DW_AT_friends`

```rust
const DW_AT_friends: DwAt;
```

### `DW_AT_program`

```rust
const DW_AT_program: DwAt;
```

### `DW_AT_private`

```rust
const DW_AT_private: DwAt;
```

### `DW_AT_protected`

```rust
const DW_AT_protected: DwAt;
```

### `DW_AT_public`

```rust
const DW_AT_public: DwAt;
```

### `DW_AT_pure_virtual`

```rust
const DW_AT_pure_virtual: DwAt;
```

### `DW_AT_virtual`

```rust
const DW_AT_virtual: DwAt;
```

### `DW_AT_specification_v1`

```rust
const DW_AT_specification_v1: DwAt;
```

### `DW_AT_sibling`

```rust
const DW_AT_sibling: DwAt;
```

### `DW_AT_location`

```rust
const DW_AT_location: DwAt;
```

### `DW_AT_name`

```rust
const DW_AT_name: DwAt;
```

### `DW_AT_ordering`

```rust
const DW_AT_ordering: DwAt;
```

### `DW_AT_byte_size`

```rust
const DW_AT_byte_size: DwAt;
```

### `DW_AT_bit_offset`

```rust
const DW_AT_bit_offset: DwAt;
```

### `DW_AT_bit_size`

```rust
const DW_AT_bit_size: DwAt;
```

### `DW_AT_stmt_list`

```rust
const DW_AT_stmt_list: DwAt;
```

### `DW_AT_low_pc`

```rust
const DW_AT_low_pc: DwAt;
```

### `DW_AT_high_pc`

```rust
const DW_AT_high_pc: DwAt;
```

### `DW_AT_language`

```rust
const DW_AT_language: DwAt;
```

### `DW_AT_discr`

```rust
const DW_AT_discr: DwAt;
```

### `DW_AT_discr_value`

```rust
const DW_AT_discr_value: DwAt;
```

### `DW_AT_visibility`

```rust
const DW_AT_visibility: DwAt;
```

### `DW_AT_import`

```rust
const DW_AT_import: DwAt;
```

### `DW_AT_string_length`

```rust
const DW_AT_string_length: DwAt;
```

### `DW_AT_common_reference`

```rust
const DW_AT_common_reference: DwAt;
```

### `DW_AT_comp_dir`

```rust
const DW_AT_comp_dir: DwAt;
```

### `DW_AT_const_value`

```rust
const DW_AT_const_value: DwAt;
```

### `DW_AT_containing_type`

```rust
const DW_AT_containing_type: DwAt;
```

### `DW_AT_default_value`

```rust
const DW_AT_default_value: DwAt;
```

### `DW_AT_inline`

```rust
const DW_AT_inline: DwAt;
```

### `DW_AT_is_optional`

```rust
const DW_AT_is_optional: DwAt;
```

### `DW_AT_lower_bound`

```rust
const DW_AT_lower_bound: DwAt;
```

### `DW_AT_producer`

```rust
const DW_AT_producer: DwAt;
```

### `DW_AT_prototyped`

```rust
const DW_AT_prototyped: DwAt;
```

### `DW_AT_return_addr`

```rust
const DW_AT_return_addr: DwAt;
```

### `DW_AT_start_scope`

```rust
const DW_AT_start_scope: DwAt;
```

### `DW_AT_bit_stride`

```rust
const DW_AT_bit_stride: DwAt;
```

### `DW_AT_upper_bound`

```rust
const DW_AT_upper_bound: DwAt;
```

### `DW_AT_abstract_origin`

```rust
const DW_AT_abstract_origin: DwAt;
```

### `DW_AT_accessibility`

```rust
const DW_AT_accessibility: DwAt;
```

### `DW_AT_address_class`

```rust
const DW_AT_address_class: DwAt;
```

### `DW_AT_artificial`

```rust
const DW_AT_artificial: DwAt;
```

### `DW_AT_base_types`

```rust
const DW_AT_base_types: DwAt;
```

### `DW_AT_calling_convention`

```rust
const DW_AT_calling_convention: DwAt;
```

### `DW_AT_count`

```rust
const DW_AT_count: DwAt;
```

### `DW_AT_data_member_location`

```rust
const DW_AT_data_member_location: DwAt;
```

### `DW_AT_decl_column`

```rust
const DW_AT_decl_column: DwAt;
```

### `DW_AT_decl_file`

```rust
const DW_AT_decl_file: DwAt;
```

### `DW_AT_decl_line`

```rust
const DW_AT_decl_line: DwAt;
```

### `DW_AT_declaration`

```rust
const DW_AT_declaration: DwAt;
```

### `DW_AT_discr_list`

```rust
const DW_AT_discr_list: DwAt;
```

### `DW_AT_encoding`

```rust
const DW_AT_encoding: DwAt;
```

### `DW_AT_external`

```rust
const DW_AT_external: DwAt;
```

### `DW_AT_frame_base`

```rust
const DW_AT_frame_base: DwAt;
```

### `DW_AT_friend`

```rust
const DW_AT_friend: DwAt;
```

### `DW_AT_identifier_case`

```rust
const DW_AT_identifier_case: DwAt;
```

### `DW_AT_macro_info`

```rust
const DW_AT_macro_info: DwAt;
```

### `DW_AT_namelist_item`

```rust
const DW_AT_namelist_item: DwAt;
```

### `DW_AT_priority`

```rust
const DW_AT_priority: DwAt;
```

### `DW_AT_segment`

```rust
const DW_AT_segment: DwAt;
```

### `DW_AT_specification`

```rust
const DW_AT_specification: DwAt;
```

### `DW_AT_static_link`

```rust
const DW_AT_static_link: DwAt;
```

### `DW_AT_type`

```rust
const DW_AT_type: DwAt;
```

### `DW_AT_use_location`

```rust
const DW_AT_use_location: DwAt;
```

### `DW_AT_variable_parameter`

```rust
const DW_AT_variable_parameter: DwAt;
```

### `DW_AT_virtuality`

```rust
const DW_AT_virtuality: DwAt;
```

### `DW_AT_vtable_elem_location`

```rust
const DW_AT_vtable_elem_location: DwAt;
```

### `DW_AT_allocated`

```rust
const DW_AT_allocated: DwAt;
```

### `DW_AT_associated`

```rust
const DW_AT_associated: DwAt;
```

### `DW_AT_data_location`

```rust
const DW_AT_data_location: DwAt;
```

### `DW_AT_byte_stride`

```rust
const DW_AT_byte_stride: DwAt;
```

### `DW_AT_entry_pc`

```rust
const DW_AT_entry_pc: DwAt;
```

### `DW_AT_use_UTF8`

```rust
const DW_AT_use_UTF8: DwAt;
```

### `DW_AT_extension`

```rust
const DW_AT_extension: DwAt;
```

### `DW_AT_ranges`

```rust
const DW_AT_ranges: DwAt;
```

### `DW_AT_trampoline`

```rust
const DW_AT_trampoline: DwAt;
```

### `DW_AT_call_column`

```rust
const DW_AT_call_column: DwAt;
```

### `DW_AT_call_file`

```rust
const DW_AT_call_file: DwAt;
```

### `DW_AT_call_line`

```rust
const DW_AT_call_line: DwAt;
```

### `DW_AT_description`

```rust
const DW_AT_description: DwAt;
```

### `DW_AT_binary_scale`

```rust
const DW_AT_binary_scale: DwAt;
```

### `DW_AT_decimal_scale`

```rust
const DW_AT_decimal_scale: DwAt;
```

### `DW_AT_small`

```rust
const DW_AT_small: DwAt;
```

### `DW_AT_decimal_sign`

```rust
const DW_AT_decimal_sign: DwAt;
```

### `DW_AT_digit_count`

```rust
const DW_AT_digit_count: DwAt;
```

### `DW_AT_picture_string`

```rust
const DW_AT_picture_string: DwAt;
```

### `DW_AT_mutable`

```rust
const DW_AT_mutable: DwAt;
```

### `DW_AT_threads_scaled`

```rust
const DW_AT_threads_scaled: DwAt;
```

### `DW_AT_explicit`

```rust
const DW_AT_explicit: DwAt;
```

### `DW_AT_object_pointer`

```rust
const DW_AT_object_pointer: DwAt;
```

### `DW_AT_endianity`

```rust
const DW_AT_endianity: DwAt;
```

### `DW_AT_elemental`

```rust
const DW_AT_elemental: DwAt;
```

### `DW_AT_pure`

```rust
const DW_AT_pure: DwAt;
```

### `DW_AT_recursive`

```rust
const DW_AT_recursive: DwAt;
```

### `DW_AT_signature`

```rust
const DW_AT_signature: DwAt;
```

### `DW_AT_main_subprogram`

```rust
const DW_AT_main_subprogram: DwAt;
```

### `DW_AT_data_bit_offset`

```rust
const DW_AT_data_bit_offset: DwAt;
```

### `DW_AT_const_expr`

```rust
const DW_AT_const_expr: DwAt;
```

### `DW_AT_enum_class`

```rust
const DW_AT_enum_class: DwAt;
```

### `DW_AT_linkage_name`

```rust
const DW_AT_linkage_name: DwAt;
```

### `DW_AT_string_length_bit_size`

```rust
const DW_AT_string_length_bit_size: DwAt;
```

### `DW_AT_string_length_byte_size`

```rust
const DW_AT_string_length_byte_size: DwAt;
```

### `DW_AT_rank`

```rust
const DW_AT_rank: DwAt;
```

### `DW_AT_str_offsets_base`

```rust
const DW_AT_str_offsets_base: DwAt;
```

### `DW_AT_addr_base`

```rust
const DW_AT_addr_base: DwAt;
```

### `DW_AT_rnglists_base`

```rust
const DW_AT_rnglists_base: DwAt;
```

### `DW_AT_dwo_name`

```rust
const DW_AT_dwo_name: DwAt;
```

### `DW_AT_reference`

```rust
const DW_AT_reference: DwAt;
```

### `DW_AT_rvalue_reference`

```rust
const DW_AT_rvalue_reference: DwAt;
```

### `DW_AT_macros`

```rust
const DW_AT_macros: DwAt;
```

### `DW_AT_call_all_calls`

```rust
const DW_AT_call_all_calls: DwAt;
```

### `DW_AT_call_all_source_calls`

```rust
const DW_AT_call_all_source_calls: DwAt;
```

### `DW_AT_call_all_tail_calls`

```rust
const DW_AT_call_all_tail_calls: DwAt;
```

### `DW_AT_call_return_pc`

```rust
const DW_AT_call_return_pc: DwAt;
```

### `DW_AT_call_value`

```rust
const DW_AT_call_value: DwAt;
```

### `DW_AT_call_origin`

```rust
const DW_AT_call_origin: DwAt;
```

### `DW_AT_call_parameter`

```rust
const DW_AT_call_parameter: DwAt;
```

### `DW_AT_call_pc`

```rust
const DW_AT_call_pc: DwAt;
```

### `DW_AT_call_tail_call`

```rust
const DW_AT_call_tail_call: DwAt;
```

### `DW_AT_call_target`

```rust
const DW_AT_call_target: DwAt;
```

### `DW_AT_call_target_clobbered`

```rust
const DW_AT_call_target_clobbered: DwAt;
```

### `DW_AT_call_data_location`

```rust
const DW_AT_call_data_location: DwAt;
```

### `DW_AT_call_data_value`

```rust
const DW_AT_call_data_value: DwAt;
```

### `DW_AT_noreturn`

```rust
const DW_AT_noreturn: DwAt;
```

### `DW_AT_alignment`

```rust
const DW_AT_alignment: DwAt;
```

### `DW_AT_export_symbols`

```rust
const DW_AT_export_symbols: DwAt;
```

### `DW_AT_deleted`

```rust
const DW_AT_deleted: DwAt;
```

### `DW_AT_defaulted`

```rust
const DW_AT_defaulted: DwAt;
```

### `DW_AT_loclists_base`

```rust
const DW_AT_loclists_base: DwAt;
```

### `DW_AT_lo_user`

```rust
const DW_AT_lo_user: DwAt;
```

### `DW_AT_hi_user`

```rust
const DW_AT_hi_user: DwAt;
```

### `DW_AT_MIPS_fde`

```rust
const DW_AT_MIPS_fde: DwAt;
```

### `DW_AT_MIPS_loop_begin`

```rust
const DW_AT_MIPS_loop_begin: DwAt;
```

### `DW_AT_MIPS_tail_loop_begin`

```rust
const DW_AT_MIPS_tail_loop_begin: DwAt;
```

### `DW_AT_MIPS_epilog_begin`

```rust
const DW_AT_MIPS_epilog_begin: DwAt;
```

### `DW_AT_MIPS_loop_unroll_factor`

```rust
const DW_AT_MIPS_loop_unroll_factor: DwAt;
```

### `DW_AT_MIPS_software_pipeline_depth`

```rust
const DW_AT_MIPS_software_pipeline_depth: DwAt;
```

### `DW_AT_MIPS_linkage_name`

```rust
const DW_AT_MIPS_linkage_name: DwAt;
```

### `DW_AT_MIPS_stride`

```rust
const DW_AT_MIPS_stride: DwAt;
```

### `DW_AT_MIPS_abstract_name`

```rust
const DW_AT_MIPS_abstract_name: DwAt;
```

### `DW_AT_MIPS_clone_origin`

```rust
const DW_AT_MIPS_clone_origin: DwAt;
```

### `DW_AT_MIPS_has_inlines`

```rust
const DW_AT_MIPS_has_inlines: DwAt;
```

### `DW_AT_MIPS_stride_byte`

```rust
const DW_AT_MIPS_stride_byte: DwAt;
```

### `DW_AT_MIPS_stride_elem`

```rust
const DW_AT_MIPS_stride_elem: DwAt;
```

### `DW_AT_MIPS_ptr_dopetype`

```rust
const DW_AT_MIPS_ptr_dopetype: DwAt;
```

### `DW_AT_MIPS_allocatable_dopetype`

```rust
const DW_AT_MIPS_allocatable_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_shape_dopetype`

```rust
const DW_AT_MIPS_assumed_shape_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_size`

```rust
const DW_AT_MIPS_assumed_size: DwAt;
```

### `DW_AT_INTEL_other_endian`

```rust
const DW_AT_INTEL_other_endian: DwAt;
```

### `DW_AT_sf_names`

```rust
const DW_AT_sf_names: DwAt;
```

### `DW_AT_src_info`

```rust
const DW_AT_src_info: DwAt;
```

### `DW_AT_mac_info`

```rust
const DW_AT_mac_info: DwAt;
```

### `DW_AT_src_coords`

```rust
const DW_AT_src_coords: DwAt;
```

### `DW_AT_body_begin`

```rust
const DW_AT_body_begin: DwAt;
```

### `DW_AT_body_end`

```rust
const DW_AT_body_end: DwAt;
```

### `DW_AT_GNU_vector`

```rust
const DW_AT_GNU_vector: DwAt;
```

### `DW_AT_GNU_guarded_by`

```rust
const DW_AT_GNU_guarded_by: DwAt;
```

### `DW_AT_GNU_pt_guarded_by`

```rust
const DW_AT_GNU_pt_guarded_by: DwAt;
```

### `DW_AT_GNU_guarded`

```rust
const DW_AT_GNU_guarded: DwAt;
```

### `DW_AT_GNU_pt_guarded`

```rust
const DW_AT_GNU_pt_guarded: DwAt;
```

### `DW_AT_GNU_locks_excluded`

```rust
const DW_AT_GNU_locks_excluded: DwAt;
```

### `DW_AT_GNU_exclusive_locks_required`

```rust
const DW_AT_GNU_exclusive_locks_required: DwAt;
```

### `DW_AT_GNU_shared_locks_required`

```rust
const DW_AT_GNU_shared_locks_required: DwAt;
```

### `DW_AT_GNU_odr_signature`

```rust
const DW_AT_GNU_odr_signature: DwAt;
```

### `DW_AT_GNU_template_name`

```rust
const DW_AT_GNU_template_name: DwAt;
```

### `DW_AT_GNU_call_site_value`

```rust
const DW_AT_GNU_call_site_value: DwAt;
```

### `DW_AT_GNU_call_site_data_value`

```rust
const DW_AT_GNU_call_site_data_value: DwAt;
```

### `DW_AT_GNU_call_site_target`

```rust
const DW_AT_GNU_call_site_target: DwAt;
```

### `DW_AT_GNU_call_site_target_clobbered`

```rust
const DW_AT_GNU_call_site_target_clobbered: DwAt;
```

### `DW_AT_GNU_tail_call`

```rust
const DW_AT_GNU_tail_call: DwAt;
```

### `DW_AT_GNU_all_tail_call_sites`

```rust
const DW_AT_GNU_all_tail_call_sites: DwAt;
```

### `DW_AT_GNU_all_call_sites`

```rust
const DW_AT_GNU_all_call_sites: DwAt;
```

### `DW_AT_GNU_all_source_call_sites`

```rust
const DW_AT_GNU_all_source_call_sites: DwAt;
```

### `DW_AT_GNU_macros`

```rust
const DW_AT_GNU_macros: DwAt;
```

### `DW_AT_GNU_deleted`

```rust
const DW_AT_GNU_deleted: DwAt;
```

### `DW_AT_GNU_dwo_name`

```rust
const DW_AT_GNU_dwo_name: DwAt;
```

### `DW_AT_GNU_dwo_id`

```rust
const DW_AT_GNU_dwo_id: DwAt;
```

### `DW_AT_GNU_ranges_base`

```rust
const DW_AT_GNU_ranges_base: DwAt;
```

### `DW_AT_GNU_addr_base`

```rust
const DW_AT_GNU_addr_base: DwAt;
```

### `DW_AT_GNU_pubnames`

```rust
const DW_AT_GNU_pubnames: DwAt;
```

### `DW_AT_GNU_pubtypes`

```rust
const DW_AT_GNU_pubtypes: DwAt;
```

### `DW_AT_GNU_discriminator`

```rust
const DW_AT_GNU_discriminator: DwAt;
```

### `DW_AT_GNU_locviews`

```rust
const DW_AT_GNU_locviews: DwAt;
```

### `DW_AT_GNU_entry_view`

```rust
const DW_AT_GNU_entry_view: DwAt;
```

### `DW_AT_SUN_template`

```rust
const DW_AT_SUN_template: DwAt;
```

### `DW_AT_SUN_alignment`

```rust
const DW_AT_SUN_alignment: DwAt;
```

### `DW_AT_SUN_vtable`

```rust
const DW_AT_SUN_vtable: DwAt;
```

### `DW_AT_SUN_count_guarantee`

```rust
const DW_AT_SUN_count_guarantee: DwAt;
```

### `DW_AT_SUN_command_line`

```rust
const DW_AT_SUN_command_line: DwAt;
```

### `DW_AT_SUN_vbase`

```rust
const DW_AT_SUN_vbase: DwAt;
```

### `DW_AT_SUN_compile_options`

```rust
const DW_AT_SUN_compile_options: DwAt;
```

### `DW_AT_SUN_language`

```rust
const DW_AT_SUN_language: DwAt;
```

### `DW_AT_SUN_browser_file`

```rust
const DW_AT_SUN_browser_file: DwAt;
```

### `DW_AT_SUN_vtable_abi`

```rust
const DW_AT_SUN_vtable_abi: DwAt;
```

### `DW_AT_SUN_func_offsets`

```rust
const DW_AT_SUN_func_offsets: DwAt;
```

### `DW_AT_SUN_cf_kind`

```rust
const DW_AT_SUN_cf_kind: DwAt;
```

### `DW_AT_SUN_vtable_index`

```rust
const DW_AT_SUN_vtable_index: DwAt;
```

### `DW_AT_SUN_omp_tpriv_addr`

```rust
const DW_AT_SUN_omp_tpriv_addr: DwAt;
```

### `DW_AT_SUN_omp_child_func`

```rust
const DW_AT_SUN_omp_child_func: DwAt;
```

### `DW_AT_SUN_func_offset`

```rust
const DW_AT_SUN_func_offset: DwAt;
```

### `DW_AT_SUN_memop_type_ref`

```rust
const DW_AT_SUN_memop_type_ref: DwAt;
```

### `DW_AT_SUN_profile_id`

```rust
const DW_AT_SUN_profile_id: DwAt;
```

### `DW_AT_SUN_memop_signature`

```rust
const DW_AT_SUN_memop_signature: DwAt;
```

### `DW_AT_SUN_obj_dir`

```rust
const DW_AT_SUN_obj_dir: DwAt;
```

### `DW_AT_SUN_obj_file`

```rust
const DW_AT_SUN_obj_file: DwAt;
```

### `DW_AT_SUN_original_name`

```rust
const DW_AT_SUN_original_name: DwAt;
```

### `DW_AT_SUN_hwcprof_signature`

```rust
const DW_AT_SUN_hwcprof_signature: DwAt;
```

### `DW_AT_SUN_amd64_parmdump`

```rust
const DW_AT_SUN_amd64_parmdump: DwAt;
```

### `DW_AT_SUN_part_link_name`

```rust
const DW_AT_SUN_part_link_name: DwAt;
```

### `DW_AT_SUN_link_name`

```rust
const DW_AT_SUN_link_name: DwAt;
```

### `DW_AT_SUN_pass_with_const`

```rust
const DW_AT_SUN_pass_with_const: DwAt;
```

### `DW_AT_SUN_return_with_const`

```rust
const DW_AT_SUN_return_with_const: DwAt;
```

### `DW_AT_SUN_import_by_name`

```rust
const DW_AT_SUN_import_by_name: DwAt;
```

### `DW_AT_SUN_f90_pointer`

```rust
const DW_AT_SUN_f90_pointer: DwAt;
```

### `DW_AT_SUN_pass_by_ref`

```rust
const DW_AT_SUN_pass_by_ref: DwAt;
```

### `DW_AT_SUN_f90_allocatable`

```rust
const DW_AT_SUN_f90_allocatable: DwAt;
```

### `DW_AT_SUN_f90_assumed_shape_array`

```rust
const DW_AT_SUN_f90_assumed_shape_array: DwAt;
```

### `DW_AT_SUN_c_vla`

```rust
const DW_AT_SUN_c_vla: DwAt;
```

### `DW_AT_SUN_return_value_ptr`

```rust
const DW_AT_SUN_return_value_ptr: DwAt;
```

### `DW_AT_SUN_dtor_start`

```rust
const DW_AT_SUN_dtor_start: DwAt;
```

### `DW_AT_SUN_dtor_length`

```rust
const DW_AT_SUN_dtor_length: DwAt;
```

### `DW_AT_SUN_dtor_state_initial`

```rust
const DW_AT_SUN_dtor_state_initial: DwAt;
```

### `DW_AT_SUN_dtor_state_final`

```rust
const DW_AT_SUN_dtor_state_final: DwAt;
```

### `DW_AT_SUN_dtor_state_deltas`

```rust
const DW_AT_SUN_dtor_state_deltas: DwAt;
```

### `DW_AT_SUN_import_by_lname`

```rust
const DW_AT_SUN_import_by_lname: DwAt;
```

### `DW_AT_SUN_f90_use_only`

```rust
const DW_AT_SUN_f90_use_only: DwAt;
```

### `DW_AT_SUN_namelist_spec`

```rust
const DW_AT_SUN_namelist_spec: DwAt;
```

### `DW_AT_SUN_is_omp_child_func`

```rust
const DW_AT_SUN_is_omp_child_func: DwAt;
```

### `DW_AT_SUN_fortran_main_alias`

```rust
const DW_AT_SUN_fortran_main_alias: DwAt;
```

### `DW_AT_SUN_fortran_based`

```rust
const DW_AT_SUN_fortran_based: DwAt;
```

### `DW_AT_ALTIUM_loclist`

```rust
const DW_AT_ALTIUM_loclist: DwAt;
```

### `DW_AT_use_GNAT_descriptive_type`

```rust
const DW_AT_use_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNAT_descriptive_type`

```rust
const DW_AT_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNU_numerator`

```rust
const DW_AT_GNU_numerator: DwAt;
```

### `DW_AT_GNU_denominator`

```rust
const DW_AT_GNU_denominator: DwAt;
```

### `DW_AT_GNU_bias`

```rust
const DW_AT_GNU_bias: DwAt;
```

### `DW_AT_upc_threads_scaled`

```rust
const DW_AT_upc_threads_scaled: DwAt;
```

### `DW_AT_PGI_lbase`

```rust
const DW_AT_PGI_lbase: DwAt;
```

### `DW_AT_PGI_soffset`

```rust
const DW_AT_PGI_soffset: DwAt;
```

### `DW_AT_PGI_lstride`

```rust
const DW_AT_PGI_lstride: DwAt;
```

### `DW_AT_BORLAND_property_read`

```rust
const DW_AT_BORLAND_property_read: DwAt;
```

### `DW_AT_BORLAND_property_write`

```rust
const DW_AT_BORLAND_property_write: DwAt;
```

### `DW_AT_BORLAND_property_implements`

```rust
const DW_AT_BORLAND_property_implements: DwAt;
```

### `DW_AT_BORLAND_property_index`

```rust
const DW_AT_BORLAND_property_index: DwAt;
```

### `DW_AT_BORLAND_property_default`

```rust
const DW_AT_BORLAND_property_default: DwAt;
```

### `DW_AT_BORLAND_Delphi_unit`

```rust
const DW_AT_BORLAND_Delphi_unit: DwAt;
```

### `DW_AT_BORLAND_Delphi_class`

```rust
const DW_AT_BORLAND_Delphi_class: DwAt;
```

### `DW_AT_BORLAND_Delphi_record`

```rust
const DW_AT_BORLAND_Delphi_record: DwAt;
```

### `DW_AT_BORLAND_Delphi_metaclass`

```rust
const DW_AT_BORLAND_Delphi_metaclass: DwAt;
```

### `DW_AT_BORLAND_Delphi_constructor`

```rust
const DW_AT_BORLAND_Delphi_constructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_destructor`

```rust
const DW_AT_BORLAND_Delphi_destructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_anonymous_method`

```rust
const DW_AT_BORLAND_Delphi_anonymous_method: DwAt;
```

### `DW_AT_BORLAND_Delphi_interface`

```rust
const DW_AT_BORLAND_Delphi_interface: DwAt;
```

### `DW_AT_BORLAND_Delphi_ABI`

```rust
const DW_AT_BORLAND_Delphi_ABI: DwAt;
```

### `DW_AT_BORLAND_Delphi_return`

```rust
const DW_AT_BORLAND_Delphi_return: DwAt;
```

### `DW_AT_BORLAND_Delphi_frameptr`

```rust
const DW_AT_BORLAND_Delphi_frameptr: DwAt;
```

### `DW_AT_BORLAND_closure`

```rust
const DW_AT_BORLAND_closure: DwAt;
```

### `DW_AT_LLVM_include_path`

```rust
const DW_AT_LLVM_include_path: DwAt;
```

### `DW_AT_LLVM_config_macros`

```rust
const DW_AT_LLVM_config_macros: DwAt;
```

### `DW_AT_LLVM_isysroot`

```rust
const DW_AT_LLVM_isysroot: DwAt;
```

### `DW_AT_APPLE_optimized`

```rust
const DW_AT_APPLE_optimized: DwAt;
```

### `DW_AT_APPLE_flags`

```rust
const DW_AT_APPLE_flags: DwAt;
```

### `DW_AT_APPLE_isa`

```rust
const DW_AT_APPLE_isa: DwAt;
```

### `DW_AT_APPLE_block`

```rust
const DW_AT_APPLE_block: DwAt;
```

### `DW_AT_APPLE_major_runtime_vers`

```rust
const DW_AT_APPLE_major_runtime_vers: DwAt;
```

### `DW_AT_APPLE_runtime_class`

```rust
const DW_AT_APPLE_runtime_class: DwAt;
```

### `DW_AT_APPLE_omit_frame_ptr`

```rust
const DW_AT_APPLE_omit_frame_ptr: DwAt;
```

### `DW_AT_APPLE_property_name`

```rust
const DW_AT_APPLE_property_name: DwAt;
```

### `DW_AT_APPLE_property_getter`

```rust
const DW_AT_APPLE_property_getter: DwAt;
```

### `DW_AT_APPLE_property_setter`

```rust
const DW_AT_APPLE_property_setter: DwAt;
```

### `DW_AT_APPLE_property_attribute`

```rust
const DW_AT_APPLE_property_attribute: DwAt;
```

### `DW_AT_APPLE_objc_complete_type`

```rust
const DW_AT_APPLE_objc_complete_type: DwAt;
```

### `DW_AT_APPLE_property`

```rust
const DW_AT_APPLE_property: DwAt;
```

### `DW_FORM_null`

```rust
const DW_FORM_null: DwForm;
```

### `DW_FORM_ref`

```rust
const DW_FORM_ref: DwForm;
```

### `DW_FORM_addr`

```rust
const DW_FORM_addr: DwForm;
```

### `DW_FORM_block2`

```rust
const DW_FORM_block2: DwForm;
```

### `DW_FORM_block4`

```rust
const DW_FORM_block4: DwForm;
```

### `DW_FORM_data2`

```rust
const DW_FORM_data2: DwForm;
```

### `DW_FORM_data4`

```rust
const DW_FORM_data4: DwForm;
```

### `DW_FORM_data8`

```rust
const DW_FORM_data8: DwForm;
```

### `DW_FORM_string`

```rust
const DW_FORM_string: DwForm;
```

### `DW_FORM_block`

```rust
const DW_FORM_block: DwForm;
```

### `DW_FORM_block1`

```rust
const DW_FORM_block1: DwForm;
```

### `DW_FORM_data1`

```rust
const DW_FORM_data1: DwForm;
```

### `DW_FORM_flag`

```rust
const DW_FORM_flag: DwForm;
```

### `DW_FORM_sdata`

```rust
const DW_FORM_sdata: DwForm;
```

### `DW_FORM_strp`

```rust
const DW_FORM_strp: DwForm;
```

### `DW_FORM_udata`

```rust
const DW_FORM_udata: DwForm;
```

### `DW_FORM_ref_addr`

```rust
const DW_FORM_ref_addr: DwForm;
```

### `DW_FORM_ref1`

```rust
const DW_FORM_ref1: DwForm;
```

### `DW_FORM_ref2`

```rust
const DW_FORM_ref2: DwForm;
```

### `DW_FORM_ref4`

```rust
const DW_FORM_ref4: DwForm;
```

### `DW_FORM_ref8`

```rust
const DW_FORM_ref8: DwForm;
```

### `DW_FORM_ref_udata`

```rust
const DW_FORM_ref_udata: DwForm;
```

### `DW_FORM_indirect`

```rust
const DW_FORM_indirect: DwForm;
```

### `DW_FORM_sec_offset`

```rust
const DW_FORM_sec_offset: DwForm;
```

### `DW_FORM_exprloc`

```rust
const DW_FORM_exprloc: DwForm;
```

### `DW_FORM_flag_present`

```rust
const DW_FORM_flag_present: DwForm;
```

### `DW_FORM_ref_sig8`

```rust
const DW_FORM_ref_sig8: DwForm;
```

### `DW_FORM_strx`

```rust
const DW_FORM_strx: DwForm;
```

### `DW_FORM_addrx`

```rust
const DW_FORM_addrx: DwForm;
```

### `DW_FORM_ref_sup4`

```rust
const DW_FORM_ref_sup4: DwForm;
```

### `DW_FORM_strp_sup`

```rust
const DW_FORM_strp_sup: DwForm;
```

### `DW_FORM_data16`

```rust
const DW_FORM_data16: DwForm;
```

### `DW_FORM_line_strp`

```rust
const DW_FORM_line_strp: DwForm;
```

### `DW_FORM_implicit_const`

```rust
const DW_FORM_implicit_const: DwForm;
```

### `DW_FORM_loclistx`

```rust
const DW_FORM_loclistx: DwForm;
```

### `DW_FORM_rnglistx`

```rust
const DW_FORM_rnglistx: DwForm;
```

### `DW_FORM_ref_sup8`

```rust
const DW_FORM_ref_sup8: DwForm;
```

### `DW_FORM_strx1`

```rust
const DW_FORM_strx1: DwForm;
```

### `DW_FORM_strx2`

```rust
const DW_FORM_strx2: DwForm;
```

### `DW_FORM_strx3`

```rust
const DW_FORM_strx3: DwForm;
```

### `DW_FORM_strx4`

```rust
const DW_FORM_strx4: DwForm;
```

### `DW_FORM_addrx1`

```rust
const DW_FORM_addrx1: DwForm;
```

### `DW_FORM_addrx2`

```rust
const DW_FORM_addrx2: DwForm;
```

### `DW_FORM_addrx3`

```rust
const DW_FORM_addrx3: DwForm;
```

### `DW_FORM_addrx4`

```rust
const DW_FORM_addrx4: DwForm;
```

### `DW_FORM_GNU_addr_index`

```rust
const DW_FORM_GNU_addr_index: DwForm;
```

### `DW_FORM_GNU_str_index`

```rust
const DW_FORM_GNU_str_index: DwForm;
```

### `DW_FORM_GNU_ref_alt`

```rust
const DW_FORM_GNU_ref_alt: DwForm;
```

### `DW_FORM_GNU_strp_alt`

```rust
const DW_FORM_GNU_strp_alt: DwForm;
```

### `DW_ATE_address`

```rust
const DW_ATE_address: DwAte;
```

### `DW_ATE_boolean`

```rust
const DW_ATE_boolean: DwAte;
```

### `DW_ATE_complex_float`

```rust
const DW_ATE_complex_float: DwAte;
```

### `DW_ATE_float`

```rust
const DW_ATE_float: DwAte;
```

### `DW_ATE_signed`

```rust
const DW_ATE_signed: DwAte;
```

### `DW_ATE_signed_char`

```rust
const DW_ATE_signed_char: DwAte;
```

### `DW_ATE_unsigned`

```rust
const DW_ATE_unsigned: DwAte;
```

### `DW_ATE_unsigned_char`

```rust
const DW_ATE_unsigned_char: DwAte;
```

### `DW_ATE_imaginary_float`

```rust
const DW_ATE_imaginary_float: DwAte;
```

### `DW_ATE_packed_decimal`

```rust
const DW_ATE_packed_decimal: DwAte;
```

### `DW_ATE_numeric_string`

```rust
const DW_ATE_numeric_string: DwAte;
```

### `DW_ATE_edited`

```rust
const DW_ATE_edited: DwAte;
```

### `DW_ATE_signed_fixed`

```rust
const DW_ATE_signed_fixed: DwAte;
```

### `DW_ATE_unsigned_fixed`

```rust
const DW_ATE_unsigned_fixed: DwAte;
```

### `DW_ATE_decimal_float`

```rust
const DW_ATE_decimal_float: DwAte;
```

### `DW_ATE_UTF`

```rust
const DW_ATE_UTF: DwAte;
```

### `DW_ATE_UCS`

```rust
const DW_ATE_UCS: DwAte;
```

### `DW_ATE_ASCII`

```rust
const DW_ATE_ASCII: DwAte;
```

### `DW_ATE_lo_user`

```rust
const DW_ATE_lo_user: DwAte;
```

### `DW_ATE_hi_user`

```rust
const DW_ATE_hi_user: DwAte;
```

### `DW_LLE_end_of_list`

```rust
const DW_LLE_end_of_list: DwLle;
```

### `DW_LLE_base_addressx`

```rust
const DW_LLE_base_addressx: DwLle;
```

### `DW_LLE_startx_endx`

```rust
const DW_LLE_startx_endx: DwLle;
```

### `DW_LLE_startx_length`

```rust
const DW_LLE_startx_length: DwLle;
```

### `DW_LLE_offset_pair`

```rust
const DW_LLE_offset_pair: DwLle;
```

### `DW_LLE_default_location`

```rust
const DW_LLE_default_location: DwLle;
```

### `DW_LLE_base_address`

```rust
const DW_LLE_base_address: DwLle;
```

### `DW_LLE_start_end`

```rust
const DW_LLE_start_end: DwLle;
```

### `DW_LLE_start_length`

```rust
const DW_LLE_start_length: DwLle;
```

### `DW_LLE_GNU_view_pair`

```rust
const DW_LLE_GNU_view_pair: DwLle;
```

### `DW_DS_unsigned`

```rust
const DW_DS_unsigned: DwDs;
```

### `DW_DS_leading_overpunch`

```rust
const DW_DS_leading_overpunch: DwDs;
```

### `DW_DS_trailing_overpunch`

```rust
const DW_DS_trailing_overpunch: DwDs;
```

### `DW_DS_leading_separate`

```rust
const DW_DS_leading_separate: DwDs;
```

### `DW_DS_trailing_separate`

```rust
const DW_DS_trailing_separate: DwDs;
```

### `DW_END_default`

```rust
const DW_END_default: DwEnd;
```

### `DW_END_big`

```rust
const DW_END_big: DwEnd;
```

### `DW_END_little`

```rust
const DW_END_little: DwEnd;
```

### `DW_END_lo_user`

```rust
const DW_END_lo_user: DwEnd;
```

### `DW_END_hi_user`

```rust
const DW_END_hi_user: DwEnd;
```

### `DW_ACCESS_public`

```rust
const DW_ACCESS_public: DwAccess;
```

### `DW_ACCESS_protected`

```rust
const DW_ACCESS_protected: DwAccess;
```

### `DW_ACCESS_private`

```rust
const DW_ACCESS_private: DwAccess;
```

### `DW_VIS_local`

```rust
const DW_VIS_local: DwVis;
```

### `DW_VIS_exported`

```rust
const DW_VIS_exported: DwVis;
```

### `DW_VIS_qualified`

```rust
const DW_VIS_qualified: DwVis;
```

### `DW_VIRTUALITY_none`

```rust
const DW_VIRTUALITY_none: DwVirtuality;
```

### `DW_VIRTUALITY_virtual`

```rust
const DW_VIRTUALITY_virtual: DwVirtuality;
```

### `DW_VIRTUALITY_pure_virtual`

```rust
const DW_VIRTUALITY_pure_virtual: DwVirtuality;
```

### `DW_LANG_C89`

```rust
const DW_LANG_C89: DwLang;
```

### `DW_LANG_C`

```rust
const DW_LANG_C: DwLang;
```

### `DW_LANG_Ada83`

```rust
const DW_LANG_Ada83: DwLang;
```

### `DW_LANG_C_plus_plus`

```rust
const DW_LANG_C_plus_plus: DwLang;
```

### `DW_LANG_Cobol74`

```rust
const DW_LANG_Cobol74: DwLang;
```

### `DW_LANG_Cobol85`

```rust
const DW_LANG_Cobol85: DwLang;
```

### `DW_LANG_Fortran77`

```rust
const DW_LANG_Fortran77: DwLang;
```

### `DW_LANG_Fortran90`

```rust
const DW_LANG_Fortran90: DwLang;
```

### `DW_LANG_Pascal83`

```rust
const DW_LANG_Pascal83: DwLang;
```

### `DW_LANG_Modula2`

```rust
const DW_LANG_Modula2: DwLang;
```

### `DW_LANG_Java`

```rust
const DW_LANG_Java: DwLang;
```

### `DW_LANG_C99`

```rust
const DW_LANG_C99: DwLang;
```

### `DW_LANG_Ada95`

```rust
const DW_LANG_Ada95: DwLang;
```

### `DW_LANG_Fortran95`

```rust
const DW_LANG_Fortran95: DwLang;
```

### `DW_LANG_PLI`

```rust
const DW_LANG_PLI: DwLang;
```

### `DW_LANG_ObjC`

```rust
const DW_LANG_ObjC: DwLang;
```

### `DW_LANG_ObjC_plus_plus`

```rust
const DW_LANG_ObjC_plus_plus: DwLang;
```

### `DW_LANG_UPC`

```rust
const DW_LANG_UPC: DwLang;
```

### `DW_LANG_D`

```rust
const DW_LANG_D: DwLang;
```

### `DW_LANG_Python`

```rust
const DW_LANG_Python: DwLang;
```

### `DW_LANG_OpenCL`

```rust
const DW_LANG_OpenCL: DwLang;
```

### `DW_LANG_Go`

```rust
const DW_LANG_Go: DwLang;
```

### `DW_LANG_Modula3`

```rust
const DW_LANG_Modula3: DwLang;
```

### `DW_LANG_Haskell`

```rust
const DW_LANG_Haskell: DwLang;
```

### `DW_LANG_C_plus_plus_03`

```rust
const DW_LANG_C_plus_plus_03: DwLang;
```

### `DW_LANG_C_plus_plus_11`

```rust
const DW_LANG_C_plus_plus_11: DwLang;
```

### `DW_LANG_OCaml`

```rust
const DW_LANG_OCaml: DwLang;
```

### `DW_LANG_Rust`

```rust
const DW_LANG_Rust: DwLang;
```

### `DW_LANG_C11`

```rust
const DW_LANG_C11: DwLang;
```

### `DW_LANG_Swift`

```rust
const DW_LANG_Swift: DwLang;
```

### `DW_LANG_Julia`

```rust
const DW_LANG_Julia: DwLang;
```

### `DW_LANG_Dylan`

```rust
const DW_LANG_Dylan: DwLang;
```

### `DW_LANG_C_plus_plus_14`

```rust
const DW_LANG_C_plus_plus_14: DwLang;
```

### `DW_LANG_Fortran03`

```rust
const DW_LANG_Fortran03: DwLang;
```

### `DW_LANG_Fortran08`

```rust
const DW_LANG_Fortran08: DwLang;
```

### `DW_LANG_RenderScript`

```rust
const DW_LANG_RenderScript: DwLang;
```

### `DW_LANG_BLISS`

```rust
const DW_LANG_BLISS: DwLang;
```

### `DW_LANG_Kotlin`

```rust
const DW_LANG_Kotlin: DwLang;
```

### `DW_LANG_Zig`

```rust
const DW_LANG_Zig: DwLang;
```

### `DW_LANG_Crystal`

```rust
const DW_LANG_Crystal: DwLang;
```

### `DW_LANG_C_plus_plus_17`

```rust
const DW_LANG_C_plus_plus_17: DwLang;
```

### `DW_LANG_C_plus_plus_20`

```rust
const DW_LANG_C_plus_plus_20: DwLang;
```

### `DW_LANG_C17`

```rust
const DW_LANG_C17: DwLang;
```

### `DW_LANG_Fortran18`

```rust
const DW_LANG_Fortran18: DwLang;
```

### `DW_LANG_Ada2005`

```rust
const DW_LANG_Ada2005: DwLang;
```

### `DW_LANG_Ada2012`

```rust
const DW_LANG_Ada2012: DwLang;
```

### `DW_LANG_lo_user`

```rust
const DW_LANG_lo_user: DwLang;
```

### `DW_LANG_hi_user`

```rust
const DW_LANG_hi_user: DwLang;
```

### `DW_LANG_Mips_Assembler`

```rust
const DW_LANG_Mips_Assembler: DwLang;
```

### `DW_LANG_GOOGLE_RenderScript`

```rust
const DW_LANG_GOOGLE_RenderScript: DwLang;
```

### `DW_LANG_SUN_Assembler`

```rust
const DW_LANG_SUN_Assembler: DwLang;
```

### `DW_LANG_ALTIUM_Assembler`

```rust
const DW_LANG_ALTIUM_Assembler: DwLang;
```

### `DW_LANG_BORLAND_Delphi`

```rust
const DW_LANG_BORLAND_Delphi: DwLang;
```

### `DW_ADDR_none`

```rust
const DW_ADDR_none: DwAddr;
```

### `DW_ID_case_sensitive`

```rust
const DW_ID_case_sensitive: DwId;
```

### `DW_ID_up_case`

```rust
const DW_ID_up_case: DwId;
```

### `DW_ID_down_case`

```rust
const DW_ID_down_case: DwId;
```

### `DW_ID_case_insensitive`

```rust
const DW_ID_case_insensitive: DwId;
```

### `DW_CC_normal`

```rust
const DW_CC_normal: DwCc;
```

### `DW_CC_program`

```rust
const DW_CC_program: DwCc;
```

### `DW_CC_nocall`

```rust
const DW_CC_nocall: DwCc;
```

### `DW_CC_pass_by_reference`

```rust
const DW_CC_pass_by_reference: DwCc;
```

### `DW_CC_pass_by_value`

```rust
const DW_CC_pass_by_value: DwCc;
```

### `DW_CC_lo_user`

```rust
const DW_CC_lo_user: DwCc;
```

### `DW_CC_hi_user`

```rust
const DW_CC_hi_user: DwCc;
```

### `DW_INL_not_inlined`

```rust
const DW_INL_not_inlined: DwInl;
```

### `DW_INL_inlined`

```rust
const DW_INL_inlined: DwInl;
```

### `DW_INL_declared_not_inlined`

```rust
const DW_INL_declared_not_inlined: DwInl;
```

### `DW_INL_declared_inlined`

```rust
const DW_INL_declared_inlined: DwInl;
```

### `DW_ORD_row_major`

```rust
const DW_ORD_row_major: DwOrd;
```

### `DW_ORD_col_major`

```rust
const DW_ORD_col_major: DwOrd;
```

### `DW_DSC_label`

```rust
const DW_DSC_label: DwDsc;
```

### `DW_DSC_range`

```rust
const DW_DSC_range: DwDsc;
```

### `DW_IDX_compile_unit`

```rust
const DW_IDX_compile_unit: DwIdx;
```

### `DW_IDX_type_unit`

```rust
const DW_IDX_type_unit: DwIdx;
```

### `DW_IDX_die_offset`

```rust
const DW_IDX_die_offset: DwIdx;
```

### `DW_IDX_parent`

```rust
const DW_IDX_parent: DwIdx;
```

### `DW_IDX_type_hash`

```rust
const DW_IDX_type_hash: DwIdx;
```

### `DW_IDX_lo_user`

```rust
const DW_IDX_lo_user: DwIdx;
```

### `DW_IDX_hi_user`

```rust
const DW_IDX_hi_user: DwIdx;
```

### `DW_DEFAULTED_no`

```rust
const DW_DEFAULTED_no: DwDefaulted;
```

### `DW_DEFAULTED_in_class`

```rust
const DW_DEFAULTED_in_class: DwDefaulted;
```

### `DW_DEFAULTED_out_of_class`

```rust
const DW_DEFAULTED_out_of_class: DwDefaulted;
```

### `DW_LNS_copy`

```rust
const DW_LNS_copy: DwLns;
```

### `DW_LNS_advance_pc`

```rust
const DW_LNS_advance_pc: DwLns;
```

### `DW_LNS_advance_line`

```rust
const DW_LNS_advance_line: DwLns;
```

### `DW_LNS_set_file`

```rust
const DW_LNS_set_file: DwLns;
```

### `DW_LNS_set_column`

```rust
const DW_LNS_set_column: DwLns;
```

### `DW_LNS_negate_stmt`

```rust
const DW_LNS_negate_stmt: DwLns;
```

### `DW_LNS_set_basic_block`

```rust
const DW_LNS_set_basic_block: DwLns;
```

### `DW_LNS_const_add_pc`

```rust
const DW_LNS_const_add_pc: DwLns;
```

### `DW_LNS_fixed_advance_pc`

```rust
const DW_LNS_fixed_advance_pc: DwLns;
```

### `DW_LNS_set_prologue_end`

```rust
const DW_LNS_set_prologue_end: DwLns;
```

### `DW_LNS_set_epilogue_begin`

```rust
const DW_LNS_set_epilogue_begin: DwLns;
```

### `DW_LNS_set_isa`

```rust
const DW_LNS_set_isa: DwLns;
```

### `DW_LNE_end_sequence`

```rust
const DW_LNE_end_sequence: DwLne;
```

### `DW_LNE_set_address`

```rust
const DW_LNE_set_address: DwLne;
```

### `DW_LNE_define_file`

```rust
const DW_LNE_define_file: DwLne;
```

### `DW_LNE_set_discriminator`

```rust
const DW_LNE_set_discriminator: DwLne;
```

### `DW_LNE_lo_user`

```rust
const DW_LNE_lo_user: DwLne;
```

### `DW_LNE_hi_user`

```rust
const DW_LNE_hi_user: DwLne;
```

### `DW_LNCT_path`

```rust
const DW_LNCT_path: DwLnct;
```

### `DW_LNCT_directory_index`

```rust
const DW_LNCT_directory_index: DwLnct;
```

### `DW_LNCT_timestamp`

```rust
const DW_LNCT_timestamp: DwLnct;
```

### `DW_LNCT_size`

```rust
const DW_LNCT_size: DwLnct;
```

### `DW_LNCT_MD5`

```rust
const DW_LNCT_MD5: DwLnct;
```

### `DW_LNCT_lo_user`

```rust
const DW_LNCT_lo_user: DwLnct;
```

### `DW_LNCT_LLVM_source`

```rust
const DW_LNCT_LLVM_source: DwLnct;
```

### `DW_LNCT_hi_user`

```rust
const DW_LNCT_hi_user: DwLnct;
```

### `DW_MACINFO_define`

```rust
const DW_MACINFO_define: DwMacinfo;
```

### `DW_MACINFO_undef`

```rust
const DW_MACINFO_undef: DwMacinfo;
```

### `DW_MACINFO_start_file`

```rust
const DW_MACINFO_start_file: DwMacinfo;
```

### `DW_MACINFO_end_file`

```rust
const DW_MACINFO_end_file: DwMacinfo;
```

### `DW_MACINFO_vendor_ext`

```rust
const DW_MACINFO_vendor_ext: DwMacinfo;
```

### `DW_MACRO_define`

```rust
const DW_MACRO_define: DwMacro;
```

### `DW_MACRO_undef`

```rust
const DW_MACRO_undef: DwMacro;
```

### `DW_MACRO_start_file`

```rust
const DW_MACRO_start_file: DwMacro;
```

### `DW_MACRO_end_file`

```rust
const DW_MACRO_end_file: DwMacro;
```

### `DW_MACRO_define_strp`

```rust
const DW_MACRO_define_strp: DwMacro;
```

### `DW_MACRO_undef_strp`

```rust
const DW_MACRO_undef_strp: DwMacro;
```

### `DW_MACRO_import`

```rust
const DW_MACRO_import: DwMacro;
```

### `DW_MACRO_define_sup`

```rust
const DW_MACRO_define_sup: DwMacro;
```

### `DW_MACRO_undef_sup`

```rust
const DW_MACRO_undef_sup: DwMacro;
```

### `DW_MACRO_import_sup`

```rust
const DW_MACRO_import_sup: DwMacro;
```

### `DW_MACRO_define_strx`

```rust
const DW_MACRO_define_strx: DwMacro;
```

### `DW_MACRO_undef_strx`

```rust
const DW_MACRO_undef_strx: DwMacro;
```

### `DW_MACRO_lo_user`

```rust
const DW_MACRO_lo_user: DwMacro;
```

### `DW_MACRO_hi_user`

```rust
const DW_MACRO_hi_user: DwMacro;
```

### `DW_RLE_end_of_list`

```rust
const DW_RLE_end_of_list: DwRle;
```

### `DW_RLE_base_addressx`

```rust
const DW_RLE_base_addressx: DwRle;
```

### `DW_RLE_startx_endx`

```rust
const DW_RLE_startx_endx: DwRle;
```

### `DW_RLE_startx_length`

```rust
const DW_RLE_startx_length: DwRle;
```

### `DW_RLE_offset_pair`

```rust
const DW_RLE_offset_pair: DwRle;
```

### `DW_RLE_base_address`

```rust
const DW_RLE_base_address: DwRle;
```

### `DW_RLE_start_end`

```rust
const DW_RLE_start_end: DwRle;
```

### `DW_RLE_start_length`

```rust
const DW_RLE_start_length: DwRle;
```

### `DW_OP_addr`

```rust
const DW_OP_addr: DwOp;
```

### `DW_OP_deref`

```rust
const DW_OP_deref: DwOp;
```

### `DW_OP_const1u`

```rust
const DW_OP_const1u: DwOp;
```

### `DW_OP_const1s`

```rust
const DW_OP_const1s: DwOp;
```

### `DW_OP_const2u`

```rust
const DW_OP_const2u: DwOp;
```

### `DW_OP_const2s`

```rust
const DW_OP_const2s: DwOp;
```

### `DW_OP_const4u`

```rust
const DW_OP_const4u: DwOp;
```

### `DW_OP_const4s`

```rust
const DW_OP_const4s: DwOp;
```

### `DW_OP_const8u`

```rust
const DW_OP_const8u: DwOp;
```

### `DW_OP_const8s`

```rust
const DW_OP_const8s: DwOp;
```

### `DW_OP_constu`

```rust
const DW_OP_constu: DwOp;
```

### `DW_OP_consts`

```rust
const DW_OP_consts: DwOp;
```

### `DW_OP_dup`

```rust
const DW_OP_dup: DwOp;
```

### `DW_OP_drop`

```rust
const DW_OP_drop: DwOp;
```

### `DW_OP_over`

```rust
const DW_OP_over: DwOp;
```

### `DW_OP_pick`

```rust
const DW_OP_pick: DwOp;
```

### `DW_OP_swap`

```rust
const DW_OP_swap: DwOp;
```

### `DW_OP_rot`

```rust
const DW_OP_rot: DwOp;
```

### `DW_OP_xderef`

```rust
const DW_OP_xderef: DwOp;
```

### `DW_OP_abs`

```rust
const DW_OP_abs: DwOp;
```

### `DW_OP_and`

```rust
const DW_OP_and: DwOp;
```

### `DW_OP_div`

```rust
const DW_OP_div: DwOp;
```

### `DW_OP_minus`

```rust
const DW_OP_minus: DwOp;
```

### `DW_OP_mod`

```rust
const DW_OP_mod: DwOp;
```

### `DW_OP_mul`

```rust
const DW_OP_mul: DwOp;
```

### `DW_OP_neg`

```rust
const DW_OP_neg: DwOp;
```

### `DW_OP_not`

```rust
const DW_OP_not: DwOp;
```

### `DW_OP_or`

```rust
const DW_OP_or: DwOp;
```

### `DW_OP_plus`

```rust
const DW_OP_plus: DwOp;
```

### `DW_OP_plus_uconst`

```rust
const DW_OP_plus_uconst: DwOp;
```

### `DW_OP_shl`

```rust
const DW_OP_shl: DwOp;
```

### `DW_OP_shr`

```rust
const DW_OP_shr: DwOp;
```

### `DW_OP_shra`

```rust
const DW_OP_shra: DwOp;
```

### `DW_OP_xor`

```rust
const DW_OP_xor: DwOp;
```

### `DW_OP_bra`

```rust
const DW_OP_bra: DwOp;
```

### `DW_OP_eq`

```rust
const DW_OP_eq: DwOp;
```

### `DW_OP_ge`

```rust
const DW_OP_ge: DwOp;
```

### `DW_OP_gt`

```rust
const DW_OP_gt: DwOp;
```

### `DW_OP_le`

```rust
const DW_OP_le: DwOp;
```

### `DW_OP_lt`

```rust
const DW_OP_lt: DwOp;
```

### `DW_OP_ne`

```rust
const DW_OP_ne: DwOp;
```

### `DW_OP_skip`

```rust
const DW_OP_skip: DwOp;
```

### `DW_OP_lit0`

```rust
const DW_OP_lit0: DwOp;
```

### `DW_OP_lit1`

```rust
const DW_OP_lit1: DwOp;
```

### `DW_OP_lit2`

```rust
const DW_OP_lit2: DwOp;
```

### `DW_OP_lit3`

```rust
const DW_OP_lit3: DwOp;
```

### `DW_OP_lit4`

```rust
const DW_OP_lit4: DwOp;
```

### `DW_OP_lit5`

```rust
const DW_OP_lit5: DwOp;
```

### `DW_OP_lit6`

```rust
const DW_OP_lit6: DwOp;
```

### `DW_OP_lit7`

```rust
const DW_OP_lit7: DwOp;
```

### `DW_OP_lit8`

```rust
const DW_OP_lit8: DwOp;
```

### `DW_OP_lit9`

```rust
const DW_OP_lit9: DwOp;
```

### `DW_OP_lit10`

```rust
const DW_OP_lit10: DwOp;
```

### `DW_OP_lit11`

```rust
const DW_OP_lit11: DwOp;
```

### `DW_OP_lit12`

```rust
const DW_OP_lit12: DwOp;
```

### `DW_OP_lit13`

```rust
const DW_OP_lit13: DwOp;
```

### `DW_OP_lit14`

```rust
const DW_OP_lit14: DwOp;
```

### `DW_OP_lit15`

```rust
const DW_OP_lit15: DwOp;
```

### `DW_OP_lit16`

```rust
const DW_OP_lit16: DwOp;
```

### `DW_OP_lit17`

```rust
const DW_OP_lit17: DwOp;
```

### `DW_OP_lit18`

```rust
const DW_OP_lit18: DwOp;
```

### `DW_OP_lit19`

```rust
const DW_OP_lit19: DwOp;
```

### `DW_OP_lit20`

```rust
const DW_OP_lit20: DwOp;
```

### `DW_OP_lit21`

```rust
const DW_OP_lit21: DwOp;
```

### `DW_OP_lit22`

```rust
const DW_OP_lit22: DwOp;
```

### `DW_OP_lit23`

```rust
const DW_OP_lit23: DwOp;
```

### `DW_OP_lit24`

```rust
const DW_OP_lit24: DwOp;
```

### `DW_OP_lit25`

```rust
const DW_OP_lit25: DwOp;
```

### `DW_OP_lit26`

```rust
const DW_OP_lit26: DwOp;
```

### `DW_OP_lit27`

```rust
const DW_OP_lit27: DwOp;
```

### `DW_OP_lit28`

```rust
const DW_OP_lit28: DwOp;
```

### `DW_OP_lit29`

```rust
const DW_OP_lit29: DwOp;
```

### `DW_OP_lit30`

```rust
const DW_OP_lit30: DwOp;
```

### `DW_OP_lit31`

```rust
const DW_OP_lit31: DwOp;
```

### `DW_OP_reg0`

```rust
const DW_OP_reg0: DwOp;
```

### `DW_OP_reg1`

```rust
const DW_OP_reg1: DwOp;
```

### `DW_OP_reg2`

```rust
const DW_OP_reg2: DwOp;
```

### `DW_OP_reg3`

```rust
const DW_OP_reg3: DwOp;
```

### `DW_OP_reg4`

```rust
const DW_OP_reg4: DwOp;
```

### `DW_OP_reg5`

```rust
const DW_OP_reg5: DwOp;
```

### `DW_OP_reg6`

```rust
const DW_OP_reg6: DwOp;
```

### `DW_OP_reg7`

```rust
const DW_OP_reg7: DwOp;
```

### `DW_OP_reg8`

```rust
const DW_OP_reg8: DwOp;
```

### `DW_OP_reg9`

```rust
const DW_OP_reg9: DwOp;
```

### `DW_OP_reg10`

```rust
const DW_OP_reg10: DwOp;
```

### `DW_OP_reg11`

```rust
const DW_OP_reg11: DwOp;
```

### `DW_OP_reg12`

```rust
const DW_OP_reg12: DwOp;
```

### `DW_OP_reg13`

```rust
const DW_OP_reg13: DwOp;
```

### `DW_OP_reg14`

```rust
const DW_OP_reg14: DwOp;
```

### `DW_OP_reg15`

```rust
const DW_OP_reg15: DwOp;
```

### `DW_OP_reg16`

```rust
const DW_OP_reg16: DwOp;
```

### `DW_OP_reg17`

```rust
const DW_OP_reg17: DwOp;
```

### `DW_OP_reg18`

```rust
const DW_OP_reg18: DwOp;
```

### `DW_OP_reg19`

```rust
const DW_OP_reg19: DwOp;
```

### `DW_OP_reg20`

```rust
const DW_OP_reg20: DwOp;
```

### `DW_OP_reg21`

```rust
const DW_OP_reg21: DwOp;
```

### `DW_OP_reg22`

```rust
const DW_OP_reg22: DwOp;
```

### `DW_OP_reg23`

```rust
const DW_OP_reg23: DwOp;
```

### `DW_OP_reg24`

```rust
const DW_OP_reg24: DwOp;
```

### `DW_OP_reg25`

```rust
const DW_OP_reg25: DwOp;
```

### `DW_OP_reg26`

```rust
const DW_OP_reg26: DwOp;
```

### `DW_OP_reg27`

```rust
const DW_OP_reg27: DwOp;
```

### `DW_OP_reg28`

```rust
const DW_OP_reg28: DwOp;
```

### `DW_OP_reg29`

```rust
const DW_OP_reg29: DwOp;
```

### `DW_OP_reg30`

```rust
const DW_OP_reg30: DwOp;
```

### `DW_OP_reg31`

```rust
const DW_OP_reg31: DwOp;
```

### `DW_OP_breg0`

```rust
const DW_OP_breg0: DwOp;
```

### `DW_OP_breg1`

```rust
const DW_OP_breg1: DwOp;
```

### `DW_OP_breg2`

```rust
const DW_OP_breg2: DwOp;
```

### `DW_OP_breg3`

```rust
const DW_OP_breg3: DwOp;
```

### `DW_OP_breg4`

```rust
const DW_OP_breg4: DwOp;
```

### `DW_OP_breg5`

```rust
const DW_OP_breg5: DwOp;
```

### `DW_OP_breg6`

```rust
const DW_OP_breg6: DwOp;
```

### `DW_OP_breg7`

```rust
const DW_OP_breg7: DwOp;
```

### `DW_OP_breg8`

```rust
const DW_OP_breg8: DwOp;
```

### `DW_OP_breg9`

```rust
const DW_OP_breg9: DwOp;
```

### `DW_OP_breg10`

```rust
const DW_OP_breg10: DwOp;
```

### `DW_OP_breg11`

```rust
const DW_OP_breg11: DwOp;
```

### `DW_OP_breg12`

```rust
const DW_OP_breg12: DwOp;
```

### `DW_OP_breg13`

```rust
const DW_OP_breg13: DwOp;
```

### `DW_OP_breg14`

```rust
const DW_OP_breg14: DwOp;
```

### `DW_OP_breg15`

```rust
const DW_OP_breg15: DwOp;
```

### `DW_OP_breg16`

```rust
const DW_OP_breg16: DwOp;
```

### `DW_OP_breg17`

```rust
const DW_OP_breg17: DwOp;
```

### `DW_OP_breg18`

```rust
const DW_OP_breg18: DwOp;
```

### `DW_OP_breg19`

```rust
const DW_OP_breg19: DwOp;
```

### `DW_OP_breg20`

```rust
const DW_OP_breg20: DwOp;
```

### `DW_OP_breg21`

```rust
const DW_OP_breg21: DwOp;
```

### `DW_OP_breg22`

```rust
const DW_OP_breg22: DwOp;
```

### `DW_OP_breg23`

```rust
const DW_OP_breg23: DwOp;
```

### `DW_OP_breg24`

```rust
const DW_OP_breg24: DwOp;
```

### `DW_OP_breg25`

```rust
const DW_OP_breg25: DwOp;
```

### `DW_OP_breg26`

```rust
const DW_OP_breg26: DwOp;
```

### `DW_OP_breg27`

```rust
const DW_OP_breg27: DwOp;
```

### `DW_OP_breg28`

```rust
const DW_OP_breg28: DwOp;
```

### `DW_OP_breg29`

```rust
const DW_OP_breg29: DwOp;
```

### `DW_OP_breg30`

```rust
const DW_OP_breg30: DwOp;
```

### `DW_OP_breg31`

```rust
const DW_OP_breg31: DwOp;
```

### `DW_OP_regx`

```rust
const DW_OP_regx: DwOp;
```

### `DW_OP_fbreg`

```rust
const DW_OP_fbreg: DwOp;
```

### `DW_OP_bregx`

```rust
const DW_OP_bregx: DwOp;
```

### `DW_OP_piece`

```rust
const DW_OP_piece: DwOp;
```

### `DW_OP_deref_size`

```rust
const DW_OP_deref_size: DwOp;
```

### `DW_OP_xderef_size`

```rust
const DW_OP_xderef_size: DwOp;
```

### `DW_OP_nop`

```rust
const DW_OP_nop: DwOp;
```

### `DW_OP_push_object_address`

```rust
const DW_OP_push_object_address: DwOp;
```

### `DW_OP_call2`

```rust
const DW_OP_call2: DwOp;
```

### `DW_OP_call4`

```rust
const DW_OP_call4: DwOp;
```

### `DW_OP_call_ref`

```rust
const DW_OP_call_ref: DwOp;
```

### `DW_OP_form_tls_address`

```rust
const DW_OP_form_tls_address: DwOp;
```

### `DW_OP_call_frame_cfa`

```rust
const DW_OP_call_frame_cfa: DwOp;
```

### `DW_OP_bit_piece`

```rust
const DW_OP_bit_piece: DwOp;
```

### `DW_OP_implicit_value`

```rust
const DW_OP_implicit_value: DwOp;
```

### `DW_OP_stack_value`

```rust
const DW_OP_stack_value: DwOp;
```

### `DW_OP_implicit_pointer`

```rust
const DW_OP_implicit_pointer: DwOp;
```

### `DW_OP_addrx`

```rust
const DW_OP_addrx: DwOp;
```

### `DW_OP_constx`

```rust
const DW_OP_constx: DwOp;
```

### `DW_OP_entry_value`

```rust
const DW_OP_entry_value: DwOp;
```

### `DW_OP_const_type`

```rust
const DW_OP_const_type: DwOp;
```

### `DW_OP_regval_type`

```rust
const DW_OP_regval_type: DwOp;
```

### `DW_OP_deref_type`

```rust
const DW_OP_deref_type: DwOp;
```

### `DW_OP_xderef_type`

```rust
const DW_OP_xderef_type: DwOp;
```

### `DW_OP_convert`

```rust
const DW_OP_convert: DwOp;
```

### `DW_OP_reinterpret`

```rust
const DW_OP_reinterpret: DwOp;
```

### `DW_OP_GNU_push_tls_address`

```rust
const DW_OP_GNU_push_tls_address: DwOp;
```

### `DW_OP_GNU_implicit_pointer`

```rust
const DW_OP_GNU_implicit_pointer: DwOp;
```

### `DW_OP_GNU_entry_value`

```rust
const DW_OP_GNU_entry_value: DwOp;
```

### `DW_OP_GNU_const_type`

```rust
const DW_OP_GNU_const_type: DwOp;
```

### `DW_OP_GNU_regval_type`

```rust
const DW_OP_GNU_regval_type: DwOp;
```

### `DW_OP_GNU_deref_type`

```rust
const DW_OP_GNU_deref_type: DwOp;
```

### `DW_OP_GNU_convert`

```rust
const DW_OP_GNU_convert: DwOp;
```

### `DW_OP_GNU_reinterpret`

```rust
const DW_OP_GNU_reinterpret: DwOp;
```

### `DW_OP_GNU_parameter_ref`

```rust
const DW_OP_GNU_parameter_ref: DwOp;
```

### `DW_OP_GNU_addr_index`

```rust
const DW_OP_GNU_addr_index: DwOp;
```

### `DW_OP_GNU_const_index`

```rust
const DW_OP_GNU_const_index: DwOp;
```

### `DW_OP_WASM_location`

```rust
const DW_OP_WASM_location: DwOp;
```

### `DW_EH_PE_uleb128`

```rust
const DW_EH_PE_uleb128: DwEhPe;
```

### `DW_EH_PE_udata2`

```rust
const DW_EH_PE_udata2: DwEhPe;
```

### `DW_EH_PE_udata4`

```rust
const DW_EH_PE_udata4: DwEhPe;
```

### `DW_EH_PE_udata8`

```rust
const DW_EH_PE_udata8: DwEhPe;
```

### `DW_EH_PE_sleb128`

```rust
const DW_EH_PE_sleb128: DwEhPe;
```

### `DW_EH_PE_sdata2`

```rust
const DW_EH_PE_sdata2: DwEhPe;
```

### `DW_EH_PE_sdata4`

```rust
const DW_EH_PE_sdata4: DwEhPe;
```

### `DW_EH_PE_sdata8`

```rust
const DW_EH_PE_sdata8: DwEhPe;
```

### `DW_EH_PE_pcrel`

```rust
const DW_EH_PE_pcrel: DwEhPe;
```

### `DW_EH_PE_textrel`

```rust
const DW_EH_PE_textrel: DwEhPe;
```

### `DW_EH_PE_datarel`

```rust
const DW_EH_PE_datarel: DwEhPe;
```

### `DW_EH_PE_funcrel`

```rust
const DW_EH_PE_funcrel: DwEhPe;
```

### `DW_EH_PE_aligned`

```rust
const DW_EH_PE_aligned: DwEhPe;
```

### `DW_EH_PE_indirect`

```rust
const DW_EH_PE_indirect: DwEhPe;
```

### `DW_EH_PE_absptr`

```rust
const DW_EH_PE_absptr: DwEhPe;
```

### `DW_EH_PE_omit`

```rust
const DW_EH_PE_omit: DwEhPe;
```

### `DW_EH_PE_FORMAT_MASK`

```rust
const DW_EH_PE_FORMAT_MASK: u8 = 15u8;
```

### `DW_EH_PE_APPLICATION_MASK`

```rust
const DW_EH_PE_APPLICATION_MASK: u8 = 112u8;
```

## Macros

### `registers!`

### `dw!`

