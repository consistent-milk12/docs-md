*[libc](../../../../index.md) / [unix](../../../index.md) / [linux_like](../../index.md) / [linux](../index.md) / [gnu](index.md)*

---

# Module `gnu`

## Contents

- [Modules](#modules)
  - [`b64`](#b64)
  - [`x86_64`](#x86-64)
- [Structs](#structs)
  - [`aiocb`](#aiocb)
  - [`__exit_status`](#exit-status)
  - [`__timeval`](#timeval)
  - [`glob64_t`](#glob64-t)
  - [`msghdr`](#msghdr)
  - [`cmsghdr`](#cmsghdr)
  - [`termios`](#termios)
  - [`mallinfo`](#mallinfo)
  - [`mallinfo2`](#mallinfo2)
  - [`nl_pktinfo`](#nl-pktinfo)
  - [`nl_mmap_req`](#nl-mmap-req)
  - [`nl_mmap_hdr`](#nl-mmap-hdr)
  - [`ntptimeval`](#ntptimeval)
  - [`regex_t`](#regex-t)
  - [`Elf64_Chdr`](#elf64-chdr)
  - [`Elf32_Chdr`](#elf32-chdr)
  - [`seminfo`](#seminfo)
  - [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args)
  - [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry)
  - [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit)
  - [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp)
  - [`ptrace_syscall_info`](#ptrace-syscall-info)
  - [`ptrace_sud_config`](#ptrace-sud-config)
  - [`iocb`](#iocb)
  - [`tcp_info`](#tcp-info)
  - [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd)
  - [`fanotify_event_info_error`](#fanotify-event-info-error)
  - [`sem_t`](#sem-t)
  - [`mbstate_t`](#mbstate-t)
  - [`fpos64_t`](#fpos64-t)
  - [`fpos_t`](#fpos-t)
  - [`timespec`](#timespec)
  - [`utmpx`](#utmpx)
  - [`sifields_sigchld`](#sifields-sigchld)
  - [`siginfo_f`](#siginfo-f)
  - [`sigset_t`](#sigset-t)
  - [`sysinfo`](#sysinfo)
  - [`msqid_ds`](#msqid-ds)
  - [`semid_ds`](#semid-ds)
  - [`timex`](#timex)
- [Functions](#functions)
  - [`fgetspent_r`](#fgetspent-r)
  - [`sgetspent_r`](#sgetspent-r)
  - [`getspent_r`](#getspent-r)
  - [`qsort_r`](#qsort-r)
  - [`sendmmsg`](#sendmmsg)
  - [`recvmmsg`](#recvmmsg)
  - [`getrlimit64`](#getrlimit64)
  - [`setrlimit64`](#setrlimit64)
  - [`getrlimit`](#getrlimit)
  - [`setrlimit`](#setrlimit)
  - [`prlimit`](#prlimit)
  - [`prlimit64`](#prlimit64)
  - [`utmpname`](#utmpname)
  - [`utmpxname`](#utmpxname)
  - [`getutxent`](#getutxent)
  - [`getutxid`](#getutxid)
  - [`getutxline`](#getutxline)
  - [`pututxline`](#pututxline)
  - [`setutxent`](#setutxent)
  - [`endutxent`](#endutxent)
  - [`getpt`](#getpt)
  - [`mallopt`](#mallopt)
  - [`gettimeofday`](#gettimeofday)
  - [`getentropy`](#getentropy)
  - [`getrandom`](#getrandom)
  - [`getauxval`](#getauxval)
  - [`adjtimex`](#adjtimex)
  - [`ntp_adjtime`](#ntp-adjtime)
  - [`ntp_gettime`](#ntp-gettime)
  - [`clock_adjtime`](#clock-adjtime)
  - [`fanotify_mark`](#fanotify-mark)
  - [`preadv2`](#preadv2)
  - [`pwritev2`](#pwritev2)
  - [`preadv64v2`](#preadv64v2)
  - [`pwritev64v2`](#pwritev64v2)
  - [`renameat2`](#renameat2)
  - [`explicit_bzero`](#explicit-bzero)
  - [`reallocarray`](#reallocarray)
  - [`ctermid`](#ctermid)
  - [`backtrace`](#backtrace)
  - [`backtrace_symbols`](#backtrace-symbols)
  - [`backtrace_symbols_fd`](#backtrace-symbols-fd)
  - [`glob64`](#glob64)
  - [`globfree64`](#globfree64)
  - [`ptrace`](#ptrace)
  - [`pthread_attr_getaffinity_np`](#pthread-attr-getaffinity-np)
  - [`pthread_attr_setaffinity_np`](#pthread-attr-setaffinity-np)
  - [`getpriority`](#getpriority)
  - [`setpriority`](#setpriority)
  - [`pthread_rwlockattr_getkind_np`](#pthread-rwlockattr-getkind-np)
  - [`pthread_rwlockattr_setkind_np`](#pthread-rwlockattr-setkind-np)
  - [`pthread_sigqueue`](#pthread-sigqueue)
  - [`mallinfo`](#mallinfo)
  - [`mallinfo2`](#mallinfo2)
  - [`malloc_stats`](#malloc-stats)
  - [`malloc_info`](#malloc-info)
  - [`malloc_usable_size`](#malloc-usable-size)
  - [`getpwent_r`](#getpwent-r)
  - [`getgrent_r`](#getgrent-r)
  - [`fgetpwent_r`](#fgetpwent-r)
  - [`fgetgrent_r`](#fgetgrent-r)
  - [`putpwent`](#putpwent)
  - [`putgrent`](#putgrent)
  - [`sethostid`](#sethostid)
  - [`memfd_create`](#memfd-create)
  - [`mlock2`](#mlock2)
  - [`euidaccess`](#euidaccess)
  - [`eaccess`](#eaccess)
  - [`asctime_r`](#asctime-r)
  - [`ctime_r`](#ctime-r)
  - [`dirname`](#dirname)
  - [`posix_basename`](#posix-basename)
  - [`gnu_basename`](#gnu-basename)
  - [`dlmopen`](#dlmopen)
  - [`dlinfo`](#dlinfo)
  - [`dladdr1`](#dladdr1)
  - [`dlvsym`](#dlvsym)
  - [`malloc_trim`](#malloc-trim)
  - [`gnu_get_libc_release`](#gnu-get-libc-release)
  - [`gnu_get_libc_version`](#gnu-get-libc-version)
  - [`posix_spawn_file_actions_addchdir_np`](#posix-spawn-file-actions-addchdir-np)
  - [`posix_spawn_file_actions_addfchdir_np`](#posix-spawn-file-actions-addfchdir-np)
  - [`posix_spawn_file_actions_addclosefrom_np`](#posix-spawn-file-actions-addclosefrom-np)
  - [`posix_spawn_file_actions_addtcsetpgrp_np`](#posix-spawn-file-actions-addtcsetpgrp-np)
  - [`getmntent_r`](#getmntent-r)
  - [`execveat`](#execveat)
  - [`close_range`](#close-range)
  - [`mq_notify`](#mq-notify)
  - [`epoll_pwait2`](#epoll-pwait2)
  - [`mempcpy`](#mempcpy)
- [Type Aliases](#type-aliases)
  - [`pthread_t`](#pthread-t)
  - [`__priority_which_t`](#priority-which-t)
  - [`__rlimit_resource_t`](#rlimit-resource-t)
  - [`Lmid_t`](#lmid-t)
  - [`regoff_t`](#regoff-t)
  - [`__kernel_rwf_t`](#kernel-rwf-t)
  - [`Ioctl`](#ioctl)
  - [`ino_t`](#ino-t)
  - [`off_t`](#off-t)
  - [`blkcnt_t`](#blkcnt-t)
  - [`shmatt_t`](#shmatt-t)
  - [`msgqnum_t`](#msgqnum-t)
  - [`msglen_t`](#msglen-t)
  - [`fsblkcnt_t`](#fsblkcnt-t)
  - [`fsfilcnt_t`](#fsfilcnt-t)
  - [`rlim_t`](#rlim-t)
  - [`__syscall_ulong_t`](#syscall-ulong-t)
  - [`__fsword_t`](#fsword-t)
  - [`clock_t`](#clock-t)
  - [`time_t`](#time-t)
- [Constants](#constants)
  - [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift)
  - [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask)
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
  - [`MAP_HUGE_SHIFT`](#map-huge-shift)
  - [`MAP_HUGE_MASK`](#map-huge-mask)
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
  - [`PRIO_PROCESS`](#prio-process)
  - [`PRIO_PGRP`](#prio-pgrp)
  - [`PRIO_USER`](#prio-user)
  - [`MS_RMT_MASK`](#ms-rmt-mask)
  - [`__UT_LINESIZE`](#ut-linesize)
  - [`__UT_NAMESIZE`](#ut-namesize)
  - [`__UT_HOSTSIZE`](#ut-hostsize)
  - [`EMPTY`](#empty)
  - [`RUN_LVL`](#run-lvl)
  - [`BOOT_TIME`](#boot-time)
  - [`NEW_TIME`](#new-time)
  - [`OLD_TIME`](#old-time)
  - [`INIT_PROCESS`](#init-process)
  - [`LOGIN_PROCESS`](#login-process)
  - [`USER_PROCESS`](#user-process)
  - [`DEAD_PROCESS`](#dead-process)
  - [`ACCOUNTING`](#accounting)
  - [`LM_ID_BASE`](#lm-id-base)
  - [`LM_ID_NEWLM`](#lm-id-newlm)
  - [`RTLD_DI_LMID`](#rtld-di-lmid)
  - [`RTLD_DI_LINKMAP`](#rtld-di-linkmap)
  - [`RTLD_DI_CONFIGADDR`](#rtld-di-configaddr)
  - [`RTLD_DI_SERINFO`](#rtld-di-serinfo)
  - [`RTLD_DI_SERINFOSIZE`](#rtld-di-serinfosize)
  - [`RTLD_DI_ORIGIN`](#rtld-di-origin)
  - [`RTLD_DI_PROFILENAME`](#rtld-di-profilename)
  - [`RTLD_DI_PROFILEOUT`](#rtld-di-profileout)
  - [`RTLD_DI_TLS_MODID`](#rtld-di-tls-modid)
  - [`RTLD_DI_TLS_DATA`](#rtld-di-tls-data)
  - [`SOCK_NONBLOCK`](#sock-nonblock)
  - [`SOL_RXRPC`](#sol-rxrpc)
  - [`SOL_PPPOL2TP`](#sol-pppol2tp)
  - [`SOL_PNPIPE`](#sol-pnpipe)
  - [`SOL_RDS`](#sol-rds)
  - [`SOL_IUCV`](#sol-iucv)
  - [`SOL_CAIF`](#sol-caif)
  - [`SOL_NFC`](#sol-nfc)
  - [`MSG_TRYHARD`](#msg-tryhard)
  - [`LC_PAPER`](#lc-paper)
  - [`LC_NAME`](#lc-name)
  - [`LC_ADDRESS`](#lc-address)
  - [`LC_TELEPHONE`](#lc-telephone)
  - [`LC_MEASUREMENT`](#lc-measurement)
  - [`LC_IDENTIFICATION`](#lc-identification)
  - [`LC_PAPER_MASK`](#lc-paper-mask)
  - [`LC_NAME_MASK`](#lc-name-mask)
  - [`LC_ADDRESS_MASK`](#lc-address-mask)
  - [`LC_TELEPHONE_MASK`](#lc-telephone-mask)
  - [`LC_MEASUREMENT_MASK`](#lc-measurement-mask)
  - [`LC_IDENTIFICATION_MASK`](#lc-identification-mask)
  - [`LC_ALL_MASK`](#lc-all-mask)
  - [`ENOTSUP`](#enotsup)
  - [`SOCK_SEQPACKET`](#sock-seqpacket)
  - [`SOCK_DCCP`](#sock-dccp)
  - [`SOCK_PACKET`](#sock-packet)
  - [`AF_IB`](#af-ib)
  - [`AF_MPLS`](#af-mpls)
  - [`AF_NFC`](#af-nfc)
  - [`AF_VSOCK`](#af-vsock)
  - [`AF_XDP`](#af-xdp)
  - [`PF_IB`](#pf-ib)
  - [`PF_MPLS`](#pf-mpls)
  - [`PF_NFC`](#pf-nfc)
  - [`PF_VSOCK`](#pf-vsock)
  - [`PF_XDP`](#pf-xdp)
  - [`SIGEV_THREAD_ID`](#sigev-thread-id)
  - [`BUFSIZ`](#bufsiz)
  - [`TMP_MAX`](#tmp-max)
  - [`FOPEN_MAX`](#fopen-max)
  - [`FILENAME_MAX`](#filename-max)
  - [`POSIX_MADV_DONTNEED`](#posix-madv-dontneed)
  - [`_CS_GNU_LIBC_VERSION`](#cs-gnu-libc-version)
  - [`_CS_GNU_LIBPTHREAD_VERSION`](#cs-gnu-libpthread-version)
  - [`_CS_V6_ENV`](#cs-v6-env)
  - [`_CS_V7_ENV`](#cs-v7-env)
  - [`_SC_EQUIV_CLASS_MAX`](#sc-equiv-class-max)
  - [`_SC_CHARCLASS_NAME_MAX`](#sc-charclass-name-max)
  - [`_SC_PII`](#sc-pii)
  - [`_SC_PII_XTI`](#sc-pii-xti)
  - [`_SC_PII_SOCKET`](#sc-pii-socket)
  - [`_SC_PII_INTERNET`](#sc-pii-internet)
  - [`_SC_PII_OSI`](#sc-pii-osi)
  - [`_SC_POLL`](#sc-poll)
  - [`_SC_SELECT`](#sc-select)
  - [`_SC_PII_INTERNET_STREAM`](#sc-pii-internet-stream)
  - [`_SC_PII_INTERNET_DGRAM`](#sc-pii-internet-dgram)
  - [`_SC_PII_OSI_COTS`](#sc-pii-osi-cots)
  - [`_SC_PII_OSI_CLTS`](#sc-pii-osi-clts)
  - [`_SC_PII_OSI_M`](#sc-pii-osi-m)
  - [`_SC_T_IOV_MAX`](#sc-t-iov-max)
  - [`_SC_2_C_VERSION`](#sc-2-c-version)
  - [`_SC_CHAR_BIT`](#sc-char-bit)
  - [`_SC_CHAR_MAX`](#sc-char-max)
  - [`_SC_CHAR_MIN`](#sc-char-min)
  - [`_SC_INT_MAX`](#sc-int-max)
  - [`_SC_INT_MIN`](#sc-int-min)
  - [`_SC_LONG_BIT`](#sc-long-bit)
  - [`_SC_WORD_BIT`](#sc-word-bit)
  - [`_SC_MB_LEN_MAX`](#sc-mb-len-max)
  - [`_SC_SSIZE_MAX`](#sc-ssize-max)
  - [`_SC_SCHAR_MAX`](#sc-schar-max)
  - [`_SC_SCHAR_MIN`](#sc-schar-min)
  - [`_SC_SHRT_MAX`](#sc-shrt-max)
  - [`_SC_SHRT_MIN`](#sc-shrt-min)
  - [`_SC_UCHAR_MAX`](#sc-uchar-max)
  - [`_SC_UINT_MAX`](#sc-uint-max)
  - [`_SC_ULONG_MAX`](#sc-ulong-max)
  - [`_SC_USHRT_MAX`](#sc-ushrt-max)
  - [`_SC_NL_ARGMAX`](#sc-nl-argmax)
  - [`_SC_NL_LANGMAX`](#sc-nl-langmax)
  - [`_SC_NL_MSGMAX`](#sc-nl-msgmax)
  - [`_SC_NL_NMAX`](#sc-nl-nmax)
  - [`_SC_NL_SETMAX`](#sc-nl-setmax)
  - [`_SC_NL_TEXTMAX`](#sc-nl-textmax)
  - [`_SC_BASE`](#sc-base)
  - [`_SC_C_LANG_SUPPORT`](#sc-c-lang-support)
  - [`_SC_C_LANG_SUPPORT_R`](#sc-c-lang-support-r)
  - [`_SC_DEVICE_IO`](#sc-device-io)
  - [`_SC_DEVICE_SPECIFIC`](#sc-device-specific)
  - [`_SC_DEVICE_SPECIFIC_R`](#sc-device-specific-r)
  - [`_SC_FD_MGMT`](#sc-fd-mgmt)
  - [`_SC_FIFO`](#sc-fifo)
  - [`_SC_PIPE`](#sc-pipe)
  - [`_SC_FILE_ATTRIBUTES`](#sc-file-attributes)
  - [`_SC_FILE_LOCKING`](#sc-file-locking)
  - [`_SC_FILE_SYSTEM`](#sc-file-system)
  - [`_SC_MULTI_PROCESS`](#sc-multi-process)
  - [`_SC_SINGLE_PROCESS`](#sc-single-process)
  - [`_SC_NETWORKING`](#sc-networking)
  - [`_SC_REGEX_VERSION`](#sc-regex-version)
  - [`_SC_SIGNALS`](#sc-signals)
  - [`_SC_SYSTEM_DATABASE`](#sc-system-database)
  - [`_SC_SYSTEM_DATABASE_R`](#sc-system-database-r)
  - [`_SC_USER_GROUPS`](#sc-user-groups)
  - [`_SC_USER_GROUPS_R`](#sc-user-groups-r)
  - [`_SC_LEVEL1_ICACHE_SIZE`](#sc-level1-icache-size)
  - [`_SC_LEVEL1_ICACHE_ASSOC`](#sc-level1-icache-assoc)
  - [`_SC_LEVEL1_ICACHE_LINESIZE`](#sc-level1-icache-linesize)
  - [`_SC_LEVEL1_DCACHE_SIZE`](#sc-level1-dcache-size)
  - [`_SC_LEVEL1_DCACHE_ASSOC`](#sc-level1-dcache-assoc)
  - [`_SC_LEVEL1_DCACHE_LINESIZE`](#sc-level1-dcache-linesize)
  - [`_SC_LEVEL2_CACHE_SIZE`](#sc-level2-cache-size)
  - [`_SC_LEVEL2_CACHE_ASSOC`](#sc-level2-cache-assoc)
  - [`_SC_LEVEL2_CACHE_LINESIZE`](#sc-level2-cache-linesize)
  - [`_SC_LEVEL3_CACHE_SIZE`](#sc-level3-cache-size)
  - [`_SC_LEVEL3_CACHE_ASSOC`](#sc-level3-cache-assoc)
  - [`_SC_LEVEL3_CACHE_LINESIZE`](#sc-level3-cache-linesize)
  - [`_SC_LEVEL4_CACHE_SIZE`](#sc-level4-cache-size)
  - [`_SC_LEVEL4_CACHE_ASSOC`](#sc-level4-cache-assoc)
  - [`_SC_LEVEL4_CACHE_LINESIZE`](#sc-level4-cache-linesize)
  - [`O_ACCMODE`](#o-accmode)
  - [`ST_RELATIME`](#st-relatime)
  - [`NI_MAXHOST`](#ni-maxhost)
  - [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic)
  - [`XFS_SUPER_MAGIC`](#xfs-super-magic)
  - [`CPU_SETSIZE`](#cpu-setsize)
  - [`PTRACE_TRACEME`](#ptrace-traceme)
  - [`PTRACE_PEEKTEXT`](#ptrace-peektext)
  - [`PTRACE_PEEKDATA`](#ptrace-peekdata)
  - [`PTRACE_PEEKUSER`](#ptrace-peekuser)
  - [`PTRACE_POKETEXT`](#ptrace-poketext)
  - [`PTRACE_POKEDATA`](#ptrace-pokedata)
  - [`PTRACE_POKEUSER`](#ptrace-pokeuser)
  - [`PTRACE_CONT`](#ptrace-cont)
  - [`PTRACE_KILL`](#ptrace-kill)
  - [`PTRACE_SINGLESTEP`](#ptrace-singlestep)
  - [`PTRACE_ATTACH`](#ptrace-attach)
  - [`PTRACE_SYSCALL`](#ptrace-syscall)
  - [`PTRACE_SETOPTIONS`](#ptrace-setoptions)
  - [`PTRACE_GETEVENTMSG`](#ptrace-geteventmsg)
  - [`PTRACE_GETSIGINFO`](#ptrace-getsiginfo)
  - [`PTRACE_SETSIGINFO`](#ptrace-setsiginfo)
  - [`PTRACE_GETREGSET`](#ptrace-getregset)
  - [`PTRACE_SETREGSET`](#ptrace-setregset)
  - [`PTRACE_SEIZE`](#ptrace-seize)
  - [`PTRACE_INTERRUPT`](#ptrace-interrupt)
  - [`PTRACE_LISTEN`](#ptrace-listen)
  - [`PTRACE_PEEKSIGINFO`](#ptrace-peeksiginfo)
  - [`PTRACE_GETSIGMASK`](#ptrace-getsigmask)
  - [`PTRACE_SETSIGMASK`](#ptrace-setsigmask)
  - [`PTRACE_GET_SYSCALL_INFO`](#ptrace-get-syscall-info)
  - [`PTRACE_SYSCALL_INFO_NONE`](#ptrace-syscall-info-none)
  - [`PTRACE_SYSCALL_INFO_ENTRY`](#ptrace-syscall-info-entry)
  - [`PTRACE_SYSCALL_INFO_EXIT`](#ptrace-syscall-info-exit)
  - [`PTRACE_SYSCALL_INFO_SECCOMP`](#ptrace-syscall-info-seccomp)
  - [`PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-set-syscall-user-dispatch-config)
  - [`PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-get-syscall-user-dispatch-config)
  - [`TCA_PAD`](#tca-pad)
  - [`TCA_DUMP_INVISIBLE`](#tca-dump-invisible)
  - [`TCA_CHAIN`](#tca-chain)
  - [`TCA_HW_OFFLOAD`](#tca-hw-offload)
  - [`RTM_DELNETCONF`](#rtm-delnetconf)
  - [`RTM_NEWSTATS`](#rtm-newstats)
  - [`RTM_GETSTATS`](#rtm-getstats)
  - [`RTM_NEWCACHEREPORT`](#rtm-newcachereport)
  - [`RTM_F_LOOKUP_TABLE`](#rtm-f-lookup-table)
  - [`RTM_F_FIB_MATCH`](#rtm-f-fib-match)
  - [`RTA_VIA`](#rta-via)
  - [`RTA_NEWDST`](#rta-newdst)
  - [`RTA_PREF`](#rta-pref)
  - [`RTA_ENCAP_TYPE`](#rta-encap-type)
  - [`RTA_ENCAP`](#rta-encap)
  - [`RTA_EXPIRES`](#rta-expires)
  - [`RTA_PAD`](#rta-pad)
  - [`RTA_UID`](#rta-uid)
  - [`RTA_TTL_PROPAGATE`](#rta-ttl-propagate)
  - [`NTF_EXT_LEARNED`](#ntf-ext-learned)
  - [`NTF_OFFLOADED`](#ntf-offloaded)
  - [`NDA_MASTER`](#nda-master)
  - [`NDA_LINK_NETNSID`](#nda-link-netnsid)
  - [`NDA_SRC_VNI`](#nda-src-vni)
  - [`UNAME26`](#uname26)
  - [`FDPIC_FUNCPTRS`](#fdpic-funcptrs)
  - [`MAX_LINKS`](#max-links)
  - [`GENL_UNS_ADMIN_PERM`](#genl-uns-admin-perm)
  - [`GENL_ID_VFS_DQUOT`](#genl-id-vfs-dquot)
  - [`GENL_ID_PMCRAID`](#genl-id-pmcraid)
  - [`ELFOSABI_ARM_AEABI`](#elfosabi-arm-aeabi)
  - [`CLONE_NEWTIME`](#clone-newtime)
  - [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand)
  - [`CLONE_INTO_CGROUP`](#clone-into-cgroup)
  - [`M_MXFAST`](#m-mxfast)
  - [`M_NLBLKS`](#m-nlblks)
  - [`M_GRAIN`](#m-grain)
  - [`M_KEEP`](#m-keep)
  - [`M_TRIM_THRESHOLD`](#m-trim-threshold)
  - [`M_TOP_PAD`](#m-top-pad)
  - [`M_MMAP_THRESHOLD`](#m-mmap-threshold)
  - [`M_MMAP_MAX`](#m-mmap-max)
  - [`M_CHECK_ACTION`](#m-check-action)
  - [`M_PERTURB`](#m-perturb)
  - [`M_ARENA_TEST`](#m-arena-test)
  - [`M_ARENA_MAX`](#m-arena-max)
  - [`SOMAXCONN`](#somaxconn)
  - [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks)
  - [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts)
  - [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path)
  - [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks)
  - [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts)
  - [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path)
  - [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group)
  - [`MOVE_MOUNT_BENEATH`](#move-mount-beneath)
  - [`ADJ_OFFSET`](#adj-offset)
  - [`ADJ_FREQUENCY`](#adj-frequency)
  - [`ADJ_MAXERROR`](#adj-maxerror)
  - [`ADJ_ESTERROR`](#adj-esterror)
  - [`ADJ_STATUS`](#adj-status)
  - [`ADJ_TIMECONST`](#adj-timeconst)
  - [`ADJ_TAI`](#adj-tai)
  - [`ADJ_SETOFFSET`](#adj-setoffset)
  - [`ADJ_MICRO`](#adj-micro)
  - [`ADJ_NANO`](#adj-nano)
  - [`ADJ_TICK`](#adj-tick)
  - [`ADJ_OFFSET_SINGLESHOT`](#adj-offset-singleshot)
  - [`ADJ_OFFSET_SS_READ`](#adj-offset-ss-read)
  - [`MOD_OFFSET`](#mod-offset)
  - [`MOD_FREQUENCY`](#mod-frequency)
  - [`MOD_MAXERROR`](#mod-maxerror)
  - [`MOD_ESTERROR`](#mod-esterror)
  - [`MOD_STATUS`](#mod-status)
  - [`MOD_TIMECONST`](#mod-timeconst)
  - [`MOD_CLKB`](#mod-clkb)
  - [`MOD_CLKA`](#mod-clka)
  - [`MOD_TAI`](#mod-tai)
  - [`MOD_MICRO`](#mod-micro)
  - [`MOD_NANO`](#mod-nano)
  - [`STA_PLL`](#sta-pll)
  - [`STA_PPSFREQ`](#sta-ppsfreq)
  - [`STA_PPSTIME`](#sta-ppstime)
  - [`STA_FLL`](#sta-fll)
  - [`STA_INS`](#sta-ins)
  - [`STA_DEL`](#sta-del)
  - [`STA_UNSYNC`](#sta-unsync)
  - [`STA_FREQHOLD`](#sta-freqhold)
  - [`STA_PPSSIGNAL`](#sta-ppssignal)
  - [`STA_PPSJITTER`](#sta-ppsjitter)
  - [`STA_PPSWANDER`](#sta-ppswander)
  - [`STA_PPSERROR`](#sta-ppserror)
  - [`STA_CLOCKERR`](#sta-clockerr)
  - [`STA_NANO`](#sta-nano)
  - [`STA_MODE`](#sta-mode)
  - [`STA_CLK`](#sta-clk)
  - [`STA_RONLY`](#sta-ronly)
  - [`NTP_API`](#ntp-api)
  - [`TIME_OK`](#time-ok)
  - [`TIME_INS`](#time-ins)
  - [`TIME_DEL`](#time-del)
  - [`TIME_OOP`](#time-oop)
  - [`TIME_WAIT`](#time-wait)
  - [`TIME_ERROR`](#time-error)
  - [`TIME_BAD`](#time-bad)
  - [`MAXTC`](#maxtc)
  - [`GLOB_PERIOD`](#glob-period)
  - [`GLOB_ALTDIRFUNC`](#glob-altdirfunc)
  - [`GLOB_BRACE`](#glob-brace)
  - [`GLOB_NOMAGIC`](#glob-nomagic)
  - [`GLOB_TILDE`](#glob-tilde)
  - [`GLOB_ONLYDIR`](#glob-onlydir)
  - [`GLOB_TILDE_CHECK`](#glob-tilde-check)
  - [`MADV_COLLAPSE`](#madv-collapse)
  - [`PTHREAD_STACK_MIN`](#pthread-stack-min)
  - [`PTHREAD_MUTEX_ADAPTIVE_NP`](#pthread-mutex-adaptive-np)
  - [`REG_STARTEND`](#reg-startend)
  - [`REG_EEND`](#reg-eend)
  - [`REG_ESIZE`](#reg-esize)
  - [`REG_ERPAREN`](#reg-erparen)
  - [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t)
  - [`O_LARGEFILE`](#o-largefile)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`b64`](#b64) | mod | 64-bit specific definitions for linux-like values |
| [`x86_64`](#x86-64) | mod | x86_64-specific definitions for 64-bit linux-like values |
| [`aiocb`](#aiocb) | struct |  |
| [`__exit_status`](#exit-status) | struct |  |
| [`__timeval`](#timeval) | struct |  |
| [`glob64_t`](#glob64-t) | struct |  |
| [`msghdr`](#msghdr) | struct |  |
| [`cmsghdr`](#cmsghdr) | struct |  |
| [`termios`](#termios) | struct |  |
| [`mallinfo`](#mallinfo) | struct |  |
| [`mallinfo2`](#mallinfo2) | struct |  |
| [`nl_pktinfo`](#nl-pktinfo) | struct |  |
| [`nl_mmap_req`](#nl-mmap-req) | struct |  |
| [`nl_mmap_hdr`](#nl-mmap-hdr) | struct |  |
| [`ntptimeval`](#ntptimeval) | struct |  |
| [`regex_t`](#regex-t) | struct |  |
| [`Elf64_Chdr`](#elf64-chdr) | struct |  |
| [`Elf32_Chdr`](#elf32-chdr) | struct |  |
| [`seminfo`](#seminfo) | struct |  |
| [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp) | struct |  |
| [`ptrace_syscall_info`](#ptrace-syscall-info) | struct |  |
| [`ptrace_sud_config`](#ptrace-sud-config) | struct |  |
| [`iocb`](#iocb) | struct |  |
| [`tcp_info`](#tcp-info) | struct |  |
| [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd) | struct |  |
| [`fanotify_event_info_error`](#fanotify-event-info-error) | struct |  |
| [`sem_t`](#sem-t) | struct |  |
| [`mbstate_t`](#mbstate-t) | struct |  |
| [`fpos64_t`](#fpos64-t) | struct |  |
| [`fpos_t`](#fpos-t) | struct |  |
| [`timespec`](#timespec) | struct |  |
| [`utmpx`](#utmpx) | struct |  |
| [`sifields_sigchld`](#sifields-sigchld) | struct |  |
| [`siginfo_f`](#siginfo-f) | struct |  |
| [`sigset_t`](#sigset-t) | struct |  |
| [`sysinfo`](#sysinfo) | struct |  |
| [`msqid_ds`](#msqid-ds) | struct |  |
| [`semid_ds`](#semid-ds) | struct |  |
| [`timex`](#timex) | struct |  |
| [`fgetspent_r`](#fgetspent-r) | fn |  |
| [`sgetspent_r`](#sgetspent-r) | fn |  |
| [`getspent_r`](#getspent-r) | fn |  |
| [`qsort_r`](#qsort-r) | fn |  |
| [`sendmmsg`](#sendmmsg) | fn |  |
| [`recvmmsg`](#recvmmsg) | fn |  |
| [`getrlimit64`](#getrlimit64) | fn |  |
| [`setrlimit64`](#setrlimit64) | fn |  |
| [`getrlimit`](#getrlimit) | fn |  |
| [`setrlimit`](#setrlimit) | fn |  |
| [`prlimit`](#prlimit) | fn |  |
| [`prlimit64`](#prlimit64) | fn |  |
| [`utmpname`](#utmpname) | fn |  |
| [`utmpxname`](#utmpxname) | fn |  |
| [`getutxent`](#getutxent) | fn |  |
| [`getutxid`](#getutxid) | fn |  |
| [`getutxline`](#getutxline) | fn |  |
| [`pututxline`](#pututxline) | fn |  |
| [`setutxent`](#setutxent) | fn |  |
| [`endutxent`](#endutxent) | fn |  |
| [`getpt`](#getpt) | fn |  |
| [`mallopt`](#mallopt) | fn |  |
| [`gettimeofday`](#gettimeofday) | fn |  |
| [`getentropy`](#getentropy) | fn |  |
| [`getrandom`](#getrandom) | fn |  |
| [`getauxval`](#getauxval) | fn |  |
| [`adjtimex`](#adjtimex) | fn |  |
| [`ntp_adjtime`](#ntp-adjtime) | fn |  |
| [`ntp_gettime`](#ntp-gettime) | fn |  |
| [`clock_adjtime`](#clock-adjtime) | fn |  |
| [`fanotify_mark`](#fanotify-mark) | fn |  |
| [`preadv2`](#preadv2) | fn |  |
| [`pwritev2`](#pwritev2) | fn |  |
| [`preadv64v2`](#preadv64v2) | fn |  |
| [`pwritev64v2`](#pwritev64v2) | fn |  |
| [`renameat2`](#renameat2) | fn |  |
| [`explicit_bzero`](#explicit-bzero) | fn |  |
| [`reallocarray`](#reallocarray) | fn |  |
| [`ctermid`](#ctermid) | fn |  |
| [`backtrace`](#backtrace) | fn |  |
| [`backtrace_symbols`](#backtrace-symbols) | fn |  |
| [`backtrace_symbols_fd`](#backtrace-symbols-fd) | fn |  |
| [`glob64`](#glob64) | fn |  |
| [`globfree64`](#globfree64) | fn |  |
| [`ptrace`](#ptrace) | fn |  |
| [`pthread_attr_getaffinity_np`](#pthread-attr-getaffinity-np) | fn |  |
| [`pthread_attr_setaffinity_np`](#pthread-attr-setaffinity-np) | fn |  |
| [`getpriority`](#getpriority) | fn |  |
| [`setpriority`](#setpriority) | fn |  |
| [`pthread_rwlockattr_getkind_np`](#pthread-rwlockattr-getkind-np) | fn |  |
| [`pthread_rwlockattr_setkind_np`](#pthread-rwlockattr-setkind-np) | fn |  |
| [`pthread_sigqueue`](#pthread-sigqueue) | fn |  |
| [`mallinfo`](#mallinfo) | fn |  |
| [`mallinfo2`](#mallinfo2) | fn |  |
| [`malloc_stats`](#malloc-stats) | fn |  |
| [`malloc_info`](#malloc-info) | fn |  |
| [`malloc_usable_size`](#malloc-usable-size) | fn |  |
| [`getpwent_r`](#getpwent-r) | fn |  |
| [`getgrent_r`](#getgrent-r) | fn |  |
| [`fgetpwent_r`](#fgetpwent-r) | fn |  |
| [`fgetgrent_r`](#fgetgrent-r) | fn |  |
| [`putpwent`](#putpwent) | fn |  |
| [`putgrent`](#putgrent) | fn |  |
| [`sethostid`](#sethostid) | fn |  |
| [`memfd_create`](#memfd-create) | fn |  |
| [`mlock2`](#mlock2) | fn |  |
| [`euidaccess`](#euidaccess) | fn |  |
| [`eaccess`](#eaccess) | fn |  |
| [`asctime_r`](#asctime-r) | fn |  |
| [`ctime_r`](#ctime-r) | fn |  |
| [`dirname`](#dirname) | fn |  |
| [`posix_basename`](#posix-basename) | fn | POSIX version of `basename(3)`, defined in `libgen.h`. |
| [`gnu_basename`](#gnu-basename) | fn | GNU version of `basename(3)`, defined in `string.h`. |
| [`dlmopen`](#dlmopen) | fn |  |
| [`dlinfo`](#dlinfo) | fn |  |
| [`dladdr1`](#dladdr1) | fn |  |
| [`dlvsym`](#dlvsym) | fn |  |
| [`malloc_trim`](#malloc-trim) | fn |  |
| [`gnu_get_libc_release`](#gnu-get-libc-release) | fn |  |
| [`gnu_get_libc_version`](#gnu-get-libc-version) | fn |  |
| [`posix_spawn_file_actions_addchdir_np`](#posix-spawn-file-actions-addchdir-np) | fn |  |
| [`posix_spawn_file_actions_addfchdir_np`](#posix-spawn-file-actions-addfchdir-np) | fn |  |
| [`posix_spawn_file_actions_addclosefrom_np`](#posix-spawn-file-actions-addclosefrom-np) | fn |  |
| [`posix_spawn_file_actions_addtcsetpgrp_np`](#posix-spawn-file-actions-addtcsetpgrp-np) | fn |  |
| [`getmntent_r`](#getmntent-r) | fn |  |
| [`execveat`](#execveat) | fn |  |
| [`close_range`](#close-range) | fn |  |
| [`mq_notify`](#mq-notify) | fn |  |
| [`epoll_pwait2`](#epoll-pwait2) | fn |  |
| [`mempcpy`](#mempcpy) | fn |  |
| [`pthread_t`](#pthread-t) | type |  |
| [`__priority_which_t`](#priority-which-t) | type |  |
| [`__rlimit_resource_t`](#rlimit-resource-t) | type |  |
| [`Lmid_t`](#lmid-t) | type |  |
| [`regoff_t`](#regoff-t) | type |  |
| [`__kernel_rwf_t`](#kernel-rwf-t) | type |  |
| [`Ioctl`](#ioctl) | type |  |
| [`ino_t`](#ino-t) | type |  |
| [`off_t`](#off-t) | type |  |
| [`blkcnt_t`](#blkcnt-t) | type |  |
| [`shmatt_t`](#shmatt-t) | type |  |
| [`msgqnum_t`](#msgqnum-t) | type |  |
| [`msglen_t`](#msglen-t) | type |  |
| [`fsblkcnt_t`](#fsblkcnt-t) | type |  |
| [`fsfilcnt_t`](#fsfilcnt-t) | type |  |
| [`rlim_t`](#rlim-t) | type |  |
| [`__syscall_ulong_t`](#syscall-ulong-t) | type |  |
| [`__fsword_t`](#fsword-t) | type |  |
| [`clock_t`](#clock-t) | type |  |
| [`time_t`](#time-t) | type |  |
| [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift) | const |  |
| [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask) | const |  |
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
| [`MAP_HUGE_SHIFT`](#map-huge-shift) | const |  |
| [`MAP_HUGE_MASK`](#map-huge-mask) | const |  |
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
| [`PRIO_PROCESS`](#prio-process) | const |  |
| [`PRIO_PGRP`](#prio-pgrp) | const |  |
| [`PRIO_USER`](#prio-user) | const |  |
| [`MS_RMT_MASK`](#ms-rmt-mask) | const |  |
| [`__UT_LINESIZE`](#ut-linesize) | const |  |
| [`__UT_NAMESIZE`](#ut-namesize) | const |  |
| [`__UT_HOSTSIZE`](#ut-hostsize) | const |  |
| [`EMPTY`](#empty) | const |  |
| [`RUN_LVL`](#run-lvl) | const |  |
| [`BOOT_TIME`](#boot-time) | const |  |
| [`NEW_TIME`](#new-time) | const |  |
| [`OLD_TIME`](#old-time) | const |  |
| [`INIT_PROCESS`](#init-process) | const |  |
| [`LOGIN_PROCESS`](#login-process) | const |  |
| [`USER_PROCESS`](#user-process) | const |  |
| [`DEAD_PROCESS`](#dead-process) | const |  |
| [`ACCOUNTING`](#accounting) | const |  |
| [`LM_ID_BASE`](#lm-id-base) | const |  |
| [`LM_ID_NEWLM`](#lm-id-newlm) | const |  |
| [`RTLD_DI_LMID`](#rtld-di-lmid) | const |  |
| [`RTLD_DI_LINKMAP`](#rtld-di-linkmap) | const |  |
| [`RTLD_DI_CONFIGADDR`](#rtld-di-configaddr) | const |  |
| [`RTLD_DI_SERINFO`](#rtld-di-serinfo) | const |  |
| [`RTLD_DI_SERINFOSIZE`](#rtld-di-serinfosize) | const |  |
| [`RTLD_DI_ORIGIN`](#rtld-di-origin) | const |  |
| [`RTLD_DI_PROFILENAME`](#rtld-di-profilename) | const |  |
| [`RTLD_DI_PROFILEOUT`](#rtld-di-profileout) | const |  |
| [`RTLD_DI_TLS_MODID`](#rtld-di-tls-modid) | const |  |
| [`RTLD_DI_TLS_DATA`](#rtld-di-tls-data) | const |  |
| [`SOCK_NONBLOCK`](#sock-nonblock) | const |  |
| [`SOL_RXRPC`](#sol-rxrpc) | const |  |
| [`SOL_PPPOL2TP`](#sol-pppol2tp) | const |  |
| [`SOL_PNPIPE`](#sol-pnpipe) | const |  |
| [`SOL_RDS`](#sol-rds) | const |  |
| [`SOL_IUCV`](#sol-iucv) | const |  |
| [`SOL_CAIF`](#sol-caif) | const |  |
| [`SOL_NFC`](#sol-nfc) | const |  |
| [`MSG_TRYHARD`](#msg-tryhard) | const |  |
| [`LC_PAPER`](#lc-paper) | const |  |
| [`LC_NAME`](#lc-name) | const |  |
| [`LC_ADDRESS`](#lc-address) | const |  |
| [`LC_TELEPHONE`](#lc-telephone) | const |  |
| [`LC_MEASUREMENT`](#lc-measurement) | const |  |
| [`LC_IDENTIFICATION`](#lc-identification) | const |  |
| [`LC_PAPER_MASK`](#lc-paper-mask) | const |  |
| [`LC_NAME_MASK`](#lc-name-mask) | const |  |
| [`LC_ADDRESS_MASK`](#lc-address-mask) | const |  |
| [`LC_TELEPHONE_MASK`](#lc-telephone-mask) | const |  |
| [`LC_MEASUREMENT_MASK`](#lc-measurement-mask) | const |  |
| [`LC_IDENTIFICATION_MASK`](#lc-identification-mask) | const |  |
| [`LC_ALL_MASK`](#lc-all-mask) | const |  |
| [`ENOTSUP`](#enotsup) | const |  |
| [`SOCK_SEQPACKET`](#sock-seqpacket) | const |  |
| [`SOCK_DCCP`](#sock-dccp) | const |  |
| [`SOCK_PACKET`](#sock-packet) | const |  |
| [`AF_IB`](#af-ib) | const |  |
| [`AF_MPLS`](#af-mpls) | const |  |
| [`AF_NFC`](#af-nfc) | const |  |
| [`AF_VSOCK`](#af-vsock) | const |  |
| [`AF_XDP`](#af-xdp) | const |  |
| [`PF_IB`](#pf-ib) | const |  |
| [`PF_MPLS`](#pf-mpls) | const |  |
| [`PF_NFC`](#pf-nfc) | const |  |
| [`PF_VSOCK`](#pf-vsock) | const |  |
| [`PF_XDP`](#pf-xdp) | const |  |
| [`SIGEV_THREAD_ID`](#sigev-thread-id) | const |  |
| [`BUFSIZ`](#bufsiz) | const |  |
| [`TMP_MAX`](#tmp-max) | const |  |
| [`FOPEN_MAX`](#fopen-max) | const |  |
| [`FILENAME_MAX`](#filename-max) | const |  |
| [`POSIX_MADV_DONTNEED`](#posix-madv-dontneed) | const |  |
| [`_CS_GNU_LIBC_VERSION`](#cs-gnu-libc-version) | const |  |
| [`_CS_GNU_LIBPTHREAD_VERSION`](#cs-gnu-libpthread-version) | const |  |
| [`_CS_V6_ENV`](#cs-v6-env) | const |  |
| [`_CS_V7_ENV`](#cs-v7-env) | const |  |
| [`_SC_EQUIV_CLASS_MAX`](#sc-equiv-class-max) | const |  |
| [`_SC_CHARCLASS_NAME_MAX`](#sc-charclass-name-max) | const |  |
| [`_SC_PII`](#sc-pii) | const |  |
| [`_SC_PII_XTI`](#sc-pii-xti) | const |  |
| [`_SC_PII_SOCKET`](#sc-pii-socket) | const |  |
| [`_SC_PII_INTERNET`](#sc-pii-internet) | const |  |
| [`_SC_PII_OSI`](#sc-pii-osi) | const |  |
| [`_SC_POLL`](#sc-poll) | const |  |
| [`_SC_SELECT`](#sc-select) | const |  |
| [`_SC_PII_INTERNET_STREAM`](#sc-pii-internet-stream) | const |  |
| [`_SC_PII_INTERNET_DGRAM`](#sc-pii-internet-dgram) | const |  |
| [`_SC_PII_OSI_COTS`](#sc-pii-osi-cots) | const |  |
| [`_SC_PII_OSI_CLTS`](#sc-pii-osi-clts) | const |  |
| [`_SC_PII_OSI_M`](#sc-pii-osi-m) | const |  |
| [`_SC_T_IOV_MAX`](#sc-t-iov-max) | const |  |
| [`_SC_2_C_VERSION`](#sc-2-c-version) | const |  |
| [`_SC_CHAR_BIT`](#sc-char-bit) | const |  |
| [`_SC_CHAR_MAX`](#sc-char-max) | const |  |
| [`_SC_CHAR_MIN`](#sc-char-min) | const |  |
| [`_SC_INT_MAX`](#sc-int-max) | const |  |
| [`_SC_INT_MIN`](#sc-int-min) | const |  |
| [`_SC_LONG_BIT`](#sc-long-bit) | const |  |
| [`_SC_WORD_BIT`](#sc-word-bit) | const |  |
| [`_SC_MB_LEN_MAX`](#sc-mb-len-max) | const |  |
| [`_SC_SSIZE_MAX`](#sc-ssize-max) | const |  |
| [`_SC_SCHAR_MAX`](#sc-schar-max) | const |  |
| [`_SC_SCHAR_MIN`](#sc-schar-min) | const |  |
| [`_SC_SHRT_MAX`](#sc-shrt-max) | const |  |
| [`_SC_SHRT_MIN`](#sc-shrt-min) | const |  |
| [`_SC_UCHAR_MAX`](#sc-uchar-max) | const |  |
| [`_SC_UINT_MAX`](#sc-uint-max) | const |  |
| [`_SC_ULONG_MAX`](#sc-ulong-max) | const |  |
| [`_SC_USHRT_MAX`](#sc-ushrt-max) | const |  |
| [`_SC_NL_ARGMAX`](#sc-nl-argmax) | const |  |
| [`_SC_NL_LANGMAX`](#sc-nl-langmax) | const |  |
| [`_SC_NL_MSGMAX`](#sc-nl-msgmax) | const |  |
| [`_SC_NL_NMAX`](#sc-nl-nmax) | const |  |
| [`_SC_NL_SETMAX`](#sc-nl-setmax) | const |  |
| [`_SC_NL_TEXTMAX`](#sc-nl-textmax) | const |  |
| [`_SC_BASE`](#sc-base) | const |  |
| [`_SC_C_LANG_SUPPORT`](#sc-c-lang-support) | const |  |
| [`_SC_C_LANG_SUPPORT_R`](#sc-c-lang-support-r) | const |  |
| [`_SC_DEVICE_IO`](#sc-device-io) | const |  |
| [`_SC_DEVICE_SPECIFIC`](#sc-device-specific) | const |  |
| [`_SC_DEVICE_SPECIFIC_R`](#sc-device-specific-r) | const |  |
| [`_SC_FD_MGMT`](#sc-fd-mgmt) | const |  |
| [`_SC_FIFO`](#sc-fifo) | const |  |
| [`_SC_PIPE`](#sc-pipe) | const |  |
| [`_SC_FILE_ATTRIBUTES`](#sc-file-attributes) | const |  |
| [`_SC_FILE_LOCKING`](#sc-file-locking) | const |  |
| [`_SC_FILE_SYSTEM`](#sc-file-system) | const |  |
| [`_SC_MULTI_PROCESS`](#sc-multi-process) | const |  |
| [`_SC_SINGLE_PROCESS`](#sc-single-process) | const |  |
| [`_SC_NETWORKING`](#sc-networking) | const |  |
| [`_SC_REGEX_VERSION`](#sc-regex-version) | const |  |
| [`_SC_SIGNALS`](#sc-signals) | const |  |
| [`_SC_SYSTEM_DATABASE`](#sc-system-database) | const |  |
| [`_SC_SYSTEM_DATABASE_R`](#sc-system-database-r) | const |  |
| [`_SC_USER_GROUPS`](#sc-user-groups) | const |  |
| [`_SC_USER_GROUPS_R`](#sc-user-groups-r) | const |  |
| [`_SC_LEVEL1_ICACHE_SIZE`](#sc-level1-icache-size) | const |  |
| [`_SC_LEVEL1_ICACHE_ASSOC`](#sc-level1-icache-assoc) | const |  |
| [`_SC_LEVEL1_ICACHE_LINESIZE`](#sc-level1-icache-linesize) | const |  |
| [`_SC_LEVEL1_DCACHE_SIZE`](#sc-level1-dcache-size) | const |  |
| [`_SC_LEVEL1_DCACHE_ASSOC`](#sc-level1-dcache-assoc) | const |  |
| [`_SC_LEVEL1_DCACHE_LINESIZE`](#sc-level1-dcache-linesize) | const |  |
| [`_SC_LEVEL2_CACHE_SIZE`](#sc-level2-cache-size) | const |  |
| [`_SC_LEVEL2_CACHE_ASSOC`](#sc-level2-cache-assoc) | const |  |
| [`_SC_LEVEL2_CACHE_LINESIZE`](#sc-level2-cache-linesize) | const |  |
| [`_SC_LEVEL3_CACHE_SIZE`](#sc-level3-cache-size) | const |  |
| [`_SC_LEVEL3_CACHE_ASSOC`](#sc-level3-cache-assoc) | const |  |
| [`_SC_LEVEL3_CACHE_LINESIZE`](#sc-level3-cache-linesize) | const |  |
| [`_SC_LEVEL4_CACHE_SIZE`](#sc-level4-cache-size) | const |  |
| [`_SC_LEVEL4_CACHE_ASSOC`](#sc-level4-cache-assoc) | const |  |
| [`_SC_LEVEL4_CACHE_LINESIZE`](#sc-level4-cache-linesize) | const |  |
| [`O_ACCMODE`](#o-accmode) | const |  |
| [`ST_RELATIME`](#st-relatime) | const |  |
| [`NI_MAXHOST`](#ni-maxhost) | const |  |
| [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic) | const |  |
| [`XFS_SUPER_MAGIC`](#xfs-super-magic) | const |  |
| [`CPU_SETSIZE`](#cpu-setsize) | const |  |
| [`PTRACE_TRACEME`](#ptrace-traceme) | const |  |
| [`PTRACE_PEEKTEXT`](#ptrace-peektext) | const |  |
| [`PTRACE_PEEKDATA`](#ptrace-peekdata) | const |  |
| [`PTRACE_PEEKUSER`](#ptrace-peekuser) | const |  |
| [`PTRACE_POKETEXT`](#ptrace-poketext) | const |  |
| [`PTRACE_POKEDATA`](#ptrace-pokedata) | const |  |
| [`PTRACE_POKEUSER`](#ptrace-pokeuser) | const |  |
| [`PTRACE_CONT`](#ptrace-cont) | const |  |
| [`PTRACE_KILL`](#ptrace-kill) | const |  |
| [`PTRACE_SINGLESTEP`](#ptrace-singlestep) | const |  |
| [`PTRACE_ATTACH`](#ptrace-attach) | const |  |
| [`PTRACE_SYSCALL`](#ptrace-syscall) | const |  |
| [`PTRACE_SETOPTIONS`](#ptrace-setoptions) | const |  |
| [`PTRACE_GETEVENTMSG`](#ptrace-geteventmsg) | const |  |
| [`PTRACE_GETSIGINFO`](#ptrace-getsiginfo) | const |  |
| [`PTRACE_SETSIGINFO`](#ptrace-setsiginfo) | const |  |
| [`PTRACE_GETREGSET`](#ptrace-getregset) | const |  |
| [`PTRACE_SETREGSET`](#ptrace-setregset) | const |  |
| [`PTRACE_SEIZE`](#ptrace-seize) | const |  |
| [`PTRACE_INTERRUPT`](#ptrace-interrupt) | const |  |
| [`PTRACE_LISTEN`](#ptrace-listen) | const |  |
| [`PTRACE_PEEKSIGINFO`](#ptrace-peeksiginfo) | const |  |
| [`PTRACE_GETSIGMASK`](#ptrace-getsigmask) | const |  |
| [`PTRACE_SETSIGMASK`](#ptrace-setsigmask) | const |  |
| [`PTRACE_GET_SYSCALL_INFO`](#ptrace-get-syscall-info) | const |  |
| [`PTRACE_SYSCALL_INFO_NONE`](#ptrace-syscall-info-none) | const |  |
| [`PTRACE_SYSCALL_INFO_ENTRY`](#ptrace-syscall-info-entry) | const |  |
| [`PTRACE_SYSCALL_INFO_EXIT`](#ptrace-syscall-info-exit) | const |  |
| [`PTRACE_SYSCALL_INFO_SECCOMP`](#ptrace-syscall-info-seccomp) | const |  |
| [`PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-set-syscall-user-dispatch-config) | const |  |
| [`PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-get-syscall-user-dispatch-config) | const |  |
| [`TCA_PAD`](#tca-pad) | const |  |
| [`TCA_DUMP_INVISIBLE`](#tca-dump-invisible) | const |  |
| [`TCA_CHAIN`](#tca-chain) | const |  |
| [`TCA_HW_OFFLOAD`](#tca-hw-offload) | const |  |
| [`RTM_DELNETCONF`](#rtm-delnetconf) | const |  |
| [`RTM_NEWSTATS`](#rtm-newstats) | const |  |
| [`RTM_GETSTATS`](#rtm-getstats) | const |  |
| [`RTM_NEWCACHEREPORT`](#rtm-newcachereport) | const |  |
| [`RTM_F_LOOKUP_TABLE`](#rtm-f-lookup-table) | const |  |
| [`RTM_F_FIB_MATCH`](#rtm-f-fib-match) | const |  |
| [`RTA_VIA`](#rta-via) | const |  |
| [`RTA_NEWDST`](#rta-newdst) | const |  |
| [`RTA_PREF`](#rta-pref) | const |  |
| [`RTA_ENCAP_TYPE`](#rta-encap-type) | const |  |
| [`RTA_ENCAP`](#rta-encap) | const |  |
| [`RTA_EXPIRES`](#rta-expires) | const |  |
| [`RTA_PAD`](#rta-pad) | const |  |
| [`RTA_UID`](#rta-uid) | const |  |
| [`RTA_TTL_PROPAGATE`](#rta-ttl-propagate) | const |  |
| [`NTF_EXT_LEARNED`](#ntf-ext-learned) | const |  |
| [`NTF_OFFLOADED`](#ntf-offloaded) | const |  |
| [`NDA_MASTER`](#nda-master) | const |  |
| [`NDA_LINK_NETNSID`](#nda-link-netnsid) | const |  |
| [`NDA_SRC_VNI`](#nda-src-vni) | const |  |
| [`UNAME26`](#uname26) | const |  |
| [`FDPIC_FUNCPTRS`](#fdpic-funcptrs) | const |  |
| [`MAX_LINKS`](#max-links) | const |  |
| [`GENL_UNS_ADMIN_PERM`](#genl-uns-admin-perm) | const |  |
| [`GENL_ID_VFS_DQUOT`](#genl-id-vfs-dquot) | const |  |
| [`GENL_ID_PMCRAID`](#genl-id-pmcraid) | const |  |
| [`ELFOSABI_ARM_AEABI`](#elfosabi-arm-aeabi) | const |  |
| [`CLONE_NEWTIME`](#clone-newtime) | const |  |
| [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand) | const |  |
| [`CLONE_INTO_CGROUP`](#clone-into-cgroup) | const |  |
| [`M_MXFAST`](#m-mxfast) | const |  |
| [`M_NLBLKS`](#m-nlblks) | const |  |
| [`M_GRAIN`](#m-grain) | const |  |
| [`M_KEEP`](#m-keep) | const |  |
| [`M_TRIM_THRESHOLD`](#m-trim-threshold) | const |  |
| [`M_TOP_PAD`](#m-top-pad) | const |  |
| [`M_MMAP_THRESHOLD`](#m-mmap-threshold) | const |  |
| [`M_MMAP_MAX`](#m-mmap-max) | const |  |
| [`M_CHECK_ACTION`](#m-check-action) | const |  |
| [`M_PERTURB`](#m-perturb) | const |  |
| [`M_ARENA_TEST`](#m-arena-test) | const |  |
| [`M_ARENA_MAX`](#m-arena-max) | const |  |
| [`SOMAXCONN`](#somaxconn) | const |  |
| [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks) | const |  |
| [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts) | const |  |
| [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path) | const |  |
| [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks) | const |  |
| [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts) | const |  |
| [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path) | const |  |
| [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group) | const |  |
| [`MOVE_MOUNT_BENEATH`](#move-mount-beneath) | const |  |
| [`ADJ_OFFSET`](#adj-offset) | const |  |
| [`ADJ_FREQUENCY`](#adj-frequency) | const |  |
| [`ADJ_MAXERROR`](#adj-maxerror) | const |  |
| [`ADJ_ESTERROR`](#adj-esterror) | const |  |
| [`ADJ_STATUS`](#adj-status) | const |  |
| [`ADJ_TIMECONST`](#adj-timeconst) | const |  |
| [`ADJ_TAI`](#adj-tai) | const |  |
| [`ADJ_SETOFFSET`](#adj-setoffset) | const |  |
| [`ADJ_MICRO`](#adj-micro) | const |  |
| [`ADJ_NANO`](#adj-nano) | const |  |
| [`ADJ_TICK`](#adj-tick) | const |  |
| [`ADJ_OFFSET_SINGLESHOT`](#adj-offset-singleshot) | const |  |
| [`ADJ_OFFSET_SS_READ`](#adj-offset-ss-read) | const |  |
| [`MOD_OFFSET`](#mod-offset) | const |  |
| [`MOD_FREQUENCY`](#mod-frequency) | const |  |
| [`MOD_MAXERROR`](#mod-maxerror) | const |  |
| [`MOD_ESTERROR`](#mod-esterror) | const |  |
| [`MOD_STATUS`](#mod-status) | const |  |
| [`MOD_TIMECONST`](#mod-timeconst) | const |  |
| [`MOD_CLKB`](#mod-clkb) | const |  |
| [`MOD_CLKA`](#mod-clka) | const |  |
| [`MOD_TAI`](#mod-tai) | const |  |
| [`MOD_MICRO`](#mod-micro) | const |  |
| [`MOD_NANO`](#mod-nano) | const |  |
| [`STA_PLL`](#sta-pll) | const |  |
| [`STA_PPSFREQ`](#sta-ppsfreq) | const |  |
| [`STA_PPSTIME`](#sta-ppstime) | const |  |
| [`STA_FLL`](#sta-fll) | const |  |
| [`STA_INS`](#sta-ins) | const |  |
| [`STA_DEL`](#sta-del) | const |  |
| [`STA_UNSYNC`](#sta-unsync) | const |  |
| [`STA_FREQHOLD`](#sta-freqhold) | const |  |
| [`STA_PPSSIGNAL`](#sta-ppssignal) | const |  |
| [`STA_PPSJITTER`](#sta-ppsjitter) | const |  |
| [`STA_PPSWANDER`](#sta-ppswander) | const |  |
| [`STA_PPSERROR`](#sta-ppserror) | const |  |
| [`STA_CLOCKERR`](#sta-clockerr) | const |  |
| [`STA_NANO`](#sta-nano) | const |  |
| [`STA_MODE`](#sta-mode) | const |  |
| [`STA_CLK`](#sta-clk) | const |  |
| [`STA_RONLY`](#sta-ronly) | const |  |
| [`NTP_API`](#ntp-api) | const |  |
| [`TIME_OK`](#time-ok) | const |  |
| [`TIME_INS`](#time-ins) | const |  |
| [`TIME_DEL`](#time-del) | const |  |
| [`TIME_OOP`](#time-oop) | const |  |
| [`TIME_WAIT`](#time-wait) | const |  |
| [`TIME_ERROR`](#time-error) | const |  |
| [`TIME_BAD`](#time-bad) | const |  |
| [`MAXTC`](#maxtc) | const |  |
| [`GLOB_PERIOD`](#glob-period) | const |  |
| [`GLOB_ALTDIRFUNC`](#glob-altdirfunc) | const |  |
| [`GLOB_BRACE`](#glob-brace) | const |  |
| [`GLOB_NOMAGIC`](#glob-nomagic) | const |  |
| [`GLOB_TILDE`](#glob-tilde) | const |  |
| [`GLOB_ONLYDIR`](#glob-onlydir) | const |  |
| [`GLOB_TILDE_CHECK`](#glob-tilde-check) | const |  |
| [`MADV_COLLAPSE`](#madv-collapse) | const |  |
| [`PTHREAD_STACK_MIN`](#pthread-stack-min) | const |  |
| [`PTHREAD_MUTEX_ADAPTIVE_NP`](#pthread-mutex-adaptive-np) | const |  |
| [`REG_STARTEND`](#reg-startend) | const |  |
| [`REG_EEND`](#reg-eend) | const |  |
| [`REG_ESIZE`](#reg-esize) | const |  |
| [`REG_ERPAREN`](#reg-erparen) | const |  |
| [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t) | const |  |
| [`O_LARGEFILE`](#o-largefile) | const |  |

## Modules

- [`b64`](b64/index.md) — 64-bit specific definitions for linux-like values
- [`x86_64`](x86_64/index.md) — x86_64-specific definitions for 64-bit linux-like values

## Structs

### `aiocb`

```rust
struct aiocb {
    pub aio_fildes: crate::c_int,
    pub aio_lio_opcode: crate::c_int,
    pub aio_reqprio: crate::c_int,
    pub aio_buf: *mut crate::c_void,
    pub aio_nbytes: crate::size_t,
    pub aio_sigevent: crate::sigevent,
    __next_prio: *mut aiocb,
    __abs_prio: crate::c_int,
    __policy: crate::c_int,
    __error_code: crate::c_int,
    __return_value: crate::ssize_t,
    pub aio_offset: off_t,
    __glibc_reserved: [crate::c_char; 32],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for aiocb`

- <span id="aiocb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for aiocb`

- <span id="aiocb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for aiocb`

- <span id="aiocb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for aiocb`

- <span id="aiocb-clone"></span>`fn clone(&self) -> aiocb` — [`aiocb`](../index.md#aiocb)

##### `impl CloneToUninit for aiocb`

- <span id="aiocb-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for aiocb`

##### `impl Debug for aiocb`

- <span id="aiocb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for aiocb`

- <span id="aiocb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for aiocb`

- <span id="aiocb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for aiocb`

- <span id="aiocb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aiocb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for aiocb`

- <span id="aiocb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aiocb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__exit_status`

```rust
struct __exit_status {
    pub e_termination: crate::c_short,
    pub e_exit: crate::c_short,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for __exit_status`

- <span id="exit-status-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __exit_status`

- <span id="exit-status-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __exit_status`

- <span id="exit-status-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __exit_status`

- <span id="exit-status-clone"></span>`fn clone(&self) -> __exit_status` — [`__exit_status`](../index.md#exit-status)

##### `impl CloneToUninit for __exit_status`

- <span id="exit-status-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __exit_status`

##### `impl Debug for __exit_status`

- <span id="exit-status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __exit_status`

- <span id="exit-status-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __exit_status`

- <span id="exit-status-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __exit_status`

- <span id="exit-status-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exit-status-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __exit_status`

- <span id="exit-status-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exit-status-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__timeval`

```rust
struct __timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for __timeval`

- <span id="timeval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __timeval`

- <span id="timeval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __timeval`

- <span id="timeval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> __timeval` — [`__timeval`](../index.md#timeval)

##### `impl CloneToUninit for __timeval`

- <span id="timeval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __timeval`

##### `impl Debug for __timeval`

- <span id="timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __timeval`

- <span id="timeval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __timeval`

- <span id="timeval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __timeval`

- <span id="timeval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timeval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __timeval`

- <span id="timeval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timeval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `glob64_t`

```rust
struct glob64_t {
    pub gl_pathc: crate::size_t,
    pub gl_pathv: *mut *mut crate::c_char,
    pub gl_offs: crate::size_t,
    pub gl_flags: crate::c_int,
    __unused1: crate::types::Padding<*mut crate::c_void>,
    __unused2: crate::types::Padding<*mut crate::c_void>,
    __unused3: crate::types::Padding<*mut crate::c_void>,
    __unused4: crate::types::Padding<*mut crate::c_void>,
    __unused5: crate::types::Padding<*mut crate::c_void>,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for glob64_t`

- <span id="glob64-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for glob64_t`

- <span id="glob64-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for glob64_t`

- <span id="glob64-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for glob64_t`

- <span id="glob64-t-clone"></span>`fn clone(&self) -> glob64_t` — [`glob64_t`](../index.md#glob64-t)

##### `impl CloneToUninit for glob64_t`

- <span id="glob64-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for glob64_t`

##### `impl Debug for glob64_t`

- <span id="glob64-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for glob64_t`

- <span id="glob64-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for glob64_t`

- <span id="glob64-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for glob64_t`

- <span id="glob64-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="glob64-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for glob64_t`

- <span id="glob64-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="glob64-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `msghdr`

```rust
struct msghdr {
    pub msg_name: *mut crate::c_void,
    pub msg_namelen: crate::socklen_t,
    pub msg_iov: *mut crate::iovec,
    pub msg_iovlen: crate::size_t,
    pub msg_control: *mut crate::c_void,
    pub msg_controllen: crate::size_t,
    pub msg_flags: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for msghdr`

- <span id="msghdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for msghdr`

- <span id="msghdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for msghdr`

- <span id="msghdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for msghdr`

- <span id="msghdr-clone"></span>`fn clone(&self) -> msghdr` — [`msghdr`](../index.md#msghdr)

##### `impl CloneToUninit for msghdr`

- <span id="msghdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for msghdr`

##### `impl Debug for msghdr`

- <span id="msghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for msghdr`

- <span id="msghdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for msghdr`

- <span id="msghdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for msghdr`

- <span id="msghdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="msghdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for msghdr`

- <span id="msghdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="msghdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `cmsghdr`

```rust
struct cmsghdr {
    pub cmsg_len: crate::size_t,
    pub cmsg_level: crate::c_int,
    pub cmsg_type: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for cmsghdr`

- <span id="cmsghdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for cmsghdr`

- <span id="cmsghdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for cmsghdr`

- <span id="cmsghdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for cmsghdr`

- <span id="cmsghdr-clone"></span>`fn clone(&self) -> cmsghdr` — [`cmsghdr`](../index.md#cmsghdr)

##### `impl CloneToUninit for cmsghdr`

- <span id="cmsghdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for cmsghdr`

##### `impl Debug for cmsghdr`

- <span id="cmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for cmsghdr`

- <span id="cmsghdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for cmsghdr`

- <span id="cmsghdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for cmsghdr`

- <span id="cmsghdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cmsghdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for cmsghdr`

- <span id="cmsghdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cmsghdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `termios`

```rust
struct termios {
    pub c_iflag: crate::tcflag_t,
    pub c_oflag: crate::tcflag_t,
    pub c_cflag: crate::tcflag_t,
    pub c_lflag: crate::tcflag_t,
    pub c_line: crate::cc_t,
    pub c_cc: [crate::cc_t; 32],
    pub c_ispeed: crate::speed_t,
    pub c_ospeed: crate::speed_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for termios`

- <span id="termios-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for termios`

- <span id="termios-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for termios`

- <span id="termios-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for termios`

- <span id="termios-clone"></span>`fn clone(&self) -> termios` — [`termios`](../index.md#termios)

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

### `mallinfo`

```rust
struct mallinfo {
    pub arena: crate::c_int,
    pub ordblks: crate::c_int,
    pub smblks: crate::c_int,
    pub hblks: crate::c_int,
    pub hblkhd: crate::c_int,
    pub usmblks: crate::c_int,
    pub fsmblks: crate::c_int,
    pub uordblks: crate::c_int,
    pub fordblks: crate::c_int,
    pub keepcost: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for mallinfo`

- <span id="mallinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for mallinfo`

- <span id="mallinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for mallinfo`

- <span id="mallinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for mallinfo`

- <span id="mallinfo-clone"></span>`fn clone(&self) -> mallinfo` — [`mallinfo`](../index.md#mallinfo)

##### `impl CloneToUninit for mallinfo`

- <span id="mallinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for mallinfo`

##### `impl Debug for mallinfo`

- <span id="mallinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for mallinfo`

- <span id="mallinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for mallinfo`

- <span id="mallinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for mallinfo`

- <span id="mallinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mallinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for mallinfo`

- <span id="mallinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mallinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `mallinfo2`

```rust
struct mallinfo2 {
    pub arena: crate::size_t,
    pub ordblks: crate::size_t,
    pub smblks: crate::size_t,
    pub hblks: crate::size_t,
    pub hblkhd: crate::size_t,
    pub usmblks: crate::size_t,
    pub fsmblks: crate::size_t,
    pub uordblks: crate::size_t,
    pub fordblks: crate::size_t,
    pub keepcost: crate::size_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for mallinfo2`

- <span id="mallinfo2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for mallinfo2`

- <span id="mallinfo2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for mallinfo2`

- <span id="mallinfo2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for mallinfo2`

- <span id="mallinfo2-clone"></span>`fn clone(&self) -> mallinfo2` — [`mallinfo2`](../index.md#mallinfo2)

##### `impl CloneToUninit for mallinfo2`

- <span id="mallinfo2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for mallinfo2`

##### `impl Debug for mallinfo2`

- <span id="mallinfo2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for mallinfo2`

- <span id="mallinfo2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for mallinfo2`

- <span id="mallinfo2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for mallinfo2`

- <span id="mallinfo2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mallinfo2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for mallinfo2`

- <span id="mallinfo2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mallinfo2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `nl_pktinfo`

```rust
struct nl_pktinfo {
    pub group: u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for nl_pktinfo`

- <span id="nl-pktinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for nl_pktinfo`

- <span id="nl-pktinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for nl_pktinfo`

- <span id="nl-pktinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for nl_pktinfo`

- <span id="nl-pktinfo-clone"></span>`fn clone(&self) -> nl_pktinfo` — [`nl_pktinfo`](../index.md#nl-pktinfo)

##### `impl CloneToUninit for nl_pktinfo`

- <span id="nl-pktinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for nl_pktinfo`

##### `impl Debug for nl_pktinfo`

- <span id="nl-pktinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for nl_pktinfo`

- <span id="nl-pktinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for nl_pktinfo`

- <span id="nl-pktinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for nl_pktinfo`

- <span id="nl-pktinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nl-pktinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for nl_pktinfo`

- <span id="nl-pktinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nl-pktinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `nl_mmap_req`

```rust
struct nl_mmap_req {
    pub nm_block_size: crate::c_uint,
    pub nm_block_nr: crate::c_uint,
    pub nm_frame_size: crate::c_uint,
    pub nm_frame_nr: crate::c_uint,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for nl_mmap_req`

- <span id="nl-mmap-req-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for nl_mmap_req`

- <span id="nl-mmap-req-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for nl_mmap_req`

- <span id="nl-mmap-req-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for nl_mmap_req`

- <span id="nl-mmap-req-clone"></span>`fn clone(&self) -> nl_mmap_req` — [`nl_mmap_req`](../index.md#nl-mmap-req)

##### `impl CloneToUninit for nl_mmap_req`

- <span id="nl-mmap-req-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for nl_mmap_req`

##### `impl Debug for nl_mmap_req`

- <span id="nl-mmap-req-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for nl_mmap_req`

- <span id="nl-mmap-req-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for nl_mmap_req`

- <span id="nl-mmap-req-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for nl_mmap_req`

- <span id="nl-mmap-req-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nl-mmap-req-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for nl_mmap_req`

- <span id="nl-mmap-req-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nl-mmap-req-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `nl_mmap_hdr`

```rust
struct nl_mmap_hdr {
    pub nm_status: crate::c_uint,
    pub nm_len: crate::c_uint,
    pub nm_group: u32,
    pub nm_pid: u32,
    pub nm_uid: u32,
    pub nm_gid: u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for nl_mmap_hdr`

- <span id="nl-mmap-hdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for nl_mmap_hdr`

- <span id="nl-mmap-hdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for nl_mmap_hdr`

- <span id="nl-mmap-hdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for nl_mmap_hdr`

- <span id="nl-mmap-hdr-clone"></span>`fn clone(&self) -> nl_mmap_hdr` — [`nl_mmap_hdr`](../index.md#nl-mmap-hdr)

##### `impl CloneToUninit for nl_mmap_hdr`

- <span id="nl-mmap-hdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for nl_mmap_hdr`

##### `impl Debug for nl_mmap_hdr`

- <span id="nl-mmap-hdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for nl_mmap_hdr`

- <span id="nl-mmap-hdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for nl_mmap_hdr`

- <span id="nl-mmap-hdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for nl_mmap_hdr`

- <span id="nl-mmap-hdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nl-mmap-hdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for nl_mmap_hdr`

- <span id="nl-mmap-hdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nl-mmap-hdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ntptimeval`

```rust
struct ntptimeval {
    pub time: crate::timeval,
    pub maxerror: crate::c_long,
    pub esterror: crate::c_long,
    pub tai: crate::c_long,
    pub __glibc_reserved1: crate::c_long,
    pub __glibc_reserved2: crate::c_long,
    pub __glibc_reserved3: crate::c_long,
    pub __glibc_reserved4: crate::c_long,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for ntptimeval`

- <span id="ntptimeval-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ntptimeval`

- <span id="ntptimeval-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ntptimeval`

- <span id="ntptimeval-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ntptimeval`

- <span id="ntptimeval-clone"></span>`fn clone(&self) -> ntptimeval` — [`ntptimeval`](../index.md#ntptimeval)

##### `impl CloneToUninit for ntptimeval`

- <span id="ntptimeval-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ntptimeval`

##### `impl Debug for ntptimeval`

- <span id="ntptimeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ntptimeval`

- <span id="ntptimeval-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ntptimeval`

- <span id="ntptimeval-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ntptimeval`

- <span id="ntptimeval-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ntptimeval-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ntptimeval`

- <span id="ntptimeval-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ntptimeval-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `regex_t`

```rust
struct regex_t {
    __buffer: *mut crate::c_void,
    __allocated: crate::size_t,
    __used: crate::size_t,
    __syntax: crate::c_ulong,
    __fastmap: *mut crate::c_char,
    __translate: *mut crate::c_char,
    __re_nsub: crate::size_t,
    __bitfield: u8,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for regex_t`

- <span id="regex-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for regex_t`

- <span id="regex-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for regex_t`

- <span id="regex-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for regex_t`

- <span id="regex-t-clone"></span>`fn clone(&self) -> regex_t` — [`regex_t`](../index.md#regex-t)

##### `impl CloneToUninit for regex_t`

- <span id="regex-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for regex_t`

##### `impl Debug for regex_t`

- <span id="regex-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for regex_t`

- <span id="regex-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for regex_t`

- <span id="regex-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for regex_t`

- <span id="regex-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regex-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for regex_t`

- <span id="regex-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regex-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Elf64_Chdr`

```rust
struct Elf64_Chdr {
    pub ch_type: crate::Elf64_Word,
    pub ch_reserved: crate::Elf64_Word,
    pub ch_size: crate::Elf64_Xword,
    pub ch_addralign: crate::Elf64_Xword,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for Elf64_Chdr`

- <span id="elf64-chdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Elf64_Chdr`

- <span id="elf64-chdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Elf64_Chdr`

- <span id="elf64-chdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Elf64_Chdr`

- <span id="elf64-chdr-clone"></span>`fn clone(&self) -> Elf64_Chdr` — [`Elf64_Chdr`](../index.md#elf64-chdr)

##### `impl CloneToUninit for Elf64_Chdr`

- <span id="elf64-chdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Elf64_Chdr`

##### `impl Debug for Elf64_Chdr`

- <span id="elf64-chdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Elf64_Chdr`

- <span id="elf64-chdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Elf64_Chdr`

- <span id="elf64-chdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Elf64_Chdr`

- <span id="elf64-chdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elf64-chdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Elf64_Chdr`

- <span id="elf64-chdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elf64-chdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Elf32_Chdr`

```rust
struct Elf32_Chdr {
    pub ch_type: crate::Elf32_Word,
    pub ch_size: crate::Elf32_Word,
    pub ch_addralign: crate::Elf32_Word,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for Elf32_Chdr`

- <span id="elf32-chdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Elf32_Chdr`

- <span id="elf32-chdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Elf32_Chdr`

- <span id="elf32-chdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Elf32_Chdr`

- <span id="elf32-chdr-clone"></span>`fn clone(&self) -> Elf32_Chdr` — [`Elf32_Chdr`](../index.md#elf32-chdr)

##### `impl CloneToUninit for Elf32_Chdr`

- <span id="elf32-chdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Elf32_Chdr`

##### `impl Debug for Elf32_Chdr`

- <span id="elf32-chdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Elf32_Chdr`

- <span id="elf32-chdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Elf32_Chdr`

- <span id="elf32-chdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Elf32_Chdr`

- <span id="elf32-chdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elf32-chdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Elf32_Chdr`

- <span id="elf32-chdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elf32-chdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `seminfo`

```rust
struct seminfo {
    pub semmap: crate::c_int,
    pub semmni: crate::c_int,
    pub semmns: crate::c_int,
    pub semmnu: crate::c_int,
    pub semmsl: crate::c_int,
    pub semopm: crate::c_int,
    pub semume: crate::c_int,
    pub semusz: crate::c_int,
    pub semvmx: crate::c_int,
    pub semaem: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for seminfo`

- <span id="seminfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for seminfo`

- <span id="seminfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for seminfo`

- <span id="seminfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for seminfo`

- <span id="seminfo-clone"></span>`fn clone(&self) -> seminfo` — [`seminfo`](../index.md#seminfo)

##### `impl CloneToUninit for seminfo`

- <span id="seminfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for seminfo`

##### `impl Debug for seminfo`

- <span id="seminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for seminfo`

- <span id="seminfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for seminfo`

- <span id="seminfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for seminfo`

- <span id="seminfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seminfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for seminfo`

- <span id="seminfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seminfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ptrace_peeksiginfo_args`

```rust
struct ptrace_peeksiginfo_args {
    pub off: crate::__u64,
    pub flags: crate::__u32,
    pub nr: crate::__s32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-clone"></span>`fn clone(&self) -> ptrace_peeksiginfo_args` — [`ptrace_peeksiginfo_args`](../index.md#ptrace-peeksiginfo-args)

##### `impl CloneToUninit for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ptrace_peeksiginfo_args`

##### `impl Debug for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ptrace-peeksiginfo-args-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ptrace-peeksiginfo-args-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__c_anonymous_ptrace_syscall_info_entry`

```rust
struct __c_anonymous_ptrace_syscall_info_entry {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_entry` — [`__c_anonymous_ptrace_syscall_info_entry`](../index.md#c-anonymous-ptrace-syscall-info-entry)

##### `impl CloneToUninit for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __c_anonymous_ptrace_syscall_info_entry`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="c-anonymous-ptrace-syscall-info-entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="c-anonymous-ptrace-syscall-info-entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__c_anonymous_ptrace_syscall_info_exit`

```rust
struct __c_anonymous_ptrace_syscall_info_exit {
    pub sval: crate::__s64,
    pub is_error: crate::__u8,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_exit` — [`__c_anonymous_ptrace_syscall_info_exit`](../index.md#c-anonymous-ptrace-syscall-info-exit)

##### `impl CloneToUninit for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __c_anonymous_ptrace_syscall_info_exit`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="c-anonymous-ptrace-syscall-info-exit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="c-anonymous-ptrace-syscall-info-exit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `__c_anonymous_ptrace_syscall_info_seccomp`

```rust
struct __c_anonymous_ptrace_syscall_info_seccomp {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
    pub ret_data: crate::__u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_seccomp` — [`__c_anonymous_ptrace_syscall_info_seccomp`](../index.md#c-anonymous-ptrace-syscall-info-seccomp)

##### `impl CloneToUninit for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for __c_anonymous_ptrace_syscall_info_seccomp`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ptrace_syscall_info`

```rust
struct ptrace_syscall_info {
    pub op: crate::__u8,
    pub pad: [crate::__u8; 3],
    pub arch: crate::__u32,
    pub instruction_pointer: crate::__u64,
    pub stack_pointer: crate::__u64,
    pub u: __c_anonymous_ptrace_syscall_info_data,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for ptrace_syscall_info`

- <span id="ptrace-syscall-info-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ptrace_syscall_info`

- <span id="ptrace-syscall-info-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ptrace_syscall_info`

- <span id="ptrace-syscall-info-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ptrace_syscall_info`

- <span id="ptrace-syscall-info-clone"></span>`fn clone(&self) -> ptrace_syscall_info` — [`ptrace_syscall_info`](../index.md#ptrace-syscall-info)

##### `impl CloneToUninit for ptrace_syscall_info`

- <span id="ptrace-syscall-info-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ptrace_syscall_info`

##### `impl Debug for ptrace_syscall_info`

- <span id="ptrace-syscall-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ptrace_syscall_info`

- <span id="ptrace-syscall-info-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ptrace_syscall_info`

- <span id="ptrace-syscall-info-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ptrace_syscall_info`

- <span id="ptrace-syscall-info-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ptrace-syscall-info-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ptrace_syscall_info`

- <span id="ptrace-syscall-info-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ptrace-syscall-info-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ptrace_sud_config`

```rust
struct ptrace_sud_config {
    pub mode: crate::__u64,
    pub selector: crate::__u64,
    pub offset: crate::__u64,
    pub len: crate::__u64,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for ptrace_sud_config`

- <span id="ptrace-sud-config-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ptrace_sud_config`

- <span id="ptrace-sud-config-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ptrace_sud_config`

- <span id="ptrace-sud-config-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ptrace_sud_config`

- <span id="ptrace-sud-config-clone"></span>`fn clone(&self) -> ptrace_sud_config` — [`ptrace_sud_config`](../index.md#ptrace-sud-config)

##### `impl CloneToUninit for ptrace_sud_config`

- <span id="ptrace-sud-config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ptrace_sud_config`

##### `impl Debug for ptrace_sud_config`

- <span id="ptrace-sud-config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ptrace_sud_config`

- <span id="ptrace-sud-config-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ptrace_sud_config`

- <span id="ptrace-sud-config-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ptrace_sud_config`

- <span id="ptrace-sud-config-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ptrace-sud-config-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ptrace_sud_config`

- <span id="ptrace-sud-config-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ptrace-sud-config-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `iocb`

```rust
struct iocb {
    pub aio_data: crate::__u64,
    pub aio_key: crate::__u32,
    pub aio_rw_flags: crate::__kernel_rwf_t,
    pub aio_lio_opcode: crate::__u16,
    pub aio_reqprio: crate::__s16,
    pub aio_fildes: crate::__u32,
    pub aio_buf: crate::__u64,
    pub aio_nbytes: crate::__u64,
    pub aio_offset: crate::__s64,
    aio_reserved2: crate::__u64,
    pub aio_flags: crate::__u32,
    pub aio_resfd: crate::__u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for iocb`

- <span id="iocb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for iocb`

- <span id="iocb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for iocb`

- <span id="iocb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for iocb`

- <span id="iocb-clone"></span>`fn clone(&self) -> iocb` — [`iocb`](../index.md#iocb)

##### `impl CloneToUninit for iocb`

- <span id="iocb-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for iocb`

##### `impl Debug for iocb`

- <span id="iocb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for iocb`

- <span id="iocb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for iocb`

- <span id="iocb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for iocb`

- <span id="iocb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iocb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for iocb`

- <span id="iocb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iocb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `tcp_info`

```rust
struct tcp_info {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    pub tcpi_snd_rcv_wscale: u8,
    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,
    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,
    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,
    pub tcpi_total_retrans: u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Fields

- **`tcpi_snd_rcv_wscale`**: `u8`

  This contains the bitfields `tcpi_snd_wscale` and `tcpi_rcv_wscale`.
  Each is 4 bits.

#### Trait Implementations

##### `impl Any for tcp_info`

- <span id="tcp-info-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for tcp_info`

- <span id="tcp-info-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for tcp_info`

- <span id="tcp-info-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for tcp_info`

- <span id="tcp-info-clone"></span>`fn clone(&self) -> tcp_info` — [`tcp_info`](../index.md#tcp-info)

##### `impl CloneToUninit for tcp_info`

- <span id="tcp-info-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for tcp_info`

##### `impl Debug for tcp_info`

- <span id="tcp-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for tcp_info`

- <span id="tcp-info-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for tcp_info`

- <span id="tcp-info-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for tcp_info`

- <span id="tcp-info-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tcp-info-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for tcp_info`

- <span id="tcp-info-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tcp-info-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fanotify_event_info_pidfd`

```rust
struct fanotify_event_info_pidfd {
    pub hdr: crate::fanotify_event_info_header,
    pub pidfd: crate::__s32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-clone"></span>`fn clone(&self) -> fanotify_event_info_pidfd` — [`fanotify_event_info_pidfd`](../index.md#fanotify-event-info-pidfd)

##### `impl CloneToUninit for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fanotify_event_info_pidfd`

##### `impl Debug for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fanotify-event-info-pidfd-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fanotify-event-info-pidfd-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fanotify_event_info_error`

```rust
struct fanotify_event_info_error {
    pub hdr: crate::fanotify_event_info_header,
    pub error: crate::__s32,
    pub error_count: crate::__u32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for fanotify_event_info_error`

- <span id="fanotify-event-info-error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fanotify_event_info_error`

- <span id="fanotify-event-info-error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fanotify_event_info_error`

- <span id="fanotify-event-info-error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fanotify_event_info_error`

- <span id="fanotify-event-info-error-clone"></span>`fn clone(&self) -> fanotify_event_info_error` — [`fanotify_event_info_error`](../index.md#fanotify-event-info-error)

##### `impl CloneToUninit for fanotify_event_info_error`

- <span id="fanotify-event-info-error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fanotify_event_info_error`

##### `impl Debug for fanotify_event_info_error`

- <span id="fanotify-event-info-error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fanotify_event_info_error`

- <span id="fanotify-event-info-error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fanotify_event_info_error`

- <span id="fanotify-event-info-error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fanotify_event_info_error`

- <span id="fanotify-event-info-error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fanotify-event-info-error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fanotify_event_info_error`

- <span id="fanotify-event-info-error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fanotify-event-info-error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sem_t`

```rust
struct sem_t {
    __size: [crate::c_char; 32],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for sem_t`

- <span id="sem-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sem_t`

- <span id="sem-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sem_t`

- <span id="sem-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sem_t`

- <span id="sem-t-clone"></span>`fn clone(&self) -> sem_t` — [`sem_t`](../index.md#sem-t)

##### `impl CloneToUninit for sem_t`

- <span id="sem-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sem_t`

##### `impl Debug for sem_t`

- <span id="sem-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sem_t`

- <span id="sem-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sem_t`

- <span id="sem-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sem_t`

- <span id="sem-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sem-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sem_t`

- <span id="sem-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sem-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `mbstate_t`

```rust
struct mbstate_t {
    __count: crate::c_int,
    __wchb: [crate::c_char; 4],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for mbstate_t`

- <span id="mbstate-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for mbstate_t`

- <span id="mbstate-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for mbstate_t`

- <span id="mbstate-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for mbstate_t`

- <span id="mbstate-t-clone"></span>`fn clone(&self) -> mbstate_t` — [`mbstate_t`](../index.md#mbstate-t)

##### `impl CloneToUninit for mbstate_t`

- <span id="mbstate-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for mbstate_t`

##### `impl Debug for mbstate_t`

- <span id="mbstate-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for mbstate_t`

- <span id="mbstate-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for mbstate_t`

- <span id="mbstate-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for mbstate_t`

- <span id="mbstate-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mbstate-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for mbstate_t`

- <span id="mbstate-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mbstate-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fpos64_t`

```rust
struct fpos64_t {
    __pos: crate::off64_t,
    __state: crate::mbstate_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for fpos64_t`

- <span id="fpos64-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fpos64_t`

- <span id="fpos64-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fpos64_t`

- <span id="fpos64-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fpos64_t`

- <span id="fpos64-t-clone"></span>`fn clone(&self) -> fpos64_t` — [`fpos64_t`](../index.md#fpos64-t)

##### `impl CloneToUninit for fpos64_t`

- <span id="fpos64-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fpos64_t`

##### `impl Debug for fpos64_t`

- <span id="fpos64-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fpos64_t`

- <span id="fpos64-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fpos64_t`

- <span id="fpos64-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fpos64_t`

- <span id="fpos64-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fpos64-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fpos64_t`

- <span id="fpos64-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fpos64-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `fpos_t`

```rust
struct fpos_t {
    __pos: off_t,
    __state: crate::mbstate_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for fpos_t`

- <span id="fpos-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fpos_t`

- <span id="fpos-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fpos_t`

- <span id="fpos-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fpos_t`

- <span id="fpos-t-clone"></span>`fn clone(&self) -> fpos_t` — [`fpos_t`](../index.md#fpos-t)

##### `impl CloneToUninit for fpos_t`

- <span id="fpos-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fpos_t`

##### `impl Debug for fpos_t`

- <span id="fpos-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for fpos_t`

- <span id="fpos-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for fpos_t`

- <span id="fpos-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for fpos_t`

- <span id="fpos-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fpos-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fpos_t`

- <span id="fpos-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fpos-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `timespec`

```rust
struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: crate::c_long,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for timespec`

- <span id="timespec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for timespec`

- <span id="timespec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for timespec`

- <span id="timespec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for timespec`

- <span id="timespec-clone"></span>`fn clone(&self) -> timespec` — [`timespec`](../index.md#timespec)

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

### `utmpx`

```rust
struct utmpx {
    pub ut_type: crate::c_short,
    pub ut_pid: crate::pid_t,
    pub ut_line: [crate::c_char; 32],
    pub ut_id: [crate::c_char; 4],
    pub ut_user: [crate::c_char; 32],
    pub ut_host: [crate::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: i32,
    pub ut_tv: __timeval,
    pub ut_addr_v6: [i32; 4],
    __glibc_reserved: [crate::c_char; 20],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:21-397`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L21-L397)*

#### Trait Implementations

##### `impl Any for utmpx`

- <span id="utmpx-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for utmpx`

- <span id="utmpx-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for utmpx`

- <span id="utmpx-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for utmpx`

- <span id="utmpx-clone"></span>`fn clone(&self) -> utmpx` — [`utmpx`](../index.md#utmpx)

##### `impl CloneToUninit for utmpx`

- <span id="utmpx-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for utmpx`

##### `impl Debug for utmpx`

- <span id="utmpx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for utmpx`

- <span id="utmpx-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for utmpx`

- <span id="utmpx-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for utmpx`

- <span id="utmpx-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utmpx-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for utmpx`

- <span id="utmpx-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utmpx-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sifields_sigchld`

```rust
struct sifields_sigchld {
    si_pid: crate::pid_t,
    si_uid: crate::uid_t,
    si_status: crate::c_int,
    si_utime: crate::c_long,
    si_stime: crate::c_long,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:425-448`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L425-L448)*

#### Trait Implementations

##### `impl Any for sifields_sigchld`

- <span id="sifields-sigchld-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sifields_sigchld`

- <span id="sifields-sigchld-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sifields_sigchld`

- <span id="sifields-sigchld-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sifields_sigchld`

- <span id="sifields-sigchld-clone"></span>`fn clone(&self) -> sifields_sigchld` — [`sifields_sigchld`](#sifields-sigchld)

##### `impl CloneToUninit for sifields_sigchld`

- <span id="sifields-sigchld-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sifields_sigchld`

##### `impl Debug for sifields_sigchld`

- <span id="sifields-sigchld-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sifields_sigchld`

- <span id="sifields-sigchld-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sifields_sigchld`

- <span id="sifields-sigchld-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sifields_sigchld`

- <span id="sifields-sigchld-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sifields-sigchld-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sifields_sigchld`

- <span id="sifields-sigchld-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sifields-sigchld-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `siginfo_f`

```rust
struct siginfo_f {
    _siginfo_base: [crate::c_int; 3],
    sifields: sifields,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:425-448`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L425-L448)*

#### Trait Implementations

##### `impl Any for siginfo_f`

- <span id="siginfo-f-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for siginfo_f`

- <span id="siginfo-f-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for siginfo_f`

- <span id="siginfo-f-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for siginfo_f`

- <span id="siginfo-f-clone"></span>`fn clone(&self) -> siginfo_f` — [`siginfo_f`](#siginfo-f)

##### `impl CloneToUninit for siginfo_f`

- <span id="siginfo-f-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for siginfo_f`

##### `impl Debug for siginfo_f`

- <span id="siginfo-f-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for siginfo_f`

- <span id="siginfo-f-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for siginfo_f`

- <span id="siginfo-f-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for siginfo_f`

- <span id="siginfo-f-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="siginfo-f-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for siginfo_f`

- <span id="siginfo-f-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="siginfo-f-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sigset_t`

```rust
struct sigset_t {
    __val: [u64; 16],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Any for sigset_t`

- <span id="sigset-t-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sigset_t`

- <span id="sigset-t-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sigset_t`

- <span id="sigset-t-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sigset_t`

- <span id="sigset-t-clone"></span>`fn clone(&self) -> sigset_t` — [`sigset_t`](#sigset-t)

##### `impl CloneToUninit for sigset_t`

- <span id="sigset-t-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sigset_t`

##### `impl Debug for sigset_t`

- <span id="sigset-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sigset_t`

- <span id="sigset-t-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sigset_t`

- <span id="sigset-t-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sigset_t`

- <span id="sigset-t-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sigset-t-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sigset_t`

- <span id="sigset-t-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sigset-t-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `sysinfo`

```rust
struct sysinfo {
    pub uptime: i64,
    pub loads: [u64; 3],
    pub totalram: u64,
    pub freeram: u64,
    pub sharedram: u64,
    pub bufferram: u64,
    pub totalswap: u64,
    pub freeswap: u64,
    pub procs: crate::c_ushort,
    pub pad: crate::c_ushort,
    pub totalhigh: u64,
    pub freehigh: u64,
    pub mem_unit: crate::c_uint,
    pub _f: [crate::c_char; 0],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Any for sysinfo`

- <span id="sysinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for sysinfo`

- <span id="sysinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for sysinfo`

- <span id="sysinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for sysinfo`

- <span id="sysinfo-clone"></span>`fn clone(&self) -> sysinfo` — [`sysinfo`](#sysinfo)

##### `impl CloneToUninit for sysinfo`

- <span id="sysinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for sysinfo`

##### `impl Debug for sysinfo`

- <span id="sysinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for sysinfo`

- <span id="sysinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for sysinfo`

- <span id="sysinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for sysinfo`

- <span id="sysinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sysinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for sysinfo`

- <span id="sysinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sysinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `msqid_ds`

```rust
struct msqid_ds {
    pub msg_perm: crate::ipc_perm,
    pub msg_stime: crate::time_t,
    pub msg_rtime: crate::time_t,
    pub msg_ctime: crate::time_t,
    pub __msg_cbytes: u64,
    pub msg_qnum: crate::msgqnum_t,
    pub msg_qbytes: crate::msglen_t,
    pub msg_lspid: crate::pid_t,
    pub msg_lrpid: crate::pid_t,
    __glibc_reserved4: u64,
    __glibc_reserved5: u64,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Any for msqid_ds`

- <span id="msqid-ds-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for msqid_ds`

- <span id="msqid-ds-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for msqid_ds`

- <span id="msqid-ds-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for msqid_ds`

- <span id="msqid-ds-clone"></span>`fn clone(&self) -> msqid_ds` — [`msqid_ds`](#msqid-ds)

##### `impl CloneToUninit for msqid_ds`

- <span id="msqid-ds-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for msqid_ds`

##### `impl Debug for msqid_ds`

- <span id="msqid-ds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for msqid_ds`

- <span id="msqid-ds-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for msqid_ds`

- <span id="msqid-ds-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for msqid_ds`

- <span id="msqid-ds-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="msqid-ds-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for msqid_ds`

- <span id="msqid-ds-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="msqid-ds-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `semid_ds`

```rust
struct semid_ds {
    pub sem_perm: ipc_perm,
    pub sem_otime: crate::time_t,
    __reserved: crate::types::Padding<crate::__syscall_ulong_t>,
    pub sem_ctime: crate::time_t,
    __reserved2: crate::types::Padding<crate::__syscall_ulong_t>,
    pub sem_nsems: crate::__syscall_ulong_t,
    __glibc_reserved3: crate::__syscall_ulong_t,
    __glibc_reserved4: crate::__syscall_ulong_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Any for semid_ds`

- <span id="semid-ds-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for semid_ds`

- <span id="semid-ds-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for semid_ds`

- <span id="semid-ds-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for semid_ds`

- <span id="semid-ds-clone"></span>`fn clone(&self) -> semid_ds` — [`semid_ds`](#semid-ds)

##### `impl CloneToUninit for semid_ds`

- <span id="semid-ds-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for semid_ds`

##### `impl Debug for semid_ds`

- <span id="semid-ds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for semid_ds`

- <span id="semid-ds-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for semid_ds`

- <span id="semid-ds-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for semid_ds`

- <span id="semid-ds-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="semid-ds-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for semid_ds`

- <span id="semid-ds-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="semid-ds-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `timex`

```rust
struct timex {
    pub modes: crate::c_uint,
    pub offset: crate::c_long,
    pub freq: crate::c_long,
    pub maxerror: crate::c_long,
    pub esterror: crate::c_long,
    pub status: crate::c_int,
    pub constant: crate::c_long,
    pub precision: crate::c_long,
    pub tolerance: crate::c_long,
    pub time: crate::timeval,
    pub tick: crate::c_long,
    pub ppsfreq: crate::c_long,
    pub jitter: crate::c_long,
    pub shift: crate::c_int,
    pub stabil: crate::c_long,
    pub jitcnt: crate::c_long,
    pub calcnt: crate::c_long,
    pub errcnt: crate::c_long,
    pub stbcnt: crate::c_long,
    pub tai: crate::c_int,
    pub __unused1: i32,
    pub __unused2: i32,
    pub __unused3: i32,
    pub __unused4: i32,
    pub __unused5: i32,
    pub __unused6: i32,
    pub __unused7: i32,
    pub __unused8: i32,
    pub __unused9: i32,
    pub __unused10: i32,
    pub __unused11: i32,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Any for timex`

- <span id="timex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for timex`

- <span id="timex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for timex`

- <span id="timex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for timex`

- <span id="timex-clone"></span>`fn clone(&self) -> timex` — [`timex`](#timex)

##### `impl CloneToUninit for timex`

- <span id="timex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for timex`

##### `impl Debug for timex`

- <span id="timex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for timex`

- <span id="timex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for timex`

- <span id="timex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for timex`

- <span id="timex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for timex`

- <span id="timex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `fgetspent_r`

```rust
unsafe fn fgetspent_r(fp: *mut crate::FILE, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:950-956`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L950-L956)*

### `sgetspent_r`

```rust
unsafe fn sgetspent_r(s: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:957-963`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L957-L963)*

### `getspent_r`

```rust
unsafe fn getspent_r(spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:964-969`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L964-L969)*

### `qsort_r`

```rust
unsafe fn qsort_r(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void, *mut c_void) -> c_int>, arg: *mut c_void)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:970-976`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L970-L976)*

### `sendmmsg`

```rust
unsafe fn sendmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:978-983`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L978-L983)*

### `recvmmsg`

```rust
unsafe fn recvmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:985-991`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L985-L991)*

### `getrlimit64`

```rust
unsafe fn getrlimit64(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:993`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L993)*

### `setrlimit64`

```rust
unsafe fn setrlimit64(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:994-995`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L994-L995)*

### `getrlimit`

```rust
unsafe fn getrlimit(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:997`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L997)*

### `setrlimit`

```rust
unsafe fn setrlimit(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:999`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L999)*

### `prlimit`

```rust
unsafe fn prlimit(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit, old_limit: *mut crate::rlimit) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1001-1006`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1001-L1006)*

### `prlimit64`

```rust
unsafe fn prlimit64(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit64, old_limit: *mut crate::rlimit64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1007-1012`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1007-L1012)*

### `utmpname`

```rust
unsafe fn utmpname(file: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1013`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1013)*

### `utmpxname`

```rust
unsafe fn utmpxname(file: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1014`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1014)*

### `getutxent`

```rust
unsafe fn getutxent() -> *mut utmpx
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1015`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1015)*

### `getutxid`

```rust
unsafe fn getutxid(ut: *const utmpx) -> *mut utmpx
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1016`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1016)*

### `getutxline`

```rust
unsafe fn getutxline(ut: *const utmpx) -> *mut utmpx
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1017`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1017)*

### `pututxline`

```rust
unsafe fn pututxline(ut: *const utmpx) -> *mut utmpx
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1018`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1018)*

### `setutxent`

```rust
unsafe fn setutxent()
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1019`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1019)*

### `endutxent`

```rust
unsafe fn endutxent()
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1020`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1020)*

### `getpt`

```rust
unsafe fn getpt() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1021`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1021)*

### `mallopt`

```rust
unsafe fn mallopt(param: c_int, value: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1022`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1022)*

### `gettimeofday`

```rust
unsafe fn gettimeofday(tp: *mut crate::timeval, tz: *mut crate::timezone) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1024`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1024)*

### `getentropy`

```rust
unsafe fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1025`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1025)*

### `getrandom`

```rust
unsafe fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1026`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1026)*

### `getauxval`

```rust
unsafe fn getauxval(type_: c_ulong) -> c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1027`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1027)*

### `adjtimex`

```rust
unsafe fn adjtimex(buf: *mut timex) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1030`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1030)*

### `ntp_adjtime`

```rust
unsafe fn ntp_adjtime(buf: *mut timex) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1032`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1032)*

### `ntp_gettime`

```rust
unsafe fn ntp_gettime(buf: *mut ntptimeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1035`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1035)*

### `clock_adjtime`

```rust
unsafe fn clock_adjtime(clk_id: crate::clockid_t, buf: *mut crate::timex) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1037`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1037)*

### `fanotify_mark`

```rust
unsafe fn fanotify_mark(fd: c_int, flags: c_uint, mask: u64, dirfd: c_int, path: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1039-1045`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1039-L1045)*

### `preadv2`

```rust
unsafe fn preadv2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1047-1053`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1047-L1053)*

### `pwritev2`

```rust
unsafe fn pwritev2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1055-1061`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1055-L1061)*

### `preadv64v2`

```rust
unsafe fn preadv64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1062-1068`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1062-L1068)*

### `pwritev64v2`

```rust
unsafe fn pwritev64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1069-1075`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1069-L1075)*

### `renameat2`

```rust
unsafe fn renameat2(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1076-1082`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1076-L1082)*

### `explicit_bzero`

```rust
unsafe fn explicit_bzero(s: *mut c_void, len: size_t)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1085`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1085)*

### `reallocarray`

```rust
unsafe fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1087`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1087)*

### `ctermid`

```rust
unsafe fn ctermid(s: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1089`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1089)*

### `backtrace`

```rust
unsafe fn backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1090`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1090)*

### `backtrace_symbols`

```rust
unsafe fn backtrace_symbols(buffer: *const *mut c_void, len: c_int) -> *mut *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1091`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1091)*

### `backtrace_symbols_fd`

```rust
unsafe fn backtrace_symbols_fd(buffer: *const *mut c_void, len: c_int, fd: c_int)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1092`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1092)*

### `glob64`

```rust
unsafe fn glob64(pattern: *const c_char, flags: c_int, errfunc: Option<fn(*const c_char, c_int) -> c_int>, pglob: *mut glob64_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1094-1099`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1094-L1099)*

### `globfree64`

```rust
unsafe fn globfree64(pglob: *mut glob64_t)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1101`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1101)*

### `ptrace`

```rust
unsafe fn ptrace(request: c_uint) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1102`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1102)*

### `pthread_attr_getaffinity_np`

```rust
unsafe fn pthread_attr_getaffinity_np(attr: *const crate::pthread_attr_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1103-1107`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1103-L1107)*

### `pthread_attr_setaffinity_np`

```rust
unsafe fn pthread_attr_setaffinity_np(attr: *mut crate::pthread_attr_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1108-1112`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1108-L1112)*

### `getpriority`

```rust
unsafe fn getpriority(which: crate::__priority_which_t, who: crate::id_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1113`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1113)*

### `setpriority`

```rust
unsafe fn setpriority(which: crate::__priority_which_t, who: crate::id_t, prio: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1114`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1114)*

### `pthread_rwlockattr_getkind_np`

```rust
unsafe fn pthread_rwlockattr_getkind_np(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1115-1118`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1115-L1118)*

### `pthread_rwlockattr_setkind_np`

```rust
unsafe fn pthread_rwlockattr_setkind_np(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1119-1122`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1119-L1122)*

### `pthread_sigqueue`

```rust
unsafe fn pthread_sigqueue(thread: crate::pthread_t, sig: c_int, value: crate::sigval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1123`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1123)*

### `mallinfo`

```rust
unsafe fn mallinfo() -> crate::mallinfo
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1124`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1124)*

### `mallinfo2`

```rust
unsafe fn mallinfo2() -> crate::mallinfo2
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1125`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1125)*

### `malloc_stats`

```rust
unsafe fn malloc_stats()
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1126`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1126)*

### `malloc_info`

```rust
unsafe fn malloc_info(options: c_int, stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1127`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1127)*

### `malloc_usable_size`

```rust
unsafe fn malloc_usable_size(ptr: *mut c_void) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1128`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1128)*

### `getpwent_r`

```rust
unsafe fn getpwent_r(pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1129-1134`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1129-L1134)*

### `getgrent_r`

```rust
unsafe fn getgrent_r(grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1135-1140`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1135-L1140)*

### `fgetpwent_r`

```rust
unsafe fn fgetpwent_r(stream: *mut crate::FILE, pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1141-1147`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1141-L1147)*

### `fgetgrent_r`

```rust
unsafe fn fgetgrent_r(stream: *mut crate::FILE, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1148-1154`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1148-L1154)*

### `putpwent`

```rust
unsafe fn putpwent(p: *const crate::passwd, stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1156`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1156)*

### `putgrent`

```rust
unsafe fn putgrent(grp: *const crate::group, stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1157`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1157)*

### `sethostid`

```rust
unsafe fn sethostid(hostid: c_long) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1159`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1159)*

### `memfd_create`

```rust
unsafe fn memfd_create(name: *const c_char, flags: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1161`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1161)*

### `mlock2`

```rust
unsafe fn mlock2(addr: *const c_void, len: size_t, flags: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1162`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1162)*

### `euidaccess`

```rust
unsafe fn euidaccess(pathname: *const c_char, mode: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1164`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1164)*

### `eaccess`

```rust
unsafe fn eaccess(pathname: *const c_char, mode: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1165`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1165)*

### `asctime_r`

```rust
unsafe fn asctime_r(tm: *const crate::tm, buf: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1167`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1167)*

### `ctime_r`

```rust
unsafe fn ctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1169`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1169)*

### `dirname`

```rust
unsafe fn dirname(path: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1171`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1171)*

### `posix_basename`

```rust
unsafe fn posix_basename(path: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1174`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1174)*

POSIX version of `basename(3)`, defined in `libgen.h`.

### `gnu_basename`

```rust
unsafe fn gnu_basename(path: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1177`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1177)*

GNU version of `basename(3)`, defined in `string.h`.

### `dlmopen`

```rust
unsafe fn dlmopen(lmid: Lmid_t, filename: *const c_char, flag: c_int) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1178`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1178)*

### `dlinfo`

```rust
unsafe fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1179`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1179)*

### `dladdr1`

```rust
unsafe fn dladdr1(addr: *const c_void, info: *mut crate::Dl_info, extra_info: *mut *mut c_void, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1180-1185`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1180-L1185)*

### `dlvsym`

```rust
unsafe fn dlvsym(handle: *mut c_void, symbol: *const c_char, version: *const c_char) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1186-1190`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1186-L1190)*

### `malloc_trim`

```rust
unsafe fn malloc_trim(__pad: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1191`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1191)*

### `gnu_get_libc_release`

```rust
unsafe fn gnu_get_libc_release() -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1192`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1192)*

### `gnu_get_libc_version`

```rust
unsafe fn gnu_get_libc_version() -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1193`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1193)*

### `posix_spawn_file_actions_addchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addchdir_np(actions: *mut crate::posix_spawn_file_actions_t, path: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1197-1200`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1197-L1200)*

### `posix_spawn_file_actions_addfchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addfchdir_np(actions: *mut crate::posix_spawn_file_actions_t, fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1202-1205`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1202-L1205)*

### `posix_spawn_file_actions_addclosefrom_np`

```rust
unsafe fn posix_spawn_file_actions_addclosefrom_np(actions: *mut crate::posix_spawn_file_actions_t, from: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1207-1210`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1207-L1210)*

### `posix_spawn_file_actions_addtcsetpgrp_np`

```rust
unsafe fn posix_spawn_file_actions_addtcsetpgrp_np(actions: *mut crate::posix_spawn_file_actions_t, tcfd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1212-1215`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1212-L1215)*

### `getmntent_r`

```rust
unsafe fn getmntent_r(stream: *mut crate::FILE, mntbuf: *mut crate::mntent, buf: *mut c_char, buflen: c_int) -> *mut crate::mntent
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1218-1223`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1218-L1223)*

### `execveat`

```rust
unsafe fn execveat(dirfd: c_int, pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1225-1231`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1225-L1231)*

### `close_range`

```rust
unsafe fn close_range(first: c_uint, last: c_uint, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1234`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1234)*

### `mq_notify`

```rust
unsafe fn mq_notify(mqdes: crate::mqd_t, sevp: *const crate::sigevent) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1236`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1236)*

### `epoll_pwait2`

```rust
unsafe fn epoll_pwait2(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1239-1245`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1239-L1245)*

### `mempcpy`

```rust
unsafe fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:1247`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L1247)*

## Type Aliases

### `pthread_t`

```rust
type pthread_t = crate::c_ulong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:4`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L4)*

### `__priority_which_t`

```rust
type __priority_which_t = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:5`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L5)*

### `__rlimit_resource_t`

```rust
type __rlimit_resource_t = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:6`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L6)*

### `Lmid_t`

```rust
type Lmid_t = crate::c_long;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:7`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L7)*

### `regoff_t`

```rust
type regoff_t = crate::c_int;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:8`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L8)*

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = crate::c_int;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:9`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L9)*

### `Ioctl`

```rust
type Ioctl = crate::c_ulong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:14`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L14)*

### `ino_t`

```rust
type ino_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:5`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L5)*

### `off_t`

```rust
type off_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:6`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L6)*

### `blkcnt_t`

```rust
type blkcnt_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:7`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L7)*

### `shmatt_t`

```rust
type shmatt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:8`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L8)*

### `msgqnum_t`

```rust
type msgqnum_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:9`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L9)*

### `msglen_t`

```rust
type msglen_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:10`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L10)*

### `fsblkcnt_t`

```rust
type fsblkcnt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:11`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L11)*

### `fsfilcnt_t`

```rust
type fsfilcnt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:12`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L12)*

### `rlim_t`

```rust
type rlim_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:13`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L13)*

### `__syscall_ulong_t`

```rust
type __syscall_ulong_t = crate::c_ulong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:17`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L17)*

### `__fsword_t`

```rust
type __fsword_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:25`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L25)*

### `clock_t`

```rust
type clock_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:26`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L26)*

### `time_t`

```rust
type time_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:27`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L27)*

## Constants

### `HUGETLB_FLAG_ENCODE_SHIFT`
```rust
const HUGETLB_FLAG_ENCODE_SHIFT: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:511`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L511)*

### `HUGETLB_FLAG_ENCODE_MASK`
```rust
const HUGETLB_FLAG_ENCODE_MASK: crate::c_int = 63i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:512`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L512)*

### `HUGETLB_FLAG_ENCODE_64KB`
```rust
const HUGETLB_FLAG_ENCODE_64KB: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:514`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L514)*

### `HUGETLB_FLAG_ENCODE_512KB`
```rust
const HUGETLB_FLAG_ENCODE_512KB: crate::c_int = 1_275_068_416i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:515`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L515)*

### `HUGETLB_FLAG_ENCODE_1MB`
```rust
const HUGETLB_FLAG_ENCODE_1MB: crate::c_int = 1_342_177_280i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:516`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L516)*

### `HUGETLB_FLAG_ENCODE_2MB`
```rust
const HUGETLB_FLAG_ENCODE_2MB: crate::c_int = 1_409_286_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:517`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L517)*

### `HUGETLB_FLAG_ENCODE_8MB`
```rust
const HUGETLB_FLAG_ENCODE_8MB: crate::c_int = 1_543_503_872i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:518`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L518)*

### `HUGETLB_FLAG_ENCODE_16MB`
```rust
const HUGETLB_FLAG_ENCODE_16MB: crate::c_int = 1_610_612_736i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:519`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L519)*

### `HUGETLB_FLAG_ENCODE_32MB`
```rust
const HUGETLB_FLAG_ENCODE_32MB: crate::c_int = 1_677_721_600i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:520`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L520)*

### `HUGETLB_FLAG_ENCODE_256MB`
```rust
const HUGETLB_FLAG_ENCODE_256MB: crate::c_int = 1_879_048_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:521`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L521)*

### `HUGETLB_FLAG_ENCODE_512MB`
```rust
const HUGETLB_FLAG_ENCODE_512MB: crate::c_int = 1_946_157_056i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:522`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L522)*

### `HUGETLB_FLAG_ENCODE_1GB`
```rust
const HUGETLB_FLAG_ENCODE_1GB: crate::c_int = 2_013_265_920i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:523`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L523)*

### `HUGETLB_FLAG_ENCODE_2GB`
```rust
const HUGETLB_FLAG_ENCODE_2GB: crate::c_int = 2_080_374_784i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:524`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L524)*

### `HUGETLB_FLAG_ENCODE_16GB`
```rust
const HUGETLB_FLAG_ENCODE_16GB: crate::c_int = -2_013_265_920i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:525`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L525)*

### `MAP_HUGE_SHIFT`
```rust
const MAP_HUGE_SHIFT: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:535`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L535)*

### `MAP_HUGE_MASK`
```rust
const MAP_HUGE_MASK: crate::c_int = 63i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:536`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L536)*

### `MAP_HUGE_64KB`
```rust
const MAP_HUGE_64KB: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:538`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L538)*

### `MAP_HUGE_512KB`
```rust
const MAP_HUGE_512KB: crate::c_int = 1_275_068_416i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:539`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L539)*

### `MAP_HUGE_1MB`
```rust
const MAP_HUGE_1MB: crate::c_int = 1_342_177_280i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:540`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L540)*

### `MAP_HUGE_2MB`
```rust
const MAP_HUGE_2MB: crate::c_int = 1_409_286_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:541`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L541)*

### `MAP_HUGE_8MB`
```rust
const MAP_HUGE_8MB: crate::c_int = 1_543_503_872i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:542`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L542)*

### `MAP_HUGE_16MB`
```rust
const MAP_HUGE_16MB: crate::c_int = 1_610_612_736i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:543`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L543)*

### `MAP_HUGE_32MB`
```rust
const MAP_HUGE_32MB: crate::c_int = 1_677_721_600i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:544`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L544)*

### `MAP_HUGE_256MB`
```rust
const MAP_HUGE_256MB: crate::c_int = 1_879_048_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:545`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L545)*

### `MAP_HUGE_512MB`
```rust
const MAP_HUGE_512MB: crate::c_int = 1_946_157_056i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:546`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L546)*

### `MAP_HUGE_1GB`
```rust
const MAP_HUGE_1GB: crate::c_int = 2_013_265_920i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:547`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L547)*

### `MAP_HUGE_2GB`
```rust
const MAP_HUGE_2GB: crate::c_int = 2_080_374_784i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:548`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L548)*

### `MAP_HUGE_16GB`
```rust
const MAP_HUGE_16GB: crate::c_int = -2_013_265_920i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:549`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L549)*

### `PRIO_PROCESS`
```rust
const PRIO_PROCESS: crate::__priority_which_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:551`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L551)*

### `PRIO_PGRP`
```rust
const PRIO_PGRP: crate::__priority_which_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:552`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L552)*

### `PRIO_USER`
```rust
const PRIO_USER: crate::__priority_which_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:553`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L553)*

### `MS_RMT_MASK`
```rust
const MS_RMT_MASK: crate::c_ulong = 41_943_121u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:555`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L555)*

### `__UT_LINESIZE`
```rust
const __UT_LINESIZE: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:557`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L557)*

### `__UT_NAMESIZE`
```rust
const __UT_NAMESIZE: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:558`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L558)*

### `__UT_HOSTSIZE`
```rust
const __UT_HOSTSIZE: usize = 256usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:559`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L559)*

### `EMPTY`
```rust
const EMPTY: crate::c_short = 0i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:560`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L560)*

### `RUN_LVL`
```rust
const RUN_LVL: crate::c_short = 1i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:561`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L561)*

### `BOOT_TIME`
```rust
const BOOT_TIME: crate::c_short = 2i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:562`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L562)*

### `NEW_TIME`
```rust
const NEW_TIME: crate::c_short = 3i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:563`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L563)*

### `OLD_TIME`
```rust
const OLD_TIME: crate::c_short = 4i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:564`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L564)*

### `INIT_PROCESS`
```rust
const INIT_PROCESS: crate::c_short = 5i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:565`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L565)*

### `LOGIN_PROCESS`
```rust
const LOGIN_PROCESS: crate::c_short = 6i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:566`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L566)*

### `USER_PROCESS`
```rust
const USER_PROCESS: crate::c_short = 7i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:567`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L567)*

### `DEAD_PROCESS`
```rust
const DEAD_PROCESS: crate::c_short = 8i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:568`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L568)*

### `ACCOUNTING`
```rust
const ACCOUNTING: crate::c_short = 9i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:569`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L569)*

### `LM_ID_BASE`
```rust
const LM_ID_BASE: crate::c_long = 0i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:572`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L572)*

### `LM_ID_NEWLM`
```rust
const LM_ID_NEWLM: crate::c_long = -1i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:573`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L573)*

### `RTLD_DI_LMID`
```rust
const RTLD_DI_LMID: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:575`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L575)*

### `RTLD_DI_LINKMAP`
```rust
const RTLD_DI_LINKMAP: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:576`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L576)*

### `RTLD_DI_CONFIGADDR`
```rust
const RTLD_DI_CONFIGADDR: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:577`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L577)*

### `RTLD_DI_SERINFO`
```rust
const RTLD_DI_SERINFO: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:578`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L578)*

### `RTLD_DI_SERINFOSIZE`
```rust
const RTLD_DI_SERINFOSIZE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:579`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L579)*

### `RTLD_DI_ORIGIN`
```rust
const RTLD_DI_ORIGIN: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:580`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L580)*

### `RTLD_DI_PROFILENAME`
```rust
const RTLD_DI_PROFILENAME: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:581`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L581)*

### `RTLD_DI_PROFILEOUT`
```rust
const RTLD_DI_PROFILEOUT: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:582`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L582)*

### `RTLD_DI_TLS_MODID`
```rust
const RTLD_DI_TLS_MODID: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:583`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L583)*

### `RTLD_DI_TLS_DATA`
```rust
const RTLD_DI_TLS_DATA: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:584`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L584)*

### `SOCK_NONBLOCK`
```rust
const SOCK_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:586`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L586)*

### `SOL_RXRPC`
```rust
const SOL_RXRPC: crate::c_int = 272i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:588`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L588)*

### `SOL_PPPOL2TP`
```rust
const SOL_PPPOL2TP: crate::c_int = 273i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:589`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L589)*

### `SOL_PNPIPE`
```rust
const SOL_PNPIPE: crate::c_int = 275i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:590`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L590)*

### `SOL_RDS`
```rust
const SOL_RDS: crate::c_int = 276i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:591`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L591)*

### `SOL_IUCV`
```rust
const SOL_IUCV: crate::c_int = 277i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:592`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L592)*

### `SOL_CAIF`
```rust
const SOL_CAIF: crate::c_int = 278i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:593`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L593)*

### `SOL_NFC`
```rust
const SOL_NFC: crate::c_int = 280i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:594`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L594)*

### `MSG_TRYHARD`
```rust
const MSG_TRYHARD: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:596`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L596)*

### `LC_PAPER`
```rust
const LC_PAPER: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:598`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L598)*

### `LC_NAME`
```rust
const LC_NAME: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:599`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L599)*

### `LC_ADDRESS`
```rust
const LC_ADDRESS: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:600`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L600)*

### `LC_TELEPHONE`
```rust
const LC_TELEPHONE: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:601`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L601)*

### `LC_MEASUREMENT`
```rust
const LC_MEASUREMENT: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:602`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L602)*

### `LC_IDENTIFICATION`
```rust
const LC_IDENTIFICATION: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:603`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L603)*

### `LC_PAPER_MASK`
```rust
const LC_PAPER_MASK: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:604`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L604)*

### `LC_NAME_MASK`
```rust
const LC_NAME_MASK: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:605`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L605)*

### `LC_ADDRESS_MASK`
```rust
const LC_ADDRESS_MASK: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:606`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L606)*

### `LC_TELEPHONE_MASK`
```rust
const LC_TELEPHONE_MASK: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:607`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L607)*

### `LC_MEASUREMENT_MASK`
```rust
const LC_MEASUREMENT_MASK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:608`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L608)*

### `LC_IDENTIFICATION_MASK`
```rust
const LC_IDENTIFICATION_MASK: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:609`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L609)*

### `LC_ALL_MASK`
```rust
const LC_ALL_MASK: crate::c_int = 8_127i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:610-621`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L610-L621)*

### `ENOTSUP`
```rust
const ENOTSUP: crate::c_int = 95i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:623`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L623)*

### `SOCK_SEQPACKET`
```rust
const SOCK_SEQPACKET: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:625`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L625)*

### `SOCK_DCCP`
```rust
const SOCK_DCCP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:626`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L626)*

### `SOCK_PACKET`
```rust
const SOCK_PACKET: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:628`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L628)*

### `AF_IB`
```rust
const AF_IB: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:630`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L630)*

### `AF_MPLS`
```rust
const AF_MPLS: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:631`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L631)*

### `AF_NFC`
```rust
const AF_NFC: crate::c_int = 39i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:632`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L632)*

### `AF_VSOCK`
```rust
const AF_VSOCK: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:633`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L633)*

### `AF_XDP`
```rust
const AF_XDP: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:634`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L634)*

### `PF_IB`
```rust
const PF_IB: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:635`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L635)*

### `PF_MPLS`
```rust
const PF_MPLS: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:636`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L636)*

### `PF_NFC`
```rust
const PF_NFC: crate::c_int = 39i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:637`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L637)*

### `PF_VSOCK`
```rust
const PF_VSOCK: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:638`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L638)*

### `PF_XDP`
```rust
const PF_XDP: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:639`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L639)*

### `SIGEV_THREAD_ID`
```rust
const SIGEV_THREAD_ID: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:641`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L641)*

### `BUFSIZ`
```rust
const BUFSIZ: crate::c_uint = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:643`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L643)*

### `TMP_MAX`
```rust
const TMP_MAX: crate::c_uint = 238_328u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:644`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L644)*

### `FOPEN_MAX`
```rust
const FOPEN_MAX: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:645`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L645)*

### `FILENAME_MAX`
```rust
const FILENAME_MAX: crate::c_uint = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:646`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L646)*

### `POSIX_MADV_DONTNEED`
```rust
const POSIX_MADV_DONTNEED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:647`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L647)*

### `_CS_GNU_LIBC_VERSION`
```rust
const _CS_GNU_LIBC_VERSION: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:648`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L648)*

### `_CS_GNU_LIBPTHREAD_VERSION`
```rust
const _CS_GNU_LIBPTHREAD_VERSION: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:649`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L649)*

### `_CS_V6_ENV`
```rust
const _CS_V6_ENV: crate::c_int = 1_148i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:650`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L650)*

### `_CS_V7_ENV`
```rust
const _CS_V7_ENV: crate::c_int = 1_149i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:651`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L651)*

### `_SC_EQUIV_CLASS_MAX`
```rust
const _SC_EQUIV_CLASS_MAX: crate::c_int = 41i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:652`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L652)*

### `_SC_CHARCLASS_NAME_MAX`
```rust
const _SC_CHARCLASS_NAME_MAX: crate::c_int = 45i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:653`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L653)*

### `_SC_PII`
```rust
const _SC_PII: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:654`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L654)*

### `_SC_PII_XTI`
```rust
const _SC_PII_XTI: crate::c_int = 54i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:655`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L655)*

### `_SC_PII_SOCKET`
```rust
const _SC_PII_SOCKET: crate::c_int = 55i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:656`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L656)*

### `_SC_PII_INTERNET`
```rust
const _SC_PII_INTERNET: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:657`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L657)*

### `_SC_PII_OSI`
```rust
const _SC_PII_OSI: crate::c_int = 57i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:658`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L658)*

### `_SC_POLL`
```rust
const _SC_POLL: crate::c_int = 58i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:659`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L659)*

### `_SC_SELECT`
```rust
const _SC_SELECT: crate::c_int = 59i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:660`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L660)*

### `_SC_PII_INTERNET_STREAM`
```rust
const _SC_PII_INTERNET_STREAM: crate::c_int = 61i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:661`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L661)*

### `_SC_PII_INTERNET_DGRAM`
```rust
const _SC_PII_INTERNET_DGRAM: crate::c_int = 62i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:662`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L662)*

### `_SC_PII_OSI_COTS`
```rust
const _SC_PII_OSI_COTS: crate::c_int = 63i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:663`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L663)*

### `_SC_PII_OSI_CLTS`
```rust
const _SC_PII_OSI_CLTS: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:664`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L664)*

### `_SC_PII_OSI_M`
```rust
const _SC_PII_OSI_M: crate::c_int = 65i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:665`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L665)*

### `_SC_T_IOV_MAX`
```rust
const _SC_T_IOV_MAX: crate::c_int = 66i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:666`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L666)*

### `_SC_2_C_VERSION`
```rust
const _SC_2_C_VERSION: crate::c_int = 96i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:667`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L667)*

### `_SC_CHAR_BIT`
```rust
const _SC_CHAR_BIT: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:668`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L668)*

### `_SC_CHAR_MAX`
```rust
const _SC_CHAR_MAX: crate::c_int = 102i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:669`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L669)*

### `_SC_CHAR_MIN`
```rust
const _SC_CHAR_MIN: crate::c_int = 103i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:670`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L670)*

### `_SC_INT_MAX`
```rust
const _SC_INT_MAX: crate::c_int = 104i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:671`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L671)*

### `_SC_INT_MIN`
```rust
const _SC_INT_MIN: crate::c_int = 105i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:672`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L672)*

### `_SC_LONG_BIT`
```rust
const _SC_LONG_BIT: crate::c_int = 106i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:673`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L673)*

### `_SC_WORD_BIT`
```rust
const _SC_WORD_BIT: crate::c_int = 107i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:674`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L674)*

### `_SC_MB_LEN_MAX`
```rust
const _SC_MB_LEN_MAX: crate::c_int = 108i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:675`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L675)*

### `_SC_SSIZE_MAX`
```rust
const _SC_SSIZE_MAX: crate::c_int = 110i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:676`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L676)*

### `_SC_SCHAR_MAX`
```rust
const _SC_SCHAR_MAX: crate::c_int = 111i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:677`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L677)*

### `_SC_SCHAR_MIN`
```rust
const _SC_SCHAR_MIN: crate::c_int = 112i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:678`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L678)*

### `_SC_SHRT_MAX`
```rust
const _SC_SHRT_MAX: crate::c_int = 113i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:679`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L679)*

### `_SC_SHRT_MIN`
```rust
const _SC_SHRT_MIN: crate::c_int = 114i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:680`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L680)*

### `_SC_UCHAR_MAX`
```rust
const _SC_UCHAR_MAX: crate::c_int = 115i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:681`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L681)*

### `_SC_UINT_MAX`
```rust
const _SC_UINT_MAX: crate::c_int = 116i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:682`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L682)*

### `_SC_ULONG_MAX`
```rust
const _SC_ULONG_MAX: crate::c_int = 117i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:683`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L683)*

### `_SC_USHRT_MAX`
```rust
const _SC_USHRT_MAX: crate::c_int = 118i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:684`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L684)*

### `_SC_NL_ARGMAX`
```rust
const _SC_NL_ARGMAX: crate::c_int = 119i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:685`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L685)*

### `_SC_NL_LANGMAX`
```rust
const _SC_NL_LANGMAX: crate::c_int = 120i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:686`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L686)*

### `_SC_NL_MSGMAX`
```rust
const _SC_NL_MSGMAX: crate::c_int = 121i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:687`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L687)*

### `_SC_NL_NMAX`
```rust
const _SC_NL_NMAX: crate::c_int = 122i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:688`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L688)*

### `_SC_NL_SETMAX`
```rust
const _SC_NL_SETMAX: crate::c_int = 123i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:689`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L689)*

### `_SC_NL_TEXTMAX`
```rust
const _SC_NL_TEXTMAX: crate::c_int = 124i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:690`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L690)*

### `_SC_BASE`
```rust
const _SC_BASE: crate::c_int = 134i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:691`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L691)*

### `_SC_C_LANG_SUPPORT`
```rust
const _SC_C_LANG_SUPPORT: crate::c_int = 135i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:692`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L692)*

### `_SC_C_LANG_SUPPORT_R`
```rust
const _SC_C_LANG_SUPPORT_R: crate::c_int = 136i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:693`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L693)*

### `_SC_DEVICE_IO`
```rust
const _SC_DEVICE_IO: crate::c_int = 140i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:694`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L694)*

### `_SC_DEVICE_SPECIFIC`
```rust
const _SC_DEVICE_SPECIFIC: crate::c_int = 141i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:695`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L695)*

### `_SC_DEVICE_SPECIFIC_R`
```rust
const _SC_DEVICE_SPECIFIC_R: crate::c_int = 142i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:696`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L696)*

### `_SC_FD_MGMT`
```rust
const _SC_FD_MGMT: crate::c_int = 143i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:697`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L697)*

### `_SC_FIFO`
```rust
const _SC_FIFO: crate::c_int = 144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:698`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L698)*

### `_SC_PIPE`
```rust
const _SC_PIPE: crate::c_int = 145i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:699`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L699)*

### `_SC_FILE_ATTRIBUTES`
```rust
const _SC_FILE_ATTRIBUTES: crate::c_int = 146i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:700`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L700)*

### `_SC_FILE_LOCKING`
```rust
const _SC_FILE_LOCKING: crate::c_int = 147i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:701`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L701)*

### `_SC_FILE_SYSTEM`
```rust
const _SC_FILE_SYSTEM: crate::c_int = 148i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:702`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L702)*

### `_SC_MULTI_PROCESS`
```rust
const _SC_MULTI_PROCESS: crate::c_int = 150i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:703`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L703)*

### `_SC_SINGLE_PROCESS`
```rust
const _SC_SINGLE_PROCESS: crate::c_int = 151i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:704`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L704)*

### `_SC_NETWORKING`
```rust
const _SC_NETWORKING: crate::c_int = 152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:705`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L705)*

### `_SC_REGEX_VERSION`
```rust
const _SC_REGEX_VERSION: crate::c_int = 156i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:706`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L706)*

### `_SC_SIGNALS`
```rust
const _SC_SIGNALS: crate::c_int = 158i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:707`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L707)*

### `_SC_SYSTEM_DATABASE`
```rust
const _SC_SYSTEM_DATABASE: crate::c_int = 162i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:708`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L708)*

### `_SC_SYSTEM_DATABASE_R`
```rust
const _SC_SYSTEM_DATABASE_R: crate::c_int = 163i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:709`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L709)*

### `_SC_USER_GROUPS`
```rust
const _SC_USER_GROUPS: crate::c_int = 166i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:710`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L710)*

### `_SC_USER_GROUPS_R`
```rust
const _SC_USER_GROUPS_R: crate::c_int = 167i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:711`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L711)*

### `_SC_LEVEL1_ICACHE_SIZE`
```rust
const _SC_LEVEL1_ICACHE_SIZE: crate::c_int = 185i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:712`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L712)*

### `_SC_LEVEL1_ICACHE_ASSOC`
```rust
const _SC_LEVEL1_ICACHE_ASSOC: crate::c_int = 186i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:713`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L713)*

### `_SC_LEVEL1_ICACHE_LINESIZE`
```rust
const _SC_LEVEL1_ICACHE_LINESIZE: crate::c_int = 187i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:714`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L714)*

### `_SC_LEVEL1_DCACHE_SIZE`
```rust
const _SC_LEVEL1_DCACHE_SIZE: crate::c_int = 188i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:715`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L715)*

### `_SC_LEVEL1_DCACHE_ASSOC`
```rust
const _SC_LEVEL1_DCACHE_ASSOC: crate::c_int = 189i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:716`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L716)*

### `_SC_LEVEL1_DCACHE_LINESIZE`
```rust
const _SC_LEVEL1_DCACHE_LINESIZE: crate::c_int = 190i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:717`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L717)*

### `_SC_LEVEL2_CACHE_SIZE`
```rust
const _SC_LEVEL2_CACHE_SIZE: crate::c_int = 191i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:718`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L718)*

### `_SC_LEVEL2_CACHE_ASSOC`
```rust
const _SC_LEVEL2_CACHE_ASSOC: crate::c_int = 192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:719`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L719)*

### `_SC_LEVEL2_CACHE_LINESIZE`
```rust
const _SC_LEVEL2_CACHE_LINESIZE: crate::c_int = 193i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:720`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L720)*

### `_SC_LEVEL3_CACHE_SIZE`
```rust
const _SC_LEVEL3_CACHE_SIZE: crate::c_int = 194i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:721`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L721)*

### `_SC_LEVEL3_CACHE_ASSOC`
```rust
const _SC_LEVEL3_CACHE_ASSOC: crate::c_int = 195i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:722`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L722)*

### `_SC_LEVEL3_CACHE_LINESIZE`
```rust
const _SC_LEVEL3_CACHE_LINESIZE: crate::c_int = 196i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:723`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L723)*

### `_SC_LEVEL4_CACHE_SIZE`
```rust
const _SC_LEVEL4_CACHE_SIZE: crate::c_int = 197i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:724`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L724)*

### `_SC_LEVEL4_CACHE_ASSOC`
```rust
const _SC_LEVEL4_CACHE_ASSOC: crate::c_int = 198i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:725`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L725)*

### `_SC_LEVEL4_CACHE_LINESIZE`
```rust
const _SC_LEVEL4_CACHE_LINESIZE: crate::c_int = 199i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:726`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L726)*

### `O_ACCMODE`
```rust
const O_ACCMODE: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:727`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L727)*

### `ST_RELATIME`
```rust
const ST_RELATIME: crate::c_ulong = 4_096u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:728`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L728)*

### `NI_MAXHOST`
```rust
const NI_MAXHOST: crate::socklen_t = 1_025u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:729`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L729)*

### `BINDERFS_SUPER_MAGIC`
```rust
const BINDERFS_SUPER_MAGIC: crate::c_long = 1_819_242_352i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:736`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L736)*

### `XFS_SUPER_MAGIC`
```rust
const XFS_SUPER_MAGIC: crate::c_long = 1_481_003_842i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:737`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L737)*

### `CPU_SETSIZE`
```rust
const CPU_SETSIZE: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:744`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L744)*

### `PTRACE_TRACEME`
```rust
const PTRACE_TRACEME: crate::c_uint = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:746`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L746)*

### `PTRACE_PEEKTEXT`
```rust
const PTRACE_PEEKTEXT: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:747`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L747)*

### `PTRACE_PEEKDATA`
```rust
const PTRACE_PEEKDATA: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:748`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L748)*

### `PTRACE_PEEKUSER`
```rust
const PTRACE_PEEKUSER: crate::c_uint = 3u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:749`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L749)*

### `PTRACE_POKETEXT`
```rust
const PTRACE_POKETEXT: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:750`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L750)*

### `PTRACE_POKEDATA`
```rust
const PTRACE_POKEDATA: crate::c_uint = 5u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:751`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L751)*

### `PTRACE_POKEUSER`
```rust
const PTRACE_POKEUSER: crate::c_uint = 6u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:752`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L752)*

### `PTRACE_CONT`
```rust
const PTRACE_CONT: crate::c_uint = 7u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:753`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L753)*

### `PTRACE_KILL`
```rust
const PTRACE_KILL: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:754`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L754)*

### `PTRACE_SINGLESTEP`
```rust
const PTRACE_SINGLESTEP: crate::c_uint = 9u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:755`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L755)*

### `PTRACE_ATTACH`
```rust
const PTRACE_ATTACH: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:756`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L756)*

### `PTRACE_SYSCALL`
```rust
const PTRACE_SYSCALL: crate::c_uint = 24u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:757`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L757)*

### `PTRACE_SETOPTIONS`
```rust
const PTRACE_SETOPTIONS: crate::c_uint = 16_896u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:758`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L758)*

### `PTRACE_GETEVENTMSG`
```rust
const PTRACE_GETEVENTMSG: crate::c_uint = 16_897u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:759`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L759)*

### `PTRACE_GETSIGINFO`
```rust
const PTRACE_GETSIGINFO: crate::c_uint = 16_898u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:760`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L760)*

### `PTRACE_SETSIGINFO`
```rust
const PTRACE_SETSIGINFO: crate::c_uint = 16_899u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:761`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L761)*

### `PTRACE_GETREGSET`
```rust
const PTRACE_GETREGSET: crate::c_uint = 16_900u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:762`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L762)*

### `PTRACE_SETREGSET`
```rust
const PTRACE_SETREGSET: crate::c_uint = 16_901u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:763`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L763)*

### `PTRACE_SEIZE`
```rust
const PTRACE_SEIZE: crate::c_uint = 16_902u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:764`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L764)*

### `PTRACE_INTERRUPT`
```rust
const PTRACE_INTERRUPT: crate::c_uint = 16_903u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:765`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L765)*

### `PTRACE_LISTEN`
```rust
const PTRACE_LISTEN: crate::c_uint = 16_904u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:766`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L766)*

### `PTRACE_PEEKSIGINFO`
```rust
const PTRACE_PEEKSIGINFO: crate::c_uint = 16_905u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:767`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L767)*

### `PTRACE_GETSIGMASK`
```rust
const PTRACE_GETSIGMASK: crate::c_uint = 16_906u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:768`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L768)*

### `PTRACE_SETSIGMASK`
```rust
const PTRACE_SETSIGMASK: crate::c_uint = 16_907u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:769`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L769)*

### `PTRACE_GET_SYSCALL_INFO`
```rust
const PTRACE_GET_SYSCALL_INFO: crate::c_uint = 16_910u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:770`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L770)*

### `PTRACE_SYSCALL_INFO_NONE`
```rust
const PTRACE_SYSCALL_INFO_NONE: crate::__u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:771`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L771)*

### `PTRACE_SYSCALL_INFO_ENTRY`
```rust
const PTRACE_SYSCALL_INFO_ENTRY: crate::__u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:772`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L772)*

### `PTRACE_SYSCALL_INFO_EXIT`
```rust
const PTRACE_SYSCALL_INFO_EXIT: crate::__u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:773`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L773)*

### `PTRACE_SYSCALL_INFO_SECCOMP`
```rust
const PTRACE_SYSCALL_INFO_SECCOMP: crate::__u8 = 3u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:774`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L774)*

### `PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`
```rust
const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 16u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:775`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L775)*

### `PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`
```rust
const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 17u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:776`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L776)*

### `TCA_PAD`
```rust
const TCA_PAD: crate::c_ushort = 9u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:779`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L779)*

### `TCA_DUMP_INVISIBLE`
```rust
const TCA_DUMP_INVISIBLE: crate::c_ushort = 10u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:780`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L780)*

### `TCA_CHAIN`
```rust
const TCA_CHAIN: crate::c_ushort = 11u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:781`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L781)*

### `TCA_HW_OFFLOAD`
```rust
const TCA_HW_OFFLOAD: crate::c_ushort = 12u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:782`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L782)*

### `RTM_DELNETCONF`
```rust
const RTM_DELNETCONF: u16 = 81u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:784`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L784)*

### `RTM_NEWSTATS`
```rust
const RTM_NEWSTATS: u16 = 92u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:785`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L785)*

### `RTM_GETSTATS`
```rust
const RTM_GETSTATS: u16 = 94u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:786`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L786)*

### `RTM_NEWCACHEREPORT`
```rust
const RTM_NEWCACHEREPORT: u16 = 96u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:787`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L787)*

### `RTM_F_LOOKUP_TABLE`
```rust
const RTM_F_LOOKUP_TABLE: crate::c_uint = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:789`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L789)*

### `RTM_F_FIB_MATCH`
```rust
const RTM_F_FIB_MATCH: crate::c_uint = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:790`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L790)*

### `RTA_VIA`
```rust
const RTA_VIA: crate::c_ushort = 18u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:792`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L792)*

### `RTA_NEWDST`
```rust
const RTA_NEWDST: crate::c_ushort = 19u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:793`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L793)*

### `RTA_PREF`
```rust
const RTA_PREF: crate::c_ushort = 20u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:794`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L794)*

### `RTA_ENCAP_TYPE`
```rust
const RTA_ENCAP_TYPE: crate::c_ushort = 21u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:795`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L795)*

### `RTA_ENCAP`
```rust
const RTA_ENCAP: crate::c_ushort = 22u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:796`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L796)*

### `RTA_EXPIRES`
```rust
const RTA_EXPIRES: crate::c_ushort = 23u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:797`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L797)*

### `RTA_PAD`
```rust
const RTA_PAD: crate::c_ushort = 24u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:798`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L798)*

### `RTA_UID`
```rust
const RTA_UID: crate::c_ushort = 25u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:799`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L799)*

### `RTA_TTL_PROPAGATE`
```rust
const RTA_TTL_PROPAGATE: crate::c_ushort = 26u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:800`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L800)*

### `NTF_EXT_LEARNED`
```rust
const NTF_EXT_LEARNED: u8 = 16u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:803`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L803)*

### `NTF_OFFLOADED`
```rust
const NTF_OFFLOADED: u8 = 32u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:804`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L804)*

### `NDA_MASTER`
```rust
const NDA_MASTER: crate::c_ushort = 9u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:806`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L806)*

### `NDA_LINK_NETNSID`
```rust
const NDA_LINK_NETNSID: crate::c_ushort = 10u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:807`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L807)*

### `NDA_SRC_VNI`
```rust
const NDA_SRC_VNI: crate::c_ushort = 11u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:808`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L808)*

### `UNAME26`
```rust
const UNAME26: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:811`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L811)*

### `FDPIC_FUNCPTRS`
```rust
const FDPIC_FUNCPTRS: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:812`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L812)*

### `MAX_LINKS`
```rust
const MAX_LINKS: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:814`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L814)*

### `GENL_UNS_ADMIN_PERM`
```rust
const GENL_UNS_ADMIN_PERM: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:816`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L816)*

### `GENL_ID_VFS_DQUOT`
```rust
const GENL_ID_VFS_DQUOT: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:818`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L818)*

### `GENL_ID_PMCRAID`
```rust
const GENL_ID_PMCRAID: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:819`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L819)*

### `ELFOSABI_ARM_AEABI`
```rust
const ELFOSABI_ARM_AEABI: u8 = 64u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:821`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L821)*

### `CLONE_NEWTIME`
```rust
const CLONE_NEWTIME: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:824`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L824)*

### `CLONE_CLEAR_SIGHAND`
```rust
const CLONE_CLEAR_SIGHAND: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:826`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L826)*

### `CLONE_INTO_CGROUP`
```rust
const CLONE_INTO_CGROUP: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:827`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L827)*

### `M_MXFAST`
```rust
const M_MXFAST: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:829`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L829)*

### `M_NLBLKS`
```rust
const M_NLBLKS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:830`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L830)*

### `M_GRAIN`
```rust
const M_GRAIN: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:831`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L831)*

### `M_KEEP`
```rust
const M_KEEP: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:832`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L832)*

### `M_TRIM_THRESHOLD`
```rust
const M_TRIM_THRESHOLD: crate::c_int = -1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:833`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L833)*

### `M_TOP_PAD`
```rust
const M_TOP_PAD: crate::c_int = -2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:834`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L834)*

### `M_MMAP_THRESHOLD`
```rust
const M_MMAP_THRESHOLD: crate::c_int = -3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:835`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L835)*

### `M_MMAP_MAX`
```rust
const M_MMAP_MAX: crate::c_int = -4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:836`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L836)*

### `M_CHECK_ACTION`
```rust
const M_CHECK_ACTION: crate::c_int = -5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:837`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L837)*

### `M_PERTURB`
```rust
const M_PERTURB: crate::c_int = -6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:838`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L838)*

### `M_ARENA_TEST`
```rust
const M_ARENA_TEST: crate::c_int = -7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:839`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L839)*

### `M_ARENA_MAX`
```rust
const M_ARENA_MAX: crate::c_int = -8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:840`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L840)*

### `SOMAXCONN`
```rust
const SOMAXCONN: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:842`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L842)*

### `MOVE_MOUNT_F_SYMLINKS`
```rust
const MOVE_MOUNT_F_SYMLINKS: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:845`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L845)*

### `MOVE_MOUNT_F_AUTOMOUNTS`
```rust
const MOVE_MOUNT_F_AUTOMOUNTS: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:846`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L846)*

### `MOVE_MOUNT_F_EMPTY_PATH`
```rust
const MOVE_MOUNT_F_EMPTY_PATH: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:847`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L847)*

### `MOVE_MOUNT_T_SYMLINKS`
```rust
const MOVE_MOUNT_T_SYMLINKS: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:848`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L848)*

### `MOVE_MOUNT_T_AUTOMOUNTS`
```rust
const MOVE_MOUNT_T_AUTOMOUNTS: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:849`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L849)*

### `MOVE_MOUNT_T_EMPTY_PATH`
```rust
const MOVE_MOUNT_T_EMPTY_PATH: crate::c_uint = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:850`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L850)*

### `MOVE_MOUNT_SET_GROUP`
```rust
const MOVE_MOUNT_SET_GROUP: crate::c_uint = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:851`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L851)*

### `MOVE_MOUNT_BENEATH`
```rust
const MOVE_MOUNT_BENEATH: crate::c_uint = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:852`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L852)*

### `ADJ_OFFSET`
```rust
const ADJ_OFFSET: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:855`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L855)*

### `ADJ_FREQUENCY`
```rust
const ADJ_FREQUENCY: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:856`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L856)*

### `ADJ_MAXERROR`
```rust
const ADJ_MAXERROR: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:857`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L857)*

### `ADJ_ESTERROR`
```rust
const ADJ_ESTERROR: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:858`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L858)*

### `ADJ_STATUS`
```rust
const ADJ_STATUS: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:859`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L859)*

### `ADJ_TIMECONST`
```rust
const ADJ_TIMECONST: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:860`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L860)*

### `ADJ_TAI`
```rust
const ADJ_TAI: crate::c_uint = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:861`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L861)*

### `ADJ_SETOFFSET`
```rust
const ADJ_SETOFFSET: crate::c_uint = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:862`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L862)*

### `ADJ_MICRO`
```rust
const ADJ_MICRO: crate::c_uint = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:863`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L863)*

### `ADJ_NANO`
```rust
const ADJ_NANO: crate::c_uint = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:864`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L864)*

### `ADJ_TICK`
```rust
const ADJ_TICK: crate::c_uint = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:865`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L865)*

### `ADJ_OFFSET_SINGLESHOT`
```rust
const ADJ_OFFSET_SINGLESHOT: crate::c_uint = 32_769u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:866`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L866)*

### `ADJ_OFFSET_SS_READ`
```rust
const ADJ_OFFSET_SS_READ: crate::c_uint = 40_961u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:867`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L867)*

### `MOD_OFFSET`
```rust
const MOD_OFFSET: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:868`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L868)*

### `MOD_FREQUENCY`
```rust
const MOD_FREQUENCY: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:869`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L869)*

### `MOD_MAXERROR`
```rust
const MOD_MAXERROR: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:870`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L870)*

### `MOD_ESTERROR`
```rust
const MOD_ESTERROR: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:871`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L871)*

### `MOD_STATUS`
```rust
const MOD_STATUS: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:872`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L872)*

### `MOD_TIMECONST`
```rust
const MOD_TIMECONST: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:873`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L873)*

### `MOD_CLKB`
```rust
const MOD_CLKB: crate::c_uint = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:874`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L874)*

### `MOD_CLKA`
```rust
const MOD_CLKA: crate::c_uint = 32_769u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:875`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L875)*

### `MOD_TAI`
```rust
const MOD_TAI: crate::c_uint = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:876`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L876)*

### `MOD_MICRO`
```rust
const MOD_MICRO: crate::c_uint = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:877`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L877)*

### `MOD_NANO`
```rust
const MOD_NANO: crate::c_uint = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:878`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L878)*

### `STA_PLL`
```rust
const STA_PLL: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:879`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L879)*

### `STA_PPSFREQ`
```rust
const STA_PPSFREQ: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:880`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L880)*

### `STA_PPSTIME`
```rust
const STA_PPSTIME: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:881`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L881)*

### `STA_FLL`
```rust
const STA_FLL: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:882`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L882)*

### `STA_INS`
```rust
const STA_INS: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:883`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L883)*

### `STA_DEL`
```rust
const STA_DEL: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:884`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L884)*

### `STA_UNSYNC`
```rust
const STA_UNSYNC: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:885`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L885)*

### `STA_FREQHOLD`
```rust
const STA_FREQHOLD: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:886`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L886)*

### `STA_PPSSIGNAL`
```rust
const STA_PPSSIGNAL: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:887`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L887)*

### `STA_PPSJITTER`
```rust
const STA_PPSJITTER: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:888`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L888)*

### `STA_PPSWANDER`
```rust
const STA_PPSWANDER: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:889`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L889)*

### `STA_PPSERROR`
```rust
const STA_PPSERROR: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:890`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L890)*

### `STA_CLOCKERR`
```rust
const STA_CLOCKERR: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:891`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L891)*

### `STA_NANO`
```rust
const STA_NANO: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:892`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L892)*

### `STA_MODE`
```rust
const STA_MODE: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:893`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L893)*

### `STA_CLK`
```rust
const STA_CLK: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:894`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L894)*

### `STA_RONLY`
```rust
const STA_RONLY: crate::c_int = 65_280i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:895-902`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L895-L902)*

### `NTP_API`
```rust
const NTP_API: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:903`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L903)*

### `TIME_OK`
```rust
const TIME_OK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:904`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L904)*

### `TIME_INS`
```rust
const TIME_INS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:905`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L905)*

### `TIME_DEL`
```rust
const TIME_DEL: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:906`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L906)*

### `TIME_OOP`
```rust
const TIME_OOP: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:907`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L907)*

### `TIME_WAIT`
```rust
const TIME_WAIT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:908`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L908)*

### `TIME_ERROR`
```rust
const TIME_ERROR: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:909`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L909)*

### `TIME_BAD`
```rust
const TIME_BAD: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:910`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L910)*

### `MAXTC`
```rust
const MAXTC: crate::c_long = 6i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:911`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L911)*

### `GLOB_PERIOD`
```rust
const GLOB_PERIOD: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:915`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L915)*

### `GLOB_ALTDIRFUNC`
```rust
const GLOB_ALTDIRFUNC: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:916`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L916)*

### `GLOB_BRACE`
```rust
const GLOB_BRACE: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:917`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L917)*

### `GLOB_NOMAGIC`
```rust
const GLOB_NOMAGIC: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:918`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L918)*

### `GLOB_TILDE`
```rust
const GLOB_TILDE: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:919`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L919)*

### `GLOB_ONLYDIR`
```rust
const GLOB_ONLYDIR: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:920`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L920)*

### `GLOB_TILDE_CHECK`
```rust
const GLOB_TILDE_CHECK: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:921`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L921)*

### `MADV_COLLAPSE`
```rust
const MADV_COLLAPSE: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:923`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L923)*

### `PTHREAD_STACK_MIN`
```rust
const PTHREAD_STACK_MIN: crate::size_t = 16_384usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:934`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L934)*

### `PTHREAD_MUTEX_ADAPTIVE_NP`
```rust
const PTHREAD_MUTEX_ADAPTIVE_NP: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:941`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L941)*

### `REG_STARTEND`
```rust
const REG_STARTEND: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:943`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L943)*

### `REG_EEND`
```rust
const REG_EEND: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:945`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L945)*

### `REG_ESIZE`
```rust
const REG_ESIZE: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:946`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L946)*

### `REG_ERPAREN`
```rust
const REG_ERPAREN: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs:947`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/mod.rs#L947)*

### `__SIZEOF_PTHREAD_RWLOCKATTR_T`
```rust
const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:181`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L181)*

### `O_LARGEFILE`
```rust
const O_LARGEFILE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:183`](../../../../../../.source_1765521767/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L183)*

