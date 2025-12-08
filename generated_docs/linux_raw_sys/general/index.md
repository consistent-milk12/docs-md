*[linux_raw_sys](../index.md) / [general](index.md)*

---

# Module `general`

## Contents

- [Structs](#structs)
  - [`__BindgenBitfieldUnit`](#__bindgenbitfieldunit)
  - [`__IncompleteArrayField`](#__incompletearrayfield)
  - [`__kernel_fd_set`](#__kernel_fd_set)
  - [`__kernel_fsid_t`](#__kernel_fsid_t)
  - [`__user_cap_header_struct`](#__user_cap_header_struct)
  - [`__user_cap_data_struct`](#__user_cap_data_struct)
  - [`vfs_cap_data`](#vfs_cap_data)
  - [`vfs_cap_data__bindgen_ty_1`](#vfs_cap_data__bindgen_ty_1)
  - [`vfs_ns_cap_data`](#vfs_ns_cap_data)
  - [`vfs_ns_cap_data__bindgen_ty_1`](#vfs_ns_cap_data__bindgen_ty_1)
  - [`f_owner_ex`](#f_owner_ex)
  - [`flock`](#flock)
  - [`flock64`](#flock64)
  - [`open_how`](#open_how)
  - [`epoll_event`](#epoll_event)
  - [`epoll_params`](#epoll_params)
  - [`fscrypt_policy_v1`](#fscrypt_policy_v1)
  - [`fscrypt_key`](#fscrypt_key)
  - [`fscrypt_policy_v2`](#fscrypt_policy_v2)
  - [`fscrypt_get_policy_ex_arg`](#fscrypt_get_policy_ex_arg)
  - [`fscrypt_key_specifier`](#fscrypt_key_specifier)
  - [`fscrypt_provisioning_key_payload`](#fscrypt_provisioning_key_payload)
  - [`fscrypt_add_key_arg`](#fscrypt_add_key_arg)
  - [`fscrypt_remove_key_arg`](#fscrypt_remove_key_arg)
  - [`fscrypt_get_key_status_arg`](#fscrypt_get_key_status_arg)
  - [`mount_attr`](#mount_attr)
  - [`statmount`](#statmount)
  - [`mnt_id_req`](#mnt_id_req)
  - [`file_clone_range`](#file_clone_range)
  - [`fstrim_range`](#fstrim_range)
  - [`fsuuid2`](#fsuuid2)
  - [`fs_sysfs_path`](#fs_sysfs_path)
  - [`file_dedupe_range_info`](#file_dedupe_range_info)
  - [`file_dedupe_range`](#file_dedupe_range)
  - [`files_stat_struct`](#files_stat_struct)
  - [`inodes_stat_t`](#inodes_stat_t)
  - [`fsxattr`](#fsxattr)
  - [`page_region`](#page_region)
  - [`pm_scan_arg`](#pm_scan_arg)
  - [`procmap_query`](#procmap_query)
  - [`futex_waitv`](#futex_waitv)
  - [`robust_list`](#robust_list)
  - [`robust_list_head`](#robust_list_head)
  - [`inotify_event`](#inotify_event)
  - [`cachestat_range`](#cachestat_range)
  - [`cachestat`](#cachestat)
  - [`pollfd`](#pollfd)
  - [`rand_pool_info`](#rand_pool_info)
  - [`vgetrandom_opaque_params`](#vgetrandom_opaque_params)
  - [`__kernel_timespec`](#__kernel_timespec)
  - [`__kernel_itimerspec`](#__kernel_itimerspec)
  - [`__kernel_old_timeval`](#__kernel_old_timeval)
  - [`__kernel_old_timespec`](#__kernel_old_timespec)
  - [`__kernel_old_itimerval`](#__kernel_old_itimerval)
  - [`__kernel_sock_timeval`](#__kernel_sock_timeval)
  - [`rusage`](#rusage)
  - [`rlimit`](#rlimit)
  - [`rlimit64`](#rlimit64)
  - [`clone_args`](#clone_args)
  - [`sigaction`](#sigaction)
  - [`sigaltstack`](#sigaltstack)
  - [`__sifields__bindgen_ty_1`](#__sifields__bindgen_ty_1)
  - [`__sifields__bindgen_ty_2`](#__sifields__bindgen_ty_2)
  - [`__sifields__bindgen_ty_3`](#__sifields__bindgen_ty_3)
  - [`__sifields__bindgen_ty_4`](#__sifields__bindgen_ty_4)
  - [`__sifields__bindgen_ty_5`](#__sifields__bindgen_ty_5)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3)
  - [`__sifields__bindgen_ty_6`](#__sifields__bindgen_ty_6)
  - [`__sifields__bindgen_ty_7`](#__sifields__bindgen_ty_7)
  - [`siginfo`](#siginfo)
  - [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo__bindgen_ty_1__bindgen_ty_1)
  - [`sigevent`](#sigevent)
  - [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent__bindgen_ty_1__bindgen_ty_1)
  - [`statx_timestamp`](#statx_timestamp)
  - [`statx`](#statx)
  - [`termios`](#termios)
  - [`termios2`](#termios2)
  - [`ktermios`](#ktermios)
  - [`winsize`](#winsize)
  - [`termio`](#termio)
  - [`timespec`](#timespec)
  - [`timeval`](#timeval)
  - [`itimerspec`](#itimerspec)
  - [`itimerval`](#itimerval)
  - [`timezone`](#timezone)
  - [`iovec`](#iovec)
  - [`dmabuf_cmsg`](#dmabuf_cmsg)
  - [`dmabuf_token`](#dmabuf_token)
  - [`xattr_args`](#xattr_args)
  - [`uffd_msg`](#uffd_msg)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd_msg__bindgen_ty_1__bindgen_ty_1)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd_msg__bindgen_ty_1__bindgen_ty_2)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd_msg__bindgen_ty_1__bindgen_ty_3)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd_msg__bindgen_ty_1__bindgen_ty_4)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd_msg__bindgen_ty_1__bindgen_ty_5)
  - [`uffdio_api`](#uffdio_api)
  - [`uffdio_range`](#uffdio_range)
  - [`uffdio_register`](#uffdio_register)
  - [`uffdio_copy`](#uffdio_copy)
  - [`uffdio_zeropage`](#uffdio_zeropage)
  - [`uffdio_writeprotect`](#uffdio_writeprotect)
  - [`uffdio_continue`](#uffdio_continue)
  - [`uffdio_poison`](#uffdio_poison)
  - [`uffdio_move`](#uffdio_move)
  - [`linux_dirent64`](#linux_dirent64)
  - [`stat`](#stat)
  - [`__old_kernel_stat`](#__old_kernel_stat)
  - [`statfs`](#statfs)
  - [`statfs64`](#statfs64)
  - [`compat_statfs64`](#compat_statfs64)
  - [`user_desc`](#user_desc)
  - [`kernel_sigset_t`](#kernel_sigset_t)
  - [`kernel_sigaction`](#kernel_sigaction)
- [Enums](#enums)
  - [`fsconfig_command`](#fsconfig_command)
  - [`procmap_query_flags`](#procmap_query_flags)
  - [`membarrier_cmd`](#membarrier_cmd)
  - [`membarrier_cmd_flag`](#membarrier_cmd_flag)
- [Type Aliases](#type-aliases)
  - [`__s8`](#__s8)
  - [`__u8`](#__u8)
  - [`__s16`](#__s16)
  - [`__u16`](#__u16)
  - [`__s32`](#__s32)
  - [`__u32`](#__u32)
  - [`__s64`](#__s64)
  - [`__u64`](#__u64)
  - [`__kernel_sighandler_t`](#__kernel_sighandler_t)
  - [`__kernel_key_t`](#__kernel_key_t)
  - [`__kernel_mqd_t`](#__kernel_mqd_t)
  - [`__kernel_old_uid_t`](#__kernel_old_uid_t)
  - [`__kernel_old_gid_t`](#__kernel_old_gid_t)
  - [`__kernel_old_dev_t`](#__kernel_old_dev_t)
  - [`__kernel_long_t`](#__kernel_long_t)
  - [`__kernel_ulong_t`](#__kernel_ulong_t)
  - [`__kernel_ino_t`](#__kernel_ino_t)
  - [`__kernel_mode_t`](#__kernel_mode_t)
  - [`__kernel_pid_t`](#__kernel_pid_t)
  - [`__kernel_ipc_pid_t`](#__kernel_ipc_pid_t)
  - [`__kernel_uid_t`](#__kernel_uid_t)
  - [`__kernel_gid_t`](#__kernel_gid_t)
  - [`__kernel_suseconds_t`](#__kernel_suseconds_t)
  - [`__kernel_daddr_t`](#__kernel_daddr_t)
  - [`__kernel_uid32_t`](#__kernel_uid32_t)
  - [`__kernel_gid32_t`](#__kernel_gid32_t)
  - [`__kernel_size_t`](#__kernel_size_t)
  - [`__kernel_ssize_t`](#__kernel_ssize_t)
  - [`__kernel_ptrdiff_t`](#__kernel_ptrdiff_t)
  - [`__kernel_off_t`](#__kernel_off_t)
  - [`__kernel_loff_t`](#__kernel_loff_t)
  - [`__kernel_old_time_t`](#__kernel_old_time_t)
  - [`__kernel_time_t`](#__kernel_time_t)
  - [`__kernel_time64_t`](#__kernel_time64_t)
  - [`__kernel_clock_t`](#__kernel_clock_t)
  - [`__kernel_timer_t`](#__kernel_timer_t)
  - [`__kernel_clockid_t`](#__kernel_clockid_t)
  - [`__kernel_caddr_t`](#__kernel_caddr_t)
  - [`__kernel_uid16_t`](#__kernel_uid16_t)
  - [`__kernel_gid16_t`](#__kernel_gid16_t)
  - [`__s128`](#__s128)
  - [`__u128`](#__u128)
  - [`__le16`](#__le16)
  - [`__be16`](#__be16)
  - [`__le32`](#__le32)
  - [`__be32`](#__be32)
  - [`__le64`](#__le64)
  - [`__be64`](#__be64)
  - [`__sum16`](#__sum16)
  - [`__wsum`](#__wsum)
  - [`__poll_t`](#__poll_t)
  - [`cap_user_header_t`](#cap_user_header_t)
  - [`cap_user_data_t`](#cap_user_data_t)
  - [`__kernel_rwf_t`](#__kernel_rwf_t)
  - [`sigset_t`](#sigset_t)
  - [`__signalfn_t`](#__signalfn_t)
  - [`__sighandler_t`](#__sighandler_t)
  - [`__restorefn_t`](#__restorefn_t)
  - [`__sigrestore_t`](#__sigrestore_t)
  - [`stack_t`](#stack_t)
  - [`sigval_t`](#sigval_t)
  - [`siginfo_t`](#siginfo_t)
  - [`sigevent_t`](#sigevent_t)
  - [`cc_t`](#cc_t)
  - [`speed_t`](#speed_t)
  - [`tcflag_t`](#tcflag_t)
  - [`__fsword_t`](#__fsword_t)
- [Constants](#constants)
  - [`LINUX_VERSION_CODE`](#linux_version_code)
  - [`LINUX_VERSION_MAJOR`](#linux_version_major)
  - [`LINUX_VERSION_PATCHLEVEL`](#linux_version_patchlevel)
  - [`LINUX_VERSION_SUBLEVEL`](#linux_version_sublevel)
  - [`__BITS_PER_LONG_LONG`](#__bits_per_long_long)
  - [`__FD_SETSIZE`](#__fd_setsize)
  - [`_LINUX_CAPABILITY_VERSION_1`](#_linux_capability_version_1)
  - [`_LINUX_CAPABILITY_U32S_1`](#_linux_capability_u32s_1)
  - [`_LINUX_CAPABILITY_VERSION_2`](#_linux_capability_version_2)
  - [`_LINUX_CAPABILITY_U32S_2`](#_linux_capability_u32s_2)
  - [`_LINUX_CAPABILITY_VERSION_3`](#_linux_capability_version_3)
  - [`_LINUX_CAPABILITY_U32S_3`](#_linux_capability_u32s_3)
  - [`VFS_CAP_REVISION_MASK`](#vfs_cap_revision_mask)
  - [`VFS_CAP_REVISION_SHIFT`](#vfs_cap_revision_shift)
  - [`VFS_CAP_FLAGS_MASK`](#vfs_cap_flags_mask)
  - [`VFS_CAP_FLAGS_EFFECTIVE`](#vfs_cap_flags_effective)
  - [`VFS_CAP_REVISION_1`](#vfs_cap_revision_1)
  - [`VFS_CAP_U32_1`](#vfs_cap_u32_1)
  - [`VFS_CAP_REVISION_2`](#vfs_cap_revision_2)
  - [`VFS_CAP_U32_2`](#vfs_cap_u32_2)
  - [`VFS_CAP_REVISION_3`](#vfs_cap_revision_3)
  - [`VFS_CAP_U32_3`](#vfs_cap_u32_3)
  - [`VFS_CAP_U32`](#vfs_cap_u32)
  - [`VFS_CAP_REVISION`](#vfs_cap_revision)
  - [`_LINUX_CAPABILITY_VERSION`](#_linux_capability_version)
  - [`_LINUX_CAPABILITY_U32S`](#_linux_capability_u32s)
  - [`CAP_CHOWN`](#cap_chown)
  - [`CAP_DAC_OVERRIDE`](#cap_dac_override)
  - [`CAP_DAC_READ_SEARCH`](#cap_dac_read_search)
  - [`CAP_FOWNER`](#cap_fowner)
  - [`CAP_FSETID`](#cap_fsetid)
  - [`CAP_KILL`](#cap_kill)
  - [`CAP_SETGID`](#cap_setgid)
  - [`CAP_SETUID`](#cap_setuid)
  - [`CAP_SETPCAP`](#cap_setpcap)
  - [`CAP_LINUX_IMMUTABLE`](#cap_linux_immutable)
  - [`CAP_NET_BIND_SERVICE`](#cap_net_bind_service)
  - [`CAP_NET_BROADCAST`](#cap_net_broadcast)
  - [`CAP_NET_ADMIN`](#cap_net_admin)
  - [`CAP_NET_RAW`](#cap_net_raw)
  - [`CAP_IPC_LOCK`](#cap_ipc_lock)
  - [`CAP_IPC_OWNER`](#cap_ipc_owner)
  - [`CAP_SYS_MODULE`](#cap_sys_module)
  - [`CAP_SYS_RAWIO`](#cap_sys_rawio)
  - [`CAP_SYS_CHROOT`](#cap_sys_chroot)
  - [`CAP_SYS_PTRACE`](#cap_sys_ptrace)
  - [`CAP_SYS_PACCT`](#cap_sys_pacct)
  - [`CAP_SYS_ADMIN`](#cap_sys_admin)
  - [`CAP_SYS_BOOT`](#cap_sys_boot)
  - [`CAP_SYS_NICE`](#cap_sys_nice)
  - [`CAP_SYS_RESOURCE`](#cap_sys_resource)
  - [`CAP_SYS_TIME`](#cap_sys_time)
  - [`CAP_SYS_TTY_CONFIG`](#cap_sys_tty_config)
  - [`CAP_MKNOD`](#cap_mknod)
  - [`CAP_LEASE`](#cap_lease)
  - [`CAP_AUDIT_WRITE`](#cap_audit_write)
  - [`CAP_AUDIT_CONTROL`](#cap_audit_control)
  - [`CAP_SETFCAP`](#cap_setfcap)
  - [`CAP_MAC_OVERRIDE`](#cap_mac_override)
  - [`CAP_MAC_ADMIN`](#cap_mac_admin)
  - [`CAP_SYSLOG`](#cap_syslog)
  - [`CAP_WAKE_ALARM`](#cap_wake_alarm)
  - [`CAP_BLOCK_SUSPEND`](#cap_block_suspend)
  - [`CAP_AUDIT_READ`](#cap_audit_read)
  - [`CAP_PERFMON`](#cap_perfmon)
  - [`CAP_BPF`](#cap_bpf)
  - [`CAP_CHECKPOINT_RESTORE`](#cap_checkpoint_restore)
  - [`CAP_LAST_CAP`](#cap_last_cap)
  - [`O_ACCMODE`](#o_accmode)
  - [`O_RDONLY`](#o_rdonly)
  - [`O_WRONLY`](#o_wronly)
  - [`O_RDWR`](#o_rdwr)
  - [`O_CREAT`](#o_creat)
  - [`O_EXCL`](#o_excl)
  - [`O_NOCTTY`](#o_noctty)
  - [`O_TRUNC`](#o_trunc)
  - [`O_APPEND`](#o_append)
  - [`O_NONBLOCK`](#o_nonblock)
  - [`O_DSYNC`](#o_dsync)
  - [`FASYNC`](#fasync)
  - [`O_DIRECT`](#o_direct)
  - [`O_LARGEFILE`](#o_largefile)
  - [`O_DIRECTORY`](#o_directory)
  - [`O_NOFOLLOW`](#o_nofollow)
  - [`O_NOATIME`](#o_noatime)
  - [`O_CLOEXEC`](#o_cloexec)
  - [`__O_SYNC`](#__o_sync)
  - [`O_SYNC`](#o_sync)
  - [`O_PATH`](#o_path)
  - [`__O_TMPFILE`](#__o_tmpfile)
  - [`O_TMPFILE`](#o_tmpfile)
  - [`O_NDELAY`](#o_ndelay)
  - [`F_DUPFD`](#f_dupfd)
  - [`F_GETFD`](#f_getfd)
  - [`F_SETFD`](#f_setfd)
  - [`F_GETFL`](#f_getfl)
  - [`F_SETFL`](#f_setfl)
  - [`F_GETLK`](#f_getlk)
  - [`F_SETLK`](#f_setlk)
  - [`F_SETLKW`](#f_setlkw)
  - [`F_SETOWN`](#f_setown)
  - [`F_GETOWN`](#f_getown)
  - [`F_SETSIG`](#f_setsig)
  - [`F_GETSIG`](#f_getsig)
  - [`F_SETOWN_EX`](#f_setown_ex)
  - [`F_GETOWN_EX`](#f_getown_ex)
  - [`F_GETOWNER_UIDS`](#f_getowner_uids)
  - [`F_OFD_GETLK`](#f_ofd_getlk)
  - [`F_OFD_SETLK`](#f_ofd_setlk)
  - [`F_OFD_SETLKW`](#f_ofd_setlkw)
  - [`F_OWNER_TID`](#f_owner_tid)
  - [`F_OWNER_PID`](#f_owner_pid)
  - [`F_OWNER_PGRP`](#f_owner_pgrp)
  - [`FD_CLOEXEC`](#fd_cloexec)
  - [`F_RDLCK`](#f_rdlck)
  - [`F_WRLCK`](#f_wrlck)
  - [`F_UNLCK`](#f_unlck)
  - [`F_EXLCK`](#f_exlck)
  - [`F_SHLCK`](#f_shlck)
  - [`LOCK_SH`](#lock_sh)
  - [`LOCK_EX`](#lock_ex)
  - [`LOCK_NB`](#lock_nb)
  - [`LOCK_UN`](#lock_un)
  - [`LOCK_MAND`](#lock_mand)
  - [`LOCK_READ`](#lock_read)
  - [`LOCK_WRITE`](#lock_write)
  - [`LOCK_RW`](#lock_rw)
  - [`F_LINUX_SPECIFIC_BASE`](#f_linux_specific_base)
  - [`RESOLVE_NO_XDEV`](#resolve_no_xdev)
  - [`RESOLVE_NO_MAGICLINKS`](#resolve_no_magiclinks)
  - [`RESOLVE_NO_SYMLINKS`](#resolve_no_symlinks)
  - [`RESOLVE_BENEATH`](#resolve_beneath)
  - [`RESOLVE_IN_ROOT`](#resolve_in_root)
  - [`RESOLVE_CACHED`](#resolve_cached)
  - [`F_SETLEASE`](#f_setlease)
  - [`F_GETLEASE`](#f_getlease)
  - [`F_NOTIFY`](#f_notify)
  - [`F_DUPFD_QUERY`](#f_dupfd_query)
  - [`F_CREATED_QUERY`](#f_created_query)
  - [`F_CANCELLK`](#f_cancellk)
  - [`F_DUPFD_CLOEXEC`](#f_dupfd_cloexec)
  - [`F_SETPIPE_SZ`](#f_setpipe_sz)
  - [`F_GETPIPE_SZ`](#f_getpipe_sz)
  - [`F_ADD_SEALS`](#f_add_seals)
  - [`F_GET_SEALS`](#f_get_seals)
  - [`F_SEAL_SEAL`](#f_seal_seal)
  - [`F_SEAL_SHRINK`](#f_seal_shrink)
  - [`F_SEAL_GROW`](#f_seal_grow)
  - [`F_SEAL_WRITE`](#f_seal_write)
  - [`F_SEAL_FUTURE_WRITE`](#f_seal_future_write)
  - [`F_SEAL_EXEC`](#f_seal_exec)
  - [`F_GET_RW_HINT`](#f_get_rw_hint)
  - [`F_SET_RW_HINT`](#f_set_rw_hint)
  - [`F_GET_FILE_RW_HINT`](#f_get_file_rw_hint)
  - [`F_SET_FILE_RW_HINT`](#f_set_file_rw_hint)
  - [`RWH_WRITE_LIFE_NOT_SET`](#rwh_write_life_not_set)
  - [`RWH_WRITE_LIFE_NONE`](#rwh_write_life_none)
  - [`RWH_WRITE_LIFE_SHORT`](#rwh_write_life_short)
  - [`RWH_WRITE_LIFE_MEDIUM`](#rwh_write_life_medium)
  - [`RWH_WRITE_LIFE_LONG`](#rwh_write_life_long)
  - [`RWH_WRITE_LIFE_EXTREME`](#rwh_write_life_extreme)
  - [`RWF_WRITE_LIFE_NOT_SET`](#rwf_write_life_not_set)
  - [`DN_ACCESS`](#dn_access)
  - [`DN_MODIFY`](#dn_modify)
  - [`DN_CREATE`](#dn_create)
  - [`DN_DELETE`](#dn_delete)
  - [`DN_RENAME`](#dn_rename)
  - [`DN_ATTRIB`](#dn_attrib)
  - [`DN_MULTISHOT`](#dn_multishot)
  - [`AT_FDCWD`](#at_fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at_symlink_nofollow)
  - [`AT_SYMLINK_FOLLOW`](#at_symlink_follow)
  - [`AT_NO_AUTOMOUNT`](#at_no_automount)
  - [`AT_EMPTY_PATH`](#at_empty_path)
  - [`AT_STATX_SYNC_TYPE`](#at_statx_sync_type)
  - [`AT_STATX_SYNC_AS_STAT`](#at_statx_sync_as_stat)
  - [`AT_STATX_FORCE_SYNC`](#at_statx_force_sync)
  - [`AT_STATX_DONT_SYNC`](#at_statx_dont_sync)
  - [`AT_RECURSIVE`](#at_recursive)
  - [`AT_RENAME_NOREPLACE`](#at_rename_noreplace)
  - [`AT_RENAME_EXCHANGE`](#at_rename_exchange)
  - [`AT_RENAME_WHITEOUT`](#at_rename_whiteout)
  - [`AT_EACCESS`](#at_eaccess)
  - [`AT_REMOVEDIR`](#at_removedir)
  - [`AT_HANDLE_FID`](#at_handle_fid)
  - [`AT_HANDLE_MNT_ID_UNIQUE`](#at_handle_mnt_id_unique)
  - [`AT_HANDLE_CONNECTABLE`](#at_handle_connectable)
  - [`AT_EXECVE_CHECK`](#at_execve_check)
  - [`EPOLL_CLOEXEC`](#epoll_cloexec)
  - [`EPOLL_CTL_ADD`](#epoll_ctl_add)
  - [`EPOLL_CTL_DEL`](#epoll_ctl_del)
  - [`EPOLL_CTL_MOD`](#epoll_ctl_mod)
  - [`EPOLL_IOC_TYPE`](#epoll_ioc_type)
  - [`POSIX_FADV_NORMAL`](#posix_fadv_normal)
  - [`POSIX_FADV_RANDOM`](#posix_fadv_random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix_fadv_sequential)
  - [`POSIX_FADV_WILLNEED`](#posix_fadv_willneed)
  - [`POSIX_FADV_DONTNEED`](#posix_fadv_dontneed)
  - [`POSIX_FADV_NOREUSE`](#posix_fadv_noreuse)
  - [`FALLOC_FL_ALLOCATE_RANGE`](#falloc_fl_allocate_range)
  - [`FALLOC_FL_KEEP_SIZE`](#falloc_fl_keep_size)
  - [`FALLOC_FL_PUNCH_HOLE`](#falloc_fl_punch_hole)
  - [`FALLOC_FL_NO_HIDE_STALE`](#falloc_fl_no_hide_stale)
  - [`FALLOC_FL_COLLAPSE_RANGE`](#falloc_fl_collapse_range)
  - [`FALLOC_FL_ZERO_RANGE`](#falloc_fl_zero_range)
  - [`FALLOC_FL_INSERT_RANGE`](#falloc_fl_insert_range)
  - [`FALLOC_FL_UNSHARE_RANGE`](#falloc_fl_unshare_range)
  - [`NR_OPEN`](#nr_open)
  - [`NGROUPS_MAX`](#ngroups_max)
  - [`ARG_MAX`](#arg_max)
  - [`LINK_MAX`](#link_max)
  - [`MAX_CANON`](#max_canon)
  - [`MAX_INPUT`](#max_input)
  - [`NAME_MAX`](#name_max)
  - [`PATH_MAX`](#path_max)
  - [`PIPE_BUF`](#pipe_buf)
  - [`XATTR_NAME_MAX`](#xattr_name_max)
  - [`XATTR_SIZE_MAX`](#xattr_size_max)
  - [`XATTR_LIST_MAX`](#xattr_list_max)
  - [`RTSIG_MAX`](#rtsig_max)
  - [`_IOC_NRBITS`](#_ioc_nrbits)
  - [`_IOC_TYPEBITS`](#_ioc_typebits)
  - [`_IOC_SIZEBITS`](#_ioc_sizebits)
  - [`_IOC_DIRBITS`](#_ioc_dirbits)
  - [`_IOC_NRMASK`](#_ioc_nrmask)
  - [`_IOC_TYPEMASK`](#_ioc_typemask)
  - [`_IOC_SIZEMASK`](#_ioc_sizemask)
  - [`_IOC_DIRMASK`](#_ioc_dirmask)
  - [`_IOC_NRSHIFT`](#_ioc_nrshift)
  - [`_IOC_TYPESHIFT`](#_ioc_typeshift)
  - [`_IOC_SIZESHIFT`](#_ioc_sizeshift)
  - [`_IOC_DIRSHIFT`](#_ioc_dirshift)
  - [`_IOC_NONE`](#_ioc_none)
  - [`_IOC_WRITE`](#_ioc_write)
  - [`_IOC_READ`](#_ioc_read)
  - [`IOC_IN`](#ioc_in)
  - [`IOC_OUT`](#ioc_out)
  - [`IOC_INOUT`](#ioc_inout)
  - [`IOCSIZE_MASK`](#iocsize_mask)
  - [`IOCSIZE_SHIFT`](#iocsize_shift)
  - [`FSCRYPT_POLICY_FLAGS_PAD_4`](#fscrypt_policy_flags_pad_4)
  - [`FSCRYPT_POLICY_FLAGS_PAD_8`](#fscrypt_policy_flags_pad_8)
  - [`FSCRYPT_POLICY_FLAGS_PAD_16`](#fscrypt_policy_flags_pad_16)
  - [`FSCRYPT_POLICY_FLAGS_PAD_32`](#fscrypt_policy_flags_pad_32)
  - [`FSCRYPT_POLICY_FLAGS_PAD_MASK`](#fscrypt_policy_flags_pad_mask)
  - [`FSCRYPT_POLICY_FLAG_DIRECT_KEY`](#fscrypt_policy_flag_direct_key)
  - [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`](#fscrypt_policy_flag_iv_ino_lblk_64)
  - [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`](#fscrypt_policy_flag_iv_ino_lblk_32)
  - [`FSCRYPT_MODE_AES_256_XTS`](#fscrypt_mode_aes_256_xts)
  - [`FSCRYPT_MODE_AES_256_CTS`](#fscrypt_mode_aes_256_cts)
  - [`FSCRYPT_MODE_AES_128_CBC`](#fscrypt_mode_aes_128_cbc)
  - [`FSCRYPT_MODE_AES_128_CTS`](#fscrypt_mode_aes_128_cts)
  - [`FSCRYPT_MODE_SM4_XTS`](#fscrypt_mode_sm4_xts)
  - [`FSCRYPT_MODE_SM4_CTS`](#fscrypt_mode_sm4_cts)
  - [`FSCRYPT_MODE_ADIANTUM`](#fscrypt_mode_adiantum)
  - [`FSCRYPT_MODE_AES_256_HCTR2`](#fscrypt_mode_aes_256_hctr2)
  - [`FSCRYPT_POLICY_V1`](#fscrypt_policy_v1)
  - [`FSCRYPT_KEY_DESCRIPTOR_SIZE`](#fscrypt_key_descriptor_size)
  - [`FSCRYPT_KEY_DESC_PREFIX`](#fscrypt_key_desc_prefix)
  - [`FSCRYPT_KEY_DESC_PREFIX_SIZE`](#fscrypt_key_desc_prefix_size)
  - [`FSCRYPT_MAX_KEY_SIZE`](#fscrypt_max_key_size)
  - [`FSCRYPT_POLICY_V2`](#fscrypt_policy_v2)
  - [`FSCRYPT_KEY_IDENTIFIER_SIZE`](#fscrypt_key_identifier_size)
  - [`FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`](#fscrypt_key_spec_type_descriptor)
  - [`FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`](#fscrypt_key_spec_type_identifier)
  - [`FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`](#fscrypt_add_key_flag_hw_wrapped)
  - [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`](#fscrypt_key_removal_status_flag_files_busy)
  - [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`](#fscrypt_key_removal_status_flag_other_users)
  - [`FSCRYPT_KEY_STATUS_ABSENT`](#fscrypt_key_status_absent)
  - [`FSCRYPT_KEY_STATUS_PRESENT`](#fscrypt_key_status_present)
  - [`FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`](#fscrypt_key_status_incompletely_removed)
  - [`FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`](#fscrypt_key_status_flag_added_by_self)
  - [`FS_KEY_DESCRIPTOR_SIZE`](#fs_key_descriptor_size)
  - [`FS_POLICY_FLAGS_PAD_4`](#fs_policy_flags_pad_4)
  - [`FS_POLICY_FLAGS_PAD_8`](#fs_policy_flags_pad_8)
  - [`FS_POLICY_FLAGS_PAD_16`](#fs_policy_flags_pad_16)
  - [`FS_POLICY_FLAGS_PAD_32`](#fs_policy_flags_pad_32)
  - [`FS_POLICY_FLAGS_PAD_MASK`](#fs_policy_flags_pad_mask)
  - [`FS_POLICY_FLAG_DIRECT_KEY`](#fs_policy_flag_direct_key)
  - [`FS_POLICY_FLAGS_VALID`](#fs_policy_flags_valid)
  - [`FS_ENCRYPTION_MODE_INVALID`](#fs_encryption_mode_invalid)
  - [`FS_ENCRYPTION_MODE_AES_256_XTS`](#fs_encryption_mode_aes_256_xts)
  - [`FS_ENCRYPTION_MODE_AES_256_GCM`](#fs_encryption_mode_aes_256_gcm)
  - [`FS_ENCRYPTION_MODE_AES_256_CBC`](#fs_encryption_mode_aes_256_cbc)
  - [`FS_ENCRYPTION_MODE_AES_256_CTS`](#fs_encryption_mode_aes_256_cts)
  - [`FS_ENCRYPTION_MODE_AES_128_CBC`](#fs_encryption_mode_aes_128_cbc)
  - [`FS_ENCRYPTION_MODE_AES_128_CTS`](#fs_encryption_mode_aes_128_cts)
  - [`FS_ENCRYPTION_MODE_ADIANTUM`](#fs_encryption_mode_adiantum)
  - [`FS_KEY_DESC_PREFIX`](#fs_key_desc_prefix)
  - [`FS_KEY_DESC_PREFIX_SIZE`](#fs_key_desc_prefix_size)
  - [`FS_MAX_KEY_SIZE`](#fs_max_key_size)
  - [`MS_RDONLY`](#ms_rdonly)
  - [`MS_NOSUID`](#ms_nosuid)
  - [`MS_NODEV`](#ms_nodev)
  - [`MS_NOEXEC`](#ms_noexec)
  - [`MS_SYNCHRONOUS`](#ms_synchronous)
  - [`MS_REMOUNT`](#ms_remount)
  - [`MS_MANDLOCK`](#ms_mandlock)
  - [`MS_DIRSYNC`](#ms_dirsync)
  - [`MS_NOSYMFOLLOW`](#ms_nosymfollow)
  - [`MS_NOATIME`](#ms_noatime)
  - [`MS_NODIRATIME`](#ms_nodiratime)
  - [`MS_BIND`](#ms_bind)
  - [`MS_MOVE`](#ms_move)
  - [`MS_REC`](#ms_rec)
  - [`MS_VERBOSE`](#ms_verbose)
  - [`MS_SILENT`](#ms_silent)
  - [`MS_POSIXACL`](#ms_posixacl)
  - [`MS_UNBINDABLE`](#ms_unbindable)
  - [`MS_PRIVATE`](#ms_private)
  - [`MS_SLAVE`](#ms_slave)
  - [`MS_SHARED`](#ms_shared)
  - [`MS_RELATIME`](#ms_relatime)
  - [`MS_KERNMOUNT`](#ms_kernmount)
  - [`MS_I_VERSION`](#ms_i_version)
  - [`MS_STRICTATIME`](#ms_strictatime)
  - [`MS_LAZYTIME`](#ms_lazytime)
  - [`MS_SUBMOUNT`](#ms_submount)
  - [`MS_NOREMOTELOCK`](#ms_noremotelock)
  - [`MS_NOSEC`](#ms_nosec)
  - [`MS_BORN`](#ms_born)
  - [`MS_ACTIVE`](#ms_active)
  - [`MS_NOUSER`](#ms_nouser)
  - [`MS_RMT_MASK`](#ms_rmt_mask)
  - [`MS_MGC_VAL`](#ms_mgc_val)
  - [`MS_MGC_MSK`](#ms_mgc_msk)
  - [`OPEN_TREE_CLONE`](#open_tree_clone)
  - [`OPEN_TREE_CLOEXEC`](#open_tree_cloexec)
  - [`MOVE_MOUNT_F_SYMLINKS`](#move_mount_f_symlinks)
  - [`MOVE_MOUNT_F_AUTOMOUNTS`](#move_mount_f_automounts)
  - [`MOVE_MOUNT_F_EMPTY_PATH`](#move_mount_f_empty_path)
  - [`MOVE_MOUNT_T_SYMLINKS`](#move_mount_t_symlinks)
  - [`MOVE_MOUNT_T_AUTOMOUNTS`](#move_mount_t_automounts)
  - [`MOVE_MOUNT_T_EMPTY_PATH`](#move_mount_t_empty_path)
  - [`MOVE_MOUNT_SET_GROUP`](#move_mount_set_group)
  - [`MOVE_MOUNT_BENEATH`](#move_mount_beneath)
  - [`MOVE_MOUNT__MASK`](#move_mount__mask)
  - [`FSOPEN_CLOEXEC`](#fsopen_cloexec)
  - [`FSPICK_CLOEXEC`](#fspick_cloexec)
  - [`FSPICK_SYMLINK_NOFOLLOW`](#fspick_symlink_nofollow)
  - [`FSPICK_NO_AUTOMOUNT`](#fspick_no_automount)
  - [`FSPICK_EMPTY_PATH`](#fspick_empty_path)
  - [`FSMOUNT_CLOEXEC`](#fsmount_cloexec)
  - [`MOUNT_ATTR_RDONLY`](#mount_attr_rdonly)
  - [`MOUNT_ATTR_NOSUID`](#mount_attr_nosuid)
  - [`MOUNT_ATTR_NODEV`](#mount_attr_nodev)
  - [`MOUNT_ATTR_NOEXEC`](#mount_attr_noexec)
  - [`MOUNT_ATTR__ATIME`](#mount_attr__atime)
  - [`MOUNT_ATTR_RELATIME`](#mount_attr_relatime)
  - [`MOUNT_ATTR_NOATIME`](#mount_attr_noatime)
  - [`MOUNT_ATTR_STRICTATIME`](#mount_attr_strictatime)
  - [`MOUNT_ATTR_NODIRATIME`](#mount_attr_nodiratime)
  - [`MOUNT_ATTR_IDMAP`](#mount_attr_idmap)
  - [`MOUNT_ATTR_NOSYMFOLLOW`](#mount_attr_nosymfollow)
  - [`MOUNT_ATTR_SIZE_VER0`](#mount_attr_size_ver0)
  - [`MNT_ID_REQ_SIZE_VER0`](#mnt_id_req_size_ver0)
  - [`MNT_ID_REQ_SIZE_VER1`](#mnt_id_req_size_ver1)
  - [`STATMOUNT_SB_BASIC`](#statmount_sb_basic)
  - [`STATMOUNT_MNT_BASIC`](#statmount_mnt_basic)
  - [`STATMOUNT_PROPAGATE_FROM`](#statmount_propagate_from)
  - [`STATMOUNT_MNT_ROOT`](#statmount_mnt_root)
  - [`STATMOUNT_MNT_POINT`](#statmount_mnt_point)
  - [`STATMOUNT_FS_TYPE`](#statmount_fs_type)
  - [`STATMOUNT_MNT_NS_ID`](#statmount_mnt_ns_id)
  - [`STATMOUNT_MNT_OPTS`](#statmount_mnt_opts)
  - [`STATMOUNT_FS_SUBTYPE`](#statmount_fs_subtype)
  - [`STATMOUNT_SB_SOURCE`](#statmount_sb_source)
  - [`STATMOUNT_OPT_ARRAY`](#statmount_opt_array)
  - [`STATMOUNT_OPT_SEC_ARRAY`](#statmount_opt_sec_array)
  - [`STATMOUNT_SUPPORTED_MASK`](#statmount_supported_mask)
  - [`STATMOUNT_MNT_UIDMAP`](#statmount_mnt_uidmap)
  - [`STATMOUNT_MNT_GIDMAP`](#statmount_mnt_gidmap)
  - [`LSMT_ROOT`](#lsmt_root)
  - [`LISTMOUNT_REVERSE`](#listmount_reverse)
  - [`INR_OPEN_CUR`](#inr_open_cur)
  - [`INR_OPEN_MAX`](#inr_open_max)
  - [`BLOCK_SIZE_BITS`](#block_size_bits)
  - [`BLOCK_SIZE`](#block_size)
  - [`IO_INTEGRITY_CHK_GUARD`](#io_integrity_chk_guard)
  - [`IO_INTEGRITY_CHK_REFTAG`](#io_integrity_chk_reftag)
  - [`IO_INTEGRITY_CHK_APPTAG`](#io_integrity_chk_apptag)
  - [`IO_INTEGRITY_VALID_FLAGS`](#io_integrity_valid_flags)
  - [`SEEK_SET`](#seek_set)
  - [`SEEK_CUR`](#seek_cur)
  - [`SEEK_END`](#seek_end)
  - [`SEEK_DATA`](#seek_data)
  - [`SEEK_HOLE`](#seek_hole)
  - [`SEEK_MAX`](#seek_max)
  - [`RENAME_NOREPLACE`](#rename_noreplace)
  - [`RENAME_EXCHANGE`](#rename_exchange)
  - [`RENAME_WHITEOUT`](#rename_whiteout)
  - [`FILE_DEDUPE_RANGE_SAME`](#file_dedupe_range_same)
  - [`FILE_DEDUPE_RANGE_DIFFERS`](#file_dedupe_range_differs)
  - [`NR_FILE`](#nr_file)
  - [`FS_XFLAG_REALTIME`](#fs_xflag_realtime)
  - [`FS_XFLAG_PREALLOC`](#fs_xflag_prealloc)
  - [`FS_XFLAG_IMMUTABLE`](#fs_xflag_immutable)
  - [`FS_XFLAG_APPEND`](#fs_xflag_append)
  - [`FS_XFLAG_SYNC`](#fs_xflag_sync)
  - [`FS_XFLAG_NOATIME`](#fs_xflag_noatime)
  - [`FS_XFLAG_NODUMP`](#fs_xflag_nodump)
  - [`FS_XFLAG_RTINHERIT`](#fs_xflag_rtinherit)
  - [`FS_XFLAG_PROJINHERIT`](#fs_xflag_projinherit)
  - [`FS_XFLAG_NOSYMLINKS`](#fs_xflag_nosymlinks)
  - [`FS_XFLAG_EXTSIZE`](#fs_xflag_extsize)
  - [`FS_XFLAG_EXTSZINHERIT`](#fs_xflag_extszinherit)
  - [`FS_XFLAG_NODEFRAG`](#fs_xflag_nodefrag)
  - [`FS_XFLAG_FILESTREAM`](#fs_xflag_filestream)
  - [`FS_XFLAG_DAX`](#fs_xflag_dax)
  - [`FS_XFLAG_COWEXTSIZE`](#fs_xflag_cowextsize)
  - [`FS_XFLAG_HASATTR`](#fs_xflag_hasattr)
  - [`BMAP_IOCTL`](#bmap_ioctl)
  - [`FSLABEL_MAX`](#fslabel_max)
  - [`FS_SECRM_FL`](#fs_secrm_fl)
  - [`FS_UNRM_FL`](#fs_unrm_fl)
  - [`FS_COMPR_FL`](#fs_compr_fl)
  - [`FS_SYNC_FL`](#fs_sync_fl)
  - [`FS_IMMUTABLE_FL`](#fs_immutable_fl)
  - [`FS_APPEND_FL`](#fs_append_fl)
  - [`FS_NODUMP_FL`](#fs_nodump_fl)
  - [`FS_NOATIME_FL`](#fs_noatime_fl)
  - [`FS_DIRTY_FL`](#fs_dirty_fl)
  - [`FS_COMPRBLK_FL`](#fs_comprblk_fl)
  - [`FS_NOCOMP_FL`](#fs_nocomp_fl)
  - [`FS_ENCRYPT_FL`](#fs_encrypt_fl)
  - [`FS_BTREE_FL`](#fs_btree_fl)
  - [`FS_INDEX_FL`](#fs_index_fl)
  - [`FS_IMAGIC_FL`](#fs_imagic_fl)
  - [`FS_JOURNAL_DATA_FL`](#fs_journal_data_fl)
  - [`FS_NOTAIL_FL`](#fs_notail_fl)
  - [`FS_DIRSYNC_FL`](#fs_dirsync_fl)
  - [`FS_TOPDIR_FL`](#fs_topdir_fl)
  - [`FS_HUGE_FILE_FL`](#fs_huge_file_fl)
  - [`FS_EXTENT_FL`](#fs_extent_fl)
  - [`FS_VERITY_FL`](#fs_verity_fl)
  - [`FS_EA_INODE_FL`](#fs_ea_inode_fl)
  - [`FS_EOFBLOCKS_FL`](#fs_eofblocks_fl)
  - [`FS_NOCOW_FL`](#fs_nocow_fl)
  - [`FS_DAX_FL`](#fs_dax_fl)
  - [`FS_INLINE_DATA_FL`](#fs_inline_data_fl)
  - [`FS_PROJINHERIT_FL`](#fs_projinherit_fl)
  - [`FS_CASEFOLD_FL`](#fs_casefold_fl)
  - [`FS_RESERVED_FL`](#fs_reserved_fl)
  - [`FS_FL_USER_VISIBLE`](#fs_fl_user_visible)
  - [`FS_FL_USER_MODIFIABLE`](#fs_fl_user_modifiable)
  - [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync_file_range_wait_before)
  - [`SYNC_FILE_RANGE_WRITE`](#sync_file_range_write)
  - [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync_file_range_wait_after)
  - [`SYNC_FILE_RANGE_WRITE_AND_WAIT`](#sync_file_range_write_and_wait)
  - [`PROCFS_IOCTL_MAGIC`](#procfs_ioctl_magic)
  - [`PAGE_IS_WPALLOWED`](#page_is_wpallowed)
  - [`PAGE_IS_WRITTEN`](#page_is_written)
  - [`PAGE_IS_FILE`](#page_is_file)
  - [`PAGE_IS_PRESENT`](#page_is_present)
  - [`PAGE_IS_SWAPPED`](#page_is_swapped)
  - [`PAGE_IS_PFNZERO`](#page_is_pfnzero)
  - [`PAGE_IS_HUGE`](#page_is_huge)
  - [`PAGE_IS_SOFT_DIRTY`](#page_is_soft_dirty)
  - [`PAGE_IS_GUARD`](#page_is_guard)
  - [`PM_SCAN_WP_MATCHING`](#pm_scan_wp_matching)
  - [`PM_SCAN_CHECK_WPASYNC`](#pm_scan_check_wpasync)
  - [`FUTEX_WAIT`](#futex_wait)
  - [`FUTEX_WAKE`](#futex_wake)
  - [`FUTEX_FD`](#futex_fd)
  - [`FUTEX_REQUEUE`](#futex_requeue)
  - [`FUTEX_CMP_REQUEUE`](#futex_cmp_requeue)
  - [`FUTEX_WAKE_OP`](#futex_wake_op)
  - [`FUTEX_LOCK_PI`](#futex_lock_pi)
  - [`FUTEX_UNLOCK_PI`](#futex_unlock_pi)
  - [`FUTEX_TRYLOCK_PI`](#futex_trylock_pi)
  - [`FUTEX_WAIT_BITSET`](#futex_wait_bitset)
  - [`FUTEX_WAKE_BITSET`](#futex_wake_bitset)
  - [`FUTEX_WAIT_REQUEUE_PI`](#futex_wait_requeue_pi)
  - [`FUTEX_CMP_REQUEUE_PI`](#futex_cmp_requeue_pi)
  - [`FUTEX_LOCK_PI2`](#futex_lock_pi2)
  - [`FUTEX_PRIVATE_FLAG`](#futex_private_flag)
  - [`FUTEX_CLOCK_REALTIME`](#futex_clock_realtime)
  - [`FUTEX_CMD_MASK`](#futex_cmd_mask)
  - [`FUTEX_WAIT_PRIVATE`](#futex_wait_private)
  - [`FUTEX_WAKE_PRIVATE`](#futex_wake_private)
  - [`FUTEX_REQUEUE_PRIVATE`](#futex_requeue_private)
  - [`FUTEX_CMP_REQUEUE_PRIVATE`](#futex_cmp_requeue_private)
  - [`FUTEX_WAKE_OP_PRIVATE`](#futex_wake_op_private)
  - [`FUTEX_LOCK_PI_PRIVATE`](#futex_lock_pi_private)
  - [`FUTEX_LOCK_PI2_PRIVATE`](#futex_lock_pi2_private)
  - [`FUTEX_UNLOCK_PI_PRIVATE`](#futex_unlock_pi_private)
  - [`FUTEX_TRYLOCK_PI_PRIVATE`](#futex_trylock_pi_private)
  - [`FUTEX_WAIT_BITSET_PRIVATE`](#futex_wait_bitset_private)
  - [`FUTEX_WAKE_BITSET_PRIVATE`](#futex_wake_bitset_private)
  - [`FUTEX_WAIT_REQUEUE_PI_PRIVATE`](#futex_wait_requeue_pi_private)
  - [`FUTEX_CMP_REQUEUE_PI_PRIVATE`](#futex_cmp_requeue_pi_private)
  - [`FUTEX2_SIZE_U8`](#futex2_size_u8)
  - [`FUTEX2_SIZE_U16`](#futex2_size_u16)
  - [`FUTEX2_SIZE_U32`](#futex2_size_u32)
  - [`FUTEX2_SIZE_U64`](#futex2_size_u64)
  - [`FUTEX2_NUMA`](#futex2_numa)
  - [`FUTEX2_MPOL`](#futex2_mpol)
  - [`FUTEX2_PRIVATE`](#futex2_private)
  - [`FUTEX2_SIZE_MASK`](#futex2_size_mask)
  - [`FUTEX_32`](#futex_32)
  - [`FUTEX_NO_NODE`](#futex_no_node)
  - [`FUTEX_WAITV_MAX`](#futex_waitv_max)
  - [`FUTEX_WAITERS`](#futex_waiters)
  - [`FUTEX_OWNER_DIED`](#futex_owner_died)
  - [`FUTEX_TID_MASK`](#futex_tid_mask)
  - [`ROBUST_LIST_LIMIT`](#robust_list_limit)
  - [`FUTEX_BITSET_MATCH_ANY`](#futex_bitset_match_any)
  - [`FUTEX_OP_SET`](#futex_op_set)
  - [`FUTEX_OP_ADD`](#futex_op_add)
  - [`FUTEX_OP_OR`](#futex_op_or)
  - [`FUTEX_OP_ANDN`](#futex_op_andn)
  - [`FUTEX_OP_XOR`](#futex_op_xor)
  - [`FUTEX_OP_OPARG_SHIFT`](#futex_op_oparg_shift)
  - [`FUTEX_OP_CMP_EQ`](#futex_op_cmp_eq)
  - [`FUTEX_OP_CMP_NE`](#futex_op_cmp_ne)
  - [`FUTEX_OP_CMP_LT`](#futex_op_cmp_lt)
  - [`FUTEX_OP_CMP_LE`](#futex_op_cmp_le)
  - [`FUTEX_OP_CMP_GT`](#futex_op_cmp_gt)
  - [`FUTEX_OP_CMP_GE`](#futex_op_cmp_ge)
  - [`IN_ACCESS`](#in_access)
  - [`IN_MODIFY`](#in_modify)
  - [`IN_ATTRIB`](#in_attrib)
  - [`IN_CLOSE_WRITE`](#in_close_write)
  - [`IN_CLOSE_NOWRITE`](#in_close_nowrite)
  - [`IN_OPEN`](#in_open)
  - [`IN_MOVED_FROM`](#in_moved_from)
  - [`IN_MOVED_TO`](#in_moved_to)
  - [`IN_CREATE`](#in_create)
  - [`IN_DELETE`](#in_delete)
  - [`IN_DELETE_SELF`](#in_delete_self)
  - [`IN_MOVE_SELF`](#in_move_self)
  - [`IN_UNMOUNT`](#in_unmount)
  - [`IN_Q_OVERFLOW`](#in_q_overflow)
  - [`IN_IGNORED`](#in_ignored)
  - [`IN_CLOSE`](#in_close)
  - [`IN_MOVE`](#in_move)
  - [`IN_ONLYDIR`](#in_onlydir)
  - [`IN_DONT_FOLLOW`](#in_dont_follow)
  - [`IN_EXCL_UNLINK`](#in_excl_unlink)
  - [`IN_MASK_CREATE`](#in_mask_create)
  - [`IN_MASK_ADD`](#in_mask_add)
  - [`IN_ISDIR`](#in_isdir)
  - [`IN_ONESHOT`](#in_oneshot)
  - [`IN_ALL_EVENTS`](#in_all_events)
  - [`IN_CLOEXEC`](#in_cloexec)
  - [`IN_NONBLOCK`](#in_nonblock)
  - [`ADFS_SUPER_MAGIC`](#adfs_super_magic)
  - [`AFFS_SUPER_MAGIC`](#affs_super_magic)
  - [`AFS_SUPER_MAGIC`](#afs_super_magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs_super_magic)
  - [`CEPH_SUPER_MAGIC`](#ceph_super_magic)
  - [`CODA_SUPER_MAGIC`](#coda_super_magic)
  - [`CRAMFS_MAGIC`](#cramfs_magic)
  - [`CRAMFS_MAGIC_WEND`](#cramfs_magic_wend)
  - [`DEBUGFS_MAGIC`](#debugfs_magic)
  - [`SECURITYFS_MAGIC`](#securityfs_magic)
  - [`SELINUX_MAGIC`](#selinux_magic)
  - [`SMACK_MAGIC`](#smack_magic)
  - [`RAMFS_MAGIC`](#ramfs_magic)
  - [`TMPFS_MAGIC`](#tmpfs_magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs_magic)
  - [`SQUASHFS_MAGIC`](#squashfs_magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs_super_magic)
  - [`EFS_SUPER_MAGIC`](#efs_super_magic)
  - [`EROFS_SUPER_MAGIC_V1`](#erofs_super_magic_v1)
  - [`EXT2_SUPER_MAGIC`](#ext2_super_magic)
  - [`EXT3_SUPER_MAGIC`](#ext3_super_magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs_super_magic)
  - [`EXT4_SUPER_MAGIC`](#ext4_super_magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs_super_magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs_super_magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs_super_magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs_super_magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs_super_magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2_super_magic)
  - [`XFS_SUPER_MAGIC`](#xfs_super_magic)
  - [`PSTOREFS_MAGIC`](#pstorefs_magic)
  - [`EFIVARFS_MAGIC`](#efivarfs_magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs_super_magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs_super_magic)
  - [`FUSE_SUPER_MAGIC`](#fuse_super_magic)
  - [`BCACHEFS_SUPER_MAGIC`](#bcachefs_super_magic)
  - [`MINIX_SUPER_MAGIC`](#minix_super_magic)
  - [`MINIX_SUPER_MAGIC2`](#minix_super_magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2_super_magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2_super_magic2)
  - [`MINIX3_SUPER_MAGIC`](#minix3_super_magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos_super_magic)
  - [`EXFAT_SUPER_MAGIC`](#exfat_super_magic)
  - [`NCP_SUPER_MAGIC`](#ncp_super_magic)
  - [`NFS_SUPER_MAGIC`](#nfs_super_magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2_super_magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom_super_magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4_super_magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6_super_magic)
  - [`AFS_FS_MAGIC`](#afs_fs_magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs_super_magic)
  - [`REISERFS_SUPER_MAGIC_STRING`](#reiserfs_super_magic_string)
  - [`REISER2FS_SUPER_MAGIC_STRING`](#reiser2fs_super_magic_string)
  - [`REISER2FS_JR_SUPER_MAGIC_STRING`](#reiser2fs_jr_super_magic_string)
  - [`SMB_SUPER_MAGIC`](#smb_super_magic)
  - [`CIFS_SUPER_MAGIC`](#cifs_super_magic)
  - [`SMB2_SUPER_MAGIC`](#smb2_super_magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup_super_magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2_super_magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup_super_magic)
  - [`STACK_END_MAGIC`](#stack_end_magic)
  - [`TRACEFS_MAGIC`](#tracefs_magic)
  - [`V9FS_MAGIC`](#v9fs_magic)
  - [`BDEVFS_MAGIC`](#bdevfs_magic)
  - [`DAXFS_MAGIC`](#daxfs_magic)
  - [`BINFMTFS_MAGIC`](#binfmtfs_magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts_super_magic)
  - [`BINDERFS_SUPER_MAGIC`](#binderfs_super_magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs_super_magic)
  - [`PIPEFS_MAGIC`](#pipefs_magic)
  - [`PROC_SUPER_MAGIC`](#proc_super_magic)
  - [`SOCKFS_MAGIC`](#sockfs_magic)
  - [`SYSFS_MAGIC`](#sysfs_magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice_super_magic)
  - [`MTD_INODE_FS_MAGIC`](#mtd_inode_fs_magic)
  - [`ANON_INODE_FS_MAGIC`](#anon_inode_fs_magic)
  - [`BTRFS_TEST_MAGIC`](#btrfs_test_magic)
  - [`NSFS_MAGIC`](#nsfs_magic)
  - [`BPF_FS_MAGIC`](#bpf_fs_magic)
  - [`AAFS_MAGIC`](#aafs_magic)
  - [`ZONEFS_MAGIC`](#zonefs_magic)
  - [`UDF_SUPER_MAGIC`](#udf_super_magic)
  - [`DMA_BUF_MAGIC`](#dma_buf_magic)
  - [`DEVMEM_MAGIC`](#devmem_magic)
  - [`SECRETMEM_MAGIC`](#secretmem_magic)
  - [`PID_FS_MAGIC`](#pid_fs_magic)
  - [`MAP_32BIT`](#map_32bit)
  - [`MAP_ABOVE4G`](#map_above4g)
  - [`PROT_READ`](#prot_read)
  - [`PROT_WRITE`](#prot_write)
  - [`PROT_EXEC`](#prot_exec)
  - [`PROT_SEM`](#prot_sem)
  - [`PROT_NONE`](#prot_none)
  - [`PROT_GROWSDOWN`](#prot_growsdown)
  - [`PROT_GROWSUP`](#prot_growsup)
  - [`MAP_TYPE`](#map_type)
  - [`MAP_FIXED`](#map_fixed)
  - [`MAP_ANONYMOUS`](#map_anonymous)
  - [`MAP_POPULATE`](#map_populate)
  - [`MAP_NONBLOCK`](#map_nonblock)
  - [`MAP_STACK`](#map_stack)
  - [`MAP_HUGETLB`](#map_hugetlb)
  - [`MAP_SYNC`](#map_sync)
  - [`MAP_FIXED_NOREPLACE`](#map_fixed_noreplace)
  - [`MAP_UNINITIALIZED`](#map_uninitialized)
  - [`MLOCK_ONFAULT`](#mlock_onfault)
  - [`MS_ASYNC`](#ms_async)
  - [`MS_INVALIDATE`](#ms_invalidate)
  - [`MS_SYNC`](#ms_sync)
  - [`MADV_NORMAL`](#madv_normal)
  - [`MADV_RANDOM`](#madv_random)
  - [`MADV_SEQUENTIAL`](#madv_sequential)
  - [`MADV_WILLNEED`](#madv_willneed)
  - [`MADV_DONTNEED`](#madv_dontneed)
  - [`MADV_FREE`](#madv_free)
  - [`MADV_REMOVE`](#madv_remove)
  - [`MADV_DONTFORK`](#madv_dontfork)
  - [`MADV_DOFORK`](#madv_dofork)
  - [`MADV_HWPOISON`](#madv_hwpoison)
  - [`MADV_SOFT_OFFLINE`](#madv_soft_offline)
  - [`MADV_MERGEABLE`](#madv_mergeable)
  - [`MADV_UNMERGEABLE`](#madv_unmergeable)
  - [`MADV_HUGEPAGE`](#madv_hugepage)
  - [`MADV_NOHUGEPAGE`](#madv_nohugepage)
  - [`MADV_DONTDUMP`](#madv_dontdump)
  - [`MADV_DODUMP`](#madv_dodump)
  - [`MADV_WIPEONFORK`](#madv_wipeonfork)
  - [`MADV_KEEPONFORK`](#madv_keeponfork)
  - [`MADV_COLD`](#madv_cold)
  - [`MADV_PAGEOUT`](#madv_pageout)
  - [`MADV_POPULATE_READ`](#madv_populate_read)
  - [`MADV_POPULATE_WRITE`](#madv_populate_write)
  - [`MADV_DONTNEED_LOCKED`](#madv_dontneed_locked)
  - [`MADV_COLLAPSE`](#madv_collapse)
  - [`MADV_GUARD_INSTALL`](#madv_guard_install)
  - [`MADV_GUARD_REMOVE`](#madv_guard_remove)
  - [`MAP_FILE`](#map_file)
  - [`PKEY_UNRESTRICTED`](#pkey_unrestricted)
  - [`PKEY_DISABLE_ACCESS`](#pkey_disable_access)
  - [`PKEY_DISABLE_WRITE`](#pkey_disable_write)
  - [`PKEY_ACCESS_MASK`](#pkey_access_mask)
  - [`MAP_GROWSDOWN`](#map_growsdown)
  - [`MAP_DENYWRITE`](#map_denywrite)
  - [`MAP_EXECUTABLE`](#map_executable)
  - [`MAP_LOCKED`](#map_locked)
  - [`MAP_NORESERVE`](#map_noreserve)
  - [`MCL_CURRENT`](#mcl_current)
  - [`MCL_FUTURE`](#mcl_future)
  - [`MCL_ONFAULT`](#mcl_onfault)
  - [`SHADOW_STACK_SET_TOKEN`](#shadow_stack_set_token)
  - [`SHADOW_STACK_SET_MARKER`](#shadow_stack_set_marker)
  - [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb_flag_encode_shift)
  - [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb_flag_encode_mask)
  - [`HUGETLB_FLAG_ENCODE_16KB`](#hugetlb_flag_encode_16kb)
  - [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb_flag_encode_64kb)
  - [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb_flag_encode_512kb)
  - [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb_flag_encode_1mb)
  - [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb_flag_encode_2mb)
  - [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb_flag_encode_8mb)
  - [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb_flag_encode_16mb)
  - [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb_flag_encode_32mb)
  - [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb_flag_encode_256mb)
  - [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb_flag_encode_512mb)
  - [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb_flag_encode_1gb)
  - [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb_flag_encode_2gb)
  - [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb_flag_encode_16gb)
  - [`MREMAP_MAYMOVE`](#mremap_maymove)
  - [`MREMAP_FIXED`](#mremap_fixed)
  - [`MREMAP_DONTUNMAP`](#mremap_dontunmap)
  - [`OVERCOMMIT_GUESS`](#overcommit_guess)
  - [`OVERCOMMIT_ALWAYS`](#overcommit_always)
  - [`OVERCOMMIT_NEVER`](#overcommit_never)
  - [`MAP_SHARED`](#map_shared)
  - [`MAP_PRIVATE`](#map_private)
  - [`MAP_SHARED_VALIDATE`](#map_shared_validate)
  - [`MAP_DROPPABLE`](#map_droppable)
  - [`MAP_HUGE_SHIFT`](#map_huge_shift)
  - [`MAP_HUGE_MASK`](#map_huge_mask)
  - [`MAP_HUGE_16KB`](#map_huge_16kb)
  - [`MAP_HUGE_64KB`](#map_huge_64kb)
  - [`MAP_HUGE_512KB`](#map_huge_512kb)
  - [`MAP_HUGE_1MB`](#map_huge_1mb)
  - [`MAP_HUGE_2MB`](#map_huge_2mb)
  - [`MAP_HUGE_8MB`](#map_huge_8mb)
  - [`MAP_HUGE_16MB`](#map_huge_16mb)
  - [`MAP_HUGE_32MB`](#map_huge_32mb)
  - [`MAP_HUGE_256MB`](#map_huge_256mb)
  - [`MAP_HUGE_512MB`](#map_huge_512mb)
  - [`MAP_HUGE_1GB`](#map_huge_1gb)
  - [`MAP_HUGE_2GB`](#map_huge_2gb)
  - [`MAP_HUGE_16GB`](#map_huge_16gb)
  - [`POLLIN`](#pollin)
  - [`POLLPRI`](#pollpri)
  - [`POLLOUT`](#pollout)
  - [`POLLERR`](#pollerr)
  - [`POLLHUP`](#pollhup)
  - [`POLLNVAL`](#pollnval)
  - [`POLLRDNORM`](#pollrdnorm)
  - [`POLLRDBAND`](#pollrdband)
  - [`POLLWRNORM`](#pollwrnorm)
  - [`POLLWRBAND`](#pollwrband)
  - [`POLLMSG`](#pollmsg)
  - [`POLLREMOVE`](#pollremove)
  - [`POLLRDHUP`](#pollrdhup)
  - [`GRND_NONBLOCK`](#grnd_nonblock)
  - [`GRND_RANDOM`](#grnd_random)
  - [`GRND_INSECURE`](#grnd_insecure)
  - [`LINUX_REBOOT_MAGIC1`](#linux_reboot_magic1)
  - [`LINUX_REBOOT_MAGIC2`](#linux_reboot_magic2)
  - [`LINUX_REBOOT_MAGIC2A`](#linux_reboot_magic2a)
  - [`LINUX_REBOOT_MAGIC2B`](#linux_reboot_magic2b)
  - [`LINUX_REBOOT_MAGIC2C`](#linux_reboot_magic2c)
  - [`LINUX_REBOOT_CMD_RESTART`](#linux_reboot_cmd_restart)
  - [`LINUX_REBOOT_CMD_HALT`](#linux_reboot_cmd_halt)
  - [`LINUX_REBOOT_CMD_CAD_ON`](#linux_reboot_cmd_cad_on)
  - [`LINUX_REBOOT_CMD_CAD_OFF`](#linux_reboot_cmd_cad_off)
  - [`LINUX_REBOOT_CMD_POWER_OFF`](#linux_reboot_cmd_power_off)
  - [`LINUX_REBOOT_CMD_RESTART2`](#linux_reboot_cmd_restart2)
  - [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux_reboot_cmd_sw_suspend)
  - [`LINUX_REBOOT_CMD_KEXEC`](#linux_reboot_cmd_kexec)
  - [`RUSAGE_SELF`](#rusage_self)
  - [`RUSAGE_CHILDREN`](#rusage_children)
  - [`RUSAGE_BOTH`](#rusage_both)
  - [`RUSAGE_THREAD`](#rusage_thread)
  - [`RLIM64_INFINITY`](#rlim64_infinity)
  - [`PRIO_MIN`](#prio_min)
  - [`PRIO_MAX`](#prio_max)
  - [`PRIO_PROCESS`](#prio_process)
  - [`PRIO_PGRP`](#prio_pgrp)
  - [`PRIO_USER`](#prio_user)
  - [`_STK_LIM`](#_stk_lim)
  - [`MLOCK_LIMIT`](#mlock_limit)
  - [`RLIMIT_CPU`](#rlimit_cpu)
  - [`RLIMIT_FSIZE`](#rlimit_fsize)
  - [`RLIMIT_DATA`](#rlimit_data)
  - [`RLIMIT_STACK`](#rlimit_stack)
  - [`RLIMIT_CORE`](#rlimit_core)
  - [`RLIMIT_RSS`](#rlimit_rss)
  - [`RLIMIT_NPROC`](#rlimit_nproc)
  - [`RLIMIT_NOFILE`](#rlimit_nofile)
  - [`RLIMIT_MEMLOCK`](#rlimit_memlock)
  - [`RLIMIT_AS`](#rlimit_as)
  - [`RLIMIT_LOCKS`](#rlimit_locks)
  - [`RLIMIT_SIGPENDING`](#rlimit_sigpending)
  - [`RLIMIT_MSGQUEUE`](#rlimit_msgqueue)
  - [`RLIMIT_NICE`](#rlimit_nice)
  - [`RLIMIT_RTPRIO`](#rlimit_rtprio)
  - [`RLIMIT_RTTIME`](#rlimit_rttime)
  - [`RLIM_NLIMITS`](#rlim_nlimits)
  - [`RLIM_INFINITY`](#rlim_infinity)
  - [`CSIGNAL`](#csignal)
  - [`CLONE_VM`](#clone_vm)
  - [`CLONE_FS`](#clone_fs)
  - [`CLONE_FILES`](#clone_files)
  - [`CLONE_SIGHAND`](#clone_sighand)
  - [`CLONE_PIDFD`](#clone_pidfd)
  - [`CLONE_PTRACE`](#clone_ptrace)
  - [`CLONE_VFORK`](#clone_vfork)
  - [`CLONE_PARENT`](#clone_parent)
  - [`CLONE_THREAD`](#clone_thread)
  - [`CLONE_NEWNS`](#clone_newns)
  - [`CLONE_SYSVSEM`](#clone_sysvsem)
  - [`CLONE_SETTLS`](#clone_settls)
  - [`CLONE_PARENT_SETTID`](#clone_parent_settid)
  - [`CLONE_CHILD_CLEARTID`](#clone_child_cleartid)
  - [`CLONE_DETACHED`](#clone_detached)
  - [`CLONE_UNTRACED`](#clone_untraced)
  - [`CLONE_CHILD_SETTID`](#clone_child_settid)
  - [`CLONE_NEWCGROUP`](#clone_newcgroup)
  - [`CLONE_NEWUTS`](#clone_newuts)
  - [`CLONE_NEWIPC`](#clone_newipc)
  - [`CLONE_NEWUSER`](#clone_newuser)
  - [`CLONE_NEWPID`](#clone_newpid)
  - [`CLONE_NEWNET`](#clone_newnet)
  - [`CLONE_IO`](#clone_io)
  - [`CLONE_CLEAR_SIGHAND`](#clone_clear_sighand)
  - [`CLONE_INTO_CGROUP`](#clone_into_cgroup)
  - [`CLONE_NEWTIME`](#clone_newtime)
  - [`CLONE_ARGS_SIZE_VER0`](#clone_args_size_ver0)
  - [`CLONE_ARGS_SIZE_VER1`](#clone_args_size_ver1)
  - [`CLONE_ARGS_SIZE_VER2`](#clone_args_size_ver2)
  - [`SCHED_NORMAL`](#sched_normal)
  - [`SCHED_FIFO`](#sched_fifo)
  - [`SCHED_RR`](#sched_rr)
  - [`SCHED_BATCH`](#sched_batch)
  - [`SCHED_IDLE`](#sched_idle)
  - [`SCHED_DEADLINE`](#sched_deadline)
  - [`SCHED_EXT`](#sched_ext)
  - [`SCHED_RESET_ON_FORK`](#sched_reset_on_fork)
  - [`SCHED_FLAG_RESET_ON_FORK`](#sched_flag_reset_on_fork)
  - [`SCHED_FLAG_RECLAIM`](#sched_flag_reclaim)
  - [`SCHED_FLAG_DL_OVERRUN`](#sched_flag_dl_overrun)
  - [`SCHED_FLAG_KEEP_POLICY`](#sched_flag_keep_policy)
  - [`SCHED_FLAG_KEEP_PARAMS`](#sched_flag_keep_params)
  - [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched_flag_util_clamp_min)
  - [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched_flag_util_clamp_max)
  - [`SCHED_FLAG_KEEP_ALL`](#sched_flag_keep_all)
  - [`SCHED_FLAG_UTIL_CLAMP`](#sched_flag_util_clamp)
  - [`SCHED_FLAG_ALL`](#sched_flag_all)
  - [`NSIG`](#nsig)
  - [`SIGHUP`](#sighup)
  - [`SIGINT`](#sigint)
  - [`SIGQUIT`](#sigquit)
  - [`SIGILL`](#sigill)
  - [`SIGTRAP`](#sigtrap)
  - [`SIGABRT`](#sigabrt)
  - [`SIGIOT`](#sigiot)
  - [`SIGBUS`](#sigbus)
  - [`SIGFPE`](#sigfpe)
  - [`SIGKILL`](#sigkill)
  - [`SIGUSR1`](#sigusr1)
  - [`SIGSEGV`](#sigsegv)
  - [`SIGUSR2`](#sigusr2)
  - [`SIGPIPE`](#sigpipe)
  - [`SIGALRM`](#sigalrm)
  - [`SIGTERM`](#sigterm)
  - [`SIGSTKFLT`](#sigstkflt)
  - [`SIGCHLD`](#sigchld)
  - [`SIGCONT`](#sigcont)
  - [`SIGSTOP`](#sigstop)
  - [`SIGTSTP`](#sigtstp)
  - [`SIGTTIN`](#sigttin)
  - [`SIGTTOU`](#sigttou)
  - [`SIGURG`](#sigurg)
  - [`SIGXCPU`](#sigxcpu)
  - [`SIGXFSZ`](#sigxfsz)
  - [`SIGVTALRM`](#sigvtalrm)
  - [`SIGPROF`](#sigprof)
  - [`SIGWINCH`](#sigwinch)
  - [`SIGIO`](#sigio)
  - [`SIGPOLL`](#sigpoll)
  - [`SIGPWR`](#sigpwr)
  - [`SIGSYS`](#sigsys)
  - [`SIGUNUSED`](#sigunused)
  - [`SIGRTMIN`](#sigrtmin)
  - [`SA_RESTORER`](#sa_restorer)
  - [`MINSIGSTKSZ`](#minsigstksz)
  - [`SIGSTKSZ`](#sigstksz)
  - [`SA_NOCLDSTOP`](#sa_nocldstop)
  - [`SA_NOCLDWAIT`](#sa_nocldwait)
  - [`SA_SIGINFO`](#sa_siginfo)
  - [`SA_UNSUPPORTED`](#sa_unsupported)
  - [`SA_EXPOSE_TAGBITS`](#sa_expose_tagbits)
  - [`SA_ONSTACK`](#sa_onstack)
  - [`SA_RESTART`](#sa_restart)
  - [`SA_NODEFER`](#sa_nodefer)
  - [`SA_RESETHAND`](#sa_resethand)
  - [`SA_NOMASK`](#sa_nomask)
  - [`SA_ONESHOT`](#sa_oneshot)
  - [`SIG_BLOCK`](#sig_block)
  - [`SIG_UNBLOCK`](#sig_unblock)
  - [`SIG_SETMASK`](#sig_setmask)
  - [`SI_MAX_SIZE`](#si_max_size)
  - [`SI_USER`](#si_user)
  - [`SI_KERNEL`](#si_kernel)
  - [`SI_QUEUE`](#si_queue)
  - [`SI_TIMER`](#si_timer)
  - [`SI_MESGQ`](#si_mesgq)
  - [`SI_ASYNCIO`](#si_asyncio)
  - [`SI_SIGIO`](#si_sigio)
  - [`SI_TKILL`](#si_tkill)
  - [`SI_DETHREAD`](#si_dethread)
  - [`SI_ASYNCNL`](#si_asyncnl)
  - [`ILL_ILLOPC`](#ill_illopc)
  - [`ILL_ILLOPN`](#ill_illopn)
  - [`ILL_ILLADR`](#ill_illadr)
  - [`ILL_ILLTRP`](#ill_illtrp)
  - [`ILL_PRVOPC`](#ill_prvopc)
  - [`ILL_PRVREG`](#ill_prvreg)
  - [`ILL_COPROC`](#ill_coproc)
  - [`ILL_BADSTK`](#ill_badstk)
  - [`ILL_BADIADDR`](#ill_badiaddr)
  - [`__ILL_BREAK`](#__ill_break)
  - [`__ILL_BNDMOD`](#__ill_bndmod)
  - [`NSIGILL`](#nsigill)
  - [`FPE_INTDIV`](#fpe_intdiv)
  - [`FPE_INTOVF`](#fpe_intovf)
  - [`FPE_FLTDIV`](#fpe_fltdiv)
  - [`FPE_FLTOVF`](#fpe_fltovf)
  - [`FPE_FLTUND`](#fpe_fltund)
  - [`FPE_FLTRES`](#fpe_fltres)
  - [`FPE_FLTINV`](#fpe_fltinv)
  - [`FPE_FLTSUB`](#fpe_fltsub)
  - [`__FPE_DECOVF`](#__fpe_decovf)
  - [`__FPE_DECDIV`](#__fpe_decdiv)
  - [`__FPE_DECERR`](#__fpe_decerr)
  - [`__FPE_INVASC`](#__fpe_invasc)
  - [`__FPE_INVDEC`](#__fpe_invdec)
  - [`FPE_FLTUNK`](#fpe_fltunk)
  - [`FPE_CONDTRAP`](#fpe_condtrap)
  - [`NSIGFPE`](#nsigfpe)
  - [`SEGV_MAPERR`](#segv_maperr)
  - [`SEGV_ACCERR`](#segv_accerr)
  - [`SEGV_BNDERR`](#segv_bnderr)
  - [`SEGV_PKUERR`](#segv_pkuerr)
  - [`SEGV_ACCADI`](#segv_accadi)
  - [`SEGV_ADIDERR`](#segv_adiderr)
  - [`SEGV_ADIPERR`](#segv_adiperr)
  - [`SEGV_MTEAERR`](#segv_mteaerr)
  - [`SEGV_MTESERR`](#segv_mteserr)
  - [`SEGV_CPERR`](#segv_cperr)
  - [`NSIGSEGV`](#nsigsegv)
  - [`BUS_ADRALN`](#bus_adraln)
  - [`BUS_ADRERR`](#bus_adrerr)
  - [`BUS_OBJERR`](#bus_objerr)
  - [`BUS_MCEERR_AR`](#bus_mceerr_ar)
  - [`BUS_MCEERR_AO`](#bus_mceerr_ao)
  - [`NSIGBUS`](#nsigbus)
  - [`TRAP_BRKPT`](#trap_brkpt)
  - [`TRAP_TRACE`](#trap_trace)
  - [`TRAP_BRANCH`](#trap_branch)
  - [`TRAP_HWBKPT`](#trap_hwbkpt)
  - [`TRAP_UNK`](#trap_unk)
  - [`TRAP_PERF`](#trap_perf)
  - [`NSIGTRAP`](#nsigtrap)
  - [`TRAP_PERF_FLAG_ASYNC`](#trap_perf_flag_async)
  - [`CLD_EXITED`](#cld_exited)
  - [`CLD_KILLED`](#cld_killed)
  - [`CLD_DUMPED`](#cld_dumped)
  - [`CLD_TRAPPED`](#cld_trapped)
  - [`CLD_STOPPED`](#cld_stopped)
  - [`CLD_CONTINUED`](#cld_continued)
  - [`NSIGCHLD`](#nsigchld)
  - [`POLL_IN`](#poll_in)
  - [`POLL_OUT`](#poll_out)
  - [`POLL_MSG`](#poll_msg)
  - [`POLL_ERR`](#poll_err)
  - [`POLL_PRI`](#poll_pri)
  - [`POLL_HUP`](#poll_hup)
  - [`NSIGPOLL`](#nsigpoll)
  - [`SYS_SECCOMP`](#sys_seccomp)
  - [`SYS_USER_DISPATCH`](#sys_user_dispatch)
  - [`NSIGSYS`](#nsigsys)
  - [`EMT_TAGOVF`](#emt_tagovf)
  - [`NSIGEMT`](#nsigemt)
  - [`SIGEV_SIGNAL`](#sigev_signal)
  - [`SIGEV_NONE`](#sigev_none)
  - [`SIGEV_THREAD`](#sigev_thread)
  - [`SIGEV_THREAD_ID`](#sigev_thread_id)
  - [`SIGEV_MAX_SIZE`](#sigev_max_size)
  - [`SS_ONSTACK`](#ss_onstack)
  - [`SS_DISABLE`](#ss_disable)
  - [`SS_AUTODISARM`](#ss_autodisarm)
  - [`SS_FLAG_BITS`](#ss_flag_bits)
  - [`S_IFMT`](#s_ifmt)
  - [`S_IFSOCK`](#s_ifsock)
  - [`S_IFLNK`](#s_iflnk)
  - [`S_IFREG`](#s_ifreg)
  - [`S_IFBLK`](#s_ifblk)
  - [`S_IFDIR`](#s_ifdir)
  - [`S_IFCHR`](#s_ifchr)
  - [`S_IFIFO`](#s_ififo)
  - [`S_ISUID`](#s_isuid)
  - [`S_ISGID`](#s_isgid)
  - [`S_ISVTX`](#s_isvtx)
  - [`S_IRWXU`](#s_irwxu)
  - [`S_IRUSR`](#s_irusr)
  - [`S_IWUSR`](#s_iwusr)
  - [`S_IXUSR`](#s_ixusr)
  - [`S_IRWXG`](#s_irwxg)
  - [`S_IRGRP`](#s_irgrp)
  - [`S_IWGRP`](#s_iwgrp)
  - [`S_IXGRP`](#s_ixgrp)
  - [`S_IRWXO`](#s_irwxo)
  - [`S_IROTH`](#s_iroth)
  - [`S_IWOTH`](#s_iwoth)
  - [`S_IXOTH`](#s_ixoth)
  - [`STATX_TYPE`](#statx_type)
  - [`STATX_MODE`](#statx_mode)
  - [`STATX_NLINK`](#statx_nlink)
  - [`STATX_UID`](#statx_uid)
  - [`STATX_GID`](#statx_gid)
  - [`STATX_ATIME`](#statx_atime)
  - [`STATX_MTIME`](#statx_mtime)
  - [`STATX_CTIME`](#statx_ctime)
  - [`STATX_INO`](#statx_ino)
  - [`STATX_SIZE`](#statx_size)
  - [`STATX_BLOCKS`](#statx_blocks)
  - [`STATX_BASIC_STATS`](#statx_basic_stats)
  - [`STATX_BTIME`](#statx_btime)
  - [`STATX_MNT_ID`](#statx_mnt_id)
  - [`STATX_DIOALIGN`](#statx_dioalign)
  - [`STATX_MNT_ID_UNIQUE`](#statx_mnt_id_unique)
  - [`STATX_SUBVOL`](#statx_subvol)
  - [`STATX_WRITE_ATOMIC`](#statx_write_atomic)
  - [`STATX_DIO_READ_ALIGN`](#statx_dio_read_align)
  - [`STATX__RESERVED`](#statx__reserved)
  - [`STATX_ALL`](#statx_all)
  - [`STATX_ATTR_COMPRESSED`](#statx_attr_compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx_attr_immutable)
  - [`STATX_ATTR_APPEND`](#statx_attr_append)
  - [`STATX_ATTR_NODUMP`](#statx_attr_nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx_attr_encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx_attr_automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx_attr_mount_root)
  - [`STATX_ATTR_VERITY`](#statx_attr_verity)
  - [`STATX_ATTR_DAX`](#statx_attr_dax)
  - [`STATX_ATTR_WRITE_ATOMIC`](#statx_attr_write_atomic)
  - [`IGNBRK`](#ignbrk)
  - [`BRKINT`](#brkint)
  - [`IGNPAR`](#ignpar)
  - [`PARMRK`](#parmrk)
  - [`INPCK`](#inpck)
  - [`ISTRIP`](#istrip)
  - [`INLCR`](#inlcr)
  - [`IGNCR`](#igncr)
  - [`ICRNL`](#icrnl)
  - [`IXANY`](#ixany)
  - [`OPOST`](#opost)
  - [`OCRNL`](#ocrnl)
  - [`ONOCR`](#onocr)
  - [`ONLRET`](#onlret)
  - [`OFILL`](#ofill)
  - [`OFDEL`](#ofdel)
  - [`B0`](#b0)
  - [`B50`](#b50)
  - [`B75`](#b75)
  - [`B110`](#b110)
  - [`B134`](#b134)
  - [`B150`](#b150)
  - [`B200`](#b200)
  - [`B300`](#b300)
  - [`B600`](#b600)
  - [`B1200`](#b1200)
  - [`B1800`](#b1800)
  - [`B2400`](#b2400)
  - [`B4800`](#b4800)
  - [`B9600`](#b9600)
  - [`B19200`](#b19200)
  - [`B38400`](#b38400)
  - [`EXTA`](#exta)
  - [`EXTB`](#extb)
  - [`ADDRB`](#addrb)
  - [`CMSPAR`](#cmspar)
  - [`CRTSCTS`](#crtscts)
  - [`IBSHIFT`](#ibshift)
  - [`TCOOFF`](#tcooff)
  - [`TCOON`](#tcoon)
  - [`TCIOFF`](#tcioff)
  - [`TCION`](#tcion)
  - [`TCIFLUSH`](#tciflush)
  - [`TCOFLUSH`](#tcoflush)
  - [`TCIOFLUSH`](#tcioflush)
  - [`NCCS`](#nccs)
  - [`VINTR`](#vintr)
  - [`VQUIT`](#vquit)
  - [`VERASE`](#verase)
  - [`VKILL`](#vkill)
  - [`VEOF`](#veof)
  - [`VTIME`](#vtime)
  - [`VMIN`](#vmin)
  - [`VSWTC`](#vswtc)
  - [`VSTART`](#vstart)
  - [`VSTOP`](#vstop)
  - [`VSUSP`](#vsusp)
  - [`VEOL`](#veol)
  - [`VREPRINT`](#vreprint)
  - [`VDISCARD`](#vdiscard)
  - [`VWERASE`](#vwerase)
  - [`VLNEXT`](#vlnext)
  - [`VEOL2`](#veol2)
  - [`IUCLC`](#iuclc)
  - [`IXON`](#ixon)
  - [`IXOFF`](#ixoff)
  - [`IMAXBEL`](#imaxbel)
  - [`IUTF8`](#iutf8)
  - [`OLCUC`](#olcuc)
  - [`ONLCR`](#onlcr)
  - [`NLDLY`](#nldly)
  - [`NL0`](#nl0)
  - [`NL1`](#nl1)
  - [`CRDLY`](#crdly)
  - [`CR0`](#cr0)
  - [`CR1`](#cr1)
  - [`CR2`](#cr2)
  - [`CR3`](#cr3)
  - [`TABDLY`](#tabdly)
  - [`TAB0`](#tab0)
  - [`TAB1`](#tab1)
  - [`TAB2`](#tab2)
  - [`TAB3`](#tab3)
  - [`XTABS`](#xtabs)
  - [`BSDLY`](#bsdly)
  - [`BS0`](#bs0)
  - [`BS1`](#bs1)
  - [`VTDLY`](#vtdly)
  - [`VT0`](#vt0)
  - [`VT1`](#vt1)
  - [`FFDLY`](#ffdly)
  - [`FF0`](#ff0)
  - [`FF1`](#ff1)
  - [`CBAUD`](#cbaud)
  - [`CSIZE`](#csize)
  - [`CS5`](#cs5)
  - [`CS6`](#cs6)
  - [`CS7`](#cs7)
  - [`CS8`](#cs8)
  - [`CSTOPB`](#cstopb)
  - [`CREAD`](#cread)
  - [`PARENB`](#parenb)
  - [`PARODD`](#parodd)
  - [`HUPCL`](#hupcl)
  - [`CLOCAL`](#clocal)
  - [`CBAUDEX`](#cbaudex)
  - [`BOTHER`](#bother)
  - [`B57600`](#b57600)
  - [`B115200`](#b115200)
  - [`B230400`](#b230400)
  - [`B460800`](#b460800)
  - [`B500000`](#b500000)
  - [`B576000`](#b576000)
  - [`B921600`](#b921600)
  - [`B1000000`](#b1000000)
  - [`B1152000`](#b1152000)
  - [`B1500000`](#b1500000)
  - [`B2000000`](#b2000000)
  - [`B2500000`](#b2500000)
  - [`B3000000`](#b3000000)
  - [`B3500000`](#b3500000)
  - [`B4000000`](#b4000000)
  - [`CIBAUD`](#cibaud)
  - [`ISIG`](#isig)
  - [`ICANON`](#icanon)
  - [`XCASE`](#xcase)
  - [`ECHO`](#echo)
  - [`ECHOE`](#echoe)
  - [`ECHOK`](#echok)
  - [`ECHONL`](#echonl)
  - [`NOFLSH`](#noflsh)
  - [`TOSTOP`](#tostop)
  - [`ECHOCTL`](#echoctl)
  - [`ECHOPRT`](#echoprt)
  - [`ECHOKE`](#echoke)
  - [`FLUSHO`](#flusho)
  - [`PENDIN`](#pendin)
  - [`IEXTEN`](#iexten)
  - [`EXTPROC`](#extproc)
  - [`TCSANOW`](#tcsanow)
  - [`TCSADRAIN`](#tcsadrain)
  - [`TCSAFLUSH`](#tcsaflush)
  - [`TIOCPKT_DATA`](#tiocpkt_data)
  - [`TIOCPKT_FLUSHREAD`](#tiocpkt_flushread)
  - [`TIOCPKT_FLUSHWRITE`](#tiocpkt_flushwrite)
  - [`TIOCPKT_STOP`](#tiocpkt_stop)
  - [`TIOCPKT_START`](#tiocpkt_start)
  - [`TIOCPKT_NOSTOP`](#tiocpkt_nostop)
  - [`TIOCPKT_DOSTOP`](#tiocpkt_dostop)
  - [`TIOCPKT_IOCTL`](#tiocpkt_ioctl)
  - [`TIOCSER_TEMT`](#tiocser_temt)
  - [`NCC`](#ncc)
  - [`TIOCM_LE`](#tiocm_le)
  - [`TIOCM_DTR`](#tiocm_dtr)
  - [`TIOCM_RTS`](#tiocm_rts)
  - [`TIOCM_ST`](#tiocm_st)
  - [`TIOCM_SR`](#tiocm_sr)
  - [`TIOCM_CTS`](#tiocm_cts)
  - [`TIOCM_CAR`](#tiocm_car)
  - [`TIOCM_RNG`](#tiocm_rng)
  - [`TIOCM_DSR`](#tiocm_dsr)
  - [`TIOCM_CD`](#tiocm_cd)
  - [`TIOCM_RI`](#tiocm_ri)
  - [`TIOCM_OUT1`](#tiocm_out1)
  - [`TIOCM_OUT2`](#tiocm_out2)
  - [`TIOCM_LOOP`](#tiocm_loop)
  - [`ITIMER_REAL`](#itimer_real)
  - [`ITIMER_VIRTUAL`](#itimer_virtual)
  - [`ITIMER_PROF`](#itimer_prof)
  - [`CLOCK_REALTIME`](#clock_realtime)
  - [`CLOCK_MONOTONIC`](#clock_monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id)
  - [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw)
  - [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse)
  - [`CLOCK_BOOTTIME`](#clock_boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock_realtime_alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock_boottime_alarm)
  - [`CLOCK_SGI_CYCLE`](#clock_sgi_cycle)
  - [`CLOCK_TAI`](#clock_tai)
  - [`MAX_CLOCKS`](#max_clocks)
  - [`CLOCKS_MASK`](#clocks_mask)
  - [`CLOCKS_MONO`](#clocks_mono)
  - [`TIMER_ABSTIME`](#timer_abstime)
  - [`UIO_FASTIOV`](#uio_fastiov)
  - [`UIO_MAXIOV`](#uio_maxiov)
  - [`__X32_SYSCALL_BIT`](#__x32_syscall_bit)
  - [`__NR_read`](#__nr_read)
  - [`__NR_write`](#__nr_write)
  - [`__NR_open`](#__nr_open)
  - [`__NR_close`](#__nr_close)
  - [`__NR_stat`](#__nr_stat)
  - [`__NR_fstat`](#__nr_fstat)
  - [`__NR_lstat`](#__nr_lstat)
  - [`__NR_poll`](#__nr_poll)
  - [`__NR_lseek`](#__nr_lseek)
  - [`__NR_mmap`](#__nr_mmap)
  - [`__NR_mprotect`](#__nr_mprotect)
  - [`__NR_munmap`](#__nr_munmap)
  - [`__NR_brk`](#__nr_brk)
  - [`__NR_rt_sigaction`](#__nr_rt_sigaction)
  - [`__NR_rt_sigprocmask`](#__nr_rt_sigprocmask)
  - [`__NR_rt_sigreturn`](#__nr_rt_sigreturn)
  - [`__NR_ioctl`](#__nr_ioctl)
  - [`__NR_pread64`](#__nr_pread64)
  - [`__NR_pwrite64`](#__nr_pwrite64)
  - [`__NR_readv`](#__nr_readv)
  - [`__NR_writev`](#__nr_writev)
  - [`__NR_access`](#__nr_access)
  - [`__NR_pipe`](#__nr_pipe)
  - [`__NR_select`](#__nr_select)
  - [`__NR_sched_yield`](#__nr_sched_yield)
  - [`__NR_mremap`](#__nr_mremap)
  - [`__NR_msync`](#__nr_msync)
  - [`__NR_mincore`](#__nr_mincore)
  - [`__NR_madvise`](#__nr_madvise)
  - [`__NR_shmget`](#__nr_shmget)
  - [`__NR_shmat`](#__nr_shmat)
  - [`__NR_shmctl`](#__nr_shmctl)
  - [`__NR_dup`](#__nr_dup)
  - [`__NR_dup2`](#__nr_dup2)
  - [`__NR_pause`](#__nr_pause)
  - [`__NR_nanosleep`](#__nr_nanosleep)
  - [`__NR_getitimer`](#__nr_getitimer)
  - [`__NR_alarm`](#__nr_alarm)
  - [`__NR_setitimer`](#__nr_setitimer)
  - [`__NR_getpid`](#__nr_getpid)
  - [`__NR_sendfile`](#__nr_sendfile)
  - [`__NR_socket`](#__nr_socket)
  - [`__NR_connect`](#__nr_connect)
  - [`__NR_accept`](#__nr_accept)
  - [`__NR_sendto`](#__nr_sendto)
  - [`__NR_recvfrom`](#__nr_recvfrom)
  - [`__NR_sendmsg`](#__nr_sendmsg)
  - [`__NR_recvmsg`](#__nr_recvmsg)
  - [`__NR_shutdown`](#__nr_shutdown)
  - [`__NR_bind`](#__nr_bind)
  - [`__NR_listen`](#__nr_listen)
  - [`__NR_getsockname`](#__nr_getsockname)
  - [`__NR_getpeername`](#__nr_getpeername)
  - [`__NR_socketpair`](#__nr_socketpair)
  - [`__NR_setsockopt`](#__nr_setsockopt)
  - [`__NR_getsockopt`](#__nr_getsockopt)
  - [`__NR_clone`](#__nr_clone)
  - [`__NR_fork`](#__nr_fork)
  - [`__NR_vfork`](#__nr_vfork)
  - [`__NR_execve`](#__nr_execve)
  - [`__NR_exit`](#__nr_exit)
  - [`__NR_wait4`](#__nr_wait4)
  - [`__NR_kill`](#__nr_kill)
  - [`__NR_uname`](#__nr_uname)
  - [`__NR_semget`](#__nr_semget)
  - [`__NR_semop`](#__nr_semop)
  - [`__NR_semctl`](#__nr_semctl)
  - [`__NR_shmdt`](#__nr_shmdt)
  - [`__NR_msgget`](#__nr_msgget)
  - [`__NR_msgsnd`](#__nr_msgsnd)
  - [`__NR_msgrcv`](#__nr_msgrcv)
  - [`__NR_msgctl`](#__nr_msgctl)
  - [`__NR_fcntl`](#__nr_fcntl)
  - [`__NR_flock`](#__nr_flock)
  - [`__NR_fsync`](#__nr_fsync)
  - [`__NR_fdatasync`](#__nr_fdatasync)
  - [`__NR_truncate`](#__nr_truncate)
  - [`__NR_ftruncate`](#__nr_ftruncate)
  - [`__NR_getdents`](#__nr_getdents)
  - [`__NR_getcwd`](#__nr_getcwd)
  - [`__NR_chdir`](#__nr_chdir)
  - [`__NR_fchdir`](#__nr_fchdir)
  - [`__NR_rename`](#__nr_rename)
  - [`__NR_mkdir`](#__nr_mkdir)
  - [`__NR_rmdir`](#__nr_rmdir)
  - [`__NR_creat`](#__nr_creat)
  - [`__NR_link`](#__nr_link)
  - [`__NR_unlink`](#__nr_unlink)
  - [`__NR_symlink`](#__nr_symlink)
  - [`__NR_readlink`](#__nr_readlink)
  - [`__NR_chmod`](#__nr_chmod)
  - [`__NR_fchmod`](#__nr_fchmod)
  - [`__NR_chown`](#__nr_chown)
  - [`__NR_fchown`](#__nr_fchown)
  - [`__NR_lchown`](#__nr_lchown)
  - [`__NR_umask`](#__nr_umask)
  - [`__NR_gettimeofday`](#__nr_gettimeofday)
  - [`__NR_getrlimit`](#__nr_getrlimit)
  - [`__NR_getrusage`](#__nr_getrusage)
  - [`__NR_sysinfo`](#__nr_sysinfo)
  - [`__NR_times`](#__nr_times)
  - [`__NR_ptrace`](#__nr_ptrace)
  - [`__NR_getuid`](#__nr_getuid)
  - [`__NR_syslog`](#__nr_syslog)
  - [`__NR_getgid`](#__nr_getgid)
  - [`__NR_setuid`](#__nr_setuid)
  - [`__NR_setgid`](#__nr_setgid)
  - [`__NR_geteuid`](#__nr_geteuid)
  - [`__NR_getegid`](#__nr_getegid)
  - [`__NR_setpgid`](#__nr_setpgid)
  - [`__NR_getppid`](#__nr_getppid)
  - [`__NR_getpgrp`](#__nr_getpgrp)
  - [`__NR_setsid`](#__nr_setsid)
  - [`__NR_setreuid`](#__nr_setreuid)
  - [`__NR_setregid`](#__nr_setregid)
  - [`__NR_getgroups`](#__nr_getgroups)
  - [`__NR_setgroups`](#__nr_setgroups)
  - [`__NR_setresuid`](#__nr_setresuid)
  - [`__NR_getresuid`](#__nr_getresuid)
  - [`__NR_setresgid`](#__nr_setresgid)
  - [`__NR_getresgid`](#__nr_getresgid)
  - [`__NR_getpgid`](#__nr_getpgid)
  - [`__NR_setfsuid`](#__nr_setfsuid)
  - [`__NR_setfsgid`](#__nr_setfsgid)
  - [`__NR_getsid`](#__nr_getsid)
  - [`__NR_capget`](#__nr_capget)
  - [`__NR_capset`](#__nr_capset)
  - [`__NR_rt_sigpending`](#__nr_rt_sigpending)
  - [`__NR_rt_sigtimedwait`](#__nr_rt_sigtimedwait)
  - [`__NR_rt_sigqueueinfo`](#__nr_rt_sigqueueinfo)
  - [`__NR_rt_sigsuspend`](#__nr_rt_sigsuspend)
  - [`__NR_sigaltstack`](#__nr_sigaltstack)
  - [`__NR_utime`](#__nr_utime)
  - [`__NR_mknod`](#__nr_mknod)
  - [`__NR_uselib`](#__nr_uselib)
  - [`__NR_personality`](#__nr_personality)
  - [`__NR_ustat`](#__nr_ustat)
  - [`__NR_statfs`](#__nr_statfs)
  - [`__NR_fstatfs`](#__nr_fstatfs)
  - [`__NR_sysfs`](#__nr_sysfs)
  - [`__NR_getpriority`](#__nr_getpriority)
  - [`__NR_setpriority`](#__nr_setpriority)
  - [`__NR_sched_setparam`](#__nr_sched_setparam)
  - [`__NR_sched_getparam`](#__nr_sched_getparam)
  - [`__NR_sched_setscheduler`](#__nr_sched_setscheduler)
  - [`__NR_sched_getscheduler`](#__nr_sched_getscheduler)
  - [`__NR_sched_get_priority_max`](#__nr_sched_get_priority_max)
  - [`__NR_sched_get_priority_min`](#__nr_sched_get_priority_min)
  - [`__NR_sched_rr_get_interval`](#__nr_sched_rr_get_interval)
  - [`__NR_mlock`](#__nr_mlock)
  - [`__NR_munlock`](#__nr_munlock)
  - [`__NR_mlockall`](#__nr_mlockall)
  - [`__NR_munlockall`](#__nr_munlockall)
  - [`__NR_vhangup`](#__nr_vhangup)
  - [`__NR_modify_ldt`](#__nr_modify_ldt)
  - [`__NR_pivot_root`](#__nr_pivot_root)
  - [`__NR__sysctl`](#__nr__sysctl)
  - [`__NR_prctl`](#__nr_prctl)
  - [`__NR_arch_prctl`](#__nr_arch_prctl)
  - [`__NR_adjtimex`](#__nr_adjtimex)
  - [`__NR_setrlimit`](#__nr_setrlimit)
  - [`__NR_chroot`](#__nr_chroot)
  - [`__NR_sync`](#__nr_sync)
  - [`__NR_acct`](#__nr_acct)
  - [`__NR_settimeofday`](#__nr_settimeofday)
  - [`__NR_mount`](#__nr_mount)
  - [`__NR_umount2`](#__nr_umount2)
  - [`__NR_swapon`](#__nr_swapon)
  - [`__NR_swapoff`](#__nr_swapoff)
  - [`__NR_reboot`](#__nr_reboot)
  - [`__NR_sethostname`](#__nr_sethostname)
  - [`__NR_setdomainname`](#__nr_setdomainname)
  - [`__NR_iopl`](#__nr_iopl)
  - [`__NR_ioperm`](#__nr_ioperm)
  - [`__NR_create_module`](#__nr_create_module)
  - [`__NR_init_module`](#__nr_init_module)
  - [`__NR_delete_module`](#__nr_delete_module)
  - [`__NR_get_kernel_syms`](#__nr_get_kernel_syms)
  - [`__NR_query_module`](#__nr_query_module)
  - [`__NR_quotactl`](#__nr_quotactl)
  - [`__NR_nfsservctl`](#__nr_nfsservctl)
  - [`__NR_getpmsg`](#__nr_getpmsg)
  - [`__NR_putpmsg`](#__nr_putpmsg)
  - [`__NR_afs_syscall`](#__nr_afs_syscall)
  - [`__NR_tuxcall`](#__nr_tuxcall)
  - [`__NR_security`](#__nr_security)
  - [`__NR_gettid`](#__nr_gettid)
  - [`__NR_readahead`](#__nr_readahead)
  - [`__NR_setxattr`](#__nr_setxattr)
  - [`__NR_lsetxattr`](#__nr_lsetxattr)
  - [`__NR_fsetxattr`](#__nr_fsetxattr)
  - [`__NR_getxattr`](#__nr_getxattr)
  - [`__NR_lgetxattr`](#__nr_lgetxattr)
  - [`__NR_fgetxattr`](#__nr_fgetxattr)
  - [`__NR_listxattr`](#__nr_listxattr)
  - [`__NR_llistxattr`](#__nr_llistxattr)
  - [`__NR_flistxattr`](#__nr_flistxattr)
  - [`__NR_removexattr`](#__nr_removexattr)
  - [`__NR_lremovexattr`](#__nr_lremovexattr)
  - [`__NR_fremovexattr`](#__nr_fremovexattr)
  - [`__NR_tkill`](#__nr_tkill)
  - [`__NR_time`](#__nr_time)
  - [`__NR_futex`](#__nr_futex)
  - [`__NR_sched_setaffinity`](#__nr_sched_setaffinity)
  - [`__NR_sched_getaffinity`](#__nr_sched_getaffinity)
  - [`__NR_set_thread_area`](#__nr_set_thread_area)
  - [`__NR_io_setup`](#__nr_io_setup)
  - [`__NR_io_destroy`](#__nr_io_destroy)
  - [`__NR_io_getevents`](#__nr_io_getevents)
  - [`__NR_io_submit`](#__nr_io_submit)
  - [`__NR_io_cancel`](#__nr_io_cancel)
  - [`__NR_get_thread_area`](#__nr_get_thread_area)
  - [`__NR_lookup_dcookie`](#__nr_lookup_dcookie)
  - [`__NR_epoll_create`](#__nr_epoll_create)
  - [`__NR_epoll_ctl_old`](#__nr_epoll_ctl_old)
  - [`__NR_epoll_wait_old`](#__nr_epoll_wait_old)
  - [`__NR_remap_file_pages`](#__nr_remap_file_pages)
  - [`__NR_getdents64`](#__nr_getdents64)
  - [`__NR_set_tid_address`](#__nr_set_tid_address)
  - [`__NR_restart_syscall`](#__nr_restart_syscall)
  - [`__NR_semtimedop`](#__nr_semtimedop)
  - [`__NR_fadvise64`](#__nr_fadvise64)
  - [`__NR_timer_create`](#__nr_timer_create)
  - [`__NR_timer_settime`](#__nr_timer_settime)
  - [`__NR_timer_gettime`](#__nr_timer_gettime)
  - [`__NR_timer_getoverrun`](#__nr_timer_getoverrun)
  - [`__NR_timer_delete`](#__nr_timer_delete)
  - [`__NR_clock_settime`](#__nr_clock_settime)
  - [`__NR_clock_gettime`](#__nr_clock_gettime)
  - [`__NR_clock_getres`](#__nr_clock_getres)
  - [`__NR_clock_nanosleep`](#__nr_clock_nanosleep)
  - [`__NR_exit_group`](#__nr_exit_group)
  - [`__NR_epoll_wait`](#__nr_epoll_wait)
  - [`__NR_epoll_ctl`](#__nr_epoll_ctl)
  - [`__NR_tgkill`](#__nr_tgkill)
  - [`__NR_utimes`](#__nr_utimes)
  - [`__NR_vserver`](#__nr_vserver)
  - [`__NR_mbind`](#__nr_mbind)
  - [`__NR_set_mempolicy`](#__nr_set_mempolicy)
  - [`__NR_get_mempolicy`](#__nr_get_mempolicy)
  - [`__NR_mq_open`](#__nr_mq_open)
  - [`__NR_mq_unlink`](#__nr_mq_unlink)
  - [`__NR_mq_timedsend`](#__nr_mq_timedsend)
  - [`__NR_mq_timedreceive`](#__nr_mq_timedreceive)
  - [`__NR_mq_notify`](#__nr_mq_notify)
  - [`__NR_mq_getsetattr`](#__nr_mq_getsetattr)
  - [`__NR_kexec_load`](#__nr_kexec_load)
  - [`__NR_waitid`](#__nr_waitid)
  - [`__NR_add_key`](#__nr_add_key)
  - [`__NR_request_key`](#__nr_request_key)
  - [`__NR_keyctl`](#__nr_keyctl)
  - [`__NR_ioprio_set`](#__nr_ioprio_set)
  - [`__NR_ioprio_get`](#__nr_ioprio_get)
  - [`__NR_inotify_init`](#__nr_inotify_init)
  - [`__NR_inotify_add_watch`](#__nr_inotify_add_watch)
  - [`__NR_inotify_rm_watch`](#__nr_inotify_rm_watch)
  - [`__NR_migrate_pages`](#__nr_migrate_pages)
  - [`__NR_openat`](#__nr_openat)
  - [`__NR_mkdirat`](#__nr_mkdirat)
  - [`__NR_mknodat`](#__nr_mknodat)
  - [`__NR_fchownat`](#__nr_fchownat)
  - [`__NR_futimesat`](#__nr_futimesat)
  - [`__NR_newfstatat`](#__nr_newfstatat)
  - [`__NR_unlinkat`](#__nr_unlinkat)
  - [`__NR_renameat`](#__nr_renameat)
  - [`__NR_linkat`](#__nr_linkat)
  - [`__NR_symlinkat`](#__nr_symlinkat)
  - [`__NR_readlinkat`](#__nr_readlinkat)
  - [`__NR_fchmodat`](#__nr_fchmodat)
  - [`__NR_faccessat`](#__nr_faccessat)
  - [`__NR_pselect6`](#__nr_pselect6)
  - [`__NR_ppoll`](#__nr_ppoll)
  - [`__NR_unshare`](#__nr_unshare)
  - [`__NR_set_robust_list`](#__nr_set_robust_list)
  - [`__NR_get_robust_list`](#__nr_get_robust_list)
  - [`__NR_splice`](#__nr_splice)
  - [`__NR_tee`](#__nr_tee)
  - [`__NR_sync_file_range`](#__nr_sync_file_range)
  - [`__NR_vmsplice`](#__nr_vmsplice)
  - [`__NR_move_pages`](#__nr_move_pages)
  - [`__NR_utimensat`](#__nr_utimensat)
  - [`__NR_epoll_pwait`](#__nr_epoll_pwait)
  - [`__NR_signalfd`](#__nr_signalfd)
  - [`__NR_timerfd_create`](#__nr_timerfd_create)
  - [`__NR_eventfd`](#__nr_eventfd)
  - [`__NR_fallocate`](#__nr_fallocate)
  - [`__NR_timerfd_settime`](#__nr_timerfd_settime)
  - [`__NR_timerfd_gettime`](#__nr_timerfd_gettime)
  - [`__NR_accept4`](#__nr_accept4)
  - [`__NR_signalfd4`](#__nr_signalfd4)
  - [`__NR_eventfd2`](#__nr_eventfd2)
  - [`__NR_epoll_create1`](#__nr_epoll_create1)
  - [`__NR_dup3`](#__nr_dup3)
  - [`__NR_pipe2`](#__nr_pipe2)
  - [`__NR_inotify_init1`](#__nr_inotify_init1)
  - [`__NR_preadv`](#__nr_preadv)
  - [`__NR_pwritev`](#__nr_pwritev)
  - [`__NR_rt_tgsigqueueinfo`](#__nr_rt_tgsigqueueinfo)
  - [`__NR_perf_event_open`](#__nr_perf_event_open)
  - [`__NR_recvmmsg`](#__nr_recvmmsg)
  - [`__NR_fanotify_init`](#__nr_fanotify_init)
  - [`__NR_fanotify_mark`](#__nr_fanotify_mark)
  - [`__NR_prlimit64`](#__nr_prlimit64)
  - [`__NR_name_to_handle_at`](#__nr_name_to_handle_at)
  - [`__NR_open_by_handle_at`](#__nr_open_by_handle_at)
  - [`__NR_clock_adjtime`](#__nr_clock_adjtime)
  - [`__NR_syncfs`](#__nr_syncfs)
  - [`__NR_sendmmsg`](#__nr_sendmmsg)
  - [`__NR_setns`](#__nr_setns)
  - [`__NR_getcpu`](#__nr_getcpu)
  - [`__NR_process_vm_readv`](#__nr_process_vm_readv)
  - [`__NR_process_vm_writev`](#__nr_process_vm_writev)
  - [`__NR_kcmp`](#__nr_kcmp)
  - [`__NR_finit_module`](#__nr_finit_module)
  - [`__NR_sched_setattr`](#__nr_sched_setattr)
  - [`__NR_sched_getattr`](#__nr_sched_getattr)
  - [`__NR_renameat2`](#__nr_renameat2)
  - [`__NR_seccomp`](#__nr_seccomp)
  - [`__NR_getrandom`](#__nr_getrandom)
  - [`__NR_memfd_create`](#__nr_memfd_create)
  - [`__NR_kexec_file_load`](#__nr_kexec_file_load)
  - [`__NR_bpf`](#__nr_bpf)
  - [`__NR_execveat`](#__nr_execveat)
  - [`__NR_userfaultfd`](#__nr_userfaultfd)
  - [`__NR_membarrier`](#__nr_membarrier)
  - [`__NR_mlock2`](#__nr_mlock2)
  - [`__NR_copy_file_range`](#__nr_copy_file_range)
  - [`__NR_preadv2`](#__nr_preadv2)
  - [`__NR_pwritev2`](#__nr_pwritev2)
  - [`__NR_pkey_mprotect`](#__nr_pkey_mprotect)
  - [`__NR_pkey_alloc`](#__nr_pkey_alloc)
  - [`__NR_pkey_free`](#__nr_pkey_free)
  - [`__NR_statx`](#__nr_statx)
  - [`__NR_io_pgetevents`](#__nr_io_pgetevents)
  - [`__NR_rseq`](#__nr_rseq)
  - [`__NR_uretprobe`](#__nr_uretprobe)
  - [`__NR_pidfd_send_signal`](#__nr_pidfd_send_signal)
  - [`__NR_io_uring_setup`](#__nr_io_uring_setup)
  - [`__NR_io_uring_enter`](#__nr_io_uring_enter)
  - [`__NR_io_uring_register`](#__nr_io_uring_register)
  - [`__NR_open_tree`](#__nr_open_tree)
  - [`__NR_move_mount`](#__nr_move_mount)
  - [`__NR_fsopen`](#__nr_fsopen)
  - [`__NR_fsconfig`](#__nr_fsconfig)
  - [`__NR_fsmount`](#__nr_fsmount)
  - [`__NR_fspick`](#__nr_fspick)
  - [`__NR_pidfd_open`](#__nr_pidfd_open)
  - [`__NR_clone3`](#__nr_clone3)
  - [`__NR_close_range`](#__nr_close_range)
  - [`__NR_openat2`](#__nr_openat2)
  - [`__NR_pidfd_getfd`](#__nr_pidfd_getfd)
  - [`__NR_faccessat2`](#__nr_faccessat2)
  - [`__NR_process_madvise`](#__nr_process_madvise)
  - [`__NR_epoll_pwait2`](#__nr_epoll_pwait2)
  - [`__NR_mount_setattr`](#__nr_mount_setattr)
  - [`__NR_quotactl_fd`](#__nr_quotactl_fd)
  - [`__NR_landlock_create_ruleset`](#__nr_landlock_create_ruleset)
  - [`__NR_landlock_add_rule`](#__nr_landlock_add_rule)
  - [`__NR_landlock_restrict_self`](#__nr_landlock_restrict_self)
  - [`__NR_memfd_secret`](#__nr_memfd_secret)
  - [`__NR_process_mrelease`](#__nr_process_mrelease)
  - [`__NR_futex_waitv`](#__nr_futex_waitv)
  - [`__NR_set_mempolicy_home_node`](#__nr_set_mempolicy_home_node)
  - [`__NR_cachestat`](#__nr_cachestat)
  - [`__NR_fchmodat2`](#__nr_fchmodat2)
  - [`__NR_map_shadow_stack`](#__nr_map_shadow_stack)
  - [`__NR_futex_wake`](#__nr_futex_wake)
  - [`__NR_futex_wait`](#__nr_futex_wait)
  - [`__NR_futex_requeue`](#__nr_futex_requeue)
  - [`__NR_statmount`](#__nr_statmount)
  - [`__NR_listmount`](#__nr_listmount)
  - [`__NR_lsm_get_self_attr`](#__nr_lsm_get_self_attr)
  - [`__NR_lsm_set_self_attr`](#__nr_lsm_set_self_attr)
  - [`__NR_lsm_list_modules`](#__nr_lsm_list_modules)
  - [`__NR_mseal`](#__nr_mseal)
  - [`__NR_setxattrat`](#__nr_setxattrat)
  - [`__NR_getxattrat`](#__nr_getxattrat)
  - [`__NR_listxattrat`](#__nr_listxattrat)
  - [`__NR_removexattrat`](#__nr_removexattrat)
  - [`__NR_open_tree_attr`](#__nr_open_tree_attr)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`__WNOTHREAD`](#__wnothread)
  - [`__WALL`](#__wall)
  - [`__WCLONE`](#__wclone)
  - [`P_ALL`](#p_all)
  - [`P_PID`](#p_pid)
  - [`P_PGID`](#p_pgid)
  - [`P_PIDFD`](#p_pidfd)
  - [`XATTR_CREATE`](#xattr_create)
  - [`XATTR_REPLACE`](#xattr_replace)
  - [`XATTR_OS2_PREFIX`](#xattr_os2_prefix)
  - [`XATTR_MAC_OSX_PREFIX`](#xattr_mac_osx_prefix)
  - [`XATTR_BTRFS_PREFIX`](#xattr_btrfs_prefix)
  - [`XATTR_HURD_PREFIX`](#xattr_hurd_prefix)
  - [`XATTR_SECURITY_PREFIX`](#xattr_security_prefix)
  - [`XATTR_SYSTEM_PREFIX`](#xattr_system_prefix)
  - [`XATTR_TRUSTED_PREFIX`](#xattr_trusted_prefix)
  - [`XATTR_USER_PREFIX`](#xattr_user_prefix)
  - [`XATTR_EVM_SUFFIX`](#xattr_evm_suffix)
  - [`XATTR_NAME_EVM`](#xattr_name_evm)
  - [`XATTR_IMA_SUFFIX`](#xattr_ima_suffix)
  - [`XATTR_NAME_IMA`](#xattr_name_ima)
  - [`XATTR_SELINUX_SUFFIX`](#xattr_selinux_suffix)
  - [`XATTR_NAME_SELINUX`](#xattr_name_selinux)
  - [`XATTR_SMACK_SUFFIX`](#xattr_smack_suffix)
  - [`XATTR_SMACK_IPIN`](#xattr_smack_ipin)
  - [`XATTR_SMACK_IPOUT`](#xattr_smack_ipout)
  - [`XATTR_SMACK_EXEC`](#xattr_smack_exec)
  - [`XATTR_SMACK_TRANSMUTE`](#xattr_smack_transmute)
  - [`XATTR_SMACK_MMAP`](#xattr_smack_mmap)
  - [`XATTR_NAME_SMACK`](#xattr_name_smack)
  - [`XATTR_NAME_SMACKIPIN`](#xattr_name_smackipin)
  - [`XATTR_NAME_SMACKIPOUT`](#xattr_name_smackipout)
  - [`XATTR_NAME_SMACKEXEC`](#xattr_name_smackexec)
  - [`XATTR_NAME_SMACKTRANSMUTE`](#xattr_name_smacktransmute)
  - [`XATTR_NAME_SMACKMMAP`](#xattr_name_smackmmap)
  - [`XATTR_APPARMOR_SUFFIX`](#xattr_apparmor_suffix)
  - [`XATTR_NAME_APPARMOR`](#xattr_name_apparmor)
  - [`XATTR_CAPS_SUFFIX`](#xattr_caps_suffix)
  - [`XATTR_NAME_CAPS`](#xattr_name_caps)
  - [`XATTR_BPF_LSM_SUFFIX`](#xattr_bpf_lsm_suffix)
  - [`XATTR_NAME_BPF_LSM`](#xattr_name_bpf_lsm)
  - [`XATTR_POSIX_ACL_ACCESS`](#xattr_posix_acl_access)
  - [`XATTR_NAME_POSIX_ACL_ACCESS`](#xattr_name_posix_acl_access)
  - [`XATTR_POSIX_ACL_DEFAULT`](#xattr_posix_acl_default)
  - [`XATTR_NAME_POSIX_ACL_DEFAULT`](#xattr_name_posix_acl_default)
  - [`MFD_CLOEXEC`](#mfd_cloexec)
  - [`MFD_ALLOW_SEALING`](#mfd_allow_sealing)
  - [`MFD_HUGETLB`](#mfd_hugetlb)
  - [`MFD_NOEXEC_SEAL`](#mfd_noexec_seal)
  - [`MFD_EXEC`](#mfd_exec)
  - [`MFD_HUGE_SHIFT`](#mfd_huge_shift)
  - [`MFD_HUGE_MASK`](#mfd_huge_mask)
  - [`MFD_HUGE_64KB`](#mfd_huge_64kb)
  - [`MFD_HUGE_512KB`](#mfd_huge_512kb)
  - [`MFD_HUGE_1MB`](#mfd_huge_1mb)
  - [`MFD_HUGE_2MB`](#mfd_huge_2mb)
  - [`MFD_HUGE_8MB`](#mfd_huge_8mb)
  - [`MFD_HUGE_16MB`](#mfd_huge_16mb)
  - [`MFD_HUGE_32MB`](#mfd_huge_32mb)
  - [`MFD_HUGE_256MB`](#mfd_huge_256mb)
  - [`MFD_HUGE_512MB`](#mfd_huge_512mb)
  - [`MFD_HUGE_1GB`](#mfd_huge_1gb)
  - [`MFD_HUGE_2GB`](#mfd_huge_2gb)
  - [`MFD_HUGE_16GB`](#mfd_huge_16gb)
  - [`TFD_TIMER_ABSTIME`](#tfd_timer_abstime)
  - [`TFD_TIMER_CANCEL_ON_SET`](#tfd_timer_cancel_on_set)
  - [`TFD_CLOEXEC`](#tfd_cloexec)
  - [`TFD_NONBLOCK`](#tfd_nonblock)
  - [`USERFAULTFD_IOC`](#userfaultfd_ioc)
  - [`_UFFDIO_REGISTER`](#_uffdio_register)
  - [`_UFFDIO_UNREGISTER`](#_uffdio_unregister)
  - [`_UFFDIO_WAKE`](#_uffdio_wake)
  - [`_UFFDIO_COPY`](#_uffdio_copy)
  - [`_UFFDIO_ZEROPAGE`](#_uffdio_zeropage)
  - [`_UFFDIO_MOVE`](#_uffdio_move)
  - [`_UFFDIO_WRITEPROTECT`](#_uffdio_writeprotect)
  - [`_UFFDIO_CONTINUE`](#_uffdio_continue)
  - [`_UFFDIO_POISON`](#_uffdio_poison)
  - [`_UFFDIO_API`](#_uffdio_api)
  - [`UFFDIO`](#uffdio)
  - [`UFFD_EVENT_PAGEFAULT`](#uffd_event_pagefault)
  - [`UFFD_EVENT_FORK`](#uffd_event_fork)
  - [`UFFD_EVENT_REMAP`](#uffd_event_remap)
  - [`UFFD_EVENT_REMOVE`](#uffd_event_remove)
  - [`UFFD_EVENT_UNMAP`](#uffd_event_unmap)
  - [`UFFD_PAGEFAULT_FLAG_WRITE`](#uffd_pagefault_flag_write)
  - [`UFFD_PAGEFAULT_FLAG_WP`](#uffd_pagefault_flag_wp)
  - [`UFFD_PAGEFAULT_FLAG_MINOR`](#uffd_pagefault_flag_minor)
  - [`UFFD_FEATURE_PAGEFAULT_FLAG_WP`](#uffd_feature_pagefault_flag_wp)
  - [`UFFD_FEATURE_EVENT_FORK`](#uffd_feature_event_fork)
  - [`UFFD_FEATURE_EVENT_REMAP`](#uffd_feature_event_remap)
  - [`UFFD_FEATURE_EVENT_REMOVE`](#uffd_feature_event_remove)
  - [`UFFD_FEATURE_MISSING_HUGETLBFS`](#uffd_feature_missing_hugetlbfs)
  - [`UFFD_FEATURE_MISSING_SHMEM`](#uffd_feature_missing_shmem)
  - [`UFFD_FEATURE_EVENT_UNMAP`](#uffd_feature_event_unmap)
  - [`UFFD_FEATURE_SIGBUS`](#uffd_feature_sigbus)
  - [`UFFD_FEATURE_THREAD_ID`](#uffd_feature_thread_id)
  - [`UFFD_FEATURE_MINOR_HUGETLBFS`](#uffd_feature_minor_hugetlbfs)
  - [`UFFD_FEATURE_MINOR_SHMEM`](#uffd_feature_minor_shmem)
  - [`UFFD_FEATURE_EXACT_ADDRESS`](#uffd_feature_exact_address)
  - [`UFFD_FEATURE_WP_HUGETLBFS_SHMEM`](#uffd_feature_wp_hugetlbfs_shmem)
  - [`UFFD_FEATURE_WP_UNPOPULATED`](#uffd_feature_wp_unpopulated)
  - [`UFFD_FEATURE_POISON`](#uffd_feature_poison)
  - [`UFFD_FEATURE_WP_ASYNC`](#uffd_feature_wp_async)
  - [`UFFD_FEATURE_MOVE`](#uffd_feature_move)
  - [`UFFD_USER_MODE_ONLY`](#uffd_user_mode_only)
  - [`DT_UNKNOWN`](#dt_unknown)
  - [`DT_FIFO`](#dt_fifo)
  - [`DT_CHR`](#dt_chr)
  - [`DT_DIR`](#dt_dir)
  - [`DT_BLK`](#dt_blk)
  - [`DT_REG`](#dt_reg)
  - [`DT_LNK`](#dt_lnk)
  - [`DT_SOCK`](#dt_sock)
  - [`STAT_HAVE_NSEC`](#stat_have_nsec)
  - [`F_OK`](#f_ok)
  - [`R_OK`](#r_ok)
  - [`W_OK`](#w_ok)
  - [`X_OK`](#x_ok)
  - [`UTIME_NOW`](#utime_now)
  - [`UTIME_OMIT`](#utime_omit)
  - [`MNT_FORCE`](#mnt_force)
  - [`MNT_DETACH`](#mnt_detach)
  - [`MNT_EXPIRE`](#mnt_expire)
  - [`UMOUNT_NOFOLLOW`](#umount_nofollow)
  - [`UMOUNT_UNUSED`](#umount_unused)
  - [`STDIN_FILENO`](#stdin_fileno)
  - [`STDOUT_FILENO`](#stdout_fileno)
  - [`STDERR_FILENO`](#stderr_fileno)
  - [`RWF_HIPRI`](#rwf_hipri)
  - [`RWF_DSYNC`](#rwf_dsync)
  - [`RWF_SYNC`](#rwf_sync)
  - [`RWF_NOWAIT`](#rwf_nowait)
  - [`RWF_APPEND`](#rwf_append)
  - [`EFD_SEMAPHORE`](#efd_semaphore)
  - [`EFD_CLOEXEC`](#efd_cloexec)
  - [`EFD_NONBLOCK`](#efd_nonblock)
  - [`EPOLLIN`](#epollin)
  - [`EPOLLPRI`](#epollpri)
  - [`EPOLLOUT`](#epollout)
  - [`EPOLLERR`](#epollerr)
  - [`EPOLLHUP`](#epollhup)
  - [`EPOLLNVAL`](#epollnval)
  - [`EPOLLRDNORM`](#epollrdnorm)
  - [`EPOLLRDBAND`](#epollrdband)
  - [`EPOLLWRNORM`](#epollwrnorm)
  - [`EPOLLWRBAND`](#epollwrband)
  - [`EPOLLMSG`](#epollmsg)
  - [`EPOLLRDHUP`](#epollrdhup)
  - [`EPOLLEXCLUSIVE`](#epollexclusive)
  - [`EPOLLWAKEUP`](#epollwakeup)
  - [`EPOLLONESHOT`](#epolloneshot)
  - [`EPOLLET`](#epollet)
  - [`TFD_SHARED_FCNTL_FLAGS`](#tfd_shared_fcntl_flags)
  - [`TFD_CREATE_FLAGS`](#tfd_create_flags)
  - [`TFD_SETTIME_FLAGS`](#tfd_settime_flags)
  - [`ARCH_SET_FS`](#arch_set_fs)
  - [`UFFD_API`](#uffd_api)
  - [`UFFDIO_REGISTER_MODE_MISSING`](#uffdio_register_mode_missing)
  - [`UFFDIO_REGISTER_MODE_WP`](#uffdio_register_mode_wp)
  - [`UFFDIO_REGISTER_MODE_MINOR`](#uffdio_register_mode_minor)
  - [`UFFDIO_COPY_MODE_DONTWAKE`](#uffdio_copy_mode_dontwake)
  - [`UFFDIO_COPY_MODE_WP`](#uffdio_copy_mode_wp)
  - [`UFFDIO_ZEROPAGE_MODE_DONTWAKE`](#uffdio_zeropage_mode_dontwake)
  - [`SPLICE_F_MOVE`](#splice_f_move)
  - [`SPLICE_F_NONBLOCK`](#splice_f_nonblock)
  - [`SPLICE_F_MORE`](#splice_f_more)
  - [`SPLICE_F_GIFT`](#splice_f_gift)
  - [`_NSIG`](#_nsig)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`__BindgenBitfieldUnit`](#__bindgenbitfieldunit) | struct |  |
| [`__IncompleteArrayField`](#__incompletearrayfield) | struct |  |
| [`__kernel_fd_set`](#__kernel_fd_set) | struct |  |
| [`__kernel_fsid_t`](#__kernel_fsid_t) | struct |  |
| [`__user_cap_header_struct`](#__user_cap_header_struct) | struct |  |
| [`__user_cap_data_struct`](#__user_cap_data_struct) | struct |  |
| [`vfs_cap_data`](#vfs_cap_data) | struct |  |
| [`vfs_cap_data__bindgen_ty_1`](#vfs_cap_data__bindgen_ty_1) | struct |  |
| [`vfs_ns_cap_data`](#vfs_ns_cap_data) | struct |  |
| [`vfs_ns_cap_data__bindgen_ty_1`](#vfs_ns_cap_data__bindgen_ty_1) | struct |  |
| [`f_owner_ex`](#f_owner_ex) | struct |  |
| [`flock`](#flock) | struct |  |
| [`flock64`](#flock64) | struct |  |
| [`open_how`](#open_how) | struct |  |
| [`epoll_event`](#epoll_event) | struct |  |
| [`epoll_params`](#epoll_params) | struct |  |
| [`fscrypt_policy_v1`](#fscrypt_policy_v1) | struct |  |
| [`fscrypt_key`](#fscrypt_key) | struct |  |
| [`fscrypt_policy_v2`](#fscrypt_policy_v2) | struct |  |
| [`fscrypt_get_policy_ex_arg`](#fscrypt_get_policy_ex_arg) | struct |  |
| [`fscrypt_key_specifier`](#fscrypt_key_specifier) | struct |  |
| [`fscrypt_provisioning_key_payload`](#fscrypt_provisioning_key_payload) | struct |  |
| [`fscrypt_add_key_arg`](#fscrypt_add_key_arg) | struct |  |
| [`fscrypt_remove_key_arg`](#fscrypt_remove_key_arg) | struct |  |
| [`fscrypt_get_key_status_arg`](#fscrypt_get_key_status_arg) | struct |  |
| [`mount_attr`](#mount_attr) | struct |  |
| [`statmount`](#statmount) | struct |  |
| [`mnt_id_req`](#mnt_id_req) | struct |  |
| [`file_clone_range`](#file_clone_range) | struct |  |
| [`fstrim_range`](#fstrim_range) | struct |  |
| [`fsuuid2`](#fsuuid2) | struct |  |
| [`fs_sysfs_path`](#fs_sysfs_path) | struct |  |
| [`file_dedupe_range_info`](#file_dedupe_range_info) | struct |  |
| [`file_dedupe_range`](#file_dedupe_range) | struct |  |
| [`files_stat_struct`](#files_stat_struct) | struct |  |
| [`inodes_stat_t`](#inodes_stat_t) | struct |  |
| [`fsxattr`](#fsxattr) | struct |  |
| [`page_region`](#page_region) | struct |  |
| [`pm_scan_arg`](#pm_scan_arg) | struct |  |
| [`procmap_query`](#procmap_query) | struct |  |
| [`futex_waitv`](#futex_waitv) | struct |  |
| [`robust_list`](#robust_list) | struct |  |
| [`robust_list_head`](#robust_list_head) | struct |  |
| [`inotify_event`](#inotify_event) | struct |  |
| [`cachestat_range`](#cachestat_range) | struct |  |
| [`cachestat`](#cachestat) | struct |  |
| [`pollfd`](#pollfd) | struct |  |
| [`rand_pool_info`](#rand_pool_info) | struct |  |
| [`vgetrandom_opaque_params`](#vgetrandom_opaque_params) | struct |  |
| [`__kernel_timespec`](#__kernel_timespec) | struct |  |
| [`__kernel_itimerspec`](#__kernel_itimerspec) | struct |  |
| [`__kernel_old_timeval`](#__kernel_old_timeval) | struct |  |
| [`__kernel_old_timespec`](#__kernel_old_timespec) | struct |  |
| [`__kernel_old_itimerval`](#__kernel_old_itimerval) | struct |  |
| [`__kernel_sock_timeval`](#__kernel_sock_timeval) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rlimit64`](#rlimit64) | struct |  |
| [`clone_args`](#clone_args) | struct |  |
| [`sigaction`](#sigaction) | struct |  |
| [`sigaltstack`](#sigaltstack) | struct |  |
| [`__sifields__bindgen_ty_1`](#__sifields__bindgen_ty_1) | struct |  |
| [`__sifields__bindgen_ty_2`](#__sifields__bindgen_ty_2) | struct |  |
| [`__sifields__bindgen_ty_3`](#__sifields__bindgen_ty_3) | struct |  |
| [`__sifields__bindgen_ty_4`](#__sifields__bindgen_ty_4) | struct |  |
| [`__sifields__bindgen_ty_5`](#__sifields__bindgen_ty_5) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3) | struct |  |
| [`__sifields__bindgen_ty_6`](#__sifields__bindgen_ty_6) | struct |  |
| [`__sifields__bindgen_ty_7`](#__sifields__bindgen_ty_7) | struct |  |
| [`siginfo`](#siginfo) | struct |  |
| [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo__bindgen_ty_1__bindgen_ty_1) | struct |  |
| [`sigevent`](#sigevent) | struct |  |
| [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent__bindgen_ty_1__bindgen_ty_1) | struct |  |
| [`statx_timestamp`](#statx_timestamp) | struct |  |
| [`statx`](#statx) | struct |  |
| [`termios`](#termios) | struct |  |
| [`termios2`](#termios2) | struct |  |
| [`ktermios`](#ktermios) | struct |  |
| [`winsize`](#winsize) | struct |  |
| [`termio`](#termio) | struct |  |
| [`timespec`](#timespec) | struct |  |
| [`timeval`](#timeval) | struct |  |
| [`itimerspec`](#itimerspec) | struct |  |
| [`itimerval`](#itimerval) | struct |  |
| [`timezone`](#timezone) | struct |  |
| [`iovec`](#iovec) | struct |  |
| [`dmabuf_cmsg`](#dmabuf_cmsg) | struct |  |
| [`dmabuf_token`](#dmabuf_token) | struct |  |
| [`xattr_args`](#xattr_args) | struct |  |
| [`uffd_msg`](#uffd_msg) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd_msg__bindgen_ty_1__bindgen_ty_1) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd_msg__bindgen_ty_1__bindgen_ty_2) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd_msg__bindgen_ty_1__bindgen_ty_3) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd_msg__bindgen_ty_1__bindgen_ty_4) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd_msg__bindgen_ty_1__bindgen_ty_5) | struct |  |
| [`uffdio_api`](#uffdio_api) | struct |  |
| [`uffdio_range`](#uffdio_range) | struct |  |
| [`uffdio_register`](#uffdio_register) | struct |  |
| [`uffdio_copy`](#uffdio_copy) | struct |  |
| [`uffdio_zeropage`](#uffdio_zeropage) | struct |  |
| [`uffdio_writeprotect`](#uffdio_writeprotect) | struct |  |
| [`uffdio_continue`](#uffdio_continue) | struct |  |
| [`uffdio_poison`](#uffdio_poison) | struct |  |
| [`uffdio_move`](#uffdio_move) | struct |  |
| [`linux_dirent64`](#linux_dirent64) | struct |  |
| [`stat`](#stat) | struct |  |
| [`__old_kernel_stat`](#__old_kernel_stat) | struct |  |
| [`statfs`](#statfs) | struct |  |
| [`statfs64`](#statfs64) | struct |  |
| [`compat_statfs64`](#compat_statfs64) | struct |  |
| [`user_desc`](#user_desc) | struct |  |
| [`kernel_sigset_t`](#kernel_sigset_t) | struct |  |
| [`kernel_sigaction`](#kernel_sigaction) | struct |  |
| [`fsconfig_command`](#fsconfig_command) | enum |  |
| [`procmap_query_flags`](#procmap_query_flags) | enum |  |
| [`membarrier_cmd`](#membarrier_cmd) | enum |  |
| [`membarrier_cmd_flag`](#membarrier_cmd_flag) | enum |  |
| [`__s8`](#__s8) | type |  |
| [`__u8`](#__u8) | type |  |
| [`__s16`](#__s16) | type |  |
| [`__u16`](#__u16) | type |  |
| [`__s32`](#__s32) | type |  |
| [`__u32`](#__u32) | type |  |
| [`__s64`](#__s64) | type |  |
| [`__u64`](#__u64) | type |  |
| [`__kernel_sighandler_t`](#__kernel_sighandler_t) | type |  |
| [`__kernel_key_t`](#__kernel_key_t) | type |  |
| [`__kernel_mqd_t`](#__kernel_mqd_t) | type |  |
| [`__kernel_old_uid_t`](#__kernel_old_uid_t) | type |  |
| [`__kernel_old_gid_t`](#__kernel_old_gid_t) | type |  |
| [`__kernel_old_dev_t`](#__kernel_old_dev_t) | type |  |
| [`__kernel_long_t`](#__kernel_long_t) | type |  |
| [`__kernel_ulong_t`](#__kernel_ulong_t) | type |  |
| [`__kernel_ino_t`](#__kernel_ino_t) | type |  |
| [`__kernel_mode_t`](#__kernel_mode_t) | type |  |
| [`__kernel_pid_t`](#__kernel_pid_t) | type |  |
| [`__kernel_ipc_pid_t`](#__kernel_ipc_pid_t) | type |  |
| [`__kernel_uid_t`](#__kernel_uid_t) | type |  |
| [`__kernel_gid_t`](#__kernel_gid_t) | type |  |
| [`__kernel_suseconds_t`](#__kernel_suseconds_t) | type |  |
| [`__kernel_daddr_t`](#__kernel_daddr_t) | type |  |
| [`__kernel_uid32_t`](#__kernel_uid32_t) | type |  |
| [`__kernel_gid32_t`](#__kernel_gid32_t) | type |  |
| [`__kernel_size_t`](#__kernel_size_t) | type |  |
| [`__kernel_ssize_t`](#__kernel_ssize_t) | type |  |
| [`__kernel_ptrdiff_t`](#__kernel_ptrdiff_t) | type |  |
| [`__kernel_off_t`](#__kernel_off_t) | type |  |
| [`__kernel_loff_t`](#__kernel_loff_t) | type |  |
| [`__kernel_old_time_t`](#__kernel_old_time_t) | type |  |
| [`__kernel_time_t`](#__kernel_time_t) | type |  |
| [`__kernel_time64_t`](#__kernel_time64_t) | type |  |
| [`__kernel_clock_t`](#__kernel_clock_t) | type |  |
| [`__kernel_timer_t`](#__kernel_timer_t) | type |  |
| [`__kernel_clockid_t`](#__kernel_clockid_t) | type |  |
| [`__kernel_caddr_t`](#__kernel_caddr_t) | type |  |
| [`__kernel_uid16_t`](#__kernel_uid16_t) | type |  |
| [`__kernel_gid16_t`](#__kernel_gid16_t) | type |  |
| [`__s128`](#__s128) | type |  |
| [`__u128`](#__u128) | type |  |
| [`__le16`](#__le16) | type |  |
| [`__be16`](#__be16) | type |  |
| [`__le32`](#__le32) | type |  |
| [`__be32`](#__be32) | type |  |
| [`__le64`](#__le64) | type |  |
| [`__be64`](#__be64) | type |  |
| [`__sum16`](#__sum16) | type |  |
| [`__wsum`](#__wsum) | type |  |
| [`__poll_t`](#__poll_t) | type |  |
| [`cap_user_header_t`](#cap_user_header_t) | type |  |
| [`cap_user_data_t`](#cap_user_data_t) | type |  |
| [`__kernel_rwf_t`](#__kernel_rwf_t) | type |  |
| [`sigset_t`](#sigset_t) | type |  |
| [`__signalfn_t`](#__signalfn_t) | type |  |
| [`__sighandler_t`](#__sighandler_t) | type |  |
| [`__restorefn_t`](#__restorefn_t) | type |  |
| [`__sigrestore_t`](#__sigrestore_t) | type |  |
| [`stack_t`](#stack_t) | type |  |
| [`sigval_t`](#sigval_t) | type |  |
| [`siginfo_t`](#siginfo_t) | type |  |
| [`sigevent_t`](#sigevent_t) | type |  |
| [`cc_t`](#cc_t) | type |  |
| [`speed_t`](#speed_t) | type |  |
| [`tcflag_t`](#tcflag_t) | type |  |
| [`__fsword_t`](#__fsword_t) | type |  |
| [`LINUX_VERSION_CODE`](#linux_version_code) | const |  |
| [`LINUX_VERSION_MAJOR`](#linux_version_major) | const |  |
| [`LINUX_VERSION_PATCHLEVEL`](#linux_version_patchlevel) | const |  |
| [`LINUX_VERSION_SUBLEVEL`](#linux_version_sublevel) | const |  |
| [`__BITS_PER_LONG_LONG`](#__bits_per_long_long) | const |  |
| [`__FD_SETSIZE`](#__fd_setsize) | const |  |
| [`_LINUX_CAPABILITY_VERSION_1`](#_linux_capability_version_1) | const |  |
| [`_LINUX_CAPABILITY_U32S_1`](#_linux_capability_u32s_1) | const |  |
| [`_LINUX_CAPABILITY_VERSION_2`](#_linux_capability_version_2) | const |  |
| [`_LINUX_CAPABILITY_U32S_2`](#_linux_capability_u32s_2) | const |  |
| [`_LINUX_CAPABILITY_VERSION_3`](#_linux_capability_version_3) | const |  |
| [`_LINUX_CAPABILITY_U32S_3`](#_linux_capability_u32s_3) | const |  |
| [`VFS_CAP_REVISION_MASK`](#vfs_cap_revision_mask) | const |  |
| [`VFS_CAP_REVISION_SHIFT`](#vfs_cap_revision_shift) | const |  |
| [`VFS_CAP_FLAGS_MASK`](#vfs_cap_flags_mask) | const |  |
| [`VFS_CAP_FLAGS_EFFECTIVE`](#vfs_cap_flags_effective) | const |  |
| [`VFS_CAP_REVISION_1`](#vfs_cap_revision_1) | const |  |
| [`VFS_CAP_U32_1`](#vfs_cap_u32_1) | const |  |
| [`VFS_CAP_REVISION_2`](#vfs_cap_revision_2) | const |  |
| [`VFS_CAP_U32_2`](#vfs_cap_u32_2) | const |  |
| [`VFS_CAP_REVISION_3`](#vfs_cap_revision_3) | const |  |
| [`VFS_CAP_U32_3`](#vfs_cap_u32_3) | const |  |
| [`VFS_CAP_U32`](#vfs_cap_u32) | const |  |
| [`VFS_CAP_REVISION`](#vfs_cap_revision) | const |  |
| [`_LINUX_CAPABILITY_VERSION`](#_linux_capability_version) | const |  |
| [`_LINUX_CAPABILITY_U32S`](#_linux_capability_u32s) | const |  |
| [`CAP_CHOWN`](#cap_chown) | const |  |
| [`CAP_DAC_OVERRIDE`](#cap_dac_override) | const |  |
| [`CAP_DAC_READ_SEARCH`](#cap_dac_read_search) | const |  |
| [`CAP_FOWNER`](#cap_fowner) | const |  |
| [`CAP_FSETID`](#cap_fsetid) | const |  |
| [`CAP_KILL`](#cap_kill) | const |  |
| [`CAP_SETGID`](#cap_setgid) | const |  |
| [`CAP_SETUID`](#cap_setuid) | const |  |
| [`CAP_SETPCAP`](#cap_setpcap) | const |  |
| [`CAP_LINUX_IMMUTABLE`](#cap_linux_immutable) | const |  |
| [`CAP_NET_BIND_SERVICE`](#cap_net_bind_service) | const |  |
| [`CAP_NET_BROADCAST`](#cap_net_broadcast) | const |  |
| [`CAP_NET_ADMIN`](#cap_net_admin) | const |  |
| [`CAP_NET_RAW`](#cap_net_raw) | const |  |
| [`CAP_IPC_LOCK`](#cap_ipc_lock) | const |  |
| [`CAP_IPC_OWNER`](#cap_ipc_owner) | const |  |
| [`CAP_SYS_MODULE`](#cap_sys_module) | const |  |
| [`CAP_SYS_RAWIO`](#cap_sys_rawio) | const |  |
| [`CAP_SYS_CHROOT`](#cap_sys_chroot) | const |  |
| [`CAP_SYS_PTRACE`](#cap_sys_ptrace) | const |  |
| [`CAP_SYS_PACCT`](#cap_sys_pacct) | const |  |
| [`CAP_SYS_ADMIN`](#cap_sys_admin) | const |  |
| [`CAP_SYS_BOOT`](#cap_sys_boot) | const |  |
| [`CAP_SYS_NICE`](#cap_sys_nice) | const |  |
| [`CAP_SYS_RESOURCE`](#cap_sys_resource) | const |  |
| [`CAP_SYS_TIME`](#cap_sys_time) | const |  |
| [`CAP_SYS_TTY_CONFIG`](#cap_sys_tty_config) | const |  |
| [`CAP_MKNOD`](#cap_mknod) | const |  |
| [`CAP_LEASE`](#cap_lease) | const |  |
| [`CAP_AUDIT_WRITE`](#cap_audit_write) | const |  |
| [`CAP_AUDIT_CONTROL`](#cap_audit_control) | const |  |
| [`CAP_SETFCAP`](#cap_setfcap) | const |  |
| [`CAP_MAC_OVERRIDE`](#cap_mac_override) | const |  |
| [`CAP_MAC_ADMIN`](#cap_mac_admin) | const |  |
| [`CAP_SYSLOG`](#cap_syslog) | const |  |
| [`CAP_WAKE_ALARM`](#cap_wake_alarm) | const |  |
| [`CAP_BLOCK_SUSPEND`](#cap_block_suspend) | const |  |
| [`CAP_AUDIT_READ`](#cap_audit_read) | const |  |
| [`CAP_PERFMON`](#cap_perfmon) | const |  |
| [`CAP_BPF`](#cap_bpf) | const |  |
| [`CAP_CHECKPOINT_RESTORE`](#cap_checkpoint_restore) | const |  |
| [`CAP_LAST_CAP`](#cap_last_cap) | const |  |
| [`O_ACCMODE`](#o_accmode) | const |  |
| [`O_RDONLY`](#o_rdonly) | const |  |
| [`O_WRONLY`](#o_wronly) | const |  |
| [`O_RDWR`](#o_rdwr) | const |  |
| [`O_CREAT`](#o_creat) | const |  |
| [`O_EXCL`](#o_excl) | const |  |
| [`O_NOCTTY`](#o_noctty) | const |  |
| [`O_TRUNC`](#o_trunc) | const |  |
| [`O_APPEND`](#o_append) | const |  |
| [`O_NONBLOCK`](#o_nonblock) | const |  |
| [`O_DSYNC`](#o_dsync) | const |  |
| [`FASYNC`](#fasync) | const |  |
| [`O_DIRECT`](#o_direct) | const |  |
| [`O_LARGEFILE`](#o_largefile) | const |  |
| [`O_DIRECTORY`](#o_directory) | const |  |
| [`O_NOFOLLOW`](#o_nofollow) | const |  |
| [`O_NOATIME`](#o_noatime) | const |  |
| [`O_CLOEXEC`](#o_cloexec) | const |  |
| [`__O_SYNC`](#__o_sync) | const |  |
| [`O_SYNC`](#o_sync) | const |  |
| [`O_PATH`](#o_path) | const |  |
| [`__O_TMPFILE`](#__o_tmpfile) | const |  |
| [`O_TMPFILE`](#o_tmpfile) | const |  |
| [`O_NDELAY`](#o_ndelay) | const |  |
| [`F_DUPFD`](#f_dupfd) | const |  |
| [`F_GETFD`](#f_getfd) | const |  |
| [`F_SETFD`](#f_setfd) | const |  |
| [`F_GETFL`](#f_getfl) | const |  |
| [`F_SETFL`](#f_setfl) | const |  |
| [`F_GETLK`](#f_getlk) | const |  |
| [`F_SETLK`](#f_setlk) | const |  |
| [`F_SETLKW`](#f_setlkw) | const |  |
| [`F_SETOWN`](#f_setown) | const |  |
| [`F_GETOWN`](#f_getown) | const |  |
| [`F_SETSIG`](#f_setsig) | const |  |
| [`F_GETSIG`](#f_getsig) | const |  |
| [`F_SETOWN_EX`](#f_setown_ex) | const |  |
| [`F_GETOWN_EX`](#f_getown_ex) | const |  |
| [`F_GETOWNER_UIDS`](#f_getowner_uids) | const |  |
| [`F_OFD_GETLK`](#f_ofd_getlk) | const |  |
| [`F_OFD_SETLK`](#f_ofd_setlk) | const |  |
| [`F_OFD_SETLKW`](#f_ofd_setlkw) | const |  |
| [`F_OWNER_TID`](#f_owner_tid) | const |  |
| [`F_OWNER_PID`](#f_owner_pid) | const |  |
| [`F_OWNER_PGRP`](#f_owner_pgrp) | const |  |
| [`FD_CLOEXEC`](#fd_cloexec) | const |  |
| [`F_RDLCK`](#f_rdlck) | const |  |
| [`F_WRLCK`](#f_wrlck) | const |  |
| [`F_UNLCK`](#f_unlck) | const |  |
| [`F_EXLCK`](#f_exlck) | const |  |
| [`F_SHLCK`](#f_shlck) | const |  |
| [`LOCK_SH`](#lock_sh) | const |  |
| [`LOCK_EX`](#lock_ex) | const |  |
| [`LOCK_NB`](#lock_nb) | const |  |
| [`LOCK_UN`](#lock_un) | const |  |
| [`LOCK_MAND`](#lock_mand) | const |  |
| [`LOCK_READ`](#lock_read) | const |  |
| [`LOCK_WRITE`](#lock_write) | const |  |
| [`LOCK_RW`](#lock_rw) | const |  |
| [`F_LINUX_SPECIFIC_BASE`](#f_linux_specific_base) | const |  |
| [`RESOLVE_NO_XDEV`](#resolve_no_xdev) | const |  |
| [`RESOLVE_NO_MAGICLINKS`](#resolve_no_magiclinks) | const |  |
| [`RESOLVE_NO_SYMLINKS`](#resolve_no_symlinks) | const |  |
| [`RESOLVE_BENEATH`](#resolve_beneath) | const |  |
| [`RESOLVE_IN_ROOT`](#resolve_in_root) | const |  |
| [`RESOLVE_CACHED`](#resolve_cached) | const |  |
| [`F_SETLEASE`](#f_setlease) | const |  |
| [`F_GETLEASE`](#f_getlease) | const |  |
| [`F_NOTIFY`](#f_notify) | const |  |
| [`F_DUPFD_QUERY`](#f_dupfd_query) | const |  |
| [`F_CREATED_QUERY`](#f_created_query) | const |  |
| [`F_CANCELLK`](#f_cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f_dupfd_cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f_setpipe_sz) | const |  |
| [`F_GETPIPE_SZ`](#f_getpipe_sz) | const |  |
| [`F_ADD_SEALS`](#f_add_seals) | const |  |
| [`F_GET_SEALS`](#f_get_seals) | const |  |
| [`F_SEAL_SEAL`](#f_seal_seal) | const |  |
| [`F_SEAL_SHRINK`](#f_seal_shrink) | const |  |
| [`F_SEAL_GROW`](#f_seal_grow) | const |  |
| [`F_SEAL_WRITE`](#f_seal_write) | const |  |
| [`F_SEAL_FUTURE_WRITE`](#f_seal_future_write) | const |  |
| [`F_SEAL_EXEC`](#f_seal_exec) | const |  |
| [`F_GET_RW_HINT`](#f_get_rw_hint) | const |  |
| [`F_SET_RW_HINT`](#f_set_rw_hint) | const |  |
| [`F_GET_FILE_RW_HINT`](#f_get_file_rw_hint) | const |  |
| [`F_SET_FILE_RW_HINT`](#f_set_file_rw_hint) | const |  |
| [`RWH_WRITE_LIFE_NOT_SET`](#rwh_write_life_not_set) | const |  |
| [`RWH_WRITE_LIFE_NONE`](#rwh_write_life_none) | const |  |
| [`RWH_WRITE_LIFE_SHORT`](#rwh_write_life_short) | const |  |
| [`RWH_WRITE_LIFE_MEDIUM`](#rwh_write_life_medium) | const |  |
| [`RWH_WRITE_LIFE_LONG`](#rwh_write_life_long) | const |  |
| [`RWH_WRITE_LIFE_EXTREME`](#rwh_write_life_extreme) | const |  |
| [`RWF_WRITE_LIFE_NOT_SET`](#rwf_write_life_not_set) | const |  |
| [`DN_ACCESS`](#dn_access) | const |  |
| [`DN_MODIFY`](#dn_modify) | const |  |
| [`DN_CREATE`](#dn_create) | const |  |
| [`DN_DELETE`](#dn_delete) | const |  |
| [`DN_RENAME`](#dn_rename) | const |  |
| [`DN_ATTRIB`](#dn_attrib) | const |  |
| [`DN_MULTISHOT`](#dn_multishot) | const |  |
| [`AT_FDCWD`](#at_fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at_symlink_nofollow) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at_symlink_follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at_no_automount) | const |  |
| [`AT_EMPTY_PATH`](#at_empty_path) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at_statx_sync_type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at_statx_sync_as_stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at_statx_force_sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at_statx_dont_sync) | const |  |
| [`AT_RECURSIVE`](#at_recursive) | const |  |
| [`AT_RENAME_NOREPLACE`](#at_rename_noreplace) | const |  |
| [`AT_RENAME_EXCHANGE`](#at_rename_exchange) | const |  |
| [`AT_RENAME_WHITEOUT`](#at_rename_whiteout) | const |  |
| [`AT_EACCESS`](#at_eaccess) | const |  |
| [`AT_REMOVEDIR`](#at_removedir) | const |  |
| [`AT_HANDLE_FID`](#at_handle_fid) | const |  |
| [`AT_HANDLE_MNT_ID_UNIQUE`](#at_handle_mnt_id_unique) | const |  |
| [`AT_HANDLE_CONNECTABLE`](#at_handle_connectable) | const |  |
| [`AT_EXECVE_CHECK`](#at_execve_check) | const |  |
| [`EPOLL_CLOEXEC`](#epoll_cloexec) | const |  |
| [`EPOLL_CTL_ADD`](#epoll_ctl_add) | const |  |
| [`EPOLL_CTL_DEL`](#epoll_ctl_del) | const |  |
| [`EPOLL_CTL_MOD`](#epoll_ctl_mod) | const |  |
| [`EPOLL_IOC_TYPE`](#epoll_ioc_type) | const |  |
| [`POSIX_FADV_NORMAL`](#posix_fadv_normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix_fadv_random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix_fadv_sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix_fadv_willneed) | const |  |
| [`POSIX_FADV_DONTNEED`](#posix_fadv_dontneed) | const |  |
| [`POSIX_FADV_NOREUSE`](#posix_fadv_noreuse) | const |  |
| [`FALLOC_FL_ALLOCATE_RANGE`](#falloc_fl_allocate_range) | const |  |
| [`FALLOC_FL_KEEP_SIZE`](#falloc_fl_keep_size) | const |  |
| [`FALLOC_FL_PUNCH_HOLE`](#falloc_fl_punch_hole) | const |  |
| [`FALLOC_FL_NO_HIDE_STALE`](#falloc_fl_no_hide_stale) | const |  |
| [`FALLOC_FL_COLLAPSE_RANGE`](#falloc_fl_collapse_range) | const |  |
| [`FALLOC_FL_ZERO_RANGE`](#falloc_fl_zero_range) | const |  |
| [`FALLOC_FL_INSERT_RANGE`](#falloc_fl_insert_range) | const |  |
| [`FALLOC_FL_UNSHARE_RANGE`](#falloc_fl_unshare_range) | const |  |
| [`NR_OPEN`](#nr_open) | const |  |
| [`NGROUPS_MAX`](#ngroups_max) | const |  |
| [`ARG_MAX`](#arg_max) | const |  |
| [`LINK_MAX`](#link_max) | const |  |
| [`MAX_CANON`](#max_canon) | const |  |
| [`MAX_INPUT`](#max_input) | const |  |
| [`NAME_MAX`](#name_max) | const |  |
| [`PATH_MAX`](#path_max) | const |  |
| [`PIPE_BUF`](#pipe_buf) | const |  |
| [`XATTR_NAME_MAX`](#xattr_name_max) | const |  |
| [`XATTR_SIZE_MAX`](#xattr_size_max) | const |  |
| [`XATTR_LIST_MAX`](#xattr_list_max) | const |  |
| [`RTSIG_MAX`](#rtsig_max) | const |  |
| [`_IOC_NRBITS`](#_ioc_nrbits) | const |  |
| [`_IOC_TYPEBITS`](#_ioc_typebits) | const |  |
| [`_IOC_SIZEBITS`](#_ioc_sizebits) | const |  |
| [`_IOC_DIRBITS`](#_ioc_dirbits) | const |  |
| [`_IOC_NRMASK`](#_ioc_nrmask) | const |  |
| [`_IOC_TYPEMASK`](#_ioc_typemask) | const |  |
| [`_IOC_SIZEMASK`](#_ioc_sizemask) | const |  |
| [`_IOC_DIRMASK`](#_ioc_dirmask) | const |  |
| [`_IOC_NRSHIFT`](#_ioc_nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#_ioc_typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#_ioc_sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#_ioc_dirshift) | const |  |
| [`_IOC_NONE`](#_ioc_none) | const |  |
| [`_IOC_WRITE`](#_ioc_write) | const |  |
| [`_IOC_READ`](#_ioc_read) | const |  |
| [`IOC_IN`](#ioc_in) | const |  |
| [`IOC_OUT`](#ioc_out) | const |  |
| [`IOC_INOUT`](#ioc_inout) | const |  |
| [`IOCSIZE_MASK`](#iocsize_mask) | const |  |
| [`IOCSIZE_SHIFT`](#iocsize_shift) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_4`](#fscrypt_policy_flags_pad_4) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_8`](#fscrypt_policy_flags_pad_8) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_16`](#fscrypt_policy_flags_pad_16) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_32`](#fscrypt_policy_flags_pad_32) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_MASK`](#fscrypt_policy_flags_pad_mask) | const |  |
| [`FSCRYPT_POLICY_FLAG_DIRECT_KEY`](#fscrypt_policy_flag_direct_key) | const |  |
| [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`](#fscrypt_policy_flag_iv_ino_lblk_64) | const |  |
| [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`](#fscrypt_policy_flag_iv_ino_lblk_32) | const |  |
| [`FSCRYPT_MODE_AES_256_XTS`](#fscrypt_mode_aes_256_xts) | const |  |
| [`FSCRYPT_MODE_AES_256_CTS`](#fscrypt_mode_aes_256_cts) | const |  |
| [`FSCRYPT_MODE_AES_128_CBC`](#fscrypt_mode_aes_128_cbc) | const |  |
| [`FSCRYPT_MODE_AES_128_CTS`](#fscrypt_mode_aes_128_cts) | const |  |
| [`FSCRYPT_MODE_SM4_XTS`](#fscrypt_mode_sm4_xts) | const |  |
| [`FSCRYPT_MODE_SM4_CTS`](#fscrypt_mode_sm4_cts) | const |  |
| [`FSCRYPT_MODE_ADIANTUM`](#fscrypt_mode_adiantum) | const |  |
| [`FSCRYPT_MODE_AES_256_HCTR2`](#fscrypt_mode_aes_256_hctr2) | const |  |
| [`FSCRYPT_POLICY_V1`](#fscrypt_policy_v1) | const |  |
| [`FSCRYPT_KEY_DESCRIPTOR_SIZE`](#fscrypt_key_descriptor_size) | const |  |
| [`FSCRYPT_KEY_DESC_PREFIX`](#fscrypt_key_desc_prefix) | const |  |
| [`FSCRYPT_KEY_DESC_PREFIX_SIZE`](#fscrypt_key_desc_prefix_size) | const |  |
| [`FSCRYPT_MAX_KEY_SIZE`](#fscrypt_max_key_size) | const |  |
| [`FSCRYPT_POLICY_V2`](#fscrypt_policy_v2) | const |  |
| [`FSCRYPT_KEY_IDENTIFIER_SIZE`](#fscrypt_key_identifier_size) | const |  |
| [`FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`](#fscrypt_key_spec_type_descriptor) | const |  |
| [`FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`](#fscrypt_key_spec_type_identifier) | const |  |
| [`FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`](#fscrypt_add_key_flag_hw_wrapped) | const |  |
| [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`](#fscrypt_key_removal_status_flag_files_busy) | const |  |
| [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`](#fscrypt_key_removal_status_flag_other_users) | const |  |
| [`FSCRYPT_KEY_STATUS_ABSENT`](#fscrypt_key_status_absent) | const |  |
| [`FSCRYPT_KEY_STATUS_PRESENT`](#fscrypt_key_status_present) | const |  |
| [`FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`](#fscrypt_key_status_incompletely_removed) | const |  |
| [`FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`](#fscrypt_key_status_flag_added_by_self) | const |  |
| [`FS_KEY_DESCRIPTOR_SIZE`](#fs_key_descriptor_size) | const |  |
| [`FS_POLICY_FLAGS_PAD_4`](#fs_policy_flags_pad_4) | const |  |
| [`FS_POLICY_FLAGS_PAD_8`](#fs_policy_flags_pad_8) | const |  |
| [`FS_POLICY_FLAGS_PAD_16`](#fs_policy_flags_pad_16) | const |  |
| [`FS_POLICY_FLAGS_PAD_32`](#fs_policy_flags_pad_32) | const |  |
| [`FS_POLICY_FLAGS_PAD_MASK`](#fs_policy_flags_pad_mask) | const |  |
| [`FS_POLICY_FLAG_DIRECT_KEY`](#fs_policy_flag_direct_key) | const |  |
| [`FS_POLICY_FLAGS_VALID`](#fs_policy_flags_valid) | const |  |
| [`FS_ENCRYPTION_MODE_INVALID`](#fs_encryption_mode_invalid) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_XTS`](#fs_encryption_mode_aes_256_xts) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_GCM`](#fs_encryption_mode_aes_256_gcm) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_CBC`](#fs_encryption_mode_aes_256_cbc) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_CTS`](#fs_encryption_mode_aes_256_cts) | const |  |
| [`FS_ENCRYPTION_MODE_AES_128_CBC`](#fs_encryption_mode_aes_128_cbc) | const |  |
| [`FS_ENCRYPTION_MODE_AES_128_CTS`](#fs_encryption_mode_aes_128_cts) | const |  |
| [`FS_ENCRYPTION_MODE_ADIANTUM`](#fs_encryption_mode_adiantum) | const |  |
| [`FS_KEY_DESC_PREFIX`](#fs_key_desc_prefix) | const |  |
| [`FS_KEY_DESC_PREFIX_SIZE`](#fs_key_desc_prefix_size) | const |  |
| [`FS_MAX_KEY_SIZE`](#fs_max_key_size) | const |  |
| [`MS_RDONLY`](#ms_rdonly) | const |  |
| [`MS_NOSUID`](#ms_nosuid) | const |  |
| [`MS_NODEV`](#ms_nodev) | const |  |
| [`MS_NOEXEC`](#ms_noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms_synchronous) | const |  |
| [`MS_REMOUNT`](#ms_remount) | const |  |
| [`MS_MANDLOCK`](#ms_mandlock) | const |  |
| [`MS_DIRSYNC`](#ms_dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms_nosymfollow) | const |  |
| [`MS_NOATIME`](#ms_noatime) | const |  |
| [`MS_NODIRATIME`](#ms_nodiratime) | const |  |
| [`MS_BIND`](#ms_bind) | const |  |
| [`MS_MOVE`](#ms_move) | const |  |
| [`MS_REC`](#ms_rec) | const |  |
| [`MS_VERBOSE`](#ms_verbose) | const |  |
| [`MS_SILENT`](#ms_silent) | const |  |
| [`MS_POSIXACL`](#ms_posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms_unbindable) | const |  |
| [`MS_PRIVATE`](#ms_private) | const |  |
| [`MS_SLAVE`](#ms_slave) | const |  |
| [`MS_SHARED`](#ms_shared) | const |  |
| [`MS_RELATIME`](#ms_relatime) | const |  |
| [`MS_KERNMOUNT`](#ms_kernmount) | const |  |
| [`MS_I_VERSION`](#ms_i_version) | const |  |
| [`MS_STRICTATIME`](#ms_strictatime) | const |  |
| [`MS_LAZYTIME`](#ms_lazytime) | const |  |
| [`MS_SUBMOUNT`](#ms_submount) | const |  |
| [`MS_NOREMOTELOCK`](#ms_noremotelock) | const |  |
| [`MS_NOSEC`](#ms_nosec) | const |  |
| [`MS_BORN`](#ms_born) | const |  |
| [`MS_ACTIVE`](#ms_active) | const |  |
| [`MS_NOUSER`](#ms_nouser) | const |  |
| [`MS_RMT_MASK`](#ms_rmt_mask) | const |  |
| [`MS_MGC_VAL`](#ms_mgc_val) | const |  |
| [`MS_MGC_MSK`](#ms_mgc_msk) | const |  |
| [`OPEN_TREE_CLONE`](#open_tree_clone) | const |  |
| [`OPEN_TREE_CLOEXEC`](#open_tree_cloexec) | const |  |
| [`MOVE_MOUNT_F_SYMLINKS`](#move_mount_f_symlinks) | const |  |
| [`MOVE_MOUNT_F_AUTOMOUNTS`](#move_mount_f_automounts) | const |  |
| [`MOVE_MOUNT_F_EMPTY_PATH`](#move_mount_f_empty_path) | const |  |
| [`MOVE_MOUNT_T_SYMLINKS`](#move_mount_t_symlinks) | const |  |
| [`MOVE_MOUNT_T_AUTOMOUNTS`](#move_mount_t_automounts) | const |  |
| [`MOVE_MOUNT_T_EMPTY_PATH`](#move_mount_t_empty_path) | const |  |
| [`MOVE_MOUNT_SET_GROUP`](#move_mount_set_group) | const |  |
| [`MOVE_MOUNT_BENEATH`](#move_mount_beneath) | const |  |
| [`MOVE_MOUNT__MASK`](#move_mount__mask) | const |  |
| [`FSOPEN_CLOEXEC`](#fsopen_cloexec) | const |  |
| [`FSPICK_CLOEXEC`](#fspick_cloexec) | const |  |
| [`FSPICK_SYMLINK_NOFOLLOW`](#fspick_symlink_nofollow) | const |  |
| [`FSPICK_NO_AUTOMOUNT`](#fspick_no_automount) | const |  |
| [`FSPICK_EMPTY_PATH`](#fspick_empty_path) | const |  |
| [`FSMOUNT_CLOEXEC`](#fsmount_cloexec) | const |  |
| [`MOUNT_ATTR_RDONLY`](#mount_attr_rdonly) | const |  |
| [`MOUNT_ATTR_NOSUID`](#mount_attr_nosuid) | const |  |
| [`MOUNT_ATTR_NODEV`](#mount_attr_nodev) | const |  |
| [`MOUNT_ATTR_NOEXEC`](#mount_attr_noexec) | const |  |
| [`MOUNT_ATTR__ATIME`](#mount_attr__atime) | const |  |
| [`MOUNT_ATTR_RELATIME`](#mount_attr_relatime) | const |  |
| [`MOUNT_ATTR_NOATIME`](#mount_attr_noatime) | const |  |
| [`MOUNT_ATTR_STRICTATIME`](#mount_attr_strictatime) | const |  |
| [`MOUNT_ATTR_NODIRATIME`](#mount_attr_nodiratime) | const |  |
| [`MOUNT_ATTR_IDMAP`](#mount_attr_idmap) | const |  |
| [`MOUNT_ATTR_NOSYMFOLLOW`](#mount_attr_nosymfollow) | const |  |
| [`MOUNT_ATTR_SIZE_VER0`](#mount_attr_size_ver0) | const |  |
| [`MNT_ID_REQ_SIZE_VER0`](#mnt_id_req_size_ver0) | const |  |
| [`MNT_ID_REQ_SIZE_VER1`](#mnt_id_req_size_ver1) | const |  |
| [`STATMOUNT_SB_BASIC`](#statmount_sb_basic) | const |  |
| [`STATMOUNT_MNT_BASIC`](#statmount_mnt_basic) | const |  |
| [`STATMOUNT_PROPAGATE_FROM`](#statmount_propagate_from) | const |  |
| [`STATMOUNT_MNT_ROOT`](#statmount_mnt_root) | const |  |
| [`STATMOUNT_MNT_POINT`](#statmount_mnt_point) | const |  |
| [`STATMOUNT_FS_TYPE`](#statmount_fs_type) | const |  |
| [`STATMOUNT_MNT_NS_ID`](#statmount_mnt_ns_id) | const |  |
| [`STATMOUNT_MNT_OPTS`](#statmount_mnt_opts) | const |  |
| [`STATMOUNT_FS_SUBTYPE`](#statmount_fs_subtype) | const |  |
| [`STATMOUNT_SB_SOURCE`](#statmount_sb_source) | const |  |
| [`STATMOUNT_OPT_ARRAY`](#statmount_opt_array) | const |  |
| [`STATMOUNT_OPT_SEC_ARRAY`](#statmount_opt_sec_array) | const |  |
| [`STATMOUNT_SUPPORTED_MASK`](#statmount_supported_mask) | const |  |
| [`STATMOUNT_MNT_UIDMAP`](#statmount_mnt_uidmap) | const |  |
| [`STATMOUNT_MNT_GIDMAP`](#statmount_mnt_gidmap) | const |  |
| [`LSMT_ROOT`](#lsmt_root) | const |  |
| [`LISTMOUNT_REVERSE`](#listmount_reverse) | const |  |
| [`INR_OPEN_CUR`](#inr_open_cur) | const |  |
| [`INR_OPEN_MAX`](#inr_open_max) | const |  |
| [`BLOCK_SIZE_BITS`](#block_size_bits) | const |  |
| [`BLOCK_SIZE`](#block_size) | const |  |
| [`IO_INTEGRITY_CHK_GUARD`](#io_integrity_chk_guard) | const |  |
| [`IO_INTEGRITY_CHK_REFTAG`](#io_integrity_chk_reftag) | const |  |
| [`IO_INTEGRITY_CHK_APPTAG`](#io_integrity_chk_apptag) | const |  |
| [`IO_INTEGRITY_VALID_FLAGS`](#io_integrity_valid_flags) | const |  |
| [`SEEK_SET`](#seek_set) | const |  |
| [`SEEK_CUR`](#seek_cur) | const |  |
| [`SEEK_END`](#seek_end) | const |  |
| [`SEEK_DATA`](#seek_data) | const |  |
| [`SEEK_HOLE`](#seek_hole) | const |  |
| [`SEEK_MAX`](#seek_max) | const |  |
| [`RENAME_NOREPLACE`](#rename_noreplace) | const |  |
| [`RENAME_EXCHANGE`](#rename_exchange) | const |  |
| [`RENAME_WHITEOUT`](#rename_whiteout) | const |  |
| [`FILE_DEDUPE_RANGE_SAME`](#file_dedupe_range_same) | const |  |
| [`FILE_DEDUPE_RANGE_DIFFERS`](#file_dedupe_range_differs) | const |  |
| [`NR_FILE`](#nr_file) | const |  |
| [`FS_XFLAG_REALTIME`](#fs_xflag_realtime) | const |  |
| [`FS_XFLAG_PREALLOC`](#fs_xflag_prealloc) | const |  |
| [`FS_XFLAG_IMMUTABLE`](#fs_xflag_immutable) | const |  |
| [`FS_XFLAG_APPEND`](#fs_xflag_append) | const |  |
| [`FS_XFLAG_SYNC`](#fs_xflag_sync) | const |  |
| [`FS_XFLAG_NOATIME`](#fs_xflag_noatime) | const |  |
| [`FS_XFLAG_NODUMP`](#fs_xflag_nodump) | const |  |
| [`FS_XFLAG_RTINHERIT`](#fs_xflag_rtinherit) | const |  |
| [`FS_XFLAG_PROJINHERIT`](#fs_xflag_projinherit) | const |  |
| [`FS_XFLAG_NOSYMLINKS`](#fs_xflag_nosymlinks) | const |  |
| [`FS_XFLAG_EXTSIZE`](#fs_xflag_extsize) | const |  |
| [`FS_XFLAG_EXTSZINHERIT`](#fs_xflag_extszinherit) | const |  |
| [`FS_XFLAG_NODEFRAG`](#fs_xflag_nodefrag) | const |  |
| [`FS_XFLAG_FILESTREAM`](#fs_xflag_filestream) | const |  |
| [`FS_XFLAG_DAX`](#fs_xflag_dax) | const |  |
| [`FS_XFLAG_COWEXTSIZE`](#fs_xflag_cowextsize) | const |  |
| [`FS_XFLAG_HASATTR`](#fs_xflag_hasattr) | const |  |
| [`BMAP_IOCTL`](#bmap_ioctl) | const |  |
| [`FSLABEL_MAX`](#fslabel_max) | const |  |
| [`FS_SECRM_FL`](#fs_secrm_fl) | const |  |
| [`FS_UNRM_FL`](#fs_unrm_fl) | const |  |
| [`FS_COMPR_FL`](#fs_compr_fl) | const |  |
| [`FS_SYNC_FL`](#fs_sync_fl) | const |  |
| [`FS_IMMUTABLE_FL`](#fs_immutable_fl) | const |  |
| [`FS_APPEND_FL`](#fs_append_fl) | const |  |
| [`FS_NODUMP_FL`](#fs_nodump_fl) | const |  |
| [`FS_NOATIME_FL`](#fs_noatime_fl) | const |  |
| [`FS_DIRTY_FL`](#fs_dirty_fl) | const |  |
| [`FS_COMPRBLK_FL`](#fs_comprblk_fl) | const |  |
| [`FS_NOCOMP_FL`](#fs_nocomp_fl) | const |  |
| [`FS_ENCRYPT_FL`](#fs_encrypt_fl) | const |  |
| [`FS_BTREE_FL`](#fs_btree_fl) | const |  |
| [`FS_INDEX_FL`](#fs_index_fl) | const |  |
| [`FS_IMAGIC_FL`](#fs_imagic_fl) | const |  |
| [`FS_JOURNAL_DATA_FL`](#fs_journal_data_fl) | const |  |
| [`FS_NOTAIL_FL`](#fs_notail_fl) | const |  |
| [`FS_DIRSYNC_FL`](#fs_dirsync_fl) | const |  |
| [`FS_TOPDIR_FL`](#fs_topdir_fl) | const |  |
| [`FS_HUGE_FILE_FL`](#fs_huge_file_fl) | const |  |
| [`FS_EXTENT_FL`](#fs_extent_fl) | const |  |
| [`FS_VERITY_FL`](#fs_verity_fl) | const |  |
| [`FS_EA_INODE_FL`](#fs_ea_inode_fl) | const |  |
| [`FS_EOFBLOCKS_FL`](#fs_eofblocks_fl) | const |  |
| [`FS_NOCOW_FL`](#fs_nocow_fl) | const |  |
| [`FS_DAX_FL`](#fs_dax_fl) | const |  |
| [`FS_INLINE_DATA_FL`](#fs_inline_data_fl) | const |  |
| [`FS_PROJINHERIT_FL`](#fs_projinherit_fl) | const |  |
| [`FS_CASEFOLD_FL`](#fs_casefold_fl) | const |  |
| [`FS_RESERVED_FL`](#fs_reserved_fl) | const |  |
| [`FS_FL_USER_VISIBLE`](#fs_fl_user_visible) | const |  |
| [`FS_FL_USER_MODIFIABLE`](#fs_fl_user_modifiable) | const |  |
| [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync_file_range_wait_before) | const |  |
| [`SYNC_FILE_RANGE_WRITE`](#sync_file_range_write) | const |  |
| [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync_file_range_wait_after) | const |  |
| [`SYNC_FILE_RANGE_WRITE_AND_WAIT`](#sync_file_range_write_and_wait) | const |  |
| [`PROCFS_IOCTL_MAGIC`](#procfs_ioctl_magic) | const |  |
| [`PAGE_IS_WPALLOWED`](#page_is_wpallowed) | const |  |
| [`PAGE_IS_WRITTEN`](#page_is_written) | const |  |
| [`PAGE_IS_FILE`](#page_is_file) | const |  |
| [`PAGE_IS_PRESENT`](#page_is_present) | const |  |
| [`PAGE_IS_SWAPPED`](#page_is_swapped) | const |  |
| [`PAGE_IS_PFNZERO`](#page_is_pfnzero) | const |  |
| [`PAGE_IS_HUGE`](#page_is_huge) | const |  |
| [`PAGE_IS_SOFT_DIRTY`](#page_is_soft_dirty) | const |  |
| [`PAGE_IS_GUARD`](#page_is_guard) | const |  |
| [`PM_SCAN_WP_MATCHING`](#pm_scan_wp_matching) | const |  |
| [`PM_SCAN_CHECK_WPASYNC`](#pm_scan_check_wpasync) | const |  |
| [`FUTEX_WAIT`](#futex_wait) | const |  |
| [`FUTEX_WAKE`](#futex_wake) | const |  |
| [`FUTEX_FD`](#futex_fd) | const |  |
| [`FUTEX_REQUEUE`](#futex_requeue) | const |  |
| [`FUTEX_CMP_REQUEUE`](#futex_cmp_requeue) | const |  |
| [`FUTEX_WAKE_OP`](#futex_wake_op) | const |  |
| [`FUTEX_LOCK_PI`](#futex_lock_pi) | const |  |
| [`FUTEX_UNLOCK_PI`](#futex_unlock_pi) | const |  |
| [`FUTEX_TRYLOCK_PI`](#futex_trylock_pi) | const |  |
| [`FUTEX_WAIT_BITSET`](#futex_wait_bitset) | const |  |
| [`FUTEX_WAKE_BITSET`](#futex_wake_bitset) | const |  |
| [`FUTEX_WAIT_REQUEUE_PI`](#futex_wait_requeue_pi) | const |  |
| [`FUTEX_CMP_REQUEUE_PI`](#futex_cmp_requeue_pi) | const |  |
| [`FUTEX_LOCK_PI2`](#futex_lock_pi2) | const |  |
| [`FUTEX_PRIVATE_FLAG`](#futex_private_flag) | const |  |
| [`FUTEX_CLOCK_REALTIME`](#futex_clock_realtime) | const |  |
| [`FUTEX_CMD_MASK`](#futex_cmd_mask) | const |  |
| [`FUTEX_WAIT_PRIVATE`](#futex_wait_private) | const |  |
| [`FUTEX_WAKE_PRIVATE`](#futex_wake_private) | const |  |
| [`FUTEX_REQUEUE_PRIVATE`](#futex_requeue_private) | const |  |
| [`FUTEX_CMP_REQUEUE_PRIVATE`](#futex_cmp_requeue_private) | const |  |
| [`FUTEX_WAKE_OP_PRIVATE`](#futex_wake_op_private) | const |  |
| [`FUTEX_LOCK_PI_PRIVATE`](#futex_lock_pi_private) | const |  |
| [`FUTEX_LOCK_PI2_PRIVATE`](#futex_lock_pi2_private) | const |  |
| [`FUTEX_UNLOCK_PI_PRIVATE`](#futex_unlock_pi_private) | const |  |
| [`FUTEX_TRYLOCK_PI_PRIVATE`](#futex_trylock_pi_private) | const |  |
| [`FUTEX_WAIT_BITSET_PRIVATE`](#futex_wait_bitset_private) | const |  |
| [`FUTEX_WAKE_BITSET_PRIVATE`](#futex_wake_bitset_private) | const |  |
| [`FUTEX_WAIT_REQUEUE_PI_PRIVATE`](#futex_wait_requeue_pi_private) | const |  |
| [`FUTEX_CMP_REQUEUE_PI_PRIVATE`](#futex_cmp_requeue_pi_private) | const |  |
| [`FUTEX2_SIZE_U8`](#futex2_size_u8) | const |  |
| [`FUTEX2_SIZE_U16`](#futex2_size_u16) | const |  |
| [`FUTEX2_SIZE_U32`](#futex2_size_u32) | const |  |
| [`FUTEX2_SIZE_U64`](#futex2_size_u64) | const |  |
| [`FUTEX2_NUMA`](#futex2_numa) | const |  |
| [`FUTEX2_MPOL`](#futex2_mpol) | const |  |
| [`FUTEX2_PRIVATE`](#futex2_private) | const |  |
| [`FUTEX2_SIZE_MASK`](#futex2_size_mask) | const |  |
| [`FUTEX_32`](#futex_32) | const |  |
| [`FUTEX_NO_NODE`](#futex_no_node) | const |  |
| [`FUTEX_WAITV_MAX`](#futex_waitv_max) | const |  |
| [`FUTEX_WAITERS`](#futex_waiters) | const |  |
| [`FUTEX_OWNER_DIED`](#futex_owner_died) | const |  |
| [`FUTEX_TID_MASK`](#futex_tid_mask) | const |  |
| [`ROBUST_LIST_LIMIT`](#robust_list_limit) | const |  |
| [`FUTEX_BITSET_MATCH_ANY`](#futex_bitset_match_any) | const |  |
| [`FUTEX_OP_SET`](#futex_op_set) | const |  |
| [`FUTEX_OP_ADD`](#futex_op_add) | const |  |
| [`FUTEX_OP_OR`](#futex_op_or) | const |  |
| [`FUTEX_OP_ANDN`](#futex_op_andn) | const |  |
| [`FUTEX_OP_XOR`](#futex_op_xor) | const |  |
| [`FUTEX_OP_OPARG_SHIFT`](#futex_op_oparg_shift) | const |  |
| [`FUTEX_OP_CMP_EQ`](#futex_op_cmp_eq) | const |  |
| [`FUTEX_OP_CMP_NE`](#futex_op_cmp_ne) | const |  |
| [`FUTEX_OP_CMP_LT`](#futex_op_cmp_lt) | const |  |
| [`FUTEX_OP_CMP_LE`](#futex_op_cmp_le) | const |  |
| [`FUTEX_OP_CMP_GT`](#futex_op_cmp_gt) | const |  |
| [`FUTEX_OP_CMP_GE`](#futex_op_cmp_ge) | const |  |
| [`IN_ACCESS`](#in_access) | const |  |
| [`IN_MODIFY`](#in_modify) | const |  |
| [`IN_ATTRIB`](#in_attrib) | const |  |
| [`IN_CLOSE_WRITE`](#in_close_write) | const |  |
| [`IN_CLOSE_NOWRITE`](#in_close_nowrite) | const |  |
| [`IN_OPEN`](#in_open) | const |  |
| [`IN_MOVED_FROM`](#in_moved_from) | const |  |
| [`IN_MOVED_TO`](#in_moved_to) | const |  |
| [`IN_CREATE`](#in_create) | const |  |
| [`IN_DELETE`](#in_delete) | const |  |
| [`IN_DELETE_SELF`](#in_delete_self) | const |  |
| [`IN_MOVE_SELF`](#in_move_self) | const |  |
| [`IN_UNMOUNT`](#in_unmount) | const |  |
| [`IN_Q_OVERFLOW`](#in_q_overflow) | const |  |
| [`IN_IGNORED`](#in_ignored) | const |  |
| [`IN_CLOSE`](#in_close) | const |  |
| [`IN_MOVE`](#in_move) | const |  |
| [`IN_ONLYDIR`](#in_onlydir) | const |  |
| [`IN_DONT_FOLLOW`](#in_dont_follow) | const |  |
| [`IN_EXCL_UNLINK`](#in_excl_unlink) | const |  |
| [`IN_MASK_CREATE`](#in_mask_create) | const |  |
| [`IN_MASK_ADD`](#in_mask_add) | const |  |
| [`IN_ISDIR`](#in_isdir) | const |  |
| [`IN_ONESHOT`](#in_oneshot) | const |  |
| [`IN_ALL_EVENTS`](#in_all_events) | const |  |
| [`IN_CLOEXEC`](#in_cloexec) | const |  |
| [`IN_NONBLOCK`](#in_nonblock) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs_super_magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs_super_magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs_super_magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs_super_magic) | const |  |
| [`CEPH_SUPER_MAGIC`](#ceph_super_magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda_super_magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs_magic) | const |  |
| [`CRAMFS_MAGIC_WEND`](#cramfs_magic_wend) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs_magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs_magic) | const |  |
| [`SELINUX_MAGIC`](#selinux_magic) | const |  |
| [`SMACK_MAGIC`](#smack_magic) | const |  |
| [`RAMFS_MAGIC`](#ramfs_magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs_magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs_magic) | const |  |
| [`SQUASHFS_MAGIC`](#squashfs_magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs_super_magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs_super_magic) | const |  |
| [`EROFS_SUPER_MAGIC_V1`](#erofs_super_magic_v1) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2_super_magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3_super_magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs_super_magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4_super_magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs_super_magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs_super_magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs_super_magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs_super_magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs_super_magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2_super_magic) | const |  |
| [`XFS_SUPER_MAGIC`](#xfs_super_magic) | const |  |
| [`PSTOREFS_MAGIC`](#pstorefs_magic) | const |  |
| [`EFIVARFS_MAGIC`](#efivarfs_magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs_super_magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs_super_magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse_super_magic) | const |  |
| [`BCACHEFS_SUPER_MAGIC`](#bcachefs_super_magic) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix_super_magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix_super_magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2_super_magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2_super_magic2) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3_super_magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos_super_magic) | const |  |
| [`EXFAT_SUPER_MAGIC`](#exfat_super_magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp_super_magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs_super_magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2_super_magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom_super_magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4_super_magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6_super_magic) | const |  |
| [`AFS_FS_MAGIC`](#afs_fs_magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs_super_magic) | const |  |
| [`REISERFS_SUPER_MAGIC_STRING`](#reiserfs_super_magic_string) | const |  |
| [`REISER2FS_SUPER_MAGIC_STRING`](#reiser2fs_super_magic_string) | const |  |
| [`REISER2FS_JR_SUPER_MAGIC_STRING`](#reiser2fs_jr_super_magic_string) | const |  |
| [`SMB_SUPER_MAGIC`](#smb_super_magic) | const |  |
| [`CIFS_SUPER_MAGIC`](#cifs_super_magic) | const |  |
| [`SMB2_SUPER_MAGIC`](#smb2_super_magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup_super_magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2_super_magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup_super_magic) | const |  |
| [`STACK_END_MAGIC`](#stack_end_magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs_magic) | const |  |
| [`V9FS_MAGIC`](#v9fs_magic) | const |  |
| [`BDEVFS_MAGIC`](#bdevfs_magic) | const |  |
| [`DAXFS_MAGIC`](#daxfs_magic) | const |  |
| [`BINFMTFS_MAGIC`](#binfmtfs_magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts_super_magic) | const |  |
| [`BINDERFS_SUPER_MAGIC`](#binderfs_super_magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs_super_magic) | const |  |
| [`PIPEFS_MAGIC`](#pipefs_magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc_super_magic) | const |  |
| [`SOCKFS_MAGIC`](#sockfs_magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs_magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice_super_magic) | const |  |
| [`MTD_INODE_FS_MAGIC`](#mtd_inode_fs_magic) | const |  |
| [`ANON_INODE_FS_MAGIC`](#anon_inode_fs_magic) | const |  |
| [`BTRFS_TEST_MAGIC`](#btrfs_test_magic) | const |  |
| [`NSFS_MAGIC`](#nsfs_magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf_fs_magic) | const |  |
| [`AAFS_MAGIC`](#aafs_magic) | const |  |
| [`ZONEFS_MAGIC`](#zonefs_magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf_super_magic) | const |  |
| [`DMA_BUF_MAGIC`](#dma_buf_magic) | const |  |
| [`DEVMEM_MAGIC`](#devmem_magic) | const |  |
| [`SECRETMEM_MAGIC`](#secretmem_magic) | const |  |
| [`PID_FS_MAGIC`](#pid_fs_magic) | const |  |
| [`MAP_32BIT`](#map_32bit) | const |  |
| [`MAP_ABOVE4G`](#map_above4g) | const |  |
| [`PROT_READ`](#prot_read) | const |  |
| [`PROT_WRITE`](#prot_write) | const |  |
| [`PROT_EXEC`](#prot_exec) | const |  |
| [`PROT_SEM`](#prot_sem) | const |  |
| [`PROT_NONE`](#prot_none) | const |  |
| [`PROT_GROWSDOWN`](#prot_growsdown) | const |  |
| [`PROT_GROWSUP`](#prot_growsup) | const |  |
| [`MAP_TYPE`](#map_type) | const |  |
| [`MAP_FIXED`](#map_fixed) | const |  |
| [`MAP_ANONYMOUS`](#map_anonymous) | const |  |
| [`MAP_POPULATE`](#map_populate) | const |  |
| [`MAP_NONBLOCK`](#map_nonblock) | const |  |
| [`MAP_STACK`](#map_stack) | const |  |
| [`MAP_HUGETLB`](#map_hugetlb) | const |  |
| [`MAP_SYNC`](#map_sync) | const |  |
| [`MAP_FIXED_NOREPLACE`](#map_fixed_noreplace) | const |  |
| [`MAP_UNINITIALIZED`](#map_uninitialized) | const |  |
| [`MLOCK_ONFAULT`](#mlock_onfault) | const |  |
| [`MS_ASYNC`](#ms_async) | const |  |
| [`MS_INVALIDATE`](#ms_invalidate) | const |  |
| [`MS_SYNC`](#ms_sync) | const |  |
| [`MADV_NORMAL`](#madv_normal) | const |  |
| [`MADV_RANDOM`](#madv_random) | const |  |
| [`MADV_SEQUENTIAL`](#madv_sequential) | const |  |
| [`MADV_WILLNEED`](#madv_willneed) | const |  |
| [`MADV_DONTNEED`](#madv_dontneed) | const |  |
| [`MADV_FREE`](#madv_free) | const |  |
| [`MADV_REMOVE`](#madv_remove) | const |  |
| [`MADV_DONTFORK`](#madv_dontfork) | const |  |
| [`MADV_DOFORK`](#madv_dofork) | const |  |
| [`MADV_HWPOISON`](#madv_hwpoison) | const |  |
| [`MADV_SOFT_OFFLINE`](#madv_soft_offline) | const |  |
| [`MADV_MERGEABLE`](#madv_mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv_unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv_hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv_nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv_dontdump) | const |  |
| [`MADV_DODUMP`](#madv_dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv_wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv_keeponfork) | const |  |
| [`MADV_COLD`](#madv_cold) | const |  |
| [`MADV_PAGEOUT`](#madv_pageout) | const |  |
| [`MADV_POPULATE_READ`](#madv_populate_read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv_populate_write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv_dontneed_locked) | const |  |
| [`MADV_COLLAPSE`](#madv_collapse) | const |  |
| [`MADV_GUARD_INSTALL`](#madv_guard_install) | const |  |
| [`MADV_GUARD_REMOVE`](#madv_guard_remove) | const |  |
| [`MAP_FILE`](#map_file) | const |  |
| [`PKEY_UNRESTRICTED`](#pkey_unrestricted) | const |  |
| [`PKEY_DISABLE_ACCESS`](#pkey_disable_access) | const |  |
| [`PKEY_DISABLE_WRITE`](#pkey_disable_write) | const |  |
| [`PKEY_ACCESS_MASK`](#pkey_access_mask) | const |  |
| [`MAP_GROWSDOWN`](#map_growsdown) | const |  |
| [`MAP_DENYWRITE`](#map_denywrite) | const |  |
| [`MAP_EXECUTABLE`](#map_executable) | const |  |
| [`MAP_LOCKED`](#map_locked) | const |  |
| [`MAP_NORESERVE`](#map_noreserve) | const |  |
| [`MCL_CURRENT`](#mcl_current) | const |  |
| [`MCL_FUTURE`](#mcl_future) | const |  |
| [`MCL_ONFAULT`](#mcl_onfault) | const |  |
| [`SHADOW_STACK_SET_TOKEN`](#shadow_stack_set_token) | const |  |
| [`SHADOW_STACK_SET_MARKER`](#shadow_stack_set_marker) | const |  |
| [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb_flag_encode_shift) | const |  |
| [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb_flag_encode_mask) | const |  |
| [`HUGETLB_FLAG_ENCODE_16KB`](#hugetlb_flag_encode_16kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb_flag_encode_64kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb_flag_encode_512kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb_flag_encode_1mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb_flag_encode_2mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb_flag_encode_8mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb_flag_encode_16mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb_flag_encode_32mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb_flag_encode_256mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb_flag_encode_512mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb_flag_encode_1gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb_flag_encode_2gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb_flag_encode_16gb) | const |  |
| [`MREMAP_MAYMOVE`](#mremap_maymove) | const |  |
| [`MREMAP_FIXED`](#mremap_fixed) | const |  |
| [`MREMAP_DONTUNMAP`](#mremap_dontunmap) | const |  |
| [`OVERCOMMIT_GUESS`](#overcommit_guess) | const |  |
| [`OVERCOMMIT_ALWAYS`](#overcommit_always) | const |  |
| [`OVERCOMMIT_NEVER`](#overcommit_never) | const |  |
| [`MAP_SHARED`](#map_shared) | const |  |
| [`MAP_PRIVATE`](#map_private) | const |  |
| [`MAP_SHARED_VALIDATE`](#map_shared_validate) | const |  |
| [`MAP_DROPPABLE`](#map_droppable) | const |  |
| [`MAP_HUGE_SHIFT`](#map_huge_shift) | const |  |
| [`MAP_HUGE_MASK`](#map_huge_mask) | const |  |
| [`MAP_HUGE_16KB`](#map_huge_16kb) | const |  |
| [`MAP_HUGE_64KB`](#map_huge_64kb) | const |  |
| [`MAP_HUGE_512KB`](#map_huge_512kb) | const |  |
| [`MAP_HUGE_1MB`](#map_huge_1mb) | const |  |
| [`MAP_HUGE_2MB`](#map_huge_2mb) | const |  |
| [`MAP_HUGE_8MB`](#map_huge_8mb) | const |  |
| [`MAP_HUGE_16MB`](#map_huge_16mb) | const |  |
| [`MAP_HUGE_32MB`](#map_huge_32mb) | const |  |
| [`MAP_HUGE_256MB`](#map_huge_256mb) | const |  |
| [`MAP_HUGE_512MB`](#map_huge_512mb) | const |  |
| [`MAP_HUGE_1GB`](#map_huge_1gb) | const |  |
| [`MAP_HUGE_2GB`](#map_huge_2gb) | const |  |
| [`MAP_HUGE_16GB`](#map_huge_16gb) | const |  |
| [`POLLIN`](#pollin) | const |  |
| [`POLLPRI`](#pollpri) | const |  |
| [`POLLOUT`](#pollout) | const |  |
| [`POLLERR`](#pollerr) | const |  |
| [`POLLHUP`](#pollhup) | const |  |
| [`POLLNVAL`](#pollnval) | const |  |
| [`POLLRDNORM`](#pollrdnorm) | const |  |
| [`POLLRDBAND`](#pollrdband) | const |  |
| [`POLLWRNORM`](#pollwrnorm) | const |  |
| [`POLLWRBAND`](#pollwrband) | const |  |
| [`POLLMSG`](#pollmsg) | const |  |
| [`POLLREMOVE`](#pollremove) | const |  |
| [`POLLRDHUP`](#pollrdhup) | const |  |
| [`GRND_NONBLOCK`](#grnd_nonblock) | const |  |
| [`GRND_RANDOM`](#grnd_random) | const |  |
| [`GRND_INSECURE`](#grnd_insecure) | const |  |
| [`LINUX_REBOOT_MAGIC1`](#linux_reboot_magic1) | const |  |
| [`LINUX_REBOOT_MAGIC2`](#linux_reboot_magic2) | const |  |
| [`LINUX_REBOOT_MAGIC2A`](#linux_reboot_magic2a) | const |  |
| [`LINUX_REBOOT_MAGIC2B`](#linux_reboot_magic2b) | const |  |
| [`LINUX_REBOOT_MAGIC2C`](#linux_reboot_magic2c) | const |  |
| [`LINUX_REBOOT_CMD_RESTART`](#linux_reboot_cmd_restart) | const |  |
| [`LINUX_REBOOT_CMD_HALT`](#linux_reboot_cmd_halt) | const |  |
| [`LINUX_REBOOT_CMD_CAD_ON`](#linux_reboot_cmd_cad_on) | const |  |
| [`LINUX_REBOOT_CMD_CAD_OFF`](#linux_reboot_cmd_cad_off) | const |  |
| [`LINUX_REBOOT_CMD_POWER_OFF`](#linux_reboot_cmd_power_off) | const |  |
| [`LINUX_REBOOT_CMD_RESTART2`](#linux_reboot_cmd_restart2) | const |  |
| [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux_reboot_cmd_sw_suspend) | const |  |
| [`LINUX_REBOOT_CMD_KEXEC`](#linux_reboot_cmd_kexec) | const |  |
| [`RUSAGE_SELF`](#rusage_self) | const |  |
| [`RUSAGE_CHILDREN`](#rusage_children) | const |  |
| [`RUSAGE_BOTH`](#rusage_both) | const |  |
| [`RUSAGE_THREAD`](#rusage_thread) | const |  |
| [`RLIM64_INFINITY`](#rlim64_infinity) | const |  |
| [`PRIO_MIN`](#prio_min) | const |  |
| [`PRIO_MAX`](#prio_max) | const |  |
| [`PRIO_PROCESS`](#prio_process) | const |  |
| [`PRIO_PGRP`](#prio_pgrp) | const |  |
| [`PRIO_USER`](#prio_user) | const |  |
| [`_STK_LIM`](#_stk_lim) | const |  |
| [`MLOCK_LIMIT`](#mlock_limit) | const |  |
| [`RLIMIT_CPU`](#rlimit_cpu) | const |  |
| [`RLIMIT_FSIZE`](#rlimit_fsize) | const |  |
| [`RLIMIT_DATA`](#rlimit_data) | const |  |
| [`RLIMIT_STACK`](#rlimit_stack) | const |  |
| [`RLIMIT_CORE`](#rlimit_core) | const |  |
| [`RLIMIT_RSS`](#rlimit_rss) | const |  |
| [`RLIMIT_NPROC`](#rlimit_nproc) | const |  |
| [`RLIMIT_NOFILE`](#rlimit_nofile) | const |  |
| [`RLIMIT_MEMLOCK`](#rlimit_memlock) | const |  |
| [`RLIMIT_AS`](#rlimit_as) | const |  |
| [`RLIMIT_LOCKS`](#rlimit_locks) | const |  |
| [`RLIMIT_SIGPENDING`](#rlimit_sigpending) | const |  |
| [`RLIMIT_MSGQUEUE`](#rlimit_msgqueue) | const |  |
| [`RLIMIT_NICE`](#rlimit_nice) | const |  |
| [`RLIMIT_RTPRIO`](#rlimit_rtprio) | const |  |
| [`RLIMIT_RTTIME`](#rlimit_rttime) | const |  |
| [`RLIM_NLIMITS`](#rlim_nlimits) | const |  |
| [`RLIM_INFINITY`](#rlim_infinity) | const |  |
| [`CSIGNAL`](#csignal) | const |  |
| [`CLONE_VM`](#clone_vm) | const |  |
| [`CLONE_FS`](#clone_fs) | const |  |
| [`CLONE_FILES`](#clone_files) | const |  |
| [`CLONE_SIGHAND`](#clone_sighand) | const |  |
| [`CLONE_PIDFD`](#clone_pidfd) | const |  |
| [`CLONE_PTRACE`](#clone_ptrace) | const |  |
| [`CLONE_VFORK`](#clone_vfork) | const |  |
| [`CLONE_PARENT`](#clone_parent) | const |  |
| [`CLONE_THREAD`](#clone_thread) | const |  |
| [`CLONE_NEWNS`](#clone_newns) | const |  |
| [`CLONE_SYSVSEM`](#clone_sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone_settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone_parent_settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone_child_cleartid) | const |  |
| [`CLONE_DETACHED`](#clone_detached) | const |  |
| [`CLONE_UNTRACED`](#clone_untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone_child_settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone_newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone_newuts) | const |  |
| [`CLONE_NEWIPC`](#clone_newipc) | const |  |
| [`CLONE_NEWUSER`](#clone_newuser) | const |  |
| [`CLONE_NEWPID`](#clone_newpid) | const |  |
| [`CLONE_NEWNET`](#clone_newnet) | const |  |
| [`CLONE_IO`](#clone_io) | const |  |
| [`CLONE_CLEAR_SIGHAND`](#clone_clear_sighand) | const |  |
| [`CLONE_INTO_CGROUP`](#clone_into_cgroup) | const |  |
| [`CLONE_NEWTIME`](#clone_newtime) | const |  |
| [`CLONE_ARGS_SIZE_VER0`](#clone_args_size_ver0) | const |  |
| [`CLONE_ARGS_SIZE_VER1`](#clone_args_size_ver1) | const |  |
| [`CLONE_ARGS_SIZE_VER2`](#clone_args_size_ver2) | const |  |
| [`SCHED_NORMAL`](#sched_normal) | const |  |
| [`SCHED_FIFO`](#sched_fifo) | const |  |
| [`SCHED_RR`](#sched_rr) | const |  |
| [`SCHED_BATCH`](#sched_batch) | const |  |
| [`SCHED_IDLE`](#sched_idle) | const |  |
| [`SCHED_DEADLINE`](#sched_deadline) | const |  |
| [`SCHED_EXT`](#sched_ext) | const |  |
| [`SCHED_RESET_ON_FORK`](#sched_reset_on_fork) | const |  |
| [`SCHED_FLAG_RESET_ON_FORK`](#sched_flag_reset_on_fork) | const |  |
| [`SCHED_FLAG_RECLAIM`](#sched_flag_reclaim) | const |  |
| [`SCHED_FLAG_DL_OVERRUN`](#sched_flag_dl_overrun) | const |  |
| [`SCHED_FLAG_KEEP_POLICY`](#sched_flag_keep_policy) | const |  |
| [`SCHED_FLAG_KEEP_PARAMS`](#sched_flag_keep_params) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched_flag_util_clamp_min) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched_flag_util_clamp_max) | const |  |
| [`SCHED_FLAG_KEEP_ALL`](#sched_flag_keep_all) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP`](#sched_flag_util_clamp) | const |  |
| [`SCHED_FLAG_ALL`](#sched_flag_all) | const |  |
| [`NSIG`](#nsig) | const |  |
| [`SIGHUP`](#sighup) | const |  |
| [`SIGINT`](#sigint) | const |  |
| [`SIGQUIT`](#sigquit) | const |  |
| [`SIGILL`](#sigill) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`SIGABRT`](#sigabrt) | const |  |
| [`SIGIOT`](#sigiot) | const |  |
| [`SIGBUS`](#sigbus) | const |  |
| [`SIGFPE`](#sigfpe) | const |  |
| [`SIGKILL`](#sigkill) | const |  |
| [`SIGUSR1`](#sigusr1) | const |  |
| [`SIGSEGV`](#sigsegv) | const |  |
| [`SIGUSR2`](#sigusr2) | const |  |
| [`SIGPIPE`](#sigpipe) | const |  |
| [`SIGALRM`](#sigalrm) | const |  |
| [`SIGTERM`](#sigterm) | const |  |
| [`SIGSTKFLT`](#sigstkflt) | const |  |
| [`SIGCHLD`](#sigchld) | const |  |
| [`SIGCONT`](#sigcont) | const |  |
| [`SIGSTOP`](#sigstop) | const |  |
| [`SIGTSTP`](#sigtstp) | const |  |
| [`SIGTTIN`](#sigttin) | const |  |
| [`SIGTTOU`](#sigttou) | const |  |
| [`SIGURG`](#sigurg) | const |  |
| [`SIGXCPU`](#sigxcpu) | const |  |
| [`SIGXFSZ`](#sigxfsz) | const |  |
| [`SIGVTALRM`](#sigvtalrm) | const |  |
| [`SIGPROF`](#sigprof) | const |  |
| [`SIGWINCH`](#sigwinch) | const |  |
| [`SIGIO`](#sigio) | const |  |
| [`SIGPOLL`](#sigpoll) | const |  |
| [`SIGPWR`](#sigpwr) | const |  |
| [`SIGSYS`](#sigsys) | const |  |
| [`SIGUNUSED`](#sigunused) | const |  |
| [`SIGRTMIN`](#sigrtmin) | const |  |
| [`SA_RESTORER`](#sa_restorer) | const |  |
| [`MINSIGSTKSZ`](#minsigstksz) | const |  |
| [`SIGSTKSZ`](#sigstksz) | const |  |
| [`SA_NOCLDSTOP`](#sa_nocldstop) | const |  |
| [`SA_NOCLDWAIT`](#sa_nocldwait) | const |  |
| [`SA_SIGINFO`](#sa_siginfo) | const |  |
| [`SA_UNSUPPORTED`](#sa_unsupported) | const |  |
| [`SA_EXPOSE_TAGBITS`](#sa_expose_tagbits) | const |  |
| [`SA_ONSTACK`](#sa_onstack) | const |  |
| [`SA_RESTART`](#sa_restart) | const |  |
| [`SA_NODEFER`](#sa_nodefer) | const |  |
| [`SA_RESETHAND`](#sa_resethand) | const |  |
| [`SA_NOMASK`](#sa_nomask) | const |  |
| [`SA_ONESHOT`](#sa_oneshot) | const |  |
| [`SIG_BLOCK`](#sig_block) | const |  |
| [`SIG_UNBLOCK`](#sig_unblock) | const |  |
| [`SIG_SETMASK`](#sig_setmask) | const |  |
| [`SI_MAX_SIZE`](#si_max_size) | const |  |
| [`SI_USER`](#si_user) | const |  |
| [`SI_KERNEL`](#si_kernel) | const |  |
| [`SI_QUEUE`](#si_queue) | const |  |
| [`SI_TIMER`](#si_timer) | const |  |
| [`SI_MESGQ`](#si_mesgq) | const |  |
| [`SI_ASYNCIO`](#si_asyncio) | const |  |
| [`SI_SIGIO`](#si_sigio) | const |  |
| [`SI_TKILL`](#si_tkill) | const |  |
| [`SI_DETHREAD`](#si_dethread) | const |  |
| [`SI_ASYNCNL`](#si_asyncnl) | const |  |
| [`ILL_ILLOPC`](#ill_illopc) | const |  |
| [`ILL_ILLOPN`](#ill_illopn) | const |  |
| [`ILL_ILLADR`](#ill_illadr) | const |  |
| [`ILL_ILLTRP`](#ill_illtrp) | const |  |
| [`ILL_PRVOPC`](#ill_prvopc) | const |  |
| [`ILL_PRVREG`](#ill_prvreg) | const |  |
| [`ILL_COPROC`](#ill_coproc) | const |  |
| [`ILL_BADSTK`](#ill_badstk) | const |  |
| [`ILL_BADIADDR`](#ill_badiaddr) | const |  |
| [`__ILL_BREAK`](#__ill_break) | const |  |
| [`__ILL_BNDMOD`](#__ill_bndmod) | const |  |
| [`NSIGILL`](#nsigill) | const |  |
| [`FPE_INTDIV`](#fpe_intdiv) | const |  |
| [`FPE_INTOVF`](#fpe_intovf) | const |  |
| [`FPE_FLTDIV`](#fpe_fltdiv) | const |  |
| [`FPE_FLTOVF`](#fpe_fltovf) | const |  |
| [`FPE_FLTUND`](#fpe_fltund) | const |  |
| [`FPE_FLTRES`](#fpe_fltres) | const |  |
| [`FPE_FLTINV`](#fpe_fltinv) | const |  |
| [`FPE_FLTSUB`](#fpe_fltsub) | const |  |
| [`__FPE_DECOVF`](#__fpe_decovf) | const |  |
| [`__FPE_DECDIV`](#__fpe_decdiv) | const |  |
| [`__FPE_DECERR`](#__fpe_decerr) | const |  |
| [`__FPE_INVASC`](#__fpe_invasc) | const |  |
| [`__FPE_INVDEC`](#__fpe_invdec) | const |  |
| [`FPE_FLTUNK`](#fpe_fltunk) | const |  |
| [`FPE_CONDTRAP`](#fpe_condtrap) | const |  |
| [`NSIGFPE`](#nsigfpe) | const |  |
| [`SEGV_MAPERR`](#segv_maperr) | const |  |
| [`SEGV_ACCERR`](#segv_accerr) | const |  |
| [`SEGV_BNDERR`](#segv_bnderr) | const |  |
| [`SEGV_PKUERR`](#segv_pkuerr) | const |  |
| [`SEGV_ACCADI`](#segv_accadi) | const |  |
| [`SEGV_ADIDERR`](#segv_adiderr) | const |  |
| [`SEGV_ADIPERR`](#segv_adiperr) | const |  |
| [`SEGV_MTEAERR`](#segv_mteaerr) | const |  |
| [`SEGV_MTESERR`](#segv_mteserr) | const |  |
| [`SEGV_CPERR`](#segv_cperr) | const |  |
| [`NSIGSEGV`](#nsigsegv) | const |  |
| [`BUS_ADRALN`](#bus_adraln) | const |  |
| [`BUS_ADRERR`](#bus_adrerr) | const |  |
| [`BUS_OBJERR`](#bus_objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus_mceerr_ar) | const |  |
| [`BUS_MCEERR_AO`](#bus_mceerr_ao) | const |  |
| [`NSIGBUS`](#nsigbus) | const |  |
| [`TRAP_BRKPT`](#trap_brkpt) | const |  |
| [`TRAP_TRACE`](#trap_trace) | const |  |
| [`TRAP_BRANCH`](#trap_branch) | const |  |
| [`TRAP_HWBKPT`](#trap_hwbkpt) | const |  |
| [`TRAP_UNK`](#trap_unk) | const |  |
| [`TRAP_PERF`](#trap_perf) | const |  |
| [`NSIGTRAP`](#nsigtrap) | const |  |
| [`TRAP_PERF_FLAG_ASYNC`](#trap_perf_flag_async) | const |  |
| [`CLD_EXITED`](#cld_exited) | const |  |
| [`CLD_KILLED`](#cld_killed) | const |  |
| [`CLD_DUMPED`](#cld_dumped) | const |  |
| [`CLD_TRAPPED`](#cld_trapped) | const |  |
| [`CLD_STOPPED`](#cld_stopped) | const |  |
| [`CLD_CONTINUED`](#cld_continued) | const |  |
| [`NSIGCHLD`](#nsigchld) | const |  |
| [`POLL_IN`](#poll_in) | const |  |
| [`POLL_OUT`](#poll_out) | const |  |
| [`POLL_MSG`](#poll_msg) | const |  |
| [`POLL_ERR`](#poll_err) | const |  |
| [`POLL_PRI`](#poll_pri) | const |  |
| [`POLL_HUP`](#poll_hup) | const |  |
| [`NSIGPOLL`](#nsigpoll) | const |  |
| [`SYS_SECCOMP`](#sys_seccomp) | const |  |
| [`SYS_USER_DISPATCH`](#sys_user_dispatch) | const |  |
| [`NSIGSYS`](#nsigsys) | const |  |
| [`EMT_TAGOVF`](#emt_tagovf) | const |  |
| [`NSIGEMT`](#nsigemt) | const |  |
| [`SIGEV_SIGNAL`](#sigev_signal) | const |  |
| [`SIGEV_NONE`](#sigev_none) | const |  |
| [`SIGEV_THREAD`](#sigev_thread) | const |  |
| [`SIGEV_THREAD_ID`](#sigev_thread_id) | const |  |
| [`SIGEV_MAX_SIZE`](#sigev_max_size) | const |  |
| [`SS_ONSTACK`](#ss_onstack) | const |  |
| [`SS_DISABLE`](#ss_disable) | const |  |
| [`SS_AUTODISARM`](#ss_autodisarm) | const |  |
| [`SS_FLAG_BITS`](#ss_flag_bits) | const |  |
| [`S_IFMT`](#s_ifmt) | const |  |
| [`S_IFSOCK`](#s_ifsock) | const |  |
| [`S_IFLNK`](#s_iflnk) | const |  |
| [`S_IFREG`](#s_ifreg) | const |  |
| [`S_IFBLK`](#s_ifblk) | const |  |
| [`S_IFDIR`](#s_ifdir) | const |  |
| [`S_IFCHR`](#s_ifchr) | const |  |
| [`S_IFIFO`](#s_ififo) | const |  |
| [`S_ISUID`](#s_isuid) | const |  |
| [`S_ISGID`](#s_isgid) | const |  |
| [`S_ISVTX`](#s_isvtx) | const |  |
| [`S_IRWXU`](#s_irwxu) | const |  |
| [`S_IRUSR`](#s_irusr) | const |  |
| [`S_IWUSR`](#s_iwusr) | const |  |
| [`S_IXUSR`](#s_ixusr) | const |  |
| [`S_IRWXG`](#s_irwxg) | const |  |
| [`S_IRGRP`](#s_irgrp) | const |  |
| [`S_IWGRP`](#s_iwgrp) | const |  |
| [`S_IXGRP`](#s_ixgrp) | const |  |
| [`S_IRWXO`](#s_irwxo) | const |  |
| [`S_IROTH`](#s_iroth) | const |  |
| [`S_IWOTH`](#s_iwoth) | const |  |
| [`S_IXOTH`](#s_ixoth) | const |  |
| [`STATX_TYPE`](#statx_type) | const |  |
| [`STATX_MODE`](#statx_mode) | const |  |
| [`STATX_NLINK`](#statx_nlink) | const |  |
| [`STATX_UID`](#statx_uid) | const |  |
| [`STATX_GID`](#statx_gid) | const |  |
| [`STATX_ATIME`](#statx_atime) | const |  |
| [`STATX_MTIME`](#statx_mtime) | const |  |
| [`STATX_CTIME`](#statx_ctime) | const |  |
| [`STATX_INO`](#statx_ino) | const |  |
| [`STATX_SIZE`](#statx_size) | const |  |
| [`STATX_BLOCKS`](#statx_blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx_basic_stats) | const |  |
| [`STATX_BTIME`](#statx_btime) | const |  |
| [`STATX_MNT_ID`](#statx_mnt_id) | const |  |
| [`STATX_DIOALIGN`](#statx_dioalign) | const |  |
| [`STATX_MNT_ID_UNIQUE`](#statx_mnt_id_unique) | const |  |
| [`STATX_SUBVOL`](#statx_subvol) | const |  |
| [`STATX_WRITE_ATOMIC`](#statx_write_atomic) | const |  |
| [`STATX_DIO_READ_ALIGN`](#statx_dio_read_align) | const |  |
| [`STATX__RESERVED`](#statx__reserved) | const |  |
| [`STATX_ALL`](#statx_all) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx_attr_compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx_attr_immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx_attr_append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx_attr_nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx_attr_encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx_attr_automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx_attr_mount_root) | const |  |
| [`STATX_ATTR_VERITY`](#statx_attr_verity) | const |  |
| [`STATX_ATTR_DAX`](#statx_attr_dax) | const |  |
| [`STATX_ATTR_WRITE_ATOMIC`](#statx_attr_write_atomic) | const |  |
| [`IGNBRK`](#ignbrk) | const |  |
| [`BRKINT`](#brkint) | const |  |
| [`IGNPAR`](#ignpar) | const |  |
| [`PARMRK`](#parmrk) | const |  |
| [`INPCK`](#inpck) | const |  |
| [`ISTRIP`](#istrip) | const |  |
| [`INLCR`](#inlcr) | const |  |
| [`IGNCR`](#igncr) | const |  |
| [`ICRNL`](#icrnl) | const |  |
| [`IXANY`](#ixany) | const |  |
| [`OPOST`](#opost) | const |  |
| [`OCRNL`](#ocrnl) | const |  |
| [`ONOCR`](#onocr) | const |  |
| [`ONLRET`](#onlret) | const |  |
| [`OFILL`](#ofill) | const |  |
| [`OFDEL`](#ofdel) | const |  |
| [`B0`](#b0) | const |  |
| [`B50`](#b50) | const |  |
| [`B75`](#b75) | const |  |
| [`B110`](#b110) | const |  |
| [`B134`](#b134) | const |  |
| [`B150`](#b150) | const |  |
| [`B200`](#b200) | const |  |
| [`B300`](#b300) | const |  |
| [`B600`](#b600) | const |  |
| [`B1200`](#b1200) | const |  |
| [`B1800`](#b1800) | const |  |
| [`B2400`](#b2400) | const |  |
| [`B4800`](#b4800) | const |  |
| [`B9600`](#b9600) | const |  |
| [`B19200`](#b19200) | const |  |
| [`B38400`](#b38400) | const |  |
| [`EXTA`](#exta) | const |  |
| [`EXTB`](#extb) | const |  |
| [`ADDRB`](#addrb) | const |  |
| [`CMSPAR`](#cmspar) | const |  |
| [`CRTSCTS`](#crtscts) | const |  |
| [`IBSHIFT`](#ibshift) | const |  |
| [`TCOOFF`](#tcooff) | const |  |
| [`TCOON`](#tcoon) | const |  |
| [`TCIOFF`](#tcioff) | const |  |
| [`TCION`](#tcion) | const |  |
| [`TCIFLUSH`](#tciflush) | const |  |
| [`TCOFLUSH`](#tcoflush) | const |  |
| [`TCIOFLUSH`](#tcioflush) | const |  |
| [`NCCS`](#nccs) | const |  |
| [`VINTR`](#vintr) | const |  |
| [`VQUIT`](#vquit) | const |  |
| [`VERASE`](#verase) | const |  |
| [`VKILL`](#vkill) | const |  |
| [`VEOF`](#veof) | const |  |
| [`VTIME`](#vtime) | const |  |
| [`VMIN`](#vmin) | const |  |
| [`VSWTC`](#vswtc) | const |  |
| [`VSTART`](#vstart) | const |  |
| [`VSTOP`](#vstop) | const |  |
| [`VSUSP`](#vsusp) | const |  |
| [`VEOL`](#veol) | const |  |
| [`VREPRINT`](#vreprint) | const |  |
| [`VDISCARD`](#vdiscard) | const |  |
| [`VWERASE`](#vwerase) | const |  |
| [`VLNEXT`](#vlnext) | const |  |
| [`VEOL2`](#veol2) | const |  |
| [`IUCLC`](#iuclc) | const |  |
| [`IXON`](#ixon) | const |  |
| [`IXOFF`](#ixoff) | const |  |
| [`IMAXBEL`](#imaxbel) | const |  |
| [`IUTF8`](#iutf8) | const |  |
| [`OLCUC`](#olcuc) | const |  |
| [`ONLCR`](#onlcr) | const |  |
| [`NLDLY`](#nldly) | const |  |
| [`NL0`](#nl0) | const |  |
| [`NL1`](#nl1) | const |  |
| [`CRDLY`](#crdly) | const |  |
| [`CR0`](#cr0) | const |  |
| [`CR1`](#cr1) | const |  |
| [`CR2`](#cr2) | const |  |
| [`CR3`](#cr3) | const |  |
| [`TABDLY`](#tabdly) | const |  |
| [`TAB0`](#tab0) | const |  |
| [`TAB1`](#tab1) | const |  |
| [`TAB2`](#tab2) | const |  |
| [`TAB3`](#tab3) | const |  |
| [`XTABS`](#xtabs) | const |  |
| [`BSDLY`](#bsdly) | const |  |
| [`BS0`](#bs0) | const |  |
| [`BS1`](#bs1) | const |  |
| [`VTDLY`](#vtdly) | const |  |
| [`VT0`](#vt0) | const |  |
| [`VT1`](#vt1) | const |  |
| [`FFDLY`](#ffdly) | const |  |
| [`FF0`](#ff0) | const |  |
| [`FF1`](#ff1) | const |  |
| [`CBAUD`](#cbaud) | const |  |
| [`CSIZE`](#csize) | const |  |
| [`CS5`](#cs5) | const |  |
| [`CS6`](#cs6) | const |  |
| [`CS7`](#cs7) | const |  |
| [`CS8`](#cs8) | const |  |
| [`CSTOPB`](#cstopb) | const |  |
| [`CREAD`](#cread) | const |  |
| [`PARENB`](#parenb) | const |  |
| [`PARODD`](#parodd) | const |  |
| [`HUPCL`](#hupcl) | const |  |
| [`CLOCAL`](#clocal) | const |  |
| [`CBAUDEX`](#cbaudex) | const |  |
| [`BOTHER`](#bother) | const |  |
| [`B57600`](#b57600) | const |  |
| [`B115200`](#b115200) | const |  |
| [`B230400`](#b230400) | const |  |
| [`B460800`](#b460800) | const |  |
| [`B500000`](#b500000) | const |  |
| [`B576000`](#b576000) | const |  |
| [`B921600`](#b921600) | const |  |
| [`B1000000`](#b1000000) | const |  |
| [`B1152000`](#b1152000) | const |  |
| [`B1500000`](#b1500000) | const |  |
| [`B2000000`](#b2000000) | const |  |
| [`B2500000`](#b2500000) | const |  |
| [`B3000000`](#b3000000) | const |  |
| [`B3500000`](#b3500000) | const |  |
| [`B4000000`](#b4000000) | const |  |
| [`CIBAUD`](#cibaud) | const |  |
| [`ISIG`](#isig) | const |  |
| [`ICANON`](#icanon) | const |  |
| [`XCASE`](#xcase) | const |  |
| [`ECHO`](#echo) | const |  |
| [`ECHOE`](#echoe) | const |  |
| [`ECHOK`](#echok) | const |  |
| [`ECHONL`](#echonl) | const |  |
| [`NOFLSH`](#noflsh) | const |  |
| [`TOSTOP`](#tostop) | const |  |
| [`ECHOCTL`](#echoctl) | const |  |
| [`ECHOPRT`](#echoprt) | const |  |
| [`ECHOKE`](#echoke) | const |  |
| [`FLUSHO`](#flusho) | const |  |
| [`PENDIN`](#pendin) | const |  |
| [`IEXTEN`](#iexten) | const |  |
| [`EXTPROC`](#extproc) | const |  |
| [`TCSANOW`](#tcsanow) | const |  |
| [`TCSADRAIN`](#tcsadrain) | const |  |
| [`TCSAFLUSH`](#tcsaflush) | const |  |
| [`TIOCPKT_DATA`](#tiocpkt_data) | const |  |
| [`TIOCPKT_FLUSHREAD`](#tiocpkt_flushread) | const |  |
| [`TIOCPKT_FLUSHWRITE`](#tiocpkt_flushwrite) | const |  |
| [`TIOCPKT_STOP`](#tiocpkt_stop) | const |  |
| [`TIOCPKT_START`](#tiocpkt_start) | const |  |
| [`TIOCPKT_NOSTOP`](#tiocpkt_nostop) | const |  |
| [`TIOCPKT_DOSTOP`](#tiocpkt_dostop) | const |  |
| [`TIOCPKT_IOCTL`](#tiocpkt_ioctl) | const |  |
| [`TIOCSER_TEMT`](#tiocser_temt) | const |  |
| [`NCC`](#ncc) | const |  |
| [`TIOCM_LE`](#tiocm_le) | const |  |
| [`TIOCM_DTR`](#tiocm_dtr) | const |  |
| [`TIOCM_RTS`](#tiocm_rts) | const |  |
| [`TIOCM_ST`](#tiocm_st) | const |  |
| [`TIOCM_SR`](#tiocm_sr) | const |  |
| [`TIOCM_CTS`](#tiocm_cts) | const |  |
| [`TIOCM_CAR`](#tiocm_car) | const |  |
| [`TIOCM_RNG`](#tiocm_rng) | const |  |
| [`TIOCM_DSR`](#tiocm_dsr) | const |  |
| [`TIOCM_CD`](#tiocm_cd) | const |  |
| [`TIOCM_RI`](#tiocm_ri) | const |  |
| [`TIOCM_OUT1`](#tiocm_out1) | const |  |
| [`TIOCM_OUT2`](#tiocm_out2) | const |  |
| [`TIOCM_LOOP`](#tiocm_loop) | const |  |
| [`ITIMER_REAL`](#itimer_real) | const |  |
| [`ITIMER_VIRTUAL`](#itimer_virtual) | const |  |
| [`ITIMER_PROF`](#itimer_prof) | const |  |
| [`CLOCK_REALTIME`](#clock_realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock_monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock_boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock_realtime_alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock_boottime_alarm) | const |  |
| [`CLOCK_SGI_CYCLE`](#clock_sgi_cycle) | const |  |
| [`CLOCK_TAI`](#clock_tai) | const |  |
| [`MAX_CLOCKS`](#max_clocks) | const |  |
| [`CLOCKS_MASK`](#clocks_mask) | const |  |
| [`CLOCKS_MONO`](#clocks_mono) | const |  |
| [`TIMER_ABSTIME`](#timer_abstime) | const |  |
| [`UIO_FASTIOV`](#uio_fastiov) | const |  |
| [`UIO_MAXIOV`](#uio_maxiov) | const |  |
| [`__X32_SYSCALL_BIT`](#__x32_syscall_bit) | const |  |
| [`__NR_read`](#__nr_read) | const |  |
| [`__NR_write`](#__nr_write) | const |  |
| [`__NR_open`](#__nr_open) | const |  |
| [`__NR_close`](#__nr_close) | const |  |
| [`__NR_stat`](#__nr_stat) | const |  |
| [`__NR_fstat`](#__nr_fstat) | const |  |
| [`__NR_lstat`](#__nr_lstat) | const |  |
| [`__NR_poll`](#__nr_poll) | const |  |
| [`__NR_lseek`](#__nr_lseek) | const |  |
| [`__NR_mmap`](#__nr_mmap) | const |  |
| [`__NR_mprotect`](#__nr_mprotect) | const |  |
| [`__NR_munmap`](#__nr_munmap) | const |  |
| [`__NR_brk`](#__nr_brk) | const |  |
| [`__NR_rt_sigaction`](#__nr_rt_sigaction) | const |  |
| [`__NR_rt_sigprocmask`](#__nr_rt_sigprocmask) | const |  |
| [`__NR_rt_sigreturn`](#__nr_rt_sigreturn) | const |  |
| [`__NR_ioctl`](#__nr_ioctl) | const |  |
| [`__NR_pread64`](#__nr_pread64) | const |  |
| [`__NR_pwrite64`](#__nr_pwrite64) | const |  |
| [`__NR_readv`](#__nr_readv) | const |  |
| [`__NR_writev`](#__nr_writev) | const |  |
| [`__NR_access`](#__nr_access) | const |  |
| [`__NR_pipe`](#__nr_pipe) | const |  |
| [`__NR_select`](#__nr_select) | const |  |
| [`__NR_sched_yield`](#__nr_sched_yield) | const |  |
| [`__NR_mremap`](#__nr_mremap) | const |  |
| [`__NR_msync`](#__nr_msync) | const |  |
| [`__NR_mincore`](#__nr_mincore) | const |  |
| [`__NR_madvise`](#__nr_madvise) | const |  |
| [`__NR_shmget`](#__nr_shmget) | const |  |
| [`__NR_shmat`](#__nr_shmat) | const |  |
| [`__NR_shmctl`](#__nr_shmctl) | const |  |
| [`__NR_dup`](#__nr_dup) | const |  |
| [`__NR_dup2`](#__nr_dup2) | const |  |
| [`__NR_pause`](#__nr_pause) | const |  |
| [`__NR_nanosleep`](#__nr_nanosleep) | const |  |
| [`__NR_getitimer`](#__nr_getitimer) | const |  |
| [`__NR_alarm`](#__nr_alarm) | const |  |
| [`__NR_setitimer`](#__nr_setitimer) | const |  |
| [`__NR_getpid`](#__nr_getpid) | const |  |
| [`__NR_sendfile`](#__nr_sendfile) | const |  |
| [`__NR_socket`](#__nr_socket) | const |  |
| [`__NR_connect`](#__nr_connect) | const |  |
| [`__NR_accept`](#__nr_accept) | const |  |
| [`__NR_sendto`](#__nr_sendto) | const |  |
| [`__NR_recvfrom`](#__nr_recvfrom) | const |  |
| [`__NR_sendmsg`](#__nr_sendmsg) | const |  |
| [`__NR_recvmsg`](#__nr_recvmsg) | const |  |
| [`__NR_shutdown`](#__nr_shutdown) | const |  |
| [`__NR_bind`](#__nr_bind) | const |  |
| [`__NR_listen`](#__nr_listen) | const |  |
| [`__NR_getsockname`](#__nr_getsockname) | const |  |
| [`__NR_getpeername`](#__nr_getpeername) | const |  |
| [`__NR_socketpair`](#__nr_socketpair) | const |  |
| [`__NR_setsockopt`](#__nr_setsockopt) | const |  |
| [`__NR_getsockopt`](#__nr_getsockopt) | const |  |
| [`__NR_clone`](#__nr_clone) | const |  |
| [`__NR_fork`](#__nr_fork) | const |  |
| [`__NR_vfork`](#__nr_vfork) | const |  |
| [`__NR_execve`](#__nr_execve) | const |  |
| [`__NR_exit`](#__nr_exit) | const |  |
| [`__NR_wait4`](#__nr_wait4) | const |  |
| [`__NR_kill`](#__nr_kill) | const |  |
| [`__NR_uname`](#__nr_uname) | const |  |
| [`__NR_semget`](#__nr_semget) | const |  |
| [`__NR_semop`](#__nr_semop) | const |  |
| [`__NR_semctl`](#__nr_semctl) | const |  |
| [`__NR_shmdt`](#__nr_shmdt) | const |  |
| [`__NR_msgget`](#__nr_msgget) | const |  |
| [`__NR_msgsnd`](#__nr_msgsnd) | const |  |
| [`__NR_msgrcv`](#__nr_msgrcv) | const |  |
| [`__NR_msgctl`](#__nr_msgctl) | const |  |
| [`__NR_fcntl`](#__nr_fcntl) | const |  |
| [`__NR_flock`](#__nr_flock) | const |  |
| [`__NR_fsync`](#__nr_fsync) | const |  |
| [`__NR_fdatasync`](#__nr_fdatasync) | const |  |
| [`__NR_truncate`](#__nr_truncate) | const |  |
| [`__NR_ftruncate`](#__nr_ftruncate) | const |  |
| [`__NR_getdents`](#__nr_getdents) | const |  |
| [`__NR_getcwd`](#__nr_getcwd) | const |  |
| [`__NR_chdir`](#__nr_chdir) | const |  |
| [`__NR_fchdir`](#__nr_fchdir) | const |  |
| [`__NR_rename`](#__nr_rename) | const |  |
| [`__NR_mkdir`](#__nr_mkdir) | const |  |
| [`__NR_rmdir`](#__nr_rmdir) | const |  |
| [`__NR_creat`](#__nr_creat) | const |  |
| [`__NR_link`](#__nr_link) | const |  |
| [`__NR_unlink`](#__nr_unlink) | const |  |
| [`__NR_symlink`](#__nr_symlink) | const |  |
| [`__NR_readlink`](#__nr_readlink) | const |  |
| [`__NR_chmod`](#__nr_chmod) | const |  |
| [`__NR_fchmod`](#__nr_fchmod) | const |  |
| [`__NR_chown`](#__nr_chown) | const |  |
| [`__NR_fchown`](#__nr_fchown) | const |  |
| [`__NR_lchown`](#__nr_lchown) | const |  |
| [`__NR_umask`](#__nr_umask) | const |  |
| [`__NR_gettimeofday`](#__nr_gettimeofday) | const |  |
| [`__NR_getrlimit`](#__nr_getrlimit) | const |  |
| [`__NR_getrusage`](#__nr_getrusage) | const |  |
| [`__NR_sysinfo`](#__nr_sysinfo) | const |  |
| [`__NR_times`](#__nr_times) | const |  |
| [`__NR_ptrace`](#__nr_ptrace) | const |  |
| [`__NR_getuid`](#__nr_getuid) | const |  |
| [`__NR_syslog`](#__nr_syslog) | const |  |
| [`__NR_getgid`](#__nr_getgid) | const |  |
| [`__NR_setuid`](#__nr_setuid) | const |  |
| [`__NR_setgid`](#__nr_setgid) | const |  |
| [`__NR_geteuid`](#__nr_geteuid) | const |  |
| [`__NR_getegid`](#__nr_getegid) | const |  |
| [`__NR_setpgid`](#__nr_setpgid) | const |  |
| [`__NR_getppid`](#__nr_getppid) | const |  |
| [`__NR_getpgrp`](#__nr_getpgrp) | const |  |
| [`__NR_setsid`](#__nr_setsid) | const |  |
| [`__NR_setreuid`](#__nr_setreuid) | const |  |
| [`__NR_setregid`](#__nr_setregid) | const |  |
| [`__NR_getgroups`](#__nr_getgroups) | const |  |
| [`__NR_setgroups`](#__nr_setgroups) | const |  |
| [`__NR_setresuid`](#__nr_setresuid) | const |  |
| [`__NR_getresuid`](#__nr_getresuid) | const |  |
| [`__NR_setresgid`](#__nr_setresgid) | const |  |
| [`__NR_getresgid`](#__nr_getresgid) | const |  |
| [`__NR_getpgid`](#__nr_getpgid) | const |  |
| [`__NR_setfsuid`](#__nr_setfsuid) | const |  |
| [`__NR_setfsgid`](#__nr_setfsgid) | const |  |
| [`__NR_getsid`](#__nr_getsid) | const |  |
| [`__NR_capget`](#__nr_capget) | const |  |
| [`__NR_capset`](#__nr_capset) | const |  |
| [`__NR_rt_sigpending`](#__nr_rt_sigpending) | const |  |
| [`__NR_rt_sigtimedwait`](#__nr_rt_sigtimedwait) | const |  |
| [`__NR_rt_sigqueueinfo`](#__nr_rt_sigqueueinfo) | const |  |
| [`__NR_rt_sigsuspend`](#__nr_rt_sigsuspend) | const |  |
| [`__NR_sigaltstack`](#__nr_sigaltstack) | const |  |
| [`__NR_utime`](#__nr_utime) | const |  |
| [`__NR_mknod`](#__nr_mknod) | const |  |
| [`__NR_uselib`](#__nr_uselib) | const |  |
| [`__NR_personality`](#__nr_personality) | const |  |
| [`__NR_ustat`](#__nr_ustat) | const |  |
| [`__NR_statfs`](#__nr_statfs) | const |  |
| [`__NR_fstatfs`](#__nr_fstatfs) | const |  |
| [`__NR_sysfs`](#__nr_sysfs) | const |  |
| [`__NR_getpriority`](#__nr_getpriority) | const |  |
| [`__NR_setpriority`](#__nr_setpriority) | const |  |
| [`__NR_sched_setparam`](#__nr_sched_setparam) | const |  |
| [`__NR_sched_getparam`](#__nr_sched_getparam) | const |  |
| [`__NR_sched_setscheduler`](#__nr_sched_setscheduler) | const |  |
| [`__NR_sched_getscheduler`](#__nr_sched_getscheduler) | const |  |
| [`__NR_sched_get_priority_max`](#__nr_sched_get_priority_max) | const |  |
| [`__NR_sched_get_priority_min`](#__nr_sched_get_priority_min) | const |  |
| [`__NR_sched_rr_get_interval`](#__nr_sched_rr_get_interval) | const |  |
| [`__NR_mlock`](#__nr_mlock) | const |  |
| [`__NR_munlock`](#__nr_munlock) | const |  |
| [`__NR_mlockall`](#__nr_mlockall) | const |  |
| [`__NR_munlockall`](#__nr_munlockall) | const |  |
| [`__NR_vhangup`](#__nr_vhangup) | const |  |
| [`__NR_modify_ldt`](#__nr_modify_ldt) | const |  |
| [`__NR_pivot_root`](#__nr_pivot_root) | const |  |
| [`__NR__sysctl`](#__nr__sysctl) | const |  |
| [`__NR_prctl`](#__nr_prctl) | const |  |
| [`__NR_arch_prctl`](#__nr_arch_prctl) | const |  |
| [`__NR_adjtimex`](#__nr_adjtimex) | const |  |
| [`__NR_setrlimit`](#__nr_setrlimit) | const |  |
| [`__NR_chroot`](#__nr_chroot) | const |  |
| [`__NR_sync`](#__nr_sync) | const |  |
| [`__NR_acct`](#__nr_acct) | const |  |
| [`__NR_settimeofday`](#__nr_settimeofday) | const |  |
| [`__NR_mount`](#__nr_mount) | const |  |
| [`__NR_umount2`](#__nr_umount2) | const |  |
| [`__NR_swapon`](#__nr_swapon) | const |  |
| [`__NR_swapoff`](#__nr_swapoff) | const |  |
| [`__NR_reboot`](#__nr_reboot) | const |  |
| [`__NR_sethostname`](#__nr_sethostname) | const |  |
| [`__NR_setdomainname`](#__nr_setdomainname) | const |  |
| [`__NR_iopl`](#__nr_iopl) | const |  |
| [`__NR_ioperm`](#__nr_ioperm) | const |  |
| [`__NR_create_module`](#__nr_create_module) | const |  |
| [`__NR_init_module`](#__nr_init_module) | const |  |
| [`__NR_delete_module`](#__nr_delete_module) | const |  |
| [`__NR_get_kernel_syms`](#__nr_get_kernel_syms) | const |  |
| [`__NR_query_module`](#__nr_query_module) | const |  |
| [`__NR_quotactl`](#__nr_quotactl) | const |  |
| [`__NR_nfsservctl`](#__nr_nfsservctl) | const |  |
| [`__NR_getpmsg`](#__nr_getpmsg) | const |  |
| [`__NR_putpmsg`](#__nr_putpmsg) | const |  |
| [`__NR_afs_syscall`](#__nr_afs_syscall) | const |  |
| [`__NR_tuxcall`](#__nr_tuxcall) | const |  |
| [`__NR_security`](#__nr_security) | const |  |
| [`__NR_gettid`](#__nr_gettid) | const |  |
| [`__NR_readahead`](#__nr_readahead) | const |  |
| [`__NR_setxattr`](#__nr_setxattr) | const |  |
| [`__NR_lsetxattr`](#__nr_lsetxattr) | const |  |
| [`__NR_fsetxattr`](#__nr_fsetxattr) | const |  |
| [`__NR_getxattr`](#__nr_getxattr) | const |  |
| [`__NR_lgetxattr`](#__nr_lgetxattr) | const |  |
| [`__NR_fgetxattr`](#__nr_fgetxattr) | const |  |
| [`__NR_listxattr`](#__nr_listxattr) | const |  |
| [`__NR_llistxattr`](#__nr_llistxattr) | const |  |
| [`__NR_flistxattr`](#__nr_flistxattr) | const |  |
| [`__NR_removexattr`](#__nr_removexattr) | const |  |
| [`__NR_lremovexattr`](#__nr_lremovexattr) | const |  |
| [`__NR_fremovexattr`](#__nr_fremovexattr) | const |  |
| [`__NR_tkill`](#__nr_tkill) | const |  |
| [`__NR_time`](#__nr_time) | const |  |
| [`__NR_futex`](#__nr_futex) | const |  |
| [`__NR_sched_setaffinity`](#__nr_sched_setaffinity) | const |  |
| [`__NR_sched_getaffinity`](#__nr_sched_getaffinity) | const |  |
| [`__NR_set_thread_area`](#__nr_set_thread_area) | const |  |
| [`__NR_io_setup`](#__nr_io_setup) | const |  |
| [`__NR_io_destroy`](#__nr_io_destroy) | const |  |
| [`__NR_io_getevents`](#__nr_io_getevents) | const |  |
| [`__NR_io_submit`](#__nr_io_submit) | const |  |
| [`__NR_io_cancel`](#__nr_io_cancel) | const |  |
| [`__NR_get_thread_area`](#__nr_get_thread_area) | const |  |
| [`__NR_lookup_dcookie`](#__nr_lookup_dcookie) | const |  |
| [`__NR_epoll_create`](#__nr_epoll_create) | const |  |
| [`__NR_epoll_ctl_old`](#__nr_epoll_ctl_old) | const |  |
| [`__NR_epoll_wait_old`](#__nr_epoll_wait_old) | const |  |
| [`__NR_remap_file_pages`](#__nr_remap_file_pages) | const |  |
| [`__NR_getdents64`](#__nr_getdents64) | const |  |
| [`__NR_set_tid_address`](#__nr_set_tid_address) | const |  |
| [`__NR_restart_syscall`](#__nr_restart_syscall) | const |  |
| [`__NR_semtimedop`](#__nr_semtimedop) | const |  |
| [`__NR_fadvise64`](#__nr_fadvise64) | const |  |
| [`__NR_timer_create`](#__nr_timer_create) | const |  |
| [`__NR_timer_settime`](#__nr_timer_settime) | const |  |
| [`__NR_timer_gettime`](#__nr_timer_gettime) | const |  |
| [`__NR_timer_getoverrun`](#__nr_timer_getoverrun) | const |  |
| [`__NR_timer_delete`](#__nr_timer_delete) | const |  |
| [`__NR_clock_settime`](#__nr_clock_settime) | const |  |
| [`__NR_clock_gettime`](#__nr_clock_gettime) | const |  |
| [`__NR_clock_getres`](#__nr_clock_getres) | const |  |
| [`__NR_clock_nanosleep`](#__nr_clock_nanosleep) | const |  |
| [`__NR_exit_group`](#__nr_exit_group) | const |  |
| [`__NR_epoll_wait`](#__nr_epoll_wait) | const |  |
| [`__NR_epoll_ctl`](#__nr_epoll_ctl) | const |  |
| [`__NR_tgkill`](#__nr_tgkill) | const |  |
| [`__NR_utimes`](#__nr_utimes) | const |  |
| [`__NR_vserver`](#__nr_vserver) | const |  |
| [`__NR_mbind`](#__nr_mbind) | const |  |
| [`__NR_set_mempolicy`](#__nr_set_mempolicy) | const |  |
| [`__NR_get_mempolicy`](#__nr_get_mempolicy) | const |  |
| [`__NR_mq_open`](#__nr_mq_open) | const |  |
| [`__NR_mq_unlink`](#__nr_mq_unlink) | const |  |
| [`__NR_mq_timedsend`](#__nr_mq_timedsend) | const |  |
| [`__NR_mq_timedreceive`](#__nr_mq_timedreceive) | const |  |
| [`__NR_mq_notify`](#__nr_mq_notify) | const |  |
| [`__NR_mq_getsetattr`](#__nr_mq_getsetattr) | const |  |
| [`__NR_kexec_load`](#__nr_kexec_load) | const |  |
| [`__NR_waitid`](#__nr_waitid) | const |  |
| [`__NR_add_key`](#__nr_add_key) | const |  |
| [`__NR_request_key`](#__nr_request_key) | const |  |
| [`__NR_keyctl`](#__nr_keyctl) | const |  |
| [`__NR_ioprio_set`](#__nr_ioprio_set) | const |  |
| [`__NR_ioprio_get`](#__nr_ioprio_get) | const |  |
| [`__NR_inotify_init`](#__nr_inotify_init) | const |  |
| [`__NR_inotify_add_watch`](#__nr_inotify_add_watch) | const |  |
| [`__NR_inotify_rm_watch`](#__nr_inotify_rm_watch) | const |  |
| [`__NR_migrate_pages`](#__nr_migrate_pages) | const |  |
| [`__NR_openat`](#__nr_openat) | const |  |
| [`__NR_mkdirat`](#__nr_mkdirat) | const |  |
| [`__NR_mknodat`](#__nr_mknodat) | const |  |
| [`__NR_fchownat`](#__nr_fchownat) | const |  |
| [`__NR_futimesat`](#__nr_futimesat) | const |  |
| [`__NR_newfstatat`](#__nr_newfstatat) | const |  |
| [`__NR_unlinkat`](#__nr_unlinkat) | const |  |
| [`__NR_renameat`](#__nr_renameat) | const |  |
| [`__NR_linkat`](#__nr_linkat) | const |  |
| [`__NR_symlinkat`](#__nr_symlinkat) | const |  |
| [`__NR_readlinkat`](#__nr_readlinkat) | const |  |
| [`__NR_fchmodat`](#__nr_fchmodat) | const |  |
| [`__NR_faccessat`](#__nr_faccessat) | const |  |
| [`__NR_pselect6`](#__nr_pselect6) | const |  |
| [`__NR_ppoll`](#__nr_ppoll) | const |  |
| [`__NR_unshare`](#__nr_unshare) | const |  |
| [`__NR_set_robust_list`](#__nr_set_robust_list) | const |  |
| [`__NR_get_robust_list`](#__nr_get_robust_list) | const |  |
| [`__NR_splice`](#__nr_splice) | const |  |
| [`__NR_tee`](#__nr_tee) | const |  |
| [`__NR_sync_file_range`](#__nr_sync_file_range) | const |  |
| [`__NR_vmsplice`](#__nr_vmsplice) | const |  |
| [`__NR_move_pages`](#__nr_move_pages) | const |  |
| [`__NR_utimensat`](#__nr_utimensat) | const |  |
| [`__NR_epoll_pwait`](#__nr_epoll_pwait) | const |  |
| [`__NR_signalfd`](#__nr_signalfd) | const |  |
| [`__NR_timerfd_create`](#__nr_timerfd_create) | const |  |
| [`__NR_eventfd`](#__nr_eventfd) | const |  |
| [`__NR_fallocate`](#__nr_fallocate) | const |  |
| [`__NR_timerfd_settime`](#__nr_timerfd_settime) | const |  |
| [`__NR_timerfd_gettime`](#__nr_timerfd_gettime) | const |  |
| [`__NR_accept4`](#__nr_accept4) | const |  |
| [`__NR_signalfd4`](#__nr_signalfd4) | const |  |
| [`__NR_eventfd2`](#__nr_eventfd2) | const |  |
| [`__NR_epoll_create1`](#__nr_epoll_create1) | const |  |
| [`__NR_dup3`](#__nr_dup3) | const |  |
| [`__NR_pipe2`](#__nr_pipe2) | const |  |
| [`__NR_inotify_init1`](#__nr_inotify_init1) | const |  |
| [`__NR_preadv`](#__nr_preadv) | const |  |
| [`__NR_pwritev`](#__nr_pwritev) | const |  |
| [`__NR_rt_tgsigqueueinfo`](#__nr_rt_tgsigqueueinfo) | const |  |
| [`__NR_perf_event_open`](#__nr_perf_event_open) | const |  |
| [`__NR_recvmmsg`](#__nr_recvmmsg) | const |  |
| [`__NR_fanotify_init`](#__nr_fanotify_init) | const |  |
| [`__NR_fanotify_mark`](#__nr_fanotify_mark) | const |  |
| [`__NR_prlimit64`](#__nr_prlimit64) | const |  |
| [`__NR_name_to_handle_at`](#__nr_name_to_handle_at) | const |  |
| [`__NR_open_by_handle_at`](#__nr_open_by_handle_at) | const |  |
| [`__NR_clock_adjtime`](#__nr_clock_adjtime) | const |  |
| [`__NR_syncfs`](#__nr_syncfs) | const |  |
| [`__NR_sendmmsg`](#__nr_sendmmsg) | const |  |
| [`__NR_setns`](#__nr_setns) | const |  |
| [`__NR_getcpu`](#__nr_getcpu) | const |  |
| [`__NR_process_vm_readv`](#__nr_process_vm_readv) | const |  |
| [`__NR_process_vm_writev`](#__nr_process_vm_writev) | const |  |
| [`__NR_kcmp`](#__nr_kcmp) | const |  |
| [`__NR_finit_module`](#__nr_finit_module) | const |  |
| [`__NR_sched_setattr`](#__nr_sched_setattr) | const |  |
| [`__NR_sched_getattr`](#__nr_sched_getattr) | const |  |
| [`__NR_renameat2`](#__nr_renameat2) | const |  |
| [`__NR_seccomp`](#__nr_seccomp) | const |  |
| [`__NR_getrandom`](#__nr_getrandom) | const |  |
| [`__NR_memfd_create`](#__nr_memfd_create) | const |  |
| [`__NR_kexec_file_load`](#__nr_kexec_file_load) | const |  |
| [`__NR_bpf`](#__nr_bpf) | const |  |
| [`__NR_execveat`](#__nr_execveat) | const |  |
| [`__NR_userfaultfd`](#__nr_userfaultfd) | const |  |
| [`__NR_membarrier`](#__nr_membarrier) | const |  |
| [`__NR_mlock2`](#__nr_mlock2) | const |  |
| [`__NR_copy_file_range`](#__nr_copy_file_range) | const |  |
| [`__NR_preadv2`](#__nr_preadv2) | const |  |
| [`__NR_pwritev2`](#__nr_pwritev2) | const |  |
| [`__NR_pkey_mprotect`](#__nr_pkey_mprotect) | const |  |
| [`__NR_pkey_alloc`](#__nr_pkey_alloc) | const |  |
| [`__NR_pkey_free`](#__nr_pkey_free) | const |  |
| [`__NR_statx`](#__nr_statx) | const |  |
| [`__NR_io_pgetevents`](#__nr_io_pgetevents) | const |  |
| [`__NR_rseq`](#__nr_rseq) | const |  |
| [`__NR_uretprobe`](#__nr_uretprobe) | const |  |
| [`__NR_pidfd_send_signal`](#__nr_pidfd_send_signal) | const |  |
| [`__NR_io_uring_setup`](#__nr_io_uring_setup) | const |  |
| [`__NR_io_uring_enter`](#__nr_io_uring_enter) | const |  |
| [`__NR_io_uring_register`](#__nr_io_uring_register) | const |  |
| [`__NR_open_tree`](#__nr_open_tree) | const |  |
| [`__NR_move_mount`](#__nr_move_mount) | const |  |
| [`__NR_fsopen`](#__nr_fsopen) | const |  |
| [`__NR_fsconfig`](#__nr_fsconfig) | const |  |
| [`__NR_fsmount`](#__nr_fsmount) | const |  |
| [`__NR_fspick`](#__nr_fspick) | const |  |
| [`__NR_pidfd_open`](#__nr_pidfd_open) | const |  |
| [`__NR_clone3`](#__nr_clone3) | const |  |
| [`__NR_close_range`](#__nr_close_range) | const |  |
| [`__NR_openat2`](#__nr_openat2) | const |  |
| [`__NR_pidfd_getfd`](#__nr_pidfd_getfd) | const |  |
| [`__NR_faccessat2`](#__nr_faccessat2) | const |  |
| [`__NR_process_madvise`](#__nr_process_madvise) | const |  |
| [`__NR_epoll_pwait2`](#__nr_epoll_pwait2) | const |  |
| [`__NR_mount_setattr`](#__nr_mount_setattr) | const |  |
| [`__NR_quotactl_fd`](#__nr_quotactl_fd) | const |  |
| [`__NR_landlock_create_ruleset`](#__nr_landlock_create_ruleset) | const |  |
| [`__NR_landlock_add_rule`](#__nr_landlock_add_rule) | const |  |
| [`__NR_landlock_restrict_self`](#__nr_landlock_restrict_self) | const |  |
| [`__NR_memfd_secret`](#__nr_memfd_secret) | const |  |
| [`__NR_process_mrelease`](#__nr_process_mrelease) | const |  |
| [`__NR_futex_waitv`](#__nr_futex_waitv) | const |  |
| [`__NR_set_mempolicy_home_node`](#__nr_set_mempolicy_home_node) | const |  |
| [`__NR_cachestat`](#__nr_cachestat) | const |  |
| [`__NR_fchmodat2`](#__nr_fchmodat2) | const |  |
| [`__NR_map_shadow_stack`](#__nr_map_shadow_stack) | const |  |
| [`__NR_futex_wake`](#__nr_futex_wake) | const |  |
| [`__NR_futex_wait`](#__nr_futex_wait) | const |  |
| [`__NR_futex_requeue`](#__nr_futex_requeue) | const |  |
| [`__NR_statmount`](#__nr_statmount) | const |  |
| [`__NR_listmount`](#__nr_listmount) | const |  |
| [`__NR_lsm_get_self_attr`](#__nr_lsm_get_self_attr) | const |  |
| [`__NR_lsm_set_self_attr`](#__nr_lsm_set_self_attr) | const |  |
| [`__NR_lsm_list_modules`](#__nr_lsm_list_modules) | const |  |
| [`__NR_mseal`](#__nr_mseal) | const |  |
| [`__NR_setxattrat`](#__nr_setxattrat) | const |  |
| [`__NR_getxattrat`](#__nr_getxattrat) | const |  |
| [`__NR_listxattrat`](#__nr_listxattrat) | const |  |
| [`__NR_removexattrat`](#__nr_removexattrat) | const |  |
| [`__NR_open_tree_attr`](#__nr_open_tree_attr) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`__WNOTHREAD`](#__wnothread) | const |  |
| [`__WALL`](#__wall) | const |  |
| [`__WCLONE`](#__wclone) | const |  |
| [`P_ALL`](#p_all) | const |  |
| [`P_PID`](#p_pid) | const |  |
| [`P_PGID`](#p_pgid) | const |  |
| [`P_PIDFD`](#p_pidfd) | const |  |
| [`XATTR_CREATE`](#xattr_create) | const |  |
| [`XATTR_REPLACE`](#xattr_replace) | const |  |
| [`XATTR_OS2_PREFIX`](#xattr_os2_prefix) | const |  |
| [`XATTR_MAC_OSX_PREFIX`](#xattr_mac_osx_prefix) | const |  |
| [`XATTR_BTRFS_PREFIX`](#xattr_btrfs_prefix) | const |  |
| [`XATTR_HURD_PREFIX`](#xattr_hurd_prefix) | const |  |
| [`XATTR_SECURITY_PREFIX`](#xattr_security_prefix) | const |  |
| [`XATTR_SYSTEM_PREFIX`](#xattr_system_prefix) | const |  |
| [`XATTR_TRUSTED_PREFIX`](#xattr_trusted_prefix) | const |  |
| [`XATTR_USER_PREFIX`](#xattr_user_prefix) | const |  |
| [`XATTR_EVM_SUFFIX`](#xattr_evm_suffix) | const |  |
| [`XATTR_NAME_EVM`](#xattr_name_evm) | const |  |
| [`XATTR_IMA_SUFFIX`](#xattr_ima_suffix) | const |  |
| [`XATTR_NAME_IMA`](#xattr_name_ima) | const |  |
| [`XATTR_SELINUX_SUFFIX`](#xattr_selinux_suffix) | const |  |
| [`XATTR_NAME_SELINUX`](#xattr_name_selinux) | const |  |
| [`XATTR_SMACK_SUFFIX`](#xattr_smack_suffix) | const |  |
| [`XATTR_SMACK_IPIN`](#xattr_smack_ipin) | const |  |
| [`XATTR_SMACK_IPOUT`](#xattr_smack_ipout) | const |  |
| [`XATTR_SMACK_EXEC`](#xattr_smack_exec) | const |  |
| [`XATTR_SMACK_TRANSMUTE`](#xattr_smack_transmute) | const |  |
| [`XATTR_SMACK_MMAP`](#xattr_smack_mmap) | const |  |
| [`XATTR_NAME_SMACK`](#xattr_name_smack) | const |  |
| [`XATTR_NAME_SMACKIPIN`](#xattr_name_smackipin) | const |  |
| [`XATTR_NAME_SMACKIPOUT`](#xattr_name_smackipout) | const |  |
| [`XATTR_NAME_SMACKEXEC`](#xattr_name_smackexec) | const |  |
| [`XATTR_NAME_SMACKTRANSMUTE`](#xattr_name_smacktransmute) | const |  |
| [`XATTR_NAME_SMACKMMAP`](#xattr_name_smackmmap) | const |  |
| [`XATTR_APPARMOR_SUFFIX`](#xattr_apparmor_suffix) | const |  |
| [`XATTR_NAME_APPARMOR`](#xattr_name_apparmor) | const |  |
| [`XATTR_CAPS_SUFFIX`](#xattr_caps_suffix) | const |  |
| [`XATTR_NAME_CAPS`](#xattr_name_caps) | const |  |
| [`XATTR_BPF_LSM_SUFFIX`](#xattr_bpf_lsm_suffix) | const |  |
| [`XATTR_NAME_BPF_LSM`](#xattr_name_bpf_lsm) | const |  |
| [`XATTR_POSIX_ACL_ACCESS`](#xattr_posix_acl_access) | const |  |
| [`XATTR_NAME_POSIX_ACL_ACCESS`](#xattr_name_posix_acl_access) | const |  |
| [`XATTR_POSIX_ACL_DEFAULT`](#xattr_posix_acl_default) | const |  |
| [`XATTR_NAME_POSIX_ACL_DEFAULT`](#xattr_name_posix_acl_default) | const |  |
| [`MFD_CLOEXEC`](#mfd_cloexec) | const |  |
| [`MFD_ALLOW_SEALING`](#mfd_allow_sealing) | const |  |
| [`MFD_HUGETLB`](#mfd_hugetlb) | const |  |
| [`MFD_NOEXEC_SEAL`](#mfd_noexec_seal) | const |  |
| [`MFD_EXEC`](#mfd_exec) | const |  |
| [`MFD_HUGE_SHIFT`](#mfd_huge_shift) | const |  |
| [`MFD_HUGE_MASK`](#mfd_huge_mask) | const |  |
| [`MFD_HUGE_64KB`](#mfd_huge_64kb) | const |  |
| [`MFD_HUGE_512KB`](#mfd_huge_512kb) | const |  |
| [`MFD_HUGE_1MB`](#mfd_huge_1mb) | const |  |
| [`MFD_HUGE_2MB`](#mfd_huge_2mb) | const |  |
| [`MFD_HUGE_8MB`](#mfd_huge_8mb) | const |  |
| [`MFD_HUGE_16MB`](#mfd_huge_16mb) | const |  |
| [`MFD_HUGE_32MB`](#mfd_huge_32mb) | const |  |
| [`MFD_HUGE_256MB`](#mfd_huge_256mb) | const |  |
| [`MFD_HUGE_512MB`](#mfd_huge_512mb) | const |  |
| [`MFD_HUGE_1GB`](#mfd_huge_1gb) | const |  |
| [`MFD_HUGE_2GB`](#mfd_huge_2gb) | const |  |
| [`MFD_HUGE_16GB`](#mfd_huge_16gb) | const |  |
| [`TFD_TIMER_ABSTIME`](#tfd_timer_abstime) | const |  |
| [`TFD_TIMER_CANCEL_ON_SET`](#tfd_timer_cancel_on_set) | const |  |
| [`TFD_CLOEXEC`](#tfd_cloexec) | const |  |
| [`TFD_NONBLOCK`](#tfd_nonblock) | const |  |
| [`USERFAULTFD_IOC`](#userfaultfd_ioc) | const |  |
| [`_UFFDIO_REGISTER`](#_uffdio_register) | const |  |
| [`_UFFDIO_UNREGISTER`](#_uffdio_unregister) | const |  |
| [`_UFFDIO_WAKE`](#_uffdio_wake) | const |  |
| [`_UFFDIO_COPY`](#_uffdio_copy) | const |  |
| [`_UFFDIO_ZEROPAGE`](#_uffdio_zeropage) | const |  |
| [`_UFFDIO_MOVE`](#_uffdio_move) | const |  |
| [`_UFFDIO_WRITEPROTECT`](#_uffdio_writeprotect) | const |  |
| [`_UFFDIO_CONTINUE`](#_uffdio_continue) | const |  |
| [`_UFFDIO_POISON`](#_uffdio_poison) | const |  |
| [`_UFFDIO_API`](#_uffdio_api) | const |  |
| [`UFFDIO`](#uffdio) | const |  |
| [`UFFD_EVENT_PAGEFAULT`](#uffd_event_pagefault) | const |  |
| [`UFFD_EVENT_FORK`](#uffd_event_fork) | const |  |
| [`UFFD_EVENT_REMAP`](#uffd_event_remap) | const |  |
| [`UFFD_EVENT_REMOVE`](#uffd_event_remove) | const |  |
| [`UFFD_EVENT_UNMAP`](#uffd_event_unmap) | const |  |
| [`UFFD_PAGEFAULT_FLAG_WRITE`](#uffd_pagefault_flag_write) | const |  |
| [`UFFD_PAGEFAULT_FLAG_WP`](#uffd_pagefault_flag_wp) | const |  |
| [`UFFD_PAGEFAULT_FLAG_MINOR`](#uffd_pagefault_flag_minor) | const |  |
| [`UFFD_FEATURE_PAGEFAULT_FLAG_WP`](#uffd_feature_pagefault_flag_wp) | const |  |
| [`UFFD_FEATURE_EVENT_FORK`](#uffd_feature_event_fork) | const |  |
| [`UFFD_FEATURE_EVENT_REMAP`](#uffd_feature_event_remap) | const |  |
| [`UFFD_FEATURE_EVENT_REMOVE`](#uffd_feature_event_remove) | const |  |
| [`UFFD_FEATURE_MISSING_HUGETLBFS`](#uffd_feature_missing_hugetlbfs) | const |  |
| [`UFFD_FEATURE_MISSING_SHMEM`](#uffd_feature_missing_shmem) | const |  |
| [`UFFD_FEATURE_EVENT_UNMAP`](#uffd_feature_event_unmap) | const |  |
| [`UFFD_FEATURE_SIGBUS`](#uffd_feature_sigbus) | const |  |
| [`UFFD_FEATURE_THREAD_ID`](#uffd_feature_thread_id) | const |  |
| [`UFFD_FEATURE_MINOR_HUGETLBFS`](#uffd_feature_minor_hugetlbfs) | const |  |
| [`UFFD_FEATURE_MINOR_SHMEM`](#uffd_feature_minor_shmem) | const |  |
| [`UFFD_FEATURE_EXACT_ADDRESS`](#uffd_feature_exact_address) | const |  |
| [`UFFD_FEATURE_WP_HUGETLBFS_SHMEM`](#uffd_feature_wp_hugetlbfs_shmem) | const |  |
| [`UFFD_FEATURE_WP_UNPOPULATED`](#uffd_feature_wp_unpopulated) | const |  |
| [`UFFD_FEATURE_POISON`](#uffd_feature_poison) | const |  |
| [`UFFD_FEATURE_WP_ASYNC`](#uffd_feature_wp_async) | const |  |
| [`UFFD_FEATURE_MOVE`](#uffd_feature_move) | const |  |
| [`UFFD_USER_MODE_ONLY`](#uffd_user_mode_only) | const |  |
| [`DT_UNKNOWN`](#dt_unknown) | const |  |
| [`DT_FIFO`](#dt_fifo) | const |  |
| [`DT_CHR`](#dt_chr) | const |  |
| [`DT_DIR`](#dt_dir) | const |  |
| [`DT_BLK`](#dt_blk) | const |  |
| [`DT_REG`](#dt_reg) | const |  |
| [`DT_LNK`](#dt_lnk) | const |  |
| [`DT_SOCK`](#dt_sock) | const |  |
| [`STAT_HAVE_NSEC`](#stat_have_nsec) | const |  |
| [`F_OK`](#f_ok) | const |  |
| [`R_OK`](#r_ok) | const |  |
| [`W_OK`](#w_ok) | const |  |
| [`X_OK`](#x_ok) | const |  |
| [`UTIME_NOW`](#utime_now) | const |  |
| [`UTIME_OMIT`](#utime_omit) | const |  |
| [`MNT_FORCE`](#mnt_force) | const |  |
| [`MNT_DETACH`](#mnt_detach) | const |  |
| [`MNT_EXPIRE`](#mnt_expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount_nofollow) | const |  |
| [`UMOUNT_UNUSED`](#umount_unused) | const |  |
| [`STDIN_FILENO`](#stdin_fileno) | const |  |
| [`STDOUT_FILENO`](#stdout_fileno) | const |  |
| [`STDERR_FILENO`](#stderr_fileno) | const |  |
| [`RWF_HIPRI`](#rwf_hipri) | const |  |
| [`RWF_DSYNC`](#rwf_dsync) | const |  |
| [`RWF_SYNC`](#rwf_sync) | const |  |
| [`RWF_NOWAIT`](#rwf_nowait) | const |  |
| [`RWF_APPEND`](#rwf_append) | const |  |
| [`EFD_SEMAPHORE`](#efd_semaphore) | const |  |
| [`EFD_CLOEXEC`](#efd_cloexec) | const |  |
| [`EFD_NONBLOCK`](#efd_nonblock) | const |  |
| [`EPOLLIN`](#epollin) | const |  |
| [`EPOLLPRI`](#epollpri) | const |  |
| [`EPOLLOUT`](#epollout) | const |  |
| [`EPOLLERR`](#epollerr) | const |  |
| [`EPOLLHUP`](#epollhup) | const |  |
| [`EPOLLNVAL`](#epollnval) | const |  |
| [`EPOLLRDNORM`](#epollrdnorm) | const |  |
| [`EPOLLRDBAND`](#epollrdband) | const |  |
| [`EPOLLWRNORM`](#epollwrnorm) | const |  |
| [`EPOLLWRBAND`](#epollwrband) | const |  |
| [`EPOLLMSG`](#epollmsg) | const |  |
| [`EPOLLRDHUP`](#epollrdhup) | const |  |
| [`EPOLLEXCLUSIVE`](#epollexclusive) | const |  |
| [`EPOLLWAKEUP`](#epollwakeup) | const |  |
| [`EPOLLONESHOT`](#epolloneshot) | const |  |
| [`EPOLLET`](#epollet) | const |  |
| [`TFD_SHARED_FCNTL_FLAGS`](#tfd_shared_fcntl_flags) | const |  |
| [`TFD_CREATE_FLAGS`](#tfd_create_flags) | const |  |
| [`TFD_SETTIME_FLAGS`](#tfd_settime_flags) | const |  |
| [`ARCH_SET_FS`](#arch_set_fs) | const |  |
| [`UFFD_API`](#uffd_api) | const |  |
| [`UFFDIO_REGISTER_MODE_MISSING`](#uffdio_register_mode_missing) | const |  |
| [`UFFDIO_REGISTER_MODE_WP`](#uffdio_register_mode_wp) | const |  |
| [`UFFDIO_REGISTER_MODE_MINOR`](#uffdio_register_mode_minor) | const |  |
| [`UFFDIO_COPY_MODE_DONTWAKE`](#uffdio_copy_mode_dontwake) | const |  |
| [`UFFDIO_COPY_MODE_WP`](#uffdio_copy_mode_wp) | const |  |
| [`UFFDIO_ZEROPAGE_MODE_DONTWAKE`](#uffdio_zeropage_mode_dontwake) | const |  |
| [`SPLICE_F_MOVE`](#splice_f_move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice_f_nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice_f_more) | const |  |
| [`SPLICE_F_GIFT`](#splice_f_gift) | const |  |
| [`_NSIG`](#_nsig) | const |  |

## Structs

### `__BindgenBitfieldUnit<Storage>`

```rust
struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
```

#### Implementations

- <span id="bindgenbitfieldunit-extract-bit"></span>`fn extract_bit(byte: u8, index: usize) -> bool`

- <span id="bindgenbitfieldunit-get-bit"></span>`fn get_bit(&self, index: usize) -> bool`

- <span id="bindgenbitfieldunit-raw-get-bit"></span>`unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool`

- <span id="bindgenbitfieldunit-change-bit"></span>`fn change_bit(byte: u8, index: usize, val: bool) -> u8`

- <span id="bindgenbitfieldunit-set-bit"></span>`fn set_bit(&mut self, index: usize, val: bool)`

- <span id="bindgenbitfieldunit-raw-set-bit"></span>`unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool)`

- <span id="bindgenbitfieldunit-get"></span>`fn get(&self, bit_offset: usize, bit_width: u8) -> u64`

- <span id="bindgenbitfieldunit-raw-get"></span>`unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64`

- <span id="bindgenbitfieldunit-set"></span>`fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64)`

- <span id="bindgenbitfieldunit-raw-set"></span>`unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64)`

#### Trait Implementations

##### `impl<Storage: clone::Clone> Clone for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-clone"></span>`fn clone(&self) -> __BindgenBitfieldUnit<Storage>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: marker::Copy> Copy for __BindgenBitfieldUnit<Storage>`

##### `impl<Storage: fmt::Debug> Debug for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Storage: default::Default> Default for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-default"></span>`fn default() -> __BindgenBitfieldUnit<Storage>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::Eq> Eq for __BindgenBitfieldUnit<Storage>`

##### `impl<Storage: hash::Hash> Hash for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Storage: cmp::Ord> Ord for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-cmp"></span>`fn cmp(&self, other: &__BindgenBitfieldUnit<Storage>) -> cmp::Ordering` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::PartialEq> PartialEq for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-eq"></span>`fn eq(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::PartialOrd> PartialOrd for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-partial-cmp"></span>`fn partial_cmp(&self, other: &__BindgenBitfieldUnit<Storage>) -> option::Option<cmp::Ordering>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage> StructuralPartialEq for __BindgenBitfieldUnit<Storage>`

### `__IncompleteArrayField<T>`

```rust
struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
```

#### Implementations

- <span id="incompletearrayfield-new"></span>`const fn new() -> Self`

- <span id="incompletearrayfield-as-ptr"></span>`fn as_ptr(&self) -> *const T`

- <span id="incompletearrayfield-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut T`

- <span id="incompletearrayfield-as-slice"></span>`unsafe fn as_slice(&self, len: usize) -> &[T]`

- <span id="incompletearrayfield-as-mut-slice"></span>`unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T]`

#### Trait Implementations

##### `impl<T> Debug for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-fmt"></span>`fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl<T: default::Default> Default for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-default"></span>`fn default() -> __IncompleteArrayField<T>` — [`__IncompleteArrayField`](#incompletearrayfield)

### `__kernel_fd_set`

```rust
struct __kernel_fd_set {
    pub fds_bits: [crate::ctypes::c_ulong; 16],
}
```

#### Trait Implementations

##### `impl Clone for __kernel_fd_set`

- <span id="kernel-fd-set-clone"></span>`fn clone(&self) -> __kernel_fd_set` — [`__kernel_fd_set`](#kernel-fd-set)

##### `impl Copy for __kernel_fd_set`

##### `impl Debug for __kernel_fd_set`

- <span id="kernel-fd-set-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_fsid_t`

```rust
struct __kernel_fsid_t {
    pub val: [crate::ctypes::c_int; 2],
}
```

#### Trait Implementations

##### `impl Clone for __kernel_fsid_t`

- <span id="kernel-fsid-t-clone"></span>`fn clone(&self) -> __kernel_fsid_t` — [`__kernel_fsid_t`](#kernel-fsid-t)

##### `impl Copy for __kernel_fsid_t`

##### `impl Debug for __kernel_fsid_t`

- <span id="kernel-fsid-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__user_cap_header_struct`

```rust
struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl Clone for __user_cap_header_struct`

- <span id="user-cap-header-struct-clone"></span>`fn clone(&self) -> __user_cap_header_struct` — [`__user_cap_header_struct`](#user-cap-header-struct)

##### `impl Copy for __user_cap_header_struct`

##### `impl Debug for __user_cap_header_struct`

- <span id="user-cap-header-struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__user_cap_data_struct`

```rust
struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
```

#### Trait Implementations

##### `impl Clone for __user_cap_data_struct`

- <span id="user-cap-data-struct-clone"></span>`fn clone(&self) -> __user_cap_data_struct` — [`__user_cap_data_struct`](#user-cap-data-struct)

##### `impl Copy for __user_cap_data_struct`

##### `impl Debug for __user_cap_data_struct`

- <span id="user-cap-data-struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `vfs_cap_data`

```rust
struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data__bindgen_ty_1; 2],
}
```

#### Trait Implementations

##### `impl Clone for vfs_cap_data`

- <span id="vfs-cap-data-clone"></span>`fn clone(&self) -> vfs_cap_data` — [`vfs_cap_data`](#vfs-cap-data)

##### `impl Copy for vfs_cap_data`

##### `impl Debug for vfs_cap_data`

- <span id="vfs-cap-data-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `vfs_cap_data__bindgen_ty_1`

```rust
struct vfs_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

#### Trait Implementations

##### `impl Clone for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-clone"></span>`fn clone(&self) -> vfs_cap_data__bindgen_ty_1` — [`vfs_cap_data__bindgen_ty_1`](#vfs-cap-data-bindgen-ty-1)

##### `impl Copy for vfs_cap_data__bindgen_ty_1`

##### `impl Debug for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `vfs_ns_cap_data`

```rust
struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_ns_cap_data__bindgen_ty_1; 2],
    pub rootid: __le32,
}
```

#### Trait Implementations

##### `impl Clone for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-clone"></span>`fn clone(&self) -> vfs_ns_cap_data` — [`vfs_ns_cap_data`](#vfs-ns-cap-data)

##### `impl Copy for vfs_ns_cap_data`

##### `impl Debug for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `vfs_ns_cap_data__bindgen_ty_1`

```rust
struct vfs_ns_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

#### Trait Implementations

##### `impl Clone for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-clone"></span>`fn clone(&self) -> vfs_ns_cap_data__bindgen_ty_1` — [`vfs_ns_cap_data__bindgen_ty_1`](#vfs-ns-cap-data-bindgen-ty-1)

##### `impl Copy for vfs_ns_cap_data__bindgen_ty_1`

##### `impl Debug for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `f_owner_ex`

```rust
struct f_owner_ex {
    pub type_: crate::ctypes::c_int,
    pub pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl Clone for f_owner_ex`

- <span id="f-owner-ex-clone"></span>`fn clone(&self) -> f_owner_ex` — [`f_owner_ex`](#f-owner-ex)

##### `impl Copy for f_owner_ex`

##### `impl Debug for f_owner_ex`

- <span id="f-owner-ex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `flock`

```rust
struct flock {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock`

- <span id="flock-clone"></span>`fn clone(&self) -> flock` — [`flock`](#flock)

##### `impl Copy for flock`

##### `impl Debug for flock`

- <span id="flock-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `flock64`

```rust
struct flock64 {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock64`

- <span id="flock64-clone"></span>`fn clone(&self) -> flock64` — [`flock64`](#flock64)

##### `impl Copy for flock64`

##### `impl Debug for flock64`

- <span id="flock64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `open_how`

```rust
struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}
```

#### Trait Implementations

##### `impl Clone for open_how`

- <span id="open-how-clone"></span>`fn clone(&self) -> open_how` — [`open_how`](#open-how)

##### `impl Copy for open_how`

##### `impl Debug for open_how`

- <span id="open-how-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: __poll_t,
    pub data: __u64,
}
```

#### Trait Implementations

##### `impl Clone for epoll_event`

- <span id="epoll-event-clone"></span>`fn clone(&self) -> epoll_event` — [`epoll_event`](#epoll-event)

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- <span id="epoll-event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_params`

```rust
struct epoll_params {
    pub busy_poll_usecs: __u32,
    pub busy_poll_budget: __u16,
    pub prefer_busy_poll: __u8,
    pub __pad: __u8,
}
```

#### Trait Implementations

##### `impl Clone for epoll_params`

- <span id="epoll-params-clone"></span>`fn clone(&self) -> epoll_params` — [`epoll_params`](#epoll-params)

##### `impl Copy for epoll_params`

##### `impl Debug for epoll_params`

- <span id="epoll-params-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fscrypt_policy_v1`

```rust
struct fscrypt_policy_v1 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub master_key_descriptor: [__u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-clone"></span>`fn clone(&self) -> fscrypt_policy_v1` — [`fscrypt_policy_v1`](#fscrypt-policy-v1)

##### `impl Copy for fscrypt_policy_v1`

##### `impl Debug for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fscrypt_key`

```rust
struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; 64],
    pub size: __u32,
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_key`

- <span id="fscrypt-key-clone"></span>`fn clone(&self) -> fscrypt_key` — [`fscrypt_key`](#fscrypt-key)

##### `impl Copy for fscrypt_key`

##### `impl Debug for fscrypt_key`

- <span id="fscrypt-key-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fscrypt_policy_v2`

```rust
struct fscrypt_policy_v2 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub log2_data_unit_size: __u8,
    pub __reserved: [__u8; 3],
    pub master_key_identifier: [__u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-clone"></span>`fn clone(&self) -> fscrypt_policy_v2` — [`fscrypt_policy_v2`](#fscrypt-policy-v2)

##### `impl Copy for fscrypt_policy_v2`

##### `impl Debug for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fscrypt_get_policy_ex_arg`

```rust
struct fscrypt_get_policy_ex_arg {
    pub policy_size: __u64,
    pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-clone"></span>`fn clone(&self) -> fscrypt_get_policy_ex_arg` — [`fscrypt_get_policy_ex_arg`](#fscrypt-get-policy-ex-arg)

##### `impl Copy for fscrypt_get_policy_ex_arg`

### `fscrypt_key_specifier`

```rust
struct fscrypt_key_specifier {
    pub type_: __u32,
    pub __reserved: __u32,
    pub u: fscrypt_key_specifier__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-clone"></span>`fn clone(&self) -> fscrypt_key_specifier` — [`fscrypt_key_specifier`](#fscrypt-key-specifier)

##### `impl Copy for fscrypt_key_specifier`

### `fscrypt_provisioning_key_payload`

```rust
struct fscrypt_provisioning_key_payload {
    pub type_: __u32,
    pub flags: __u32,
    pub raw: __IncompleteArrayField<__u8>,
}
```

#### Trait Implementations

##### `impl Debug for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fscrypt_add_key_arg`

```rust
struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: __u32,
    pub key_id: __u32,
    pub flags: __u32,
    pub __reserved: [__u32; 7],
    pub raw: __IncompleteArrayField<__u8>,
}
```

### `fscrypt_remove_key_arg`

```rust
struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: __u32,
    pub __reserved: [__u32; 5],
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-clone"></span>`fn clone(&self) -> fscrypt_remove_key_arg` — [`fscrypt_remove_key_arg`](#fscrypt-remove-key-arg)

##### `impl Copy for fscrypt_remove_key_arg`

### `fscrypt_get_key_status_arg`

```rust
struct fscrypt_get_key_status_arg {
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [__u32; 6],
    pub status: __u32,
    pub status_flags: __u32,
    pub user_count: __u32,
    pub __out_reserved: [__u32; 13],
}
```

#### Trait Implementations

##### `impl Clone for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-clone"></span>`fn clone(&self) -> fscrypt_get_key_status_arg` — [`fscrypt_get_key_status_arg`](#fscrypt-get-key-status-arg)

##### `impl Copy for fscrypt_get_key_status_arg`

### `mount_attr`

```rust
struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}
```

#### Trait Implementations

##### `impl Clone for mount_attr`

- <span id="mount-attr-clone"></span>`fn clone(&self) -> mount_attr` — [`mount_attr`](#mount-attr)

##### `impl Copy for mount_attr`

##### `impl Debug for mount_attr`

- <span id="mount-attr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statmount`

```rust
struct statmount {
    pub size: __u32,
    pub mnt_opts: __u32,
    pub mask: __u64,
    pub sb_dev_major: __u32,
    pub sb_dev_minor: __u32,
    pub sb_magic: __u64,
    pub sb_flags: __u32,
    pub fs_type: __u32,
    pub mnt_id: __u64,
    pub mnt_parent_id: __u64,
    pub mnt_id_old: __u32,
    pub mnt_parent_id_old: __u32,
    pub mnt_attr: __u64,
    pub mnt_propagation: __u64,
    pub mnt_peer_group: __u64,
    pub mnt_master: __u64,
    pub propagate_from: __u64,
    pub mnt_root: __u32,
    pub mnt_point: __u32,
    pub mnt_ns_id: __u64,
    pub fs_subtype: __u32,
    pub sb_source: __u32,
    pub opt_num: __u32,
    pub opt_array: __u32,
    pub opt_sec_num: __u32,
    pub opt_sec_array: __u32,
    pub supported_mask: __u64,
    pub mnt_uidmap_num: __u32,
    pub mnt_uidmap: __u32,
    pub mnt_gidmap_num: __u32,
    pub mnt_gidmap: __u32,
    pub __spare2: [__u64; 43],
    pub str_: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl Debug for statmount`

- <span id="statmount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mnt_id_req`

```rust
struct mnt_id_req {
    pub size: __u32,
    pub spare: __u32,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}
```

#### Trait Implementations

##### `impl Clone for mnt_id_req`

- <span id="mnt-id-req-clone"></span>`fn clone(&self) -> mnt_id_req` — [`mnt_id_req`](#mnt-id-req)

##### `impl Copy for mnt_id_req`

##### `impl Debug for mnt_id_req`

- <span id="mnt-id-req-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}
```

#### Trait Implementations

##### `impl Clone for file_clone_range`

- <span id="file-clone-range-clone"></span>`fn clone(&self) -> file_clone_range` — [`file_clone_range`](#file-clone-range)

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- <span id="file-clone-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fstrim_range`

```rust
struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}
```

#### Trait Implementations

##### `impl Clone for fstrim_range`

- <span id="fstrim-range-clone"></span>`fn clone(&self) -> fstrim_range` — [`fstrim_range`](#fstrim-range)

##### `impl Copy for fstrim_range`

##### `impl Debug for fstrim_range`

- <span id="fstrim-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fsuuid2`

```rust
struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for fsuuid2`

- <span id="fsuuid2-clone"></span>`fn clone(&self) -> fsuuid2` — [`fsuuid2`](#fsuuid2)

##### `impl Copy for fsuuid2`

##### `impl Debug for fsuuid2`

- <span id="fsuuid2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fs_sysfs_path`

```rust
struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}
```

#### Trait Implementations

##### `impl Clone for fs_sysfs_path`

- <span id="fs-sysfs-path-clone"></span>`fn clone(&self) -> fs_sysfs_path` — [`fs_sysfs_path`](#fs-sysfs-path)

##### `impl Copy for fs_sysfs_path`

##### `impl Debug for fs_sysfs_path`

- <span id="fs-sysfs-path-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_dedupe_range_info`

```rust
struct file_dedupe_range_info {
    pub dest_fd: __s64,
    pub dest_offset: __u64,
    pub bytes_deduped: __u64,
    pub status: __s32,
    pub reserved: __u32,
}
```

#### Trait Implementations

##### `impl Clone for file_dedupe_range_info`

- <span id="file-dedupe-range-info-clone"></span>`fn clone(&self) -> file_dedupe_range_info` — [`file_dedupe_range_info`](#file-dedupe-range-info)

##### `impl Copy for file_dedupe_range_info`

##### `impl Debug for file_dedupe_range_info`

- <span id="file-dedupe-range-info-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_dedupe_range`

```rust
struct file_dedupe_range {
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_count: __u16,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub info: __IncompleteArrayField<file_dedupe_range_info>,
}
```

#### Trait Implementations

##### `impl Debug for file_dedupe_range`

- <span id="file-dedupe-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `files_stat_struct`

```rust
struct files_stat_struct {
    pub nr_files: crate::ctypes::c_ulong,
    pub nr_free_files: crate::ctypes::c_ulong,
    pub max_files: crate::ctypes::c_ulong,
}
```

#### Trait Implementations

##### `impl Clone for files_stat_struct`

- <span id="files-stat-struct-clone"></span>`fn clone(&self) -> files_stat_struct` — [`files_stat_struct`](#files-stat-struct)

##### `impl Copy for files_stat_struct`

##### `impl Debug for files_stat_struct`

- <span id="files-stat-struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `inodes_stat_t`

```rust
struct inodes_stat_t {
    pub nr_inodes: crate::ctypes::c_long,
    pub nr_unused: crate::ctypes::c_long,
    pub dummy: [crate::ctypes::c_long; 5],
}
```

#### Trait Implementations

##### `impl Clone for inodes_stat_t`

- <span id="inodes-stat-t-clone"></span>`fn clone(&self) -> inodes_stat_t` — [`inodes_stat_t`](#inodes-stat-t)

##### `impl Copy for inodes_stat_t`

##### `impl Debug for inodes_stat_t`

- <span id="inodes-stat-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fsxattr`

```rust
struct fsxattr {
    pub fsx_xflags: __u32,
    pub fsx_extsize: __u32,
    pub fsx_nextents: __u32,
    pub fsx_projid: __u32,
    pub fsx_cowextsize: __u32,
    pub fsx_pad: [crate::ctypes::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for fsxattr`

- <span id="fsxattr-clone"></span>`fn clone(&self) -> fsxattr` — [`fsxattr`](#fsxattr)

##### `impl Copy for fsxattr`

##### `impl Debug for fsxattr`

- <span id="fsxattr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `page_region`

```rust
struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}
```

#### Trait Implementations

##### `impl Clone for page_region`

- <span id="page-region-clone"></span>`fn clone(&self) -> page_region` — [`page_region`](#page-region)

##### `impl Copy for page_region`

##### `impl Debug for page_region`

- <span id="page-region-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pm_scan_arg`

```rust
struct pm_scan_arg {
    pub size: __u64,
    pub flags: __u64,
    pub start: __u64,
    pub end: __u64,
    pub walk_end: __u64,
    pub vec: __u64,
    pub vec_len: __u64,
    pub max_pages: __u64,
    pub category_inverted: __u64,
    pub category_mask: __u64,
    pub category_anyof_mask: __u64,
    pub return_mask: __u64,
}
```

#### Trait Implementations

##### `impl Clone for pm_scan_arg`

- <span id="pm-scan-arg-clone"></span>`fn clone(&self) -> pm_scan_arg` — [`pm_scan_arg`](#pm-scan-arg)

##### `impl Copy for pm_scan_arg`

##### `impl Debug for pm_scan_arg`

- <span id="pm-scan-arg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `procmap_query`

```rust
struct procmap_query {
    pub size: __u64,
    pub query_flags: __u64,
    pub query_addr: __u64,
    pub vma_start: __u64,
    pub vma_end: __u64,
    pub vma_flags: __u64,
    pub vma_page_size: __u64,
    pub vma_offset: __u64,
    pub inode: __u64,
    pub dev_major: __u32,
    pub dev_minor: __u32,
    pub vma_name_size: __u32,
    pub build_id_size: __u32,
    pub vma_name_addr: __u64,
    pub build_id_addr: __u64,
}
```

#### Trait Implementations

##### `impl Clone for procmap_query`

- <span id="procmap-query-clone"></span>`fn clone(&self) -> procmap_query` — [`procmap_query`](#procmap-query)

##### `impl Copy for procmap_query`

##### `impl Debug for procmap_query`

- <span id="procmap-query-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `futex_waitv`

```rust
struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}
```

#### Trait Implementations

##### `impl Clone for futex_waitv`

- <span id="futex-waitv-clone"></span>`fn clone(&self) -> futex_waitv` — [`futex_waitv`](#futex-waitv)

##### `impl Copy for futex_waitv`

##### `impl Debug for futex_waitv`

- <span id="futex-waitv-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `robust_list`

```rust
struct robust_list {
    pub next: *mut robust_list,
}
```

#### Trait Implementations

##### `impl Clone for robust_list`

- <span id="robust-list-clone"></span>`fn clone(&self) -> robust_list` — [`robust_list`](#robust-list)

##### `impl Copy for robust_list`

##### `impl Debug for robust_list`

- <span id="robust-list-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `robust_list_head`

```rust
struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: crate::ctypes::c_long,
    pub list_op_pending: *mut robust_list,
}
```

#### Trait Implementations

##### `impl Clone for robust_list_head`

- <span id="robust-list-head-clone"></span>`fn clone(&self) -> robust_list_head` — [`robust_list_head`](#robust-list-head)

##### `impl Copy for robust_list_head`

##### `impl Debug for robust_list_head`

- <span id="robust-list-head-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `inotify_event`

```rust
struct inotify_event {
    pub wd: __s32,
    pub mask: __u32,
    pub cookie: __u32,
    pub len: __u32,
    pub name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl Debug for inotify_event`

- <span id="inotify-event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `cachestat_range`

```rust
struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl Clone for cachestat_range`

- <span id="cachestat-range-clone"></span>`fn clone(&self) -> cachestat_range` — [`cachestat_range`](#cachestat-range)

##### `impl Copy for cachestat_range`

##### `impl Debug for cachestat_range`

- <span id="cachestat-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `cachestat`

```rust
struct cachestat {
    pub nr_cache: __u64,
    pub nr_dirty: __u64,
    pub nr_writeback: __u64,
    pub nr_evicted: __u64,
    pub nr_recently_evicted: __u64,
}
```

#### Trait Implementations

##### `impl Clone for cachestat`

- <span id="cachestat-clone"></span>`fn clone(&self) -> cachestat` — [`cachestat`](#cachestat)

##### `impl Copy for cachestat`

##### `impl Debug for cachestat`

- <span id="cachestat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pollfd`

```rust
struct pollfd {
    pub fd: crate::ctypes::c_int,
    pub events: crate::ctypes::c_short,
    pub revents: crate::ctypes::c_short,
}
```

#### Trait Implementations

##### `impl Clone for pollfd`

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](#pollfd)

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- <span id="pollfd-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rand_pool_info`

```rust
struct rand_pool_info {
    pub entropy_count: crate::ctypes::c_int,
    pub buf_size: crate::ctypes::c_int,
    pub buf: __IncompleteArrayField<__u32>,
}
```

#### Trait Implementations

##### `impl Debug for rand_pool_info`

- <span id="rand-pool-info-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `vgetrandom_opaque_params`

```rust
struct vgetrandom_opaque_params {
    pub size_of_opaque_state: __u32,
    pub mmap_prot: __u32,
    pub mmap_flags: __u32,
    pub reserved: [__u32; 13],
}
```

#### Trait Implementations

##### `impl Clone for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-clone"></span>`fn clone(&self) -> vgetrandom_opaque_params` — [`vgetrandom_opaque_params`](#vgetrandom-opaque-params)

##### `impl Copy for vgetrandom_opaque_params`

##### `impl Debug for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_timespec`

```rust
struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: crate::ctypes::c_longlong,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_timespec`

- <span id="kernel-timespec-clone"></span>`fn clone(&self) -> __kernel_timespec` — [`__kernel_timespec`](#kernel-timespec)

##### `impl Copy for __kernel_timespec`

##### `impl Debug for __kernel_timespec`

- <span id="kernel-timespec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for general::__kernel_timespec`

##### `impl PartialEq for general::__kernel_timespec`

- <span id="general-kernel-timespec-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `__kernel_itimerspec`

```rust
struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec,
    pub it_value: __kernel_timespec,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_itimerspec`

- <span id="kernel-itimerspec-clone"></span>`fn clone(&self) -> __kernel_itimerspec` — [`__kernel_itimerspec`](#kernel-itimerspec)

##### `impl Copy for __kernel_itimerspec`

##### `impl Debug for __kernel_itimerspec`

- <span id="kernel-itimerspec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_old_timeval`

```rust
struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_old_timeval`

- <span id="kernel-old-timeval-clone"></span>`fn clone(&self) -> __kernel_old_timeval` — [`__kernel_old_timeval`](#kernel-old-timeval)

##### `impl Copy for __kernel_old_timeval`

##### `impl Debug for __kernel_old_timeval`

- <span id="kernel-old-timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_old_timespec`

```rust
struct __kernel_old_timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_old_timespec`

- <span id="kernel-old-timespec-clone"></span>`fn clone(&self) -> __kernel_old_timespec` — [`__kernel_old_timespec`](#kernel-old-timespec)

##### `impl Copy for __kernel_old_timespec`

##### `impl Debug for __kernel_old_timespec`

- <span id="kernel-old-timespec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_old_itimerval`

```rust
struct __kernel_old_itimerval {
    pub it_interval: __kernel_old_timeval,
    pub it_value: __kernel_old_timeval,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-clone"></span>`fn clone(&self) -> __kernel_old_itimerval` — [`__kernel_old_itimerval`](#kernel-old-itimerval)

##### `impl Copy for __kernel_old_itimerval`

##### `impl Debug for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__kernel_sock_timeval`

```rust
struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
```

#### Trait Implementations

##### `impl Clone for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-clone"></span>`fn clone(&self) -> __kernel_sock_timeval` — [`__kernel_sock_timeval`](#kernel-sock-timeval)

##### `impl Copy for __kernel_sock_timeval`

##### `impl Debug for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rusage`

```rust
struct rusage {
    pub ru_utime: __kernel_old_timeval,
    pub ru_stime: __kernel_old_timeval,
    pub ru_maxrss: __kernel_long_t,
    pub ru_ixrss: __kernel_long_t,
    pub ru_idrss: __kernel_long_t,
    pub ru_isrss: __kernel_long_t,
    pub ru_minflt: __kernel_long_t,
    pub ru_majflt: __kernel_long_t,
    pub ru_nswap: __kernel_long_t,
    pub ru_inblock: __kernel_long_t,
    pub ru_oublock: __kernel_long_t,
    pub ru_msgsnd: __kernel_long_t,
    pub ru_msgrcv: __kernel_long_t,
    pub ru_nsignals: __kernel_long_t,
    pub ru_nvcsw: __kernel_long_t,
    pub ru_nivcsw: __kernel_long_t,
}
```

#### Trait Implementations

##### `impl Clone for rusage`

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](#rusage)

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- <span id="rusage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](#rlimit)

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- <span id="rlimit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rlimit64`

```rust
struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}
```

#### Trait Implementations

##### `impl Clone for rlimit64`

- <span id="rlimit64-clone"></span>`fn clone(&self) -> rlimit64` — [`rlimit64`](#rlimit64)

##### `impl Copy for rlimit64`

##### `impl Debug for rlimit64`

- <span id="rlimit64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `clone_args`

```rust
struct clone_args {
    pub flags: __u64,
    pub pidfd: __u64,
    pub child_tid: __u64,
    pub parent_tid: __u64,
    pub exit_signal: __u64,
    pub stack: __u64,
    pub stack_size: __u64,
    pub tls: __u64,
    pub set_tid: __u64,
    pub set_tid_size: __u64,
    pub cgroup: __u64,
}
```

#### Trait Implementations

##### `impl Clone for clone_args`

- <span id="clone-args-clone"></span>`fn clone(&self) -> clone_args` — [`clone_args`](#clone-args)

##### `impl Copy for clone_args`

##### `impl Debug for clone_args`

- <span id="clone-args-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigaction`

```rust
struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
```

#### Trait Implementations

##### `impl Clone for sigaction`

- <span id="sigaction-clone"></span>`fn clone(&self) -> sigaction` — [`sigaction`](#sigaction)

##### `impl Copy for sigaction`

##### `impl Debug for sigaction`

- <span id="sigaction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigaltstack`

```rust
struct sigaltstack {
    pub ss_sp: *mut crate::ctypes::c_void,
    pub ss_flags: crate::ctypes::c_int,
    pub ss_size: __kernel_size_t,
}
```

#### Trait Implementations

##### `impl Clone for sigaltstack`

- <span id="sigaltstack-clone"></span>`fn clone(&self) -> sigaltstack` — [`sigaltstack`](#sigaltstack)

##### `impl Copy for sigaltstack`

##### `impl Debug for sigaltstack`

- <span id="sigaltstack-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_1 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_1` — [`__sifields__bindgen_ty_1`](#sifields-bindgen-ty-1)

##### `impl Copy for __sifields__bindgen_ty_1`

##### `impl Debug for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_2 {
    pub _tid: __kernel_timer_t,
    pub _overrun: crate::ctypes::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_2` — [`__sifields__bindgen_ty_2`](#sifields-bindgen-ty-2)

##### `impl Copy for __sifields__bindgen_ty_2`

### `__sifields__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_3 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_3` — [`__sifields__bindgen_ty_3`](#sifields-bindgen-ty-3)

##### `impl Copy for __sifields__bindgen_ty_3`

### `__sifields__bindgen_ty_4`

```rust
struct __sifields__bindgen_ty_4 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: crate::ctypes::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_4` — [`__sifields__bindgen_ty_4`](#sifields-bindgen-ty-4)

##### `impl Copy for __sifields__bindgen_ty_4`

##### `impl Debug for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_5`

```rust
struct __sifields__bindgen_ty_5 {
    pub _addr: *mut crate::ctypes::c_void,
    pub __bindgen_anon_1: __sifields__bindgen_ty_5__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5` — [`__sifields__bindgen_ty_5`](#sifields-bindgen-ty-5)

##### `impl Copy for __sifields__bindgen_ty_5`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _dummy_bnd: [crate::ctypes::c_char; 8],
    pub _lower: *mut crate::ctypes::c_void,
    pub _upper: *mut crate::ctypes::c_void,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1)

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub _dummy_pkey: [crate::ctypes::c_char; 8],
    pub _pkey: __u32,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2)

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub _data: crate::ctypes::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3)

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_6`

```rust
struct __sifields__bindgen_ty_6 {
    pub _band: crate::ctypes::c_long,
    pub _fd: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_6` — [`__sifields__bindgen_ty_6`](#sifields-bindgen-ty-6)

##### `impl Copy for __sifields__bindgen_ty_6`

##### `impl Debug for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__sifields__bindgen_ty_7`

```rust
struct __sifields__bindgen_ty_7 {
    pub _call_addr: *mut crate::ctypes::c_void,
    pub _syscall: crate::ctypes::c_int,
    pub _arch: crate::ctypes::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_7` — [`__sifields__bindgen_ty_7`](#sifields-bindgen-ty-7)

##### `impl Copy for __sifields__bindgen_ty_7`

##### `impl Debug for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `siginfo`

```rust
struct siginfo {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for siginfo`

- <span id="siginfo-clone"></span>`fn clone(&self) -> siginfo` — [`siginfo`](#siginfo)

##### `impl Copy for siginfo`

### `siginfo__bindgen_ty_1__bindgen_ty_1`

```rust
struct siginfo__bindgen_ty_1__bindgen_ty_1 {
    pub si_signo: crate::ctypes::c_int,
    pub si_errno: crate::ctypes::c_int,
    pub si_code: crate::ctypes::c_int,
    pub _sifields: __sifields,
}
```

#### Trait Implementations

##### `impl Clone for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> siginfo__bindgen_ty_1__bindgen_ty_1` — [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo-bindgen-ty-1-bindgen-ty-1)

##### `impl Copy for siginfo__bindgen_ty_1__bindgen_ty_1`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: crate::ctypes::c_int,
    pub sigev_notify: crate::ctypes::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for sigevent`

- <span id="sigevent-clone"></span>`fn clone(&self) -> sigevent` — [`sigevent`](#sigevent)

##### `impl Copy for sigevent`

### `sigevent__bindgen_ty_1__bindgen_ty_1`

```rust
struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::core::option::Option<fn(sigval_t)>,
    pub _attribute: *mut crate::ctypes::c_void,
}
```

#### Trait Implementations

##### `impl Clone for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> sigevent__bindgen_ty_1__bindgen_ty_1` — [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent-bindgen-ty-1-bindgen-ty-1)

##### `impl Copy for sigevent__bindgen_ty_1__bindgen_ty_1`

##### `impl Debug for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
```

#### Trait Implementations

##### `impl Clone for statx_timestamp`

- <span id="statx-timestamp-clone"></span>`fn clone(&self) -> statx_timestamp` — [`statx_timestamp`](#statx-timestamp)

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- <span id="statx-timestamp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx`

```rust
struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub stx_subvol: __u64,
    pub stx_atomic_write_unit_min: __u32,
    pub stx_atomic_write_unit_max: __u32,
    pub stx_atomic_write_segments_max: __u32,
    pub stx_dio_read_offset_align: __u32,
    pub stx_atomic_write_unit_max_opt: __u32,
    pub __spare2: [__u32; 1],
    pub __spare3: [__u64; 8],
}
```

#### Trait Implementations

##### `impl Clone for statx`

- <span id="statx-clone"></span>`fn clone(&self) -> statx` — [`statx`](#statx)

##### `impl Copy for statx`

##### `impl Debug for statx`

- <span id="statx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `termios`

```rust
struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
}
```

#### Trait Implementations

##### `impl Clone for termios`

- <span id="termios-clone"></span>`fn clone(&self) -> termios` — [`termios`](#termios)

##### `impl Copy for termios`

##### `impl Debug for termios`

- <span id="termios-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `termios2`

```rust
struct termios2 {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

#### Trait Implementations

##### `impl Clone for termios2`

- <span id="termios2-clone"></span>`fn clone(&self) -> termios2` — [`termios2`](#termios2)

##### `impl Copy for termios2`

##### `impl Debug for termios2`

- <span id="termios2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ktermios`

```rust
struct ktermios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

#### Trait Implementations

##### `impl Clone for ktermios`

- <span id="ktermios-clone"></span>`fn clone(&self) -> ktermios` — [`ktermios`](#ktermios)

##### `impl Copy for ktermios`

##### `impl Debug for ktermios`

- <span id="ktermios-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `winsize`

```rust
struct winsize {
    pub ws_row: crate::ctypes::c_ushort,
    pub ws_col: crate::ctypes::c_ushort,
    pub ws_xpixel: crate::ctypes::c_ushort,
    pub ws_ypixel: crate::ctypes::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](#winsize)

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- <span id="winsize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `termio`

```rust
struct termio {
    pub c_iflag: crate::ctypes::c_ushort,
    pub c_oflag: crate::ctypes::c_ushort,
    pub c_cflag: crate::ctypes::c_ushort,
    pub c_lflag: crate::ctypes::c_ushort,
    pub c_line: crate::ctypes::c_uchar,
    pub c_cc: [crate::ctypes::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for termio`

- <span id="termio-clone"></span>`fn clone(&self) -> termio` — [`termio`](#termio)

##### `impl Copy for termio`

##### `impl Debug for termio`

- <span id="termio-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timespec`

```rust
struct timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

#### Trait Implementations

##### `impl Clone for timespec`

- <span id="timespec-clone"></span>`fn clone(&self) -> timespec` — [`timespec`](#timespec)

##### `impl Copy for timespec`

##### `impl Debug for timespec`

- <span id="timespec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: __kernel_old_time_t,
    pub tv_usec: __kernel_suseconds_t,
}
```

#### Trait Implementations

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](#timeval)

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- <span id="timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerspec`

```rust
struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
```

#### Trait Implementations

##### `impl Clone for itimerspec`

- <span id="itimerspec-clone"></span>`fn clone(&self) -> itimerspec` — [`itimerspec`](#itimerspec)

##### `impl Copy for itimerspec`

##### `impl Debug for itimerspec`

- <span id="itimerspec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
```

#### Trait Implementations

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](#itimerval)

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- <span id="itimerval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timezone`

```rust
struct timezone {
    pub tz_minuteswest: crate::ctypes::c_int,
    pub tz_dsttime: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl Clone for timezone`

- <span id="timezone-clone"></span>`fn clone(&self) -> timezone` — [`timezone`](#timezone)

##### `impl Copy for timezone`

##### `impl Debug for timezone`

- <span id="timezone-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: __kernel_size_t,
}
```

#### Trait Implementations

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](#iovec)

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- <span id="iovec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dmabuf_cmsg`

```rust
struct dmabuf_cmsg {
    pub frag_offset: __u64,
    pub frag_size: __u32,
    pub frag_token: __u32,
    pub dmabuf_id: __u32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_cmsg`

- <span id="dmabuf-cmsg-clone"></span>`fn clone(&self) -> dmabuf_cmsg` — [`dmabuf_cmsg`](#dmabuf-cmsg)

##### `impl Copy for dmabuf_cmsg`

##### `impl Debug for dmabuf_cmsg`

- <span id="dmabuf-cmsg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dmabuf_token`

```rust
struct dmabuf_token {
    pub token_start: __u32,
    pub token_count: __u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_token`

- <span id="dmabuf-token-clone"></span>`fn clone(&self) -> dmabuf_token` — [`dmabuf_token`](#dmabuf-token)

##### `impl Copy for dmabuf_token`

##### `impl Debug for dmabuf_token`

- <span id="dmabuf-token-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xattr_args`

```rust
struct xattr_args {
    pub value: __u64,
    pub size: __u32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for xattr_args`

- <span id="xattr-args-clone"></span>`fn clone(&self) -> xattr_args` — [`xattr_args`](#xattr-args)

##### `impl Copy for xattr_args`

##### `impl Debug for xattr_args`

- <span id="xattr-args-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffd_msg`

```rust
struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub arg: uffd_msg__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg`

- <span id="uffd-msg-clone"></span>`fn clone(&self) -> uffd_msg` — [`uffd_msg`](#uffd-msg)

##### `impl Copy for uffd_msg`

### `uffd_msg__bindgen_ty_1__bindgen_ty_1`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_1 {
    pub flags: __u64,
    pub address: __u64,
    pub feat: uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_1` — [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd-msg-bindgen-ty-1-bindgen-ty-1)

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_1`

### `uffd_msg__bindgen_ty_1__bindgen_ty_2`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_2 {
    pub ufd: __u32,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_2` — [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd-msg-bindgen-ty-1-bindgen-ty-2)

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_2`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_3`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_3 {
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_3` — [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd-msg-bindgen-ty-1-bindgen-ty-3)

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_3`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_4`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_4 {
    pub start: __u64,
    pub end: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_4` — [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd-msg-bindgen-ty-1-bindgen-ty-4)

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_4`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_5`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_5 {
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_5` — [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd-msg-bindgen-ty-1-bindgen-ty-5)

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_5`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_api`

```rust
struct uffdio_api {
    pub api: __u64,
    pub features: __u64,
    pub ioctls: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_api`

- <span id="uffdio-api-clone"></span>`fn clone(&self) -> uffdio_api` — [`uffdio_api`](#uffdio-api)

##### `impl Copy for uffdio_api`

##### `impl Debug for uffdio_api`

- <span id="uffdio-api-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_range`

```rust
struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_range`

- <span id="uffdio-range-clone"></span>`fn clone(&self) -> uffdio_range` — [`uffdio_range`](#uffdio-range)

##### `impl Copy for uffdio_range`

##### `impl Debug for uffdio_range`

- <span id="uffdio-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_register`

```rust
struct uffdio_register {
    pub range: uffdio_range,
    pub mode: __u64,
    pub ioctls: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_register`

- <span id="uffdio-register-clone"></span>`fn clone(&self) -> uffdio_register` — [`uffdio_register`](#uffdio-register)

##### `impl Copy for uffdio_register`

##### `impl Debug for uffdio_register`

- <span id="uffdio-register-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_copy`

```rust
struct uffdio_copy {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub copy: __s64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_copy`

- <span id="uffdio-copy-clone"></span>`fn clone(&self) -> uffdio_copy` — [`uffdio_copy`](#uffdio-copy)

##### `impl Copy for uffdio_copy`

##### `impl Debug for uffdio_copy`

- <span id="uffdio-copy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_zeropage`

```rust
struct uffdio_zeropage {
    pub range: uffdio_range,
    pub mode: __u64,
    pub zeropage: __s64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_zeropage`

- <span id="uffdio-zeropage-clone"></span>`fn clone(&self) -> uffdio_zeropage` — [`uffdio_zeropage`](#uffdio-zeropage)

##### `impl Copy for uffdio_zeropage`

##### `impl Debug for uffdio_zeropage`

- <span id="uffdio-zeropage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_writeprotect`

```rust
struct uffdio_writeprotect {
    pub range: uffdio_range,
    pub mode: __u64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_writeprotect`

- <span id="uffdio-writeprotect-clone"></span>`fn clone(&self) -> uffdio_writeprotect` — [`uffdio_writeprotect`](#uffdio-writeprotect)

##### `impl Copy for uffdio_writeprotect`

##### `impl Debug for uffdio_writeprotect`

- <span id="uffdio-writeprotect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_continue`

```rust
struct uffdio_continue {
    pub range: uffdio_range,
    pub mode: __u64,
    pub mapped: __s64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_continue`

- <span id="uffdio-continue-clone"></span>`fn clone(&self) -> uffdio_continue` — [`uffdio_continue`](#uffdio-continue)

##### `impl Copy for uffdio_continue`

##### `impl Debug for uffdio_continue`

- <span id="uffdio-continue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_poison`

```rust
struct uffdio_poison {
    pub range: uffdio_range,
    pub mode: __u64,
    pub updated: __s64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_poison`

- <span id="uffdio-poison-clone"></span>`fn clone(&self) -> uffdio_poison` — [`uffdio_poison`](#uffdio-poison)

##### `impl Copy for uffdio_poison`

##### `impl Debug for uffdio_poison`

- <span id="uffdio-poison-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uffdio_move`

```rust
struct uffdio_move {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub move_: __s64,
}
```

#### Trait Implementations

##### `impl Clone for uffdio_move`

- <span id="uffdio-move-clone"></span>`fn clone(&self) -> uffdio_move` — [`uffdio_move`](#uffdio-move)

##### `impl Copy for uffdio_move`

##### `impl Debug for uffdio_move`

- <span id="uffdio-move-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `linux_dirent64`

```rust
struct linux_dirent64 {
    pub d_ino: crate::ctypes::c_ulong,
    pub d_off: crate::ctypes::c_long,
    pub d_reclen: __u16,
    pub d_type: __u8,
    pub d_name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl Debug for linux_dirent64`

- <span id="linux-dirent64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `stat`

```rust
struct stat {
    pub st_dev: __kernel_ulong_t,
    pub st_ino: __kernel_ulong_t,
    pub st_nlink: __kernel_ulong_t,
    pub st_mode: crate::ctypes::c_uint,
    pub st_uid: crate::ctypes::c_uint,
    pub st_gid: crate::ctypes::c_uint,
    pub __pad0: crate::ctypes::c_uint,
    pub st_rdev: __kernel_ulong_t,
    pub st_size: __kernel_long_t,
    pub st_blksize: __kernel_long_t,
    pub st_blocks: __kernel_long_t,
    pub st_atime: __kernel_ulong_t,
    pub st_atime_nsec: __kernel_ulong_t,
    pub st_mtime: __kernel_ulong_t,
    pub st_mtime_nsec: __kernel_ulong_t,
    pub st_ctime: __kernel_ulong_t,
    pub st_ctime_nsec: __kernel_ulong_t,
    pub __unused: [__kernel_long_t; 3],
}
```

#### Trait Implementations

##### `impl Clone for stat`

- <span id="stat-clone"></span>`fn clone(&self) -> stat` — [`stat`](#stat)

##### `impl Copy for stat`

##### `impl Debug for stat`

- <span id="stat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__old_kernel_stat`

```rust
struct __old_kernel_stat {
    pub st_dev: crate::ctypes::c_ushort,
    pub st_ino: crate::ctypes::c_ushort,
    pub st_mode: crate::ctypes::c_ushort,
    pub st_nlink: crate::ctypes::c_ushort,
    pub st_uid: crate::ctypes::c_ushort,
    pub st_gid: crate::ctypes::c_ushort,
    pub st_rdev: crate::ctypes::c_ushort,
    pub st_size: crate::ctypes::c_uint,
    pub st_atime: crate::ctypes::c_uint,
    pub st_mtime: crate::ctypes::c_uint,
    pub st_ctime: crate::ctypes::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for __old_kernel_stat`

- <span id="old-kernel-stat-clone"></span>`fn clone(&self) -> __old_kernel_stat` — [`__old_kernel_stat`](#old-kernel-stat)

##### `impl Copy for __old_kernel_stat`

##### `impl Debug for __old_kernel_stat`

- <span id="old-kernel-stat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statfs`

```rust
struct statfs {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __kernel_long_t,
    pub f_bfree: __kernel_long_t,
    pub f_bavail: __kernel_long_t,
    pub f_files: __kernel_long_t,
    pub f_ffree: __kernel_long_t,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

#### Trait Implementations

##### `impl Clone for statfs`

- <span id="statfs-clone"></span>`fn clone(&self) -> statfs` — [`statfs`](#statfs)

##### `impl Copy for statfs`

##### `impl Debug for statfs`

- <span id="statfs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statfs64`

```rust
struct statfs64 {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

#### Trait Implementations

##### `impl Clone for statfs64`

- <span id="statfs64-clone"></span>`fn clone(&self) -> statfs64` — [`statfs64`](#statfs64)

##### `impl Copy for statfs64`

##### `impl Debug for statfs64`

- <span id="statfs64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `compat_statfs64`

```rust
struct compat_statfs64 {
    pub f_type: __u32,
    pub f_bsize: __u32,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __u32,
    pub f_frsize: __u32,
    pub f_flags: __u32,
    pub f_spare: [__u32; 4],
}
```

#### Trait Implementations

##### `impl Clone for compat_statfs64`

- <span id="compat-statfs64-clone"></span>`fn clone(&self) -> compat_statfs64` — [`compat_statfs64`](#compat-statfs64)

##### `impl Copy for compat_statfs64`

##### `impl Debug for compat_statfs64`

- <span id="compat-statfs64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `user_desc`

```rust
struct user_desc {
    pub entry_number: crate::ctypes::c_uint,
    pub base_addr: crate::ctypes::c_uint,
    pub limit: crate::ctypes::c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1]>,
    pub __bindgen_padding_0: [u8; 3],
}
```

#### Implementations

- <span id="user-desc-seg-32bit"></span>`fn seg_32bit(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-seg-32bit"></span>`fn set_seg_32bit(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-seg-32bit-raw"></span>`unsafe fn seg_32bit_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-seg-32bit-raw"></span>`unsafe fn set_seg_32bit_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-contents"></span>`fn contents(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-contents"></span>`fn set_contents(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-contents-raw"></span>`unsafe fn contents_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-contents-raw"></span>`unsafe fn set_contents_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-read-exec-only"></span>`fn read_exec_only(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-read-exec-only"></span>`fn set_read_exec_only(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-read-exec-only-raw"></span>`unsafe fn read_exec_only_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-read-exec-only-raw"></span>`unsafe fn set_read_exec_only_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-limit-in-pages"></span>`fn limit_in_pages(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-limit-in-pages"></span>`fn set_limit_in_pages(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-limit-in-pages-raw"></span>`unsafe fn limit_in_pages_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-limit-in-pages-raw"></span>`unsafe fn set_limit_in_pages_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-seg-not-present"></span>`fn seg_not_present(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-seg-not-present"></span>`fn set_seg_not_present(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-seg-not-present-raw"></span>`unsafe fn seg_not_present_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-seg-not-present-raw"></span>`unsafe fn set_seg_not_present_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-useable"></span>`fn useable(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-useable"></span>`fn set_useable(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-useable-raw"></span>`unsafe fn useable_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-useable-raw"></span>`unsafe fn set_useable_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-lm"></span>`fn lm(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-lm"></span>`fn set_lm(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-lm-raw"></span>`unsafe fn lm_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-set-lm-raw"></span>`unsafe fn set_lm_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md)

- <span id="user-desc-new-bitfield-1"></span>`fn new_bitfield_1(seg_32bit: crate::ctypes::c_uint, contents: crate::ctypes::c_uint, read_exec_only: crate::ctypes::c_uint, limit_in_pages: crate::ctypes::c_uint, seg_not_present: crate::ctypes::c_uint, useable: crate::ctypes::c_uint, lm: crate::ctypes::c_uint) -> __BindgenBitfieldUnit<[u8; 1]>` — [`c_uint`](../ctypes/index.md), [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

#### Trait Implementations

##### `impl Clone for user_desc`

- <span id="user-desc-clone"></span>`fn clone(&self) -> user_desc` — [`user_desc`](#user-desc)

##### `impl Copy for user_desc`

##### `impl Debug for user_desc`

- <span id="user-desc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `kernel_sigset_t`

```rust
struct kernel_sigset_t {
    pub sig: [crate::ctypes::c_ulong; 1],
}
```

#### Trait Implementations

##### `impl Clone for kernel_sigset_t`

- <span id="kernel-sigset-t-clone"></span>`fn clone(&self) -> kernel_sigset_t` — [`kernel_sigset_t`](#kernel-sigset-t)

##### `impl Copy for kernel_sigset_t`

##### `impl Debug for kernel_sigset_t`

- <span id="kernel-sigset-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `kernel_sigaction`

```rust
struct kernel_sigaction {
    pub sa_handler_kernel: __kernel_sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: kernel_sigset_t,
}
```

#### Trait Implementations

##### `impl Clone for kernel_sigaction`

- <span id="kernel-sigaction-clone"></span>`fn clone(&self) -> kernel_sigaction` — [`kernel_sigaction`](#kernel-sigaction)

##### `impl Copy for kernel_sigaction`

##### `impl Debug for kernel_sigaction`

- <span id="kernel-sigaction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `fsconfig_command`

```rust
enum fsconfig_command {
    FSCONFIG_SET_FLAG,
    FSCONFIG_SET_STRING,
    FSCONFIG_SET_BINARY,
    FSCONFIG_SET_PATH,
    FSCONFIG_SET_PATH_EMPTY,
    FSCONFIG_SET_FD,
    FSCONFIG_CMD_CREATE,
    FSCONFIG_CMD_RECONFIGURE,
    FSCONFIG_CMD_CREATE_EXCL,
}
```

#### Trait Implementations

##### `impl Clone for fsconfig_command`

- <span id="fsconfig-command-clone"></span>`fn clone(&self) -> fsconfig_command` — [`fsconfig_command`](#fsconfig-command)

##### `impl Copy for fsconfig_command`

##### `impl Debug for fsconfig_command`

- <span id="fsconfig-command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for fsconfig_command`

##### `impl Hash for fsconfig_command`

- <span id="fsconfig-command-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for fsconfig_command`

- <span id="fsconfig-command-eq"></span>`fn eq(&self, other: &fsconfig_command) -> bool` — [`fsconfig_command`](#fsconfig-command)

##### `impl StructuralPartialEq for fsconfig_command`

### `procmap_query_flags`

```rust
enum procmap_query_flags {
    PROCMAP_QUERY_VMA_READABLE,
    PROCMAP_QUERY_VMA_WRITABLE,
    PROCMAP_QUERY_VMA_EXECUTABLE,
    PROCMAP_QUERY_VMA_SHARED,
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA,
    PROCMAP_QUERY_FILE_BACKED_VMA,
}
```

#### Trait Implementations

##### `impl Clone for procmap_query_flags`

- <span id="procmap-query-flags-clone"></span>`fn clone(&self) -> procmap_query_flags` — [`procmap_query_flags`](#procmap-query-flags)

##### `impl Copy for procmap_query_flags`

##### `impl Debug for procmap_query_flags`

- <span id="procmap-query-flags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for procmap_query_flags`

##### `impl Hash for procmap_query_flags`

- <span id="procmap-query-flags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for procmap_query_flags`

- <span id="procmap-query-flags-eq"></span>`fn eq(&self, other: &procmap_query_flags) -> bool` — [`procmap_query_flags`](#procmap-query-flags)

##### `impl StructuralPartialEq for procmap_query_flags`

### `membarrier_cmd`

```rust
enum membarrier_cmd {
    MEMBARRIER_CMD_QUERY,
    MEMBARRIER_CMD_GLOBAL,
    MEMBARRIER_CMD_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_GET_REGISTRATIONS,
}
```

#### Implementations

- <span id="membarrier-cmd-membarrier-cmd-shared"></span>`const MEMBARRIER_CMD_SHARED: membarrier_cmd`

#### Trait Implementations

##### `impl Clone for membarrier_cmd`

- <span id="membarrier-cmd-clone"></span>`fn clone(&self) -> membarrier_cmd` — [`membarrier_cmd`](#membarrier-cmd)

##### `impl Copy for membarrier_cmd`

##### `impl Debug for membarrier_cmd`

- <span id="membarrier-cmd-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for membarrier_cmd`

##### `impl Hash for membarrier_cmd`

- <span id="membarrier-cmd-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for membarrier_cmd`

- <span id="membarrier-cmd-eq"></span>`fn eq(&self, other: &membarrier_cmd) -> bool` — [`membarrier_cmd`](#membarrier-cmd)

##### `impl StructuralPartialEq for membarrier_cmd`

### `membarrier_cmd_flag`

```rust
enum membarrier_cmd_flag {
    MEMBARRIER_CMD_FLAG_CPU,
}
```

#### Trait Implementations

##### `impl Clone for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-clone"></span>`fn clone(&self) -> membarrier_cmd_flag` — [`membarrier_cmd_flag`](#membarrier-cmd-flag)

##### `impl Copy for membarrier_cmd_flag`

##### `impl Debug for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for membarrier_cmd_flag`

##### `impl Hash for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-eq"></span>`fn eq(&self, other: &membarrier_cmd_flag) -> bool` — [`membarrier_cmd_flag`](#membarrier-cmd-flag)

##### `impl StructuralPartialEq for membarrier_cmd_flag`

## Type Aliases

### `__s8`

```rust
type __s8 = crate::ctypes::c_schar;
```

### `__u8`

```rust
type __u8 = crate::ctypes::c_uchar;
```

### `__s16`

```rust
type __s16 = crate::ctypes::c_short;
```

### `__u16`

```rust
type __u16 = crate::ctypes::c_ushort;
```

### `__s32`

```rust
type __s32 = crate::ctypes::c_int;
```

### `__u32`

```rust
type __u32 = crate::ctypes::c_uint;
```

### `__s64`

```rust
type __s64 = crate::ctypes::c_longlong;
```

### `__u64`

```rust
type __u64 = crate::ctypes::c_ulonglong;
```

### `__kernel_sighandler_t`

```rust
type __kernel_sighandler_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

### `__kernel_key_t`

```rust
type __kernel_key_t = crate::ctypes::c_int;
```

### `__kernel_mqd_t`

```rust
type __kernel_mqd_t = crate::ctypes::c_int;
```

### `__kernel_old_uid_t`

```rust
type __kernel_old_uid_t = crate::ctypes::c_ushort;
```

### `__kernel_old_gid_t`

```rust
type __kernel_old_gid_t = crate::ctypes::c_ushort;
```

### `__kernel_old_dev_t`

```rust
type __kernel_old_dev_t = crate::ctypes::c_ulong;
```

### `__kernel_long_t`

```rust
type __kernel_long_t = crate::ctypes::c_long;
```

### `__kernel_ulong_t`

```rust
type __kernel_ulong_t = crate::ctypes::c_ulong;
```

### `__kernel_ino_t`

```rust
type __kernel_ino_t = __kernel_ulong_t;
```

### `__kernel_mode_t`

```rust
type __kernel_mode_t = crate::ctypes::c_uint;
```

### `__kernel_pid_t`

```rust
type __kernel_pid_t = crate::ctypes::c_int;
```

### `__kernel_ipc_pid_t`

```rust
type __kernel_ipc_pid_t = crate::ctypes::c_int;
```

### `__kernel_uid_t`

```rust
type __kernel_uid_t = crate::ctypes::c_uint;
```

### `__kernel_gid_t`

```rust
type __kernel_gid_t = crate::ctypes::c_uint;
```

### `__kernel_suseconds_t`

```rust
type __kernel_suseconds_t = __kernel_long_t;
```

### `__kernel_daddr_t`

```rust
type __kernel_daddr_t = crate::ctypes::c_int;
```

### `__kernel_uid32_t`

```rust
type __kernel_uid32_t = crate::ctypes::c_uint;
```

### `__kernel_gid32_t`

```rust
type __kernel_gid32_t = crate::ctypes::c_uint;
```

### `__kernel_size_t`

```rust
type __kernel_size_t = __kernel_ulong_t;
```

### `__kernel_ssize_t`

```rust
type __kernel_ssize_t = __kernel_long_t;
```

### `__kernel_ptrdiff_t`

```rust
type __kernel_ptrdiff_t = __kernel_long_t;
```

### `__kernel_off_t`

```rust
type __kernel_off_t = __kernel_long_t;
```

### `__kernel_loff_t`

```rust
type __kernel_loff_t = crate::ctypes::c_longlong;
```

### `__kernel_old_time_t`

```rust
type __kernel_old_time_t = __kernel_long_t;
```

### `__kernel_time_t`

```rust
type __kernel_time_t = __kernel_long_t;
```

### `__kernel_time64_t`

```rust
type __kernel_time64_t = crate::ctypes::c_longlong;
```

### `__kernel_clock_t`

```rust
type __kernel_clock_t = __kernel_long_t;
```

### `__kernel_timer_t`

```rust
type __kernel_timer_t = crate::ctypes::c_int;
```

### `__kernel_clockid_t`

```rust
type __kernel_clockid_t = crate::ctypes::c_int;
```

### `__kernel_caddr_t`

```rust
type __kernel_caddr_t = *mut crate::ctypes::c_char;
```

### `__kernel_uid16_t`

```rust
type __kernel_uid16_t = crate::ctypes::c_ushort;
```

### `__kernel_gid16_t`

```rust
type __kernel_gid16_t = crate::ctypes::c_ushort;
```

### `__s128`

```rust
type __s128 = i128;
```

### `__u128`

```rust
type __u128 = u128;
```

### `__le16`

```rust
type __le16 = __u16;
```

### `__be16`

```rust
type __be16 = __u16;
```

### `__le32`

```rust
type __le32 = __u32;
```

### `__be32`

```rust
type __be32 = __u32;
```

### `__le64`

```rust
type __le64 = __u64;
```

### `__be64`

```rust
type __be64 = __u64;
```

### `__sum16`

```rust
type __sum16 = __u16;
```

### `__wsum`

```rust
type __wsum = __u32;
```

### `__poll_t`

```rust
type __poll_t = crate::ctypes::c_uint;
```

### `cap_user_header_t`

```rust
type cap_user_header_t = *mut __user_cap_header_struct;
```

### `cap_user_data_t`

```rust
type cap_user_data_t = *mut __user_cap_data_struct;
```

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = crate::ctypes::c_int;
```

### `sigset_t`

```rust
type sigset_t = crate::ctypes::c_ulong;
```

### `__signalfn_t`

```rust
type __signalfn_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

### `__sighandler_t`

```rust
type __sighandler_t = __signalfn_t;
```

### `__restorefn_t`

```rust
type __restorefn_t = ::core::option::Option<fn()>;
```

### `__sigrestore_t`

```rust
type __sigrestore_t = __restorefn_t;
```

### `stack_t`

```rust
type stack_t = sigaltstack;
```

### `sigval_t`

```rust
type sigval_t = sigval;
```

### `siginfo_t`

```rust
type siginfo_t = siginfo;
```

### `sigevent_t`

```rust
type sigevent_t = sigevent;
```

### `cc_t`

```rust
type cc_t = crate::ctypes::c_uchar;
```

### `speed_t`

```rust
type speed_t = crate::ctypes::c_uint;
```

### `tcflag_t`

```rust
type tcflag_t = crate::ctypes::c_uint;
```

### `__fsword_t`

```rust
type __fsword_t = __kernel_long_t;
```

## Constants

### `LINUX_VERSION_CODE`

```rust
const LINUX_VERSION_CODE: u32 = 397_312u32;
```

### `LINUX_VERSION_MAJOR`

```rust
const LINUX_VERSION_MAJOR: u32 = 6u32;
```

### `LINUX_VERSION_PATCHLEVEL`

```rust
const LINUX_VERSION_PATCHLEVEL: u32 = 16u32;
```

### `LINUX_VERSION_SUBLEVEL`

```rust
const LINUX_VERSION_SUBLEVEL: u32 = 0u32;
```

### `__BITS_PER_LONG_LONG`

```rust
const __BITS_PER_LONG_LONG: u32 = 64u32;
```

### `__FD_SETSIZE`

```rust
const __FD_SETSIZE: u32 = 1_024u32;
```

### `_LINUX_CAPABILITY_VERSION_1`

```rust
const _LINUX_CAPABILITY_VERSION_1: u32 = 429_392_688u32;
```

### `_LINUX_CAPABILITY_U32S_1`

```rust
const _LINUX_CAPABILITY_U32S_1: u32 = 1u32;
```

### `_LINUX_CAPABILITY_VERSION_2`

```rust
const _LINUX_CAPABILITY_VERSION_2: u32 = 537_333_798u32;
```

### `_LINUX_CAPABILITY_U32S_2`

```rust
const _LINUX_CAPABILITY_U32S_2: u32 = 2u32;
```

### `_LINUX_CAPABILITY_VERSION_3`

```rust
const _LINUX_CAPABILITY_VERSION_3: u32 = 537_396_514u32;
```

### `_LINUX_CAPABILITY_U32S_3`

```rust
const _LINUX_CAPABILITY_U32S_3: u32 = 2u32;
```

### `VFS_CAP_REVISION_MASK`

```rust
const VFS_CAP_REVISION_MASK: u32 = 4_278_190_080u32;
```

### `VFS_CAP_REVISION_SHIFT`

```rust
const VFS_CAP_REVISION_SHIFT: u32 = 24u32;
```

### `VFS_CAP_FLAGS_MASK`

```rust
const VFS_CAP_FLAGS_MASK: i64 = -4_278_190_081i64;
```

### `VFS_CAP_FLAGS_EFFECTIVE`

```rust
const VFS_CAP_FLAGS_EFFECTIVE: u32 = 1u32;
```

### `VFS_CAP_REVISION_1`

```rust
const VFS_CAP_REVISION_1: u32 = 16_777_216u32;
```

### `VFS_CAP_U32_1`

```rust
const VFS_CAP_U32_1: u32 = 1u32;
```

### `VFS_CAP_REVISION_2`

```rust
const VFS_CAP_REVISION_2: u32 = 33_554_432u32;
```

### `VFS_CAP_U32_2`

```rust
const VFS_CAP_U32_2: u32 = 2u32;
```

### `VFS_CAP_REVISION_3`

```rust
const VFS_CAP_REVISION_3: u32 = 50_331_648u32;
```

### `VFS_CAP_U32_3`

```rust
const VFS_CAP_U32_3: u32 = 2u32;
```

### `VFS_CAP_U32`

```rust
const VFS_CAP_U32: u32 = 2u32;
```

### `VFS_CAP_REVISION`

```rust
const VFS_CAP_REVISION: u32 = 50_331_648u32;
```

### `_LINUX_CAPABILITY_VERSION`

```rust
const _LINUX_CAPABILITY_VERSION: u32 = 429_392_688u32;
```

### `_LINUX_CAPABILITY_U32S`

```rust
const _LINUX_CAPABILITY_U32S: u32 = 1u32;
```

### `CAP_CHOWN`

```rust
const CAP_CHOWN: u32 = 0u32;
```

### `CAP_DAC_OVERRIDE`

```rust
const CAP_DAC_OVERRIDE: u32 = 1u32;
```

### `CAP_DAC_READ_SEARCH`

```rust
const CAP_DAC_READ_SEARCH: u32 = 2u32;
```

### `CAP_FOWNER`

```rust
const CAP_FOWNER: u32 = 3u32;
```

### `CAP_FSETID`

```rust
const CAP_FSETID: u32 = 4u32;
```

### `CAP_KILL`

```rust
const CAP_KILL: u32 = 5u32;
```

### `CAP_SETGID`

```rust
const CAP_SETGID: u32 = 6u32;
```

### `CAP_SETUID`

```rust
const CAP_SETUID: u32 = 7u32;
```

### `CAP_SETPCAP`

```rust
const CAP_SETPCAP: u32 = 8u32;
```

### `CAP_LINUX_IMMUTABLE`

```rust
const CAP_LINUX_IMMUTABLE: u32 = 9u32;
```

### `CAP_NET_BIND_SERVICE`

```rust
const CAP_NET_BIND_SERVICE: u32 = 10u32;
```

### `CAP_NET_BROADCAST`

```rust
const CAP_NET_BROADCAST: u32 = 11u32;
```

### `CAP_NET_ADMIN`

```rust
const CAP_NET_ADMIN: u32 = 12u32;
```

### `CAP_NET_RAW`

```rust
const CAP_NET_RAW: u32 = 13u32;
```

### `CAP_IPC_LOCK`

```rust
const CAP_IPC_LOCK: u32 = 14u32;
```

### `CAP_IPC_OWNER`

```rust
const CAP_IPC_OWNER: u32 = 15u32;
```

### `CAP_SYS_MODULE`

```rust
const CAP_SYS_MODULE: u32 = 16u32;
```

### `CAP_SYS_RAWIO`

```rust
const CAP_SYS_RAWIO: u32 = 17u32;
```

### `CAP_SYS_CHROOT`

```rust
const CAP_SYS_CHROOT: u32 = 18u32;
```

### `CAP_SYS_PTRACE`

```rust
const CAP_SYS_PTRACE: u32 = 19u32;
```

### `CAP_SYS_PACCT`

```rust
const CAP_SYS_PACCT: u32 = 20u32;
```

### `CAP_SYS_ADMIN`

```rust
const CAP_SYS_ADMIN: u32 = 21u32;
```

### `CAP_SYS_BOOT`

```rust
const CAP_SYS_BOOT: u32 = 22u32;
```

### `CAP_SYS_NICE`

```rust
const CAP_SYS_NICE: u32 = 23u32;
```

### `CAP_SYS_RESOURCE`

```rust
const CAP_SYS_RESOURCE: u32 = 24u32;
```

### `CAP_SYS_TIME`

```rust
const CAP_SYS_TIME: u32 = 25u32;
```

### `CAP_SYS_TTY_CONFIG`

```rust
const CAP_SYS_TTY_CONFIG: u32 = 26u32;
```

### `CAP_MKNOD`

```rust
const CAP_MKNOD: u32 = 27u32;
```

### `CAP_LEASE`

```rust
const CAP_LEASE: u32 = 28u32;
```

### `CAP_AUDIT_WRITE`

```rust
const CAP_AUDIT_WRITE: u32 = 29u32;
```

### `CAP_AUDIT_CONTROL`

```rust
const CAP_AUDIT_CONTROL: u32 = 30u32;
```

### `CAP_SETFCAP`

```rust
const CAP_SETFCAP: u32 = 31u32;
```

### `CAP_MAC_OVERRIDE`

```rust
const CAP_MAC_OVERRIDE: u32 = 32u32;
```

### `CAP_MAC_ADMIN`

```rust
const CAP_MAC_ADMIN: u32 = 33u32;
```

### `CAP_SYSLOG`

```rust
const CAP_SYSLOG: u32 = 34u32;
```

### `CAP_WAKE_ALARM`

```rust
const CAP_WAKE_ALARM: u32 = 35u32;
```

### `CAP_BLOCK_SUSPEND`

```rust
const CAP_BLOCK_SUSPEND: u32 = 36u32;
```

### `CAP_AUDIT_READ`

```rust
const CAP_AUDIT_READ: u32 = 37u32;
```

### `CAP_PERFMON`

```rust
const CAP_PERFMON: u32 = 38u32;
```

### `CAP_BPF`

```rust
const CAP_BPF: u32 = 39u32;
```

### `CAP_CHECKPOINT_RESTORE`

```rust
const CAP_CHECKPOINT_RESTORE: u32 = 40u32;
```

### `CAP_LAST_CAP`

```rust
const CAP_LAST_CAP: u32 = 40u32;
```

### `O_ACCMODE`

```rust
const O_ACCMODE: u32 = 3u32;
```

### `O_RDONLY`

```rust
const O_RDONLY: u32 = 0u32;
```

### `O_WRONLY`

```rust
const O_WRONLY: u32 = 1u32;
```

### `O_RDWR`

```rust
const O_RDWR: u32 = 2u32;
```

### `O_CREAT`

```rust
const O_CREAT: u32 = 64u32;
```

### `O_EXCL`

```rust
const O_EXCL: u32 = 128u32;
```

### `O_NOCTTY`

```rust
const O_NOCTTY: u32 = 256u32;
```

### `O_TRUNC`

```rust
const O_TRUNC: u32 = 512u32;
```

### `O_APPEND`

```rust
const O_APPEND: u32 = 1_024u32;
```

### `O_NONBLOCK`

```rust
const O_NONBLOCK: u32 = 2_048u32;
```

### `O_DSYNC`

```rust
const O_DSYNC: u32 = 4_096u32;
```

### `FASYNC`

```rust
const FASYNC: u32 = 8_192u32;
```

### `O_DIRECT`

```rust
const O_DIRECT: u32 = 16_384u32;
```

### `O_LARGEFILE`

```rust
const O_LARGEFILE: u32 = 32_768u32;
```

### `O_DIRECTORY`

```rust
const O_DIRECTORY: u32 = 65_536u32;
```

### `O_NOFOLLOW`

```rust
const O_NOFOLLOW: u32 = 131_072u32;
```

### `O_NOATIME`

```rust
const O_NOATIME: u32 = 262_144u32;
```

### `O_CLOEXEC`

```rust
const O_CLOEXEC: u32 = 524_288u32;
```

### `__O_SYNC`

```rust
const __O_SYNC: u32 = 1_048_576u32;
```

### `O_SYNC`

```rust
const O_SYNC: u32 = 1_052_672u32;
```

### `O_PATH`

```rust
const O_PATH: u32 = 2_097_152u32;
```

### `__O_TMPFILE`

```rust
const __O_TMPFILE: u32 = 4_194_304u32;
```

### `O_TMPFILE`

```rust
const O_TMPFILE: u32 = 4_259_840u32;
```

### `O_NDELAY`

```rust
const O_NDELAY: u32 = 2_048u32;
```

### `F_DUPFD`

```rust
const F_DUPFD: u32 = 0u32;
```

### `F_GETFD`

```rust
const F_GETFD: u32 = 1u32;
```

### `F_SETFD`

```rust
const F_SETFD: u32 = 2u32;
```

### `F_GETFL`

```rust
const F_GETFL: u32 = 3u32;
```

### `F_SETFL`

```rust
const F_SETFL: u32 = 4u32;
```

### `F_GETLK`

```rust
const F_GETLK: u32 = 5u32;
```

### `F_SETLK`

```rust
const F_SETLK: u32 = 6u32;
```

### `F_SETLKW`

```rust
const F_SETLKW: u32 = 7u32;
```

### `F_SETOWN`

```rust
const F_SETOWN: u32 = 8u32;
```

### `F_GETOWN`

```rust
const F_GETOWN: u32 = 9u32;
```

### `F_SETSIG`

```rust
const F_SETSIG: u32 = 10u32;
```

### `F_GETSIG`

```rust
const F_GETSIG: u32 = 11u32;
```

### `F_SETOWN_EX`

```rust
const F_SETOWN_EX: u32 = 15u32;
```

### `F_GETOWN_EX`

```rust
const F_GETOWN_EX: u32 = 16u32;
```

### `F_GETOWNER_UIDS`

```rust
const F_GETOWNER_UIDS: u32 = 17u32;
```

### `F_OFD_GETLK`

```rust
const F_OFD_GETLK: u32 = 36u32;
```

### `F_OFD_SETLK`

```rust
const F_OFD_SETLK: u32 = 37u32;
```

### `F_OFD_SETLKW`

```rust
const F_OFD_SETLKW: u32 = 38u32;
```

### `F_OWNER_TID`

```rust
const F_OWNER_TID: u32 = 0u32;
```

### `F_OWNER_PID`

```rust
const F_OWNER_PID: u32 = 1u32;
```

### `F_OWNER_PGRP`

```rust
const F_OWNER_PGRP: u32 = 2u32;
```

### `FD_CLOEXEC`

```rust
const FD_CLOEXEC: u32 = 1u32;
```

### `F_RDLCK`

```rust
const F_RDLCK: u32 = 0u32;
```

### `F_WRLCK`

```rust
const F_WRLCK: u32 = 1u32;
```

### `F_UNLCK`

```rust
const F_UNLCK: u32 = 2u32;
```

### `F_EXLCK`

```rust
const F_EXLCK: u32 = 4u32;
```

### `F_SHLCK`

```rust
const F_SHLCK: u32 = 8u32;
```

### `LOCK_SH`

```rust
const LOCK_SH: u32 = 1u32;
```

### `LOCK_EX`

```rust
const LOCK_EX: u32 = 2u32;
```

### `LOCK_NB`

```rust
const LOCK_NB: u32 = 4u32;
```

### `LOCK_UN`

```rust
const LOCK_UN: u32 = 8u32;
```

### `LOCK_MAND`

```rust
const LOCK_MAND: u32 = 32u32;
```

### `LOCK_READ`

```rust
const LOCK_READ: u32 = 64u32;
```

### `LOCK_WRITE`

```rust
const LOCK_WRITE: u32 = 128u32;
```

### `LOCK_RW`

```rust
const LOCK_RW: u32 = 192u32;
```

### `F_LINUX_SPECIFIC_BASE`

```rust
const F_LINUX_SPECIFIC_BASE: u32 = 1_024u32;
```

### `RESOLVE_NO_XDEV`

```rust
const RESOLVE_NO_XDEV: u32 = 1u32;
```

### `RESOLVE_NO_MAGICLINKS`

```rust
const RESOLVE_NO_MAGICLINKS: u32 = 2u32;
```

### `RESOLVE_NO_SYMLINKS`

```rust
const RESOLVE_NO_SYMLINKS: u32 = 4u32;
```

### `RESOLVE_BENEATH`

```rust
const RESOLVE_BENEATH: u32 = 8u32;
```

### `RESOLVE_IN_ROOT`

```rust
const RESOLVE_IN_ROOT: u32 = 16u32;
```

### `RESOLVE_CACHED`

```rust
const RESOLVE_CACHED: u32 = 32u32;
```

### `F_SETLEASE`

```rust
const F_SETLEASE: u32 = 1_024u32;
```

### `F_GETLEASE`

```rust
const F_GETLEASE: u32 = 1_025u32;
```

### `F_NOTIFY`

```rust
const F_NOTIFY: u32 = 1_026u32;
```

### `F_DUPFD_QUERY`

```rust
const F_DUPFD_QUERY: u32 = 1_027u32;
```

### `F_CREATED_QUERY`

```rust
const F_CREATED_QUERY: u32 = 1_028u32;
```

### `F_CANCELLK`

```rust
const F_CANCELLK: u32 = 1_029u32;
```

### `F_DUPFD_CLOEXEC`

```rust
const F_DUPFD_CLOEXEC: u32 = 1_030u32;
```

### `F_SETPIPE_SZ`

```rust
const F_SETPIPE_SZ: u32 = 1_031u32;
```

### `F_GETPIPE_SZ`

```rust
const F_GETPIPE_SZ: u32 = 1_032u32;
```

### `F_ADD_SEALS`

```rust
const F_ADD_SEALS: u32 = 1_033u32;
```

### `F_GET_SEALS`

```rust
const F_GET_SEALS: u32 = 1_034u32;
```

### `F_SEAL_SEAL`

```rust
const F_SEAL_SEAL: u32 = 1u32;
```

### `F_SEAL_SHRINK`

```rust
const F_SEAL_SHRINK: u32 = 2u32;
```

### `F_SEAL_GROW`

```rust
const F_SEAL_GROW: u32 = 4u32;
```

### `F_SEAL_WRITE`

```rust
const F_SEAL_WRITE: u32 = 8u32;
```

### `F_SEAL_FUTURE_WRITE`

```rust
const F_SEAL_FUTURE_WRITE: u32 = 16u32;
```

### `F_SEAL_EXEC`

```rust
const F_SEAL_EXEC: u32 = 32u32;
```

### `F_GET_RW_HINT`

```rust
const F_GET_RW_HINT: u32 = 1_035u32;
```

### `F_SET_RW_HINT`

```rust
const F_SET_RW_HINT: u32 = 1_036u32;
```

### `F_GET_FILE_RW_HINT`

```rust
const F_GET_FILE_RW_HINT: u32 = 1_037u32;
```

### `F_SET_FILE_RW_HINT`

```rust
const F_SET_FILE_RW_HINT: u32 = 1_038u32;
```

### `RWH_WRITE_LIFE_NOT_SET`

```rust
const RWH_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

### `RWH_WRITE_LIFE_NONE`

```rust
const RWH_WRITE_LIFE_NONE: u32 = 1u32;
```

### `RWH_WRITE_LIFE_SHORT`

```rust
const RWH_WRITE_LIFE_SHORT: u32 = 2u32;
```

### `RWH_WRITE_LIFE_MEDIUM`

```rust
const RWH_WRITE_LIFE_MEDIUM: u32 = 3u32;
```

### `RWH_WRITE_LIFE_LONG`

```rust
const RWH_WRITE_LIFE_LONG: u32 = 4u32;
```

### `RWH_WRITE_LIFE_EXTREME`

```rust
const RWH_WRITE_LIFE_EXTREME: u32 = 5u32;
```

### `RWF_WRITE_LIFE_NOT_SET`

```rust
const RWF_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

### `DN_ACCESS`

```rust
const DN_ACCESS: u32 = 1u32;
```

### `DN_MODIFY`

```rust
const DN_MODIFY: u32 = 2u32;
```

### `DN_CREATE`

```rust
const DN_CREATE: u32 = 4u32;
```

### `DN_DELETE`

```rust
const DN_DELETE: u32 = 8u32;
```

### `DN_RENAME`

```rust
const DN_RENAME: u32 = 16u32;
```

### `DN_ATTRIB`

```rust
const DN_ATTRIB: u32 = 32u32;
```

### `DN_MULTISHOT`

```rust
const DN_MULTISHOT: u32 = 2_147_483_648u32;
```

### `AT_FDCWD`

```rust
const AT_FDCWD: i32 = -100i32;
```

### `AT_SYMLINK_NOFOLLOW`

```rust
const AT_SYMLINK_NOFOLLOW: u32 = 256u32;
```

### `AT_SYMLINK_FOLLOW`

```rust
const AT_SYMLINK_FOLLOW: u32 = 1_024u32;
```

### `AT_NO_AUTOMOUNT`

```rust
const AT_NO_AUTOMOUNT: u32 = 2_048u32;
```

### `AT_EMPTY_PATH`

```rust
const AT_EMPTY_PATH: u32 = 4_096u32;
```

### `AT_STATX_SYNC_TYPE`

```rust
const AT_STATX_SYNC_TYPE: u32 = 24_576u32;
```

### `AT_STATX_SYNC_AS_STAT`

```rust
const AT_STATX_SYNC_AS_STAT: u32 = 0u32;
```

### `AT_STATX_FORCE_SYNC`

```rust
const AT_STATX_FORCE_SYNC: u32 = 8_192u32;
```

### `AT_STATX_DONT_SYNC`

```rust
const AT_STATX_DONT_SYNC: u32 = 16_384u32;
```

### `AT_RECURSIVE`

```rust
const AT_RECURSIVE: u32 = 32_768u32;
```

### `AT_RENAME_NOREPLACE`

```rust
const AT_RENAME_NOREPLACE: u32 = 1u32;
```

### `AT_RENAME_EXCHANGE`

```rust
const AT_RENAME_EXCHANGE: u32 = 2u32;
```

### `AT_RENAME_WHITEOUT`

```rust
const AT_RENAME_WHITEOUT: u32 = 4u32;
```

### `AT_EACCESS`

```rust
const AT_EACCESS: u32 = 512u32;
```

### `AT_REMOVEDIR`

```rust
const AT_REMOVEDIR: u32 = 512u32;
```

### `AT_HANDLE_FID`

```rust
const AT_HANDLE_FID: u32 = 512u32;
```

### `AT_HANDLE_MNT_ID_UNIQUE`

```rust
const AT_HANDLE_MNT_ID_UNIQUE: u32 = 1u32;
```

### `AT_HANDLE_CONNECTABLE`

```rust
const AT_HANDLE_CONNECTABLE: u32 = 2u32;
```

### `AT_EXECVE_CHECK`

```rust
const AT_EXECVE_CHECK: u32 = 65_536u32;
```

### `EPOLL_CLOEXEC`

```rust
const EPOLL_CLOEXEC: u32 = 524_288u32;
```

### `EPOLL_CTL_ADD`

```rust
const EPOLL_CTL_ADD: u32 = 1u32;
```

### `EPOLL_CTL_DEL`

```rust
const EPOLL_CTL_DEL: u32 = 2u32;
```

### `EPOLL_CTL_MOD`

```rust
const EPOLL_CTL_MOD: u32 = 3u32;
```

### `EPOLL_IOC_TYPE`

```rust
const EPOLL_IOC_TYPE: u32 = 138u32;
```

### `POSIX_FADV_NORMAL`

```rust
const POSIX_FADV_NORMAL: u32 = 0u32;
```

### `POSIX_FADV_RANDOM`

```rust
const POSIX_FADV_RANDOM: u32 = 1u32;
```

### `POSIX_FADV_SEQUENTIAL`

```rust
const POSIX_FADV_SEQUENTIAL: u32 = 2u32;
```

### `POSIX_FADV_WILLNEED`

```rust
const POSIX_FADV_WILLNEED: u32 = 3u32;
```

### `POSIX_FADV_DONTNEED`

```rust
const POSIX_FADV_DONTNEED: u32 = 4u32;
```

### `POSIX_FADV_NOREUSE`

```rust
const POSIX_FADV_NOREUSE: u32 = 5u32;
```

### `FALLOC_FL_ALLOCATE_RANGE`

```rust
const FALLOC_FL_ALLOCATE_RANGE: u32 = 0u32;
```

### `FALLOC_FL_KEEP_SIZE`

```rust
const FALLOC_FL_KEEP_SIZE: u32 = 1u32;
```

### `FALLOC_FL_PUNCH_HOLE`

```rust
const FALLOC_FL_PUNCH_HOLE: u32 = 2u32;
```

### `FALLOC_FL_NO_HIDE_STALE`

```rust
const FALLOC_FL_NO_HIDE_STALE: u32 = 4u32;
```

### `FALLOC_FL_COLLAPSE_RANGE`

```rust
const FALLOC_FL_COLLAPSE_RANGE: u32 = 8u32;
```

### `FALLOC_FL_ZERO_RANGE`

```rust
const FALLOC_FL_ZERO_RANGE: u32 = 16u32;
```

### `FALLOC_FL_INSERT_RANGE`

```rust
const FALLOC_FL_INSERT_RANGE: u32 = 32u32;
```

### `FALLOC_FL_UNSHARE_RANGE`

```rust
const FALLOC_FL_UNSHARE_RANGE: u32 = 64u32;
```

### `NR_OPEN`

```rust
const NR_OPEN: u32 = 1_024u32;
```

### `NGROUPS_MAX`

```rust
const NGROUPS_MAX: u32 = 65_536u32;
```

### `ARG_MAX`

```rust
const ARG_MAX: u32 = 131_072u32;
```

### `LINK_MAX`

```rust
const LINK_MAX: u32 = 127u32;
```

### `MAX_CANON`

```rust
const MAX_CANON: u32 = 255u32;
```

### `MAX_INPUT`

```rust
const MAX_INPUT: u32 = 255u32;
```

### `NAME_MAX`

```rust
const NAME_MAX: u32 = 255u32;
```

### `PATH_MAX`

```rust
const PATH_MAX: u32 = 4_096u32;
```

### `PIPE_BUF`

```rust
const PIPE_BUF: u32 = 4_096u32;
```

### `XATTR_NAME_MAX`

```rust
const XATTR_NAME_MAX: u32 = 255u32;
```

### `XATTR_SIZE_MAX`

```rust
const XATTR_SIZE_MAX: u32 = 65_536u32;
```

### `XATTR_LIST_MAX`

```rust
const XATTR_LIST_MAX: u32 = 65_536u32;
```

### `RTSIG_MAX`

```rust
const RTSIG_MAX: u32 = 32u32;
```

### `_IOC_NRBITS`

```rust
const _IOC_NRBITS: u32 = 8u32;
```

### `_IOC_TYPEBITS`

```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

### `_IOC_SIZEBITS`

```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

### `_IOC_DIRBITS`

```rust
const _IOC_DIRBITS: u32 = 2u32;
```

### `_IOC_NRMASK`

```rust
const _IOC_NRMASK: u32 = 255u32;
```

### `_IOC_TYPEMASK`

```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

### `_IOC_SIZEMASK`

```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

### `_IOC_DIRMASK`

```rust
const _IOC_DIRMASK: u32 = 3u32;
```

### `_IOC_NRSHIFT`

```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

### `_IOC_TYPESHIFT`

```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

### `_IOC_SIZESHIFT`

```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

### `_IOC_DIRSHIFT`

```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

### `_IOC_NONE`

```rust
const _IOC_NONE: u32 = 0u32;
```

### `_IOC_WRITE`

```rust
const _IOC_WRITE: u32 = 1u32;
```

### `_IOC_READ`

```rust
const _IOC_READ: u32 = 2u32;
```

### `IOC_IN`

```rust
const IOC_IN: u32 = 1_073_741_824u32;
```

### `IOC_OUT`

```rust
const IOC_OUT: u32 = 2_147_483_648u32;
```

### `IOC_INOUT`

```rust
const IOC_INOUT: u32 = 3_221_225_472u32;
```

### `IOCSIZE_MASK`

```rust
const IOCSIZE_MASK: u32 = 1_073_676_288u32;
```

### `IOCSIZE_SHIFT`

```rust
const IOCSIZE_SHIFT: u32 = 16u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_4`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_8`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_16`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_32`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_MASK`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

### `FSCRYPT_POLICY_FLAG_DIRECT_KEY`

```rust
const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`

```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 8u32;
```

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`

```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 16u32;
```

### `FSCRYPT_MODE_AES_256_XTS`

```rust
const FSCRYPT_MODE_AES_256_XTS: u32 = 1u32;
```

### `FSCRYPT_MODE_AES_256_CTS`

```rust
const FSCRYPT_MODE_AES_256_CTS: u32 = 4u32;
```

### `FSCRYPT_MODE_AES_128_CBC`

```rust
const FSCRYPT_MODE_AES_128_CBC: u32 = 5u32;
```

### `FSCRYPT_MODE_AES_128_CTS`

```rust
const FSCRYPT_MODE_AES_128_CTS: u32 = 6u32;
```

### `FSCRYPT_MODE_SM4_XTS`

```rust
const FSCRYPT_MODE_SM4_XTS: u32 = 7u32;
```

### `FSCRYPT_MODE_SM4_CTS`

```rust
const FSCRYPT_MODE_SM4_CTS: u32 = 8u32;
```

### `FSCRYPT_MODE_ADIANTUM`

```rust
const FSCRYPT_MODE_ADIANTUM: u32 = 9u32;
```

### `FSCRYPT_MODE_AES_256_HCTR2`

```rust
const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10u32;
```

### `FSCRYPT_POLICY_V1`

```rust
const FSCRYPT_POLICY_V1: u32 = 0u32;
```

### `FSCRYPT_KEY_DESCRIPTOR_SIZE`

```rust
const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

### `FSCRYPT_KEY_DESC_PREFIX`

```rust
const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9];
```

### `FSCRYPT_KEY_DESC_PREFIX_SIZE`

```rust
const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

### `FSCRYPT_MAX_KEY_SIZE`

```rust
const FSCRYPT_MAX_KEY_SIZE: u32 = 64u32;
```

### `FSCRYPT_POLICY_V2`

```rust
const FSCRYPT_POLICY_V2: u32 = 2u32;
```

### `FSCRYPT_KEY_IDENTIFIER_SIZE`

```rust
const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16u32;
```

### `FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`

```rust
const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1u32;
```

### `FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`

```rust
const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2u32;
```

### `FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`

```rust
const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: u32 = 1u32;
```

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`

```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1u32;
```

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`

```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2u32;
```

### `FSCRYPT_KEY_STATUS_ABSENT`

```rust
const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1u32;
```

### `FSCRYPT_KEY_STATUS_PRESENT`

```rust
const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2u32;
```

### `FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`

```rust
const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3u32;
```

### `FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`

```rust
const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1u32;
```

### `FS_KEY_DESCRIPTOR_SIZE`

```rust
const FS_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

### `FS_POLICY_FLAGS_PAD_4`

```rust
const FS_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

### `FS_POLICY_FLAGS_PAD_8`

```rust
const FS_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

### `FS_POLICY_FLAGS_PAD_16`

```rust
const FS_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

### `FS_POLICY_FLAGS_PAD_32`

```rust
const FS_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

### `FS_POLICY_FLAGS_PAD_MASK`

```rust
const FS_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

### `FS_POLICY_FLAG_DIRECT_KEY`

```rust
const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

### `FS_POLICY_FLAGS_VALID`

```rust
const FS_POLICY_FLAGS_VALID: u32 = 7u32;
```

### `FS_ENCRYPTION_MODE_INVALID`

```rust
const FS_ENCRYPTION_MODE_INVALID: u32 = 0u32;
```

### `FS_ENCRYPTION_MODE_AES_256_XTS`

```rust
const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1u32;
```

### `FS_ENCRYPTION_MODE_AES_256_GCM`

```rust
const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2u32;
```

### `FS_ENCRYPTION_MODE_AES_256_CBC`

```rust
const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3u32;
```

### `FS_ENCRYPTION_MODE_AES_256_CTS`

```rust
const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4u32;
```

### `FS_ENCRYPTION_MODE_AES_128_CBC`

```rust
const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5u32;
```

### `FS_ENCRYPTION_MODE_AES_128_CTS`

```rust
const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6u32;
```

### `FS_ENCRYPTION_MODE_ADIANTUM`

```rust
const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9u32;
```

### `FS_KEY_DESC_PREFIX`

```rust
const FS_KEY_DESC_PREFIX: &[u8; 9];
```

### `FS_KEY_DESC_PREFIX_SIZE`

```rust
const FS_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

### `FS_MAX_KEY_SIZE`

```rust
const FS_MAX_KEY_SIZE: u32 = 64u32;
```

### `MS_RDONLY`

```rust
const MS_RDONLY: u32 = 1u32;
```

### `MS_NOSUID`

```rust
const MS_NOSUID: u32 = 2u32;
```

### `MS_NODEV`

```rust
const MS_NODEV: u32 = 4u32;
```

### `MS_NOEXEC`

```rust
const MS_NOEXEC: u32 = 8u32;
```

### `MS_SYNCHRONOUS`

```rust
const MS_SYNCHRONOUS: u32 = 16u32;
```

### `MS_REMOUNT`

```rust
const MS_REMOUNT: u32 = 32u32;
```

### `MS_MANDLOCK`

```rust
const MS_MANDLOCK: u32 = 64u32;
```

### `MS_DIRSYNC`

```rust
const MS_DIRSYNC: u32 = 128u32;
```

### `MS_NOSYMFOLLOW`

```rust
const MS_NOSYMFOLLOW: u32 = 256u32;
```

### `MS_NOATIME`

```rust
const MS_NOATIME: u32 = 1_024u32;
```

### `MS_NODIRATIME`

```rust
const MS_NODIRATIME: u32 = 2_048u32;
```

### `MS_BIND`

```rust
const MS_BIND: u32 = 4_096u32;
```

### `MS_MOVE`

```rust
const MS_MOVE: u32 = 8_192u32;
```

### `MS_REC`

```rust
const MS_REC: u32 = 16_384u32;
```

### `MS_VERBOSE`

```rust
const MS_VERBOSE: u32 = 32_768u32;
```

### `MS_SILENT`

```rust
const MS_SILENT: u32 = 32_768u32;
```

### `MS_POSIXACL`

```rust
const MS_POSIXACL: u32 = 65_536u32;
```

### `MS_UNBINDABLE`

```rust
const MS_UNBINDABLE: u32 = 131_072u32;
```

### `MS_PRIVATE`

```rust
const MS_PRIVATE: u32 = 262_144u32;
```

### `MS_SLAVE`

```rust
const MS_SLAVE: u32 = 524_288u32;
```

### `MS_SHARED`

```rust
const MS_SHARED: u32 = 1_048_576u32;
```

### `MS_RELATIME`

```rust
const MS_RELATIME: u32 = 2_097_152u32;
```

### `MS_KERNMOUNT`

```rust
const MS_KERNMOUNT: u32 = 4_194_304u32;
```

### `MS_I_VERSION`

```rust
const MS_I_VERSION: u32 = 8_388_608u32;
```

### `MS_STRICTATIME`

```rust
const MS_STRICTATIME: u32 = 16_777_216u32;
```

### `MS_LAZYTIME`

```rust
const MS_LAZYTIME: u32 = 33_554_432u32;
```

### `MS_SUBMOUNT`

```rust
const MS_SUBMOUNT: u32 = 67_108_864u32;
```

### `MS_NOREMOTELOCK`

```rust
const MS_NOREMOTELOCK: u32 = 134_217_728u32;
```

### `MS_NOSEC`

```rust
const MS_NOSEC: u32 = 268_435_456u32;
```

### `MS_BORN`

```rust
const MS_BORN: u32 = 536_870_912u32;
```

### `MS_ACTIVE`

```rust
const MS_ACTIVE: u32 = 1_073_741_824u32;
```

### `MS_NOUSER`

```rust
const MS_NOUSER: u32 = 2_147_483_648u32;
```

### `MS_RMT_MASK`

```rust
const MS_RMT_MASK: u32 = 41_943_121u32;
```

### `MS_MGC_VAL`

```rust
const MS_MGC_VAL: u32 = 3_236_757_504u32;
```

### `MS_MGC_MSK`

```rust
const MS_MGC_MSK: u32 = 4_294_901_760u32;
```

### `OPEN_TREE_CLONE`

```rust
const OPEN_TREE_CLONE: u32 = 1u32;
```

### `OPEN_TREE_CLOEXEC`

```rust
const OPEN_TREE_CLOEXEC: u32 = 524_288u32;
```

### `MOVE_MOUNT_F_SYMLINKS`

```rust
const MOVE_MOUNT_F_SYMLINKS: u32 = 1u32;
```

### `MOVE_MOUNT_F_AUTOMOUNTS`

```rust
const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2u32;
```

### `MOVE_MOUNT_F_EMPTY_PATH`

```rust
const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4u32;
```

### `MOVE_MOUNT_T_SYMLINKS`

```rust
const MOVE_MOUNT_T_SYMLINKS: u32 = 16u32;
```

### `MOVE_MOUNT_T_AUTOMOUNTS`

```rust
const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32u32;
```

### `MOVE_MOUNT_T_EMPTY_PATH`

```rust
const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64u32;
```

### `MOVE_MOUNT_SET_GROUP`

```rust
const MOVE_MOUNT_SET_GROUP: u32 = 256u32;
```

### `MOVE_MOUNT_BENEATH`

```rust
const MOVE_MOUNT_BENEATH: u32 = 512u32;
```

### `MOVE_MOUNT__MASK`

```rust
const MOVE_MOUNT__MASK: u32 = 887u32;
```

### `FSOPEN_CLOEXEC`

```rust
const FSOPEN_CLOEXEC: u32 = 1u32;
```

### `FSPICK_CLOEXEC`

```rust
const FSPICK_CLOEXEC: u32 = 1u32;
```

### `FSPICK_SYMLINK_NOFOLLOW`

```rust
const FSPICK_SYMLINK_NOFOLLOW: u32 = 2u32;
```

### `FSPICK_NO_AUTOMOUNT`

```rust
const FSPICK_NO_AUTOMOUNT: u32 = 4u32;
```

### `FSPICK_EMPTY_PATH`

```rust
const FSPICK_EMPTY_PATH: u32 = 8u32;
```

### `FSMOUNT_CLOEXEC`

```rust
const FSMOUNT_CLOEXEC: u32 = 1u32;
```

### `MOUNT_ATTR_RDONLY`

```rust
const MOUNT_ATTR_RDONLY: u32 = 1u32;
```

### `MOUNT_ATTR_NOSUID`

```rust
const MOUNT_ATTR_NOSUID: u32 = 2u32;
```

### `MOUNT_ATTR_NODEV`

```rust
const MOUNT_ATTR_NODEV: u32 = 4u32;
```

### `MOUNT_ATTR_NOEXEC`

```rust
const MOUNT_ATTR_NOEXEC: u32 = 8u32;
```

### `MOUNT_ATTR__ATIME`

```rust
const MOUNT_ATTR__ATIME: u32 = 112u32;
```

### `MOUNT_ATTR_RELATIME`

```rust
const MOUNT_ATTR_RELATIME: u32 = 0u32;
```

### `MOUNT_ATTR_NOATIME`

```rust
const MOUNT_ATTR_NOATIME: u32 = 16u32;
```

### `MOUNT_ATTR_STRICTATIME`

```rust
const MOUNT_ATTR_STRICTATIME: u32 = 32u32;
```

### `MOUNT_ATTR_NODIRATIME`

```rust
const MOUNT_ATTR_NODIRATIME: u32 = 128u32;
```

### `MOUNT_ATTR_IDMAP`

```rust
const MOUNT_ATTR_IDMAP: u32 = 1_048_576u32;
```

### `MOUNT_ATTR_NOSYMFOLLOW`

```rust
const MOUNT_ATTR_NOSYMFOLLOW: u32 = 2_097_152u32;
```

### `MOUNT_ATTR_SIZE_VER0`

```rust
const MOUNT_ATTR_SIZE_VER0: u32 = 32u32;
```

### `MNT_ID_REQ_SIZE_VER0`

```rust
const MNT_ID_REQ_SIZE_VER0: u32 = 24u32;
```

### `MNT_ID_REQ_SIZE_VER1`

```rust
const MNT_ID_REQ_SIZE_VER1: u32 = 32u32;
```

### `STATMOUNT_SB_BASIC`

```rust
const STATMOUNT_SB_BASIC: u32 = 1u32;
```

### `STATMOUNT_MNT_BASIC`

```rust
const STATMOUNT_MNT_BASIC: u32 = 2u32;
```

### `STATMOUNT_PROPAGATE_FROM`

```rust
const STATMOUNT_PROPAGATE_FROM: u32 = 4u32;
```

### `STATMOUNT_MNT_ROOT`

```rust
const STATMOUNT_MNT_ROOT: u32 = 8u32;
```

### `STATMOUNT_MNT_POINT`

```rust
const STATMOUNT_MNT_POINT: u32 = 16u32;
```

### `STATMOUNT_FS_TYPE`

```rust
const STATMOUNT_FS_TYPE: u32 = 32u32;
```

### `STATMOUNT_MNT_NS_ID`

```rust
const STATMOUNT_MNT_NS_ID: u32 = 64u32;
```

### `STATMOUNT_MNT_OPTS`

```rust
const STATMOUNT_MNT_OPTS: u32 = 128u32;
```

### `STATMOUNT_FS_SUBTYPE`

```rust
const STATMOUNT_FS_SUBTYPE: u32 = 256u32;
```

### `STATMOUNT_SB_SOURCE`

```rust
const STATMOUNT_SB_SOURCE: u32 = 512u32;
```

### `STATMOUNT_OPT_ARRAY`

```rust
const STATMOUNT_OPT_ARRAY: u32 = 1_024u32;
```

### `STATMOUNT_OPT_SEC_ARRAY`

```rust
const STATMOUNT_OPT_SEC_ARRAY: u32 = 2_048u32;
```

### `STATMOUNT_SUPPORTED_MASK`

```rust
const STATMOUNT_SUPPORTED_MASK: u32 = 4_096u32;
```

### `STATMOUNT_MNT_UIDMAP`

```rust
const STATMOUNT_MNT_UIDMAP: u32 = 8_192u32;
```

### `STATMOUNT_MNT_GIDMAP`

```rust
const STATMOUNT_MNT_GIDMAP: u32 = 16_384u32;
```

### `LSMT_ROOT`

```rust
const LSMT_ROOT: i32 = -1i32;
```

### `LISTMOUNT_REVERSE`

```rust
const LISTMOUNT_REVERSE: u32 = 1u32;
```

### `INR_OPEN_CUR`

```rust
const INR_OPEN_CUR: u32 = 1_024u32;
```

### `INR_OPEN_MAX`

```rust
const INR_OPEN_MAX: u32 = 4_096u32;
```

### `BLOCK_SIZE_BITS`

```rust
const BLOCK_SIZE_BITS: u32 = 10u32;
```

### `BLOCK_SIZE`

```rust
const BLOCK_SIZE: u32 = 1_024u32;
```

### `IO_INTEGRITY_CHK_GUARD`

```rust
const IO_INTEGRITY_CHK_GUARD: u32 = 1u32;
```

### `IO_INTEGRITY_CHK_REFTAG`

```rust
const IO_INTEGRITY_CHK_REFTAG: u32 = 2u32;
```

### `IO_INTEGRITY_CHK_APPTAG`

```rust
const IO_INTEGRITY_CHK_APPTAG: u32 = 4u32;
```

### `IO_INTEGRITY_VALID_FLAGS`

```rust
const IO_INTEGRITY_VALID_FLAGS: u32 = 7u32;
```

### `SEEK_SET`

```rust
const SEEK_SET: u32 = 0u32;
```

### `SEEK_CUR`

```rust
const SEEK_CUR: u32 = 1u32;
```

### `SEEK_END`

```rust
const SEEK_END: u32 = 2u32;
```

### `SEEK_DATA`

```rust
const SEEK_DATA: u32 = 3u32;
```

### `SEEK_HOLE`

```rust
const SEEK_HOLE: u32 = 4u32;
```

### `SEEK_MAX`

```rust
const SEEK_MAX: u32 = 4u32;
```

### `RENAME_NOREPLACE`

```rust
const RENAME_NOREPLACE: u32 = 1u32;
```

### `RENAME_EXCHANGE`

```rust
const RENAME_EXCHANGE: u32 = 2u32;
```

### `RENAME_WHITEOUT`

```rust
const RENAME_WHITEOUT: u32 = 4u32;
```

### `FILE_DEDUPE_RANGE_SAME`

```rust
const FILE_DEDUPE_RANGE_SAME: u32 = 0u32;
```

### `FILE_DEDUPE_RANGE_DIFFERS`

```rust
const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1u32;
```

### `NR_FILE`

```rust
const NR_FILE: u32 = 8_192u32;
```

### `FS_XFLAG_REALTIME`

```rust
const FS_XFLAG_REALTIME: u32 = 1u32;
```

### `FS_XFLAG_PREALLOC`

```rust
const FS_XFLAG_PREALLOC: u32 = 2u32;
```

### `FS_XFLAG_IMMUTABLE`

```rust
const FS_XFLAG_IMMUTABLE: u32 = 8u32;
```

### `FS_XFLAG_APPEND`

```rust
const FS_XFLAG_APPEND: u32 = 16u32;
```

### `FS_XFLAG_SYNC`

```rust
const FS_XFLAG_SYNC: u32 = 32u32;
```

### `FS_XFLAG_NOATIME`

```rust
const FS_XFLAG_NOATIME: u32 = 64u32;
```

### `FS_XFLAG_NODUMP`

```rust
const FS_XFLAG_NODUMP: u32 = 128u32;
```

### `FS_XFLAG_RTINHERIT`

```rust
const FS_XFLAG_RTINHERIT: u32 = 256u32;
```

### `FS_XFLAG_PROJINHERIT`

```rust
const FS_XFLAG_PROJINHERIT: u32 = 512u32;
```

### `FS_XFLAG_NOSYMLINKS`

```rust
const FS_XFLAG_NOSYMLINKS: u32 = 1_024u32;
```

### `FS_XFLAG_EXTSIZE`

```rust
const FS_XFLAG_EXTSIZE: u32 = 2_048u32;
```

### `FS_XFLAG_EXTSZINHERIT`

```rust
const FS_XFLAG_EXTSZINHERIT: u32 = 4_096u32;
```

### `FS_XFLAG_NODEFRAG`

```rust
const FS_XFLAG_NODEFRAG: u32 = 8_192u32;
```

### `FS_XFLAG_FILESTREAM`

```rust
const FS_XFLAG_FILESTREAM: u32 = 16_384u32;
```

### `FS_XFLAG_DAX`

```rust
const FS_XFLAG_DAX: u32 = 32_768u32;
```

### `FS_XFLAG_COWEXTSIZE`

```rust
const FS_XFLAG_COWEXTSIZE: u32 = 65_536u32;
```

### `FS_XFLAG_HASATTR`

```rust
const FS_XFLAG_HASATTR: u32 = 2_147_483_648u32;
```

### `BMAP_IOCTL`

```rust
const BMAP_IOCTL: u32 = 1u32;
```

### `FSLABEL_MAX`

```rust
const FSLABEL_MAX: u32 = 256u32;
```

### `FS_SECRM_FL`

```rust
const FS_SECRM_FL: u32 = 1u32;
```

### `FS_UNRM_FL`

```rust
const FS_UNRM_FL: u32 = 2u32;
```

### `FS_COMPR_FL`

```rust
const FS_COMPR_FL: u32 = 4u32;
```

### `FS_SYNC_FL`

```rust
const FS_SYNC_FL: u32 = 8u32;
```

### `FS_IMMUTABLE_FL`

```rust
const FS_IMMUTABLE_FL: u32 = 16u32;
```

### `FS_APPEND_FL`

```rust
const FS_APPEND_FL: u32 = 32u32;
```

### `FS_NODUMP_FL`

```rust
const FS_NODUMP_FL: u32 = 64u32;
```

### `FS_NOATIME_FL`

```rust
const FS_NOATIME_FL: u32 = 128u32;
```

### `FS_DIRTY_FL`

```rust
const FS_DIRTY_FL: u32 = 256u32;
```

### `FS_COMPRBLK_FL`

```rust
const FS_COMPRBLK_FL: u32 = 512u32;
```

### `FS_NOCOMP_FL`

```rust
const FS_NOCOMP_FL: u32 = 1_024u32;
```

### `FS_ENCRYPT_FL`

```rust
const FS_ENCRYPT_FL: u32 = 2_048u32;
```

### `FS_BTREE_FL`

```rust
const FS_BTREE_FL: u32 = 4_096u32;
```

### `FS_INDEX_FL`

```rust
const FS_INDEX_FL: u32 = 4_096u32;
```

### `FS_IMAGIC_FL`

```rust
const FS_IMAGIC_FL: u32 = 8_192u32;
```

### `FS_JOURNAL_DATA_FL`

```rust
const FS_JOURNAL_DATA_FL: u32 = 16_384u32;
```

### `FS_NOTAIL_FL`

```rust
const FS_NOTAIL_FL: u32 = 32_768u32;
```

### `FS_DIRSYNC_FL`

```rust
const FS_DIRSYNC_FL: u32 = 65_536u32;
```

### `FS_TOPDIR_FL`

```rust
const FS_TOPDIR_FL: u32 = 131_072u32;
```

### `FS_HUGE_FILE_FL`

```rust
const FS_HUGE_FILE_FL: u32 = 262_144u32;
```

### `FS_EXTENT_FL`

```rust
const FS_EXTENT_FL: u32 = 524_288u32;
```

### `FS_VERITY_FL`

```rust
const FS_VERITY_FL: u32 = 1_048_576u32;
```

### `FS_EA_INODE_FL`

```rust
const FS_EA_INODE_FL: u32 = 2_097_152u32;
```

### `FS_EOFBLOCKS_FL`

```rust
const FS_EOFBLOCKS_FL: u32 = 4_194_304u32;
```

### `FS_NOCOW_FL`

```rust
const FS_NOCOW_FL: u32 = 8_388_608u32;
```

### `FS_DAX_FL`

```rust
const FS_DAX_FL: u32 = 33_554_432u32;
```

### `FS_INLINE_DATA_FL`

```rust
const FS_INLINE_DATA_FL: u32 = 268_435_456u32;
```

### `FS_PROJINHERIT_FL`

```rust
const FS_PROJINHERIT_FL: u32 = 536_870_912u32;
```

### `FS_CASEFOLD_FL`

```rust
const FS_CASEFOLD_FL: u32 = 1_073_741_824u32;
```

### `FS_RESERVED_FL`

```rust
const FS_RESERVED_FL: u32 = 2_147_483_648u32;
```

### `FS_FL_USER_VISIBLE`

```rust
const FS_FL_USER_VISIBLE: u32 = 253_951u32;
```

### `FS_FL_USER_MODIFIABLE`

```rust
const FS_FL_USER_MODIFIABLE: u32 = 229_631u32;
```

### `SYNC_FILE_RANGE_WAIT_BEFORE`

```rust
const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1u32;
```

### `SYNC_FILE_RANGE_WRITE`

```rust
const SYNC_FILE_RANGE_WRITE: u32 = 2u32;
```

### `SYNC_FILE_RANGE_WAIT_AFTER`

```rust
const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4u32;
```

### `SYNC_FILE_RANGE_WRITE_AND_WAIT`

```rust
const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7u32;
```

### `PROCFS_IOCTL_MAGIC`

```rust
const PROCFS_IOCTL_MAGIC: u8 = 102u8;
```

### `PAGE_IS_WPALLOWED`

```rust
const PAGE_IS_WPALLOWED: u32 = 1u32;
```

### `PAGE_IS_WRITTEN`

```rust
const PAGE_IS_WRITTEN: u32 = 2u32;
```

### `PAGE_IS_FILE`

```rust
const PAGE_IS_FILE: u32 = 4u32;
```

### `PAGE_IS_PRESENT`

```rust
const PAGE_IS_PRESENT: u32 = 8u32;
```

### `PAGE_IS_SWAPPED`

```rust
const PAGE_IS_SWAPPED: u32 = 16u32;
```

### `PAGE_IS_PFNZERO`

```rust
const PAGE_IS_PFNZERO: u32 = 32u32;
```

### `PAGE_IS_HUGE`

```rust
const PAGE_IS_HUGE: u32 = 64u32;
```

### `PAGE_IS_SOFT_DIRTY`

```rust
const PAGE_IS_SOFT_DIRTY: u32 = 128u32;
```

### `PAGE_IS_GUARD`

```rust
const PAGE_IS_GUARD: u32 = 256u32;
```

### `PM_SCAN_WP_MATCHING`

```rust
const PM_SCAN_WP_MATCHING: u32 = 1u32;
```

### `PM_SCAN_CHECK_WPASYNC`

```rust
const PM_SCAN_CHECK_WPASYNC: u32 = 2u32;
```

### `FUTEX_WAIT`

```rust
const FUTEX_WAIT: u32 = 0u32;
```

### `FUTEX_WAKE`

```rust
const FUTEX_WAKE: u32 = 1u32;
```

### `FUTEX_FD`

```rust
const FUTEX_FD: u32 = 2u32;
```

### `FUTEX_REQUEUE`

```rust
const FUTEX_REQUEUE: u32 = 3u32;
```

### `FUTEX_CMP_REQUEUE`

```rust
const FUTEX_CMP_REQUEUE: u32 = 4u32;
```

### `FUTEX_WAKE_OP`

```rust
const FUTEX_WAKE_OP: u32 = 5u32;
```

### `FUTEX_LOCK_PI`

```rust
const FUTEX_LOCK_PI: u32 = 6u32;
```

### `FUTEX_UNLOCK_PI`

```rust
const FUTEX_UNLOCK_PI: u32 = 7u32;
```

### `FUTEX_TRYLOCK_PI`

```rust
const FUTEX_TRYLOCK_PI: u32 = 8u32;
```

### `FUTEX_WAIT_BITSET`

```rust
const FUTEX_WAIT_BITSET: u32 = 9u32;
```

### `FUTEX_WAKE_BITSET`

```rust
const FUTEX_WAKE_BITSET: u32 = 10u32;
```

### `FUTEX_WAIT_REQUEUE_PI`

```rust
const FUTEX_WAIT_REQUEUE_PI: u32 = 11u32;
```

### `FUTEX_CMP_REQUEUE_PI`

```rust
const FUTEX_CMP_REQUEUE_PI: u32 = 12u32;
```

### `FUTEX_LOCK_PI2`

```rust
const FUTEX_LOCK_PI2: u32 = 13u32;
```

### `FUTEX_PRIVATE_FLAG`

```rust
const FUTEX_PRIVATE_FLAG: u32 = 128u32;
```

### `FUTEX_CLOCK_REALTIME`

```rust
const FUTEX_CLOCK_REALTIME: u32 = 256u32;
```

### `FUTEX_CMD_MASK`

```rust
const FUTEX_CMD_MASK: i32 = -385i32;
```

### `FUTEX_WAIT_PRIVATE`

```rust
const FUTEX_WAIT_PRIVATE: u32 = 128u32;
```

### `FUTEX_WAKE_PRIVATE`

```rust
const FUTEX_WAKE_PRIVATE: u32 = 129u32;
```

### `FUTEX_REQUEUE_PRIVATE`

```rust
const FUTEX_REQUEUE_PRIVATE: u32 = 131u32;
```

### `FUTEX_CMP_REQUEUE_PRIVATE`

```rust
const FUTEX_CMP_REQUEUE_PRIVATE: u32 = 132u32;
```

### `FUTEX_WAKE_OP_PRIVATE`

```rust
const FUTEX_WAKE_OP_PRIVATE: u32 = 133u32;
```

### `FUTEX_LOCK_PI_PRIVATE`

```rust
const FUTEX_LOCK_PI_PRIVATE: u32 = 134u32;
```

### `FUTEX_LOCK_PI2_PRIVATE`

```rust
const FUTEX_LOCK_PI2_PRIVATE: u32 = 141u32;
```

### `FUTEX_UNLOCK_PI_PRIVATE`

```rust
const FUTEX_UNLOCK_PI_PRIVATE: u32 = 135u32;
```

### `FUTEX_TRYLOCK_PI_PRIVATE`

```rust
const FUTEX_TRYLOCK_PI_PRIVATE: u32 = 136u32;
```

### `FUTEX_WAIT_BITSET_PRIVATE`

```rust
const FUTEX_WAIT_BITSET_PRIVATE: u32 = 137u32;
```

### `FUTEX_WAKE_BITSET_PRIVATE`

```rust
const FUTEX_WAKE_BITSET_PRIVATE: u32 = 138u32;
```

### `FUTEX_WAIT_REQUEUE_PI_PRIVATE`

```rust
const FUTEX_WAIT_REQUEUE_PI_PRIVATE: u32 = 139u32;
```

### `FUTEX_CMP_REQUEUE_PI_PRIVATE`

```rust
const FUTEX_CMP_REQUEUE_PI_PRIVATE: u32 = 140u32;
```

### `FUTEX2_SIZE_U8`

```rust
const FUTEX2_SIZE_U8: u32 = 0u32;
```

### `FUTEX2_SIZE_U16`

```rust
const FUTEX2_SIZE_U16: u32 = 1u32;
```

### `FUTEX2_SIZE_U32`

```rust
const FUTEX2_SIZE_U32: u32 = 2u32;
```

### `FUTEX2_SIZE_U64`

```rust
const FUTEX2_SIZE_U64: u32 = 3u32;
```

### `FUTEX2_NUMA`

```rust
const FUTEX2_NUMA: u32 = 4u32;
```

### `FUTEX2_MPOL`

```rust
const FUTEX2_MPOL: u32 = 8u32;
```

### `FUTEX2_PRIVATE`

```rust
const FUTEX2_PRIVATE: u32 = 128u32;
```

### `FUTEX2_SIZE_MASK`

```rust
const FUTEX2_SIZE_MASK: u32 = 3u32;
```

### `FUTEX_32`

```rust
const FUTEX_32: u32 = 2u32;
```

### `FUTEX_NO_NODE`

```rust
const FUTEX_NO_NODE: i32 = -1i32;
```

### `FUTEX_WAITV_MAX`

```rust
const FUTEX_WAITV_MAX: u32 = 128u32;
```

### `FUTEX_WAITERS`

```rust
const FUTEX_WAITERS: u32 = 2_147_483_648u32;
```

### `FUTEX_OWNER_DIED`

```rust
const FUTEX_OWNER_DIED: u32 = 1_073_741_824u32;
```

### `FUTEX_TID_MASK`

```rust
const FUTEX_TID_MASK: u32 = 1_073_741_823u32;
```

### `ROBUST_LIST_LIMIT`

```rust
const ROBUST_LIST_LIMIT: u32 = 2_048u32;
```

### `FUTEX_BITSET_MATCH_ANY`

```rust
const FUTEX_BITSET_MATCH_ANY: u32 = 4_294_967_295u32;
```

### `FUTEX_OP_SET`

```rust
const FUTEX_OP_SET: u32 = 0u32;
```

### `FUTEX_OP_ADD`

```rust
const FUTEX_OP_ADD: u32 = 1u32;
```

### `FUTEX_OP_OR`

```rust
const FUTEX_OP_OR: u32 = 2u32;
```

### `FUTEX_OP_ANDN`

```rust
const FUTEX_OP_ANDN: u32 = 3u32;
```

### `FUTEX_OP_XOR`

```rust
const FUTEX_OP_XOR: u32 = 4u32;
```

### `FUTEX_OP_OPARG_SHIFT`

```rust
const FUTEX_OP_OPARG_SHIFT: u32 = 8u32;
```

### `FUTEX_OP_CMP_EQ`

```rust
const FUTEX_OP_CMP_EQ: u32 = 0u32;
```

### `FUTEX_OP_CMP_NE`

```rust
const FUTEX_OP_CMP_NE: u32 = 1u32;
```

### `FUTEX_OP_CMP_LT`

```rust
const FUTEX_OP_CMP_LT: u32 = 2u32;
```

### `FUTEX_OP_CMP_LE`

```rust
const FUTEX_OP_CMP_LE: u32 = 3u32;
```

### `FUTEX_OP_CMP_GT`

```rust
const FUTEX_OP_CMP_GT: u32 = 4u32;
```

### `FUTEX_OP_CMP_GE`

```rust
const FUTEX_OP_CMP_GE: u32 = 5u32;
```

### `IN_ACCESS`

```rust
const IN_ACCESS: u32 = 1u32;
```

### `IN_MODIFY`

```rust
const IN_MODIFY: u32 = 2u32;
```

### `IN_ATTRIB`

```rust
const IN_ATTRIB: u32 = 4u32;
```

### `IN_CLOSE_WRITE`

```rust
const IN_CLOSE_WRITE: u32 = 8u32;
```

### `IN_CLOSE_NOWRITE`

```rust
const IN_CLOSE_NOWRITE: u32 = 16u32;
```

### `IN_OPEN`

```rust
const IN_OPEN: u32 = 32u32;
```

### `IN_MOVED_FROM`

```rust
const IN_MOVED_FROM: u32 = 64u32;
```

### `IN_MOVED_TO`

```rust
const IN_MOVED_TO: u32 = 128u32;
```

### `IN_CREATE`

```rust
const IN_CREATE: u32 = 256u32;
```

### `IN_DELETE`

```rust
const IN_DELETE: u32 = 512u32;
```

### `IN_DELETE_SELF`

```rust
const IN_DELETE_SELF: u32 = 1_024u32;
```

### `IN_MOVE_SELF`

```rust
const IN_MOVE_SELF: u32 = 2_048u32;
```

### `IN_UNMOUNT`

```rust
const IN_UNMOUNT: u32 = 8_192u32;
```

### `IN_Q_OVERFLOW`

```rust
const IN_Q_OVERFLOW: u32 = 16_384u32;
```

### `IN_IGNORED`

```rust
const IN_IGNORED: u32 = 32_768u32;
```

### `IN_CLOSE`

```rust
const IN_CLOSE: u32 = 24u32;
```

### `IN_MOVE`

```rust
const IN_MOVE: u32 = 192u32;
```

### `IN_ONLYDIR`

```rust
const IN_ONLYDIR: u32 = 16_777_216u32;
```

### `IN_DONT_FOLLOW`

```rust
const IN_DONT_FOLLOW: u32 = 33_554_432u32;
```

### `IN_EXCL_UNLINK`

```rust
const IN_EXCL_UNLINK: u32 = 67_108_864u32;
```

### `IN_MASK_CREATE`

```rust
const IN_MASK_CREATE: u32 = 268_435_456u32;
```

### `IN_MASK_ADD`

```rust
const IN_MASK_ADD: u32 = 536_870_912u32;
```

### `IN_ISDIR`

```rust
const IN_ISDIR: u32 = 1_073_741_824u32;
```

### `IN_ONESHOT`

```rust
const IN_ONESHOT: u32 = 2_147_483_648u32;
```

### `IN_ALL_EVENTS`

```rust
const IN_ALL_EVENTS: u32 = 4_095u32;
```

### `IN_CLOEXEC`

```rust
const IN_CLOEXEC: u32 = 524_288u32;
```

### `IN_NONBLOCK`

```rust
const IN_NONBLOCK: u32 = 2_048u32;
```

### `ADFS_SUPER_MAGIC`

```rust
const ADFS_SUPER_MAGIC: u32 = 44_533u32;
```

### `AFFS_SUPER_MAGIC`

```rust
const AFFS_SUPER_MAGIC: u32 = 44_543u32;
```

### `AFS_SUPER_MAGIC`

```rust
const AFS_SUPER_MAGIC: u32 = 1_397_113_167u32;
```

### `AUTOFS_SUPER_MAGIC`

```rust
const AUTOFS_SUPER_MAGIC: u32 = 391u32;
```

### `CEPH_SUPER_MAGIC`

```rust
const CEPH_SUPER_MAGIC: u32 = 12_805_120u32;
```

### `CODA_SUPER_MAGIC`

```rust
const CODA_SUPER_MAGIC: u32 = 1_937_076_805u32;
```

### `CRAMFS_MAGIC`

```rust
const CRAMFS_MAGIC: u32 = 684_539_205u32;
```

### `CRAMFS_MAGIC_WEND`

```rust
const CRAMFS_MAGIC_WEND: u32 = 1_161_678_120u32;
```

### `DEBUGFS_MAGIC`

```rust
const DEBUGFS_MAGIC: u32 = 1_684_170_528u32;
```

### `SECURITYFS_MAGIC`

```rust
const SECURITYFS_MAGIC: u32 = 1_935_894_131u32;
```

### `SELINUX_MAGIC`

```rust
const SELINUX_MAGIC: u32 = 4_185_718_668u32;
```

### `SMACK_MAGIC`

```rust
const SMACK_MAGIC: u32 = 1_128_357_203u32;
```

### `RAMFS_MAGIC`

```rust
const RAMFS_MAGIC: u32 = 2_240_043_254u32;
```

### `TMPFS_MAGIC`

```rust
const TMPFS_MAGIC: u32 = 16_914_836u32;
```

### `HUGETLBFS_MAGIC`

```rust
const HUGETLBFS_MAGIC: u32 = 2_508_478_710u32;
```

### `SQUASHFS_MAGIC`

```rust
const SQUASHFS_MAGIC: u32 = 1_936_814_952u32;
```

### `ECRYPTFS_SUPER_MAGIC`

```rust
const ECRYPTFS_SUPER_MAGIC: u32 = 61_791u32;
```

### `EFS_SUPER_MAGIC`

```rust
const EFS_SUPER_MAGIC: u32 = 4_278_867u32;
```

### `EROFS_SUPER_MAGIC_V1`

```rust
const EROFS_SUPER_MAGIC_V1: u32 = 3_774_210_530u32;
```

### `EXT2_SUPER_MAGIC`

```rust
const EXT2_SUPER_MAGIC: u32 = 61_267u32;
```

### `EXT3_SUPER_MAGIC`

```rust
const EXT3_SUPER_MAGIC: u32 = 61_267u32;
```

### `XENFS_SUPER_MAGIC`

```rust
const XENFS_SUPER_MAGIC: u32 = 2_881_100_148u32;
```

### `EXT4_SUPER_MAGIC`

```rust
const EXT4_SUPER_MAGIC: u32 = 61_267u32;
```

### `BTRFS_SUPER_MAGIC`

```rust
const BTRFS_SUPER_MAGIC: u32 = 2_435_016_766u32;
```

### `NILFS_SUPER_MAGIC`

```rust
const NILFS_SUPER_MAGIC: u32 = 13_364u32;
```

### `F2FS_SUPER_MAGIC`

```rust
const F2FS_SUPER_MAGIC: u32 = 4_076_150_800u32;
```

### `HPFS_SUPER_MAGIC`

```rust
const HPFS_SUPER_MAGIC: u32 = 4_187_351_113u32;
```

### `ISOFS_SUPER_MAGIC`

```rust
const ISOFS_SUPER_MAGIC: u32 = 38_496u32;
```

### `JFFS2_SUPER_MAGIC`

```rust
const JFFS2_SUPER_MAGIC: u32 = 29_366u32;
```

### `XFS_SUPER_MAGIC`

```rust
const XFS_SUPER_MAGIC: u32 = 1_481_003_842u32;
```

### `PSTOREFS_MAGIC`

```rust
const PSTOREFS_MAGIC: u32 = 1_634_035_564u32;
```

### `EFIVARFS_MAGIC`

```rust
const EFIVARFS_MAGIC: u32 = 3_730_735_588u32;
```

### `HOSTFS_SUPER_MAGIC`

```rust
const HOSTFS_SUPER_MAGIC: u32 = 12_648_430u32;
```

### `OVERLAYFS_SUPER_MAGIC`

```rust
const OVERLAYFS_SUPER_MAGIC: u32 = 2_035_054_128u32;
```

### `FUSE_SUPER_MAGIC`

```rust
const FUSE_SUPER_MAGIC: u32 = 1_702_057_286u32;
```

### `BCACHEFS_SUPER_MAGIC`

```rust
const BCACHEFS_SUPER_MAGIC: u32 = 3_393_526_350u32;
```

### `MINIX_SUPER_MAGIC`

```rust
const MINIX_SUPER_MAGIC: u32 = 4_991u32;
```

### `MINIX_SUPER_MAGIC2`

```rust
const MINIX_SUPER_MAGIC2: u32 = 5_007u32;
```

### `MINIX2_SUPER_MAGIC`

```rust
const MINIX2_SUPER_MAGIC: u32 = 9_320u32;
```

### `MINIX2_SUPER_MAGIC2`

```rust
const MINIX2_SUPER_MAGIC2: u32 = 9_336u32;
```

### `MINIX3_SUPER_MAGIC`

```rust
const MINIX3_SUPER_MAGIC: u32 = 19_802u32;
```

### `MSDOS_SUPER_MAGIC`

```rust
const MSDOS_SUPER_MAGIC: u32 = 19_780u32;
```

### `EXFAT_SUPER_MAGIC`

```rust
const EXFAT_SUPER_MAGIC: u32 = 538_032_816u32;
```

### `NCP_SUPER_MAGIC`

```rust
const NCP_SUPER_MAGIC: u32 = 22_092u32;
```

### `NFS_SUPER_MAGIC`

```rust
const NFS_SUPER_MAGIC: u32 = 26_985u32;
```

### `OCFS2_SUPER_MAGIC`

```rust
const OCFS2_SUPER_MAGIC: u32 = 1_952_539_503u32;
```

### `OPENPROM_SUPER_MAGIC`

```rust
const OPENPROM_SUPER_MAGIC: u32 = 40_865u32;
```

### `QNX4_SUPER_MAGIC`

```rust
const QNX4_SUPER_MAGIC: u32 = 47u32;
```

### `QNX6_SUPER_MAGIC`

```rust
const QNX6_SUPER_MAGIC: u32 = 1_746_473_250u32;
```

### `AFS_FS_MAGIC`

```rust
const AFS_FS_MAGIC: u32 = 1_799_439_955u32;
```

### `REISERFS_SUPER_MAGIC`

```rust
const REISERFS_SUPER_MAGIC: u32 = 1_382_369_651u32;
```

### `REISERFS_SUPER_MAGIC_STRING`

```rust
const REISERFS_SUPER_MAGIC_STRING: &[u8; 9];
```

### `REISER2FS_SUPER_MAGIC_STRING`

```rust
const REISER2FS_SUPER_MAGIC_STRING: &[u8; 10];
```

### `REISER2FS_JR_SUPER_MAGIC_STRING`

```rust
const REISER2FS_JR_SUPER_MAGIC_STRING: &[u8; 10];
```

### `SMB_SUPER_MAGIC`

```rust
const SMB_SUPER_MAGIC: u32 = 20_859u32;
```

### `CIFS_SUPER_MAGIC`

```rust
const CIFS_SUPER_MAGIC: u32 = 4_283_649_346u32;
```

### `SMB2_SUPER_MAGIC`

```rust
const SMB2_SUPER_MAGIC: u32 = 4_266_872_130u32;
```

### `CGROUP_SUPER_MAGIC`

```rust
const CGROUP_SUPER_MAGIC: u32 = 2_613_483u32;
```

### `CGROUP2_SUPER_MAGIC`

```rust
const CGROUP2_SUPER_MAGIC: u32 = 1_667_723_888u32;
```

### `RDTGROUP_SUPER_MAGIC`

```rust
const RDTGROUP_SUPER_MAGIC: u32 = 124_082_209u32;
```

### `STACK_END_MAGIC`

```rust
const STACK_END_MAGIC: u32 = 1_470_918_301u32;
```

### `TRACEFS_MAGIC`

```rust
const TRACEFS_MAGIC: u32 = 1_953_653_091u32;
```

### `V9FS_MAGIC`

```rust
const V9FS_MAGIC: u32 = 16_914_839u32;
```

### `BDEVFS_MAGIC`

```rust
const BDEVFS_MAGIC: u32 = 1_650_746_742u32;
```

### `DAXFS_MAGIC`

```rust
const DAXFS_MAGIC: u32 = 1_684_300_152u32;
```

### `BINFMTFS_MAGIC`

```rust
const BINFMTFS_MAGIC: u32 = 1_112_100_429u32;
```

### `DEVPTS_SUPER_MAGIC`

```rust
const DEVPTS_SUPER_MAGIC: u32 = 7_377u32;
```

### `BINDERFS_SUPER_MAGIC`

```rust
const BINDERFS_SUPER_MAGIC: u32 = 1_819_242_352u32;
```

### `FUTEXFS_SUPER_MAGIC`

```rust
const FUTEXFS_SUPER_MAGIC: u32 = 195_894_762u32;
```

### `PIPEFS_MAGIC`

```rust
const PIPEFS_MAGIC: u32 = 1_346_981_957u32;
```

### `PROC_SUPER_MAGIC`

```rust
const PROC_SUPER_MAGIC: u32 = 40_864u32;
```

### `SOCKFS_MAGIC`

```rust
const SOCKFS_MAGIC: u32 = 1_397_703_499u32;
```

### `SYSFS_MAGIC`

```rust
const SYSFS_MAGIC: u32 = 1_650_812_274u32;
```

### `USBDEVICE_SUPER_MAGIC`

```rust
const USBDEVICE_SUPER_MAGIC: u32 = 40_866u32;
```

### `MTD_INODE_FS_MAGIC`

```rust
const MTD_INODE_FS_MAGIC: u32 = 288_389_204u32;
```

### `ANON_INODE_FS_MAGIC`

```rust
const ANON_INODE_FS_MAGIC: u32 = 151_263_540u32;
```

### `BTRFS_TEST_MAGIC`

```rust
const BTRFS_TEST_MAGIC: u32 = 1_936_880_249u32;
```

### `NSFS_MAGIC`

```rust
const NSFS_MAGIC: u32 = 1_853_056_627u32;
```

### `BPF_FS_MAGIC`

```rust
const BPF_FS_MAGIC: u32 = 3_405_662_737u32;
```

### `AAFS_MAGIC`

```rust
const AAFS_MAGIC: u32 = 1_513_908_720u32;
```

### `ZONEFS_MAGIC`

```rust
const ZONEFS_MAGIC: u32 = 1_515_144_787u32;
```

### `UDF_SUPER_MAGIC`

```rust
const UDF_SUPER_MAGIC: u32 = 352_400_198u32;
```

### `DMA_BUF_MAGIC`

```rust
const DMA_BUF_MAGIC: u32 = 1_145_913_666u32;
```

### `DEVMEM_MAGIC`

```rust
const DEVMEM_MAGIC: u32 = 1_162_691_661u32;
```

### `SECRETMEM_MAGIC`

```rust
const SECRETMEM_MAGIC: u32 = 1_397_048_141u32;
```

### `PID_FS_MAGIC`

```rust
const PID_FS_MAGIC: u32 = 1_346_978_886u32;
```

### `MAP_32BIT`

```rust
const MAP_32BIT: u32 = 64u32;
```

### `MAP_ABOVE4G`

```rust
const MAP_ABOVE4G: u32 = 128u32;
```

### `PROT_READ`

```rust
const PROT_READ: u32 = 1u32;
```

### `PROT_WRITE`

```rust
const PROT_WRITE: u32 = 2u32;
```

### `PROT_EXEC`

```rust
const PROT_EXEC: u32 = 4u32;
```

### `PROT_SEM`

```rust
const PROT_SEM: u32 = 8u32;
```

### `PROT_NONE`

```rust
const PROT_NONE: u32 = 0u32;
```

### `PROT_GROWSDOWN`

```rust
const PROT_GROWSDOWN: u32 = 16_777_216u32;
```

### `PROT_GROWSUP`

```rust
const PROT_GROWSUP: u32 = 33_554_432u32;
```

### `MAP_TYPE`

```rust
const MAP_TYPE: u32 = 15u32;
```

### `MAP_FIXED`

```rust
const MAP_FIXED: u32 = 16u32;
```

### `MAP_ANONYMOUS`

```rust
const MAP_ANONYMOUS: u32 = 32u32;
```

### `MAP_POPULATE`

```rust
const MAP_POPULATE: u32 = 32_768u32;
```

### `MAP_NONBLOCK`

```rust
const MAP_NONBLOCK: u32 = 65_536u32;
```

### `MAP_STACK`

```rust
const MAP_STACK: u32 = 131_072u32;
```

### `MAP_HUGETLB`

```rust
const MAP_HUGETLB: u32 = 262_144u32;
```

### `MAP_SYNC`

```rust
const MAP_SYNC: u32 = 524_288u32;
```

### `MAP_FIXED_NOREPLACE`

```rust
const MAP_FIXED_NOREPLACE: u32 = 1_048_576u32;
```

### `MAP_UNINITIALIZED`

```rust
const MAP_UNINITIALIZED: u32 = 67_108_864u32;
```

### `MLOCK_ONFAULT`

```rust
const MLOCK_ONFAULT: u32 = 1u32;
```

### `MS_ASYNC`

```rust
const MS_ASYNC: u32 = 1u32;
```

### `MS_INVALIDATE`

```rust
const MS_INVALIDATE: u32 = 2u32;
```

### `MS_SYNC`

```rust
const MS_SYNC: u32 = 4u32;
```

### `MADV_NORMAL`

```rust
const MADV_NORMAL: u32 = 0u32;
```

### `MADV_RANDOM`

```rust
const MADV_RANDOM: u32 = 1u32;
```

### `MADV_SEQUENTIAL`

```rust
const MADV_SEQUENTIAL: u32 = 2u32;
```

### `MADV_WILLNEED`

```rust
const MADV_WILLNEED: u32 = 3u32;
```

### `MADV_DONTNEED`

```rust
const MADV_DONTNEED: u32 = 4u32;
```

### `MADV_FREE`

```rust
const MADV_FREE: u32 = 8u32;
```

### `MADV_REMOVE`

```rust
const MADV_REMOVE: u32 = 9u32;
```

### `MADV_DONTFORK`

```rust
const MADV_DONTFORK: u32 = 10u32;
```

### `MADV_DOFORK`

```rust
const MADV_DOFORK: u32 = 11u32;
```

### `MADV_HWPOISON`

```rust
const MADV_HWPOISON: u32 = 100u32;
```

### `MADV_SOFT_OFFLINE`

```rust
const MADV_SOFT_OFFLINE: u32 = 101u32;
```

### `MADV_MERGEABLE`

```rust
const MADV_MERGEABLE: u32 = 12u32;
```

### `MADV_UNMERGEABLE`

```rust
const MADV_UNMERGEABLE: u32 = 13u32;
```

### `MADV_HUGEPAGE`

```rust
const MADV_HUGEPAGE: u32 = 14u32;
```

### `MADV_NOHUGEPAGE`

```rust
const MADV_NOHUGEPAGE: u32 = 15u32;
```

### `MADV_DONTDUMP`

```rust
const MADV_DONTDUMP: u32 = 16u32;
```

### `MADV_DODUMP`

```rust
const MADV_DODUMP: u32 = 17u32;
```

### `MADV_WIPEONFORK`

```rust
const MADV_WIPEONFORK: u32 = 18u32;
```

### `MADV_KEEPONFORK`

```rust
const MADV_KEEPONFORK: u32 = 19u32;
```

### `MADV_COLD`

```rust
const MADV_COLD: u32 = 20u32;
```

### `MADV_PAGEOUT`

```rust
const MADV_PAGEOUT: u32 = 21u32;
```

### `MADV_POPULATE_READ`

```rust
const MADV_POPULATE_READ: u32 = 22u32;
```

### `MADV_POPULATE_WRITE`

```rust
const MADV_POPULATE_WRITE: u32 = 23u32;
```

### `MADV_DONTNEED_LOCKED`

```rust
const MADV_DONTNEED_LOCKED: u32 = 24u32;
```

### `MADV_COLLAPSE`

```rust
const MADV_COLLAPSE: u32 = 25u32;
```

### `MADV_GUARD_INSTALL`

```rust
const MADV_GUARD_INSTALL: u32 = 102u32;
```

### `MADV_GUARD_REMOVE`

```rust
const MADV_GUARD_REMOVE: u32 = 103u32;
```

### `MAP_FILE`

```rust
const MAP_FILE: u32 = 0u32;
```

### `PKEY_UNRESTRICTED`

```rust
const PKEY_UNRESTRICTED: u32 = 0u32;
```

### `PKEY_DISABLE_ACCESS`

```rust
const PKEY_DISABLE_ACCESS: u32 = 1u32;
```

### `PKEY_DISABLE_WRITE`

```rust
const PKEY_DISABLE_WRITE: u32 = 2u32;
```

### `PKEY_ACCESS_MASK`

```rust
const PKEY_ACCESS_MASK: u32 = 3u32;
```

### `MAP_GROWSDOWN`

```rust
const MAP_GROWSDOWN: u32 = 256u32;
```

### `MAP_DENYWRITE`

```rust
const MAP_DENYWRITE: u32 = 2_048u32;
```

### `MAP_EXECUTABLE`

```rust
const MAP_EXECUTABLE: u32 = 4_096u32;
```

### `MAP_LOCKED`

```rust
const MAP_LOCKED: u32 = 8_192u32;
```

### `MAP_NORESERVE`

```rust
const MAP_NORESERVE: u32 = 16_384u32;
```

### `MCL_CURRENT`

```rust
const MCL_CURRENT: u32 = 1u32;
```

### `MCL_FUTURE`

```rust
const MCL_FUTURE: u32 = 2u32;
```

### `MCL_ONFAULT`

```rust
const MCL_ONFAULT: u32 = 4u32;
```

### `SHADOW_STACK_SET_TOKEN`

```rust
const SHADOW_STACK_SET_TOKEN: u32 = 1u32;
```

### `SHADOW_STACK_SET_MARKER`

```rust
const SHADOW_STACK_SET_MARKER: u32 = 2u32;
```

### `HUGETLB_FLAG_ENCODE_SHIFT`

```rust
const HUGETLB_FLAG_ENCODE_SHIFT: u32 = 26u32;
```

### `HUGETLB_FLAG_ENCODE_MASK`

```rust
const HUGETLB_FLAG_ENCODE_MASK: u32 = 63u32;
```

### `HUGETLB_FLAG_ENCODE_16KB`

```rust
const HUGETLB_FLAG_ENCODE_16KB: u32 = 939_524_096u32;
```

### `HUGETLB_FLAG_ENCODE_64KB`

```rust
const HUGETLB_FLAG_ENCODE_64KB: u32 = 1_073_741_824u32;
```

### `HUGETLB_FLAG_ENCODE_512KB`

```rust
const HUGETLB_FLAG_ENCODE_512KB: u32 = 1_275_068_416u32;
```

### `HUGETLB_FLAG_ENCODE_1MB`

```rust
const HUGETLB_FLAG_ENCODE_1MB: u32 = 1_342_177_280u32;
```

### `HUGETLB_FLAG_ENCODE_2MB`

```rust
const HUGETLB_FLAG_ENCODE_2MB: u32 = 1_409_286_144u32;
```

### `HUGETLB_FLAG_ENCODE_8MB`

```rust
const HUGETLB_FLAG_ENCODE_8MB: u32 = 1_543_503_872u32;
```

### `HUGETLB_FLAG_ENCODE_16MB`

```rust
const HUGETLB_FLAG_ENCODE_16MB: u32 = 1_610_612_736u32;
```

### `HUGETLB_FLAG_ENCODE_32MB`

```rust
const HUGETLB_FLAG_ENCODE_32MB: u32 = 1_677_721_600u32;
```

### `HUGETLB_FLAG_ENCODE_256MB`

```rust
const HUGETLB_FLAG_ENCODE_256MB: u32 = 1_879_048_192u32;
```

### `HUGETLB_FLAG_ENCODE_512MB`

```rust
const HUGETLB_FLAG_ENCODE_512MB: u32 = 1_946_157_056u32;
```

### `HUGETLB_FLAG_ENCODE_1GB`

```rust
const HUGETLB_FLAG_ENCODE_1GB: u32 = 2_013_265_920u32;
```

### `HUGETLB_FLAG_ENCODE_2GB`

```rust
const HUGETLB_FLAG_ENCODE_2GB: u32 = 2_080_374_784u32;
```

### `HUGETLB_FLAG_ENCODE_16GB`

```rust
const HUGETLB_FLAG_ENCODE_16GB: u32 = 2_281_701_376u32;
```

### `MREMAP_MAYMOVE`

```rust
const MREMAP_MAYMOVE: u32 = 1u32;
```

### `MREMAP_FIXED`

```rust
const MREMAP_FIXED: u32 = 2u32;
```

### `MREMAP_DONTUNMAP`

```rust
const MREMAP_DONTUNMAP: u32 = 4u32;
```

### `OVERCOMMIT_GUESS`

```rust
const OVERCOMMIT_GUESS: u32 = 0u32;
```

### `OVERCOMMIT_ALWAYS`

```rust
const OVERCOMMIT_ALWAYS: u32 = 1u32;
```

### `OVERCOMMIT_NEVER`

```rust
const OVERCOMMIT_NEVER: u32 = 2u32;
```

### `MAP_SHARED`

```rust
const MAP_SHARED: u32 = 1u32;
```

### `MAP_PRIVATE`

```rust
const MAP_PRIVATE: u32 = 2u32;
```

### `MAP_SHARED_VALIDATE`

```rust
const MAP_SHARED_VALIDATE: u32 = 3u32;
```

### `MAP_DROPPABLE`

```rust
const MAP_DROPPABLE: u32 = 8u32;
```

### `MAP_HUGE_SHIFT`

```rust
const MAP_HUGE_SHIFT: u32 = 26u32;
```

### `MAP_HUGE_MASK`

```rust
const MAP_HUGE_MASK: u32 = 63u32;
```

### `MAP_HUGE_16KB`

```rust
const MAP_HUGE_16KB: u32 = 939_524_096u32;
```

### `MAP_HUGE_64KB`

```rust
const MAP_HUGE_64KB: u32 = 1_073_741_824u32;
```

### `MAP_HUGE_512KB`

```rust
const MAP_HUGE_512KB: u32 = 1_275_068_416u32;
```

### `MAP_HUGE_1MB`

```rust
const MAP_HUGE_1MB: u32 = 1_342_177_280u32;
```

### `MAP_HUGE_2MB`

```rust
const MAP_HUGE_2MB: u32 = 1_409_286_144u32;
```

### `MAP_HUGE_8MB`

```rust
const MAP_HUGE_8MB: u32 = 1_543_503_872u32;
```

### `MAP_HUGE_16MB`

```rust
const MAP_HUGE_16MB: u32 = 1_610_612_736u32;
```

### `MAP_HUGE_32MB`

```rust
const MAP_HUGE_32MB: u32 = 1_677_721_600u32;
```

### `MAP_HUGE_256MB`

```rust
const MAP_HUGE_256MB: u32 = 1_879_048_192u32;
```

### `MAP_HUGE_512MB`

```rust
const MAP_HUGE_512MB: u32 = 1_946_157_056u32;
```

### `MAP_HUGE_1GB`

```rust
const MAP_HUGE_1GB: u32 = 2_013_265_920u32;
```

### `MAP_HUGE_2GB`

```rust
const MAP_HUGE_2GB: u32 = 2_080_374_784u32;
```

### `MAP_HUGE_16GB`

```rust
const MAP_HUGE_16GB: u32 = 2_281_701_376u32;
```

### `POLLIN`

```rust
const POLLIN: u32 = 1u32;
```

### `POLLPRI`

```rust
const POLLPRI: u32 = 2u32;
```

### `POLLOUT`

```rust
const POLLOUT: u32 = 4u32;
```

### `POLLERR`

```rust
const POLLERR: u32 = 8u32;
```

### `POLLHUP`

```rust
const POLLHUP: u32 = 16u32;
```

### `POLLNVAL`

```rust
const POLLNVAL: u32 = 32u32;
```

### `POLLRDNORM`

```rust
const POLLRDNORM: u32 = 64u32;
```

### `POLLRDBAND`

```rust
const POLLRDBAND: u32 = 128u32;
```

### `POLLWRNORM`

```rust
const POLLWRNORM: u32 = 256u32;
```

### `POLLWRBAND`

```rust
const POLLWRBAND: u32 = 512u32;
```

### `POLLMSG`

```rust
const POLLMSG: u32 = 1_024u32;
```

### `POLLREMOVE`

```rust
const POLLREMOVE: u32 = 4_096u32;
```

### `POLLRDHUP`

```rust
const POLLRDHUP: u32 = 8_192u32;
```

### `GRND_NONBLOCK`

```rust
const GRND_NONBLOCK: u32 = 1u32;
```

### `GRND_RANDOM`

```rust
const GRND_RANDOM: u32 = 2u32;
```

### `GRND_INSECURE`

```rust
const GRND_INSECURE: u32 = 4u32;
```

### `LINUX_REBOOT_MAGIC1`

```rust
const LINUX_REBOOT_MAGIC1: u32 = 4_276_215_469u32;
```

### `LINUX_REBOOT_MAGIC2`

```rust
const LINUX_REBOOT_MAGIC2: u32 = 672_274_793u32;
```

### `LINUX_REBOOT_MAGIC2A`

```rust
const LINUX_REBOOT_MAGIC2A: u32 = 85_072_278u32;
```

### `LINUX_REBOOT_MAGIC2B`

```rust
const LINUX_REBOOT_MAGIC2B: u32 = 369_367_448u32;
```

### `LINUX_REBOOT_MAGIC2C`

```rust
const LINUX_REBOOT_MAGIC2C: u32 = 537_993_216u32;
```

### `LINUX_REBOOT_CMD_RESTART`

```rust
const LINUX_REBOOT_CMD_RESTART: u32 = 19_088_743u32;
```

### `LINUX_REBOOT_CMD_HALT`

```rust
const LINUX_REBOOT_CMD_HALT: u32 = 3_454_992_675u32;
```

### `LINUX_REBOOT_CMD_CAD_ON`

```rust
const LINUX_REBOOT_CMD_CAD_ON: u32 = 2_309_737_967u32;
```

### `LINUX_REBOOT_CMD_CAD_OFF`

```rust
const LINUX_REBOOT_CMD_CAD_OFF: u32 = 0u32;
```

### `LINUX_REBOOT_CMD_POWER_OFF`

```rust
const LINUX_REBOOT_CMD_POWER_OFF: u32 = 1_126_301_404u32;
```

### `LINUX_REBOOT_CMD_RESTART2`

```rust
const LINUX_REBOOT_CMD_RESTART2: u32 = 2_712_847_316u32;
```

### `LINUX_REBOOT_CMD_SW_SUSPEND`

```rust
const LINUX_REBOOT_CMD_SW_SUSPEND: u32 = 3_489_725_666u32;
```

### `LINUX_REBOOT_CMD_KEXEC`

```rust
const LINUX_REBOOT_CMD_KEXEC: u32 = 1_163_412_803u32;
```

### `RUSAGE_SELF`

```rust
const RUSAGE_SELF: u32 = 0u32;
```

### `RUSAGE_CHILDREN`

```rust
const RUSAGE_CHILDREN: i32 = -1i32;
```

### `RUSAGE_BOTH`

```rust
const RUSAGE_BOTH: i32 = -2i32;
```

### `RUSAGE_THREAD`

```rust
const RUSAGE_THREAD: u32 = 1u32;
```

### `RLIM64_INFINITY`

```rust
const RLIM64_INFINITY: i32 = -1i32;
```

### `PRIO_MIN`

```rust
const PRIO_MIN: i32 = -20i32;
```

### `PRIO_MAX`

```rust
const PRIO_MAX: u32 = 20u32;
```

### `PRIO_PROCESS`

```rust
const PRIO_PROCESS: u32 = 0u32;
```

### `PRIO_PGRP`

```rust
const PRIO_PGRP: u32 = 1u32;
```

### `PRIO_USER`

```rust
const PRIO_USER: u32 = 2u32;
```

### `_STK_LIM`

```rust
const _STK_LIM: u32 = 8_388_608u32;
```

### `MLOCK_LIMIT`

```rust
const MLOCK_LIMIT: u32 = 8_388_608u32;
```

### `RLIMIT_CPU`

```rust
const RLIMIT_CPU: u32 = 0u32;
```

### `RLIMIT_FSIZE`

```rust
const RLIMIT_FSIZE: u32 = 1u32;
```

### `RLIMIT_DATA`

```rust
const RLIMIT_DATA: u32 = 2u32;
```

### `RLIMIT_STACK`

```rust
const RLIMIT_STACK: u32 = 3u32;
```

### `RLIMIT_CORE`

```rust
const RLIMIT_CORE: u32 = 4u32;
```

### `RLIMIT_RSS`

```rust
const RLIMIT_RSS: u32 = 5u32;
```

### `RLIMIT_NPROC`

```rust
const RLIMIT_NPROC: u32 = 6u32;
```

### `RLIMIT_NOFILE`

```rust
const RLIMIT_NOFILE: u32 = 7u32;
```

### `RLIMIT_MEMLOCK`

```rust
const RLIMIT_MEMLOCK: u32 = 8u32;
```

### `RLIMIT_AS`

```rust
const RLIMIT_AS: u32 = 9u32;
```

### `RLIMIT_LOCKS`

```rust
const RLIMIT_LOCKS: u32 = 10u32;
```

### `RLIMIT_SIGPENDING`

```rust
const RLIMIT_SIGPENDING: u32 = 11u32;
```

### `RLIMIT_MSGQUEUE`

```rust
const RLIMIT_MSGQUEUE: u32 = 12u32;
```

### `RLIMIT_NICE`

```rust
const RLIMIT_NICE: u32 = 13u32;
```

### `RLIMIT_RTPRIO`

```rust
const RLIMIT_RTPRIO: u32 = 14u32;
```

### `RLIMIT_RTTIME`

```rust
const RLIMIT_RTTIME: u32 = 15u32;
```

### `RLIM_NLIMITS`

```rust
const RLIM_NLIMITS: u32 = 16u32;
```

### `RLIM_INFINITY`

```rust
const RLIM_INFINITY: i32 = -1i32;
```

### `CSIGNAL`

```rust
const CSIGNAL: u32 = 255u32;
```

### `CLONE_VM`

```rust
const CLONE_VM: u32 = 256u32;
```

### `CLONE_FS`

```rust
const CLONE_FS: u32 = 512u32;
```

### `CLONE_FILES`

```rust
const CLONE_FILES: u32 = 1_024u32;
```

### `CLONE_SIGHAND`

```rust
const CLONE_SIGHAND: u32 = 2_048u32;
```

### `CLONE_PIDFD`

```rust
const CLONE_PIDFD: u32 = 4_096u32;
```

### `CLONE_PTRACE`

```rust
const CLONE_PTRACE: u32 = 8_192u32;
```

### `CLONE_VFORK`

```rust
const CLONE_VFORK: u32 = 16_384u32;
```

### `CLONE_PARENT`

```rust
const CLONE_PARENT: u32 = 32_768u32;
```

### `CLONE_THREAD`

```rust
const CLONE_THREAD: u32 = 65_536u32;
```

### `CLONE_NEWNS`

```rust
const CLONE_NEWNS: u32 = 131_072u32;
```

### `CLONE_SYSVSEM`

```rust
const CLONE_SYSVSEM: u32 = 262_144u32;
```

### `CLONE_SETTLS`

```rust
const CLONE_SETTLS: u32 = 524_288u32;
```

### `CLONE_PARENT_SETTID`

```rust
const CLONE_PARENT_SETTID: u32 = 1_048_576u32;
```

### `CLONE_CHILD_CLEARTID`

```rust
const CLONE_CHILD_CLEARTID: u32 = 2_097_152u32;
```

### `CLONE_DETACHED`

```rust
const CLONE_DETACHED: u32 = 4_194_304u32;
```

### `CLONE_UNTRACED`

```rust
const CLONE_UNTRACED: u32 = 8_388_608u32;
```

### `CLONE_CHILD_SETTID`

```rust
const CLONE_CHILD_SETTID: u32 = 16_777_216u32;
```

### `CLONE_NEWCGROUP`

```rust
const CLONE_NEWCGROUP: u32 = 33_554_432u32;
```

### `CLONE_NEWUTS`

```rust
const CLONE_NEWUTS: u32 = 67_108_864u32;
```

### `CLONE_NEWIPC`

```rust
const CLONE_NEWIPC: u32 = 134_217_728u32;
```

### `CLONE_NEWUSER`

```rust
const CLONE_NEWUSER: u32 = 268_435_456u32;
```

### `CLONE_NEWPID`

```rust
const CLONE_NEWPID: u32 = 536_870_912u32;
```

### `CLONE_NEWNET`

```rust
const CLONE_NEWNET: u32 = 1_073_741_824u32;
```

### `CLONE_IO`

```rust
const CLONE_IO: u32 = 2_147_483_648u32;
```

### `CLONE_CLEAR_SIGHAND`

```rust
const CLONE_CLEAR_SIGHAND: u64 = 4_294_967_296u64;
```

### `CLONE_INTO_CGROUP`

```rust
const CLONE_INTO_CGROUP: u64 = 8_589_934_592u64;
```

### `CLONE_NEWTIME`

```rust
const CLONE_NEWTIME: u32 = 128u32;
```

### `CLONE_ARGS_SIZE_VER0`

```rust
const CLONE_ARGS_SIZE_VER0: u32 = 64u32;
```

### `CLONE_ARGS_SIZE_VER1`

```rust
const CLONE_ARGS_SIZE_VER1: u32 = 80u32;
```

### `CLONE_ARGS_SIZE_VER2`

```rust
const CLONE_ARGS_SIZE_VER2: u32 = 88u32;
```

### `SCHED_NORMAL`

```rust
const SCHED_NORMAL: u32 = 0u32;
```

### `SCHED_FIFO`

```rust
const SCHED_FIFO: u32 = 1u32;
```

### `SCHED_RR`

```rust
const SCHED_RR: u32 = 2u32;
```

### `SCHED_BATCH`

```rust
const SCHED_BATCH: u32 = 3u32;
```

### `SCHED_IDLE`

```rust
const SCHED_IDLE: u32 = 5u32;
```

### `SCHED_DEADLINE`

```rust
const SCHED_DEADLINE: u32 = 6u32;
```

### `SCHED_EXT`

```rust
const SCHED_EXT: u32 = 7u32;
```

### `SCHED_RESET_ON_FORK`

```rust
const SCHED_RESET_ON_FORK: u32 = 1_073_741_824u32;
```

### `SCHED_FLAG_RESET_ON_FORK`

```rust
const SCHED_FLAG_RESET_ON_FORK: u32 = 1u32;
```

### `SCHED_FLAG_RECLAIM`

```rust
const SCHED_FLAG_RECLAIM: u32 = 2u32;
```

### `SCHED_FLAG_DL_OVERRUN`

```rust
const SCHED_FLAG_DL_OVERRUN: u32 = 4u32;
```

### `SCHED_FLAG_KEEP_POLICY`

```rust
const SCHED_FLAG_KEEP_POLICY: u32 = 8u32;
```

### `SCHED_FLAG_KEEP_PARAMS`

```rust
const SCHED_FLAG_KEEP_PARAMS: u32 = 16u32;
```

### `SCHED_FLAG_UTIL_CLAMP_MIN`

```rust
const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 32u32;
```

### `SCHED_FLAG_UTIL_CLAMP_MAX`

```rust
const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 64u32;
```

### `SCHED_FLAG_KEEP_ALL`

```rust
const SCHED_FLAG_KEEP_ALL: u32 = 24u32;
```

### `SCHED_FLAG_UTIL_CLAMP`

```rust
const SCHED_FLAG_UTIL_CLAMP: u32 = 96u32;
```

### `SCHED_FLAG_ALL`

```rust
const SCHED_FLAG_ALL: u32 = 127u32;
```

### `NSIG`

```rust
const NSIG: u32 = 32u32;
```

### `SIGHUP`

```rust
const SIGHUP: u32 = 1u32;
```

### `SIGINT`

```rust
const SIGINT: u32 = 2u32;
```

### `SIGQUIT`

```rust
const SIGQUIT: u32 = 3u32;
```

### `SIGILL`

```rust
const SIGILL: u32 = 4u32;
```

### `SIGTRAP`

```rust
const SIGTRAP: u32 = 5u32;
```

### `SIGABRT`

```rust
const SIGABRT: u32 = 6u32;
```

### `SIGIOT`

```rust
const SIGIOT: u32 = 6u32;
```

### `SIGBUS`

```rust
const SIGBUS: u32 = 7u32;
```

### `SIGFPE`

```rust
const SIGFPE: u32 = 8u32;
```

### `SIGKILL`

```rust
const SIGKILL: u32 = 9u32;
```

### `SIGUSR1`

```rust
const SIGUSR1: u32 = 10u32;
```

### `SIGSEGV`

```rust
const SIGSEGV: u32 = 11u32;
```

### `SIGUSR2`

```rust
const SIGUSR2: u32 = 12u32;
```

### `SIGPIPE`

```rust
const SIGPIPE: u32 = 13u32;
```

### `SIGALRM`

```rust
const SIGALRM: u32 = 14u32;
```

### `SIGTERM`

```rust
const SIGTERM: u32 = 15u32;
```

### `SIGSTKFLT`

```rust
const SIGSTKFLT: u32 = 16u32;
```

### `SIGCHLD`

```rust
const SIGCHLD: u32 = 17u32;
```

### `SIGCONT`

```rust
const SIGCONT: u32 = 18u32;
```

### `SIGSTOP`

```rust
const SIGSTOP: u32 = 19u32;
```

### `SIGTSTP`

```rust
const SIGTSTP: u32 = 20u32;
```

### `SIGTTIN`

```rust
const SIGTTIN: u32 = 21u32;
```

### `SIGTTOU`

```rust
const SIGTTOU: u32 = 22u32;
```

### `SIGURG`

```rust
const SIGURG: u32 = 23u32;
```

### `SIGXCPU`

```rust
const SIGXCPU: u32 = 24u32;
```

### `SIGXFSZ`

```rust
const SIGXFSZ: u32 = 25u32;
```

### `SIGVTALRM`

```rust
const SIGVTALRM: u32 = 26u32;
```

### `SIGPROF`

```rust
const SIGPROF: u32 = 27u32;
```

### `SIGWINCH`

```rust
const SIGWINCH: u32 = 28u32;
```

### `SIGIO`

```rust
const SIGIO: u32 = 29u32;
```

### `SIGPOLL`

```rust
const SIGPOLL: u32 = 29u32;
```

### `SIGPWR`

```rust
const SIGPWR: u32 = 30u32;
```

### `SIGSYS`

```rust
const SIGSYS: u32 = 31u32;
```

### `SIGUNUSED`

```rust
const SIGUNUSED: u32 = 31u32;
```

### `SIGRTMIN`

```rust
const SIGRTMIN: u32 = 32u32;
```

### `SA_RESTORER`

```rust
const SA_RESTORER: u32 = 67_108_864u32;
```

### `MINSIGSTKSZ`

```rust
const MINSIGSTKSZ: u32 = 2_048u32;
```

### `SIGSTKSZ`

```rust
const SIGSTKSZ: u32 = 8_192u32;
```

### `SA_NOCLDSTOP`

```rust
const SA_NOCLDSTOP: u32 = 1u32;
```

### `SA_NOCLDWAIT`

```rust
const SA_NOCLDWAIT: u32 = 2u32;
```

### `SA_SIGINFO`

```rust
const SA_SIGINFO: u32 = 4u32;
```

### `SA_UNSUPPORTED`

```rust
const SA_UNSUPPORTED: u32 = 1_024u32;
```

### `SA_EXPOSE_TAGBITS`

```rust
const SA_EXPOSE_TAGBITS: u32 = 2_048u32;
```

### `SA_ONSTACK`

```rust
const SA_ONSTACK: u32 = 134_217_728u32;
```

### `SA_RESTART`

```rust
const SA_RESTART: u32 = 268_435_456u32;
```

### `SA_NODEFER`

```rust
const SA_NODEFER: u32 = 1_073_741_824u32;
```

### `SA_RESETHAND`

```rust
const SA_RESETHAND: u32 = 2_147_483_648u32;
```

### `SA_NOMASK`

```rust
const SA_NOMASK: u32 = 1_073_741_824u32;
```

### `SA_ONESHOT`

```rust
const SA_ONESHOT: u32 = 2_147_483_648u32;
```

### `SIG_BLOCK`

```rust
const SIG_BLOCK: u32 = 0u32;
```

### `SIG_UNBLOCK`

```rust
const SIG_UNBLOCK: u32 = 1u32;
```

### `SIG_SETMASK`

```rust
const SIG_SETMASK: u32 = 2u32;
```

### `SI_MAX_SIZE`

```rust
const SI_MAX_SIZE: u32 = 128u32;
```

### `SI_USER`

```rust
const SI_USER: u32 = 0u32;
```

### `SI_KERNEL`

```rust
const SI_KERNEL: u32 = 128u32;
```

### `SI_QUEUE`

```rust
const SI_QUEUE: i32 = -1i32;
```

### `SI_TIMER`

```rust
const SI_TIMER: i32 = -2i32;
```

### `SI_MESGQ`

```rust
const SI_MESGQ: i32 = -3i32;
```

### `SI_ASYNCIO`

```rust
const SI_ASYNCIO: i32 = -4i32;
```

### `SI_SIGIO`

```rust
const SI_SIGIO: i32 = -5i32;
```

### `SI_TKILL`

```rust
const SI_TKILL: i32 = -6i32;
```

### `SI_DETHREAD`

```rust
const SI_DETHREAD: i32 = -7i32;
```

### `SI_ASYNCNL`

```rust
const SI_ASYNCNL: i32 = -60i32;
```

### `ILL_ILLOPC`

```rust
const ILL_ILLOPC: u32 = 1u32;
```

### `ILL_ILLOPN`

```rust
const ILL_ILLOPN: u32 = 2u32;
```

### `ILL_ILLADR`

```rust
const ILL_ILLADR: u32 = 3u32;
```

### `ILL_ILLTRP`

```rust
const ILL_ILLTRP: u32 = 4u32;
```

### `ILL_PRVOPC`

```rust
const ILL_PRVOPC: u32 = 5u32;
```

### `ILL_PRVREG`

```rust
const ILL_PRVREG: u32 = 6u32;
```

### `ILL_COPROC`

```rust
const ILL_COPROC: u32 = 7u32;
```

### `ILL_BADSTK`

```rust
const ILL_BADSTK: u32 = 8u32;
```

### `ILL_BADIADDR`

```rust
const ILL_BADIADDR: u32 = 9u32;
```

### `__ILL_BREAK`

```rust
const __ILL_BREAK: u32 = 10u32;
```

### `__ILL_BNDMOD`

```rust
const __ILL_BNDMOD: u32 = 11u32;
```

### `NSIGILL`

```rust
const NSIGILL: u32 = 11u32;
```

### `FPE_INTDIV`

```rust
const FPE_INTDIV: u32 = 1u32;
```

### `FPE_INTOVF`

```rust
const FPE_INTOVF: u32 = 2u32;
```

### `FPE_FLTDIV`

```rust
const FPE_FLTDIV: u32 = 3u32;
```

### `FPE_FLTOVF`

```rust
const FPE_FLTOVF: u32 = 4u32;
```

### `FPE_FLTUND`

```rust
const FPE_FLTUND: u32 = 5u32;
```

### `FPE_FLTRES`

```rust
const FPE_FLTRES: u32 = 6u32;
```

### `FPE_FLTINV`

```rust
const FPE_FLTINV: u32 = 7u32;
```

### `FPE_FLTSUB`

```rust
const FPE_FLTSUB: u32 = 8u32;
```

### `__FPE_DECOVF`

```rust
const __FPE_DECOVF: u32 = 9u32;
```

### `__FPE_DECDIV`

```rust
const __FPE_DECDIV: u32 = 10u32;
```

### `__FPE_DECERR`

```rust
const __FPE_DECERR: u32 = 11u32;
```

### `__FPE_INVASC`

```rust
const __FPE_INVASC: u32 = 12u32;
```

### `__FPE_INVDEC`

```rust
const __FPE_INVDEC: u32 = 13u32;
```

### `FPE_FLTUNK`

```rust
const FPE_FLTUNK: u32 = 14u32;
```

### `FPE_CONDTRAP`

```rust
const FPE_CONDTRAP: u32 = 15u32;
```

### `NSIGFPE`

```rust
const NSIGFPE: u32 = 15u32;
```

### `SEGV_MAPERR`

```rust
const SEGV_MAPERR: u32 = 1u32;
```

### `SEGV_ACCERR`

```rust
const SEGV_ACCERR: u32 = 2u32;
```

### `SEGV_BNDERR`

```rust
const SEGV_BNDERR: u32 = 3u32;
```

### `SEGV_PKUERR`

```rust
const SEGV_PKUERR: u32 = 4u32;
```

### `SEGV_ACCADI`

```rust
const SEGV_ACCADI: u32 = 5u32;
```

### `SEGV_ADIDERR`

```rust
const SEGV_ADIDERR: u32 = 6u32;
```

### `SEGV_ADIPERR`

```rust
const SEGV_ADIPERR: u32 = 7u32;
```

### `SEGV_MTEAERR`

```rust
const SEGV_MTEAERR: u32 = 8u32;
```

### `SEGV_MTESERR`

```rust
const SEGV_MTESERR: u32 = 9u32;
```

### `SEGV_CPERR`

```rust
const SEGV_CPERR: u32 = 10u32;
```

### `NSIGSEGV`

```rust
const NSIGSEGV: u32 = 10u32;
```

### `BUS_ADRALN`

```rust
const BUS_ADRALN: u32 = 1u32;
```

### `BUS_ADRERR`

```rust
const BUS_ADRERR: u32 = 2u32;
```

### `BUS_OBJERR`

```rust
const BUS_OBJERR: u32 = 3u32;
```

### `BUS_MCEERR_AR`

```rust
const BUS_MCEERR_AR: u32 = 4u32;
```

### `BUS_MCEERR_AO`

```rust
const BUS_MCEERR_AO: u32 = 5u32;
```

### `NSIGBUS`

```rust
const NSIGBUS: u32 = 5u32;
```

### `TRAP_BRKPT`

```rust
const TRAP_BRKPT: u32 = 1u32;
```

### `TRAP_TRACE`

```rust
const TRAP_TRACE: u32 = 2u32;
```

### `TRAP_BRANCH`

```rust
const TRAP_BRANCH: u32 = 3u32;
```

### `TRAP_HWBKPT`

```rust
const TRAP_HWBKPT: u32 = 4u32;
```

### `TRAP_UNK`

```rust
const TRAP_UNK: u32 = 5u32;
```

### `TRAP_PERF`

```rust
const TRAP_PERF: u32 = 6u32;
```

### `NSIGTRAP`

```rust
const NSIGTRAP: u32 = 6u32;
```

### `TRAP_PERF_FLAG_ASYNC`

```rust
const TRAP_PERF_FLAG_ASYNC: u32 = 1u32;
```

### `CLD_EXITED`

```rust
const CLD_EXITED: u32 = 1u32;
```

### `CLD_KILLED`

```rust
const CLD_KILLED: u32 = 2u32;
```

### `CLD_DUMPED`

```rust
const CLD_DUMPED: u32 = 3u32;
```

### `CLD_TRAPPED`

```rust
const CLD_TRAPPED: u32 = 4u32;
```

### `CLD_STOPPED`

```rust
const CLD_STOPPED: u32 = 5u32;
```

### `CLD_CONTINUED`

```rust
const CLD_CONTINUED: u32 = 6u32;
```

### `NSIGCHLD`

```rust
const NSIGCHLD: u32 = 6u32;
```

### `POLL_IN`

```rust
const POLL_IN: u32 = 1u32;
```

### `POLL_OUT`

```rust
const POLL_OUT: u32 = 2u32;
```

### `POLL_MSG`

```rust
const POLL_MSG: u32 = 3u32;
```

### `POLL_ERR`

```rust
const POLL_ERR: u32 = 4u32;
```

### `POLL_PRI`

```rust
const POLL_PRI: u32 = 5u32;
```

### `POLL_HUP`

```rust
const POLL_HUP: u32 = 6u32;
```

### `NSIGPOLL`

```rust
const NSIGPOLL: u32 = 6u32;
```

### `SYS_SECCOMP`

```rust
const SYS_SECCOMP: u32 = 1u32;
```

### `SYS_USER_DISPATCH`

```rust
const SYS_USER_DISPATCH: u32 = 2u32;
```

### `NSIGSYS`

```rust
const NSIGSYS: u32 = 2u32;
```

### `EMT_TAGOVF`

```rust
const EMT_TAGOVF: u32 = 1u32;
```

### `NSIGEMT`

```rust
const NSIGEMT: u32 = 1u32;
```

### `SIGEV_SIGNAL`

```rust
const SIGEV_SIGNAL: u32 = 0u32;
```

### `SIGEV_NONE`

```rust
const SIGEV_NONE: u32 = 1u32;
```

### `SIGEV_THREAD`

```rust
const SIGEV_THREAD: u32 = 2u32;
```

### `SIGEV_THREAD_ID`

```rust
const SIGEV_THREAD_ID: u32 = 4u32;
```

### `SIGEV_MAX_SIZE`

```rust
const SIGEV_MAX_SIZE: u32 = 64u32;
```

### `SS_ONSTACK`

```rust
const SS_ONSTACK: u32 = 1u32;
```

### `SS_DISABLE`

```rust
const SS_DISABLE: u32 = 2u32;
```

### `SS_AUTODISARM`

```rust
const SS_AUTODISARM: u32 = 2_147_483_648u32;
```

### `SS_FLAG_BITS`

```rust
const SS_FLAG_BITS: u32 = 2_147_483_648u32;
```

### `S_IFMT`

```rust
const S_IFMT: u32 = 61_440u32;
```

### `S_IFSOCK`

```rust
const S_IFSOCK: u32 = 49_152u32;
```

### `S_IFLNK`

```rust
const S_IFLNK: u32 = 40_960u32;
```

### `S_IFREG`

```rust
const S_IFREG: u32 = 32_768u32;
```

### `S_IFBLK`

```rust
const S_IFBLK: u32 = 24_576u32;
```

### `S_IFDIR`

```rust
const S_IFDIR: u32 = 16_384u32;
```

### `S_IFCHR`

```rust
const S_IFCHR: u32 = 8_192u32;
```

### `S_IFIFO`

```rust
const S_IFIFO: u32 = 4_096u32;
```

### `S_ISUID`

```rust
const S_ISUID: u32 = 2_048u32;
```

### `S_ISGID`

```rust
const S_ISGID: u32 = 1_024u32;
```

### `S_ISVTX`

```rust
const S_ISVTX: u32 = 512u32;
```

### `S_IRWXU`

```rust
const S_IRWXU: u32 = 448u32;
```

### `S_IRUSR`

```rust
const S_IRUSR: u32 = 256u32;
```

### `S_IWUSR`

```rust
const S_IWUSR: u32 = 128u32;
```

### `S_IXUSR`

```rust
const S_IXUSR: u32 = 64u32;
```

### `S_IRWXG`

```rust
const S_IRWXG: u32 = 56u32;
```

### `S_IRGRP`

```rust
const S_IRGRP: u32 = 32u32;
```

### `S_IWGRP`

```rust
const S_IWGRP: u32 = 16u32;
```

### `S_IXGRP`

```rust
const S_IXGRP: u32 = 8u32;
```

### `S_IRWXO`

```rust
const S_IRWXO: u32 = 7u32;
```

### `S_IROTH`

```rust
const S_IROTH: u32 = 4u32;
```

### `S_IWOTH`

```rust
const S_IWOTH: u32 = 2u32;
```

### `S_IXOTH`

```rust
const S_IXOTH: u32 = 1u32;
```

### `STATX_TYPE`

```rust
const STATX_TYPE: u32 = 1u32;
```

### `STATX_MODE`

```rust
const STATX_MODE: u32 = 2u32;
```

### `STATX_NLINK`

```rust
const STATX_NLINK: u32 = 4u32;
```

### `STATX_UID`

```rust
const STATX_UID: u32 = 8u32;
```

### `STATX_GID`

```rust
const STATX_GID: u32 = 16u32;
```

### `STATX_ATIME`

```rust
const STATX_ATIME: u32 = 32u32;
```

### `STATX_MTIME`

```rust
const STATX_MTIME: u32 = 64u32;
```

### `STATX_CTIME`

```rust
const STATX_CTIME: u32 = 128u32;
```

### `STATX_INO`

```rust
const STATX_INO: u32 = 256u32;
```

### `STATX_SIZE`

```rust
const STATX_SIZE: u32 = 512u32;
```

### `STATX_BLOCKS`

```rust
const STATX_BLOCKS: u32 = 1_024u32;
```

### `STATX_BASIC_STATS`

```rust
const STATX_BASIC_STATS: u32 = 2_047u32;
```

### `STATX_BTIME`

```rust
const STATX_BTIME: u32 = 2_048u32;
```

### `STATX_MNT_ID`

```rust
const STATX_MNT_ID: u32 = 4_096u32;
```

### `STATX_DIOALIGN`

```rust
const STATX_DIOALIGN: u32 = 8_192u32;
```

### `STATX_MNT_ID_UNIQUE`

```rust
const STATX_MNT_ID_UNIQUE: u32 = 16_384u32;
```

### `STATX_SUBVOL`

```rust
const STATX_SUBVOL: u32 = 32_768u32;
```

### `STATX_WRITE_ATOMIC`

```rust
const STATX_WRITE_ATOMIC: u32 = 65_536u32;
```

### `STATX_DIO_READ_ALIGN`

```rust
const STATX_DIO_READ_ALIGN: u32 = 131_072u32;
```

### `STATX__RESERVED`

```rust
const STATX__RESERVED: u32 = 2_147_483_648u32;
```

### `STATX_ALL`

```rust
const STATX_ALL: u32 = 4_095u32;
```

### `STATX_ATTR_COMPRESSED`

```rust
const STATX_ATTR_COMPRESSED: u32 = 4u32;
```

### `STATX_ATTR_IMMUTABLE`

```rust
const STATX_ATTR_IMMUTABLE: u32 = 16u32;
```

### `STATX_ATTR_APPEND`

```rust
const STATX_ATTR_APPEND: u32 = 32u32;
```

### `STATX_ATTR_NODUMP`

```rust
const STATX_ATTR_NODUMP: u32 = 64u32;
```

### `STATX_ATTR_ENCRYPTED`

```rust
const STATX_ATTR_ENCRYPTED: u32 = 2_048u32;
```

### `STATX_ATTR_AUTOMOUNT`

```rust
const STATX_ATTR_AUTOMOUNT: u32 = 4_096u32;
```

### `STATX_ATTR_MOUNT_ROOT`

```rust
const STATX_ATTR_MOUNT_ROOT: u32 = 8_192u32;
```

### `STATX_ATTR_VERITY`

```rust
const STATX_ATTR_VERITY: u32 = 1_048_576u32;
```

### `STATX_ATTR_DAX`

```rust
const STATX_ATTR_DAX: u32 = 2_097_152u32;
```

### `STATX_ATTR_WRITE_ATOMIC`

```rust
const STATX_ATTR_WRITE_ATOMIC: u32 = 4_194_304u32;
```

### `IGNBRK`

```rust
const IGNBRK: u32 = 1u32;
```

### `BRKINT`

```rust
const BRKINT: u32 = 2u32;
```

### `IGNPAR`

```rust
const IGNPAR: u32 = 4u32;
```

### `PARMRK`

```rust
const PARMRK: u32 = 8u32;
```

### `INPCK`

```rust
const INPCK: u32 = 16u32;
```

### `ISTRIP`

```rust
const ISTRIP: u32 = 32u32;
```

### `INLCR`

```rust
const INLCR: u32 = 64u32;
```

### `IGNCR`

```rust
const IGNCR: u32 = 128u32;
```

### `ICRNL`

```rust
const ICRNL: u32 = 256u32;
```

### `IXANY`

```rust
const IXANY: u32 = 2_048u32;
```

### `OPOST`

```rust
const OPOST: u32 = 1u32;
```

### `OCRNL`

```rust
const OCRNL: u32 = 8u32;
```

### `ONOCR`

```rust
const ONOCR: u32 = 16u32;
```

### `ONLRET`

```rust
const ONLRET: u32 = 32u32;
```

### `OFILL`

```rust
const OFILL: u32 = 64u32;
```

### `OFDEL`

```rust
const OFDEL: u32 = 128u32;
```

### `B0`

```rust
const B0: u32 = 0u32;
```

### `B50`

```rust
const B50: u32 = 1u32;
```

### `B75`

```rust
const B75: u32 = 2u32;
```

### `B110`

```rust
const B110: u32 = 3u32;
```

### `B134`

```rust
const B134: u32 = 4u32;
```

### `B150`

```rust
const B150: u32 = 5u32;
```

### `B200`

```rust
const B200: u32 = 6u32;
```

### `B300`

```rust
const B300: u32 = 7u32;
```

### `B600`

```rust
const B600: u32 = 8u32;
```

### `B1200`

```rust
const B1200: u32 = 9u32;
```

### `B1800`

```rust
const B1800: u32 = 10u32;
```

### `B2400`

```rust
const B2400: u32 = 11u32;
```

### `B4800`

```rust
const B4800: u32 = 12u32;
```

### `B9600`

```rust
const B9600: u32 = 13u32;
```

### `B19200`

```rust
const B19200: u32 = 14u32;
```

### `B38400`

```rust
const B38400: u32 = 15u32;
```

### `EXTA`

```rust
const EXTA: u32 = 14u32;
```

### `EXTB`

```rust
const EXTB: u32 = 15u32;
```

### `ADDRB`

```rust
const ADDRB: u32 = 536_870_912u32;
```

### `CMSPAR`

```rust
const CMSPAR: u32 = 1_073_741_824u32;
```

### `CRTSCTS`

```rust
const CRTSCTS: u32 = 2_147_483_648u32;
```

### `IBSHIFT`

```rust
const IBSHIFT: u32 = 16u32;
```

### `TCOOFF`

```rust
const TCOOFF: u32 = 0u32;
```

### `TCOON`

```rust
const TCOON: u32 = 1u32;
```

### `TCIOFF`

```rust
const TCIOFF: u32 = 2u32;
```

### `TCION`

```rust
const TCION: u32 = 3u32;
```

### `TCIFLUSH`

```rust
const TCIFLUSH: u32 = 0u32;
```

### `TCOFLUSH`

```rust
const TCOFLUSH: u32 = 1u32;
```

### `TCIOFLUSH`

```rust
const TCIOFLUSH: u32 = 2u32;
```

### `NCCS`

```rust
const NCCS: u32 = 19u32;
```

### `VINTR`

```rust
const VINTR: u32 = 0u32;
```

### `VQUIT`

```rust
const VQUIT: u32 = 1u32;
```

### `VERASE`

```rust
const VERASE: u32 = 2u32;
```

### `VKILL`

```rust
const VKILL: u32 = 3u32;
```

### `VEOF`

```rust
const VEOF: u32 = 4u32;
```

### `VTIME`

```rust
const VTIME: u32 = 5u32;
```

### `VMIN`

```rust
const VMIN: u32 = 6u32;
```

### `VSWTC`

```rust
const VSWTC: u32 = 7u32;
```

### `VSTART`

```rust
const VSTART: u32 = 8u32;
```

### `VSTOP`

```rust
const VSTOP: u32 = 9u32;
```

### `VSUSP`

```rust
const VSUSP: u32 = 10u32;
```

### `VEOL`

```rust
const VEOL: u32 = 11u32;
```

### `VREPRINT`

```rust
const VREPRINT: u32 = 12u32;
```

### `VDISCARD`

```rust
const VDISCARD: u32 = 13u32;
```

### `VWERASE`

```rust
const VWERASE: u32 = 14u32;
```

### `VLNEXT`

```rust
const VLNEXT: u32 = 15u32;
```

### `VEOL2`

```rust
const VEOL2: u32 = 16u32;
```

### `IUCLC`

```rust
const IUCLC: u32 = 512u32;
```

### `IXON`

```rust
const IXON: u32 = 1_024u32;
```

### `IXOFF`

```rust
const IXOFF: u32 = 4_096u32;
```

### `IMAXBEL`

```rust
const IMAXBEL: u32 = 8_192u32;
```

### `IUTF8`

```rust
const IUTF8: u32 = 16_384u32;
```

### `OLCUC`

```rust
const OLCUC: u32 = 2u32;
```

### `ONLCR`

```rust
const ONLCR: u32 = 4u32;
```

### `NLDLY`

```rust
const NLDLY: u32 = 256u32;
```

### `NL0`

```rust
const NL0: u32 = 0u32;
```

### `NL1`

```rust
const NL1: u32 = 256u32;
```

### `CRDLY`

```rust
const CRDLY: u32 = 1_536u32;
```

### `CR0`

```rust
const CR0: u32 = 0u32;
```

### `CR1`

```rust
const CR1: u32 = 512u32;
```

### `CR2`

```rust
const CR2: u32 = 1_024u32;
```

### `CR3`

```rust
const CR3: u32 = 1_536u32;
```

### `TABDLY`

```rust
const TABDLY: u32 = 6_144u32;
```

### `TAB0`

```rust
const TAB0: u32 = 0u32;
```

### `TAB1`

```rust
const TAB1: u32 = 2_048u32;
```

### `TAB2`

```rust
const TAB2: u32 = 4_096u32;
```

### `TAB3`

```rust
const TAB3: u32 = 6_144u32;
```

### `XTABS`

```rust
const XTABS: u32 = 6_144u32;
```

### `BSDLY`

```rust
const BSDLY: u32 = 8_192u32;
```

### `BS0`

```rust
const BS0: u32 = 0u32;
```

### `BS1`

```rust
const BS1: u32 = 8_192u32;
```

### `VTDLY`

```rust
const VTDLY: u32 = 16_384u32;
```

### `VT0`

```rust
const VT0: u32 = 0u32;
```

### `VT1`

```rust
const VT1: u32 = 16_384u32;
```

### `FFDLY`

```rust
const FFDLY: u32 = 32_768u32;
```

### `FF0`

```rust
const FF0: u32 = 0u32;
```

### `FF1`

```rust
const FF1: u32 = 32_768u32;
```

### `CBAUD`

```rust
const CBAUD: u32 = 4_111u32;
```

### `CSIZE`

```rust
const CSIZE: u32 = 48u32;
```

### `CS5`

```rust
const CS5: u32 = 0u32;
```

### `CS6`

```rust
const CS6: u32 = 16u32;
```

### `CS7`

```rust
const CS7: u32 = 32u32;
```

### `CS8`

```rust
const CS8: u32 = 48u32;
```

### `CSTOPB`

```rust
const CSTOPB: u32 = 64u32;
```

### `CREAD`

```rust
const CREAD: u32 = 128u32;
```

### `PARENB`

```rust
const PARENB: u32 = 256u32;
```

### `PARODD`

```rust
const PARODD: u32 = 512u32;
```

### `HUPCL`

```rust
const HUPCL: u32 = 1_024u32;
```

### `CLOCAL`

```rust
const CLOCAL: u32 = 2_048u32;
```

### `CBAUDEX`

```rust
const CBAUDEX: u32 = 4_096u32;
```

### `BOTHER`

```rust
const BOTHER: u32 = 4_096u32;
```

### `B57600`

```rust
const B57600: u32 = 4_097u32;
```

### `B115200`

```rust
const B115200: u32 = 4_098u32;
```

### `B230400`

```rust
const B230400: u32 = 4_099u32;
```

### `B460800`

```rust
const B460800: u32 = 4_100u32;
```

### `B500000`

```rust
const B500000: u32 = 4_101u32;
```

### `B576000`

```rust
const B576000: u32 = 4_102u32;
```

### `B921600`

```rust
const B921600: u32 = 4_103u32;
```

### `B1000000`

```rust
const B1000000: u32 = 4_104u32;
```

### `B1152000`

```rust
const B1152000: u32 = 4_105u32;
```

### `B1500000`

```rust
const B1500000: u32 = 4_106u32;
```

### `B2000000`

```rust
const B2000000: u32 = 4_107u32;
```

### `B2500000`

```rust
const B2500000: u32 = 4_108u32;
```

### `B3000000`

```rust
const B3000000: u32 = 4_109u32;
```

### `B3500000`

```rust
const B3500000: u32 = 4_110u32;
```

### `B4000000`

```rust
const B4000000: u32 = 4_111u32;
```

### `CIBAUD`

```rust
const CIBAUD: u32 = 269_418_496u32;
```

### `ISIG`

```rust
const ISIG: u32 = 1u32;
```

### `ICANON`

```rust
const ICANON: u32 = 2u32;
```

### `XCASE`

```rust
const XCASE: u32 = 4u32;
```

### `ECHO`

```rust
const ECHO: u32 = 8u32;
```

### `ECHOE`

```rust
const ECHOE: u32 = 16u32;
```

### `ECHOK`

```rust
const ECHOK: u32 = 32u32;
```

### `ECHONL`

```rust
const ECHONL: u32 = 64u32;
```

### `NOFLSH`

```rust
const NOFLSH: u32 = 128u32;
```

### `TOSTOP`

```rust
const TOSTOP: u32 = 256u32;
```

### `ECHOCTL`

```rust
const ECHOCTL: u32 = 512u32;
```

### `ECHOPRT`

```rust
const ECHOPRT: u32 = 1_024u32;
```

### `ECHOKE`

```rust
const ECHOKE: u32 = 2_048u32;
```

### `FLUSHO`

```rust
const FLUSHO: u32 = 4_096u32;
```

### `PENDIN`

```rust
const PENDIN: u32 = 16_384u32;
```

### `IEXTEN`

```rust
const IEXTEN: u32 = 32_768u32;
```

### `EXTPROC`

```rust
const EXTPROC: u32 = 65_536u32;
```

### `TCSANOW`

```rust
const TCSANOW: u32 = 0u32;
```

### `TCSADRAIN`

```rust
const TCSADRAIN: u32 = 1u32;
```

### `TCSAFLUSH`

```rust
const TCSAFLUSH: u32 = 2u32;
```

### `TIOCPKT_DATA`

```rust
const TIOCPKT_DATA: u32 = 0u32;
```

### `TIOCPKT_FLUSHREAD`

```rust
const TIOCPKT_FLUSHREAD: u32 = 1u32;
```

### `TIOCPKT_FLUSHWRITE`

```rust
const TIOCPKT_FLUSHWRITE: u32 = 2u32;
```

### `TIOCPKT_STOP`

```rust
const TIOCPKT_STOP: u32 = 4u32;
```

### `TIOCPKT_START`

```rust
const TIOCPKT_START: u32 = 8u32;
```

### `TIOCPKT_NOSTOP`

```rust
const TIOCPKT_NOSTOP: u32 = 16u32;
```

### `TIOCPKT_DOSTOP`

```rust
const TIOCPKT_DOSTOP: u32 = 32u32;
```

### `TIOCPKT_IOCTL`

```rust
const TIOCPKT_IOCTL: u32 = 64u32;
```

### `TIOCSER_TEMT`

```rust
const TIOCSER_TEMT: u32 = 1u32;
```

### `NCC`

```rust
const NCC: u32 = 8u32;
```

### `TIOCM_LE`

```rust
const TIOCM_LE: u32 = 1u32;
```

### `TIOCM_DTR`

```rust
const TIOCM_DTR: u32 = 2u32;
```

### `TIOCM_RTS`

```rust
const TIOCM_RTS: u32 = 4u32;
```

### `TIOCM_ST`

```rust
const TIOCM_ST: u32 = 8u32;
```

### `TIOCM_SR`

```rust
const TIOCM_SR: u32 = 16u32;
```

### `TIOCM_CTS`

```rust
const TIOCM_CTS: u32 = 32u32;
```

### `TIOCM_CAR`

```rust
const TIOCM_CAR: u32 = 64u32;
```

### `TIOCM_RNG`

```rust
const TIOCM_RNG: u32 = 128u32;
```

### `TIOCM_DSR`

```rust
const TIOCM_DSR: u32 = 256u32;
```

### `TIOCM_CD`

```rust
const TIOCM_CD: u32 = 64u32;
```

### `TIOCM_RI`

```rust
const TIOCM_RI: u32 = 128u32;
```

### `TIOCM_OUT1`

```rust
const TIOCM_OUT1: u32 = 8_192u32;
```

### `TIOCM_OUT2`

```rust
const TIOCM_OUT2: u32 = 16_384u32;
```

### `TIOCM_LOOP`

```rust
const TIOCM_LOOP: u32 = 32_768u32;
```

### `ITIMER_REAL`

```rust
const ITIMER_REAL: u32 = 0u32;
```

### `ITIMER_VIRTUAL`

```rust
const ITIMER_VIRTUAL: u32 = 1u32;
```

### `ITIMER_PROF`

```rust
const ITIMER_PROF: u32 = 2u32;
```

### `CLOCK_REALTIME`

```rust
const CLOCK_REALTIME: u32 = 0u32;
```

### `CLOCK_MONOTONIC`

```rust
const CLOCK_MONOTONIC: u32 = 1u32;
```

### `CLOCK_PROCESS_CPUTIME_ID`

```rust
const CLOCK_PROCESS_CPUTIME_ID: u32 = 2u32;
```

### `CLOCK_THREAD_CPUTIME_ID`

```rust
const CLOCK_THREAD_CPUTIME_ID: u32 = 3u32;
```

### `CLOCK_MONOTONIC_RAW`

```rust
const CLOCK_MONOTONIC_RAW: u32 = 4u32;
```

### `CLOCK_REALTIME_COARSE`

```rust
const CLOCK_REALTIME_COARSE: u32 = 5u32;
```

### `CLOCK_MONOTONIC_COARSE`

```rust
const CLOCK_MONOTONIC_COARSE: u32 = 6u32;
```

### `CLOCK_BOOTTIME`

```rust
const CLOCK_BOOTTIME: u32 = 7u32;
```

### `CLOCK_REALTIME_ALARM`

```rust
const CLOCK_REALTIME_ALARM: u32 = 8u32;
```

### `CLOCK_BOOTTIME_ALARM`

```rust
const CLOCK_BOOTTIME_ALARM: u32 = 9u32;
```

### `CLOCK_SGI_CYCLE`

```rust
const CLOCK_SGI_CYCLE: u32 = 10u32;
```

### `CLOCK_TAI`

```rust
const CLOCK_TAI: u32 = 11u32;
```

### `MAX_CLOCKS`

```rust
const MAX_CLOCKS: u32 = 16u32;
```

### `CLOCKS_MASK`

```rust
const CLOCKS_MASK: u32 = 1u32;
```

### `CLOCKS_MONO`

```rust
const CLOCKS_MONO: u32 = 1u32;
```

### `TIMER_ABSTIME`

```rust
const TIMER_ABSTIME: u32 = 1u32;
```

### `UIO_FASTIOV`

```rust
const UIO_FASTIOV: u32 = 8u32;
```

### `UIO_MAXIOV`

```rust
const UIO_MAXIOV: u32 = 1_024u32;
```

### `__X32_SYSCALL_BIT`

```rust
const __X32_SYSCALL_BIT: u32 = 1_073_741_824u32;
```

### `__NR_read`

```rust
const __NR_read: u32 = 0u32;
```

### `__NR_write`

```rust
const __NR_write: u32 = 1u32;
```

### `__NR_open`

```rust
const __NR_open: u32 = 2u32;
```

### `__NR_close`

```rust
const __NR_close: u32 = 3u32;
```

### `__NR_stat`

```rust
const __NR_stat: u32 = 4u32;
```

### `__NR_fstat`

```rust
const __NR_fstat: u32 = 5u32;
```

### `__NR_lstat`

```rust
const __NR_lstat: u32 = 6u32;
```

### `__NR_poll`

```rust
const __NR_poll: u32 = 7u32;
```

### `__NR_lseek`

```rust
const __NR_lseek: u32 = 8u32;
```

### `__NR_mmap`

```rust
const __NR_mmap: u32 = 9u32;
```

### `__NR_mprotect`

```rust
const __NR_mprotect: u32 = 10u32;
```

### `__NR_munmap`

```rust
const __NR_munmap: u32 = 11u32;
```

### `__NR_brk`

```rust
const __NR_brk: u32 = 12u32;
```

### `__NR_rt_sigaction`

```rust
const __NR_rt_sigaction: u32 = 13u32;
```

### `__NR_rt_sigprocmask`

```rust
const __NR_rt_sigprocmask: u32 = 14u32;
```

### `__NR_rt_sigreturn`

```rust
const __NR_rt_sigreturn: u32 = 15u32;
```

### `__NR_ioctl`

```rust
const __NR_ioctl: u32 = 16u32;
```

### `__NR_pread64`

```rust
const __NR_pread64: u32 = 17u32;
```

### `__NR_pwrite64`

```rust
const __NR_pwrite64: u32 = 18u32;
```

### `__NR_readv`

```rust
const __NR_readv: u32 = 19u32;
```

### `__NR_writev`

```rust
const __NR_writev: u32 = 20u32;
```

### `__NR_access`

```rust
const __NR_access: u32 = 21u32;
```

### `__NR_pipe`

```rust
const __NR_pipe: u32 = 22u32;
```

### `__NR_select`

```rust
const __NR_select: u32 = 23u32;
```

### `__NR_sched_yield`

```rust
const __NR_sched_yield: u32 = 24u32;
```

### `__NR_mremap`

```rust
const __NR_mremap: u32 = 25u32;
```

### `__NR_msync`

```rust
const __NR_msync: u32 = 26u32;
```

### `__NR_mincore`

```rust
const __NR_mincore: u32 = 27u32;
```

### `__NR_madvise`

```rust
const __NR_madvise: u32 = 28u32;
```

### `__NR_shmget`

```rust
const __NR_shmget: u32 = 29u32;
```

### `__NR_shmat`

```rust
const __NR_shmat: u32 = 30u32;
```

### `__NR_shmctl`

```rust
const __NR_shmctl: u32 = 31u32;
```

### `__NR_dup`

```rust
const __NR_dup: u32 = 32u32;
```

### `__NR_dup2`

```rust
const __NR_dup2: u32 = 33u32;
```

### `__NR_pause`

```rust
const __NR_pause: u32 = 34u32;
```

### `__NR_nanosleep`

```rust
const __NR_nanosleep: u32 = 35u32;
```

### `__NR_getitimer`

```rust
const __NR_getitimer: u32 = 36u32;
```

### `__NR_alarm`

```rust
const __NR_alarm: u32 = 37u32;
```

### `__NR_setitimer`

```rust
const __NR_setitimer: u32 = 38u32;
```

### `__NR_getpid`

```rust
const __NR_getpid: u32 = 39u32;
```

### `__NR_sendfile`

```rust
const __NR_sendfile: u32 = 40u32;
```

### `__NR_socket`

```rust
const __NR_socket: u32 = 41u32;
```

### `__NR_connect`

```rust
const __NR_connect: u32 = 42u32;
```

### `__NR_accept`

```rust
const __NR_accept: u32 = 43u32;
```

### `__NR_sendto`

```rust
const __NR_sendto: u32 = 44u32;
```

### `__NR_recvfrom`

```rust
const __NR_recvfrom: u32 = 45u32;
```

### `__NR_sendmsg`

```rust
const __NR_sendmsg: u32 = 46u32;
```

### `__NR_recvmsg`

```rust
const __NR_recvmsg: u32 = 47u32;
```

### `__NR_shutdown`

```rust
const __NR_shutdown: u32 = 48u32;
```

### `__NR_bind`

```rust
const __NR_bind: u32 = 49u32;
```

### `__NR_listen`

```rust
const __NR_listen: u32 = 50u32;
```

### `__NR_getsockname`

```rust
const __NR_getsockname: u32 = 51u32;
```

### `__NR_getpeername`

```rust
const __NR_getpeername: u32 = 52u32;
```

### `__NR_socketpair`

```rust
const __NR_socketpair: u32 = 53u32;
```

### `__NR_setsockopt`

```rust
const __NR_setsockopt: u32 = 54u32;
```

### `__NR_getsockopt`

```rust
const __NR_getsockopt: u32 = 55u32;
```

### `__NR_clone`

```rust
const __NR_clone: u32 = 56u32;
```

### `__NR_fork`

```rust
const __NR_fork: u32 = 57u32;
```

### `__NR_vfork`

```rust
const __NR_vfork: u32 = 58u32;
```

### `__NR_execve`

```rust
const __NR_execve: u32 = 59u32;
```

### `__NR_exit`

```rust
const __NR_exit: u32 = 60u32;
```

### `__NR_wait4`

```rust
const __NR_wait4: u32 = 61u32;
```

### `__NR_kill`

```rust
const __NR_kill: u32 = 62u32;
```

### `__NR_uname`

```rust
const __NR_uname: u32 = 63u32;
```

### `__NR_semget`

```rust
const __NR_semget: u32 = 64u32;
```

### `__NR_semop`

```rust
const __NR_semop: u32 = 65u32;
```

### `__NR_semctl`

```rust
const __NR_semctl: u32 = 66u32;
```

### `__NR_shmdt`

```rust
const __NR_shmdt: u32 = 67u32;
```

### `__NR_msgget`

```rust
const __NR_msgget: u32 = 68u32;
```

### `__NR_msgsnd`

```rust
const __NR_msgsnd: u32 = 69u32;
```

### `__NR_msgrcv`

```rust
const __NR_msgrcv: u32 = 70u32;
```

### `__NR_msgctl`

```rust
const __NR_msgctl: u32 = 71u32;
```

### `__NR_fcntl`

```rust
const __NR_fcntl: u32 = 72u32;
```

### `__NR_flock`

```rust
const __NR_flock: u32 = 73u32;
```

### `__NR_fsync`

```rust
const __NR_fsync: u32 = 74u32;
```

### `__NR_fdatasync`

```rust
const __NR_fdatasync: u32 = 75u32;
```

### `__NR_truncate`

```rust
const __NR_truncate: u32 = 76u32;
```

### `__NR_ftruncate`

```rust
const __NR_ftruncate: u32 = 77u32;
```

### `__NR_getdents`

```rust
const __NR_getdents: u32 = 78u32;
```

### `__NR_getcwd`

```rust
const __NR_getcwd: u32 = 79u32;
```

### `__NR_chdir`

```rust
const __NR_chdir: u32 = 80u32;
```

### `__NR_fchdir`

```rust
const __NR_fchdir: u32 = 81u32;
```

### `__NR_rename`

```rust
const __NR_rename: u32 = 82u32;
```

### `__NR_mkdir`

```rust
const __NR_mkdir: u32 = 83u32;
```

### `__NR_rmdir`

```rust
const __NR_rmdir: u32 = 84u32;
```

### `__NR_creat`

```rust
const __NR_creat: u32 = 85u32;
```

### `__NR_link`

```rust
const __NR_link: u32 = 86u32;
```

### `__NR_unlink`

```rust
const __NR_unlink: u32 = 87u32;
```

### `__NR_symlink`

```rust
const __NR_symlink: u32 = 88u32;
```

### `__NR_readlink`

```rust
const __NR_readlink: u32 = 89u32;
```

### `__NR_chmod`

```rust
const __NR_chmod: u32 = 90u32;
```

### `__NR_fchmod`

```rust
const __NR_fchmod: u32 = 91u32;
```

### `__NR_chown`

```rust
const __NR_chown: u32 = 92u32;
```

### `__NR_fchown`

```rust
const __NR_fchown: u32 = 93u32;
```

### `__NR_lchown`

```rust
const __NR_lchown: u32 = 94u32;
```

### `__NR_umask`

```rust
const __NR_umask: u32 = 95u32;
```

### `__NR_gettimeofday`

```rust
const __NR_gettimeofday: u32 = 96u32;
```

### `__NR_getrlimit`

```rust
const __NR_getrlimit: u32 = 97u32;
```

### `__NR_getrusage`

```rust
const __NR_getrusage: u32 = 98u32;
```

### `__NR_sysinfo`

```rust
const __NR_sysinfo: u32 = 99u32;
```

### `__NR_times`

```rust
const __NR_times: u32 = 100u32;
```

### `__NR_ptrace`

```rust
const __NR_ptrace: u32 = 101u32;
```

### `__NR_getuid`

```rust
const __NR_getuid: u32 = 102u32;
```

### `__NR_syslog`

```rust
const __NR_syslog: u32 = 103u32;
```

### `__NR_getgid`

```rust
const __NR_getgid: u32 = 104u32;
```

### `__NR_setuid`

```rust
const __NR_setuid: u32 = 105u32;
```

### `__NR_setgid`

```rust
const __NR_setgid: u32 = 106u32;
```

### `__NR_geteuid`

```rust
const __NR_geteuid: u32 = 107u32;
```

### `__NR_getegid`

```rust
const __NR_getegid: u32 = 108u32;
```

### `__NR_setpgid`

```rust
const __NR_setpgid: u32 = 109u32;
```

### `__NR_getppid`

```rust
const __NR_getppid: u32 = 110u32;
```

### `__NR_getpgrp`

```rust
const __NR_getpgrp: u32 = 111u32;
```

### `__NR_setsid`

```rust
const __NR_setsid: u32 = 112u32;
```

### `__NR_setreuid`

```rust
const __NR_setreuid: u32 = 113u32;
```

### `__NR_setregid`

```rust
const __NR_setregid: u32 = 114u32;
```

### `__NR_getgroups`

```rust
const __NR_getgroups: u32 = 115u32;
```

### `__NR_setgroups`

```rust
const __NR_setgroups: u32 = 116u32;
```

### `__NR_setresuid`

```rust
const __NR_setresuid: u32 = 117u32;
```

### `__NR_getresuid`

```rust
const __NR_getresuid: u32 = 118u32;
```

### `__NR_setresgid`

```rust
const __NR_setresgid: u32 = 119u32;
```

### `__NR_getresgid`

```rust
const __NR_getresgid: u32 = 120u32;
```

### `__NR_getpgid`

```rust
const __NR_getpgid: u32 = 121u32;
```

### `__NR_setfsuid`

```rust
const __NR_setfsuid: u32 = 122u32;
```

### `__NR_setfsgid`

```rust
const __NR_setfsgid: u32 = 123u32;
```

### `__NR_getsid`

```rust
const __NR_getsid: u32 = 124u32;
```

### `__NR_capget`

```rust
const __NR_capget: u32 = 125u32;
```

### `__NR_capset`

```rust
const __NR_capset: u32 = 126u32;
```

### `__NR_rt_sigpending`

```rust
const __NR_rt_sigpending: u32 = 127u32;
```

### `__NR_rt_sigtimedwait`

```rust
const __NR_rt_sigtimedwait: u32 = 128u32;
```

### `__NR_rt_sigqueueinfo`

```rust
const __NR_rt_sigqueueinfo: u32 = 129u32;
```

### `__NR_rt_sigsuspend`

```rust
const __NR_rt_sigsuspend: u32 = 130u32;
```

### `__NR_sigaltstack`

```rust
const __NR_sigaltstack: u32 = 131u32;
```

### `__NR_utime`

```rust
const __NR_utime: u32 = 132u32;
```

### `__NR_mknod`

```rust
const __NR_mknod: u32 = 133u32;
```

### `__NR_uselib`

```rust
const __NR_uselib: u32 = 134u32;
```

### `__NR_personality`

```rust
const __NR_personality: u32 = 135u32;
```

### `__NR_ustat`

```rust
const __NR_ustat: u32 = 136u32;
```

### `__NR_statfs`

```rust
const __NR_statfs: u32 = 137u32;
```

### `__NR_fstatfs`

```rust
const __NR_fstatfs: u32 = 138u32;
```

### `__NR_sysfs`

```rust
const __NR_sysfs: u32 = 139u32;
```

### `__NR_getpriority`

```rust
const __NR_getpriority: u32 = 140u32;
```

### `__NR_setpriority`

```rust
const __NR_setpriority: u32 = 141u32;
```

### `__NR_sched_setparam`

```rust
const __NR_sched_setparam: u32 = 142u32;
```

### `__NR_sched_getparam`

```rust
const __NR_sched_getparam: u32 = 143u32;
```

### `__NR_sched_setscheduler`

```rust
const __NR_sched_setscheduler: u32 = 144u32;
```

### `__NR_sched_getscheduler`

```rust
const __NR_sched_getscheduler: u32 = 145u32;
```

### `__NR_sched_get_priority_max`

```rust
const __NR_sched_get_priority_max: u32 = 146u32;
```

### `__NR_sched_get_priority_min`

```rust
const __NR_sched_get_priority_min: u32 = 147u32;
```

### `__NR_sched_rr_get_interval`

```rust
const __NR_sched_rr_get_interval: u32 = 148u32;
```

### `__NR_mlock`

```rust
const __NR_mlock: u32 = 149u32;
```

### `__NR_munlock`

```rust
const __NR_munlock: u32 = 150u32;
```

### `__NR_mlockall`

```rust
const __NR_mlockall: u32 = 151u32;
```

### `__NR_munlockall`

```rust
const __NR_munlockall: u32 = 152u32;
```

### `__NR_vhangup`

```rust
const __NR_vhangup: u32 = 153u32;
```

### `__NR_modify_ldt`

```rust
const __NR_modify_ldt: u32 = 154u32;
```

### `__NR_pivot_root`

```rust
const __NR_pivot_root: u32 = 155u32;
```

### `__NR__sysctl`

```rust
const __NR__sysctl: u32 = 156u32;
```

### `__NR_prctl`

```rust
const __NR_prctl: u32 = 157u32;
```

### `__NR_arch_prctl`

```rust
const __NR_arch_prctl: u32 = 158u32;
```

### `__NR_adjtimex`

```rust
const __NR_adjtimex: u32 = 159u32;
```

### `__NR_setrlimit`

```rust
const __NR_setrlimit: u32 = 160u32;
```

### `__NR_chroot`

```rust
const __NR_chroot: u32 = 161u32;
```

### `__NR_sync`

```rust
const __NR_sync: u32 = 162u32;
```

### `__NR_acct`

```rust
const __NR_acct: u32 = 163u32;
```

### `__NR_settimeofday`

```rust
const __NR_settimeofday: u32 = 164u32;
```

### `__NR_mount`

```rust
const __NR_mount: u32 = 165u32;
```

### `__NR_umount2`

```rust
const __NR_umount2: u32 = 166u32;
```

### `__NR_swapon`

```rust
const __NR_swapon: u32 = 167u32;
```

### `__NR_swapoff`

```rust
const __NR_swapoff: u32 = 168u32;
```

### `__NR_reboot`

```rust
const __NR_reboot: u32 = 169u32;
```

### `__NR_sethostname`

```rust
const __NR_sethostname: u32 = 170u32;
```

### `__NR_setdomainname`

```rust
const __NR_setdomainname: u32 = 171u32;
```

### `__NR_iopl`

```rust
const __NR_iopl: u32 = 172u32;
```

### `__NR_ioperm`

```rust
const __NR_ioperm: u32 = 173u32;
```

### `__NR_create_module`

```rust
const __NR_create_module: u32 = 174u32;
```

### `__NR_init_module`

```rust
const __NR_init_module: u32 = 175u32;
```

### `__NR_delete_module`

```rust
const __NR_delete_module: u32 = 176u32;
```

### `__NR_get_kernel_syms`

```rust
const __NR_get_kernel_syms: u32 = 177u32;
```

### `__NR_query_module`

```rust
const __NR_query_module: u32 = 178u32;
```

### `__NR_quotactl`

```rust
const __NR_quotactl: u32 = 179u32;
```

### `__NR_nfsservctl`

```rust
const __NR_nfsservctl: u32 = 180u32;
```

### `__NR_getpmsg`

```rust
const __NR_getpmsg: u32 = 181u32;
```

### `__NR_putpmsg`

```rust
const __NR_putpmsg: u32 = 182u32;
```

### `__NR_afs_syscall`

```rust
const __NR_afs_syscall: u32 = 183u32;
```

### `__NR_tuxcall`

```rust
const __NR_tuxcall: u32 = 184u32;
```

### `__NR_security`

```rust
const __NR_security: u32 = 185u32;
```

### `__NR_gettid`

```rust
const __NR_gettid: u32 = 186u32;
```

### `__NR_readahead`

```rust
const __NR_readahead: u32 = 187u32;
```

### `__NR_setxattr`

```rust
const __NR_setxattr: u32 = 188u32;
```

### `__NR_lsetxattr`

```rust
const __NR_lsetxattr: u32 = 189u32;
```

### `__NR_fsetxattr`

```rust
const __NR_fsetxattr: u32 = 190u32;
```

### `__NR_getxattr`

```rust
const __NR_getxattr: u32 = 191u32;
```

### `__NR_lgetxattr`

```rust
const __NR_lgetxattr: u32 = 192u32;
```

### `__NR_fgetxattr`

```rust
const __NR_fgetxattr: u32 = 193u32;
```

### `__NR_listxattr`

```rust
const __NR_listxattr: u32 = 194u32;
```

### `__NR_llistxattr`

```rust
const __NR_llistxattr: u32 = 195u32;
```

### `__NR_flistxattr`

```rust
const __NR_flistxattr: u32 = 196u32;
```

### `__NR_removexattr`

```rust
const __NR_removexattr: u32 = 197u32;
```

### `__NR_lremovexattr`

```rust
const __NR_lremovexattr: u32 = 198u32;
```

### `__NR_fremovexattr`

```rust
const __NR_fremovexattr: u32 = 199u32;
```

### `__NR_tkill`

```rust
const __NR_tkill: u32 = 200u32;
```

### `__NR_time`

```rust
const __NR_time: u32 = 201u32;
```

### `__NR_futex`

```rust
const __NR_futex: u32 = 202u32;
```

### `__NR_sched_setaffinity`

```rust
const __NR_sched_setaffinity: u32 = 203u32;
```

### `__NR_sched_getaffinity`

```rust
const __NR_sched_getaffinity: u32 = 204u32;
```

### `__NR_set_thread_area`

```rust
const __NR_set_thread_area: u32 = 205u32;
```

### `__NR_io_setup`

```rust
const __NR_io_setup: u32 = 206u32;
```

### `__NR_io_destroy`

```rust
const __NR_io_destroy: u32 = 207u32;
```

### `__NR_io_getevents`

```rust
const __NR_io_getevents: u32 = 208u32;
```

### `__NR_io_submit`

```rust
const __NR_io_submit: u32 = 209u32;
```

### `__NR_io_cancel`

```rust
const __NR_io_cancel: u32 = 210u32;
```

### `__NR_get_thread_area`

```rust
const __NR_get_thread_area: u32 = 211u32;
```

### `__NR_lookup_dcookie`

```rust
const __NR_lookup_dcookie: u32 = 212u32;
```

### `__NR_epoll_create`

```rust
const __NR_epoll_create: u32 = 213u32;
```

### `__NR_epoll_ctl_old`

```rust
const __NR_epoll_ctl_old: u32 = 214u32;
```

### `__NR_epoll_wait_old`

```rust
const __NR_epoll_wait_old: u32 = 215u32;
```

### `__NR_remap_file_pages`

```rust
const __NR_remap_file_pages: u32 = 216u32;
```

### `__NR_getdents64`

```rust
const __NR_getdents64: u32 = 217u32;
```

### `__NR_set_tid_address`

```rust
const __NR_set_tid_address: u32 = 218u32;
```

### `__NR_restart_syscall`

```rust
const __NR_restart_syscall: u32 = 219u32;
```

### `__NR_semtimedop`

```rust
const __NR_semtimedop: u32 = 220u32;
```

### `__NR_fadvise64`

```rust
const __NR_fadvise64: u32 = 221u32;
```

### `__NR_timer_create`

```rust
const __NR_timer_create: u32 = 222u32;
```

### `__NR_timer_settime`

```rust
const __NR_timer_settime: u32 = 223u32;
```

### `__NR_timer_gettime`

```rust
const __NR_timer_gettime: u32 = 224u32;
```

### `__NR_timer_getoverrun`

```rust
const __NR_timer_getoverrun: u32 = 225u32;
```

### `__NR_timer_delete`

```rust
const __NR_timer_delete: u32 = 226u32;
```

### `__NR_clock_settime`

```rust
const __NR_clock_settime: u32 = 227u32;
```

### `__NR_clock_gettime`

```rust
const __NR_clock_gettime: u32 = 228u32;
```

### `__NR_clock_getres`

```rust
const __NR_clock_getres: u32 = 229u32;
```

### `__NR_clock_nanosleep`

```rust
const __NR_clock_nanosleep: u32 = 230u32;
```

### `__NR_exit_group`

```rust
const __NR_exit_group: u32 = 231u32;
```

### `__NR_epoll_wait`

```rust
const __NR_epoll_wait: u32 = 232u32;
```

### `__NR_epoll_ctl`

```rust
const __NR_epoll_ctl: u32 = 233u32;
```

### `__NR_tgkill`

```rust
const __NR_tgkill: u32 = 234u32;
```

### `__NR_utimes`

```rust
const __NR_utimes: u32 = 235u32;
```

### `__NR_vserver`

```rust
const __NR_vserver: u32 = 236u32;
```

### `__NR_mbind`

```rust
const __NR_mbind: u32 = 237u32;
```

### `__NR_set_mempolicy`

```rust
const __NR_set_mempolicy: u32 = 238u32;
```

### `__NR_get_mempolicy`

```rust
const __NR_get_mempolicy: u32 = 239u32;
```

### `__NR_mq_open`

```rust
const __NR_mq_open: u32 = 240u32;
```

### `__NR_mq_unlink`

```rust
const __NR_mq_unlink: u32 = 241u32;
```

### `__NR_mq_timedsend`

```rust
const __NR_mq_timedsend: u32 = 242u32;
```

### `__NR_mq_timedreceive`

```rust
const __NR_mq_timedreceive: u32 = 243u32;
```

### `__NR_mq_notify`

```rust
const __NR_mq_notify: u32 = 244u32;
```

### `__NR_mq_getsetattr`

```rust
const __NR_mq_getsetattr: u32 = 245u32;
```

### `__NR_kexec_load`

```rust
const __NR_kexec_load: u32 = 246u32;
```

### `__NR_waitid`

```rust
const __NR_waitid: u32 = 247u32;
```

### `__NR_add_key`

```rust
const __NR_add_key: u32 = 248u32;
```

### `__NR_request_key`

```rust
const __NR_request_key: u32 = 249u32;
```

### `__NR_keyctl`

```rust
const __NR_keyctl: u32 = 250u32;
```

### `__NR_ioprio_set`

```rust
const __NR_ioprio_set: u32 = 251u32;
```

### `__NR_ioprio_get`

```rust
const __NR_ioprio_get: u32 = 252u32;
```

### `__NR_inotify_init`

```rust
const __NR_inotify_init: u32 = 253u32;
```

### `__NR_inotify_add_watch`

```rust
const __NR_inotify_add_watch: u32 = 254u32;
```

### `__NR_inotify_rm_watch`

```rust
const __NR_inotify_rm_watch: u32 = 255u32;
```

### `__NR_migrate_pages`

```rust
const __NR_migrate_pages: u32 = 256u32;
```

### `__NR_openat`

```rust
const __NR_openat: u32 = 257u32;
```

### `__NR_mkdirat`

```rust
const __NR_mkdirat: u32 = 258u32;
```

### `__NR_mknodat`

```rust
const __NR_mknodat: u32 = 259u32;
```

### `__NR_fchownat`

```rust
const __NR_fchownat: u32 = 260u32;
```

### `__NR_futimesat`

```rust
const __NR_futimesat: u32 = 261u32;
```

### `__NR_newfstatat`

```rust
const __NR_newfstatat: u32 = 262u32;
```

### `__NR_unlinkat`

```rust
const __NR_unlinkat: u32 = 263u32;
```

### `__NR_renameat`

```rust
const __NR_renameat: u32 = 264u32;
```

### `__NR_linkat`

```rust
const __NR_linkat: u32 = 265u32;
```

### `__NR_symlinkat`

```rust
const __NR_symlinkat: u32 = 266u32;
```

### `__NR_readlinkat`

```rust
const __NR_readlinkat: u32 = 267u32;
```

### `__NR_fchmodat`

```rust
const __NR_fchmodat: u32 = 268u32;
```

### `__NR_faccessat`

```rust
const __NR_faccessat: u32 = 269u32;
```

### `__NR_pselect6`

```rust
const __NR_pselect6: u32 = 270u32;
```

### `__NR_ppoll`

```rust
const __NR_ppoll: u32 = 271u32;
```

### `__NR_unshare`

```rust
const __NR_unshare: u32 = 272u32;
```

### `__NR_set_robust_list`

```rust
const __NR_set_robust_list: u32 = 273u32;
```

### `__NR_get_robust_list`

```rust
const __NR_get_robust_list: u32 = 274u32;
```

### `__NR_splice`

```rust
const __NR_splice: u32 = 275u32;
```

### `__NR_tee`

```rust
const __NR_tee: u32 = 276u32;
```

### `__NR_sync_file_range`

```rust
const __NR_sync_file_range: u32 = 277u32;
```

### `__NR_vmsplice`

```rust
const __NR_vmsplice: u32 = 278u32;
```

### `__NR_move_pages`

```rust
const __NR_move_pages: u32 = 279u32;
```

### `__NR_utimensat`

```rust
const __NR_utimensat: u32 = 280u32;
```

### `__NR_epoll_pwait`

```rust
const __NR_epoll_pwait: u32 = 281u32;
```

### `__NR_signalfd`

```rust
const __NR_signalfd: u32 = 282u32;
```

### `__NR_timerfd_create`

```rust
const __NR_timerfd_create: u32 = 283u32;
```

### `__NR_eventfd`

```rust
const __NR_eventfd: u32 = 284u32;
```

### `__NR_fallocate`

```rust
const __NR_fallocate: u32 = 285u32;
```

### `__NR_timerfd_settime`

```rust
const __NR_timerfd_settime: u32 = 286u32;
```

### `__NR_timerfd_gettime`

```rust
const __NR_timerfd_gettime: u32 = 287u32;
```

### `__NR_accept4`

```rust
const __NR_accept4: u32 = 288u32;
```

### `__NR_signalfd4`

```rust
const __NR_signalfd4: u32 = 289u32;
```

### `__NR_eventfd2`

```rust
const __NR_eventfd2: u32 = 290u32;
```

### `__NR_epoll_create1`

```rust
const __NR_epoll_create1: u32 = 291u32;
```

### `__NR_dup3`

```rust
const __NR_dup3: u32 = 292u32;
```

### `__NR_pipe2`

```rust
const __NR_pipe2: u32 = 293u32;
```

### `__NR_inotify_init1`

```rust
const __NR_inotify_init1: u32 = 294u32;
```

### `__NR_preadv`

```rust
const __NR_preadv: u32 = 295u32;
```

### `__NR_pwritev`

```rust
const __NR_pwritev: u32 = 296u32;
```

### `__NR_rt_tgsigqueueinfo`

```rust
const __NR_rt_tgsigqueueinfo: u32 = 297u32;
```

### `__NR_perf_event_open`

```rust
const __NR_perf_event_open: u32 = 298u32;
```

### `__NR_recvmmsg`

```rust
const __NR_recvmmsg: u32 = 299u32;
```

### `__NR_fanotify_init`

```rust
const __NR_fanotify_init: u32 = 300u32;
```

### `__NR_fanotify_mark`

```rust
const __NR_fanotify_mark: u32 = 301u32;
```

### `__NR_prlimit64`

```rust
const __NR_prlimit64: u32 = 302u32;
```

### `__NR_name_to_handle_at`

```rust
const __NR_name_to_handle_at: u32 = 303u32;
```

### `__NR_open_by_handle_at`

```rust
const __NR_open_by_handle_at: u32 = 304u32;
```

### `__NR_clock_adjtime`

```rust
const __NR_clock_adjtime: u32 = 305u32;
```

### `__NR_syncfs`

```rust
const __NR_syncfs: u32 = 306u32;
```

### `__NR_sendmmsg`

```rust
const __NR_sendmmsg: u32 = 307u32;
```

### `__NR_setns`

```rust
const __NR_setns: u32 = 308u32;
```

### `__NR_getcpu`

```rust
const __NR_getcpu: u32 = 309u32;
```

### `__NR_process_vm_readv`

```rust
const __NR_process_vm_readv: u32 = 310u32;
```

### `__NR_process_vm_writev`

```rust
const __NR_process_vm_writev: u32 = 311u32;
```

### `__NR_kcmp`

```rust
const __NR_kcmp: u32 = 312u32;
```

### `__NR_finit_module`

```rust
const __NR_finit_module: u32 = 313u32;
```

### `__NR_sched_setattr`

```rust
const __NR_sched_setattr: u32 = 314u32;
```

### `__NR_sched_getattr`

```rust
const __NR_sched_getattr: u32 = 315u32;
```

### `__NR_renameat2`

```rust
const __NR_renameat2: u32 = 316u32;
```

### `__NR_seccomp`

```rust
const __NR_seccomp: u32 = 317u32;
```

### `__NR_getrandom`

```rust
const __NR_getrandom: u32 = 318u32;
```

### `__NR_memfd_create`

```rust
const __NR_memfd_create: u32 = 319u32;
```

### `__NR_kexec_file_load`

```rust
const __NR_kexec_file_load: u32 = 320u32;
```

### `__NR_bpf`

```rust
const __NR_bpf: u32 = 321u32;
```

### `__NR_execveat`

```rust
const __NR_execveat: u32 = 322u32;
```

### `__NR_userfaultfd`

```rust
const __NR_userfaultfd: u32 = 323u32;
```

### `__NR_membarrier`

```rust
const __NR_membarrier: u32 = 324u32;
```

### `__NR_mlock2`

```rust
const __NR_mlock2: u32 = 325u32;
```

### `__NR_copy_file_range`

```rust
const __NR_copy_file_range: u32 = 326u32;
```

### `__NR_preadv2`

```rust
const __NR_preadv2: u32 = 327u32;
```

### `__NR_pwritev2`

```rust
const __NR_pwritev2: u32 = 328u32;
```

### `__NR_pkey_mprotect`

```rust
const __NR_pkey_mprotect: u32 = 329u32;
```

### `__NR_pkey_alloc`

```rust
const __NR_pkey_alloc: u32 = 330u32;
```

### `__NR_pkey_free`

```rust
const __NR_pkey_free: u32 = 331u32;
```

### `__NR_statx`

```rust
const __NR_statx: u32 = 332u32;
```

### `__NR_io_pgetevents`

```rust
const __NR_io_pgetevents: u32 = 333u32;
```

### `__NR_rseq`

```rust
const __NR_rseq: u32 = 334u32;
```

### `__NR_uretprobe`

```rust
const __NR_uretprobe: u32 = 335u32;
```

### `__NR_pidfd_send_signal`

```rust
const __NR_pidfd_send_signal: u32 = 424u32;
```

### `__NR_io_uring_setup`

```rust
const __NR_io_uring_setup: u32 = 425u32;
```

### `__NR_io_uring_enter`

```rust
const __NR_io_uring_enter: u32 = 426u32;
```

### `__NR_io_uring_register`

```rust
const __NR_io_uring_register: u32 = 427u32;
```

### `__NR_open_tree`

```rust
const __NR_open_tree: u32 = 428u32;
```

### `__NR_move_mount`

```rust
const __NR_move_mount: u32 = 429u32;
```

### `__NR_fsopen`

```rust
const __NR_fsopen: u32 = 430u32;
```

### `__NR_fsconfig`

```rust
const __NR_fsconfig: u32 = 431u32;
```

### `__NR_fsmount`

```rust
const __NR_fsmount: u32 = 432u32;
```

### `__NR_fspick`

```rust
const __NR_fspick: u32 = 433u32;
```

### `__NR_pidfd_open`

```rust
const __NR_pidfd_open: u32 = 434u32;
```

### `__NR_clone3`

```rust
const __NR_clone3: u32 = 435u32;
```

### `__NR_close_range`

```rust
const __NR_close_range: u32 = 436u32;
```

### `__NR_openat2`

```rust
const __NR_openat2: u32 = 437u32;
```

### `__NR_pidfd_getfd`

```rust
const __NR_pidfd_getfd: u32 = 438u32;
```

### `__NR_faccessat2`

```rust
const __NR_faccessat2: u32 = 439u32;
```

### `__NR_process_madvise`

```rust
const __NR_process_madvise: u32 = 440u32;
```

### `__NR_epoll_pwait2`

```rust
const __NR_epoll_pwait2: u32 = 441u32;
```

### `__NR_mount_setattr`

```rust
const __NR_mount_setattr: u32 = 442u32;
```

### `__NR_quotactl_fd`

```rust
const __NR_quotactl_fd: u32 = 443u32;
```

### `__NR_landlock_create_ruleset`

```rust
const __NR_landlock_create_ruleset: u32 = 444u32;
```

### `__NR_landlock_add_rule`

```rust
const __NR_landlock_add_rule: u32 = 445u32;
```

### `__NR_landlock_restrict_self`

```rust
const __NR_landlock_restrict_self: u32 = 446u32;
```

### `__NR_memfd_secret`

```rust
const __NR_memfd_secret: u32 = 447u32;
```

### `__NR_process_mrelease`

```rust
const __NR_process_mrelease: u32 = 448u32;
```

### `__NR_futex_waitv`

```rust
const __NR_futex_waitv: u32 = 449u32;
```

### `__NR_set_mempolicy_home_node`

```rust
const __NR_set_mempolicy_home_node: u32 = 450u32;
```

### `__NR_cachestat`

```rust
const __NR_cachestat: u32 = 451u32;
```

### `__NR_fchmodat2`

```rust
const __NR_fchmodat2: u32 = 452u32;
```

### `__NR_map_shadow_stack`

```rust
const __NR_map_shadow_stack: u32 = 453u32;
```

### `__NR_futex_wake`

```rust
const __NR_futex_wake: u32 = 454u32;
```

### `__NR_futex_wait`

```rust
const __NR_futex_wait: u32 = 455u32;
```

### `__NR_futex_requeue`

```rust
const __NR_futex_requeue: u32 = 456u32;
```

### `__NR_statmount`

```rust
const __NR_statmount: u32 = 457u32;
```

### `__NR_listmount`

```rust
const __NR_listmount: u32 = 458u32;
```

### `__NR_lsm_get_self_attr`

```rust
const __NR_lsm_get_self_attr: u32 = 459u32;
```

### `__NR_lsm_set_self_attr`

```rust
const __NR_lsm_set_self_attr: u32 = 460u32;
```

### `__NR_lsm_list_modules`

```rust
const __NR_lsm_list_modules: u32 = 461u32;
```

### `__NR_mseal`

```rust
const __NR_mseal: u32 = 462u32;
```

### `__NR_setxattrat`

```rust
const __NR_setxattrat: u32 = 463u32;
```

### `__NR_getxattrat`

```rust
const __NR_getxattrat: u32 = 464u32;
```

### `__NR_listxattrat`

```rust
const __NR_listxattrat: u32 = 465u32;
```

### `__NR_removexattrat`

```rust
const __NR_removexattrat: u32 = 466u32;
```

### `__NR_open_tree_attr`

```rust
const __NR_open_tree_attr: u32 = 467u32;
```

### `WNOHANG`

```rust
const WNOHANG: u32 = 1u32;
```

### `WUNTRACED`

```rust
const WUNTRACED: u32 = 2u32;
```

### `WSTOPPED`

```rust
const WSTOPPED: u32 = 2u32;
```

### `WEXITED`

```rust
const WEXITED: u32 = 4u32;
```

### `WCONTINUED`

```rust
const WCONTINUED: u32 = 8u32;
```

### `WNOWAIT`

```rust
const WNOWAIT: u32 = 16_777_216u32;
```

### `__WNOTHREAD`

```rust
const __WNOTHREAD: u32 = 536_870_912u32;
```

### `__WALL`

```rust
const __WALL: u32 = 1_073_741_824u32;
```

### `__WCLONE`

```rust
const __WCLONE: u32 = 2_147_483_648u32;
```

### `P_ALL`

```rust
const P_ALL: u32 = 0u32;
```

### `P_PID`

```rust
const P_PID: u32 = 1u32;
```

### `P_PGID`

```rust
const P_PGID: u32 = 2u32;
```

### `P_PIDFD`

```rust
const P_PIDFD: u32 = 3u32;
```

### `XATTR_CREATE`

```rust
const XATTR_CREATE: u32 = 1u32;
```

### `XATTR_REPLACE`

```rust
const XATTR_REPLACE: u32 = 2u32;
```

### `XATTR_OS2_PREFIX`

```rust
const XATTR_OS2_PREFIX: &[u8; 5];
```

### `XATTR_MAC_OSX_PREFIX`

```rust
const XATTR_MAC_OSX_PREFIX: &[u8; 5];
```

### `XATTR_BTRFS_PREFIX`

```rust
const XATTR_BTRFS_PREFIX: &[u8; 7];
```

### `XATTR_HURD_PREFIX`

```rust
const XATTR_HURD_PREFIX: &[u8; 5];
```

### `XATTR_SECURITY_PREFIX`

```rust
const XATTR_SECURITY_PREFIX: &[u8; 10];
```

### `XATTR_SYSTEM_PREFIX`

```rust
const XATTR_SYSTEM_PREFIX: &[u8; 8];
```

### `XATTR_TRUSTED_PREFIX`

```rust
const XATTR_TRUSTED_PREFIX: &[u8; 9];
```

### `XATTR_USER_PREFIX`

```rust
const XATTR_USER_PREFIX: &[u8; 6];
```

### `XATTR_EVM_SUFFIX`

```rust
const XATTR_EVM_SUFFIX: &[u8; 4];
```

### `XATTR_NAME_EVM`

```rust
const XATTR_NAME_EVM: &[u8; 13];
```

### `XATTR_IMA_SUFFIX`

```rust
const XATTR_IMA_SUFFIX: &[u8; 4];
```

### `XATTR_NAME_IMA`

```rust
const XATTR_NAME_IMA: &[u8; 13];
```

### `XATTR_SELINUX_SUFFIX`

```rust
const XATTR_SELINUX_SUFFIX: &[u8; 8];
```

### `XATTR_NAME_SELINUX`

```rust
const XATTR_NAME_SELINUX: &[u8; 17];
```

### `XATTR_SMACK_SUFFIX`

```rust
const XATTR_SMACK_SUFFIX: &[u8; 8];
```

### `XATTR_SMACK_IPIN`

```rust
const XATTR_SMACK_IPIN: &[u8; 12];
```

### `XATTR_SMACK_IPOUT`

```rust
const XATTR_SMACK_IPOUT: &[u8; 13];
```

### `XATTR_SMACK_EXEC`

```rust
const XATTR_SMACK_EXEC: &[u8; 12];
```

### `XATTR_SMACK_TRANSMUTE`

```rust
const XATTR_SMACK_TRANSMUTE: &[u8; 17];
```

### `XATTR_SMACK_MMAP`

```rust
const XATTR_SMACK_MMAP: &[u8; 12];
```

### `XATTR_NAME_SMACK`

```rust
const XATTR_NAME_SMACK: &[u8; 17];
```

### `XATTR_NAME_SMACKIPIN`

```rust
const XATTR_NAME_SMACKIPIN: &[u8; 21];
```

### `XATTR_NAME_SMACKIPOUT`

```rust
const XATTR_NAME_SMACKIPOUT: &[u8; 22];
```

### `XATTR_NAME_SMACKEXEC`

```rust
const XATTR_NAME_SMACKEXEC: &[u8; 21];
```

### `XATTR_NAME_SMACKTRANSMUTE`

```rust
const XATTR_NAME_SMACKTRANSMUTE: &[u8; 26];
```

### `XATTR_NAME_SMACKMMAP`

```rust
const XATTR_NAME_SMACKMMAP: &[u8; 21];
```

### `XATTR_APPARMOR_SUFFIX`

```rust
const XATTR_APPARMOR_SUFFIX: &[u8; 9];
```

### `XATTR_NAME_APPARMOR`

```rust
const XATTR_NAME_APPARMOR: &[u8; 18];
```

### `XATTR_CAPS_SUFFIX`

```rust
const XATTR_CAPS_SUFFIX: &[u8; 11];
```

### `XATTR_NAME_CAPS`

```rust
const XATTR_NAME_CAPS: &[u8; 20];
```

### `XATTR_BPF_LSM_SUFFIX`

```rust
const XATTR_BPF_LSM_SUFFIX: &[u8; 5];
```

### `XATTR_NAME_BPF_LSM`

```rust
const XATTR_NAME_BPF_LSM: &[u8; 14];
```

### `XATTR_POSIX_ACL_ACCESS`

```rust
const XATTR_POSIX_ACL_ACCESS: &[u8; 17];
```

### `XATTR_NAME_POSIX_ACL_ACCESS`

```rust
const XATTR_NAME_POSIX_ACL_ACCESS: &[u8; 24];
```

### `XATTR_POSIX_ACL_DEFAULT`

```rust
const XATTR_POSIX_ACL_DEFAULT: &[u8; 18];
```

### `XATTR_NAME_POSIX_ACL_DEFAULT`

```rust
const XATTR_NAME_POSIX_ACL_DEFAULT: &[u8; 25];
```

### `MFD_CLOEXEC`

```rust
const MFD_CLOEXEC: u32 = 1u32;
```

### `MFD_ALLOW_SEALING`

```rust
const MFD_ALLOW_SEALING: u32 = 2u32;
```

### `MFD_HUGETLB`

```rust
const MFD_HUGETLB: u32 = 4u32;
```

### `MFD_NOEXEC_SEAL`

```rust
const MFD_NOEXEC_SEAL: u32 = 8u32;
```

### `MFD_EXEC`

```rust
const MFD_EXEC: u32 = 16u32;
```

### `MFD_HUGE_SHIFT`

```rust
const MFD_HUGE_SHIFT: u32 = 26u32;
```

### `MFD_HUGE_MASK`

```rust
const MFD_HUGE_MASK: u32 = 63u32;
```

### `MFD_HUGE_64KB`

```rust
const MFD_HUGE_64KB: u32 = 1_073_741_824u32;
```

### `MFD_HUGE_512KB`

```rust
const MFD_HUGE_512KB: u32 = 1_275_068_416u32;
```

### `MFD_HUGE_1MB`

```rust
const MFD_HUGE_1MB: u32 = 1_342_177_280u32;
```

### `MFD_HUGE_2MB`

```rust
const MFD_HUGE_2MB: u32 = 1_409_286_144u32;
```

### `MFD_HUGE_8MB`

```rust
const MFD_HUGE_8MB: u32 = 1_543_503_872u32;
```

### `MFD_HUGE_16MB`

```rust
const MFD_HUGE_16MB: u32 = 1_610_612_736u32;
```

### `MFD_HUGE_32MB`

```rust
const MFD_HUGE_32MB: u32 = 1_677_721_600u32;
```

### `MFD_HUGE_256MB`

```rust
const MFD_HUGE_256MB: u32 = 1_879_048_192u32;
```

### `MFD_HUGE_512MB`

```rust
const MFD_HUGE_512MB: u32 = 1_946_157_056u32;
```

### `MFD_HUGE_1GB`

```rust
const MFD_HUGE_1GB: u32 = 2_013_265_920u32;
```

### `MFD_HUGE_2GB`

```rust
const MFD_HUGE_2GB: u32 = 2_080_374_784u32;
```

### `MFD_HUGE_16GB`

```rust
const MFD_HUGE_16GB: u32 = 2_281_701_376u32;
```

### `TFD_TIMER_ABSTIME`

```rust
const TFD_TIMER_ABSTIME: u32 = 1u32;
```

### `TFD_TIMER_CANCEL_ON_SET`

```rust
const TFD_TIMER_CANCEL_ON_SET: u32 = 2u32;
```

### `TFD_CLOEXEC`

```rust
const TFD_CLOEXEC: u32 = 524_288u32;
```

### `TFD_NONBLOCK`

```rust
const TFD_NONBLOCK: u32 = 2_048u32;
```

### `USERFAULTFD_IOC`

```rust
const USERFAULTFD_IOC: u32 = 170u32;
```

### `_UFFDIO_REGISTER`

```rust
const _UFFDIO_REGISTER: u32 = 0u32;
```

### `_UFFDIO_UNREGISTER`

```rust
const _UFFDIO_UNREGISTER: u32 = 1u32;
```

### `_UFFDIO_WAKE`

```rust
const _UFFDIO_WAKE: u32 = 2u32;
```

### `_UFFDIO_COPY`

```rust
const _UFFDIO_COPY: u32 = 3u32;
```

### `_UFFDIO_ZEROPAGE`

```rust
const _UFFDIO_ZEROPAGE: u32 = 4u32;
```

### `_UFFDIO_MOVE`

```rust
const _UFFDIO_MOVE: u32 = 5u32;
```

### `_UFFDIO_WRITEPROTECT`

```rust
const _UFFDIO_WRITEPROTECT: u32 = 6u32;
```

### `_UFFDIO_CONTINUE`

```rust
const _UFFDIO_CONTINUE: u32 = 7u32;
```

### `_UFFDIO_POISON`

```rust
const _UFFDIO_POISON: u32 = 8u32;
```

### `_UFFDIO_API`

```rust
const _UFFDIO_API: u32 = 63u32;
```

### `UFFDIO`

```rust
const UFFDIO: u32 = 170u32;
```

### `UFFD_EVENT_PAGEFAULT`

```rust
const UFFD_EVENT_PAGEFAULT: u32 = 18u32;
```

### `UFFD_EVENT_FORK`

```rust
const UFFD_EVENT_FORK: u32 = 19u32;
```

### `UFFD_EVENT_REMAP`

```rust
const UFFD_EVENT_REMAP: u32 = 20u32;
```

### `UFFD_EVENT_REMOVE`

```rust
const UFFD_EVENT_REMOVE: u32 = 21u32;
```

### `UFFD_EVENT_UNMAP`

```rust
const UFFD_EVENT_UNMAP: u32 = 22u32;
```

### `UFFD_PAGEFAULT_FLAG_WRITE`

```rust
const UFFD_PAGEFAULT_FLAG_WRITE: u32 = 1u32;
```

### `UFFD_PAGEFAULT_FLAG_WP`

```rust
const UFFD_PAGEFAULT_FLAG_WP: u32 = 2u32;
```

### `UFFD_PAGEFAULT_FLAG_MINOR`

```rust
const UFFD_PAGEFAULT_FLAG_MINOR: u32 = 4u32;
```

### `UFFD_FEATURE_PAGEFAULT_FLAG_WP`

```rust
const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u32 = 1u32;
```

### `UFFD_FEATURE_EVENT_FORK`

```rust
const UFFD_FEATURE_EVENT_FORK: u32 = 2u32;
```

### `UFFD_FEATURE_EVENT_REMAP`

```rust
const UFFD_FEATURE_EVENT_REMAP: u32 = 4u32;
```

### `UFFD_FEATURE_EVENT_REMOVE`

```rust
const UFFD_FEATURE_EVENT_REMOVE: u32 = 8u32;
```

### `UFFD_FEATURE_MISSING_HUGETLBFS`

```rust
const UFFD_FEATURE_MISSING_HUGETLBFS: u32 = 16u32;
```

### `UFFD_FEATURE_MISSING_SHMEM`

```rust
const UFFD_FEATURE_MISSING_SHMEM: u32 = 32u32;
```

### `UFFD_FEATURE_EVENT_UNMAP`

```rust
const UFFD_FEATURE_EVENT_UNMAP: u32 = 64u32;
```

### `UFFD_FEATURE_SIGBUS`

```rust
const UFFD_FEATURE_SIGBUS: u32 = 128u32;
```

### `UFFD_FEATURE_THREAD_ID`

```rust
const UFFD_FEATURE_THREAD_ID: u32 = 256u32;
```

### `UFFD_FEATURE_MINOR_HUGETLBFS`

```rust
const UFFD_FEATURE_MINOR_HUGETLBFS: u32 = 512u32;
```

### `UFFD_FEATURE_MINOR_SHMEM`

```rust
const UFFD_FEATURE_MINOR_SHMEM: u32 = 1_024u32;
```

### `UFFD_FEATURE_EXACT_ADDRESS`

```rust
const UFFD_FEATURE_EXACT_ADDRESS: u32 = 2_048u32;
```

### `UFFD_FEATURE_WP_HUGETLBFS_SHMEM`

```rust
const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: u32 = 4_096u32;
```

### `UFFD_FEATURE_WP_UNPOPULATED`

```rust
const UFFD_FEATURE_WP_UNPOPULATED: u32 = 8_192u32;
```

### `UFFD_FEATURE_POISON`

```rust
const UFFD_FEATURE_POISON: u32 = 16_384u32;
```

### `UFFD_FEATURE_WP_ASYNC`

```rust
const UFFD_FEATURE_WP_ASYNC: u32 = 32_768u32;
```

### `UFFD_FEATURE_MOVE`

```rust
const UFFD_FEATURE_MOVE: u32 = 65_536u32;
```

### `UFFD_USER_MODE_ONLY`

```rust
const UFFD_USER_MODE_ONLY: u32 = 1u32;
```

### `DT_UNKNOWN`

```rust
const DT_UNKNOWN: u32 = 0u32;
```

### `DT_FIFO`

```rust
const DT_FIFO: u32 = 1u32;
```

### `DT_CHR`

```rust
const DT_CHR: u32 = 2u32;
```

### `DT_DIR`

```rust
const DT_DIR: u32 = 4u32;
```

### `DT_BLK`

```rust
const DT_BLK: u32 = 6u32;
```

### `DT_REG`

```rust
const DT_REG: u32 = 8u32;
```

### `DT_LNK`

```rust
const DT_LNK: u32 = 10u32;
```

### `DT_SOCK`

```rust
const DT_SOCK: u32 = 12u32;
```

### `STAT_HAVE_NSEC`

```rust
const STAT_HAVE_NSEC: u32 = 1u32;
```

### `F_OK`

```rust
const F_OK: u32 = 0u32;
```

### `R_OK`

```rust
const R_OK: u32 = 4u32;
```

### `W_OK`

```rust
const W_OK: u32 = 2u32;
```

### `X_OK`

```rust
const X_OK: u32 = 1u32;
```

### `UTIME_NOW`

```rust
const UTIME_NOW: u32 = 1_073_741_823u32;
```

### `UTIME_OMIT`

```rust
const UTIME_OMIT: u32 = 1_073_741_822u32;
```

### `MNT_FORCE`

```rust
const MNT_FORCE: u32 = 1u32;
```

### `MNT_DETACH`

```rust
const MNT_DETACH: u32 = 2u32;
```

### `MNT_EXPIRE`

```rust
const MNT_EXPIRE: u32 = 4u32;
```

### `UMOUNT_NOFOLLOW`

```rust
const UMOUNT_NOFOLLOW: u32 = 8u32;
```

### `UMOUNT_UNUSED`

```rust
const UMOUNT_UNUSED: u32 = 2_147_483_648u32;
```

### `STDIN_FILENO`

```rust
const STDIN_FILENO: u32 = 0u32;
```

### `STDOUT_FILENO`

```rust
const STDOUT_FILENO: u32 = 1u32;
```

### `STDERR_FILENO`

```rust
const STDERR_FILENO: u32 = 2u32;
```

### `RWF_HIPRI`

```rust
const RWF_HIPRI: u32 = 1u32;
```

### `RWF_DSYNC`

```rust
const RWF_DSYNC: u32 = 2u32;
```

### `RWF_SYNC`

```rust
const RWF_SYNC: u32 = 4u32;
```

### `RWF_NOWAIT`

```rust
const RWF_NOWAIT: u32 = 8u32;
```

### `RWF_APPEND`

```rust
const RWF_APPEND: u32 = 16u32;
```

### `EFD_SEMAPHORE`

```rust
const EFD_SEMAPHORE: u32 = 1u32;
```

### `EFD_CLOEXEC`

```rust
const EFD_CLOEXEC: u32 = 524_288u32;
```

### `EFD_NONBLOCK`

```rust
const EFD_NONBLOCK: u32 = 2_048u32;
```

### `EPOLLIN`

```rust
const EPOLLIN: u32 = 1u32;
```

### `EPOLLPRI`

```rust
const EPOLLPRI: u32 = 2u32;
```

### `EPOLLOUT`

```rust
const EPOLLOUT: u32 = 4u32;
```

### `EPOLLERR`

```rust
const EPOLLERR: u32 = 8u32;
```

### `EPOLLHUP`

```rust
const EPOLLHUP: u32 = 16u32;
```

### `EPOLLNVAL`

```rust
const EPOLLNVAL: u32 = 32u32;
```

### `EPOLLRDNORM`

```rust
const EPOLLRDNORM: u32 = 64u32;
```

### `EPOLLRDBAND`

```rust
const EPOLLRDBAND: u32 = 128u32;
```

### `EPOLLWRNORM`

```rust
const EPOLLWRNORM: u32 = 256u32;
```

### `EPOLLWRBAND`

```rust
const EPOLLWRBAND: u32 = 512u32;
```

### `EPOLLMSG`

```rust
const EPOLLMSG: u32 = 1_024u32;
```

### `EPOLLRDHUP`

```rust
const EPOLLRDHUP: u32 = 8_192u32;
```

### `EPOLLEXCLUSIVE`

```rust
const EPOLLEXCLUSIVE: u32 = 268_435_456u32;
```

### `EPOLLWAKEUP`

```rust
const EPOLLWAKEUP: u32 = 536_870_912u32;
```

### `EPOLLONESHOT`

```rust
const EPOLLONESHOT: u32 = 1_073_741_824u32;
```

### `EPOLLET`

```rust
const EPOLLET: u32 = 2_147_483_648u32;
```

### `TFD_SHARED_FCNTL_FLAGS`

```rust
const TFD_SHARED_FCNTL_FLAGS: u32 = 526_336u32;
```

### `TFD_CREATE_FLAGS`

```rust
const TFD_CREATE_FLAGS: u32 = 526_336u32;
```

### `TFD_SETTIME_FLAGS`

```rust
const TFD_SETTIME_FLAGS: u32 = 1u32;
```

### `ARCH_SET_FS`

```rust
const ARCH_SET_FS: u32 = 4_098u32;
```

### `UFFD_API`

```rust
const UFFD_API: u32 = 170u32;
```

### `UFFDIO_REGISTER_MODE_MISSING`

```rust
const UFFDIO_REGISTER_MODE_MISSING: u32 = 1u32;
```

### `UFFDIO_REGISTER_MODE_WP`

```rust
const UFFDIO_REGISTER_MODE_WP: u32 = 2u32;
```

### `UFFDIO_REGISTER_MODE_MINOR`

```rust
const UFFDIO_REGISTER_MODE_MINOR: u32 = 4u32;
```

### `UFFDIO_COPY_MODE_DONTWAKE`

```rust
const UFFDIO_COPY_MODE_DONTWAKE: u32 = 1u32;
```

### `UFFDIO_COPY_MODE_WP`

```rust
const UFFDIO_COPY_MODE_WP: u32 = 2u32;
```

### `UFFDIO_ZEROPAGE_MODE_DONTWAKE`

```rust
const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u32 = 1u32;
```

### `SPLICE_F_MOVE`

```rust
const SPLICE_F_MOVE: u32 = 1u32;
```

### `SPLICE_F_NONBLOCK`

```rust
const SPLICE_F_NONBLOCK: u32 = 2u32;
```

### `SPLICE_F_MORE`

```rust
const SPLICE_F_MORE: u32 = 4u32;
```

### `SPLICE_F_GIFT`

```rust
const SPLICE_F_GIFT: u32 = 8u32;
```

### `_NSIG`

```rust
const _NSIG: u32 = 64u32;
```

