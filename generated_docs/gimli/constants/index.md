*[gimli](../index.md) / [constants](index.md)*

---

# Module `constants`

Constant definitions.

The DWARF spec's `DW_AT_*` type is represented as `struct DwAt(u16)`,
`DW_FORM_*` as `DwForm(u16)`, etc.

There are also exported const definitions for each constant.

## Contents

- [Structs](#structs)
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
- [Constants](#constants)
  - [`DW_SECT_INFO`](#dw-sect-info)
  - [`DW_SECT_ABBREV`](#dw-sect-abbrev)
  - [`DW_SECT_LINE`](#dw-sect-line)
  - [`DW_SECT_LOCLISTS`](#dw-sect-loclists)
  - [`DW_SECT_STR_OFFSETS`](#dw-sect-str-offsets)
  - [`DW_SECT_MACRO`](#dw-sect-macro)
  - [`DW_SECT_RNGLISTS`](#dw-sect-rnglists)
  - [`DW_SECT_V2_INFO`](#dw-sect-v2-info)
  - [`DW_SECT_V2_TYPES`](#dw-sect-v2-types)
  - [`DW_SECT_V2_ABBREV`](#dw-sect-v2-abbrev)
  - [`DW_SECT_V2_LINE`](#dw-sect-v2-line)
  - [`DW_SECT_V2_LOC`](#dw-sect-v2-loc)
  - [`DW_SECT_V2_STR_OFFSETS`](#dw-sect-v2-str-offsets)
  - [`DW_SECT_V2_MACINFO`](#dw-sect-v2-macinfo)
  - [`DW_SECT_V2_MACRO`](#dw-sect-v2-macro)
  - [`DW_UT_compile`](#dw-ut-compile)
  - [`DW_UT_type`](#dw-ut-type)
  - [`DW_UT_partial`](#dw-ut-partial)
  - [`DW_UT_skeleton`](#dw-ut-skeleton)
  - [`DW_UT_split_compile`](#dw-ut-split-compile)
  - [`DW_UT_split_type`](#dw-ut-split-type)
  - [`DW_UT_lo_user`](#dw-ut-lo-user)
  - [`DW_UT_hi_user`](#dw-ut-hi-user)
  - [`DW_CFA_advance_loc`](#dw-cfa-advance-loc)
  - [`DW_CFA_offset`](#dw-cfa-offset)
  - [`DW_CFA_restore`](#dw-cfa-restore)
  - [`DW_CFA_nop`](#dw-cfa-nop)
  - [`DW_CFA_set_loc`](#dw-cfa-set-loc)
  - [`DW_CFA_advance_loc1`](#dw-cfa-advance-loc1)
  - [`DW_CFA_advance_loc2`](#dw-cfa-advance-loc2)
  - [`DW_CFA_advance_loc4`](#dw-cfa-advance-loc4)
  - [`DW_CFA_offset_extended`](#dw-cfa-offset-extended)
  - [`DW_CFA_restore_extended`](#dw-cfa-restore-extended)
  - [`DW_CFA_undefined`](#dw-cfa-undefined)
  - [`DW_CFA_same_value`](#dw-cfa-same-value)
  - [`DW_CFA_register`](#dw-cfa-register)
  - [`DW_CFA_remember_state`](#dw-cfa-remember-state)
  - [`DW_CFA_restore_state`](#dw-cfa-restore-state)
  - [`DW_CFA_def_cfa`](#dw-cfa-def-cfa)
  - [`DW_CFA_def_cfa_register`](#dw-cfa-def-cfa-register)
  - [`DW_CFA_def_cfa_offset`](#dw-cfa-def-cfa-offset)
  - [`DW_CFA_def_cfa_expression`](#dw-cfa-def-cfa-expression)
  - [`DW_CFA_expression`](#dw-cfa-expression)
  - [`DW_CFA_offset_extended_sf`](#dw-cfa-offset-extended-sf)
  - [`DW_CFA_def_cfa_sf`](#dw-cfa-def-cfa-sf)
  - [`DW_CFA_def_cfa_offset_sf`](#dw-cfa-def-cfa-offset-sf)
  - [`DW_CFA_val_offset`](#dw-cfa-val-offset)
  - [`DW_CFA_val_offset_sf`](#dw-cfa-val-offset-sf)
  - [`DW_CFA_val_expression`](#dw-cfa-val-expression)
  - [`DW_CFA_lo_user`](#dw-cfa-lo-user)
  - [`DW_CFA_hi_user`](#dw-cfa-hi-user)
  - [`DW_CFA_MIPS_advance_loc8`](#dw-cfa-mips-advance-loc8)
  - [`DW_CFA_GNU_window_save`](#dw-cfa-gnu-window-save)
  - [`DW_CFA_GNU_args_size`](#dw-cfa-gnu-args-size)
  - [`DW_CFA_GNU_negative_offset_extended`](#dw-cfa-gnu-negative-offset-extended)
  - [`DW_CFA_AARCH64_negate_ra_state`](#dw-cfa-aarch64-negate-ra-state)
  - [`DW_CHILDREN_no`](#dw-children-no)
  - [`DW_CHILDREN_yes`](#dw-children-yes)
  - [`DW_TAG_null`](#dw-tag-null)
  - [`DW_TAG_global_subroutine`](#dw-tag-global-subroutine)
  - [`DW_TAG_global_variable`](#dw-tag-global-variable)
  - [`DW_TAG_local_variable`](#dw-tag-local-variable)
  - [`DW_TAG_subroutine`](#dw-tag-subroutine)
  - [`DW_TAG_array_type`](#dw-tag-array-type)
  - [`DW_TAG_class_type`](#dw-tag-class-type)
  - [`DW_TAG_entry_point`](#dw-tag-entry-point)
  - [`DW_TAG_enumeration_type`](#dw-tag-enumeration-type)
  - [`DW_TAG_formal_parameter`](#dw-tag-formal-parameter)
  - [`DW_TAG_imported_declaration`](#dw-tag-imported-declaration)
  - [`DW_TAG_label`](#dw-tag-label)
  - [`DW_TAG_lexical_block`](#dw-tag-lexical-block)
  - [`DW_TAG_member`](#dw-tag-member)
  - [`DW_TAG_pointer_type`](#dw-tag-pointer-type)
  - [`DW_TAG_reference_type`](#dw-tag-reference-type)
  - [`DW_TAG_compile_unit`](#dw-tag-compile-unit)
  - [`DW_TAG_string_type`](#dw-tag-string-type)
  - [`DW_TAG_structure_type`](#dw-tag-structure-type)
  - [`DW_TAG_subroutine_type`](#dw-tag-subroutine-type)
  - [`DW_TAG_typedef`](#dw-tag-typedef)
  - [`DW_TAG_union_type`](#dw-tag-union-type)
  - [`DW_TAG_unspecified_parameters`](#dw-tag-unspecified-parameters)
  - [`DW_TAG_variant`](#dw-tag-variant)
  - [`DW_TAG_common_block`](#dw-tag-common-block)
  - [`DW_TAG_common_inclusion`](#dw-tag-common-inclusion)
  - [`DW_TAG_inheritance`](#dw-tag-inheritance)
  - [`DW_TAG_inlined_subroutine`](#dw-tag-inlined-subroutine)
  - [`DW_TAG_module`](#dw-tag-module)
  - [`DW_TAG_ptr_to_member_type`](#dw-tag-ptr-to-member-type)
  - [`DW_TAG_set_type`](#dw-tag-set-type)
  - [`DW_TAG_subrange_type`](#dw-tag-subrange-type)
  - [`DW_TAG_with_stmt`](#dw-tag-with-stmt)
  - [`DW_TAG_access_declaration`](#dw-tag-access-declaration)
  - [`DW_TAG_base_type`](#dw-tag-base-type)
  - [`DW_TAG_catch_block`](#dw-tag-catch-block)
  - [`DW_TAG_const_type`](#dw-tag-const-type)
  - [`DW_TAG_constant`](#dw-tag-constant)
  - [`DW_TAG_enumerator`](#dw-tag-enumerator)
  - [`DW_TAG_file_type`](#dw-tag-file-type)
  - [`DW_TAG_friend`](#dw-tag-friend)
  - [`DW_TAG_namelist`](#dw-tag-namelist)
  - [`DW_TAG_namelist_item`](#dw-tag-namelist-item)
  - [`DW_TAG_packed_type`](#dw-tag-packed-type)
  - [`DW_TAG_subprogram`](#dw-tag-subprogram)
  - [`DW_TAG_template_type_parameter`](#dw-tag-template-type-parameter)
  - [`DW_TAG_template_value_parameter`](#dw-tag-template-value-parameter)
  - [`DW_TAG_thrown_type`](#dw-tag-thrown-type)
  - [`DW_TAG_try_block`](#dw-tag-try-block)
  - [`DW_TAG_variant_part`](#dw-tag-variant-part)
  - [`DW_TAG_variable`](#dw-tag-variable)
  - [`DW_TAG_volatile_type`](#dw-tag-volatile-type)
  - [`DW_TAG_dwarf_procedure`](#dw-tag-dwarf-procedure)
  - [`DW_TAG_restrict_type`](#dw-tag-restrict-type)
  - [`DW_TAG_interface_type`](#dw-tag-interface-type)
  - [`DW_TAG_namespace`](#dw-tag-namespace)
  - [`DW_TAG_imported_module`](#dw-tag-imported-module)
  - [`DW_TAG_unspecified_type`](#dw-tag-unspecified-type)
  - [`DW_TAG_partial_unit`](#dw-tag-partial-unit)
  - [`DW_TAG_imported_unit`](#dw-tag-imported-unit)
  - [`DW_TAG_condition`](#dw-tag-condition)
  - [`DW_TAG_shared_type`](#dw-tag-shared-type)
  - [`DW_TAG_type_unit`](#dw-tag-type-unit)
  - [`DW_TAG_rvalue_reference_type`](#dw-tag-rvalue-reference-type)
  - [`DW_TAG_template_alias`](#dw-tag-template-alias)
  - [`DW_TAG_coarray_type`](#dw-tag-coarray-type)
  - [`DW_TAG_generic_subrange`](#dw-tag-generic-subrange)
  - [`DW_TAG_dynamic_type`](#dw-tag-dynamic-type)
  - [`DW_TAG_atomic_type`](#dw-tag-atomic-type)
  - [`DW_TAG_call_site`](#dw-tag-call-site)
  - [`DW_TAG_call_site_parameter`](#dw-tag-call-site-parameter)
  - [`DW_TAG_skeleton_unit`](#dw-tag-skeleton-unit)
  - [`DW_TAG_immutable_type`](#dw-tag-immutable-type)
  - [`DW_TAG_lo_user`](#dw-tag-lo-user)
  - [`DW_TAG_hi_user`](#dw-tag-hi-user)
  - [`DW_TAG_MIPS_loop`](#dw-tag-mips-loop)
  - [`DW_TAG_HP_array_descriptor`](#dw-tag-hp-array-descriptor)
  - [`DW_TAG_HP_Bliss_field`](#dw-tag-hp-bliss-field)
  - [`DW_TAG_HP_Bliss_field_set`](#dw-tag-hp-bliss-field-set)
  - [`DW_TAG_format_label`](#dw-tag-format-label)
  - [`DW_TAG_function_template`](#dw-tag-function-template)
  - [`DW_TAG_class_template`](#dw-tag-class-template)
  - [`DW_TAG_GNU_BINCL`](#dw-tag-gnu-bincl)
  - [`DW_TAG_GNU_EINCL`](#dw-tag-gnu-eincl)
  - [`DW_TAG_GNU_template_template_param`](#dw-tag-gnu-template-template-param)
  - [`DW_TAG_GNU_template_parameter_pack`](#dw-tag-gnu-template-parameter-pack)
  - [`DW_TAG_GNU_formal_parameter_pack`](#dw-tag-gnu-formal-parameter-pack)
  - [`DW_TAG_GNU_call_site`](#dw-tag-gnu-call-site)
  - [`DW_TAG_GNU_call_site_parameter`](#dw-tag-gnu-call-site-parameter)
  - [`DW_TAG_APPLE_property`](#dw-tag-apple-property)
  - [`DW_TAG_SUN_function_template`](#dw-tag-sun-function-template)
  - [`DW_TAG_SUN_class_template`](#dw-tag-sun-class-template)
  - [`DW_TAG_SUN_struct_template`](#dw-tag-sun-struct-template)
  - [`DW_TAG_SUN_union_template`](#dw-tag-sun-union-template)
  - [`DW_TAG_SUN_indirect_inheritance`](#dw-tag-sun-indirect-inheritance)
  - [`DW_TAG_SUN_codeflags`](#dw-tag-sun-codeflags)
  - [`DW_TAG_SUN_memop_info`](#dw-tag-sun-memop-info)
  - [`DW_TAG_SUN_omp_child_func`](#dw-tag-sun-omp-child-func)
  - [`DW_TAG_SUN_rtti_descriptor`](#dw-tag-sun-rtti-descriptor)
  - [`DW_TAG_SUN_dtor_info`](#dw-tag-sun-dtor-info)
  - [`DW_TAG_SUN_dtor`](#dw-tag-sun-dtor)
  - [`DW_TAG_SUN_f90_interface`](#dw-tag-sun-f90-interface)
  - [`DW_TAG_SUN_fortran_vax_structure`](#dw-tag-sun-fortran-vax-structure)
  - [`DW_TAG_ALTIUM_circ_type`](#dw-tag-altium-circ-type)
  - [`DW_TAG_ALTIUM_mwa_circ_type`](#dw-tag-altium-mwa-circ-type)
  - [`DW_TAG_ALTIUM_rev_carry_type`](#dw-tag-altium-rev-carry-type)
  - [`DW_TAG_ALTIUM_rom`](#dw-tag-altium-rom)
  - [`DW_TAG_upc_shared_type`](#dw-tag-upc-shared-type)
  - [`DW_TAG_upc_strict_type`](#dw-tag-upc-strict-type)
  - [`DW_TAG_upc_relaxed_type`](#dw-tag-upc-relaxed-type)
  - [`DW_TAG_PGI_kanji_type`](#dw-tag-pgi-kanji-type)
  - [`DW_TAG_PGI_interface_block`](#dw-tag-pgi-interface-block)
  - [`DW_TAG_BORLAND_property`](#dw-tag-borland-property)
  - [`DW_TAG_BORLAND_Delphi_string`](#dw-tag-borland-delphi-string)
  - [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw-tag-borland-delphi-dynamic-array)
  - [`DW_TAG_BORLAND_Delphi_set`](#dw-tag-borland-delphi-set)
  - [`DW_TAG_BORLAND_Delphi_variant`](#dw-tag-borland-delphi-variant)
  - [`DW_AT_null`](#dw-at-null)
  - [`DW_AT_fund_type`](#dw-at-fund-type)
  - [`DW_AT_mod_fund_type`](#dw-at-mod-fund-type)
  - [`DW_AT_user_def_type`](#dw-at-user-def-type)
  - [`DW_AT_mod_u_d_type`](#dw-at-mod-u-d-type)
  - [`DW_AT_subscr_data`](#dw-at-subscr-data)
  - [`DW_AT_element_list`](#dw-at-element-list)
  - [`DW_AT_member`](#dw-at-member)
  - [`DW_AT_friends`](#dw-at-friends)
  - [`DW_AT_program`](#dw-at-program)
  - [`DW_AT_private`](#dw-at-private)
  - [`DW_AT_protected`](#dw-at-protected)
  - [`DW_AT_public`](#dw-at-public)
  - [`DW_AT_pure_virtual`](#dw-at-pure-virtual)
  - [`DW_AT_virtual`](#dw-at-virtual)
  - [`DW_AT_specification_v1`](#dw-at-specification-v1)
  - [`DW_AT_sibling`](#dw-at-sibling)
  - [`DW_AT_location`](#dw-at-location)
  - [`DW_AT_name`](#dw-at-name)
  - [`DW_AT_ordering`](#dw-at-ordering)
  - [`DW_AT_byte_size`](#dw-at-byte-size)
  - [`DW_AT_bit_offset`](#dw-at-bit-offset)
  - [`DW_AT_bit_size`](#dw-at-bit-size)
  - [`DW_AT_stmt_list`](#dw-at-stmt-list)
  - [`DW_AT_low_pc`](#dw-at-low-pc)
  - [`DW_AT_high_pc`](#dw-at-high-pc)
  - [`DW_AT_language`](#dw-at-language)
  - [`DW_AT_discr`](#dw-at-discr)
  - [`DW_AT_discr_value`](#dw-at-discr-value)
  - [`DW_AT_visibility`](#dw-at-visibility)
  - [`DW_AT_import`](#dw-at-import)
  - [`DW_AT_string_length`](#dw-at-string-length)
  - [`DW_AT_common_reference`](#dw-at-common-reference)
  - [`DW_AT_comp_dir`](#dw-at-comp-dir)
  - [`DW_AT_const_value`](#dw-at-const-value)
  - [`DW_AT_containing_type`](#dw-at-containing-type)
  - [`DW_AT_default_value`](#dw-at-default-value)
  - [`DW_AT_inline`](#dw-at-inline)
  - [`DW_AT_is_optional`](#dw-at-is-optional)
  - [`DW_AT_lower_bound`](#dw-at-lower-bound)
  - [`DW_AT_producer`](#dw-at-producer)
  - [`DW_AT_prototyped`](#dw-at-prototyped)
  - [`DW_AT_return_addr`](#dw-at-return-addr)
  - [`DW_AT_start_scope`](#dw-at-start-scope)
  - [`DW_AT_bit_stride`](#dw-at-bit-stride)
  - [`DW_AT_upper_bound`](#dw-at-upper-bound)
  - [`DW_AT_abstract_origin`](#dw-at-abstract-origin)
  - [`DW_AT_accessibility`](#dw-at-accessibility)
  - [`DW_AT_address_class`](#dw-at-address-class)
  - [`DW_AT_artificial`](#dw-at-artificial)
  - [`DW_AT_base_types`](#dw-at-base-types)
  - [`DW_AT_calling_convention`](#dw-at-calling-convention)
  - [`DW_AT_count`](#dw-at-count)
  - [`DW_AT_data_member_location`](#dw-at-data-member-location)
  - [`DW_AT_decl_column`](#dw-at-decl-column)
  - [`DW_AT_decl_file`](#dw-at-decl-file)
  - [`DW_AT_decl_line`](#dw-at-decl-line)
  - [`DW_AT_declaration`](#dw-at-declaration)
  - [`DW_AT_discr_list`](#dw-at-discr-list)
  - [`DW_AT_encoding`](#dw-at-encoding)
  - [`DW_AT_external`](#dw-at-external)
  - [`DW_AT_frame_base`](#dw-at-frame-base)
  - [`DW_AT_friend`](#dw-at-friend)
  - [`DW_AT_identifier_case`](#dw-at-identifier-case)
  - [`DW_AT_macro_info`](#dw-at-macro-info)
  - [`DW_AT_namelist_item`](#dw-at-namelist-item)
  - [`DW_AT_priority`](#dw-at-priority)
  - [`DW_AT_segment`](#dw-at-segment)
  - [`DW_AT_specification`](#dw-at-specification)
  - [`DW_AT_static_link`](#dw-at-static-link)
  - [`DW_AT_type`](#dw-at-type)
  - [`DW_AT_use_location`](#dw-at-use-location)
  - [`DW_AT_variable_parameter`](#dw-at-variable-parameter)
  - [`DW_AT_virtuality`](#dw-at-virtuality)
  - [`DW_AT_vtable_elem_location`](#dw-at-vtable-elem-location)
  - [`DW_AT_allocated`](#dw-at-allocated)
  - [`DW_AT_associated`](#dw-at-associated)
  - [`DW_AT_data_location`](#dw-at-data-location)
  - [`DW_AT_byte_stride`](#dw-at-byte-stride)
  - [`DW_AT_entry_pc`](#dw-at-entry-pc)
  - [`DW_AT_use_UTF8`](#dw-at-use-utf8)
  - [`DW_AT_extension`](#dw-at-extension)
  - [`DW_AT_ranges`](#dw-at-ranges)
  - [`DW_AT_trampoline`](#dw-at-trampoline)
  - [`DW_AT_call_column`](#dw-at-call-column)
  - [`DW_AT_call_file`](#dw-at-call-file)
  - [`DW_AT_call_line`](#dw-at-call-line)
  - [`DW_AT_description`](#dw-at-description)
  - [`DW_AT_binary_scale`](#dw-at-binary-scale)
  - [`DW_AT_decimal_scale`](#dw-at-decimal-scale)
  - [`DW_AT_small`](#dw-at-small)
  - [`DW_AT_decimal_sign`](#dw-at-decimal-sign)
  - [`DW_AT_digit_count`](#dw-at-digit-count)
  - [`DW_AT_picture_string`](#dw-at-picture-string)
  - [`DW_AT_mutable`](#dw-at-mutable)
  - [`DW_AT_threads_scaled`](#dw-at-threads-scaled)
  - [`DW_AT_explicit`](#dw-at-explicit)
  - [`DW_AT_object_pointer`](#dw-at-object-pointer)
  - [`DW_AT_endianity`](#dw-at-endianity)
  - [`DW_AT_elemental`](#dw-at-elemental)
  - [`DW_AT_pure`](#dw-at-pure)
  - [`DW_AT_recursive`](#dw-at-recursive)
  - [`DW_AT_signature`](#dw-at-signature)
  - [`DW_AT_main_subprogram`](#dw-at-main-subprogram)
  - [`DW_AT_data_bit_offset`](#dw-at-data-bit-offset)
  - [`DW_AT_const_expr`](#dw-at-const-expr)
  - [`DW_AT_enum_class`](#dw-at-enum-class)
  - [`DW_AT_linkage_name`](#dw-at-linkage-name)
  - [`DW_AT_string_length_bit_size`](#dw-at-string-length-bit-size)
  - [`DW_AT_string_length_byte_size`](#dw-at-string-length-byte-size)
  - [`DW_AT_rank`](#dw-at-rank)
  - [`DW_AT_str_offsets_base`](#dw-at-str-offsets-base)
  - [`DW_AT_addr_base`](#dw-at-addr-base)
  - [`DW_AT_rnglists_base`](#dw-at-rnglists-base)
  - [`DW_AT_dwo_name`](#dw-at-dwo-name)
  - [`DW_AT_reference`](#dw-at-reference)
  - [`DW_AT_rvalue_reference`](#dw-at-rvalue-reference)
  - [`DW_AT_macros`](#dw-at-macros)
  - [`DW_AT_call_all_calls`](#dw-at-call-all-calls)
  - [`DW_AT_call_all_source_calls`](#dw-at-call-all-source-calls)
  - [`DW_AT_call_all_tail_calls`](#dw-at-call-all-tail-calls)
  - [`DW_AT_call_return_pc`](#dw-at-call-return-pc)
  - [`DW_AT_call_value`](#dw-at-call-value)
  - [`DW_AT_call_origin`](#dw-at-call-origin)
  - [`DW_AT_call_parameter`](#dw-at-call-parameter)
  - [`DW_AT_call_pc`](#dw-at-call-pc)
  - [`DW_AT_call_tail_call`](#dw-at-call-tail-call)
  - [`DW_AT_call_target`](#dw-at-call-target)
  - [`DW_AT_call_target_clobbered`](#dw-at-call-target-clobbered)
  - [`DW_AT_call_data_location`](#dw-at-call-data-location)
  - [`DW_AT_call_data_value`](#dw-at-call-data-value)
  - [`DW_AT_noreturn`](#dw-at-noreturn)
  - [`DW_AT_alignment`](#dw-at-alignment)
  - [`DW_AT_export_symbols`](#dw-at-export-symbols)
  - [`DW_AT_deleted`](#dw-at-deleted)
  - [`DW_AT_defaulted`](#dw-at-defaulted)
  - [`DW_AT_loclists_base`](#dw-at-loclists-base)
  - [`DW_AT_lo_user`](#dw-at-lo-user)
  - [`DW_AT_hi_user`](#dw-at-hi-user)
  - [`DW_AT_MIPS_fde`](#dw-at-mips-fde)
  - [`DW_AT_MIPS_loop_begin`](#dw-at-mips-loop-begin)
  - [`DW_AT_MIPS_tail_loop_begin`](#dw-at-mips-tail-loop-begin)
  - [`DW_AT_MIPS_epilog_begin`](#dw-at-mips-epilog-begin)
  - [`DW_AT_MIPS_loop_unroll_factor`](#dw-at-mips-loop-unroll-factor)
  - [`DW_AT_MIPS_software_pipeline_depth`](#dw-at-mips-software-pipeline-depth)
  - [`DW_AT_MIPS_linkage_name`](#dw-at-mips-linkage-name)
  - [`DW_AT_MIPS_stride`](#dw-at-mips-stride)
  - [`DW_AT_MIPS_abstract_name`](#dw-at-mips-abstract-name)
  - [`DW_AT_MIPS_clone_origin`](#dw-at-mips-clone-origin)
  - [`DW_AT_MIPS_has_inlines`](#dw-at-mips-has-inlines)
  - [`DW_AT_MIPS_stride_byte`](#dw-at-mips-stride-byte)
  - [`DW_AT_MIPS_stride_elem`](#dw-at-mips-stride-elem)
  - [`DW_AT_MIPS_ptr_dopetype`](#dw-at-mips-ptr-dopetype)
  - [`DW_AT_MIPS_allocatable_dopetype`](#dw-at-mips-allocatable-dopetype)
  - [`DW_AT_MIPS_assumed_shape_dopetype`](#dw-at-mips-assumed-shape-dopetype)
  - [`DW_AT_MIPS_assumed_size`](#dw-at-mips-assumed-size)
  - [`DW_AT_INTEL_other_endian`](#dw-at-intel-other-endian)
  - [`DW_AT_sf_names`](#dw-at-sf-names)
  - [`DW_AT_src_info`](#dw-at-src-info)
  - [`DW_AT_mac_info`](#dw-at-mac-info)
  - [`DW_AT_src_coords`](#dw-at-src-coords)
  - [`DW_AT_body_begin`](#dw-at-body-begin)
  - [`DW_AT_body_end`](#dw-at-body-end)
  - [`DW_AT_GNU_vector`](#dw-at-gnu-vector)
  - [`DW_AT_GNU_guarded_by`](#dw-at-gnu-guarded-by)
  - [`DW_AT_GNU_pt_guarded_by`](#dw-at-gnu-pt-guarded-by)
  - [`DW_AT_GNU_guarded`](#dw-at-gnu-guarded)
  - [`DW_AT_GNU_pt_guarded`](#dw-at-gnu-pt-guarded)
  - [`DW_AT_GNU_locks_excluded`](#dw-at-gnu-locks-excluded)
  - [`DW_AT_GNU_exclusive_locks_required`](#dw-at-gnu-exclusive-locks-required)
  - [`DW_AT_GNU_shared_locks_required`](#dw-at-gnu-shared-locks-required)
  - [`DW_AT_GNU_odr_signature`](#dw-at-gnu-odr-signature)
  - [`DW_AT_GNU_template_name`](#dw-at-gnu-template-name)
  - [`DW_AT_GNU_call_site_value`](#dw-at-gnu-call-site-value)
  - [`DW_AT_GNU_call_site_data_value`](#dw-at-gnu-call-site-data-value)
  - [`DW_AT_GNU_call_site_target`](#dw-at-gnu-call-site-target)
  - [`DW_AT_GNU_call_site_target_clobbered`](#dw-at-gnu-call-site-target-clobbered)
  - [`DW_AT_GNU_tail_call`](#dw-at-gnu-tail-call)
  - [`DW_AT_GNU_all_tail_call_sites`](#dw-at-gnu-all-tail-call-sites)
  - [`DW_AT_GNU_all_call_sites`](#dw-at-gnu-all-call-sites)
  - [`DW_AT_GNU_all_source_call_sites`](#dw-at-gnu-all-source-call-sites)
  - [`DW_AT_GNU_macros`](#dw-at-gnu-macros)
  - [`DW_AT_GNU_deleted`](#dw-at-gnu-deleted)
  - [`DW_AT_GNU_dwo_name`](#dw-at-gnu-dwo-name)
  - [`DW_AT_GNU_dwo_id`](#dw-at-gnu-dwo-id)
  - [`DW_AT_GNU_ranges_base`](#dw-at-gnu-ranges-base)
  - [`DW_AT_GNU_addr_base`](#dw-at-gnu-addr-base)
  - [`DW_AT_GNU_pubnames`](#dw-at-gnu-pubnames)
  - [`DW_AT_GNU_pubtypes`](#dw-at-gnu-pubtypes)
  - [`DW_AT_GNU_discriminator`](#dw-at-gnu-discriminator)
  - [`DW_AT_GNU_locviews`](#dw-at-gnu-locviews)
  - [`DW_AT_GNU_entry_view`](#dw-at-gnu-entry-view)
  - [`DW_AT_SUN_template`](#dw-at-sun-template)
  - [`DW_AT_SUN_alignment`](#dw-at-sun-alignment)
  - [`DW_AT_SUN_vtable`](#dw-at-sun-vtable)
  - [`DW_AT_SUN_count_guarantee`](#dw-at-sun-count-guarantee)
  - [`DW_AT_SUN_command_line`](#dw-at-sun-command-line)
  - [`DW_AT_SUN_vbase`](#dw-at-sun-vbase)
  - [`DW_AT_SUN_compile_options`](#dw-at-sun-compile-options)
  - [`DW_AT_SUN_language`](#dw-at-sun-language)
  - [`DW_AT_SUN_browser_file`](#dw-at-sun-browser-file)
  - [`DW_AT_SUN_vtable_abi`](#dw-at-sun-vtable-abi)
  - [`DW_AT_SUN_func_offsets`](#dw-at-sun-func-offsets)
  - [`DW_AT_SUN_cf_kind`](#dw-at-sun-cf-kind)
  - [`DW_AT_SUN_vtable_index`](#dw-at-sun-vtable-index)
  - [`DW_AT_SUN_omp_tpriv_addr`](#dw-at-sun-omp-tpriv-addr)
  - [`DW_AT_SUN_omp_child_func`](#dw-at-sun-omp-child-func)
  - [`DW_AT_SUN_func_offset`](#dw-at-sun-func-offset)
  - [`DW_AT_SUN_memop_type_ref`](#dw-at-sun-memop-type-ref)
  - [`DW_AT_SUN_profile_id`](#dw-at-sun-profile-id)
  - [`DW_AT_SUN_memop_signature`](#dw-at-sun-memop-signature)
  - [`DW_AT_SUN_obj_dir`](#dw-at-sun-obj-dir)
  - [`DW_AT_SUN_obj_file`](#dw-at-sun-obj-file)
  - [`DW_AT_SUN_original_name`](#dw-at-sun-original-name)
  - [`DW_AT_SUN_hwcprof_signature`](#dw-at-sun-hwcprof-signature)
  - [`DW_AT_SUN_amd64_parmdump`](#dw-at-sun-amd64-parmdump)
  - [`DW_AT_SUN_part_link_name`](#dw-at-sun-part-link-name)
  - [`DW_AT_SUN_link_name`](#dw-at-sun-link-name)
  - [`DW_AT_SUN_pass_with_const`](#dw-at-sun-pass-with-const)
  - [`DW_AT_SUN_return_with_const`](#dw-at-sun-return-with-const)
  - [`DW_AT_SUN_import_by_name`](#dw-at-sun-import-by-name)
  - [`DW_AT_SUN_f90_pointer`](#dw-at-sun-f90-pointer)
  - [`DW_AT_SUN_pass_by_ref`](#dw-at-sun-pass-by-ref)
  - [`DW_AT_SUN_f90_allocatable`](#dw-at-sun-f90-allocatable)
  - [`DW_AT_SUN_f90_assumed_shape_array`](#dw-at-sun-f90-assumed-shape-array)
  - [`DW_AT_SUN_c_vla`](#dw-at-sun-c-vla)
  - [`DW_AT_SUN_return_value_ptr`](#dw-at-sun-return-value-ptr)
  - [`DW_AT_SUN_dtor_start`](#dw-at-sun-dtor-start)
  - [`DW_AT_SUN_dtor_length`](#dw-at-sun-dtor-length)
  - [`DW_AT_SUN_dtor_state_initial`](#dw-at-sun-dtor-state-initial)
  - [`DW_AT_SUN_dtor_state_final`](#dw-at-sun-dtor-state-final)
  - [`DW_AT_SUN_dtor_state_deltas`](#dw-at-sun-dtor-state-deltas)
  - [`DW_AT_SUN_import_by_lname`](#dw-at-sun-import-by-lname)
  - [`DW_AT_SUN_f90_use_only`](#dw-at-sun-f90-use-only)
  - [`DW_AT_SUN_namelist_spec`](#dw-at-sun-namelist-spec)
  - [`DW_AT_SUN_is_omp_child_func`](#dw-at-sun-is-omp-child-func)
  - [`DW_AT_SUN_fortran_main_alias`](#dw-at-sun-fortran-main-alias)
  - [`DW_AT_SUN_fortran_based`](#dw-at-sun-fortran-based)
  - [`DW_AT_ALTIUM_loclist`](#dw-at-altium-loclist)
  - [`DW_AT_use_GNAT_descriptive_type`](#dw-at-use-gnat-descriptive-type)
  - [`DW_AT_GNAT_descriptive_type`](#dw-at-gnat-descriptive-type)
  - [`DW_AT_GNU_numerator`](#dw-at-gnu-numerator)
  - [`DW_AT_GNU_denominator`](#dw-at-gnu-denominator)
  - [`DW_AT_GNU_bias`](#dw-at-gnu-bias)
  - [`DW_AT_upc_threads_scaled`](#dw-at-upc-threads-scaled)
  - [`DW_AT_PGI_lbase`](#dw-at-pgi-lbase)
  - [`DW_AT_PGI_soffset`](#dw-at-pgi-soffset)
  - [`DW_AT_PGI_lstride`](#dw-at-pgi-lstride)
  - [`DW_AT_BORLAND_property_read`](#dw-at-borland-property-read)
  - [`DW_AT_BORLAND_property_write`](#dw-at-borland-property-write)
  - [`DW_AT_BORLAND_property_implements`](#dw-at-borland-property-implements)
  - [`DW_AT_BORLAND_property_index`](#dw-at-borland-property-index)
  - [`DW_AT_BORLAND_property_default`](#dw-at-borland-property-default)
  - [`DW_AT_BORLAND_Delphi_unit`](#dw-at-borland-delphi-unit)
  - [`DW_AT_BORLAND_Delphi_class`](#dw-at-borland-delphi-class)
  - [`DW_AT_BORLAND_Delphi_record`](#dw-at-borland-delphi-record)
  - [`DW_AT_BORLAND_Delphi_metaclass`](#dw-at-borland-delphi-metaclass)
  - [`DW_AT_BORLAND_Delphi_constructor`](#dw-at-borland-delphi-constructor)
  - [`DW_AT_BORLAND_Delphi_destructor`](#dw-at-borland-delphi-destructor)
  - [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw-at-borland-delphi-anonymous-method)
  - [`DW_AT_BORLAND_Delphi_interface`](#dw-at-borland-delphi-interface)
  - [`DW_AT_BORLAND_Delphi_ABI`](#dw-at-borland-delphi-abi)
  - [`DW_AT_BORLAND_Delphi_return`](#dw-at-borland-delphi-return)
  - [`DW_AT_BORLAND_Delphi_frameptr`](#dw-at-borland-delphi-frameptr)
  - [`DW_AT_BORLAND_closure`](#dw-at-borland-closure)
  - [`DW_AT_LLVM_include_path`](#dw-at-llvm-include-path)
  - [`DW_AT_LLVM_config_macros`](#dw-at-llvm-config-macros)
  - [`DW_AT_LLVM_isysroot`](#dw-at-llvm-isysroot)
  - [`DW_AT_APPLE_optimized`](#dw-at-apple-optimized)
  - [`DW_AT_APPLE_flags`](#dw-at-apple-flags)
  - [`DW_AT_APPLE_isa`](#dw-at-apple-isa)
  - [`DW_AT_APPLE_block`](#dw-at-apple-block)
  - [`DW_AT_APPLE_major_runtime_vers`](#dw-at-apple-major-runtime-vers)
  - [`DW_AT_APPLE_runtime_class`](#dw-at-apple-runtime-class)
  - [`DW_AT_APPLE_omit_frame_ptr`](#dw-at-apple-omit-frame-ptr)
  - [`DW_AT_APPLE_property_name`](#dw-at-apple-property-name)
  - [`DW_AT_APPLE_property_getter`](#dw-at-apple-property-getter)
  - [`DW_AT_APPLE_property_setter`](#dw-at-apple-property-setter)
  - [`DW_AT_APPLE_property_attribute`](#dw-at-apple-property-attribute)
  - [`DW_AT_APPLE_objc_complete_type`](#dw-at-apple-objc-complete-type)
  - [`DW_AT_APPLE_property`](#dw-at-apple-property)
  - [`DW_FORM_null`](#dw-form-null)
  - [`DW_FORM_ref`](#dw-form-ref)
  - [`DW_FORM_addr`](#dw-form-addr)
  - [`DW_FORM_block2`](#dw-form-block2)
  - [`DW_FORM_block4`](#dw-form-block4)
  - [`DW_FORM_data2`](#dw-form-data2)
  - [`DW_FORM_data4`](#dw-form-data4)
  - [`DW_FORM_data8`](#dw-form-data8)
  - [`DW_FORM_string`](#dw-form-string)
  - [`DW_FORM_block`](#dw-form-block)
  - [`DW_FORM_block1`](#dw-form-block1)
  - [`DW_FORM_data1`](#dw-form-data1)
  - [`DW_FORM_flag`](#dw-form-flag)
  - [`DW_FORM_sdata`](#dw-form-sdata)
  - [`DW_FORM_strp`](#dw-form-strp)
  - [`DW_FORM_udata`](#dw-form-udata)
  - [`DW_FORM_ref_addr`](#dw-form-ref-addr)
  - [`DW_FORM_ref1`](#dw-form-ref1)
  - [`DW_FORM_ref2`](#dw-form-ref2)
  - [`DW_FORM_ref4`](#dw-form-ref4)
  - [`DW_FORM_ref8`](#dw-form-ref8)
  - [`DW_FORM_ref_udata`](#dw-form-ref-udata)
  - [`DW_FORM_indirect`](#dw-form-indirect)
  - [`DW_FORM_sec_offset`](#dw-form-sec-offset)
  - [`DW_FORM_exprloc`](#dw-form-exprloc)
  - [`DW_FORM_flag_present`](#dw-form-flag-present)
  - [`DW_FORM_ref_sig8`](#dw-form-ref-sig8)
  - [`DW_FORM_strx`](#dw-form-strx)
  - [`DW_FORM_addrx`](#dw-form-addrx)
  - [`DW_FORM_ref_sup4`](#dw-form-ref-sup4)
  - [`DW_FORM_strp_sup`](#dw-form-strp-sup)
  - [`DW_FORM_data16`](#dw-form-data16)
  - [`DW_FORM_line_strp`](#dw-form-line-strp)
  - [`DW_FORM_implicit_const`](#dw-form-implicit-const)
  - [`DW_FORM_loclistx`](#dw-form-loclistx)
  - [`DW_FORM_rnglistx`](#dw-form-rnglistx)
  - [`DW_FORM_ref_sup8`](#dw-form-ref-sup8)
  - [`DW_FORM_strx1`](#dw-form-strx1)
  - [`DW_FORM_strx2`](#dw-form-strx2)
  - [`DW_FORM_strx3`](#dw-form-strx3)
  - [`DW_FORM_strx4`](#dw-form-strx4)
  - [`DW_FORM_addrx1`](#dw-form-addrx1)
  - [`DW_FORM_addrx2`](#dw-form-addrx2)
  - [`DW_FORM_addrx3`](#dw-form-addrx3)
  - [`DW_FORM_addrx4`](#dw-form-addrx4)
  - [`DW_FORM_GNU_addr_index`](#dw-form-gnu-addr-index)
  - [`DW_FORM_GNU_str_index`](#dw-form-gnu-str-index)
  - [`DW_FORM_GNU_ref_alt`](#dw-form-gnu-ref-alt)
  - [`DW_FORM_GNU_strp_alt`](#dw-form-gnu-strp-alt)
  - [`DW_ATE_address`](#dw-ate-address)
  - [`DW_ATE_boolean`](#dw-ate-boolean)
  - [`DW_ATE_complex_float`](#dw-ate-complex-float)
  - [`DW_ATE_float`](#dw-ate-float)
  - [`DW_ATE_signed`](#dw-ate-signed)
  - [`DW_ATE_signed_char`](#dw-ate-signed-char)
  - [`DW_ATE_unsigned`](#dw-ate-unsigned)
  - [`DW_ATE_unsigned_char`](#dw-ate-unsigned-char)
  - [`DW_ATE_imaginary_float`](#dw-ate-imaginary-float)
  - [`DW_ATE_packed_decimal`](#dw-ate-packed-decimal)
  - [`DW_ATE_numeric_string`](#dw-ate-numeric-string)
  - [`DW_ATE_edited`](#dw-ate-edited)
  - [`DW_ATE_signed_fixed`](#dw-ate-signed-fixed)
  - [`DW_ATE_unsigned_fixed`](#dw-ate-unsigned-fixed)
  - [`DW_ATE_decimal_float`](#dw-ate-decimal-float)
  - [`DW_ATE_UTF`](#dw-ate-utf)
  - [`DW_ATE_UCS`](#dw-ate-ucs)
  - [`DW_ATE_ASCII`](#dw-ate-ascii)
  - [`DW_ATE_lo_user`](#dw-ate-lo-user)
  - [`DW_ATE_hi_user`](#dw-ate-hi-user)
  - [`DW_LLE_end_of_list`](#dw-lle-end-of-list)
  - [`DW_LLE_base_addressx`](#dw-lle-base-addressx)
  - [`DW_LLE_startx_endx`](#dw-lle-startx-endx)
  - [`DW_LLE_startx_length`](#dw-lle-startx-length)
  - [`DW_LLE_offset_pair`](#dw-lle-offset-pair)
  - [`DW_LLE_default_location`](#dw-lle-default-location)
  - [`DW_LLE_base_address`](#dw-lle-base-address)
  - [`DW_LLE_start_end`](#dw-lle-start-end)
  - [`DW_LLE_start_length`](#dw-lle-start-length)
  - [`DW_LLE_GNU_view_pair`](#dw-lle-gnu-view-pair)
  - [`DW_DS_unsigned`](#dw-ds-unsigned)
  - [`DW_DS_leading_overpunch`](#dw-ds-leading-overpunch)
  - [`DW_DS_trailing_overpunch`](#dw-ds-trailing-overpunch)
  - [`DW_DS_leading_separate`](#dw-ds-leading-separate)
  - [`DW_DS_trailing_separate`](#dw-ds-trailing-separate)
  - [`DW_END_default`](#dw-end-default)
  - [`DW_END_big`](#dw-end-big)
  - [`DW_END_little`](#dw-end-little)
  - [`DW_END_lo_user`](#dw-end-lo-user)
  - [`DW_END_hi_user`](#dw-end-hi-user)
  - [`DW_ACCESS_public`](#dw-access-public)
  - [`DW_ACCESS_protected`](#dw-access-protected)
  - [`DW_ACCESS_private`](#dw-access-private)
  - [`DW_VIS_local`](#dw-vis-local)
  - [`DW_VIS_exported`](#dw-vis-exported)
  - [`DW_VIS_qualified`](#dw-vis-qualified)
  - [`DW_VIRTUALITY_none`](#dw-virtuality-none)
  - [`DW_VIRTUALITY_virtual`](#dw-virtuality-virtual)
  - [`DW_VIRTUALITY_pure_virtual`](#dw-virtuality-pure-virtual)
  - [`DW_LANG_C89`](#dw-lang-c89)
  - [`DW_LANG_C`](#dw-lang-c)
  - [`DW_LANG_Ada83`](#dw-lang-ada83)
  - [`DW_LANG_C_plus_plus`](#dw-lang-c-plus-plus)
  - [`DW_LANG_Cobol74`](#dw-lang-cobol74)
  - [`DW_LANG_Cobol85`](#dw-lang-cobol85)
  - [`DW_LANG_Fortran77`](#dw-lang-fortran77)
  - [`DW_LANG_Fortran90`](#dw-lang-fortran90)
  - [`DW_LANG_Pascal83`](#dw-lang-pascal83)
  - [`DW_LANG_Modula2`](#dw-lang-modula2)
  - [`DW_LANG_Java`](#dw-lang-java)
  - [`DW_LANG_C99`](#dw-lang-c99)
  - [`DW_LANG_Ada95`](#dw-lang-ada95)
  - [`DW_LANG_Fortran95`](#dw-lang-fortran95)
  - [`DW_LANG_PLI`](#dw-lang-pli)
  - [`DW_LANG_ObjC`](#dw-lang-objc)
  - [`DW_LANG_ObjC_plus_plus`](#dw-lang-objc-plus-plus)
  - [`DW_LANG_UPC`](#dw-lang-upc)
  - [`DW_LANG_D`](#dw-lang-d)
  - [`DW_LANG_Python`](#dw-lang-python)
  - [`DW_LANG_OpenCL`](#dw-lang-opencl)
  - [`DW_LANG_Go`](#dw-lang-go)
  - [`DW_LANG_Modula3`](#dw-lang-modula3)
  - [`DW_LANG_Haskell`](#dw-lang-haskell)
  - [`DW_LANG_C_plus_plus_03`](#dw-lang-c-plus-plus-03)
  - [`DW_LANG_C_plus_plus_11`](#dw-lang-c-plus-plus-11)
  - [`DW_LANG_OCaml`](#dw-lang-ocaml)
  - [`DW_LANG_Rust`](#dw-lang-rust)
  - [`DW_LANG_C11`](#dw-lang-c11)
  - [`DW_LANG_Swift`](#dw-lang-swift)
  - [`DW_LANG_Julia`](#dw-lang-julia)
  - [`DW_LANG_Dylan`](#dw-lang-dylan)
  - [`DW_LANG_C_plus_plus_14`](#dw-lang-c-plus-plus-14)
  - [`DW_LANG_Fortran03`](#dw-lang-fortran03)
  - [`DW_LANG_Fortran08`](#dw-lang-fortran08)
  - [`DW_LANG_RenderScript`](#dw-lang-renderscript)
  - [`DW_LANG_BLISS`](#dw-lang-bliss)
  - [`DW_LANG_Kotlin`](#dw-lang-kotlin)
  - [`DW_LANG_Zig`](#dw-lang-zig)
  - [`DW_LANG_Crystal`](#dw-lang-crystal)
  - [`DW_LANG_C_plus_plus_17`](#dw-lang-c-plus-plus-17)
  - [`DW_LANG_C_plus_plus_20`](#dw-lang-c-plus-plus-20)
  - [`DW_LANG_C17`](#dw-lang-c17)
  - [`DW_LANG_Fortran18`](#dw-lang-fortran18)
  - [`DW_LANG_Ada2005`](#dw-lang-ada2005)
  - [`DW_LANG_Ada2012`](#dw-lang-ada2012)
  - [`DW_LANG_lo_user`](#dw-lang-lo-user)
  - [`DW_LANG_hi_user`](#dw-lang-hi-user)
  - [`DW_LANG_Mips_Assembler`](#dw-lang-mips-assembler)
  - [`DW_LANG_GOOGLE_RenderScript`](#dw-lang-google-renderscript)
  - [`DW_LANG_SUN_Assembler`](#dw-lang-sun-assembler)
  - [`DW_LANG_ALTIUM_Assembler`](#dw-lang-altium-assembler)
  - [`DW_LANG_BORLAND_Delphi`](#dw-lang-borland-delphi)
  - [`DW_ADDR_none`](#dw-addr-none)
  - [`DW_ID_case_sensitive`](#dw-id-case-sensitive)
  - [`DW_ID_up_case`](#dw-id-up-case)
  - [`DW_ID_down_case`](#dw-id-down-case)
  - [`DW_ID_case_insensitive`](#dw-id-case-insensitive)
  - [`DW_CC_normal`](#dw-cc-normal)
  - [`DW_CC_program`](#dw-cc-program)
  - [`DW_CC_nocall`](#dw-cc-nocall)
  - [`DW_CC_pass_by_reference`](#dw-cc-pass-by-reference)
  - [`DW_CC_pass_by_value`](#dw-cc-pass-by-value)
  - [`DW_CC_lo_user`](#dw-cc-lo-user)
  - [`DW_CC_hi_user`](#dw-cc-hi-user)
  - [`DW_INL_not_inlined`](#dw-inl-not-inlined)
  - [`DW_INL_inlined`](#dw-inl-inlined)
  - [`DW_INL_declared_not_inlined`](#dw-inl-declared-not-inlined)
  - [`DW_INL_declared_inlined`](#dw-inl-declared-inlined)
  - [`DW_ORD_row_major`](#dw-ord-row-major)
  - [`DW_ORD_col_major`](#dw-ord-col-major)
  - [`DW_DSC_label`](#dw-dsc-label)
  - [`DW_DSC_range`](#dw-dsc-range)
  - [`DW_IDX_compile_unit`](#dw-idx-compile-unit)
  - [`DW_IDX_type_unit`](#dw-idx-type-unit)
  - [`DW_IDX_die_offset`](#dw-idx-die-offset)
  - [`DW_IDX_parent`](#dw-idx-parent)
  - [`DW_IDX_type_hash`](#dw-idx-type-hash)
  - [`DW_IDX_lo_user`](#dw-idx-lo-user)
  - [`DW_IDX_hi_user`](#dw-idx-hi-user)
  - [`DW_DEFAULTED_no`](#dw-defaulted-no)
  - [`DW_DEFAULTED_in_class`](#dw-defaulted-in-class)
  - [`DW_DEFAULTED_out_of_class`](#dw-defaulted-out-of-class)
  - [`DW_LNS_copy`](#dw-lns-copy)
  - [`DW_LNS_advance_pc`](#dw-lns-advance-pc)
  - [`DW_LNS_advance_line`](#dw-lns-advance-line)
  - [`DW_LNS_set_file`](#dw-lns-set-file)
  - [`DW_LNS_set_column`](#dw-lns-set-column)
  - [`DW_LNS_negate_stmt`](#dw-lns-negate-stmt)
  - [`DW_LNS_set_basic_block`](#dw-lns-set-basic-block)
  - [`DW_LNS_const_add_pc`](#dw-lns-const-add-pc)
  - [`DW_LNS_fixed_advance_pc`](#dw-lns-fixed-advance-pc)
  - [`DW_LNS_set_prologue_end`](#dw-lns-set-prologue-end)
  - [`DW_LNS_set_epilogue_begin`](#dw-lns-set-epilogue-begin)
  - [`DW_LNS_set_isa`](#dw-lns-set-isa)
  - [`DW_LNE_end_sequence`](#dw-lne-end-sequence)
  - [`DW_LNE_set_address`](#dw-lne-set-address)
  - [`DW_LNE_define_file`](#dw-lne-define-file)
  - [`DW_LNE_set_discriminator`](#dw-lne-set-discriminator)
  - [`DW_LNE_lo_user`](#dw-lne-lo-user)
  - [`DW_LNE_hi_user`](#dw-lne-hi-user)
  - [`DW_LNCT_path`](#dw-lnct-path)
  - [`DW_LNCT_directory_index`](#dw-lnct-directory-index)
  - [`DW_LNCT_timestamp`](#dw-lnct-timestamp)
  - [`DW_LNCT_size`](#dw-lnct-size)
  - [`DW_LNCT_MD5`](#dw-lnct-md5)
  - [`DW_LNCT_lo_user`](#dw-lnct-lo-user)
  - [`DW_LNCT_LLVM_source`](#dw-lnct-llvm-source)
  - [`DW_LNCT_hi_user`](#dw-lnct-hi-user)
  - [`DW_MACINFO_define`](#dw-macinfo-define)
  - [`DW_MACINFO_undef`](#dw-macinfo-undef)
  - [`DW_MACINFO_start_file`](#dw-macinfo-start-file)
  - [`DW_MACINFO_end_file`](#dw-macinfo-end-file)
  - [`DW_MACINFO_vendor_ext`](#dw-macinfo-vendor-ext)
  - [`DW_MACRO_define`](#dw-macro-define)
  - [`DW_MACRO_undef`](#dw-macro-undef)
  - [`DW_MACRO_start_file`](#dw-macro-start-file)
  - [`DW_MACRO_end_file`](#dw-macro-end-file)
  - [`DW_MACRO_define_strp`](#dw-macro-define-strp)
  - [`DW_MACRO_undef_strp`](#dw-macro-undef-strp)
  - [`DW_MACRO_import`](#dw-macro-import)
  - [`DW_MACRO_define_sup`](#dw-macro-define-sup)
  - [`DW_MACRO_undef_sup`](#dw-macro-undef-sup)
  - [`DW_MACRO_import_sup`](#dw-macro-import-sup)
  - [`DW_MACRO_define_strx`](#dw-macro-define-strx)
  - [`DW_MACRO_undef_strx`](#dw-macro-undef-strx)
  - [`DW_MACRO_lo_user`](#dw-macro-lo-user)
  - [`DW_MACRO_hi_user`](#dw-macro-hi-user)
  - [`DW_RLE_end_of_list`](#dw-rle-end-of-list)
  - [`DW_RLE_base_addressx`](#dw-rle-base-addressx)
  - [`DW_RLE_startx_endx`](#dw-rle-startx-endx)
  - [`DW_RLE_startx_length`](#dw-rle-startx-length)
  - [`DW_RLE_offset_pair`](#dw-rle-offset-pair)
  - [`DW_RLE_base_address`](#dw-rle-base-address)
  - [`DW_RLE_start_end`](#dw-rle-start-end)
  - [`DW_RLE_start_length`](#dw-rle-start-length)
  - [`DW_OP_addr`](#dw-op-addr)
  - [`DW_OP_deref`](#dw-op-deref)
  - [`DW_OP_const1u`](#dw-op-const1u)
  - [`DW_OP_const1s`](#dw-op-const1s)
  - [`DW_OP_const2u`](#dw-op-const2u)
  - [`DW_OP_const2s`](#dw-op-const2s)
  - [`DW_OP_const4u`](#dw-op-const4u)
  - [`DW_OP_const4s`](#dw-op-const4s)
  - [`DW_OP_const8u`](#dw-op-const8u)
  - [`DW_OP_const8s`](#dw-op-const8s)
  - [`DW_OP_constu`](#dw-op-constu)
  - [`DW_OP_consts`](#dw-op-consts)
  - [`DW_OP_dup`](#dw-op-dup)
  - [`DW_OP_drop`](#dw-op-drop)
  - [`DW_OP_over`](#dw-op-over)
  - [`DW_OP_pick`](#dw-op-pick)
  - [`DW_OP_swap`](#dw-op-swap)
  - [`DW_OP_rot`](#dw-op-rot)
  - [`DW_OP_xderef`](#dw-op-xderef)
  - [`DW_OP_abs`](#dw-op-abs)
  - [`DW_OP_and`](#dw-op-and)
  - [`DW_OP_div`](#dw-op-div)
  - [`DW_OP_minus`](#dw-op-minus)
  - [`DW_OP_mod`](#dw-op-mod)
  - [`DW_OP_mul`](#dw-op-mul)
  - [`DW_OP_neg`](#dw-op-neg)
  - [`DW_OP_not`](#dw-op-not)
  - [`DW_OP_or`](#dw-op-or)
  - [`DW_OP_plus`](#dw-op-plus)
  - [`DW_OP_plus_uconst`](#dw-op-plus-uconst)
  - [`DW_OP_shl`](#dw-op-shl)
  - [`DW_OP_shr`](#dw-op-shr)
  - [`DW_OP_shra`](#dw-op-shra)
  - [`DW_OP_xor`](#dw-op-xor)
  - [`DW_OP_bra`](#dw-op-bra)
  - [`DW_OP_eq`](#dw-op-eq)
  - [`DW_OP_ge`](#dw-op-ge)
  - [`DW_OP_gt`](#dw-op-gt)
  - [`DW_OP_le`](#dw-op-le)
  - [`DW_OP_lt`](#dw-op-lt)
  - [`DW_OP_ne`](#dw-op-ne)
  - [`DW_OP_skip`](#dw-op-skip)
  - [`DW_OP_lit0`](#dw-op-lit0)
  - [`DW_OP_lit1`](#dw-op-lit1)
  - [`DW_OP_lit2`](#dw-op-lit2)
  - [`DW_OP_lit3`](#dw-op-lit3)
  - [`DW_OP_lit4`](#dw-op-lit4)
  - [`DW_OP_lit5`](#dw-op-lit5)
  - [`DW_OP_lit6`](#dw-op-lit6)
  - [`DW_OP_lit7`](#dw-op-lit7)
  - [`DW_OP_lit8`](#dw-op-lit8)
  - [`DW_OP_lit9`](#dw-op-lit9)
  - [`DW_OP_lit10`](#dw-op-lit10)
  - [`DW_OP_lit11`](#dw-op-lit11)
  - [`DW_OP_lit12`](#dw-op-lit12)
  - [`DW_OP_lit13`](#dw-op-lit13)
  - [`DW_OP_lit14`](#dw-op-lit14)
  - [`DW_OP_lit15`](#dw-op-lit15)
  - [`DW_OP_lit16`](#dw-op-lit16)
  - [`DW_OP_lit17`](#dw-op-lit17)
  - [`DW_OP_lit18`](#dw-op-lit18)
  - [`DW_OP_lit19`](#dw-op-lit19)
  - [`DW_OP_lit20`](#dw-op-lit20)
  - [`DW_OP_lit21`](#dw-op-lit21)
  - [`DW_OP_lit22`](#dw-op-lit22)
  - [`DW_OP_lit23`](#dw-op-lit23)
  - [`DW_OP_lit24`](#dw-op-lit24)
  - [`DW_OP_lit25`](#dw-op-lit25)
  - [`DW_OP_lit26`](#dw-op-lit26)
  - [`DW_OP_lit27`](#dw-op-lit27)
  - [`DW_OP_lit28`](#dw-op-lit28)
  - [`DW_OP_lit29`](#dw-op-lit29)
  - [`DW_OP_lit30`](#dw-op-lit30)
  - [`DW_OP_lit31`](#dw-op-lit31)
  - [`DW_OP_reg0`](#dw-op-reg0)
  - [`DW_OP_reg1`](#dw-op-reg1)
  - [`DW_OP_reg2`](#dw-op-reg2)
  - [`DW_OP_reg3`](#dw-op-reg3)
  - [`DW_OP_reg4`](#dw-op-reg4)
  - [`DW_OP_reg5`](#dw-op-reg5)
  - [`DW_OP_reg6`](#dw-op-reg6)
  - [`DW_OP_reg7`](#dw-op-reg7)
  - [`DW_OP_reg8`](#dw-op-reg8)
  - [`DW_OP_reg9`](#dw-op-reg9)
  - [`DW_OP_reg10`](#dw-op-reg10)
  - [`DW_OP_reg11`](#dw-op-reg11)
  - [`DW_OP_reg12`](#dw-op-reg12)
  - [`DW_OP_reg13`](#dw-op-reg13)
  - [`DW_OP_reg14`](#dw-op-reg14)
  - [`DW_OP_reg15`](#dw-op-reg15)
  - [`DW_OP_reg16`](#dw-op-reg16)
  - [`DW_OP_reg17`](#dw-op-reg17)
  - [`DW_OP_reg18`](#dw-op-reg18)
  - [`DW_OP_reg19`](#dw-op-reg19)
  - [`DW_OP_reg20`](#dw-op-reg20)
  - [`DW_OP_reg21`](#dw-op-reg21)
  - [`DW_OP_reg22`](#dw-op-reg22)
  - [`DW_OP_reg23`](#dw-op-reg23)
  - [`DW_OP_reg24`](#dw-op-reg24)
  - [`DW_OP_reg25`](#dw-op-reg25)
  - [`DW_OP_reg26`](#dw-op-reg26)
  - [`DW_OP_reg27`](#dw-op-reg27)
  - [`DW_OP_reg28`](#dw-op-reg28)
  - [`DW_OP_reg29`](#dw-op-reg29)
  - [`DW_OP_reg30`](#dw-op-reg30)
  - [`DW_OP_reg31`](#dw-op-reg31)
  - [`DW_OP_breg0`](#dw-op-breg0)
  - [`DW_OP_breg1`](#dw-op-breg1)
  - [`DW_OP_breg2`](#dw-op-breg2)
  - [`DW_OP_breg3`](#dw-op-breg3)
  - [`DW_OP_breg4`](#dw-op-breg4)
  - [`DW_OP_breg5`](#dw-op-breg5)
  - [`DW_OP_breg6`](#dw-op-breg6)
  - [`DW_OP_breg7`](#dw-op-breg7)
  - [`DW_OP_breg8`](#dw-op-breg8)
  - [`DW_OP_breg9`](#dw-op-breg9)
  - [`DW_OP_breg10`](#dw-op-breg10)
  - [`DW_OP_breg11`](#dw-op-breg11)
  - [`DW_OP_breg12`](#dw-op-breg12)
  - [`DW_OP_breg13`](#dw-op-breg13)
  - [`DW_OP_breg14`](#dw-op-breg14)
  - [`DW_OP_breg15`](#dw-op-breg15)
  - [`DW_OP_breg16`](#dw-op-breg16)
  - [`DW_OP_breg17`](#dw-op-breg17)
  - [`DW_OP_breg18`](#dw-op-breg18)
  - [`DW_OP_breg19`](#dw-op-breg19)
  - [`DW_OP_breg20`](#dw-op-breg20)
  - [`DW_OP_breg21`](#dw-op-breg21)
  - [`DW_OP_breg22`](#dw-op-breg22)
  - [`DW_OP_breg23`](#dw-op-breg23)
  - [`DW_OP_breg24`](#dw-op-breg24)
  - [`DW_OP_breg25`](#dw-op-breg25)
  - [`DW_OP_breg26`](#dw-op-breg26)
  - [`DW_OP_breg27`](#dw-op-breg27)
  - [`DW_OP_breg28`](#dw-op-breg28)
  - [`DW_OP_breg29`](#dw-op-breg29)
  - [`DW_OP_breg30`](#dw-op-breg30)
  - [`DW_OP_breg31`](#dw-op-breg31)
  - [`DW_OP_regx`](#dw-op-regx)
  - [`DW_OP_fbreg`](#dw-op-fbreg)
  - [`DW_OP_bregx`](#dw-op-bregx)
  - [`DW_OP_piece`](#dw-op-piece)
  - [`DW_OP_deref_size`](#dw-op-deref-size)
  - [`DW_OP_xderef_size`](#dw-op-xderef-size)
  - [`DW_OP_nop`](#dw-op-nop)
  - [`DW_OP_push_object_address`](#dw-op-push-object-address)
  - [`DW_OP_call2`](#dw-op-call2)
  - [`DW_OP_call4`](#dw-op-call4)
  - [`DW_OP_call_ref`](#dw-op-call-ref)
  - [`DW_OP_form_tls_address`](#dw-op-form-tls-address)
  - [`DW_OP_call_frame_cfa`](#dw-op-call-frame-cfa)
  - [`DW_OP_bit_piece`](#dw-op-bit-piece)
  - [`DW_OP_implicit_value`](#dw-op-implicit-value)
  - [`DW_OP_stack_value`](#dw-op-stack-value)
  - [`DW_OP_implicit_pointer`](#dw-op-implicit-pointer)
  - [`DW_OP_addrx`](#dw-op-addrx)
  - [`DW_OP_constx`](#dw-op-constx)
  - [`DW_OP_entry_value`](#dw-op-entry-value)
  - [`DW_OP_const_type`](#dw-op-const-type)
  - [`DW_OP_regval_type`](#dw-op-regval-type)
  - [`DW_OP_deref_type`](#dw-op-deref-type)
  - [`DW_OP_xderef_type`](#dw-op-xderef-type)
  - [`DW_OP_convert`](#dw-op-convert)
  - [`DW_OP_reinterpret`](#dw-op-reinterpret)
  - [`DW_OP_GNU_push_tls_address`](#dw-op-gnu-push-tls-address)
  - [`DW_OP_GNU_implicit_pointer`](#dw-op-gnu-implicit-pointer)
  - [`DW_OP_GNU_entry_value`](#dw-op-gnu-entry-value)
  - [`DW_OP_GNU_const_type`](#dw-op-gnu-const-type)
  - [`DW_OP_GNU_regval_type`](#dw-op-gnu-regval-type)
  - [`DW_OP_GNU_deref_type`](#dw-op-gnu-deref-type)
  - [`DW_OP_GNU_convert`](#dw-op-gnu-convert)
  - [`DW_OP_GNU_reinterpret`](#dw-op-gnu-reinterpret)
  - [`DW_OP_GNU_parameter_ref`](#dw-op-gnu-parameter-ref)
  - [`DW_OP_GNU_addr_index`](#dw-op-gnu-addr-index)
  - [`DW_OP_GNU_const_index`](#dw-op-gnu-const-index)
  - [`DW_OP_WASM_location`](#dw-op-wasm-location)
  - [`DW_EH_PE_uleb128`](#dw-eh-pe-uleb128)
  - [`DW_EH_PE_udata2`](#dw-eh-pe-udata2)
  - [`DW_EH_PE_udata4`](#dw-eh-pe-udata4)
  - [`DW_EH_PE_udata8`](#dw-eh-pe-udata8)
  - [`DW_EH_PE_sleb128`](#dw-eh-pe-sleb128)
  - [`DW_EH_PE_sdata2`](#dw-eh-pe-sdata2)
  - [`DW_EH_PE_sdata4`](#dw-eh-pe-sdata4)
  - [`DW_EH_PE_sdata8`](#dw-eh-pe-sdata8)
  - [`DW_EH_PE_pcrel`](#dw-eh-pe-pcrel)
  - [`DW_EH_PE_textrel`](#dw-eh-pe-textrel)
  - [`DW_EH_PE_datarel`](#dw-eh-pe-datarel)
  - [`DW_EH_PE_funcrel`](#dw-eh-pe-funcrel)
  - [`DW_EH_PE_aligned`](#dw-eh-pe-aligned)
  - [`DW_EH_PE_indirect`](#dw-eh-pe-indirect)
  - [`DW_EH_PE_absptr`](#dw-eh-pe-absptr)
  - [`DW_EH_PE_omit`](#dw-eh-pe-omit)
  - [`DW_EH_PE_FORMAT_MASK`](#dw-eh-pe-format-mask)
  - [`DW_EH_PE_APPLICATION_MASK`](#dw-eh-pe-application-mask)
- [Macros](#macros)
  - [`dw!`](#dw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
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
| [`DW_SECT_INFO`](#dw-sect-info) | const |  |
| [`DW_SECT_ABBREV`](#dw-sect-abbrev) | const |  |
| [`DW_SECT_LINE`](#dw-sect-line) | const |  |
| [`DW_SECT_LOCLISTS`](#dw-sect-loclists) | const |  |
| [`DW_SECT_STR_OFFSETS`](#dw-sect-str-offsets) | const |  |
| [`DW_SECT_MACRO`](#dw-sect-macro) | const |  |
| [`DW_SECT_RNGLISTS`](#dw-sect-rnglists) | const |  |
| [`DW_SECT_V2_INFO`](#dw-sect-v2-info) | const |  |
| [`DW_SECT_V2_TYPES`](#dw-sect-v2-types) | const |  |
| [`DW_SECT_V2_ABBREV`](#dw-sect-v2-abbrev) | const |  |
| [`DW_SECT_V2_LINE`](#dw-sect-v2-line) | const |  |
| [`DW_SECT_V2_LOC`](#dw-sect-v2-loc) | const |  |
| [`DW_SECT_V2_STR_OFFSETS`](#dw-sect-v2-str-offsets) | const |  |
| [`DW_SECT_V2_MACINFO`](#dw-sect-v2-macinfo) | const |  |
| [`DW_SECT_V2_MACRO`](#dw-sect-v2-macro) | const |  |
| [`DW_UT_compile`](#dw-ut-compile) | const |  |
| [`DW_UT_type`](#dw-ut-type) | const |  |
| [`DW_UT_partial`](#dw-ut-partial) | const |  |
| [`DW_UT_skeleton`](#dw-ut-skeleton) | const |  |
| [`DW_UT_split_compile`](#dw-ut-split-compile) | const |  |
| [`DW_UT_split_type`](#dw-ut-split-type) | const |  |
| [`DW_UT_lo_user`](#dw-ut-lo-user) | const |  |
| [`DW_UT_hi_user`](#dw-ut-hi-user) | const |  |
| [`DW_CFA_advance_loc`](#dw-cfa-advance-loc) | const |  |
| [`DW_CFA_offset`](#dw-cfa-offset) | const |  |
| [`DW_CFA_restore`](#dw-cfa-restore) | const |  |
| [`DW_CFA_nop`](#dw-cfa-nop) | const |  |
| [`DW_CFA_set_loc`](#dw-cfa-set-loc) | const |  |
| [`DW_CFA_advance_loc1`](#dw-cfa-advance-loc1) | const |  |
| [`DW_CFA_advance_loc2`](#dw-cfa-advance-loc2) | const |  |
| [`DW_CFA_advance_loc4`](#dw-cfa-advance-loc4) | const |  |
| [`DW_CFA_offset_extended`](#dw-cfa-offset-extended) | const |  |
| [`DW_CFA_restore_extended`](#dw-cfa-restore-extended) | const |  |
| [`DW_CFA_undefined`](#dw-cfa-undefined) | const |  |
| [`DW_CFA_same_value`](#dw-cfa-same-value) | const |  |
| [`DW_CFA_register`](#dw-cfa-register) | const |  |
| [`DW_CFA_remember_state`](#dw-cfa-remember-state) | const |  |
| [`DW_CFA_restore_state`](#dw-cfa-restore-state) | const |  |
| [`DW_CFA_def_cfa`](#dw-cfa-def-cfa) | const |  |
| [`DW_CFA_def_cfa_register`](#dw-cfa-def-cfa-register) | const |  |
| [`DW_CFA_def_cfa_offset`](#dw-cfa-def-cfa-offset) | const |  |
| [`DW_CFA_def_cfa_expression`](#dw-cfa-def-cfa-expression) | const |  |
| [`DW_CFA_expression`](#dw-cfa-expression) | const |  |
| [`DW_CFA_offset_extended_sf`](#dw-cfa-offset-extended-sf) | const |  |
| [`DW_CFA_def_cfa_sf`](#dw-cfa-def-cfa-sf) | const |  |
| [`DW_CFA_def_cfa_offset_sf`](#dw-cfa-def-cfa-offset-sf) | const |  |
| [`DW_CFA_val_offset`](#dw-cfa-val-offset) | const |  |
| [`DW_CFA_val_offset_sf`](#dw-cfa-val-offset-sf) | const |  |
| [`DW_CFA_val_expression`](#dw-cfa-val-expression) | const |  |
| [`DW_CFA_lo_user`](#dw-cfa-lo-user) | const |  |
| [`DW_CFA_hi_user`](#dw-cfa-hi-user) | const |  |
| [`DW_CFA_MIPS_advance_loc8`](#dw-cfa-mips-advance-loc8) | const |  |
| [`DW_CFA_GNU_window_save`](#dw-cfa-gnu-window-save) | const |  |
| [`DW_CFA_GNU_args_size`](#dw-cfa-gnu-args-size) | const |  |
| [`DW_CFA_GNU_negative_offset_extended`](#dw-cfa-gnu-negative-offset-extended) | const |  |
| [`DW_CFA_AARCH64_negate_ra_state`](#dw-cfa-aarch64-negate-ra-state) | const |  |
| [`DW_CHILDREN_no`](#dw-children-no) | const |  |
| [`DW_CHILDREN_yes`](#dw-children-yes) | const |  |
| [`DW_TAG_null`](#dw-tag-null) | const |  |
| [`DW_TAG_global_subroutine`](#dw-tag-global-subroutine) | const |  |
| [`DW_TAG_global_variable`](#dw-tag-global-variable) | const |  |
| [`DW_TAG_local_variable`](#dw-tag-local-variable) | const |  |
| [`DW_TAG_subroutine`](#dw-tag-subroutine) | const |  |
| [`DW_TAG_array_type`](#dw-tag-array-type) | const |  |
| [`DW_TAG_class_type`](#dw-tag-class-type) | const |  |
| [`DW_TAG_entry_point`](#dw-tag-entry-point) | const |  |
| [`DW_TAG_enumeration_type`](#dw-tag-enumeration-type) | const |  |
| [`DW_TAG_formal_parameter`](#dw-tag-formal-parameter) | const |  |
| [`DW_TAG_imported_declaration`](#dw-tag-imported-declaration) | const |  |
| [`DW_TAG_label`](#dw-tag-label) | const |  |
| [`DW_TAG_lexical_block`](#dw-tag-lexical-block) | const |  |
| [`DW_TAG_member`](#dw-tag-member) | const |  |
| [`DW_TAG_pointer_type`](#dw-tag-pointer-type) | const |  |
| [`DW_TAG_reference_type`](#dw-tag-reference-type) | const |  |
| [`DW_TAG_compile_unit`](#dw-tag-compile-unit) | const |  |
| [`DW_TAG_string_type`](#dw-tag-string-type) | const |  |
| [`DW_TAG_structure_type`](#dw-tag-structure-type) | const |  |
| [`DW_TAG_subroutine_type`](#dw-tag-subroutine-type) | const |  |
| [`DW_TAG_typedef`](#dw-tag-typedef) | const |  |
| [`DW_TAG_union_type`](#dw-tag-union-type) | const |  |
| [`DW_TAG_unspecified_parameters`](#dw-tag-unspecified-parameters) | const |  |
| [`DW_TAG_variant`](#dw-tag-variant) | const |  |
| [`DW_TAG_common_block`](#dw-tag-common-block) | const |  |
| [`DW_TAG_common_inclusion`](#dw-tag-common-inclusion) | const |  |
| [`DW_TAG_inheritance`](#dw-tag-inheritance) | const |  |
| [`DW_TAG_inlined_subroutine`](#dw-tag-inlined-subroutine) | const |  |
| [`DW_TAG_module`](#dw-tag-module) | const |  |
| [`DW_TAG_ptr_to_member_type`](#dw-tag-ptr-to-member-type) | const |  |
| [`DW_TAG_set_type`](#dw-tag-set-type) | const |  |
| [`DW_TAG_subrange_type`](#dw-tag-subrange-type) | const |  |
| [`DW_TAG_with_stmt`](#dw-tag-with-stmt) | const |  |
| [`DW_TAG_access_declaration`](#dw-tag-access-declaration) | const |  |
| [`DW_TAG_base_type`](#dw-tag-base-type) | const |  |
| [`DW_TAG_catch_block`](#dw-tag-catch-block) | const |  |
| [`DW_TAG_const_type`](#dw-tag-const-type) | const |  |
| [`DW_TAG_constant`](#dw-tag-constant) | const |  |
| [`DW_TAG_enumerator`](#dw-tag-enumerator) | const |  |
| [`DW_TAG_file_type`](#dw-tag-file-type) | const |  |
| [`DW_TAG_friend`](#dw-tag-friend) | const |  |
| [`DW_TAG_namelist`](#dw-tag-namelist) | const |  |
| [`DW_TAG_namelist_item`](#dw-tag-namelist-item) | const |  |
| [`DW_TAG_packed_type`](#dw-tag-packed-type) | const |  |
| [`DW_TAG_subprogram`](#dw-tag-subprogram) | const |  |
| [`DW_TAG_template_type_parameter`](#dw-tag-template-type-parameter) | const |  |
| [`DW_TAG_template_value_parameter`](#dw-tag-template-value-parameter) | const |  |
| [`DW_TAG_thrown_type`](#dw-tag-thrown-type) | const |  |
| [`DW_TAG_try_block`](#dw-tag-try-block) | const |  |
| [`DW_TAG_variant_part`](#dw-tag-variant-part) | const |  |
| [`DW_TAG_variable`](#dw-tag-variable) | const |  |
| [`DW_TAG_volatile_type`](#dw-tag-volatile-type) | const |  |
| [`DW_TAG_dwarf_procedure`](#dw-tag-dwarf-procedure) | const |  |
| [`DW_TAG_restrict_type`](#dw-tag-restrict-type) | const |  |
| [`DW_TAG_interface_type`](#dw-tag-interface-type) | const |  |
| [`DW_TAG_namespace`](#dw-tag-namespace) | const |  |
| [`DW_TAG_imported_module`](#dw-tag-imported-module) | const |  |
| [`DW_TAG_unspecified_type`](#dw-tag-unspecified-type) | const |  |
| [`DW_TAG_partial_unit`](#dw-tag-partial-unit) | const |  |
| [`DW_TAG_imported_unit`](#dw-tag-imported-unit) | const |  |
| [`DW_TAG_condition`](#dw-tag-condition) | const |  |
| [`DW_TAG_shared_type`](#dw-tag-shared-type) | const |  |
| [`DW_TAG_type_unit`](#dw-tag-type-unit) | const |  |
| [`DW_TAG_rvalue_reference_type`](#dw-tag-rvalue-reference-type) | const |  |
| [`DW_TAG_template_alias`](#dw-tag-template-alias) | const |  |
| [`DW_TAG_coarray_type`](#dw-tag-coarray-type) | const |  |
| [`DW_TAG_generic_subrange`](#dw-tag-generic-subrange) | const |  |
| [`DW_TAG_dynamic_type`](#dw-tag-dynamic-type) | const |  |
| [`DW_TAG_atomic_type`](#dw-tag-atomic-type) | const |  |
| [`DW_TAG_call_site`](#dw-tag-call-site) | const |  |
| [`DW_TAG_call_site_parameter`](#dw-tag-call-site-parameter) | const |  |
| [`DW_TAG_skeleton_unit`](#dw-tag-skeleton-unit) | const |  |
| [`DW_TAG_immutable_type`](#dw-tag-immutable-type) | const |  |
| [`DW_TAG_lo_user`](#dw-tag-lo-user) | const |  |
| [`DW_TAG_hi_user`](#dw-tag-hi-user) | const |  |
| [`DW_TAG_MIPS_loop`](#dw-tag-mips-loop) | const |  |
| [`DW_TAG_HP_array_descriptor`](#dw-tag-hp-array-descriptor) | const |  |
| [`DW_TAG_HP_Bliss_field`](#dw-tag-hp-bliss-field) | const |  |
| [`DW_TAG_HP_Bliss_field_set`](#dw-tag-hp-bliss-field-set) | const |  |
| [`DW_TAG_format_label`](#dw-tag-format-label) | const |  |
| [`DW_TAG_function_template`](#dw-tag-function-template) | const |  |
| [`DW_TAG_class_template`](#dw-tag-class-template) | const |  |
| [`DW_TAG_GNU_BINCL`](#dw-tag-gnu-bincl) | const |  |
| [`DW_TAG_GNU_EINCL`](#dw-tag-gnu-eincl) | const |  |
| [`DW_TAG_GNU_template_template_param`](#dw-tag-gnu-template-template-param) | const |  |
| [`DW_TAG_GNU_template_parameter_pack`](#dw-tag-gnu-template-parameter-pack) | const |  |
| [`DW_TAG_GNU_formal_parameter_pack`](#dw-tag-gnu-formal-parameter-pack) | const |  |
| [`DW_TAG_GNU_call_site`](#dw-tag-gnu-call-site) | const |  |
| [`DW_TAG_GNU_call_site_parameter`](#dw-tag-gnu-call-site-parameter) | const |  |
| [`DW_TAG_APPLE_property`](#dw-tag-apple-property) | const |  |
| [`DW_TAG_SUN_function_template`](#dw-tag-sun-function-template) | const |  |
| [`DW_TAG_SUN_class_template`](#dw-tag-sun-class-template) | const |  |
| [`DW_TAG_SUN_struct_template`](#dw-tag-sun-struct-template) | const |  |
| [`DW_TAG_SUN_union_template`](#dw-tag-sun-union-template) | const |  |
| [`DW_TAG_SUN_indirect_inheritance`](#dw-tag-sun-indirect-inheritance) | const |  |
| [`DW_TAG_SUN_codeflags`](#dw-tag-sun-codeflags) | const |  |
| [`DW_TAG_SUN_memop_info`](#dw-tag-sun-memop-info) | const |  |
| [`DW_TAG_SUN_omp_child_func`](#dw-tag-sun-omp-child-func) | const |  |
| [`DW_TAG_SUN_rtti_descriptor`](#dw-tag-sun-rtti-descriptor) | const |  |
| [`DW_TAG_SUN_dtor_info`](#dw-tag-sun-dtor-info) | const |  |
| [`DW_TAG_SUN_dtor`](#dw-tag-sun-dtor) | const |  |
| [`DW_TAG_SUN_f90_interface`](#dw-tag-sun-f90-interface) | const |  |
| [`DW_TAG_SUN_fortran_vax_structure`](#dw-tag-sun-fortran-vax-structure) | const |  |
| [`DW_TAG_ALTIUM_circ_type`](#dw-tag-altium-circ-type) | const |  |
| [`DW_TAG_ALTIUM_mwa_circ_type`](#dw-tag-altium-mwa-circ-type) | const |  |
| [`DW_TAG_ALTIUM_rev_carry_type`](#dw-tag-altium-rev-carry-type) | const |  |
| [`DW_TAG_ALTIUM_rom`](#dw-tag-altium-rom) | const |  |
| [`DW_TAG_upc_shared_type`](#dw-tag-upc-shared-type) | const |  |
| [`DW_TAG_upc_strict_type`](#dw-tag-upc-strict-type) | const |  |
| [`DW_TAG_upc_relaxed_type`](#dw-tag-upc-relaxed-type) | const |  |
| [`DW_TAG_PGI_kanji_type`](#dw-tag-pgi-kanji-type) | const |  |
| [`DW_TAG_PGI_interface_block`](#dw-tag-pgi-interface-block) | const |  |
| [`DW_TAG_BORLAND_property`](#dw-tag-borland-property) | const |  |
| [`DW_TAG_BORLAND_Delphi_string`](#dw-tag-borland-delphi-string) | const |  |
| [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw-tag-borland-delphi-dynamic-array) | const |  |
| [`DW_TAG_BORLAND_Delphi_set`](#dw-tag-borland-delphi-set) | const |  |
| [`DW_TAG_BORLAND_Delphi_variant`](#dw-tag-borland-delphi-variant) | const |  |
| [`DW_AT_null`](#dw-at-null) | const |  |
| [`DW_AT_fund_type`](#dw-at-fund-type) | const |  |
| [`DW_AT_mod_fund_type`](#dw-at-mod-fund-type) | const |  |
| [`DW_AT_user_def_type`](#dw-at-user-def-type) | const |  |
| [`DW_AT_mod_u_d_type`](#dw-at-mod-u-d-type) | const |  |
| [`DW_AT_subscr_data`](#dw-at-subscr-data) | const |  |
| [`DW_AT_element_list`](#dw-at-element-list) | const |  |
| [`DW_AT_member`](#dw-at-member) | const |  |
| [`DW_AT_friends`](#dw-at-friends) | const |  |
| [`DW_AT_program`](#dw-at-program) | const |  |
| [`DW_AT_private`](#dw-at-private) | const |  |
| [`DW_AT_protected`](#dw-at-protected) | const |  |
| [`DW_AT_public`](#dw-at-public) | const |  |
| [`DW_AT_pure_virtual`](#dw-at-pure-virtual) | const |  |
| [`DW_AT_virtual`](#dw-at-virtual) | const |  |
| [`DW_AT_specification_v1`](#dw-at-specification-v1) | const |  |
| [`DW_AT_sibling`](#dw-at-sibling) | const |  |
| [`DW_AT_location`](#dw-at-location) | const |  |
| [`DW_AT_name`](#dw-at-name) | const |  |
| [`DW_AT_ordering`](#dw-at-ordering) | const |  |
| [`DW_AT_byte_size`](#dw-at-byte-size) | const |  |
| [`DW_AT_bit_offset`](#dw-at-bit-offset) | const |  |
| [`DW_AT_bit_size`](#dw-at-bit-size) | const |  |
| [`DW_AT_stmt_list`](#dw-at-stmt-list) | const |  |
| [`DW_AT_low_pc`](#dw-at-low-pc) | const |  |
| [`DW_AT_high_pc`](#dw-at-high-pc) | const |  |
| [`DW_AT_language`](#dw-at-language) | const |  |
| [`DW_AT_discr`](#dw-at-discr) | const |  |
| [`DW_AT_discr_value`](#dw-at-discr-value) | const |  |
| [`DW_AT_visibility`](#dw-at-visibility) | const |  |
| [`DW_AT_import`](#dw-at-import) | const |  |
| [`DW_AT_string_length`](#dw-at-string-length) | const |  |
| [`DW_AT_common_reference`](#dw-at-common-reference) | const |  |
| [`DW_AT_comp_dir`](#dw-at-comp-dir) | const |  |
| [`DW_AT_const_value`](#dw-at-const-value) | const |  |
| [`DW_AT_containing_type`](#dw-at-containing-type) | const |  |
| [`DW_AT_default_value`](#dw-at-default-value) | const |  |
| [`DW_AT_inline`](#dw-at-inline) | const |  |
| [`DW_AT_is_optional`](#dw-at-is-optional) | const |  |
| [`DW_AT_lower_bound`](#dw-at-lower-bound) | const |  |
| [`DW_AT_producer`](#dw-at-producer) | const |  |
| [`DW_AT_prototyped`](#dw-at-prototyped) | const |  |
| [`DW_AT_return_addr`](#dw-at-return-addr) | const |  |
| [`DW_AT_start_scope`](#dw-at-start-scope) | const |  |
| [`DW_AT_bit_stride`](#dw-at-bit-stride) | const |  |
| [`DW_AT_upper_bound`](#dw-at-upper-bound) | const |  |
| [`DW_AT_abstract_origin`](#dw-at-abstract-origin) | const |  |
| [`DW_AT_accessibility`](#dw-at-accessibility) | const |  |
| [`DW_AT_address_class`](#dw-at-address-class) | const |  |
| [`DW_AT_artificial`](#dw-at-artificial) | const |  |
| [`DW_AT_base_types`](#dw-at-base-types) | const |  |
| [`DW_AT_calling_convention`](#dw-at-calling-convention) | const |  |
| [`DW_AT_count`](#dw-at-count) | const |  |
| [`DW_AT_data_member_location`](#dw-at-data-member-location) | const |  |
| [`DW_AT_decl_column`](#dw-at-decl-column) | const |  |
| [`DW_AT_decl_file`](#dw-at-decl-file) | const |  |
| [`DW_AT_decl_line`](#dw-at-decl-line) | const |  |
| [`DW_AT_declaration`](#dw-at-declaration) | const |  |
| [`DW_AT_discr_list`](#dw-at-discr-list) | const |  |
| [`DW_AT_encoding`](#dw-at-encoding) | const |  |
| [`DW_AT_external`](#dw-at-external) | const |  |
| [`DW_AT_frame_base`](#dw-at-frame-base) | const |  |
| [`DW_AT_friend`](#dw-at-friend) | const |  |
| [`DW_AT_identifier_case`](#dw-at-identifier-case) | const |  |
| [`DW_AT_macro_info`](#dw-at-macro-info) | const |  |
| [`DW_AT_namelist_item`](#dw-at-namelist-item) | const |  |
| [`DW_AT_priority`](#dw-at-priority) | const |  |
| [`DW_AT_segment`](#dw-at-segment) | const |  |
| [`DW_AT_specification`](#dw-at-specification) | const |  |
| [`DW_AT_static_link`](#dw-at-static-link) | const |  |
| [`DW_AT_type`](#dw-at-type) | const |  |
| [`DW_AT_use_location`](#dw-at-use-location) | const |  |
| [`DW_AT_variable_parameter`](#dw-at-variable-parameter) | const |  |
| [`DW_AT_virtuality`](#dw-at-virtuality) | const |  |
| [`DW_AT_vtable_elem_location`](#dw-at-vtable-elem-location) | const |  |
| [`DW_AT_allocated`](#dw-at-allocated) | const |  |
| [`DW_AT_associated`](#dw-at-associated) | const |  |
| [`DW_AT_data_location`](#dw-at-data-location) | const |  |
| [`DW_AT_byte_stride`](#dw-at-byte-stride) | const |  |
| [`DW_AT_entry_pc`](#dw-at-entry-pc) | const |  |
| [`DW_AT_use_UTF8`](#dw-at-use-utf8) | const |  |
| [`DW_AT_extension`](#dw-at-extension) | const |  |
| [`DW_AT_ranges`](#dw-at-ranges) | const |  |
| [`DW_AT_trampoline`](#dw-at-trampoline) | const |  |
| [`DW_AT_call_column`](#dw-at-call-column) | const |  |
| [`DW_AT_call_file`](#dw-at-call-file) | const |  |
| [`DW_AT_call_line`](#dw-at-call-line) | const |  |
| [`DW_AT_description`](#dw-at-description) | const |  |
| [`DW_AT_binary_scale`](#dw-at-binary-scale) | const |  |
| [`DW_AT_decimal_scale`](#dw-at-decimal-scale) | const |  |
| [`DW_AT_small`](#dw-at-small) | const |  |
| [`DW_AT_decimal_sign`](#dw-at-decimal-sign) | const |  |
| [`DW_AT_digit_count`](#dw-at-digit-count) | const |  |
| [`DW_AT_picture_string`](#dw-at-picture-string) | const |  |
| [`DW_AT_mutable`](#dw-at-mutable) | const |  |
| [`DW_AT_threads_scaled`](#dw-at-threads-scaled) | const |  |
| [`DW_AT_explicit`](#dw-at-explicit) | const |  |
| [`DW_AT_object_pointer`](#dw-at-object-pointer) | const |  |
| [`DW_AT_endianity`](#dw-at-endianity) | const |  |
| [`DW_AT_elemental`](#dw-at-elemental) | const |  |
| [`DW_AT_pure`](#dw-at-pure) | const |  |
| [`DW_AT_recursive`](#dw-at-recursive) | const |  |
| [`DW_AT_signature`](#dw-at-signature) | const |  |
| [`DW_AT_main_subprogram`](#dw-at-main-subprogram) | const |  |
| [`DW_AT_data_bit_offset`](#dw-at-data-bit-offset) | const |  |
| [`DW_AT_const_expr`](#dw-at-const-expr) | const |  |
| [`DW_AT_enum_class`](#dw-at-enum-class) | const |  |
| [`DW_AT_linkage_name`](#dw-at-linkage-name) | const |  |
| [`DW_AT_string_length_bit_size`](#dw-at-string-length-bit-size) | const |  |
| [`DW_AT_string_length_byte_size`](#dw-at-string-length-byte-size) | const |  |
| [`DW_AT_rank`](#dw-at-rank) | const |  |
| [`DW_AT_str_offsets_base`](#dw-at-str-offsets-base) | const |  |
| [`DW_AT_addr_base`](#dw-at-addr-base) | const |  |
| [`DW_AT_rnglists_base`](#dw-at-rnglists-base) | const |  |
| [`DW_AT_dwo_name`](#dw-at-dwo-name) | const |  |
| [`DW_AT_reference`](#dw-at-reference) | const |  |
| [`DW_AT_rvalue_reference`](#dw-at-rvalue-reference) | const |  |
| [`DW_AT_macros`](#dw-at-macros) | const |  |
| [`DW_AT_call_all_calls`](#dw-at-call-all-calls) | const |  |
| [`DW_AT_call_all_source_calls`](#dw-at-call-all-source-calls) | const |  |
| [`DW_AT_call_all_tail_calls`](#dw-at-call-all-tail-calls) | const |  |
| [`DW_AT_call_return_pc`](#dw-at-call-return-pc) | const |  |
| [`DW_AT_call_value`](#dw-at-call-value) | const |  |
| [`DW_AT_call_origin`](#dw-at-call-origin) | const |  |
| [`DW_AT_call_parameter`](#dw-at-call-parameter) | const |  |
| [`DW_AT_call_pc`](#dw-at-call-pc) | const |  |
| [`DW_AT_call_tail_call`](#dw-at-call-tail-call) | const |  |
| [`DW_AT_call_target`](#dw-at-call-target) | const |  |
| [`DW_AT_call_target_clobbered`](#dw-at-call-target-clobbered) | const |  |
| [`DW_AT_call_data_location`](#dw-at-call-data-location) | const |  |
| [`DW_AT_call_data_value`](#dw-at-call-data-value) | const |  |
| [`DW_AT_noreturn`](#dw-at-noreturn) | const |  |
| [`DW_AT_alignment`](#dw-at-alignment) | const |  |
| [`DW_AT_export_symbols`](#dw-at-export-symbols) | const |  |
| [`DW_AT_deleted`](#dw-at-deleted) | const |  |
| [`DW_AT_defaulted`](#dw-at-defaulted) | const |  |
| [`DW_AT_loclists_base`](#dw-at-loclists-base) | const |  |
| [`DW_AT_lo_user`](#dw-at-lo-user) | const |  |
| [`DW_AT_hi_user`](#dw-at-hi-user) | const |  |
| [`DW_AT_MIPS_fde`](#dw-at-mips-fde) | const |  |
| [`DW_AT_MIPS_loop_begin`](#dw-at-mips-loop-begin) | const |  |
| [`DW_AT_MIPS_tail_loop_begin`](#dw-at-mips-tail-loop-begin) | const |  |
| [`DW_AT_MIPS_epilog_begin`](#dw-at-mips-epilog-begin) | const |  |
| [`DW_AT_MIPS_loop_unroll_factor`](#dw-at-mips-loop-unroll-factor) | const |  |
| [`DW_AT_MIPS_software_pipeline_depth`](#dw-at-mips-software-pipeline-depth) | const |  |
| [`DW_AT_MIPS_linkage_name`](#dw-at-mips-linkage-name) | const |  |
| [`DW_AT_MIPS_stride`](#dw-at-mips-stride) | const |  |
| [`DW_AT_MIPS_abstract_name`](#dw-at-mips-abstract-name) | const |  |
| [`DW_AT_MIPS_clone_origin`](#dw-at-mips-clone-origin) | const |  |
| [`DW_AT_MIPS_has_inlines`](#dw-at-mips-has-inlines) | const |  |
| [`DW_AT_MIPS_stride_byte`](#dw-at-mips-stride-byte) | const |  |
| [`DW_AT_MIPS_stride_elem`](#dw-at-mips-stride-elem) | const |  |
| [`DW_AT_MIPS_ptr_dopetype`](#dw-at-mips-ptr-dopetype) | const |  |
| [`DW_AT_MIPS_allocatable_dopetype`](#dw-at-mips-allocatable-dopetype) | const |  |
| [`DW_AT_MIPS_assumed_shape_dopetype`](#dw-at-mips-assumed-shape-dopetype) | const |  |
| [`DW_AT_MIPS_assumed_size`](#dw-at-mips-assumed-size) | const |  |
| [`DW_AT_INTEL_other_endian`](#dw-at-intel-other-endian) | const |  |
| [`DW_AT_sf_names`](#dw-at-sf-names) | const |  |
| [`DW_AT_src_info`](#dw-at-src-info) | const |  |
| [`DW_AT_mac_info`](#dw-at-mac-info) | const |  |
| [`DW_AT_src_coords`](#dw-at-src-coords) | const |  |
| [`DW_AT_body_begin`](#dw-at-body-begin) | const |  |
| [`DW_AT_body_end`](#dw-at-body-end) | const |  |
| [`DW_AT_GNU_vector`](#dw-at-gnu-vector) | const |  |
| [`DW_AT_GNU_guarded_by`](#dw-at-gnu-guarded-by) | const |  |
| [`DW_AT_GNU_pt_guarded_by`](#dw-at-gnu-pt-guarded-by) | const |  |
| [`DW_AT_GNU_guarded`](#dw-at-gnu-guarded) | const |  |
| [`DW_AT_GNU_pt_guarded`](#dw-at-gnu-pt-guarded) | const |  |
| [`DW_AT_GNU_locks_excluded`](#dw-at-gnu-locks-excluded) | const |  |
| [`DW_AT_GNU_exclusive_locks_required`](#dw-at-gnu-exclusive-locks-required) | const |  |
| [`DW_AT_GNU_shared_locks_required`](#dw-at-gnu-shared-locks-required) | const |  |
| [`DW_AT_GNU_odr_signature`](#dw-at-gnu-odr-signature) | const |  |
| [`DW_AT_GNU_template_name`](#dw-at-gnu-template-name) | const |  |
| [`DW_AT_GNU_call_site_value`](#dw-at-gnu-call-site-value) | const |  |
| [`DW_AT_GNU_call_site_data_value`](#dw-at-gnu-call-site-data-value) | const |  |
| [`DW_AT_GNU_call_site_target`](#dw-at-gnu-call-site-target) | const |  |
| [`DW_AT_GNU_call_site_target_clobbered`](#dw-at-gnu-call-site-target-clobbered) | const |  |
| [`DW_AT_GNU_tail_call`](#dw-at-gnu-tail-call) | const |  |
| [`DW_AT_GNU_all_tail_call_sites`](#dw-at-gnu-all-tail-call-sites) | const |  |
| [`DW_AT_GNU_all_call_sites`](#dw-at-gnu-all-call-sites) | const |  |
| [`DW_AT_GNU_all_source_call_sites`](#dw-at-gnu-all-source-call-sites) | const |  |
| [`DW_AT_GNU_macros`](#dw-at-gnu-macros) | const |  |
| [`DW_AT_GNU_deleted`](#dw-at-gnu-deleted) | const |  |
| [`DW_AT_GNU_dwo_name`](#dw-at-gnu-dwo-name) | const |  |
| [`DW_AT_GNU_dwo_id`](#dw-at-gnu-dwo-id) | const |  |
| [`DW_AT_GNU_ranges_base`](#dw-at-gnu-ranges-base) | const |  |
| [`DW_AT_GNU_addr_base`](#dw-at-gnu-addr-base) | const |  |
| [`DW_AT_GNU_pubnames`](#dw-at-gnu-pubnames) | const |  |
| [`DW_AT_GNU_pubtypes`](#dw-at-gnu-pubtypes) | const |  |
| [`DW_AT_GNU_discriminator`](#dw-at-gnu-discriminator) | const |  |
| [`DW_AT_GNU_locviews`](#dw-at-gnu-locviews) | const |  |
| [`DW_AT_GNU_entry_view`](#dw-at-gnu-entry-view) | const |  |
| [`DW_AT_SUN_template`](#dw-at-sun-template) | const |  |
| [`DW_AT_SUN_alignment`](#dw-at-sun-alignment) | const |  |
| [`DW_AT_SUN_vtable`](#dw-at-sun-vtable) | const |  |
| [`DW_AT_SUN_count_guarantee`](#dw-at-sun-count-guarantee) | const |  |
| [`DW_AT_SUN_command_line`](#dw-at-sun-command-line) | const |  |
| [`DW_AT_SUN_vbase`](#dw-at-sun-vbase) | const |  |
| [`DW_AT_SUN_compile_options`](#dw-at-sun-compile-options) | const |  |
| [`DW_AT_SUN_language`](#dw-at-sun-language) | const |  |
| [`DW_AT_SUN_browser_file`](#dw-at-sun-browser-file) | const |  |
| [`DW_AT_SUN_vtable_abi`](#dw-at-sun-vtable-abi) | const |  |
| [`DW_AT_SUN_func_offsets`](#dw-at-sun-func-offsets) | const |  |
| [`DW_AT_SUN_cf_kind`](#dw-at-sun-cf-kind) | const |  |
| [`DW_AT_SUN_vtable_index`](#dw-at-sun-vtable-index) | const |  |
| [`DW_AT_SUN_omp_tpriv_addr`](#dw-at-sun-omp-tpriv-addr) | const |  |
| [`DW_AT_SUN_omp_child_func`](#dw-at-sun-omp-child-func) | const |  |
| [`DW_AT_SUN_func_offset`](#dw-at-sun-func-offset) | const |  |
| [`DW_AT_SUN_memop_type_ref`](#dw-at-sun-memop-type-ref) | const |  |
| [`DW_AT_SUN_profile_id`](#dw-at-sun-profile-id) | const |  |
| [`DW_AT_SUN_memop_signature`](#dw-at-sun-memop-signature) | const |  |
| [`DW_AT_SUN_obj_dir`](#dw-at-sun-obj-dir) | const |  |
| [`DW_AT_SUN_obj_file`](#dw-at-sun-obj-file) | const |  |
| [`DW_AT_SUN_original_name`](#dw-at-sun-original-name) | const |  |
| [`DW_AT_SUN_hwcprof_signature`](#dw-at-sun-hwcprof-signature) | const |  |
| [`DW_AT_SUN_amd64_parmdump`](#dw-at-sun-amd64-parmdump) | const |  |
| [`DW_AT_SUN_part_link_name`](#dw-at-sun-part-link-name) | const |  |
| [`DW_AT_SUN_link_name`](#dw-at-sun-link-name) | const |  |
| [`DW_AT_SUN_pass_with_const`](#dw-at-sun-pass-with-const) | const |  |
| [`DW_AT_SUN_return_with_const`](#dw-at-sun-return-with-const) | const |  |
| [`DW_AT_SUN_import_by_name`](#dw-at-sun-import-by-name) | const |  |
| [`DW_AT_SUN_f90_pointer`](#dw-at-sun-f90-pointer) | const |  |
| [`DW_AT_SUN_pass_by_ref`](#dw-at-sun-pass-by-ref) | const |  |
| [`DW_AT_SUN_f90_allocatable`](#dw-at-sun-f90-allocatable) | const |  |
| [`DW_AT_SUN_f90_assumed_shape_array`](#dw-at-sun-f90-assumed-shape-array) | const |  |
| [`DW_AT_SUN_c_vla`](#dw-at-sun-c-vla) | const |  |
| [`DW_AT_SUN_return_value_ptr`](#dw-at-sun-return-value-ptr) | const |  |
| [`DW_AT_SUN_dtor_start`](#dw-at-sun-dtor-start) | const |  |
| [`DW_AT_SUN_dtor_length`](#dw-at-sun-dtor-length) | const |  |
| [`DW_AT_SUN_dtor_state_initial`](#dw-at-sun-dtor-state-initial) | const |  |
| [`DW_AT_SUN_dtor_state_final`](#dw-at-sun-dtor-state-final) | const |  |
| [`DW_AT_SUN_dtor_state_deltas`](#dw-at-sun-dtor-state-deltas) | const |  |
| [`DW_AT_SUN_import_by_lname`](#dw-at-sun-import-by-lname) | const |  |
| [`DW_AT_SUN_f90_use_only`](#dw-at-sun-f90-use-only) | const |  |
| [`DW_AT_SUN_namelist_spec`](#dw-at-sun-namelist-spec) | const |  |
| [`DW_AT_SUN_is_omp_child_func`](#dw-at-sun-is-omp-child-func) | const |  |
| [`DW_AT_SUN_fortran_main_alias`](#dw-at-sun-fortran-main-alias) | const |  |
| [`DW_AT_SUN_fortran_based`](#dw-at-sun-fortran-based) | const |  |
| [`DW_AT_ALTIUM_loclist`](#dw-at-altium-loclist) | const |  |
| [`DW_AT_use_GNAT_descriptive_type`](#dw-at-use-gnat-descriptive-type) | const |  |
| [`DW_AT_GNAT_descriptive_type`](#dw-at-gnat-descriptive-type) | const |  |
| [`DW_AT_GNU_numerator`](#dw-at-gnu-numerator) | const |  |
| [`DW_AT_GNU_denominator`](#dw-at-gnu-denominator) | const |  |
| [`DW_AT_GNU_bias`](#dw-at-gnu-bias) | const |  |
| [`DW_AT_upc_threads_scaled`](#dw-at-upc-threads-scaled) | const |  |
| [`DW_AT_PGI_lbase`](#dw-at-pgi-lbase) | const |  |
| [`DW_AT_PGI_soffset`](#dw-at-pgi-soffset) | const |  |
| [`DW_AT_PGI_lstride`](#dw-at-pgi-lstride) | const |  |
| [`DW_AT_BORLAND_property_read`](#dw-at-borland-property-read) | const |  |
| [`DW_AT_BORLAND_property_write`](#dw-at-borland-property-write) | const |  |
| [`DW_AT_BORLAND_property_implements`](#dw-at-borland-property-implements) | const |  |
| [`DW_AT_BORLAND_property_index`](#dw-at-borland-property-index) | const |  |
| [`DW_AT_BORLAND_property_default`](#dw-at-borland-property-default) | const |  |
| [`DW_AT_BORLAND_Delphi_unit`](#dw-at-borland-delphi-unit) | const |  |
| [`DW_AT_BORLAND_Delphi_class`](#dw-at-borland-delphi-class) | const |  |
| [`DW_AT_BORLAND_Delphi_record`](#dw-at-borland-delphi-record) | const |  |
| [`DW_AT_BORLAND_Delphi_metaclass`](#dw-at-borland-delphi-metaclass) | const |  |
| [`DW_AT_BORLAND_Delphi_constructor`](#dw-at-borland-delphi-constructor) | const |  |
| [`DW_AT_BORLAND_Delphi_destructor`](#dw-at-borland-delphi-destructor) | const |  |
| [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw-at-borland-delphi-anonymous-method) | const |  |
| [`DW_AT_BORLAND_Delphi_interface`](#dw-at-borland-delphi-interface) | const |  |
| [`DW_AT_BORLAND_Delphi_ABI`](#dw-at-borland-delphi-abi) | const |  |
| [`DW_AT_BORLAND_Delphi_return`](#dw-at-borland-delphi-return) | const |  |
| [`DW_AT_BORLAND_Delphi_frameptr`](#dw-at-borland-delphi-frameptr) | const |  |
| [`DW_AT_BORLAND_closure`](#dw-at-borland-closure) | const |  |
| [`DW_AT_LLVM_include_path`](#dw-at-llvm-include-path) | const |  |
| [`DW_AT_LLVM_config_macros`](#dw-at-llvm-config-macros) | const |  |
| [`DW_AT_LLVM_isysroot`](#dw-at-llvm-isysroot) | const |  |
| [`DW_AT_APPLE_optimized`](#dw-at-apple-optimized) | const |  |
| [`DW_AT_APPLE_flags`](#dw-at-apple-flags) | const |  |
| [`DW_AT_APPLE_isa`](#dw-at-apple-isa) | const |  |
| [`DW_AT_APPLE_block`](#dw-at-apple-block) | const |  |
| [`DW_AT_APPLE_major_runtime_vers`](#dw-at-apple-major-runtime-vers) | const |  |
| [`DW_AT_APPLE_runtime_class`](#dw-at-apple-runtime-class) | const |  |
| [`DW_AT_APPLE_omit_frame_ptr`](#dw-at-apple-omit-frame-ptr) | const |  |
| [`DW_AT_APPLE_property_name`](#dw-at-apple-property-name) | const |  |
| [`DW_AT_APPLE_property_getter`](#dw-at-apple-property-getter) | const |  |
| [`DW_AT_APPLE_property_setter`](#dw-at-apple-property-setter) | const |  |
| [`DW_AT_APPLE_property_attribute`](#dw-at-apple-property-attribute) | const |  |
| [`DW_AT_APPLE_objc_complete_type`](#dw-at-apple-objc-complete-type) | const |  |
| [`DW_AT_APPLE_property`](#dw-at-apple-property) | const |  |
| [`DW_FORM_null`](#dw-form-null) | const |  |
| [`DW_FORM_ref`](#dw-form-ref) | const |  |
| [`DW_FORM_addr`](#dw-form-addr) | const |  |
| [`DW_FORM_block2`](#dw-form-block2) | const |  |
| [`DW_FORM_block4`](#dw-form-block4) | const |  |
| [`DW_FORM_data2`](#dw-form-data2) | const |  |
| [`DW_FORM_data4`](#dw-form-data4) | const |  |
| [`DW_FORM_data8`](#dw-form-data8) | const |  |
| [`DW_FORM_string`](#dw-form-string) | const |  |
| [`DW_FORM_block`](#dw-form-block) | const |  |
| [`DW_FORM_block1`](#dw-form-block1) | const |  |
| [`DW_FORM_data1`](#dw-form-data1) | const |  |
| [`DW_FORM_flag`](#dw-form-flag) | const |  |
| [`DW_FORM_sdata`](#dw-form-sdata) | const |  |
| [`DW_FORM_strp`](#dw-form-strp) | const |  |
| [`DW_FORM_udata`](#dw-form-udata) | const |  |
| [`DW_FORM_ref_addr`](#dw-form-ref-addr) | const |  |
| [`DW_FORM_ref1`](#dw-form-ref1) | const |  |
| [`DW_FORM_ref2`](#dw-form-ref2) | const |  |
| [`DW_FORM_ref4`](#dw-form-ref4) | const |  |
| [`DW_FORM_ref8`](#dw-form-ref8) | const |  |
| [`DW_FORM_ref_udata`](#dw-form-ref-udata) | const |  |
| [`DW_FORM_indirect`](#dw-form-indirect) | const |  |
| [`DW_FORM_sec_offset`](#dw-form-sec-offset) | const |  |
| [`DW_FORM_exprloc`](#dw-form-exprloc) | const |  |
| [`DW_FORM_flag_present`](#dw-form-flag-present) | const |  |
| [`DW_FORM_ref_sig8`](#dw-form-ref-sig8) | const |  |
| [`DW_FORM_strx`](#dw-form-strx) | const |  |
| [`DW_FORM_addrx`](#dw-form-addrx) | const |  |
| [`DW_FORM_ref_sup4`](#dw-form-ref-sup4) | const |  |
| [`DW_FORM_strp_sup`](#dw-form-strp-sup) | const |  |
| [`DW_FORM_data16`](#dw-form-data16) | const |  |
| [`DW_FORM_line_strp`](#dw-form-line-strp) | const |  |
| [`DW_FORM_implicit_const`](#dw-form-implicit-const) | const |  |
| [`DW_FORM_loclistx`](#dw-form-loclistx) | const |  |
| [`DW_FORM_rnglistx`](#dw-form-rnglistx) | const |  |
| [`DW_FORM_ref_sup8`](#dw-form-ref-sup8) | const |  |
| [`DW_FORM_strx1`](#dw-form-strx1) | const |  |
| [`DW_FORM_strx2`](#dw-form-strx2) | const |  |
| [`DW_FORM_strx3`](#dw-form-strx3) | const |  |
| [`DW_FORM_strx4`](#dw-form-strx4) | const |  |
| [`DW_FORM_addrx1`](#dw-form-addrx1) | const |  |
| [`DW_FORM_addrx2`](#dw-form-addrx2) | const |  |
| [`DW_FORM_addrx3`](#dw-form-addrx3) | const |  |
| [`DW_FORM_addrx4`](#dw-form-addrx4) | const |  |
| [`DW_FORM_GNU_addr_index`](#dw-form-gnu-addr-index) | const |  |
| [`DW_FORM_GNU_str_index`](#dw-form-gnu-str-index) | const |  |
| [`DW_FORM_GNU_ref_alt`](#dw-form-gnu-ref-alt) | const |  |
| [`DW_FORM_GNU_strp_alt`](#dw-form-gnu-strp-alt) | const |  |
| [`DW_ATE_address`](#dw-ate-address) | const |  |
| [`DW_ATE_boolean`](#dw-ate-boolean) | const |  |
| [`DW_ATE_complex_float`](#dw-ate-complex-float) | const |  |
| [`DW_ATE_float`](#dw-ate-float) | const |  |
| [`DW_ATE_signed`](#dw-ate-signed) | const |  |
| [`DW_ATE_signed_char`](#dw-ate-signed-char) | const |  |
| [`DW_ATE_unsigned`](#dw-ate-unsigned) | const |  |
| [`DW_ATE_unsigned_char`](#dw-ate-unsigned-char) | const |  |
| [`DW_ATE_imaginary_float`](#dw-ate-imaginary-float) | const |  |
| [`DW_ATE_packed_decimal`](#dw-ate-packed-decimal) | const |  |
| [`DW_ATE_numeric_string`](#dw-ate-numeric-string) | const |  |
| [`DW_ATE_edited`](#dw-ate-edited) | const |  |
| [`DW_ATE_signed_fixed`](#dw-ate-signed-fixed) | const |  |
| [`DW_ATE_unsigned_fixed`](#dw-ate-unsigned-fixed) | const |  |
| [`DW_ATE_decimal_float`](#dw-ate-decimal-float) | const |  |
| [`DW_ATE_UTF`](#dw-ate-utf) | const |  |
| [`DW_ATE_UCS`](#dw-ate-ucs) | const |  |
| [`DW_ATE_ASCII`](#dw-ate-ascii) | const |  |
| [`DW_ATE_lo_user`](#dw-ate-lo-user) | const |  |
| [`DW_ATE_hi_user`](#dw-ate-hi-user) | const |  |
| [`DW_LLE_end_of_list`](#dw-lle-end-of-list) | const |  |
| [`DW_LLE_base_addressx`](#dw-lle-base-addressx) | const |  |
| [`DW_LLE_startx_endx`](#dw-lle-startx-endx) | const |  |
| [`DW_LLE_startx_length`](#dw-lle-startx-length) | const |  |
| [`DW_LLE_offset_pair`](#dw-lle-offset-pair) | const |  |
| [`DW_LLE_default_location`](#dw-lle-default-location) | const |  |
| [`DW_LLE_base_address`](#dw-lle-base-address) | const |  |
| [`DW_LLE_start_end`](#dw-lle-start-end) | const |  |
| [`DW_LLE_start_length`](#dw-lle-start-length) | const |  |
| [`DW_LLE_GNU_view_pair`](#dw-lle-gnu-view-pair) | const |  |
| [`DW_DS_unsigned`](#dw-ds-unsigned) | const |  |
| [`DW_DS_leading_overpunch`](#dw-ds-leading-overpunch) | const |  |
| [`DW_DS_trailing_overpunch`](#dw-ds-trailing-overpunch) | const |  |
| [`DW_DS_leading_separate`](#dw-ds-leading-separate) | const |  |
| [`DW_DS_trailing_separate`](#dw-ds-trailing-separate) | const |  |
| [`DW_END_default`](#dw-end-default) | const |  |
| [`DW_END_big`](#dw-end-big) | const |  |
| [`DW_END_little`](#dw-end-little) | const |  |
| [`DW_END_lo_user`](#dw-end-lo-user) | const |  |
| [`DW_END_hi_user`](#dw-end-hi-user) | const |  |
| [`DW_ACCESS_public`](#dw-access-public) | const |  |
| [`DW_ACCESS_protected`](#dw-access-protected) | const |  |
| [`DW_ACCESS_private`](#dw-access-private) | const |  |
| [`DW_VIS_local`](#dw-vis-local) | const |  |
| [`DW_VIS_exported`](#dw-vis-exported) | const |  |
| [`DW_VIS_qualified`](#dw-vis-qualified) | const |  |
| [`DW_VIRTUALITY_none`](#dw-virtuality-none) | const |  |
| [`DW_VIRTUALITY_virtual`](#dw-virtuality-virtual) | const |  |
| [`DW_VIRTUALITY_pure_virtual`](#dw-virtuality-pure-virtual) | const |  |
| [`DW_LANG_C89`](#dw-lang-c89) | const |  |
| [`DW_LANG_C`](#dw-lang-c) | const |  |
| [`DW_LANG_Ada83`](#dw-lang-ada83) | const |  |
| [`DW_LANG_C_plus_plus`](#dw-lang-c-plus-plus) | const |  |
| [`DW_LANG_Cobol74`](#dw-lang-cobol74) | const |  |
| [`DW_LANG_Cobol85`](#dw-lang-cobol85) | const |  |
| [`DW_LANG_Fortran77`](#dw-lang-fortran77) | const |  |
| [`DW_LANG_Fortran90`](#dw-lang-fortran90) | const |  |
| [`DW_LANG_Pascal83`](#dw-lang-pascal83) | const |  |
| [`DW_LANG_Modula2`](#dw-lang-modula2) | const |  |
| [`DW_LANG_Java`](#dw-lang-java) | const |  |
| [`DW_LANG_C99`](#dw-lang-c99) | const |  |
| [`DW_LANG_Ada95`](#dw-lang-ada95) | const |  |
| [`DW_LANG_Fortran95`](#dw-lang-fortran95) | const |  |
| [`DW_LANG_PLI`](#dw-lang-pli) | const |  |
| [`DW_LANG_ObjC`](#dw-lang-objc) | const |  |
| [`DW_LANG_ObjC_plus_plus`](#dw-lang-objc-plus-plus) | const |  |
| [`DW_LANG_UPC`](#dw-lang-upc) | const |  |
| [`DW_LANG_D`](#dw-lang-d) | const |  |
| [`DW_LANG_Python`](#dw-lang-python) | const |  |
| [`DW_LANG_OpenCL`](#dw-lang-opencl) | const |  |
| [`DW_LANG_Go`](#dw-lang-go) | const |  |
| [`DW_LANG_Modula3`](#dw-lang-modula3) | const |  |
| [`DW_LANG_Haskell`](#dw-lang-haskell) | const |  |
| [`DW_LANG_C_plus_plus_03`](#dw-lang-c-plus-plus-03) | const |  |
| [`DW_LANG_C_plus_plus_11`](#dw-lang-c-plus-plus-11) | const |  |
| [`DW_LANG_OCaml`](#dw-lang-ocaml) | const |  |
| [`DW_LANG_Rust`](#dw-lang-rust) | const |  |
| [`DW_LANG_C11`](#dw-lang-c11) | const |  |
| [`DW_LANG_Swift`](#dw-lang-swift) | const |  |
| [`DW_LANG_Julia`](#dw-lang-julia) | const |  |
| [`DW_LANG_Dylan`](#dw-lang-dylan) | const |  |
| [`DW_LANG_C_plus_plus_14`](#dw-lang-c-plus-plus-14) | const |  |
| [`DW_LANG_Fortran03`](#dw-lang-fortran03) | const |  |
| [`DW_LANG_Fortran08`](#dw-lang-fortran08) | const |  |
| [`DW_LANG_RenderScript`](#dw-lang-renderscript) | const |  |
| [`DW_LANG_BLISS`](#dw-lang-bliss) | const |  |
| [`DW_LANG_Kotlin`](#dw-lang-kotlin) | const |  |
| [`DW_LANG_Zig`](#dw-lang-zig) | const |  |
| [`DW_LANG_Crystal`](#dw-lang-crystal) | const |  |
| [`DW_LANG_C_plus_plus_17`](#dw-lang-c-plus-plus-17) | const |  |
| [`DW_LANG_C_plus_plus_20`](#dw-lang-c-plus-plus-20) | const |  |
| [`DW_LANG_C17`](#dw-lang-c17) | const |  |
| [`DW_LANG_Fortran18`](#dw-lang-fortran18) | const |  |
| [`DW_LANG_Ada2005`](#dw-lang-ada2005) | const |  |
| [`DW_LANG_Ada2012`](#dw-lang-ada2012) | const |  |
| [`DW_LANG_lo_user`](#dw-lang-lo-user) | const |  |
| [`DW_LANG_hi_user`](#dw-lang-hi-user) | const |  |
| [`DW_LANG_Mips_Assembler`](#dw-lang-mips-assembler) | const |  |
| [`DW_LANG_GOOGLE_RenderScript`](#dw-lang-google-renderscript) | const |  |
| [`DW_LANG_SUN_Assembler`](#dw-lang-sun-assembler) | const |  |
| [`DW_LANG_ALTIUM_Assembler`](#dw-lang-altium-assembler) | const |  |
| [`DW_LANG_BORLAND_Delphi`](#dw-lang-borland-delphi) | const |  |
| [`DW_ADDR_none`](#dw-addr-none) | const |  |
| [`DW_ID_case_sensitive`](#dw-id-case-sensitive) | const |  |
| [`DW_ID_up_case`](#dw-id-up-case) | const |  |
| [`DW_ID_down_case`](#dw-id-down-case) | const |  |
| [`DW_ID_case_insensitive`](#dw-id-case-insensitive) | const |  |
| [`DW_CC_normal`](#dw-cc-normal) | const |  |
| [`DW_CC_program`](#dw-cc-program) | const |  |
| [`DW_CC_nocall`](#dw-cc-nocall) | const |  |
| [`DW_CC_pass_by_reference`](#dw-cc-pass-by-reference) | const |  |
| [`DW_CC_pass_by_value`](#dw-cc-pass-by-value) | const |  |
| [`DW_CC_lo_user`](#dw-cc-lo-user) | const |  |
| [`DW_CC_hi_user`](#dw-cc-hi-user) | const |  |
| [`DW_INL_not_inlined`](#dw-inl-not-inlined) | const |  |
| [`DW_INL_inlined`](#dw-inl-inlined) | const |  |
| [`DW_INL_declared_not_inlined`](#dw-inl-declared-not-inlined) | const |  |
| [`DW_INL_declared_inlined`](#dw-inl-declared-inlined) | const |  |
| [`DW_ORD_row_major`](#dw-ord-row-major) | const |  |
| [`DW_ORD_col_major`](#dw-ord-col-major) | const |  |
| [`DW_DSC_label`](#dw-dsc-label) | const |  |
| [`DW_DSC_range`](#dw-dsc-range) | const |  |
| [`DW_IDX_compile_unit`](#dw-idx-compile-unit) | const |  |
| [`DW_IDX_type_unit`](#dw-idx-type-unit) | const |  |
| [`DW_IDX_die_offset`](#dw-idx-die-offset) | const |  |
| [`DW_IDX_parent`](#dw-idx-parent) | const |  |
| [`DW_IDX_type_hash`](#dw-idx-type-hash) | const |  |
| [`DW_IDX_lo_user`](#dw-idx-lo-user) | const |  |
| [`DW_IDX_hi_user`](#dw-idx-hi-user) | const |  |
| [`DW_DEFAULTED_no`](#dw-defaulted-no) | const |  |
| [`DW_DEFAULTED_in_class`](#dw-defaulted-in-class) | const |  |
| [`DW_DEFAULTED_out_of_class`](#dw-defaulted-out-of-class) | const |  |
| [`DW_LNS_copy`](#dw-lns-copy) | const |  |
| [`DW_LNS_advance_pc`](#dw-lns-advance-pc) | const |  |
| [`DW_LNS_advance_line`](#dw-lns-advance-line) | const |  |
| [`DW_LNS_set_file`](#dw-lns-set-file) | const |  |
| [`DW_LNS_set_column`](#dw-lns-set-column) | const |  |
| [`DW_LNS_negate_stmt`](#dw-lns-negate-stmt) | const |  |
| [`DW_LNS_set_basic_block`](#dw-lns-set-basic-block) | const |  |
| [`DW_LNS_const_add_pc`](#dw-lns-const-add-pc) | const |  |
| [`DW_LNS_fixed_advance_pc`](#dw-lns-fixed-advance-pc) | const |  |
| [`DW_LNS_set_prologue_end`](#dw-lns-set-prologue-end) | const |  |
| [`DW_LNS_set_epilogue_begin`](#dw-lns-set-epilogue-begin) | const |  |
| [`DW_LNS_set_isa`](#dw-lns-set-isa) | const |  |
| [`DW_LNE_end_sequence`](#dw-lne-end-sequence) | const |  |
| [`DW_LNE_set_address`](#dw-lne-set-address) | const |  |
| [`DW_LNE_define_file`](#dw-lne-define-file) | const |  |
| [`DW_LNE_set_discriminator`](#dw-lne-set-discriminator) | const |  |
| [`DW_LNE_lo_user`](#dw-lne-lo-user) | const |  |
| [`DW_LNE_hi_user`](#dw-lne-hi-user) | const |  |
| [`DW_LNCT_path`](#dw-lnct-path) | const |  |
| [`DW_LNCT_directory_index`](#dw-lnct-directory-index) | const |  |
| [`DW_LNCT_timestamp`](#dw-lnct-timestamp) | const |  |
| [`DW_LNCT_size`](#dw-lnct-size) | const |  |
| [`DW_LNCT_MD5`](#dw-lnct-md5) | const |  |
| [`DW_LNCT_lo_user`](#dw-lnct-lo-user) | const |  |
| [`DW_LNCT_LLVM_source`](#dw-lnct-llvm-source) | const |  |
| [`DW_LNCT_hi_user`](#dw-lnct-hi-user) | const |  |
| [`DW_MACINFO_define`](#dw-macinfo-define) | const |  |
| [`DW_MACINFO_undef`](#dw-macinfo-undef) | const |  |
| [`DW_MACINFO_start_file`](#dw-macinfo-start-file) | const |  |
| [`DW_MACINFO_end_file`](#dw-macinfo-end-file) | const |  |
| [`DW_MACINFO_vendor_ext`](#dw-macinfo-vendor-ext) | const |  |
| [`DW_MACRO_define`](#dw-macro-define) | const |  |
| [`DW_MACRO_undef`](#dw-macro-undef) | const |  |
| [`DW_MACRO_start_file`](#dw-macro-start-file) | const |  |
| [`DW_MACRO_end_file`](#dw-macro-end-file) | const |  |
| [`DW_MACRO_define_strp`](#dw-macro-define-strp) | const |  |
| [`DW_MACRO_undef_strp`](#dw-macro-undef-strp) | const |  |
| [`DW_MACRO_import`](#dw-macro-import) | const |  |
| [`DW_MACRO_define_sup`](#dw-macro-define-sup) | const |  |
| [`DW_MACRO_undef_sup`](#dw-macro-undef-sup) | const |  |
| [`DW_MACRO_import_sup`](#dw-macro-import-sup) | const |  |
| [`DW_MACRO_define_strx`](#dw-macro-define-strx) | const |  |
| [`DW_MACRO_undef_strx`](#dw-macro-undef-strx) | const |  |
| [`DW_MACRO_lo_user`](#dw-macro-lo-user) | const |  |
| [`DW_MACRO_hi_user`](#dw-macro-hi-user) | const |  |
| [`DW_RLE_end_of_list`](#dw-rle-end-of-list) | const |  |
| [`DW_RLE_base_addressx`](#dw-rle-base-addressx) | const |  |
| [`DW_RLE_startx_endx`](#dw-rle-startx-endx) | const |  |
| [`DW_RLE_startx_length`](#dw-rle-startx-length) | const |  |
| [`DW_RLE_offset_pair`](#dw-rle-offset-pair) | const |  |
| [`DW_RLE_base_address`](#dw-rle-base-address) | const |  |
| [`DW_RLE_start_end`](#dw-rle-start-end) | const |  |
| [`DW_RLE_start_length`](#dw-rle-start-length) | const |  |
| [`DW_OP_addr`](#dw-op-addr) | const |  |
| [`DW_OP_deref`](#dw-op-deref) | const |  |
| [`DW_OP_const1u`](#dw-op-const1u) | const |  |
| [`DW_OP_const1s`](#dw-op-const1s) | const |  |
| [`DW_OP_const2u`](#dw-op-const2u) | const |  |
| [`DW_OP_const2s`](#dw-op-const2s) | const |  |
| [`DW_OP_const4u`](#dw-op-const4u) | const |  |
| [`DW_OP_const4s`](#dw-op-const4s) | const |  |
| [`DW_OP_const8u`](#dw-op-const8u) | const |  |
| [`DW_OP_const8s`](#dw-op-const8s) | const |  |
| [`DW_OP_constu`](#dw-op-constu) | const |  |
| [`DW_OP_consts`](#dw-op-consts) | const |  |
| [`DW_OP_dup`](#dw-op-dup) | const |  |
| [`DW_OP_drop`](#dw-op-drop) | const |  |
| [`DW_OP_over`](#dw-op-over) | const |  |
| [`DW_OP_pick`](#dw-op-pick) | const |  |
| [`DW_OP_swap`](#dw-op-swap) | const |  |
| [`DW_OP_rot`](#dw-op-rot) | const |  |
| [`DW_OP_xderef`](#dw-op-xderef) | const |  |
| [`DW_OP_abs`](#dw-op-abs) | const |  |
| [`DW_OP_and`](#dw-op-and) | const |  |
| [`DW_OP_div`](#dw-op-div) | const |  |
| [`DW_OP_minus`](#dw-op-minus) | const |  |
| [`DW_OP_mod`](#dw-op-mod) | const |  |
| [`DW_OP_mul`](#dw-op-mul) | const |  |
| [`DW_OP_neg`](#dw-op-neg) | const |  |
| [`DW_OP_not`](#dw-op-not) | const |  |
| [`DW_OP_or`](#dw-op-or) | const |  |
| [`DW_OP_plus`](#dw-op-plus) | const |  |
| [`DW_OP_plus_uconst`](#dw-op-plus-uconst) | const |  |
| [`DW_OP_shl`](#dw-op-shl) | const |  |
| [`DW_OP_shr`](#dw-op-shr) | const |  |
| [`DW_OP_shra`](#dw-op-shra) | const |  |
| [`DW_OP_xor`](#dw-op-xor) | const |  |
| [`DW_OP_bra`](#dw-op-bra) | const |  |
| [`DW_OP_eq`](#dw-op-eq) | const |  |
| [`DW_OP_ge`](#dw-op-ge) | const |  |
| [`DW_OP_gt`](#dw-op-gt) | const |  |
| [`DW_OP_le`](#dw-op-le) | const |  |
| [`DW_OP_lt`](#dw-op-lt) | const |  |
| [`DW_OP_ne`](#dw-op-ne) | const |  |
| [`DW_OP_skip`](#dw-op-skip) | const |  |
| [`DW_OP_lit0`](#dw-op-lit0) | const |  |
| [`DW_OP_lit1`](#dw-op-lit1) | const |  |
| [`DW_OP_lit2`](#dw-op-lit2) | const |  |
| [`DW_OP_lit3`](#dw-op-lit3) | const |  |
| [`DW_OP_lit4`](#dw-op-lit4) | const |  |
| [`DW_OP_lit5`](#dw-op-lit5) | const |  |
| [`DW_OP_lit6`](#dw-op-lit6) | const |  |
| [`DW_OP_lit7`](#dw-op-lit7) | const |  |
| [`DW_OP_lit8`](#dw-op-lit8) | const |  |
| [`DW_OP_lit9`](#dw-op-lit9) | const |  |
| [`DW_OP_lit10`](#dw-op-lit10) | const |  |
| [`DW_OP_lit11`](#dw-op-lit11) | const |  |
| [`DW_OP_lit12`](#dw-op-lit12) | const |  |
| [`DW_OP_lit13`](#dw-op-lit13) | const |  |
| [`DW_OP_lit14`](#dw-op-lit14) | const |  |
| [`DW_OP_lit15`](#dw-op-lit15) | const |  |
| [`DW_OP_lit16`](#dw-op-lit16) | const |  |
| [`DW_OP_lit17`](#dw-op-lit17) | const |  |
| [`DW_OP_lit18`](#dw-op-lit18) | const |  |
| [`DW_OP_lit19`](#dw-op-lit19) | const |  |
| [`DW_OP_lit20`](#dw-op-lit20) | const |  |
| [`DW_OP_lit21`](#dw-op-lit21) | const |  |
| [`DW_OP_lit22`](#dw-op-lit22) | const |  |
| [`DW_OP_lit23`](#dw-op-lit23) | const |  |
| [`DW_OP_lit24`](#dw-op-lit24) | const |  |
| [`DW_OP_lit25`](#dw-op-lit25) | const |  |
| [`DW_OP_lit26`](#dw-op-lit26) | const |  |
| [`DW_OP_lit27`](#dw-op-lit27) | const |  |
| [`DW_OP_lit28`](#dw-op-lit28) | const |  |
| [`DW_OP_lit29`](#dw-op-lit29) | const |  |
| [`DW_OP_lit30`](#dw-op-lit30) | const |  |
| [`DW_OP_lit31`](#dw-op-lit31) | const |  |
| [`DW_OP_reg0`](#dw-op-reg0) | const |  |
| [`DW_OP_reg1`](#dw-op-reg1) | const |  |
| [`DW_OP_reg2`](#dw-op-reg2) | const |  |
| [`DW_OP_reg3`](#dw-op-reg3) | const |  |
| [`DW_OP_reg4`](#dw-op-reg4) | const |  |
| [`DW_OP_reg5`](#dw-op-reg5) | const |  |
| [`DW_OP_reg6`](#dw-op-reg6) | const |  |
| [`DW_OP_reg7`](#dw-op-reg7) | const |  |
| [`DW_OP_reg8`](#dw-op-reg8) | const |  |
| [`DW_OP_reg9`](#dw-op-reg9) | const |  |
| [`DW_OP_reg10`](#dw-op-reg10) | const |  |
| [`DW_OP_reg11`](#dw-op-reg11) | const |  |
| [`DW_OP_reg12`](#dw-op-reg12) | const |  |
| [`DW_OP_reg13`](#dw-op-reg13) | const |  |
| [`DW_OP_reg14`](#dw-op-reg14) | const |  |
| [`DW_OP_reg15`](#dw-op-reg15) | const |  |
| [`DW_OP_reg16`](#dw-op-reg16) | const |  |
| [`DW_OP_reg17`](#dw-op-reg17) | const |  |
| [`DW_OP_reg18`](#dw-op-reg18) | const |  |
| [`DW_OP_reg19`](#dw-op-reg19) | const |  |
| [`DW_OP_reg20`](#dw-op-reg20) | const |  |
| [`DW_OP_reg21`](#dw-op-reg21) | const |  |
| [`DW_OP_reg22`](#dw-op-reg22) | const |  |
| [`DW_OP_reg23`](#dw-op-reg23) | const |  |
| [`DW_OP_reg24`](#dw-op-reg24) | const |  |
| [`DW_OP_reg25`](#dw-op-reg25) | const |  |
| [`DW_OP_reg26`](#dw-op-reg26) | const |  |
| [`DW_OP_reg27`](#dw-op-reg27) | const |  |
| [`DW_OP_reg28`](#dw-op-reg28) | const |  |
| [`DW_OP_reg29`](#dw-op-reg29) | const |  |
| [`DW_OP_reg30`](#dw-op-reg30) | const |  |
| [`DW_OP_reg31`](#dw-op-reg31) | const |  |
| [`DW_OP_breg0`](#dw-op-breg0) | const |  |
| [`DW_OP_breg1`](#dw-op-breg1) | const |  |
| [`DW_OP_breg2`](#dw-op-breg2) | const |  |
| [`DW_OP_breg3`](#dw-op-breg3) | const |  |
| [`DW_OP_breg4`](#dw-op-breg4) | const |  |
| [`DW_OP_breg5`](#dw-op-breg5) | const |  |
| [`DW_OP_breg6`](#dw-op-breg6) | const |  |
| [`DW_OP_breg7`](#dw-op-breg7) | const |  |
| [`DW_OP_breg8`](#dw-op-breg8) | const |  |
| [`DW_OP_breg9`](#dw-op-breg9) | const |  |
| [`DW_OP_breg10`](#dw-op-breg10) | const |  |
| [`DW_OP_breg11`](#dw-op-breg11) | const |  |
| [`DW_OP_breg12`](#dw-op-breg12) | const |  |
| [`DW_OP_breg13`](#dw-op-breg13) | const |  |
| [`DW_OP_breg14`](#dw-op-breg14) | const |  |
| [`DW_OP_breg15`](#dw-op-breg15) | const |  |
| [`DW_OP_breg16`](#dw-op-breg16) | const |  |
| [`DW_OP_breg17`](#dw-op-breg17) | const |  |
| [`DW_OP_breg18`](#dw-op-breg18) | const |  |
| [`DW_OP_breg19`](#dw-op-breg19) | const |  |
| [`DW_OP_breg20`](#dw-op-breg20) | const |  |
| [`DW_OP_breg21`](#dw-op-breg21) | const |  |
| [`DW_OP_breg22`](#dw-op-breg22) | const |  |
| [`DW_OP_breg23`](#dw-op-breg23) | const |  |
| [`DW_OP_breg24`](#dw-op-breg24) | const |  |
| [`DW_OP_breg25`](#dw-op-breg25) | const |  |
| [`DW_OP_breg26`](#dw-op-breg26) | const |  |
| [`DW_OP_breg27`](#dw-op-breg27) | const |  |
| [`DW_OP_breg28`](#dw-op-breg28) | const |  |
| [`DW_OP_breg29`](#dw-op-breg29) | const |  |
| [`DW_OP_breg30`](#dw-op-breg30) | const |  |
| [`DW_OP_breg31`](#dw-op-breg31) | const |  |
| [`DW_OP_regx`](#dw-op-regx) | const |  |
| [`DW_OP_fbreg`](#dw-op-fbreg) | const |  |
| [`DW_OP_bregx`](#dw-op-bregx) | const |  |
| [`DW_OP_piece`](#dw-op-piece) | const |  |
| [`DW_OP_deref_size`](#dw-op-deref-size) | const |  |
| [`DW_OP_xderef_size`](#dw-op-xderef-size) | const |  |
| [`DW_OP_nop`](#dw-op-nop) | const |  |
| [`DW_OP_push_object_address`](#dw-op-push-object-address) | const |  |
| [`DW_OP_call2`](#dw-op-call2) | const |  |
| [`DW_OP_call4`](#dw-op-call4) | const |  |
| [`DW_OP_call_ref`](#dw-op-call-ref) | const |  |
| [`DW_OP_form_tls_address`](#dw-op-form-tls-address) | const |  |
| [`DW_OP_call_frame_cfa`](#dw-op-call-frame-cfa) | const |  |
| [`DW_OP_bit_piece`](#dw-op-bit-piece) | const |  |
| [`DW_OP_implicit_value`](#dw-op-implicit-value) | const |  |
| [`DW_OP_stack_value`](#dw-op-stack-value) | const |  |
| [`DW_OP_implicit_pointer`](#dw-op-implicit-pointer) | const |  |
| [`DW_OP_addrx`](#dw-op-addrx) | const |  |
| [`DW_OP_constx`](#dw-op-constx) | const |  |
| [`DW_OP_entry_value`](#dw-op-entry-value) | const |  |
| [`DW_OP_const_type`](#dw-op-const-type) | const |  |
| [`DW_OP_regval_type`](#dw-op-regval-type) | const |  |
| [`DW_OP_deref_type`](#dw-op-deref-type) | const |  |
| [`DW_OP_xderef_type`](#dw-op-xderef-type) | const |  |
| [`DW_OP_convert`](#dw-op-convert) | const |  |
| [`DW_OP_reinterpret`](#dw-op-reinterpret) | const |  |
| [`DW_OP_GNU_push_tls_address`](#dw-op-gnu-push-tls-address) | const |  |
| [`DW_OP_GNU_implicit_pointer`](#dw-op-gnu-implicit-pointer) | const |  |
| [`DW_OP_GNU_entry_value`](#dw-op-gnu-entry-value) | const |  |
| [`DW_OP_GNU_const_type`](#dw-op-gnu-const-type) | const |  |
| [`DW_OP_GNU_regval_type`](#dw-op-gnu-regval-type) | const |  |
| [`DW_OP_GNU_deref_type`](#dw-op-gnu-deref-type) | const |  |
| [`DW_OP_GNU_convert`](#dw-op-gnu-convert) | const |  |
| [`DW_OP_GNU_reinterpret`](#dw-op-gnu-reinterpret) | const |  |
| [`DW_OP_GNU_parameter_ref`](#dw-op-gnu-parameter-ref) | const |  |
| [`DW_OP_GNU_addr_index`](#dw-op-gnu-addr-index) | const |  |
| [`DW_OP_GNU_const_index`](#dw-op-gnu-const-index) | const |  |
| [`DW_OP_WASM_location`](#dw-op-wasm-location) | const |  |
| [`DW_EH_PE_uleb128`](#dw-eh-pe-uleb128) | const |  |
| [`DW_EH_PE_udata2`](#dw-eh-pe-udata2) | const |  |
| [`DW_EH_PE_udata4`](#dw-eh-pe-udata4) | const |  |
| [`DW_EH_PE_udata8`](#dw-eh-pe-udata8) | const |  |
| [`DW_EH_PE_sleb128`](#dw-eh-pe-sleb128) | const |  |
| [`DW_EH_PE_sdata2`](#dw-eh-pe-sdata2) | const |  |
| [`DW_EH_PE_sdata4`](#dw-eh-pe-sdata4) | const |  |
| [`DW_EH_PE_sdata8`](#dw-eh-pe-sdata8) | const |  |
| [`DW_EH_PE_pcrel`](#dw-eh-pe-pcrel) | const |  |
| [`DW_EH_PE_textrel`](#dw-eh-pe-textrel) | const |  |
| [`DW_EH_PE_datarel`](#dw-eh-pe-datarel) | const |  |
| [`DW_EH_PE_funcrel`](#dw-eh-pe-funcrel) | const |  |
| [`DW_EH_PE_aligned`](#dw-eh-pe-aligned) | const |  |
| [`DW_EH_PE_indirect`](#dw-eh-pe-indirect) | const |  |
| [`DW_EH_PE_absptr`](#dw-eh-pe-absptr) | const |  |
| [`DW_EH_PE_omit`](#dw-eh-pe-omit) | const |  |
| [`DW_EH_PE_FORMAT_MASK`](#dw-eh-pe-format-mask) | const |  |
| [`DW_EH_PE_APPLICATION_MASK`](#dw-eh-pe-application-mask) | const |  |
| [`dw!`](#dw) | macro |  |

## Structs

### `DwSect`

```rust
struct DwSect(u32);
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

#### Implementations

- <span id="dwsect-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSect`

- <span id="dwsect-clone"></span>`fn clone(&self) -> DwSect` — [`DwSect`](../index.md#dwsect)

##### `impl Copy for DwSect`

##### `impl Debug for DwSect`

- <span id="dwsect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSect`

- <span id="dwsect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSect`

##### `impl Hash for DwSect`

- <span id="dwsect-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSect`

- <span id="dwsect-cmp"></span>`fn cmp(&self, other: &DwSect) -> cmp::Ordering` — [`DwSect`](../index.md#dwsect)

##### `impl PartialEq for DwSect`

- <span id="dwsect-eq"></span>`fn eq(&self, other: &DwSect) -> bool` — [`DwSect`](../index.md#dwsect)

##### `impl PartialOrd for DwSect`

- <span id="dwsect-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSect) -> option::Option<cmp::Ordering>` — [`DwSect`](../index.md#dwsect)

##### `impl StructuralPartialEq for DwSect`

##### `impl ToString for DwSect`

- <span id="dwsect-to-string"></span>`fn to_string(&self) -> String`

### `DwSectV2`

```rust
struct DwSectV2(u32);
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

The section type field in a `.dwp` unit index with version 2.

#### Implementations

- <span id="dwsectv2-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSectV2`

- <span id="dwsectv2-clone"></span>`fn clone(&self) -> DwSectV2` — [`DwSectV2`](../index.md#dwsectv2)

##### `impl Copy for DwSectV2`

##### `impl Debug for DwSectV2`

- <span id="dwsectv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSectV2`

- <span id="dwsectv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSectV2`

##### `impl Hash for DwSectV2`

- <span id="dwsectv2-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSectV2`

- <span id="dwsectv2-cmp"></span>`fn cmp(&self, other: &DwSectV2) -> cmp::Ordering` — [`DwSectV2`](../index.md#dwsectv2)

##### `impl PartialEq for DwSectV2`

- <span id="dwsectv2-eq"></span>`fn eq(&self, other: &DwSectV2) -> bool` — [`DwSectV2`](../index.md#dwsectv2)

##### `impl PartialOrd for DwSectV2`

- <span id="dwsectv2-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSectV2) -> option::Option<cmp::Ordering>` — [`DwSectV2`](../index.md#dwsectv2)

##### `impl StructuralPartialEq for DwSectV2`

##### `impl ToString for DwSectV2`

- <span id="dwsectv2-to-string"></span>`fn to_string(&self) -> String`

### `DwUt`

```rust
struct DwUt(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

#### Implementations

- <span id="dwut-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwUt`

- <span id="dwut-clone"></span>`fn clone(&self) -> DwUt` — [`DwUt`](../index.md#dwut)

##### `impl Copy for DwUt`

##### `impl Debug for DwUt`

- <span id="dwut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwUt`

- <span id="dwut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwUt`

##### `impl Hash for DwUt`

- <span id="dwut-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwUt`

- <span id="dwut-cmp"></span>`fn cmp(&self, other: &DwUt) -> cmp::Ordering` — [`DwUt`](../index.md#dwut)

##### `impl PartialEq for DwUt`

- <span id="dwut-eq"></span>`fn eq(&self, other: &DwUt) -> bool` — [`DwUt`](../index.md#dwut)

##### `impl PartialOrd for DwUt`

- <span id="dwut-partial-cmp"></span>`fn partial_cmp(&self, other: &DwUt) -> option::Option<cmp::Ordering>` — [`DwUt`](../index.md#dwut)

##### `impl StructuralPartialEq for DwUt`

##### `impl ToString for DwUt`

- <span id="dwut-to-string"></span>`fn to_string(&self) -> String`

### `DwCfa`

```rust
struct DwCfa(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

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

- <span id="dwcfa-clone"></span>`fn clone(&self) -> DwCfa` — [`DwCfa`](../index.md#dwcfa)

##### `impl Copy for DwCfa`

##### `impl Debug for DwCfa`

- <span id="dwcfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCfa`

- <span id="dwcfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCfa`

##### `impl Hash for DwCfa`

- <span id="dwcfa-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCfa`

- <span id="dwcfa-cmp"></span>`fn cmp(&self, other: &DwCfa) -> cmp::Ordering` — [`DwCfa`](../index.md#dwcfa)

##### `impl PartialEq for DwCfa`

- <span id="dwcfa-eq"></span>`fn eq(&self, other: &DwCfa) -> bool` — [`DwCfa`](../index.md#dwcfa)

##### `impl PartialOrd for DwCfa`

- <span id="dwcfa-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCfa) -> option::Option<cmp::Ordering>` — [`DwCfa`](../index.md#dwcfa)

##### `impl StructuralPartialEq for DwCfa`

##### `impl ToString for DwCfa`

- <span id="dwcfa-to-string"></span>`fn to_string(&self) -> String`

### `DwChildren`

```rust
struct DwChildren(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:196-203`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L196-L203)*

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

#### Implementations

- <span id="dwchildren-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwChildren`

- <span id="dwchildren-clone"></span>`fn clone(&self) -> DwChildren` — [`DwChildren`](../index.md#dwchildren)

##### `impl Copy for DwChildren`

##### `impl Debug for DwChildren`

- <span id="dwchildren-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwChildren`

- <span id="dwchildren-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwChildren`

##### `impl Hash for DwChildren`

- <span id="dwchildren-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwChildren`

- <span id="dwchildren-cmp"></span>`fn cmp(&self, other: &DwChildren) -> cmp::Ordering` — [`DwChildren`](../index.md#dwchildren)

##### `impl PartialEq for DwChildren`

- <span id="dwchildren-eq"></span>`fn eq(&self, other: &DwChildren) -> bool` — [`DwChildren`](../index.md#dwchildren)

##### `impl PartialOrd for DwChildren`

- <span id="dwchildren-partial-cmp"></span>`fn partial_cmp(&self, other: &DwChildren) -> option::Option<cmp::Ordering>` — [`DwChildren`](../index.md#dwchildren)

##### `impl StructuralPartialEq for DwChildren`

##### `impl ToString for DwChildren`

- <span id="dwchildren-to-string"></span>`fn to_string(&self) -> String`

### `DwTag`

```rust
struct DwTag(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

#### Implementations

- <span id="dwtag-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwTag`

- <span id="dwtag-clone"></span>`fn clone(&self) -> DwTag` — [`DwTag`](../index.md#dwtag)

##### `impl Copy for DwTag`

##### `impl Debug for DwTag`

- <span id="dwtag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwTag`

- <span id="dwtag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwTag`

##### `impl Hash for DwTag`

- <span id="dwtag-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwTag`

- <span id="dwtag-cmp"></span>`fn cmp(&self, other: &DwTag) -> cmp::Ordering` — [`DwTag`](../index.md#dwtag)

##### `impl PartialEq for DwTag`

- <span id="dwtag-eq"></span>`fn eq(&self, other: &DwTag) -> bool` — [`DwTag`](../index.md#dwtag)

##### `impl PartialOrd for DwTag`

- <span id="dwtag-partial-cmp"></span>`fn partial_cmp(&self, other: &DwTag) -> option::Option<cmp::Ordering>` — [`DwTag`](../index.md#dwtag)

##### `impl StructuralPartialEq for DwTag`

##### `impl ToString for DwTag`

- <span id="dwtag-to-string"></span>`fn to_string(&self) -> String`

### `DwAt`

```rust
struct DwAt(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

#### Implementations

- <span id="dwat-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAt`

- <span id="dwat-clone"></span>`fn clone(&self) -> DwAt` — [`DwAt`](../index.md#dwat)

##### `impl Copy for DwAt`

##### `impl Debug for DwAt`

- <span id="dwat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAt`

- <span id="dwat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAt`

##### `impl Hash for DwAt`

- <span id="dwat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAt`

- <span id="dwat-cmp"></span>`fn cmp(&self, other: &DwAt) -> cmp::Ordering` — [`DwAt`](../index.md#dwat)

##### `impl PartialEq for DwAt`

- <span id="dwat-eq"></span>`fn eq(&self, other: &DwAt) -> bool` — [`DwAt`](../index.md#dwat)

##### `impl PartialOrd for DwAt`

- <span id="dwat-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAt) -> option::Option<cmp::Ordering>` — [`DwAt`](../index.md#dwat)

##### `impl StructuralPartialEq for DwAt`

##### `impl ToString for DwAt`

- <span id="dwat-to-string"></span>`fn to_string(&self) -> String`

### `DwForm`

```rust
struct DwForm(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

#### Implementations

- <span id="dwform-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwForm`

- <span id="dwform-clone"></span>`fn clone(&self) -> DwForm` — [`DwForm`](../index.md#dwform)

##### `impl Copy for DwForm`

##### `impl Debug for DwForm`

- <span id="dwform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwForm`

- <span id="dwform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwForm`

##### `impl Hash for DwForm`

- <span id="dwform-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwForm`

- <span id="dwform-cmp"></span>`fn cmp(&self, other: &DwForm) -> cmp::Ordering` — [`DwForm`](../index.md#dwform)

##### `impl PartialEq for DwForm`

- <span id="dwform-eq"></span>`fn eq(&self, other: &DwForm) -> bool` — [`DwForm`](../index.md#dwform)

##### `impl PartialOrd for DwForm`

- <span id="dwform-partial-cmp"></span>`fn partial_cmp(&self, other: &DwForm) -> option::Option<cmp::Ordering>` — [`DwForm`](../index.md#dwform)

##### `impl StructuralPartialEq for DwForm`

##### `impl ToString for DwForm`

- <span id="dwform-to-string"></span>`fn to_string(&self) -> String`

### `DwAte`

```rust
struct DwAte(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

#### Implementations

- <span id="dwate-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAte`

- <span id="dwate-clone"></span>`fn clone(&self) -> DwAte` — [`DwAte`](../index.md#dwate)

##### `impl Copy for DwAte`

##### `impl Debug for DwAte`

- <span id="dwate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAte`

- <span id="dwate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAte`

##### `impl Hash for DwAte`

- <span id="dwate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAte`

- <span id="dwate-cmp"></span>`fn cmp(&self, other: &DwAte) -> cmp::Ordering` — [`DwAte`](../index.md#dwate)

##### `impl PartialEq for DwAte`

- <span id="dwate-eq"></span>`fn eq(&self, other: &DwAte) -> bool` — [`DwAte`](../index.md#dwate)

##### `impl PartialOrd for DwAte`

- <span id="dwate-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAte) -> option::Option<cmp::Ordering>` — [`DwAte`](../index.md#dwate)

##### `impl StructuralPartialEq for DwAte`

##### `impl ToString for DwAte`

- <span id="dwate-to-string"></span>`fn to_string(&self) -> String`

### `DwLle`

```rust
struct DwLle(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

#### Implementations

- <span id="dwlle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLle`

- <span id="dwlle-clone"></span>`fn clone(&self) -> DwLle` — [`DwLle`](../index.md#dwlle)

##### `impl Copy for DwLle`

##### `impl Debug for DwLle`

- <span id="dwlle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLle`

- <span id="dwlle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLle`

##### `impl Hash for DwLle`

- <span id="dwlle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLle`

- <span id="dwlle-cmp"></span>`fn cmp(&self, other: &DwLle) -> cmp::Ordering` — [`DwLle`](../index.md#dwlle)

##### `impl PartialEq for DwLle`

- <span id="dwlle-eq"></span>`fn eq(&self, other: &DwLle) -> bool` — [`DwLle`](../index.md#dwlle)

##### `impl PartialOrd for DwLle`

- <span id="dwlle-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLle) -> option::Option<cmp::Ordering>` — [`DwLle`](../index.md#dwlle)

##### `impl StructuralPartialEq for DwLle`

##### `impl ToString for DwLle`

- <span id="dwlle-to-string"></span>`fn to_string(&self) -> String`

### `DwDs`

```rust
struct DwDs(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

#### Implementations

- <span id="dwds-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDs`

- <span id="dwds-clone"></span>`fn clone(&self) -> DwDs` — [`DwDs`](../index.md#dwds)

##### `impl Copy for DwDs`

##### `impl Debug for DwDs`

- <span id="dwds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDs`

- <span id="dwds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDs`

##### `impl Hash for DwDs`

- <span id="dwds-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDs`

- <span id="dwds-cmp"></span>`fn cmp(&self, other: &DwDs) -> cmp::Ordering` — [`DwDs`](../index.md#dwds)

##### `impl PartialEq for DwDs`

- <span id="dwds-eq"></span>`fn eq(&self, other: &DwDs) -> bool` — [`DwDs`](../index.md#dwds)

##### `impl PartialOrd for DwDs`

- <span id="dwds-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDs) -> option::Option<cmp::Ordering>` — [`DwDs`](../index.md#dwds)

##### `impl StructuralPartialEq for DwDs`

##### `impl ToString for DwDs`

- <span id="dwds-to-string"></span>`fn to_string(&self) -> String`

### `DwEnd`

```rust
struct DwEnd(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

#### Implementations

- <span id="dwend-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwEnd`

- <span id="dwend-clone"></span>`fn clone(&self) -> DwEnd` — [`DwEnd`](../index.md#dwend)

##### `impl Copy for DwEnd`

##### `impl Debug for DwEnd`

- <span id="dwend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEnd`

- <span id="dwend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEnd`

##### `impl Hash for DwEnd`

- <span id="dwend-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEnd`

- <span id="dwend-cmp"></span>`fn cmp(&self, other: &DwEnd) -> cmp::Ordering` — [`DwEnd`](../index.md#dwend)

##### `impl PartialEq for DwEnd`

- <span id="dwend-eq"></span>`fn eq(&self, other: &DwEnd) -> bool` — [`DwEnd`](../index.md#dwend)

##### `impl PartialOrd for DwEnd`

- <span id="dwend-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEnd) -> option::Option<cmp::Ordering>` — [`DwEnd`](../index.md#dwend)

##### `impl StructuralPartialEq for DwEnd`

##### `impl ToString for DwEnd`

- <span id="dwend-to-string"></span>`fn to_string(&self) -> String`

### `DwAccess`

```rust
struct DwAccess(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:834-842`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L834-L842)*

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

#### Implementations

- <span id="dwaccess-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAccess`

- <span id="dwaccess-clone"></span>`fn clone(&self) -> DwAccess` — [`DwAccess`](../index.md#dwaccess)

##### `impl Copy for DwAccess`

##### `impl Debug for DwAccess`

- <span id="dwaccess-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAccess`

- <span id="dwaccess-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAccess`

##### `impl Hash for DwAccess`

- <span id="dwaccess-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAccess`

- <span id="dwaccess-cmp"></span>`fn cmp(&self, other: &DwAccess) -> cmp::Ordering` — [`DwAccess`](../index.md#dwaccess)

##### `impl PartialEq for DwAccess`

- <span id="dwaccess-eq"></span>`fn eq(&self, other: &DwAccess) -> bool` — [`DwAccess`](../index.md#dwaccess)

##### `impl PartialOrd for DwAccess`

- <span id="dwaccess-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAccess) -> option::Option<cmp::Ordering>` — [`DwAccess`](../index.md#dwaccess)

##### `impl StructuralPartialEq for DwAccess`

##### `impl ToString for DwAccess`

- <span id="dwaccess-to-string"></span>`fn to_string(&self) -> String`

### `DwVis`

```rust
struct DwVis(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:844-852`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L844-L852)*

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

#### Implementations

- <span id="dwvis-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVis`

- <span id="dwvis-clone"></span>`fn clone(&self) -> DwVis` — [`DwVis`](../index.md#dwvis)

##### `impl Copy for DwVis`

##### `impl Debug for DwVis`

- <span id="dwvis-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVis`

- <span id="dwvis-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVis`

##### `impl Hash for DwVis`

- <span id="dwvis-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVis`

- <span id="dwvis-cmp"></span>`fn cmp(&self, other: &DwVis) -> cmp::Ordering` — [`DwVis`](../index.md#dwvis)

##### `impl PartialEq for DwVis`

- <span id="dwvis-eq"></span>`fn eq(&self, other: &DwVis) -> bool` — [`DwVis`](../index.md#dwvis)

##### `impl PartialOrd for DwVis`

- <span id="dwvis-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVis) -> option::Option<cmp::Ordering>` — [`DwVis`](../index.md#dwvis)

##### `impl StructuralPartialEq for DwVis`

##### `impl ToString for DwVis`

- <span id="dwvis-to-string"></span>`fn to_string(&self) -> String`

### `DwVirtuality`

```rust
struct DwVirtuality(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:854-862`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L854-L862)*

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

#### Implementations

- <span id="dwvirtuality-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVirtuality`

- <span id="dwvirtuality-clone"></span>`fn clone(&self) -> DwVirtuality` — [`DwVirtuality`](../index.md#dwvirtuality)

##### `impl Copy for DwVirtuality`

##### `impl Debug for DwVirtuality`

- <span id="dwvirtuality-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVirtuality`

- <span id="dwvirtuality-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVirtuality`

##### `impl Hash for DwVirtuality`

- <span id="dwvirtuality-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVirtuality`

- <span id="dwvirtuality-cmp"></span>`fn cmp(&self, other: &DwVirtuality) -> cmp::Ordering` — [`DwVirtuality`](../index.md#dwvirtuality)

##### `impl PartialEq for DwVirtuality`

- <span id="dwvirtuality-eq"></span>`fn eq(&self, other: &DwVirtuality) -> bool` — [`DwVirtuality`](../index.md#dwvirtuality)

##### `impl PartialOrd for DwVirtuality`

- <span id="dwvirtuality-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVirtuality) -> option::Option<cmp::Ordering>` — [`DwVirtuality`](../index.md#dwvirtuality)

##### `impl StructuralPartialEq for DwVirtuality`

##### `impl ToString for DwVirtuality`

- <span id="dwvirtuality-to-string"></span>`fn to_string(&self) -> String`

### `DwLang`

```rust
struct DwLang(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

#### Implementations

- <span id="dwlang-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLang`

- <span id="dwlang-clone"></span>`fn clone(&self) -> DwLang` — [`DwLang`](../index.md#dwlang)

##### `impl Copy for DwLang`

##### `impl Debug for DwLang`

- <span id="dwlang-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLang`

- <span id="dwlang-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLang`

##### `impl Hash for DwLang`

- <span id="dwlang-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLang`

- <span id="dwlang-cmp"></span>`fn cmp(&self, other: &DwLang) -> cmp::Ordering` — [`DwLang`](../index.md#dwlang)

##### `impl PartialEq for DwLang`

- <span id="dwlang-eq"></span>`fn eq(&self, other: &DwLang) -> bool` — [`DwLang`](../index.md#dwlang)

##### `impl PartialOrd for DwLang`

- <span id="dwlang-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLang) -> option::Option<cmp::Ordering>` — [`DwLang`](../index.md#dwlang)

##### `impl StructuralPartialEq for DwLang`

##### `impl ToString for DwLang`

- <span id="dwlang-to-string"></span>`fn to_string(&self) -> String`

### `DwAddr`

```rust
struct DwAddr(u64);
```

*Defined in [`gimli-0.32.3/src/constants.rs:962-969`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L962-L969)*

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

#### Implementations

- <span id="dwaddr-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAddr`

- <span id="dwaddr-clone"></span>`fn clone(&self) -> DwAddr` — [`DwAddr`](../index.md#dwaddr)

##### `impl Copy for DwAddr`

##### `impl Debug for DwAddr`

- <span id="dwaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAddr`

- <span id="dwaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAddr`

##### `impl Hash for DwAddr`

- <span id="dwaddr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAddr`

- <span id="dwaddr-cmp"></span>`fn cmp(&self, other: &DwAddr) -> cmp::Ordering` — [`DwAddr`](../index.md#dwaddr)

##### `impl PartialEq for DwAddr`

- <span id="dwaddr-eq"></span>`fn eq(&self, other: &DwAddr) -> bool` — [`DwAddr`](../index.md#dwaddr)

##### `impl PartialOrd for DwAddr`

- <span id="dwaddr-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAddr) -> option::Option<cmp::Ordering>` — [`DwAddr`](../index.md#dwaddr)

##### `impl StructuralPartialEq for DwAddr`

##### `impl ToString for DwAddr`

- <span id="dwaddr-to-string"></span>`fn to_string(&self) -> String`

### `DwId`

```rust
struct DwId(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:971-980`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L971-L980)*

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

#### Implementations

- <span id="dwid-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwId`

- <span id="dwid-clone"></span>`fn clone(&self) -> DwId` — [`DwId`](../index.md#dwid)

##### `impl Copy for DwId`

##### `impl Debug for DwId`

- <span id="dwid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwId`

- <span id="dwid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwId`

##### `impl Hash for DwId`

- <span id="dwid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwId`

- <span id="dwid-cmp"></span>`fn cmp(&self, other: &DwId) -> cmp::Ordering` — [`DwId`](../index.md#dwid)

##### `impl PartialEq for DwId`

- <span id="dwid-eq"></span>`fn eq(&self, other: &DwId) -> bool` — [`DwId`](../index.md#dwid)

##### `impl PartialOrd for DwId`

- <span id="dwid-partial-cmp"></span>`fn partial_cmp(&self, other: &DwId) -> option::Option<cmp::Ordering>` — [`DwId`](../index.md#dwid)

##### `impl StructuralPartialEq for DwId`

##### `impl ToString for DwId`

- <span id="dwid-to-string"></span>`fn to_string(&self) -> String`

### `DwCc`

```rust
struct DwCc(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

#### Implementations

- <span id="dwcc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCc`

- <span id="dwcc-clone"></span>`fn clone(&self) -> DwCc` — [`DwCc`](../index.md#dwcc)

##### `impl Copy for DwCc`

##### `impl Debug for DwCc`

- <span id="dwcc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCc`

- <span id="dwcc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCc`

##### `impl Hash for DwCc`

- <span id="dwcc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCc`

- <span id="dwcc-cmp"></span>`fn cmp(&self, other: &DwCc) -> cmp::Ordering` — [`DwCc`](../index.md#dwcc)

##### `impl PartialEq for DwCc`

- <span id="dwcc-eq"></span>`fn eq(&self, other: &DwCc) -> bool` — [`DwCc`](../index.md#dwcc)

##### `impl PartialOrd for DwCc`

- <span id="dwcc-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCc) -> option::Option<cmp::Ordering>` — [`DwCc`](../index.md#dwcc)

##### `impl StructuralPartialEq for DwCc`

##### `impl ToString for DwCc`

- <span id="dwcc-to-string"></span>`fn to_string(&self) -> String`

### `DwInl`

```rust
struct DwInl(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:996-1005`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L996-L1005)*

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

#### Implementations

- <span id="dwinl-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwInl`

- <span id="dwinl-clone"></span>`fn clone(&self) -> DwInl` — [`DwInl`](../index.md#dwinl)

##### `impl Copy for DwInl`

##### `impl Debug for DwInl`

- <span id="dwinl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwInl`

- <span id="dwinl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwInl`

##### `impl Hash for DwInl`

- <span id="dwinl-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwInl`

- <span id="dwinl-cmp"></span>`fn cmp(&self, other: &DwInl) -> cmp::Ordering` — [`DwInl`](../index.md#dwinl)

##### `impl PartialEq for DwInl`

- <span id="dwinl-eq"></span>`fn eq(&self, other: &DwInl) -> bool` — [`DwInl`](../index.md#dwinl)

##### `impl PartialOrd for DwInl`

- <span id="dwinl-partial-cmp"></span>`fn partial_cmp(&self, other: &DwInl) -> option::Option<cmp::Ordering>` — [`DwInl`](../index.md#dwinl)

##### `impl StructuralPartialEq for DwInl`

##### `impl ToString for DwInl`

- <span id="dwinl-to-string"></span>`fn to_string(&self) -> String`

### `DwOrd`

```rust
struct DwOrd(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1007-1014`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1007-L1014)*

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

#### Implementations

- <span id="dword-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOrd`

- <span id="dword-clone"></span>`fn clone(&self) -> DwOrd` — [`DwOrd`](../index.md#dword)

##### `impl Copy for DwOrd`

##### `impl Debug for DwOrd`

- <span id="dword-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOrd`

- <span id="dword-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOrd`

##### `impl Hash for DwOrd`

- <span id="dword-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOrd`

- <span id="dword-cmp"></span>`fn cmp(&self, other: &DwOrd) -> cmp::Ordering` — [`DwOrd`](../index.md#dword)

##### `impl PartialEq for DwOrd`

- <span id="dword-eq"></span>`fn eq(&self, other: &DwOrd) -> bool` — [`DwOrd`](../index.md#dword)

##### `impl PartialOrd for DwOrd`

- <span id="dword-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOrd) -> option::Option<cmp::Ordering>` — [`DwOrd`](../index.md#dword)

##### `impl StructuralPartialEq for DwOrd`

##### `impl ToString for DwOrd`

- <span id="dword-to-string"></span>`fn to_string(&self) -> String`

### `DwDsc`

```rust
struct DwDsc(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1016-1023`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1016-L1023)*

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

#### Implementations

- <span id="dwdsc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDsc`

- <span id="dwdsc-clone"></span>`fn clone(&self) -> DwDsc` — [`DwDsc`](../index.md#dwdsc)

##### `impl Copy for DwDsc`

##### `impl Debug for DwDsc`

- <span id="dwdsc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDsc`

- <span id="dwdsc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDsc`

##### `impl Hash for DwDsc`

- <span id="dwdsc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDsc`

- <span id="dwdsc-cmp"></span>`fn cmp(&self, other: &DwDsc) -> cmp::Ordering` — [`DwDsc`](../index.md#dwdsc)

##### `impl PartialEq for DwDsc`

- <span id="dwdsc-eq"></span>`fn eq(&self, other: &DwDsc) -> bool` — [`DwDsc`](../index.md#dwdsc)

##### `impl PartialOrd for DwDsc`

- <span id="dwdsc-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDsc) -> option::Option<cmp::Ordering>` — [`DwDsc`](../index.md#dwdsc)

##### `impl StructuralPartialEq for DwDsc`

##### `impl ToString for DwDsc`

- <span id="dwdsc-to-string"></span>`fn to_string(&self) -> String`

### `DwIdx`

```rust
struct DwIdx(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

Name index attribute encodings.

See Section 7.19, Table 7.23.

#### Implementations

- <span id="dwidx-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwIdx`

- <span id="dwidx-clone"></span>`fn clone(&self) -> DwIdx` — [`DwIdx`](../index.md#dwidx)

##### `impl Copy for DwIdx`

##### `impl Debug for DwIdx`

- <span id="dwidx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwIdx`

- <span id="dwidx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwIdx`

##### `impl Hash for DwIdx`

- <span id="dwidx-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwIdx`

- <span id="dwidx-cmp"></span>`fn cmp(&self, other: &DwIdx) -> cmp::Ordering` — [`DwIdx`](../index.md#dwidx)

##### `impl PartialEq for DwIdx`

- <span id="dwidx-eq"></span>`fn eq(&self, other: &DwIdx) -> bool` — [`DwIdx`](../index.md#dwidx)

##### `impl PartialOrd for DwIdx`

- <span id="dwidx-partial-cmp"></span>`fn partial_cmp(&self, other: &DwIdx) -> option::Option<cmp::Ordering>` — [`DwIdx`](../index.md#dwidx)

##### `impl StructuralPartialEq for DwIdx`

##### `impl ToString for DwIdx`

- <span id="dwidx-to-string"></span>`fn to_string(&self) -> String`

### `DwDefaulted`

```rust
struct DwDefaulted(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1039-1047`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1039-L1047)*

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

#### Implementations

- <span id="dwdefaulted-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDefaulted`

- <span id="dwdefaulted-clone"></span>`fn clone(&self) -> DwDefaulted` — [`DwDefaulted`](../index.md#dwdefaulted)

##### `impl Copy for DwDefaulted`

##### `impl Debug for DwDefaulted`

- <span id="dwdefaulted-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDefaulted`

- <span id="dwdefaulted-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDefaulted`

##### `impl Hash for DwDefaulted`

- <span id="dwdefaulted-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDefaulted`

- <span id="dwdefaulted-cmp"></span>`fn cmp(&self, other: &DwDefaulted) -> cmp::Ordering` — [`DwDefaulted`](../index.md#dwdefaulted)

##### `impl PartialEq for DwDefaulted`

- <span id="dwdefaulted-eq"></span>`fn eq(&self, other: &DwDefaulted) -> bool` — [`DwDefaulted`](../index.md#dwdefaulted)

##### `impl PartialOrd for DwDefaulted`

- <span id="dwdefaulted-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDefaulted) -> option::Option<cmp::Ordering>` — [`DwDefaulted`](../index.md#dwdefaulted)

##### `impl StructuralPartialEq for DwDefaulted`

##### `impl ToString for DwDefaulted`

- <span id="dwdefaulted-to-string"></span>`fn to_string(&self) -> String`

### `DwLns`

```rust
struct DwLns(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

#### Implementations

- <span id="dwlns-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLns`

- <span id="dwlns-clone"></span>`fn clone(&self) -> DwLns` — [`DwLns`](../index.md#dwlns)

##### `impl Copy for DwLns`

##### `impl Debug for DwLns`

- <span id="dwlns-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLns`

- <span id="dwlns-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLns`

##### `impl Hash for DwLns`

- <span id="dwlns-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLns`

- <span id="dwlns-cmp"></span>`fn cmp(&self, other: &DwLns) -> cmp::Ordering` — [`DwLns`](../index.md#dwlns)

##### `impl PartialEq for DwLns`

- <span id="dwlns-eq"></span>`fn eq(&self, other: &DwLns) -> bool` — [`DwLns`](../index.md#dwlns)

##### `impl PartialOrd for DwLns`

- <span id="dwlns-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLns) -> option::Option<cmp::Ordering>` — [`DwLns`](../index.md#dwlns)

##### `impl StructuralPartialEq for DwLns`

##### `impl ToString for DwLns`

- <span id="dwlns-to-string"></span>`fn to_string(&self) -> String`

### `DwLne`

```rust
struct DwLne(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

#### Implementations

- <span id="dwlne-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLne`

- <span id="dwlne-clone"></span>`fn clone(&self) -> DwLne` — [`DwLne`](../index.md#dwlne)

##### `impl Copy for DwLne`

##### `impl Debug for DwLne`

- <span id="dwlne-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLne`

- <span id="dwlne-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLne`

##### `impl Hash for DwLne`

- <span id="dwlne-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLne`

- <span id="dwlne-cmp"></span>`fn cmp(&self, other: &DwLne) -> cmp::Ordering` — [`DwLne`](../index.md#dwlne)

##### `impl PartialEq for DwLne`

- <span id="dwlne-eq"></span>`fn eq(&self, other: &DwLne) -> bool` — [`DwLne`](../index.md#dwlne)

##### `impl PartialOrd for DwLne`

- <span id="dwlne-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLne) -> option::Option<cmp::Ordering>` — [`DwLne`](../index.md#dwlne)

##### `impl StructuralPartialEq for DwLne`

##### `impl ToString for DwLne`

- <span id="dwlne-to-string"></span>`fn to_string(&self) -> String`

### `DwLnct`

```rust
struct DwLnct(u16);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

#### Implementations

- <span id="dwlnct-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLnct`

- <span id="dwlnct-clone"></span>`fn clone(&self) -> DwLnct` — [`DwLnct`](../index.md#dwlnct)

##### `impl Copy for DwLnct`

##### `impl Debug for DwLnct`

- <span id="dwlnct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLnct`

- <span id="dwlnct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLnct`

##### `impl Hash for DwLnct`

- <span id="dwlnct-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLnct`

- <span id="dwlnct-cmp"></span>`fn cmp(&self, other: &DwLnct) -> cmp::Ordering` — [`DwLnct`](../index.md#dwlnct)

##### `impl PartialEq for DwLnct`

- <span id="dwlnct-eq"></span>`fn eq(&self, other: &DwLnct) -> bool` — [`DwLnct`](../index.md#dwlnct)

##### `impl PartialOrd for DwLnct`

- <span id="dwlnct-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLnct) -> option::Option<cmp::Ordering>` — [`DwLnct`](../index.md#dwlnct)

##### `impl StructuralPartialEq for DwLnct`

##### `impl ToString for DwLnct`

- <span id="dwlnct-to-string"></span>`fn to_string(&self) -> String`

### `DwMacinfo`

```rust
struct DwMacinfo(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

Type codes for macro definitions in the `.debug_macinfo` section.

See Section 7.22, Figure 39 for DWARF 4.

#### Implementations

- <span id="dwmacinfo-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacinfo`

- <span id="dwmacinfo-clone"></span>`fn clone(&self) -> DwMacinfo` — [`DwMacinfo`](../index.md#dwmacinfo)

##### `impl Copy for DwMacinfo`

##### `impl Debug for DwMacinfo`

- <span id="dwmacinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacinfo`

- <span id="dwmacinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacinfo`

##### `impl Hash for DwMacinfo`

- <span id="dwmacinfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacinfo`

- <span id="dwmacinfo-cmp"></span>`fn cmp(&self, other: &DwMacinfo) -> cmp::Ordering` — [`DwMacinfo`](../index.md#dwmacinfo)

##### `impl PartialEq for DwMacinfo`

- <span id="dwmacinfo-eq"></span>`fn eq(&self, other: &DwMacinfo) -> bool` — [`DwMacinfo`](../index.md#dwmacinfo)

##### `impl PartialOrd for DwMacinfo`

- <span id="dwmacinfo-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacinfo) -> option::Option<cmp::Ordering>` — [`DwMacinfo`](../index.md#dwmacinfo)

##### `impl StructuralPartialEq for DwMacinfo`

##### `impl ToString for DwMacinfo`

- <span id="dwmacinfo-to-string"></span>`fn to_string(&self) -> String`

### `DwMacro`

```rust
struct DwMacro(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

The encodings for macro information entry types.

See Section 7.23, Table 7.28 for DWARF 5.

#### Implementations

- <span id="dwmacro-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacro`

- <span id="dwmacro-clone"></span>`fn clone(&self) -> DwMacro` — [`DwMacro`](../index.md#dwmacro)

##### `impl Copy for DwMacro`

##### `impl Debug for DwMacro`

- <span id="dwmacro-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacro`

- <span id="dwmacro-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacro`

##### `impl Hash for DwMacro`

- <span id="dwmacro-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacro`

- <span id="dwmacro-cmp"></span>`fn cmp(&self, other: &DwMacro) -> cmp::Ordering` — [`DwMacro`](../index.md#dwmacro)

##### `impl PartialEq for DwMacro`

- <span id="dwmacro-eq"></span>`fn eq(&self, other: &DwMacro) -> bool` — [`DwMacro`](../index.md#dwmacro)

##### `impl PartialOrd for DwMacro`

- <span id="dwmacro-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacro) -> option::Option<cmp::Ordering>` — [`DwMacro`](../index.md#dwmacro)

##### `impl StructuralPartialEq for DwMacro`

##### `impl ToString for DwMacro`

- <span id="dwmacro-to-string"></span>`fn to_string(&self) -> String`

### `DwRle`

```rust
struct DwRle(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

Range list entry encoding values.

See Section 7.25, Table 7.30.

#### Implementations

- <span id="dwrle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwRle`

- <span id="dwrle-clone"></span>`fn clone(&self) -> DwRle` — [`DwRle`](../index.md#dwrle)

##### `impl Copy for DwRle`

##### `impl Debug for DwRle`

- <span id="dwrle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwRle`

- <span id="dwrle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwRle`

##### `impl Hash for DwRle`

- <span id="dwrle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwRle`

- <span id="dwrle-cmp"></span>`fn cmp(&self, other: &DwRle) -> cmp::Ordering` — [`DwRle`](../index.md#dwrle)

##### `impl PartialEq for DwRle`

- <span id="dwrle-eq"></span>`fn eq(&self, other: &DwRle) -> bool` — [`DwRle`](../index.md#dwrle)

##### `impl PartialOrd for DwRle`

- <span id="dwrle-partial-cmp"></span>`fn partial_cmp(&self, other: &DwRle) -> option::Option<cmp::Ordering>` — [`DwRle`](../index.md#dwrle)

##### `impl StructuralPartialEq for DwRle`

##### `impl ToString for DwRle`

- <span id="dwrle-to-string"></span>`fn to_string(&self) -> String`

### `DwOp`

```rust
struct DwOp(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

#### Implementations

- <span id="dwop-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOp`

- <span id="dwop-clone"></span>`fn clone(&self) -> DwOp` — [`DwOp`](../index.md#dwop)

##### `impl Copy for DwOp`

##### `impl Debug for DwOp`

- <span id="dwop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOp`

- <span id="dwop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOp`

##### `impl Hash for DwOp`

- <span id="dwop-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOp`

- <span id="dwop-cmp"></span>`fn cmp(&self, other: &DwOp) -> cmp::Ordering` — [`DwOp`](../index.md#dwop)

##### `impl PartialEq for DwOp`

- <span id="dwop-eq"></span>`fn eq(&self, other: &DwOp) -> bool` — [`DwOp`](../index.md#dwop)

##### `impl PartialOrd for DwOp`

- <span id="dwop-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOp) -> option::Option<cmp::Ordering>` — [`DwOp`](../index.md#dwop)

##### `impl StructuralPartialEq for DwOp`

##### `impl ToString for DwOp`

- <span id="dwop-to-string"></span>`fn to_string(&self) -> String`

### `DwEhPe`

```rust
struct DwEhPe(u8);
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

#### Implementations

- <span id="dwehpe-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl BitOr for DwEhPe`

- <span id="dwehpe-bitor-type-output"></span>`type Output = DwEhPe`

- <span id="dwehpe-bitor"></span>`fn bitor(self, rhs: DwEhPe) -> DwEhPe` — [`DwEhPe`](../index.md#dwehpe)

##### `impl Clone for DwEhPe`

- <span id="dwehpe-clone"></span>`fn clone(&self) -> DwEhPe` — [`DwEhPe`](../index.md#dwehpe)

##### `impl Copy for DwEhPe`

##### `impl Debug for DwEhPe`

- <span id="dwehpe-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEhPe`

- <span id="dwehpe-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEhPe`

##### `impl Hash for DwEhPe`

- <span id="dwehpe-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEhPe`

- <span id="dwehpe-cmp"></span>`fn cmp(&self, other: &DwEhPe) -> cmp::Ordering` — [`DwEhPe`](../index.md#dwehpe)

##### `impl PartialEq for DwEhPe`

- <span id="dwehpe-eq"></span>`fn eq(&self, other: &DwEhPe) -> bool` — [`DwEhPe`](../index.md#dwehpe)

##### `impl PartialOrd for DwEhPe`

- <span id="dwehpe-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEhPe) -> option::Option<cmp::Ordering>` — [`DwEhPe`](../index.md#dwehpe)

##### `impl StructuralPartialEq for DwEhPe`

##### `impl ToString for DwEhPe`

- <span id="dwehpe-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `DW_SECT_INFO`
```rust
const DW_SECT_INFO: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_ABBREV`
```rust
const DW_SECT_ABBREV: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_LINE`
```rust
const DW_SECT_LINE: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_LOCLISTS`
```rust
const DW_SECT_LOCLISTS: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_STR_OFFSETS`
```rust
const DW_SECT_STR_OFFSETS: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_MACRO`
```rust
const DW_SECT_MACRO: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_RNGLISTS`
```rust
const DW_SECT_RNGLISTS: DwSect;
```

*Defined in [`gimli-0.32.3/src/constants.rs:104-118`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L104-L118)*

### `DW_SECT_V2_INFO`
```rust
const DW_SECT_V2_INFO: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_TYPES`
```rust
const DW_SECT_V2_TYPES: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_ABBREV`
```rust
const DW_SECT_V2_ABBREV: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_LINE`
```rust
const DW_SECT_V2_LINE: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_LOC`
```rust
const DW_SECT_V2_LOC: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_STR_OFFSETS`
```rust
const DW_SECT_V2_STR_OFFSETS: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_MACINFO`
```rust
const DW_SECT_V2_MACINFO: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_SECT_V2_MACRO`
```rust
const DW_SECT_V2_MACRO: DwSectV2;
```

*Defined in [`gimli-0.32.3/src/constants.rs:120-131`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L120-L131)*

### `DW_UT_compile`
```rust
const DW_UT_compile: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_type`
```rust
const DW_UT_type: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_partial`
```rust
const DW_UT_partial: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_skeleton`
```rust
const DW_UT_skeleton: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_split_compile`
```rust
const DW_UT_split_compile: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_split_type`
```rust
const DW_UT_split_type: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_lo_user`
```rust
const DW_UT_lo_user: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_UT_hi_user`
```rust
const DW_UT_hi_user: DwUt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:133-146`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L133-L146)*

### `DW_CFA_advance_loc`
```rust
const DW_CFA_advance_loc: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_offset`
```rust
const DW_CFA_offset: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_restore`
```rust
const DW_CFA_restore: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_nop`
```rust
const DW_CFA_nop: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_set_loc`
```rust
const DW_CFA_set_loc: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_advance_loc1`
```rust
const DW_CFA_advance_loc1: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_advance_loc2`
```rust
const DW_CFA_advance_loc2: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_advance_loc4`
```rust
const DW_CFA_advance_loc4: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_offset_extended`
```rust
const DW_CFA_offset_extended: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_restore_extended`
```rust
const DW_CFA_restore_extended: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_undefined`
```rust
const DW_CFA_undefined: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_same_value`
```rust
const DW_CFA_same_value: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_register`
```rust
const DW_CFA_register: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_remember_state`
```rust
const DW_CFA_remember_state: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_restore_state`
```rust
const DW_CFA_restore_state: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa`
```rust
const DW_CFA_def_cfa: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa_register`
```rust
const DW_CFA_def_cfa_register: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa_offset`
```rust
const DW_CFA_def_cfa_offset: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa_expression`
```rust
const DW_CFA_def_cfa_expression: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_expression`
```rust
const DW_CFA_expression: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_offset_extended_sf`
```rust
const DW_CFA_offset_extended_sf: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa_sf`
```rust
const DW_CFA_def_cfa_sf: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_def_cfa_offset_sf`
```rust
const DW_CFA_def_cfa_offset_sf: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_val_offset`
```rust
const DW_CFA_val_offset: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_val_offset_sf`
```rust
const DW_CFA_val_offset_sf: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_val_expression`
```rust
const DW_CFA_val_expression: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_lo_user`
```rust
const DW_CFA_lo_user: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_hi_user`
```rust
const DW_CFA_hi_user: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_MIPS_advance_loc8`
```rust
const DW_CFA_MIPS_advance_loc8: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_GNU_window_save`
```rust
const DW_CFA_GNU_window_save: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_GNU_args_size`
```rust
const DW_CFA_GNU_args_size: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_GNU_negative_offset_extended`
```rust
const DW_CFA_GNU_negative_offset_extended: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CFA_AARCH64_negate_ra_state`
```rust
const DW_CFA_AARCH64_negate_ra_state: DwCfa;
```

*Defined in [`gimli-0.32.3/src/constants.rs:148-194`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L148-L194)*

### `DW_CHILDREN_no`
```rust
const DW_CHILDREN_no: DwChildren;
```

*Defined in [`gimli-0.32.3/src/constants.rs:196-203`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L196-L203)*

### `DW_CHILDREN_yes`
```rust
const DW_CHILDREN_yes: DwChildren;
```

*Defined in [`gimli-0.32.3/src/constants.rs:196-203`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L196-L203)*

### `DW_TAG_null`
```rust
const DW_TAG_null: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_global_subroutine`
```rust
const DW_TAG_global_subroutine: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_global_variable`
```rust
const DW_TAG_global_variable: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_local_variable`
```rust
const DW_TAG_local_variable: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_subroutine`
```rust
const DW_TAG_subroutine: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_array_type`
```rust
const DW_TAG_array_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_class_type`
```rust
const DW_TAG_class_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_entry_point`
```rust
const DW_TAG_entry_point: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_enumeration_type`
```rust
const DW_TAG_enumeration_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_formal_parameter`
```rust
const DW_TAG_formal_parameter: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_imported_declaration`
```rust
const DW_TAG_imported_declaration: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_label`
```rust
const DW_TAG_label: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_lexical_block`
```rust
const DW_TAG_lexical_block: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_member`
```rust
const DW_TAG_member: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_pointer_type`
```rust
const DW_TAG_pointer_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_reference_type`
```rust
const DW_TAG_reference_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_compile_unit`
```rust
const DW_TAG_compile_unit: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_string_type`
```rust
const DW_TAG_string_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_structure_type`
```rust
const DW_TAG_structure_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_subroutine_type`
```rust
const DW_TAG_subroutine_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_typedef`
```rust
const DW_TAG_typedef: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_union_type`
```rust
const DW_TAG_union_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_unspecified_parameters`
```rust
const DW_TAG_unspecified_parameters: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_variant`
```rust
const DW_TAG_variant: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_common_block`
```rust
const DW_TAG_common_block: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_common_inclusion`
```rust
const DW_TAG_common_inclusion: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_inheritance`
```rust
const DW_TAG_inheritance: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_inlined_subroutine`
```rust
const DW_TAG_inlined_subroutine: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_module`
```rust
const DW_TAG_module: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_ptr_to_member_type`
```rust
const DW_TAG_ptr_to_member_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_set_type`
```rust
const DW_TAG_set_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_subrange_type`
```rust
const DW_TAG_subrange_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_with_stmt`
```rust
const DW_TAG_with_stmt: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_access_declaration`
```rust
const DW_TAG_access_declaration: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_base_type`
```rust
const DW_TAG_base_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_catch_block`
```rust
const DW_TAG_catch_block: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_const_type`
```rust
const DW_TAG_const_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_constant`
```rust
const DW_TAG_constant: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_enumerator`
```rust
const DW_TAG_enumerator: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_file_type`
```rust
const DW_TAG_file_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_friend`
```rust
const DW_TAG_friend: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_namelist`
```rust
const DW_TAG_namelist: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_namelist_item`
```rust
const DW_TAG_namelist_item: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_packed_type`
```rust
const DW_TAG_packed_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_subprogram`
```rust
const DW_TAG_subprogram: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_template_type_parameter`
```rust
const DW_TAG_template_type_parameter: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_template_value_parameter`
```rust
const DW_TAG_template_value_parameter: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_thrown_type`
```rust
const DW_TAG_thrown_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_try_block`
```rust
const DW_TAG_try_block: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_variant_part`
```rust
const DW_TAG_variant_part: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_variable`
```rust
const DW_TAG_variable: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_volatile_type`
```rust
const DW_TAG_volatile_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_dwarf_procedure`
```rust
const DW_TAG_dwarf_procedure: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_restrict_type`
```rust
const DW_TAG_restrict_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_interface_type`
```rust
const DW_TAG_interface_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_namespace`
```rust
const DW_TAG_namespace: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_imported_module`
```rust
const DW_TAG_imported_module: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_unspecified_type`
```rust
const DW_TAG_unspecified_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_partial_unit`
```rust
const DW_TAG_partial_unit: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_imported_unit`
```rust
const DW_TAG_imported_unit: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_condition`
```rust
const DW_TAG_condition: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_shared_type`
```rust
const DW_TAG_shared_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_type_unit`
```rust
const DW_TAG_type_unit: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_rvalue_reference_type`
```rust
const DW_TAG_rvalue_reference_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_template_alias`
```rust
const DW_TAG_template_alias: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_coarray_type`
```rust
const DW_TAG_coarray_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_generic_subrange`
```rust
const DW_TAG_generic_subrange: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_dynamic_type`
```rust
const DW_TAG_dynamic_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_atomic_type`
```rust
const DW_TAG_atomic_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_call_site`
```rust
const DW_TAG_call_site: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_call_site_parameter`
```rust
const DW_TAG_call_site_parameter: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_skeleton_unit`
```rust
const DW_TAG_skeleton_unit: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_immutable_type`
```rust
const DW_TAG_immutable_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_lo_user`
```rust
const DW_TAG_lo_user: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_hi_user`
```rust
const DW_TAG_hi_user: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_MIPS_loop`
```rust
const DW_TAG_MIPS_loop: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_HP_array_descriptor`
```rust
const DW_TAG_HP_array_descriptor: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_HP_Bliss_field`
```rust
const DW_TAG_HP_Bliss_field: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_HP_Bliss_field_set`
```rust
const DW_TAG_HP_Bliss_field_set: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_format_label`
```rust
const DW_TAG_format_label: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_function_template`
```rust
const DW_TAG_function_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_class_template`
```rust
const DW_TAG_class_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_BINCL`
```rust
const DW_TAG_GNU_BINCL: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_EINCL`
```rust
const DW_TAG_GNU_EINCL: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_template_template_param`
```rust
const DW_TAG_GNU_template_template_param: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_template_parameter_pack`
```rust
const DW_TAG_GNU_template_parameter_pack: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_formal_parameter_pack`
```rust
const DW_TAG_GNU_formal_parameter_pack: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_call_site`
```rust
const DW_TAG_GNU_call_site: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_GNU_call_site_parameter`
```rust
const DW_TAG_GNU_call_site_parameter: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_APPLE_property`
```rust
const DW_TAG_APPLE_property: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_function_template`
```rust
const DW_TAG_SUN_function_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_class_template`
```rust
const DW_TAG_SUN_class_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_struct_template`
```rust
const DW_TAG_SUN_struct_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_union_template`
```rust
const DW_TAG_SUN_union_template: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_indirect_inheritance`
```rust
const DW_TAG_SUN_indirect_inheritance: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_codeflags`
```rust
const DW_TAG_SUN_codeflags: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_memop_info`
```rust
const DW_TAG_SUN_memop_info: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_omp_child_func`
```rust
const DW_TAG_SUN_omp_child_func: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_rtti_descriptor`
```rust
const DW_TAG_SUN_rtti_descriptor: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_dtor_info`
```rust
const DW_TAG_SUN_dtor_info: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_dtor`
```rust
const DW_TAG_SUN_dtor: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_f90_interface`
```rust
const DW_TAG_SUN_f90_interface: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_SUN_fortran_vax_structure`
```rust
const DW_TAG_SUN_fortran_vax_structure: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_ALTIUM_circ_type`
```rust
const DW_TAG_ALTIUM_circ_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_ALTIUM_mwa_circ_type`
```rust
const DW_TAG_ALTIUM_mwa_circ_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_ALTIUM_rev_carry_type`
```rust
const DW_TAG_ALTIUM_rev_carry_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_ALTIUM_rom`
```rust
const DW_TAG_ALTIUM_rom: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_upc_shared_type`
```rust
const DW_TAG_upc_shared_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_upc_strict_type`
```rust
const DW_TAG_upc_strict_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_upc_relaxed_type`
```rust
const DW_TAG_upc_relaxed_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_PGI_kanji_type`
```rust
const DW_TAG_PGI_kanji_type: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_PGI_interface_block`
```rust
const DW_TAG_PGI_interface_block: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_BORLAND_property`
```rust
const DW_TAG_BORLAND_property: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_BORLAND_Delphi_string`
```rust
const DW_TAG_BORLAND_Delphi_string: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_BORLAND_Delphi_dynamic_array`
```rust
const DW_TAG_BORLAND_Delphi_dynamic_array: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_BORLAND_Delphi_set`
```rust
const DW_TAG_BORLAND_Delphi_set: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_TAG_BORLAND_Delphi_variant`
```rust
const DW_TAG_BORLAND_Delphi_variant: DwTag;
```

*Defined in [`gimli-0.32.3/src/constants.rs:205-357`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L205-L357)*

### `DW_AT_null`
```rust
const DW_AT_null: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_fund_type`
```rust
const DW_AT_fund_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_mod_fund_type`
```rust
const DW_AT_mod_fund_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_user_def_type`
```rust
const DW_AT_user_def_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_mod_u_d_type`
```rust
const DW_AT_mod_u_d_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_subscr_data`
```rust
const DW_AT_subscr_data: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_element_list`
```rust
const DW_AT_element_list: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_member`
```rust
const DW_AT_member: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_friends`
```rust
const DW_AT_friends: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_program`
```rust
const DW_AT_program: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_private`
```rust
const DW_AT_private: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_protected`
```rust
const DW_AT_protected: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_public`
```rust
const DW_AT_public: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_pure_virtual`
```rust
const DW_AT_pure_virtual: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_virtual`
```rust
const DW_AT_virtual: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_specification_v1`
```rust
const DW_AT_specification_v1: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_sibling`
```rust
const DW_AT_sibling: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_location`
```rust
const DW_AT_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_name`
```rust
const DW_AT_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_ordering`
```rust
const DW_AT_ordering: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_byte_size`
```rust
const DW_AT_byte_size: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_bit_offset`
```rust
const DW_AT_bit_offset: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_bit_size`
```rust
const DW_AT_bit_size: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_stmt_list`
```rust
const DW_AT_stmt_list: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_low_pc`
```rust
const DW_AT_low_pc: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_high_pc`
```rust
const DW_AT_high_pc: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_language`
```rust
const DW_AT_language: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_discr`
```rust
const DW_AT_discr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_discr_value`
```rust
const DW_AT_discr_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_visibility`
```rust
const DW_AT_visibility: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_import`
```rust
const DW_AT_import: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_string_length`
```rust
const DW_AT_string_length: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_common_reference`
```rust
const DW_AT_common_reference: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_comp_dir`
```rust
const DW_AT_comp_dir: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_const_value`
```rust
const DW_AT_const_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_containing_type`
```rust
const DW_AT_containing_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_default_value`
```rust
const DW_AT_default_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_inline`
```rust
const DW_AT_inline: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_is_optional`
```rust
const DW_AT_is_optional: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_lower_bound`
```rust
const DW_AT_lower_bound: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_producer`
```rust
const DW_AT_producer: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_prototyped`
```rust
const DW_AT_prototyped: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_return_addr`
```rust
const DW_AT_return_addr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_start_scope`
```rust
const DW_AT_start_scope: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_bit_stride`
```rust
const DW_AT_bit_stride: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_upper_bound`
```rust
const DW_AT_upper_bound: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_abstract_origin`
```rust
const DW_AT_abstract_origin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_accessibility`
```rust
const DW_AT_accessibility: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_address_class`
```rust
const DW_AT_address_class: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_artificial`
```rust
const DW_AT_artificial: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_base_types`
```rust
const DW_AT_base_types: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_calling_convention`
```rust
const DW_AT_calling_convention: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_count`
```rust
const DW_AT_count: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_data_member_location`
```rust
const DW_AT_data_member_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_decl_column`
```rust
const DW_AT_decl_column: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_decl_file`
```rust
const DW_AT_decl_file: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_decl_line`
```rust
const DW_AT_decl_line: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_declaration`
```rust
const DW_AT_declaration: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_discr_list`
```rust
const DW_AT_discr_list: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_encoding`
```rust
const DW_AT_encoding: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_external`
```rust
const DW_AT_external: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_frame_base`
```rust
const DW_AT_frame_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_friend`
```rust
const DW_AT_friend: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_identifier_case`
```rust
const DW_AT_identifier_case: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_macro_info`
```rust
const DW_AT_macro_info: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_namelist_item`
```rust
const DW_AT_namelist_item: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_priority`
```rust
const DW_AT_priority: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_segment`
```rust
const DW_AT_segment: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_specification`
```rust
const DW_AT_specification: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_static_link`
```rust
const DW_AT_static_link: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_type`
```rust
const DW_AT_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_use_location`
```rust
const DW_AT_use_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_variable_parameter`
```rust
const DW_AT_variable_parameter: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_virtuality`
```rust
const DW_AT_virtuality: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_vtable_elem_location`
```rust
const DW_AT_vtable_elem_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_allocated`
```rust
const DW_AT_allocated: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_associated`
```rust
const DW_AT_associated: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_data_location`
```rust
const DW_AT_data_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_byte_stride`
```rust
const DW_AT_byte_stride: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_entry_pc`
```rust
const DW_AT_entry_pc: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_use_UTF8`
```rust
const DW_AT_use_UTF8: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_extension`
```rust
const DW_AT_extension: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_ranges`
```rust
const DW_AT_ranges: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_trampoline`
```rust
const DW_AT_trampoline: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_column`
```rust
const DW_AT_call_column: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_file`
```rust
const DW_AT_call_file: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_line`
```rust
const DW_AT_call_line: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_description`
```rust
const DW_AT_description: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_binary_scale`
```rust
const DW_AT_binary_scale: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_decimal_scale`
```rust
const DW_AT_decimal_scale: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_small`
```rust
const DW_AT_small: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_decimal_sign`
```rust
const DW_AT_decimal_sign: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_digit_count`
```rust
const DW_AT_digit_count: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_picture_string`
```rust
const DW_AT_picture_string: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_mutable`
```rust
const DW_AT_mutable: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_threads_scaled`
```rust
const DW_AT_threads_scaled: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_explicit`
```rust
const DW_AT_explicit: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_object_pointer`
```rust
const DW_AT_object_pointer: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_endianity`
```rust
const DW_AT_endianity: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_elemental`
```rust
const DW_AT_elemental: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_pure`
```rust
const DW_AT_pure: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_recursive`
```rust
const DW_AT_recursive: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_signature`
```rust
const DW_AT_signature: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_main_subprogram`
```rust
const DW_AT_main_subprogram: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_data_bit_offset`
```rust
const DW_AT_data_bit_offset: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_const_expr`
```rust
const DW_AT_const_expr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_enum_class`
```rust
const DW_AT_enum_class: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_linkage_name`
```rust
const DW_AT_linkage_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_string_length_bit_size`
```rust
const DW_AT_string_length_bit_size: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_string_length_byte_size`
```rust
const DW_AT_string_length_byte_size: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_rank`
```rust
const DW_AT_rank: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_str_offsets_base`
```rust
const DW_AT_str_offsets_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_addr_base`
```rust
const DW_AT_addr_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_rnglists_base`
```rust
const DW_AT_rnglists_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_dwo_name`
```rust
const DW_AT_dwo_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_reference`
```rust
const DW_AT_reference: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_rvalue_reference`
```rust
const DW_AT_rvalue_reference: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_macros`
```rust
const DW_AT_macros: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_all_calls`
```rust
const DW_AT_call_all_calls: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_all_source_calls`
```rust
const DW_AT_call_all_source_calls: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_all_tail_calls`
```rust
const DW_AT_call_all_tail_calls: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_return_pc`
```rust
const DW_AT_call_return_pc: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_value`
```rust
const DW_AT_call_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_origin`
```rust
const DW_AT_call_origin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_parameter`
```rust
const DW_AT_call_parameter: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_pc`
```rust
const DW_AT_call_pc: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_tail_call`
```rust
const DW_AT_call_tail_call: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_target`
```rust
const DW_AT_call_target: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_target_clobbered`
```rust
const DW_AT_call_target_clobbered: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_data_location`
```rust
const DW_AT_call_data_location: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_call_data_value`
```rust
const DW_AT_call_data_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_noreturn`
```rust
const DW_AT_noreturn: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_alignment`
```rust
const DW_AT_alignment: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_export_symbols`
```rust
const DW_AT_export_symbols: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_deleted`
```rust
const DW_AT_deleted: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_defaulted`
```rust
const DW_AT_defaulted: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_loclists_base`
```rust
const DW_AT_loclists_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_lo_user`
```rust
const DW_AT_lo_user: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_hi_user`
```rust
const DW_AT_hi_user: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_fde`
```rust
const DW_AT_MIPS_fde: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_loop_begin`
```rust
const DW_AT_MIPS_loop_begin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_tail_loop_begin`
```rust
const DW_AT_MIPS_tail_loop_begin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_epilog_begin`
```rust
const DW_AT_MIPS_epilog_begin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_loop_unroll_factor`
```rust
const DW_AT_MIPS_loop_unroll_factor: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_software_pipeline_depth`
```rust
const DW_AT_MIPS_software_pipeline_depth: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_linkage_name`
```rust
const DW_AT_MIPS_linkage_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_stride`
```rust
const DW_AT_MIPS_stride: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_abstract_name`
```rust
const DW_AT_MIPS_abstract_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_clone_origin`
```rust
const DW_AT_MIPS_clone_origin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_has_inlines`
```rust
const DW_AT_MIPS_has_inlines: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_stride_byte`
```rust
const DW_AT_MIPS_stride_byte: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_stride_elem`
```rust
const DW_AT_MIPS_stride_elem: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_ptr_dopetype`
```rust
const DW_AT_MIPS_ptr_dopetype: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_allocatable_dopetype`
```rust
const DW_AT_MIPS_allocatable_dopetype: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_assumed_shape_dopetype`
```rust
const DW_AT_MIPS_assumed_shape_dopetype: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_MIPS_assumed_size`
```rust
const DW_AT_MIPS_assumed_size: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_INTEL_other_endian`
```rust
const DW_AT_INTEL_other_endian: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_sf_names`
```rust
const DW_AT_sf_names: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_src_info`
```rust
const DW_AT_src_info: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_mac_info`
```rust
const DW_AT_mac_info: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_src_coords`
```rust
const DW_AT_src_coords: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_body_begin`
```rust
const DW_AT_body_begin: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_body_end`
```rust
const DW_AT_body_end: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_vector`
```rust
const DW_AT_GNU_vector: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_guarded_by`
```rust
const DW_AT_GNU_guarded_by: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_pt_guarded_by`
```rust
const DW_AT_GNU_pt_guarded_by: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_guarded`
```rust
const DW_AT_GNU_guarded: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_pt_guarded`
```rust
const DW_AT_GNU_pt_guarded: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_locks_excluded`
```rust
const DW_AT_GNU_locks_excluded: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_exclusive_locks_required`
```rust
const DW_AT_GNU_exclusive_locks_required: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_shared_locks_required`
```rust
const DW_AT_GNU_shared_locks_required: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_odr_signature`
```rust
const DW_AT_GNU_odr_signature: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_template_name`
```rust
const DW_AT_GNU_template_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_call_site_value`
```rust
const DW_AT_GNU_call_site_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_call_site_data_value`
```rust
const DW_AT_GNU_call_site_data_value: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_call_site_target`
```rust
const DW_AT_GNU_call_site_target: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_call_site_target_clobbered`
```rust
const DW_AT_GNU_call_site_target_clobbered: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_tail_call`
```rust
const DW_AT_GNU_tail_call: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_all_tail_call_sites`
```rust
const DW_AT_GNU_all_tail_call_sites: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_all_call_sites`
```rust
const DW_AT_GNU_all_call_sites: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_all_source_call_sites`
```rust
const DW_AT_GNU_all_source_call_sites: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_macros`
```rust
const DW_AT_GNU_macros: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_deleted`
```rust
const DW_AT_GNU_deleted: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_dwo_name`
```rust
const DW_AT_GNU_dwo_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_dwo_id`
```rust
const DW_AT_GNU_dwo_id: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_ranges_base`
```rust
const DW_AT_GNU_ranges_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_addr_base`
```rust
const DW_AT_GNU_addr_base: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_pubnames`
```rust
const DW_AT_GNU_pubnames: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_pubtypes`
```rust
const DW_AT_GNU_pubtypes: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_discriminator`
```rust
const DW_AT_GNU_discriminator: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_locviews`
```rust
const DW_AT_GNU_locviews: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_entry_view`
```rust
const DW_AT_GNU_entry_view: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_template`
```rust
const DW_AT_SUN_template: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_alignment`
```rust
const DW_AT_SUN_alignment: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_vtable`
```rust
const DW_AT_SUN_vtable: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_count_guarantee`
```rust
const DW_AT_SUN_count_guarantee: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_command_line`
```rust
const DW_AT_SUN_command_line: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_vbase`
```rust
const DW_AT_SUN_vbase: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_compile_options`
```rust
const DW_AT_SUN_compile_options: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_language`
```rust
const DW_AT_SUN_language: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_browser_file`
```rust
const DW_AT_SUN_browser_file: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_vtable_abi`
```rust
const DW_AT_SUN_vtable_abi: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_func_offsets`
```rust
const DW_AT_SUN_func_offsets: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_cf_kind`
```rust
const DW_AT_SUN_cf_kind: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_vtable_index`
```rust
const DW_AT_SUN_vtable_index: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_omp_tpriv_addr`
```rust
const DW_AT_SUN_omp_tpriv_addr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_omp_child_func`
```rust
const DW_AT_SUN_omp_child_func: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_func_offset`
```rust
const DW_AT_SUN_func_offset: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_memop_type_ref`
```rust
const DW_AT_SUN_memop_type_ref: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_profile_id`
```rust
const DW_AT_SUN_profile_id: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_memop_signature`
```rust
const DW_AT_SUN_memop_signature: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_obj_dir`
```rust
const DW_AT_SUN_obj_dir: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_obj_file`
```rust
const DW_AT_SUN_obj_file: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_original_name`
```rust
const DW_AT_SUN_original_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_hwcprof_signature`
```rust
const DW_AT_SUN_hwcprof_signature: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_amd64_parmdump`
```rust
const DW_AT_SUN_amd64_parmdump: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_part_link_name`
```rust
const DW_AT_SUN_part_link_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_link_name`
```rust
const DW_AT_SUN_link_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_pass_with_const`
```rust
const DW_AT_SUN_pass_with_const: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_return_with_const`
```rust
const DW_AT_SUN_return_with_const: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_import_by_name`
```rust
const DW_AT_SUN_import_by_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_f90_pointer`
```rust
const DW_AT_SUN_f90_pointer: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_pass_by_ref`
```rust
const DW_AT_SUN_pass_by_ref: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_f90_allocatable`
```rust
const DW_AT_SUN_f90_allocatable: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_f90_assumed_shape_array`
```rust
const DW_AT_SUN_f90_assumed_shape_array: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_c_vla`
```rust
const DW_AT_SUN_c_vla: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_return_value_ptr`
```rust
const DW_AT_SUN_return_value_ptr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_dtor_start`
```rust
const DW_AT_SUN_dtor_start: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_dtor_length`
```rust
const DW_AT_SUN_dtor_length: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_dtor_state_initial`
```rust
const DW_AT_SUN_dtor_state_initial: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_dtor_state_final`
```rust
const DW_AT_SUN_dtor_state_final: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_dtor_state_deltas`
```rust
const DW_AT_SUN_dtor_state_deltas: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_import_by_lname`
```rust
const DW_AT_SUN_import_by_lname: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_f90_use_only`
```rust
const DW_AT_SUN_f90_use_only: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_namelist_spec`
```rust
const DW_AT_SUN_namelist_spec: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_is_omp_child_func`
```rust
const DW_AT_SUN_is_omp_child_func: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_fortran_main_alias`
```rust
const DW_AT_SUN_fortran_main_alias: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_SUN_fortran_based`
```rust
const DW_AT_SUN_fortran_based: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_ALTIUM_loclist`
```rust
const DW_AT_ALTIUM_loclist: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_use_GNAT_descriptive_type`
```rust
const DW_AT_use_GNAT_descriptive_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNAT_descriptive_type`
```rust
const DW_AT_GNAT_descriptive_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_numerator`
```rust
const DW_AT_GNU_numerator: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_denominator`
```rust
const DW_AT_GNU_denominator: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_GNU_bias`
```rust
const DW_AT_GNU_bias: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_upc_threads_scaled`
```rust
const DW_AT_upc_threads_scaled: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_PGI_lbase`
```rust
const DW_AT_PGI_lbase: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_PGI_soffset`
```rust
const DW_AT_PGI_soffset: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_PGI_lstride`
```rust
const DW_AT_PGI_lstride: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_property_read`
```rust
const DW_AT_BORLAND_property_read: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_property_write`
```rust
const DW_AT_BORLAND_property_write: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_property_implements`
```rust
const DW_AT_BORLAND_property_implements: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_property_index`
```rust
const DW_AT_BORLAND_property_index: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_property_default`
```rust
const DW_AT_BORLAND_property_default: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_unit`
```rust
const DW_AT_BORLAND_Delphi_unit: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_class`
```rust
const DW_AT_BORLAND_Delphi_class: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_record`
```rust
const DW_AT_BORLAND_Delphi_record: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_metaclass`
```rust
const DW_AT_BORLAND_Delphi_metaclass: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_constructor`
```rust
const DW_AT_BORLAND_Delphi_constructor: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_destructor`
```rust
const DW_AT_BORLAND_Delphi_destructor: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_anonymous_method`
```rust
const DW_AT_BORLAND_Delphi_anonymous_method: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_interface`
```rust
const DW_AT_BORLAND_Delphi_interface: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_ABI`
```rust
const DW_AT_BORLAND_Delphi_ABI: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_return`
```rust
const DW_AT_BORLAND_Delphi_return: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_Delphi_frameptr`
```rust
const DW_AT_BORLAND_Delphi_frameptr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_BORLAND_closure`
```rust
const DW_AT_BORLAND_closure: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_LLVM_include_path`
```rust
const DW_AT_LLVM_include_path: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_LLVM_config_macros`
```rust
const DW_AT_LLVM_config_macros: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_LLVM_isysroot`
```rust
const DW_AT_LLVM_isysroot: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_optimized`
```rust
const DW_AT_APPLE_optimized: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_flags`
```rust
const DW_AT_APPLE_flags: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_isa`
```rust
const DW_AT_APPLE_isa: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_block`
```rust
const DW_AT_APPLE_block: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_major_runtime_vers`
```rust
const DW_AT_APPLE_major_runtime_vers: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_runtime_class`
```rust
const DW_AT_APPLE_runtime_class: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_omit_frame_ptr`
```rust
const DW_AT_APPLE_omit_frame_ptr: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_property_name`
```rust
const DW_AT_APPLE_property_name: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_property_getter`
```rust
const DW_AT_APPLE_property_getter: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_property_setter`
```rust
const DW_AT_APPLE_property_setter: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_property_attribute`
```rust
const DW_AT_APPLE_property_attribute: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_objc_complete_type`
```rust
const DW_AT_APPLE_objc_complete_type: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_AT_APPLE_property`
```rust
const DW_AT_APPLE_property: DwAt;
```

*Defined in [`gimli-0.32.3/src/constants.rs:359-689`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L359-L689)*

### `DW_FORM_null`
```rust
const DW_FORM_null: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref`
```rust
const DW_FORM_ref: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addr`
```rust
const DW_FORM_addr: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_block2`
```rust
const DW_FORM_block2: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_block4`
```rust
const DW_FORM_block4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_data2`
```rust
const DW_FORM_data2: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_data4`
```rust
const DW_FORM_data4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_data8`
```rust
const DW_FORM_data8: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_string`
```rust
const DW_FORM_string: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_block`
```rust
const DW_FORM_block: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_block1`
```rust
const DW_FORM_block1: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_data1`
```rust
const DW_FORM_data1: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_flag`
```rust
const DW_FORM_flag: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_sdata`
```rust
const DW_FORM_sdata: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strp`
```rust
const DW_FORM_strp: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_udata`
```rust
const DW_FORM_udata: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref_addr`
```rust
const DW_FORM_ref_addr: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref1`
```rust
const DW_FORM_ref1: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref2`
```rust
const DW_FORM_ref2: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref4`
```rust
const DW_FORM_ref4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref8`
```rust
const DW_FORM_ref8: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref_udata`
```rust
const DW_FORM_ref_udata: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_indirect`
```rust
const DW_FORM_indirect: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_sec_offset`
```rust
const DW_FORM_sec_offset: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_exprloc`
```rust
const DW_FORM_exprloc: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_flag_present`
```rust
const DW_FORM_flag_present: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref_sig8`
```rust
const DW_FORM_ref_sig8: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strx`
```rust
const DW_FORM_strx: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addrx`
```rust
const DW_FORM_addrx: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref_sup4`
```rust
const DW_FORM_ref_sup4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strp_sup`
```rust
const DW_FORM_strp_sup: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_data16`
```rust
const DW_FORM_data16: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_line_strp`
```rust
const DW_FORM_line_strp: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_implicit_const`
```rust
const DW_FORM_implicit_const: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_loclistx`
```rust
const DW_FORM_loclistx: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_rnglistx`
```rust
const DW_FORM_rnglistx: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_ref_sup8`
```rust
const DW_FORM_ref_sup8: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strx1`
```rust
const DW_FORM_strx1: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strx2`
```rust
const DW_FORM_strx2: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strx3`
```rust
const DW_FORM_strx3: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_strx4`
```rust
const DW_FORM_strx4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addrx1`
```rust
const DW_FORM_addrx1: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addrx2`
```rust
const DW_FORM_addrx2: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addrx3`
```rust
const DW_FORM_addrx3: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_addrx4`
```rust
const DW_FORM_addrx4: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_GNU_addr_index`
```rust
const DW_FORM_GNU_addr_index: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_GNU_str_index`
```rust
const DW_FORM_GNU_str_index: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_GNU_ref_alt`
```rust
const DW_FORM_GNU_ref_alt: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_FORM_GNU_strp_alt`
```rust
const DW_FORM_GNU_strp_alt: DwForm;
```

*Defined in [`gimli-0.32.3/src/constants.rs:691-759`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L691-L759)*

### `DW_ATE_address`
```rust
const DW_ATE_address: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_boolean`
```rust
const DW_ATE_boolean: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_complex_float`
```rust
const DW_ATE_complex_float: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_float`
```rust
const DW_ATE_float: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_signed`
```rust
const DW_ATE_signed: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_signed_char`
```rust
const DW_ATE_signed_char: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_unsigned`
```rust
const DW_ATE_unsigned: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_unsigned_char`
```rust
const DW_ATE_unsigned_char: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_imaginary_float`
```rust
const DW_ATE_imaginary_float: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_packed_decimal`
```rust
const DW_ATE_packed_decimal: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_numeric_string`
```rust
const DW_ATE_numeric_string: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_edited`
```rust
const DW_ATE_edited: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_signed_fixed`
```rust
const DW_ATE_signed_fixed: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_unsigned_fixed`
```rust
const DW_ATE_unsigned_fixed: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_decimal_float`
```rust
const DW_ATE_decimal_float: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_UTF`
```rust
const DW_ATE_UTF: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_UCS`
```rust
const DW_ATE_UCS: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_ASCII`
```rust
const DW_ATE_ASCII: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_lo_user`
```rust
const DW_ATE_lo_user: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_ATE_hi_user`
```rust
const DW_ATE_hi_user: DwAte;
```

*Defined in [`gimli-0.32.3/src/constants.rs:761-791`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L761-L791)*

### `DW_LLE_end_of_list`
```rust
const DW_LLE_end_of_list: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_base_addressx`
```rust
const DW_LLE_base_addressx: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_startx_endx`
```rust
const DW_LLE_startx_endx: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_startx_length`
```rust
const DW_LLE_startx_length: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_offset_pair`
```rust
const DW_LLE_offset_pair: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_default_location`
```rust
const DW_LLE_default_location: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_base_address`
```rust
const DW_LLE_base_address: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_start_end`
```rust
const DW_LLE_start_end: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_start_length`
```rust
const DW_LLE_start_length: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_LLE_GNU_view_pair`
```rust
const DW_LLE_GNU_view_pair: DwLle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:793-808`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L793-L808)*

### `DW_DS_unsigned`
```rust
const DW_DS_unsigned: DwDs;
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

### `DW_DS_leading_overpunch`
```rust
const DW_DS_leading_overpunch: DwDs;
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

### `DW_DS_trailing_overpunch`
```rust
const DW_DS_trailing_overpunch: DwDs;
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

### `DW_DS_leading_separate`
```rust
const DW_DS_leading_separate: DwDs;
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

### `DW_DS_trailing_separate`
```rust
const DW_DS_trailing_separate: DwDs;
```

*Defined in [`gimli-0.32.3/src/constants.rs:810-820`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L810-L820)*

### `DW_END_default`
```rust
const DW_END_default: DwEnd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

### `DW_END_big`
```rust
const DW_END_big: DwEnd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

### `DW_END_little`
```rust
const DW_END_little: DwEnd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

### `DW_END_lo_user`
```rust
const DW_END_lo_user: DwEnd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

### `DW_END_hi_user`
```rust
const DW_END_hi_user: DwEnd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:822-832`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L822-L832)*

### `DW_ACCESS_public`
```rust
const DW_ACCESS_public: DwAccess;
```

*Defined in [`gimli-0.32.3/src/constants.rs:834-842`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L834-L842)*

### `DW_ACCESS_protected`
```rust
const DW_ACCESS_protected: DwAccess;
```

*Defined in [`gimli-0.32.3/src/constants.rs:834-842`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L834-L842)*

### `DW_ACCESS_private`
```rust
const DW_ACCESS_private: DwAccess;
```

*Defined in [`gimli-0.32.3/src/constants.rs:834-842`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L834-L842)*

### `DW_VIS_local`
```rust
const DW_VIS_local: DwVis;
```

*Defined in [`gimli-0.32.3/src/constants.rs:844-852`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L844-L852)*

### `DW_VIS_exported`
```rust
const DW_VIS_exported: DwVis;
```

*Defined in [`gimli-0.32.3/src/constants.rs:844-852`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L844-L852)*

### `DW_VIS_qualified`
```rust
const DW_VIS_qualified: DwVis;
```

*Defined in [`gimli-0.32.3/src/constants.rs:844-852`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L844-L852)*

### `DW_VIRTUALITY_none`
```rust
const DW_VIRTUALITY_none: DwVirtuality;
```

*Defined in [`gimli-0.32.3/src/constants.rs:854-862`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L854-L862)*

### `DW_VIRTUALITY_virtual`
```rust
const DW_VIRTUALITY_virtual: DwVirtuality;
```

*Defined in [`gimli-0.32.3/src/constants.rs:854-862`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L854-L862)*

### `DW_VIRTUALITY_pure_virtual`
```rust
const DW_VIRTUALITY_pure_virtual: DwVirtuality;
```

*Defined in [`gimli-0.32.3/src/constants.rs:854-862`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L854-L862)*

### `DW_LANG_C89`
```rust
const DW_LANG_C89: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C`
```rust
const DW_LANG_C: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Ada83`
```rust
const DW_LANG_Ada83: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus`
```rust
const DW_LANG_C_plus_plus: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Cobol74`
```rust
const DW_LANG_Cobol74: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Cobol85`
```rust
const DW_LANG_Cobol85: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran77`
```rust
const DW_LANG_Fortran77: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran90`
```rust
const DW_LANG_Fortran90: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Pascal83`
```rust
const DW_LANG_Pascal83: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Modula2`
```rust
const DW_LANG_Modula2: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Java`
```rust
const DW_LANG_Java: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C99`
```rust
const DW_LANG_C99: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Ada95`
```rust
const DW_LANG_Ada95: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran95`
```rust
const DW_LANG_Fortran95: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_PLI`
```rust
const DW_LANG_PLI: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_ObjC`
```rust
const DW_LANG_ObjC: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_ObjC_plus_plus`
```rust
const DW_LANG_ObjC_plus_plus: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_UPC`
```rust
const DW_LANG_UPC: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_D`
```rust
const DW_LANG_D: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Python`
```rust
const DW_LANG_Python: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_OpenCL`
```rust
const DW_LANG_OpenCL: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Go`
```rust
const DW_LANG_Go: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Modula3`
```rust
const DW_LANG_Modula3: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Haskell`
```rust
const DW_LANG_Haskell: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus_03`
```rust
const DW_LANG_C_plus_plus_03: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus_11`
```rust
const DW_LANG_C_plus_plus_11: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_OCaml`
```rust
const DW_LANG_OCaml: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Rust`
```rust
const DW_LANG_Rust: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C11`
```rust
const DW_LANG_C11: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Swift`
```rust
const DW_LANG_Swift: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Julia`
```rust
const DW_LANG_Julia: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Dylan`
```rust
const DW_LANG_Dylan: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus_14`
```rust
const DW_LANG_C_plus_plus_14: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran03`
```rust
const DW_LANG_Fortran03: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran08`
```rust
const DW_LANG_Fortran08: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_RenderScript`
```rust
const DW_LANG_RenderScript: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_BLISS`
```rust
const DW_LANG_BLISS: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Kotlin`
```rust
const DW_LANG_Kotlin: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Zig`
```rust
const DW_LANG_Zig: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Crystal`
```rust
const DW_LANG_Crystal: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus_17`
```rust
const DW_LANG_C_plus_plus_17: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C_plus_plus_20`
```rust
const DW_LANG_C_plus_plus_20: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_C17`
```rust
const DW_LANG_C17: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Fortran18`
```rust
const DW_LANG_Fortran18: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Ada2005`
```rust
const DW_LANG_Ada2005: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Ada2012`
```rust
const DW_LANG_Ada2012: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_lo_user`
```rust
const DW_LANG_lo_user: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_hi_user`
```rust
const DW_LANG_hi_user: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_Mips_Assembler`
```rust
const DW_LANG_Mips_Assembler: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_GOOGLE_RenderScript`
```rust
const DW_LANG_GOOGLE_RenderScript: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_SUN_Assembler`
```rust
const DW_LANG_SUN_Assembler: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_ALTIUM_Assembler`
```rust
const DW_LANG_ALTIUM_Assembler: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_LANG_BORLAND_Delphi`
```rust
const DW_LANG_BORLAND_Delphi: DwLang;
```

*Defined in [`gimli-0.32.3/src/constants.rs:864-924`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L864-L924)*

### `DW_ADDR_none`
```rust
const DW_ADDR_none: DwAddr;
```

*Defined in [`gimli-0.32.3/src/constants.rs:962-969`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L962-L969)*

### `DW_ID_case_sensitive`
```rust
const DW_ID_case_sensitive: DwId;
```

*Defined in [`gimli-0.32.3/src/constants.rs:971-980`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L971-L980)*

### `DW_ID_up_case`
```rust
const DW_ID_up_case: DwId;
```

*Defined in [`gimli-0.32.3/src/constants.rs:971-980`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L971-L980)*

### `DW_ID_down_case`
```rust
const DW_ID_down_case: DwId;
```

*Defined in [`gimli-0.32.3/src/constants.rs:971-980`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L971-L980)*

### `DW_ID_case_insensitive`
```rust
const DW_ID_case_insensitive: DwId;
```

*Defined in [`gimli-0.32.3/src/constants.rs:971-980`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L971-L980)*

### `DW_CC_normal`
```rust
const DW_CC_normal: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_program`
```rust
const DW_CC_program: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_nocall`
```rust
const DW_CC_nocall: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_pass_by_reference`
```rust
const DW_CC_pass_by_reference: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_pass_by_value`
```rust
const DW_CC_pass_by_value: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_lo_user`
```rust
const DW_CC_lo_user: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_CC_hi_user`
```rust
const DW_CC_hi_user: DwCc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:982-994`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L982-L994)*

### `DW_INL_not_inlined`
```rust
const DW_INL_not_inlined: DwInl;
```

*Defined in [`gimli-0.32.3/src/constants.rs:996-1005`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L996-L1005)*

### `DW_INL_inlined`
```rust
const DW_INL_inlined: DwInl;
```

*Defined in [`gimli-0.32.3/src/constants.rs:996-1005`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L996-L1005)*

### `DW_INL_declared_not_inlined`
```rust
const DW_INL_declared_not_inlined: DwInl;
```

*Defined in [`gimli-0.32.3/src/constants.rs:996-1005`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L996-L1005)*

### `DW_INL_declared_inlined`
```rust
const DW_INL_declared_inlined: DwInl;
```

*Defined in [`gimli-0.32.3/src/constants.rs:996-1005`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L996-L1005)*

### `DW_ORD_row_major`
```rust
const DW_ORD_row_major: DwOrd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1007-1014`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1007-L1014)*

### `DW_ORD_col_major`
```rust
const DW_ORD_col_major: DwOrd;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1007-1014`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1007-L1014)*

### `DW_DSC_label`
```rust
const DW_DSC_label: DwDsc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1016-1023`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1016-L1023)*

### `DW_DSC_range`
```rust
const DW_DSC_range: DwDsc;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1016-1023`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1016-L1023)*

### `DW_IDX_compile_unit`
```rust
const DW_IDX_compile_unit: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_type_unit`
```rust
const DW_IDX_type_unit: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_die_offset`
```rust
const DW_IDX_die_offset: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_parent`
```rust
const DW_IDX_parent: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_type_hash`
```rust
const DW_IDX_type_hash: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_lo_user`
```rust
const DW_IDX_lo_user: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_IDX_hi_user`
```rust
const DW_IDX_hi_user: DwIdx;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1025-1037`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1025-L1037)*

### `DW_DEFAULTED_no`
```rust
const DW_DEFAULTED_no: DwDefaulted;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1039-1047`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1039-L1047)*

### `DW_DEFAULTED_in_class`
```rust
const DW_DEFAULTED_in_class: DwDefaulted;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1039-1047`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1039-L1047)*

### `DW_DEFAULTED_out_of_class`
```rust
const DW_DEFAULTED_out_of_class: DwDefaulted;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1039-1047`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1039-L1047)*

### `DW_LNS_copy`
```rust
const DW_LNS_copy: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_advance_pc`
```rust
const DW_LNS_advance_pc: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_advance_line`
```rust
const DW_LNS_advance_line: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_file`
```rust
const DW_LNS_set_file: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_column`
```rust
const DW_LNS_set_column: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_negate_stmt`
```rust
const DW_LNS_negate_stmt: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_basic_block`
```rust
const DW_LNS_set_basic_block: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_const_add_pc`
```rust
const DW_LNS_const_add_pc: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_fixed_advance_pc`
```rust
const DW_LNS_fixed_advance_pc: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_prologue_end`
```rust
const DW_LNS_set_prologue_end: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_epilogue_begin`
```rust
const DW_LNS_set_epilogue_begin: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNS_set_isa`
```rust
const DW_LNS_set_isa: DwLns;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1049-1066`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1049-L1066)*

### `DW_LNE_end_sequence`
```rust
const DW_LNE_end_sequence: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNE_set_address`
```rust
const DW_LNE_set_address: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNE_define_file`
```rust
const DW_LNE_define_file: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNE_set_discriminator`
```rust
const DW_LNE_set_discriminator: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNE_lo_user`
```rust
const DW_LNE_lo_user: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNE_hi_user`
```rust
const DW_LNE_hi_user: DwLne;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1068-1080`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1068-L1080)*

### `DW_LNCT_path`
```rust
const DW_LNCT_path: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_directory_index`
```rust
const DW_LNCT_directory_index: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_timestamp`
```rust
const DW_LNCT_timestamp: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_size`
```rust
const DW_LNCT_size: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_MD5`
```rust
const DW_LNCT_MD5: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_lo_user`
```rust
const DW_LNCT_lo_user: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_LLVM_source`
```rust
const DW_LNCT_LLVM_source: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_LNCT_hi_user`
```rust
const DW_LNCT_hi_user: DwLnct;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1082-1097`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1082-L1097)*

### `DW_MACINFO_define`
```rust
const DW_MACINFO_define: DwMacinfo;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

### `DW_MACINFO_undef`
```rust
const DW_MACINFO_undef: DwMacinfo;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

### `DW_MACINFO_start_file`
```rust
const DW_MACINFO_start_file: DwMacinfo;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

### `DW_MACINFO_end_file`
```rust
const DW_MACINFO_end_file: DwMacinfo;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

### `DW_MACINFO_vendor_ext`
```rust
const DW_MACINFO_vendor_ext: DwMacinfo;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1099-1109`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1099-L1109)*

### `DW_MACRO_define`
```rust
const DW_MACRO_define: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_undef`
```rust
const DW_MACRO_undef: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_start_file`
```rust
const DW_MACRO_start_file: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_end_file`
```rust
const DW_MACRO_end_file: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_define_strp`
```rust
const DW_MACRO_define_strp: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_undef_strp`
```rust
const DW_MACRO_undef_strp: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_import`
```rust
const DW_MACRO_import: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_define_sup`
```rust
const DW_MACRO_define_sup: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_undef_sup`
```rust
const DW_MACRO_undef_sup: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_import_sup`
```rust
const DW_MACRO_import_sup: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_define_strx`
```rust
const DW_MACRO_define_strx: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_undef_strx`
```rust
const DW_MACRO_undef_strx: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_lo_user`
```rust
const DW_MACRO_lo_user: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_MACRO_hi_user`
```rust
const DW_MACRO_hi_user: DwMacro;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1111-1130`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1111-L1130)*

### `DW_RLE_end_of_list`
```rust
const DW_RLE_end_of_list: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_base_addressx`
```rust
const DW_RLE_base_addressx: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_startx_endx`
```rust
const DW_RLE_startx_endx: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_startx_length`
```rust
const DW_RLE_startx_length: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_offset_pair`
```rust
const DW_RLE_offset_pair: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_base_address`
```rust
const DW_RLE_base_address: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_start_end`
```rust
const DW_RLE_start_end: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_RLE_start_length`
```rust
const DW_RLE_start_length: DwRle;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1132-1145`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1132-L1145)*

### `DW_OP_addr`
```rust
const DW_OP_addr: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_deref`
```rust
const DW_OP_deref: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const1u`
```rust
const DW_OP_const1u: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const1s`
```rust
const DW_OP_const1s: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const2u`
```rust
const DW_OP_const2u: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const2s`
```rust
const DW_OP_const2s: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const4u`
```rust
const DW_OP_const4u: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const4s`
```rust
const DW_OP_const4s: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const8u`
```rust
const DW_OP_const8u: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const8s`
```rust
const DW_OP_const8s: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_constu`
```rust
const DW_OP_constu: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_consts`
```rust
const DW_OP_consts: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_dup`
```rust
const DW_OP_dup: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_drop`
```rust
const DW_OP_drop: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_over`
```rust
const DW_OP_over: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_pick`
```rust
const DW_OP_pick: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_swap`
```rust
const DW_OP_swap: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_rot`
```rust
const DW_OP_rot: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_xderef`
```rust
const DW_OP_xderef: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_abs`
```rust
const DW_OP_abs: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_and`
```rust
const DW_OP_and: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_div`
```rust
const DW_OP_div: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_minus`
```rust
const DW_OP_minus: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_mod`
```rust
const DW_OP_mod: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_mul`
```rust
const DW_OP_mul: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_neg`
```rust
const DW_OP_neg: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_not`
```rust
const DW_OP_not: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_or`
```rust
const DW_OP_or: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_plus`
```rust
const DW_OP_plus: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_plus_uconst`
```rust
const DW_OP_plus_uconst: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_shl`
```rust
const DW_OP_shl: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_shr`
```rust
const DW_OP_shr: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_shra`
```rust
const DW_OP_shra: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_xor`
```rust
const DW_OP_xor: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_bra`
```rust
const DW_OP_bra: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_eq`
```rust
const DW_OP_eq: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_ge`
```rust
const DW_OP_ge: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_gt`
```rust
const DW_OP_gt: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_le`
```rust
const DW_OP_le: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lt`
```rust
const DW_OP_lt: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_ne`
```rust
const DW_OP_ne: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_skip`
```rust
const DW_OP_skip: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit0`
```rust
const DW_OP_lit0: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit1`
```rust
const DW_OP_lit1: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit2`
```rust
const DW_OP_lit2: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit3`
```rust
const DW_OP_lit3: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit4`
```rust
const DW_OP_lit4: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit5`
```rust
const DW_OP_lit5: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit6`
```rust
const DW_OP_lit6: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit7`
```rust
const DW_OP_lit7: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit8`
```rust
const DW_OP_lit8: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit9`
```rust
const DW_OP_lit9: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit10`
```rust
const DW_OP_lit10: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit11`
```rust
const DW_OP_lit11: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit12`
```rust
const DW_OP_lit12: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit13`
```rust
const DW_OP_lit13: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit14`
```rust
const DW_OP_lit14: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit15`
```rust
const DW_OP_lit15: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit16`
```rust
const DW_OP_lit16: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit17`
```rust
const DW_OP_lit17: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit18`
```rust
const DW_OP_lit18: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit19`
```rust
const DW_OP_lit19: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit20`
```rust
const DW_OP_lit20: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit21`
```rust
const DW_OP_lit21: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit22`
```rust
const DW_OP_lit22: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit23`
```rust
const DW_OP_lit23: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit24`
```rust
const DW_OP_lit24: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit25`
```rust
const DW_OP_lit25: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit26`
```rust
const DW_OP_lit26: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit27`
```rust
const DW_OP_lit27: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit28`
```rust
const DW_OP_lit28: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit29`
```rust
const DW_OP_lit29: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit30`
```rust
const DW_OP_lit30: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_lit31`
```rust
const DW_OP_lit31: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg0`
```rust
const DW_OP_reg0: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg1`
```rust
const DW_OP_reg1: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg2`
```rust
const DW_OP_reg2: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg3`
```rust
const DW_OP_reg3: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg4`
```rust
const DW_OP_reg4: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg5`
```rust
const DW_OP_reg5: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg6`
```rust
const DW_OP_reg6: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg7`
```rust
const DW_OP_reg7: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg8`
```rust
const DW_OP_reg8: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg9`
```rust
const DW_OP_reg9: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg10`
```rust
const DW_OP_reg10: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg11`
```rust
const DW_OP_reg11: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg12`
```rust
const DW_OP_reg12: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg13`
```rust
const DW_OP_reg13: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg14`
```rust
const DW_OP_reg14: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg15`
```rust
const DW_OP_reg15: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg16`
```rust
const DW_OP_reg16: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg17`
```rust
const DW_OP_reg17: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg18`
```rust
const DW_OP_reg18: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg19`
```rust
const DW_OP_reg19: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg20`
```rust
const DW_OP_reg20: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg21`
```rust
const DW_OP_reg21: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg22`
```rust
const DW_OP_reg22: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg23`
```rust
const DW_OP_reg23: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg24`
```rust
const DW_OP_reg24: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg25`
```rust
const DW_OP_reg25: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg26`
```rust
const DW_OP_reg26: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg27`
```rust
const DW_OP_reg27: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg28`
```rust
const DW_OP_reg28: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg29`
```rust
const DW_OP_reg29: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg30`
```rust
const DW_OP_reg30: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reg31`
```rust
const DW_OP_reg31: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg0`
```rust
const DW_OP_breg0: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg1`
```rust
const DW_OP_breg1: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg2`
```rust
const DW_OP_breg2: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg3`
```rust
const DW_OP_breg3: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg4`
```rust
const DW_OP_breg4: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg5`
```rust
const DW_OP_breg5: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg6`
```rust
const DW_OP_breg6: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg7`
```rust
const DW_OP_breg7: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg8`
```rust
const DW_OP_breg8: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg9`
```rust
const DW_OP_breg9: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg10`
```rust
const DW_OP_breg10: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg11`
```rust
const DW_OP_breg11: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg12`
```rust
const DW_OP_breg12: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg13`
```rust
const DW_OP_breg13: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg14`
```rust
const DW_OP_breg14: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg15`
```rust
const DW_OP_breg15: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg16`
```rust
const DW_OP_breg16: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg17`
```rust
const DW_OP_breg17: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg18`
```rust
const DW_OP_breg18: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg19`
```rust
const DW_OP_breg19: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg20`
```rust
const DW_OP_breg20: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg21`
```rust
const DW_OP_breg21: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg22`
```rust
const DW_OP_breg22: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg23`
```rust
const DW_OP_breg23: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg24`
```rust
const DW_OP_breg24: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg25`
```rust
const DW_OP_breg25: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg26`
```rust
const DW_OP_breg26: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg27`
```rust
const DW_OP_breg27: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg28`
```rust
const DW_OP_breg28: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg29`
```rust
const DW_OP_breg29: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg30`
```rust
const DW_OP_breg30: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_breg31`
```rust
const DW_OP_breg31: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_regx`
```rust
const DW_OP_regx: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_fbreg`
```rust
const DW_OP_fbreg: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_bregx`
```rust
const DW_OP_bregx: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_piece`
```rust
const DW_OP_piece: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_deref_size`
```rust
const DW_OP_deref_size: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_xderef_size`
```rust
const DW_OP_xderef_size: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_nop`
```rust
const DW_OP_nop: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_push_object_address`
```rust
const DW_OP_push_object_address: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_call2`
```rust
const DW_OP_call2: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_call4`
```rust
const DW_OP_call4: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_call_ref`
```rust
const DW_OP_call_ref: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_form_tls_address`
```rust
const DW_OP_form_tls_address: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_call_frame_cfa`
```rust
const DW_OP_call_frame_cfa: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_bit_piece`
```rust
const DW_OP_bit_piece: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_implicit_value`
```rust
const DW_OP_implicit_value: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_stack_value`
```rust
const DW_OP_stack_value: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_implicit_pointer`
```rust
const DW_OP_implicit_pointer: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_addrx`
```rust
const DW_OP_addrx: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_constx`
```rust
const DW_OP_constx: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_entry_value`
```rust
const DW_OP_entry_value: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_const_type`
```rust
const DW_OP_const_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_regval_type`
```rust
const DW_OP_regval_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_deref_type`
```rust
const DW_OP_deref_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_xderef_type`
```rust
const DW_OP_xderef_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_convert`
```rust
const DW_OP_convert: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_reinterpret`
```rust
const DW_OP_reinterpret: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_push_tls_address`
```rust
const DW_OP_GNU_push_tls_address: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_implicit_pointer`
```rust
const DW_OP_GNU_implicit_pointer: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_entry_value`
```rust
const DW_OP_GNU_entry_value: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_const_type`
```rust
const DW_OP_GNU_const_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_regval_type`
```rust
const DW_OP_GNU_regval_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_deref_type`
```rust
const DW_OP_GNU_deref_type: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_convert`
```rust
const DW_OP_GNU_convert: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_reinterpret`
```rust
const DW_OP_GNU_reinterpret: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_parameter_ref`
```rust
const DW_OP_GNU_parameter_ref: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_addr_index`
```rust
const DW_OP_GNU_addr_index: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_GNU_const_index`
```rust
const DW_OP_GNU_const_index: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_OP_WASM_location`
```rust
const DW_OP_WASM_location: DwOp;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1147-1332`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1147-L1332)*

### `DW_EH_PE_uleb128`
```rust
const DW_EH_PE_uleb128: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_udata2`
```rust
const DW_EH_PE_udata2: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_udata4`
```rust
const DW_EH_PE_udata4: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_udata8`
```rust
const DW_EH_PE_udata8: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_sleb128`
```rust
const DW_EH_PE_sleb128: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_sdata2`
```rust
const DW_EH_PE_sdata2: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_sdata4`
```rust
const DW_EH_PE_sdata4: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_sdata8`
```rust
const DW_EH_PE_sdata8: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_pcrel`
```rust
const DW_EH_PE_pcrel: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_textrel`
```rust
const DW_EH_PE_textrel: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_datarel`
```rust
const DW_EH_PE_datarel: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_funcrel`
```rust
const DW_EH_PE_funcrel: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_aligned`
```rust
const DW_EH_PE_aligned: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_indirect`
```rust
const DW_EH_PE_indirect: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_absptr`
```rust
const DW_EH_PE_absptr: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_omit`
```rust
const DW_EH_PE_omit: DwEhPe;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1334-1390`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1334-L1390)*

### `DW_EH_PE_FORMAT_MASK`
```rust
const DW_EH_PE_FORMAT_MASK: u8 = 15u8;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1392`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1392)*

### `DW_EH_PE_APPLICATION_MASK`
```rust
const DW_EH_PE_APPLICATION_MASK: u8 = 112u8;
```

*Defined in [`gimli-0.32.3/src/constants.rs:1395`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L1395)*

## Macros

### `dw!`

*Defined in [`gimli-0.32.3/src/constants.rs:58-102`](../../../.source_1765521767/gimli-0.32.3/src/constants.rs#L58-L102)*

