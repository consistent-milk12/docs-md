*[linux_raw_sys](../index.md) / [general](index.md)*

---

# Module `general`

## Contents

- [Structs](#structs)
  - [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)
  - [`__IncompleteArrayField`](#incompletearrayfield)
  - [`__kernel_fd_set`](#kernel-fd-set)
  - [`__kernel_fsid_t`](#kernel-fsid-t)
  - [`__user_cap_header_struct`](#user-cap-header-struct)
  - [`__user_cap_data_struct`](#user-cap-data-struct)
  - [`vfs_cap_data`](#vfs-cap-data)
  - [`vfs_cap_data__bindgen_ty_1`](#vfs-cap-data-bindgen-ty-1)
  - [`vfs_ns_cap_data`](#vfs-ns-cap-data)
  - [`vfs_ns_cap_data__bindgen_ty_1`](#vfs-ns-cap-data-bindgen-ty-1)
  - [`f_owner_ex`](#f-owner-ex)
  - [`flock`](#flock)
  - [`flock64`](#flock64)
  - [`open_how`](#open-how)
  - [`epoll_event`](#epoll-event)
  - [`epoll_params`](#epoll-params)
  - [`fscrypt_policy_v1`](#fscrypt-policy-v1)
  - [`fscrypt_key`](#fscrypt-key)
  - [`fscrypt_policy_v2`](#fscrypt-policy-v2)
  - [`fscrypt_get_policy_ex_arg`](#fscrypt-get-policy-ex-arg)
  - [`fscrypt_key_specifier`](#fscrypt-key-specifier)
  - [`fscrypt_provisioning_key_payload`](#fscrypt-provisioning-key-payload)
  - [`fscrypt_add_key_arg`](#fscrypt-add-key-arg)
  - [`fscrypt_remove_key_arg`](#fscrypt-remove-key-arg)
  - [`fscrypt_get_key_status_arg`](#fscrypt-get-key-status-arg)
  - [`mount_attr`](#mount-attr)
  - [`statmount`](#statmount)
  - [`mnt_id_req`](#mnt-id-req)
  - [`file_clone_range`](#file-clone-range)
  - [`fstrim_range`](#fstrim-range)
  - [`fsuuid2`](#fsuuid2)
  - [`fs_sysfs_path`](#fs-sysfs-path)
  - [`file_dedupe_range_info`](#file-dedupe-range-info)
  - [`file_dedupe_range`](#file-dedupe-range)
  - [`files_stat_struct`](#files-stat-struct)
  - [`inodes_stat_t`](#inodes-stat-t)
  - [`fsxattr`](#fsxattr)
  - [`page_region`](#page-region)
  - [`pm_scan_arg`](#pm-scan-arg)
  - [`procmap_query`](#procmap-query)
  - [`futex_waitv`](#futex-waitv)
  - [`robust_list`](#robust-list)
  - [`robust_list_head`](#robust-list-head)
  - [`inotify_event`](#inotify-event)
  - [`cachestat_range`](#cachestat-range)
  - [`cachestat`](#cachestat)
  - [`pollfd`](#pollfd)
  - [`rand_pool_info`](#rand-pool-info)
  - [`vgetrandom_opaque_params`](#vgetrandom-opaque-params)
  - [`__kernel_timespec`](#kernel-timespec)
  - [`__kernel_itimerspec`](#kernel-itimerspec)
  - [`__kernel_old_timeval`](#kernel-old-timeval)
  - [`__kernel_old_timespec`](#kernel-old-timespec)
  - [`__kernel_old_itimerval`](#kernel-old-itimerval)
  - [`__kernel_sock_timeval`](#kernel-sock-timeval)
  - [`rusage`](#rusage)
  - [`rlimit`](#rlimit)
  - [`rlimit64`](#rlimit64)
  - [`clone_args`](#clone-args)
  - [`sigaction`](#sigaction)
  - [`sigaltstack`](#sigaltstack)
  - [`__sifields__bindgen_ty_1`](#sifields-bindgen-ty-1)
  - [`__sifields__bindgen_ty_2`](#sifields-bindgen-ty-2)
  - [`__sifields__bindgen_ty_3`](#sifields-bindgen-ty-3)
  - [`__sifields__bindgen_ty_4`](#sifields-bindgen-ty-4)
  - [`__sifields__bindgen_ty_5`](#sifields-bindgen-ty-5)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2)
  - [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3)
  - [`__sifields__bindgen_ty_6`](#sifields-bindgen-ty-6)
  - [`__sifields__bindgen_ty_7`](#sifields-bindgen-ty-7)
  - [`siginfo`](#siginfo)
  - [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo-bindgen-ty-1-bindgen-ty-1)
  - [`sigevent`](#sigevent)
  - [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent-bindgen-ty-1-bindgen-ty-1)
  - [`statx_timestamp`](#statx-timestamp)
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
  - [`dmabuf_cmsg`](#dmabuf-cmsg)
  - [`dmabuf_token`](#dmabuf-token)
  - [`xattr_args`](#xattr-args)
  - [`uffd_msg`](#uffd-msg)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd-msg-bindgen-ty-1-bindgen-ty-1)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd-msg-bindgen-ty-1-bindgen-ty-2)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd-msg-bindgen-ty-1-bindgen-ty-3)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd-msg-bindgen-ty-1-bindgen-ty-4)
  - [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd-msg-bindgen-ty-1-bindgen-ty-5)
  - [`uffdio_api`](#uffdio-api)
  - [`uffdio_range`](#uffdio-range)
  - [`uffdio_register`](#uffdio-register)
  - [`uffdio_copy`](#uffdio-copy)
  - [`uffdio_zeropage`](#uffdio-zeropage)
  - [`uffdio_writeprotect`](#uffdio-writeprotect)
  - [`uffdio_continue`](#uffdio-continue)
  - [`uffdio_poison`](#uffdio-poison)
  - [`uffdio_move`](#uffdio-move)
  - [`linux_dirent64`](#linux-dirent64)
  - [`stat`](#stat)
  - [`__old_kernel_stat`](#old-kernel-stat)
  - [`statfs`](#statfs)
  - [`statfs64`](#statfs64)
  - [`compat_statfs64`](#compat-statfs64)
  - [`user_desc`](#user-desc)
  - [`kernel_sigset_t`](#kernel-sigset-t)
  - [`kernel_sigaction`](#kernel-sigaction)
- [Enums](#enums)
  - [`fsconfig_command`](#fsconfig-command)
  - [`procmap_query_flags`](#procmap-query-flags)
  - [`membarrier_cmd`](#membarrier-cmd)
  - [`membarrier_cmd_flag`](#membarrier-cmd-flag)
- [Type Aliases](#type-aliases)
  - [`__s8`](#s8)
  - [`__u8`](#u8)
  - [`__s16`](#s16)
  - [`__u16`](#u16)
  - [`__s32`](#s32)
  - [`__u32`](#u32)
  - [`__s64`](#s64)
  - [`__u64`](#u64)
  - [`__kernel_sighandler_t`](#kernel-sighandler-t)
  - [`__kernel_key_t`](#kernel-key-t)
  - [`__kernel_mqd_t`](#kernel-mqd-t)
  - [`__kernel_old_uid_t`](#kernel-old-uid-t)
  - [`__kernel_old_gid_t`](#kernel-old-gid-t)
  - [`__kernel_old_dev_t`](#kernel-old-dev-t)
  - [`__kernel_long_t`](#kernel-long-t)
  - [`__kernel_ulong_t`](#kernel-ulong-t)
  - [`__kernel_ino_t`](#kernel-ino-t)
  - [`__kernel_mode_t`](#kernel-mode-t)
  - [`__kernel_pid_t`](#kernel-pid-t)
  - [`__kernel_ipc_pid_t`](#kernel-ipc-pid-t)
  - [`__kernel_uid_t`](#kernel-uid-t)
  - [`__kernel_gid_t`](#kernel-gid-t)
  - [`__kernel_suseconds_t`](#kernel-suseconds-t)
  - [`__kernel_daddr_t`](#kernel-daddr-t)
  - [`__kernel_uid32_t`](#kernel-uid32-t)
  - [`__kernel_gid32_t`](#kernel-gid32-t)
  - [`__kernel_size_t`](#kernel-size-t)
  - [`__kernel_ssize_t`](#kernel-ssize-t)
  - [`__kernel_ptrdiff_t`](#kernel-ptrdiff-t)
  - [`__kernel_off_t`](#kernel-off-t)
  - [`__kernel_loff_t`](#kernel-loff-t)
  - [`__kernel_old_time_t`](#kernel-old-time-t)
  - [`__kernel_time_t`](#kernel-time-t)
  - [`__kernel_time64_t`](#kernel-time64-t)
  - [`__kernel_clock_t`](#kernel-clock-t)
  - [`__kernel_timer_t`](#kernel-timer-t)
  - [`__kernel_clockid_t`](#kernel-clockid-t)
  - [`__kernel_caddr_t`](#kernel-caddr-t)
  - [`__kernel_uid16_t`](#kernel-uid16-t)
  - [`__kernel_gid16_t`](#kernel-gid16-t)
  - [`__s128`](#s128)
  - [`__u128`](#u128)
  - [`__le16`](#le16)
  - [`__be16`](#be16)
  - [`__le32`](#le32)
  - [`__be32`](#be32)
  - [`__le64`](#le64)
  - [`__be64`](#be64)
  - [`__sum16`](#sum16)
  - [`__wsum`](#wsum)
  - [`__poll_t`](#poll-t)
  - [`cap_user_header_t`](#cap-user-header-t)
  - [`cap_user_data_t`](#cap-user-data-t)
  - [`__kernel_rwf_t`](#kernel-rwf-t)
  - [`sigset_t`](#sigset-t)
  - [`__signalfn_t`](#signalfn-t)
  - [`__sighandler_t`](#sighandler-t)
  - [`__restorefn_t`](#restorefn-t)
  - [`__sigrestore_t`](#sigrestore-t)
  - [`stack_t`](#stack-t)
  - [`sigval_t`](#sigval-t)
  - [`siginfo_t`](#siginfo-t)
  - [`sigevent_t`](#sigevent-t)
  - [`cc_t`](#cc-t)
  - [`speed_t`](#speed-t)
  - [`tcflag_t`](#tcflag-t)
  - [`__fsword_t`](#fsword-t)
- [Constants](#constants)
  - [`LINUX_VERSION_CODE`](#linux-version-code)
  - [`LINUX_VERSION_MAJOR`](#linux-version-major)
  - [`LINUX_VERSION_PATCHLEVEL`](#linux-version-patchlevel)
  - [`LINUX_VERSION_SUBLEVEL`](#linux-version-sublevel)
  - [`__BITS_PER_LONG_LONG`](#bits-per-long-long)
  - [`__FD_SETSIZE`](#fd-setsize)
  - [`_LINUX_CAPABILITY_VERSION_1`](#linux-capability-version-1)
  - [`_LINUX_CAPABILITY_U32S_1`](#linux-capability-u32s-1)
  - [`_LINUX_CAPABILITY_VERSION_2`](#linux-capability-version-2)
  - [`_LINUX_CAPABILITY_U32S_2`](#linux-capability-u32s-2)
  - [`_LINUX_CAPABILITY_VERSION_3`](#linux-capability-version-3)
  - [`_LINUX_CAPABILITY_U32S_3`](#linux-capability-u32s-3)
  - [`VFS_CAP_REVISION_MASK`](#vfs-cap-revision-mask)
  - [`VFS_CAP_REVISION_SHIFT`](#vfs-cap-revision-shift)
  - [`VFS_CAP_FLAGS_MASK`](#vfs-cap-flags-mask)
  - [`VFS_CAP_FLAGS_EFFECTIVE`](#vfs-cap-flags-effective)
  - [`VFS_CAP_REVISION_1`](#vfs-cap-revision-1)
  - [`VFS_CAP_U32_1`](#vfs-cap-u32-1)
  - [`VFS_CAP_REVISION_2`](#vfs-cap-revision-2)
  - [`VFS_CAP_U32_2`](#vfs-cap-u32-2)
  - [`VFS_CAP_REVISION_3`](#vfs-cap-revision-3)
  - [`VFS_CAP_U32_3`](#vfs-cap-u32-3)
  - [`VFS_CAP_U32`](#vfs-cap-u32)
  - [`VFS_CAP_REVISION`](#vfs-cap-revision)
  - [`_LINUX_CAPABILITY_VERSION`](#linux-capability-version)
  - [`_LINUX_CAPABILITY_U32S`](#linux-capability-u32s)
  - [`CAP_CHOWN`](#cap-chown)
  - [`CAP_DAC_OVERRIDE`](#cap-dac-override)
  - [`CAP_DAC_READ_SEARCH`](#cap-dac-read-search)
  - [`CAP_FOWNER`](#cap-fowner)
  - [`CAP_FSETID`](#cap-fsetid)
  - [`CAP_KILL`](#cap-kill)
  - [`CAP_SETGID`](#cap-setgid)
  - [`CAP_SETUID`](#cap-setuid)
  - [`CAP_SETPCAP`](#cap-setpcap)
  - [`CAP_LINUX_IMMUTABLE`](#cap-linux-immutable)
  - [`CAP_NET_BIND_SERVICE`](#cap-net-bind-service)
  - [`CAP_NET_BROADCAST`](#cap-net-broadcast)
  - [`CAP_NET_ADMIN`](#cap-net-admin)
  - [`CAP_NET_RAW`](#cap-net-raw)
  - [`CAP_IPC_LOCK`](#cap-ipc-lock)
  - [`CAP_IPC_OWNER`](#cap-ipc-owner)
  - [`CAP_SYS_MODULE`](#cap-sys-module)
  - [`CAP_SYS_RAWIO`](#cap-sys-rawio)
  - [`CAP_SYS_CHROOT`](#cap-sys-chroot)
  - [`CAP_SYS_PTRACE`](#cap-sys-ptrace)
  - [`CAP_SYS_PACCT`](#cap-sys-pacct)
  - [`CAP_SYS_ADMIN`](#cap-sys-admin)
  - [`CAP_SYS_BOOT`](#cap-sys-boot)
  - [`CAP_SYS_NICE`](#cap-sys-nice)
  - [`CAP_SYS_RESOURCE`](#cap-sys-resource)
  - [`CAP_SYS_TIME`](#cap-sys-time)
  - [`CAP_SYS_TTY_CONFIG`](#cap-sys-tty-config)
  - [`CAP_MKNOD`](#cap-mknod)
  - [`CAP_LEASE`](#cap-lease)
  - [`CAP_AUDIT_WRITE`](#cap-audit-write)
  - [`CAP_AUDIT_CONTROL`](#cap-audit-control)
  - [`CAP_SETFCAP`](#cap-setfcap)
  - [`CAP_MAC_OVERRIDE`](#cap-mac-override)
  - [`CAP_MAC_ADMIN`](#cap-mac-admin)
  - [`CAP_SYSLOG`](#cap-syslog)
  - [`CAP_WAKE_ALARM`](#cap-wake-alarm)
  - [`CAP_BLOCK_SUSPEND`](#cap-block-suspend)
  - [`CAP_AUDIT_READ`](#cap-audit-read)
  - [`CAP_PERFMON`](#cap-perfmon)
  - [`CAP_BPF`](#cap-bpf)
  - [`CAP_CHECKPOINT_RESTORE`](#cap-checkpoint-restore)
  - [`CAP_LAST_CAP`](#cap-last-cap)
  - [`O_ACCMODE`](#o-accmode)
  - [`O_RDONLY`](#o-rdonly)
  - [`O_WRONLY`](#o-wronly)
  - [`O_RDWR`](#o-rdwr)
  - [`O_CREAT`](#o-creat)
  - [`O_EXCL`](#o-excl)
  - [`O_NOCTTY`](#o-noctty)
  - [`O_TRUNC`](#o-trunc)
  - [`O_APPEND`](#o-append)
  - [`O_NONBLOCK`](#o-nonblock)
  - [`O_DSYNC`](#o-dsync)
  - [`FASYNC`](#fasync)
  - [`O_DIRECT`](#o-direct)
  - [`O_LARGEFILE`](#o-largefile)
  - [`O_DIRECTORY`](#o-directory)
  - [`O_NOFOLLOW`](#o-nofollow)
  - [`O_NOATIME`](#o-noatime)
  - [`O_CLOEXEC`](#o-cloexec)
  - [`__O_SYNC`](#o-sync)
  - [`O_SYNC`](#o-sync)
  - [`O_PATH`](#o-path)
  - [`__O_TMPFILE`](#o-tmpfile)
  - [`O_TMPFILE`](#o-tmpfile)
  - [`O_NDELAY`](#o-ndelay)
  - [`F_DUPFD`](#f-dupfd)
  - [`F_GETFD`](#f-getfd)
  - [`F_SETFD`](#f-setfd)
  - [`F_GETFL`](#f-getfl)
  - [`F_SETFL`](#f-setfl)
  - [`F_GETLK`](#f-getlk)
  - [`F_SETLK`](#f-setlk)
  - [`F_SETLKW`](#f-setlkw)
  - [`F_SETOWN`](#f-setown)
  - [`F_GETOWN`](#f-getown)
  - [`F_SETSIG`](#f-setsig)
  - [`F_GETSIG`](#f-getsig)
  - [`F_SETOWN_EX`](#f-setown-ex)
  - [`F_GETOWN_EX`](#f-getown-ex)
  - [`F_GETOWNER_UIDS`](#f-getowner-uids)
  - [`F_OFD_GETLK`](#f-ofd-getlk)
  - [`F_OFD_SETLK`](#f-ofd-setlk)
  - [`F_OFD_SETLKW`](#f-ofd-setlkw)
  - [`F_OWNER_TID`](#f-owner-tid)
  - [`F_OWNER_PID`](#f-owner-pid)
  - [`F_OWNER_PGRP`](#f-owner-pgrp)
  - [`FD_CLOEXEC`](#fd-cloexec)
  - [`F_RDLCK`](#f-rdlck)
  - [`F_WRLCK`](#f-wrlck)
  - [`F_UNLCK`](#f-unlck)
  - [`F_EXLCK`](#f-exlck)
  - [`F_SHLCK`](#f-shlck)
  - [`LOCK_SH`](#lock-sh)
  - [`LOCK_EX`](#lock-ex)
  - [`LOCK_NB`](#lock-nb)
  - [`LOCK_UN`](#lock-un)
  - [`LOCK_MAND`](#lock-mand)
  - [`LOCK_READ`](#lock-read)
  - [`LOCK_WRITE`](#lock-write)
  - [`LOCK_RW`](#lock-rw)
  - [`F_LINUX_SPECIFIC_BASE`](#f-linux-specific-base)
  - [`RESOLVE_NO_XDEV`](#resolve-no-xdev)
  - [`RESOLVE_NO_MAGICLINKS`](#resolve-no-magiclinks)
  - [`RESOLVE_NO_SYMLINKS`](#resolve-no-symlinks)
  - [`RESOLVE_BENEATH`](#resolve-beneath)
  - [`RESOLVE_IN_ROOT`](#resolve-in-root)
  - [`RESOLVE_CACHED`](#resolve-cached)
  - [`F_SETLEASE`](#f-setlease)
  - [`F_GETLEASE`](#f-getlease)
  - [`F_NOTIFY`](#f-notify)
  - [`F_DUPFD_QUERY`](#f-dupfd-query)
  - [`F_CREATED_QUERY`](#f-created-query)
  - [`F_CANCELLK`](#f-cancellk)
  - [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec)
  - [`F_SETPIPE_SZ`](#f-setpipe-sz)
  - [`F_GETPIPE_SZ`](#f-getpipe-sz)
  - [`F_ADD_SEALS`](#f-add-seals)
  - [`F_GET_SEALS`](#f-get-seals)
  - [`F_SEAL_SEAL`](#f-seal-seal)
  - [`F_SEAL_SHRINK`](#f-seal-shrink)
  - [`F_SEAL_GROW`](#f-seal-grow)
  - [`F_SEAL_WRITE`](#f-seal-write)
  - [`F_SEAL_FUTURE_WRITE`](#f-seal-future-write)
  - [`F_SEAL_EXEC`](#f-seal-exec)
  - [`F_GET_RW_HINT`](#f-get-rw-hint)
  - [`F_SET_RW_HINT`](#f-set-rw-hint)
  - [`F_GET_FILE_RW_HINT`](#f-get-file-rw-hint)
  - [`F_SET_FILE_RW_HINT`](#f-set-file-rw-hint)
  - [`RWH_WRITE_LIFE_NOT_SET`](#rwh-write-life-not-set)
  - [`RWH_WRITE_LIFE_NONE`](#rwh-write-life-none)
  - [`RWH_WRITE_LIFE_SHORT`](#rwh-write-life-short)
  - [`RWH_WRITE_LIFE_MEDIUM`](#rwh-write-life-medium)
  - [`RWH_WRITE_LIFE_LONG`](#rwh-write-life-long)
  - [`RWH_WRITE_LIFE_EXTREME`](#rwh-write-life-extreme)
  - [`RWF_WRITE_LIFE_NOT_SET`](#rwf-write-life-not-set)
  - [`DN_ACCESS`](#dn-access)
  - [`DN_MODIFY`](#dn-modify)
  - [`DN_CREATE`](#dn-create)
  - [`DN_DELETE`](#dn-delete)
  - [`DN_RENAME`](#dn-rename)
  - [`DN_ATTRIB`](#dn-attrib)
  - [`DN_MULTISHOT`](#dn-multishot)
  - [`AT_FDCWD`](#at-fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow)
  - [`AT_SYMLINK_FOLLOW`](#at-symlink-follow)
  - [`AT_NO_AUTOMOUNT`](#at-no-automount)
  - [`AT_EMPTY_PATH`](#at-empty-path)
  - [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type)
  - [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat)
  - [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync)
  - [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync)
  - [`AT_RECURSIVE`](#at-recursive)
  - [`AT_RENAME_NOREPLACE`](#at-rename-noreplace)
  - [`AT_RENAME_EXCHANGE`](#at-rename-exchange)
  - [`AT_RENAME_WHITEOUT`](#at-rename-whiteout)
  - [`AT_EACCESS`](#at-eaccess)
  - [`AT_REMOVEDIR`](#at-removedir)
  - [`AT_HANDLE_FID`](#at-handle-fid)
  - [`AT_HANDLE_MNT_ID_UNIQUE`](#at-handle-mnt-id-unique)
  - [`AT_HANDLE_CONNECTABLE`](#at-handle-connectable)
  - [`AT_EXECVE_CHECK`](#at-execve-check)
  - [`EPOLL_CLOEXEC`](#epoll-cloexec)
  - [`EPOLL_CTL_ADD`](#epoll-ctl-add)
  - [`EPOLL_CTL_DEL`](#epoll-ctl-del)
  - [`EPOLL_CTL_MOD`](#epoll-ctl-mod)
  - [`EPOLL_IOC_TYPE`](#epoll-ioc-type)
  - [`POSIX_FADV_NORMAL`](#posix-fadv-normal)
  - [`POSIX_FADV_RANDOM`](#posix-fadv-random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential)
  - [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed)
  - [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed)
  - [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse)
  - [`FALLOC_FL_ALLOCATE_RANGE`](#falloc-fl-allocate-range)
  - [`FALLOC_FL_KEEP_SIZE`](#falloc-fl-keep-size)
  - [`FALLOC_FL_PUNCH_HOLE`](#falloc-fl-punch-hole)
  - [`FALLOC_FL_NO_HIDE_STALE`](#falloc-fl-no-hide-stale)
  - [`FALLOC_FL_COLLAPSE_RANGE`](#falloc-fl-collapse-range)
  - [`FALLOC_FL_ZERO_RANGE`](#falloc-fl-zero-range)
  - [`FALLOC_FL_INSERT_RANGE`](#falloc-fl-insert-range)
  - [`FALLOC_FL_UNSHARE_RANGE`](#falloc-fl-unshare-range)
  - [`NR_OPEN`](#nr-open)
  - [`NGROUPS_MAX`](#ngroups-max)
  - [`ARG_MAX`](#arg-max)
  - [`LINK_MAX`](#link-max)
  - [`MAX_CANON`](#max-canon)
  - [`MAX_INPUT`](#max-input)
  - [`NAME_MAX`](#name-max)
  - [`PATH_MAX`](#path-max)
  - [`PIPE_BUF`](#pipe-buf)
  - [`XATTR_NAME_MAX`](#xattr-name-max)
  - [`XATTR_SIZE_MAX`](#xattr-size-max)
  - [`XATTR_LIST_MAX`](#xattr-list-max)
  - [`RTSIG_MAX`](#rtsig-max)
  - [`_IOC_NRBITS`](#ioc-nrbits)
  - [`_IOC_TYPEBITS`](#ioc-typebits)
  - [`_IOC_SIZEBITS`](#ioc-sizebits)
  - [`_IOC_DIRBITS`](#ioc-dirbits)
  - [`_IOC_NRMASK`](#ioc-nrmask)
  - [`_IOC_TYPEMASK`](#ioc-typemask)
  - [`_IOC_SIZEMASK`](#ioc-sizemask)
  - [`_IOC_DIRMASK`](#ioc-dirmask)
  - [`_IOC_NRSHIFT`](#ioc-nrshift)
  - [`_IOC_TYPESHIFT`](#ioc-typeshift)
  - [`_IOC_SIZESHIFT`](#ioc-sizeshift)
  - [`_IOC_DIRSHIFT`](#ioc-dirshift)
  - [`_IOC_NONE`](#ioc-none)
  - [`_IOC_WRITE`](#ioc-write)
  - [`_IOC_READ`](#ioc-read)
  - [`IOC_IN`](#ioc-in)
  - [`IOC_OUT`](#ioc-out)
  - [`IOC_INOUT`](#ioc-inout)
  - [`IOCSIZE_MASK`](#iocsize-mask)
  - [`IOCSIZE_SHIFT`](#iocsize-shift)
  - [`FSCRYPT_POLICY_FLAGS_PAD_4`](#fscrypt-policy-flags-pad-4)
  - [`FSCRYPT_POLICY_FLAGS_PAD_8`](#fscrypt-policy-flags-pad-8)
  - [`FSCRYPT_POLICY_FLAGS_PAD_16`](#fscrypt-policy-flags-pad-16)
  - [`FSCRYPT_POLICY_FLAGS_PAD_32`](#fscrypt-policy-flags-pad-32)
  - [`FSCRYPT_POLICY_FLAGS_PAD_MASK`](#fscrypt-policy-flags-pad-mask)
  - [`FSCRYPT_POLICY_FLAG_DIRECT_KEY`](#fscrypt-policy-flag-direct-key)
  - [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`](#fscrypt-policy-flag-iv-ino-lblk-64)
  - [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`](#fscrypt-policy-flag-iv-ino-lblk-32)
  - [`FSCRYPT_MODE_AES_256_XTS`](#fscrypt-mode-aes-256-xts)
  - [`FSCRYPT_MODE_AES_256_CTS`](#fscrypt-mode-aes-256-cts)
  - [`FSCRYPT_MODE_AES_128_CBC`](#fscrypt-mode-aes-128-cbc)
  - [`FSCRYPT_MODE_AES_128_CTS`](#fscrypt-mode-aes-128-cts)
  - [`FSCRYPT_MODE_SM4_XTS`](#fscrypt-mode-sm4-xts)
  - [`FSCRYPT_MODE_SM4_CTS`](#fscrypt-mode-sm4-cts)
  - [`FSCRYPT_MODE_ADIANTUM`](#fscrypt-mode-adiantum)
  - [`FSCRYPT_MODE_AES_256_HCTR2`](#fscrypt-mode-aes-256-hctr2)
  - [`FSCRYPT_POLICY_V1`](#fscrypt-policy-v1)
  - [`FSCRYPT_KEY_DESCRIPTOR_SIZE`](#fscrypt-key-descriptor-size)
  - [`FSCRYPT_KEY_DESC_PREFIX`](#fscrypt-key-desc-prefix)
  - [`FSCRYPT_KEY_DESC_PREFIX_SIZE`](#fscrypt-key-desc-prefix-size)
  - [`FSCRYPT_MAX_KEY_SIZE`](#fscrypt-max-key-size)
  - [`FSCRYPT_POLICY_V2`](#fscrypt-policy-v2)
  - [`FSCRYPT_KEY_IDENTIFIER_SIZE`](#fscrypt-key-identifier-size)
  - [`FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`](#fscrypt-key-spec-type-descriptor)
  - [`FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`](#fscrypt-key-spec-type-identifier)
  - [`FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`](#fscrypt-add-key-flag-hw-wrapped)
  - [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`](#fscrypt-key-removal-status-flag-files-busy)
  - [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`](#fscrypt-key-removal-status-flag-other-users)
  - [`FSCRYPT_KEY_STATUS_ABSENT`](#fscrypt-key-status-absent)
  - [`FSCRYPT_KEY_STATUS_PRESENT`](#fscrypt-key-status-present)
  - [`FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`](#fscrypt-key-status-incompletely-removed)
  - [`FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`](#fscrypt-key-status-flag-added-by-self)
  - [`FS_KEY_DESCRIPTOR_SIZE`](#fs-key-descriptor-size)
  - [`FS_POLICY_FLAGS_PAD_4`](#fs-policy-flags-pad-4)
  - [`FS_POLICY_FLAGS_PAD_8`](#fs-policy-flags-pad-8)
  - [`FS_POLICY_FLAGS_PAD_16`](#fs-policy-flags-pad-16)
  - [`FS_POLICY_FLAGS_PAD_32`](#fs-policy-flags-pad-32)
  - [`FS_POLICY_FLAGS_PAD_MASK`](#fs-policy-flags-pad-mask)
  - [`FS_POLICY_FLAG_DIRECT_KEY`](#fs-policy-flag-direct-key)
  - [`FS_POLICY_FLAGS_VALID`](#fs-policy-flags-valid)
  - [`FS_ENCRYPTION_MODE_INVALID`](#fs-encryption-mode-invalid)
  - [`FS_ENCRYPTION_MODE_AES_256_XTS`](#fs-encryption-mode-aes-256-xts)
  - [`FS_ENCRYPTION_MODE_AES_256_GCM`](#fs-encryption-mode-aes-256-gcm)
  - [`FS_ENCRYPTION_MODE_AES_256_CBC`](#fs-encryption-mode-aes-256-cbc)
  - [`FS_ENCRYPTION_MODE_AES_256_CTS`](#fs-encryption-mode-aes-256-cts)
  - [`FS_ENCRYPTION_MODE_AES_128_CBC`](#fs-encryption-mode-aes-128-cbc)
  - [`FS_ENCRYPTION_MODE_AES_128_CTS`](#fs-encryption-mode-aes-128-cts)
  - [`FS_ENCRYPTION_MODE_ADIANTUM`](#fs-encryption-mode-adiantum)
  - [`FS_KEY_DESC_PREFIX`](#fs-key-desc-prefix)
  - [`FS_KEY_DESC_PREFIX_SIZE`](#fs-key-desc-prefix-size)
  - [`FS_MAX_KEY_SIZE`](#fs-max-key-size)
  - [`MS_RDONLY`](#ms-rdonly)
  - [`MS_NOSUID`](#ms-nosuid)
  - [`MS_NODEV`](#ms-nodev)
  - [`MS_NOEXEC`](#ms-noexec)
  - [`MS_SYNCHRONOUS`](#ms-synchronous)
  - [`MS_REMOUNT`](#ms-remount)
  - [`MS_MANDLOCK`](#ms-mandlock)
  - [`MS_DIRSYNC`](#ms-dirsync)
  - [`MS_NOSYMFOLLOW`](#ms-nosymfollow)
  - [`MS_NOATIME`](#ms-noatime)
  - [`MS_NODIRATIME`](#ms-nodiratime)
  - [`MS_BIND`](#ms-bind)
  - [`MS_MOVE`](#ms-move)
  - [`MS_REC`](#ms-rec)
  - [`MS_VERBOSE`](#ms-verbose)
  - [`MS_SILENT`](#ms-silent)
  - [`MS_POSIXACL`](#ms-posixacl)
  - [`MS_UNBINDABLE`](#ms-unbindable)
  - [`MS_PRIVATE`](#ms-private)
  - [`MS_SLAVE`](#ms-slave)
  - [`MS_SHARED`](#ms-shared)
  - [`MS_RELATIME`](#ms-relatime)
  - [`MS_KERNMOUNT`](#ms-kernmount)
  - [`MS_I_VERSION`](#ms-i-version)
  - [`MS_STRICTATIME`](#ms-strictatime)
  - [`MS_LAZYTIME`](#ms-lazytime)
  - [`MS_SUBMOUNT`](#ms-submount)
  - [`MS_NOREMOTELOCK`](#ms-noremotelock)
  - [`MS_NOSEC`](#ms-nosec)
  - [`MS_BORN`](#ms-born)
  - [`MS_ACTIVE`](#ms-active)
  - [`MS_NOUSER`](#ms-nouser)
  - [`MS_RMT_MASK`](#ms-rmt-mask)
  - [`MS_MGC_VAL`](#ms-mgc-val)
  - [`MS_MGC_MSK`](#ms-mgc-msk)
  - [`OPEN_TREE_CLONE`](#open-tree-clone)
  - [`OPEN_TREE_CLOEXEC`](#open-tree-cloexec)
  - [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks)
  - [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts)
  - [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path)
  - [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks)
  - [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts)
  - [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path)
  - [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group)
  - [`MOVE_MOUNT_BENEATH`](#move-mount-beneath)
  - [`MOVE_MOUNT__MASK`](#move-mount-mask)
  - [`FSOPEN_CLOEXEC`](#fsopen-cloexec)
  - [`FSPICK_CLOEXEC`](#fspick-cloexec)
  - [`FSPICK_SYMLINK_NOFOLLOW`](#fspick-symlink-nofollow)
  - [`FSPICK_NO_AUTOMOUNT`](#fspick-no-automount)
  - [`FSPICK_EMPTY_PATH`](#fspick-empty-path)
  - [`FSMOUNT_CLOEXEC`](#fsmount-cloexec)
  - [`MOUNT_ATTR_RDONLY`](#mount-attr-rdonly)
  - [`MOUNT_ATTR_NOSUID`](#mount-attr-nosuid)
  - [`MOUNT_ATTR_NODEV`](#mount-attr-nodev)
  - [`MOUNT_ATTR_NOEXEC`](#mount-attr-noexec)
  - [`MOUNT_ATTR__ATIME`](#mount-attr-atime)
  - [`MOUNT_ATTR_RELATIME`](#mount-attr-relatime)
  - [`MOUNT_ATTR_NOATIME`](#mount-attr-noatime)
  - [`MOUNT_ATTR_STRICTATIME`](#mount-attr-strictatime)
  - [`MOUNT_ATTR_NODIRATIME`](#mount-attr-nodiratime)
  - [`MOUNT_ATTR_IDMAP`](#mount-attr-idmap)
  - [`MOUNT_ATTR_NOSYMFOLLOW`](#mount-attr-nosymfollow)
  - [`MOUNT_ATTR_SIZE_VER0`](#mount-attr-size-ver0)
  - [`MNT_ID_REQ_SIZE_VER0`](#mnt-id-req-size-ver0)
  - [`MNT_ID_REQ_SIZE_VER1`](#mnt-id-req-size-ver1)
  - [`STATMOUNT_SB_BASIC`](#statmount-sb-basic)
  - [`STATMOUNT_MNT_BASIC`](#statmount-mnt-basic)
  - [`STATMOUNT_PROPAGATE_FROM`](#statmount-propagate-from)
  - [`STATMOUNT_MNT_ROOT`](#statmount-mnt-root)
  - [`STATMOUNT_MNT_POINT`](#statmount-mnt-point)
  - [`STATMOUNT_FS_TYPE`](#statmount-fs-type)
  - [`STATMOUNT_MNT_NS_ID`](#statmount-mnt-ns-id)
  - [`STATMOUNT_MNT_OPTS`](#statmount-mnt-opts)
  - [`STATMOUNT_FS_SUBTYPE`](#statmount-fs-subtype)
  - [`STATMOUNT_SB_SOURCE`](#statmount-sb-source)
  - [`STATMOUNT_OPT_ARRAY`](#statmount-opt-array)
  - [`STATMOUNT_OPT_SEC_ARRAY`](#statmount-opt-sec-array)
  - [`STATMOUNT_SUPPORTED_MASK`](#statmount-supported-mask)
  - [`STATMOUNT_MNT_UIDMAP`](#statmount-mnt-uidmap)
  - [`STATMOUNT_MNT_GIDMAP`](#statmount-mnt-gidmap)
  - [`LSMT_ROOT`](#lsmt-root)
  - [`LISTMOUNT_REVERSE`](#listmount-reverse)
  - [`INR_OPEN_CUR`](#inr-open-cur)
  - [`INR_OPEN_MAX`](#inr-open-max)
  - [`BLOCK_SIZE_BITS`](#block-size-bits)
  - [`BLOCK_SIZE`](#block-size)
  - [`IO_INTEGRITY_CHK_GUARD`](#io-integrity-chk-guard)
  - [`IO_INTEGRITY_CHK_REFTAG`](#io-integrity-chk-reftag)
  - [`IO_INTEGRITY_CHK_APPTAG`](#io-integrity-chk-apptag)
  - [`IO_INTEGRITY_VALID_FLAGS`](#io-integrity-valid-flags)
  - [`SEEK_SET`](#seek-set)
  - [`SEEK_CUR`](#seek-cur)
  - [`SEEK_END`](#seek-end)
  - [`SEEK_DATA`](#seek-data)
  - [`SEEK_HOLE`](#seek-hole)
  - [`SEEK_MAX`](#seek-max)
  - [`RENAME_NOREPLACE`](#rename-noreplace)
  - [`RENAME_EXCHANGE`](#rename-exchange)
  - [`RENAME_WHITEOUT`](#rename-whiteout)
  - [`FILE_DEDUPE_RANGE_SAME`](#file-dedupe-range-same)
  - [`FILE_DEDUPE_RANGE_DIFFERS`](#file-dedupe-range-differs)
  - [`NR_FILE`](#nr-file)
  - [`FS_XFLAG_REALTIME`](#fs-xflag-realtime)
  - [`FS_XFLAG_PREALLOC`](#fs-xflag-prealloc)
  - [`FS_XFLAG_IMMUTABLE`](#fs-xflag-immutable)
  - [`FS_XFLAG_APPEND`](#fs-xflag-append)
  - [`FS_XFLAG_SYNC`](#fs-xflag-sync)
  - [`FS_XFLAG_NOATIME`](#fs-xflag-noatime)
  - [`FS_XFLAG_NODUMP`](#fs-xflag-nodump)
  - [`FS_XFLAG_RTINHERIT`](#fs-xflag-rtinherit)
  - [`FS_XFLAG_PROJINHERIT`](#fs-xflag-projinherit)
  - [`FS_XFLAG_NOSYMLINKS`](#fs-xflag-nosymlinks)
  - [`FS_XFLAG_EXTSIZE`](#fs-xflag-extsize)
  - [`FS_XFLAG_EXTSZINHERIT`](#fs-xflag-extszinherit)
  - [`FS_XFLAG_NODEFRAG`](#fs-xflag-nodefrag)
  - [`FS_XFLAG_FILESTREAM`](#fs-xflag-filestream)
  - [`FS_XFLAG_DAX`](#fs-xflag-dax)
  - [`FS_XFLAG_COWEXTSIZE`](#fs-xflag-cowextsize)
  - [`FS_XFLAG_HASATTR`](#fs-xflag-hasattr)
  - [`BMAP_IOCTL`](#bmap-ioctl)
  - [`FSLABEL_MAX`](#fslabel-max)
  - [`FS_SECRM_FL`](#fs-secrm-fl)
  - [`FS_UNRM_FL`](#fs-unrm-fl)
  - [`FS_COMPR_FL`](#fs-compr-fl)
  - [`FS_SYNC_FL`](#fs-sync-fl)
  - [`FS_IMMUTABLE_FL`](#fs-immutable-fl)
  - [`FS_APPEND_FL`](#fs-append-fl)
  - [`FS_NODUMP_FL`](#fs-nodump-fl)
  - [`FS_NOATIME_FL`](#fs-noatime-fl)
  - [`FS_DIRTY_FL`](#fs-dirty-fl)
  - [`FS_COMPRBLK_FL`](#fs-comprblk-fl)
  - [`FS_NOCOMP_FL`](#fs-nocomp-fl)
  - [`FS_ENCRYPT_FL`](#fs-encrypt-fl)
  - [`FS_BTREE_FL`](#fs-btree-fl)
  - [`FS_INDEX_FL`](#fs-index-fl)
  - [`FS_IMAGIC_FL`](#fs-imagic-fl)
  - [`FS_JOURNAL_DATA_FL`](#fs-journal-data-fl)
  - [`FS_NOTAIL_FL`](#fs-notail-fl)
  - [`FS_DIRSYNC_FL`](#fs-dirsync-fl)
  - [`FS_TOPDIR_FL`](#fs-topdir-fl)
  - [`FS_HUGE_FILE_FL`](#fs-huge-file-fl)
  - [`FS_EXTENT_FL`](#fs-extent-fl)
  - [`FS_VERITY_FL`](#fs-verity-fl)
  - [`FS_EA_INODE_FL`](#fs-ea-inode-fl)
  - [`FS_EOFBLOCKS_FL`](#fs-eofblocks-fl)
  - [`FS_NOCOW_FL`](#fs-nocow-fl)
  - [`FS_DAX_FL`](#fs-dax-fl)
  - [`FS_INLINE_DATA_FL`](#fs-inline-data-fl)
  - [`FS_PROJINHERIT_FL`](#fs-projinherit-fl)
  - [`FS_CASEFOLD_FL`](#fs-casefold-fl)
  - [`FS_RESERVED_FL`](#fs-reserved-fl)
  - [`FS_FL_USER_VISIBLE`](#fs-fl-user-visible)
  - [`FS_FL_USER_MODIFIABLE`](#fs-fl-user-modifiable)
  - [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync-file-range-wait-before)
  - [`SYNC_FILE_RANGE_WRITE`](#sync-file-range-write)
  - [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync-file-range-wait-after)
  - [`SYNC_FILE_RANGE_WRITE_AND_WAIT`](#sync-file-range-write-and-wait)
  - [`PROCFS_IOCTL_MAGIC`](#procfs-ioctl-magic)
  - [`PAGE_IS_WPALLOWED`](#page-is-wpallowed)
  - [`PAGE_IS_WRITTEN`](#page-is-written)
  - [`PAGE_IS_FILE`](#page-is-file)
  - [`PAGE_IS_PRESENT`](#page-is-present)
  - [`PAGE_IS_SWAPPED`](#page-is-swapped)
  - [`PAGE_IS_PFNZERO`](#page-is-pfnzero)
  - [`PAGE_IS_HUGE`](#page-is-huge)
  - [`PAGE_IS_SOFT_DIRTY`](#page-is-soft-dirty)
  - [`PAGE_IS_GUARD`](#page-is-guard)
  - [`PM_SCAN_WP_MATCHING`](#pm-scan-wp-matching)
  - [`PM_SCAN_CHECK_WPASYNC`](#pm-scan-check-wpasync)
  - [`FUTEX_WAIT`](#futex-wait)
  - [`FUTEX_WAKE`](#futex-wake)
  - [`FUTEX_FD`](#futex-fd)
  - [`FUTEX_REQUEUE`](#futex-requeue)
  - [`FUTEX_CMP_REQUEUE`](#futex-cmp-requeue)
  - [`FUTEX_WAKE_OP`](#futex-wake-op)
  - [`FUTEX_LOCK_PI`](#futex-lock-pi)
  - [`FUTEX_UNLOCK_PI`](#futex-unlock-pi)
  - [`FUTEX_TRYLOCK_PI`](#futex-trylock-pi)
  - [`FUTEX_WAIT_BITSET`](#futex-wait-bitset)
  - [`FUTEX_WAKE_BITSET`](#futex-wake-bitset)
  - [`FUTEX_WAIT_REQUEUE_PI`](#futex-wait-requeue-pi)
  - [`FUTEX_CMP_REQUEUE_PI`](#futex-cmp-requeue-pi)
  - [`FUTEX_LOCK_PI2`](#futex-lock-pi2)
  - [`FUTEX_PRIVATE_FLAG`](#futex-private-flag)
  - [`FUTEX_CLOCK_REALTIME`](#futex-clock-realtime)
  - [`FUTEX_CMD_MASK`](#futex-cmd-mask)
  - [`FUTEX_WAIT_PRIVATE`](#futex-wait-private)
  - [`FUTEX_WAKE_PRIVATE`](#futex-wake-private)
  - [`FUTEX_REQUEUE_PRIVATE`](#futex-requeue-private)
  - [`FUTEX_CMP_REQUEUE_PRIVATE`](#futex-cmp-requeue-private)
  - [`FUTEX_WAKE_OP_PRIVATE`](#futex-wake-op-private)
  - [`FUTEX_LOCK_PI_PRIVATE`](#futex-lock-pi-private)
  - [`FUTEX_LOCK_PI2_PRIVATE`](#futex-lock-pi2-private)
  - [`FUTEX_UNLOCK_PI_PRIVATE`](#futex-unlock-pi-private)
  - [`FUTEX_TRYLOCK_PI_PRIVATE`](#futex-trylock-pi-private)
  - [`FUTEX_WAIT_BITSET_PRIVATE`](#futex-wait-bitset-private)
  - [`FUTEX_WAKE_BITSET_PRIVATE`](#futex-wake-bitset-private)
  - [`FUTEX_WAIT_REQUEUE_PI_PRIVATE`](#futex-wait-requeue-pi-private)
  - [`FUTEX_CMP_REQUEUE_PI_PRIVATE`](#futex-cmp-requeue-pi-private)
  - [`FUTEX2_SIZE_U8`](#futex2-size-u8)
  - [`FUTEX2_SIZE_U16`](#futex2-size-u16)
  - [`FUTEX2_SIZE_U32`](#futex2-size-u32)
  - [`FUTEX2_SIZE_U64`](#futex2-size-u64)
  - [`FUTEX2_NUMA`](#futex2-numa)
  - [`FUTEX2_MPOL`](#futex2-mpol)
  - [`FUTEX2_PRIVATE`](#futex2-private)
  - [`FUTEX2_SIZE_MASK`](#futex2-size-mask)
  - [`FUTEX_32`](#futex-32)
  - [`FUTEX_NO_NODE`](#futex-no-node)
  - [`FUTEX_WAITV_MAX`](#futex-waitv-max)
  - [`FUTEX_WAITERS`](#futex-waiters)
  - [`FUTEX_OWNER_DIED`](#futex-owner-died)
  - [`FUTEX_TID_MASK`](#futex-tid-mask)
  - [`ROBUST_LIST_LIMIT`](#robust-list-limit)
  - [`FUTEX_BITSET_MATCH_ANY`](#futex-bitset-match-any)
  - [`FUTEX_OP_SET`](#futex-op-set)
  - [`FUTEX_OP_ADD`](#futex-op-add)
  - [`FUTEX_OP_OR`](#futex-op-or)
  - [`FUTEX_OP_ANDN`](#futex-op-andn)
  - [`FUTEX_OP_XOR`](#futex-op-xor)
  - [`FUTEX_OP_OPARG_SHIFT`](#futex-op-oparg-shift)
  - [`FUTEX_OP_CMP_EQ`](#futex-op-cmp-eq)
  - [`FUTEX_OP_CMP_NE`](#futex-op-cmp-ne)
  - [`FUTEX_OP_CMP_LT`](#futex-op-cmp-lt)
  - [`FUTEX_OP_CMP_LE`](#futex-op-cmp-le)
  - [`FUTEX_OP_CMP_GT`](#futex-op-cmp-gt)
  - [`FUTEX_OP_CMP_GE`](#futex-op-cmp-ge)
  - [`IN_ACCESS`](#in-access)
  - [`IN_MODIFY`](#in-modify)
  - [`IN_ATTRIB`](#in-attrib)
  - [`IN_CLOSE_WRITE`](#in-close-write)
  - [`IN_CLOSE_NOWRITE`](#in-close-nowrite)
  - [`IN_OPEN`](#in-open)
  - [`IN_MOVED_FROM`](#in-moved-from)
  - [`IN_MOVED_TO`](#in-moved-to)
  - [`IN_CREATE`](#in-create)
  - [`IN_DELETE`](#in-delete)
  - [`IN_DELETE_SELF`](#in-delete-self)
  - [`IN_MOVE_SELF`](#in-move-self)
  - [`IN_UNMOUNT`](#in-unmount)
  - [`IN_Q_OVERFLOW`](#in-q-overflow)
  - [`IN_IGNORED`](#in-ignored)
  - [`IN_CLOSE`](#in-close)
  - [`IN_MOVE`](#in-move)
  - [`IN_ONLYDIR`](#in-onlydir)
  - [`IN_DONT_FOLLOW`](#in-dont-follow)
  - [`IN_EXCL_UNLINK`](#in-excl-unlink)
  - [`IN_MASK_CREATE`](#in-mask-create)
  - [`IN_MASK_ADD`](#in-mask-add)
  - [`IN_ISDIR`](#in-isdir)
  - [`IN_ONESHOT`](#in-oneshot)
  - [`IN_ALL_EVENTS`](#in-all-events)
  - [`IN_CLOEXEC`](#in-cloexec)
  - [`IN_NONBLOCK`](#in-nonblock)
  - [`ADFS_SUPER_MAGIC`](#adfs-super-magic)
  - [`AFFS_SUPER_MAGIC`](#affs-super-magic)
  - [`AFS_SUPER_MAGIC`](#afs-super-magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic)
  - [`CEPH_SUPER_MAGIC`](#ceph-super-magic)
  - [`CODA_SUPER_MAGIC`](#coda-super-magic)
  - [`CRAMFS_MAGIC`](#cramfs-magic)
  - [`CRAMFS_MAGIC_WEND`](#cramfs-magic-wend)
  - [`DEBUGFS_MAGIC`](#debugfs-magic)
  - [`SECURITYFS_MAGIC`](#securityfs-magic)
  - [`SELINUX_MAGIC`](#selinux-magic)
  - [`SMACK_MAGIC`](#smack-magic)
  - [`RAMFS_MAGIC`](#ramfs-magic)
  - [`TMPFS_MAGIC`](#tmpfs-magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs-magic)
  - [`SQUASHFS_MAGIC`](#squashfs-magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic)
  - [`EFS_SUPER_MAGIC`](#efs-super-magic)
  - [`EROFS_SUPER_MAGIC_V1`](#erofs-super-magic-v1)
  - [`EXT2_SUPER_MAGIC`](#ext2-super-magic)
  - [`EXT3_SUPER_MAGIC`](#ext3-super-magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs-super-magic)
  - [`EXT4_SUPER_MAGIC`](#ext4-super-magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs-super-magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs-super-magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs-super-magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs-super-magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic)
  - [`XFS_SUPER_MAGIC`](#xfs-super-magic)
  - [`PSTOREFS_MAGIC`](#pstorefs-magic)
  - [`EFIVARFS_MAGIC`](#efivarfs-magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic)
  - [`FUSE_SUPER_MAGIC`](#fuse-super-magic)
  - [`BCACHEFS_SUPER_MAGIC`](#bcachefs-super-magic)
  - [`MINIX_SUPER_MAGIC`](#minix-super-magic)
  - [`MINIX_SUPER_MAGIC2`](#minix-super-magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2-super-magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2)
  - [`MINIX3_SUPER_MAGIC`](#minix3-super-magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos-super-magic)
  - [`EXFAT_SUPER_MAGIC`](#exfat-super-magic)
  - [`NCP_SUPER_MAGIC`](#ncp-super-magic)
  - [`NFS_SUPER_MAGIC`](#nfs-super-magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4-super-magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6-super-magic)
  - [`AFS_FS_MAGIC`](#afs-fs-magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic)
  - [`REISERFS_SUPER_MAGIC_STRING`](#reiserfs-super-magic-string)
  - [`REISER2FS_SUPER_MAGIC_STRING`](#reiser2fs-super-magic-string)
  - [`REISER2FS_JR_SUPER_MAGIC_STRING`](#reiser2fs-jr-super-magic-string)
  - [`SMB_SUPER_MAGIC`](#smb-super-magic)
  - [`CIFS_SUPER_MAGIC`](#cifs-super-magic)
  - [`SMB2_SUPER_MAGIC`](#smb2-super-magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic)
  - [`STACK_END_MAGIC`](#stack-end-magic)
  - [`TRACEFS_MAGIC`](#tracefs-magic)
  - [`V9FS_MAGIC`](#v9fs-magic)
  - [`BDEVFS_MAGIC`](#bdevfs-magic)
  - [`DAXFS_MAGIC`](#daxfs-magic)
  - [`BINFMTFS_MAGIC`](#binfmtfs-magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic)
  - [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic)
  - [`PIPEFS_MAGIC`](#pipefs-magic)
  - [`PROC_SUPER_MAGIC`](#proc-super-magic)
  - [`SOCKFS_MAGIC`](#sockfs-magic)
  - [`SYSFS_MAGIC`](#sysfs-magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic)
  - [`MTD_INODE_FS_MAGIC`](#mtd-inode-fs-magic)
  - [`ANON_INODE_FS_MAGIC`](#anon-inode-fs-magic)
  - [`BTRFS_TEST_MAGIC`](#btrfs-test-magic)
  - [`NSFS_MAGIC`](#nsfs-magic)
  - [`BPF_FS_MAGIC`](#bpf-fs-magic)
  - [`AAFS_MAGIC`](#aafs-magic)
  - [`ZONEFS_MAGIC`](#zonefs-magic)
  - [`UDF_SUPER_MAGIC`](#udf-super-magic)
  - [`DMA_BUF_MAGIC`](#dma-buf-magic)
  - [`DEVMEM_MAGIC`](#devmem-magic)
  - [`SECRETMEM_MAGIC`](#secretmem-magic)
  - [`PID_FS_MAGIC`](#pid-fs-magic)
  - [`MAP_32BIT`](#map-32bit)
  - [`MAP_ABOVE4G`](#map-above4g)
  - [`PROT_READ`](#prot-read)
  - [`PROT_WRITE`](#prot-write)
  - [`PROT_EXEC`](#prot-exec)
  - [`PROT_SEM`](#prot-sem)
  - [`PROT_NONE`](#prot-none)
  - [`PROT_GROWSDOWN`](#prot-growsdown)
  - [`PROT_GROWSUP`](#prot-growsup)
  - [`MAP_TYPE`](#map-type)
  - [`MAP_FIXED`](#map-fixed)
  - [`MAP_ANONYMOUS`](#map-anonymous)
  - [`MAP_POPULATE`](#map-populate)
  - [`MAP_NONBLOCK`](#map-nonblock)
  - [`MAP_STACK`](#map-stack)
  - [`MAP_HUGETLB`](#map-hugetlb)
  - [`MAP_SYNC`](#map-sync)
  - [`MAP_FIXED_NOREPLACE`](#map-fixed-noreplace)
  - [`MAP_UNINITIALIZED`](#map-uninitialized)
  - [`MLOCK_ONFAULT`](#mlock-onfault)
  - [`MS_ASYNC`](#ms-async)
  - [`MS_INVALIDATE`](#ms-invalidate)
  - [`MS_SYNC`](#ms-sync)
  - [`MADV_NORMAL`](#madv-normal)
  - [`MADV_RANDOM`](#madv-random)
  - [`MADV_SEQUENTIAL`](#madv-sequential)
  - [`MADV_WILLNEED`](#madv-willneed)
  - [`MADV_DONTNEED`](#madv-dontneed)
  - [`MADV_FREE`](#madv-free)
  - [`MADV_REMOVE`](#madv-remove)
  - [`MADV_DONTFORK`](#madv-dontfork)
  - [`MADV_DOFORK`](#madv-dofork)
  - [`MADV_HWPOISON`](#madv-hwpoison)
  - [`MADV_SOFT_OFFLINE`](#madv-soft-offline)
  - [`MADV_MERGEABLE`](#madv-mergeable)
  - [`MADV_UNMERGEABLE`](#madv-unmergeable)
  - [`MADV_HUGEPAGE`](#madv-hugepage)
  - [`MADV_NOHUGEPAGE`](#madv-nohugepage)
  - [`MADV_DONTDUMP`](#madv-dontdump)
  - [`MADV_DODUMP`](#madv-dodump)
  - [`MADV_WIPEONFORK`](#madv-wipeonfork)
  - [`MADV_KEEPONFORK`](#madv-keeponfork)
  - [`MADV_COLD`](#madv-cold)
  - [`MADV_PAGEOUT`](#madv-pageout)
  - [`MADV_POPULATE_READ`](#madv-populate-read)
  - [`MADV_POPULATE_WRITE`](#madv-populate-write)
  - [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked)
  - [`MADV_COLLAPSE`](#madv-collapse)
  - [`MADV_GUARD_INSTALL`](#madv-guard-install)
  - [`MADV_GUARD_REMOVE`](#madv-guard-remove)
  - [`MAP_FILE`](#map-file)
  - [`PKEY_UNRESTRICTED`](#pkey-unrestricted)
  - [`PKEY_DISABLE_ACCESS`](#pkey-disable-access)
  - [`PKEY_DISABLE_WRITE`](#pkey-disable-write)
  - [`PKEY_ACCESS_MASK`](#pkey-access-mask)
  - [`MAP_GROWSDOWN`](#map-growsdown)
  - [`MAP_DENYWRITE`](#map-denywrite)
  - [`MAP_EXECUTABLE`](#map-executable)
  - [`MAP_LOCKED`](#map-locked)
  - [`MAP_NORESERVE`](#map-noreserve)
  - [`MCL_CURRENT`](#mcl-current)
  - [`MCL_FUTURE`](#mcl-future)
  - [`MCL_ONFAULT`](#mcl-onfault)
  - [`SHADOW_STACK_SET_TOKEN`](#shadow-stack-set-token)
  - [`SHADOW_STACK_SET_MARKER`](#shadow-stack-set-marker)
  - [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift)
  - [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask)
  - [`HUGETLB_FLAG_ENCODE_16KB`](#hugetlb-flag-encode-16kb)
  - [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb-flag-encode-64kb)
  - [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb-flag-encode-512kb)
  - [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb-flag-encode-1mb)
  - [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb-flag-encode-2mb)
  - [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb-flag-encode-8mb)
  - [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb-flag-encode-16mb)
  - [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb-flag-encode-32mb)
  - [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb-flag-encode-256mb)
  - [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb-flag-encode-512mb)
  - [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb-flag-encode-1gb)
  - [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb-flag-encode-2gb)
  - [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb-flag-encode-16gb)
  - [`MREMAP_MAYMOVE`](#mremap-maymove)
  - [`MREMAP_FIXED`](#mremap-fixed)
  - [`MREMAP_DONTUNMAP`](#mremap-dontunmap)
  - [`OVERCOMMIT_GUESS`](#overcommit-guess)
  - [`OVERCOMMIT_ALWAYS`](#overcommit-always)
  - [`OVERCOMMIT_NEVER`](#overcommit-never)
  - [`MAP_SHARED`](#map-shared)
  - [`MAP_PRIVATE`](#map-private)
  - [`MAP_SHARED_VALIDATE`](#map-shared-validate)
  - [`MAP_DROPPABLE`](#map-droppable)
  - [`MAP_HUGE_SHIFT`](#map-huge-shift)
  - [`MAP_HUGE_MASK`](#map-huge-mask)
  - [`MAP_HUGE_16KB`](#map-huge-16kb)
  - [`MAP_HUGE_64KB`](#map-huge-64kb)
  - [`MAP_HUGE_512KB`](#map-huge-512kb)
  - [`MAP_HUGE_1MB`](#map-huge-1mb)
  - [`MAP_HUGE_2MB`](#map-huge-2mb)
  - [`MAP_HUGE_8MB`](#map-huge-8mb)
  - [`MAP_HUGE_16MB`](#map-huge-16mb)
  - [`MAP_HUGE_32MB`](#map-huge-32mb)
  - [`MAP_HUGE_256MB`](#map-huge-256mb)
  - [`MAP_HUGE_512MB`](#map-huge-512mb)
  - [`MAP_HUGE_1GB`](#map-huge-1gb)
  - [`MAP_HUGE_2GB`](#map-huge-2gb)
  - [`MAP_HUGE_16GB`](#map-huge-16gb)
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
  - [`GRND_NONBLOCK`](#grnd-nonblock)
  - [`GRND_RANDOM`](#grnd-random)
  - [`GRND_INSECURE`](#grnd-insecure)
  - [`LINUX_REBOOT_MAGIC1`](#linux-reboot-magic1)
  - [`LINUX_REBOOT_MAGIC2`](#linux-reboot-magic2)
  - [`LINUX_REBOOT_MAGIC2A`](#linux-reboot-magic2a)
  - [`LINUX_REBOOT_MAGIC2B`](#linux-reboot-magic2b)
  - [`LINUX_REBOOT_MAGIC2C`](#linux-reboot-magic2c)
  - [`LINUX_REBOOT_CMD_RESTART`](#linux-reboot-cmd-restart)
  - [`LINUX_REBOOT_CMD_HALT`](#linux-reboot-cmd-halt)
  - [`LINUX_REBOOT_CMD_CAD_ON`](#linux-reboot-cmd-cad-on)
  - [`LINUX_REBOOT_CMD_CAD_OFF`](#linux-reboot-cmd-cad-off)
  - [`LINUX_REBOOT_CMD_POWER_OFF`](#linux-reboot-cmd-power-off)
  - [`LINUX_REBOOT_CMD_RESTART2`](#linux-reboot-cmd-restart2)
  - [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux-reboot-cmd-sw-suspend)
  - [`LINUX_REBOOT_CMD_KEXEC`](#linux-reboot-cmd-kexec)
  - [`RUSAGE_SELF`](#rusage-self)
  - [`RUSAGE_CHILDREN`](#rusage-children)
  - [`RUSAGE_BOTH`](#rusage-both)
  - [`RUSAGE_THREAD`](#rusage-thread)
  - [`RLIM64_INFINITY`](#rlim64-infinity)
  - [`PRIO_MIN`](#prio-min)
  - [`PRIO_MAX`](#prio-max)
  - [`PRIO_PROCESS`](#prio-process)
  - [`PRIO_PGRP`](#prio-pgrp)
  - [`PRIO_USER`](#prio-user)
  - [`_STK_LIM`](#stk-lim)
  - [`MLOCK_LIMIT`](#mlock-limit)
  - [`RLIMIT_CPU`](#rlimit-cpu)
  - [`RLIMIT_FSIZE`](#rlimit-fsize)
  - [`RLIMIT_DATA`](#rlimit-data)
  - [`RLIMIT_STACK`](#rlimit-stack)
  - [`RLIMIT_CORE`](#rlimit-core)
  - [`RLIMIT_RSS`](#rlimit-rss)
  - [`RLIMIT_NPROC`](#rlimit-nproc)
  - [`RLIMIT_NOFILE`](#rlimit-nofile)
  - [`RLIMIT_MEMLOCK`](#rlimit-memlock)
  - [`RLIMIT_AS`](#rlimit-as)
  - [`RLIMIT_LOCKS`](#rlimit-locks)
  - [`RLIMIT_SIGPENDING`](#rlimit-sigpending)
  - [`RLIMIT_MSGQUEUE`](#rlimit-msgqueue)
  - [`RLIMIT_NICE`](#rlimit-nice)
  - [`RLIMIT_RTPRIO`](#rlimit-rtprio)
  - [`RLIMIT_RTTIME`](#rlimit-rttime)
  - [`RLIM_NLIMITS`](#rlim-nlimits)
  - [`RLIM_INFINITY`](#rlim-infinity)
  - [`CSIGNAL`](#csignal)
  - [`CLONE_VM`](#clone-vm)
  - [`CLONE_FS`](#clone-fs)
  - [`CLONE_FILES`](#clone-files)
  - [`CLONE_SIGHAND`](#clone-sighand)
  - [`CLONE_PIDFD`](#clone-pidfd)
  - [`CLONE_PTRACE`](#clone-ptrace)
  - [`CLONE_VFORK`](#clone-vfork)
  - [`CLONE_PARENT`](#clone-parent)
  - [`CLONE_THREAD`](#clone-thread)
  - [`CLONE_NEWNS`](#clone-newns)
  - [`CLONE_SYSVSEM`](#clone-sysvsem)
  - [`CLONE_SETTLS`](#clone-settls)
  - [`CLONE_PARENT_SETTID`](#clone-parent-settid)
  - [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid)
  - [`CLONE_DETACHED`](#clone-detached)
  - [`CLONE_UNTRACED`](#clone-untraced)
  - [`CLONE_CHILD_SETTID`](#clone-child-settid)
  - [`CLONE_NEWCGROUP`](#clone-newcgroup)
  - [`CLONE_NEWUTS`](#clone-newuts)
  - [`CLONE_NEWIPC`](#clone-newipc)
  - [`CLONE_NEWUSER`](#clone-newuser)
  - [`CLONE_NEWPID`](#clone-newpid)
  - [`CLONE_NEWNET`](#clone-newnet)
  - [`CLONE_IO`](#clone-io)
  - [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand)
  - [`CLONE_INTO_CGROUP`](#clone-into-cgroup)
  - [`CLONE_NEWTIME`](#clone-newtime)
  - [`CLONE_ARGS_SIZE_VER0`](#clone-args-size-ver0)
  - [`CLONE_ARGS_SIZE_VER1`](#clone-args-size-ver1)
  - [`CLONE_ARGS_SIZE_VER2`](#clone-args-size-ver2)
  - [`SCHED_NORMAL`](#sched-normal)
  - [`SCHED_FIFO`](#sched-fifo)
  - [`SCHED_RR`](#sched-rr)
  - [`SCHED_BATCH`](#sched-batch)
  - [`SCHED_IDLE`](#sched-idle)
  - [`SCHED_DEADLINE`](#sched-deadline)
  - [`SCHED_EXT`](#sched-ext)
  - [`SCHED_RESET_ON_FORK`](#sched-reset-on-fork)
  - [`SCHED_FLAG_RESET_ON_FORK`](#sched-flag-reset-on-fork)
  - [`SCHED_FLAG_RECLAIM`](#sched-flag-reclaim)
  - [`SCHED_FLAG_DL_OVERRUN`](#sched-flag-dl-overrun)
  - [`SCHED_FLAG_KEEP_POLICY`](#sched-flag-keep-policy)
  - [`SCHED_FLAG_KEEP_PARAMS`](#sched-flag-keep-params)
  - [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched-flag-util-clamp-min)
  - [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched-flag-util-clamp-max)
  - [`SCHED_FLAG_KEEP_ALL`](#sched-flag-keep-all)
  - [`SCHED_FLAG_UTIL_CLAMP`](#sched-flag-util-clamp)
  - [`SCHED_FLAG_ALL`](#sched-flag-all)
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
  - [`SA_RESTORER`](#sa-restorer)
  - [`MINSIGSTKSZ`](#minsigstksz)
  - [`SIGSTKSZ`](#sigstksz)
  - [`SA_NOCLDSTOP`](#sa-nocldstop)
  - [`SA_NOCLDWAIT`](#sa-nocldwait)
  - [`SA_SIGINFO`](#sa-siginfo)
  - [`SA_UNSUPPORTED`](#sa-unsupported)
  - [`SA_EXPOSE_TAGBITS`](#sa-expose-tagbits)
  - [`SA_ONSTACK`](#sa-onstack)
  - [`SA_RESTART`](#sa-restart)
  - [`SA_NODEFER`](#sa-nodefer)
  - [`SA_RESETHAND`](#sa-resethand)
  - [`SA_NOMASK`](#sa-nomask)
  - [`SA_ONESHOT`](#sa-oneshot)
  - [`SIG_BLOCK`](#sig-block)
  - [`SIG_UNBLOCK`](#sig-unblock)
  - [`SIG_SETMASK`](#sig-setmask)
  - [`SI_MAX_SIZE`](#si-max-size)
  - [`SI_USER`](#si-user)
  - [`SI_KERNEL`](#si-kernel)
  - [`SI_QUEUE`](#si-queue)
  - [`SI_TIMER`](#si-timer)
  - [`SI_MESGQ`](#si-mesgq)
  - [`SI_ASYNCIO`](#si-asyncio)
  - [`SI_SIGIO`](#si-sigio)
  - [`SI_TKILL`](#si-tkill)
  - [`SI_DETHREAD`](#si-dethread)
  - [`SI_ASYNCNL`](#si-asyncnl)
  - [`ILL_ILLOPC`](#ill-illopc)
  - [`ILL_ILLOPN`](#ill-illopn)
  - [`ILL_ILLADR`](#ill-illadr)
  - [`ILL_ILLTRP`](#ill-illtrp)
  - [`ILL_PRVOPC`](#ill-prvopc)
  - [`ILL_PRVREG`](#ill-prvreg)
  - [`ILL_COPROC`](#ill-coproc)
  - [`ILL_BADSTK`](#ill-badstk)
  - [`ILL_BADIADDR`](#ill-badiaddr)
  - [`__ILL_BREAK`](#ill-break)
  - [`__ILL_BNDMOD`](#ill-bndmod)
  - [`NSIGILL`](#nsigill)
  - [`FPE_INTDIV`](#fpe-intdiv)
  - [`FPE_INTOVF`](#fpe-intovf)
  - [`FPE_FLTDIV`](#fpe-fltdiv)
  - [`FPE_FLTOVF`](#fpe-fltovf)
  - [`FPE_FLTUND`](#fpe-fltund)
  - [`FPE_FLTRES`](#fpe-fltres)
  - [`FPE_FLTINV`](#fpe-fltinv)
  - [`FPE_FLTSUB`](#fpe-fltsub)
  - [`__FPE_DECOVF`](#fpe-decovf)
  - [`__FPE_DECDIV`](#fpe-decdiv)
  - [`__FPE_DECERR`](#fpe-decerr)
  - [`__FPE_INVASC`](#fpe-invasc)
  - [`__FPE_INVDEC`](#fpe-invdec)
  - [`FPE_FLTUNK`](#fpe-fltunk)
  - [`FPE_CONDTRAP`](#fpe-condtrap)
  - [`NSIGFPE`](#nsigfpe)
  - [`SEGV_MAPERR`](#segv-maperr)
  - [`SEGV_ACCERR`](#segv-accerr)
  - [`SEGV_BNDERR`](#segv-bnderr)
  - [`SEGV_PKUERR`](#segv-pkuerr)
  - [`SEGV_ACCADI`](#segv-accadi)
  - [`SEGV_ADIDERR`](#segv-adiderr)
  - [`SEGV_ADIPERR`](#segv-adiperr)
  - [`SEGV_MTEAERR`](#segv-mteaerr)
  - [`SEGV_MTESERR`](#segv-mteserr)
  - [`SEGV_CPERR`](#segv-cperr)
  - [`NSIGSEGV`](#nsigsegv)
  - [`BUS_ADRALN`](#bus-adraln)
  - [`BUS_ADRERR`](#bus-adrerr)
  - [`BUS_OBJERR`](#bus-objerr)
  - [`BUS_MCEERR_AR`](#bus-mceerr-ar)
  - [`BUS_MCEERR_AO`](#bus-mceerr-ao)
  - [`NSIGBUS`](#nsigbus)
  - [`TRAP_BRKPT`](#trap-brkpt)
  - [`TRAP_TRACE`](#trap-trace)
  - [`TRAP_BRANCH`](#trap-branch)
  - [`TRAP_HWBKPT`](#trap-hwbkpt)
  - [`TRAP_UNK`](#trap-unk)
  - [`TRAP_PERF`](#trap-perf)
  - [`NSIGTRAP`](#nsigtrap)
  - [`TRAP_PERF_FLAG_ASYNC`](#trap-perf-flag-async)
  - [`CLD_EXITED`](#cld-exited)
  - [`CLD_KILLED`](#cld-killed)
  - [`CLD_DUMPED`](#cld-dumped)
  - [`CLD_TRAPPED`](#cld-trapped)
  - [`CLD_STOPPED`](#cld-stopped)
  - [`CLD_CONTINUED`](#cld-continued)
  - [`NSIGCHLD`](#nsigchld)
  - [`POLL_IN`](#poll-in)
  - [`POLL_OUT`](#poll-out)
  - [`POLL_MSG`](#poll-msg)
  - [`POLL_ERR`](#poll-err)
  - [`POLL_PRI`](#poll-pri)
  - [`POLL_HUP`](#poll-hup)
  - [`NSIGPOLL`](#nsigpoll)
  - [`SYS_SECCOMP`](#sys-seccomp)
  - [`SYS_USER_DISPATCH`](#sys-user-dispatch)
  - [`NSIGSYS`](#nsigsys)
  - [`EMT_TAGOVF`](#emt-tagovf)
  - [`NSIGEMT`](#nsigemt)
  - [`SIGEV_SIGNAL`](#sigev-signal)
  - [`SIGEV_NONE`](#sigev-none)
  - [`SIGEV_THREAD`](#sigev-thread)
  - [`SIGEV_THREAD_ID`](#sigev-thread-id)
  - [`SIGEV_MAX_SIZE`](#sigev-max-size)
  - [`SS_ONSTACK`](#ss-onstack)
  - [`SS_DISABLE`](#ss-disable)
  - [`SS_AUTODISARM`](#ss-autodisarm)
  - [`SS_FLAG_BITS`](#ss-flag-bits)
  - [`S_IFMT`](#s-ifmt)
  - [`S_IFSOCK`](#s-ifsock)
  - [`S_IFLNK`](#s-iflnk)
  - [`S_IFREG`](#s-ifreg)
  - [`S_IFBLK`](#s-ifblk)
  - [`S_IFDIR`](#s-ifdir)
  - [`S_IFCHR`](#s-ifchr)
  - [`S_IFIFO`](#s-ififo)
  - [`S_ISUID`](#s-isuid)
  - [`S_ISGID`](#s-isgid)
  - [`S_ISVTX`](#s-isvtx)
  - [`S_IRWXU`](#s-irwxu)
  - [`S_IRUSR`](#s-irusr)
  - [`S_IWUSR`](#s-iwusr)
  - [`S_IXUSR`](#s-ixusr)
  - [`S_IRWXG`](#s-irwxg)
  - [`S_IRGRP`](#s-irgrp)
  - [`S_IWGRP`](#s-iwgrp)
  - [`S_IXGRP`](#s-ixgrp)
  - [`S_IRWXO`](#s-irwxo)
  - [`S_IROTH`](#s-iroth)
  - [`S_IWOTH`](#s-iwoth)
  - [`S_IXOTH`](#s-ixoth)
  - [`STATX_TYPE`](#statx-type)
  - [`STATX_MODE`](#statx-mode)
  - [`STATX_NLINK`](#statx-nlink)
  - [`STATX_UID`](#statx-uid)
  - [`STATX_GID`](#statx-gid)
  - [`STATX_ATIME`](#statx-atime)
  - [`STATX_MTIME`](#statx-mtime)
  - [`STATX_CTIME`](#statx-ctime)
  - [`STATX_INO`](#statx-ino)
  - [`STATX_SIZE`](#statx-size)
  - [`STATX_BLOCKS`](#statx-blocks)
  - [`STATX_BASIC_STATS`](#statx-basic-stats)
  - [`STATX_BTIME`](#statx-btime)
  - [`STATX_MNT_ID`](#statx-mnt-id)
  - [`STATX_DIOALIGN`](#statx-dioalign)
  - [`STATX_MNT_ID_UNIQUE`](#statx-mnt-id-unique)
  - [`STATX_SUBVOL`](#statx-subvol)
  - [`STATX_WRITE_ATOMIC`](#statx-write-atomic)
  - [`STATX_DIO_READ_ALIGN`](#statx-dio-read-align)
  - [`STATX__RESERVED`](#statx-reserved)
  - [`STATX_ALL`](#statx-all)
  - [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable)
  - [`STATX_ATTR_APPEND`](#statx-attr-append)
  - [`STATX_ATTR_NODUMP`](#statx-attr-nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root)
  - [`STATX_ATTR_VERITY`](#statx-attr-verity)
  - [`STATX_ATTR_DAX`](#statx-attr-dax)
  - [`STATX_ATTR_WRITE_ATOMIC`](#statx-attr-write-atomic)
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
  - [`TIOCPKT_DATA`](#tiocpkt-data)
  - [`TIOCPKT_FLUSHREAD`](#tiocpkt-flushread)
  - [`TIOCPKT_FLUSHWRITE`](#tiocpkt-flushwrite)
  - [`TIOCPKT_STOP`](#tiocpkt-stop)
  - [`TIOCPKT_START`](#tiocpkt-start)
  - [`TIOCPKT_NOSTOP`](#tiocpkt-nostop)
  - [`TIOCPKT_DOSTOP`](#tiocpkt-dostop)
  - [`TIOCPKT_IOCTL`](#tiocpkt-ioctl)
  - [`TIOCSER_TEMT`](#tiocser-temt)
  - [`NCC`](#ncc)
  - [`TIOCM_LE`](#tiocm-le)
  - [`TIOCM_DTR`](#tiocm-dtr)
  - [`TIOCM_RTS`](#tiocm-rts)
  - [`TIOCM_ST`](#tiocm-st)
  - [`TIOCM_SR`](#tiocm-sr)
  - [`TIOCM_CTS`](#tiocm-cts)
  - [`TIOCM_CAR`](#tiocm-car)
  - [`TIOCM_RNG`](#tiocm-rng)
  - [`TIOCM_DSR`](#tiocm-dsr)
  - [`TIOCM_CD`](#tiocm-cd)
  - [`TIOCM_RI`](#tiocm-ri)
  - [`TIOCM_OUT1`](#tiocm-out1)
  - [`TIOCM_OUT2`](#tiocm-out2)
  - [`TIOCM_LOOP`](#tiocm-loop)
  - [`ITIMER_REAL`](#itimer-real)
  - [`ITIMER_VIRTUAL`](#itimer-virtual)
  - [`ITIMER_PROF`](#itimer-prof)
  - [`CLOCK_REALTIME`](#clock-realtime)
  - [`CLOCK_MONOTONIC`](#clock-monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id)
  - [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw)
  - [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse)
  - [`CLOCK_BOOTTIME`](#clock-boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm)
  - [`CLOCK_SGI_CYCLE`](#clock-sgi-cycle)
  - [`CLOCK_TAI`](#clock-tai)
  - [`MAX_CLOCKS`](#max-clocks)
  - [`CLOCKS_MASK`](#clocks-mask)
  - [`CLOCKS_MONO`](#clocks-mono)
  - [`TIMER_ABSTIME`](#timer-abstime)
  - [`UIO_FASTIOV`](#uio-fastiov)
  - [`UIO_MAXIOV`](#uio-maxiov)
  - [`__X32_SYSCALL_BIT`](#x32-syscall-bit)
  - [`__NR_read`](#nr-read)
  - [`__NR_write`](#nr-write)
  - [`__NR_open`](#nr-open)
  - [`__NR_close`](#nr-close)
  - [`__NR_stat`](#nr-stat)
  - [`__NR_fstat`](#nr-fstat)
  - [`__NR_lstat`](#nr-lstat)
  - [`__NR_poll`](#nr-poll)
  - [`__NR_lseek`](#nr-lseek)
  - [`__NR_mmap`](#nr-mmap)
  - [`__NR_mprotect`](#nr-mprotect)
  - [`__NR_munmap`](#nr-munmap)
  - [`__NR_brk`](#nr-brk)
  - [`__NR_rt_sigaction`](#nr-rt-sigaction)
  - [`__NR_rt_sigprocmask`](#nr-rt-sigprocmask)
  - [`__NR_rt_sigreturn`](#nr-rt-sigreturn)
  - [`__NR_ioctl`](#nr-ioctl)
  - [`__NR_pread64`](#nr-pread64)
  - [`__NR_pwrite64`](#nr-pwrite64)
  - [`__NR_readv`](#nr-readv)
  - [`__NR_writev`](#nr-writev)
  - [`__NR_access`](#nr-access)
  - [`__NR_pipe`](#nr-pipe)
  - [`__NR_select`](#nr-select)
  - [`__NR_sched_yield`](#nr-sched-yield)
  - [`__NR_mremap`](#nr-mremap)
  - [`__NR_msync`](#nr-msync)
  - [`__NR_mincore`](#nr-mincore)
  - [`__NR_madvise`](#nr-madvise)
  - [`__NR_shmget`](#nr-shmget)
  - [`__NR_shmat`](#nr-shmat)
  - [`__NR_shmctl`](#nr-shmctl)
  - [`__NR_dup`](#nr-dup)
  - [`__NR_dup2`](#nr-dup2)
  - [`__NR_pause`](#nr-pause)
  - [`__NR_nanosleep`](#nr-nanosleep)
  - [`__NR_getitimer`](#nr-getitimer)
  - [`__NR_alarm`](#nr-alarm)
  - [`__NR_setitimer`](#nr-setitimer)
  - [`__NR_getpid`](#nr-getpid)
  - [`__NR_sendfile`](#nr-sendfile)
  - [`__NR_socket`](#nr-socket)
  - [`__NR_connect`](#nr-connect)
  - [`__NR_accept`](#nr-accept)
  - [`__NR_sendto`](#nr-sendto)
  - [`__NR_recvfrom`](#nr-recvfrom)
  - [`__NR_sendmsg`](#nr-sendmsg)
  - [`__NR_recvmsg`](#nr-recvmsg)
  - [`__NR_shutdown`](#nr-shutdown)
  - [`__NR_bind`](#nr-bind)
  - [`__NR_listen`](#nr-listen)
  - [`__NR_getsockname`](#nr-getsockname)
  - [`__NR_getpeername`](#nr-getpeername)
  - [`__NR_socketpair`](#nr-socketpair)
  - [`__NR_setsockopt`](#nr-setsockopt)
  - [`__NR_getsockopt`](#nr-getsockopt)
  - [`__NR_clone`](#nr-clone)
  - [`__NR_fork`](#nr-fork)
  - [`__NR_vfork`](#nr-vfork)
  - [`__NR_execve`](#nr-execve)
  - [`__NR_exit`](#nr-exit)
  - [`__NR_wait4`](#nr-wait4)
  - [`__NR_kill`](#nr-kill)
  - [`__NR_uname`](#nr-uname)
  - [`__NR_semget`](#nr-semget)
  - [`__NR_semop`](#nr-semop)
  - [`__NR_semctl`](#nr-semctl)
  - [`__NR_shmdt`](#nr-shmdt)
  - [`__NR_msgget`](#nr-msgget)
  - [`__NR_msgsnd`](#nr-msgsnd)
  - [`__NR_msgrcv`](#nr-msgrcv)
  - [`__NR_msgctl`](#nr-msgctl)
  - [`__NR_fcntl`](#nr-fcntl)
  - [`__NR_flock`](#nr-flock)
  - [`__NR_fsync`](#nr-fsync)
  - [`__NR_fdatasync`](#nr-fdatasync)
  - [`__NR_truncate`](#nr-truncate)
  - [`__NR_ftruncate`](#nr-ftruncate)
  - [`__NR_getdents`](#nr-getdents)
  - [`__NR_getcwd`](#nr-getcwd)
  - [`__NR_chdir`](#nr-chdir)
  - [`__NR_fchdir`](#nr-fchdir)
  - [`__NR_rename`](#nr-rename)
  - [`__NR_mkdir`](#nr-mkdir)
  - [`__NR_rmdir`](#nr-rmdir)
  - [`__NR_creat`](#nr-creat)
  - [`__NR_link`](#nr-link)
  - [`__NR_unlink`](#nr-unlink)
  - [`__NR_symlink`](#nr-symlink)
  - [`__NR_readlink`](#nr-readlink)
  - [`__NR_chmod`](#nr-chmod)
  - [`__NR_fchmod`](#nr-fchmod)
  - [`__NR_chown`](#nr-chown)
  - [`__NR_fchown`](#nr-fchown)
  - [`__NR_lchown`](#nr-lchown)
  - [`__NR_umask`](#nr-umask)
  - [`__NR_gettimeofday`](#nr-gettimeofday)
  - [`__NR_getrlimit`](#nr-getrlimit)
  - [`__NR_getrusage`](#nr-getrusage)
  - [`__NR_sysinfo`](#nr-sysinfo)
  - [`__NR_times`](#nr-times)
  - [`__NR_ptrace`](#nr-ptrace)
  - [`__NR_getuid`](#nr-getuid)
  - [`__NR_syslog`](#nr-syslog)
  - [`__NR_getgid`](#nr-getgid)
  - [`__NR_setuid`](#nr-setuid)
  - [`__NR_setgid`](#nr-setgid)
  - [`__NR_geteuid`](#nr-geteuid)
  - [`__NR_getegid`](#nr-getegid)
  - [`__NR_setpgid`](#nr-setpgid)
  - [`__NR_getppid`](#nr-getppid)
  - [`__NR_getpgrp`](#nr-getpgrp)
  - [`__NR_setsid`](#nr-setsid)
  - [`__NR_setreuid`](#nr-setreuid)
  - [`__NR_setregid`](#nr-setregid)
  - [`__NR_getgroups`](#nr-getgroups)
  - [`__NR_setgroups`](#nr-setgroups)
  - [`__NR_setresuid`](#nr-setresuid)
  - [`__NR_getresuid`](#nr-getresuid)
  - [`__NR_setresgid`](#nr-setresgid)
  - [`__NR_getresgid`](#nr-getresgid)
  - [`__NR_getpgid`](#nr-getpgid)
  - [`__NR_setfsuid`](#nr-setfsuid)
  - [`__NR_setfsgid`](#nr-setfsgid)
  - [`__NR_getsid`](#nr-getsid)
  - [`__NR_capget`](#nr-capget)
  - [`__NR_capset`](#nr-capset)
  - [`__NR_rt_sigpending`](#nr-rt-sigpending)
  - [`__NR_rt_sigtimedwait`](#nr-rt-sigtimedwait)
  - [`__NR_rt_sigqueueinfo`](#nr-rt-sigqueueinfo)
  - [`__NR_rt_sigsuspend`](#nr-rt-sigsuspend)
  - [`__NR_sigaltstack`](#nr-sigaltstack)
  - [`__NR_utime`](#nr-utime)
  - [`__NR_mknod`](#nr-mknod)
  - [`__NR_uselib`](#nr-uselib)
  - [`__NR_personality`](#nr-personality)
  - [`__NR_ustat`](#nr-ustat)
  - [`__NR_statfs`](#nr-statfs)
  - [`__NR_fstatfs`](#nr-fstatfs)
  - [`__NR_sysfs`](#nr-sysfs)
  - [`__NR_getpriority`](#nr-getpriority)
  - [`__NR_setpriority`](#nr-setpriority)
  - [`__NR_sched_setparam`](#nr-sched-setparam)
  - [`__NR_sched_getparam`](#nr-sched-getparam)
  - [`__NR_sched_setscheduler`](#nr-sched-setscheduler)
  - [`__NR_sched_getscheduler`](#nr-sched-getscheduler)
  - [`__NR_sched_get_priority_max`](#nr-sched-get-priority-max)
  - [`__NR_sched_get_priority_min`](#nr-sched-get-priority-min)
  - [`__NR_sched_rr_get_interval`](#nr-sched-rr-get-interval)
  - [`__NR_mlock`](#nr-mlock)
  - [`__NR_munlock`](#nr-munlock)
  - [`__NR_mlockall`](#nr-mlockall)
  - [`__NR_munlockall`](#nr-munlockall)
  - [`__NR_vhangup`](#nr-vhangup)
  - [`__NR_modify_ldt`](#nr-modify-ldt)
  - [`__NR_pivot_root`](#nr-pivot-root)
  - [`__NR__sysctl`](#nr-sysctl)
  - [`__NR_prctl`](#nr-prctl)
  - [`__NR_arch_prctl`](#nr-arch-prctl)
  - [`__NR_adjtimex`](#nr-adjtimex)
  - [`__NR_setrlimit`](#nr-setrlimit)
  - [`__NR_chroot`](#nr-chroot)
  - [`__NR_sync`](#nr-sync)
  - [`__NR_acct`](#nr-acct)
  - [`__NR_settimeofday`](#nr-settimeofday)
  - [`__NR_mount`](#nr-mount)
  - [`__NR_umount2`](#nr-umount2)
  - [`__NR_swapon`](#nr-swapon)
  - [`__NR_swapoff`](#nr-swapoff)
  - [`__NR_reboot`](#nr-reboot)
  - [`__NR_sethostname`](#nr-sethostname)
  - [`__NR_setdomainname`](#nr-setdomainname)
  - [`__NR_iopl`](#nr-iopl)
  - [`__NR_ioperm`](#nr-ioperm)
  - [`__NR_create_module`](#nr-create-module)
  - [`__NR_init_module`](#nr-init-module)
  - [`__NR_delete_module`](#nr-delete-module)
  - [`__NR_get_kernel_syms`](#nr-get-kernel-syms)
  - [`__NR_query_module`](#nr-query-module)
  - [`__NR_quotactl`](#nr-quotactl)
  - [`__NR_nfsservctl`](#nr-nfsservctl)
  - [`__NR_getpmsg`](#nr-getpmsg)
  - [`__NR_putpmsg`](#nr-putpmsg)
  - [`__NR_afs_syscall`](#nr-afs-syscall)
  - [`__NR_tuxcall`](#nr-tuxcall)
  - [`__NR_security`](#nr-security)
  - [`__NR_gettid`](#nr-gettid)
  - [`__NR_readahead`](#nr-readahead)
  - [`__NR_setxattr`](#nr-setxattr)
  - [`__NR_lsetxattr`](#nr-lsetxattr)
  - [`__NR_fsetxattr`](#nr-fsetxattr)
  - [`__NR_getxattr`](#nr-getxattr)
  - [`__NR_lgetxattr`](#nr-lgetxattr)
  - [`__NR_fgetxattr`](#nr-fgetxattr)
  - [`__NR_listxattr`](#nr-listxattr)
  - [`__NR_llistxattr`](#nr-llistxattr)
  - [`__NR_flistxattr`](#nr-flistxattr)
  - [`__NR_removexattr`](#nr-removexattr)
  - [`__NR_lremovexattr`](#nr-lremovexattr)
  - [`__NR_fremovexattr`](#nr-fremovexattr)
  - [`__NR_tkill`](#nr-tkill)
  - [`__NR_time`](#nr-time)
  - [`__NR_futex`](#nr-futex)
  - [`__NR_sched_setaffinity`](#nr-sched-setaffinity)
  - [`__NR_sched_getaffinity`](#nr-sched-getaffinity)
  - [`__NR_set_thread_area`](#nr-set-thread-area)
  - [`__NR_io_setup`](#nr-io-setup)
  - [`__NR_io_destroy`](#nr-io-destroy)
  - [`__NR_io_getevents`](#nr-io-getevents)
  - [`__NR_io_submit`](#nr-io-submit)
  - [`__NR_io_cancel`](#nr-io-cancel)
  - [`__NR_get_thread_area`](#nr-get-thread-area)
  - [`__NR_lookup_dcookie`](#nr-lookup-dcookie)
  - [`__NR_epoll_create`](#nr-epoll-create)
  - [`__NR_epoll_ctl_old`](#nr-epoll-ctl-old)
  - [`__NR_epoll_wait_old`](#nr-epoll-wait-old)
  - [`__NR_remap_file_pages`](#nr-remap-file-pages)
  - [`__NR_getdents64`](#nr-getdents64)
  - [`__NR_set_tid_address`](#nr-set-tid-address)
  - [`__NR_restart_syscall`](#nr-restart-syscall)
  - [`__NR_semtimedop`](#nr-semtimedop)
  - [`__NR_fadvise64`](#nr-fadvise64)
  - [`__NR_timer_create`](#nr-timer-create)
  - [`__NR_timer_settime`](#nr-timer-settime)
  - [`__NR_timer_gettime`](#nr-timer-gettime)
  - [`__NR_timer_getoverrun`](#nr-timer-getoverrun)
  - [`__NR_timer_delete`](#nr-timer-delete)
  - [`__NR_clock_settime`](#nr-clock-settime)
  - [`__NR_clock_gettime`](#nr-clock-gettime)
  - [`__NR_clock_getres`](#nr-clock-getres)
  - [`__NR_clock_nanosleep`](#nr-clock-nanosleep)
  - [`__NR_exit_group`](#nr-exit-group)
  - [`__NR_epoll_wait`](#nr-epoll-wait)
  - [`__NR_epoll_ctl`](#nr-epoll-ctl)
  - [`__NR_tgkill`](#nr-tgkill)
  - [`__NR_utimes`](#nr-utimes)
  - [`__NR_vserver`](#nr-vserver)
  - [`__NR_mbind`](#nr-mbind)
  - [`__NR_set_mempolicy`](#nr-set-mempolicy)
  - [`__NR_get_mempolicy`](#nr-get-mempolicy)
  - [`__NR_mq_open`](#nr-mq-open)
  - [`__NR_mq_unlink`](#nr-mq-unlink)
  - [`__NR_mq_timedsend`](#nr-mq-timedsend)
  - [`__NR_mq_timedreceive`](#nr-mq-timedreceive)
  - [`__NR_mq_notify`](#nr-mq-notify)
  - [`__NR_mq_getsetattr`](#nr-mq-getsetattr)
  - [`__NR_kexec_load`](#nr-kexec-load)
  - [`__NR_waitid`](#nr-waitid)
  - [`__NR_add_key`](#nr-add-key)
  - [`__NR_request_key`](#nr-request-key)
  - [`__NR_keyctl`](#nr-keyctl)
  - [`__NR_ioprio_set`](#nr-ioprio-set)
  - [`__NR_ioprio_get`](#nr-ioprio-get)
  - [`__NR_inotify_init`](#nr-inotify-init)
  - [`__NR_inotify_add_watch`](#nr-inotify-add-watch)
  - [`__NR_inotify_rm_watch`](#nr-inotify-rm-watch)
  - [`__NR_migrate_pages`](#nr-migrate-pages)
  - [`__NR_openat`](#nr-openat)
  - [`__NR_mkdirat`](#nr-mkdirat)
  - [`__NR_mknodat`](#nr-mknodat)
  - [`__NR_fchownat`](#nr-fchownat)
  - [`__NR_futimesat`](#nr-futimesat)
  - [`__NR_newfstatat`](#nr-newfstatat)
  - [`__NR_unlinkat`](#nr-unlinkat)
  - [`__NR_renameat`](#nr-renameat)
  - [`__NR_linkat`](#nr-linkat)
  - [`__NR_symlinkat`](#nr-symlinkat)
  - [`__NR_readlinkat`](#nr-readlinkat)
  - [`__NR_fchmodat`](#nr-fchmodat)
  - [`__NR_faccessat`](#nr-faccessat)
  - [`__NR_pselect6`](#nr-pselect6)
  - [`__NR_ppoll`](#nr-ppoll)
  - [`__NR_unshare`](#nr-unshare)
  - [`__NR_set_robust_list`](#nr-set-robust-list)
  - [`__NR_get_robust_list`](#nr-get-robust-list)
  - [`__NR_splice`](#nr-splice)
  - [`__NR_tee`](#nr-tee)
  - [`__NR_sync_file_range`](#nr-sync-file-range)
  - [`__NR_vmsplice`](#nr-vmsplice)
  - [`__NR_move_pages`](#nr-move-pages)
  - [`__NR_utimensat`](#nr-utimensat)
  - [`__NR_epoll_pwait`](#nr-epoll-pwait)
  - [`__NR_signalfd`](#nr-signalfd)
  - [`__NR_timerfd_create`](#nr-timerfd-create)
  - [`__NR_eventfd`](#nr-eventfd)
  - [`__NR_fallocate`](#nr-fallocate)
  - [`__NR_timerfd_settime`](#nr-timerfd-settime)
  - [`__NR_timerfd_gettime`](#nr-timerfd-gettime)
  - [`__NR_accept4`](#nr-accept4)
  - [`__NR_signalfd4`](#nr-signalfd4)
  - [`__NR_eventfd2`](#nr-eventfd2)
  - [`__NR_epoll_create1`](#nr-epoll-create1)
  - [`__NR_dup3`](#nr-dup3)
  - [`__NR_pipe2`](#nr-pipe2)
  - [`__NR_inotify_init1`](#nr-inotify-init1)
  - [`__NR_preadv`](#nr-preadv)
  - [`__NR_pwritev`](#nr-pwritev)
  - [`__NR_rt_tgsigqueueinfo`](#nr-rt-tgsigqueueinfo)
  - [`__NR_perf_event_open`](#nr-perf-event-open)
  - [`__NR_recvmmsg`](#nr-recvmmsg)
  - [`__NR_fanotify_init`](#nr-fanotify-init)
  - [`__NR_fanotify_mark`](#nr-fanotify-mark)
  - [`__NR_prlimit64`](#nr-prlimit64)
  - [`__NR_name_to_handle_at`](#nr-name-to-handle-at)
  - [`__NR_open_by_handle_at`](#nr-open-by-handle-at)
  - [`__NR_clock_adjtime`](#nr-clock-adjtime)
  - [`__NR_syncfs`](#nr-syncfs)
  - [`__NR_sendmmsg`](#nr-sendmmsg)
  - [`__NR_setns`](#nr-setns)
  - [`__NR_getcpu`](#nr-getcpu)
  - [`__NR_process_vm_readv`](#nr-process-vm-readv)
  - [`__NR_process_vm_writev`](#nr-process-vm-writev)
  - [`__NR_kcmp`](#nr-kcmp)
  - [`__NR_finit_module`](#nr-finit-module)
  - [`__NR_sched_setattr`](#nr-sched-setattr)
  - [`__NR_sched_getattr`](#nr-sched-getattr)
  - [`__NR_renameat2`](#nr-renameat2)
  - [`__NR_seccomp`](#nr-seccomp)
  - [`__NR_getrandom`](#nr-getrandom)
  - [`__NR_memfd_create`](#nr-memfd-create)
  - [`__NR_kexec_file_load`](#nr-kexec-file-load)
  - [`__NR_bpf`](#nr-bpf)
  - [`__NR_execveat`](#nr-execveat)
  - [`__NR_userfaultfd`](#nr-userfaultfd)
  - [`__NR_membarrier`](#nr-membarrier)
  - [`__NR_mlock2`](#nr-mlock2)
  - [`__NR_copy_file_range`](#nr-copy-file-range)
  - [`__NR_preadv2`](#nr-preadv2)
  - [`__NR_pwritev2`](#nr-pwritev2)
  - [`__NR_pkey_mprotect`](#nr-pkey-mprotect)
  - [`__NR_pkey_alloc`](#nr-pkey-alloc)
  - [`__NR_pkey_free`](#nr-pkey-free)
  - [`__NR_statx`](#nr-statx)
  - [`__NR_io_pgetevents`](#nr-io-pgetevents)
  - [`__NR_rseq`](#nr-rseq)
  - [`__NR_uretprobe`](#nr-uretprobe)
  - [`__NR_pidfd_send_signal`](#nr-pidfd-send-signal)
  - [`__NR_io_uring_setup`](#nr-io-uring-setup)
  - [`__NR_io_uring_enter`](#nr-io-uring-enter)
  - [`__NR_io_uring_register`](#nr-io-uring-register)
  - [`__NR_open_tree`](#nr-open-tree)
  - [`__NR_move_mount`](#nr-move-mount)
  - [`__NR_fsopen`](#nr-fsopen)
  - [`__NR_fsconfig`](#nr-fsconfig)
  - [`__NR_fsmount`](#nr-fsmount)
  - [`__NR_fspick`](#nr-fspick)
  - [`__NR_pidfd_open`](#nr-pidfd-open)
  - [`__NR_clone3`](#nr-clone3)
  - [`__NR_close_range`](#nr-close-range)
  - [`__NR_openat2`](#nr-openat2)
  - [`__NR_pidfd_getfd`](#nr-pidfd-getfd)
  - [`__NR_faccessat2`](#nr-faccessat2)
  - [`__NR_process_madvise`](#nr-process-madvise)
  - [`__NR_epoll_pwait2`](#nr-epoll-pwait2)
  - [`__NR_mount_setattr`](#nr-mount-setattr)
  - [`__NR_quotactl_fd`](#nr-quotactl-fd)
  - [`__NR_landlock_create_ruleset`](#nr-landlock-create-ruleset)
  - [`__NR_landlock_add_rule`](#nr-landlock-add-rule)
  - [`__NR_landlock_restrict_self`](#nr-landlock-restrict-self)
  - [`__NR_memfd_secret`](#nr-memfd-secret)
  - [`__NR_process_mrelease`](#nr-process-mrelease)
  - [`__NR_futex_waitv`](#nr-futex-waitv)
  - [`__NR_set_mempolicy_home_node`](#nr-set-mempolicy-home-node)
  - [`__NR_cachestat`](#nr-cachestat)
  - [`__NR_fchmodat2`](#nr-fchmodat2)
  - [`__NR_map_shadow_stack`](#nr-map-shadow-stack)
  - [`__NR_futex_wake`](#nr-futex-wake)
  - [`__NR_futex_wait`](#nr-futex-wait)
  - [`__NR_futex_requeue`](#nr-futex-requeue)
  - [`__NR_statmount`](#nr-statmount)
  - [`__NR_listmount`](#nr-listmount)
  - [`__NR_lsm_get_self_attr`](#nr-lsm-get-self-attr)
  - [`__NR_lsm_set_self_attr`](#nr-lsm-set-self-attr)
  - [`__NR_lsm_list_modules`](#nr-lsm-list-modules)
  - [`__NR_mseal`](#nr-mseal)
  - [`__NR_setxattrat`](#nr-setxattrat)
  - [`__NR_getxattrat`](#nr-getxattrat)
  - [`__NR_listxattrat`](#nr-listxattrat)
  - [`__NR_removexattrat`](#nr-removexattrat)
  - [`__NR_open_tree_attr`](#nr-open-tree-attr)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`__WNOTHREAD`](#wnothread)
  - [`__WALL`](#wall)
  - [`__WCLONE`](#wclone)
  - [`P_ALL`](#p-all)
  - [`P_PID`](#p-pid)
  - [`P_PGID`](#p-pgid)
  - [`P_PIDFD`](#p-pidfd)
  - [`XATTR_CREATE`](#xattr-create)
  - [`XATTR_REPLACE`](#xattr-replace)
  - [`XATTR_OS2_PREFIX`](#xattr-os2-prefix)
  - [`XATTR_MAC_OSX_PREFIX`](#xattr-mac-osx-prefix)
  - [`XATTR_BTRFS_PREFIX`](#xattr-btrfs-prefix)
  - [`XATTR_HURD_PREFIX`](#xattr-hurd-prefix)
  - [`XATTR_SECURITY_PREFIX`](#xattr-security-prefix)
  - [`XATTR_SYSTEM_PREFIX`](#xattr-system-prefix)
  - [`XATTR_TRUSTED_PREFIX`](#xattr-trusted-prefix)
  - [`XATTR_USER_PREFIX`](#xattr-user-prefix)
  - [`XATTR_EVM_SUFFIX`](#xattr-evm-suffix)
  - [`XATTR_NAME_EVM`](#xattr-name-evm)
  - [`XATTR_IMA_SUFFIX`](#xattr-ima-suffix)
  - [`XATTR_NAME_IMA`](#xattr-name-ima)
  - [`XATTR_SELINUX_SUFFIX`](#xattr-selinux-suffix)
  - [`XATTR_NAME_SELINUX`](#xattr-name-selinux)
  - [`XATTR_SMACK_SUFFIX`](#xattr-smack-suffix)
  - [`XATTR_SMACK_IPIN`](#xattr-smack-ipin)
  - [`XATTR_SMACK_IPOUT`](#xattr-smack-ipout)
  - [`XATTR_SMACK_EXEC`](#xattr-smack-exec)
  - [`XATTR_SMACK_TRANSMUTE`](#xattr-smack-transmute)
  - [`XATTR_SMACK_MMAP`](#xattr-smack-mmap)
  - [`XATTR_NAME_SMACK`](#xattr-name-smack)
  - [`XATTR_NAME_SMACKIPIN`](#xattr-name-smackipin)
  - [`XATTR_NAME_SMACKIPOUT`](#xattr-name-smackipout)
  - [`XATTR_NAME_SMACKEXEC`](#xattr-name-smackexec)
  - [`XATTR_NAME_SMACKTRANSMUTE`](#xattr-name-smacktransmute)
  - [`XATTR_NAME_SMACKMMAP`](#xattr-name-smackmmap)
  - [`XATTR_APPARMOR_SUFFIX`](#xattr-apparmor-suffix)
  - [`XATTR_NAME_APPARMOR`](#xattr-name-apparmor)
  - [`XATTR_CAPS_SUFFIX`](#xattr-caps-suffix)
  - [`XATTR_NAME_CAPS`](#xattr-name-caps)
  - [`XATTR_BPF_LSM_SUFFIX`](#xattr-bpf-lsm-suffix)
  - [`XATTR_NAME_BPF_LSM`](#xattr-name-bpf-lsm)
  - [`XATTR_POSIX_ACL_ACCESS`](#xattr-posix-acl-access)
  - [`XATTR_NAME_POSIX_ACL_ACCESS`](#xattr-name-posix-acl-access)
  - [`XATTR_POSIX_ACL_DEFAULT`](#xattr-posix-acl-default)
  - [`XATTR_NAME_POSIX_ACL_DEFAULT`](#xattr-name-posix-acl-default)
  - [`MFD_CLOEXEC`](#mfd-cloexec)
  - [`MFD_ALLOW_SEALING`](#mfd-allow-sealing)
  - [`MFD_HUGETLB`](#mfd-hugetlb)
  - [`MFD_NOEXEC_SEAL`](#mfd-noexec-seal)
  - [`MFD_EXEC`](#mfd-exec)
  - [`MFD_HUGE_SHIFT`](#mfd-huge-shift)
  - [`MFD_HUGE_MASK`](#mfd-huge-mask)
  - [`MFD_HUGE_64KB`](#mfd-huge-64kb)
  - [`MFD_HUGE_512KB`](#mfd-huge-512kb)
  - [`MFD_HUGE_1MB`](#mfd-huge-1mb)
  - [`MFD_HUGE_2MB`](#mfd-huge-2mb)
  - [`MFD_HUGE_8MB`](#mfd-huge-8mb)
  - [`MFD_HUGE_16MB`](#mfd-huge-16mb)
  - [`MFD_HUGE_32MB`](#mfd-huge-32mb)
  - [`MFD_HUGE_256MB`](#mfd-huge-256mb)
  - [`MFD_HUGE_512MB`](#mfd-huge-512mb)
  - [`MFD_HUGE_1GB`](#mfd-huge-1gb)
  - [`MFD_HUGE_2GB`](#mfd-huge-2gb)
  - [`MFD_HUGE_16GB`](#mfd-huge-16gb)
  - [`TFD_TIMER_ABSTIME`](#tfd-timer-abstime)
  - [`TFD_TIMER_CANCEL_ON_SET`](#tfd-timer-cancel-on-set)
  - [`TFD_CLOEXEC`](#tfd-cloexec)
  - [`TFD_NONBLOCK`](#tfd-nonblock)
  - [`USERFAULTFD_IOC`](#userfaultfd-ioc)
  - [`_UFFDIO_REGISTER`](#uffdio-register)
  - [`_UFFDIO_UNREGISTER`](#uffdio-unregister)
  - [`_UFFDIO_WAKE`](#uffdio-wake)
  - [`_UFFDIO_COPY`](#uffdio-copy)
  - [`_UFFDIO_ZEROPAGE`](#uffdio-zeropage)
  - [`_UFFDIO_MOVE`](#uffdio-move)
  - [`_UFFDIO_WRITEPROTECT`](#uffdio-writeprotect)
  - [`_UFFDIO_CONTINUE`](#uffdio-continue)
  - [`_UFFDIO_POISON`](#uffdio-poison)
  - [`_UFFDIO_API`](#uffdio-api)
  - [`UFFDIO`](#uffdio)
  - [`UFFD_EVENT_PAGEFAULT`](#uffd-event-pagefault)
  - [`UFFD_EVENT_FORK`](#uffd-event-fork)
  - [`UFFD_EVENT_REMAP`](#uffd-event-remap)
  - [`UFFD_EVENT_REMOVE`](#uffd-event-remove)
  - [`UFFD_EVENT_UNMAP`](#uffd-event-unmap)
  - [`UFFD_PAGEFAULT_FLAG_WRITE`](#uffd-pagefault-flag-write)
  - [`UFFD_PAGEFAULT_FLAG_WP`](#uffd-pagefault-flag-wp)
  - [`UFFD_PAGEFAULT_FLAG_MINOR`](#uffd-pagefault-flag-minor)
  - [`UFFD_FEATURE_PAGEFAULT_FLAG_WP`](#uffd-feature-pagefault-flag-wp)
  - [`UFFD_FEATURE_EVENT_FORK`](#uffd-feature-event-fork)
  - [`UFFD_FEATURE_EVENT_REMAP`](#uffd-feature-event-remap)
  - [`UFFD_FEATURE_EVENT_REMOVE`](#uffd-feature-event-remove)
  - [`UFFD_FEATURE_MISSING_HUGETLBFS`](#uffd-feature-missing-hugetlbfs)
  - [`UFFD_FEATURE_MISSING_SHMEM`](#uffd-feature-missing-shmem)
  - [`UFFD_FEATURE_EVENT_UNMAP`](#uffd-feature-event-unmap)
  - [`UFFD_FEATURE_SIGBUS`](#uffd-feature-sigbus)
  - [`UFFD_FEATURE_THREAD_ID`](#uffd-feature-thread-id)
  - [`UFFD_FEATURE_MINOR_HUGETLBFS`](#uffd-feature-minor-hugetlbfs)
  - [`UFFD_FEATURE_MINOR_SHMEM`](#uffd-feature-minor-shmem)
  - [`UFFD_FEATURE_EXACT_ADDRESS`](#uffd-feature-exact-address)
  - [`UFFD_FEATURE_WP_HUGETLBFS_SHMEM`](#uffd-feature-wp-hugetlbfs-shmem)
  - [`UFFD_FEATURE_WP_UNPOPULATED`](#uffd-feature-wp-unpopulated)
  - [`UFFD_FEATURE_POISON`](#uffd-feature-poison)
  - [`UFFD_FEATURE_WP_ASYNC`](#uffd-feature-wp-async)
  - [`UFFD_FEATURE_MOVE`](#uffd-feature-move)
  - [`UFFD_USER_MODE_ONLY`](#uffd-user-mode-only)
  - [`DT_UNKNOWN`](#dt-unknown)
  - [`DT_FIFO`](#dt-fifo)
  - [`DT_CHR`](#dt-chr)
  - [`DT_DIR`](#dt-dir)
  - [`DT_BLK`](#dt-blk)
  - [`DT_REG`](#dt-reg)
  - [`DT_LNK`](#dt-lnk)
  - [`DT_SOCK`](#dt-sock)
  - [`STAT_HAVE_NSEC`](#stat-have-nsec)
  - [`F_OK`](#f-ok)
  - [`R_OK`](#r-ok)
  - [`W_OK`](#w-ok)
  - [`X_OK`](#x-ok)
  - [`UTIME_NOW`](#utime-now)
  - [`UTIME_OMIT`](#utime-omit)
  - [`MNT_FORCE`](#mnt-force)
  - [`MNT_DETACH`](#mnt-detach)
  - [`MNT_EXPIRE`](#mnt-expire)
  - [`UMOUNT_NOFOLLOW`](#umount-nofollow)
  - [`UMOUNT_UNUSED`](#umount-unused)
  - [`STDIN_FILENO`](#stdin-fileno)
  - [`STDOUT_FILENO`](#stdout-fileno)
  - [`STDERR_FILENO`](#stderr-fileno)
  - [`RWF_HIPRI`](#rwf-hipri)
  - [`RWF_DSYNC`](#rwf-dsync)
  - [`RWF_SYNC`](#rwf-sync)
  - [`RWF_NOWAIT`](#rwf-nowait)
  - [`RWF_APPEND`](#rwf-append)
  - [`EFD_SEMAPHORE`](#efd-semaphore)
  - [`EFD_CLOEXEC`](#efd-cloexec)
  - [`EFD_NONBLOCK`](#efd-nonblock)
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
  - [`TFD_SHARED_FCNTL_FLAGS`](#tfd-shared-fcntl-flags)
  - [`TFD_CREATE_FLAGS`](#tfd-create-flags)
  - [`TFD_SETTIME_FLAGS`](#tfd-settime-flags)
  - [`ARCH_SET_FS`](#arch-set-fs)
  - [`UFFD_API`](#uffd-api)
  - [`UFFDIO_REGISTER_MODE_MISSING`](#uffdio-register-mode-missing)
  - [`UFFDIO_REGISTER_MODE_WP`](#uffdio-register-mode-wp)
  - [`UFFDIO_REGISTER_MODE_MINOR`](#uffdio-register-mode-minor)
  - [`UFFDIO_COPY_MODE_DONTWAKE`](#uffdio-copy-mode-dontwake)
  - [`UFFDIO_COPY_MODE_WP`](#uffdio-copy-mode-wp)
  - [`UFFDIO_ZEROPAGE_MODE_DONTWAKE`](#uffdio-zeropage-mode-dontwake)
  - [`SPLICE_F_MOVE`](#splice-f-move)
  - [`SPLICE_F_NONBLOCK`](#splice-f-nonblock)
  - [`SPLICE_F_MORE`](#splice-f-more)
  - [`SPLICE_F_GIFT`](#splice-f-gift)
  - [`_NSIG`](#nsig)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`__BindgenBitfieldUnit`](#bindgenbitfieldunit) | struct |  |
| [`__IncompleteArrayField`](#incompletearrayfield) | struct |  |
| [`__kernel_fd_set`](#kernel-fd-set) | struct |  |
| [`__kernel_fsid_t`](#kernel-fsid-t) | struct |  |
| [`__user_cap_header_struct`](#user-cap-header-struct) | struct |  |
| [`__user_cap_data_struct`](#user-cap-data-struct) | struct |  |
| [`vfs_cap_data`](#vfs-cap-data) | struct |  |
| [`vfs_cap_data__bindgen_ty_1`](#vfs-cap-data-bindgen-ty-1) | struct |  |
| [`vfs_ns_cap_data`](#vfs-ns-cap-data) | struct |  |
| [`vfs_ns_cap_data__bindgen_ty_1`](#vfs-ns-cap-data-bindgen-ty-1) | struct |  |
| [`f_owner_ex`](#f-owner-ex) | struct |  |
| [`flock`](#flock) | struct |  |
| [`flock64`](#flock64) | struct |  |
| [`open_how`](#open-how) | struct |  |
| [`epoll_event`](#epoll-event) | struct |  |
| [`epoll_params`](#epoll-params) | struct |  |
| [`fscrypt_policy_v1`](#fscrypt-policy-v1) | struct |  |
| [`fscrypt_key`](#fscrypt-key) | struct |  |
| [`fscrypt_policy_v2`](#fscrypt-policy-v2) | struct |  |
| [`fscrypt_get_policy_ex_arg`](#fscrypt-get-policy-ex-arg) | struct |  |
| [`fscrypt_key_specifier`](#fscrypt-key-specifier) | struct |  |
| [`fscrypt_provisioning_key_payload`](#fscrypt-provisioning-key-payload) | struct |  |
| [`fscrypt_add_key_arg`](#fscrypt-add-key-arg) | struct |  |
| [`fscrypt_remove_key_arg`](#fscrypt-remove-key-arg) | struct |  |
| [`fscrypt_get_key_status_arg`](#fscrypt-get-key-status-arg) | struct |  |
| [`mount_attr`](#mount-attr) | struct |  |
| [`statmount`](#statmount) | struct |  |
| [`mnt_id_req`](#mnt-id-req) | struct |  |
| [`file_clone_range`](#file-clone-range) | struct |  |
| [`fstrim_range`](#fstrim-range) | struct |  |
| [`fsuuid2`](#fsuuid2) | struct |  |
| [`fs_sysfs_path`](#fs-sysfs-path) | struct |  |
| [`file_dedupe_range_info`](#file-dedupe-range-info) | struct |  |
| [`file_dedupe_range`](#file-dedupe-range) | struct |  |
| [`files_stat_struct`](#files-stat-struct) | struct |  |
| [`inodes_stat_t`](#inodes-stat-t) | struct |  |
| [`fsxattr`](#fsxattr) | struct |  |
| [`page_region`](#page-region) | struct |  |
| [`pm_scan_arg`](#pm-scan-arg) | struct |  |
| [`procmap_query`](#procmap-query) | struct |  |
| [`futex_waitv`](#futex-waitv) | struct |  |
| [`robust_list`](#robust-list) | struct |  |
| [`robust_list_head`](#robust-list-head) | struct |  |
| [`inotify_event`](#inotify-event) | struct |  |
| [`cachestat_range`](#cachestat-range) | struct |  |
| [`cachestat`](#cachestat) | struct |  |
| [`pollfd`](#pollfd) | struct |  |
| [`rand_pool_info`](#rand-pool-info) | struct |  |
| [`vgetrandom_opaque_params`](#vgetrandom-opaque-params) | struct |  |
| [`__kernel_timespec`](#kernel-timespec) | struct |  |
| [`__kernel_itimerspec`](#kernel-itimerspec) | struct |  |
| [`__kernel_old_timeval`](#kernel-old-timeval) | struct |  |
| [`__kernel_old_timespec`](#kernel-old-timespec) | struct |  |
| [`__kernel_old_itimerval`](#kernel-old-itimerval) | struct |  |
| [`__kernel_sock_timeval`](#kernel-sock-timeval) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rlimit64`](#rlimit64) | struct |  |
| [`clone_args`](#clone-args) | struct |  |
| [`sigaction`](#sigaction) | struct |  |
| [`sigaltstack`](#sigaltstack) | struct |  |
| [`__sifields__bindgen_ty_1`](#sifields-bindgen-ty-1) | struct |  |
| [`__sifields__bindgen_ty_2`](#sifields-bindgen-ty-2) | struct |  |
| [`__sifields__bindgen_ty_3`](#sifields-bindgen-ty-3) | struct |  |
| [`__sifields__bindgen_ty_4`](#sifields-bindgen-ty-4) | struct |  |
| [`__sifields__bindgen_ty_5`](#sifields-bindgen-ty-5) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2) | struct |  |
| [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3) | struct |  |
| [`__sifields__bindgen_ty_6`](#sifields-bindgen-ty-6) | struct |  |
| [`__sifields__bindgen_ty_7`](#sifields-bindgen-ty-7) | struct |  |
| [`siginfo`](#siginfo) | struct |  |
| [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo-bindgen-ty-1-bindgen-ty-1) | struct |  |
| [`sigevent`](#sigevent) | struct |  |
| [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent-bindgen-ty-1-bindgen-ty-1) | struct |  |
| [`statx_timestamp`](#statx-timestamp) | struct |  |
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
| [`dmabuf_cmsg`](#dmabuf-cmsg) | struct |  |
| [`dmabuf_token`](#dmabuf-token) | struct |  |
| [`xattr_args`](#xattr-args) | struct |  |
| [`uffd_msg`](#uffd-msg) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd-msg-bindgen-ty-1-bindgen-ty-1) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd-msg-bindgen-ty-1-bindgen-ty-2) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd-msg-bindgen-ty-1-bindgen-ty-3) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd-msg-bindgen-ty-1-bindgen-ty-4) | struct |  |
| [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd-msg-bindgen-ty-1-bindgen-ty-5) | struct |  |
| [`uffdio_api`](#uffdio-api) | struct |  |
| [`uffdio_range`](#uffdio-range) | struct |  |
| [`uffdio_register`](#uffdio-register) | struct |  |
| [`uffdio_copy`](#uffdio-copy) | struct |  |
| [`uffdio_zeropage`](#uffdio-zeropage) | struct |  |
| [`uffdio_writeprotect`](#uffdio-writeprotect) | struct |  |
| [`uffdio_continue`](#uffdio-continue) | struct |  |
| [`uffdio_poison`](#uffdio-poison) | struct |  |
| [`uffdio_move`](#uffdio-move) | struct |  |
| [`linux_dirent64`](#linux-dirent64) | struct |  |
| [`stat`](#stat) | struct |  |
| [`__old_kernel_stat`](#old-kernel-stat) | struct |  |
| [`statfs`](#statfs) | struct |  |
| [`statfs64`](#statfs64) | struct |  |
| [`compat_statfs64`](#compat-statfs64) | struct |  |
| [`user_desc`](#user-desc) | struct |  |
| [`kernel_sigset_t`](#kernel-sigset-t) | struct |  |
| [`kernel_sigaction`](#kernel-sigaction) | struct |  |
| [`fsconfig_command`](#fsconfig-command) | enum |  |
| [`procmap_query_flags`](#procmap-query-flags) | enum |  |
| [`membarrier_cmd`](#membarrier-cmd) | enum |  |
| [`membarrier_cmd_flag`](#membarrier-cmd-flag) | enum |  |
| [`__s8`](#s8) | type |  |
| [`__u8`](#u8) | type |  |
| [`__s16`](#s16) | type |  |
| [`__u16`](#u16) | type |  |
| [`__s32`](#s32) | type |  |
| [`__u32`](#u32) | type |  |
| [`__s64`](#s64) | type |  |
| [`__u64`](#u64) | type |  |
| [`__kernel_sighandler_t`](#kernel-sighandler-t) | type |  |
| [`__kernel_key_t`](#kernel-key-t) | type |  |
| [`__kernel_mqd_t`](#kernel-mqd-t) | type |  |
| [`__kernel_old_uid_t`](#kernel-old-uid-t) | type |  |
| [`__kernel_old_gid_t`](#kernel-old-gid-t) | type |  |
| [`__kernel_old_dev_t`](#kernel-old-dev-t) | type |  |
| [`__kernel_long_t`](#kernel-long-t) | type |  |
| [`__kernel_ulong_t`](#kernel-ulong-t) | type |  |
| [`__kernel_ino_t`](#kernel-ino-t) | type |  |
| [`__kernel_mode_t`](#kernel-mode-t) | type |  |
| [`__kernel_pid_t`](#kernel-pid-t) | type |  |
| [`__kernel_ipc_pid_t`](#kernel-ipc-pid-t) | type |  |
| [`__kernel_uid_t`](#kernel-uid-t) | type |  |
| [`__kernel_gid_t`](#kernel-gid-t) | type |  |
| [`__kernel_suseconds_t`](#kernel-suseconds-t) | type |  |
| [`__kernel_daddr_t`](#kernel-daddr-t) | type |  |
| [`__kernel_uid32_t`](#kernel-uid32-t) | type |  |
| [`__kernel_gid32_t`](#kernel-gid32-t) | type |  |
| [`__kernel_size_t`](#kernel-size-t) | type |  |
| [`__kernel_ssize_t`](#kernel-ssize-t) | type |  |
| [`__kernel_ptrdiff_t`](#kernel-ptrdiff-t) | type |  |
| [`__kernel_off_t`](#kernel-off-t) | type |  |
| [`__kernel_loff_t`](#kernel-loff-t) | type |  |
| [`__kernel_old_time_t`](#kernel-old-time-t) | type |  |
| [`__kernel_time_t`](#kernel-time-t) | type |  |
| [`__kernel_time64_t`](#kernel-time64-t) | type |  |
| [`__kernel_clock_t`](#kernel-clock-t) | type |  |
| [`__kernel_timer_t`](#kernel-timer-t) | type |  |
| [`__kernel_clockid_t`](#kernel-clockid-t) | type |  |
| [`__kernel_caddr_t`](#kernel-caddr-t) | type |  |
| [`__kernel_uid16_t`](#kernel-uid16-t) | type |  |
| [`__kernel_gid16_t`](#kernel-gid16-t) | type |  |
| [`__s128`](#s128) | type |  |
| [`__u128`](#u128) | type |  |
| [`__le16`](#le16) | type |  |
| [`__be16`](#be16) | type |  |
| [`__le32`](#le32) | type |  |
| [`__be32`](#be32) | type |  |
| [`__le64`](#le64) | type |  |
| [`__be64`](#be64) | type |  |
| [`__sum16`](#sum16) | type |  |
| [`__wsum`](#wsum) | type |  |
| [`__poll_t`](#poll-t) | type |  |
| [`cap_user_header_t`](#cap-user-header-t) | type |  |
| [`cap_user_data_t`](#cap-user-data-t) | type |  |
| [`__kernel_rwf_t`](#kernel-rwf-t) | type |  |
| [`sigset_t`](#sigset-t) | type |  |
| [`__signalfn_t`](#signalfn-t) | type |  |
| [`__sighandler_t`](#sighandler-t) | type |  |
| [`__restorefn_t`](#restorefn-t) | type |  |
| [`__sigrestore_t`](#sigrestore-t) | type |  |
| [`stack_t`](#stack-t) | type |  |
| [`sigval_t`](#sigval-t) | type |  |
| [`siginfo_t`](#siginfo-t) | type |  |
| [`sigevent_t`](#sigevent-t) | type |  |
| [`cc_t`](#cc-t) | type |  |
| [`speed_t`](#speed-t) | type |  |
| [`tcflag_t`](#tcflag-t) | type |  |
| [`__fsword_t`](#fsword-t) | type |  |
| [`LINUX_VERSION_CODE`](#linux-version-code) | const |  |
| [`LINUX_VERSION_MAJOR`](#linux-version-major) | const |  |
| [`LINUX_VERSION_PATCHLEVEL`](#linux-version-patchlevel) | const |  |
| [`LINUX_VERSION_SUBLEVEL`](#linux-version-sublevel) | const |  |
| [`__BITS_PER_LONG_LONG`](#bits-per-long-long) | const |  |
| [`__FD_SETSIZE`](#fd-setsize) | const |  |
| [`_LINUX_CAPABILITY_VERSION_1`](#linux-capability-version-1) | const |  |
| [`_LINUX_CAPABILITY_U32S_1`](#linux-capability-u32s-1) | const |  |
| [`_LINUX_CAPABILITY_VERSION_2`](#linux-capability-version-2) | const |  |
| [`_LINUX_CAPABILITY_U32S_2`](#linux-capability-u32s-2) | const |  |
| [`_LINUX_CAPABILITY_VERSION_3`](#linux-capability-version-3) | const |  |
| [`_LINUX_CAPABILITY_U32S_3`](#linux-capability-u32s-3) | const |  |
| [`VFS_CAP_REVISION_MASK`](#vfs-cap-revision-mask) | const |  |
| [`VFS_CAP_REVISION_SHIFT`](#vfs-cap-revision-shift) | const |  |
| [`VFS_CAP_FLAGS_MASK`](#vfs-cap-flags-mask) | const |  |
| [`VFS_CAP_FLAGS_EFFECTIVE`](#vfs-cap-flags-effective) | const |  |
| [`VFS_CAP_REVISION_1`](#vfs-cap-revision-1) | const |  |
| [`VFS_CAP_U32_1`](#vfs-cap-u32-1) | const |  |
| [`VFS_CAP_REVISION_2`](#vfs-cap-revision-2) | const |  |
| [`VFS_CAP_U32_2`](#vfs-cap-u32-2) | const |  |
| [`VFS_CAP_REVISION_3`](#vfs-cap-revision-3) | const |  |
| [`VFS_CAP_U32_3`](#vfs-cap-u32-3) | const |  |
| [`VFS_CAP_U32`](#vfs-cap-u32) | const |  |
| [`VFS_CAP_REVISION`](#vfs-cap-revision) | const |  |
| [`_LINUX_CAPABILITY_VERSION`](#linux-capability-version) | const |  |
| [`_LINUX_CAPABILITY_U32S`](#linux-capability-u32s) | const |  |
| [`CAP_CHOWN`](#cap-chown) | const |  |
| [`CAP_DAC_OVERRIDE`](#cap-dac-override) | const |  |
| [`CAP_DAC_READ_SEARCH`](#cap-dac-read-search) | const |  |
| [`CAP_FOWNER`](#cap-fowner) | const |  |
| [`CAP_FSETID`](#cap-fsetid) | const |  |
| [`CAP_KILL`](#cap-kill) | const |  |
| [`CAP_SETGID`](#cap-setgid) | const |  |
| [`CAP_SETUID`](#cap-setuid) | const |  |
| [`CAP_SETPCAP`](#cap-setpcap) | const |  |
| [`CAP_LINUX_IMMUTABLE`](#cap-linux-immutable) | const |  |
| [`CAP_NET_BIND_SERVICE`](#cap-net-bind-service) | const |  |
| [`CAP_NET_BROADCAST`](#cap-net-broadcast) | const |  |
| [`CAP_NET_ADMIN`](#cap-net-admin) | const |  |
| [`CAP_NET_RAW`](#cap-net-raw) | const |  |
| [`CAP_IPC_LOCK`](#cap-ipc-lock) | const |  |
| [`CAP_IPC_OWNER`](#cap-ipc-owner) | const |  |
| [`CAP_SYS_MODULE`](#cap-sys-module) | const |  |
| [`CAP_SYS_RAWIO`](#cap-sys-rawio) | const |  |
| [`CAP_SYS_CHROOT`](#cap-sys-chroot) | const |  |
| [`CAP_SYS_PTRACE`](#cap-sys-ptrace) | const |  |
| [`CAP_SYS_PACCT`](#cap-sys-pacct) | const |  |
| [`CAP_SYS_ADMIN`](#cap-sys-admin) | const |  |
| [`CAP_SYS_BOOT`](#cap-sys-boot) | const |  |
| [`CAP_SYS_NICE`](#cap-sys-nice) | const |  |
| [`CAP_SYS_RESOURCE`](#cap-sys-resource) | const |  |
| [`CAP_SYS_TIME`](#cap-sys-time) | const |  |
| [`CAP_SYS_TTY_CONFIG`](#cap-sys-tty-config) | const |  |
| [`CAP_MKNOD`](#cap-mknod) | const |  |
| [`CAP_LEASE`](#cap-lease) | const |  |
| [`CAP_AUDIT_WRITE`](#cap-audit-write) | const |  |
| [`CAP_AUDIT_CONTROL`](#cap-audit-control) | const |  |
| [`CAP_SETFCAP`](#cap-setfcap) | const |  |
| [`CAP_MAC_OVERRIDE`](#cap-mac-override) | const |  |
| [`CAP_MAC_ADMIN`](#cap-mac-admin) | const |  |
| [`CAP_SYSLOG`](#cap-syslog) | const |  |
| [`CAP_WAKE_ALARM`](#cap-wake-alarm) | const |  |
| [`CAP_BLOCK_SUSPEND`](#cap-block-suspend) | const |  |
| [`CAP_AUDIT_READ`](#cap-audit-read) | const |  |
| [`CAP_PERFMON`](#cap-perfmon) | const |  |
| [`CAP_BPF`](#cap-bpf) | const |  |
| [`CAP_CHECKPOINT_RESTORE`](#cap-checkpoint-restore) | const |  |
| [`CAP_LAST_CAP`](#cap-last-cap) | const |  |
| [`O_ACCMODE`](#o-accmode) | const |  |
| [`O_RDONLY`](#o-rdonly) | const |  |
| [`O_WRONLY`](#o-wronly) | const |  |
| [`O_RDWR`](#o-rdwr) | const |  |
| [`O_CREAT`](#o-creat) | const |  |
| [`O_EXCL`](#o-excl) | const |  |
| [`O_NOCTTY`](#o-noctty) | const |  |
| [`O_TRUNC`](#o-trunc) | const |  |
| [`O_APPEND`](#o-append) | const |  |
| [`O_NONBLOCK`](#o-nonblock) | const |  |
| [`O_DSYNC`](#o-dsync) | const |  |
| [`FASYNC`](#fasync) | const |  |
| [`O_DIRECT`](#o-direct) | const |  |
| [`O_LARGEFILE`](#o-largefile) | const |  |
| [`O_DIRECTORY`](#o-directory) | const |  |
| [`O_NOFOLLOW`](#o-nofollow) | const |  |
| [`O_NOATIME`](#o-noatime) | const |  |
| [`O_CLOEXEC`](#o-cloexec) | const |  |
| [`__O_SYNC`](#o-sync) | const |  |
| [`O_SYNC`](#o-sync) | const |  |
| [`O_PATH`](#o-path) | const |  |
| [`__O_TMPFILE`](#o-tmpfile) | const |  |
| [`O_TMPFILE`](#o-tmpfile) | const |  |
| [`O_NDELAY`](#o-ndelay) | const |  |
| [`F_DUPFD`](#f-dupfd) | const |  |
| [`F_GETFD`](#f-getfd) | const |  |
| [`F_SETFD`](#f-setfd) | const |  |
| [`F_GETFL`](#f-getfl) | const |  |
| [`F_SETFL`](#f-setfl) | const |  |
| [`F_GETLK`](#f-getlk) | const |  |
| [`F_SETLK`](#f-setlk) | const |  |
| [`F_SETLKW`](#f-setlkw) | const |  |
| [`F_SETOWN`](#f-setown) | const |  |
| [`F_GETOWN`](#f-getown) | const |  |
| [`F_SETSIG`](#f-setsig) | const |  |
| [`F_GETSIG`](#f-getsig) | const |  |
| [`F_SETOWN_EX`](#f-setown-ex) | const |  |
| [`F_GETOWN_EX`](#f-getown-ex) | const |  |
| [`F_GETOWNER_UIDS`](#f-getowner-uids) | const |  |
| [`F_OFD_GETLK`](#f-ofd-getlk) | const |  |
| [`F_OFD_SETLK`](#f-ofd-setlk) | const |  |
| [`F_OFD_SETLKW`](#f-ofd-setlkw) | const |  |
| [`F_OWNER_TID`](#f-owner-tid) | const |  |
| [`F_OWNER_PID`](#f-owner-pid) | const |  |
| [`F_OWNER_PGRP`](#f-owner-pgrp) | const |  |
| [`FD_CLOEXEC`](#fd-cloexec) | const |  |
| [`F_RDLCK`](#f-rdlck) | const |  |
| [`F_WRLCK`](#f-wrlck) | const |  |
| [`F_UNLCK`](#f-unlck) | const |  |
| [`F_EXLCK`](#f-exlck) | const |  |
| [`F_SHLCK`](#f-shlck) | const |  |
| [`LOCK_SH`](#lock-sh) | const |  |
| [`LOCK_EX`](#lock-ex) | const |  |
| [`LOCK_NB`](#lock-nb) | const |  |
| [`LOCK_UN`](#lock-un) | const |  |
| [`LOCK_MAND`](#lock-mand) | const |  |
| [`LOCK_READ`](#lock-read) | const |  |
| [`LOCK_WRITE`](#lock-write) | const |  |
| [`LOCK_RW`](#lock-rw) | const |  |
| [`F_LINUX_SPECIFIC_BASE`](#f-linux-specific-base) | const |  |
| [`RESOLVE_NO_XDEV`](#resolve-no-xdev) | const |  |
| [`RESOLVE_NO_MAGICLINKS`](#resolve-no-magiclinks) | const |  |
| [`RESOLVE_NO_SYMLINKS`](#resolve-no-symlinks) | const |  |
| [`RESOLVE_BENEATH`](#resolve-beneath) | const |  |
| [`RESOLVE_IN_ROOT`](#resolve-in-root) | const |  |
| [`RESOLVE_CACHED`](#resolve-cached) | const |  |
| [`F_SETLEASE`](#f-setlease) | const |  |
| [`F_GETLEASE`](#f-getlease) | const |  |
| [`F_NOTIFY`](#f-notify) | const |  |
| [`F_DUPFD_QUERY`](#f-dupfd-query) | const |  |
| [`F_CREATED_QUERY`](#f-created-query) | const |  |
| [`F_CANCELLK`](#f-cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f-setpipe-sz) | const |  |
| [`F_GETPIPE_SZ`](#f-getpipe-sz) | const |  |
| [`F_ADD_SEALS`](#f-add-seals) | const |  |
| [`F_GET_SEALS`](#f-get-seals) | const |  |
| [`F_SEAL_SEAL`](#f-seal-seal) | const |  |
| [`F_SEAL_SHRINK`](#f-seal-shrink) | const |  |
| [`F_SEAL_GROW`](#f-seal-grow) | const |  |
| [`F_SEAL_WRITE`](#f-seal-write) | const |  |
| [`F_SEAL_FUTURE_WRITE`](#f-seal-future-write) | const |  |
| [`F_SEAL_EXEC`](#f-seal-exec) | const |  |
| [`F_GET_RW_HINT`](#f-get-rw-hint) | const |  |
| [`F_SET_RW_HINT`](#f-set-rw-hint) | const |  |
| [`F_GET_FILE_RW_HINT`](#f-get-file-rw-hint) | const |  |
| [`F_SET_FILE_RW_HINT`](#f-set-file-rw-hint) | const |  |
| [`RWH_WRITE_LIFE_NOT_SET`](#rwh-write-life-not-set) | const |  |
| [`RWH_WRITE_LIFE_NONE`](#rwh-write-life-none) | const |  |
| [`RWH_WRITE_LIFE_SHORT`](#rwh-write-life-short) | const |  |
| [`RWH_WRITE_LIFE_MEDIUM`](#rwh-write-life-medium) | const |  |
| [`RWH_WRITE_LIFE_LONG`](#rwh-write-life-long) | const |  |
| [`RWH_WRITE_LIFE_EXTREME`](#rwh-write-life-extreme) | const |  |
| [`RWF_WRITE_LIFE_NOT_SET`](#rwf-write-life-not-set) | const |  |
| [`DN_ACCESS`](#dn-access) | const |  |
| [`DN_MODIFY`](#dn-modify) | const |  |
| [`DN_CREATE`](#dn-create) | const |  |
| [`DN_DELETE`](#dn-delete) | const |  |
| [`DN_RENAME`](#dn-rename) | const |  |
| [`DN_ATTRIB`](#dn-attrib) | const |  |
| [`DN_MULTISHOT`](#dn-multishot) | const |  |
| [`AT_FDCWD`](#at-fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at-symlink-follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at-no-automount) | const |  |
| [`AT_EMPTY_PATH`](#at-empty-path) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync) | const |  |
| [`AT_RECURSIVE`](#at-recursive) | const |  |
| [`AT_RENAME_NOREPLACE`](#at-rename-noreplace) | const |  |
| [`AT_RENAME_EXCHANGE`](#at-rename-exchange) | const |  |
| [`AT_RENAME_WHITEOUT`](#at-rename-whiteout) | const |  |
| [`AT_EACCESS`](#at-eaccess) | const |  |
| [`AT_REMOVEDIR`](#at-removedir) | const |  |
| [`AT_HANDLE_FID`](#at-handle-fid) | const |  |
| [`AT_HANDLE_MNT_ID_UNIQUE`](#at-handle-mnt-id-unique) | const |  |
| [`AT_HANDLE_CONNECTABLE`](#at-handle-connectable) | const |  |
| [`AT_EXECVE_CHECK`](#at-execve-check) | const |  |
| [`EPOLL_CLOEXEC`](#epoll-cloexec) | const |  |
| [`EPOLL_CTL_ADD`](#epoll-ctl-add) | const |  |
| [`EPOLL_CTL_DEL`](#epoll-ctl-del) | const |  |
| [`EPOLL_CTL_MOD`](#epoll-ctl-mod) | const |  |
| [`EPOLL_IOC_TYPE`](#epoll-ioc-type) | const |  |
| [`POSIX_FADV_NORMAL`](#posix-fadv-normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix-fadv-random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed) | const |  |
| [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed) | const |  |
| [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse) | const |  |
| [`FALLOC_FL_ALLOCATE_RANGE`](#falloc-fl-allocate-range) | const |  |
| [`FALLOC_FL_KEEP_SIZE`](#falloc-fl-keep-size) | const |  |
| [`FALLOC_FL_PUNCH_HOLE`](#falloc-fl-punch-hole) | const |  |
| [`FALLOC_FL_NO_HIDE_STALE`](#falloc-fl-no-hide-stale) | const |  |
| [`FALLOC_FL_COLLAPSE_RANGE`](#falloc-fl-collapse-range) | const |  |
| [`FALLOC_FL_ZERO_RANGE`](#falloc-fl-zero-range) | const |  |
| [`FALLOC_FL_INSERT_RANGE`](#falloc-fl-insert-range) | const |  |
| [`FALLOC_FL_UNSHARE_RANGE`](#falloc-fl-unshare-range) | const |  |
| [`NR_OPEN`](#nr-open) | const |  |
| [`NGROUPS_MAX`](#ngroups-max) | const |  |
| [`ARG_MAX`](#arg-max) | const |  |
| [`LINK_MAX`](#link-max) | const |  |
| [`MAX_CANON`](#max-canon) | const |  |
| [`MAX_INPUT`](#max-input) | const |  |
| [`NAME_MAX`](#name-max) | const |  |
| [`PATH_MAX`](#path-max) | const |  |
| [`PIPE_BUF`](#pipe-buf) | const |  |
| [`XATTR_NAME_MAX`](#xattr-name-max) | const |  |
| [`XATTR_SIZE_MAX`](#xattr-size-max) | const |  |
| [`XATTR_LIST_MAX`](#xattr-list-max) | const |  |
| [`RTSIG_MAX`](#rtsig-max) | const |  |
| [`_IOC_NRBITS`](#ioc-nrbits) | const |  |
| [`_IOC_TYPEBITS`](#ioc-typebits) | const |  |
| [`_IOC_SIZEBITS`](#ioc-sizebits) | const |  |
| [`_IOC_DIRBITS`](#ioc-dirbits) | const |  |
| [`_IOC_NRMASK`](#ioc-nrmask) | const |  |
| [`_IOC_TYPEMASK`](#ioc-typemask) | const |  |
| [`_IOC_SIZEMASK`](#ioc-sizemask) | const |  |
| [`_IOC_DIRMASK`](#ioc-dirmask) | const |  |
| [`_IOC_NRSHIFT`](#ioc-nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#ioc-typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#ioc-sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#ioc-dirshift) | const |  |
| [`_IOC_NONE`](#ioc-none) | const |  |
| [`_IOC_WRITE`](#ioc-write) | const |  |
| [`_IOC_READ`](#ioc-read) | const |  |
| [`IOC_IN`](#ioc-in) | const |  |
| [`IOC_OUT`](#ioc-out) | const |  |
| [`IOC_INOUT`](#ioc-inout) | const |  |
| [`IOCSIZE_MASK`](#iocsize-mask) | const |  |
| [`IOCSIZE_SHIFT`](#iocsize-shift) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_4`](#fscrypt-policy-flags-pad-4) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_8`](#fscrypt-policy-flags-pad-8) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_16`](#fscrypt-policy-flags-pad-16) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_32`](#fscrypt-policy-flags-pad-32) | const |  |
| [`FSCRYPT_POLICY_FLAGS_PAD_MASK`](#fscrypt-policy-flags-pad-mask) | const |  |
| [`FSCRYPT_POLICY_FLAG_DIRECT_KEY`](#fscrypt-policy-flag-direct-key) | const |  |
| [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`](#fscrypt-policy-flag-iv-ino-lblk-64) | const |  |
| [`FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`](#fscrypt-policy-flag-iv-ino-lblk-32) | const |  |
| [`FSCRYPT_MODE_AES_256_XTS`](#fscrypt-mode-aes-256-xts) | const |  |
| [`FSCRYPT_MODE_AES_256_CTS`](#fscrypt-mode-aes-256-cts) | const |  |
| [`FSCRYPT_MODE_AES_128_CBC`](#fscrypt-mode-aes-128-cbc) | const |  |
| [`FSCRYPT_MODE_AES_128_CTS`](#fscrypt-mode-aes-128-cts) | const |  |
| [`FSCRYPT_MODE_SM4_XTS`](#fscrypt-mode-sm4-xts) | const |  |
| [`FSCRYPT_MODE_SM4_CTS`](#fscrypt-mode-sm4-cts) | const |  |
| [`FSCRYPT_MODE_ADIANTUM`](#fscrypt-mode-adiantum) | const |  |
| [`FSCRYPT_MODE_AES_256_HCTR2`](#fscrypt-mode-aes-256-hctr2) | const |  |
| [`FSCRYPT_POLICY_V1`](#fscrypt-policy-v1) | const |  |
| [`FSCRYPT_KEY_DESCRIPTOR_SIZE`](#fscrypt-key-descriptor-size) | const |  |
| [`FSCRYPT_KEY_DESC_PREFIX`](#fscrypt-key-desc-prefix) | const |  |
| [`FSCRYPT_KEY_DESC_PREFIX_SIZE`](#fscrypt-key-desc-prefix-size) | const |  |
| [`FSCRYPT_MAX_KEY_SIZE`](#fscrypt-max-key-size) | const |  |
| [`FSCRYPT_POLICY_V2`](#fscrypt-policy-v2) | const |  |
| [`FSCRYPT_KEY_IDENTIFIER_SIZE`](#fscrypt-key-identifier-size) | const |  |
| [`FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`](#fscrypt-key-spec-type-descriptor) | const |  |
| [`FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`](#fscrypt-key-spec-type-identifier) | const |  |
| [`FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`](#fscrypt-add-key-flag-hw-wrapped) | const |  |
| [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`](#fscrypt-key-removal-status-flag-files-busy) | const |  |
| [`FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`](#fscrypt-key-removal-status-flag-other-users) | const |  |
| [`FSCRYPT_KEY_STATUS_ABSENT`](#fscrypt-key-status-absent) | const |  |
| [`FSCRYPT_KEY_STATUS_PRESENT`](#fscrypt-key-status-present) | const |  |
| [`FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`](#fscrypt-key-status-incompletely-removed) | const |  |
| [`FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`](#fscrypt-key-status-flag-added-by-self) | const |  |
| [`FS_KEY_DESCRIPTOR_SIZE`](#fs-key-descriptor-size) | const |  |
| [`FS_POLICY_FLAGS_PAD_4`](#fs-policy-flags-pad-4) | const |  |
| [`FS_POLICY_FLAGS_PAD_8`](#fs-policy-flags-pad-8) | const |  |
| [`FS_POLICY_FLAGS_PAD_16`](#fs-policy-flags-pad-16) | const |  |
| [`FS_POLICY_FLAGS_PAD_32`](#fs-policy-flags-pad-32) | const |  |
| [`FS_POLICY_FLAGS_PAD_MASK`](#fs-policy-flags-pad-mask) | const |  |
| [`FS_POLICY_FLAG_DIRECT_KEY`](#fs-policy-flag-direct-key) | const |  |
| [`FS_POLICY_FLAGS_VALID`](#fs-policy-flags-valid) | const |  |
| [`FS_ENCRYPTION_MODE_INVALID`](#fs-encryption-mode-invalid) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_XTS`](#fs-encryption-mode-aes-256-xts) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_GCM`](#fs-encryption-mode-aes-256-gcm) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_CBC`](#fs-encryption-mode-aes-256-cbc) | const |  |
| [`FS_ENCRYPTION_MODE_AES_256_CTS`](#fs-encryption-mode-aes-256-cts) | const |  |
| [`FS_ENCRYPTION_MODE_AES_128_CBC`](#fs-encryption-mode-aes-128-cbc) | const |  |
| [`FS_ENCRYPTION_MODE_AES_128_CTS`](#fs-encryption-mode-aes-128-cts) | const |  |
| [`FS_ENCRYPTION_MODE_ADIANTUM`](#fs-encryption-mode-adiantum) | const |  |
| [`FS_KEY_DESC_PREFIX`](#fs-key-desc-prefix) | const |  |
| [`FS_KEY_DESC_PREFIX_SIZE`](#fs-key-desc-prefix-size) | const |  |
| [`FS_MAX_KEY_SIZE`](#fs-max-key-size) | const |  |
| [`MS_RDONLY`](#ms-rdonly) | const |  |
| [`MS_NOSUID`](#ms-nosuid) | const |  |
| [`MS_NODEV`](#ms-nodev) | const |  |
| [`MS_NOEXEC`](#ms-noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms-synchronous) | const |  |
| [`MS_REMOUNT`](#ms-remount) | const |  |
| [`MS_MANDLOCK`](#ms-mandlock) | const |  |
| [`MS_DIRSYNC`](#ms-dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms-nosymfollow) | const |  |
| [`MS_NOATIME`](#ms-noatime) | const |  |
| [`MS_NODIRATIME`](#ms-nodiratime) | const |  |
| [`MS_BIND`](#ms-bind) | const |  |
| [`MS_MOVE`](#ms-move) | const |  |
| [`MS_REC`](#ms-rec) | const |  |
| [`MS_VERBOSE`](#ms-verbose) | const |  |
| [`MS_SILENT`](#ms-silent) | const |  |
| [`MS_POSIXACL`](#ms-posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms-unbindable) | const |  |
| [`MS_PRIVATE`](#ms-private) | const |  |
| [`MS_SLAVE`](#ms-slave) | const |  |
| [`MS_SHARED`](#ms-shared) | const |  |
| [`MS_RELATIME`](#ms-relatime) | const |  |
| [`MS_KERNMOUNT`](#ms-kernmount) | const |  |
| [`MS_I_VERSION`](#ms-i-version) | const |  |
| [`MS_STRICTATIME`](#ms-strictatime) | const |  |
| [`MS_LAZYTIME`](#ms-lazytime) | const |  |
| [`MS_SUBMOUNT`](#ms-submount) | const |  |
| [`MS_NOREMOTELOCK`](#ms-noremotelock) | const |  |
| [`MS_NOSEC`](#ms-nosec) | const |  |
| [`MS_BORN`](#ms-born) | const |  |
| [`MS_ACTIVE`](#ms-active) | const |  |
| [`MS_NOUSER`](#ms-nouser) | const |  |
| [`MS_RMT_MASK`](#ms-rmt-mask) | const |  |
| [`MS_MGC_VAL`](#ms-mgc-val) | const |  |
| [`MS_MGC_MSK`](#ms-mgc-msk) | const |  |
| [`OPEN_TREE_CLONE`](#open-tree-clone) | const |  |
| [`OPEN_TREE_CLOEXEC`](#open-tree-cloexec) | const |  |
| [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks) | const |  |
| [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts) | const |  |
| [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path) | const |  |
| [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks) | const |  |
| [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts) | const |  |
| [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path) | const |  |
| [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group) | const |  |
| [`MOVE_MOUNT_BENEATH`](#move-mount-beneath) | const |  |
| [`MOVE_MOUNT__MASK`](#move-mount-mask) | const |  |
| [`FSOPEN_CLOEXEC`](#fsopen-cloexec) | const |  |
| [`FSPICK_CLOEXEC`](#fspick-cloexec) | const |  |
| [`FSPICK_SYMLINK_NOFOLLOW`](#fspick-symlink-nofollow) | const |  |
| [`FSPICK_NO_AUTOMOUNT`](#fspick-no-automount) | const |  |
| [`FSPICK_EMPTY_PATH`](#fspick-empty-path) | const |  |
| [`FSMOUNT_CLOEXEC`](#fsmount-cloexec) | const |  |
| [`MOUNT_ATTR_RDONLY`](#mount-attr-rdonly) | const |  |
| [`MOUNT_ATTR_NOSUID`](#mount-attr-nosuid) | const |  |
| [`MOUNT_ATTR_NODEV`](#mount-attr-nodev) | const |  |
| [`MOUNT_ATTR_NOEXEC`](#mount-attr-noexec) | const |  |
| [`MOUNT_ATTR__ATIME`](#mount-attr-atime) | const |  |
| [`MOUNT_ATTR_RELATIME`](#mount-attr-relatime) | const |  |
| [`MOUNT_ATTR_NOATIME`](#mount-attr-noatime) | const |  |
| [`MOUNT_ATTR_STRICTATIME`](#mount-attr-strictatime) | const |  |
| [`MOUNT_ATTR_NODIRATIME`](#mount-attr-nodiratime) | const |  |
| [`MOUNT_ATTR_IDMAP`](#mount-attr-idmap) | const |  |
| [`MOUNT_ATTR_NOSYMFOLLOW`](#mount-attr-nosymfollow) | const |  |
| [`MOUNT_ATTR_SIZE_VER0`](#mount-attr-size-ver0) | const |  |
| [`MNT_ID_REQ_SIZE_VER0`](#mnt-id-req-size-ver0) | const |  |
| [`MNT_ID_REQ_SIZE_VER1`](#mnt-id-req-size-ver1) | const |  |
| [`STATMOUNT_SB_BASIC`](#statmount-sb-basic) | const |  |
| [`STATMOUNT_MNT_BASIC`](#statmount-mnt-basic) | const |  |
| [`STATMOUNT_PROPAGATE_FROM`](#statmount-propagate-from) | const |  |
| [`STATMOUNT_MNT_ROOT`](#statmount-mnt-root) | const |  |
| [`STATMOUNT_MNT_POINT`](#statmount-mnt-point) | const |  |
| [`STATMOUNT_FS_TYPE`](#statmount-fs-type) | const |  |
| [`STATMOUNT_MNT_NS_ID`](#statmount-mnt-ns-id) | const |  |
| [`STATMOUNT_MNT_OPTS`](#statmount-mnt-opts) | const |  |
| [`STATMOUNT_FS_SUBTYPE`](#statmount-fs-subtype) | const |  |
| [`STATMOUNT_SB_SOURCE`](#statmount-sb-source) | const |  |
| [`STATMOUNT_OPT_ARRAY`](#statmount-opt-array) | const |  |
| [`STATMOUNT_OPT_SEC_ARRAY`](#statmount-opt-sec-array) | const |  |
| [`STATMOUNT_SUPPORTED_MASK`](#statmount-supported-mask) | const |  |
| [`STATMOUNT_MNT_UIDMAP`](#statmount-mnt-uidmap) | const |  |
| [`STATMOUNT_MNT_GIDMAP`](#statmount-mnt-gidmap) | const |  |
| [`LSMT_ROOT`](#lsmt-root) | const |  |
| [`LISTMOUNT_REVERSE`](#listmount-reverse) | const |  |
| [`INR_OPEN_CUR`](#inr-open-cur) | const |  |
| [`INR_OPEN_MAX`](#inr-open-max) | const |  |
| [`BLOCK_SIZE_BITS`](#block-size-bits) | const |  |
| [`BLOCK_SIZE`](#block-size) | const |  |
| [`IO_INTEGRITY_CHK_GUARD`](#io-integrity-chk-guard) | const |  |
| [`IO_INTEGRITY_CHK_REFTAG`](#io-integrity-chk-reftag) | const |  |
| [`IO_INTEGRITY_CHK_APPTAG`](#io-integrity-chk-apptag) | const |  |
| [`IO_INTEGRITY_VALID_FLAGS`](#io-integrity-valid-flags) | const |  |
| [`SEEK_SET`](#seek-set) | const |  |
| [`SEEK_CUR`](#seek-cur) | const |  |
| [`SEEK_END`](#seek-end) | const |  |
| [`SEEK_DATA`](#seek-data) | const |  |
| [`SEEK_HOLE`](#seek-hole) | const |  |
| [`SEEK_MAX`](#seek-max) | const |  |
| [`RENAME_NOREPLACE`](#rename-noreplace) | const |  |
| [`RENAME_EXCHANGE`](#rename-exchange) | const |  |
| [`RENAME_WHITEOUT`](#rename-whiteout) | const |  |
| [`FILE_DEDUPE_RANGE_SAME`](#file-dedupe-range-same) | const |  |
| [`FILE_DEDUPE_RANGE_DIFFERS`](#file-dedupe-range-differs) | const |  |
| [`NR_FILE`](#nr-file) | const |  |
| [`FS_XFLAG_REALTIME`](#fs-xflag-realtime) | const |  |
| [`FS_XFLAG_PREALLOC`](#fs-xflag-prealloc) | const |  |
| [`FS_XFLAG_IMMUTABLE`](#fs-xflag-immutable) | const |  |
| [`FS_XFLAG_APPEND`](#fs-xflag-append) | const |  |
| [`FS_XFLAG_SYNC`](#fs-xflag-sync) | const |  |
| [`FS_XFLAG_NOATIME`](#fs-xflag-noatime) | const |  |
| [`FS_XFLAG_NODUMP`](#fs-xflag-nodump) | const |  |
| [`FS_XFLAG_RTINHERIT`](#fs-xflag-rtinherit) | const |  |
| [`FS_XFLAG_PROJINHERIT`](#fs-xflag-projinherit) | const |  |
| [`FS_XFLAG_NOSYMLINKS`](#fs-xflag-nosymlinks) | const |  |
| [`FS_XFLAG_EXTSIZE`](#fs-xflag-extsize) | const |  |
| [`FS_XFLAG_EXTSZINHERIT`](#fs-xflag-extszinherit) | const |  |
| [`FS_XFLAG_NODEFRAG`](#fs-xflag-nodefrag) | const |  |
| [`FS_XFLAG_FILESTREAM`](#fs-xflag-filestream) | const |  |
| [`FS_XFLAG_DAX`](#fs-xflag-dax) | const |  |
| [`FS_XFLAG_COWEXTSIZE`](#fs-xflag-cowextsize) | const |  |
| [`FS_XFLAG_HASATTR`](#fs-xflag-hasattr) | const |  |
| [`BMAP_IOCTL`](#bmap-ioctl) | const |  |
| [`FSLABEL_MAX`](#fslabel-max) | const |  |
| [`FS_SECRM_FL`](#fs-secrm-fl) | const |  |
| [`FS_UNRM_FL`](#fs-unrm-fl) | const |  |
| [`FS_COMPR_FL`](#fs-compr-fl) | const |  |
| [`FS_SYNC_FL`](#fs-sync-fl) | const |  |
| [`FS_IMMUTABLE_FL`](#fs-immutable-fl) | const |  |
| [`FS_APPEND_FL`](#fs-append-fl) | const |  |
| [`FS_NODUMP_FL`](#fs-nodump-fl) | const |  |
| [`FS_NOATIME_FL`](#fs-noatime-fl) | const |  |
| [`FS_DIRTY_FL`](#fs-dirty-fl) | const |  |
| [`FS_COMPRBLK_FL`](#fs-comprblk-fl) | const |  |
| [`FS_NOCOMP_FL`](#fs-nocomp-fl) | const |  |
| [`FS_ENCRYPT_FL`](#fs-encrypt-fl) | const |  |
| [`FS_BTREE_FL`](#fs-btree-fl) | const |  |
| [`FS_INDEX_FL`](#fs-index-fl) | const |  |
| [`FS_IMAGIC_FL`](#fs-imagic-fl) | const |  |
| [`FS_JOURNAL_DATA_FL`](#fs-journal-data-fl) | const |  |
| [`FS_NOTAIL_FL`](#fs-notail-fl) | const |  |
| [`FS_DIRSYNC_FL`](#fs-dirsync-fl) | const |  |
| [`FS_TOPDIR_FL`](#fs-topdir-fl) | const |  |
| [`FS_HUGE_FILE_FL`](#fs-huge-file-fl) | const |  |
| [`FS_EXTENT_FL`](#fs-extent-fl) | const |  |
| [`FS_VERITY_FL`](#fs-verity-fl) | const |  |
| [`FS_EA_INODE_FL`](#fs-ea-inode-fl) | const |  |
| [`FS_EOFBLOCKS_FL`](#fs-eofblocks-fl) | const |  |
| [`FS_NOCOW_FL`](#fs-nocow-fl) | const |  |
| [`FS_DAX_FL`](#fs-dax-fl) | const |  |
| [`FS_INLINE_DATA_FL`](#fs-inline-data-fl) | const |  |
| [`FS_PROJINHERIT_FL`](#fs-projinherit-fl) | const |  |
| [`FS_CASEFOLD_FL`](#fs-casefold-fl) | const |  |
| [`FS_RESERVED_FL`](#fs-reserved-fl) | const |  |
| [`FS_FL_USER_VISIBLE`](#fs-fl-user-visible) | const |  |
| [`FS_FL_USER_MODIFIABLE`](#fs-fl-user-modifiable) | const |  |
| [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync-file-range-wait-before) | const |  |
| [`SYNC_FILE_RANGE_WRITE`](#sync-file-range-write) | const |  |
| [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync-file-range-wait-after) | const |  |
| [`SYNC_FILE_RANGE_WRITE_AND_WAIT`](#sync-file-range-write-and-wait) | const |  |
| [`PROCFS_IOCTL_MAGIC`](#procfs-ioctl-magic) | const |  |
| [`PAGE_IS_WPALLOWED`](#page-is-wpallowed) | const |  |
| [`PAGE_IS_WRITTEN`](#page-is-written) | const |  |
| [`PAGE_IS_FILE`](#page-is-file) | const |  |
| [`PAGE_IS_PRESENT`](#page-is-present) | const |  |
| [`PAGE_IS_SWAPPED`](#page-is-swapped) | const |  |
| [`PAGE_IS_PFNZERO`](#page-is-pfnzero) | const |  |
| [`PAGE_IS_HUGE`](#page-is-huge) | const |  |
| [`PAGE_IS_SOFT_DIRTY`](#page-is-soft-dirty) | const |  |
| [`PAGE_IS_GUARD`](#page-is-guard) | const |  |
| [`PM_SCAN_WP_MATCHING`](#pm-scan-wp-matching) | const |  |
| [`PM_SCAN_CHECK_WPASYNC`](#pm-scan-check-wpasync) | const |  |
| [`FUTEX_WAIT`](#futex-wait) | const |  |
| [`FUTEX_WAKE`](#futex-wake) | const |  |
| [`FUTEX_FD`](#futex-fd) | const |  |
| [`FUTEX_REQUEUE`](#futex-requeue) | const |  |
| [`FUTEX_CMP_REQUEUE`](#futex-cmp-requeue) | const |  |
| [`FUTEX_WAKE_OP`](#futex-wake-op) | const |  |
| [`FUTEX_LOCK_PI`](#futex-lock-pi) | const |  |
| [`FUTEX_UNLOCK_PI`](#futex-unlock-pi) | const |  |
| [`FUTEX_TRYLOCK_PI`](#futex-trylock-pi) | const |  |
| [`FUTEX_WAIT_BITSET`](#futex-wait-bitset) | const |  |
| [`FUTEX_WAKE_BITSET`](#futex-wake-bitset) | const |  |
| [`FUTEX_WAIT_REQUEUE_PI`](#futex-wait-requeue-pi) | const |  |
| [`FUTEX_CMP_REQUEUE_PI`](#futex-cmp-requeue-pi) | const |  |
| [`FUTEX_LOCK_PI2`](#futex-lock-pi2) | const |  |
| [`FUTEX_PRIVATE_FLAG`](#futex-private-flag) | const |  |
| [`FUTEX_CLOCK_REALTIME`](#futex-clock-realtime) | const |  |
| [`FUTEX_CMD_MASK`](#futex-cmd-mask) | const |  |
| [`FUTEX_WAIT_PRIVATE`](#futex-wait-private) | const |  |
| [`FUTEX_WAKE_PRIVATE`](#futex-wake-private) | const |  |
| [`FUTEX_REQUEUE_PRIVATE`](#futex-requeue-private) | const |  |
| [`FUTEX_CMP_REQUEUE_PRIVATE`](#futex-cmp-requeue-private) | const |  |
| [`FUTEX_WAKE_OP_PRIVATE`](#futex-wake-op-private) | const |  |
| [`FUTEX_LOCK_PI_PRIVATE`](#futex-lock-pi-private) | const |  |
| [`FUTEX_LOCK_PI2_PRIVATE`](#futex-lock-pi2-private) | const |  |
| [`FUTEX_UNLOCK_PI_PRIVATE`](#futex-unlock-pi-private) | const |  |
| [`FUTEX_TRYLOCK_PI_PRIVATE`](#futex-trylock-pi-private) | const |  |
| [`FUTEX_WAIT_BITSET_PRIVATE`](#futex-wait-bitset-private) | const |  |
| [`FUTEX_WAKE_BITSET_PRIVATE`](#futex-wake-bitset-private) | const |  |
| [`FUTEX_WAIT_REQUEUE_PI_PRIVATE`](#futex-wait-requeue-pi-private) | const |  |
| [`FUTEX_CMP_REQUEUE_PI_PRIVATE`](#futex-cmp-requeue-pi-private) | const |  |
| [`FUTEX2_SIZE_U8`](#futex2-size-u8) | const |  |
| [`FUTEX2_SIZE_U16`](#futex2-size-u16) | const |  |
| [`FUTEX2_SIZE_U32`](#futex2-size-u32) | const |  |
| [`FUTEX2_SIZE_U64`](#futex2-size-u64) | const |  |
| [`FUTEX2_NUMA`](#futex2-numa) | const |  |
| [`FUTEX2_MPOL`](#futex2-mpol) | const |  |
| [`FUTEX2_PRIVATE`](#futex2-private) | const |  |
| [`FUTEX2_SIZE_MASK`](#futex2-size-mask) | const |  |
| [`FUTEX_32`](#futex-32) | const |  |
| [`FUTEX_NO_NODE`](#futex-no-node) | const |  |
| [`FUTEX_WAITV_MAX`](#futex-waitv-max) | const |  |
| [`FUTEX_WAITERS`](#futex-waiters) | const |  |
| [`FUTEX_OWNER_DIED`](#futex-owner-died) | const |  |
| [`FUTEX_TID_MASK`](#futex-tid-mask) | const |  |
| [`ROBUST_LIST_LIMIT`](#robust-list-limit) | const |  |
| [`FUTEX_BITSET_MATCH_ANY`](#futex-bitset-match-any) | const |  |
| [`FUTEX_OP_SET`](#futex-op-set) | const |  |
| [`FUTEX_OP_ADD`](#futex-op-add) | const |  |
| [`FUTEX_OP_OR`](#futex-op-or) | const |  |
| [`FUTEX_OP_ANDN`](#futex-op-andn) | const |  |
| [`FUTEX_OP_XOR`](#futex-op-xor) | const |  |
| [`FUTEX_OP_OPARG_SHIFT`](#futex-op-oparg-shift) | const |  |
| [`FUTEX_OP_CMP_EQ`](#futex-op-cmp-eq) | const |  |
| [`FUTEX_OP_CMP_NE`](#futex-op-cmp-ne) | const |  |
| [`FUTEX_OP_CMP_LT`](#futex-op-cmp-lt) | const |  |
| [`FUTEX_OP_CMP_LE`](#futex-op-cmp-le) | const |  |
| [`FUTEX_OP_CMP_GT`](#futex-op-cmp-gt) | const |  |
| [`FUTEX_OP_CMP_GE`](#futex-op-cmp-ge) | const |  |
| [`IN_ACCESS`](#in-access) | const |  |
| [`IN_MODIFY`](#in-modify) | const |  |
| [`IN_ATTRIB`](#in-attrib) | const |  |
| [`IN_CLOSE_WRITE`](#in-close-write) | const |  |
| [`IN_CLOSE_NOWRITE`](#in-close-nowrite) | const |  |
| [`IN_OPEN`](#in-open) | const |  |
| [`IN_MOVED_FROM`](#in-moved-from) | const |  |
| [`IN_MOVED_TO`](#in-moved-to) | const |  |
| [`IN_CREATE`](#in-create) | const |  |
| [`IN_DELETE`](#in-delete) | const |  |
| [`IN_DELETE_SELF`](#in-delete-self) | const |  |
| [`IN_MOVE_SELF`](#in-move-self) | const |  |
| [`IN_UNMOUNT`](#in-unmount) | const |  |
| [`IN_Q_OVERFLOW`](#in-q-overflow) | const |  |
| [`IN_IGNORED`](#in-ignored) | const |  |
| [`IN_CLOSE`](#in-close) | const |  |
| [`IN_MOVE`](#in-move) | const |  |
| [`IN_ONLYDIR`](#in-onlydir) | const |  |
| [`IN_DONT_FOLLOW`](#in-dont-follow) | const |  |
| [`IN_EXCL_UNLINK`](#in-excl-unlink) | const |  |
| [`IN_MASK_CREATE`](#in-mask-create) | const |  |
| [`IN_MASK_ADD`](#in-mask-add) | const |  |
| [`IN_ISDIR`](#in-isdir) | const |  |
| [`IN_ONESHOT`](#in-oneshot) | const |  |
| [`IN_ALL_EVENTS`](#in-all-events) | const |  |
| [`IN_CLOEXEC`](#in-cloexec) | const |  |
| [`IN_NONBLOCK`](#in-nonblock) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs-super-magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs-super-magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs-super-magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic) | const |  |
| [`CEPH_SUPER_MAGIC`](#ceph-super-magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda-super-magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs-magic) | const |  |
| [`CRAMFS_MAGIC_WEND`](#cramfs-magic-wend) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs-magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs-magic) | const |  |
| [`SELINUX_MAGIC`](#selinux-magic) | const |  |
| [`SMACK_MAGIC`](#smack-magic) | const |  |
| [`RAMFS_MAGIC`](#ramfs-magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs-magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs-magic) | const |  |
| [`SQUASHFS_MAGIC`](#squashfs-magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs-super-magic) | const |  |
| [`EROFS_SUPER_MAGIC_V1`](#erofs-super-magic-v1) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2-super-magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3-super-magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs-super-magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4-super-magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs-super-magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs-super-magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs-super-magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs-super-magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic) | const |  |
| [`XFS_SUPER_MAGIC`](#xfs-super-magic) | const |  |
| [`PSTOREFS_MAGIC`](#pstorefs-magic) | const |  |
| [`EFIVARFS_MAGIC`](#efivarfs-magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse-super-magic) | const |  |
| [`BCACHEFS_SUPER_MAGIC`](#bcachefs-super-magic) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix-super-magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix-super-magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2-super-magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3-super-magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos-super-magic) | const |  |
| [`EXFAT_SUPER_MAGIC`](#exfat-super-magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp-super-magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs-super-magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4-super-magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6-super-magic) | const |  |
| [`AFS_FS_MAGIC`](#afs-fs-magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic) | const |  |
| [`REISERFS_SUPER_MAGIC_STRING`](#reiserfs-super-magic-string) | const |  |
| [`REISER2FS_SUPER_MAGIC_STRING`](#reiser2fs-super-magic-string) | const |  |
| [`REISER2FS_JR_SUPER_MAGIC_STRING`](#reiser2fs-jr-super-magic-string) | const |  |
| [`SMB_SUPER_MAGIC`](#smb-super-magic) | const |  |
| [`CIFS_SUPER_MAGIC`](#cifs-super-magic) | const |  |
| [`SMB2_SUPER_MAGIC`](#smb2-super-magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic) | const |  |
| [`STACK_END_MAGIC`](#stack-end-magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs-magic) | const |  |
| [`V9FS_MAGIC`](#v9fs-magic) | const |  |
| [`BDEVFS_MAGIC`](#bdevfs-magic) | const |  |
| [`DAXFS_MAGIC`](#daxfs-magic) | const |  |
| [`BINFMTFS_MAGIC`](#binfmtfs-magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic) | const |  |
| [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic) | const |  |
| [`PIPEFS_MAGIC`](#pipefs-magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc-super-magic) | const |  |
| [`SOCKFS_MAGIC`](#sockfs-magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs-magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic) | const |  |
| [`MTD_INODE_FS_MAGIC`](#mtd-inode-fs-magic) | const |  |
| [`ANON_INODE_FS_MAGIC`](#anon-inode-fs-magic) | const |  |
| [`BTRFS_TEST_MAGIC`](#btrfs-test-magic) | const |  |
| [`NSFS_MAGIC`](#nsfs-magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf-fs-magic) | const |  |
| [`AAFS_MAGIC`](#aafs-magic) | const |  |
| [`ZONEFS_MAGIC`](#zonefs-magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf-super-magic) | const |  |
| [`DMA_BUF_MAGIC`](#dma-buf-magic) | const |  |
| [`DEVMEM_MAGIC`](#devmem-magic) | const |  |
| [`SECRETMEM_MAGIC`](#secretmem-magic) | const |  |
| [`PID_FS_MAGIC`](#pid-fs-magic) | const |  |
| [`MAP_32BIT`](#map-32bit) | const |  |
| [`MAP_ABOVE4G`](#map-above4g) | const |  |
| [`PROT_READ`](#prot-read) | const |  |
| [`PROT_WRITE`](#prot-write) | const |  |
| [`PROT_EXEC`](#prot-exec) | const |  |
| [`PROT_SEM`](#prot-sem) | const |  |
| [`PROT_NONE`](#prot-none) | const |  |
| [`PROT_GROWSDOWN`](#prot-growsdown) | const |  |
| [`PROT_GROWSUP`](#prot-growsup) | const |  |
| [`MAP_TYPE`](#map-type) | const |  |
| [`MAP_FIXED`](#map-fixed) | const |  |
| [`MAP_ANONYMOUS`](#map-anonymous) | const |  |
| [`MAP_POPULATE`](#map-populate) | const |  |
| [`MAP_NONBLOCK`](#map-nonblock) | const |  |
| [`MAP_STACK`](#map-stack) | const |  |
| [`MAP_HUGETLB`](#map-hugetlb) | const |  |
| [`MAP_SYNC`](#map-sync) | const |  |
| [`MAP_FIXED_NOREPLACE`](#map-fixed-noreplace) | const |  |
| [`MAP_UNINITIALIZED`](#map-uninitialized) | const |  |
| [`MLOCK_ONFAULT`](#mlock-onfault) | const |  |
| [`MS_ASYNC`](#ms-async) | const |  |
| [`MS_INVALIDATE`](#ms-invalidate) | const |  |
| [`MS_SYNC`](#ms-sync) | const |  |
| [`MADV_NORMAL`](#madv-normal) | const |  |
| [`MADV_RANDOM`](#madv-random) | const |  |
| [`MADV_SEQUENTIAL`](#madv-sequential) | const |  |
| [`MADV_WILLNEED`](#madv-willneed) | const |  |
| [`MADV_DONTNEED`](#madv-dontneed) | const |  |
| [`MADV_FREE`](#madv-free) | const |  |
| [`MADV_REMOVE`](#madv-remove) | const |  |
| [`MADV_DONTFORK`](#madv-dontfork) | const |  |
| [`MADV_DOFORK`](#madv-dofork) | const |  |
| [`MADV_HWPOISON`](#madv-hwpoison) | const |  |
| [`MADV_SOFT_OFFLINE`](#madv-soft-offline) | const |  |
| [`MADV_MERGEABLE`](#madv-mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv-unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv-hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv-nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv-dontdump) | const |  |
| [`MADV_DODUMP`](#madv-dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv-wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv-keeponfork) | const |  |
| [`MADV_COLD`](#madv-cold) | const |  |
| [`MADV_PAGEOUT`](#madv-pageout) | const |  |
| [`MADV_POPULATE_READ`](#madv-populate-read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv-populate-write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked) | const |  |
| [`MADV_COLLAPSE`](#madv-collapse) | const |  |
| [`MADV_GUARD_INSTALL`](#madv-guard-install) | const |  |
| [`MADV_GUARD_REMOVE`](#madv-guard-remove) | const |  |
| [`MAP_FILE`](#map-file) | const |  |
| [`PKEY_UNRESTRICTED`](#pkey-unrestricted) | const |  |
| [`PKEY_DISABLE_ACCESS`](#pkey-disable-access) | const |  |
| [`PKEY_DISABLE_WRITE`](#pkey-disable-write) | const |  |
| [`PKEY_ACCESS_MASK`](#pkey-access-mask) | const |  |
| [`MAP_GROWSDOWN`](#map-growsdown) | const |  |
| [`MAP_DENYWRITE`](#map-denywrite) | const |  |
| [`MAP_EXECUTABLE`](#map-executable) | const |  |
| [`MAP_LOCKED`](#map-locked) | const |  |
| [`MAP_NORESERVE`](#map-noreserve) | const |  |
| [`MCL_CURRENT`](#mcl-current) | const |  |
| [`MCL_FUTURE`](#mcl-future) | const |  |
| [`MCL_ONFAULT`](#mcl-onfault) | const |  |
| [`SHADOW_STACK_SET_TOKEN`](#shadow-stack-set-token) | const |  |
| [`SHADOW_STACK_SET_MARKER`](#shadow-stack-set-marker) | const |  |
| [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift) | const |  |
| [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask) | const |  |
| [`HUGETLB_FLAG_ENCODE_16KB`](#hugetlb-flag-encode-16kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb-flag-encode-64kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb-flag-encode-512kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb-flag-encode-1mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb-flag-encode-2mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb-flag-encode-8mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb-flag-encode-16mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb-flag-encode-32mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb-flag-encode-256mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb-flag-encode-512mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb-flag-encode-1gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb-flag-encode-2gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb-flag-encode-16gb) | const |  |
| [`MREMAP_MAYMOVE`](#mremap-maymove) | const |  |
| [`MREMAP_FIXED`](#mremap-fixed) | const |  |
| [`MREMAP_DONTUNMAP`](#mremap-dontunmap) | const |  |
| [`OVERCOMMIT_GUESS`](#overcommit-guess) | const |  |
| [`OVERCOMMIT_ALWAYS`](#overcommit-always) | const |  |
| [`OVERCOMMIT_NEVER`](#overcommit-never) | const |  |
| [`MAP_SHARED`](#map-shared) | const |  |
| [`MAP_PRIVATE`](#map-private) | const |  |
| [`MAP_SHARED_VALIDATE`](#map-shared-validate) | const |  |
| [`MAP_DROPPABLE`](#map-droppable) | const |  |
| [`MAP_HUGE_SHIFT`](#map-huge-shift) | const |  |
| [`MAP_HUGE_MASK`](#map-huge-mask) | const |  |
| [`MAP_HUGE_16KB`](#map-huge-16kb) | const |  |
| [`MAP_HUGE_64KB`](#map-huge-64kb) | const |  |
| [`MAP_HUGE_512KB`](#map-huge-512kb) | const |  |
| [`MAP_HUGE_1MB`](#map-huge-1mb) | const |  |
| [`MAP_HUGE_2MB`](#map-huge-2mb) | const |  |
| [`MAP_HUGE_8MB`](#map-huge-8mb) | const |  |
| [`MAP_HUGE_16MB`](#map-huge-16mb) | const |  |
| [`MAP_HUGE_32MB`](#map-huge-32mb) | const |  |
| [`MAP_HUGE_256MB`](#map-huge-256mb) | const |  |
| [`MAP_HUGE_512MB`](#map-huge-512mb) | const |  |
| [`MAP_HUGE_1GB`](#map-huge-1gb) | const |  |
| [`MAP_HUGE_2GB`](#map-huge-2gb) | const |  |
| [`MAP_HUGE_16GB`](#map-huge-16gb) | const |  |
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
| [`GRND_NONBLOCK`](#grnd-nonblock) | const |  |
| [`GRND_RANDOM`](#grnd-random) | const |  |
| [`GRND_INSECURE`](#grnd-insecure) | const |  |
| [`LINUX_REBOOT_MAGIC1`](#linux-reboot-magic1) | const |  |
| [`LINUX_REBOOT_MAGIC2`](#linux-reboot-magic2) | const |  |
| [`LINUX_REBOOT_MAGIC2A`](#linux-reboot-magic2a) | const |  |
| [`LINUX_REBOOT_MAGIC2B`](#linux-reboot-magic2b) | const |  |
| [`LINUX_REBOOT_MAGIC2C`](#linux-reboot-magic2c) | const |  |
| [`LINUX_REBOOT_CMD_RESTART`](#linux-reboot-cmd-restart) | const |  |
| [`LINUX_REBOOT_CMD_HALT`](#linux-reboot-cmd-halt) | const |  |
| [`LINUX_REBOOT_CMD_CAD_ON`](#linux-reboot-cmd-cad-on) | const |  |
| [`LINUX_REBOOT_CMD_CAD_OFF`](#linux-reboot-cmd-cad-off) | const |  |
| [`LINUX_REBOOT_CMD_POWER_OFF`](#linux-reboot-cmd-power-off) | const |  |
| [`LINUX_REBOOT_CMD_RESTART2`](#linux-reboot-cmd-restart2) | const |  |
| [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux-reboot-cmd-sw-suspend) | const |  |
| [`LINUX_REBOOT_CMD_KEXEC`](#linux-reboot-cmd-kexec) | const |  |
| [`RUSAGE_SELF`](#rusage-self) | const |  |
| [`RUSAGE_CHILDREN`](#rusage-children) | const |  |
| [`RUSAGE_BOTH`](#rusage-both) | const |  |
| [`RUSAGE_THREAD`](#rusage-thread) | const |  |
| [`RLIM64_INFINITY`](#rlim64-infinity) | const |  |
| [`PRIO_MIN`](#prio-min) | const |  |
| [`PRIO_MAX`](#prio-max) | const |  |
| [`PRIO_PROCESS`](#prio-process) | const |  |
| [`PRIO_PGRP`](#prio-pgrp) | const |  |
| [`PRIO_USER`](#prio-user) | const |  |
| [`_STK_LIM`](#stk-lim) | const |  |
| [`MLOCK_LIMIT`](#mlock-limit) | const |  |
| [`RLIMIT_CPU`](#rlimit-cpu) | const |  |
| [`RLIMIT_FSIZE`](#rlimit-fsize) | const |  |
| [`RLIMIT_DATA`](#rlimit-data) | const |  |
| [`RLIMIT_STACK`](#rlimit-stack) | const |  |
| [`RLIMIT_CORE`](#rlimit-core) | const |  |
| [`RLIMIT_RSS`](#rlimit-rss) | const |  |
| [`RLIMIT_NPROC`](#rlimit-nproc) | const |  |
| [`RLIMIT_NOFILE`](#rlimit-nofile) | const |  |
| [`RLIMIT_MEMLOCK`](#rlimit-memlock) | const |  |
| [`RLIMIT_AS`](#rlimit-as) | const |  |
| [`RLIMIT_LOCKS`](#rlimit-locks) | const |  |
| [`RLIMIT_SIGPENDING`](#rlimit-sigpending) | const |  |
| [`RLIMIT_MSGQUEUE`](#rlimit-msgqueue) | const |  |
| [`RLIMIT_NICE`](#rlimit-nice) | const |  |
| [`RLIMIT_RTPRIO`](#rlimit-rtprio) | const |  |
| [`RLIMIT_RTTIME`](#rlimit-rttime) | const |  |
| [`RLIM_NLIMITS`](#rlim-nlimits) | const |  |
| [`RLIM_INFINITY`](#rlim-infinity) | const |  |
| [`CSIGNAL`](#csignal) | const |  |
| [`CLONE_VM`](#clone-vm) | const |  |
| [`CLONE_FS`](#clone-fs) | const |  |
| [`CLONE_FILES`](#clone-files) | const |  |
| [`CLONE_SIGHAND`](#clone-sighand) | const |  |
| [`CLONE_PIDFD`](#clone-pidfd) | const |  |
| [`CLONE_PTRACE`](#clone-ptrace) | const |  |
| [`CLONE_VFORK`](#clone-vfork) | const |  |
| [`CLONE_PARENT`](#clone-parent) | const |  |
| [`CLONE_THREAD`](#clone-thread) | const |  |
| [`CLONE_NEWNS`](#clone-newns) | const |  |
| [`CLONE_SYSVSEM`](#clone-sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone-settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone-parent-settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid) | const |  |
| [`CLONE_DETACHED`](#clone-detached) | const |  |
| [`CLONE_UNTRACED`](#clone-untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone-child-settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone-newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone-newuts) | const |  |
| [`CLONE_NEWIPC`](#clone-newipc) | const |  |
| [`CLONE_NEWUSER`](#clone-newuser) | const |  |
| [`CLONE_NEWPID`](#clone-newpid) | const |  |
| [`CLONE_NEWNET`](#clone-newnet) | const |  |
| [`CLONE_IO`](#clone-io) | const |  |
| [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand) | const |  |
| [`CLONE_INTO_CGROUP`](#clone-into-cgroup) | const |  |
| [`CLONE_NEWTIME`](#clone-newtime) | const |  |
| [`CLONE_ARGS_SIZE_VER0`](#clone-args-size-ver0) | const |  |
| [`CLONE_ARGS_SIZE_VER1`](#clone-args-size-ver1) | const |  |
| [`CLONE_ARGS_SIZE_VER2`](#clone-args-size-ver2) | const |  |
| [`SCHED_NORMAL`](#sched-normal) | const |  |
| [`SCHED_FIFO`](#sched-fifo) | const |  |
| [`SCHED_RR`](#sched-rr) | const |  |
| [`SCHED_BATCH`](#sched-batch) | const |  |
| [`SCHED_IDLE`](#sched-idle) | const |  |
| [`SCHED_DEADLINE`](#sched-deadline) | const |  |
| [`SCHED_EXT`](#sched-ext) | const |  |
| [`SCHED_RESET_ON_FORK`](#sched-reset-on-fork) | const |  |
| [`SCHED_FLAG_RESET_ON_FORK`](#sched-flag-reset-on-fork) | const |  |
| [`SCHED_FLAG_RECLAIM`](#sched-flag-reclaim) | const |  |
| [`SCHED_FLAG_DL_OVERRUN`](#sched-flag-dl-overrun) | const |  |
| [`SCHED_FLAG_KEEP_POLICY`](#sched-flag-keep-policy) | const |  |
| [`SCHED_FLAG_KEEP_PARAMS`](#sched-flag-keep-params) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched-flag-util-clamp-min) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched-flag-util-clamp-max) | const |  |
| [`SCHED_FLAG_KEEP_ALL`](#sched-flag-keep-all) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP`](#sched-flag-util-clamp) | const |  |
| [`SCHED_FLAG_ALL`](#sched-flag-all) | const |  |
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
| [`SA_RESTORER`](#sa-restorer) | const |  |
| [`MINSIGSTKSZ`](#minsigstksz) | const |  |
| [`SIGSTKSZ`](#sigstksz) | const |  |
| [`SA_NOCLDSTOP`](#sa-nocldstop) | const |  |
| [`SA_NOCLDWAIT`](#sa-nocldwait) | const |  |
| [`SA_SIGINFO`](#sa-siginfo) | const |  |
| [`SA_UNSUPPORTED`](#sa-unsupported) | const |  |
| [`SA_EXPOSE_TAGBITS`](#sa-expose-tagbits) | const |  |
| [`SA_ONSTACK`](#sa-onstack) | const |  |
| [`SA_RESTART`](#sa-restart) | const |  |
| [`SA_NODEFER`](#sa-nodefer) | const |  |
| [`SA_RESETHAND`](#sa-resethand) | const |  |
| [`SA_NOMASK`](#sa-nomask) | const |  |
| [`SA_ONESHOT`](#sa-oneshot) | const |  |
| [`SIG_BLOCK`](#sig-block) | const |  |
| [`SIG_UNBLOCK`](#sig-unblock) | const |  |
| [`SIG_SETMASK`](#sig-setmask) | const |  |
| [`SI_MAX_SIZE`](#si-max-size) | const |  |
| [`SI_USER`](#si-user) | const |  |
| [`SI_KERNEL`](#si-kernel) | const |  |
| [`SI_QUEUE`](#si-queue) | const |  |
| [`SI_TIMER`](#si-timer) | const |  |
| [`SI_MESGQ`](#si-mesgq) | const |  |
| [`SI_ASYNCIO`](#si-asyncio) | const |  |
| [`SI_SIGIO`](#si-sigio) | const |  |
| [`SI_TKILL`](#si-tkill) | const |  |
| [`SI_DETHREAD`](#si-dethread) | const |  |
| [`SI_ASYNCNL`](#si-asyncnl) | const |  |
| [`ILL_ILLOPC`](#ill-illopc) | const |  |
| [`ILL_ILLOPN`](#ill-illopn) | const |  |
| [`ILL_ILLADR`](#ill-illadr) | const |  |
| [`ILL_ILLTRP`](#ill-illtrp) | const |  |
| [`ILL_PRVOPC`](#ill-prvopc) | const |  |
| [`ILL_PRVREG`](#ill-prvreg) | const |  |
| [`ILL_COPROC`](#ill-coproc) | const |  |
| [`ILL_BADSTK`](#ill-badstk) | const |  |
| [`ILL_BADIADDR`](#ill-badiaddr) | const |  |
| [`__ILL_BREAK`](#ill-break) | const |  |
| [`__ILL_BNDMOD`](#ill-bndmod) | const |  |
| [`NSIGILL`](#nsigill) | const |  |
| [`FPE_INTDIV`](#fpe-intdiv) | const |  |
| [`FPE_INTOVF`](#fpe-intovf) | const |  |
| [`FPE_FLTDIV`](#fpe-fltdiv) | const |  |
| [`FPE_FLTOVF`](#fpe-fltovf) | const |  |
| [`FPE_FLTUND`](#fpe-fltund) | const |  |
| [`FPE_FLTRES`](#fpe-fltres) | const |  |
| [`FPE_FLTINV`](#fpe-fltinv) | const |  |
| [`FPE_FLTSUB`](#fpe-fltsub) | const |  |
| [`__FPE_DECOVF`](#fpe-decovf) | const |  |
| [`__FPE_DECDIV`](#fpe-decdiv) | const |  |
| [`__FPE_DECERR`](#fpe-decerr) | const |  |
| [`__FPE_INVASC`](#fpe-invasc) | const |  |
| [`__FPE_INVDEC`](#fpe-invdec) | const |  |
| [`FPE_FLTUNK`](#fpe-fltunk) | const |  |
| [`FPE_CONDTRAP`](#fpe-condtrap) | const |  |
| [`NSIGFPE`](#nsigfpe) | const |  |
| [`SEGV_MAPERR`](#segv-maperr) | const |  |
| [`SEGV_ACCERR`](#segv-accerr) | const |  |
| [`SEGV_BNDERR`](#segv-bnderr) | const |  |
| [`SEGV_PKUERR`](#segv-pkuerr) | const |  |
| [`SEGV_ACCADI`](#segv-accadi) | const |  |
| [`SEGV_ADIDERR`](#segv-adiderr) | const |  |
| [`SEGV_ADIPERR`](#segv-adiperr) | const |  |
| [`SEGV_MTEAERR`](#segv-mteaerr) | const |  |
| [`SEGV_MTESERR`](#segv-mteserr) | const |  |
| [`SEGV_CPERR`](#segv-cperr) | const |  |
| [`NSIGSEGV`](#nsigsegv) | const |  |
| [`BUS_ADRALN`](#bus-adraln) | const |  |
| [`BUS_ADRERR`](#bus-adrerr) | const |  |
| [`BUS_OBJERR`](#bus-objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus-mceerr-ar) | const |  |
| [`BUS_MCEERR_AO`](#bus-mceerr-ao) | const |  |
| [`NSIGBUS`](#nsigbus) | const |  |
| [`TRAP_BRKPT`](#trap-brkpt) | const |  |
| [`TRAP_TRACE`](#trap-trace) | const |  |
| [`TRAP_BRANCH`](#trap-branch) | const |  |
| [`TRAP_HWBKPT`](#trap-hwbkpt) | const |  |
| [`TRAP_UNK`](#trap-unk) | const |  |
| [`TRAP_PERF`](#trap-perf) | const |  |
| [`NSIGTRAP`](#nsigtrap) | const |  |
| [`TRAP_PERF_FLAG_ASYNC`](#trap-perf-flag-async) | const |  |
| [`CLD_EXITED`](#cld-exited) | const |  |
| [`CLD_KILLED`](#cld-killed) | const |  |
| [`CLD_DUMPED`](#cld-dumped) | const |  |
| [`CLD_TRAPPED`](#cld-trapped) | const |  |
| [`CLD_STOPPED`](#cld-stopped) | const |  |
| [`CLD_CONTINUED`](#cld-continued) | const |  |
| [`NSIGCHLD`](#nsigchld) | const |  |
| [`POLL_IN`](#poll-in) | const |  |
| [`POLL_OUT`](#poll-out) | const |  |
| [`POLL_MSG`](#poll-msg) | const |  |
| [`POLL_ERR`](#poll-err) | const |  |
| [`POLL_PRI`](#poll-pri) | const |  |
| [`POLL_HUP`](#poll-hup) | const |  |
| [`NSIGPOLL`](#nsigpoll) | const |  |
| [`SYS_SECCOMP`](#sys-seccomp) | const |  |
| [`SYS_USER_DISPATCH`](#sys-user-dispatch) | const |  |
| [`NSIGSYS`](#nsigsys) | const |  |
| [`EMT_TAGOVF`](#emt-tagovf) | const |  |
| [`NSIGEMT`](#nsigemt) | const |  |
| [`SIGEV_SIGNAL`](#sigev-signal) | const |  |
| [`SIGEV_NONE`](#sigev-none) | const |  |
| [`SIGEV_THREAD`](#sigev-thread) | const |  |
| [`SIGEV_THREAD_ID`](#sigev-thread-id) | const |  |
| [`SIGEV_MAX_SIZE`](#sigev-max-size) | const |  |
| [`SS_ONSTACK`](#ss-onstack) | const |  |
| [`SS_DISABLE`](#ss-disable) | const |  |
| [`SS_AUTODISARM`](#ss-autodisarm) | const |  |
| [`SS_FLAG_BITS`](#ss-flag-bits) | const |  |
| [`S_IFMT`](#s-ifmt) | const |  |
| [`S_IFSOCK`](#s-ifsock) | const |  |
| [`S_IFLNK`](#s-iflnk) | const |  |
| [`S_IFREG`](#s-ifreg) | const |  |
| [`S_IFBLK`](#s-ifblk) | const |  |
| [`S_IFDIR`](#s-ifdir) | const |  |
| [`S_IFCHR`](#s-ifchr) | const |  |
| [`S_IFIFO`](#s-ififo) | const |  |
| [`S_ISUID`](#s-isuid) | const |  |
| [`S_ISGID`](#s-isgid) | const |  |
| [`S_ISVTX`](#s-isvtx) | const |  |
| [`S_IRWXU`](#s-irwxu) | const |  |
| [`S_IRUSR`](#s-irusr) | const |  |
| [`S_IWUSR`](#s-iwusr) | const |  |
| [`S_IXUSR`](#s-ixusr) | const |  |
| [`S_IRWXG`](#s-irwxg) | const |  |
| [`S_IRGRP`](#s-irgrp) | const |  |
| [`S_IWGRP`](#s-iwgrp) | const |  |
| [`S_IXGRP`](#s-ixgrp) | const |  |
| [`S_IRWXO`](#s-irwxo) | const |  |
| [`S_IROTH`](#s-iroth) | const |  |
| [`S_IWOTH`](#s-iwoth) | const |  |
| [`S_IXOTH`](#s-ixoth) | const |  |
| [`STATX_TYPE`](#statx-type) | const |  |
| [`STATX_MODE`](#statx-mode) | const |  |
| [`STATX_NLINK`](#statx-nlink) | const |  |
| [`STATX_UID`](#statx-uid) | const |  |
| [`STATX_GID`](#statx-gid) | const |  |
| [`STATX_ATIME`](#statx-atime) | const |  |
| [`STATX_MTIME`](#statx-mtime) | const |  |
| [`STATX_CTIME`](#statx-ctime) | const |  |
| [`STATX_INO`](#statx-ino) | const |  |
| [`STATX_SIZE`](#statx-size) | const |  |
| [`STATX_BLOCKS`](#statx-blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx-basic-stats) | const |  |
| [`STATX_BTIME`](#statx-btime) | const |  |
| [`STATX_MNT_ID`](#statx-mnt-id) | const |  |
| [`STATX_DIOALIGN`](#statx-dioalign) | const |  |
| [`STATX_MNT_ID_UNIQUE`](#statx-mnt-id-unique) | const |  |
| [`STATX_SUBVOL`](#statx-subvol) | const |  |
| [`STATX_WRITE_ATOMIC`](#statx-write-atomic) | const |  |
| [`STATX_DIO_READ_ALIGN`](#statx-dio-read-align) | const |  |
| [`STATX__RESERVED`](#statx-reserved) | const |  |
| [`STATX_ALL`](#statx-all) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx-attr-append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx-attr-nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root) | const |  |
| [`STATX_ATTR_VERITY`](#statx-attr-verity) | const |  |
| [`STATX_ATTR_DAX`](#statx-attr-dax) | const |  |
| [`STATX_ATTR_WRITE_ATOMIC`](#statx-attr-write-atomic) | const |  |
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
| [`TIOCPKT_DATA`](#tiocpkt-data) | const |  |
| [`TIOCPKT_FLUSHREAD`](#tiocpkt-flushread) | const |  |
| [`TIOCPKT_FLUSHWRITE`](#tiocpkt-flushwrite) | const |  |
| [`TIOCPKT_STOP`](#tiocpkt-stop) | const |  |
| [`TIOCPKT_START`](#tiocpkt-start) | const |  |
| [`TIOCPKT_NOSTOP`](#tiocpkt-nostop) | const |  |
| [`TIOCPKT_DOSTOP`](#tiocpkt-dostop) | const |  |
| [`TIOCPKT_IOCTL`](#tiocpkt-ioctl) | const |  |
| [`TIOCSER_TEMT`](#tiocser-temt) | const |  |
| [`NCC`](#ncc) | const |  |
| [`TIOCM_LE`](#tiocm-le) | const |  |
| [`TIOCM_DTR`](#tiocm-dtr) | const |  |
| [`TIOCM_RTS`](#tiocm-rts) | const |  |
| [`TIOCM_ST`](#tiocm-st) | const |  |
| [`TIOCM_SR`](#tiocm-sr) | const |  |
| [`TIOCM_CTS`](#tiocm-cts) | const |  |
| [`TIOCM_CAR`](#tiocm-car) | const |  |
| [`TIOCM_RNG`](#tiocm-rng) | const |  |
| [`TIOCM_DSR`](#tiocm-dsr) | const |  |
| [`TIOCM_CD`](#tiocm-cd) | const |  |
| [`TIOCM_RI`](#tiocm-ri) | const |  |
| [`TIOCM_OUT1`](#tiocm-out1) | const |  |
| [`TIOCM_OUT2`](#tiocm-out2) | const |  |
| [`TIOCM_LOOP`](#tiocm-loop) | const |  |
| [`ITIMER_REAL`](#itimer-real) | const |  |
| [`ITIMER_VIRTUAL`](#itimer-virtual) | const |  |
| [`ITIMER_PROF`](#itimer-prof) | const |  |
| [`CLOCK_REALTIME`](#clock-realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock-monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock-boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm) | const |  |
| [`CLOCK_SGI_CYCLE`](#clock-sgi-cycle) | const |  |
| [`CLOCK_TAI`](#clock-tai) | const |  |
| [`MAX_CLOCKS`](#max-clocks) | const |  |
| [`CLOCKS_MASK`](#clocks-mask) | const |  |
| [`CLOCKS_MONO`](#clocks-mono) | const |  |
| [`TIMER_ABSTIME`](#timer-abstime) | const |  |
| [`UIO_FASTIOV`](#uio-fastiov) | const |  |
| [`UIO_MAXIOV`](#uio-maxiov) | const |  |
| [`__X32_SYSCALL_BIT`](#x32-syscall-bit) | const |  |
| [`__NR_read`](#nr-read) | const |  |
| [`__NR_write`](#nr-write) | const |  |
| [`__NR_open`](#nr-open) | const |  |
| [`__NR_close`](#nr-close) | const |  |
| [`__NR_stat`](#nr-stat) | const |  |
| [`__NR_fstat`](#nr-fstat) | const |  |
| [`__NR_lstat`](#nr-lstat) | const |  |
| [`__NR_poll`](#nr-poll) | const |  |
| [`__NR_lseek`](#nr-lseek) | const |  |
| [`__NR_mmap`](#nr-mmap) | const |  |
| [`__NR_mprotect`](#nr-mprotect) | const |  |
| [`__NR_munmap`](#nr-munmap) | const |  |
| [`__NR_brk`](#nr-brk) | const |  |
| [`__NR_rt_sigaction`](#nr-rt-sigaction) | const |  |
| [`__NR_rt_sigprocmask`](#nr-rt-sigprocmask) | const |  |
| [`__NR_rt_sigreturn`](#nr-rt-sigreturn) | const |  |
| [`__NR_ioctl`](#nr-ioctl) | const |  |
| [`__NR_pread64`](#nr-pread64) | const |  |
| [`__NR_pwrite64`](#nr-pwrite64) | const |  |
| [`__NR_readv`](#nr-readv) | const |  |
| [`__NR_writev`](#nr-writev) | const |  |
| [`__NR_access`](#nr-access) | const |  |
| [`__NR_pipe`](#nr-pipe) | const |  |
| [`__NR_select`](#nr-select) | const |  |
| [`__NR_sched_yield`](#nr-sched-yield) | const |  |
| [`__NR_mremap`](#nr-mremap) | const |  |
| [`__NR_msync`](#nr-msync) | const |  |
| [`__NR_mincore`](#nr-mincore) | const |  |
| [`__NR_madvise`](#nr-madvise) | const |  |
| [`__NR_shmget`](#nr-shmget) | const |  |
| [`__NR_shmat`](#nr-shmat) | const |  |
| [`__NR_shmctl`](#nr-shmctl) | const |  |
| [`__NR_dup`](#nr-dup) | const |  |
| [`__NR_dup2`](#nr-dup2) | const |  |
| [`__NR_pause`](#nr-pause) | const |  |
| [`__NR_nanosleep`](#nr-nanosleep) | const |  |
| [`__NR_getitimer`](#nr-getitimer) | const |  |
| [`__NR_alarm`](#nr-alarm) | const |  |
| [`__NR_setitimer`](#nr-setitimer) | const |  |
| [`__NR_getpid`](#nr-getpid) | const |  |
| [`__NR_sendfile`](#nr-sendfile) | const |  |
| [`__NR_socket`](#nr-socket) | const |  |
| [`__NR_connect`](#nr-connect) | const |  |
| [`__NR_accept`](#nr-accept) | const |  |
| [`__NR_sendto`](#nr-sendto) | const |  |
| [`__NR_recvfrom`](#nr-recvfrom) | const |  |
| [`__NR_sendmsg`](#nr-sendmsg) | const |  |
| [`__NR_recvmsg`](#nr-recvmsg) | const |  |
| [`__NR_shutdown`](#nr-shutdown) | const |  |
| [`__NR_bind`](#nr-bind) | const |  |
| [`__NR_listen`](#nr-listen) | const |  |
| [`__NR_getsockname`](#nr-getsockname) | const |  |
| [`__NR_getpeername`](#nr-getpeername) | const |  |
| [`__NR_socketpair`](#nr-socketpair) | const |  |
| [`__NR_setsockopt`](#nr-setsockopt) | const |  |
| [`__NR_getsockopt`](#nr-getsockopt) | const |  |
| [`__NR_clone`](#nr-clone) | const |  |
| [`__NR_fork`](#nr-fork) | const |  |
| [`__NR_vfork`](#nr-vfork) | const |  |
| [`__NR_execve`](#nr-execve) | const |  |
| [`__NR_exit`](#nr-exit) | const |  |
| [`__NR_wait4`](#nr-wait4) | const |  |
| [`__NR_kill`](#nr-kill) | const |  |
| [`__NR_uname`](#nr-uname) | const |  |
| [`__NR_semget`](#nr-semget) | const |  |
| [`__NR_semop`](#nr-semop) | const |  |
| [`__NR_semctl`](#nr-semctl) | const |  |
| [`__NR_shmdt`](#nr-shmdt) | const |  |
| [`__NR_msgget`](#nr-msgget) | const |  |
| [`__NR_msgsnd`](#nr-msgsnd) | const |  |
| [`__NR_msgrcv`](#nr-msgrcv) | const |  |
| [`__NR_msgctl`](#nr-msgctl) | const |  |
| [`__NR_fcntl`](#nr-fcntl) | const |  |
| [`__NR_flock`](#nr-flock) | const |  |
| [`__NR_fsync`](#nr-fsync) | const |  |
| [`__NR_fdatasync`](#nr-fdatasync) | const |  |
| [`__NR_truncate`](#nr-truncate) | const |  |
| [`__NR_ftruncate`](#nr-ftruncate) | const |  |
| [`__NR_getdents`](#nr-getdents) | const |  |
| [`__NR_getcwd`](#nr-getcwd) | const |  |
| [`__NR_chdir`](#nr-chdir) | const |  |
| [`__NR_fchdir`](#nr-fchdir) | const |  |
| [`__NR_rename`](#nr-rename) | const |  |
| [`__NR_mkdir`](#nr-mkdir) | const |  |
| [`__NR_rmdir`](#nr-rmdir) | const |  |
| [`__NR_creat`](#nr-creat) | const |  |
| [`__NR_link`](#nr-link) | const |  |
| [`__NR_unlink`](#nr-unlink) | const |  |
| [`__NR_symlink`](#nr-symlink) | const |  |
| [`__NR_readlink`](#nr-readlink) | const |  |
| [`__NR_chmod`](#nr-chmod) | const |  |
| [`__NR_fchmod`](#nr-fchmod) | const |  |
| [`__NR_chown`](#nr-chown) | const |  |
| [`__NR_fchown`](#nr-fchown) | const |  |
| [`__NR_lchown`](#nr-lchown) | const |  |
| [`__NR_umask`](#nr-umask) | const |  |
| [`__NR_gettimeofday`](#nr-gettimeofday) | const |  |
| [`__NR_getrlimit`](#nr-getrlimit) | const |  |
| [`__NR_getrusage`](#nr-getrusage) | const |  |
| [`__NR_sysinfo`](#nr-sysinfo) | const |  |
| [`__NR_times`](#nr-times) | const |  |
| [`__NR_ptrace`](#nr-ptrace) | const |  |
| [`__NR_getuid`](#nr-getuid) | const |  |
| [`__NR_syslog`](#nr-syslog) | const |  |
| [`__NR_getgid`](#nr-getgid) | const |  |
| [`__NR_setuid`](#nr-setuid) | const |  |
| [`__NR_setgid`](#nr-setgid) | const |  |
| [`__NR_geteuid`](#nr-geteuid) | const |  |
| [`__NR_getegid`](#nr-getegid) | const |  |
| [`__NR_setpgid`](#nr-setpgid) | const |  |
| [`__NR_getppid`](#nr-getppid) | const |  |
| [`__NR_getpgrp`](#nr-getpgrp) | const |  |
| [`__NR_setsid`](#nr-setsid) | const |  |
| [`__NR_setreuid`](#nr-setreuid) | const |  |
| [`__NR_setregid`](#nr-setregid) | const |  |
| [`__NR_getgroups`](#nr-getgroups) | const |  |
| [`__NR_setgroups`](#nr-setgroups) | const |  |
| [`__NR_setresuid`](#nr-setresuid) | const |  |
| [`__NR_getresuid`](#nr-getresuid) | const |  |
| [`__NR_setresgid`](#nr-setresgid) | const |  |
| [`__NR_getresgid`](#nr-getresgid) | const |  |
| [`__NR_getpgid`](#nr-getpgid) | const |  |
| [`__NR_setfsuid`](#nr-setfsuid) | const |  |
| [`__NR_setfsgid`](#nr-setfsgid) | const |  |
| [`__NR_getsid`](#nr-getsid) | const |  |
| [`__NR_capget`](#nr-capget) | const |  |
| [`__NR_capset`](#nr-capset) | const |  |
| [`__NR_rt_sigpending`](#nr-rt-sigpending) | const |  |
| [`__NR_rt_sigtimedwait`](#nr-rt-sigtimedwait) | const |  |
| [`__NR_rt_sigqueueinfo`](#nr-rt-sigqueueinfo) | const |  |
| [`__NR_rt_sigsuspend`](#nr-rt-sigsuspend) | const |  |
| [`__NR_sigaltstack`](#nr-sigaltstack) | const |  |
| [`__NR_utime`](#nr-utime) | const |  |
| [`__NR_mknod`](#nr-mknod) | const |  |
| [`__NR_uselib`](#nr-uselib) | const |  |
| [`__NR_personality`](#nr-personality) | const |  |
| [`__NR_ustat`](#nr-ustat) | const |  |
| [`__NR_statfs`](#nr-statfs) | const |  |
| [`__NR_fstatfs`](#nr-fstatfs) | const |  |
| [`__NR_sysfs`](#nr-sysfs) | const |  |
| [`__NR_getpriority`](#nr-getpriority) | const |  |
| [`__NR_setpriority`](#nr-setpriority) | const |  |
| [`__NR_sched_setparam`](#nr-sched-setparam) | const |  |
| [`__NR_sched_getparam`](#nr-sched-getparam) | const |  |
| [`__NR_sched_setscheduler`](#nr-sched-setscheduler) | const |  |
| [`__NR_sched_getscheduler`](#nr-sched-getscheduler) | const |  |
| [`__NR_sched_get_priority_max`](#nr-sched-get-priority-max) | const |  |
| [`__NR_sched_get_priority_min`](#nr-sched-get-priority-min) | const |  |
| [`__NR_sched_rr_get_interval`](#nr-sched-rr-get-interval) | const |  |
| [`__NR_mlock`](#nr-mlock) | const |  |
| [`__NR_munlock`](#nr-munlock) | const |  |
| [`__NR_mlockall`](#nr-mlockall) | const |  |
| [`__NR_munlockall`](#nr-munlockall) | const |  |
| [`__NR_vhangup`](#nr-vhangup) | const |  |
| [`__NR_modify_ldt`](#nr-modify-ldt) | const |  |
| [`__NR_pivot_root`](#nr-pivot-root) | const |  |
| [`__NR__sysctl`](#nr-sysctl) | const |  |
| [`__NR_prctl`](#nr-prctl) | const |  |
| [`__NR_arch_prctl`](#nr-arch-prctl) | const |  |
| [`__NR_adjtimex`](#nr-adjtimex) | const |  |
| [`__NR_setrlimit`](#nr-setrlimit) | const |  |
| [`__NR_chroot`](#nr-chroot) | const |  |
| [`__NR_sync`](#nr-sync) | const |  |
| [`__NR_acct`](#nr-acct) | const |  |
| [`__NR_settimeofday`](#nr-settimeofday) | const |  |
| [`__NR_mount`](#nr-mount) | const |  |
| [`__NR_umount2`](#nr-umount2) | const |  |
| [`__NR_swapon`](#nr-swapon) | const |  |
| [`__NR_swapoff`](#nr-swapoff) | const |  |
| [`__NR_reboot`](#nr-reboot) | const |  |
| [`__NR_sethostname`](#nr-sethostname) | const |  |
| [`__NR_setdomainname`](#nr-setdomainname) | const |  |
| [`__NR_iopl`](#nr-iopl) | const |  |
| [`__NR_ioperm`](#nr-ioperm) | const |  |
| [`__NR_create_module`](#nr-create-module) | const |  |
| [`__NR_init_module`](#nr-init-module) | const |  |
| [`__NR_delete_module`](#nr-delete-module) | const |  |
| [`__NR_get_kernel_syms`](#nr-get-kernel-syms) | const |  |
| [`__NR_query_module`](#nr-query-module) | const |  |
| [`__NR_quotactl`](#nr-quotactl) | const |  |
| [`__NR_nfsservctl`](#nr-nfsservctl) | const |  |
| [`__NR_getpmsg`](#nr-getpmsg) | const |  |
| [`__NR_putpmsg`](#nr-putpmsg) | const |  |
| [`__NR_afs_syscall`](#nr-afs-syscall) | const |  |
| [`__NR_tuxcall`](#nr-tuxcall) | const |  |
| [`__NR_security`](#nr-security) | const |  |
| [`__NR_gettid`](#nr-gettid) | const |  |
| [`__NR_readahead`](#nr-readahead) | const |  |
| [`__NR_setxattr`](#nr-setxattr) | const |  |
| [`__NR_lsetxattr`](#nr-lsetxattr) | const |  |
| [`__NR_fsetxattr`](#nr-fsetxattr) | const |  |
| [`__NR_getxattr`](#nr-getxattr) | const |  |
| [`__NR_lgetxattr`](#nr-lgetxattr) | const |  |
| [`__NR_fgetxattr`](#nr-fgetxattr) | const |  |
| [`__NR_listxattr`](#nr-listxattr) | const |  |
| [`__NR_llistxattr`](#nr-llistxattr) | const |  |
| [`__NR_flistxattr`](#nr-flistxattr) | const |  |
| [`__NR_removexattr`](#nr-removexattr) | const |  |
| [`__NR_lremovexattr`](#nr-lremovexattr) | const |  |
| [`__NR_fremovexattr`](#nr-fremovexattr) | const |  |
| [`__NR_tkill`](#nr-tkill) | const |  |
| [`__NR_time`](#nr-time) | const |  |
| [`__NR_futex`](#nr-futex) | const |  |
| [`__NR_sched_setaffinity`](#nr-sched-setaffinity) | const |  |
| [`__NR_sched_getaffinity`](#nr-sched-getaffinity) | const |  |
| [`__NR_set_thread_area`](#nr-set-thread-area) | const |  |
| [`__NR_io_setup`](#nr-io-setup) | const |  |
| [`__NR_io_destroy`](#nr-io-destroy) | const |  |
| [`__NR_io_getevents`](#nr-io-getevents) | const |  |
| [`__NR_io_submit`](#nr-io-submit) | const |  |
| [`__NR_io_cancel`](#nr-io-cancel) | const |  |
| [`__NR_get_thread_area`](#nr-get-thread-area) | const |  |
| [`__NR_lookup_dcookie`](#nr-lookup-dcookie) | const |  |
| [`__NR_epoll_create`](#nr-epoll-create) | const |  |
| [`__NR_epoll_ctl_old`](#nr-epoll-ctl-old) | const |  |
| [`__NR_epoll_wait_old`](#nr-epoll-wait-old) | const |  |
| [`__NR_remap_file_pages`](#nr-remap-file-pages) | const |  |
| [`__NR_getdents64`](#nr-getdents64) | const |  |
| [`__NR_set_tid_address`](#nr-set-tid-address) | const |  |
| [`__NR_restart_syscall`](#nr-restart-syscall) | const |  |
| [`__NR_semtimedop`](#nr-semtimedop) | const |  |
| [`__NR_fadvise64`](#nr-fadvise64) | const |  |
| [`__NR_timer_create`](#nr-timer-create) | const |  |
| [`__NR_timer_settime`](#nr-timer-settime) | const |  |
| [`__NR_timer_gettime`](#nr-timer-gettime) | const |  |
| [`__NR_timer_getoverrun`](#nr-timer-getoverrun) | const |  |
| [`__NR_timer_delete`](#nr-timer-delete) | const |  |
| [`__NR_clock_settime`](#nr-clock-settime) | const |  |
| [`__NR_clock_gettime`](#nr-clock-gettime) | const |  |
| [`__NR_clock_getres`](#nr-clock-getres) | const |  |
| [`__NR_clock_nanosleep`](#nr-clock-nanosleep) | const |  |
| [`__NR_exit_group`](#nr-exit-group) | const |  |
| [`__NR_epoll_wait`](#nr-epoll-wait) | const |  |
| [`__NR_epoll_ctl`](#nr-epoll-ctl) | const |  |
| [`__NR_tgkill`](#nr-tgkill) | const |  |
| [`__NR_utimes`](#nr-utimes) | const |  |
| [`__NR_vserver`](#nr-vserver) | const |  |
| [`__NR_mbind`](#nr-mbind) | const |  |
| [`__NR_set_mempolicy`](#nr-set-mempolicy) | const |  |
| [`__NR_get_mempolicy`](#nr-get-mempolicy) | const |  |
| [`__NR_mq_open`](#nr-mq-open) | const |  |
| [`__NR_mq_unlink`](#nr-mq-unlink) | const |  |
| [`__NR_mq_timedsend`](#nr-mq-timedsend) | const |  |
| [`__NR_mq_timedreceive`](#nr-mq-timedreceive) | const |  |
| [`__NR_mq_notify`](#nr-mq-notify) | const |  |
| [`__NR_mq_getsetattr`](#nr-mq-getsetattr) | const |  |
| [`__NR_kexec_load`](#nr-kexec-load) | const |  |
| [`__NR_waitid`](#nr-waitid) | const |  |
| [`__NR_add_key`](#nr-add-key) | const |  |
| [`__NR_request_key`](#nr-request-key) | const |  |
| [`__NR_keyctl`](#nr-keyctl) | const |  |
| [`__NR_ioprio_set`](#nr-ioprio-set) | const |  |
| [`__NR_ioprio_get`](#nr-ioprio-get) | const |  |
| [`__NR_inotify_init`](#nr-inotify-init) | const |  |
| [`__NR_inotify_add_watch`](#nr-inotify-add-watch) | const |  |
| [`__NR_inotify_rm_watch`](#nr-inotify-rm-watch) | const |  |
| [`__NR_migrate_pages`](#nr-migrate-pages) | const |  |
| [`__NR_openat`](#nr-openat) | const |  |
| [`__NR_mkdirat`](#nr-mkdirat) | const |  |
| [`__NR_mknodat`](#nr-mknodat) | const |  |
| [`__NR_fchownat`](#nr-fchownat) | const |  |
| [`__NR_futimesat`](#nr-futimesat) | const |  |
| [`__NR_newfstatat`](#nr-newfstatat) | const |  |
| [`__NR_unlinkat`](#nr-unlinkat) | const |  |
| [`__NR_renameat`](#nr-renameat) | const |  |
| [`__NR_linkat`](#nr-linkat) | const |  |
| [`__NR_symlinkat`](#nr-symlinkat) | const |  |
| [`__NR_readlinkat`](#nr-readlinkat) | const |  |
| [`__NR_fchmodat`](#nr-fchmodat) | const |  |
| [`__NR_faccessat`](#nr-faccessat) | const |  |
| [`__NR_pselect6`](#nr-pselect6) | const |  |
| [`__NR_ppoll`](#nr-ppoll) | const |  |
| [`__NR_unshare`](#nr-unshare) | const |  |
| [`__NR_set_robust_list`](#nr-set-robust-list) | const |  |
| [`__NR_get_robust_list`](#nr-get-robust-list) | const |  |
| [`__NR_splice`](#nr-splice) | const |  |
| [`__NR_tee`](#nr-tee) | const |  |
| [`__NR_sync_file_range`](#nr-sync-file-range) | const |  |
| [`__NR_vmsplice`](#nr-vmsplice) | const |  |
| [`__NR_move_pages`](#nr-move-pages) | const |  |
| [`__NR_utimensat`](#nr-utimensat) | const |  |
| [`__NR_epoll_pwait`](#nr-epoll-pwait) | const |  |
| [`__NR_signalfd`](#nr-signalfd) | const |  |
| [`__NR_timerfd_create`](#nr-timerfd-create) | const |  |
| [`__NR_eventfd`](#nr-eventfd) | const |  |
| [`__NR_fallocate`](#nr-fallocate) | const |  |
| [`__NR_timerfd_settime`](#nr-timerfd-settime) | const |  |
| [`__NR_timerfd_gettime`](#nr-timerfd-gettime) | const |  |
| [`__NR_accept4`](#nr-accept4) | const |  |
| [`__NR_signalfd4`](#nr-signalfd4) | const |  |
| [`__NR_eventfd2`](#nr-eventfd2) | const |  |
| [`__NR_epoll_create1`](#nr-epoll-create1) | const |  |
| [`__NR_dup3`](#nr-dup3) | const |  |
| [`__NR_pipe2`](#nr-pipe2) | const |  |
| [`__NR_inotify_init1`](#nr-inotify-init1) | const |  |
| [`__NR_preadv`](#nr-preadv) | const |  |
| [`__NR_pwritev`](#nr-pwritev) | const |  |
| [`__NR_rt_tgsigqueueinfo`](#nr-rt-tgsigqueueinfo) | const |  |
| [`__NR_perf_event_open`](#nr-perf-event-open) | const |  |
| [`__NR_recvmmsg`](#nr-recvmmsg) | const |  |
| [`__NR_fanotify_init`](#nr-fanotify-init) | const |  |
| [`__NR_fanotify_mark`](#nr-fanotify-mark) | const |  |
| [`__NR_prlimit64`](#nr-prlimit64) | const |  |
| [`__NR_name_to_handle_at`](#nr-name-to-handle-at) | const |  |
| [`__NR_open_by_handle_at`](#nr-open-by-handle-at) | const |  |
| [`__NR_clock_adjtime`](#nr-clock-adjtime) | const |  |
| [`__NR_syncfs`](#nr-syncfs) | const |  |
| [`__NR_sendmmsg`](#nr-sendmmsg) | const |  |
| [`__NR_setns`](#nr-setns) | const |  |
| [`__NR_getcpu`](#nr-getcpu) | const |  |
| [`__NR_process_vm_readv`](#nr-process-vm-readv) | const |  |
| [`__NR_process_vm_writev`](#nr-process-vm-writev) | const |  |
| [`__NR_kcmp`](#nr-kcmp) | const |  |
| [`__NR_finit_module`](#nr-finit-module) | const |  |
| [`__NR_sched_setattr`](#nr-sched-setattr) | const |  |
| [`__NR_sched_getattr`](#nr-sched-getattr) | const |  |
| [`__NR_renameat2`](#nr-renameat2) | const |  |
| [`__NR_seccomp`](#nr-seccomp) | const |  |
| [`__NR_getrandom`](#nr-getrandom) | const |  |
| [`__NR_memfd_create`](#nr-memfd-create) | const |  |
| [`__NR_kexec_file_load`](#nr-kexec-file-load) | const |  |
| [`__NR_bpf`](#nr-bpf) | const |  |
| [`__NR_execveat`](#nr-execveat) | const |  |
| [`__NR_userfaultfd`](#nr-userfaultfd) | const |  |
| [`__NR_membarrier`](#nr-membarrier) | const |  |
| [`__NR_mlock2`](#nr-mlock2) | const |  |
| [`__NR_copy_file_range`](#nr-copy-file-range) | const |  |
| [`__NR_preadv2`](#nr-preadv2) | const |  |
| [`__NR_pwritev2`](#nr-pwritev2) | const |  |
| [`__NR_pkey_mprotect`](#nr-pkey-mprotect) | const |  |
| [`__NR_pkey_alloc`](#nr-pkey-alloc) | const |  |
| [`__NR_pkey_free`](#nr-pkey-free) | const |  |
| [`__NR_statx`](#nr-statx) | const |  |
| [`__NR_io_pgetevents`](#nr-io-pgetevents) | const |  |
| [`__NR_rseq`](#nr-rseq) | const |  |
| [`__NR_uretprobe`](#nr-uretprobe) | const |  |
| [`__NR_pidfd_send_signal`](#nr-pidfd-send-signal) | const |  |
| [`__NR_io_uring_setup`](#nr-io-uring-setup) | const |  |
| [`__NR_io_uring_enter`](#nr-io-uring-enter) | const |  |
| [`__NR_io_uring_register`](#nr-io-uring-register) | const |  |
| [`__NR_open_tree`](#nr-open-tree) | const |  |
| [`__NR_move_mount`](#nr-move-mount) | const |  |
| [`__NR_fsopen`](#nr-fsopen) | const |  |
| [`__NR_fsconfig`](#nr-fsconfig) | const |  |
| [`__NR_fsmount`](#nr-fsmount) | const |  |
| [`__NR_fspick`](#nr-fspick) | const |  |
| [`__NR_pidfd_open`](#nr-pidfd-open) | const |  |
| [`__NR_clone3`](#nr-clone3) | const |  |
| [`__NR_close_range`](#nr-close-range) | const |  |
| [`__NR_openat2`](#nr-openat2) | const |  |
| [`__NR_pidfd_getfd`](#nr-pidfd-getfd) | const |  |
| [`__NR_faccessat2`](#nr-faccessat2) | const |  |
| [`__NR_process_madvise`](#nr-process-madvise) | const |  |
| [`__NR_epoll_pwait2`](#nr-epoll-pwait2) | const |  |
| [`__NR_mount_setattr`](#nr-mount-setattr) | const |  |
| [`__NR_quotactl_fd`](#nr-quotactl-fd) | const |  |
| [`__NR_landlock_create_ruleset`](#nr-landlock-create-ruleset) | const |  |
| [`__NR_landlock_add_rule`](#nr-landlock-add-rule) | const |  |
| [`__NR_landlock_restrict_self`](#nr-landlock-restrict-self) | const |  |
| [`__NR_memfd_secret`](#nr-memfd-secret) | const |  |
| [`__NR_process_mrelease`](#nr-process-mrelease) | const |  |
| [`__NR_futex_waitv`](#nr-futex-waitv) | const |  |
| [`__NR_set_mempolicy_home_node`](#nr-set-mempolicy-home-node) | const |  |
| [`__NR_cachestat`](#nr-cachestat) | const |  |
| [`__NR_fchmodat2`](#nr-fchmodat2) | const |  |
| [`__NR_map_shadow_stack`](#nr-map-shadow-stack) | const |  |
| [`__NR_futex_wake`](#nr-futex-wake) | const |  |
| [`__NR_futex_wait`](#nr-futex-wait) | const |  |
| [`__NR_futex_requeue`](#nr-futex-requeue) | const |  |
| [`__NR_statmount`](#nr-statmount) | const |  |
| [`__NR_listmount`](#nr-listmount) | const |  |
| [`__NR_lsm_get_self_attr`](#nr-lsm-get-self-attr) | const |  |
| [`__NR_lsm_set_self_attr`](#nr-lsm-set-self-attr) | const |  |
| [`__NR_lsm_list_modules`](#nr-lsm-list-modules) | const |  |
| [`__NR_mseal`](#nr-mseal) | const |  |
| [`__NR_setxattrat`](#nr-setxattrat) | const |  |
| [`__NR_getxattrat`](#nr-getxattrat) | const |  |
| [`__NR_listxattrat`](#nr-listxattrat) | const |  |
| [`__NR_removexattrat`](#nr-removexattrat) | const |  |
| [`__NR_open_tree_attr`](#nr-open-tree-attr) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`__WNOTHREAD`](#wnothread) | const |  |
| [`__WALL`](#wall) | const |  |
| [`__WCLONE`](#wclone) | const |  |
| [`P_ALL`](#p-all) | const |  |
| [`P_PID`](#p-pid) | const |  |
| [`P_PGID`](#p-pgid) | const |  |
| [`P_PIDFD`](#p-pidfd) | const |  |
| [`XATTR_CREATE`](#xattr-create) | const |  |
| [`XATTR_REPLACE`](#xattr-replace) | const |  |
| [`XATTR_OS2_PREFIX`](#xattr-os2-prefix) | const |  |
| [`XATTR_MAC_OSX_PREFIX`](#xattr-mac-osx-prefix) | const |  |
| [`XATTR_BTRFS_PREFIX`](#xattr-btrfs-prefix) | const |  |
| [`XATTR_HURD_PREFIX`](#xattr-hurd-prefix) | const |  |
| [`XATTR_SECURITY_PREFIX`](#xattr-security-prefix) | const |  |
| [`XATTR_SYSTEM_PREFIX`](#xattr-system-prefix) | const |  |
| [`XATTR_TRUSTED_PREFIX`](#xattr-trusted-prefix) | const |  |
| [`XATTR_USER_PREFIX`](#xattr-user-prefix) | const |  |
| [`XATTR_EVM_SUFFIX`](#xattr-evm-suffix) | const |  |
| [`XATTR_NAME_EVM`](#xattr-name-evm) | const |  |
| [`XATTR_IMA_SUFFIX`](#xattr-ima-suffix) | const |  |
| [`XATTR_NAME_IMA`](#xattr-name-ima) | const |  |
| [`XATTR_SELINUX_SUFFIX`](#xattr-selinux-suffix) | const |  |
| [`XATTR_NAME_SELINUX`](#xattr-name-selinux) | const |  |
| [`XATTR_SMACK_SUFFIX`](#xattr-smack-suffix) | const |  |
| [`XATTR_SMACK_IPIN`](#xattr-smack-ipin) | const |  |
| [`XATTR_SMACK_IPOUT`](#xattr-smack-ipout) | const |  |
| [`XATTR_SMACK_EXEC`](#xattr-smack-exec) | const |  |
| [`XATTR_SMACK_TRANSMUTE`](#xattr-smack-transmute) | const |  |
| [`XATTR_SMACK_MMAP`](#xattr-smack-mmap) | const |  |
| [`XATTR_NAME_SMACK`](#xattr-name-smack) | const |  |
| [`XATTR_NAME_SMACKIPIN`](#xattr-name-smackipin) | const |  |
| [`XATTR_NAME_SMACKIPOUT`](#xattr-name-smackipout) | const |  |
| [`XATTR_NAME_SMACKEXEC`](#xattr-name-smackexec) | const |  |
| [`XATTR_NAME_SMACKTRANSMUTE`](#xattr-name-smacktransmute) | const |  |
| [`XATTR_NAME_SMACKMMAP`](#xattr-name-smackmmap) | const |  |
| [`XATTR_APPARMOR_SUFFIX`](#xattr-apparmor-suffix) | const |  |
| [`XATTR_NAME_APPARMOR`](#xattr-name-apparmor) | const |  |
| [`XATTR_CAPS_SUFFIX`](#xattr-caps-suffix) | const |  |
| [`XATTR_NAME_CAPS`](#xattr-name-caps) | const |  |
| [`XATTR_BPF_LSM_SUFFIX`](#xattr-bpf-lsm-suffix) | const |  |
| [`XATTR_NAME_BPF_LSM`](#xattr-name-bpf-lsm) | const |  |
| [`XATTR_POSIX_ACL_ACCESS`](#xattr-posix-acl-access) | const |  |
| [`XATTR_NAME_POSIX_ACL_ACCESS`](#xattr-name-posix-acl-access) | const |  |
| [`XATTR_POSIX_ACL_DEFAULT`](#xattr-posix-acl-default) | const |  |
| [`XATTR_NAME_POSIX_ACL_DEFAULT`](#xattr-name-posix-acl-default) | const |  |
| [`MFD_CLOEXEC`](#mfd-cloexec) | const |  |
| [`MFD_ALLOW_SEALING`](#mfd-allow-sealing) | const |  |
| [`MFD_HUGETLB`](#mfd-hugetlb) | const |  |
| [`MFD_NOEXEC_SEAL`](#mfd-noexec-seal) | const |  |
| [`MFD_EXEC`](#mfd-exec) | const |  |
| [`MFD_HUGE_SHIFT`](#mfd-huge-shift) | const |  |
| [`MFD_HUGE_MASK`](#mfd-huge-mask) | const |  |
| [`MFD_HUGE_64KB`](#mfd-huge-64kb) | const |  |
| [`MFD_HUGE_512KB`](#mfd-huge-512kb) | const |  |
| [`MFD_HUGE_1MB`](#mfd-huge-1mb) | const |  |
| [`MFD_HUGE_2MB`](#mfd-huge-2mb) | const |  |
| [`MFD_HUGE_8MB`](#mfd-huge-8mb) | const |  |
| [`MFD_HUGE_16MB`](#mfd-huge-16mb) | const |  |
| [`MFD_HUGE_32MB`](#mfd-huge-32mb) | const |  |
| [`MFD_HUGE_256MB`](#mfd-huge-256mb) | const |  |
| [`MFD_HUGE_512MB`](#mfd-huge-512mb) | const |  |
| [`MFD_HUGE_1GB`](#mfd-huge-1gb) | const |  |
| [`MFD_HUGE_2GB`](#mfd-huge-2gb) | const |  |
| [`MFD_HUGE_16GB`](#mfd-huge-16gb) | const |  |
| [`TFD_TIMER_ABSTIME`](#tfd-timer-abstime) | const |  |
| [`TFD_TIMER_CANCEL_ON_SET`](#tfd-timer-cancel-on-set) | const |  |
| [`TFD_CLOEXEC`](#tfd-cloexec) | const |  |
| [`TFD_NONBLOCK`](#tfd-nonblock) | const |  |
| [`USERFAULTFD_IOC`](#userfaultfd-ioc) | const |  |
| [`_UFFDIO_REGISTER`](#uffdio-register) | const |  |
| [`_UFFDIO_UNREGISTER`](#uffdio-unregister) | const |  |
| [`_UFFDIO_WAKE`](#uffdio-wake) | const |  |
| [`_UFFDIO_COPY`](#uffdio-copy) | const |  |
| [`_UFFDIO_ZEROPAGE`](#uffdio-zeropage) | const |  |
| [`_UFFDIO_MOVE`](#uffdio-move) | const |  |
| [`_UFFDIO_WRITEPROTECT`](#uffdio-writeprotect) | const |  |
| [`_UFFDIO_CONTINUE`](#uffdio-continue) | const |  |
| [`_UFFDIO_POISON`](#uffdio-poison) | const |  |
| [`_UFFDIO_API`](#uffdio-api) | const |  |
| [`UFFDIO`](#uffdio) | const |  |
| [`UFFD_EVENT_PAGEFAULT`](#uffd-event-pagefault) | const |  |
| [`UFFD_EVENT_FORK`](#uffd-event-fork) | const |  |
| [`UFFD_EVENT_REMAP`](#uffd-event-remap) | const |  |
| [`UFFD_EVENT_REMOVE`](#uffd-event-remove) | const |  |
| [`UFFD_EVENT_UNMAP`](#uffd-event-unmap) | const |  |
| [`UFFD_PAGEFAULT_FLAG_WRITE`](#uffd-pagefault-flag-write) | const |  |
| [`UFFD_PAGEFAULT_FLAG_WP`](#uffd-pagefault-flag-wp) | const |  |
| [`UFFD_PAGEFAULT_FLAG_MINOR`](#uffd-pagefault-flag-minor) | const |  |
| [`UFFD_FEATURE_PAGEFAULT_FLAG_WP`](#uffd-feature-pagefault-flag-wp) | const |  |
| [`UFFD_FEATURE_EVENT_FORK`](#uffd-feature-event-fork) | const |  |
| [`UFFD_FEATURE_EVENT_REMAP`](#uffd-feature-event-remap) | const |  |
| [`UFFD_FEATURE_EVENT_REMOVE`](#uffd-feature-event-remove) | const |  |
| [`UFFD_FEATURE_MISSING_HUGETLBFS`](#uffd-feature-missing-hugetlbfs) | const |  |
| [`UFFD_FEATURE_MISSING_SHMEM`](#uffd-feature-missing-shmem) | const |  |
| [`UFFD_FEATURE_EVENT_UNMAP`](#uffd-feature-event-unmap) | const |  |
| [`UFFD_FEATURE_SIGBUS`](#uffd-feature-sigbus) | const |  |
| [`UFFD_FEATURE_THREAD_ID`](#uffd-feature-thread-id) | const |  |
| [`UFFD_FEATURE_MINOR_HUGETLBFS`](#uffd-feature-minor-hugetlbfs) | const |  |
| [`UFFD_FEATURE_MINOR_SHMEM`](#uffd-feature-minor-shmem) | const |  |
| [`UFFD_FEATURE_EXACT_ADDRESS`](#uffd-feature-exact-address) | const |  |
| [`UFFD_FEATURE_WP_HUGETLBFS_SHMEM`](#uffd-feature-wp-hugetlbfs-shmem) | const |  |
| [`UFFD_FEATURE_WP_UNPOPULATED`](#uffd-feature-wp-unpopulated) | const |  |
| [`UFFD_FEATURE_POISON`](#uffd-feature-poison) | const |  |
| [`UFFD_FEATURE_WP_ASYNC`](#uffd-feature-wp-async) | const |  |
| [`UFFD_FEATURE_MOVE`](#uffd-feature-move) | const |  |
| [`UFFD_USER_MODE_ONLY`](#uffd-user-mode-only) | const |  |
| [`DT_UNKNOWN`](#dt-unknown) | const |  |
| [`DT_FIFO`](#dt-fifo) | const |  |
| [`DT_CHR`](#dt-chr) | const |  |
| [`DT_DIR`](#dt-dir) | const |  |
| [`DT_BLK`](#dt-blk) | const |  |
| [`DT_REG`](#dt-reg) | const |  |
| [`DT_LNK`](#dt-lnk) | const |  |
| [`DT_SOCK`](#dt-sock) | const |  |
| [`STAT_HAVE_NSEC`](#stat-have-nsec) | const |  |
| [`F_OK`](#f-ok) | const |  |
| [`R_OK`](#r-ok) | const |  |
| [`W_OK`](#w-ok) | const |  |
| [`X_OK`](#x-ok) | const |  |
| [`UTIME_NOW`](#utime-now) | const |  |
| [`UTIME_OMIT`](#utime-omit) | const |  |
| [`MNT_FORCE`](#mnt-force) | const |  |
| [`MNT_DETACH`](#mnt-detach) | const |  |
| [`MNT_EXPIRE`](#mnt-expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount-nofollow) | const |  |
| [`UMOUNT_UNUSED`](#umount-unused) | const |  |
| [`STDIN_FILENO`](#stdin-fileno) | const |  |
| [`STDOUT_FILENO`](#stdout-fileno) | const |  |
| [`STDERR_FILENO`](#stderr-fileno) | const |  |
| [`RWF_HIPRI`](#rwf-hipri) | const |  |
| [`RWF_DSYNC`](#rwf-dsync) | const |  |
| [`RWF_SYNC`](#rwf-sync) | const |  |
| [`RWF_NOWAIT`](#rwf-nowait) | const |  |
| [`RWF_APPEND`](#rwf-append) | const |  |
| [`EFD_SEMAPHORE`](#efd-semaphore) | const |  |
| [`EFD_CLOEXEC`](#efd-cloexec) | const |  |
| [`EFD_NONBLOCK`](#efd-nonblock) | const |  |
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
| [`TFD_SHARED_FCNTL_FLAGS`](#tfd-shared-fcntl-flags) | const |  |
| [`TFD_CREATE_FLAGS`](#tfd-create-flags) | const |  |
| [`TFD_SETTIME_FLAGS`](#tfd-settime-flags) | const |  |
| [`ARCH_SET_FS`](#arch-set-fs) | const |  |
| [`UFFD_API`](#uffd-api) | const |  |
| [`UFFDIO_REGISTER_MODE_MISSING`](#uffdio-register-mode-missing) | const |  |
| [`UFFDIO_REGISTER_MODE_WP`](#uffdio-register-mode-wp) | const |  |
| [`UFFDIO_REGISTER_MODE_MINOR`](#uffdio-register-mode-minor) | const |  |
| [`UFFDIO_COPY_MODE_DONTWAKE`](#uffdio-copy-mode-dontwake) | const |  |
| [`UFFDIO_COPY_MODE_WP`](#uffdio-copy-mode-wp) | const |  |
| [`UFFDIO_ZEROPAGE_MODE_DONTWAKE`](#uffdio-zeropage-mode-dontwake) | const |  |
| [`SPLICE_F_MOVE`](#splice-f-move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice-f-nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice-f-more) | const |  |
| [`SPLICE_F_GIFT`](#splice-f-gift) | const |  |
| [`_NSIG`](#nsig) | const |  |

## Structs

### `__BindgenBitfieldUnit<Storage>`

```rust
struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:72-74`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L72-L74)*

#### Implementations

- <span id="bindgenbitfieldunit-new"></span>`const fn new(storage: Storage) -> Self`

#### Trait Implementations

##### `impl Any for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Storage: clone::Clone> Clone for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-clone"></span>`fn clone(&self) -> __BindgenBitfieldUnit<Storage>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl CloneToUninit for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Storage: marker::Copy> Copy for __BindgenBitfieldUnit<Storage>`

##### `impl<Storage: fmt::Debug> Debug for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Storage: default::Default> Default for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-default"></span>`fn default() -> __BindgenBitfieldUnit<Storage>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::Eq> Eq for __BindgenBitfieldUnit<Storage>`

##### `impl<T> From for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Storage: hash::Hash> Hash for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Storage: cmp::Ord> Ord for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-ord-cmp"></span>`fn cmp(&self, other: &__BindgenBitfieldUnit<Storage>) -> cmp::Ordering` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::PartialEq> PartialEq for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-partialeq-eq"></span>`fn eq(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage: cmp::PartialOrd> PartialOrd for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &__BindgenBitfieldUnit<Storage>) -> option::Option<cmp::Ordering>` — [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

##### `impl<Storage> StructuralPartialEq for __BindgenBitfieldUnit<Storage>`

##### `impl<U> TryFrom for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bindgenbitfieldunit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __BindgenBitfieldUnit<Storage>`

- <span id="bindgenbitfieldunit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bindgenbitfieldunit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__IncompleteArrayField<T>`

```rust
struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:77`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L77)*

#### Implementations

- <span id="incompletearrayfield-new"></span>`const fn new() -> Self`

- <span id="incompletearrayfield-as-ptr"></span>`fn as_ptr(&self) -> *const T`

- <span id="incompletearrayfield-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut T`

- <span id="incompletearrayfield-as-slice"></span>`unsafe fn as_slice(&self, len: usize) -> &[T]`

- <span id="incompletearrayfield-as-mut-slice"></span>`unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T]`

#### Trait Implementations

##### `impl<T> Any for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-debug-fmt"></span>`fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl<T: default::Default> Default for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-default"></span>`fn default() -> __IncompleteArrayField<T>` — [`__IncompleteArrayField`](#incompletearrayfield)

##### `impl<T> From for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="incompletearrayfield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for __IncompleteArrayField<T>`

- <span id="incompletearrayfield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="incompletearrayfield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_fd_set`

```rust
struct __kernel_fd_set {
    pub fds_bits: [crate::ctypes::c_ulong; 16],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:80-82`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L80-L82)*

#### Trait Implementations

##### `impl Any for __kernel_fd_set`

- <span id="kernel-fd-set-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_fd_set`

- <span id="kernel-fd-set-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_fd_set`

- <span id="kernel-fd-set-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_fd_set`

- <span id="kernel-fd-set-clone"></span>`fn clone(&self) -> __kernel_fd_set` — [`__kernel_fd_set`](#kernel-fd-set)

##### `impl CloneToUninit for __kernel_fd_set`

- <span id="kernel-fd-set-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_fd_set`

##### `impl Debug for __kernel_fd_set`

- <span id="kernel-fd-set-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_fd_set`

- <span id="kernel-fd-set-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_fd_set`

- <span id="kernel-fd-set-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_fd_set`

- <span id="kernel-fd-set-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-fd-set-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_fd_set`

- <span id="kernel-fd-set-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-fd-set-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_fsid_t`

```rust
struct __kernel_fsid_t {
    pub val: [crate::ctypes::c_int; 2],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:85-87`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L85-L87)*

#### Trait Implementations

##### `impl Any for __kernel_fsid_t`

- <span id="kernel-fsid-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_fsid_t`

- <span id="kernel-fsid-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_fsid_t`

- <span id="kernel-fsid-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_fsid_t`

- <span id="kernel-fsid-t-clone"></span>`fn clone(&self) -> __kernel_fsid_t` — [`__kernel_fsid_t`](#kernel-fsid-t)

##### `impl CloneToUninit for __kernel_fsid_t`

- <span id="kernel-fsid-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_fsid_t`

##### `impl Debug for __kernel_fsid_t`

- <span id="kernel-fsid-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_fsid_t`

- <span id="kernel-fsid-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_fsid_t`

- <span id="kernel-fsid-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_fsid_t`

- <span id="kernel-fsid-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-fsid-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_fsid_t`

- <span id="kernel-fsid-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-fsid-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__user_cap_header_struct`

```rust
struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: crate::ctypes::c_int,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:90-93`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L90-L93)*

#### Trait Implementations

##### `impl Any for __user_cap_header_struct`

- <span id="user-cap-header-struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __user_cap_header_struct`

- <span id="user-cap-header-struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __user_cap_header_struct`

- <span id="user-cap-header-struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __user_cap_header_struct`

- <span id="user-cap-header-struct-clone"></span>`fn clone(&self) -> __user_cap_header_struct` — [`__user_cap_header_struct`](#user-cap-header-struct)

##### `impl CloneToUninit for __user_cap_header_struct`

- <span id="user-cap-header-struct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __user_cap_header_struct`

##### `impl Debug for __user_cap_header_struct`

- <span id="user-cap-header-struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __user_cap_header_struct`

- <span id="user-cap-header-struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __user_cap_header_struct`

- <span id="user-cap-header-struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __user_cap_header_struct`

- <span id="user-cap-header-struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="user-cap-header-struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __user_cap_header_struct`

- <span id="user-cap-header-struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="user-cap-header-struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__user_cap_data_struct`

```rust
struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:96-100`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L96-L100)*

#### Trait Implementations

##### `impl Any for __user_cap_data_struct`

- <span id="user-cap-data-struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __user_cap_data_struct`

- <span id="user-cap-data-struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __user_cap_data_struct`

- <span id="user-cap-data-struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __user_cap_data_struct`

- <span id="user-cap-data-struct-clone"></span>`fn clone(&self) -> __user_cap_data_struct` — [`__user_cap_data_struct`](#user-cap-data-struct)

##### `impl CloneToUninit for __user_cap_data_struct`

- <span id="user-cap-data-struct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __user_cap_data_struct`

##### `impl Debug for __user_cap_data_struct`

- <span id="user-cap-data-struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __user_cap_data_struct`

- <span id="user-cap-data-struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __user_cap_data_struct`

- <span id="user-cap-data-struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __user_cap_data_struct`

- <span id="user-cap-data-struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="user-cap-data-struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __user_cap_data_struct`

- <span id="user-cap-data-struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="user-cap-data-struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `vfs_cap_data`

```rust
struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data__bindgen_ty_1; 2],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:103-106`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L103-L106)*

#### Trait Implementations

##### `impl Any for vfs_cap_data`

- <span id="vfs-cap-data-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for vfs_cap_data`

- <span id="vfs-cap-data-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for vfs_cap_data`

- <span id="vfs-cap-data-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for vfs_cap_data`

- <span id="vfs-cap-data-clone"></span>`fn clone(&self) -> vfs_cap_data` — [`vfs_cap_data`](#vfs-cap-data)

##### `impl CloneToUninit for vfs_cap_data`

- <span id="vfs-cap-data-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for vfs_cap_data`

##### `impl Debug for vfs_cap_data`

- <span id="vfs-cap-data-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for vfs_cap_data`

- <span id="vfs-cap-data-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for vfs_cap_data`

- <span id="vfs-cap-data-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for vfs_cap_data`

- <span id="vfs-cap-data-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vfs-cap-data-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for vfs_cap_data`

- <span id="vfs-cap-data-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vfs-cap-data-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `vfs_cap_data__bindgen_ty_1`

```rust
struct vfs_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:109-112`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L109-L112)*

#### Trait Implementations

##### `impl Any for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-clone"></span>`fn clone(&self) -> vfs_cap_data__bindgen_ty_1` — [`vfs_cap_data__bindgen_ty_1`](#vfs-cap-data-bindgen-ty-1)

##### `impl CloneToUninit for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for vfs_cap_data__bindgen_ty_1`

##### `impl Debug for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vfs-cap-data-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for vfs_cap_data__bindgen_ty_1`

- <span id="vfs-cap-data-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vfs-cap-data-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `vfs_ns_cap_data`

```rust
struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_ns_cap_data__bindgen_ty_1; 2],
    pub rootid: __le32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:115-119`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L115-L119)*

#### Trait Implementations

##### `impl Any for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-clone"></span>`fn clone(&self) -> vfs_ns_cap_data` — [`vfs_ns_cap_data`](#vfs-ns-cap-data)

##### `impl CloneToUninit for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for vfs_ns_cap_data`

##### `impl Debug for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vfs-ns-cap-data-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for vfs_ns_cap_data`

- <span id="vfs-ns-cap-data-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vfs-ns-cap-data-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `vfs_ns_cap_data__bindgen_ty_1`

```rust
struct vfs_ns_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:122-125`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L122-L125)*

#### Trait Implementations

##### `impl Any for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-clone"></span>`fn clone(&self) -> vfs_ns_cap_data__bindgen_ty_1` — [`vfs_ns_cap_data__bindgen_ty_1`](#vfs-ns-cap-data-bindgen-ty-1)

##### `impl CloneToUninit for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for vfs_ns_cap_data__bindgen_ty_1`

##### `impl Debug for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vfs-ns-cap-data-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for vfs_ns_cap_data__bindgen_ty_1`

- <span id="vfs-ns-cap-data-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vfs-ns-cap-data-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `f_owner_ex`

```rust
struct f_owner_ex {
    pub type_: crate::ctypes::c_int,
    pub pid: __kernel_pid_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:128-131`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L128-L131)*

#### Trait Implementations

##### `impl Any for f_owner_ex`

- <span id="f-owner-ex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for f_owner_ex`

- <span id="f-owner-ex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for f_owner_ex`

- <span id="f-owner-ex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for f_owner_ex`

- <span id="f-owner-ex-clone"></span>`fn clone(&self) -> f_owner_ex` — [`f_owner_ex`](#f-owner-ex)

##### `impl CloneToUninit for f_owner_ex`

- <span id="f-owner-ex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for f_owner_ex`

##### `impl Debug for f_owner_ex`

- <span id="f-owner-ex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for f_owner_ex`

- <span id="f-owner-ex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for f_owner_ex`

- <span id="f-owner-ex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for f_owner_ex`

- <span id="f-owner-ex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="f-owner-ex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for f_owner_ex`

- <span id="f-owner-ex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="f-owner-ex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:134-140`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L134-L140)*

#### Trait Implementations

##### `impl Any for flock`

- <span id="flock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for flock`

- <span id="flock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for flock`

- <span id="flock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for flock`

- <span id="flock-clone"></span>`fn clone(&self) -> flock` — [`flock`](#flock)

##### `impl CloneToUninit for flock`

- <span id="flock-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for flock`

##### `impl Debug for flock`

- <span id="flock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for flock`

- <span id="flock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for flock`

- <span id="flock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for flock`

- <span id="flock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for flock`

- <span id="flock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:143-149`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L143-L149)*

#### Trait Implementations

##### `impl Any for flock64`

- <span id="flock64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for flock64`

- <span id="flock64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for flock64`

- <span id="flock64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for flock64`

- <span id="flock64-clone"></span>`fn clone(&self) -> flock64` — [`flock64`](#flock64)

##### `impl CloneToUninit for flock64`

- <span id="flock64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for flock64`

##### `impl Debug for flock64`

- <span id="flock64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for flock64`

- <span id="flock64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for flock64`

- <span id="flock64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for flock64`

- <span id="flock64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flock64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for flock64`

- <span id="flock64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flock64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `open_how`

```rust
struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:152-156`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L152-L156)*

#### Trait Implementations

##### `impl Any for open_how`

- <span id="open-how-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for open_how`

- <span id="open-how-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for open_how`

- <span id="open-how-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for open_how`

- <span id="open-how-clone"></span>`fn clone(&self) -> open_how` — [`open_how`](#open-how)

##### `impl CloneToUninit for open_how`

- <span id="open-how-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for open_how`

##### `impl Debug for open_how`

- <span id="open-how-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for open_how`

- <span id="open-how-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for open_how`

- <span id="open-how-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for open_how`

- <span id="open-how-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="open-how-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for open_how`

- <span id="open-how-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="open-how-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `epoll_event`

```rust
struct epoll_event {
    pub events: __poll_t,
    pub data: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:159-162`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L159-L162)*

#### Trait Implementations

##### `impl Any for epoll_event`

- <span id="epoll-event-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for epoll_event`

- <span id="epoll-event-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for epoll_event`

- <span id="epoll-event-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for epoll_event`

- <span id="epoll-event-clone"></span>`fn clone(&self) -> epoll_event` — [`epoll_event`](#epoll-event)

##### `impl CloneToUninit for epoll_event`

- <span id="epoll-event-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- <span id="epoll-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for epoll_event`

- <span id="epoll-event-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for epoll_event`

- <span id="epoll-event-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for epoll_event`

- <span id="epoll-event-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="epoll-event-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for epoll_event`

- <span id="epoll-event-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="epoll-event-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `epoll_params`

```rust
struct epoll_params {
    pub busy_poll_usecs: __u32,
    pub busy_poll_budget: __u16,
    pub prefer_busy_poll: __u8,
    pub __pad: __u8,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:165-170`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L165-L170)*

#### Trait Implementations

##### `impl Any for epoll_params`

- <span id="epoll-params-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for epoll_params`

- <span id="epoll-params-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for epoll_params`

- <span id="epoll-params-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for epoll_params`

- <span id="epoll-params-clone"></span>`fn clone(&self) -> epoll_params` — [`epoll_params`](#epoll-params)

##### `impl CloneToUninit for epoll_params`

- <span id="epoll-params-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for epoll_params`

##### `impl Debug for epoll_params`

- <span id="epoll-params-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for epoll_params`

- <span id="epoll-params-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for epoll_params`

- <span id="epoll-params-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for epoll_params`

- <span id="epoll-params-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="epoll-params-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for epoll_params`

- <span id="epoll-params-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="epoll-params-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:173-179`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L173-L179)*

#### Trait Implementations

##### `impl Any for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-clone"></span>`fn clone(&self) -> fscrypt_policy_v1` — [`fscrypt_policy_v1`](#fscrypt-policy-v1)

##### `impl CloneToUninit for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_policy_v1`

##### `impl Debug for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-policy-v1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_policy_v1`

- <span id="fscrypt-policy-v1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-policy-v1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_key`

```rust
struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; 64],
    pub size: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:182-186`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L182-L186)*

#### Trait Implementations

##### `impl Any for fscrypt_key`

- <span id="fscrypt-key-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_key`

- <span id="fscrypt-key-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_key`

- <span id="fscrypt-key-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_key`

- <span id="fscrypt-key-clone"></span>`fn clone(&self) -> fscrypt_key` — [`fscrypt_key`](#fscrypt-key)

##### `impl CloneToUninit for fscrypt_key`

- <span id="fscrypt-key-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_key`

##### `impl Debug for fscrypt_key`

- <span id="fscrypt-key-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fscrypt_key`

- <span id="fscrypt-key-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_key`

- <span id="fscrypt-key-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_key`

- <span id="fscrypt-key-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-key-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_key`

- <span id="fscrypt-key-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-key-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:189-197`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L189-L197)*

#### Trait Implementations

##### `impl Any for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-clone"></span>`fn clone(&self) -> fscrypt_policy_v2` — [`fscrypt_policy_v2`](#fscrypt-policy-v2)

##### `impl CloneToUninit for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_policy_v2`

##### `impl Debug for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-policy-v2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_policy_v2`

- <span id="fscrypt-policy-v2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-policy-v2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_get_policy_ex_arg`

```rust
struct fscrypt_get_policy_ex_arg {
    pub policy_size: __u64,
    pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:200-203`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L200-L203)*

#### Trait Implementations

##### `impl Any for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-clone"></span>`fn clone(&self) -> fscrypt_get_policy_ex_arg` — [`fscrypt_get_policy_ex_arg`](#fscrypt-get-policy-ex-arg)

##### `impl CloneToUninit for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_get_policy_ex_arg`

##### `impl<T> From for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-get-policy-ex-arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_get_policy_ex_arg`

- <span id="fscrypt-get-policy-ex-arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-get-policy-ex-arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_key_specifier`

```rust
struct fscrypt_key_specifier {
    pub type_: __u32,
    pub __reserved: __u32,
    pub u: fscrypt_key_specifier__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:206-210`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L206-L210)*

#### Trait Implementations

##### `impl Any for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-clone"></span>`fn clone(&self) -> fscrypt_key_specifier` — [`fscrypt_key_specifier`](#fscrypt-key-specifier)

##### `impl CloneToUninit for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_key_specifier`

##### `impl<T> From for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-key-specifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_key_specifier`

- <span id="fscrypt-key-specifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-key-specifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_provisioning_key_payload`

```rust
struct fscrypt_provisioning_key_payload {
    pub type_: __u32,
    pub flags: __u32,
    pub raw: __IncompleteArrayField<__u8>,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:213-217`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L213-L217)*

#### Trait Implementations

##### `impl Any for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-provisioning-key-payload-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_provisioning_key_payload`

- <span id="fscrypt-provisioning-key-payload-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-provisioning-key-payload-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:219-226`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L219-L226)*

#### Trait Implementations

##### `impl Any for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-add-key-arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_add_key_arg`

- <span id="fscrypt-add-key-arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-add-key-arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_remove_key_arg`

```rust
struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: __u32,
    pub __reserved: [__u32; 5],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:229-233`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L229-L233)*

#### Trait Implementations

##### `impl Any for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-clone"></span>`fn clone(&self) -> fscrypt_remove_key_arg` — [`fscrypt_remove_key_arg`](#fscrypt-remove-key-arg)

##### `impl CloneToUninit for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_remove_key_arg`

##### `impl<T> From for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-remove-key-arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_remove_key_arg`

- <span id="fscrypt-remove-key-arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-remove-key-arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:236-243`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L236-L243)*

#### Trait Implementations

##### `impl Any for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-clone"></span>`fn clone(&self) -> fscrypt_get_key_status_arg` — [`fscrypt_get_key_status_arg`](#fscrypt-get-key-status-arg)

##### `impl CloneToUninit for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fscrypt_get_key_status_arg`

##### `impl<T> From for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fscrypt-get-key-status-arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fscrypt_get_key_status_arg`

- <span id="fscrypt-get-key-status-arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fscrypt-get-key-status-arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `mount_attr`

```rust
struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:246-251`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L246-L251)*

#### Trait Implementations

##### `impl Any for mount_attr`

- <span id="mount-attr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for mount_attr`

- <span id="mount-attr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for mount_attr`

- <span id="mount-attr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for mount_attr`

- <span id="mount-attr-clone"></span>`fn clone(&self) -> mount_attr` — [`mount_attr`](#mount-attr)

##### `impl CloneToUninit for mount_attr`

- <span id="mount-attr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for mount_attr`

##### `impl Debug for mount_attr`

- <span id="mount-attr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for mount_attr`

- <span id="mount-attr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for mount_attr`

- <span id="mount-attr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for mount_attr`

- <span id="mount-attr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mount-attr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for mount_attr`

- <span id="mount-attr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mount-attr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:254-288`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L254-L288)*

#### Trait Implementations

##### `impl Any for statmount`

- <span id="statmount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for statmount`

- <span id="statmount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for statmount`

- <span id="statmount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for statmount`

- <span id="statmount-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for statmount`

- <span id="statmount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for statmount`

- <span id="statmount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for statmount`

- <span id="statmount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statmount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for statmount`

- <span id="statmount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statmount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:291-297`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L291-L297)*

#### Trait Implementations

##### `impl Any for mnt_id_req`

- <span id="mnt-id-req-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for mnt_id_req`

- <span id="mnt-id-req-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for mnt_id_req`

- <span id="mnt-id-req-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for mnt_id_req`

- <span id="mnt-id-req-clone"></span>`fn clone(&self) -> mnt_id_req` — [`mnt_id_req`](#mnt-id-req)

##### `impl CloneToUninit for mnt_id_req`

- <span id="mnt-id-req-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for mnt_id_req`

##### `impl Debug for mnt_id_req`

- <span id="mnt-id-req-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for mnt_id_req`

- <span id="mnt-id-req-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for mnt_id_req`

- <span id="mnt-id-req-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for mnt_id_req`

- <span id="mnt-id-req-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mnt-id-req-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for mnt_id_req`

- <span id="mnt-id-req-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mnt-id-req-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:300-305`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L300-L305)*

#### Trait Implementations

##### `impl Any for file_clone_range`

- <span id="file-clone-range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for file_clone_range`

- <span id="file-clone-range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for file_clone_range`

- <span id="file-clone-range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for file_clone_range`

- <span id="file-clone-range-clone"></span>`fn clone(&self) -> file_clone_range` — [`file_clone_range`](#file-clone-range)

##### `impl CloneToUninit for file_clone_range`

- <span id="file-clone-range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- <span id="file-clone-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for file_clone_range`

- <span id="file-clone-range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for file_clone_range`

- <span id="file-clone-range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for file_clone_range`

- <span id="file-clone-range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-clone-range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for file_clone_range`

- <span id="file-clone-range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-clone-range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fstrim_range`

```rust
struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:308-312`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L308-L312)*

#### Trait Implementations

##### `impl Any for fstrim_range`

- <span id="fstrim-range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fstrim_range`

- <span id="fstrim-range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fstrim_range`

- <span id="fstrim-range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fstrim_range`

- <span id="fstrim-range-clone"></span>`fn clone(&self) -> fstrim_range` — [`fstrim_range`](#fstrim-range)

##### `impl CloneToUninit for fstrim_range`

- <span id="fstrim-range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fstrim_range`

##### `impl Debug for fstrim_range`

- <span id="fstrim-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fstrim_range`

- <span id="fstrim-range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fstrim_range`

- <span id="fstrim-range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fstrim_range`

- <span id="fstrim-range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fstrim-range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fstrim_range`

- <span id="fstrim-range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fstrim-range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fsuuid2`

```rust
struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:315-318`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L315-L318)*

#### Trait Implementations

##### `impl Any for fsuuid2`

- <span id="fsuuid2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fsuuid2`

- <span id="fsuuid2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fsuuid2`

- <span id="fsuuid2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fsuuid2`

- <span id="fsuuid2-clone"></span>`fn clone(&self) -> fsuuid2` — [`fsuuid2`](#fsuuid2)

##### `impl CloneToUninit for fsuuid2`

- <span id="fsuuid2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fsuuid2`

##### `impl Debug for fsuuid2`

- <span id="fsuuid2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fsuuid2`

- <span id="fsuuid2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fsuuid2`

- <span id="fsuuid2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fsuuid2`

- <span id="fsuuid2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fsuuid2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fsuuid2`

- <span id="fsuuid2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fsuuid2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fs_sysfs_path`

```rust
struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:321-324`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L321-L324)*

#### Trait Implementations

##### `impl Any for fs_sysfs_path`

- <span id="fs-sysfs-path-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fs_sysfs_path`

- <span id="fs-sysfs-path-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fs_sysfs_path`

- <span id="fs-sysfs-path-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fs_sysfs_path`

- <span id="fs-sysfs-path-clone"></span>`fn clone(&self) -> fs_sysfs_path` — [`fs_sysfs_path`](#fs-sysfs-path)

##### `impl CloneToUninit for fs_sysfs_path`

- <span id="fs-sysfs-path-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fs_sysfs_path`

##### `impl Debug for fs_sysfs_path`

- <span id="fs-sysfs-path-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fs_sysfs_path`

- <span id="fs-sysfs-path-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fs_sysfs_path`

- <span id="fs-sysfs-path-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fs_sysfs_path`

- <span id="fs-sysfs-path-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fs-sysfs-path-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fs_sysfs_path`

- <span id="fs-sysfs-path-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fs-sysfs-path-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:327-333`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L327-L333)*

#### Trait Implementations

##### `impl Any for file_dedupe_range_info`

- <span id="file-dedupe-range-info-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for file_dedupe_range_info`

- <span id="file-dedupe-range-info-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for file_dedupe_range_info`

- <span id="file-dedupe-range-info-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for file_dedupe_range_info`

- <span id="file-dedupe-range-info-clone"></span>`fn clone(&self) -> file_dedupe_range_info` — [`file_dedupe_range_info`](#file-dedupe-range-info)

##### `impl CloneToUninit for file_dedupe_range_info`

- <span id="file-dedupe-range-info-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for file_dedupe_range_info`

##### `impl Debug for file_dedupe_range_info`

- <span id="file-dedupe-range-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for file_dedupe_range_info`

- <span id="file-dedupe-range-info-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for file_dedupe_range_info`

- <span id="file-dedupe-range-info-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for file_dedupe_range_info`

- <span id="file-dedupe-range-info-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-dedupe-range-info-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for file_dedupe_range_info`

- <span id="file-dedupe-range-info-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-dedupe-range-info-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:336-343`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L336-L343)*

#### Trait Implementations

##### `impl Any for file_dedupe_range`

- <span id="file-dedupe-range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for file_dedupe_range`

- <span id="file-dedupe-range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for file_dedupe_range`

- <span id="file-dedupe-range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for file_dedupe_range`

- <span id="file-dedupe-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for file_dedupe_range`

- <span id="file-dedupe-range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for file_dedupe_range`

- <span id="file-dedupe-range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for file_dedupe_range`

- <span id="file-dedupe-range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-dedupe-range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for file_dedupe_range`

- <span id="file-dedupe-range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-dedupe-range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `files_stat_struct`

```rust
struct files_stat_struct {
    pub nr_files: crate::ctypes::c_ulong,
    pub nr_free_files: crate::ctypes::c_ulong,
    pub max_files: crate::ctypes::c_ulong,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:346-350`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L346-L350)*

#### Trait Implementations

##### `impl Any for files_stat_struct`

- <span id="files-stat-struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for files_stat_struct`

- <span id="files-stat-struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for files_stat_struct`

- <span id="files-stat-struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for files_stat_struct`

- <span id="files-stat-struct-clone"></span>`fn clone(&self) -> files_stat_struct` — [`files_stat_struct`](#files-stat-struct)

##### `impl CloneToUninit for files_stat_struct`

- <span id="files-stat-struct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for files_stat_struct`

##### `impl Debug for files_stat_struct`

- <span id="files-stat-struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for files_stat_struct`

- <span id="files-stat-struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for files_stat_struct`

- <span id="files-stat-struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for files_stat_struct`

- <span id="files-stat-struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="files-stat-struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for files_stat_struct`

- <span id="files-stat-struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="files-stat-struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `inodes_stat_t`

```rust
struct inodes_stat_t {
    pub nr_inodes: crate::ctypes::c_long,
    pub nr_unused: crate::ctypes::c_long,
    pub dummy: [crate::ctypes::c_long; 5],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:353-357`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L353-L357)*

#### Trait Implementations

##### `impl Any for inodes_stat_t`

- <span id="inodes-stat-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for inodes_stat_t`

- <span id="inodes-stat-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for inodes_stat_t`

- <span id="inodes-stat-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for inodes_stat_t`

- <span id="inodes-stat-t-clone"></span>`fn clone(&self) -> inodes_stat_t` — [`inodes_stat_t`](#inodes-stat-t)

##### `impl CloneToUninit for inodes_stat_t`

- <span id="inodes-stat-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for inodes_stat_t`

##### `impl Debug for inodes_stat_t`

- <span id="inodes-stat-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for inodes_stat_t`

- <span id="inodes-stat-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for inodes_stat_t`

- <span id="inodes-stat-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for inodes_stat_t`

- <span id="inodes-stat-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inodes-stat-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for inodes_stat_t`

- <span id="inodes-stat-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inodes-stat-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:360-367`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L360-L367)*

#### Trait Implementations

##### `impl Any for fsxattr`

- <span id="fsxattr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fsxattr`

- <span id="fsxattr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fsxattr`

- <span id="fsxattr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fsxattr`

- <span id="fsxattr-clone"></span>`fn clone(&self) -> fsxattr` — [`fsxattr`](#fsxattr)

##### `impl CloneToUninit for fsxattr`

- <span id="fsxattr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fsxattr`

##### `impl Debug for fsxattr`

- <span id="fsxattr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fsxattr`

- <span id="fsxattr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fsxattr`

- <span id="fsxattr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fsxattr`

- <span id="fsxattr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fsxattr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fsxattr`

- <span id="fsxattr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fsxattr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `page_region`

```rust
struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:370-374`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L370-L374)*

#### Trait Implementations

##### `impl Any for page_region`

- <span id="page-region-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for page_region`

- <span id="page-region-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for page_region`

- <span id="page-region-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for page_region`

- <span id="page-region-clone"></span>`fn clone(&self) -> page_region` — [`page_region`](#page-region)

##### `impl CloneToUninit for page_region`

- <span id="page-region-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for page_region`

##### `impl Debug for page_region`

- <span id="page-region-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for page_region`

- <span id="page-region-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for page_region`

- <span id="page-region-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for page_region`

- <span id="page-region-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="page-region-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for page_region`

- <span id="page-region-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="page-region-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:377-390`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L377-L390)*

#### Trait Implementations

##### `impl Any for pm_scan_arg`

- <span id="pm-scan-arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for pm_scan_arg`

- <span id="pm-scan-arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for pm_scan_arg`

- <span id="pm-scan-arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for pm_scan_arg`

- <span id="pm-scan-arg-clone"></span>`fn clone(&self) -> pm_scan_arg` — [`pm_scan_arg`](#pm-scan-arg)

##### `impl CloneToUninit for pm_scan_arg`

- <span id="pm-scan-arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for pm_scan_arg`

##### `impl Debug for pm_scan_arg`

- <span id="pm-scan-arg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for pm_scan_arg`

- <span id="pm-scan-arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for pm_scan_arg`

- <span id="pm-scan-arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for pm_scan_arg`

- <span id="pm-scan-arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pm-scan-arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for pm_scan_arg`

- <span id="pm-scan-arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pm-scan-arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:393-409`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L393-L409)*

#### Trait Implementations

##### `impl Any for procmap_query`

- <span id="procmap-query-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for procmap_query`

- <span id="procmap-query-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for procmap_query`

- <span id="procmap-query-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for procmap_query`

- <span id="procmap-query-clone"></span>`fn clone(&self) -> procmap_query` — [`procmap_query`](#procmap-query)

##### `impl CloneToUninit for procmap_query`

- <span id="procmap-query-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for procmap_query`

##### `impl Debug for procmap_query`

- <span id="procmap-query-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for procmap_query`

- <span id="procmap-query-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for procmap_query`

- <span id="procmap-query-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for procmap_query`

- <span id="procmap-query-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="procmap-query-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for procmap_query`

- <span id="procmap-query-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="procmap-query-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `futex_waitv`

```rust
struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:412-417`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L412-L417)*

#### Trait Implementations

##### `impl Any for futex_waitv`

- <span id="futex-waitv-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for futex_waitv`

- <span id="futex-waitv-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for futex_waitv`

- <span id="futex-waitv-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for futex_waitv`

- <span id="futex-waitv-clone"></span>`fn clone(&self) -> futex_waitv` — [`futex_waitv`](#futex-waitv)

##### `impl CloneToUninit for futex_waitv`

- <span id="futex-waitv-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for futex_waitv`

##### `impl Debug for futex_waitv`

- <span id="futex-waitv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for futex_waitv`

- <span id="futex-waitv-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for futex_waitv`

- <span id="futex-waitv-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for futex_waitv`

- <span id="futex-waitv-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="futex-waitv-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for futex_waitv`

- <span id="futex-waitv-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="futex-waitv-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `robust_list`

```rust
struct robust_list {
    pub next: *mut robust_list,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:420-422`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L420-L422)*

#### Trait Implementations

##### `impl Any for robust_list`

- <span id="robust-list-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for robust_list`

- <span id="robust-list-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for robust_list`

- <span id="robust-list-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for robust_list`

- <span id="robust-list-clone"></span>`fn clone(&self) -> robust_list` — [`robust_list`](#robust-list)

##### `impl CloneToUninit for robust_list`

- <span id="robust-list-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for robust_list`

##### `impl Debug for robust_list`

- <span id="robust-list-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for robust_list`

- <span id="robust-list-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for robust_list`

- <span id="robust-list-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for robust_list`

- <span id="robust-list-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="robust-list-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for robust_list`

- <span id="robust-list-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="robust-list-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `robust_list_head`

```rust
struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: crate::ctypes::c_long,
    pub list_op_pending: *mut robust_list,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:425-429`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L425-L429)*

#### Trait Implementations

##### `impl Any for robust_list_head`

- <span id="robust-list-head-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for robust_list_head`

- <span id="robust-list-head-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for robust_list_head`

- <span id="robust-list-head-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for robust_list_head`

- <span id="robust-list-head-clone"></span>`fn clone(&self) -> robust_list_head` — [`robust_list_head`](#robust-list-head)

##### `impl CloneToUninit for robust_list_head`

- <span id="robust-list-head-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for robust_list_head`

##### `impl Debug for robust_list_head`

- <span id="robust-list-head-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for robust_list_head`

- <span id="robust-list-head-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for robust_list_head`

- <span id="robust-list-head-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for robust_list_head`

- <span id="robust-list-head-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="robust-list-head-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for robust_list_head`

- <span id="robust-list-head-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="robust-list-head-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:432-438`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L432-L438)*

#### Trait Implementations

##### `impl Any for inotify_event`

- <span id="inotify-event-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for inotify_event`

- <span id="inotify-event-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for inotify_event`

- <span id="inotify-event-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for inotify_event`

- <span id="inotify-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for inotify_event`

- <span id="inotify-event-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for inotify_event`

- <span id="inotify-event-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for inotify_event`

- <span id="inotify-event-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inotify-event-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for inotify_event`

- <span id="inotify-event-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inotify-event-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `cachestat_range`

```rust
struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:441-444`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L441-L444)*

#### Trait Implementations

##### `impl Any for cachestat_range`

- <span id="cachestat-range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for cachestat_range`

- <span id="cachestat-range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for cachestat_range`

- <span id="cachestat-range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for cachestat_range`

- <span id="cachestat-range-clone"></span>`fn clone(&self) -> cachestat_range` — [`cachestat_range`](#cachestat-range)

##### `impl CloneToUninit for cachestat_range`

- <span id="cachestat-range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for cachestat_range`

##### `impl Debug for cachestat_range`

- <span id="cachestat-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for cachestat_range`

- <span id="cachestat-range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for cachestat_range`

- <span id="cachestat-range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for cachestat_range`

- <span id="cachestat-range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cachestat-range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for cachestat_range`

- <span id="cachestat-range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cachestat-range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:447-453`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L447-L453)*

#### Trait Implementations

##### `impl Any for cachestat`

- <span id="cachestat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for cachestat`

- <span id="cachestat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for cachestat`

- <span id="cachestat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for cachestat`

- <span id="cachestat-clone"></span>`fn clone(&self) -> cachestat` — [`cachestat`](#cachestat)

##### `impl CloneToUninit for cachestat`

- <span id="cachestat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for cachestat`

##### `impl Debug for cachestat`

- <span id="cachestat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for cachestat`

- <span id="cachestat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for cachestat`

- <span id="cachestat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for cachestat`

- <span id="cachestat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cachestat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for cachestat`

- <span id="cachestat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cachestat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `pollfd`

```rust
struct pollfd {
    pub fd: crate::ctypes::c_int,
    pub events: crate::ctypes::c_short,
    pub revents: crate::ctypes::c_short,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:456-460`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L456-L460)*

#### Trait Implementations

##### `impl Any for pollfd`

- <span id="pollfd-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for pollfd`

- <span id="pollfd-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for pollfd`

- <span id="pollfd-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for pollfd`

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](#pollfd)

##### `impl CloneToUninit for pollfd`

- <span id="pollfd-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- <span id="pollfd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for pollfd`

- <span id="pollfd-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for pollfd`

- <span id="pollfd-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for pollfd`

- <span id="pollfd-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pollfd-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for pollfd`

- <span id="pollfd-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pollfd-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `rand_pool_info`

```rust
struct rand_pool_info {
    pub entropy_count: crate::ctypes::c_int,
    pub buf_size: crate::ctypes::c_int,
    pub buf: __IncompleteArrayField<__u32>,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:463-467`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L463-L467)*

#### Trait Implementations

##### `impl Any for rand_pool_info`

- <span id="rand-pool-info-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for rand_pool_info`

- <span id="rand-pool-info-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for rand_pool_info`

- <span id="rand-pool-info-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for rand_pool_info`

- <span id="rand-pool-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for rand_pool_info`

- <span id="rand-pool-info-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for rand_pool_info`

- <span id="rand-pool-info-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for rand_pool_info`

- <span id="rand-pool-info-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rand-pool-info-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for rand_pool_info`

- <span id="rand-pool-info-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rand-pool-info-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `vgetrandom_opaque_params`

```rust
struct vgetrandom_opaque_params {
    pub size_of_opaque_state: __u32,
    pub mmap_prot: __u32,
    pub mmap_flags: __u32,
    pub reserved: [__u32; 13],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:470-475`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L470-L475)*

#### Trait Implementations

##### `impl Any for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-clone"></span>`fn clone(&self) -> vgetrandom_opaque_params` — [`vgetrandom_opaque_params`](#vgetrandom-opaque-params)

##### `impl CloneToUninit for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for vgetrandom_opaque_params`

##### `impl Debug for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vgetrandom-opaque-params-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for vgetrandom_opaque_params`

- <span id="vgetrandom-opaque-params-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vgetrandom-opaque-params-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_timespec`

```rust
struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: crate::ctypes::c_longlong,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:478-481`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L478-L481)*

#### Trait Implementations

##### `impl Any for __kernel_timespec`

- <span id="kernel-timespec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_timespec`

- <span id="kernel-timespec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_timespec`

- <span id="kernel-timespec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_timespec`

- <span id="kernel-timespec-clone"></span>`fn clone(&self) -> __kernel_timespec` — [`__kernel_timespec`](#kernel-timespec)

##### `impl CloneToUninit for __kernel_timespec`

- <span id="kernel-timespec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_timespec`

##### `impl Debug for __kernel_timespec`

- <span id="kernel-timespec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for general::__kernel_timespec`

##### `impl<T> From for __kernel_timespec`

- <span id="kernel-timespec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_timespec`

- <span id="kernel-timespec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for general::__kernel_timespec`

- <span id="general-kernel-timespec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<U> TryFrom for __kernel_timespec`

- <span id="kernel-timespec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-timespec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_timespec`

- <span id="kernel-timespec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-timespec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_itimerspec`

```rust
struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec,
    pub it_value: __kernel_timespec,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:484-487`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L484-L487)*

#### Trait Implementations

##### `impl Any for __kernel_itimerspec`

- <span id="kernel-itimerspec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_itimerspec`

- <span id="kernel-itimerspec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_itimerspec`

- <span id="kernel-itimerspec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_itimerspec`

- <span id="kernel-itimerspec-clone"></span>`fn clone(&self) -> __kernel_itimerspec` — [`__kernel_itimerspec`](#kernel-itimerspec)

##### `impl CloneToUninit for __kernel_itimerspec`

- <span id="kernel-itimerspec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_itimerspec`

##### `impl Debug for __kernel_itimerspec`

- <span id="kernel-itimerspec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_itimerspec`

- <span id="kernel-itimerspec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_itimerspec`

- <span id="kernel-itimerspec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_itimerspec`

- <span id="kernel-itimerspec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-itimerspec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_itimerspec`

- <span id="kernel-itimerspec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-itimerspec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_old_timeval`

```rust
struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:490-493`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L490-L493)*

#### Trait Implementations

##### `impl Any for __kernel_old_timeval`

- <span id="kernel-old-timeval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_old_timeval`

- <span id="kernel-old-timeval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_old_timeval`

- <span id="kernel-old-timeval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_old_timeval`

- <span id="kernel-old-timeval-clone"></span>`fn clone(&self) -> __kernel_old_timeval` — [`__kernel_old_timeval`](#kernel-old-timeval)

##### `impl CloneToUninit for __kernel_old_timeval`

- <span id="kernel-old-timeval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_old_timeval`

##### `impl Debug for __kernel_old_timeval`

- <span id="kernel-old-timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_old_timeval`

- <span id="kernel-old-timeval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_old_timeval`

- <span id="kernel-old-timeval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_old_timeval`

- <span id="kernel-old-timeval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-old-timeval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_old_timeval`

- <span id="kernel-old-timeval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-old-timeval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_old_timespec`

```rust
struct __kernel_old_timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:496-499`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L496-L499)*

#### Trait Implementations

##### `impl Any for __kernel_old_timespec`

- <span id="kernel-old-timespec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_old_timespec`

- <span id="kernel-old-timespec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_old_timespec`

- <span id="kernel-old-timespec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_old_timespec`

- <span id="kernel-old-timespec-clone"></span>`fn clone(&self) -> __kernel_old_timespec` — [`__kernel_old_timespec`](#kernel-old-timespec)

##### `impl CloneToUninit for __kernel_old_timespec`

- <span id="kernel-old-timespec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_old_timespec`

##### `impl Debug for __kernel_old_timespec`

- <span id="kernel-old-timespec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_old_timespec`

- <span id="kernel-old-timespec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_old_timespec`

- <span id="kernel-old-timespec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_old_timespec`

- <span id="kernel-old-timespec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-old-timespec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_old_timespec`

- <span id="kernel-old-timespec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-old-timespec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_old_itimerval`

```rust
struct __kernel_old_itimerval {
    pub it_interval: __kernel_old_timeval,
    pub it_value: __kernel_old_timeval,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:502-505`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L502-L505)*

#### Trait Implementations

##### `impl Any for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-clone"></span>`fn clone(&self) -> __kernel_old_itimerval` — [`__kernel_old_itimerval`](#kernel-old-itimerval)

##### `impl CloneToUninit for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_old_itimerval`

##### `impl Debug for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-old-itimerval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_old_itimerval`

- <span id="kernel-old-itimerval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-old-itimerval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__kernel_sock_timeval`

```rust
struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:508-511`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L508-L511)*

#### Trait Implementations

##### `impl Any for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-clone"></span>`fn clone(&self) -> __kernel_sock_timeval` — [`__kernel_sock_timeval`](#kernel-sock-timeval)

##### `impl CloneToUninit for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __kernel_sock_timeval`

##### `impl Debug for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-sock-timeval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __kernel_sock_timeval`

- <span id="kernel-sock-timeval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-sock-timeval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:514-531`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L514-L531)*

#### Trait Implementations

##### `impl Any for rusage`

- <span id="rusage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for rusage`

- <span id="rusage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for rusage`

- <span id="rusage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for rusage`

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](#rusage)

##### `impl CloneToUninit for rusage`

- <span id="rusage-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- <span id="rusage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for rusage`

- <span id="rusage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for rusage`

- <span id="rusage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for rusage`

- <span id="rusage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rusage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for rusage`

- <span id="rusage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rusage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:534-537`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L534-L537)*

#### Trait Implementations

##### `impl Any for rlimit`

- <span id="rlimit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for rlimit`

- <span id="rlimit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for rlimit`

- <span id="rlimit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](#rlimit)

##### `impl CloneToUninit for rlimit`

- <span id="rlimit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- <span id="rlimit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for rlimit`

- <span id="rlimit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for rlimit`

- <span id="rlimit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for rlimit`

- <span id="rlimit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rlimit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for rlimit`

- <span id="rlimit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rlimit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `rlimit64`

```rust
struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:540-543`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L540-L543)*

#### Trait Implementations

##### `impl Any for rlimit64`

- <span id="rlimit64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for rlimit64`

- <span id="rlimit64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for rlimit64`

- <span id="rlimit64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for rlimit64`

- <span id="rlimit64-clone"></span>`fn clone(&self) -> rlimit64` — [`rlimit64`](#rlimit64)

##### `impl CloneToUninit for rlimit64`

- <span id="rlimit64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for rlimit64`

##### `impl Debug for rlimit64`

- <span id="rlimit64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for rlimit64`

- <span id="rlimit64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for rlimit64`

- <span id="rlimit64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for rlimit64`

- <span id="rlimit64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rlimit64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for rlimit64`

- <span id="rlimit64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rlimit64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:546-558`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L546-L558)*

#### Trait Implementations

##### `impl Any for clone_args`

- <span id="clone-args-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for clone_args`

- <span id="clone-args-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for clone_args`

- <span id="clone-args-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for clone_args`

- <span id="clone-args-clone"></span>`fn clone(&self) -> clone_args` — [`clone_args`](#clone-args)

##### `impl CloneToUninit for clone_args`

- <span id="clone-args-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for clone_args`

##### `impl Debug for clone_args`

- <span id="clone-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for clone_args`

- <span id="clone-args-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for clone_args`

- <span id="clone-args-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for clone_args`

- <span id="clone-args-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clone-args-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for clone_args`

- <span id="clone-args-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clone-args-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sigaction`

```rust
struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:561-566`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L561-L566)*

#### Trait Implementations

##### `impl Any for sigaction`

- <span id="sigaction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sigaction`

- <span id="sigaction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sigaction`

- <span id="sigaction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sigaction`

- <span id="sigaction-clone"></span>`fn clone(&self) -> sigaction` — [`sigaction`](#sigaction)

##### `impl CloneToUninit for sigaction`

- <span id="sigaction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sigaction`

##### `impl Debug for sigaction`

- <span id="sigaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sigaction`

- <span id="sigaction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sigaction`

- <span id="sigaction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sigaction`

- <span id="sigaction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sigaction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sigaction`

- <span id="sigaction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sigaction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sigaltstack`

```rust
struct sigaltstack {
    pub ss_sp: *mut crate::ctypes::c_void,
    pub ss_flags: crate::ctypes::c_int,
    pub ss_size: __kernel_size_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:569-573`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L569-L573)*

#### Trait Implementations

##### `impl Any for sigaltstack`

- <span id="sigaltstack-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sigaltstack`

- <span id="sigaltstack-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sigaltstack`

- <span id="sigaltstack-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sigaltstack`

- <span id="sigaltstack-clone"></span>`fn clone(&self) -> sigaltstack` — [`sigaltstack`](#sigaltstack)

##### `impl CloneToUninit for sigaltstack`

- <span id="sigaltstack-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sigaltstack`

##### `impl Debug for sigaltstack`

- <span id="sigaltstack-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sigaltstack`

- <span id="sigaltstack-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sigaltstack`

- <span id="sigaltstack-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sigaltstack`

- <span id="sigaltstack-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sigaltstack-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sigaltstack`

- <span id="sigaltstack-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sigaltstack-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_1 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:576-579`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L576-L579)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_1` — [`__sifields__bindgen_ty_1`](#sifields-bindgen-ty-1)

##### `impl CloneToUninit for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_1`

##### `impl Debug for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_1`

- <span id="sifields-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_2 {
    pub _tid: __kernel_timer_t,
    pub _overrun: crate::ctypes::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: crate::ctypes::c_int,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:582-587`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L582-L587)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_2` — [`__sifields__bindgen_ty_2`](#sifields-bindgen-ty-2)

##### `impl CloneToUninit for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_2`

##### `impl<T> From for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_2`

- <span id="sifields-bindgen-ty-2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_3 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:590-594`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L590-L594)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_3` — [`__sifields__bindgen_ty_3`](#sifields-bindgen-ty-3)

##### `impl CloneToUninit for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_3`

##### `impl<T> From for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_3`

- <span id="sifields-bindgen-ty-3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:597-603`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L597-L603)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_4` — [`__sifields__bindgen_ty_4`](#sifields-bindgen-ty-4)

##### `impl CloneToUninit for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_4`

##### `impl Debug for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-4-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_4`

- <span id="sifields-bindgen-ty-4-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-4-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_5`

```rust
struct __sifields__bindgen_ty_5 {
    pub _addr: *mut crate::ctypes::c_void,
    pub __bindgen_anon_1: __sifields__bindgen_ty_5__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:606-609`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L606-L609)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5` — [`__sifields__bindgen_ty_5`](#sifields-bindgen-ty-5)

##### `impl CloneToUninit for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_5`

##### `impl<T> From for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_5`

- <span id="sifields-bindgen-ty-5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _dummy_bnd: [crate::ctypes::c_char; 8],
    pub _lower: *mut crate::ctypes::c_void,
    pub _upper: *mut crate::ctypes::c_void,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:612-616`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L612-L616)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1)

##### `impl CloneToUninit for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub _dummy_pkey: [crate::ctypes::c_char; 8],
    pub _pkey: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:619-622`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L619-L622)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2)

##### `impl CloneToUninit for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub _data: crate::ctypes::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:625-629`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L625-L629)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3` — [`__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`](#sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3)

##### `impl CloneToUninit for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

##### `impl Debug for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-5-bindgen-ty-1-bindgen-ty-3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_6`

```rust
struct __sifields__bindgen_ty_6 {
    pub _band: crate::ctypes::c_long,
    pub _fd: crate::ctypes::c_int,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:632-635`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L632-L635)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_6` — [`__sifields__bindgen_ty_6`](#sifields-bindgen-ty-6)

##### `impl CloneToUninit for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_6`

##### `impl Debug for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-6-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_6`

- <span id="sifields-bindgen-ty-6-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-6-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_7`

```rust
struct __sifields__bindgen_ty_7 {
    pub _call_addr: *mut crate::ctypes::c_void,
    pub _syscall: crate::ctypes::c_int,
    pub _arch: crate::ctypes::c_uint,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:638-642`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L638-L642)*

#### Trait Implementations

##### `impl Any for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-clone"></span>`fn clone(&self) -> __sifields__bindgen_ty_7` — [`__sifields__bindgen_ty_7`](#sifields-bindgen-ty-7)

##### `impl CloneToUninit for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __sifields__bindgen_ty_7`

##### `impl Debug for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-bindgen-ty-7-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __sifields__bindgen_ty_7`

- <span id="sifields-bindgen-ty-7-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-bindgen-ty-7-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `siginfo`

```rust
struct siginfo {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:645-647`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L645-L647)*

#### Trait Implementations

##### `impl Any for siginfo`

- <span id="siginfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for siginfo`

- <span id="siginfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for siginfo`

- <span id="siginfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for siginfo`

- <span id="siginfo-clone"></span>`fn clone(&self) -> siginfo` — [`siginfo`](#siginfo)

##### `impl CloneToUninit for siginfo`

- <span id="siginfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for siginfo`

##### `impl<T> From for siginfo`

- <span id="siginfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for siginfo`

- <span id="siginfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for siginfo`

- <span id="siginfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="siginfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for siginfo`

- <span id="siginfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="siginfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `siginfo__bindgen_ty_1__bindgen_ty_1`

```rust
struct siginfo__bindgen_ty_1__bindgen_ty_1 {
    pub si_signo: crate::ctypes::c_int,
    pub si_errno: crate::ctypes::c_int,
    pub si_code: crate::ctypes::c_int,
    pub _sifields: __sifields,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:650-655`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L650-L655)*

#### Trait Implementations

##### `impl Any for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> siginfo__bindgen_ty_1__bindgen_ty_1` — [`siginfo__bindgen_ty_1__bindgen_ty_1`](#siginfo-bindgen-ty-1-bindgen-ty-1)

##### `impl CloneToUninit for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for siginfo__bindgen_ty_1__bindgen_ty_1`

##### `impl<T> From for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for siginfo__bindgen_ty_1__bindgen_ty_1`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="siginfo-bindgen-ty-1-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: crate::ctypes::c_int,
    pub sigev_notify: crate::ctypes::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:658-663`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L658-L663)*

#### Trait Implementations

##### `impl Any for sigevent`

- <span id="sigevent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sigevent`

- <span id="sigevent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sigevent`

- <span id="sigevent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sigevent`

- <span id="sigevent-clone"></span>`fn clone(&self) -> sigevent` — [`sigevent`](#sigevent)

##### `impl CloneToUninit for sigevent`

- <span id="sigevent-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sigevent`

##### `impl<T> From for sigevent`

- <span id="sigevent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sigevent`

- <span id="sigevent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sigevent`

- <span id="sigevent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sigevent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sigevent`

- <span id="sigevent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sigevent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sigevent__bindgen_ty_1__bindgen_ty_1`

```rust
struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::core::option::Option<fn(sigval_t)>,
    pub _attribute: *mut crate::ctypes::c_void,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:666-669`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L666-L669)*

#### Trait Implementations

##### `impl Any for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> sigevent__bindgen_ty_1__bindgen_ty_1` — [`sigevent__bindgen_ty_1__bindgen_ty_1`](#sigevent-bindgen-ty-1-bindgen-ty-1)

##### `impl CloneToUninit for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sigevent__bindgen_ty_1__bindgen_ty_1`

##### `impl Debug for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sigevent__bindgen_ty_1__bindgen_ty_1`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sigevent-bindgen-ty-1-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:672-676`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L672-L676)*

#### Trait Implementations

##### `impl Any for statx_timestamp`

- <span id="statx-timestamp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for statx_timestamp`

- <span id="statx-timestamp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for statx_timestamp`

- <span id="statx-timestamp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for statx_timestamp`

- <span id="statx-timestamp-clone"></span>`fn clone(&self) -> statx_timestamp` — [`statx_timestamp`](#statx-timestamp)

##### `impl CloneToUninit for statx_timestamp`

- <span id="statx-timestamp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- <span id="statx-timestamp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for statx_timestamp`

- <span id="statx-timestamp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for statx_timestamp`

- <span id="statx-timestamp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for statx_timestamp`

- <span id="statx-timestamp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statx-timestamp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for statx_timestamp`

- <span id="statx-timestamp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statx-timestamp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:679-711`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L679-L711)*

#### Trait Implementations

##### `impl Any for statx`

- <span id="statx-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for statx`

- <span id="statx-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for statx`

- <span id="statx-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for statx`

- <span id="statx-clone"></span>`fn clone(&self) -> statx` — [`statx`](#statx)

##### `impl CloneToUninit for statx`

- <span id="statx-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for statx`

##### `impl Debug for statx`

- <span id="statx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for statx`

- <span id="statx-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for statx`

- <span id="statx-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for statx`

- <span id="statx-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statx-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for statx`

- <span id="statx-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statx-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:714-721`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L714-L721)*

#### Trait Implementations

##### `impl Any for termios`

- <span id="termios-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for termios`

- <span id="termios-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for termios`

- <span id="termios-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for termios`

- <span id="termios-clone"></span>`fn clone(&self) -> termios` — [`termios`](#termios)

##### `impl CloneToUninit for termios`

- <span id="termios-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for termios`

##### `impl Debug for termios`

- <span id="termios-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for termios`

- <span id="termios-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for termios`

- <span id="termios-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for termios`

- <span id="termios-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termios-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for termios`

- <span id="termios-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termios-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:724-733`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L724-L733)*

#### Trait Implementations

##### `impl Any for termios2`

- <span id="termios2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for termios2`

- <span id="termios2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for termios2`

- <span id="termios2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for termios2`

- <span id="termios2-clone"></span>`fn clone(&self) -> termios2` — [`termios2`](#termios2)

##### `impl CloneToUninit for termios2`

- <span id="termios2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for termios2`

##### `impl Debug for termios2`

- <span id="termios2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for termios2`

- <span id="termios2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for termios2`

- <span id="termios2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for termios2`

- <span id="termios2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termios2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for termios2`

- <span id="termios2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termios2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:736-745`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L736-L745)*

#### Trait Implementations

##### `impl Any for ktermios`

- <span id="ktermios-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ktermios`

- <span id="ktermios-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ktermios`

- <span id="ktermios-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ktermios`

- <span id="ktermios-clone"></span>`fn clone(&self) -> ktermios` — [`ktermios`](#ktermios)

##### `impl CloneToUninit for ktermios`

- <span id="ktermios-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ktermios`

##### `impl Debug for ktermios`

- <span id="ktermios-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ktermios`

- <span id="ktermios-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ktermios`

- <span id="ktermios-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ktermios`

- <span id="ktermios-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ktermios-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ktermios`

- <span id="ktermios-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ktermios-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `winsize`

```rust
struct winsize {
    pub ws_row: crate::ctypes::c_ushort,
    pub ws_col: crate::ctypes::c_ushort,
    pub ws_xpixel: crate::ctypes::c_ushort,
    pub ws_ypixel: crate::ctypes::c_ushort,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:748-753`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L748-L753)*

#### Trait Implementations

##### `impl Any for winsize`

- <span id="winsize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for winsize`

- <span id="winsize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for winsize`

- <span id="winsize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](#winsize)

##### `impl CloneToUninit for winsize`

- <span id="winsize-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- <span id="winsize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for winsize`

- <span id="winsize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for winsize`

- <span id="winsize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for winsize`

- <span id="winsize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winsize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for winsize`

- <span id="winsize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winsize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:756-763`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L756-L763)*

#### Trait Implementations

##### `impl Any for termio`

- <span id="termio-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for termio`

- <span id="termio-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for termio`

- <span id="termio-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for termio`

- <span id="termio-clone"></span>`fn clone(&self) -> termio` — [`termio`](#termio)

##### `impl CloneToUninit for termio`

- <span id="termio-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for termio`

##### `impl Debug for termio`

- <span id="termio-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for termio`

- <span id="termio-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for termio`

- <span id="termio-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for termio`

- <span id="termio-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termio-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for termio`

- <span id="termio-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termio-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `timespec`

```rust
struct timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:766-769`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L766-L769)*

#### Trait Implementations

##### `impl Any for timespec`

- <span id="timespec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for timespec`

- <span id="timespec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for timespec`

- <span id="timespec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for timespec`

- <span id="timespec-clone"></span>`fn clone(&self) -> timespec` — [`timespec`](#timespec)

##### `impl CloneToUninit for timespec`

- <span id="timespec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for timespec`

##### `impl Debug for timespec`

- <span id="timespec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for timespec`

- <span id="timespec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for timespec`

- <span id="timespec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for timespec`

- <span id="timespec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timespec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for timespec`

- <span id="timespec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timespec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `timeval`

```rust
struct timeval {
    pub tv_sec: __kernel_old_time_t,
    pub tv_usec: __kernel_suseconds_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:772-775`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L772-L775)*

#### Trait Implementations

##### `impl Any for timeval`

- <span id="timeval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for timeval`

- <span id="timeval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for timeval`

- <span id="timeval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](#timeval)

##### `impl CloneToUninit for timeval`

- <span id="timeval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- <span id="timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for timeval`

- <span id="timeval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for timeval`

- <span id="timeval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for timeval`

- <span id="timeval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timeval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for timeval`

- <span id="timeval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timeval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `itimerspec`

```rust
struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:778-781`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L778-L781)*

#### Trait Implementations

##### `impl Any for itimerspec`

- <span id="itimerspec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for itimerspec`

- <span id="itimerspec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for itimerspec`

- <span id="itimerspec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for itimerspec`

- <span id="itimerspec-clone"></span>`fn clone(&self) -> itimerspec` — [`itimerspec`](#itimerspec)

##### `impl CloneToUninit for itimerspec`

- <span id="itimerspec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for itimerspec`

##### `impl Debug for itimerspec`

- <span id="itimerspec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for itimerspec`

- <span id="itimerspec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for itimerspec`

- <span id="itimerspec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for itimerspec`

- <span id="itimerspec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itimerspec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for itimerspec`

- <span id="itimerspec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itimerspec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:784-787`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L784-L787)*

#### Trait Implementations

##### `impl Any for itimerval`

- <span id="itimerval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for itimerval`

- <span id="itimerval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for itimerval`

- <span id="itimerval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](#itimerval)

##### `impl CloneToUninit for itimerval`

- <span id="itimerval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- <span id="itimerval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for itimerval`

- <span id="itimerval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for itimerval`

- <span id="itimerval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for itimerval`

- <span id="itimerval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itimerval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for itimerval`

- <span id="itimerval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itimerval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `timezone`

```rust
struct timezone {
    pub tz_minuteswest: crate::ctypes::c_int,
    pub tz_dsttime: crate::ctypes::c_int,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:790-793`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L790-L793)*

#### Trait Implementations

##### `impl Any for timezone`

- <span id="timezone-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for timezone`

- <span id="timezone-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for timezone`

- <span id="timezone-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for timezone`

- <span id="timezone-clone"></span>`fn clone(&self) -> timezone` — [`timezone`](#timezone)

##### `impl CloneToUninit for timezone`

- <span id="timezone-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for timezone`

##### `impl Debug for timezone`

- <span id="timezone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for timezone`

- <span id="timezone-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for timezone`

- <span id="timezone-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for timezone`

- <span id="timezone-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timezone-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for timezone`

- <span id="timezone-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timezone-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: __kernel_size_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:796-799`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L796-L799)*

#### Trait Implementations

##### `impl Any for iovec`

- <span id="iovec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for iovec`

- <span id="iovec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for iovec`

- <span id="iovec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](#iovec)

##### `impl CloneToUninit for iovec`

- <span id="iovec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- <span id="iovec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for iovec`

- <span id="iovec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for iovec`

- <span id="iovec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for iovec`

- <span id="iovec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iovec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for iovec`

- <span id="iovec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iovec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:802-808`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L802-L808)*

#### Trait Implementations

##### `impl Any for dmabuf_cmsg`

- <span id="dmabuf-cmsg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for dmabuf_cmsg`

- <span id="dmabuf-cmsg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for dmabuf_cmsg`

- <span id="dmabuf-cmsg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for dmabuf_cmsg`

- <span id="dmabuf-cmsg-clone"></span>`fn clone(&self) -> dmabuf_cmsg` — [`dmabuf_cmsg`](#dmabuf-cmsg)

##### `impl CloneToUninit for dmabuf_cmsg`

- <span id="dmabuf-cmsg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for dmabuf_cmsg`

##### `impl Debug for dmabuf_cmsg`

- <span id="dmabuf-cmsg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for dmabuf_cmsg`

- <span id="dmabuf-cmsg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for dmabuf_cmsg`

- <span id="dmabuf-cmsg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for dmabuf_cmsg`

- <span id="dmabuf-cmsg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dmabuf-cmsg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for dmabuf_cmsg`

- <span id="dmabuf-cmsg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dmabuf-cmsg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `dmabuf_token`

```rust
struct dmabuf_token {
    pub token_start: __u32,
    pub token_count: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:811-814`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L811-L814)*

#### Trait Implementations

##### `impl Any for dmabuf_token`

- <span id="dmabuf-token-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for dmabuf_token`

- <span id="dmabuf-token-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for dmabuf_token`

- <span id="dmabuf-token-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for dmabuf_token`

- <span id="dmabuf-token-clone"></span>`fn clone(&self) -> dmabuf_token` — [`dmabuf_token`](#dmabuf-token)

##### `impl CloneToUninit for dmabuf_token`

- <span id="dmabuf-token-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for dmabuf_token`

##### `impl Debug for dmabuf_token`

- <span id="dmabuf-token-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for dmabuf_token`

- <span id="dmabuf-token-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for dmabuf_token`

- <span id="dmabuf-token-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for dmabuf_token`

- <span id="dmabuf-token-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dmabuf-token-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for dmabuf_token`

- <span id="dmabuf-token-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dmabuf-token-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `xattr_args`

```rust
struct xattr_args {
    pub value: __u64,
    pub size: __u32,
    pub flags: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:817-821`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L817-L821)*

#### Trait Implementations

##### `impl Any for xattr_args`

- <span id="xattr-args-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for xattr_args`

- <span id="xattr-args-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for xattr_args`

- <span id="xattr-args-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for xattr_args`

- <span id="xattr-args-clone"></span>`fn clone(&self) -> xattr_args` — [`xattr_args`](#xattr-args)

##### `impl CloneToUninit for xattr_args`

- <span id="xattr-args-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for xattr_args`

##### `impl Debug for xattr_args`

- <span id="xattr-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for xattr_args`

- <span id="xattr-args-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for xattr_args`

- <span id="xattr-args-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for xattr_args`

- <span id="xattr-args-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xattr-args-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for xattr_args`

- <span id="xattr-args-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xattr-args-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:824-830`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L824-L830)*

#### Trait Implementations

##### `impl Any for uffd_msg`

- <span id="uffd-msg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg`

- <span id="uffd-msg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg`

- <span id="uffd-msg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg`

- <span id="uffd-msg-clone"></span>`fn clone(&self) -> uffd_msg` — [`uffd_msg`](#uffd-msg)

##### `impl CloneToUninit for uffd_msg`

- <span id="uffd-msg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg`

##### `impl<T> From for uffd_msg`

- <span id="uffd-msg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg`

- <span id="uffd-msg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg`

- <span id="uffd-msg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg`

- <span id="uffd-msg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_1`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_1 {
    pub flags: __u64,
    pub address: __u64,
    pub feat: uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:833-837`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L833-L837)*

#### Trait Implementations

##### `impl Any for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_1` — [`uffd_msg__bindgen_ty_1__bindgen_ty_1`](#uffd-msg-bindgen-ty-1-bindgen-ty-1)

##### `impl CloneToUninit for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_1`

##### `impl<T> From for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg__bindgen_ty_1__bindgen_ty_1`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_2`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_2 {
    pub ufd: __u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:840-842`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L840-L842)*

#### Trait Implementations

##### `impl Any for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_2` — [`uffd_msg__bindgen_ty_1__bindgen_ty_2`](#uffd-msg-bindgen-ty-1-bindgen-ty-2)

##### `impl CloneToUninit for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_2`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg__bindgen_ty_1__bindgen_ty_2`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_3`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_3 {
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:845-849`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L845-L849)*

#### Trait Implementations

##### `impl Any for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_3` — [`uffd_msg__bindgen_ty_1__bindgen_ty_3`](#uffd-msg-bindgen-ty-1-bindgen-ty-3)

##### `impl CloneToUninit for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_3`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg__bindgen_ty_1__bindgen_ty_3`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_4`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_4 {
    pub start: __u64,
    pub end: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:852-855`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L852-L855)*

#### Trait Implementations

##### `impl Any for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_4` — [`uffd_msg__bindgen_ty_1__bindgen_ty_4`](#uffd-msg-bindgen-ty-1-bindgen-ty-4)

##### `impl CloneToUninit for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_4`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg__bindgen_ty_1__bindgen_ty_4`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-4-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_5`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_5 {
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:858-862`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L858-L862)*

#### Trait Implementations

##### `impl Any for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-clone"></span>`fn clone(&self) -> uffd_msg__bindgen_ty_1__bindgen_ty_5` — [`uffd_msg__bindgen_ty_1__bindgen_ty_5`](#uffd-msg-bindgen-ty-1-bindgen-ty-5)

##### `impl CloneToUninit for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffd_msg__bindgen_ty_1__bindgen_ty_5`

##### `impl Debug for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffd_msg__bindgen_ty_1__bindgen_ty_5`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffd-msg-bindgen-ty-1-bindgen-ty-5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_api`

```rust
struct uffdio_api {
    pub api: __u64,
    pub features: __u64,
    pub ioctls: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:865-869`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L865-L869)*

#### Trait Implementations

##### `impl Any for uffdio_api`

- <span id="uffdio-api-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_api`

- <span id="uffdio-api-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_api`

- <span id="uffdio-api-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_api`

- <span id="uffdio-api-clone"></span>`fn clone(&self) -> uffdio_api` — [`uffdio_api`](#uffdio-api)

##### `impl CloneToUninit for uffdio_api`

- <span id="uffdio-api-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_api`

##### `impl Debug for uffdio_api`

- <span id="uffdio-api-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_api`

- <span id="uffdio-api-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_api`

- <span id="uffdio-api-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_api`

- <span id="uffdio-api-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-api-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_api`

- <span id="uffdio-api-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-api-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_range`

```rust
struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:872-875`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L872-L875)*

#### Trait Implementations

##### `impl Any for uffdio_range`

- <span id="uffdio-range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_range`

- <span id="uffdio-range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_range`

- <span id="uffdio-range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_range`

- <span id="uffdio-range-clone"></span>`fn clone(&self) -> uffdio_range` — [`uffdio_range`](#uffdio-range)

##### `impl CloneToUninit for uffdio_range`

- <span id="uffdio-range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_range`

##### `impl Debug for uffdio_range`

- <span id="uffdio-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_range`

- <span id="uffdio-range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_range`

- <span id="uffdio-range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_range`

- <span id="uffdio-range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_range`

- <span id="uffdio-range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_register`

```rust
struct uffdio_register {
    pub range: uffdio_range,
    pub mode: __u64,
    pub ioctls: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:878-882`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L878-L882)*

#### Trait Implementations

##### `impl Any for uffdio_register`

- <span id="uffdio-register-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_register`

- <span id="uffdio-register-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_register`

- <span id="uffdio-register-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_register`

- <span id="uffdio-register-clone"></span>`fn clone(&self) -> uffdio_register` — [`uffdio_register`](#uffdio-register)

##### `impl CloneToUninit for uffdio_register`

- <span id="uffdio-register-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_register`

##### `impl Debug for uffdio_register`

- <span id="uffdio-register-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_register`

- <span id="uffdio-register-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_register`

- <span id="uffdio-register-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_register`

- <span id="uffdio-register-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-register-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_register`

- <span id="uffdio-register-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-register-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:885-891`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L885-L891)*

#### Trait Implementations

##### `impl Any for uffdio_copy`

- <span id="uffdio-copy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_copy`

- <span id="uffdio-copy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_copy`

- <span id="uffdio-copy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_copy`

- <span id="uffdio-copy-clone"></span>`fn clone(&self) -> uffdio_copy` — [`uffdio_copy`](#uffdio-copy)

##### `impl CloneToUninit for uffdio_copy`

- <span id="uffdio-copy-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_copy`

##### `impl Debug for uffdio_copy`

- <span id="uffdio-copy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_copy`

- <span id="uffdio-copy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_copy`

- <span id="uffdio-copy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_copy`

- <span id="uffdio-copy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-copy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_copy`

- <span id="uffdio-copy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-copy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_zeropage`

```rust
struct uffdio_zeropage {
    pub range: uffdio_range,
    pub mode: __u64,
    pub zeropage: __s64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:894-898`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L894-L898)*

#### Trait Implementations

##### `impl Any for uffdio_zeropage`

- <span id="uffdio-zeropage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_zeropage`

- <span id="uffdio-zeropage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_zeropage`

- <span id="uffdio-zeropage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_zeropage`

- <span id="uffdio-zeropage-clone"></span>`fn clone(&self) -> uffdio_zeropage` — [`uffdio_zeropage`](#uffdio-zeropage)

##### `impl CloneToUninit for uffdio_zeropage`

- <span id="uffdio-zeropage-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_zeropage`

##### `impl Debug for uffdio_zeropage`

- <span id="uffdio-zeropage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_zeropage`

- <span id="uffdio-zeropage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_zeropage`

- <span id="uffdio-zeropage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_zeropage`

- <span id="uffdio-zeropage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-zeropage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_zeropage`

- <span id="uffdio-zeropage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-zeropage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_writeprotect`

```rust
struct uffdio_writeprotect {
    pub range: uffdio_range,
    pub mode: __u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:901-904`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L901-L904)*

#### Trait Implementations

##### `impl Any for uffdio_writeprotect`

- <span id="uffdio-writeprotect-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_writeprotect`

- <span id="uffdio-writeprotect-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_writeprotect`

- <span id="uffdio-writeprotect-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_writeprotect`

- <span id="uffdio-writeprotect-clone"></span>`fn clone(&self) -> uffdio_writeprotect` — [`uffdio_writeprotect`](#uffdio-writeprotect)

##### `impl CloneToUninit for uffdio_writeprotect`

- <span id="uffdio-writeprotect-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_writeprotect`

##### `impl Debug for uffdio_writeprotect`

- <span id="uffdio-writeprotect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_writeprotect`

- <span id="uffdio-writeprotect-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_writeprotect`

- <span id="uffdio-writeprotect-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_writeprotect`

- <span id="uffdio-writeprotect-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-writeprotect-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_writeprotect`

- <span id="uffdio-writeprotect-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-writeprotect-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_continue`

```rust
struct uffdio_continue {
    pub range: uffdio_range,
    pub mode: __u64,
    pub mapped: __s64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:907-911`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L907-L911)*

#### Trait Implementations

##### `impl Any for uffdio_continue`

- <span id="uffdio-continue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_continue`

- <span id="uffdio-continue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_continue`

- <span id="uffdio-continue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_continue`

- <span id="uffdio-continue-clone"></span>`fn clone(&self) -> uffdio_continue` — [`uffdio_continue`](#uffdio-continue)

##### `impl CloneToUninit for uffdio_continue`

- <span id="uffdio-continue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_continue`

##### `impl Debug for uffdio_continue`

- <span id="uffdio-continue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_continue`

- <span id="uffdio-continue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_continue`

- <span id="uffdio-continue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_continue`

- <span id="uffdio-continue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-continue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_continue`

- <span id="uffdio-continue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-continue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `uffdio_poison`

```rust
struct uffdio_poison {
    pub range: uffdio_range,
    pub mode: __u64,
    pub updated: __s64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:914-918`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L914-L918)*

#### Trait Implementations

##### `impl Any for uffdio_poison`

- <span id="uffdio-poison-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_poison`

- <span id="uffdio-poison-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_poison`

- <span id="uffdio-poison-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_poison`

- <span id="uffdio-poison-clone"></span>`fn clone(&self) -> uffdio_poison` — [`uffdio_poison`](#uffdio-poison)

##### `impl CloneToUninit for uffdio_poison`

- <span id="uffdio-poison-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_poison`

##### `impl Debug for uffdio_poison`

- <span id="uffdio-poison-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_poison`

- <span id="uffdio-poison-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_poison`

- <span id="uffdio-poison-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_poison`

- <span id="uffdio-poison-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-poison-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_poison`

- <span id="uffdio-poison-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-poison-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:921-927`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L921-L927)*

#### Trait Implementations

##### `impl Any for uffdio_move`

- <span id="uffdio-move-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for uffdio_move`

- <span id="uffdio-move-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for uffdio_move`

- <span id="uffdio-move-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for uffdio_move`

- <span id="uffdio-move-clone"></span>`fn clone(&self) -> uffdio_move` — [`uffdio_move`](#uffdio-move)

##### `impl CloneToUninit for uffdio_move`

- <span id="uffdio-move-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for uffdio_move`

##### `impl Debug for uffdio_move`

- <span id="uffdio-move-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for uffdio_move`

- <span id="uffdio-move-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for uffdio_move`

- <span id="uffdio-move-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for uffdio_move`

- <span id="uffdio-move-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uffdio-move-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for uffdio_move`

- <span id="uffdio-move-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uffdio-move-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:930-936`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L930-L936)*

#### Trait Implementations

##### `impl Any for linux_dirent64`

- <span id="linux-dirent64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for linux_dirent64`

- <span id="linux-dirent64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for linux_dirent64`

- <span id="linux-dirent64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for linux_dirent64`

- <span id="linux-dirent64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for linux_dirent64`

- <span id="linux-dirent64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for linux_dirent64`

- <span id="linux-dirent64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for linux_dirent64`

- <span id="linux-dirent64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linux-dirent64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for linux_dirent64`

- <span id="linux-dirent64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linux-dirent64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:939-958`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L939-L958)*

#### Trait Implementations

##### `impl Any for stat`

- <span id="stat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for stat`

- <span id="stat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for stat`

- <span id="stat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for stat`

- <span id="stat-clone"></span>`fn clone(&self) -> stat` — [`stat`](#stat)

##### `impl CloneToUninit for stat`

- <span id="stat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for stat`

##### `impl Debug for stat`

- <span id="stat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for stat`

- <span id="stat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for stat`

- <span id="stat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for stat`

- <span id="stat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for stat`

- <span id="stat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:961-973`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L961-L973)*

#### Trait Implementations

##### `impl Any for __old_kernel_stat`

- <span id="old-kernel-stat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __old_kernel_stat`

- <span id="old-kernel-stat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __old_kernel_stat`

- <span id="old-kernel-stat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __old_kernel_stat`

- <span id="old-kernel-stat-clone"></span>`fn clone(&self) -> __old_kernel_stat` — [`__old_kernel_stat`](#old-kernel-stat)

##### `impl CloneToUninit for __old_kernel_stat`

- <span id="old-kernel-stat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __old_kernel_stat`

##### `impl Debug for __old_kernel_stat`

- <span id="old-kernel-stat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __old_kernel_stat`

- <span id="old-kernel-stat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __old_kernel_stat`

- <span id="old-kernel-stat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __old_kernel_stat`

- <span id="old-kernel-stat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="old-kernel-stat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __old_kernel_stat`

- <span id="old-kernel-stat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="old-kernel-stat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:976-989`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L976-L989)*

#### Trait Implementations

##### `impl Any for statfs`

- <span id="statfs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for statfs`

- <span id="statfs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for statfs`

- <span id="statfs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for statfs`

- <span id="statfs-clone"></span>`fn clone(&self) -> statfs` — [`statfs`](#statfs)

##### `impl CloneToUninit for statfs`

- <span id="statfs-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for statfs`

##### `impl Debug for statfs`

- <span id="statfs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for statfs`

- <span id="statfs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for statfs`

- <span id="statfs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for statfs`

- <span id="statfs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statfs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for statfs`

- <span id="statfs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statfs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:992-1005`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L992-L1005)*

#### Trait Implementations

##### `impl Any for statfs64`

- <span id="statfs64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for statfs64`

- <span id="statfs64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for statfs64`

- <span id="statfs64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for statfs64`

- <span id="statfs64-clone"></span>`fn clone(&self) -> statfs64` — [`statfs64`](#statfs64)

##### `impl CloneToUninit for statfs64`

- <span id="statfs64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for statfs64`

##### `impl Debug for statfs64`

- <span id="statfs64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for statfs64`

- <span id="statfs64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for statfs64`

- <span id="statfs64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for statfs64`

- <span id="statfs64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statfs64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for statfs64`

- <span id="statfs64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statfs64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1008-1021`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1008-L1021)*

#### Trait Implementations

##### `impl Any for compat_statfs64`

- <span id="compat-statfs64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for compat_statfs64`

- <span id="compat-statfs64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for compat_statfs64`

- <span id="compat-statfs64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for compat_statfs64`

- <span id="compat-statfs64-clone"></span>`fn clone(&self) -> compat_statfs64` — [`compat_statfs64`](#compat-statfs64)

##### `impl CloneToUninit for compat_statfs64`

- <span id="compat-statfs64-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for compat_statfs64`

##### `impl Debug for compat_statfs64`

- <span id="compat-statfs64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for compat_statfs64`

- <span id="compat-statfs64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for compat_statfs64`

- <span id="compat-statfs64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for compat_statfs64`

- <span id="compat-statfs64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compat-statfs64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for compat_statfs64`

- <span id="compat-statfs64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compat-statfs64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1024-1031`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1024-L1031)*

#### Implementations

- <span id="user-desc-seg-32bit"></span>`fn seg_32bit(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-seg-32bit"></span>`fn set_seg_32bit(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-seg-32bit-raw"></span>`unsafe fn seg_32bit_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-seg-32bit-raw"></span>`unsafe fn set_seg_32bit_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-contents"></span>`fn contents(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-contents"></span>`fn set_contents(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-contents-raw"></span>`unsafe fn contents_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-contents-raw"></span>`unsafe fn set_contents_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-read-exec-only"></span>`fn read_exec_only(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-read-exec-only"></span>`fn set_read_exec_only(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-read-exec-only-raw"></span>`unsafe fn read_exec_only_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-read-exec-only-raw"></span>`unsafe fn set_read_exec_only_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-limit-in-pages"></span>`fn limit_in_pages(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-limit-in-pages"></span>`fn set_limit_in_pages(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-limit-in-pages-raw"></span>`unsafe fn limit_in_pages_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-limit-in-pages-raw"></span>`unsafe fn set_limit_in_pages_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-seg-not-present"></span>`fn seg_not_present(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-seg-not-present"></span>`fn set_seg_not_present(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-seg-not-present-raw"></span>`unsafe fn seg_not_present_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-seg-not-present-raw"></span>`unsafe fn set_seg_not_present_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-useable"></span>`fn useable(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-useable"></span>`fn set_useable(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-useable-raw"></span>`unsafe fn useable_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-useable-raw"></span>`unsafe fn set_useable_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-lm"></span>`fn lm(&self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-lm"></span>`fn set_lm(&mut self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-lm-raw"></span>`unsafe fn lm_raw(this: *const Self) -> crate::ctypes::c_uint` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-set-lm-raw"></span>`unsafe fn set_lm_raw(this: *mut Self, val: crate::ctypes::c_uint)` — [`c_uint`](../ctypes/index.md#c-uint)

- <span id="user-desc-new-bitfield-1"></span>`fn new_bitfield_1(seg_32bit: crate::ctypes::c_uint, contents: crate::ctypes::c_uint, read_exec_only: crate::ctypes::c_uint, limit_in_pages: crate::ctypes::c_uint, seg_not_present: crate::ctypes::c_uint, useable: crate::ctypes::c_uint, lm: crate::ctypes::c_uint) -> __BindgenBitfieldUnit<[u8; 1]>` — [`c_uint`](../ctypes/index.md#c-uint), [`__BindgenBitfieldUnit`](#bindgenbitfieldunit)

#### Trait Implementations

##### `impl Any for user_desc`

- <span id="user-desc-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for user_desc`

- <span id="user-desc-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for user_desc`

- <span id="user-desc-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for user_desc`

- <span id="user-desc-clone"></span>`fn clone(&self) -> user_desc` — [`user_desc`](#user-desc)

##### `impl CloneToUninit for user_desc`

- <span id="user-desc-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for user_desc`

##### `impl Debug for user_desc`

- <span id="user-desc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for user_desc`

- <span id="user-desc-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for user_desc`

- <span id="user-desc-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for user_desc`

- <span id="user-desc-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="user-desc-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for user_desc`

- <span id="user-desc-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="user-desc-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `kernel_sigset_t`

```rust
struct kernel_sigset_t {
    pub sig: [crate::ctypes::c_ulong; 1],
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1034-1036`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1034-L1036)*

#### Trait Implementations

##### `impl Any for kernel_sigset_t`

- <span id="kernel-sigset-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for kernel_sigset_t`

- <span id="kernel-sigset-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for kernel_sigset_t`

- <span id="kernel-sigset-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for kernel_sigset_t`

- <span id="kernel-sigset-t-clone"></span>`fn clone(&self) -> kernel_sigset_t` — [`kernel_sigset_t`](#kernel-sigset-t)

##### `impl CloneToUninit for kernel_sigset_t`

- <span id="kernel-sigset-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for kernel_sigset_t`

##### `impl Debug for kernel_sigset_t`

- <span id="kernel-sigset-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for kernel_sigset_t`

- <span id="kernel-sigset-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for kernel_sigset_t`

- <span id="kernel-sigset-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for kernel_sigset_t`

- <span id="kernel-sigset-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-sigset-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for kernel_sigset_t`

- <span id="kernel-sigset-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-sigset-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `kernel_sigaction`

```rust
struct kernel_sigaction {
    pub sa_handler_kernel: __kernel_sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: kernel_sigset_t,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1039-1044`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1039-L1044)*

#### Trait Implementations

##### `impl Any for kernel_sigaction`

- <span id="kernel-sigaction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for kernel_sigaction`

- <span id="kernel-sigaction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for kernel_sigaction`

- <span id="kernel-sigaction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for kernel_sigaction`

- <span id="kernel-sigaction-clone"></span>`fn clone(&self) -> kernel_sigaction` — [`kernel_sigaction`](#kernel-sigaction)

##### `impl CloneToUninit for kernel_sigaction`

- <span id="kernel-sigaction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for kernel_sigaction`

##### `impl Debug for kernel_sigaction`

- <span id="kernel-sigaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for kernel_sigaction`

- <span id="kernel-sigaction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for kernel_sigaction`

- <span id="kernel-sigaction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for kernel_sigaction`

- <span id="kernel-sigaction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kernel-sigaction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for kernel_sigaction`

- <span id="kernel-sigaction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kernel-sigaction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2835-2845`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2835-L2845)*

#### Trait Implementations

##### `impl Any for fsconfig_command`

- <span id="fsconfig-command-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fsconfig_command`

- <span id="fsconfig-command-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fsconfig_command`

- <span id="fsconfig-command-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fsconfig_command`

- <span id="fsconfig-command-clone"></span>`fn clone(&self) -> fsconfig_command` — [`fsconfig_command`](#fsconfig-command)

##### `impl CloneToUninit for fsconfig_command`

- <span id="fsconfig-command-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fsconfig_command`

##### `impl Debug for fsconfig_command`

- <span id="fsconfig-command-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for fsconfig_command`

##### `impl<T> From for fsconfig_command`

- <span id="fsconfig-command-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for fsconfig_command`

- <span id="fsconfig-command-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for fsconfig_command`

- <span id="fsconfig-command-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for fsconfig_command`

- <span id="fsconfig-command-partialeq-eq"></span>`fn eq(&self, other: &fsconfig_command) -> bool` — [`fsconfig_command`](#fsconfig-command)

##### `impl StructuralPartialEq for fsconfig_command`

##### `impl<U> TryFrom for fsconfig_command`

- <span id="fsconfig-command-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fsconfig-command-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fsconfig_command`

- <span id="fsconfig-command-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fsconfig-command-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2849-2856`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2849-L2856)*

#### Trait Implementations

##### `impl Any for procmap_query_flags`

- <span id="procmap-query-flags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for procmap_query_flags`

- <span id="procmap-query-flags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for procmap_query_flags`

- <span id="procmap-query-flags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for procmap_query_flags`

- <span id="procmap-query-flags-clone"></span>`fn clone(&self) -> procmap_query_flags` — [`procmap_query_flags`](#procmap-query-flags)

##### `impl CloneToUninit for procmap_query_flags`

- <span id="procmap-query-flags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for procmap_query_flags`

##### `impl Debug for procmap_query_flags`

- <span id="procmap-query-flags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for procmap_query_flags`

##### `impl<T> From for procmap_query_flags`

- <span id="procmap-query-flags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for procmap_query_flags`

- <span id="procmap-query-flags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for procmap_query_flags`

- <span id="procmap-query-flags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for procmap_query_flags`

- <span id="procmap-query-flags-partialeq-eq"></span>`fn eq(&self, other: &procmap_query_flags) -> bool` — [`procmap_query_flags`](#procmap-query-flags)

##### `impl StructuralPartialEq for procmap_query_flags`

##### `impl<U> TryFrom for procmap_query_flags`

- <span id="procmap-query-flags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="procmap-query-flags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for procmap_query_flags`

- <span id="procmap-query-flags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="procmap-query-flags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2860-2872`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2860-L2872)*

#### Implementations

- <span id="membarrier-cmd-const-membarrier-cmd-shared"></span>`const MEMBARRIER_CMD_SHARED: membarrier_cmd`

#### Trait Implementations

##### `impl Any for membarrier_cmd`

- <span id="membarrier-cmd-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for membarrier_cmd`

- <span id="membarrier-cmd-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for membarrier_cmd`

- <span id="membarrier-cmd-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for membarrier_cmd`

- <span id="membarrier-cmd-clone"></span>`fn clone(&self) -> membarrier_cmd` — [`membarrier_cmd`](#membarrier-cmd)

##### `impl CloneToUninit for membarrier_cmd`

- <span id="membarrier-cmd-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for membarrier_cmd`

##### `impl Debug for membarrier_cmd`

- <span id="membarrier-cmd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for membarrier_cmd`

##### `impl<T> From for membarrier_cmd`

- <span id="membarrier-cmd-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for membarrier_cmd`

- <span id="membarrier-cmd-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for membarrier_cmd`

- <span id="membarrier-cmd-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for membarrier_cmd`

- <span id="membarrier-cmd-partialeq-eq"></span>`fn eq(&self, other: &membarrier_cmd) -> bool` — [`membarrier_cmd`](#membarrier-cmd)

##### `impl StructuralPartialEq for membarrier_cmd`

##### `impl<U> TryFrom for membarrier_cmd`

- <span id="membarrier-cmd-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="membarrier-cmd-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for membarrier_cmd`

- <span id="membarrier-cmd-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="membarrier-cmd-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `membarrier_cmd_flag`

```rust
enum membarrier_cmd_flag {
    MEMBARRIER_CMD_FLAG_CPU,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2876-2878`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2876-L2878)*

#### Trait Implementations

##### `impl Any for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-clone"></span>`fn clone(&self) -> membarrier_cmd_flag` — [`membarrier_cmd_flag`](#membarrier-cmd-flag)

##### `impl CloneToUninit for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for membarrier_cmd_flag`

##### `impl Debug for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for membarrier_cmd_flag`

##### `impl<T> From for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-partialeq-eq"></span>`fn eq(&self, other: &membarrier_cmd_flag) -> bool` — [`membarrier_cmd_flag`](#membarrier-cmd-flag)

##### `impl StructuralPartialEq for membarrier_cmd_flag`

##### `impl<U> TryFrom for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="membarrier-cmd-flag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for membarrier_cmd_flag`

- <span id="membarrier-cmd-flag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="membarrier-cmd-flag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `__s8`

```rust
type __s8 = crate::ctypes::c_schar;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:3`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L3)*

### `__u8`

```rust
type __u8 = crate::ctypes::c_uchar;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:4`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L4)*

### `__s16`

```rust
type __s16 = crate::ctypes::c_short;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:5`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L5)*

### `__u16`

```rust
type __u16 = crate::ctypes::c_ushort;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:6`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L6)*

### `__s32`

```rust
type __s32 = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:7`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L7)*

### `__u32`

```rust
type __u32 = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:8`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L8)*

### `__s64`

```rust
type __s64 = crate::ctypes::c_longlong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:9`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L9)*

### `__u64`

```rust
type __u64 = crate::ctypes::c_ulonglong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:10`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L10)*

### `__kernel_sighandler_t`

```rust
type __kernel_sighandler_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:11`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L11)*

### `__kernel_key_t`

```rust
type __kernel_key_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:12`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L12)*

### `__kernel_mqd_t`

```rust
type __kernel_mqd_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:13`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L13)*

### `__kernel_old_uid_t`

```rust
type __kernel_old_uid_t = crate::ctypes::c_ushort;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:14`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L14)*

### `__kernel_old_gid_t`

```rust
type __kernel_old_gid_t = crate::ctypes::c_ushort;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:15`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L15)*

### `__kernel_old_dev_t`

```rust
type __kernel_old_dev_t = crate::ctypes::c_ulong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:16`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L16)*

### `__kernel_long_t`

```rust
type __kernel_long_t = crate::ctypes::c_long;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:17`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L17)*

### `__kernel_ulong_t`

```rust
type __kernel_ulong_t = crate::ctypes::c_ulong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:18`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L18)*

### `__kernel_ino_t`

```rust
type __kernel_ino_t = __kernel_ulong_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:19`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L19)*

### `__kernel_mode_t`

```rust
type __kernel_mode_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:20`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L20)*

### `__kernel_pid_t`

```rust
type __kernel_pid_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:21`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L21)*

### `__kernel_ipc_pid_t`

```rust
type __kernel_ipc_pid_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:22`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L22)*

### `__kernel_uid_t`

```rust
type __kernel_uid_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:23`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L23)*

### `__kernel_gid_t`

```rust
type __kernel_gid_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:24`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L24)*

### `__kernel_suseconds_t`

```rust
type __kernel_suseconds_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:25`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L25)*

### `__kernel_daddr_t`

```rust
type __kernel_daddr_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:26`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L26)*

### `__kernel_uid32_t`

```rust
type __kernel_uid32_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:27`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L27)*

### `__kernel_gid32_t`

```rust
type __kernel_gid32_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:28`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L28)*

### `__kernel_size_t`

```rust
type __kernel_size_t = __kernel_ulong_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:29`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L29)*

### `__kernel_ssize_t`

```rust
type __kernel_ssize_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:30`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L30)*

### `__kernel_ptrdiff_t`

```rust
type __kernel_ptrdiff_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:31`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L31)*

### `__kernel_off_t`

```rust
type __kernel_off_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:32`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L32)*

### `__kernel_loff_t`

```rust
type __kernel_loff_t = crate::ctypes::c_longlong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:33`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L33)*

### `__kernel_old_time_t`

```rust
type __kernel_old_time_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:34`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L34)*

### `__kernel_time_t`

```rust
type __kernel_time_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:35`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L35)*

### `__kernel_time64_t`

```rust
type __kernel_time64_t = crate::ctypes::c_longlong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:36`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L36)*

### `__kernel_clock_t`

```rust
type __kernel_clock_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:37`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L37)*

### `__kernel_timer_t`

```rust
type __kernel_timer_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:38`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L38)*

### `__kernel_clockid_t`

```rust
type __kernel_clockid_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:39`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L39)*

### `__kernel_caddr_t`

```rust
type __kernel_caddr_t = *mut crate::ctypes::c_char;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:40`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L40)*

### `__kernel_uid16_t`

```rust
type __kernel_uid16_t = crate::ctypes::c_ushort;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:41`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L41)*

### `__kernel_gid16_t`

```rust
type __kernel_gid16_t = crate::ctypes::c_ushort;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:42`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L42)*

### `__s128`

```rust
type __s128 = i128;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:43`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L43)*

### `__u128`

```rust
type __u128 = u128;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:44`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L44)*

### `__le16`

```rust
type __le16 = __u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:45`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L45)*

### `__be16`

```rust
type __be16 = __u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:46`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L46)*

### `__le32`

```rust
type __le32 = __u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:47`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L47)*

### `__be32`

```rust
type __be32 = __u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:48`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L48)*

### `__le64`

```rust
type __le64 = __u64;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:49`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L49)*

### `__be64`

```rust
type __be64 = __u64;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:50`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L50)*

### `__sum16`

```rust
type __sum16 = __u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:51`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L51)*

### `__wsum`

```rust
type __wsum = __u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:52`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L52)*

### `__poll_t`

```rust
type __poll_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:53`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L53)*

### `cap_user_header_t`

```rust
type cap_user_header_t = *mut __user_cap_header_struct;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:54`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L54)*

### `cap_user_data_t`

```rust
type cap_user_data_t = *mut __user_cap_data_struct;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:55`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L55)*

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = crate::ctypes::c_int;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:56`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L56)*

### `sigset_t`

```rust
type sigset_t = crate::ctypes::c_ulong;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:57`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L57)*

### `__signalfn_t`

```rust
type __signalfn_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:58`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L58)*

### `__sighandler_t`

```rust
type __sighandler_t = __signalfn_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:59`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L59)*

### `__restorefn_t`

```rust
type __restorefn_t = ::core::option::Option<fn()>;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:60`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L60)*

### `__sigrestore_t`

```rust
type __sigrestore_t = __restorefn_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:61`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L61)*

### `stack_t`

```rust
type stack_t = sigaltstack;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:62`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L62)*

### `sigval_t`

```rust
type sigval_t = sigval;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:63`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L63)*

### `siginfo_t`

```rust
type siginfo_t = siginfo;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:64`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L64)*

### `sigevent_t`

```rust
type sigevent_t = sigevent;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:65`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L65)*

### `cc_t`

```rust
type cc_t = crate::ctypes::c_uchar;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:66`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L66)*

### `speed_t`

```rust
type speed_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:67`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L67)*

### `tcflag_t`

```rust
type tcflag_t = crate::ctypes::c_uint;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:68`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L68)*

### `__fsword_t`

```rust
type __fsword_t = __kernel_long_t;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:69`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L69)*

## Constants

### `LINUX_VERSION_CODE`
```rust
const LINUX_VERSION_CODE: u32 = 397_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1045`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1045)*

### `LINUX_VERSION_MAJOR`
```rust
const LINUX_VERSION_MAJOR: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1046`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1046)*

### `LINUX_VERSION_PATCHLEVEL`
```rust
const LINUX_VERSION_PATCHLEVEL: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1047`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1047)*

### `LINUX_VERSION_SUBLEVEL`
```rust
const LINUX_VERSION_SUBLEVEL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1048`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1048)*

### `__BITS_PER_LONG_LONG`
```rust
const __BITS_PER_LONG_LONG: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1049`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1049)*

### `__FD_SETSIZE`
```rust
const __FD_SETSIZE: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1050`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1050)*

### `_LINUX_CAPABILITY_VERSION_1`
```rust
const _LINUX_CAPABILITY_VERSION_1: u32 = 429_392_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1051`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1051)*

### `_LINUX_CAPABILITY_U32S_1`
```rust
const _LINUX_CAPABILITY_U32S_1: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1052`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1052)*

### `_LINUX_CAPABILITY_VERSION_2`
```rust
const _LINUX_CAPABILITY_VERSION_2: u32 = 537_333_798u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1053`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1053)*

### `_LINUX_CAPABILITY_U32S_2`
```rust
const _LINUX_CAPABILITY_U32S_2: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1054`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1054)*

### `_LINUX_CAPABILITY_VERSION_3`
```rust
const _LINUX_CAPABILITY_VERSION_3: u32 = 537_396_514u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1055`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1055)*

### `_LINUX_CAPABILITY_U32S_3`
```rust
const _LINUX_CAPABILITY_U32S_3: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1056`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1056)*

### `VFS_CAP_REVISION_MASK`
```rust
const VFS_CAP_REVISION_MASK: u32 = 4_278_190_080u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1057`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1057)*

### `VFS_CAP_REVISION_SHIFT`
```rust
const VFS_CAP_REVISION_SHIFT: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1058`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1058)*

### `VFS_CAP_FLAGS_MASK`
```rust
const VFS_CAP_FLAGS_MASK: i64 = -4_278_190_081i64;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1059`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1059)*

### `VFS_CAP_FLAGS_EFFECTIVE`
```rust
const VFS_CAP_FLAGS_EFFECTIVE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1060`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1060)*

### `VFS_CAP_REVISION_1`
```rust
const VFS_CAP_REVISION_1: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1061`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1061)*

### `VFS_CAP_U32_1`
```rust
const VFS_CAP_U32_1: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1062`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1062)*

### `VFS_CAP_REVISION_2`
```rust
const VFS_CAP_REVISION_2: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1063`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1063)*

### `VFS_CAP_U32_2`
```rust
const VFS_CAP_U32_2: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1064`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1064)*

### `VFS_CAP_REVISION_3`
```rust
const VFS_CAP_REVISION_3: u32 = 50_331_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1065`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1065)*

### `VFS_CAP_U32_3`
```rust
const VFS_CAP_U32_3: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1066`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1066)*

### `VFS_CAP_U32`
```rust
const VFS_CAP_U32: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1067`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1067)*

### `VFS_CAP_REVISION`
```rust
const VFS_CAP_REVISION: u32 = 50_331_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1068`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1068)*

### `_LINUX_CAPABILITY_VERSION`
```rust
const _LINUX_CAPABILITY_VERSION: u32 = 429_392_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1069`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1069)*

### `_LINUX_CAPABILITY_U32S`
```rust
const _LINUX_CAPABILITY_U32S: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1070`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1070)*

### `CAP_CHOWN`
```rust
const CAP_CHOWN: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1071`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1071)*

### `CAP_DAC_OVERRIDE`
```rust
const CAP_DAC_OVERRIDE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1072`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1072)*

### `CAP_DAC_READ_SEARCH`
```rust
const CAP_DAC_READ_SEARCH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1073`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1073)*

### `CAP_FOWNER`
```rust
const CAP_FOWNER: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1074`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1074)*

### `CAP_FSETID`
```rust
const CAP_FSETID: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1075`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1075)*

### `CAP_KILL`
```rust
const CAP_KILL: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1076`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1076)*

### `CAP_SETGID`
```rust
const CAP_SETGID: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1077`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1077)*

### `CAP_SETUID`
```rust
const CAP_SETUID: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1078`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1078)*

### `CAP_SETPCAP`
```rust
const CAP_SETPCAP: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1079`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1079)*

### `CAP_LINUX_IMMUTABLE`
```rust
const CAP_LINUX_IMMUTABLE: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1080`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1080)*

### `CAP_NET_BIND_SERVICE`
```rust
const CAP_NET_BIND_SERVICE: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1081`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1081)*

### `CAP_NET_BROADCAST`
```rust
const CAP_NET_BROADCAST: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1082`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1082)*

### `CAP_NET_ADMIN`
```rust
const CAP_NET_ADMIN: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1083`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1083)*

### `CAP_NET_RAW`
```rust
const CAP_NET_RAW: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1084`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1084)*

### `CAP_IPC_LOCK`
```rust
const CAP_IPC_LOCK: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1085`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1085)*

### `CAP_IPC_OWNER`
```rust
const CAP_IPC_OWNER: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1086`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1086)*

### `CAP_SYS_MODULE`
```rust
const CAP_SYS_MODULE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1087`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1087)*

### `CAP_SYS_RAWIO`
```rust
const CAP_SYS_RAWIO: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1088`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1088)*

### `CAP_SYS_CHROOT`
```rust
const CAP_SYS_CHROOT: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1089`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1089)*

### `CAP_SYS_PTRACE`
```rust
const CAP_SYS_PTRACE: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1090`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1090)*

### `CAP_SYS_PACCT`
```rust
const CAP_SYS_PACCT: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1091`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1091)*

### `CAP_SYS_ADMIN`
```rust
const CAP_SYS_ADMIN: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1092`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1092)*

### `CAP_SYS_BOOT`
```rust
const CAP_SYS_BOOT: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1093`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1093)*

### `CAP_SYS_NICE`
```rust
const CAP_SYS_NICE: u32 = 23u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1094`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1094)*

### `CAP_SYS_RESOURCE`
```rust
const CAP_SYS_RESOURCE: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1095`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1095)*

### `CAP_SYS_TIME`
```rust
const CAP_SYS_TIME: u32 = 25u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1096`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1096)*

### `CAP_SYS_TTY_CONFIG`
```rust
const CAP_SYS_TTY_CONFIG: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1097`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1097)*

### `CAP_MKNOD`
```rust
const CAP_MKNOD: u32 = 27u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1098`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1098)*

### `CAP_LEASE`
```rust
const CAP_LEASE: u32 = 28u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1099`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1099)*

### `CAP_AUDIT_WRITE`
```rust
const CAP_AUDIT_WRITE: u32 = 29u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1100`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1100)*

### `CAP_AUDIT_CONTROL`
```rust
const CAP_AUDIT_CONTROL: u32 = 30u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1101`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1101)*

### `CAP_SETFCAP`
```rust
const CAP_SETFCAP: u32 = 31u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1102`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1102)*

### `CAP_MAC_OVERRIDE`
```rust
const CAP_MAC_OVERRIDE: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1103`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1103)*

### `CAP_MAC_ADMIN`
```rust
const CAP_MAC_ADMIN: u32 = 33u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1104`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1104)*

### `CAP_SYSLOG`
```rust
const CAP_SYSLOG: u32 = 34u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1105`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1105)*

### `CAP_WAKE_ALARM`
```rust
const CAP_WAKE_ALARM: u32 = 35u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1106`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1106)*

### `CAP_BLOCK_SUSPEND`
```rust
const CAP_BLOCK_SUSPEND: u32 = 36u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1107`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1107)*

### `CAP_AUDIT_READ`
```rust
const CAP_AUDIT_READ: u32 = 37u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1108`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1108)*

### `CAP_PERFMON`
```rust
const CAP_PERFMON: u32 = 38u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1109`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1109)*

### `CAP_BPF`
```rust
const CAP_BPF: u32 = 39u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1110`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1110)*

### `CAP_CHECKPOINT_RESTORE`
```rust
const CAP_CHECKPOINT_RESTORE: u32 = 40u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1111`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1111)*

### `CAP_LAST_CAP`
```rust
const CAP_LAST_CAP: u32 = 40u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1112`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1112)*

### `O_ACCMODE`
```rust
const O_ACCMODE: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1113`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1113)*

### `O_RDONLY`
```rust
const O_RDONLY: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1114`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1114)*

### `O_WRONLY`
```rust
const O_WRONLY: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1115`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1115)*

### `O_RDWR`
```rust
const O_RDWR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1116`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1116)*

### `O_CREAT`
```rust
const O_CREAT: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1117`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1117)*

### `O_EXCL`
```rust
const O_EXCL: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1118`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1118)*

### `O_NOCTTY`
```rust
const O_NOCTTY: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1119`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1119)*

### `O_TRUNC`
```rust
const O_TRUNC: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1120`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1120)*

### `O_APPEND`
```rust
const O_APPEND: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1121`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1121)*

### `O_NONBLOCK`
```rust
const O_NONBLOCK: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1122`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1122)*

### `O_DSYNC`
```rust
const O_DSYNC: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1123`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1123)*

### `FASYNC`
```rust
const FASYNC: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1124`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1124)*

### `O_DIRECT`
```rust
const O_DIRECT: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1125`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1125)*

### `O_LARGEFILE`
```rust
const O_LARGEFILE: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1126`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1126)*

### `O_DIRECTORY`
```rust
const O_DIRECTORY: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1127`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1127)*

### `O_NOFOLLOW`
```rust
const O_NOFOLLOW: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1128`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1128)*

### `O_NOATIME`
```rust
const O_NOATIME: u32 = 262_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1129`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1129)*

### `O_CLOEXEC`
```rust
const O_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1130`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1130)*

### `__O_SYNC`
```rust
const __O_SYNC: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1131`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1131)*

### `O_SYNC`
```rust
const O_SYNC: u32 = 1_052_672u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1132`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1132)*

### `O_PATH`
```rust
const O_PATH: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1133`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1133)*

### `__O_TMPFILE`
```rust
const __O_TMPFILE: u32 = 4_194_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1134`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1134)*

### `O_TMPFILE`
```rust
const O_TMPFILE: u32 = 4_259_840u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1135`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1135)*

### `O_NDELAY`
```rust
const O_NDELAY: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1136`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1136)*

### `F_DUPFD`
```rust
const F_DUPFD: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1137`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1137)*

### `F_GETFD`
```rust
const F_GETFD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1138`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1138)*

### `F_SETFD`
```rust
const F_SETFD: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1139`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1139)*

### `F_GETFL`
```rust
const F_GETFL: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1140`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1140)*

### `F_SETFL`
```rust
const F_SETFL: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1141`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1141)*

### `F_GETLK`
```rust
const F_GETLK: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1142`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1142)*

### `F_SETLK`
```rust
const F_SETLK: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1143`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1143)*

### `F_SETLKW`
```rust
const F_SETLKW: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1144`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1144)*

### `F_SETOWN`
```rust
const F_SETOWN: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1145`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1145)*

### `F_GETOWN`
```rust
const F_GETOWN: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1146`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1146)*

### `F_SETSIG`
```rust
const F_SETSIG: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1147`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1147)*

### `F_GETSIG`
```rust
const F_GETSIG: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1148`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1148)*

### `F_SETOWN_EX`
```rust
const F_SETOWN_EX: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1149`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1149)*

### `F_GETOWN_EX`
```rust
const F_GETOWN_EX: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1150`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1150)*

### `F_GETOWNER_UIDS`
```rust
const F_GETOWNER_UIDS: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1151`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1151)*

### `F_OFD_GETLK`
```rust
const F_OFD_GETLK: u32 = 36u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1152`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1152)*

### `F_OFD_SETLK`
```rust
const F_OFD_SETLK: u32 = 37u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1153`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1153)*

### `F_OFD_SETLKW`
```rust
const F_OFD_SETLKW: u32 = 38u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1154`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1154)*

### `F_OWNER_TID`
```rust
const F_OWNER_TID: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1155`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1155)*

### `F_OWNER_PID`
```rust
const F_OWNER_PID: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1156`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1156)*

### `F_OWNER_PGRP`
```rust
const F_OWNER_PGRP: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1157`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1157)*

### `FD_CLOEXEC`
```rust
const FD_CLOEXEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1158`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1158)*

### `F_RDLCK`
```rust
const F_RDLCK: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1159`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1159)*

### `F_WRLCK`
```rust
const F_WRLCK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1160`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1160)*

### `F_UNLCK`
```rust
const F_UNLCK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1161`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1161)*

### `F_EXLCK`
```rust
const F_EXLCK: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1162`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1162)*

### `F_SHLCK`
```rust
const F_SHLCK: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1163`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1163)*

### `LOCK_SH`
```rust
const LOCK_SH: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1164`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1164)*

### `LOCK_EX`
```rust
const LOCK_EX: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1165`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1165)*

### `LOCK_NB`
```rust
const LOCK_NB: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1166`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1166)*

### `LOCK_UN`
```rust
const LOCK_UN: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1167`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1167)*

### `LOCK_MAND`
```rust
const LOCK_MAND: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1168`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1168)*

### `LOCK_READ`
```rust
const LOCK_READ: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1169`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1169)*

### `LOCK_WRITE`
```rust
const LOCK_WRITE: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1170`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1170)*

### `LOCK_RW`
```rust
const LOCK_RW: u32 = 192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1171`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1171)*

### `F_LINUX_SPECIFIC_BASE`
```rust
const F_LINUX_SPECIFIC_BASE: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1172`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1172)*

### `RESOLVE_NO_XDEV`
```rust
const RESOLVE_NO_XDEV: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1173`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1173)*

### `RESOLVE_NO_MAGICLINKS`
```rust
const RESOLVE_NO_MAGICLINKS: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1174`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1174)*

### `RESOLVE_NO_SYMLINKS`
```rust
const RESOLVE_NO_SYMLINKS: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1175`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1175)*

### `RESOLVE_BENEATH`
```rust
const RESOLVE_BENEATH: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1176`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1176)*

### `RESOLVE_IN_ROOT`
```rust
const RESOLVE_IN_ROOT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1177`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1177)*

### `RESOLVE_CACHED`
```rust
const RESOLVE_CACHED: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1178`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1178)*

### `F_SETLEASE`
```rust
const F_SETLEASE: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1179`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1179)*

### `F_GETLEASE`
```rust
const F_GETLEASE: u32 = 1_025u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1180`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1180)*

### `F_NOTIFY`
```rust
const F_NOTIFY: u32 = 1_026u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1181`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1181)*

### `F_DUPFD_QUERY`
```rust
const F_DUPFD_QUERY: u32 = 1_027u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1182`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1182)*

### `F_CREATED_QUERY`
```rust
const F_CREATED_QUERY: u32 = 1_028u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1183`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1183)*

### `F_CANCELLK`
```rust
const F_CANCELLK: u32 = 1_029u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1184`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1184)*

### `F_DUPFD_CLOEXEC`
```rust
const F_DUPFD_CLOEXEC: u32 = 1_030u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1185`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1185)*

### `F_SETPIPE_SZ`
```rust
const F_SETPIPE_SZ: u32 = 1_031u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1186`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1186)*

### `F_GETPIPE_SZ`
```rust
const F_GETPIPE_SZ: u32 = 1_032u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1187`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1187)*

### `F_ADD_SEALS`
```rust
const F_ADD_SEALS: u32 = 1_033u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1188`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1188)*

### `F_GET_SEALS`
```rust
const F_GET_SEALS: u32 = 1_034u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1189`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1189)*

### `F_SEAL_SEAL`
```rust
const F_SEAL_SEAL: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1190`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1190)*

### `F_SEAL_SHRINK`
```rust
const F_SEAL_SHRINK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1191`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1191)*

### `F_SEAL_GROW`
```rust
const F_SEAL_GROW: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1192`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1192)*

### `F_SEAL_WRITE`
```rust
const F_SEAL_WRITE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1193`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1193)*

### `F_SEAL_FUTURE_WRITE`
```rust
const F_SEAL_FUTURE_WRITE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1194`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1194)*

### `F_SEAL_EXEC`
```rust
const F_SEAL_EXEC: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1195`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1195)*

### `F_GET_RW_HINT`
```rust
const F_GET_RW_HINT: u32 = 1_035u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1196`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1196)*

### `F_SET_RW_HINT`
```rust
const F_SET_RW_HINT: u32 = 1_036u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1197`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1197)*

### `F_GET_FILE_RW_HINT`
```rust
const F_GET_FILE_RW_HINT: u32 = 1_037u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1198`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1198)*

### `F_SET_FILE_RW_HINT`
```rust
const F_SET_FILE_RW_HINT: u32 = 1_038u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1199`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1199)*

### `RWH_WRITE_LIFE_NOT_SET`
```rust
const RWH_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1200`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1200)*

### `RWH_WRITE_LIFE_NONE`
```rust
const RWH_WRITE_LIFE_NONE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1201`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1201)*

### `RWH_WRITE_LIFE_SHORT`
```rust
const RWH_WRITE_LIFE_SHORT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1202`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1202)*

### `RWH_WRITE_LIFE_MEDIUM`
```rust
const RWH_WRITE_LIFE_MEDIUM: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1203`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1203)*

### `RWH_WRITE_LIFE_LONG`
```rust
const RWH_WRITE_LIFE_LONG: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1204`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1204)*

### `RWH_WRITE_LIFE_EXTREME`
```rust
const RWH_WRITE_LIFE_EXTREME: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1205`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1205)*

### `RWF_WRITE_LIFE_NOT_SET`
```rust
const RWF_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1206`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1206)*

### `DN_ACCESS`
```rust
const DN_ACCESS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1207`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1207)*

### `DN_MODIFY`
```rust
const DN_MODIFY: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1208`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1208)*

### `DN_CREATE`
```rust
const DN_CREATE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1209`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1209)*

### `DN_DELETE`
```rust
const DN_DELETE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1210`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1210)*

### `DN_RENAME`
```rust
const DN_RENAME: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1211`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1211)*

### `DN_ATTRIB`
```rust
const DN_ATTRIB: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1212`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1212)*

### `DN_MULTISHOT`
```rust
const DN_MULTISHOT: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1213`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1213)*

### `AT_FDCWD`
```rust
const AT_FDCWD: i32 = -100i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1214`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1214)*

### `AT_SYMLINK_NOFOLLOW`
```rust
const AT_SYMLINK_NOFOLLOW: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1215`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1215)*

### `AT_SYMLINK_FOLLOW`
```rust
const AT_SYMLINK_FOLLOW: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1216`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1216)*

### `AT_NO_AUTOMOUNT`
```rust
const AT_NO_AUTOMOUNT: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1217`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1217)*

### `AT_EMPTY_PATH`
```rust
const AT_EMPTY_PATH: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1218`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1218)*

### `AT_STATX_SYNC_TYPE`
```rust
const AT_STATX_SYNC_TYPE: u32 = 24_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1219`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1219)*

### `AT_STATX_SYNC_AS_STAT`
```rust
const AT_STATX_SYNC_AS_STAT: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1220`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1220)*

### `AT_STATX_FORCE_SYNC`
```rust
const AT_STATX_FORCE_SYNC: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1221`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1221)*

### `AT_STATX_DONT_SYNC`
```rust
const AT_STATX_DONT_SYNC: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1222`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1222)*

### `AT_RECURSIVE`
```rust
const AT_RECURSIVE: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1223`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1223)*

### `AT_RENAME_NOREPLACE`
```rust
const AT_RENAME_NOREPLACE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1224`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1224)*

### `AT_RENAME_EXCHANGE`
```rust
const AT_RENAME_EXCHANGE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1225`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1225)*

### `AT_RENAME_WHITEOUT`
```rust
const AT_RENAME_WHITEOUT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1226`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1226)*

### `AT_EACCESS`
```rust
const AT_EACCESS: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1227`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1227)*

### `AT_REMOVEDIR`
```rust
const AT_REMOVEDIR: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1228`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1228)*

### `AT_HANDLE_FID`
```rust
const AT_HANDLE_FID: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1229`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1229)*

### `AT_HANDLE_MNT_ID_UNIQUE`
```rust
const AT_HANDLE_MNT_ID_UNIQUE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1230`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1230)*

### `AT_HANDLE_CONNECTABLE`
```rust
const AT_HANDLE_CONNECTABLE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1231`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1231)*

### `AT_EXECVE_CHECK`
```rust
const AT_EXECVE_CHECK: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1232`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1232)*

### `EPOLL_CLOEXEC`
```rust
const EPOLL_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1233`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1233)*

### `EPOLL_CTL_ADD`
```rust
const EPOLL_CTL_ADD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1234`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1234)*

### `EPOLL_CTL_DEL`
```rust
const EPOLL_CTL_DEL: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1235`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1235)*

### `EPOLL_CTL_MOD`
```rust
const EPOLL_CTL_MOD: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1236`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1236)*

### `EPOLL_IOC_TYPE`
```rust
const EPOLL_IOC_TYPE: u32 = 138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1237`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1237)*

### `POSIX_FADV_NORMAL`
```rust
const POSIX_FADV_NORMAL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1238`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1238)*

### `POSIX_FADV_RANDOM`
```rust
const POSIX_FADV_RANDOM: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1239`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1239)*

### `POSIX_FADV_SEQUENTIAL`
```rust
const POSIX_FADV_SEQUENTIAL: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1240`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1240)*

### `POSIX_FADV_WILLNEED`
```rust
const POSIX_FADV_WILLNEED: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1241`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1241)*

### `POSIX_FADV_DONTNEED`
```rust
const POSIX_FADV_DONTNEED: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1242`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1242)*

### `POSIX_FADV_NOREUSE`
```rust
const POSIX_FADV_NOREUSE: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1243`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1243)*

### `FALLOC_FL_ALLOCATE_RANGE`
```rust
const FALLOC_FL_ALLOCATE_RANGE: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1244`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1244)*

### `FALLOC_FL_KEEP_SIZE`
```rust
const FALLOC_FL_KEEP_SIZE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1245`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1245)*

### `FALLOC_FL_PUNCH_HOLE`
```rust
const FALLOC_FL_PUNCH_HOLE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1246`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1246)*

### `FALLOC_FL_NO_HIDE_STALE`
```rust
const FALLOC_FL_NO_HIDE_STALE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1247`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1247)*

### `FALLOC_FL_COLLAPSE_RANGE`
```rust
const FALLOC_FL_COLLAPSE_RANGE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1248`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1248)*

### `FALLOC_FL_ZERO_RANGE`
```rust
const FALLOC_FL_ZERO_RANGE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1249`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1249)*

### `FALLOC_FL_INSERT_RANGE`
```rust
const FALLOC_FL_INSERT_RANGE: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1250`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1250)*

### `FALLOC_FL_UNSHARE_RANGE`
```rust
const FALLOC_FL_UNSHARE_RANGE: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1251`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1251)*

### `NR_OPEN`
```rust
const NR_OPEN: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1252`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1252)*

### `NGROUPS_MAX`
```rust
const NGROUPS_MAX: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1253`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1253)*

### `ARG_MAX`
```rust
const ARG_MAX: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1254`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1254)*

### `LINK_MAX`
```rust
const LINK_MAX: u32 = 127u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1255`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1255)*

### `MAX_CANON`
```rust
const MAX_CANON: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1256`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1256)*

### `MAX_INPUT`
```rust
const MAX_INPUT: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1257`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1257)*

### `NAME_MAX`
```rust
const NAME_MAX: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1258`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1258)*

### `PATH_MAX`
```rust
const PATH_MAX: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1259`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1259)*

### `PIPE_BUF`
```rust
const PIPE_BUF: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1260`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1260)*

### `XATTR_NAME_MAX`
```rust
const XATTR_NAME_MAX: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1261`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1261)*

### `XATTR_SIZE_MAX`
```rust
const XATTR_SIZE_MAX: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1262`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1262)*

### `XATTR_LIST_MAX`
```rust
const XATTR_LIST_MAX: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1263`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1263)*

### `RTSIG_MAX`
```rust
const RTSIG_MAX: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1264`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1264)*

### `_IOC_NRBITS`
```rust
const _IOC_NRBITS: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1265`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1265)*

### `_IOC_TYPEBITS`
```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1266`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1266)*

### `_IOC_SIZEBITS`
```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1267`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1267)*

### `_IOC_DIRBITS`
```rust
const _IOC_DIRBITS: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1268`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1268)*

### `_IOC_NRMASK`
```rust
const _IOC_NRMASK: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1269`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1269)*

### `_IOC_TYPEMASK`
```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1270`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1270)*

### `_IOC_SIZEMASK`
```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1271`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1271)*

### `_IOC_DIRMASK`
```rust
const _IOC_DIRMASK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1272`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1272)*

### `_IOC_NRSHIFT`
```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1273`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1273)*

### `_IOC_TYPESHIFT`
```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1274`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1274)*

### `_IOC_SIZESHIFT`
```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1275`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1275)*

### `_IOC_DIRSHIFT`
```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1276`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1276)*

### `_IOC_NONE`
```rust
const _IOC_NONE: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1277`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1277)*

### `_IOC_WRITE`
```rust
const _IOC_WRITE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1278`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1278)*

### `_IOC_READ`
```rust
const _IOC_READ: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1279`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1279)*

### `IOC_IN`
```rust
const IOC_IN: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1280`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1280)*

### `IOC_OUT`
```rust
const IOC_OUT: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1281`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1281)*

### `IOC_INOUT`
```rust
const IOC_INOUT: u32 = 3_221_225_472u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1282`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1282)*

### `IOCSIZE_MASK`
```rust
const IOCSIZE_MASK: u32 = 1_073_676_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1283`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1283)*

### `IOCSIZE_SHIFT`
```rust
const IOCSIZE_SHIFT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1284`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1284)*

### `FSCRYPT_POLICY_FLAGS_PAD_4`
```rust
const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1285`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1285)*

### `FSCRYPT_POLICY_FLAGS_PAD_8`
```rust
const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1286`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1286)*

### `FSCRYPT_POLICY_FLAGS_PAD_16`
```rust
const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1287`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1287)*

### `FSCRYPT_POLICY_FLAGS_PAD_32`
```rust
const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1288`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1288)*

### `FSCRYPT_POLICY_FLAGS_PAD_MASK`
```rust
const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1289`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1289)*

### `FSCRYPT_POLICY_FLAG_DIRECT_KEY`
```rust
const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1290`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1290)*

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`
```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1291`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1291)*

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`
```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1292`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1292)*

### `FSCRYPT_MODE_AES_256_XTS`
```rust
const FSCRYPT_MODE_AES_256_XTS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1293`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1293)*

### `FSCRYPT_MODE_AES_256_CTS`
```rust
const FSCRYPT_MODE_AES_256_CTS: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1294`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1294)*

### `FSCRYPT_MODE_AES_128_CBC`
```rust
const FSCRYPT_MODE_AES_128_CBC: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1295`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1295)*

### `FSCRYPT_MODE_AES_128_CTS`
```rust
const FSCRYPT_MODE_AES_128_CTS: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1296`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1296)*

### `FSCRYPT_MODE_SM4_XTS`
```rust
const FSCRYPT_MODE_SM4_XTS: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1297`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1297)*

### `FSCRYPT_MODE_SM4_CTS`
```rust
const FSCRYPT_MODE_SM4_CTS: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1298`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1298)*

### `FSCRYPT_MODE_ADIANTUM`
```rust
const FSCRYPT_MODE_ADIANTUM: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1299`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1299)*

### `FSCRYPT_MODE_AES_256_HCTR2`
```rust
const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1300`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1300)*

### `FSCRYPT_POLICY_V1`
```rust
const FSCRYPT_POLICY_V1: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1301`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1301)*

### `FSCRYPT_KEY_DESCRIPTOR_SIZE`
```rust
const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1302`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1302)*

### `FSCRYPT_KEY_DESC_PREFIX`
```rust
const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1303`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1303)*

### `FSCRYPT_KEY_DESC_PREFIX_SIZE`
```rust
const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1304`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1304)*

### `FSCRYPT_MAX_KEY_SIZE`
```rust
const FSCRYPT_MAX_KEY_SIZE: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1305`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1305)*

### `FSCRYPT_POLICY_V2`
```rust
const FSCRYPT_POLICY_V2: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1306`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1306)*

### `FSCRYPT_KEY_IDENTIFIER_SIZE`
```rust
const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1307`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1307)*

### `FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`
```rust
const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1308`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1308)*

### `FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`
```rust
const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1309`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1309)*

### `FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`
```rust
const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1310`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1310)*

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`
```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1311`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1311)*

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`
```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1312`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1312)*

### `FSCRYPT_KEY_STATUS_ABSENT`
```rust
const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1313`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1313)*

### `FSCRYPT_KEY_STATUS_PRESENT`
```rust
const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1314`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1314)*

### `FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`
```rust
const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1315`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1315)*

### `FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`
```rust
const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1316`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1316)*

### `FS_KEY_DESCRIPTOR_SIZE`
```rust
const FS_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1317`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1317)*

### `FS_POLICY_FLAGS_PAD_4`
```rust
const FS_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1318`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1318)*

### `FS_POLICY_FLAGS_PAD_8`
```rust
const FS_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1319`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1319)*

### `FS_POLICY_FLAGS_PAD_16`
```rust
const FS_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1320`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1320)*

### `FS_POLICY_FLAGS_PAD_32`
```rust
const FS_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1321`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1321)*

### `FS_POLICY_FLAGS_PAD_MASK`
```rust
const FS_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1322`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1322)*

### `FS_POLICY_FLAG_DIRECT_KEY`
```rust
const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1323`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1323)*

### `FS_POLICY_FLAGS_VALID`
```rust
const FS_POLICY_FLAGS_VALID: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1324`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1324)*

### `FS_ENCRYPTION_MODE_INVALID`
```rust
const FS_ENCRYPTION_MODE_INVALID: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1325`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1325)*

### `FS_ENCRYPTION_MODE_AES_256_XTS`
```rust
const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1326`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1326)*

### `FS_ENCRYPTION_MODE_AES_256_GCM`
```rust
const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1327`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1327)*

### `FS_ENCRYPTION_MODE_AES_256_CBC`
```rust
const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1328`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1328)*

### `FS_ENCRYPTION_MODE_AES_256_CTS`
```rust
const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1329`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1329)*

### `FS_ENCRYPTION_MODE_AES_128_CBC`
```rust
const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1330`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1330)*

### `FS_ENCRYPTION_MODE_AES_128_CTS`
```rust
const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1331`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1331)*

### `FS_ENCRYPTION_MODE_ADIANTUM`
```rust
const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1332`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1332)*

### `FS_KEY_DESC_PREFIX`
```rust
const FS_KEY_DESC_PREFIX: &[u8; 9];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1333`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1333)*

### `FS_KEY_DESC_PREFIX_SIZE`
```rust
const FS_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1334`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1334)*

### `FS_MAX_KEY_SIZE`
```rust
const FS_MAX_KEY_SIZE: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1335`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1335)*

### `MS_RDONLY`
```rust
const MS_RDONLY: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1336`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1336)*

### `MS_NOSUID`
```rust
const MS_NOSUID: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1337`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1337)*

### `MS_NODEV`
```rust
const MS_NODEV: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1338`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1338)*

### `MS_NOEXEC`
```rust
const MS_NOEXEC: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1339`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1339)*

### `MS_SYNCHRONOUS`
```rust
const MS_SYNCHRONOUS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1340`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1340)*

### `MS_REMOUNT`
```rust
const MS_REMOUNT: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1341`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1341)*

### `MS_MANDLOCK`
```rust
const MS_MANDLOCK: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1342`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1342)*

### `MS_DIRSYNC`
```rust
const MS_DIRSYNC: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1343`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1343)*

### `MS_NOSYMFOLLOW`
```rust
const MS_NOSYMFOLLOW: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1344`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1344)*

### `MS_NOATIME`
```rust
const MS_NOATIME: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1345`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1345)*

### `MS_NODIRATIME`
```rust
const MS_NODIRATIME: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1346`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1346)*

### `MS_BIND`
```rust
const MS_BIND: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1347`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1347)*

### `MS_MOVE`
```rust
const MS_MOVE: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1348`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1348)*

### `MS_REC`
```rust
const MS_REC: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1349`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1349)*

### `MS_VERBOSE`
```rust
const MS_VERBOSE: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1350`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1350)*

### `MS_SILENT`
```rust
const MS_SILENT: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1351`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1351)*

### `MS_POSIXACL`
```rust
const MS_POSIXACL: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1352`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1352)*

### `MS_UNBINDABLE`
```rust
const MS_UNBINDABLE: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1353`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1353)*

### `MS_PRIVATE`
```rust
const MS_PRIVATE: u32 = 262_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1354`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1354)*

### `MS_SLAVE`
```rust
const MS_SLAVE: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1355`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1355)*

### `MS_SHARED`
```rust
const MS_SHARED: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1356`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1356)*

### `MS_RELATIME`
```rust
const MS_RELATIME: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1357`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1357)*

### `MS_KERNMOUNT`
```rust
const MS_KERNMOUNT: u32 = 4_194_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1358`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1358)*

### `MS_I_VERSION`
```rust
const MS_I_VERSION: u32 = 8_388_608u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1359`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1359)*

### `MS_STRICTATIME`
```rust
const MS_STRICTATIME: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1360`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1360)*

### `MS_LAZYTIME`
```rust
const MS_LAZYTIME: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1361`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1361)*

### `MS_SUBMOUNT`
```rust
const MS_SUBMOUNT: u32 = 67_108_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1362`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1362)*

### `MS_NOREMOTELOCK`
```rust
const MS_NOREMOTELOCK: u32 = 134_217_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1363`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1363)*

### `MS_NOSEC`
```rust
const MS_NOSEC: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1364`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1364)*

### `MS_BORN`
```rust
const MS_BORN: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1365`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1365)*

### `MS_ACTIVE`
```rust
const MS_ACTIVE: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1366`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1366)*

### `MS_NOUSER`
```rust
const MS_NOUSER: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1367`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1367)*

### `MS_RMT_MASK`
```rust
const MS_RMT_MASK: u32 = 41_943_121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1368`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1368)*

### `MS_MGC_VAL`
```rust
const MS_MGC_VAL: u32 = 3_236_757_504u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1369`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1369)*

### `MS_MGC_MSK`
```rust
const MS_MGC_MSK: u32 = 4_294_901_760u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1370`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1370)*

### `OPEN_TREE_CLONE`
```rust
const OPEN_TREE_CLONE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1371`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1371)*

### `OPEN_TREE_CLOEXEC`
```rust
const OPEN_TREE_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1372`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1372)*

### `MOVE_MOUNT_F_SYMLINKS`
```rust
const MOVE_MOUNT_F_SYMLINKS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1373`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1373)*

### `MOVE_MOUNT_F_AUTOMOUNTS`
```rust
const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1374`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1374)*

### `MOVE_MOUNT_F_EMPTY_PATH`
```rust
const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1375`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1375)*

### `MOVE_MOUNT_T_SYMLINKS`
```rust
const MOVE_MOUNT_T_SYMLINKS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1376`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1376)*

### `MOVE_MOUNT_T_AUTOMOUNTS`
```rust
const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1377`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1377)*

### `MOVE_MOUNT_T_EMPTY_PATH`
```rust
const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1378`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1378)*

### `MOVE_MOUNT_SET_GROUP`
```rust
const MOVE_MOUNT_SET_GROUP: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1379`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1379)*

### `MOVE_MOUNT_BENEATH`
```rust
const MOVE_MOUNT_BENEATH: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1380`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1380)*

### `MOVE_MOUNT__MASK`
```rust
const MOVE_MOUNT__MASK: u32 = 887u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1381`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1381)*

### `FSOPEN_CLOEXEC`
```rust
const FSOPEN_CLOEXEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1382`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1382)*

### `FSPICK_CLOEXEC`
```rust
const FSPICK_CLOEXEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1383`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1383)*

### `FSPICK_SYMLINK_NOFOLLOW`
```rust
const FSPICK_SYMLINK_NOFOLLOW: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1384`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1384)*

### `FSPICK_NO_AUTOMOUNT`
```rust
const FSPICK_NO_AUTOMOUNT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1385`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1385)*

### `FSPICK_EMPTY_PATH`
```rust
const FSPICK_EMPTY_PATH: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1386`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1386)*

### `FSMOUNT_CLOEXEC`
```rust
const FSMOUNT_CLOEXEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1387`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1387)*

### `MOUNT_ATTR_RDONLY`
```rust
const MOUNT_ATTR_RDONLY: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1388`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1388)*

### `MOUNT_ATTR_NOSUID`
```rust
const MOUNT_ATTR_NOSUID: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1389`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1389)*

### `MOUNT_ATTR_NODEV`
```rust
const MOUNT_ATTR_NODEV: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1390`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1390)*

### `MOUNT_ATTR_NOEXEC`
```rust
const MOUNT_ATTR_NOEXEC: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1391`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1391)*

### `MOUNT_ATTR__ATIME`
```rust
const MOUNT_ATTR__ATIME: u32 = 112u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1392`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1392)*

### `MOUNT_ATTR_RELATIME`
```rust
const MOUNT_ATTR_RELATIME: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1393`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1393)*

### `MOUNT_ATTR_NOATIME`
```rust
const MOUNT_ATTR_NOATIME: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1394`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1394)*

### `MOUNT_ATTR_STRICTATIME`
```rust
const MOUNT_ATTR_STRICTATIME: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1395`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1395)*

### `MOUNT_ATTR_NODIRATIME`
```rust
const MOUNT_ATTR_NODIRATIME: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1396`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1396)*

### `MOUNT_ATTR_IDMAP`
```rust
const MOUNT_ATTR_IDMAP: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1397`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1397)*

### `MOUNT_ATTR_NOSYMFOLLOW`
```rust
const MOUNT_ATTR_NOSYMFOLLOW: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1398`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1398)*

### `MOUNT_ATTR_SIZE_VER0`
```rust
const MOUNT_ATTR_SIZE_VER0: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1399`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1399)*

### `MNT_ID_REQ_SIZE_VER0`
```rust
const MNT_ID_REQ_SIZE_VER0: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1400`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1400)*

### `MNT_ID_REQ_SIZE_VER1`
```rust
const MNT_ID_REQ_SIZE_VER1: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1401`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1401)*

### `STATMOUNT_SB_BASIC`
```rust
const STATMOUNT_SB_BASIC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1402`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1402)*

### `STATMOUNT_MNT_BASIC`
```rust
const STATMOUNT_MNT_BASIC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1403`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1403)*

### `STATMOUNT_PROPAGATE_FROM`
```rust
const STATMOUNT_PROPAGATE_FROM: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1404`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1404)*

### `STATMOUNT_MNT_ROOT`
```rust
const STATMOUNT_MNT_ROOT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1405`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1405)*

### `STATMOUNT_MNT_POINT`
```rust
const STATMOUNT_MNT_POINT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1406`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1406)*

### `STATMOUNT_FS_TYPE`
```rust
const STATMOUNT_FS_TYPE: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1407`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1407)*

### `STATMOUNT_MNT_NS_ID`
```rust
const STATMOUNT_MNT_NS_ID: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1408`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1408)*

### `STATMOUNT_MNT_OPTS`
```rust
const STATMOUNT_MNT_OPTS: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1409`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1409)*

### `STATMOUNT_FS_SUBTYPE`
```rust
const STATMOUNT_FS_SUBTYPE: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1410`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1410)*

### `STATMOUNT_SB_SOURCE`
```rust
const STATMOUNT_SB_SOURCE: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1411`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1411)*

### `STATMOUNT_OPT_ARRAY`
```rust
const STATMOUNT_OPT_ARRAY: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1412`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1412)*

### `STATMOUNT_OPT_SEC_ARRAY`
```rust
const STATMOUNT_OPT_SEC_ARRAY: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1413`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1413)*

### `STATMOUNT_SUPPORTED_MASK`
```rust
const STATMOUNT_SUPPORTED_MASK: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1414`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1414)*

### `STATMOUNT_MNT_UIDMAP`
```rust
const STATMOUNT_MNT_UIDMAP: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1415`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1415)*

### `STATMOUNT_MNT_GIDMAP`
```rust
const STATMOUNT_MNT_GIDMAP: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1416`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1416)*

### `LSMT_ROOT`
```rust
const LSMT_ROOT: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1417`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1417)*

### `LISTMOUNT_REVERSE`
```rust
const LISTMOUNT_REVERSE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1418`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1418)*

### `INR_OPEN_CUR`
```rust
const INR_OPEN_CUR: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1419`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1419)*

### `INR_OPEN_MAX`
```rust
const INR_OPEN_MAX: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1420`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1420)*

### `BLOCK_SIZE_BITS`
```rust
const BLOCK_SIZE_BITS: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1421`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1421)*

### `BLOCK_SIZE`
```rust
const BLOCK_SIZE: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1422`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1422)*

### `IO_INTEGRITY_CHK_GUARD`
```rust
const IO_INTEGRITY_CHK_GUARD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1423`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1423)*

### `IO_INTEGRITY_CHK_REFTAG`
```rust
const IO_INTEGRITY_CHK_REFTAG: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1424`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1424)*

### `IO_INTEGRITY_CHK_APPTAG`
```rust
const IO_INTEGRITY_CHK_APPTAG: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1425`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1425)*

### `IO_INTEGRITY_VALID_FLAGS`
```rust
const IO_INTEGRITY_VALID_FLAGS: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1426`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1426)*

### `SEEK_SET`
```rust
const SEEK_SET: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1427`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1427)*

### `SEEK_CUR`
```rust
const SEEK_CUR: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1428`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1428)*

### `SEEK_END`
```rust
const SEEK_END: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1429`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1429)*

### `SEEK_DATA`
```rust
const SEEK_DATA: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1430`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1430)*

### `SEEK_HOLE`
```rust
const SEEK_HOLE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1431`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1431)*

### `SEEK_MAX`
```rust
const SEEK_MAX: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1432`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1432)*

### `RENAME_NOREPLACE`
```rust
const RENAME_NOREPLACE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1433`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1433)*

### `RENAME_EXCHANGE`
```rust
const RENAME_EXCHANGE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1434`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1434)*

### `RENAME_WHITEOUT`
```rust
const RENAME_WHITEOUT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1435`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1435)*

### `FILE_DEDUPE_RANGE_SAME`
```rust
const FILE_DEDUPE_RANGE_SAME: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1436`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1436)*

### `FILE_DEDUPE_RANGE_DIFFERS`
```rust
const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1437`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1437)*

### `NR_FILE`
```rust
const NR_FILE: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1438`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1438)*

### `FS_XFLAG_REALTIME`
```rust
const FS_XFLAG_REALTIME: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1439`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1439)*

### `FS_XFLAG_PREALLOC`
```rust
const FS_XFLAG_PREALLOC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1440`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1440)*

### `FS_XFLAG_IMMUTABLE`
```rust
const FS_XFLAG_IMMUTABLE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1441`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1441)*

### `FS_XFLAG_APPEND`
```rust
const FS_XFLAG_APPEND: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1442`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1442)*

### `FS_XFLAG_SYNC`
```rust
const FS_XFLAG_SYNC: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1443`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1443)*

### `FS_XFLAG_NOATIME`
```rust
const FS_XFLAG_NOATIME: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1444`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1444)*

### `FS_XFLAG_NODUMP`
```rust
const FS_XFLAG_NODUMP: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1445`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1445)*

### `FS_XFLAG_RTINHERIT`
```rust
const FS_XFLAG_RTINHERIT: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1446`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1446)*

### `FS_XFLAG_PROJINHERIT`
```rust
const FS_XFLAG_PROJINHERIT: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1447`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1447)*

### `FS_XFLAG_NOSYMLINKS`
```rust
const FS_XFLAG_NOSYMLINKS: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1448`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1448)*

### `FS_XFLAG_EXTSIZE`
```rust
const FS_XFLAG_EXTSIZE: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1449`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1449)*

### `FS_XFLAG_EXTSZINHERIT`
```rust
const FS_XFLAG_EXTSZINHERIT: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1450`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1450)*

### `FS_XFLAG_NODEFRAG`
```rust
const FS_XFLAG_NODEFRAG: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1451`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1451)*

### `FS_XFLAG_FILESTREAM`
```rust
const FS_XFLAG_FILESTREAM: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1452`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1452)*

### `FS_XFLAG_DAX`
```rust
const FS_XFLAG_DAX: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1453`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1453)*

### `FS_XFLAG_COWEXTSIZE`
```rust
const FS_XFLAG_COWEXTSIZE: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1454`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1454)*

### `FS_XFLAG_HASATTR`
```rust
const FS_XFLAG_HASATTR: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1455`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1455)*

### `BMAP_IOCTL`
```rust
const BMAP_IOCTL: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1456`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1456)*

### `FSLABEL_MAX`
```rust
const FSLABEL_MAX: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1457`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1457)*

### `FS_SECRM_FL`
```rust
const FS_SECRM_FL: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1458`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1458)*

### `FS_UNRM_FL`
```rust
const FS_UNRM_FL: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1459`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1459)*

### `FS_COMPR_FL`
```rust
const FS_COMPR_FL: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1460`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1460)*

### `FS_SYNC_FL`
```rust
const FS_SYNC_FL: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1461`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1461)*

### `FS_IMMUTABLE_FL`
```rust
const FS_IMMUTABLE_FL: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1462`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1462)*

### `FS_APPEND_FL`
```rust
const FS_APPEND_FL: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1463`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1463)*

### `FS_NODUMP_FL`
```rust
const FS_NODUMP_FL: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1464`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1464)*

### `FS_NOATIME_FL`
```rust
const FS_NOATIME_FL: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1465`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1465)*

### `FS_DIRTY_FL`
```rust
const FS_DIRTY_FL: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1466`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1466)*

### `FS_COMPRBLK_FL`
```rust
const FS_COMPRBLK_FL: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1467`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1467)*

### `FS_NOCOMP_FL`
```rust
const FS_NOCOMP_FL: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1468`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1468)*

### `FS_ENCRYPT_FL`
```rust
const FS_ENCRYPT_FL: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1469`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1469)*

### `FS_BTREE_FL`
```rust
const FS_BTREE_FL: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1470`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1470)*

### `FS_INDEX_FL`
```rust
const FS_INDEX_FL: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1471`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1471)*

### `FS_IMAGIC_FL`
```rust
const FS_IMAGIC_FL: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1472`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1472)*

### `FS_JOURNAL_DATA_FL`
```rust
const FS_JOURNAL_DATA_FL: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1473`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1473)*

### `FS_NOTAIL_FL`
```rust
const FS_NOTAIL_FL: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1474`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1474)*

### `FS_DIRSYNC_FL`
```rust
const FS_DIRSYNC_FL: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1475`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1475)*

### `FS_TOPDIR_FL`
```rust
const FS_TOPDIR_FL: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1476`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1476)*

### `FS_HUGE_FILE_FL`
```rust
const FS_HUGE_FILE_FL: u32 = 262_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1477`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1477)*

### `FS_EXTENT_FL`
```rust
const FS_EXTENT_FL: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1478`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1478)*

### `FS_VERITY_FL`
```rust
const FS_VERITY_FL: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1479`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1479)*

### `FS_EA_INODE_FL`
```rust
const FS_EA_INODE_FL: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1480`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1480)*

### `FS_EOFBLOCKS_FL`
```rust
const FS_EOFBLOCKS_FL: u32 = 4_194_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1481`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1481)*

### `FS_NOCOW_FL`
```rust
const FS_NOCOW_FL: u32 = 8_388_608u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1482`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1482)*

### `FS_DAX_FL`
```rust
const FS_DAX_FL: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1483`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1483)*

### `FS_INLINE_DATA_FL`
```rust
const FS_INLINE_DATA_FL: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1484`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1484)*

### `FS_PROJINHERIT_FL`
```rust
const FS_PROJINHERIT_FL: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1485`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1485)*

### `FS_CASEFOLD_FL`
```rust
const FS_CASEFOLD_FL: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1486`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1486)*

### `FS_RESERVED_FL`
```rust
const FS_RESERVED_FL: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1487`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1487)*

### `FS_FL_USER_VISIBLE`
```rust
const FS_FL_USER_VISIBLE: u32 = 253_951u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1488`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1488)*

### `FS_FL_USER_MODIFIABLE`
```rust
const FS_FL_USER_MODIFIABLE: u32 = 229_631u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1489`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1489)*

### `SYNC_FILE_RANGE_WAIT_BEFORE`
```rust
const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1490`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1490)*

### `SYNC_FILE_RANGE_WRITE`
```rust
const SYNC_FILE_RANGE_WRITE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1491`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1491)*

### `SYNC_FILE_RANGE_WAIT_AFTER`
```rust
const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1492`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1492)*

### `SYNC_FILE_RANGE_WRITE_AND_WAIT`
```rust
const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1493`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1493)*

### `PROCFS_IOCTL_MAGIC`
```rust
const PROCFS_IOCTL_MAGIC: u8 = 102u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1494`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1494)*

### `PAGE_IS_WPALLOWED`
```rust
const PAGE_IS_WPALLOWED: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1495`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1495)*

### `PAGE_IS_WRITTEN`
```rust
const PAGE_IS_WRITTEN: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1496`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1496)*

### `PAGE_IS_FILE`
```rust
const PAGE_IS_FILE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1497`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1497)*

### `PAGE_IS_PRESENT`
```rust
const PAGE_IS_PRESENT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1498`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1498)*

### `PAGE_IS_SWAPPED`
```rust
const PAGE_IS_SWAPPED: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1499`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1499)*

### `PAGE_IS_PFNZERO`
```rust
const PAGE_IS_PFNZERO: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1500`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1500)*

### `PAGE_IS_HUGE`
```rust
const PAGE_IS_HUGE: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1501`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1501)*

### `PAGE_IS_SOFT_DIRTY`
```rust
const PAGE_IS_SOFT_DIRTY: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1502`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1502)*

### `PAGE_IS_GUARD`
```rust
const PAGE_IS_GUARD: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1503`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1503)*

### `PM_SCAN_WP_MATCHING`
```rust
const PM_SCAN_WP_MATCHING: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1504`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1504)*

### `PM_SCAN_CHECK_WPASYNC`
```rust
const PM_SCAN_CHECK_WPASYNC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1505`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1505)*

### `FUTEX_WAIT`
```rust
const FUTEX_WAIT: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1506`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1506)*

### `FUTEX_WAKE`
```rust
const FUTEX_WAKE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1507`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1507)*

### `FUTEX_FD`
```rust
const FUTEX_FD: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1508`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1508)*

### `FUTEX_REQUEUE`
```rust
const FUTEX_REQUEUE: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1509`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1509)*

### `FUTEX_CMP_REQUEUE`
```rust
const FUTEX_CMP_REQUEUE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1510`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1510)*

### `FUTEX_WAKE_OP`
```rust
const FUTEX_WAKE_OP: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1511`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1511)*

### `FUTEX_LOCK_PI`
```rust
const FUTEX_LOCK_PI: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1512`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1512)*

### `FUTEX_UNLOCK_PI`
```rust
const FUTEX_UNLOCK_PI: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1513`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1513)*

### `FUTEX_TRYLOCK_PI`
```rust
const FUTEX_TRYLOCK_PI: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1514`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1514)*

### `FUTEX_WAIT_BITSET`
```rust
const FUTEX_WAIT_BITSET: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1515`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1515)*

### `FUTEX_WAKE_BITSET`
```rust
const FUTEX_WAKE_BITSET: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1516`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1516)*

### `FUTEX_WAIT_REQUEUE_PI`
```rust
const FUTEX_WAIT_REQUEUE_PI: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1517`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1517)*

### `FUTEX_CMP_REQUEUE_PI`
```rust
const FUTEX_CMP_REQUEUE_PI: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1518`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1518)*

### `FUTEX_LOCK_PI2`
```rust
const FUTEX_LOCK_PI2: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1519`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1519)*

### `FUTEX_PRIVATE_FLAG`
```rust
const FUTEX_PRIVATE_FLAG: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1520`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1520)*

### `FUTEX_CLOCK_REALTIME`
```rust
const FUTEX_CLOCK_REALTIME: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1521`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1521)*

### `FUTEX_CMD_MASK`
```rust
const FUTEX_CMD_MASK: i32 = -385i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1522`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1522)*

### `FUTEX_WAIT_PRIVATE`
```rust
const FUTEX_WAIT_PRIVATE: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1523`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1523)*

### `FUTEX_WAKE_PRIVATE`
```rust
const FUTEX_WAKE_PRIVATE: u32 = 129u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1524`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1524)*

### `FUTEX_REQUEUE_PRIVATE`
```rust
const FUTEX_REQUEUE_PRIVATE: u32 = 131u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1525`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1525)*

### `FUTEX_CMP_REQUEUE_PRIVATE`
```rust
const FUTEX_CMP_REQUEUE_PRIVATE: u32 = 132u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1526`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1526)*

### `FUTEX_WAKE_OP_PRIVATE`
```rust
const FUTEX_WAKE_OP_PRIVATE: u32 = 133u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1527`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1527)*

### `FUTEX_LOCK_PI_PRIVATE`
```rust
const FUTEX_LOCK_PI_PRIVATE: u32 = 134u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1528`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1528)*

### `FUTEX_LOCK_PI2_PRIVATE`
```rust
const FUTEX_LOCK_PI2_PRIVATE: u32 = 141u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1529`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1529)*

### `FUTEX_UNLOCK_PI_PRIVATE`
```rust
const FUTEX_UNLOCK_PI_PRIVATE: u32 = 135u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1530`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1530)*

### `FUTEX_TRYLOCK_PI_PRIVATE`
```rust
const FUTEX_TRYLOCK_PI_PRIVATE: u32 = 136u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1531`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1531)*

### `FUTEX_WAIT_BITSET_PRIVATE`
```rust
const FUTEX_WAIT_BITSET_PRIVATE: u32 = 137u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1532`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1532)*

### `FUTEX_WAKE_BITSET_PRIVATE`
```rust
const FUTEX_WAKE_BITSET_PRIVATE: u32 = 138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1533`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1533)*

### `FUTEX_WAIT_REQUEUE_PI_PRIVATE`
```rust
const FUTEX_WAIT_REQUEUE_PI_PRIVATE: u32 = 139u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1534`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1534)*

### `FUTEX_CMP_REQUEUE_PI_PRIVATE`
```rust
const FUTEX_CMP_REQUEUE_PI_PRIVATE: u32 = 140u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1535`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1535)*

### `FUTEX2_SIZE_U8`
```rust
const FUTEX2_SIZE_U8: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1536`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1536)*

### `FUTEX2_SIZE_U16`
```rust
const FUTEX2_SIZE_U16: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1537`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1537)*

### `FUTEX2_SIZE_U32`
```rust
const FUTEX2_SIZE_U32: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1538`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1538)*

### `FUTEX2_SIZE_U64`
```rust
const FUTEX2_SIZE_U64: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1539`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1539)*

### `FUTEX2_NUMA`
```rust
const FUTEX2_NUMA: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1540`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1540)*

### `FUTEX2_MPOL`
```rust
const FUTEX2_MPOL: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1541`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1541)*

### `FUTEX2_PRIVATE`
```rust
const FUTEX2_PRIVATE: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1542`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1542)*

### `FUTEX2_SIZE_MASK`
```rust
const FUTEX2_SIZE_MASK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1543`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1543)*

### `FUTEX_32`
```rust
const FUTEX_32: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1544`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1544)*

### `FUTEX_NO_NODE`
```rust
const FUTEX_NO_NODE: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1545`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1545)*

### `FUTEX_WAITV_MAX`
```rust
const FUTEX_WAITV_MAX: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1546`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1546)*

### `FUTEX_WAITERS`
```rust
const FUTEX_WAITERS: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1547`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1547)*

### `FUTEX_OWNER_DIED`
```rust
const FUTEX_OWNER_DIED: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1548`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1548)*

### `FUTEX_TID_MASK`
```rust
const FUTEX_TID_MASK: u32 = 1_073_741_823u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1549`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1549)*

### `ROBUST_LIST_LIMIT`
```rust
const ROBUST_LIST_LIMIT: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1550`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1550)*

### `FUTEX_BITSET_MATCH_ANY`
```rust
const FUTEX_BITSET_MATCH_ANY: u32 = 4_294_967_295u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1551`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1551)*

### `FUTEX_OP_SET`
```rust
const FUTEX_OP_SET: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1552`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1552)*

### `FUTEX_OP_ADD`
```rust
const FUTEX_OP_ADD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1553`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1553)*

### `FUTEX_OP_OR`
```rust
const FUTEX_OP_OR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1554`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1554)*

### `FUTEX_OP_ANDN`
```rust
const FUTEX_OP_ANDN: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1555`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1555)*

### `FUTEX_OP_XOR`
```rust
const FUTEX_OP_XOR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1556`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1556)*

### `FUTEX_OP_OPARG_SHIFT`
```rust
const FUTEX_OP_OPARG_SHIFT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1557`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1557)*

### `FUTEX_OP_CMP_EQ`
```rust
const FUTEX_OP_CMP_EQ: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1558`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1558)*

### `FUTEX_OP_CMP_NE`
```rust
const FUTEX_OP_CMP_NE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1559`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1559)*

### `FUTEX_OP_CMP_LT`
```rust
const FUTEX_OP_CMP_LT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1560`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1560)*

### `FUTEX_OP_CMP_LE`
```rust
const FUTEX_OP_CMP_LE: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1561`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1561)*

### `FUTEX_OP_CMP_GT`
```rust
const FUTEX_OP_CMP_GT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1562`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1562)*

### `FUTEX_OP_CMP_GE`
```rust
const FUTEX_OP_CMP_GE: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1563`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1563)*

### `IN_ACCESS`
```rust
const IN_ACCESS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1564`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1564)*

### `IN_MODIFY`
```rust
const IN_MODIFY: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1565`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1565)*

### `IN_ATTRIB`
```rust
const IN_ATTRIB: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1566`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1566)*

### `IN_CLOSE_WRITE`
```rust
const IN_CLOSE_WRITE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1567`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1567)*

### `IN_CLOSE_NOWRITE`
```rust
const IN_CLOSE_NOWRITE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1568`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1568)*

### `IN_OPEN`
```rust
const IN_OPEN: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1569`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1569)*

### `IN_MOVED_FROM`
```rust
const IN_MOVED_FROM: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1570`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1570)*

### `IN_MOVED_TO`
```rust
const IN_MOVED_TO: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1571`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1571)*

### `IN_CREATE`
```rust
const IN_CREATE: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1572`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1572)*

### `IN_DELETE`
```rust
const IN_DELETE: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1573`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1573)*

### `IN_DELETE_SELF`
```rust
const IN_DELETE_SELF: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1574`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1574)*

### `IN_MOVE_SELF`
```rust
const IN_MOVE_SELF: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1575`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1575)*

### `IN_UNMOUNT`
```rust
const IN_UNMOUNT: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1576`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1576)*

### `IN_Q_OVERFLOW`
```rust
const IN_Q_OVERFLOW: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1577`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1577)*

### `IN_IGNORED`
```rust
const IN_IGNORED: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1578`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1578)*

### `IN_CLOSE`
```rust
const IN_CLOSE: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1579`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1579)*

### `IN_MOVE`
```rust
const IN_MOVE: u32 = 192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1580`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1580)*

### `IN_ONLYDIR`
```rust
const IN_ONLYDIR: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1581`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1581)*

### `IN_DONT_FOLLOW`
```rust
const IN_DONT_FOLLOW: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1582`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1582)*

### `IN_EXCL_UNLINK`
```rust
const IN_EXCL_UNLINK: u32 = 67_108_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1583`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1583)*

### `IN_MASK_CREATE`
```rust
const IN_MASK_CREATE: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1584`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1584)*

### `IN_MASK_ADD`
```rust
const IN_MASK_ADD: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1585`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1585)*

### `IN_ISDIR`
```rust
const IN_ISDIR: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1586`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1586)*

### `IN_ONESHOT`
```rust
const IN_ONESHOT: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1587`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1587)*

### `IN_ALL_EVENTS`
```rust
const IN_ALL_EVENTS: u32 = 4_095u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1588`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1588)*

### `IN_CLOEXEC`
```rust
const IN_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1589`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1589)*

### `IN_NONBLOCK`
```rust
const IN_NONBLOCK: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1590`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1590)*

### `ADFS_SUPER_MAGIC`
```rust
const ADFS_SUPER_MAGIC: u32 = 44_533u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1591`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1591)*

### `AFFS_SUPER_MAGIC`
```rust
const AFFS_SUPER_MAGIC: u32 = 44_543u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1592`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1592)*

### `AFS_SUPER_MAGIC`
```rust
const AFS_SUPER_MAGIC: u32 = 1_397_113_167u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1593`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1593)*

### `AUTOFS_SUPER_MAGIC`
```rust
const AUTOFS_SUPER_MAGIC: u32 = 391u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1594`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1594)*

### `CEPH_SUPER_MAGIC`
```rust
const CEPH_SUPER_MAGIC: u32 = 12_805_120u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1595`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1595)*

### `CODA_SUPER_MAGIC`
```rust
const CODA_SUPER_MAGIC: u32 = 1_937_076_805u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1596`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1596)*

### `CRAMFS_MAGIC`
```rust
const CRAMFS_MAGIC: u32 = 684_539_205u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1597`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1597)*

### `CRAMFS_MAGIC_WEND`
```rust
const CRAMFS_MAGIC_WEND: u32 = 1_161_678_120u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1598`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1598)*

### `DEBUGFS_MAGIC`
```rust
const DEBUGFS_MAGIC: u32 = 1_684_170_528u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1599`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1599)*

### `SECURITYFS_MAGIC`
```rust
const SECURITYFS_MAGIC: u32 = 1_935_894_131u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1600`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1600)*

### `SELINUX_MAGIC`
```rust
const SELINUX_MAGIC: u32 = 4_185_718_668u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1601`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1601)*

### `SMACK_MAGIC`
```rust
const SMACK_MAGIC: u32 = 1_128_357_203u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1602`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1602)*

### `RAMFS_MAGIC`
```rust
const RAMFS_MAGIC: u32 = 2_240_043_254u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1603`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1603)*

### `TMPFS_MAGIC`
```rust
const TMPFS_MAGIC: u32 = 16_914_836u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1604`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1604)*

### `HUGETLBFS_MAGIC`
```rust
const HUGETLBFS_MAGIC: u32 = 2_508_478_710u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1605`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1605)*

### `SQUASHFS_MAGIC`
```rust
const SQUASHFS_MAGIC: u32 = 1_936_814_952u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1606`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1606)*

### `ECRYPTFS_SUPER_MAGIC`
```rust
const ECRYPTFS_SUPER_MAGIC: u32 = 61_791u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1607`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1607)*

### `EFS_SUPER_MAGIC`
```rust
const EFS_SUPER_MAGIC: u32 = 4_278_867u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1608`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1608)*

### `EROFS_SUPER_MAGIC_V1`
```rust
const EROFS_SUPER_MAGIC_V1: u32 = 3_774_210_530u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1609`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1609)*

### `EXT2_SUPER_MAGIC`
```rust
const EXT2_SUPER_MAGIC: u32 = 61_267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1610`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1610)*

### `EXT3_SUPER_MAGIC`
```rust
const EXT3_SUPER_MAGIC: u32 = 61_267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1611`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1611)*

### `XENFS_SUPER_MAGIC`
```rust
const XENFS_SUPER_MAGIC: u32 = 2_881_100_148u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1612`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1612)*

### `EXT4_SUPER_MAGIC`
```rust
const EXT4_SUPER_MAGIC: u32 = 61_267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1613`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1613)*

### `BTRFS_SUPER_MAGIC`
```rust
const BTRFS_SUPER_MAGIC: u32 = 2_435_016_766u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1614`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1614)*

### `NILFS_SUPER_MAGIC`
```rust
const NILFS_SUPER_MAGIC: u32 = 13_364u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1615`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1615)*

### `F2FS_SUPER_MAGIC`
```rust
const F2FS_SUPER_MAGIC: u32 = 4_076_150_800u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1616`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1616)*

### `HPFS_SUPER_MAGIC`
```rust
const HPFS_SUPER_MAGIC: u32 = 4_187_351_113u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1617`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1617)*

### `ISOFS_SUPER_MAGIC`
```rust
const ISOFS_SUPER_MAGIC: u32 = 38_496u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1618`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1618)*

### `JFFS2_SUPER_MAGIC`
```rust
const JFFS2_SUPER_MAGIC: u32 = 29_366u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1619`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1619)*

### `XFS_SUPER_MAGIC`
```rust
const XFS_SUPER_MAGIC: u32 = 1_481_003_842u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1620`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1620)*

### `PSTOREFS_MAGIC`
```rust
const PSTOREFS_MAGIC: u32 = 1_634_035_564u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1621`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1621)*

### `EFIVARFS_MAGIC`
```rust
const EFIVARFS_MAGIC: u32 = 3_730_735_588u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1622`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1622)*

### `HOSTFS_SUPER_MAGIC`
```rust
const HOSTFS_SUPER_MAGIC: u32 = 12_648_430u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1623`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1623)*

### `OVERLAYFS_SUPER_MAGIC`
```rust
const OVERLAYFS_SUPER_MAGIC: u32 = 2_035_054_128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1624`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1624)*

### `FUSE_SUPER_MAGIC`
```rust
const FUSE_SUPER_MAGIC: u32 = 1_702_057_286u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1625`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1625)*

### `BCACHEFS_SUPER_MAGIC`
```rust
const BCACHEFS_SUPER_MAGIC: u32 = 3_393_526_350u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1626`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1626)*

### `MINIX_SUPER_MAGIC`
```rust
const MINIX_SUPER_MAGIC: u32 = 4_991u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1627`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1627)*

### `MINIX_SUPER_MAGIC2`
```rust
const MINIX_SUPER_MAGIC2: u32 = 5_007u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1628`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1628)*

### `MINIX2_SUPER_MAGIC`
```rust
const MINIX2_SUPER_MAGIC: u32 = 9_320u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1629`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1629)*

### `MINIX2_SUPER_MAGIC2`
```rust
const MINIX2_SUPER_MAGIC2: u32 = 9_336u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1630`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1630)*

### `MINIX3_SUPER_MAGIC`
```rust
const MINIX3_SUPER_MAGIC: u32 = 19_802u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1631`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1631)*

### `MSDOS_SUPER_MAGIC`
```rust
const MSDOS_SUPER_MAGIC: u32 = 19_780u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1632`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1632)*

### `EXFAT_SUPER_MAGIC`
```rust
const EXFAT_SUPER_MAGIC: u32 = 538_032_816u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1633`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1633)*

### `NCP_SUPER_MAGIC`
```rust
const NCP_SUPER_MAGIC: u32 = 22_092u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1634`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1634)*

### `NFS_SUPER_MAGIC`
```rust
const NFS_SUPER_MAGIC: u32 = 26_985u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1635`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1635)*

### `OCFS2_SUPER_MAGIC`
```rust
const OCFS2_SUPER_MAGIC: u32 = 1_952_539_503u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1636`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1636)*

### `OPENPROM_SUPER_MAGIC`
```rust
const OPENPROM_SUPER_MAGIC: u32 = 40_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1637`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1637)*

### `QNX4_SUPER_MAGIC`
```rust
const QNX4_SUPER_MAGIC: u32 = 47u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1638`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1638)*

### `QNX6_SUPER_MAGIC`
```rust
const QNX6_SUPER_MAGIC: u32 = 1_746_473_250u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1639`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1639)*

### `AFS_FS_MAGIC`
```rust
const AFS_FS_MAGIC: u32 = 1_799_439_955u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1640`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1640)*

### `REISERFS_SUPER_MAGIC`
```rust
const REISERFS_SUPER_MAGIC: u32 = 1_382_369_651u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1641`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1641)*

### `REISERFS_SUPER_MAGIC_STRING`
```rust
const REISERFS_SUPER_MAGIC_STRING: &[u8; 9];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1642`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1642)*

### `REISER2FS_SUPER_MAGIC_STRING`
```rust
const REISER2FS_SUPER_MAGIC_STRING: &[u8; 10];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1643`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1643)*

### `REISER2FS_JR_SUPER_MAGIC_STRING`
```rust
const REISER2FS_JR_SUPER_MAGIC_STRING: &[u8; 10];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1644`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1644)*

### `SMB_SUPER_MAGIC`
```rust
const SMB_SUPER_MAGIC: u32 = 20_859u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1645`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1645)*

### `CIFS_SUPER_MAGIC`
```rust
const CIFS_SUPER_MAGIC: u32 = 4_283_649_346u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1646`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1646)*

### `SMB2_SUPER_MAGIC`
```rust
const SMB2_SUPER_MAGIC: u32 = 4_266_872_130u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1647`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1647)*

### `CGROUP_SUPER_MAGIC`
```rust
const CGROUP_SUPER_MAGIC: u32 = 2_613_483u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1648`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1648)*

### `CGROUP2_SUPER_MAGIC`
```rust
const CGROUP2_SUPER_MAGIC: u32 = 1_667_723_888u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1649`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1649)*

### `RDTGROUP_SUPER_MAGIC`
```rust
const RDTGROUP_SUPER_MAGIC: u32 = 124_082_209u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1650`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1650)*

### `STACK_END_MAGIC`
```rust
const STACK_END_MAGIC: u32 = 1_470_918_301u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1651`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1651)*

### `TRACEFS_MAGIC`
```rust
const TRACEFS_MAGIC: u32 = 1_953_653_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1652`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1652)*

### `V9FS_MAGIC`
```rust
const V9FS_MAGIC: u32 = 16_914_839u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1653`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1653)*

### `BDEVFS_MAGIC`
```rust
const BDEVFS_MAGIC: u32 = 1_650_746_742u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1654`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1654)*

### `DAXFS_MAGIC`
```rust
const DAXFS_MAGIC: u32 = 1_684_300_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1655`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1655)*

### `BINFMTFS_MAGIC`
```rust
const BINFMTFS_MAGIC: u32 = 1_112_100_429u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1656`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1656)*

### `DEVPTS_SUPER_MAGIC`
```rust
const DEVPTS_SUPER_MAGIC: u32 = 7_377u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1657`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1657)*

### `BINDERFS_SUPER_MAGIC`
```rust
const BINDERFS_SUPER_MAGIC: u32 = 1_819_242_352u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1658`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1658)*

### `FUTEXFS_SUPER_MAGIC`
```rust
const FUTEXFS_SUPER_MAGIC: u32 = 195_894_762u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1659`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1659)*

### `PIPEFS_MAGIC`
```rust
const PIPEFS_MAGIC: u32 = 1_346_981_957u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1660`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1660)*

### `PROC_SUPER_MAGIC`
```rust
const PROC_SUPER_MAGIC: u32 = 40_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1661`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1661)*

### `SOCKFS_MAGIC`
```rust
const SOCKFS_MAGIC: u32 = 1_397_703_499u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1662`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1662)*

### `SYSFS_MAGIC`
```rust
const SYSFS_MAGIC: u32 = 1_650_812_274u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1663`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1663)*

### `USBDEVICE_SUPER_MAGIC`
```rust
const USBDEVICE_SUPER_MAGIC: u32 = 40_866u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1664`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1664)*

### `MTD_INODE_FS_MAGIC`
```rust
const MTD_INODE_FS_MAGIC: u32 = 288_389_204u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1665`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1665)*

### `ANON_INODE_FS_MAGIC`
```rust
const ANON_INODE_FS_MAGIC: u32 = 151_263_540u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1666`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1666)*

### `BTRFS_TEST_MAGIC`
```rust
const BTRFS_TEST_MAGIC: u32 = 1_936_880_249u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1667`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1667)*

### `NSFS_MAGIC`
```rust
const NSFS_MAGIC: u32 = 1_853_056_627u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1668`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1668)*

### `BPF_FS_MAGIC`
```rust
const BPF_FS_MAGIC: u32 = 3_405_662_737u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1669`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1669)*

### `AAFS_MAGIC`
```rust
const AAFS_MAGIC: u32 = 1_513_908_720u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1670`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1670)*

### `ZONEFS_MAGIC`
```rust
const ZONEFS_MAGIC: u32 = 1_515_144_787u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1671`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1671)*

### `UDF_SUPER_MAGIC`
```rust
const UDF_SUPER_MAGIC: u32 = 352_400_198u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1672`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1672)*

### `DMA_BUF_MAGIC`
```rust
const DMA_BUF_MAGIC: u32 = 1_145_913_666u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1673`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1673)*

### `DEVMEM_MAGIC`
```rust
const DEVMEM_MAGIC: u32 = 1_162_691_661u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1674`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1674)*

### `SECRETMEM_MAGIC`
```rust
const SECRETMEM_MAGIC: u32 = 1_397_048_141u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1675`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1675)*

### `PID_FS_MAGIC`
```rust
const PID_FS_MAGIC: u32 = 1_346_978_886u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1676`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1676)*

### `MAP_32BIT`
```rust
const MAP_32BIT: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1677`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1677)*

### `MAP_ABOVE4G`
```rust
const MAP_ABOVE4G: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1678`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1678)*

### `PROT_READ`
```rust
const PROT_READ: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1679`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1679)*

### `PROT_WRITE`
```rust
const PROT_WRITE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1680`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1680)*

### `PROT_EXEC`
```rust
const PROT_EXEC: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1681`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1681)*

### `PROT_SEM`
```rust
const PROT_SEM: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1682`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1682)*

### `PROT_NONE`
```rust
const PROT_NONE: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1683`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1683)*

### `PROT_GROWSDOWN`
```rust
const PROT_GROWSDOWN: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1684`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1684)*

### `PROT_GROWSUP`
```rust
const PROT_GROWSUP: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1685`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1685)*

### `MAP_TYPE`
```rust
const MAP_TYPE: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1686`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1686)*

### `MAP_FIXED`
```rust
const MAP_FIXED: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1687`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1687)*

### `MAP_ANONYMOUS`
```rust
const MAP_ANONYMOUS: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1688`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1688)*

### `MAP_POPULATE`
```rust
const MAP_POPULATE: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1689`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1689)*

### `MAP_NONBLOCK`
```rust
const MAP_NONBLOCK: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1690`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1690)*

### `MAP_STACK`
```rust
const MAP_STACK: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1691`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1691)*

### `MAP_HUGETLB`
```rust
const MAP_HUGETLB: u32 = 262_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1692`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1692)*

### `MAP_SYNC`
```rust
const MAP_SYNC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1693`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1693)*

### `MAP_FIXED_NOREPLACE`
```rust
const MAP_FIXED_NOREPLACE: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1694`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1694)*

### `MAP_UNINITIALIZED`
```rust
const MAP_UNINITIALIZED: u32 = 67_108_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1695`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1695)*

### `MLOCK_ONFAULT`
```rust
const MLOCK_ONFAULT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1696`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1696)*

### `MS_ASYNC`
```rust
const MS_ASYNC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1697`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1697)*

### `MS_INVALIDATE`
```rust
const MS_INVALIDATE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1698`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1698)*

### `MS_SYNC`
```rust
const MS_SYNC: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1699`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1699)*

### `MADV_NORMAL`
```rust
const MADV_NORMAL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1700`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1700)*

### `MADV_RANDOM`
```rust
const MADV_RANDOM: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1701`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1701)*

### `MADV_SEQUENTIAL`
```rust
const MADV_SEQUENTIAL: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1702`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1702)*

### `MADV_WILLNEED`
```rust
const MADV_WILLNEED: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1703`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1703)*

### `MADV_DONTNEED`
```rust
const MADV_DONTNEED: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1704`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1704)*

### `MADV_FREE`
```rust
const MADV_FREE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1705`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1705)*

### `MADV_REMOVE`
```rust
const MADV_REMOVE: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1706`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1706)*

### `MADV_DONTFORK`
```rust
const MADV_DONTFORK: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1707`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1707)*

### `MADV_DOFORK`
```rust
const MADV_DOFORK: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1708`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1708)*

### `MADV_HWPOISON`
```rust
const MADV_HWPOISON: u32 = 100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1709`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1709)*

### `MADV_SOFT_OFFLINE`
```rust
const MADV_SOFT_OFFLINE: u32 = 101u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1710`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1710)*

### `MADV_MERGEABLE`
```rust
const MADV_MERGEABLE: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1711`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1711)*

### `MADV_UNMERGEABLE`
```rust
const MADV_UNMERGEABLE: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1712`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1712)*

### `MADV_HUGEPAGE`
```rust
const MADV_HUGEPAGE: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1713`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1713)*

### `MADV_NOHUGEPAGE`
```rust
const MADV_NOHUGEPAGE: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1714`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1714)*

### `MADV_DONTDUMP`
```rust
const MADV_DONTDUMP: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1715`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1715)*

### `MADV_DODUMP`
```rust
const MADV_DODUMP: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1716`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1716)*

### `MADV_WIPEONFORK`
```rust
const MADV_WIPEONFORK: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1717`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1717)*

### `MADV_KEEPONFORK`
```rust
const MADV_KEEPONFORK: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1718`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1718)*

### `MADV_COLD`
```rust
const MADV_COLD: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1719`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1719)*

### `MADV_PAGEOUT`
```rust
const MADV_PAGEOUT: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1720`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1720)*

### `MADV_POPULATE_READ`
```rust
const MADV_POPULATE_READ: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1721`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1721)*

### `MADV_POPULATE_WRITE`
```rust
const MADV_POPULATE_WRITE: u32 = 23u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1722`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1722)*

### `MADV_DONTNEED_LOCKED`
```rust
const MADV_DONTNEED_LOCKED: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1723`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1723)*

### `MADV_COLLAPSE`
```rust
const MADV_COLLAPSE: u32 = 25u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1724`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1724)*

### `MADV_GUARD_INSTALL`
```rust
const MADV_GUARD_INSTALL: u32 = 102u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1725`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1725)*

### `MADV_GUARD_REMOVE`
```rust
const MADV_GUARD_REMOVE: u32 = 103u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1726`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1726)*

### `MAP_FILE`
```rust
const MAP_FILE: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1727`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1727)*

### `PKEY_UNRESTRICTED`
```rust
const PKEY_UNRESTRICTED: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1728`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1728)*

### `PKEY_DISABLE_ACCESS`
```rust
const PKEY_DISABLE_ACCESS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1729`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1729)*

### `PKEY_DISABLE_WRITE`
```rust
const PKEY_DISABLE_WRITE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1730`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1730)*

### `PKEY_ACCESS_MASK`
```rust
const PKEY_ACCESS_MASK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1731`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1731)*

### `MAP_GROWSDOWN`
```rust
const MAP_GROWSDOWN: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1732`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1732)*

### `MAP_DENYWRITE`
```rust
const MAP_DENYWRITE: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1733`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1733)*

### `MAP_EXECUTABLE`
```rust
const MAP_EXECUTABLE: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1734`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1734)*

### `MAP_LOCKED`
```rust
const MAP_LOCKED: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1735`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1735)*

### `MAP_NORESERVE`
```rust
const MAP_NORESERVE: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1736`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1736)*

### `MCL_CURRENT`
```rust
const MCL_CURRENT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1737`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1737)*

### `MCL_FUTURE`
```rust
const MCL_FUTURE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1738`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1738)*

### `MCL_ONFAULT`
```rust
const MCL_ONFAULT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1739`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1739)*

### `SHADOW_STACK_SET_TOKEN`
```rust
const SHADOW_STACK_SET_TOKEN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1740`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1740)*

### `SHADOW_STACK_SET_MARKER`
```rust
const SHADOW_STACK_SET_MARKER: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1741`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1741)*

### `HUGETLB_FLAG_ENCODE_SHIFT`
```rust
const HUGETLB_FLAG_ENCODE_SHIFT: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1742`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1742)*

### `HUGETLB_FLAG_ENCODE_MASK`
```rust
const HUGETLB_FLAG_ENCODE_MASK: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1743`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1743)*

### `HUGETLB_FLAG_ENCODE_16KB`
```rust
const HUGETLB_FLAG_ENCODE_16KB: u32 = 939_524_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1744`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1744)*

### `HUGETLB_FLAG_ENCODE_64KB`
```rust
const HUGETLB_FLAG_ENCODE_64KB: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1745`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1745)*

### `HUGETLB_FLAG_ENCODE_512KB`
```rust
const HUGETLB_FLAG_ENCODE_512KB: u32 = 1_275_068_416u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1746`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1746)*

### `HUGETLB_FLAG_ENCODE_1MB`
```rust
const HUGETLB_FLAG_ENCODE_1MB: u32 = 1_342_177_280u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1747`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1747)*

### `HUGETLB_FLAG_ENCODE_2MB`
```rust
const HUGETLB_FLAG_ENCODE_2MB: u32 = 1_409_286_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1748`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1748)*

### `HUGETLB_FLAG_ENCODE_8MB`
```rust
const HUGETLB_FLAG_ENCODE_8MB: u32 = 1_543_503_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1749`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1749)*

### `HUGETLB_FLAG_ENCODE_16MB`
```rust
const HUGETLB_FLAG_ENCODE_16MB: u32 = 1_610_612_736u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1750`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1750)*

### `HUGETLB_FLAG_ENCODE_32MB`
```rust
const HUGETLB_FLAG_ENCODE_32MB: u32 = 1_677_721_600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1751`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1751)*

### `HUGETLB_FLAG_ENCODE_256MB`
```rust
const HUGETLB_FLAG_ENCODE_256MB: u32 = 1_879_048_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1752`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1752)*

### `HUGETLB_FLAG_ENCODE_512MB`
```rust
const HUGETLB_FLAG_ENCODE_512MB: u32 = 1_946_157_056u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1753`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1753)*

### `HUGETLB_FLAG_ENCODE_1GB`
```rust
const HUGETLB_FLAG_ENCODE_1GB: u32 = 2_013_265_920u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1754`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1754)*

### `HUGETLB_FLAG_ENCODE_2GB`
```rust
const HUGETLB_FLAG_ENCODE_2GB: u32 = 2_080_374_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1755`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1755)*

### `HUGETLB_FLAG_ENCODE_16GB`
```rust
const HUGETLB_FLAG_ENCODE_16GB: u32 = 2_281_701_376u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1756`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1756)*

### `MREMAP_MAYMOVE`
```rust
const MREMAP_MAYMOVE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1757`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1757)*

### `MREMAP_FIXED`
```rust
const MREMAP_FIXED: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1758`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1758)*

### `MREMAP_DONTUNMAP`
```rust
const MREMAP_DONTUNMAP: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1759`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1759)*

### `OVERCOMMIT_GUESS`
```rust
const OVERCOMMIT_GUESS: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1760`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1760)*

### `OVERCOMMIT_ALWAYS`
```rust
const OVERCOMMIT_ALWAYS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1761`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1761)*

### `OVERCOMMIT_NEVER`
```rust
const OVERCOMMIT_NEVER: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1762`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1762)*

### `MAP_SHARED`
```rust
const MAP_SHARED: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1763`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1763)*

### `MAP_PRIVATE`
```rust
const MAP_PRIVATE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1764`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1764)*

### `MAP_SHARED_VALIDATE`
```rust
const MAP_SHARED_VALIDATE: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1765`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1765)*

### `MAP_DROPPABLE`
```rust
const MAP_DROPPABLE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1766`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1766)*

### `MAP_HUGE_SHIFT`
```rust
const MAP_HUGE_SHIFT: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1767`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1767)*

### `MAP_HUGE_MASK`
```rust
const MAP_HUGE_MASK: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1768`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1768)*

### `MAP_HUGE_16KB`
```rust
const MAP_HUGE_16KB: u32 = 939_524_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1769`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1769)*

### `MAP_HUGE_64KB`
```rust
const MAP_HUGE_64KB: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1770`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1770)*

### `MAP_HUGE_512KB`
```rust
const MAP_HUGE_512KB: u32 = 1_275_068_416u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1771`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1771)*

### `MAP_HUGE_1MB`
```rust
const MAP_HUGE_1MB: u32 = 1_342_177_280u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1772`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1772)*

### `MAP_HUGE_2MB`
```rust
const MAP_HUGE_2MB: u32 = 1_409_286_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1773`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1773)*

### `MAP_HUGE_8MB`
```rust
const MAP_HUGE_8MB: u32 = 1_543_503_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1774`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1774)*

### `MAP_HUGE_16MB`
```rust
const MAP_HUGE_16MB: u32 = 1_610_612_736u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1775`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1775)*

### `MAP_HUGE_32MB`
```rust
const MAP_HUGE_32MB: u32 = 1_677_721_600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1776`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1776)*

### `MAP_HUGE_256MB`
```rust
const MAP_HUGE_256MB: u32 = 1_879_048_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1777`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1777)*

### `MAP_HUGE_512MB`
```rust
const MAP_HUGE_512MB: u32 = 1_946_157_056u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1778`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1778)*

### `MAP_HUGE_1GB`
```rust
const MAP_HUGE_1GB: u32 = 2_013_265_920u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1779`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1779)*

### `MAP_HUGE_2GB`
```rust
const MAP_HUGE_2GB: u32 = 2_080_374_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1780`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1780)*

### `MAP_HUGE_16GB`
```rust
const MAP_HUGE_16GB: u32 = 2_281_701_376u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1781`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1781)*

### `POLLIN`
```rust
const POLLIN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1782`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1782)*

### `POLLPRI`
```rust
const POLLPRI: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1783`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1783)*

### `POLLOUT`
```rust
const POLLOUT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1784`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1784)*

### `POLLERR`
```rust
const POLLERR: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1785`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1785)*

### `POLLHUP`
```rust
const POLLHUP: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1786`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1786)*

### `POLLNVAL`
```rust
const POLLNVAL: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1787`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1787)*

### `POLLRDNORM`
```rust
const POLLRDNORM: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1788`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1788)*

### `POLLRDBAND`
```rust
const POLLRDBAND: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1789`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1789)*

### `POLLWRNORM`
```rust
const POLLWRNORM: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1790`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1790)*

### `POLLWRBAND`
```rust
const POLLWRBAND: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1791`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1791)*

### `POLLMSG`
```rust
const POLLMSG: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1792`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1792)*

### `POLLREMOVE`
```rust
const POLLREMOVE: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1793`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1793)*

### `POLLRDHUP`
```rust
const POLLRDHUP: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1794`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1794)*

### `GRND_NONBLOCK`
```rust
const GRND_NONBLOCK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1795`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1795)*

### `GRND_RANDOM`
```rust
const GRND_RANDOM: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1796`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1796)*

### `GRND_INSECURE`
```rust
const GRND_INSECURE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1797`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1797)*

### `LINUX_REBOOT_MAGIC1`
```rust
const LINUX_REBOOT_MAGIC1: u32 = 4_276_215_469u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1798`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1798)*

### `LINUX_REBOOT_MAGIC2`
```rust
const LINUX_REBOOT_MAGIC2: u32 = 672_274_793u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1799`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1799)*

### `LINUX_REBOOT_MAGIC2A`
```rust
const LINUX_REBOOT_MAGIC2A: u32 = 85_072_278u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1800`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1800)*

### `LINUX_REBOOT_MAGIC2B`
```rust
const LINUX_REBOOT_MAGIC2B: u32 = 369_367_448u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1801`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1801)*

### `LINUX_REBOOT_MAGIC2C`
```rust
const LINUX_REBOOT_MAGIC2C: u32 = 537_993_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1802`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1802)*

### `LINUX_REBOOT_CMD_RESTART`
```rust
const LINUX_REBOOT_CMD_RESTART: u32 = 19_088_743u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1803`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1803)*

### `LINUX_REBOOT_CMD_HALT`
```rust
const LINUX_REBOOT_CMD_HALT: u32 = 3_454_992_675u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1804`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1804)*

### `LINUX_REBOOT_CMD_CAD_ON`
```rust
const LINUX_REBOOT_CMD_CAD_ON: u32 = 2_309_737_967u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1805`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1805)*

### `LINUX_REBOOT_CMD_CAD_OFF`
```rust
const LINUX_REBOOT_CMD_CAD_OFF: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1806`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1806)*

### `LINUX_REBOOT_CMD_POWER_OFF`
```rust
const LINUX_REBOOT_CMD_POWER_OFF: u32 = 1_126_301_404u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1807`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1807)*

### `LINUX_REBOOT_CMD_RESTART2`
```rust
const LINUX_REBOOT_CMD_RESTART2: u32 = 2_712_847_316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1808`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1808)*

### `LINUX_REBOOT_CMD_SW_SUSPEND`
```rust
const LINUX_REBOOT_CMD_SW_SUSPEND: u32 = 3_489_725_666u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1809`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1809)*

### `LINUX_REBOOT_CMD_KEXEC`
```rust
const LINUX_REBOOT_CMD_KEXEC: u32 = 1_163_412_803u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1810`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1810)*

### `RUSAGE_SELF`
```rust
const RUSAGE_SELF: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1811`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1811)*

### `RUSAGE_CHILDREN`
```rust
const RUSAGE_CHILDREN: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1812`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1812)*

### `RUSAGE_BOTH`
```rust
const RUSAGE_BOTH: i32 = -2i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1813`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1813)*

### `RUSAGE_THREAD`
```rust
const RUSAGE_THREAD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1814`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1814)*

### `RLIM64_INFINITY`
```rust
const RLIM64_INFINITY: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1815`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1815)*

### `PRIO_MIN`
```rust
const PRIO_MIN: i32 = -20i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1816`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1816)*

### `PRIO_MAX`
```rust
const PRIO_MAX: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1817`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1817)*

### `PRIO_PROCESS`
```rust
const PRIO_PROCESS: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1818`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1818)*

### `PRIO_PGRP`
```rust
const PRIO_PGRP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1819`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1819)*

### `PRIO_USER`
```rust
const PRIO_USER: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1820`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1820)*

### `_STK_LIM`
```rust
const _STK_LIM: u32 = 8_388_608u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1821`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1821)*

### `MLOCK_LIMIT`
```rust
const MLOCK_LIMIT: u32 = 8_388_608u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1822`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1822)*

### `RLIMIT_CPU`
```rust
const RLIMIT_CPU: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1823`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1823)*

### `RLIMIT_FSIZE`
```rust
const RLIMIT_FSIZE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1824`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1824)*

### `RLIMIT_DATA`
```rust
const RLIMIT_DATA: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1825`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1825)*

### `RLIMIT_STACK`
```rust
const RLIMIT_STACK: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1826`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1826)*

### `RLIMIT_CORE`
```rust
const RLIMIT_CORE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1827`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1827)*

### `RLIMIT_RSS`
```rust
const RLIMIT_RSS: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1828`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1828)*

### `RLIMIT_NPROC`
```rust
const RLIMIT_NPROC: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1829`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1829)*

### `RLIMIT_NOFILE`
```rust
const RLIMIT_NOFILE: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1830`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1830)*

### `RLIMIT_MEMLOCK`
```rust
const RLIMIT_MEMLOCK: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1831`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1831)*

### `RLIMIT_AS`
```rust
const RLIMIT_AS: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1832`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1832)*

### `RLIMIT_LOCKS`
```rust
const RLIMIT_LOCKS: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1833`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1833)*

### `RLIMIT_SIGPENDING`
```rust
const RLIMIT_SIGPENDING: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1834`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1834)*

### `RLIMIT_MSGQUEUE`
```rust
const RLIMIT_MSGQUEUE: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1835`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1835)*

### `RLIMIT_NICE`
```rust
const RLIMIT_NICE: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1836`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1836)*

### `RLIMIT_RTPRIO`
```rust
const RLIMIT_RTPRIO: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1837`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1837)*

### `RLIMIT_RTTIME`
```rust
const RLIMIT_RTTIME: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1838`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1838)*

### `RLIM_NLIMITS`
```rust
const RLIM_NLIMITS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1839`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1839)*

### `RLIM_INFINITY`
```rust
const RLIM_INFINITY: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1840`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1840)*

### `CSIGNAL`
```rust
const CSIGNAL: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1841`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1841)*

### `CLONE_VM`
```rust
const CLONE_VM: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1842`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1842)*

### `CLONE_FS`
```rust
const CLONE_FS: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1843`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1843)*

### `CLONE_FILES`
```rust
const CLONE_FILES: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1844`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1844)*

### `CLONE_SIGHAND`
```rust
const CLONE_SIGHAND: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1845`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1845)*

### `CLONE_PIDFD`
```rust
const CLONE_PIDFD: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1846`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1846)*

### `CLONE_PTRACE`
```rust
const CLONE_PTRACE: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1847`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1847)*

### `CLONE_VFORK`
```rust
const CLONE_VFORK: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1848`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1848)*

### `CLONE_PARENT`
```rust
const CLONE_PARENT: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1849`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1849)*

### `CLONE_THREAD`
```rust
const CLONE_THREAD: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1850`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1850)*

### `CLONE_NEWNS`
```rust
const CLONE_NEWNS: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1851`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1851)*

### `CLONE_SYSVSEM`
```rust
const CLONE_SYSVSEM: u32 = 262_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1852`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1852)*

### `CLONE_SETTLS`
```rust
const CLONE_SETTLS: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1853`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1853)*

### `CLONE_PARENT_SETTID`
```rust
const CLONE_PARENT_SETTID: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1854`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1854)*

### `CLONE_CHILD_CLEARTID`
```rust
const CLONE_CHILD_CLEARTID: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1855`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1855)*

### `CLONE_DETACHED`
```rust
const CLONE_DETACHED: u32 = 4_194_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1856`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1856)*

### `CLONE_UNTRACED`
```rust
const CLONE_UNTRACED: u32 = 8_388_608u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1857`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1857)*

### `CLONE_CHILD_SETTID`
```rust
const CLONE_CHILD_SETTID: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1858`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1858)*

### `CLONE_NEWCGROUP`
```rust
const CLONE_NEWCGROUP: u32 = 33_554_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1859`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1859)*

### `CLONE_NEWUTS`
```rust
const CLONE_NEWUTS: u32 = 67_108_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1860`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1860)*

### `CLONE_NEWIPC`
```rust
const CLONE_NEWIPC: u32 = 134_217_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1861`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1861)*

### `CLONE_NEWUSER`
```rust
const CLONE_NEWUSER: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1862`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1862)*

### `CLONE_NEWPID`
```rust
const CLONE_NEWPID: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1863`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1863)*

### `CLONE_NEWNET`
```rust
const CLONE_NEWNET: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1864`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1864)*

### `CLONE_IO`
```rust
const CLONE_IO: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1865`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1865)*

### `CLONE_CLEAR_SIGHAND`
```rust
const CLONE_CLEAR_SIGHAND: u64 = 4_294_967_296u64;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1866`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1866)*

### `CLONE_INTO_CGROUP`
```rust
const CLONE_INTO_CGROUP: u64 = 8_589_934_592u64;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1867`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1867)*

### `CLONE_NEWTIME`
```rust
const CLONE_NEWTIME: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1868`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1868)*

### `CLONE_ARGS_SIZE_VER0`
```rust
const CLONE_ARGS_SIZE_VER0: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1869`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1869)*

### `CLONE_ARGS_SIZE_VER1`
```rust
const CLONE_ARGS_SIZE_VER1: u32 = 80u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1870`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1870)*

### `CLONE_ARGS_SIZE_VER2`
```rust
const CLONE_ARGS_SIZE_VER2: u32 = 88u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1871`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1871)*

### `SCHED_NORMAL`
```rust
const SCHED_NORMAL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1872`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1872)*

### `SCHED_FIFO`
```rust
const SCHED_FIFO: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1873`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1873)*

### `SCHED_RR`
```rust
const SCHED_RR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1874`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1874)*

### `SCHED_BATCH`
```rust
const SCHED_BATCH: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1875`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1875)*

### `SCHED_IDLE`
```rust
const SCHED_IDLE: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1876`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1876)*

### `SCHED_DEADLINE`
```rust
const SCHED_DEADLINE: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1877`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1877)*

### `SCHED_EXT`
```rust
const SCHED_EXT: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1878`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1878)*

### `SCHED_RESET_ON_FORK`
```rust
const SCHED_RESET_ON_FORK: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1879`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1879)*

### `SCHED_FLAG_RESET_ON_FORK`
```rust
const SCHED_FLAG_RESET_ON_FORK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1880`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1880)*

### `SCHED_FLAG_RECLAIM`
```rust
const SCHED_FLAG_RECLAIM: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1881`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1881)*

### `SCHED_FLAG_DL_OVERRUN`
```rust
const SCHED_FLAG_DL_OVERRUN: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1882`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1882)*

### `SCHED_FLAG_KEEP_POLICY`
```rust
const SCHED_FLAG_KEEP_POLICY: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1883`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1883)*

### `SCHED_FLAG_KEEP_PARAMS`
```rust
const SCHED_FLAG_KEEP_PARAMS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1884`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1884)*

### `SCHED_FLAG_UTIL_CLAMP_MIN`
```rust
const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1885`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1885)*

### `SCHED_FLAG_UTIL_CLAMP_MAX`
```rust
const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1886`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1886)*

### `SCHED_FLAG_KEEP_ALL`
```rust
const SCHED_FLAG_KEEP_ALL: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1887`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1887)*

### `SCHED_FLAG_UTIL_CLAMP`
```rust
const SCHED_FLAG_UTIL_CLAMP: u32 = 96u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1888`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1888)*

### `SCHED_FLAG_ALL`
```rust
const SCHED_FLAG_ALL: u32 = 127u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1889`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1889)*

### `NSIG`
```rust
const NSIG: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1890`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1890)*

### `SIGHUP`
```rust
const SIGHUP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1891`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1891)*

### `SIGINT`
```rust
const SIGINT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1892`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1892)*

### `SIGQUIT`
```rust
const SIGQUIT: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1893`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1893)*

### `SIGILL`
```rust
const SIGILL: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1894`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1894)*

### `SIGTRAP`
```rust
const SIGTRAP: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1895`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1895)*

### `SIGABRT`
```rust
const SIGABRT: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1896`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1896)*

### `SIGIOT`
```rust
const SIGIOT: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1897`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1897)*

### `SIGBUS`
```rust
const SIGBUS: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1898`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1898)*

### `SIGFPE`
```rust
const SIGFPE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1899`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1899)*

### `SIGKILL`
```rust
const SIGKILL: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1900`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1900)*

### `SIGUSR1`
```rust
const SIGUSR1: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1901`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1901)*

### `SIGSEGV`
```rust
const SIGSEGV: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1902`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1902)*

### `SIGUSR2`
```rust
const SIGUSR2: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1903`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1903)*

### `SIGPIPE`
```rust
const SIGPIPE: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1904`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1904)*

### `SIGALRM`
```rust
const SIGALRM: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1905`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1905)*

### `SIGTERM`
```rust
const SIGTERM: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1906`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1906)*

### `SIGSTKFLT`
```rust
const SIGSTKFLT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1907`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1907)*

### `SIGCHLD`
```rust
const SIGCHLD: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1908`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1908)*

### `SIGCONT`
```rust
const SIGCONT: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1909`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1909)*

### `SIGSTOP`
```rust
const SIGSTOP: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1910`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1910)*

### `SIGTSTP`
```rust
const SIGTSTP: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1911`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1911)*

### `SIGTTIN`
```rust
const SIGTTIN: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1912`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1912)*

### `SIGTTOU`
```rust
const SIGTTOU: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1913`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1913)*

### `SIGURG`
```rust
const SIGURG: u32 = 23u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1914`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1914)*

### `SIGXCPU`
```rust
const SIGXCPU: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1915`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1915)*

### `SIGXFSZ`
```rust
const SIGXFSZ: u32 = 25u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1916`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1916)*

### `SIGVTALRM`
```rust
const SIGVTALRM: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1917`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1917)*

### `SIGPROF`
```rust
const SIGPROF: u32 = 27u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1918`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1918)*

### `SIGWINCH`
```rust
const SIGWINCH: u32 = 28u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1919`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1919)*

### `SIGIO`
```rust
const SIGIO: u32 = 29u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1920`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1920)*

### `SIGPOLL`
```rust
const SIGPOLL: u32 = 29u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1921`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1921)*

### `SIGPWR`
```rust
const SIGPWR: u32 = 30u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1922`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1922)*

### `SIGSYS`
```rust
const SIGSYS: u32 = 31u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1923`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1923)*

### `SIGUNUSED`
```rust
const SIGUNUSED: u32 = 31u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1924`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1924)*

### `SIGRTMIN`
```rust
const SIGRTMIN: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1925`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1925)*

### `SA_RESTORER`
```rust
const SA_RESTORER: u32 = 67_108_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1926`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1926)*

### `MINSIGSTKSZ`
```rust
const MINSIGSTKSZ: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1927`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1927)*

### `SIGSTKSZ`
```rust
const SIGSTKSZ: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1928`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1928)*

### `SA_NOCLDSTOP`
```rust
const SA_NOCLDSTOP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1929`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1929)*

### `SA_NOCLDWAIT`
```rust
const SA_NOCLDWAIT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1930`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1930)*

### `SA_SIGINFO`
```rust
const SA_SIGINFO: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1931`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1931)*

### `SA_UNSUPPORTED`
```rust
const SA_UNSUPPORTED: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1932`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1932)*

### `SA_EXPOSE_TAGBITS`
```rust
const SA_EXPOSE_TAGBITS: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1933`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1933)*

### `SA_ONSTACK`
```rust
const SA_ONSTACK: u32 = 134_217_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1934`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1934)*

### `SA_RESTART`
```rust
const SA_RESTART: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1935`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1935)*

### `SA_NODEFER`
```rust
const SA_NODEFER: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1936`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1936)*

### `SA_RESETHAND`
```rust
const SA_RESETHAND: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1937`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1937)*

### `SA_NOMASK`
```rust
const SA_NOMASK: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1938`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1938)*

### `SA_ONESHOT`
```rust
const SA_ONESHOT: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1939`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1939)*

### `SIG_BLOCK`
```rust
const SIG_BLOCK: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1940`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1940)*

### `SIG_UNBLOCK`
```rust
const SIG_UNBLOCK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1941`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1941)*

### `SIG_SETMASK`
```rust
const SIG_SETMASK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1942`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1942)*

### `SI_MAX_SIZE`
```rust
const SI_MAX_SIZE: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1943`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1943)*

### `SI_USER`
```rust
const SI_USER: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1944`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1944)*

### `SI_KERNEL`
```rust
const SI_KERNEL: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1945`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1945)*

### `SI_QUEUE`
```rust
const SI_QUEUE: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1946`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1946)*

### `SI_TIMER`
```rust
const SI_TIMER: i32 = -2i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1947`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1947)*

### `SI_MESGQ`
```rust
const SI_MESGQ: i32 = -3i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1948`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1948)*

### `SI_ASYNCIO`
```rust
const SI_ASYNCIO: i32 = -4i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1949`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1949)*

### `SI_SIGIO`
```rust
const SI_SIGIO: i32 = -5i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1950`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1950)*

### `SI_TKILL`
```rust
const SI_TKILL: i32 = -6i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1951`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1951)*

### `SI_DETHREAD`
```rust
const SI_DETHREAD: i32 = -7i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1952`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1952)*

### `SI_ASYNCNL`
```rust
const SI_ASYNCNL: i32 = -60i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1953`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1953)*

### `ILL_ILLOPC`
```rust
const ILL_ILLOPC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1954`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1954)*

### `ILL_ILLOPN`
```rust
const ILL_ILLOPN: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1955`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1955)*

### `ILL_ILLADR`
```rust
const ILL_ILLADR: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1956`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1956)*

### `ILL_ILLTRP`
```rust
const ILL_ILLTRP: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1957`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1957)*

### `ILL_PRVOPC`
```rust
const ILL_PRVOPC: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1958`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1958)*

### `ILL_PRVREG`
```rust
const ILL_PRVREG: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1959`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1959)*

### `ILL_COPROC`
```rust
const ILL_COPROC: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1960`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1960)*

### `ILL_BADSTK`
```rust
const ILL_BADSTK: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1961`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1961)*

### `ILL_BADIADDR`
```rust
const ILL_BADIADDR: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1962`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1962)*

### `__ILL_BREAK`
```rust
const __ILL_BREAK: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1963`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1963)*

### `__ILL_BNDMOD`
```rust
const __ILL_BNDMOD: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1964`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1964)*

### `NSIGILL`
```rust
const NSIGILL: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1965`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1965)*

### `FPE_INTDIV`
```rust
const FPE_INTDIV: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1966`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1966)*

### `FPE_INTOVF`
```rust
const FPE_INTOVF: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1967`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1967)*

### `FPE_FLTDIV`
```rust
const FPE_FLTDIV: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1968`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1968)*

### `FPE_FLTOVF`
```rust
const FPE_FLTOVF: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1969`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1969)*

### `FPE_FLTUND`
```rust
const FPE_FLTUND: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1970`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1970)*

### `FPE_FLTRES`
```rust
const FPE_FLTRES: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1971`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1971)*

### `FPE_FLTINV`
```rust
const FPE_FLTINV: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1972`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1972)*

### `FPE_FLTSUB`
```rust
const FPE_FLTSUB: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1973`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1973)*

### `__FPE_DECOVF`
```rust
const __FPE_DECOVF: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1974`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1974)*

### `__FPE_DECDIV`
```rust
const __FPE_DECDIV: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1975`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1975)*

### `__FPE_DECERR`
```rust
const __FPE_DECERR: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1976`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1976)*

### `__FPE_INVASC`
```rust
const __FPE_INVASC: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1977`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1977)*

### `__FPE_INVDEC`
```rust
const __FPE_INVDEC: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1978`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1978)*

### `FPE_FLTUNK`
```rust
const FPE_FLTUNK: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1979`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1979)*

### `FPE_CONDTRAP`
```rust
const FPE_CONDTRAP: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1980`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1980)*

### `NSIGFPE`
```rust
const NSIGFPE: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1981`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1981)*

### `SEGV_MAPERR`
```rust
const SEGV_MAPERR: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1982`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1982)*

### `SEGV_ACCERR`
```rust
const SEGV_ACCERR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1983`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1983)*

### `SEGV_BNDERR`
```rust
const SEGV_BNDERR: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1984`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1984)*

### `SEGV_PKUERR`
```rust
const SEGV_PKUERR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1985`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1985)*

### `SEGV_ACCADI`
```rust
const SEGV_ACCADI: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1986`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1986)*

### `SEGV_ADIDERR`
```rust
const SEGV_ADIDERR: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1987`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1987)*

### `SEGV_ADIPERR`
```rust
const SEGV_ADIPERR: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1988`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1988)*

### `SEGV_MTEAERR`
```rust
const SEGV_MTEAERR: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1989`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1989)*

### `SEGV_MTESERR`
```rust
const SEGV_MTESERR: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1990`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1990)*

### `SEGV_CPERR`
```rust
const SEGV_CPERR: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1991`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1991)*

### `NSIGSEGV`
```rust
const NSIGSEGV: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1992`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1992)*

### `BUS_ADRALN`
```rust
const BUS_ADRALN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1993`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1993)*

### `BUS_ADRERR`
```rust
const BUS_ADRERR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1994`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1994)*

### `BUS_OBJERR`
```rust
const BUS_OBJERR: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1995`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1995)*

### `BUS_MCEERR_AR`
```rust
const BUS_MCEERR_AR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1996`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1996)*

### `BUS_MCEERR_AO`
```rust
const BUS_MCEERR_AO: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1997`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1997)*

### `NSIGBUS`
```rust
const NSIGBUS: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1998`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1998)*

### `TRAP_BRKPT`
```rust
const TRAP_BRKPT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:1999`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L1999)*

### `TRAP_TRACE`
```rust
const TRAP_TRACE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2000`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2000)*

### `TRAP_BRANCH`
```rust
const TRAP_BRANCH: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2001`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2001)*

### `TRAP_HWBKPT`
```rust
const TRAP_HWBKPT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2002`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2002)*

### `TRAP_UNK`
```rust
const TRAP_UNK: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2003`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2003)*

### `TRAP_PERF`
```rust
const TRAP_PERF: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2004`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2004)*

### `NSIGTRAP`
```rust
const NSIGTRAP: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2005`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2005)*

### `TRAP_PERF_FLAG_ASYNC`
```rust
const TRAP_PERF_FLAG_ASYNC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2006`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2006)*

### `CLD_EXITED`
```rust
const CLD_EXITED: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2007`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2007)*

### `CLD_KILLED`
```rust
const CLD_KILLED: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2008`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2008)*

### `CLD_DUMPED`
```rust
const CLD_DUMPED: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2009`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2009)*

### `CLD_TRAPPED`
```rust
const CLD_TRAPPED: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2010`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2010)*

### `CLD_STOPPED`
```rust
const CLD_STOPPED: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2011`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2011)*

### `CLD_CONTINUED`
```rust
const CLD_CONTINUED: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2012`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2012)*

### `NSIGCHLD`
```rust
const NSIGCHLD: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2013`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2013)*

### `POLL_IN`
```rust
const POLL_IN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2014`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2014)*

### `POLL_OUT`
```rust
const POLL_OUT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2015`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2015)*

### `POLL_MSG`
```rust
const POLL_MSG: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2016`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2016)*

### `POLL_ERR`
```rust
const POLL_ERR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2017`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2017)*

### `POLL_PRI`
```rust
const POLL_PRI: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2018`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2018)*

### `POLL_HUP`
```rust
const POLL_HUP: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2019`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2019)*

### `NSIGPOLL`
```rust
const NSIGPOLL: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2020`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2020)*

### `SYS_SECCOMP`
```rust
const SYS_SECCOMP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2021`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2021)*

### `SYS_USER_DISPATCH`
```rust
const SYS_USER_DISPATCH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2022`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2022)*

### `NSIGSYS`
```rust
const NSIGSYS: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2023`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2023)*

### `EMT_TAGOVF`
```rust
const EMT_TAGOVF: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2024`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2024)*

### `NSIGEMT`
```rust
const NSIGEMT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2025`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2025)*

### `SIGEV_SIGNAL`
```rust
const SIGEV_SIGNAL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2026`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2026)*

### `SIGEV_NONE`
```rust
const SIGEV_NONE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2027`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2027)*

### `SIGEV_THREAD`
```rust
const SIGEV_THREAD: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2028`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2028)*

### `SIGEV_THREAD_ID`
```rust
const SIGEV_THREAD_ID: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2029`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2029)*

### `SIGEV_MAX_SIZE`
```rust
const SIGEV_MAX_SIZE: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2030`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2030)*

### `SS_ONSTACK`
```rust
const SS_ONSTACK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2031`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2031)*

### `SS_DISABLE`
```rust
const SS_DISABLE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2032`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2032)*

### `SS_AUTODISARM`
```rust
const SS_AUTODISARM: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2033`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2033)*

### `SS_FLAG_BITS`
```rust
const SS_FLAG_BITS: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2034`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2034)*

### `S_IFMT`
```rust
const S_IFMT: u32 = 61_440u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2035`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2035)*

### `S_IFSOCK`
```rust
const S_IFSOCK: u32 = 49_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2036`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2036)*

### `S_IFLNK`
```rust
const S_IFLNK: u32 = 40_960u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2037`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2037)*

### `S_IFREG`
```rust
const S_IFREG: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2038`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2038)*

### `S_IFBLK`
```rust
const S_IFBLK: u32 = 24_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2039`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2039)*

### `S_IFDIR`
```rust
const S_IFDIR: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2040`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2040)*

### `S_IFCHR`
```rust
const S_IFCHR: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2041`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2041)*

### `S_IFIFO`
```rust
const S_IFIFO: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2042`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2042)*

### `S_ISUID`
```rust
const S_ISUID: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2043`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2043)*

### `S_ISGID`
```rust
const S_ISGID: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2044`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2044)*

### `S_ISVTX`
```rust
const S_ISVTX: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2045`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2045)*

### `S_IRWXU`
```rust
const S_IRWXU: u32 = 448u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2046`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2046)*

### `S_IRUSR`
```rust
const S_IRUSR: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2047`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2047)*

### `S_IWUSR`
```rust
const S_IWUSR: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2048`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2048)*

### `S_IXUSR`
```rust
const S_IXUSR: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2049`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2049)*

### `S_IRWXG`
```rust
const S_IRWXG: u32 = 56u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2050`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2050)*

### `S_IRGRP`
```rust
const S_IRGRP: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2051`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2051)*

### `S_IWGRP`
```rust
const S_IWGRP: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2052`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2052)*

### `S_IXGRP`
```rust
const S_IXGRP: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2053`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2053)*

### `S_IRWXO`
```rust
const S_IRWXO: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2054`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2054)*

### `S_IROTH`
```rust
const S_IROTH: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2055`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2055)*

### `S_IWOTH`
```rust
const S_IWOTH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2056`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2056)*

### `S_IXOTH`
```rust
const S_IXOTH: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2057`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2057)*

### `STATX_TYPE`
```rust
const STATX_TYPE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2058`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2058)*

### `STATX_MODE`
```rust
const STATX_MODE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2059`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2059)*

### `STATX_NLINK`
```rust
const STATX_NLINK: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2060`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2060)*

### `STATX_UID`
```rust
const STATX_UID: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2061`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2061)*

### `STATX_GID`
```rust
const STATX_GID: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2062`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2062)*

### `STATX_ATIME`
```rust
const STATX_ATIME: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2063`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2063)*

### `STATX_MTIME`
```rust
const STATX_MTIME: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2064`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2064)*

### `STATX_CTIME`
```rust
const STATX_CTIME: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2065`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2065)*

### `STATX_INO`
```rust
const STATX_INO: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2066`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2066)*

### `STATX_SIZE`
```rust
const STATX_SIZE: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2067`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2067)*

### `STATX_BLOCKS`
```rust
const STATX_BLOCKS: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2068`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2068)*

### `STATX_BASIC_STATS`
```rust
const STATX_BASIC_STATS: u32 = 2_047u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2069`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2069)*

### `STATX_BTIME`
```rust
const STATX_BTIME: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2070`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2070)*

### `STATX_MNT_ID`
```rust
const STATX_MNT_ID: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2071`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2071)*

### `STATX_DIOALIGN`
```rust
const STATX_DIOALIGN: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2072`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2072)*

### `STATX_MNT_ID_UNIQUE`
```rust
const STATX_MNT_ID_UNIQUE: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2073`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2073)*

### `STATX_SUBVOL`
```rust
const STATX_SUBVOL: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2074`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2074)*

### `STATX_WRITE_ATOMIC`
```rust
const STATX_WRITE_ATOMIC: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2075`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2075)*

### `STATX_DIO_READ_ALIGN`
```rust
const STATX_DIO_READ_ALIGN: u32 = 131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2076`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2076)*

### `STATX__RESERVED`
```rust
const STATX__RESERVED: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2077`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2077)*

### `STATX_ALL`
```rust
const STATX_ALL: u32 = 4_095u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2078`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2078)*

### `STATX_ATTR_COMPRESSED`
```rust
const STATX_ATTR_COMPRESSED: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2079`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2079)*

### `STATX_ATTR_IMMUTABLE`
```rust
const STATX_ATTR_IMMUTABLE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2080`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2080)*

### `STATX_ATTR_APPEND`
```rust
const STATX_ATTR_APPEND: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2081`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2081)*

### `STATX_ATTR_NODUMP`
```rust
const STATX_ATTR_NODUMP: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2082`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2082)*

### `STATX_ATTR_ENCRYPTED`
```rust
const STATX_ATTR_ENCRYPTED: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2083`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2083)*

### `STATX_ATTR_AUTOMOUNT`
```rust
const STATX_ATTR_AUTOMOUNT: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2084`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2084)*

### `STATX_ATTR_MOUNT_ROOT`
```rust
const STATX_ATTR_MOUNT_ROOT: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2085`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2085)*

### `STATX_ATTR_VERITY`
```rust
const STATX_ATTR_VERITY: u32 = 1_048_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2086`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2086)*

### `STATX_ATTR_DAX`
```rust
const STATX_ATTR_DAX: u32 = 2_097_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2087`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2087)*

### `STATX_ATTR_WRITE_ATOMIC`
```rust
const STATX_ATTR_WRITE_ATOMIC: u32 = 4_194_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2088`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2088)*

### `IGNBRK`
```rust
const IGNBRK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2089`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2089)*

### `BRKINT`
```rust
const BRKINT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2090`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2090)*

### `IGNPAR`
```rust
const IGNPAR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2091`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2091)*

### `PARMRK`
```rust
const PARMRK: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2092`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2092)*

### `INPCK`
```rust
const INPCK: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2093`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2093)*

### `ISTRIP`
```rust
const ISTRIP: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2094`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2094)*

### `INLCR`
```rust
const INLCR: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2095`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2095)*

### `IGNCR`
```rust
const IGNCR: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2096`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2096)*

### `ICRNL`
```rust
const ICRNL: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2097`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2097)*

### `IXANY`
```rust
const IXANY: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2098`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2098)*

### `OPOST`
```rust
const OPOST: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2099`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2099)*

### `OCRNL`
```rust
const OCRNL: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2100`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2100)*

### `ONOCR`
```rust
const ONOCR: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2101`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2101)*

### `ONLRET`
```rust
const ONLRET: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2102`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2102)*

### `OFILL`
```rust
const OFILL: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2103`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2103)*

### `OFDEL`
```rust
const OFDEL: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2104`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2104)*

### `B0`
```rust
const B0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2105`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2105)*

### `B50`
```rust
const B50: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2106`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2106)*

### `B75`
```rust
const B75: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2107`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2107)*

### `B110`
```rust
const B110: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2108`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2108)*

### `B134`
```rust
const B134: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2109`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2109)*

### `B150`
```rust
const B150: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2110`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2110)*

### `B200`
```rust
const B200: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2111`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2111)*

### `B300`
```rust
const B300: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2112`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2112)*

### `B600`
```rust
const B600: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2113`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2113)*

### `B1200`
```rust
const B1200: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2114`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2114)*

### `B1800`
```rust
const B1800: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2115`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2115)*

### `B2400`
```rust
const B2400: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2116`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2116)*

### `B4800`
```rust
const B4800: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2117`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2117)*

### `B9600`
```rust
const B9600: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2118`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2118)*

### `B19200`
```rust
const B19200: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2119`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2119)*

### `B38400`
```rust
const B38400: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2120`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2120)*

### `EXTA`
```rust
const EXTA: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2121`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2121)*

### `EXTB`
```rust
const EXTB: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2122`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2122)*

### `ADDRB`
```rust
const ADDRB: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2123`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2123)*

### `CMSPAR`
```rust
const CMSPAR: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2124`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2124)*

### `CRTSCTS`
```rust
const CRTSCTS: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2125`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2125)*

### `IBSHIFT`
```rust
const IBSHIFT: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2126`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2126)*

### `TCOOFF`
```rust
const TCOOFF: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2127`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2127)*

### `TCOON`
```rust
const TCOON: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2128`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2128)*

### `TCIOFF`
```rust
const TCIOFF: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2129`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2129)*

### `TCION`
```rust
const TCION: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2130`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2130)*

### `TCIFLUSH`
```rust
const TCIFLUSH: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2131`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2131)*

### `TCOFLUSH`
```rust
const TCOFLUSH: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2132`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2132)*

### `TCIOFLUSH`
```rust
const TCIOFLUSH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2133`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2133)*

### `NCCS`
```rust
const NCCS: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2134`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2134)*

### `VINTR`
```rust
const VINTR: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2135`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2135)*

### `VQUIT`
```rust
const VQUIT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2136`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2136)*

### `VERASE`
```rust
const VERASE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2137`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2137)*

### `VKILL`
```rust
const VKILL: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2138`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2138)*

### `VEOF`
```rust
const VEOF: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2139`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2139)*

### `VTIME`
```rust
const VTIME: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2140`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2140)*

### `VMIN`
```rust
const VMIN: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2141`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2141)*

### `VSWTC`
```rust
const VSWTC: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2142`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2142)*

### `VSTART`
```rust
const VSTART: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2143`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2143)*

### `VSTOP`
```rust
const VSTOP: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2144`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2144)*

### `VSUSP`
```rust
const VSUSP: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2145`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2145)*

### `VEOL`
```rust
const VEOL: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2146`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2146)*

### `VREPRINT`
```rust
const VREPRINT: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2147`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2147)*

### `VDISCARD`
```rust
const VDISCARD: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2148`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2148)*

### `VWERASE`
```rust
const VWERASE: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2149`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2149)*

### `VLNEXT`
```rust
const VLNEXT: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2150`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2150)*

### `VEOL2`
```rust
const VEOL2: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2151`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2151)*

### `IUCLC`
```rust
const IUCLC: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2152`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2152)*

### `IXON`
```rust
const IXON: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2153`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2153)*

### `IXOFF`
```rust
const IXOFF: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2154`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2154)*

### `IMAXBEL`
```rust
const IMAXBEL: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2155`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2155)*

### `IUTF8`
```rust
const IUTF8: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2156`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2156)*

### `OLCUC`
```rust
const OLCUC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2157`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2157)*

### `ONLCR`
```rust
const ONLCR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2158`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2158)*

### `NLDLY`
```rust
const NLDLY: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2159`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2159)*

### `NL0`
```rust
const NL0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2160`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2160)*

### `NL1`
```rust
const NL1: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2161`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2161)*

### `CRDLY`
```rust
const CRDLY: u32 = 1_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2162`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2162)*

### `CR0`
```rust
const CR0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2163`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2163)*

### `CR1`
```rust
const CR1: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2164`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2164)*

### `CR2`
```rust
const CR2: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2165`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2165)*

### `CR3`
```rust
const CR3: u32 = 1_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2166`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2166)*

### `TABDLY`
```rust
const TABDLY: u32 = 6_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2167`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2167)*

### `TAB0`
```rust
const TAB0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2168`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2168)*

### `TAB1`
```rust
const TAB1: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2169`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2169)*

### `TAB2`
```rust
const TAB2: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2170`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2170)*

### `TAB3`
```rust
const TAB3: u32 = 6_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2171`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2171)*

### `XTABS`
```rust
const XTABS: u32 = 6_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2172`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2172)*

### `BSDLY`
```rust
const BSDLY: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2173`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2173)*

### `BS0`
```rust
const BS0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2174`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2174)*

### `BS1`
```rust
const BS1: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2175`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2175)*

### `VTDLY`
```rust
const VTDLY: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2176`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2176)*

### `VT0`
```rust
const VT0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2177`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2177)*

### `VT1`
```rust
const VT1: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2178`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2178)*

### `FFDLY`
```rust
const FFDLY: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2179`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2179)*

### `FF0`
```rust
const FF0: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2180`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2180)*

### `FF1`
```rust
const FF1: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2181`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2181)*

### `CBAUD`
```rust
const CBAUD: u32 = 4_111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2182`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2182)*

### `CSIZE`
```rust
const CSIZE: u32 = 48u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2183`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2183)*

### `CS5`
```rust
const CS5: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2184`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2184)*

### `CS6`
```rust
const CS6: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2185`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2185)*

### `CS7`
```rust
const CS7: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2186`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2186)*

### `CS8`
```rust
const CS8: u32 = 48u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2187`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2187)*

### `CSTOPB`
```rust
const CSTOPB: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2188`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2188)*

### `CREAD`
```rust
const CREAD: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2189`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2189)*

### `PARENB`
```rust
const PARENB: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2190`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2190)*

### `PARODD`
```rust
const PARODD: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2191`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2191)*

### `HUPCL`
```rust
const HUPCL: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2192`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2192)*

### `CLOCAL`
```rust
const CLOCAL: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2193`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2193)*

### `CBAUDEX`
```rust
const CBAUDEX: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2194`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2194)*

### `BOTHER`
```rust
const BOTHER: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2195`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2195)*

### `B57600`
```rust
const B57600: u32 = 4_097u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2196`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2196)*

### `B115200`
```rust
const B115200: u32 = 4_098u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2197`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2197)*

### `B230400`
```rust
const B230400: u32 = 4_099u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2198`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2198)*

### `B460800`
```rust
const B460800: u32 = 4_100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2199`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2199)*

### `B500000`
```rust
const B500000: u32 = 4_101u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2200`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2200)*

### `B576000`
```rust
const B576000: u32 = 4_102u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2201`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2201)*

### `B921600`
```rust
const B921600: u32 = 4_103u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2202`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2202)*

### `B1000000`
```rust
const B1000000: u32 = 4_104u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2203`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2203)*

### `B1152000`
```rust
const B1152000: u32 = 4_105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2204`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2204)*

### `B1500000`
```rust
const B1500000: u32 = 4_106u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2205`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2205)*

### `B2000000`
```rust
const B2000000: u32 = 4_107u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2206`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2206)*

### `B2500000`
```rust
const B2500000: u32 = 4_108u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2207`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2207)*

### `B3000000`
```rust
const B3000000: u32 = 4_109u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2208`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2208)*

### `B3500000`
```rust
const B3500000: u32 = 4_110u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2209`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2209)*

### `B4000000`
```rust
const B4000000: u32 = 4_111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2210`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2210)*

### `CIBAUD`
```rust
const CIBAUD: u32 = 269_418_496u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2211`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2211)*

### `ISIG`
```rust
const ISIG: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2212`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2212)*

### `ICANON`
```rust
const ICANON: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2213`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2213)*

### `XCASE`
```rust
const XCASE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2214`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2214)*

### `ECHO`
```rust
const ECHO: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2215`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2215)*

### `ECHOE`
```rust
const ECHOE: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2216`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2216)*

### `ECHOK`
```rust
const ECHOK: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2217`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2217)*

### `ECHONL`
```rust
const ECHONL: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2218`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2218)*

### `NOFLSH`
```rust
const NOFLSH: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2219`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2219)*

### `TOSTOP`
```rust
const TOSTOP: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2220`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2220)*

### `ECHOCTL`
```rust
const ECHOCTL: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2221`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2221)*

### `ECHOPRT`
```rust
const ECHOPRT: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2222`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2222)*

### `ECHOKE`
```rust
const ECHOKE: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2223`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2223)*

### `FLUSHO`
```rust
const FLUSHO: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2224`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2224)*

### `PENDIN`
```rust
const PENDIN: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2225`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2225)*

### `IEXTEN`
```rust
const IEXTEN: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2226`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2226)*

### `EXTPROC`
```rust
const EXTPROC: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2227`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2227)*

### `TCSANOW`
```rust
const TCSANOW: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2228`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2228)*

### `TCSADRAIN`
```rust
const TCSADRAIN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2229`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2229)*

### `TCSAFLUSH`
```rust
const TCSAFLUSH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2230`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2230)*

### `TIOCPKT_DATA`
```rust
const TIOCPKT_DATA: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2231`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2231)*

### `TIOCPKT_FLUSHREAD`
```rust
const TIOCPKT_FLUSHREAD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2232`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2232)*

### `TIOCPKT_FLUSHWRITE`
```rust
const TIOCPKT_FLUSHWRITE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2233`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2233)*

### `TIOCPKT_STOP`
```rust
const TIOCPKT_STOP: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2234`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2234)*

### `TIOCPKT_START`
```rust
const TIOCPKT_START: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2235`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2235)*

### `TIOCPKT_NOSTOP`
```rust
const TIOCPKT_NOSTOP: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2236`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2236)*

### `TIOCPKT_DOSTOP`
```rust
const TIOCPKT_DOSTOP: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2237`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2237)*

### `TIOCPKT_IOCTL`
```rust
const TIOCPKT_IOCTL: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2238`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2238)*

### `TIOCSER_TEMT`
```rust
const TIOCSER_TEMT: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2239`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2239)*

### `NCC`
```rust
const NCC: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2240`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2240)*

### `TIOCM_LE`
```rust
const TIOCM_LE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2241`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2241)*

### `TIOCM_DTR`
```rust
const TIOCM_DTR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2242`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2242)*

### `TIOCM_RTS`
```rust
const TIOCM_RTS: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2243`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2243)*

### `TIOCM_ST`
```rust
const TIOCM_ST: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2244`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2244)*

### `TIOCM_SR`
```rust
const TIOCM_SR: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2245`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2245)*

### `TIOCM_CTS`
```rust
const TIOCM_CTS: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2246`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2246)*

### `TIOCM_CAR`
```rust
const TIOCM_CAR: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2247`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2247)*

### `TIOCM_RNG`
```rust
const TIOCM_RNG: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2248`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2248)*

### `TIOCM_DSR`
```rust
const TIOCM_DSR: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2249`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2249)*

### `TIOCM_CD`
```rust
const TIOCM_CD: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2250`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2250)*

### `TIOCM_RI`
```rust
const TIOCM_RI: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2251`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2251)*

### `TIOCM_OUT1`
```rust
const TIOCM_OUT1: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2252`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2252)*

### `TIOCM_OUT2`
```rust
const TIOCM_OUT2: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2253`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2253)*

### `TIOCM_LOOP`
```rust
const TIOCM_LOOP: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2254`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2254)*

### `ITIMER_REAL`
```rust
const ITIMER_REAL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2255`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2255)*

### `ITIMER_VIRTUAL`
```rust
const ITIMER_VIRTUAL: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2256`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2256)*

### `ITIMER_PROF`
```rust
const ITIMER_PROF: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2257`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2257)*

### `CLOCK_REALTIME`
```rust
const CLOCK_REALTIME: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2258`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2258)*

### `CLOCK_MONOTONIC`
```rust
const CLOCK_MONOTONIC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2259`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2259)*

### `CLOCK_PROCESS_CPUTIME_ID`
```rust
const CLOCK_PROCESS_CPUTIME_ID: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2260`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2260)*

### `CLOCK_THREAD_CPUTIME_ID`
```rust
const CLOCK_THREAD_CPUTIME_ID: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2261`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2261)*

### `CLOCK_MONOTONIC_RAW`
```rust
const CLOCK_MONOTONIC_RAW: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2262`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2262)*

### `CLOCK_REALTIME_COARSE`
```rust
const CLOCK_REALTIME_COARSE: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2263`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2263)*

### `CLOCK_MONOTONIC_COARSE`
```rust
const CLOCK_MONOTONIC_COARSE: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2264`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2264)*

### `CLOCK_BOOTTIME`
```rust
const CLOCK_BOOTTIME: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2265`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2265)*

### `CLOCK_REALTIME_ALARM`
```rust
const CLOCK_REALTIME_ALARM: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2266`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2266)*

### `CLOCK_BOOTTIME_ALARM`
```rust
const CLOCK_BOOTTIME_ALARM: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2267`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2267)*

### `CLOCK_SGI_CYCLE`
```rust
const CLOCK_SGI_CYCLE: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2268`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2268)*

### `CLOCK_TAI`
```rust
const CLOCK_TAI: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2269`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2269)*

### `MAX_CLOCKS`
```rust
const MAX_CLOCKS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2270`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2270)*

### `CLOCKS_MASK`
```rust
const CLOCKS_MASK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2271`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2271)*

### `CLOCKS_MONO`
```rust
const CLOCKS_MONO: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2272`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2272)*

### `TIMER_ABSTIME`
```rust
const TIMER_ABSTIME: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2273`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2273)*

### `UIO_FASTIOV`
```rust
const UIO_FASTIOV: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2274`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2274)*

### `UIO_MAXIOV`
```rust
const UIO_MAXIOV: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2275`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2275)*

### `__X32_SYSCALL_BIT`
```rust
const __X32_SYSCALL_BIT: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2276`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2276)*

### `__NR_read`
```rust
const __NR_read: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2277`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2277)*

### `__NR_write`
```rust
const __NR_write: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2278`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2278)*

### `__NR_open`
```rust
const __NR_open: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2279`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2279)*

### `__NR_close`
```rust
const __NR_close: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2280`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2280)*

### `__NR_stat`
```rust
const __NR_stat: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2281`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2281)*

### `__NR_fstat`
```rust
const __NR_fstat: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2282`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2282)*

### `__NR_lstat`
```rust
const __NR_lstat: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2283`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2283)*

### `__NR_poll`
```rust
const __NR_poll: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2284`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2284)*

### `__NR_lseek`
```rust
const __NR_lseek: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2285`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2285)*

### `__NR_mmap`
```rust
const __NR_mmap: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2286`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2286)*

### `__NR_mprotect`
```rust
const __NR_mprotect: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2287`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2287)*

### `__NR_munmap`
```rust
const __NR_munmap: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2288`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2288)*

### `__NR_brk`
```rust
const __NR_brk: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2289`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2289)*

### `__NR_rt_sigaction`
```rust
const __NR_rt_sigaction: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2290`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2290)*

### `__NR_rt_sigprocmask`
```rust
const __NR_rt_sigprocmask: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2291`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2291)*

### `__NR_rt_sigreturn`
```rust
const __NR_rt_sigreturn: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2292`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2292)*

### `__NR_ioctl`
```rust
const __NR_ioctl: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2293`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2293)*

### `__NR_pread64`
```rust
const __NR_pread64: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2294`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2294)*

### `__NR_pwrite64`
```rust
const __NR_pwrite64: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2295`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2295)*

### `__NR_readv`
```rust
const __NR_readv: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2296`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2296)*

### `__NR_writev`
```rust
const __NR_writev: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2297`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2297)*

### `__NR_access`
```rust
const __NR_access: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2298`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2298)*

### `__NR_pipe`
```rust
const __NR_pipe: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2299`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2299)*

### `__NR_select`
```rust
const __NR_select: u32 = 23u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2300`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2300)*

### `__NR_sched_yield`
```rust
const __NR_sched_yield: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2301`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2301)*

### `__NR_mremap`
```rust
const __NR_mremap: u32 = 25u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2302`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2302)*

### `__NR_msync`
```rust
const __NR_msync: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2303`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2303)*

### `__NR_mincore`
```rust
const __NR_mincore: u32 = 27u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2304`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2304)*

### `__NR_madvise`
```rust
const __NR_madvise: u32 = 28u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2305`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2305)*

### `__NR_shmget`
```rust
const __NR_shmget: u32 = 29u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2306`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2306)*

### `__NR_shmat`
```rust
const __NR_shmat: u32 = 30u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2307`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2307)*

### `__NR_shmctl`
```rust
const __NR_shmctl: u32 = 31u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2308`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2308)*

### `__NR_dup`
```rust
const __NR_dup: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2309`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2309)*

### `__NR_dup2`
```rust
const __NR_dup2: u32 = 33u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2310`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2310)*

### `__NR_pause`
```rust
const __NR_pause: u32 = 34u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2311`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2311)*

### `__NR_nanosleep`
```rust
const __NR_nanosleep: u32 = 35u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2312`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2312)*

### `__NR_getitimer`
```rust
const __NR_getitimer: u32 = 36u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2313`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2313)*

### `__NR_alarm`
```rust
const __NR_alarm: u32 = 37u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2314`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2314)*

### `__NR_setitimer`
```rust
const __NR_setitimer: u32 = 38u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2315`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2315)*

### `__NR_getpid`
```rust
const __NR_getpid: u32 = 39u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2316`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2316)*

### `__NR_sendfile`
```rust
const __NR_sendfile: u32 = 40u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2317`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2317)*

### `__NR_socket`
```rust
const __NR_socket: u32 = 41u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2318`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2318)*

### `__NR_connect`
```rust
const __NR_connect: u32 = 42u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2319`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2319)*

### `__NR_accept`
```rust
const __NR_accept: u32 = 43u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2320`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2320)*

### `__NR_sendto`
```rust
const __NR_sendto: u32 = 44u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2321`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2321)*

### `__NR_recvfrom`
```rust
const __NR_recvfrom: u32 = 45u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2322`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2322)*

### `__NR_sendmsg`
```rust
const __NR_sendmsg: u32 = 46u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2323`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2323)*

### `__NR_recvmsg`
```rust
const __NR_recvmsg: u32 = 47u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2324`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2324)*

### `__NR_shutdown`
```rust
const __NR_shutdown: u32 = 48u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2325`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2325)*

### `__NR_bind`
```rust
const __NR_bind: u32 = 49u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2326`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2326)*

### `__NR_listen`
```rust
const __NR_listen: u32 = 50u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2327`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2327)*

### `__NR_getsockname`
```rust
const __NR_getsockname: u32 = 51u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2328`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2328)*

### `__NR_getpeername`
```rust
const __NR_getpeername: u32 = 52u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2329`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2329)*

### `__NR_socketpair`
```rust
const __NR_socketpair: u32 = 53u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2330`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2330)*

### `__NR_setsockopt`
```rust
const __NR_setsockopt: u32 = 54u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2331`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2331)*

### `__NR_getsockopt`
```rust
const __NR_getsockopt: u32 = 55u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2332`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2332)*

### `__NR_clone`
```rust
const __NR_clone: u32 = 56u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2333`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2333)*

### `__NR_fork`
```rust
const __NR_fork: u32 = 57u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2334`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2334)*

### `__NR_vfork`
```rust
const __NR_vfork: u32 = 58u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2335`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2335)*

### `__NR_execve`
```rust
const __NR_execve: u32 = 59u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2336`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2336)*

### `__NR_exit`
```rust
const __NR_exit: u32 = 60u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2337`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2337)*

### `__NR_wait4`
```rust
const __NR_wait4: u32 = 61u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2338`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2338)*

### `__NR_kill`
```rust
const __NR_kill: u32 = 62u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2339`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2339)*

### `__NR_uname`
```rust
const __NR_uname: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2340`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2340)*

### `__NR_semget`
```rust
const __NR_semget: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2341`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2341)*

### `__NR_semop`
```rust
const __NR_semop: u32 = 65u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2342`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2342)*

### `__NR_semctl`
```rust
const __NR_semctl: u32 = 66u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2343`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2343)*

### `__NR_shmdt`
```rust
const __NR_shmdt: u32 = 67u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2344`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2344)*

### `__NR_msgget`
```rust
const __NR_msgget: u32 = 68u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2345`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2345)*

### `__NR_msgsnd`
```rust
const __NR_msgsnd: u32 = 69u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2346`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2346)*

### `__NR_msgrcv`
```rust
const __NR_msgrcv: u32 = 70u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2347`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2347)*

### `__NR_msgctl`
```rust
const __NR_msgctl: u32 = 71u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2348`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2348)*

### `__NR_fcntl`
```rust
const __NR_fcntl: u32 = 72u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2349`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2349)*

### `__NR_flock`
```rust
const __NR_flock: u32 = 73u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2350`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2350)*

### `__NR_fsync`
```rust
const __NR_fsync: u32 = 74u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2351`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2351)*

### `__NR_fdatasync`
```rust
const __NR_fdatasync: u32 = 75u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2352`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2352)*

### `__NR_truncate`
```rust
const __NR_truncate: u32 = 76u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2353`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2353)*

### `__NR_ftruncate`
```rust
const __NR_ftruncate: u32 = 77u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2354`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2354)*

### `__NR_getdents`
```rust
const __NR_getdents: u32 = 78u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2355`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2355)*

### `__NR_getcwd`
```rust
const __NR_getcwd: u32 = 79u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2356`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2356)*

### `__NR_chdir`
```rust
const __NR_chdir: u32 = 80u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2357`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2357)*

### `__NR_fchdir`
```rust
const __NR_fchdir: u32 = 81u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2358`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2358)*

### `__NR_rename`
```rust
const __NR_rename: u32 = 82u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2359`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2359)*

### `__NR_mkdir`
```rust
const __NR_mkdir: u32 = 83u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2360`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2360)*

### `__NR_rmdir`
```rust
const __NR_rmdir: u32 = 84u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2361`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2361)*

### `__NR_creat`
```rust
const __NR_creat: u32 = 85u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2362`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2362)*

### `__NR_link`
```rust
const __NR_link: u32 = 86u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2363`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2363)*

### `__NR_unlink`
```rust
const __NR_unlink: u32 = 87u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2364`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2364)*

### `__NR_symlink`
```rust
const __NR_symlink: u32 = 88u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2365`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2365)*

### `__NR_readlink`
```rust
const __NR_readlink: u32 = 89u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2366`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2366)*

### `__NR_chmod`
```rust
const __NR_chmod: u32 = 90u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2367`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2367)*

### `__NR_fchmod`
```rust
const __NR_fchmod: u32 = 91u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2368`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2368)*

### `__NR_chown`
```rust
const __NR_chown: u32 = 92u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2369`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2369)*

### `__NR_fchown`
```rust
const __NR_fchown: u32 = 93u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2370`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2370)*

### `__NR_lchown`
```rust
const __NR_lchown: u32 = 94u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2371`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2371)*

### `__NR_umask`
```rust
const __NR_umask: u32 = 95u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2372`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2372)*

### `__NR_gettimeofday`
```rust
const __NR_gettimeofday: u32 = 96u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2373`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2373)*

### `__NR_getrlimit`
```rust
const __NR_getrlimit: u32 = 97u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2374`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2374)*

### `__NR_getrusage`
```rust
const __NR_getrusage: u32 = 98u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2375`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2375)*

### `__NR_sysinfo`
```rust
const __NR_sysinfo: u32 = 99u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2376`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2376)*

### `__NR_times`
```rust
const __NR_times: u32 = 100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2377`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2377)*

### `__NR_ptrace`
```rust
const __NR_ptrace: u32 = 101u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2378`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2378)*

### `__NR_getuid`
```rust
const __NR_getuid: u32 = 102u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2379`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2379)*

### `__NR_syslog`
```rust
const __NR_syslog: u32 = 103u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2380`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2380)*

### `__NR_getgid`
```rust
const __NR_getgid: u32 = 104u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2381`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2381)*

### `__NR_setuid`
```rust
const __NR_setuid: u32 = 105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2382`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2382)*

### `__NR_setgid`
```rust
const __NR_setgid: u32 = 106u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2383`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2383)*

### `__NR_geteuid`
```rust
const __NR_geteuid: u32 = 107u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2384`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2384)*

### `__NR_getegid`
```rust
const __NR_getegid: u32 = 108u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2385`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2385)*

### `__NR_setpgid`
```rust
const __NR_setpgid: u32 = 109u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2386`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2386)*

### `__NR_getppid`
```rust
const __NR_getppid: u32 = 110u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2387`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2387)*

### `__NR_getpgrp`
```rust
const __NR_getpgrp: u32 = 111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2388`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2388)*

### `__NR_setsid`
```rust
const __NR_setsid: u32 = 112u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2389`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2389)*

### `__NR_setreuid`
```rust
const __NR_setreuid: u32 = 113u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2390`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2390)*

### `__NR_setregid`
```rust
const __NR_setregid: u32 = 114u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2391`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2391)*

### `__NR_getgroups`
```rust
const __NR_getgroups: u32 = 115u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2392`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2392)*

### `__NR_setgroups`
```rust
const __NR_setgroups: u32 = 116u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2393`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2393)*

### `__NR_setresuid`
```rust
const __NR_setresuid: u32 = 117u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2394`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2394)*

### `__NR_getresuid`
```rust
const __NR_getresuid: u32 = 118u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2395`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2395)*

### `__NR_setresgid`
```rust
const __NR_setresgid: u32 = 119u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2396`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2396)*

### `__NR_getresgid`
```rust
const __NR_getresgid: u32 = 120u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2397`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2397)*

### `__NR_getpgid`
```rust
const __NR_getpgid: u32 = 121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2398`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2398)*

### `__NR_setfsuid`
```rust
const __NR_setfsuid: u32 = 122u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2399`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2399)*

### `__NR_setfsgid`
```rust
const __NR_setfsgid: u32 = 123u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2400`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2400)*

### `__NR_getsid`
```rust
const __NR_getsid: u32 = 124u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2401`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2401)*

### `__NR_capget`
```rust
const __NR_capget: u32 = 125u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2402`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2402)*

### `__NR_capset`
```rust
const __NR_capset: u32 = 126u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2403`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2403)*

### `__NR_rt_sigpending`
```rust
const __NR_rt_sigpending: u32 = 127u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2404`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2404)*

### `__NR_rt_sigtimedwait`
```rust
const __NR_rt_sigtimedwait: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2405`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2405)*

### `__NR_rt_sigqueueinfo`
```rust
const __NR_rt_sigqueueinfo: u32 = 129u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2406`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2406)*

### `__NR_rt_sigsuspend`
```rust
const __NR_rt_sigsuspend: u32 = 130u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2407`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2407)*

### `__NR_sigaltstack`
```rust
const __NR_sigaltstack: u32 = 131u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2408`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2408)*

### `__NR_utime`
```rust
const __NR_utime: u32 = 132u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2409`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2409)*

### `__NR_mknod`
```rust
const __NR_mknod: u32 = 133u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2410`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2410)*

### `__NR_uselib`
```rust
const __NR_uselib: u32 = 134u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2411`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2411)*

### `__NR_personality`
```rust
const __NR_personality: u32 = 135u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2412`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2412)*

### `__NR_ustat`
```rust
const __NR_ustat: u32 = 136u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2413`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2413)*

### `__NR_statfs`
```rust
const __NR_statfs: u32 = 137u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2414`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2414)*

### `__NR_fstatfs`
```rust
const __NR_fstatfs: u32 = 138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2415`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2415)*

### `__NR_sysfs`
```rust
const __NR_sysfs: u32 = 139u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2416`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2416)*

### `__NR_getpriority`
```rust
const __NR_getpriority: u32 = 140u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2417`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2417)*

### `__NR_setpriority`
```rust
const __NR_setpriority: u32 = 141u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2418`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2418)*

### `__NR_sched_setparam`
```rust
const __NR_sched_setparam: u32 = 142u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2419`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2419)*

### `__NR_sched_getparam`
```rust
const __NR_sched_getparam: u32 = 143u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2420`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2420)*

### `__NR_sched_setscheduler`
```rust
const __NR_sched_setscheduler: u32 = 144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2421`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2421)*

### `__NR_sched_getscheduler`
```rust
const __NR_sched_getscheduler: u32 = 145u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2422`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2422)*

### `__NR_sched_get_priority_max`
```rust
const __NR_sched_get_priority_max: u32 = 146u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2423`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2423)*

### `__NR_sched_get_priority_min`
```rust
const __NR_sched_get_priority_min: u32 = 147u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2424`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2424)*

### `__NR_sched_rr_get_interval`
```rust
const __NR_sched_rr_get_interval: u32 = 148u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2425`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2425)*

### `__NR_mlock`
```rust
const __NR_mlock: u32 = 149u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2426`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2426)*

### `__NR_munlock`
```rust
const __NR_munlock: u32 = 150u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2427`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2427)*

### `__NR_mlockall`
```rust
const __NR_mlockall: u32 = 151u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2428`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2428)*

### `__NR_munlockall`
```rust
const __NR_munlockall: u32 = 152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2429`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2429)*

### `__NR_vhangup`
```rust
const __NR_vhangup: u32 = 153u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2430`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2430)*

### `__NR_modify_ldt`
```rust
const __NR_modify_ldt: u32 = 154u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2431`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2431)*

### `__NR_pivot_root`
```rust
const __NR_pivot_root: u32 = 155u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2432`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2432)*

### `__NR__sysctl`
```rust
const __NR__sysctl: u32 = 156u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2433`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2433)*

### `__NR_prctl`
```rust
const __NR_prctl: u32 = 157u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2434`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2434)*

### `__NR_arch_prctl`
```rust
const __NR_arch_prctl: u32 = 158u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2435`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2435)*

### `__NR_adjtimex`
```rust
const __NR_adjtimex: u32 = 159u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2436`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2436)*

### `__NR_setrlimit`
```rust
const __NR_setrlimit: u32 = 160u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2437`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2437)*

### `__NR_chroot`
```rust
const __NR_chroot: u32 = 161u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2438`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2438)*

### `__NR_sync`
```rust
const __NR_sync: u32 = 162u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2439`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2439)*

### `__NR_acct`
```rust
const __NR_acct: u32 = 163u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2440`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2440)*

### `__NR_settimeofday`
```rust
const __NR_settimeofday: u32 = 164u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2441`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2441)*

### `__NR_mount`
```rust
const __NR_mount: u32 = 165u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2442`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2442)*

### `__NR_umount2`
```rust
const __NR_umount2: u32 = 166u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2443`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2443)*

### `__NR_swapon`
```rust
const __NR_swapon: u32 = 167u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2444`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2444)*

### `__NR_swapoff`
```rust
const __NR_swapoff: u32 = 168u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2445`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2445)*

### `__NR_reboot`
```rust
const __NR_reboot: u32 = 169u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2446`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2446)*

### `__NR_sethostname`
```rust
const __NR_sethostname: u32 = 170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2447`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2447)*

### `__NR_setdomainname`
```rust
const __NR_setdomainname: u32 = 171u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2448`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2448)*

### `__NR_iopl`
```rust
const __NR_iopl: u32 = 172u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2449`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2449)*

### `__NR_ioperm`
```rust
const __NR_ioperm: u32 = 173u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2450`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2450)*

### `__NR_create_module`
```rust
const __NR_create_module: u32 = 174u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2451`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2451)*

### `__NR_init_module`
```rust
const __NR_init_module: u32 = 175u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2452`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2452)*

### `__NR_delete_module`
```rust
const __NR_delete_module: u32 = 176u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2453`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2453)*

### `__NR_get_kernel_syms`
```rust
const __NR_get_kernel_syms: u32 = 177u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2454`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2454)*

### `__NR_query_module`
```rust
const __NR_query_module: u32 = 178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2455`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2455)*

### `__NR_quotactl`
```rust
const __NR_quotactl: u32 = 179u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2456`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2456)*

### `__NR_nfsservctl`
```rust
const __NR_nfsservctl: u32 = 180u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2457`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2457)*

### `__NR_getpmsg`
```rust
const __NR_getpmsg: u32 = 181u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2458`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2458)*

### `__NR_putpmsg`
```rust
const __NR_putpmsg: u32 = 182u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2459`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2459)*

### `__NR_afs_syscall`
```rust
const __NR_afs_syscall: u32 = 183u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2460`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2460)*

### `__NR_tuxcall`
```rust
const __NR_tuxcall: u32 = 184u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2461`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2461)*

### `__NR_security`
```rust
const __NR_security: u32 = 185u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2462`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2462)*

### `__NR_gettid`
```rust
const __NR_gettid: u32 = 186u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2463`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2463)*

### `__NR_readahead`
```rust
const __NR_readahead: u32 = 187u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2464`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2464)*

### `__NR_setxattr`
```rust
const __NR_setxattr: u32 = 188u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2465`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2465)*

### `__NR_lsetxattr`
```rust
const __NR_lsetxattr: u32 = 189u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2466`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2466)*

### `__NR_fsetxattr`
```rust
const __NR_fsetxattr: u32 = 190u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2467`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2467)*

### `__NR_getxattr`
```rust
const __NR_getxattr: u32 = 191u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2468`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2468)*

### `__NR_lgetxattr`
```rust
const __NR_lgetxattr: u32 = 192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2469`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2469)*

### `__NR_fgetxattr`
```rust
const __NR_fgetxattr: u32 = 193u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2470`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2470)*

### `__NR_listxattr`
```rust
const __NR_listxattr: u32 = 194u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2471`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2471)*

### `__NR_llistxattr`
```rust
const __NR_llistxattr: u32 = 195u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2472`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2472)*

### `__NR_flistxattr`
```rust
const __NR_flistxattr: u32 = 196u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2473`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2473)*

### `__NR_removexattr`
```rust
const __NR_removexattr: u32 = 197u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2474`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2474)*

### `__NR_lremovexattr`
```rust
const __NR_lremovexattr: u32 = 198u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2475`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2475)*

### `__NR_fremovexattr`
```rust
const __NR_fremovexattr: u32 = 199u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2476`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2476)*

### `__NR_tkill`
```rust
const __NR_tkill: u32 = 200u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2477`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2477)*

### `__NR_time`
```rust
const __NR_time: u32 = 201u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2478`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2478)*

### `__NR_futex`
```rust
const __NR_futex: u32 = 202u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2479`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2479)*

### `__NR_sched_setaffinity`
```rust
const __NR_sched_setaffinity: u32 = 203u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2480`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2480)*

### `__NR_sched_getaffinity`
```rust
const __NR_sched_getaffinity: u32 = 204u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2481`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2481)*

### `__NR_set_thread_area`
```rust
const __NR_set_thread_area: u32 = 205u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2482`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2482)*

### `__NR_io_setup`
```rust
const __NR_io_setup: u32 = 206u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2483`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2483)*

### `__NR_io_destroy`
```rust
const __NR_io_destroy: u32 = 207u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2484`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2484)*

### `__NR_io_getevents`
```rust
const __NR_io_getevents: u32 = 208u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2485`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2485)*

### `__NR_io_submit`
```rust
const __NR_io_submit: u32 = 209u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2486`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2486)*

### `__NR_io_cancel`
```rust
const __NR_io_cancel: u32 = 210u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2487`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2487)*

### `__NR_get_thread_area`
```rust
const __NR_get_thread_area: u32 = 211u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2488`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2488)*

### `__NR_lookup_dcookie`
```rust
const __NR_lookup_dcookie: u32 = 212u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2489`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2489)*

### `__NR_epoll_create`
```rust
const __NR_epoll_create: u32 = 213u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2490`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2490)*

### `__NR_epoll_ctl_old`
```rust
const __NR_epoll_ctl_old: u32 = 214u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2491`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2491)*

### `__NR_epoll_wait_old`
```rust
const __NR_epoll_wait_old: u32 = 215u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2492`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2492)*

### `__NR_remap_file_pages`
```rust
const __NR_remap_file_pages: u32 = 216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2493`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2493)*

### `__NR_getdents64`
```rust
const __NR_getdents64: u32 = 217u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2494`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2494)*

### `__NR_set_tid_address`
```rust
const __NR_set_tid_address: u32 = 218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2495`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2495)*

### `__NR_restart_syscall`
```rust
const __NR_restart_syscall: u32 = 219u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2496`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2496)*

### `__NR_semtimedop`
```rust
const __NR_semtimedop: u32 = 220u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2497`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2497)*

### `__NR_fadvise64`
```rust
const __NR_fadvise64: u32 = 221u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2498`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2498)*

### `__NR_timer_create`
```rust
const __NR_timer_create: u32 = 222u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2499`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2499)*

### `__NR_timer_settime`
```rust
const __NR_timer_settime: u32 = 223u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2500`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2500)*

### `__NR_timer_gettime`
```rust
const __NR_timer_gettime: u32 = 224u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2501`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2501)*

### `__NR_timer_getoverrun`
```rust
const __NR_timer_getoverrun: u32 = 225u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2502`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2502)*

### `__NR_timer_delete`
```rust
const __NR_timer_delete: u32 = 226u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2503`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2503)*

### `__NR_clock_settime`
```rust
const __NR_clock_settime: u32 = 227u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2504`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2504)*

### `__NR_clock_gettime`
```rust
const __NR_clock_gettime: u32 = 228u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2505`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2505)*

### `__NR_clock_getres`
```rust
const __NR_clock_getres: u32 = 229u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2506`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2506)*

### `__NR_clock_nanosleep`
```rust
const __NR_clock_nanosleep: u32 = 230u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2507`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2507)*

### `__NR_exit_group`
```rust
const __NR_exit_group: u32 = 231u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2508`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2508)*

### `__NR_epoll_wait`
```rust
const __NR_epoll_wait: u32 = 232u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2509`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2509)*

### `__NR_epoll_ctl`
```rust
const __NR_epoll_ctl: u32 = 233u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2510`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2510)*

### `__NR_tgkill`
```rust
const __NR_tgkill: u32 = 234u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2511`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2511)*

### `__NR_utimes`
```rust
const __NR_utimes: u32 = 235u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2512`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2512)*

### `__NR_vserver`
```rust
const __NR_vserver: u32 = 236u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2513`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2513)*

### `__NR_mbind`
```rust
const __NR_mbind: u32 = 237u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2514`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2514)*

### `__NR_set_mempolicy`
```rust
const __NR_set_mempolicy: u32 = 238u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2515`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2515)*

### `__NR_get_mempolicy`
```rust
const __NR_get_mempolicy: u32 = 239u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2516`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2516)*

### `__NR_mq_open`
```rust
const __NR_mq_open: u32 = 240u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2517`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2517)*

### `__NR_mq_unlink`
```rust
const __NR_mq_unlink: u32 = 241u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2518`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2518)*

### `__NR_mq_timedsend`
```rust
const __NR_mq_timedsend: u32 = 242u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2519`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2519)*

### `__NR_mq_timedreceive`
```rust
const __NR_mq_timedreceive: u32 = 243u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2520`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2520)*

### `__NR_mq_notify`
```rust
const __NR_mq_notify: u32 = 244u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2521`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2521)*

### `__NR_mq_getsetattr`
```rust
const __NR_mq_getsetattr: u32 = 245u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2522`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2522)*

### `__NR_kexec_load`
```rust
const __NR_kexec_load: u32 = 246u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2523`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2523)*

### `__NR_waitid`
```rust
const __NR_waitid: u32 = 247u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2524`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2524)*

### `__NR_add_key`
```rust
const __NR_add_key: u32 = 248u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2525`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2525)*

### `__NR_request_key`
```rust
const __NR_request_key: u32 = 249u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2526`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2526)*

### `__NR_keyctl`
```rust
const __NR_keyctl: u32 = 250u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2527`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2527)*

### `__NR_ioprio_set`
```rust
const __NR_ioprio_set: u32 = 251u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2528`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2528)*

### `__NR_ioprio_get`
```rust
const __NR_ioprio_get: u32 = 252u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2529`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2529)*

### `__NR_inotify_init`
```rust
const __NR_inotify_init: u32 = 253u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2530`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2530)*

### `__NR_inotify_add_watch`
```rust
const __NR_inotify_add_watch: u32 = 254u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2531`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2531)*

### `__NR_inotify_rm_watch`
```rust
const __NR_inotify_rm_watch: u32 = 255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2532`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2532)*

### `__NR_migrate_pages`
```rust
const __NR_migrate_pages: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2533`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2533)*

### `__NR_openat`
```rust
const __NR_openat: u32 = 257u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2534`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2534)*

### `__NR_mkdirat`
```rust
const __NR_mkdirat: u32 = 258u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2535`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2535)*

### `__NR_mknodat`
```rust
const __NR_mknodat: u32 = 259u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2536`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2536)*

### `__NR_fchownat`
```rust
const __NR_fchownat: u32 = 260u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2537`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2537)*

### `__NR_futimesat`
```rust
const __NR_futimesat: u32 = 261u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2538`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2538)*

### `__NR_newfstatat`
```rust
const __NR_newfstatat: u32 = 262u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2539`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2539)*

### `__NR_unlinkat`
```rust
const __NR_unlinkat: u32 = 263u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2540`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2540)*

### `__NR_renameat`
```rust
const __NR_renameat: u32 = 264u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2541`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2541)*

### `__NR_linkat`
```rust
const __NR_linkat: u32 = 265u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2542`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2542)*

### `__NR_symlinkat`
```rust
const __NR_symlinkat: u32 = 266u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2543`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2543)*

### `__NR_readlinkat`
```rust
const __NR_readlinkat: u32 = 267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2544`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2544)*

### `__NR_fchmodat`
```rust
const __NR_fchmodat: u32 = 268u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2545`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2545)*

### `__NR_faccessat`
```rust
const __NR_faccessat: u32 = 269u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2546`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2546)*

### `__NR_pselect6`
```rust
const __NR_pselect6: u32 = 270u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2547`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2547)*

### `__NR_ppoll`
```rust
const __NR_ppoll: u32 = 271u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2548`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2548)*

### `__NR_unshare`
```rust
const __NR_unshare: u32 = 272u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2549`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2549)*

### `__NR_set_robust_list`
```rust
const __NR_set_robust_list: u32 = 273u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2550`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2550)*

### `__NR_get_robust_list`
```rust
const __NR_get_robust_list: u32 = 274u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2551`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2551)*

### `__NR_splice`
```rust
const __NR_splice: u32 = 275u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2552`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2552)*

### `__NR_tee`
```rust
const __NR_tee: u32 = 276u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2553`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2553)*

### `__NR_sync_file_range`
```rust
const __NR_sync_file_range: u32 = 277u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2554`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2554)*

### `__NR_vmsplice`
```rust
const __NR_vmsplice: u32 = 278u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2555`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2555)*

### `__NR_move_pages`
```rust
const __NR_move_pages: u32 = 279u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2556`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2556)*

### `__NR_utimensat`
```rust
const __NR_utimensat: u32 = 280u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2557`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2557)*

### `__NR_epoll_pwait`
```rust
const __NR_epoll_pwait: u32 = 281u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2558`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2558)*

### `__NR_signalfd`
```rust
const __NR_signalfd: u32 = 282u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2559`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2559)*

### `__NR_timerfd_create`
```rust
const __NR_timerfd_create: u32 = 283u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2560`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2560)*

### `__NR_eventfd`
```rust
const __NR_eventfd: u32 = 284u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2561`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2561)*

### `__NR_fallocate`
```rust
const __NR_fallocate: u32 = 285u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2562`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2562)*

### `__NR_timerfd_settime`
```rust
const __NR_timerfd_settime: u32 = 286u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2563`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2563)*

### `__NR_timerfd_gettime`
```rust
const __NR_timerfd_gettime: u32 = 287u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2564`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2564)*

### `__NR_accept4`
```rust
const __NR_accept4: u32 = 288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2565`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2565)*

### `__NR_signalfd4`
```rust
const __NR_signalfd4: u32 = 289u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2566`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2566)*

### `__NR_eventfd2`
```rust
const __NR_eventfd2: u32 = 290u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2567`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2567)*

### `__NR_epoll_create1`
```rust
const __NR_epoll_create1: u32 = 291u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2568`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2568)*

### `__NR_dup3`
```rust
const __NR_dup3: u32 = 292u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2569`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2569)*

### `__NR_pipe2`
```rust
const __NR_pipe2: u32 = 293u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2570`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2570)*

### `__NR_inotify_init1`
```rust
const __NR_inotify_init1: u32 = 294u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2571`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2571)*

### `__NR_preadv`
```rust
const __NR_preadv: u32 = 295u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2572`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2572)*

### `__NR_pwritev`
```rust
const __NR_pwritev: u32 = 296u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2573`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2573)*

### `__NR_rt_tgsigqueueinfo`
```rust
const __NR_rt_tgsigqueueinfo: u32 = 297u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2574`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2574)*

### `__NR_perf_event_open`
```rust
const __NR_perf_event_open: u32 = 298u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2575`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2575)*

### `__NR_recvmmsg`
```rust
const __NR_recvmmsg: u32 = 299u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2576`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2576)*

### `__NR_fanotify_init`
```rust
const __NR_fanotify_init: u32 = 300u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2577`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2577)*

### `__NR_fanotify_mark`
```rust
const __NR_fanotify_mark: u32 = 301u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2578`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2578)*

### `__NR_prlimit64`
```rust
const __NR_prlimit64: u32 = 302u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2579`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2579)*

### `__NR_name_to_handle_at`
```rust
const __NR_name_to_handle_at: u32 = 303u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2580`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2580)*

### `__NR_open_by_handle_at`
```rust
const __NR_open_by_handle_at: u32 = 304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2581`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2581)*

### `__NR_clock_adjtime`
```rust
const __NR_clock_adjtime: u32 = 305u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2582`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2582)*

### `__NR_syncfs`
```rust
const __NR_syncfs: u32 = 306u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2583`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2583)*

### `__NR_sendmmsg`
```rust
const __NR_sendmmsg: u32 = 307u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2584`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2584)*

### `__NR_setns`
```rust
const __NR_setns: u32 = 308u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2585`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2585)*

### `__NR_getcpu`
```rust
const __NR_getcpu: u32 = 309u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2586`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2586)*

### `__NR_process_vm_readv`
```rust
const __NR_process_vm_readv: u32 = 310u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2587`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2587)*

### `__NR_process_vm_writev`
```rust
const __NR_process_vm_writev: u32 = 311u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2588`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2588)*

### `__NR_kcmp`
```rust
const __NR_kcmp: u32 = 312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2589`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2589)*

### `__NR_finit_module`
```rust
const __NR_finit_module: u32 = 313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2590`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2590)*

### `__NR_sched_setattr`
```rust
const __NR_sched_setattr: u32 = 314u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2591`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2591)*

### `__NR_sched_getattr`
```rust
const __NR_sched_getattr: u32 = 315u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2592`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2592)*

### `__NR_renameat2`
```rust
const __NR_renameat2: u32 = 316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2593`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2593)*

### `__NR_seccomp`
```rust
const __NR_seccomp: u32 = 317u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2594`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2594)*

### `__NR_getrandom`
```rust
const __NR_getrandom: u32 = 318u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2595`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2595)*

### `__NR_memfd_create`
```rust
const __NR_memfd_create: u32 = 319u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2596`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2596)*

### `__NR_kexec_file_load`
```rust
const __NR_kexec_file_load: u32 = 320u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2597`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2597)*

### `__NR_bpf`
```rust
const __NR_bpf: u32 = 321u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2598`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2598)*

### `__NR_execveat`
```rust
const __NR_execveat: u32 = 322u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2599`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2599)*

### `__NR_userfaultfd`
```rust
const __NR_userfaultfd: u32 = 323u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2600`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2600)*

### `__NR_membarrier`
```rust
const __NR_membarrier: u32 = 324u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2601`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2601)*

### `__NR_mlock2`
```rust
const __NR_mlock2: u32 = 325u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2602`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2602)*

### `__NR_copy_file_range`
```rust
const __NR_copy_file_range: u32 = 326u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2603`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2603)*

### `__NR_preadv2`
```rust
const __NR_preadv2: u32 = 327u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2604`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2604)*

### `__NR_pwritev2`
```rust
const __NR_pwritev2: u32 = 328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2605`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2605)*

### `__NR_pkey_mprotect`
```rust
const __NR_pkey_mprotect: u32 = 329u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2606`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2606)*

### `__NR_pkey_alloc`
```rust
const __NR_pkey_alloc: u32 = 330u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2607`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2607)*

### `__NR_pkey_free`
```rust
const __NR_pkey_free: u32 = 331u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2608`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2608)*

### `__NR_statx`
```rust
const __NR_statx: u32 = 332u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2609`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2609)*

### `__NR_io_pgetevents`
```rust
const __NR_io_pgetevents: u32 = 333u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2610`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2610)*

### `__NR_rseq`
```rust
const __NR_rseq: u32 = 334u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2611`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2611)*

### `__NR_uretprobe`
```rust
const __NR_uretprobe: u32 = 335u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2612`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2612)*

### `__NR_pidfd_send_signal`
```rust
const __NR_pidfd_send_signal: u32 = 424u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2613`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2613)*

### `__NR_io_uring_setup`
```rust
const __NR_io_uring_setup: u32 = 425u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2614`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2614)*

### `__NR_io_uring_enter`
```rust
const __NR_io_uring_enter: u32 = 426u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2615`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2615)*

### `__NR_io_uring_register`
```rust
const __NR_io_uring_register: u32 = 427u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2616`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2616)*

### `__NR_open_tree`
```rust
const __NR_open_tree: u32 = 428u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2617`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2617)*

### `__NR_move_mount`
```rust
const __NR_move_mount: u32 = 429u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2618`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2618)*

### `__NR_fsopen`
```rust
const __NR_fsopen: u32 = 430u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2619`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2619)*

### `__NR_fsconfig`
```rust
const __NR_fsconfig: u32 = 431u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2620`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2620)*

### `__NR_fsmount`
```rust
const __NR_fsmount: u32 = 432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2621`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2621)*

### `__NR_fspick`
```rust
const __NR_fspick: u32 = 433u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2622`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2622)*

### `__NR_pidfd_open`
```rust
const __NR_pidfd_open: u32 = 434u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2623`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2623)*

### `__NR_clone3`
```rust
const __NR_clone3: u32 = 435u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2624`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2624)*

### `__NR_close_range`
```rust
const __NR_close_range: u32 = 436u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2625`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2625)*

### `__NR_openat2`
```rust
const __NR_openat2: u32 = 437u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2626`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2626)*

### `__NR_pidfd_getfd`
```rust
const __NR_pidfd_getfd: u32 = 438u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2627`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2627)*

### `__NR_faccessat2`
```rust
const __NR_faccessat2: u32 = 439u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2628`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2628)*

### `__NR_process_madvise`
```rust
const __NR_process_madvise: u32 = 440u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2629`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2629)*

### `__NR_epoll_pwait2`
```rust
const __NR_epoll_pwait2: u32 = 441u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2630`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2630)*

### `__NR_mount_setattr`
```rust
const __NR_mount_setattr: u32 = 442u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2631`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2631)*

### `__NR_quotactl_fd`
```rust
const __NR_quotactl_fd: u32 = 443u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2632`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2632)*

### `__NR_landlock_create_ruleset`
```rust
const __NR_landlock_create_ruleset: u32 = 444u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2633`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2633)*

### `__NR_landlock_add_rule`
```rust
const __NR_landlock_add_rule: u32 = 445u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2634`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2634)*

### `__NR_landlock_restrict_self`
```rust
const __NR_landlock_restrict_self: u32 = 446u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2635`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2635)*

### `__NR_memfd_secret`
```rust
const __NR_memfd_secret: u32 = 447u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2636`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2636)*

### `__NR_process_mrelease`
```rust
const __NR_process_mrelease: u32 = 448u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2637`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2637)*

### `__NR_futex_waitv`
```rust
const __NR_futex_waitv: u32 = 449u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2638`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2638)*

### `__NR_set_mempolicy_home_node`
```rust
const __NR_set_mempolicy_home_node: u32 = 450u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2639`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2639)*

### `__NR_cachestat`
```rust
const __NR_cachestat: u32 = 451u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2640`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2640)*

### `__NR_fchmodat2`
```rust
const __NR_fchmodat2: u32 = 452u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2641`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2641)*

### `__NR_map_shadow_stack`
```rust
const __NR_map_shadow_stack: u32 = 453u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2642`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2642)*

### `__NR_futex_wake`
```rust
const __NR_futex_wake: u32 = 454u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2643`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2643)*

### `__NR_futex_wait`
```rust
const __NR_futex_wait: u32 = 455u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2644`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2644)*

### `__NR_futex_requeue`
```rust
const __NR_futex_requeue: u32 = 456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2645`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2645)*

### `__NR_statmount`
```rust
const __NR_statmount: u32 = 457u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2646`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2646)*

### `__NR_listmount`
```rust
const __NR_listmount: u32 = 458u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2647`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2647)*

### `__NR_lsm_get_self_attr`
```rust
const __NR_lsm_get_self_attr: u32 = 459u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2648`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2648)*

### `__NR_lsm_set_self_attr`
```rust
const __NR_lsm_set_self_attr: u32 = 460u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2649`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2649)*

### `__NR_lsm_list_modules`
```rust
const __NR_lsm_list_modules: u32 = 461u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2650`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2650)*

### `__NR_mseal`
```rust
const __NR_mseal: u32 = 462u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2651`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2651)*

### `__NR_setxattrat`
```rust
const __NR_setxattrat: u32 = 463u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2652`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2652)*

### `__NR_getxattrat`
```rust
const __NR_getxattrat: u32 = 464u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2653`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2653)*

### `__NR_listxattrat`
```rust
const __NR_listxattrat: u32 = 465u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2654`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2654)*

### `__NR_removexattrat`
```rust
const __NR_removexattrat: u32 = 466u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2655`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2655)*

### `__NR_open_tree_attr`
```rust
const __NR_open_tree_attr: u32 = 467u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2656`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2656)*

### `WNOHANG`
```rust
const WNOHANG: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2657`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2657)*

### `WUNTRACED`
```rust
const WUNTRACED: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2658`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2658)*

### `WSTOPPED`
```rust
const WSTOPPED: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2659`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2659)*

### `WEXITED`
```rust
const WEXITED: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2660`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2660)*

### `WCONTINUED`
```rust
const WCONTINUED: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2661`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2661)*

### `WNOWAIT`
```rust
const WNOWAIT: u32 = 16_777_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2662`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2662)*

### `__WNOTHREAD`
```rust
const __WNOTHREAD: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2663`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2663)*

### `__WALL`
```rust
const __WALL: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2664`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2664)*

### `__WCLONE`
```rust
const __WCLONE: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2665`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2665)*

### `P_ALL`
```rust
const P_ALL: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2666`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2666)*

### `P_PID`
```rust
const P_PID: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2667`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2667)*

### `P_PGID`
```rust
const P_PGID: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2668`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2668)*

### `P_PIDFD`
```rust
const P_PIDFD: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2669`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2669)*

### `XATTR_CREATE`
```rust
const XATTR_CREATE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2670`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2670)*

### `XATTR_REPLACE`
```rust
const XATTR_REPLACE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2671`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2671)*

### `XATTR_OS2_PREFIX`
```rust
const XATTR_OS2_PREFIX: &[u8; 5];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2672`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2672)*

### `XATTR_MAC_OSX_PREFIX`
```rust
const XATTR_MAC_OSX_PREFIX: &[u8; 5];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2673`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2673)*

### `XATTR_BTRFS_PREFIX`
```rust
const XATTR_BTRFS_PREFIX: &[u8; 7];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2674`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2674)*

### `XATTR_HURD_PREFIX`
```rust
const XATTR_HURD_PREFIX: &[u8; 5];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2675`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2675)*

### `XATTR_SECURITY_PREFIX`
```rust
const XATTR_SECURITY_PREFIX: &[u8; 10];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2676`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2676)*

### `XATTR_SYSTEM_PREFIX`
```rust
const XATTR_SYSTEM_PREFIX: &[u8; 8];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2677`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2677)*

### `XATTR_TRUSTED_PREFIX`
```rust
const XATTR_TRUSTED_PREFIX: &[u8; 9];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2678`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2678)*

### `XATTR_USER_PREFIX`
```rust
const XATTR_USER_PREFIX: &[u8; 6];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2679`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2679)*

### `XATTR_EVM_SUFFIX`
```rust
const XATTR_EVM_SUFFIX: &[u8; 4];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2680`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2680)*

### `XATTR_NAME_EVM`
```rust
const XATTR_NAME_EVM: &[u8; 13];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2681`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2681)*

### `XATTR_IMA_SUFFIX`
```rust
const XATTR_IMA_SUFFIX: &[u8; 4];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2682`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2682)*

### `XATTR_NAME_IMA`
```rust
const XATTR_NAME_IMA: &[u8; 13];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2683`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2683)*

### `XATTR_SELINUX_SUFFIX`
```rust
const XATTR_SELINUX_SUFFIX: &[u8; 8];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2684`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2684)*

### `XATTR_NAME_SELINUX`
```rust
const XATTR_NAME_SELINUX: &[u8; 17];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2685`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2685)*

### `XATTR_SMACK_SUFFIX`
```rust
const XATTR_SMACK_SUFFIX: &[u8; 8];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2686`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2686)*

### `XATTR_SMACK_IPIN`
```rust
const XATTR_SMACK_IPIN: &[u8; 12];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2687`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2687)*

### `XATTR_SMACK_IPOUT`
```rust
const XATTR_SMACK_IPOUT: &[u8; 13];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2688`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2688)*

### `XATTR_SMACK_EXEC`
```rust
const XATTR_SMACK_EXEC: &[u8; 12];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2689`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2689)*

### `XATTR_SMACK_TRANSMUTE`
```rust
const XATTR_SMACK_TRANSMUTE: &[u8; 17];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2690`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2690)*

### `XATTR_SMACK_MMAP`
```rust
const XATTR_SMACK_MMAP: &[u8; 12];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2691`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2691)*

### `XATTR_NAME_SMACK`
```rust
const XATTR_NAME_SMACK: &[u8; 17];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2692`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2692)*

### `XATTR_NAME_SMACKIPIN`
```rust
const XATTR_NAME_SMACKIPIN: &[u8; 21];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2693`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2693)*

### `XATTR_NAME_SMACKIPOUT`
```rust
const XATTR_NAME_SMACKIPOUT: &[u8; 22];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2694`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2694)*

### `XATTR_NAME_SMACKEXEC`
```rust
const XATTR_NAME_SMACKEXEC: &[u8; 21];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2695`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2695)*

### `XATTR_NAME_SMACKTRANSMUTE`
```rust
const XATTR_NAME_SMACKTRANSMUTE: &[u8; 26];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2696`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2696)*

### `XATTR_NAME_SMACKMMAP`
```rust
const XATTR_NAME_SMACKMMAP: &[u8; 21];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2697`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2697)*

### `XATTR_APPARMOR_SUFFIX`
```rust
const XATTR_APPARMOR_SUFFIX: &[u8; 9];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2698`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2698)*

### `XATTR_NAME_APPARMOR`
```rust
const XATTR_NAME_APPARMOR: &[u8; 18];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2699`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2699)*

### `XATTR_CAPS_SUFFIX`
```rust
const XATTR_CAPS_SUFFIX: &[u8; 11];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2700`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2700)*

### `XATTR_NAME_CAPS`
```rust
const XATTR_NAME_CAPS: &[u8; 20];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2701`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2701)*

### `XATTR_BPF_LSM_SUFFIX`
```rust
const XATTR_BPF_LSM_SUFFIX: &[u8; 5];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2702`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2702)*

### `XATTR_NAME_BPF_LSM`
```rust
const XATTR_NAME_BPF_LSM: &[u8; 14];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2703`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2703)*

### `XATTR_POSIX_ACL_ACCESS`
```rust
const XATTR_POSIX_ACL_ACCESS: &[u8; 17];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2704`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2704)*

### `XATTR_NAME_POSIX_ACL_ACCESS`
```rust
const XATTR_NAME_POSIX_ACL_ACCESS: &[u8; 24];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2705`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2705)*

### `XATTR_POSIX_ACL_DEFAULT`
```rust
const XATTR_POSIX_ACL_DEFAULT: &[u8; 18];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2706`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2706)*

### `XATTR_NAME_POSIX_ACL_DEFAULT`
```rust
const XATTR_NAME_POSIX_ACL_DEFAULT: &[u8; 25];
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2707`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2707)*

### `MFD_CLOEXEC`
```rust
const MFD_CLOEXEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2708`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2708)*

### `MFD_ALLOW_SEALING`
```rust
const MFD_ALLOW_SEALING: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2709`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2709)*

### `MFD_HUGETLB`
```rust
const MFD_HUGETLB: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2710`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2710)*

### `MFD_NOEXEC_SEAL`
```rust
const MFD_NOEXEC_SEAL: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2711`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2711)*

### `MFD_EXEC`
```rust
const MFD_EXEC: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2712`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2712)*

### `MFD_HUGE_SHIFT`
```rust
const MFD_HUGE_SHIFT: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2713`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2713)*

### `MFD_HUGE_MASK`
```rust
const MFD_HUGE_MASK: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2714`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2714)*

### `MFD_HUGE_64KB`
```rust
const MFD_HUGE_64KB: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2715`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2715)*

### `MFD_HUGE_512KB`
```rust
const MFD_HUGE_512KB: u32 = 1_275_068_416u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2716`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2716)*

### `MFD_HUGE_1MB`
```rust
const MFD_HUGE_1MB: u32 = 1_342_177_280u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2717`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2717)*

### `MFD_HUGE_2MB`
```rust
const MFD_HUGE_2MB: u32 = 1_409_286_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2718`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2718)*

### `MFD_HUGE_8MB`
```rust
const MFD_HUGE_8MB: u32 = 1_543_503_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2719`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2719)*

### `MFD_HUGE_16MB`
```rust
const MFD_HUGE_16MB: u32 = 1_610_612_736u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2720`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2720)*

### `MFD_HUGE_32MB`
```rust
const MFD_HUGE_32MB: u32 = 1_677_721_600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2721`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2721)*

### `MFD_HUGE_256MB`
```rust
const MFD_HUGE_256MB: u32 = 1_879_048_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2722`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2722)*

### `MFD_HUGE_512MB`
```rust
const MFD_HUGE_512MB: u32 = 1_946_157_056u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2723`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2723)*

### `MFD_HUGE_1GB`
```rust
const MFD_HUGE_1GB: u32 = 2_013_265_920u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2724`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2724)*

### `MFD_HUGE_2GB`
```rust
const MFD_HUGE_2GB: u32 = 2_080_374_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2725`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2725)*

### `MFD_HUGE_16GB`
```rust
const MFD_HUGE_16GB: u32 = 2_281_701_376u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2726`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2726)*

### `TFD_TIMER_ABSTIME`
```rust
const TFD_TIMER_ABSTIME: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2727`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2727)*

### `TFD_TIMER_CANCEL_ON_SET`
```rust
const TFD_TIMER_CANCEL_ON_SET: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2728`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2728)*

### `TFD_CLOEXEC`
```rust
const TFD_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2729`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2729)*

### `TFD_NONBLOCK`
```rust
const TFD_NONBLOCK: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2730`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2730)*

### `USERFAULTFD_IOC`
```rust
const USERFAULTFD_IOC: u32 = 170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2731`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2731)*

### `_UFFDIO_REGISTER`
```rust
const _UFFDIO_REGISTER: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2732`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2732)*

### `_UFFDIO_UNREGISTER`
```rust
const _UFFDIO_UNREGISTER: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2733`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2733)*

### `_UFFDIO_WAKE`
```rust
const _UFFDIO_WAKE: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2734`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2734)*

### `_UFFDIO_COPY`
```rust
const _UFFDIO_COPY: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2735`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2735)*

### `_UFFDIO_ZEROPAGE`
```rust
const _UFFDIO_ZEROPAGE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2736`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2736)*

### `_UFFDIO_MOVE`
```rust
const _UFFDIO_MOVE: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2737`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2737)*

### `_UFFDIO_WRITEPROTECT`
```rust
const _UFFDIO_WRITEPROTECT: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2738`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2738)*

### `_UFFDIO_CONTINUE`
```rust
const _UFFDIO_CONTINUE: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2739`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2739)*

### `_UFFDIO_POISON`
```rust
const _UFFDIO_POISON: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2740`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2740)*

### `_UFFDIO_API`
```rust
const _UFFDIO_API: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2741`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2741)*

### `UFFDIO`
```rust
const UFFDIO: u32 = 170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2742`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2742)*

### `UFFD_EVENT_PAGEFAULT`
```rust
const UFFD_EVENT_PAGEFAULT: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2743`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2743)*

### `UFFD_EVENT_FORK`
```rust
const UFFD_EVENT_FORK: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2744`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2744)*

### `UFFD_EVENT_REMAP`
```rust
const UFFD_EVENT_REMAP: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2745`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2745)*

### `UFFD_EVENT_REMOVE`
```rust
const UFFD_EVENT_REMOVE: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2746`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2746)*

### `UFFD_EVENT_UNMAP`
```rust
const UFFD_EVENT_UNMAP: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2747`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2747)*

### `UFFD_PAGEFAULT_FLAG_WRITE`
```rust
const UFFD_PAGEFAULT_FLAG_WRITE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2748`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2748)*

### `UFFD_PAGEFAULT_FLAG_WP`
```rust
const UFFD_PAGEFAULT_FLAG_WP: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2749`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2749)*

### `UFFD_PAGEFAULT_FLAG_MINOR`
```rust
const UFFD_PAGEFAULT_FLAG_MINOR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2750`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2750)*

### `UFFD_FEATURE_PAGEFAULT_FLAG_WP`
```rust
const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2751`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2751)*

### `UFFD_FEATURE_EVENT_FORK`
```rust
const UFFD_FEATURE_EVENT_FORK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2752`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2752)*

### `UFFD_FEATURE_EVENT_REMAP`
```rust
const UFFD_FEATURE_EVENT_REMAP: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2753`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2753)*

### `UFFD_FEATURE_EVENT_REMOVE`
```rust
const UFFD_FEATURE_EVENT_REMOVE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2754`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2754)*

### `UFFD_FEATURE_MISSING_HUGETLBFS`
```rust
const UFFD_FEATURE_MISSING_HUGETLBFS: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2755`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2755)*

### `UFFD_FEATURE_MISSING_SHMEM`
```rust
const UFFD_FEATURE_MISSING_SHMEM: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2756`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2756)*

### `UFFD_FEATURE_EVENT_UNMAP`
```rust
const UFFD_FEATURE_EVENT_UNMAP: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2757`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2757)*

### `UFFD_FEATURE_SIGBUS`
```rust
const UFFD_FEATURE_SIGBUS: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2758`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2758)*

### `UFFD_FEATURE_THREAD_ID`
```rust
const UFFD_FEATURE_THREAD_ID: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2759`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2759)*

### `UFFD_FEATURE_MINOR_HUGETLBFS`
```rust
const UFFD_FEATURE_MINOR_HUGETLBFS: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2760`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2760)*

### `UFFD_FEATURE_MINOR_SHMEM`
```rust
const UFFD_FEATURE_MINOR_SHMEM: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2761`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2761)*

### `UFFD_FEATURE_EXACT_ADDRESS`
```rust
const UFFD_FEATURE_EXACT_ADDRESS: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2762`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2762)*

### `UFFD_FEATURE_WP_HUGETLBFS_SHMEM`
```rust
const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2763`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2763)*

### `UFFD_FEATURE_WP_UNPOPULATED`
```rust
const UFFD_FEATURE_WP_UNPOPULATED: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2764`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2764)*

### `UFFD_FEATURE_POISON`
```rust
const UFFD_FEATURE_POISON: u32 = 16_384u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2765`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2765)*

### `UFFD_FEATURE_WP_ASYNC`
```rust
const UFFD_FEATURE_WP_ASYNC: u32 = 32_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2766`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2766)*

### `UFFD_FEATURE_MOVE`
```rust
const UFFD_FEATURE_MOVE: u32 = 65_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2767`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2767)*

### `UFFD_USER_MODE_ONLY`
```rust
const UFFD_USER_MODE_ONLY: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2768`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2768)*

### `DT_UNKNOWN`
```rust
const DT_UNKNOWN: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2769`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2769)*

### `DT_FIFO`
```rust
const DT_FIFO: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2770`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2770)*

### `DT_CHR`
```rust
const DT_CHR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2771`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2771)*

### `DT_DIR`
```rust
const DT_DIR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2772`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2772)*

### `DT_BLK`
```rust
const DT_BLK: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2773`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2773)*

### `DT_REG`
```rust
const DT_REG: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2774`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2774)*

### `DT_LNK`
```rust
const DT_LNK: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2775`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2775)*

### `DT_SOCK`
```rust
const DT_SOCK: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2776`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2776)*

### `STAT_HAVE_NSEC`
```rust
const STAT_HAVE_NSEC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2777`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2777)*

### `F_OK`
```rust
const F_OK: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2778`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2778)*

### `R_OK`
```rust
const R_OK: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2779`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2779)*

### `W_OK`
```rust
const W_OK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2780`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2780)*

### `X_OK`
```rust
const X_OK: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2781`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2781)*

### `UTIME_NOW`
```rust
const UTIME_NOW: u32 = 1_073_741_823u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2782`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2782)*

### `UTIME_OMIT`
```rust
const UTIME_OMIT: u32 = 1_073_741_822u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2783`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2783)*

### `MNT_FORCE`
```rust
const MNT_FORCE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2784`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2784)*

### `MNT_DETACH`
```rust
const MNT_DETACH: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2785`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2785)*

### `MNT_EXPIRE`
```rust
const MNT_EXPIRE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2786`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2786)*

### `UMOUNT_NOFOLLOW`
```rust
const UMOUNT_NOFOLLOW: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2787`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2787)*

### `UMOUNT_UNUSED`
```rust
const UMOUNT_UNUSED: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2788`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2788)*

### `STDIN_FILENO`
```rust
const STDIN_FILENO: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2789`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2789)*

### `STDOUT_FILENO`
```rust
const STDOUT_FILENO: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2790`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2790)*

### `STDERR_FILENO`
```rust
const STDERR_FILENO: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2791`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2791)*

### `RWF_HIPRI`
```rust
const RWF_HIPRI: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2792`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2792)*

### `RWF_DSYNC`
```rust
const RWF_DSYNC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2793`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2793)*

### `RWF_SYNC`
```rust
const RWF_SYNC: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2794`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2794)*

### `RWF_NOWAIT`
```rust
const RWF_NOWAIT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2795`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2795)*

### `RWF_APPEND`
```rust
const RWF_APPEND: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2796`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2796)*

### `EFD_SEMAPHORE`
```rust
const EFD_SEMAPHORE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2797`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2797)*

### `EFD_CLOEXEC`
```rust
const EFD_CLOEXEC: u32 = 524_288u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2798`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2798)*

### `EFD_NONBLOCK`
```rust
const EFD_NONBLOCK: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2799`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2799)*

### `EPOLLIN`
```rust
const EPOLLIN: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2800`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2800)*

### `EPOLLPRI`
```rust
const EPOLLPRI: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2801`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2801)*

### `EPOLLOUT`
```rust
const EPOLLOUT: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2802`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2802)*

### `EPOLLERR`
```rust
const EPOLLERR: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2803`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2803)*

### `EPOLLHUP`
```rust
const EPOLLHUP: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2804`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2804)*

### `EPOLLNVAL`
```rust
const EPOLLNVAL: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2805`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2805)*

### `EPOLLRDNORM`
```rust
const EPOLLRDNORM: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2806`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2806)*

### `EPOLLRDBAND`
```rust
const EPOLLRDBAND: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2807`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2807)*

### `EPOLLWRNORM`
```rust
const EPOLLWRNORM: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2808`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2808)*

### `EPOLLWRBAND`
```rust
const EPOLLWRBAND: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2809`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2809)*

### `EPOLLMSG`
```rust
const EPOLLMSG: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2810`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2810)*

### `EPOLLRDHUP`
```rust
const EPOLLRDHUP: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2811`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2811)*

### `EPOLLEXCLUSIVE`
```rust
const EPOLLEXCLUSIVE: u32 = 268_435_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2812`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2812)*

### `EPOLLWAKEUP`
```rust
const EPOLLWAKEUP: u32 = 536_870_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2813`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2813)*

### `EPOLLONESHOT`
```rust
const EPOLLONESHOT: u32 = 1_073_741_824u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2814`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2814)*

### `EPOLLET`
```rust
const EPOLLET: u32 = 2_147_483_648u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2815`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2815)*

### `TFD_SHARED_FCNTL_FLAGS`
```rust
const TFD_SHARED_FCNTL_FLAGS: u32 = 526_336u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2816`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2816)*

### `TFD_CREATE_FLAGS`
```rust
const TFD_CREATE_FLAGS: u32 = 526_336u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2817`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2817)*

### `TFD_SETTIME_FLAGS`
```rust
const TFD_SETTIME_FLAGS: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2818`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2818)*

### `ARCH_SET_FS`
```rust
const ARCH_SET_FS: u32 = 4_098u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2819`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2819)*

### `UFFD_API`
```rust
const UFFD_API: u32 = 170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2820`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2820)*

### `UFFDIO_REGISTER_MODE_MISSING`
```rust
const UFFDIO_REGISTER_MODE_MISSING: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2821`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2821)*

### `UFFDIO_REGISTER_MODE_WP`
```rust
const UFFDIO_REGISTER_MODE_WP: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2822`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2822)*

### `UFFDIO_REGISTER_MODE_MINOR`
```rust
const UFFDIO_REGISTER_MODE_MINOR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2823`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2823)*

### `UFFDIO_COPY_MODE_DONTWAKE`
```rust
const UFFDIO_COPY_MODE_DONTWAKE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2824`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2824)*

### `UFFDIO_COPY_MODE_WP`
```rust
const UFFDIO_COPY_MODE_WP: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2825`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2825)*

### `UFFDIO_ZEROPAGE_MODE_DONTWAKE`
```rust
const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2826`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2826)*

### `SPLICE_F_MOVE`
```rust
const SPLICE_F_MOVE: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2827`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2827)*

### `SPLICE_F_NONBLOCK`
```rust
const SPLICE_F_NONBLOCK: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2828`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2828)*

### `SPLICE_F_MORE`
```rust
const SPLICE_F_MORE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2829`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2829)*

### `SPLICE_F_GIFT`
```rust
const SPLICE_F_GIFT: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2830`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2830)*

### `_NSIG`
```rust
const _NSIG: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/general.rs:2831`](../../../.source_1765521767/linux-raw-sys-0.11.0/src/x86_64/general.rs#L2831)*

