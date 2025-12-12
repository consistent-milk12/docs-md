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
  - [`linux_uapi`](#linux-uapi)
  - [`glibc`](#glibc)
  - [`bcm`](#bcm)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`bcm_timeval`](#bcm-timeval)
  - [`bcm_msg_head`](#bcm-msg-head)
  - [`j1939_filter`](#j1939-filter)
  - [`can_frame`](#can-frame)
  - [`canfd_frame`](#canfd-frame)
  - [`canxl_frame`](#canxl-frame)
  - [`sockaddr_can`](#sockaddr-can)
  - [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)
  - [`can_filter`](#can-filter)
  - [`rtentry`](#rtentry)
- [Type Aliases](#type-aliases)
  - [`pgn_t`](#pgn-t)
  - [`priority_t`](#priority-t)
  - [`name_t`](#name-t)
  - [`canid_t`](#canid-t)
  - [`can_err_mask_t`](#can-err-mask-t)
- [Constants](#constants)
  - [`TX_SETUP`](#tx-setup)
  - [`TX_DELETE`](#tx-delete)
  - [`TX_READ`](#tx-read)
  - [`TX_SEND`](#tx-send)
  - [`RX_SETUP`](#rx-setup)
  - [`RX_DELETE`](#rx-delete)
  - [`RX_READ`](#rx-read)
  - [`TX_STATUS`](#tx-status)
  - [`TX_EXPIRED`](#tx-expired)
  - [`RX_STATUS`](#rx-status)
  - [`RX_TIMEOUT`](#rx-timeout)
  - [`RX_CHANGED`](#rx-changed)
  - [`SETTIMER`](#settimer)
  - [`STARTTIMER`](#starttimer)
  - [`TX_COUNTEVT`](#tx-countevt)
  - [`TX_ANNOUNCE`](#tx-announce)
  - [`TX_CP_CAN_ID`](#tx-cp-can-id)
  - [`RX_FILTER_ID`](#rx-filter-id)
  - [`RX_CHECK_DLC`](#rx-check-dlc)
  - [`RX_NO_AUTOTIMER`](#rx-no-autotimer)
  - [`RX_ANNOUNCE_RESUME`](#rx-announce-resume)
  - [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx)
  - [`RX_RTR_FRAME`](#rx-rtr-frame)
  - [`CAN_FD_FRAME`](#can-fd-frame)
  - [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr)
  - [`J1939_IDLE_ADDR`](#j1939-idle-addr)
  - [`J1939_NO_ADDR`](#j1939-no-addr)
  - [`J1939_NO_NAME`](#j1939-no-name)
  - [`J1939_PGN_REQUEST`](#j1939-pgn-request)
  - [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed)
  - [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded)
  - [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max)
  - [`J1939_PGN_MAX`](#j1939-pgn-max)
  - [`J1939_NO_PGN`](#j1939-no-pgn)
  - [`SOL_CAN_J1939`](#sol-can-j1939)
  - [`SO_J1939_FILTER`](#so-j1939-filter)
  - [`SO_J1939_PROMISC`](#so-j1939-promisc)
  - [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio)
  - [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue)
  - [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr)
  - [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name)
  - [`SCM_J1939_PRIO`](#scm-j1939-prio)
  - [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue)
  - [`J1939_NLA_PAD`](#j1939-nla-pad)
  - [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked)
  - [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size)
  - [`J1939_NLA_PGN`](#j1939-nla-pgn)
  - [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name)
  - [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name)
  - [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr)
  - [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr)
  - [`J1939_EE_INFO_NONE`](#j1939-ee-info-none)
  - [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort)
  - [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts)
  - [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo)
  - [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort)
  - [`J1939_FILTER_MAX`](#j1939-filter-max)
  - [`SOL_CAN_RAW`](#sol-can-raw)
  - [`CAN_RAW_FILTER_MAX`](#can-raw-filter-max)
  - [`CAN_RAW_FILTER`](#can-raw-filter)
  - [`CAN_RAW_ERR_FILTER`](#can-raw-err-filter)
  - [`CAN_RAW_LOOPBACK`](#can-raw-loopback)
  - [`CAN_RAW_RECV_OWN_MSGS`](#can-raw-recv-own-msgs)
  - [`CAN_RAW_FD_FRAMES`](#can-raw-fd-frames)
  - [`CAN_RAW_JOIN_FILTERS`](#can-raw-join-filters)
  - [`CAN_RAW_XL_FRAMES`](#can-raw-xl-frames)
  - [`CAN_EFF_FLAG`](#can-eff-flag)
  - [`CAN_RTR_FLAG`](#can-rtr-flag)
  - [`CAN_ERR_FLAG`](#can-err-flag)
  - [`CAN_SFF_MASK`](#can-sff-mask)
  - [`CAN_EFF_MASK`](#can-eff-mask)
  - [`CAN_ERR_MASK`](#can-err-mask)
  - [`CANXL_PRIO_MASK`](#canxl-prio-mask)
  - [`CAN_SFF_ID_BITS`](#can-sff-id-bits)
  - [`CAN_EFF_ID_BITS`](#can-eff-id-bits)
  - [`CANXL_PRIO_BITS`](#canxl-prio-bits)
  - [`CAN_MAX_DLC`](#can-max-dlc)
  - [`CAN_MAX_DLEN`](#can-max-dlen)
  - [`CANFD_MAX_DLC`](#canfd-max-dlc)
  - [`CANFD_MAX_DLEN`](#canfd-max-dlen)
  - [`CANXL_MIN_DLC`](#canxl-min-dlc)
  - [`CANXL_MAX_DLC`](#canxl-max-dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask)
  - [`CANXL_MIN_DLEN`](#canxl-min-dlen)
  - [`CANXL_MAX_DLEN`](#canxl-max-dlen)
  - [`CANFD_BRS`](#canfd-brs)
  - [`CANFD_ESI`](#canfd-esi)
  - [`CANFD_FDF`](#canfd-fdf)
  - [`CANXL_XLF`](#canxl-xlf)
  - [`CANXL_SEC`](#canxl-sec)
  - [`CAN_MTU`](#can-mtu)
  - [`CANFD_MTU`](#canfd-mtu)
  - [`CANXL_MTU`](#canxl-mtu)
  - [`CANXL_HDR_SIZE`](#canxl-hdr-size)
  - [`CANXL_MIN_MTU`](#canxl-min-mtu)
  - [`CANXL_MAX_MTU`](#canxl-max-mtu)
  - [`CAN_RAW`](#can-raw)
  - [`CAN_BCM`](#can-bcm)
  - [`CAN_TP16`](#can-tp16)
  - [`CAN_TP20`](#can-tp20)
  - [`CAN_MCNET`](#can-mcnet)
  - [`CAN_ISOTP`](#can-isotp)
  - [`CAN_J1939`](#can-j1939)
  - [`CAN_NPROTO`](#can-nproto)
  - [`SOL_CAN_BASE`](#sol-can-base)
  - [`CAN_INV_FILTER`](#can-inv-filter)
  - [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring)
  - [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring)
  - [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring)
  - [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring)
  - [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring)
  - [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring)
  - [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key)
  - [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring)
  - [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change)
  - [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default)
  - [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring)
  - [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring)
  - [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring)
  - [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring)
  - [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring)
  - [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring)
  - [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring)
  - [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id)
  - [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring)
  - [`KEYCTL_UPDATE`](#keyctl-update)
  - [`KEYCTL_REVOKE`](#keyctl-revoke)
  - [`KEYCTL_CHOWN`](#keyctl-chown)
  - [`KEYCTL_SETPERM`](#keyctl-setperm)
  - [`KEYCTL_DESCRIBE`](#keyctl-describe)
  - [`KEYCTL_CLEAR`](#keyctl-clear)
  - [`KEYCTL_LINK`](#keyctl-link)
  - [`KEYCTL_UNLINK`](#keyctl-unlink)
  - [`KEYCTL_SEARCH`](#keyctl-search)
  - [`KEYCTL_READ`](#keyctl-read)
  - [`KEYCTL_INSTANTIATE`](#keyctl-instantiate)
  - [`KEYCTL_NEGATE`](#keyctl-negate)
  - [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring)
  - [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout)
  - [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority)
  - [`KEYCTL_GET_SECURITY`](#keyctl-get-security)
  - [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent)
  - [`KEYCTL_REJECT`](#keyctl-reject)
  - [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov)
  - [`KEYCTL_INVALIDATE`](#keyctl-invalidate)
  - [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent)
  - [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute)
  - [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query)
  - [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt)
  - [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt)
  - [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign)
  - [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify)
  - [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring)
  - [`KEYCTL_MOVE`](#keyctl-move)
  - [`KEYCTL_CAPABILITIES`](#keyctl-capabilities)
  - [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt)
  - [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt)
  - [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign)
  - [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify)
  - [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities)
  - [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings)
  - [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman)
  - [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key)
  - [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key)
  - [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate)
  - [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring)
  - [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move)
  - [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name)
  - [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod | Interfaces that are common across multiple platforms |
| [`linux_uapi`](#linux-uapi) | mod | This directory maps to `include/uapi` in the Linux source tree. |
| [`glibc`](#glibc) | mod | GNU libc. |
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`bcm_timeval`](#bcm-timeval) | struct |  |
| [`bcm_msg_head`](#bcm-msg-head) | struct |  |
| [`j1939_filter`](#j1939-filter) | struct |  |
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`rtentry`](#rtentry) | struct |  |
| [`pgn_t`](#pgn-t) | type |  |
| [`priority_t`](#priority-t) | type |  |
| [`name_t`](#name-t) | type |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`TX_SETUP`](#tx-setup) | const |  |
| [`TX_DELETE`](#tx-delete) | const |  |
| [`TX_READ`](#tx-read) | const |  |
| [`TX_SEND`](#tx-send) | const |  |
| [`RX_SETUP`](#rx-setup) | const |  |
| [`RX_DELETE`](#rx-delete) | const |  |
| [`RX_READ`](#rx-read) | const |  |
| [`TX_STATUS`](#tx-status) | const |  |
| [`TX_EXPIRED`](#tx-expired) | const |  |
| [`RX_STATUS`](#rx-status) | const |  |
| [`RX_TIMEOUT`](#rx-timeout) | const |  |
| [`RX_CHANGED`](#rx-changed) | const |  |
| [`SETTIMER`](#settimer) | const |  |
| [`STARTTIMER`](#starttimer) | const |  |
| [`TX_COUNTEVT`](#tx-countevt) | const |  |
| [`TX_ANNOUNCE`](#tx-announce) | const |  |
| [`TX_CP_CAN_ID`](#tx-cp-can-id) | const |  |
| [`RX_FILTER_ID`](#rx-filter-id) | const |  |
| [`RX_CHECK_DLC`](#rx-check-dlc) | const |  |
| [`RX_NO_AUTOTIMER`](#rx-no-autotimer) | const |  |
| [`RX_ANNOUNCE_RESUME`](#rx-announce-resume) | const |  |
| [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx) | const |  |
| [`RX_RTR_FRAME`](#rx-rtr-frame) | const |  |
| [`CAN_FD_FRAME`](#can-fd-frame) | const |  |
| [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr) | const |  |
| [`J1939_IDLE_ADDR`](#j1939-idle-addr) | const |  |
| [`J1939_NO_ADDR`](#j1939-no-addr) | const |  |
| [`J1939_NO_NAME`](#j1939-no-name) | const |  |
| [`J1939_PGN_REQUEST`](#j1939-pgn-request) | const |  |
| [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed) | const |  |
| [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded) | const |  |
| [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max) | const |  |
| [`J1939_PGN_MAX`](#j1939-pgn-max) | const |  |
| [`J1939_NO_PGN`](#j1939-no-pgn) | const |  |
| [`SOL_CAN_J1939`](#sol-can-j1939) | const |  |
| [`SO_J1939_FILTER`](#so-j1939-filter) | const |  |
| [`SO_J1939_PROMISC`](#so-j1939-promisc) | const |  |
| [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio) | const |  |
| [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue) | const |  |
| [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr) | const |  |
| [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name) | const |  |
| [`SCM_J1939_PRIO`](#scm-j1939-prio) | const |  |
| [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue) | const |  |
| [`J1939_NLA_PAD`](#j1939-nla-pad) | const |  |
| [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked) | const |  |
| [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size) | const |  |
| [`J1939_NLA_PGN`](#j1939-nla-pgn) | const |  |
| [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name) | const |  |
| [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name) | const |  |
| [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr) | const |  |
| [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr) | const |  |
| [`J1939_EE_INFO_NONE`](#j1939-ee-info-none) | const |  |
| [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort) | const |  |
| [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts) | const |  |
| [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo) | const |  |
| [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort) | const |  |
| [`J1939_FILTER_MAX`](#j1939-filter-max) | const |  |
| [`SOL_CAN_RAW`](#sol-can-raw) | const |  |
| [`CAN_RAW_FILTER_MAX`](#can-raw-filter-max) | const |  |
| [`CAN_RAW_FILTER`](#can-raw-filter) | const |  |
| [`CAN_RAW_ERR_FILTER`](#can-raw-err-filter) | const |  |
| [`CAN_RAW_LOOPBACK`](#can-raw-loopback) | const |  |
| [`CAN_RAW_RECV_OWN_MSGS`](#can-raw-recv-own-msgs) | const |  |
| [`CAN_RAW_FD_FRAMES`](#can-raw-fd-frames) | const |  |
| [`CAN_RAW_JOIN_FILTERS`](#can-raw-join-filters) | const |  |
| [`CAN_RAW_XL_FRAMES`](#can-raw-xl-frames) | const |  |
| [`CAN_EFF_FLAG`](#can-eff-flag) | const |  |
| [`CAN_RTR_FLAG`](#can-rtr-flag) | const |  |
| [`CAN_ERR_FLAG`](#can-err-flag) | const |  |
| [`CAN_SFF_MASK`](#can-sff-mask) | const |  |
| [`CAN_EFF_MASK`](#can-eff-mask) | const |  |
| [`CAN_ERR_MASK`](#can-err-mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl-prio-mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can-sff-id-bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can-eff-id-bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl-prio-bits) | const |  |
| [`CAN_MAX_DLC`](#can-max-dlc) | const |  |
| [`CAN_MAX_DLEN`](#can-max-dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd-max-dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd-max-dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl-min-dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl-max-dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl-min-dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl-max-dlen) | const |  |
| [`CANFD_BRS`](#canfd-brs) | const |  |
| [`CANFD_ESI`](#canfd-esi) | const |  |
| [`CANFD_FDF`](#canfd-fdf) | const |  |
| [`CANXL_XLF`](#canxl-xlf) | const |  |
| [`CANXL_SEC`](#canxl-sec) | const |  |
| [`CAN_MTU`](#can-mtu) | const |  |
| [`CANFD_MTU`](#canfd-mtu) | const |  |
| [`CANXL_MTU`](#canxl-mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl-hdr-size) | const |  |
| [`CANXL_MIN_MTU`](#canxl-min-mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl-max-mtu) | const |  |
| [`CAN_RAW`](#can-raw) | const |  |
| [`CAN_BCM`](#can-bcm) | const |  |
| [`CAN_TP16`](#can-tp16) | const |  |
| [`CAN_TP20`](#can-tp20) | const |  |
| [`CAN_MCNET`](#can-mcnet) | const |  |
| [`CAN_ISOTP`](#can-isotp) | const |  |
| [`CAN_J1939`](#can-j1939) | const |  |
| [`CAN_NPROTO`](#can-nproto) | const |  |
| [`SOL_CAN_BASE`](#sol-can-base) | const |  |
| [`CAN_INV_FILTER`](#can-inv-filter) | const |  |
| [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring) | const |  |
| [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring) | const |  |
| [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring) | const |  |
| [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring) | const |  |
| [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring) | const |  |
| [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring) | const |  |
| [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key) | const |  |
| [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring) | const |  |
| [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change) | const |  |
| [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default) | const |  |
| [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring) | const |  |
| [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring) | const |  |
| [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring) | const |  |
| [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring) | const |  |
| [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id) | const |  |
| [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring) | const |  |
| [`KEYCTL_UPDATE`](#keyctl-update) | const |  |
| [`KEYCTL_REVOKE`](#keyctl-revoke) | const |  |
| [`KEYCTL_CHOWN`](#keyctl-chown) | const |  |
| [`KEYCTL_SETPERM`](#keyctl-setperm) | const |  |
| [`KEYCTL_DESCRIBE`](#keyctl-describe) | const |  |
| [`KEYCTL_CLEAR`](#keyctl-clear) | const |  |
| [`KEYCTL_LINK`](#keyctl-link) | const |  |
| [`KEYCTL_UNLINK`](#keyctl-unlink) | const |  |
| [`KEYCTL_SEARCH`](#keyctl-search) | const |  |
| [`KEYCTL_READ`](#keyctl-read) | const |  |
| [`KEYCTL_INSTANTIATE`](#keyctl-instantiate) | const |  |
| [`KEYCTL_NEGATE`](#keyctl-negate) | const |  |
| [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring) | const |  |
| [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout) | const |  |
| [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority) | const |  |
| [`KEYCTL_GET_SECURITY`](#keyctl-get-security) | const |  |
| [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent) | const |  |
| [`KEYCTL_REJECT`](#keyctl-reject) | const |  |
| [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov) | const |  |
| [`KEYCTL_INVALIDATE`](#keyctl-invalidate) | const |  |
| [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent) | const |  |
| [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute) | const |  |
| [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query) | const |  |
| [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt) | const |  |
| [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt) | const |  |
| [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign) | const |  |
| [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify) | const |  |
| [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring) | const |  |
| [`KEYCTL_MOVE`](#keyctl-move) | const |  |
| [`KEYCTL_CAPABILITIES`](#keyctl-capabilities) | const |  |
| [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt) | const |  |
| [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt) | const |  |
| [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign) | const |  |
| [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify) | const |  |
| [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities) | const |  |
| [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings) | const |  |
| [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman) | const |  |
| [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key) | const |  |
| [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key) | const |  |
| [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate) | const |  |
| [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring) | const |  |
| [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move) | const |  |
| [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name) | const |  |
| [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag) | const |  |

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:5-21`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L5-L21)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:5-21`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L5-L21)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:49-58`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L49-L58)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:38-49`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L38-L49)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:55-65`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L55-L65)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:70-79`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L70-L79)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:102-113`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L102-L113)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:115-131`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L115-L131)*

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

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/unix/linux/net/route.rs:8-30`](../../../.source_1765521767/libc-0.2.178/src/new/glibc/sysdeps/unix/linux/net/route.rs#L8-L30)*

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

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:16`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L16)*

### `priority_t`

```rust
type priority_t = u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:17`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L17)*

### `name_t`

```rust
type name_t = u64;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:18`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L18)*

### `canid_t`

```rust
type canid_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:18`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L18)*

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:24`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L24)*

## Constants

### `TX_SETUP`
```rust
const TX_SETUP: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_DELETE`
```rust
const TX_DELETE: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_READ`
```rust
const TX_READ: u32 = 3u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_SEND`
```rust
const TX_SEND: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_SETUP`
```rust
const RX_SETUP: u32 = 5u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_DELETE`
```rust
const RX_DELETE: u32 = 6u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_READ`
```rust
const RX_READ: u32 = 7u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_STATUS`
```rust
const TX_STATUS: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `TX_EXPIRED`
```rust
const TX_EXPIRED: u32 = 9u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_STATUS`
```rust
const RX_STATUS: u32 = 10u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_TIMEOUT`
```rust
const RX_TIMEOUT: u32 = 11u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `RX_CHANGED`
```rust
const RX_CHANGED: u32 = 12u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:23-39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L23-L39)*

### `SETTIMER`
```rust
const SETTIMER: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:41`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L41)*

### `STARTTIMER`
```rust
const STARTTIMER: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:42`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L42)*

### `TX_COUNTEVT`
```rust
const TX_COUNTEVT: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:43`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L43)*

### `TX_ANNOUNCE`
```rust
const TX_ANNOUNCE: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:44`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L44)*

### `TX_CP_CAN_ID`
```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:45`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L45)*

### `RX_FILTER_ID`
```rust
const RX_FILTER_ID: u32 = 32u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:46`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L46)*

### `RX_CHECK_DLC`
```rust
const RX_CHECK_DLC: u32 = 64u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:47`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L47)*

### `RX_NO_AUTOTIMER`
```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:48`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L48)*

### `RX_ANNOUNCE_RESUME`
```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:49`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L49)*

### `TX_RESET_MULTI_IDX`
```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:50`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L50)*

### `RX_RTR_FRAME`
```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:51`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L51)*

### `CAN_FD_FRAME`
```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs:52`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/bcm.rs#L52)*

### `J1939_MAX_UNICAST_ADDR`
```rust
const J1939_MAX_UNICAST_ADDR: crate::c_uchar = 253u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:5`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L5)*

### `J1939_IDLE_ADDR`
```rust
const J1939_IDLE_ADDR: crate::c_uchar = 254u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:6`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L6)*

### `J1939_NO_ADDR`
```rust
const J1939_NO_ADDR: crate::c_uchar = 255u8;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:7`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L7)*

### `J1939_NO_NAME`
```rust
const J1939_NO_NAME: crate::c_ulong = 0u64;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:8`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L8)*

### `J1939_PGN_REQUEST`
```rust
const J1939_PGN_REQUEST: crate::c_uint = 59_904u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:9`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L9)*

### `J1939_PGN_ADDRESS_CLAIMED`
```rust
const J1939_PGN_ADDRESS_CLAIMED: crate::c_uint = 60_928u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:10`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L10)*

### `J1939_PGN_ADDRESS_COMMANDED`
```rust
const J1939_PGN_ADDRESS_COMMANDED: crate::c_uint = 65_240u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:11`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L11)*

### `J1939_PGN_PDU1_MAX`
```rust
const J1939_PGN_PDU1_MAX: crate::c_uint = 261_888u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:12`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L12)*

### `J1939_PGN_MAX`
```rust
const J1939_PGN_MAX: crate::c_uint = 262_143u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:13`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L13)*

### `J1939_NO_PGN`
```rust
const J1939_NO_PGN: crate::c_uint = 262_144u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:14`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L14)*

### `SOL_CAN_J1939`
```rust
const SOL_CAN_J1939: crate::c_int = 107i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:20`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L20)*

### `SO_J1939_FILTER`
```rust
const SO_J1939_FILTER: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:24`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L24)*

### `SO_J1939_PROMISC`
```rust
const SO_J1939_PROMISC: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:25`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L25)*

### `SO_J1939_SEND_PRIO`
```rust
const SO_J1939_SEND_PRIO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:26`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L26)*

### `SO_J1939_ERRQUEUE`
```rust
const SO_J1939_ERRQUEUE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:27`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L27)*

### `SCM_J1939_DEST_ADDR`
```rust
const SCM_J1939_DEST_ADDR: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:29`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L29)*

### `SCM_J1939_DEST_NAME`
```rust
const SCM_J1939_DEST_NAME: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:30`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L30)*

### `SCM_J1939_PRIO`
```rust
const SCM_J1939_PRIO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:31`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L31)*

### `SCM_J1939_ERRQUEUE`
```rust
const SCM_J1939_ERRQUEUE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:32`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L32)*

### `J1939_NLA_PAD`
```rust
const J1939_NLA_PAD: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:34`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L34)*

### `J1939_NLA_BYTES_ACKED`
```rust
const J1939_NLA_BYTES_ACKED: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:35`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L35)*

### `J1939_NLA_TOTAL_SIZE`
```rust
const J1939_NLA_TOTAL_SIZE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:36`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L36)*

### `J1939_NLA_PGN`
```rust
const J1939_NLA_PGN: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:37`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L37)*

### `J1939_NLA_SRC_NAME`
```rust
const J1939_NLA_SRC_NAME: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:38`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L38)*

### `J1939_NLA_DEST_NAME`
```rust
const J1939_NLA_DEST_NAME: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L39)*

### `J1939_NLA_SRC_ADDR`
```rust
const J1939_NLA_SRC_ADDR: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:40`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L40)*

### `J1939_NLA_DEST_ADDR`
```rust
const J1939_NLA_DEST_ADDR: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:41`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L41)*

### `J1939_EE_INFO_NONE`
```rust
const J1939_EE_INFO_NONE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:43`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L43)*

### `J1939_EE_INFO_TX_ABORT`
```rust
const J1939_EE_INFO_TX_ABORT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:44`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L44)*

### `J1939_EE_INFO_RX_RTS`
```rust
const J1939_EE_INFO_RX_RTS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:45`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L45)*

### `J1939_EE_INFO_RX_DPO`
```rust
const J1939_EE_INFO_RX_DPO: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:46`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L46)*

### `J1939_EE_INFO_RX_ABORT`
```rust
const J1939_EE_INFO_RX_ABORT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:47`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L47)*

### `J1939_FILTER_MAX`
```rust
const J1939_FILTER_MAX: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs:60`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/j1939.rs#L60)*

### `SOL_CAN_RAW`
```rust
const SOL_CAN_RAW: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:5`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L5)*

### `CAN_RAW_FILTER_MAX`
```rust
const CAN_RAW_FILTER_MAX: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:6`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L6)*

### `CAN_RAW_FILTER`
```rust
const CAN_RAW_FILTER: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:9`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L9)*

### `CAN_RAW_ERR_FILTER`
```rust
const CAN_RAW_ERR_FILTER: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:10`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L10)*

### `CAN_RAW_LOOPBACK`
```rust
const CAN_RAW_LOOPBACK: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:11`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L11)*

### `CAN_RAW_RECV_OWN_MSGS`
```rust
const CAN_RAW_RECV_OWN_MSGS: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:12`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L12)*

### `CAN_RAW_FD_FRAMES`
```rust
const CAN_RAW_FD_FRAMES: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:13`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L13)*

### `CAN_RAW_JOIN_FILTERS`
```rust
const CAN_RAW_JOIN_FILTERS: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:14`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L14)*

### `CAN_RAW_XL_FRAMES`
```rust
const CAN_RAW_XL_FRAMES: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs:15`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can/raw.rs#L15)*

### `CAN_EFF_FLAG`
```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:9`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L9)*

### `CAN_RTR_FLAG`
```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:10`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L10)*

### `CAN_ERR_FLAG`
```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:11`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L11)*

### `CAN_SFF_MASK`
```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:13`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L13)*

### `CAN_EFF_MASK`
```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:14`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L14)*

### `CAN_ERR_MASK`
```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:15`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L15)*

### `CANXL_PRIO_MASK`
```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:16`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L16)*

### `CAN_SFF_ID_BITS`
```rust
const CAN_SFF_ID_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:20`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L20)*

### `CAN_EFF_ID_BITS`
```rust
const CAN_EFF_ID_BITS: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:21`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L21)*

### `CANXL_PRIO_BITS`
```rust
const CANXL_PRIO_BITS: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:22`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L22)*

### `CAN_MAX_DLC`
```rust
const CAN_MAX_DLC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:26`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L26)*

### `CAN_MAX_DLEN`
```rust
const CAN_MAX_DLEN: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:27`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L27)*

### `CANFD_MAX_DLC`
```rust
const CANFD_MAX_DLC: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:29`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L29)*

### `CANFD_MAX_DLEN`
```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:30`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L30)*

### `CANXL_MIN_DLC`
```rust
const CANXL_MIN_DLC: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:32`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L32)*

### `CANXL_MAX_DLC`
```rust
const CANXL_MAX_DLC: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:33`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L33)*

### `CANXL_MAX_DLC_MASK`
```rust
const CANXL_MAX_DLC_MASK: crate::c_int = 2_047i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:34`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L34)*

### `CANXL_MIN_DLEN`
```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:35`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L35)*

### `CANXL_MAX_DLEN`
```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:36`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L36)*

### `CANFD_BRS`
```rust
const CANFD_BRS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:51`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L51)*

### `CANFD_ESI`
```rust
const CANFD_ESI: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:52`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L52)*

### `CANFD_FDF`
```rust
const CANFD_FDF: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:53`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L53)*

### `CANXL_XLF`
```rust
const CANXL_XLF: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:67`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L67)*

### `CANXL_SEC`
```rust
const CANXL_SEC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:68`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L68)*

### `CAN_MTU`
```rust
const CAN_MTU: usize = 16usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:81`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L81)*

### `CANFD_MTU`
```rust
const CANFD_MTU: usize = 72usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:82`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L82)*

### `CANXL_MTU`
```rust
const CANXL_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:83`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L83)*

### `CANXL_HDR_SIZE`
```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:87`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L87)*

### `CANXL_MIN_MTU`
```rust
const CANXL_MIN_MTU: usize = 76usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:88`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L88)*

### `CANXL_MAX_MTU`
```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:89`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L89)*

### `CAN_RAW`
```rust
const CAN_RAW: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:91`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L91)*

### `CAN_BCM`
```rust
const CAN_BCM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:92`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L92)*

### `CAN_TP16`
```rust
const CAN_TP16: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:93`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L93)*

### `CAN_TP20`
```rust
const CAN_TP20: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:94`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L94)*

### `CAN_MCNET`
```rust
const CAN_MCNET: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:95`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L95)*

### `CAN_ISOTP`
```rust
const CAN_ISOTP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:96`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L96)*

### `CAN_J1939`
```rust
const CAN_J1939: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:97`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L97)*

### `CAN_NPROTO`
```rust
const CAN_NPROTO: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:98`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L98)*

### `SOL_CAN_BASE`
```rust
const SOL_CAN_BASE: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:100`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L100)*

### `CAN_INV_FILTER`
```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/can.rs:133`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/can.rs#L133)*

### `KEY_SPEC_THREAD_KEYRING`
```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:3`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L3)*

### `KEY_SPEC_PROCESS_KEYRING`
```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:4`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L4)*

### `KEY_SPEC_SESSION_KEYRING`
```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:5`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L5)*

### `KEY_SPEC_USER_KEYRING`
```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:6`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L6)*

### `KEY_SPEC_USER_SESSION_KEYRING`
```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:7`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L7)*

### `KEY_SPEC_GROUP_KEYRING`
```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:8`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L8)*

### `KEY_SPEC_REQKEY_AUTH_KEY`
```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:9`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L9)*

### `KEY_SPEC_REQUESTOR_KEYRING`
```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:10`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L10)*

### `KEY_REQKEY_DEFL_NO_CHANGE`
```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:12`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L12)*

### `KEY_REQKEY_DEFL_DEFAULT`
```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:13`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L13)*

### `KEY_REQKEY_DEFL_THREAD_KEYRING`
```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:14`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L14)*

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`
```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:15`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L15)*

### `KEY_REQKEY_DEFL_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:16`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L16)*

### `KEY_REQKEY_DEFL_USER_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:17`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L17)*

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:18`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L18)*

### `KEY_REQKEY_DEFL_GROUP_KEYRING`
```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:19`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L19)*

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`
```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:20`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L20)*

### `KEYCTL_GET_KEYRING_ID`
```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:22`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L22)*

### `KEYCTL_JOIN_SESSION_KEYRING`
```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:23`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L23)*

### `KEYCTL_UPDATE`
```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:24`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L24)*

### `KEYCTL_REVOKE`
```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:25`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L25)*

### `KEYCTL_CHOWN`
```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:26`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L26)*

### `KEYCTL_SETPERM`
```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:27`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L27)*

### `KEYCTL_DESCRIBE`
```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:28`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L28)*

### `KEYCTL_CLEAR`
```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:29`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L29)*

### `KEYCTL_LINK`
```rust
const KEYCTL_LINK: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:30`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L30)*

### `KEYCTL_UNLINK`
```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:31`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L31)*

### `KEYCTL_SEARCH`
```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:32`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L32)*

### `KEYCTL_READ`
```rust
const KEYCTL_READ: u32 = 11u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:33`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L33)*

### `KEYCTL_INSTANTIATE`
```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:34`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L34)*

### `KEYCTL_NEGATE`
```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:35`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L35)*

### `KEYCTL_SET_REQKEY_KEYRING`
```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:36`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L36)*

### `KEYCTL_SET_TIMEOUT`
```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:37`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L37)*

### `KEYCTL_ASSUME_AUTHORITY`
```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:38`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L38)*

### `KEYCTL_GET_SECURITY`
```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:39`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L39)*

### `KEYCTL_SESSION_TO_PARENT`
```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:40`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L40)*

### `KEYCTL_REJECT`
```rust
const KEYCTL_REJECT: u32 = 19u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:41`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L41)*

### `KEYCTL_INSTANTIATE_IOV`
```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:42`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L42)*

### `KEYCTL_INVALIDATE`
```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:43`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L43)*

### `KEYCTL_GET_PERSISTENT`
```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:44`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L44)*

### `KEYCTL_DH_COMPUTE`
```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:45`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L45)*

### `KEYCTL_PKEY_QUERY`
```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:46`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L46)*

### `KEYCTL_PKEY_ENCRYPT`
```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:47`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L47)*

### `KEYCTL_PKEY_DECRYPT`
```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:48`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L48)*

### `KEYCTL_PKEY_SIGN`
```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:49`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L49)*

### `KEYCTL_PKEY_VERIFY`
```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:50`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L50)*

### `KEYCTL_RESTRICT_KEYRING`
```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:51`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L51)*

### `KEYCTL_MOVE`
```rust
const KEYCTL_MOVE: u32 = 30u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:52`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L52)*

### `KEYCTL_CAPABILITIES`
```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:53`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L53)*

### `KEYCTL_SUPPORTS_ENCRYPT`
```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:55`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L55)*

### `KEYCTL_SUPPORTS_DECRYPT`
```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:56`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L56)*

### `KEYCTL_SUPPORTS_SIGN`
```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:57`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L57)*

### `KEYCTL_SUPPORTS_VERIFY`
```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:58`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L58)*

### `KEYCTL_CAPS0_CAPABILITIES`
```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:60`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L60)*

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`
```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:61`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L61)*

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`
```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:62`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L62)*

### `KEYCTL_CAPS0_PUBLIC_KEY`
```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:63`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L63)*

### `KEYCTL_CAPS0_BIG_KEY`
```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:64`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L64)*

### `KEYCTL_CAPS0_INVALIDATE`
```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:65`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L65)*

### `KEYCTL_CAPS0_RESTRICT_KEYRING`
```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:66`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L66)*

### `KEYCTL_CAPS0_MOVE`
```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:67`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L67)*

### `KEYCTL_CAPS1_NS_KEYRING_NAME`
```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:68`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L68)*

### `KEYCTL_CAPS1_NS_KEY_TAG`
```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:69`](../../../.source_1765521767/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L69)*

