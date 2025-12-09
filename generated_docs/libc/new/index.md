*[libc](../index.md) / [new](index.md)*

---

# Module `new`

This module contains the future directory structure. If possible, new definitions should
get added here.

Eventually everything should be moved over, and we will move this directory to the top
level in `src`.

# Basic structure

Each child module here represents a library or group of libraries that we are binding. Each of
these has several submodules, representing either a directory or a header file in that library.

`#include`s turn into `pub use ...*;` statements. Then at the root level (here), we choose
which top-level headers we want to reexport the definitions for.

All modules are only crate-public since we don't reexport this structure.

## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`linux_uapi`](#linux_uapi)
  - [`glibc`](#glibc)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`bcm_timeval`](#bcm_timeval)
  - [`bcm_msg_head`](#bcm_msg_head)
  - [`j1939_filter`](#j1939_filter)
  - [`can_frame`](#can_frame)
  - [`canfd_frame`](#canfd_frame)
  - [`canxl_frame`](#canxl_frame)
  - [`sockaddr_can`](#sockaddr_can)
  - [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939)
  - [`can_filter`](#can_filter)
  - [`rtentry`](#rtentry)
- [Type Aliases](#type-aliases)
  - [`pgn_t`](#pgn_t)
  - [`priority_t`](#priority_t)
  - [`name_t`](#name_t)
  - [`canid_t`](#canid_t)
  - [`can_err_mask_t`](#can_err_mask_t)
- [Constants](#constants)
  - [`TX_SETUP`](#tx_setup)
  - [`TX_DELETE`](#tx_delete)
  - [`TX_READ`](#tx_read)
  - [`TX_SEND`](#tx_send)
  - [`RX_SETUP`](#rx_setup)
  - [`RX_DELETE`](#rx_delete)
  - [`RX_READ`](#rx_read)
  - [`TX_STATUS`](#tx_status)
  - [`TX_EXPIRED`](#tx_expired)
  - [`RX_STATUS`](#rx_status)
  - [`RX_TIMEOUT`](#rx_timeout)
  - [`RX_CHANGED`](#rx_changed)
  - [`SETTIMER`](#settimer)
  - [`STARTTIMER`](#starttimer)
  - [`TX_COUNTEVT`](#tx_countevt)
  - [`TX_ANNOUNCE`](#tx_announce)
  - [`TX_CP_CAN_ID`](#tx_cp_can_id)
  - [`RX_FILTER_ID`](#rx_filter_id)
  - [`RX_CHECK_DLC`](#rx_check_dlc)
  - [`RX_NO_AUTOTIMER`](#rx_no_autotimer)
  - [`RX_ANNOUNCE_RESUME`](#rx_announce_resume)
  - [`TX_RESET_MULTI_IDX`](#tx_reset_multi_idx)
  - [`RX_RTR_FRAME`](#rx_rtr_frame)
  - [`CAN_FD_FRAME`](#can_fd_frame)
  - [`J1939_MAX_UNICAST_ADDR`](#j1939_max_unicast_addr)
  - [`J1939_IDLE_ADDR`](#j1939_idle_addr)
  - [`J1939_NO_ADDR`](#j1939_no_addr)
  - [`J1939_NO_NAME`](#j1939_no_name)
  - [`J1939_PGN_REQUEST`](#j1939_pgn_request)
  - [`J1939_PGN_ADDRESS_CLAIMED`](#j1939_pgn_address_claimed)
  - [`J1939_PGN_ADDRESS_COMMANDED`](#j1939_pgn_address_commanded)
  - [`J1939_PGN_PDU1_MAX`](#j1939_pgn_pdu1_max)
  - [`J1939_PGN_MAX`](#j1939_pgn_max)
  - [`J1939_NO_PGN`](#j1939_no_pgn)
  - [`SOL_CAN_J1939`](#sol_can_j1939)
  - [`SO_J1939_FILTER`](#so_j1939_filter)
  - [`SO_J1939_PROMISC`](#so_j1939_promisc)
  - [`SO_J1939_SEND_PRIO`](#so_j1939_send_prio)
  - [`SO_J1939_ERRQUEUE`](#so_j1939_errqueue)
  - [`SCM_J1939_DEST_ADDR`](#scm_j1939_dest_addr)
  - [`SCM_J1939_DEST_NAME`](#scm_j1939_dest_name)
  - [`SCM_J1939_PRIO`](#scm_j1939_prio)
  - [`SCM_J1939_ERRQUEUE`](#scm_j1939_errqueue)
  - [`J1939_NLA_PAD`](#j1939_nla_pad)
  - [`J1939_NLA_BYTES_ACKED`](#j1939_nla_bytes_acked)
  - [`J1939_NLA_TOTAL_SIZE`](#j1939_nla_total_size)
  - [`J1939_NLA_PGN`](#j1939_nla_pgn)
  - [`J1939_NLA_SRC_NAME`](#j1939_nla_src_name)
  - [`J1939_NLA_DEST_NAME`](#j1939_nla_dest_name)
  - [`J1939_NLA_SRC_ADDR`](#j1939_nla_src_addr)
  - [`J1939_NLA_DEST_ADDR`](#j1939_nla_dest_addr)
  - [`J1939_EE_INFO_NONE`](#j1939_ee_info_none)
  - [`J1939_EE_INFO_TX_ABORT`](#j1939_ee_info_tx_abort)
  - [`J1939_EE_INFO_RX_RTS`](#j1939_ee_info_rx_rts)
  - [`J1939_EE_INFO_RX_DPO`](#j1939_ee_info_rx_dpo)
  - [`J1939_EE_INFO_RX_ABORT`](#j1939_ee_info_rx_abort)
  - [`J1939_FILTER_MAX`](#j1939_filter_max)
  - [`SOL_CAN_RAW`](#sol_can_raw)
  - [`CAN_RAW_FILTER_MAX`](#can_raw_filter_max)
  - [`CAN_RAW_FILTER`](#can_raw_filter)
  - [`CAN_RAW_ERR_FILTER`](#can_raw_err_filter)
  - [`CAN_RAW_LOOPBACK`](#can_raw_loopback)
  - [`CAN_RAW_RECV_OWN_MSGS`](#can_raw_recv_own_msgs)
  - [`CAN_RAW_FD_FRAMES`](#can_raw_fd_frames)
  - [`CAN_RAW_JOIN_FILTERS`](#can_raw_join_filters)
  - [`CAN_RAW_XL_FRAMES`](#can_raw_xl_frames)
  - [`CAN_EFF_FLAG`](#can_eff_flag)
  - [`CAN_RTR_FLAG`](#can_rtr_flag)
  - [`CAN_ERR_FLAG`](#can_err_flag)
  - [`CAN_SFF_MASK`](#can_sff_mask)
  - [`CAN_EFF_MASK`](#can_eff_mask)
  - [`CAN_ERR_MASK`](#can_err_mask)
  - [`CANXL_PRIO_MASK`](#canxl_prio_mask)
  - [`CAN_SFF_ID_BITS`](#can_sff_id_bits)
  - [`CAN_EFF_ID_BITS`](#can_eff_id_bits)
  - [`CANXL_PRIO_BITS`](#canxl_prio_bits)
  - [`CAN_MAX_DLC`](#can_max_dlc)
  - [`CAN_MAX_DLEN`](#can_max_dlen)
  - [`CANFD_MAX_DLC`](#canfd_max_dlc)
  - [`CANFD_MAX_DLEN`](#canfd_max_dlen)
  - [`CANXL_MIN_DLC`](#canxl_min_dlc)
  - [`CANXL_MAX_DLC`](#canxl_max_dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl_max_dlc_mask)
  - [`CANXL_MIN_DLEN`](#canxl_min_dlen)
  - [`CANXL_MAX_DLEN`](#canxl_max_dlen)
  - [`CANFD_BRS`](#canfd_brs)
  - [`CANFD_ESI`](#canfd_esi)
  - [`CANFD_FDF`](#canfd_fdf)
  - [`CANXL_XLF`](#canxl_xlf)
  - [`CANXL_SEC`](#canxl_sec)
  - [`CAN_MTU`](#can_mtu)
  - [`CANFD_MTU`](#canfd_mtu)
  - [`CANXL_MTU`](#canxl_mtu)
  - [`CANXL_HDR_SIZE`](#canxl_hdr_size)
  - [`CANXL_MIN_MTU`](#canxl_min_mtu)
  - [`CANXL_MAX_MTU`](#canxl_max_mtu)
  - [`CAN_RAW`](#can_raw)
  - [`CAN_BCM`](#can_bcm)
  - [`CAN_TP16`](#can_tp16)
  - [`CAN_TP20`](#can_tp20)
  - [`CAN_MCNET`](#can_mcnet)
  - [`CAN_ISOTP`](#can_isotp)
  - [`CAN_J1939`](#can_j1939)
  - [`CAN_NPROTO`](#can_nproto)
  - [`SOL_CAN_BASE`](#sol_can_base)
  - [`CAN_INV_FILTER`](#can_inv_filter)
  - [`KEY_SPEC_THREAD_KEYRING`](#key_spec_thread_keyring)
  - [`KEY_SPEC_PROCESS_KEYRING`](#key_spec_process_keyring)
  - [`KEY_SPEC_SESSION_KEYRING`](#key_spec_session_keyring)
  - [`KEY_SPEC_USER_KEYRING`](#key_spec_user_keyring)
  - [`KEY_SPEC_USER_SESSION_KEYRING`](#key_spec_user_session_keyring)
  - [`KEY_SPEC_GROUP_KEYRING`](#key_spec_group_keyring)
  - [`KEY_SPEC_REQKEY_AUTH_KEY`](#key_spec_reqkey_auth_key)
  - [`KEY_SPEC_REQUESTOR_KEYRING`](#key_spec_requestor_keyring)
  - [`KEY_REQKEY_DEFL_NO_CHANGE`](#key_reqkey_defl_no_change)
  - [`KEY_REQKEY_DEFL_DEFAULT`](#key_reqkey_defl_default)
  - [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key_reqkey_defl_thread_keyring)
  - [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key_reqkey_defl_process_keyring)
  - [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key_reqkey_defl_session_keyring)
  - [`KEY_REQKEY_DEFL_USER_KEYRING`](#key_reqkey_defl_user_keyring)
  - [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key_reqkey_defl_user_session_keyring)
  - [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key_reqkey_defl_group_keyring)
  - [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key_reqkey_defl_requestor_keyring)
  - [`KEYCTL_GET_KEYRING_ID`](#keyctl_get_keyring_id)
  - [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl_join_session_keyring)
  - [`KEYCTL_UPDATE`](#keyctl_update)
  - [`KEYCTL_REVOKE`](#keyctl_revoke)
  - [`KEYCTL_CHOWN`](#keyctl_chown)
  - [`KEYCTL_SETPERM`](#keyctl_setperm)
  - [`KEYCTL_DESCRIBE`](#keyctl_describe)
  - [`KEYCTL_CLEAR`](#keyctl_clear)
  - [`KEYCTL_LINK`](#keyctl_link)
  - [`KEYCTL_UNLINK`](#keyctl_unlink)
  - [`KEYCTL_SEARCH`](#keyctl_search)
  - [`KEYCTL_READ`](#keyctl_read)
  - [`KEYCTL_INSTANTIATE`](#keyctl_instantiate)
  - [`KEYCTL_NEGATE`](#keyctl_negate)
  - [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl_set_reqkey_keyring)
  - [`KEYCTL_SET_TIMEOUT`](#keyctl_set_timeout)
  - [`KEYCTL_ASSUME_AUTHORITY`](#keyctl_assume_authority)
  - [`KEYCTL_GET_SECURITY`](#keyctl_get_security)
  - [`KEYCTL_SESSION_TO_PARENT`](#keyctl_session_to_parent)
  - [`KEYCTL_REJECT`](#keyctl_reject)
  - [`KEYCTL_INSTANTIATE_IOV`](#keyctl_instantiate_iov)
  - [`KEYCTL_INVALIDATE`](#keyctl_invalidate)
  - [`KEYCTL_GET_PERSISTENT`](#keyctl_get_persistent)
  - [`KEYCTL_DH_COMPUTE`](#keyctl_dh_compute)
  - [`KEYCTL_PKEY_QUERY`](#keyctl_pkey_query)
  - [`KEYCTL_PKEY_ENCRYPT`](#keyctl_pkey_encrypt)
  - [`KEYCTL_PKEY_DECRYPT`](#keyctl_pkey_decrypt)
  - [`KEYCTL_PKEY_SIGN`](#keyctl_pkey_sign)
  - [`KEYCTL_PKEY_VERIFY`](#keyctl_pkey_verify)
  - [`KEYCTL_RESTRICT_KEYRING`](#keyctl_restrict_keyring)
  - [`KEYCTL_MOVE`](#keyctl_move)
  - [`KEYCTL_CAPABILITIES`](#keyctl_capabilities)
  - [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl_supports_encrypt)
  - [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl_supports_decrypt)
  - [`KEYCTL_SUPPORTS_SIGN`](#keyctl_supports_sign)
  - [`KEYCTL_SUPPORTS_VERIFY`](#keyctl_supports_verify)
  - [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl_caps0_capabilities)
  - [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl_caps0_persistent_keyrings)
  - [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl_caps0_diffie_hellman)
  - [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl_caps0_public_key)
  - [`KEYCTL_CAPS0_BIG_KEY`](#keyctl_caps0_big_key)
  - [`KEYCTL_CAPS0_INVALIDATE`](#keyctl_caps0_invalidate)
  - [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl_caps0_restrict_keyring)
  - [`KEYCTL_CAPS0_MOVE`](#keyctl_caps0_move)
  - [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl_caps1_ns_keyring_name)
  - [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl_caps1_ns_key_tag)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod | Interfaces that are common across multiple platforms |
| [`linux_uapi`](#linux_uapi) | mod | This directory maps to `include/uapi` in the Linux source tree. |
| [`glibc`](#glibc) | mod | GNU libc. |
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`bcm_timeval`](#bcm_timeval) | struct |  |
| [`bcm_msg_head`](#bcm_msg_head) | struct |  |
| [`j1939_filter`](#j1939_filter) | struct |  |
| [`can_frame`](#can_frame) | struct |  |
| [`canfd_frame`](#canfd_frame) | struct |  |
| [`canxl_frame`](#canxl_frame) | struct |  |
| [`sockaddr_can`](#sockaddr_can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939) | struct |  |
| [`can_filter`](#can_filter) | struct |  |
| [`rtentry`](#rtentry) | struct |  |
| [`pgn_t`](#pgn_t) | type |  |
| [`priority_t`](#priority_t) | type |  |
| [`name_t`](#name_t) | type |  |
| [`canid_t`](#canid_t) | type |  |
| [`can_err_mask_t`](#can_err_mask_t) | type |  |
| [`TX_SETUP`](#tx_setup) | const |  |
| [`TX_DELETE`](#tx_delete) | const |  |
| [`TX_READ`](#tx_read) | const |  |
| [`TX_SEND`](#tx_send) | const |  |
| [`RX_SETUP`](#rx_setup) | const |  |
| [`RX_DELETE`](#rx_delete) | const |  |
| [`RX_READ`](#rx_read) | const |  |
| [`TX_STATUS`](#tx_status) | const |  |
| [`TX_EXPIRED`](#tx_expired) | const |  |
| [`RX_STATUS`](#rx_status) | const |  |
| [`RX_TIMEOUT`](#rx_timeout) | const |  |
| [`RX_CHANGED`](#rx_changed) | const |  |
| [`SETTIMER`](#settimer) | const |  |
| [`STARTTIMER`](#starttimer) | const |  |
| [`TX_COUNTEVT`](#tx_countevt) | const |  |
| [`TX_ANNOUNCE`](#tx_announce) | const |  |
| [`TX_CP_CAN_ID`](#tx_cp_can_id) | const |  |
| [`RX_FILTER_ID`](#rx_filter_id) | const |  |
| [`RX_CHECK_DLC`](#rx_check_dlc) | const |  |
| [`RX_NO_AUTOTIMER`](#rx_no_autotimer) | const |  |
| [`RX_ANNOUNCE_RESUME`](#rx_announce_resume) | const |  |
| [`TX_RESET_MULTI_IDX`](#tx_reset_multi_idx) | const |  |
| [`RX_RTR_FRAME`](#rx_rtr_frame) | const |  |
| [`CAN_FD_FRAME`](#can_fd_frame) | const |  |
| [`J1939_MAX_UNICAST_ADDR`](#j1939_max_unicast_addr) | const |  |
| [`J1939_IDLE_ADDR`](#j1939_idle_addr) | const |  |
| [`J1939_NO_ADDR`](#j1939_no_addr) | const |  |
| [`J1939_NO_NAME`](#j1939_no_name) | const |  |
| [`J1939_PGN_REQUEST`](#j1939_pgn_request) | const |  |
| [`J1939_PGN_ADDRESS_CLAIMED`](#j1939_pgn_address_claimed) | const |  |
| [`J1939_PGN_ADDRESS_COMMANDED`](#j1939_pgn_address_commanded) | const |  |
| [`J1939_PGN_PDU1_MAX`](#j1939_pgn_pdu1_max) | const |  |
| [`J1939_PGN_MAX`](#j1939_pgn_max) | const |  |
| [`J1939_NO_PGN`](#j1939_no_pgn) | const |  |
| [`SOL_CAN_J1939`](#sol_can_j1939) | const |  |
| [`SO_J1939_FILTER`](#so_j1939_filter) | const |  |
| [`SO_J1939_PROMISC`](#so_j1939_promisc) | const |  |
| [`SO_J1939_SEND_PRIO`](#so_j1939_send_prio) | const |  |
| [`SO_J1939_ERRQUEUE`](#so_j1939_errqueue) | const |  |
| [`SCM_J1939_DEST_ADDR`](#scm_j1939_dest_addr) | const |  |
| [`SCM_J1939_DEST_NAME`](#scm_j1939_dest_name) | const |  |
| [`SCM_J1939_PRIO`](#scm_j1939_prio) | const |  |
| [`SCM_J1939_ERRQUEUE`](#scm_j1939_errqueue) | const |  |
| [`J1939_NLA_PAD`](#j1939_nla_pad) | const |  |
| [`J1939_NLA_BYTES_ACKED`](#j1939_nla_bytes_acked) | const |  |
| [`J1939_NLA_TOTAL_SIZE`](#j1939_nla_total_size) | const |  |
| [`J1939_NLA_PGN`](#j1939_nla_pgn) | const |  |
| [`J1939_NLA_SRC_NAME`](#j1939_nla_src_name) | const |  |
| [`J1939_NLA_DEST_NAME`](#j1939_nla_dest_name) | const |  |
| [`J1939_NLA_SRC_ADDR`](#j1939_nla_src_addr) | const |  |
| [`J1939_NLA_DEST_ADDR`](#j1939_nla_dest_addr) | const |  |
| [`J1939_EE_INFO_NONE`](#j1939_ee_info_none) | const |  |
| [`J1939_EE_INFO_TX_ABORT`](#j1939_ee_info_tx_abort) | const |  |
| [`J1939_EE_INFO_RX_RTS`](#j1939_ee_info_rx_rts) | const |  |
| [`J1939_EE_INFO_RX_DPO`](#j1939_ee_info_rx_dpo) | const |  |
| [`J1939_EE_INFO_RX_ABORT`](#j1939_ee_info_rx_abort) | const |  |
| [`J1939_FILTER_MAX`](#j1939_filter_max) | const |  |
| [`SOL_CAN_RAW`](#sol_can_raw) | const |  |
| [`CAN_RAW_FILTER_MAX`](#can_raw_filter_max) | const |  |
| [`CAN_RAW_FILTER`](#can_raw_filter) | const |  |
| [`CAN_RAW_ERR_FILTER`](#can_raw_err_filter) | const |  |
| [`CAN_RAW_LOOPBACK`](#can_raw_loopback) | const |  |
| [`CAN_RAW_RECV_OWN_MSGS`](#can_raw_recv_own_msgs) | const |  |
| [`CAN_RAW_FD_FRAMES`](#can_raw_fd_frames) | const |  |
| [`CAN_RAW_JOIN_FILTERS`](#can_raw_join_filters) | const |  |
| [`CAN_RAW_XL_FRAMES`](#can_raw_xl_frames) | const |  |
| [`CAN_EFF_FLAG`](#can_eff_flag) | const |  |
| [`CAN_RTR_FLAG`](#can_rtr_flag) | const |  |
| [`CAN_ERR_FLAG`](#can_err_flag) | const |  |
| [`CAN_SFF_MASK`](#can_sff_mask) | const |  |
| [`CAN_EFF_MASK`](#can_eff_mask) | const |  |
| [`CAN_ERR_MASK`](#can_err_mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl_prio_mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can_sff_id_bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can_eff_id_bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl_prio_bits) | const |  |
| [`CAN_MAX_DLC`](#can_max_dlc) | const |  |
| [`CAN_MAX_DLEN`](#can_max_dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd_max_dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd_max_dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl_min_dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl_max_dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl_max_dlc_mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl_min_dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl_max_dlen) | const |  |
| [`CANFD_BRS`](#canfd_brs) | const |  |
| [`CANFD_ESI`](#canfd_esi) | const |  |
| [`CANFD_FDF`](#canfd_fdf) | const |  |
| [`CANXL_XLF`](#canxl_xlf) | const |  |
| [`CANXL_SEC`](#canxl_sec) | const |  |
| [`CAN_MTU`](#can_mtu) | const |  |
| [`CANFD_MTU`](#canfd_mtu) | const |  |
| [`CANXL_MTU`](#canxl_mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl_hdr_size) | const |  |
| [`CANXL_MIN_MTU`](#canxl_min_mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl_max_mtu) | const |  |
| [`CAN_RAW`](#can_raw) | const |  |
| [`CAN_BCM`](#can_bcm) | const |  |
| [`CAN_TP16`](#can_tp16) | const |  |
| [`CAN_TP20`](#can_tp20) | const |  |
| [`CAN_MCNET`](#can_mcnet) | const |  |
| [`CAN_ISOTP`](#can_isotp) | const |  |
| [`CAN_J1939`](#can_j1939) | const |  |
| [`CAN_NPROTO`](#can_nproto) | const |  |
| [`SOL_CAN_BASE`](#sol_can_base) | const |  |
| [`CAN_INV_FILTER`](#can_inv_filter) | const |  |
| [`KEY_SPEC_THREAD_KEYRING`](#key_spec_thread_keyring) | const |  |
| [`KEY_SPEC_PROCESS_KEYRING`](#key_spec_process_keyring) | const |  |
| [`KEY_SPEC_SESSION_KEYRING`](#key_spec_session_keyring) | const |  |
| [`KEY_SPEC_USER_KEYRING`](#key_spec_user_keyring) | const |  |
| [`KEY_SPEC_USER_SESSION_KEYRING`](#key_spec_user_session_keyring) | const |  |
| [`KEY_SPEC_GROUP_KEYRING`](#key_spec_group_keyring) | const |  |
| [`KEY_SPEC_REQKEY_AUTH_KEY`](#key_spec_reqkey_auth_key) | const |  |
| [`KEY_SPEC_REQUESTOR_KEYRING`](#key_spec_requestor_keyring) | const |  |
| [`KEY_REQKEY_DEFL_NO_CHANGE`](#key_reqkey_defl_no_change) | const |  |
| [`KEY_REQKEY_DEFL_DEFAULT`](#key_reqkey_defl_default) | const |  |
| [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key_reqkey_defl_thread_keyring) | const |  |
| [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key_reqkey_defl_process_keyring) | const |  |
| [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key_reqkey_defl_session_keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_KEYRING`](#key_reqkey_defl_user_keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key_reqkey_defl_user_session_keyring) | const |  |
| [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key_reqkey_defl_group_keyring) | const |  |
| [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key_reqkey_defl_requestor_keyring) | const |  |
| [`KEYCTL_GET_KEYRING_ID`](#keyctl_get_keyring_id) | const |  |
| [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl_join_session_keyring) | const |  |
| [`KEYCTL_UPDATE`](#keyctl_update) | const |  |
| [`KEYCTL_REVOKE`](#keyctl_revoke) | const |  |
| [`KEYCTL_CHOWN`](#keyctl_chown) | const |  |
| [`KEYCTL_SETPERM`](#keyctl_setperm) | const |  |
| [`KEYCTL_DESCRIBE`](#keyctl_describe) | const |  |
| [`KEYCTL_CLEAR`](#keyctl_clear) | const |  |
| [`KEYCTL_LINK`](#keyctl_link) | const |  |
| [`KEYCTL_UNLINK`](#keyctl_unlink) | const |  |
| [`KEYCTL_SEARCH`](#keyctl_search) | const |  |
| [`KEYCTL_READ`](#keyctl_read) | const |  |
| [`KEYCTL_INSTANTIATE`](#keyctl_instantiate) | const |  |
| [`KEYCTL_NEGATE`](#keyctl_negate) | const |  |
| [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl_set_reqkey_keyring) | const |  |
| [`KEYCTL_SET_TIMEOUT`](#keyctl_set_timeout) | const |  |
| [`KEYCTL_ASSUME_AUTHORITY`](#keyctl_assume_authority) | const |  |
| [`KEYCTL_GET_SECURITY`](#keyctl_get_security) | const |  |
| [`KEYCTL_SESSION_TO_PARENT`](#keyctl_session_to_parent) | const |  |
| [`KEYCTL_REJECT`](#keyctl_reject) | const |  |
| [`KEYCTL_INSTANTIATE_IOV`](#keyctl_instantiate_iov) | const |  |
| [`KEYCTL_INVALIDATE`](#keyctl_invalidate) | const |  |
| [`KEYCTL_GET_PERSISTENT`](#keyctl_get_persistent) | const |  |
| [`KEYCTL_DH_COMPUTE`](#keyctl_dh_compute) | const |  |
| [`KEYCTL_PKEY_QUERY`](#keyctl_pkey_query) | const |  |
| [`KEYCTL_PKEY_ENCRYPT`](#keyctl_pkey_encrypt) | const |  |
| [`KEYCTL_PKEY_DECRYPT`](#keyctl_pkey_decrypt) | const |  |
| [`KEYCTL_PKEY_SIGN`](#keyctl_pkey_sign) | const |  |
| [`KEYCTL_PKEY_VERIFY`](#keyctl_pkey_verify) | const |  |
| [`KEYCTL_RESTRICT_KEYRING`](#keyctl_restrict_keyring) | const |  |
| [`KEYCTL_MOVE`](#keyctl_move) | const |  |
| [`KEYCTL_CAPABILITIES`](#keyctl_capabilities) | const |  |
| [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl_supports_encrypt) | const |  |
| [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl_supports_decrypt) | const |  |
| [`KEYCTL_SUPPORTS_SIGN`](#keyctl_supports_sign) | const |  |
| [`KEYCTL_SUPPORTS_VERIFY`](#keyctl_supports_verify) | const |  |
| [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl_caps0_capabilities) | const |  |
| [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl_caps0_persistent_keyrings) | const |  |
| [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl_caps0_diffie_hellman) | const |  |
| [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl_caps0_public_key) | const |  |
| [`KEYCTL_CAPS0_BIG_KEY`](#keyctl_caps0_big_key) | const |  |
| [`KEYCTL_CAPS0_INVALIDATE`](#keyctl_caps0_invalidate) | const |  |
| [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl_caps0_restrict_keyring) | const |  |
| [`KEYCTL_CAPS0_MOVE`](#keyctl_caps0_move) | const |  |
| [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl_caps1_ns_keyring_name) | const |  |
| [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl_caps1_ns_key_tag) | const |  |

## Modules

- [`common`](common/index.md) — Interfaces that are common across multiple platforms
- [`linux_uapi`](linux_uapi/index.md) — This directory maps to `include/uapi` in the Linux source tree.
- [`glibc`](glibc/index.md) — GNU libc.
- [`bcm`](bcm/index.md) — Header: `linux/can/bcm.h`
- [`j1939`](j1939/index.md) — `linux/can/j1939.h`
- [`raw`](raw/index.md) — Header: `linux/can/raw.h`

## Structs

### `bcm_timeval`

```rust
struct bcm_timeval {
    pub tv_sec: crate::c_long,
    pub tv_usec: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for bcm_timeval`

- <span id="bcm-timeval-clone"></span>`fn clone(&self) -> bcm_timeval` — [`bcm_timeval`](#bcm-timeval)

##### `impl Copy for bcm_timeval`

##### `impl Debug for bcm_timeval`

- <span id="bcm-timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `bcm_msg_head`

```rust
struct bcm_msg_head {
    pub opcode: u32,
    pub flags: u32,
    pub count: u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: u32,
    pub frames: [can_frame; 0],
}
```

#### Trait Implementations

##### `impl Clone for bcm_msg_head`

- <span id="bcm-msg-head-clone"></span>`fn clone(&self) -> bcm_msg_head` — [`bcm_msg_head`](#bcm-msg-head)

##### `impl Copy for bcm_msg_head`

##### `impl Debug for bcm_msg_head`

- <span id="bcm-msg-head-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `j1939_filter`

```rust
struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: u8,
    pub addr_mask: u8,
}
```

#### Trait Implementations

##### `impl Clone for j1939_filter`

- <span id="j1939-filter-clone"></span>`fn clone(&self) -> j1939_filter` — [`j1939_filter`](#j1939-filter)

##### `impl Copy for j1939_filter`

##### `impl Debug for j1939_filter`

- <span id="j1939-filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_frame`

```rust
struct can_frame {
    pub can_id: canid_t,
    pub can_dlc: u8,
    __pad: crate::types::Padding<u8>,
    __res0: u8,
    pub len8_dlc: u8,
    pub data: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for can_frame`

- <span id="can-frame-clone"></span>`fn clone(&self) -> can_frame` — [`can_frame`](#can-frame)

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- <span id="can-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canfd_frame`

```rust
struct canfd_frame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    __res0: u8,
    __res1: u8,
    pub data: [u8; 64],
}
```

#### Trait Implementations

##### `impl Clone for canfd_frame`

- <span id="canfd-frame-clone"></span>`fn clone(&self) -> canfd_frame` — [`canfd_frame`](#canfd-frame)

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- <span id="canfd-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canxl_frame`

```rust
struct canxl_frame {
    pub prio: canid_t,
    pub flags: u8,
    pub sdt: u8,
    pub len: u16,
    pub af: u32,
    pub data: [u8; 2048],
}
```

#### Trait Implementations

##### `impl Clone for canxl_frame`

- <span id="canxl-frame-clone"></span>`fn clone(&self) -> canxl_frame` — [`canxl_frame`](#canxl-frame)

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- <span id="canxl-frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_can`

```rust
struct sockaddr_can {
    pub can_family: crate::sa_family_t,
    pub can_ifindex: crate::c_int,
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_can`

- <span id="sockaddr-can-clone"></span>`fn clone(&self) -> sockaddr_can` — [`sockaddr_can`](#sockaddr-can)

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- <span id="sockaddr-can-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_j1939`

```rust
struct __c_anonymous_sockaddr_can_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for can_filter`

- <span id="can-filter-clone"></span>`fn clone(&self) -> can_filter` — [`can_filter`](#can-filter)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- <span id="can-filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rtentry`

```rust
struct rtentry {
    pub rt_pad1: crate::c_ulong,
    pub rt_dst: crate::sockaddr,
    pub rt_gateway: crate::sockaddr,
    pub rt_genmask: crate::sockaddr,
    pub rt_flags: crate::c_ushort,
    pub rt_pad2: crate::c_short,
    pub rt_pad3: crate::c_ulong,
    pub rt_tos: crate::c_uchar,
    pub rt_class: crate::c_uchar,
    pub rt_pad4: [crate::c_short; 3],
    pub rt_metric: crate::c_short,
    pub rt_dev: *mut crate::c_char,
    pub rt_mtu: crate::c_ulong,
    pub rt_window: crate::c_ulong,
    pub rt_irtt: crate::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for rtentry`

- <span id="rtentry-clone"></span>`fn clone(&self) -> rtentry` — [`rtentry`](#rtentry)

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

- <span id="rtentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `pgn_t`

```rust
type pgn_t = u32;
```

### `priority_t`

```rust
type priority_t = u8;
```

### `name_t`

```rust
type name_t = u64;
```

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

## Constants

### `TX_SETUP`

```rust
const TX_SETUP: u32 = 1u32;
```

### `TX_DELETE`

```rust
const TX_DELETE: u32 = 2u32;
```

### `TX_READ`

```rust
const TX_READ: u32 = 3u32;
```

### `TX_SEND`

```rust
const TX_SEND: u32 = 4u32;
```

### `RX_SETUP`

```rust
const RX_SETUP: u32 = 5u32;
```

### `RX_DELETE`

```rust
const RX_DELETE: u32 = 6u32;
```

### `RX_READ`

```rust
const RX_READ: u32 = 7u32;
```

### `TX_STATUS`

```rust
const TX_STATUS: u32 = 8u32;
```

### `TX_EXPIRED`

```rust
const TX_EXPIRED: u32 = 9u32;
```

### `RX_STATUS`

```rust
const RX_STATUS: u32 = 10u32;
```

### `RX_TIMEOUT`

```rust
const RX_TIMEOUT: u32 = 11u32;
```

### `RX_CHANGED`

```rust
const RX_CHANGED: u32 = 12u32;
```

### `SETTIMER`

```rust
const SETTIMER: u32 = 1u32;
```

### `STARTTIMER`

```rust
const STARTTIMER: u32 = 2u32;
```

### `TX_COUNTEVT`

```rust
const TX_COUNTEVT: u32 = 4u32;
```

### `TX_ANNOUNCE`

```rust
const TX_ANNOUNCE: u32 = 8u32;
```

### `TX_CP_CAN_ID`

```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

### `RX_FILTER_ID`

```rust
const RX_FILTER_ID: u32 = 32u32;
```

### `RX_CHECK_DLC`

```rust
const RX_CHECK_DLC: u32 = 64u32;
```

### `RX_NO_AUTOTIMER`

```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

### `RX_ANNOUNCE_RESUME`

```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

### `TX_RESET_MULTI_IDX`

```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

### `RX_RTR_FRAME`

```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

### `CAN_FD_FRAME`

```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

### `J1939_MAX_UNICAST_ADDR`

```rust
const J1939_MAX_UNICAST_ADDR: crate::c_uchar = 253u8;
```

### `J1939_IDLE_ADDR`

```rust
const J1939_IDLE_ADDR: crate::c_uchar = 254u8;
```

### `J1939_NO_ADDR`

```rust
const J1939_NO_ADDR: crate::c_uchar = 255u8;
```

### `J1939_NO_NAME`

```rust
const J1939_NO_NAME: crate::c_ulong = 0u64;
```

### `J1939_PGN_REQUEST`

```rust
const J1939_PGN_REQUEST: crate::c_uint = 59_904u32;
```

### `J1939_PGN_ADDRESS_CLAIMED`

```rust
const J1939_PGN_ADDRESS_CLAIMED: crate::c_uint = 60_928u32;
```

### `J1939_PGN_ADDRESS_COMMANDED`

```rust
const J1939_PGN_ADDRESS_COMMANDED: crate::c_uint = 65_240u32;
```

### `J1939_PGN_PDU1_MAX`

```rust
const J1939_PGN_PDU1_MAX: crate::c_uint = 261_888u32;
```

### `J1939_PGN_MAX`

```rust
const J1939_PGN_MAX: crate::c_uint = 262_143u32;
```

### `J1939_NO_PGN`

```rust
const J1939_NO_PGN: crate::c_uint = 262_144u32;
```

### `SOL_CAN_J1939`

```rust
const SOL_CAN_J1939: crate::c_int = 107i32;
```

### `SO_J1939_FILTER`

```rust
const SO_J1939_FILTER: crate::c_int = 1i32;
```

### `SO_J1939_PROMISC`

```rust
const SO_J1939_PROMISC: crate::c_int = 2i32;
```

### `SO_J1939_SEND_PRIO`

```rust
const SO_J1939_SEND_PRIO: crate::c_int = 3i32;
```

### `SO_J1939_ERRQUEUE`

```rust
const SO_J1939_ERRQUEUE: crate::c_int = 4i32;
```

### `SCM_J1939_DEST_ADDR`

```rust
const SCM_J1939_DEST_ADDR: crate::c_int = 1i32;
```

### `SCM_J1939_DEST_NAME`

```rust
const SCM_J1939_DEST_NAME: crate::c_int = 2i32;
```

### `SCM_J1939_PRIO`

```rust
const SCM_J1939_PRIO: crate::c_int = 3i32;
```

### `SCM_J1939_ERRQUEUE`

```rust
const SCM_J1939_ERRQUEUE: crate::c_int = 4i32;
```

### `J1939_NLA_PAD`

```rust
const J1939_NLA_PAD: crate::c_int = 0i32;
```

### `J1939_NLA_BYTES_ACKED`

```rust
const J1939_NLA_BYTES_ACKED: crate::c_int = 1i32;
```

### `J1939_NLA_TOTAL_SIZE`

```rust
const J1939_NLA_TOTAL_SIZE: crate::c_int = 2i32;
```

### `J1939_NLA_PGN`

```rust
const J1939_NLA_PGN: crate::c_int = 3i32;
```

### `J1939_NLA_SRC_NAME`

```rust
const J1939_NLA_SRC_NAME: crate::c_int = 4i32;
```

### `J1939_NLA_DEST_NAME`

```rust
const J1939_NLA_DEST_NAME: crate::c_int = 5i32;
```

### `J1939_NLA_SRC_ADDR`

```rust
const J1939_NLA_SRC_ADDR: crate::c_int = 6i32;
```

### `J1939_NLA_DEST_ADDR`

```rust
const J1939_NLA_DEST_ADDR: crate::c_int = 7i32;
```

### `J1939_EE_INFO_NONE`

```rust
const J1939_EE_INFO_NONE: crate::c_int = 0i32;
```

### `J1939_EE_INFO_TX_ABORT`

```rust
const J1939_EE_INFO_TX_ABORT: crate::c_int = 1i32;
```

### `J1939_EE_INFO_RX_RTS`

```rust
const J1939_EE_INFO_RX_RTS: crate::c_int = 2i32;
```

### `J1939_EE_INFO_RX_DPO`

```rust
const J1939_EE_INFO_RX_DPO: crate::c_int = 3i32;
```

### `J1939_EE_INFO_RX_ABORT`

```rust
const J1939_EE_INFO_RX_ABORT: crate::c_int = 4i32;
```

### `J1939_FILTER_MAX`

```rust
const J1939_FILTER_MAX: crate::c_int = 512i32;
```

### `SOL_CAN_RAW`

```rust
const SOL_CAN_RAW: crate::c_int = 101i32;
```

### `CAN_RAW_FILTER_MAX`

```rust
const CAN_RAW_FILTER_MAX: crate::c_int = 512i32;
```

### `CAN_RAW_FILTER`

```rust
const CAN_RAW_FILTER: crate::c_int = 1i32;
```

### `CAN_RAW_ERR_FILTER`

```rust
const CAN_RAW_ERR_FILTER: crate::c_int = 2i32;
```

### `CAN_RAW_LOOPBACK`

```rust
const CAN_RAW_LOOPBACK: crate::c_int = 3i32;
```

### `CAN_RAW_RECV_OWN_MSGS`

```rust
const CAN_RAW_RECV_OWN_MSGS: crate::c_int = 4i32;
```

### `CAN_RAW_FD_FRAMES`

```rust
const CAN_RAW_FD_FRAMES: crate::c_int = 5i32;
```

### `CAN_RAW_JOIN_FILTERS`

```rust
const CAN_RAW_JOIN_FILTERS: crate::c_int = 6i32;
```

### `CAN_RAW_XL_FRAMES`

```rust
const CAN_RAW_XL_FRAMES: crate::c_int = 7i32;
```

### `CAN_EFF_FLAG`

```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

### `CAN_RTR_FLAG`

```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

### `CAN_ERR_FLAG`

```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

### `CAN_SFF_MASK`

```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

### `CAN_EFF_MASK`

```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

### `CAN_ERR_MASK`

```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

### `CANXL_PRIO_MASK`

```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

### `CAN_SFF_ID_BITS`

```rust
const CAN_SFF_ID_BITS: crate::c_int = 11i32;
```

### `CAN_EFF_ID_BITS`

```rust
const CAN_EFF_ID_BITS: crate::c_int = 29i32;
```

### `CANXL_PRIO_BITS`

```rust
const CANXL_PRIO_BITS: crate::c_int = 11i32;
```

### `CAN_MAX_DLC`

```rust
const CAN_MAX_DLC: crate::c_int = 8i32;
```

### `CAN_MAX_DLEN`

```rust
const CAN_MAX_DLEN: usize = 8usize;
```

### `CANFD_MAX_DLC`

```rust
const CANFD_MAX_DLC: crate::c_int = 15i32;
```

### `CANFD_MAX_DLEN`

```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

### `CANXL_MIN_DLC`

```rust
const CANXL_MIN_DLC: crate::c_int = 0i32;
```

### `CANXL_MAX_DLC`

```rust
const CANXL_MAX_DLC: crate::c_int = 2_047i32;
```

### `CANXL_MAX_DLC_MASK`

```rust
const CANXL_MAX_DLC_MASK: crate::c_int = 2_047i32;
```

### `CANXL_MIN_DLEN`

```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

### `CANXL_MAX_DLEN`

```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

### `CANFD_BRS`

```rust
const CANFD_BRS: crate::c_int = 1i32;
```

### `CANFD_ESI`

```rust
const CANFD_ESI: crate::c_int = 2i32;
```

### `CANFD_FDF`

```rust
const CANFD_FDF: crate::c_int = 4i32;
```

### `CANXL_XLF`

```rust
const CANXL_XLF: crate::c_int = 128i32;
```

### `CANXL_SEC`

```rust
const CANXL_SEC: crate::c_int = 1i32;
```

### `CAN_MTU`

```rust
const CAN_MTU: usize = 16usize;
```

### `CANFD_MTU`

```rust
const CANFD_MTU: usize = 72usize;
```

### `CANXL_MTU`

```rust
const CANXL_MTU: usize = 2_060usize;
```

### `CANXL_HDR_SIZE`

```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

### `CANXL_MIN_MTU`

```rust
const CANXL_MIN_MTU: usize = 76usize;
```

### `CANXL_MAX_MTU`

```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

### `CAN_RAW`

```rust
const CAN_RAW: crate::c_int = 1i32;
```

### `CAN_BCM`

```rust
const CAN_BCM: crate::c_int = 2i32;
```

### `CAN_TP16`

```rust
const CAN_TP16: crate::c_int = 3i32;
```

### `CAN_TP20`

```rust
const CAN_TP20: crate::c_int = 4i32;
```

### `CAN_MCNET`

```rust
const CAN_MCNET: crate::c_int = 5i32;
```

### `CAN_ISOTP`

```rust
const CAN_ISOTP: crate::c_int = 6i32;
```

### `CAN_J1939`

```rust
const CAN_J1939: crate::c_int = 7i32;
```

### `CAN_NPROTO`

```rust
const CAN_NPROTO: crate::c_int = 8i32;
```

### `SOL_CAN_BASE`

```rust
const SOL_CAN_BASE: crate::c_int = 100i32;
```

### `CAN_INV_FILTER`

```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

### `KEY_SPEC_THREAD_KEYRING`

```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

### `KEY_SPEC_PROCESS_KEYRING`

```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

### `KEY_SPEC_SESSION_KEYRING`

```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

### `KEY_SPEC_USER_KEYRING`

```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

### `KEY_SPEC_USER_SESSION_KEYRING`

```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

### `KEY_SPEC_GROUP_KEYRING`

```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

### `KEY_SPEC_REQKEY_AUTH_KEY`

```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

### `KEY_SPEC_REQUESTOR_KEYRING`

```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

### `KEY_REQKEY_DEFL_NO_CHANGE`

```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

### `KEY_REQKEY_DEFL_DEFAULT`

```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

### `KEY_REQKEY_DEFL_THREAD_KEYRING`

```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`

```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

### `KEY_REQKEY_DEFL_SESSION_KEYRING`

```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

### `KEY_REQKEY_DEFL_USER_KEYRING`

```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`

```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

### `KEY_REQKEY_DEFL_GROUP_KEYRING`

```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`

```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

### `KEYCTL_GET_KEYRING_ID`

```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

### `KEYCTL_JOIN_SESSION_KEYRING`

```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

### `KEYCTL_UPDATE`

```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

### `KEYCTL_REVOKE`

```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

### `KEYCTL_CHOWN`

```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

### `KEYCTL_SETPERM`

```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

### `KEYCTL_DESCRIBE`

```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

### `KEYCTL_CLEAR`

```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

### `KEYCTL_LINK`

```rust
const KEYCTL_LINK: u32 = 8u32;
```

### `KEYCTL_UNLINK`

```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

### `KEYCTL_SEARCH`

```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

### `KEYCTL_READ`

```rust
const KEYCTL_READ: u32 = 11u32;
```

### `KEYCTL_INSTANTIATE`

```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

### `KEYCTL_NEGATE`

```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

### `KEYCTL_SET_REQKEY_KEYRING`

```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

### `KEYCTL_SET_TIMEOUT`

```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

### `KEYCTL_ASSUME_AUTHORITY`

```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

### `KEYCTL_GET_SECURITY`

```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

### `KEYCTL_SESSION_TO_PARENT`

```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

### `KEYCTL_REJECT`

```rust
const KEYCTL_REJECT: u32 = 19u32;
```

### `KEYCTL_INSTANTIATE_IOV`

```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

### `KEYCTL_INVALIDATE`

```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

### `KEYCTL_GET_PERSISTENT`

```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

### `KEYCTL_DH_COMPUTE`

```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

### `KEYCTL_PKEY_QUERY`

```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

### `KEYCTL_PKEY_ENCRYPT`

```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

### `KEYCTL_PKEY_DECRYPT`

```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

### `KEYCTL_PKEY_SIGN`

```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

### `KEYCTL_PKEY_VERIFY`

```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

### `KEYCTL_RESTRICT_KEYRING`

```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

### `KEYCTL_MOVE`

```rust
const KEYCTL_MOVE: u32 = 30u32;
```

### `KEYCTL_CAPABILITIES`

```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

### `KEYCTL_SUPPORTS_ENCRYPT`

```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

### `KEYCTL_SUPPORTS_DECRYPT`

```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

### `KEYCTL_SUPPORTS_SIGN`

```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

### `KEYCTL_SUPPORTS_VERIFY`

```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

### `KEYCTL_CAPS0_CAPABILITIES`

```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`

```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`

```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

### `KEYCTL_CAPS0_PUBLIC_KEY`

```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

### `KEYCTL_CAPS0_BIG_KEY`

```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

### `KEYCTL_CAPS0_INVALIDATE`

```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

### `KEYCTL_CAPS0_RESTRICT_KEYRING`

```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

### `KEYCTL_CAPS0_MOVE`

```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

### `KEYCTL_CAPS1_NS_KEYRING_NAME`

```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

### `KEYCTL_CAPS1_NS_KEY_TAG`

```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

