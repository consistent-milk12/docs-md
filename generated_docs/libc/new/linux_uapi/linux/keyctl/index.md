*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [keyctl](index.md)*

---

# Module `keyctl`

Header: `linux/keyctl.h`

## Contents

- [Constants](#constants)
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

## Constants

### `KEY_SPEC_THREAD_KEYRING`
```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:3`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L3)*

### `KEY_SPEC_PROCESS_KEYRING`
```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:4`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L4)*

### `KEY_SPEC_SESSION_KEYRING`
```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:5`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L5)*

### `KEY_SPEC_USER_KEYRING`
```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:6`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L6)*

### `KEY_SPEC_USER_SESSION_KEYRING`
```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:7`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L7)*

### `KEY_SPEC_GROUP_KEYRING`
```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:8`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L8)*

### `KEY_SPEC_REQKEY_AUTH_KEY`
```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:9`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L9)*

### `KEY_SPEC_REQUESTOR_KEYRING`
```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:10`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L10)*

### `KEY_REQKEY_DEFL_NO_CHANGE`
```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:12`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L12)*

### `KEY_REQKEY_DEFL_DEFAULT`
```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:13`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L13)*

### `KEY_REQKEY_DEFL_THREAD_KEYRING`
```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:14`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L14)*

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`
```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:15`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L15)*

### `KEY_REQKEY_DEFL_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:16`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L16)*

### `KEY_REQKEY_DEFL_USER_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:17`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L17)*

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:18`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L18)*

### `KEY_REQKEY_DEFL_GROUP_KEYRING`
```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:19`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L19)*

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`
```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:20`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L20)*

### `KEYCTL_GET_KEYRING_ID`
```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:22`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L22)*

### `KEYCTL_JOIN_SESSION_KEYRING`
```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:23`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L23)*

### `KEYCTL_UPDATE`
```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:24`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L24)*

### `KEYCTL_REVOKE`
```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:25`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L25)*

### `KEYCTL_CHOWN`
```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:26`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L26)*

### `KEYCTL_SETPERM`
```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:27`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L27)*

### `KEYCTL_DESCRIBE`
```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:28`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L28)*

### `KEYCTL_CLEAR`
```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:29`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L29)*

### `KEYCTL_LINK`
```rust
const KEYCTL_LINK: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:30`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L30)*

### `KEYCTL_UNLINK`
```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:31`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L31)*

### `KEYCTL_SEARCH`
```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:32`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L32)*

### `KEYCTL_READ`
```rust
const KEYCTL_READ: u32 = 11u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:33`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L33)*

### `KEYCTL_INSTANTIATE`
```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:34`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L34)*

### `KEYCTL_NEGATE`
```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:35`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L35)*

### `KEYCTL_SET_REQKEY_KEYRING`
```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:36`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L36)*

### `KEYCTL_SET_TIMEOUT`
```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:37`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L37)*

### `KEYCTL_ASSUME_AUTHORITY`
```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:38`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L38)*

### `KEYCTL_GET_SECURITY`
```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:39`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L39)*

### `KEYCTL_SESSION_TO_PARENT`
```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:40`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L40)*

### `KEYCTL_REJECT`
```rust
const KEYCTL_REJECT: u32 = 19u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:41`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L41)*

### `KEYCTL_INSTANTIATE_IOV`
```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:42`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L42)*

### `KEYCTL_INVALIDATE`
```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:43`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L43)*

### `KEYCTL_GET_PERSISTENT`
```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:44`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L44)*

### `KEYCTL_DH_COMPUTE`
```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:45`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L45)*

### `KEYCTL_PKEY_QUERY`
```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:46`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L46)*

### `KEYCTL_PKEY_ENCRYPT`
```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:47`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L47)*

### `KEYCTL_PKEY_DECRYPT`
```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:48`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L48)*

### `KEYCTL_PKEY_SIGN`
```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:49`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L49)*

### `KEYCTL_PKEY_VERIFY`
```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:50`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L50)*

### `KEYCTL_RESTRICT_KEYRING`
```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:51`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L51)*

### `KEYCTL_MOVE`
```rust
const KEYCTL_MOVE: u32 = 30u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:52`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L52)*

### `KEYCTL_CAPABILITIES`
```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:53`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L53)*

### `KEYCTL_SUPPORTS_ENCRYPT`
```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:55`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L55)*

### `KEYCTL_SUPPORTS_DECRYPT`
```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:56`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L56)*

### `KEYCTL_SUPPORTS_SIGN`
```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:57`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L57)*

### `KEYCTL_SUPPORTS_VERIFY`
```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:58`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L58)*

### `KEYCTL_CAPS0_CAPABILITIES`
```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:60`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L60)*

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`
```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:61`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L61)*

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`
```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:62`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L62)*

### `KEYCTL_CAPS0_PUBLIC_KEY`
```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:63`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L63)*

### `KEYCTL_CAPS0_BIG_KEY`
```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:64`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L64)*

### `KEYCTL_CAPS0_INVALIDATE`
```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:65`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L65)*

### `KEYCTL_CAPS0_RESTRICT_KEYRING`
```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:66`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L66)*

### `KEYCTL_CAPS0_MOVE`
```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:67`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L67)*

### `KEYCTL_CAPS1_NS_KEYRING_NAME`
```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:68`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L68)*

### `KEYCTL_CAPS1_NS_KEY_TAG`
```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs:69`](../../../../../../.source_1765210505/libc-0.2.178/src/new/linux_uapi/linux/keyctl.rs#L69)*

