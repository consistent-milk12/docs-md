*[libc](../../../../../../../index.md) / [unix](../../../../../../index.md) / [linux_like](../../../../../index.md) / [linux](../../../../index.md) / [gnu](../../../index.md) / [b64](../../index.md) / [x86_64](../index.md) / [not_x32](index.md)*

---

# Module `not_x32`

## Contents

- [Structs](#structs)
  - [`statvfs`](#statvfs)
- [Functions](#functions)
  - [`sysctl`](#sysctl)
- [Constants](#constants)
  - [`__SIZEOF_PTHREAD_MUTEX_T`](#sizeof-pthread-mutex-t)
  - [`__SIZEOF_PTHREAD_RWLOCK_T`](#sizeof-pthread-rwlock-t)
  - [`__SIZEOF_PTHREAD_BARRIER_T`](#sizeof-pthread-barrier-t)
  - [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread-recursive-mutex-initializer-np)
  - [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread-errorcheck-mutex-initializer-np)
  - [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread-adaptive-mutex-initializer-np)
  - [`SYS_read`](#sys-read)
  - [`SYS_write`](#sys-write)
  - [`SYS_open`](#sys-open)
  - [`SYS_close`](#sys-close)
  - [`SYS_stat`](#sys-stat)
  - [`SYS_fstat`](#sys-fstat)
  - [`SYS_lstat`](#sys-lstat)
  - [`SYS_poll`](#sys-poll)
  - [`SYS_lseek`](#sys-lseek)
  - [`SYS_mmap`](#sys-mmap)
  - [`SYS_mprotect`](#sys-mprotect)
  - [`SYS_munmap`](#sys-munmap)
  - [`SYS_brk`](#sys-brk)
  - [`SYS_rt_sigaction`](#sys-rt-sigaction)
  - [`SYS_rt_sigprocmask`](#sys-rt-sigprocmask)
  - [`SYS_rt_sigreturn`](#sys-rt-sigreturn)
  - [`SYS_ioctl`](#sys-ioctl)
  - [`SYS_pread64`](#sys-pread64)
  - [`SYS_pwrite64`](#sys-pwrite64)
  - [`SYS_readv`](#sys-readv)
  - [`SYS_writev`](#sys-writev)
  - [`SYS_access`](#sys-access)
  - [`SYS_pipe`](#sys-pipe)
  - [`SYS_select`](#sys-select)
  - [`SYS_sched_yield`](#sys-sched-yield)
  - [`SYS_mremap`](#sys-mremap)
  - [`SYS_msync`](#sys-msync)
  - [`SYS_mincore`](#sys-mincore)
  - [`SYS_madvise`](#sys-madvise)
  - [`SYS_shmget`](#sys-shmget)
  - [`SYS_shmat`](#sys-shmat)
  - [`SYS_shmctl`](#sys-shmctl)
  - [`SYS_dup`](#sys-dup)
  - [`SYS_dup2`](#sys-dup2)
  - [`SYS_pause`](#sys-pause)
  - [`SYS_nanosleep`](#sys-nanosleep)
  - [`SYS_getitimer`](#sys-getitimer)
  - [`SYS_alarm`](#sys-alarm)
  - [`SYS_setitimer`](#sys-setitimer)
  - [`SYS_getpid`](#sys-getpid)
  - [`SYS_sendfile`](#sys-sendfile)
  - [`SYS_socket`](#sys-socket)
  - [`SYS_connect`](#sys-connect)
  - [`SYS_accept`](#sys-accept)
  - [`SYS_sendto`](#sys-sendto)
  - [`SYS_recvfrom`](#sys-recvfrom)
  - [`SYS_sendmsg`](#sys-sendmsg)
  - [`SYS_recvmsg`](#sys-recvmsg)
  - [`SYS_shutdown`](#sys-shutdown)
  - [`SYS_bind`](#sys-bind)
  - [`SYS_listen`](#sys-listen)
  - [`SYS_getsockname`](#sys-getsockname)
  - [`SYS_getpeername`](#sys-getpeername)
  - [`SYS_socketpair`](#sys-socketpair)
  - [`SYS_setsockopt`](#sys-setsockopt)
  - [`SYS_getsockopt`](#sys-getsockopt)
  - [`SYS_clone`](#sys-clone)
  - [`SYS_fork`](#sys-fork)
  - [`SYS_vfork`](#sys-vfork)
  - [`SYS_execve`](#sys-execve)
  - [`SYS_exit`](#sys-exit)
  - [`SYS_wait4`](#sys-wait4)
  - [`SYS_kill`](#sys-kill)
  - [`SYS_uname`](#sys-uname)
  - [`SYS_semget`](#sys-semget)
  - [`SYS_semop`](#sys-semop)
  - [`SYS_semctl`](#sys-semctl)
  - [`SYS_shmdt`](#sys-shmdt)
  - [`SYS_msgget`](#sys-msgget)
  - [`SYS_msgsnd`](#sys-msgsnd)
  - [`SYS_msgrcv`](#sys-msgrcv)
  - [`SYS_msgctl`](#sys-msgctl)
  - [`SYS_fcntl`](#sys-fcntl)
  - [`SYS_flock`](#sys-flock)
  - [`SYS_fsync`](#sys-fsync)
  - [`SYS_fdatasync`](#sys-fdatasync)
  - [`SYS_truncate`](#sys-truncate)
  - [`SYS_ftruncate`](#sys-ftruncate)
  - [`SYS_getdents`](#sys-getdents)
  - [`SYS_getcwd`](#sys-getcwd)
  - [`SYS_chdir`](#sys-chdir)
  - [`SYS_fchdir`](#sys-fchdir)
  - [`SYS_rename`](#sys-rename)
  - [`SYS_mkdir`](#sys-mkdir)
  - [`SYS_rmdir`](#sys-rmdir)
  - [`SYS_creat`](#sys-creat)
  - [`SYS_link`](#sys-link)
  - [`SYS_unlink`](#sys-unlink)
  - [`SYS_symlink`](#sys-symlink)
  - [`SYS_readlink`](#sys-readlink)
  - [`SYS_chmod`](#sys-chmod)
  - [`SYS_fchmod`](#sys-fchmod)
  - [`SYS_chown`](#sys-chown)
  - [`SYS_fchown`](#sys-fchown)
  - [`SYS_lchown`](#sys-lchown)
  - [`SYS_umask`](#sys-umask)
  - [`SYS_gettimeofday`](#sys-gettimeofday)
  - [`SYS_getrlimit`](#sys-getrlimit)
  - [`SYS_getrusage`](#sys-getrusage)
  - [`SYS_sysinfo`](#sys-sysinfo)
  - [`SYS_times`](#sys-times)
  - [`SYS_ptrace`](#sys-ptrace)
  - [`SYS_getuid`](#sys-getuid)
  - [`SYS_syslog`](#sys-syslog)
  - [`SYS_getgid`](#sys-getgid)
  - [`SYS_setuid`](#sys-setuid)
  - [`SYS_setgid`](#sys-setgid)
  - [`SYS_geteuid`](#sys-geteuid)
  - [`SYS_getegid`](#sys-getegid)
  - [`SYS_setpgid`](#sys-setpgid)
  - [`SYS_getppid`](#sys-getppid)
  - [`SYS_getpgrp`](#sys-getpgrp)
  - [`SYS_setsid`](#sys-setsid)
  - [`SYS_setreuid`](#sys-setreuid)
  - [`SYS_setregid`](#sys-setregid)
  - [`SYS_getgroups`](#sys-getgroups)
  - [`SYS_setgroups`](#sys-setgroups)
  - [`SYS_setresuid`](#sys-setresuid)
  - [`SYS_getresuid`](#sys-getresuid)
  - [`SYS_setresgid`](#sys-setresgid)
  - [`SYS_getresgid`](#sys-getresgid)
  - [`SYS_getpgid`](#sys-getpgid)
  - [`SYS_setfsuid`](#sys-setfsuid)
  - [`SYS_setfsgid`](#sys-setfsgid)
  - [`SYS_getsid`](#sys-getsid)
  - [`SYS_capget`](#sys-capget)
  - [`SYS_capset`](#sys-capset)
  - [`SYS_rt_sigpending`](#sys-rt-sigpending)
  - [`SYS_rt_sigtimedwait`](#sys-rt-sigtimedwait)
  - [`SYS_rt_sigqueueinfo`](#sys-rt-sigqueueinfo)
  - [`SYS_rt_sigsuspend`](#sys-rt-sigsuspend)
  - [`SYS_sigaltstack`](#sys-sigaltstack)
  - [`SYS_utime`](#sys-utime)
  - [`SYS_mknod`](#sys-mknod)
  - [`SYS_uselib`](#sys-uselib)
  - [`SYS_personality`](#sys-personality)
  - [`SYS_ustat`](#sys-ustat)
  - [`SYS_statfs`](#sys-statfs)
  - [`SYS_fstatfs`](#sys-fstatfs)
  - [`SYS_sysfs`](#sys-sysfs)
  - [`SYS_getpriority`](#sys-getpriority)
  - [`SYS_setpriority`](#sys-setpriority)
  - [`SYS_sched_setparam`](#sys-sched-setparam)
  - [`SYS_sched_getparam`](#sys-sched-getparam)
  - [`SYS_sched_setscheduler`](#sys-sched-setscheduler)
  - [`SYS_sched_getscheduler`](#sys-sched-getscheduler)
  - [`SYS_sched_get_priority_max`](#sys-sched-get-priority-max)
  - [`SYS_sched_get_priority_min`](#sys-sched-get-priority-min)
  - [`SYS_sched_rr_get_interval`](#sys-sched-rr-get-interval)
  - [`SYS_mlock`](#sys-mlock)
  - [`SYS_munlock`](#sys-munlock)
  - [`SYS_mlockall`](#sys-mlockall)
  - [`SYS_munlockall`](#sys-munlockall)
  - [`SYS_vhangup`](#sys-vhangup)
  - [`SYS_modify_ldt`](#sys-modify-ldt)
  - [`SYS_pivot_root`](#sys-pivot-root)
  - [`SYS__sysctl`](#sys-sysctl)
  - [`SYS_prctl`](#sys-prctl)
  - [`SYS_arch_prctl`](#sys-arch-prctl)
  - [`SYS_adjtimex`](#sys-adjtimex)
  - [`SYS_setrlimit`](#sys-setrlimit)
  - [`SYS_chroot`](#sys-chroot)
  - [`SYS_sync`](#sys-sync)
  - [`SYS_acct`](#sys-acct)
  - [`SYS_settimeofday`](#sys-settimeofday)
  - [`SYS_mount`](#sys-mount)
  - [`SYS_umount2`](#sys-umount2)
  - [`SYS_swapon`](#sys-swapon)
  - [`SYS_swapoff`](#sys-swapoff)
  - [`SYS_reboot`](#sys-reboot)
  - [`SYS_sethostname`](#sys-sethostname)
  - [`SYS_setdomainname`](#sys-setdomainname)
  - [`SYS_iopl`](#sys-iopl)
  - [`SYS_ioperm`](#sys-ioperm)
  - [`SYS_create_module`](#sys-create-module)
  - [`SYS_init_module`](#sys-init-module)
  - [`SYS_delete_module`](#sys-delete-module)
  - [`SYS_get_kernel_syms`](#sys-get-kernel-syms)
  - [`SYS_query_module`](#sys-query-module)
  - [`SYS_quotactl`](#sys-quotactl)
  - [`SYS_nfsservctl`](#sys-nfsservctl)
  - [`SYS_getpmsg`](#sys-getpmsg)
  - [`SYS_putpmsg`](#sys-putpmsg)
  - [`SYS_afs_syscall`](#sys-afs-syscall)
  - [`SYS_tuxcall`](#sys-tuxcall)
  - [`SYS_security`](#sys-security)
  - [`SYS_gettid`](#sys-gettid)
  - [`SYS_readahead`](#sys-readahead)
  - [`SYS_setxattr`](#sys-setxattr)
  - [`SYS_lsetxattr`](#sys-lsetxattr)
  - [`SYS_fsetxattr`](#sys-fsetxattr)
  - [`SYS_getxattr`](#sys-getxattr)
  - [`SYS_lgetxattr`](#sys-lgetxattr)
  - [`SYS_fgetxattr`](#sys-fgetxattr)
  - [`SYS_listxattr`](#sys-listxattr)
  - [`SYS_llistxattr`](#sys-llistxattr)
  - [`SYS_flistxattr`](#sys-flistxattr)
  - [`SYS_removexattr`](#sys-removexattr)
  - [`SYS_lremovexattr`](#sys-lremovexattr)
  - [`SYS_fremovexattr`](#sys-fremovexattr)
  - [`SYS_tkill`](#sys-tkill)
  - [`SYS_time`](#sys-time)
  - [`SYS_futex`](#sys-futex)
  - [`SYS_sched_setaffinity`](#sys-sched-setaffinity)
  - [`SYS_sched_getaffinity`](#sys-sched-getaffinity)
  - [`SYS_set_thread_area`](#sys-set-thread-area)
  - [`SYS_io_setup`](#sys-io-setup)
  - [`SYS_io_destroy`](#sys-io-destroy)
  - [`SYS_io_getevents`](#sys-io-getevents)
  - [`SYS_io_submit`](#sys-io-submit)
  - [`SYS_io_cancel`](#sys-io-cancel)
  - [`SYS_get_thread_area`](#sys-get-thread-area)
  - [`SYS_lookup_dcookie`](#sys-lookup-dcookie)
  - [`SYS_epoll_create`](#sys-epoll-create)
  - [`SYS_epoll_ctl_old`](#sys-epoll-ctl-old)
  - [`SYS_epoll_wait_old`](#sys-epoll-wait-old)
  - [`SYS_remap_file_pages`](#sys-remap-file-pages)
  - [`SYS_getdents64`](#sys-getdents64)
  - [`SYS_set_tid_address`](#sys-set-tid-address)
  - [`SYS_restart_syscall`](#sys-restart-syscall)
  - [`SYS_semtimedop`](#sys-semtimedop)
  - [`SYS_fadvise64`](#sys-fadvise64)
  - [`SYS_timer_create`](#sys-timer-create)
  - [`SYS_timer_settime`](#sys-timer-settime)
  - [`SYS_timer_gettime`](#sys-timer-gettime)
  - [`SYS_timer_getoverrun`](#sys-timer-getoverrun)
  - [`SYS_timer_delete`](#sys-timer-delete)
  - [`SYS_clock_settime`](#sys-clock-settime)
  - [`SYS_clock_gettime`](#sys-clock-gettime)
  - [`SYS_clock_getres`](#sys-clock-getres)
  - [`SYS_clock_nanosleep`](#sys-clock-nanosleep)
  - [`SYS_exit_group`](#sys-exit-group)
  - [`SYS_epoll_wait`](#sys-epoll-wait)
  - [`SYS_epoll_ctl`](#sys-epoll-ctl)
  - [`SYS_tgkill`](#sys-tgkill)
  - [`SYS_utimes`](#sys-utimes)
  - [`SYS_vserver`](#sys-vserver)
  - [`SYS_mbind`](#sys-mbind)
  - [`SYS_set_mempolicy`](#sys-set-mempolicy)
  - [`SYS_get_mempolicy`](#sys-get-mempolicy)
  - [`SYS_mq_open`](#sys-mq-open)
  - [`SYS_mq_unlink`](#sys-mq-unlink)
  - [`SYS_mq_timedsend`](#sys-mq-timedsend)
  - [`SYS_mq_timedreceive`](#sys-mq-timedreceive)
  - [`SYS_mq_notify`](#sys-mq-notify)
  - [`SYS_mq_getsetattr`](#sys-mq-getsetattr)
  - [`SYS_kexec_load`](#sys-kexec-load)
  - [`SYS_waitid`](#sys-waitid)
  - [`SYS_add_key`](#sys-add-key)
  - [`SYS_request_key`](#sys-request-key)
  - [`SYS_keyctl`](#sys-keyctl)
  - [`SYS_ioprio_set`](#sys-ioprio-set)
  - [`SYS_ioprio_get`](#sys-ioprio-get)
  - [`SYS_inotify_init`](#sys-inotify-init)
  - [`SYS_inotify_add_watch`](#sys-inotify-add-watch)
  - [`SYS_inotify_rm_watch`](#sys-inotify-rm-watch)
  - [`SYS_migrate_pages`](#sys-migrate-pages)
  - [`SYS_openat`](#sys-openat)
  - [`SYS_mkdirat`](#sys-mkdirat)
  - [`SYS_mknodat`](#sys-mknodat)
  - [`SYS_fchownat`](#sys-fchownat)
  - [`SYS_futimesat`](#sys-futimesat)
  - [`SYS_newfstatat`](#sys-newfstatat)
  - [`SYS_unlinkat`](#sys-unlinkat)
  - [`SYS_renameat`](#sys-renameat)
  - [`SYS_linkat`](#sys-linkat)
  - [`SYS_symlinkat`](#sys-symlinkat)
  - [`SYS_readlinkat`](#sys-readlinkat)
  - [`SYS_fchmodat`](#sys-fchmodat)
  - [`SYS_faccessat`](#sys-faccessat)
  - [`SYS_pselect6`](#sys-pselect6)
  - [`SYS_ppoll`](#sys-ppoll)
  - [`SYS_unshare`](#sys-unshare)
  - [`SYS_set_robust_list`](#sys-set-robust-list)
  - [`SYS_get_robust_list`](#sys-get-robust-list)
  - [`SYS_splice`](#sys-splice)
  - [`SYS_tee`](#sys-tee)
  - [`SYS_sync_file_range`](#sys-sync-file-range)
  - [`SYS_vmsplice`](#sys-vmsplice)
  - [`SYS_move_pages`](#sys-move-pages)
  - [`SYS_utimensat`](#sys-utimensat)
  - [`SYS_epoll_pwait`](#sys-epoll-pwait)
  - [`SYS_signalfd`](#sys-signalfd)
  - [`SYS_timerfd_create`](#sys-timerfd-create)
  - [`SYS_eventfd`](#sys-eventfd)
  - [`SYS_fallocate`](#sys-fallocate)
  - [`SYS_timerfd_settime`](#sys-timerfd-settime)
  - [`SYS_timerfd_gettime`](#sys-timerfd-gettime)
  - [`SYS_accept4`](#sys-accept4)
  - [`SYS_signalfd4`](#sys-signalfd4)
  - [`SYS_eventfd2`](#sys-eventfd2)
  - [`SYS_epoll_create1`](#sys-epoll-create1)
  - [`SYS_dup3`](#sys-dup3)
  - [`SYS_pipe2`](#sys-pipe2)
  - [`SYS_inotify_init1`](#sys-inotify-init1)
  - [`SYS_preadv`](#sys-preadv)
  - [`SYS_pwritev`](#sys-pwritev)
  - [`SYS_rt_tgsigqueueinfo`](#sys-rt-tgsigqueueinfo)
  - [`SYS_perf_event_open`](#sys-perf-event-open)
  - [`SYS_recvmmsg`](#sys-recvmmsg)
  - [`SYS_fanotify_init`](#sys-fanotify-init)
  - [`SYS_fanotify_mark`](#sys-fanotify-mark)
  - [`SYS_prlimit64`](#sys-prlimit64)
  - [`SYS_name_to_handle_at`](#sys-name-to-handle-at)
  - [`SYS_open_by_handle_at`](#sys-open-by-handle-at)
  - [`SYS_clock_adjtime`](#sys-clock-adjtime)
  - [`SYS_syncfs`](#sys-syncfs)
  - [`SYS_sendmmsg`](#sys-sendmmsg)
  - [`SYS_setns`](#sys-setns)
  - [`SYS_getcpu`](#sys-getcpu)
  - [`SYS_process_vm_readv`](#sys-process-vm-readv)
  - [`SYS_process_vm_writev`](#sys-process-vm-writev)
  - [`SYS_kcmp`](#sys-kcmp)
  - [`SYS_finit_module`](#sys-finit-module)
  - [`SYS_sched_setattr`](#sys-sched-setattr)
  - [`SYS_sched_getattr`](#sys-sched-getattr)
  - [`SYS_renameat2`](#sys-renameat2)
  - [`SYS_seccomp`](#sys-seccomp)
  - [`SYS_getrandom`](#sys-getrandom)
  - [`SYS_memfd_create`](#sys-memfd-create)
  - [`SYS_kexec_file_load`](#sys-kexec-file-load)
  - [`SYS_bpf`](#sys-bpf)
  - [`SYS_execveat`](#sys-execveat)
  - [`SYS_userfaultfd`](#sys-userfaultfd)
  - [`SYS_membarrier`](#sys-membarrier)
  - [`SYS_mlock2`](#sys-mlock2)
  - [`SYS_copy_file_range`](#sys-copy-file-range)
  - [`SYS_preadv2`](#sys-preadv2)
  - [`SYS_pwritev2`](#sys-pwritev2)
  - [`SYS_pkey_mprotect`](#sys-pkey-mprotect)
  - [`SYS_pkey_alloc`](#sys-pkey-alloc)
  - [`SYS_pkey_free`](#sys-pkey-free)
  - [`SYS_statx`](#sys-statx)
  - [`SYS_rseq`](#sys-rseq)
  - [`SYS_pidfd_send_signal`](#sys-pidfd-send-signal)
  - [`SYS_io_uring_setup`](#sys-io-uring-setup)
  - [`SYS_io_uring_enter`](#sys-io-uring-enter)
  - [`SYS_io_uring_register`](#sys-io-uring-register)
  - [`SYS_open_tree`](#sys-open-tree)
  - [`SYS_move_mount`](#sys-move-mount)
  - [`SYS_fsopen`](#sys-fsopen)
  - [`SYS_fsconfig`](#sys-fsconfig)
  - [`SYS_fsmount`](#sys-fsmount)
  - [`SYS_fspick`](#sys-fspick)
  - [`SYS_pidfd_open`](#sys-pidfd-open)
  - [`SYS_clone3`](#sys-clone3)
  - [`SYS_close_range`](#sys-close-range)
  - [`SYS_openat2`](#sys-openat2)
  - [`SYS_pidfd_getfd`](#sys-pidfd-getfd)
  - [`SYS_faccessat2`](#sys-faccessat2)
  - [`SYS_process_madvise`](#sys-process-madvise)
  - [`SYS_epoll_pwait2`](#sys-epoll-pwait2)
  - [`SYS_mount_setattr`](#sys-mount-setattr)
  - [`SYS_quotactl_fd`](#sys-quotactl-fd)
  - [`SYS_landlock_create_ruleset`](#sys-landlock-create-ruleset)
  - [`SYS_landlock_add_rule`](#sys-landlock-add-rule)
  - [`SYS_landlock_restrict_self`](#sys-landlock-restrict-self)
  - [`SYS_memfd_secret`](#sys-memfd-secret)
  - [`SYS_process_mrelease`](#sys-process-mrelease)
  - [`SYS_futex_waitv`](#sys-futex-waitv)
  - [`SYS_set_mempolicy_home_node`](#sys-set-mempolicy-home-node)
  - [`SYS_fchmodat2`](#sys-fchmodat2)
  - [`SYS_mseal`](#sys-mseal)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`statvfs`](#statvfs) | struct |  |
| [`sysctl`](#sysctl) | fn |  |
| [`__SIZEOF_PTHREAD_MUTEX_T`](#sizeof-pthread-mutex-t) | const |  |
| [`__SIZEOF_PTHREAD_RWLOCK_T`](#sizeof-pthread-rwlock-t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIER_T`](#sizeof-pthread-barrier-t) | const |  |
| [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread-recursive-mutex-initializer-np) | const |  |
| [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread-errorcheck-mutex-initializer-np) | const |  |
| [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread-adaptive-mutex-initializer-np) | const |  |
| [`SYS_read`](#sys-read) | const |  |
| [`SYS_write`](#sys-write) | const |  |
| [`SYS_open`](#sys-open) | const |  |
| [`SYS_close`](#sys-close) | const |  |
| [`SYS_stat`](#sys-stat) | const |  |
| [`SYS_fstat`](#sys-fstat) | const |  |
| [`SYS_lstat`](#sys-lstat) | const |  |
| [`SYS_poll`](#sys-poll) | const |  |
| [`SYS_lseek`](#sys-lseek) | const |  |
| [`SYS_mmap`](#sys-mmap) | const |  |
| [`SYS_mprotect`](#sys-mprotect) | const |  |
| [`SYS_munmap`](#sys-munmap) | const |  |
| [`SYS_brk`](#sys-brk) | const |  |
| [`SYS_rt_sigaction`](#sys-rt-sigaction) | const |  |
| [`SYS_rt_sigprocmask`](#sys-rt-sigprocmask) | const |  |
| [`SYS_rt_sigreturn`](#sys-rt-sigreturn) | const |  |
| [`SYS_ioctl`](#sys-ioctl) | const |  |
| [`SYS_pread64`](#sys-pread64) | const |  |
| [`SYS_pwrite64`](#sys-pwrite64) | const |  |
| [`SYS_readv`](#sys-readv) | const |  |
| [`SYS_writev`](#sys-writev) | const |  |
| [`SYS_access`](#sys-access) | const |  |
| [`SYS_pipe`](#sys-pipe) | const |  |
| [`SYS_select`](#sys-select) | const |  |
| [`SYS_sched_yield`](#sys-sched-yield) | const |  |
| [`SYS_mremap`](#sys-mremap) | const |  |
| [`SYS_msync`](#sys-msync) | const |  |
| [`SYS_mincore`](#sys-mincore) | const |  |
| [`SYS_madvise`](#sys-madvise) | const |  |
| [`SYS_shmget`](#sys-shmget) | const |  |
| [`SYS_shmat`](#sys-shmat) | const |  |
| [`SYS_shmctl`](#sys-shmctl) | const |  |
| [`SYS_dup`](#sys-dup) | const |  |
| [`SYS_dup2`](#sys-dup2) | const |  |
| [`SYS_pause`](#sys-pause) | const |  |
| [`SYS_nanosleep`](#sys-nanosleep) | const |  |
| [`SYS_getitimer`](#sys-getitimer) | const |  |
| [`SYS_alarm`](#sys-alarm) | const |  |
| [`SYS_setitimer`](#sys-setitimer) | const |  |
| [`SYS_getpid`](#sys-getpid) | const |  |
| [`SYS_sendfile`](#sys-sendfile) | const |  |
| [`SYS_socket`](#sys-socket) | const |  |
| [`SYS_connect`](#sys-connect) | const |  |
| [`SYS_accept`](#sys-accept) | const |  |
| [`SYS_sendto`](#sys-sendto) | const |  |
| [`SYS_recvfrom`](#sys-recvfrom) | const |  |
| [`SYS_sendmsg`](#sys-sendmsg) | const |  |
| [`SYS_recvmsg`](#sys-recvmsg) | const |  |
| [`SYS_shutdown`](#sys-shutdown) | const |  |
| [`SYS_bind`](#sys-bind) | const |  |
| [`SYS_listen`](#sys-listen) | const |  |
| [`SYS_getsockname`](#sys-getsockname) | const |  |
| [`SYS_getpeername`](#sys-getpeername) | const |  |
| [`SYS_socketpair`](#sys-socketpair) | const |  |
| [`SYS_setsockopt`](#sys-setsockopt) | const |  |
| [`SYS_getsockopt`](#sys-getsockopt) | const |  |
| [`SYS_clone`](#sys-clone) | const |  |
| [`SYS_fork`](#sys-fork) | const |  |
| [`SYS_vfork`](#sys-vfork) | const |  |
| [`SYS_execve`](#sys-execve) | const |  |
| [`SYS_exit`](#sys-exit) | const |  |
| [`SYS_wait4`](#sys-wait4) | const |  |
| [`SYS_kill`](#sys-kill) | const |  |
| [`SYS_uname`](#sys-uname) | const |  |
| [`SYS_semget`](#sys-semget) | const |  |
| [`SYS_semop`](#sys-semop) | const |  |
| [`SYS_semctl`](#sys-semctl) | const |  |
| [`SYS_shmdt`](#sys-shmdt) | const |  |
| [`SYS_msgget`](#sys-msgget) | const |  |
| [`SYS_msgsnd`](#sys-msgsnd) | const |  |
| [`SYS_msgrcv`](#sys-msgrcv) | const |  |
| [`SYS_msgctl`](#sys-msgctl) | const |  |
| [`SYS_fcntl`](#sys-fcntl) | const |  |
| [`SYS_flock`](#sys-flock) | const |  |
| [`SYS_fsync`](#sys-fsync) | const |  |
| [`SYS_fdatasync`](#sys-fdatasync) | const |  |
| [`SYS_truncate`](#sys-truncate) | const |  |
| [`SYS_ftruncate`](#sys-ftruncate) | const |  |
| [`SYS_getdents`](#sys-getdents) | const |  |
| [`SYS_getcwd`](#sys-getcwd) | const |  |
| [`SYS_chdir`](#sys-chdir) | const |  |
| [`SYS_fchdir`](#sys-fchdir) | const |  |
| [`SYS_rename`](#sys-rename) | const |  |
| [`SYS_mkdir`](#sys-mkdir) | const |  |
| [`SYS_rmdir`](#sys-rmdir) | const |  |
| [`SYS_creat`](#sys-creat) | const |  |
| [`SYS_link`](#sys-link) | const |  |
| [`SYS_unlink`](#sys-unlink) | const |  |
| [`SYS_symlink`](#sys-symlink) | const |  |
| [`SYS_readlink`](#sys-readlink) | const |  |
| [`SYS_chmod`](#sys-chmod) | const |  |
| [`SYS_fchmod`](#sys-fchmod) | const |  |
| [`SYS_chown`](#sys-chown) | const |  |
| [`SYS_fchown`](#sys-fchown) | const |  |
| [`SYS_lchown`](#sys-lchown) | const |  |
| [`SYS_umask`](#sys-umask) | const |  |
| [`SYS_gettimeofday`](#sys-gettimeofday) | const |  |
| [`SYS_getrlimit`](#sys-getrlimit) | const |  |
| [`SYS_getrusage`](#sys-getrusage) | const |  |
| [`SYS_sysinfo`](#sys-sysinfo) | const |  |
| [`SYS_times`](#sys-times) | const |  |
| [`SYS_ptrace`](#sys-ptrace) | const |  |
| [`SYS_getuid`](#sys-getuid) | const |  |
| [`SYS_syslog`](#sys-syslog) | const |  |
| [`SYS_getgid`](#sys-getgid) | const |  |
| [`SYS_setuid`](#sys-setuid) | const |  |
| [`SYS_setgid`](#sys-setgid) | const |  |
| [`SYS_geteuid`](#sys-geteuid) | const |  |
| [`SYS_getegid`](#sys-getegid) | const |  |
| [`SYS_setpgid`](#sys-setpgid) | const |  |
| [`SYS_getppid`](#sys-getppid) | const |  |
| [`SYS_getpgrp`](#sys-getpgrp) | const |  |
| [`SYS_setsid`](#sys-setsid) | const |  |
| [`SYS_setreuid`](#sys-setreuid) | const |  |
| [`SYS_setregid`](#sys-setregid) | const |  |
| [`SYS_getgroups`](#sys-getgroups) | const |  |
| [`SYS_setgroups`](#sys-setgroups) | const |  |
| [`SYS_setresuid`](#sys-setresuid) | const |  |
| [`SYS_getresuid`](#sys-getresuid) | const |  |
| [`SYS_setresgid`](#sys-setresgid) | const |  |
| [`SYS_getresgid`](#sys-getresgid) | const |  |
| [`SYS_getpgid`](#sys-getpgid) | const |  |
| [`SYS_setfsuid`](#sys-setfsuid) | const |  |
| [`SYS_setfsgid`](#sys-setfsgid) | const |  |
| [`SYS_getsid`](#sys-getsid) | const |  |
| [`SYS_capget`](#sys-capget) | const |  |
| [`SYS_capset`](#sys-capset) | const |  |
| [`SYS_rt_sigpending`](#sys-rt-sigpending) | const |  |
| [`SYS_rt_sigtimedwait`](#sys-rt-sigtimedwait) | const |  |
| [`SYS_rt_sigqueueinfo`](#sys-rt-sigqueueinfo) | const |  |
| [`SYS_rt_sigsuspend`](#sys-rt-sigsuspend) | const |  |
| [`SYS_sigaltstack`](#sys-sigaltstack) | const |  |
| [`SYS_utime`](#sys-utime) | const |  |
| [`SYS_mknod`](#sys-mknod) | const |  |
| [`SYS_uselib`](#sys-uselib) | const |  |
| [`SYS_personality`](#sys-personality) | const |  |
| [`SYS_ustat`](#sys-ustat) | const |  |
| [`SYS_statfs`](#sys-statfs) | const |  |
| [`SYS_fstatfs`](#sys-fstatfs) | const |  |
| [`SYS_sysfs`](#sys-sysfs) | const |  |
| [`SYS_getpriority`](#sys-getpriority) | const |  |
| [`SYS_setpriority`](#sys-setpriority) | const |  |
| [`SYS_sched_setparam`](#sys-sched-setparam) | const |  |
| [`SYS_sched_getparam`](#sys-sched-getparam) | const |  |
| [`SYS_sched_setscheduler`](#sys-sched-setscheduler) | const |  |
| [`SYS_sched_getscheduler`](#sys-sched-getscheduler) | const |  |
| [`SYS_sched_get_priority_max`](#sys-sched-get-priority-max) | const |  |
| [`SYS_sched_get_priority_min`](#sys-sched-get-priority-min) | const |  |
| [`SYS_sched_rr_get_interval`](#sys-sched-rr-get-interval) | const |  |
| [`SYS_mlock`](#sys-mlock) | const |  |
| [`SYS_munlock`](#sys-munlock) | const |  |
| [`SYS_mlockall`](#sys-mlockall) | const |  |
| [`SYS_munlockall`](#sys-munlockall) | const |  |
| [`SYS_vhangup`](#sys-vhangup) | const |  |
| [`SYS_modify_ldt`](#sys-modify-ldt) | const |  |
| [`SYS_pivot_root`](#sys-pivot-root) | const |  |
| [`SYS__sysctl`](#sys-sysctl) | const |  |
| [`SYS_prctl`](#sys-prctl) | const |  |
| [`SYS_arch_prctl`](#sys-arch-prctl) | const |  |
| [`SYS_adjtimex`](#sys-adjtimex) | const |  |
| [`SYS_setrlimit`](#sys-setrlimit) | const |  |
| [`SYS_chroot`](#sys-chroot) | const |  |
| [`SYS_sync`](#sys-sync) | const |  |
| [`SYS_acct`](#sys-acct) | const |  |
| [`SYS_settimeofday`](#sys-settimeofday) | const |  |
| [`SYS_mount`](#sys-mount) | const |  |
| [`SYS_umount2`](#sys-umount2) | const |  |
| [`SYS_swapon`](#sys-swapon) | const |  |
| [`SYS_swapoff`](#sys-swapoff) | const |  |
| [`SYS_reboot`](#sys-reboot) | const |  |
| [`SYS_sethostname`](#sys-sethostname) | const |  |
| [`SYS_setdomainname`](#sys-setdomainname) | const |  |
| [`SYS_iopl`](#sys-iopl) | const |  |
| [`SYS_ioperm`](#sys-ioperm) | const |  |
| [`SYS_create_module`](#sys-create-module) | const |  |
| [`SYS_init_module`](#sys-init-module) | const |  |
| [`SYS_delete_module`](#sys-delete-module) | const |  |
| [`SYS_get_kernel_syms`](#sys-get-kernel-syms) | const |  |
| [`SYS_query_module`](#sys-query-module) | const |  |
| [`SYS_quotactl`](#sys-quotactl) | const |  |
| [`SYS_nfsservctl`](#sys-nfsservctl) | const |  |
| [`SYS_getpmsg`](#sys-getpmsg) | const |  |
| [`SYS_putpmsg`](#sys-putpmsg) | const |  |
| [`SYS_afs_syscall`](#sys-afs-syscall) | const |  |
| [`SYS_tuxcall`](#sys-tuxcall) | const |  |
| [`SYS_security`](#sys-security) | const |  |
| [`SYS_gettid`](#sys-gettid) | const |  |
| [`SYS_readahead`](#sys-readahead) | const |  |
| [`SYS_setxattr`](#sys-setxattr) | const |  |
| [`SYS_lsetxattr`](#sys-lsetxattr) | const |  |
| [`SYS_fsetxattr`](#sys-fsetxattr) | const |  |
| [`SYS_getxattr`](#sys-getxattr) | const |  |
| [`SYS_lgetxattr`](#sys-lgetxattr) | const |  |
| [`SYS_fgetxattr`](#sys-fgetxattr) | const |  |
| [`SYS_listxattr`](#sys-listxattr) | const |  |
| [`SYS_llistxattr`](#sys-llistxattr) | const |  |
| [`SYS_flistxattr`](#sys-flistxattr) | const |  |
| [`SYS_removexattr`](#sys-removexattr) | const |  |
| [`SYS_lremovexattr`](#sys-lremovexattr) | const |  |
| [`SYS_fremovexattr`](#sys-fremovexattr) | const |  |
| [`SYS_tkill`](#sys-tkill) | const |  |
| [`SYS_time`](#sys-time) | const |  |
| [`SYS_futex`](#sys-futex) | const |  |
| [`SYS_sched_setaffinity`](#sys-sched-setaffinity) | const |  |
| [`SYS_sched_getaffinity`](#sys-sched-getaffinity) | const |  |
| [`SYS_set_thread_area`](#sys-set-thread-area) | const |  |
| [`SYS_io_setup`](#sys-io-setup) | const |  |
| [`SYS_io_destroy`](#sys-io-destroy) | const |  |
| [`SYS_io_getevents`](#sys-io-getevents) | const |  |
| [`SYS_io_submit`](#sys-io-submit) | const |  |
| [`SYS_io_cancel`](#sys-io-cancel) | const |  |
| [`SYS_get_thread_area`](#sys-get-thread-area) | const |  |
| [`SYS_lookup_dcookie`](#sys-lookup-dcookie) | const |  |
| [`SYS_epoll_create`](#sys-epoll-create) | const |  |
| [`SYS_epoll_ctl_old`](#sys-epoll-ctl-old) | const |  |
| [`SYS_epoll_wait_old`](#sys-epoll-wait-old) | const |  |
| [`SYS_remap_file_pages`](#sys-remap-file-pages) | const |  |
| [`SYS_getdents64`](#sys-getdents64) | const |  |
| [`SYS_set_tid_address`](#sys-set-tid-address) | const |  |
| [`SYS_restart_syscall`](#sys-restart-syscall) | const |  |
| [`SYS_semtimedop`](#sys-semtimedop) | const |  |
| [`SYS_fadvise64`](#sys-fadvise64) | const |  |
| [`SYS_timer_create`](#sys-timer-create) | const |  |
| [`SYS_timer_settime`](#sys-timer-settime) | const |  |
| [`SYS_timer_gettime`](#sys-timer-gettime) | const |  |
| [`SYS_timer_getoverrun`](#sys-timer-getoverrun) | const |  |
| [`SYS_timer_delete`](#sys-timer-delete) | const |  |
| [`SYS_clock_settime`](#sys-clock-settime) | const |  |
| [`SYS_clock_gettime`](#sys-clock-gettime) | const |  |
| [`SYS_clock_getres`](#sys-clock-getres) | const |  |
| [`SYS_clock_nanosleep`](#sys-clock-nanosleep) | const |  |
| [`SYS_exit_group`](#sys-exit-group) | const |  |
| [`SYS_epoll_wait`](#sys-epoll-wait) | const |  |
| [`SYS_epoll_ctl`](#sys-epoll-ctl) | const |  |
| [`SYS_tgkill`](#sys-tgkill) | const |  |
| [`SYS_utimes`](#sys-utimes) | const |  |
| [`SYS_vserver`](#sys-vserver) | const |  |
| [`SYS_mbind`](#sys-mbind) | const |  |
| [`SYS_set_mempolicy`](#sys-set-mempolicy) | const |  |
| [`SYS_get_mempolicy`](#sys-get-mempolicy) | const |  |
| [`SYS_mq_open`](#sys-mq-open) | const |  |
| [`SYS_mq_unlink`](#sys-mq-unlink) | const |  |
| [`SYS_mq_timedsend`](#sys-mq-timedsend) | const |  |
| [`SYS_mq_timedreceive`](#sys-mq-timedreceive) | const |  |
| [`SYS_mq_notify`](#sys-mq-notify) | const |  |
| [`SYS_mq_getsetattr`](#sys-mq-getsetattr) | const |  |
| [`SYS_kexec_load`](#sys-kexec-load) | const |  |
| [`SYS_waitid`](#sys-waitid) | const |  |
| [`SYS_add_key`](#sys-add-key) | const |  |
| [`SYS_request_key`](#sys-request-key) | const |  |
| [`SYS_keyctl`](#sys-keyctl) | const |  |
| [`SYS_ioprio_set`](#sys-ioprio-set) | const |  |
| [`SYS_ioprio_get`](#sys-ioprio-get) | const |  |
| [`SYS_inotify_init`](#sys-inotify-init) | const |  |
| [`SYS_inotify_add_watch`](#sys-inotify-add-watch) | const |  |
| [`SYS_inotify_rm_watch`](#sys-inotify-rm-watch) | const |  |
| [`SYS_migrate_pages`](#sys-migrate-pages) | const |  |
| [`SYS_openat`](#sys-openat) | const |  |
| [`SYS_mkdirat`](#sys-mkdirat) | const |  |
| [`SYS_mknodat`](#sys-mknodat) | const |  |
| [`SYS_fchownat`](#sys-fchownat) | const |  |
| [`SYS_futimesat`](#sys-futimesat) | const |  |
| [`SYS_newfstatat`](#sys-newfstatat) | const |  |
| [`SYS_unlinkat`](#sys-unlinkat) | const |  |
| [`SYS_renameat`](#sys-renameat) | const |  |
| [`SYS_linkat`](#sys-linkat) | const |  |
| [`SYS_symlinkat`](#sys-symlinkat) | const |  |
| [`SYS_readlinkat`](#sys-readlinkat) | const |  |
| [`SYS_fchmodat`](#sys-fchmodat) | const |  |
| [`SYS_faccessat`](#sys-faccessat) | const |  |
| [`SYS_pselect6`](#sys-pselect6) | const |  |
| [`SYS_ppoll`](#sys-ppoll) | const |  |
| [`SYS_unshare`](#sys-unshare) | const |  |
| [`SYS_set_robust_list`](#sys-set-robust-list) | const |  |
| [`SYS_get_robust_list`](#sys-get-robust-list) | const |  |
| [`SYS_splice`](#sys-splice) | const |  |
| [`SYS_tee`](#sys-tee) | const |  |
| [`SYS_sync_file_range`](#sys-sync-file-range) | const |  |
| [`SYS_vmsplice`](#sys-vmsplice) | const |  |
| [`SYS_move_pages`](#sys-move-pages) | const |  |
| [`SYS_utimensat`](#sys-utimensat) | const |  |
| [`SYS_epoll_pwait`](#sys-epoll-pwait) | const |  |
| [`SYS_signalfd`](#sys-signalfd) | const |  |
| [`SYS_timerfd_create`](#sys-timerfd-create) | const |  |
| [`SYS_eventfd`](#sys-eventfd) | const |  |
| [`SYS_fallocate`](#sys-fallocate) | const |  |
| [`SYS_timerfd_settime`](#sys-timerfd-settime) | const |  |
| [`SYS_timerfd_gettime`](#sys-timerfd-gettime) | const |  |
| [`SYS_accept4`](#sys-accept4) | const |  |
| [`SYS_signalfd4`](#sys-signalfd4) | const |  |
| [`SYS_eventfd2`](#sys-eventfd2) | const |  |
| [`SYS_epoll_create1`](#sys-epoll-create1) | const |  |
| [`SYS_dup3`](#sys-dup3) | const |  |
| [`SYS_pipe2`](#sys-pipe2) | const |  |
| [`SYS_inotify_init1`](#sys-inotify-init1) | const |  |
| [`SYS_preadv`](#sys-preadv) | const |  |
| [`SYS_pwritev`](#sys-pwritev) | const |  |
| [`SYS_rt_tgsigqueueinfo`](#sys-rt-tgsigqueueinfo) | const |  |
| [`SYS_perf_event_open`](#sys-perf-event-open) | const |  |
| [`SYS_recvmmsg`](#sys-recvmmsg) | const |  |
| [`SYS_fanotify_init`](#sys-fanotify-init) | const |  |
| [`SYS_fanotify_mark`](#sys-fanotify-mark) | const |  |
| [`SYS_prlimit64`](#sys-prlimit64) | const |  |
| [`SYS_name_to_handle_at`](#sys-name-to-handle-at) | const |  |
| [`SYS_open_by_handle_at`](#sys-open-by-handle-at) | const |  |
| [`SYS_clock_adjtime`](#sys-clock-adjtime) | const |  |
| [`SYS_syncfs`](#sys-syncfs) | const |  |
| [`SYS_sendmmsg`](#sys-sendmmsg) | const |  |
| [`SYS_setns`](#sys-setns) | const |  |
| [`SYS_getcpu`](#sys-getcpu) | const |  |
| [`SYS_process_vm_readv`](#sys-process-vm-readv) | const |  |
| [`SYS_process_vm_writev`](#sys-process-vm-writev) | const |  |
| [`SYS_kcmp`](#sys-kcmp) | const |  |
| [`SYS_finit_module`](#sys-finit-module) | const |  |
| [`SYS_sched_setattr`](#sys-sched-setattr) | const |  |
| [`SYS_sched_getattr`](#sys-sched-getattr) | const |  |
| [`SYS_renameat2`](#sys-renameat2) | const |  |
| [`SYS_seccomp`](#sys-seccomp) | const |  |
| [`SYS_getrandom`](#sys-getrandom) | const |  |
| [`SYS_memfd_create`](#sys-memfd-create) | const |  |
| [`SYS_kexec_file_load`](#sys-kexec-file-load) | const |  |
| [`SYS_bpf`](#sys-bpf) | const |  |
| [`SYS_execveat`](#sys-execveat) | const |  |
| [`SYS_userfaultfd`](#sys-userfaultfd) | const |  |
| [`SYS_membarrier`](#sys-membarrier) | const |  |
| [`SYS_mlock2`](#sys-mlock2) | const |  |
| [`SYS_copy_file_range`](#sys-copy-file-range) | const |  |
| [`SYS_preadv2`](#sys-preadv2) | const |  |
| [`SYS_pwritev2`](#sys-pwritev2) | const |  |
| [`SYS_pkey_mprotect`](#sys-pkey-mprotect) | const |  |
| [`SYS_pkey_alloc`](#sys-pkey-alloc) | const |  |
| [`SYS_pkey_free`](#sys-pkey-free) | const |  |
| [`SYS_statx`](#sys-statx) | const |  |
| [`SYS_rseq`](#sys-rseq) | const |  |
| [`SYS_pidfd_send_signal`](#sys-pidfd-send-signal) | const |  |
| [`SYS_io_uring_setup`](#sys-io-uring-setup) | const |  |
| [`SYS_io_uring_enter`](#sys-io-uring-enter) | const |  |
| [`SYS_io_uring_register`](#sys-io-uring-register) | const |  |
| [`SYS_open_tree`](#sys-open-tree) | const |  |
| [`SYS_move_mount`](#sys-move-mount) | const |  |
| [`SYS_fsopen`](#sys-fsopen) | const |  |
| [`SYS_fsconfig`](#sys-fsconfig) | const |  |
| [`SYS_fsmount`](#sys-fsmount) | const |  |
| [`SYS_fspick`](#sys-fspick) | const |  |
| [`SYS_pidfd_open`](#sys-pidfd-open) | const |  |
| [`SYS_clone3`](#sys-clone3) | const |  |
| [`SYS_close_range`](#sys-close-range) | const |  |
| [`SYS_openat2`](#sys-openat2) | const |  |
| [`SYS_pidfd_getfd`](#sys-pidfd-getfd) | const |  |
| [`SYS_faccessat2`](#sys-faccessat2) | const |  |
| [`SYS_process_madvise`](#sys-process-madvise) | const |  |
| [`SYS_epoll_pwait2`](#sys-epoll-pwait2) | const |  |
| [`SYS_mount_setattr`](#sys-mount-setattr) | const |  |
| [`SYS_quotactl_fd`](#sys-quotactl-fd) | const |  |
| [`SYS_landlock_create_ruleset`](#sys-landlock-create-ruleset) | const |  |
| [`SYS_landlock_add_rule`](#sys-landlock-add-rule) | const |  |
| [`SYS_landlock_restrict_self`](#sys-landlock-restrict-self) | const |  |
| [`SYS_memfd_secret`](#sys-memfd-secret) | const |  |
| [`SYS_process_mrelease`](#sys-process-mrelease) | const |  |
| [`SYS_futex_waitv`](#sys-futex-waitv) | const |  |
| [`SYS_set_mempolicy_home_node`](#sys-set-mempolicy-home-node) | const |  |
| [`SYS_fchmodat2`](#sys-fchmodat2) | const |  |
| [`SYS_mseal`](#sys-mseal) | const |  |

## Structs

### `statvfs`

```rust
struct statvfs {
    pub f_bsize: crate::c_ulong,
    pub f_frsize: crate::c_ulong,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_favail: crate::fsfilcnt_t,
    pub f_fsid: crate::c_ulong,
    pub f_flag: crate::c_ulong,
    pub f_namemax: crate::c_ulong,
    __f_spare: [crate::c_int; 6],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:4-19`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L4-L19)*

#### Trait Implementations

##### `impl Clone for statvfs`

- <span id="statvfs-clone"></span>`fn clone(&self) -> statvfs` — [`statvfs`](../index.md#statvfs)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- <span id="statvfs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:438-445`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L438-L445)*

## Constants

### `__SIZEOF_PTHREAD_MUTEX_T`
```rust
const __SIZEOF_PTHREAD_MUTEX_T: usize = 40usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:21`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L21)*

### `__SIZEOF_PTHREAD_RWLOCK_T`
```rust
const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:22`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L22)*

### `__SIZEOF_PTHREAD_BARRIER_T`
```rust
const __SIZEOF_PTHREAD_BARRIER_T: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:23`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L23)*

### `PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:26-31`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L26-L31)*

### `PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:33-38`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L33-L38)*

### `PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:40-45`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L40-L45)*

### `SYS_read`
```rust
const SYS_read: crate::c_long = 0i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:70`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L70)*

### `SYS_write`
```rust
const SYS_write: crate::c_long = 1i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:71`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L71)*

### `SYS_open`
```rust
const SYS_open: crate::c_long = 2i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:72`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L72)*

### `SYS_close`
```rust
const SYS_close: crate::c_long = 3i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:73`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L73)*

### `SYS_stat`
```rust
const SYS_stat: crate::c_long = 4i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:74`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L74)*

### `SYS_fstat`
```rust
const SYS_fstat: crate::c_long = 5i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:75`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L75)*

### `SYS_lstat`
```rust
const SYS_lstat: crate::c_long = 6i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:76`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L76)*

### `SYS_poll`
```rust
const SYS_poll: crate::c_long = 7i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:77`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L77)*

### `SYS_lseek`
```rust
const SYS_lseek: crate::c_long = 8i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:78`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L78)*

### `SYS_mmap`
```rust
const SYS_mmap: crate::c_long = 9i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:79`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L79)*

### `SYS_mprotect`
```rust
const SYS_mprotect: crate::c_long = 10i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:80`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L80)*

### `SYS_munmap`
```rust
const SYS_munmap: crate::c_long = 11i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:81`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L81)*

### `SYS_brk`
```rust
const SYS_brk: crate::c_long = 12i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:82`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L82)*

### `SYS_rt_sigaction`
```rust
const SYS_rt_sigaction: crate::c_long = 13i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:83`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L83)*

### `SYS_rt_sigprocmask`
```rust
const SYS_rt_sigprocmask: crate::c_long = 14i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:84`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L84)*

### `SYS_rt_sigreturn`
```rust
const SYS_rt_sigreturn: crate::c_long = 15i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:85`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L85)*

### `SYS_ioctl`
```rust
const SYS_ioctl: crate::c_long = 16i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:86`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L86)*

### `SYS_pread64`
```rust
const SYS_pread64: crate::c_long = 17i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:87`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L87)*

### `SYS_pwrite64`
```rust
const SYS_pwrite64: crate::c_long = 18i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:88`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L88)*

### `SYS_readv`
```rust
const SYS_readv: crate::c_long = 19i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:89`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L89)*

### `SYS_writev`
```rust
const SYS_writev: crate::c_long = 20i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:90`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L90)*

### `SYS_access`
```rust
const SYS_access: crate::c_long = 21i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:91`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L91)*

### `SYS_pipe`
```rust
const SYS_pipe: crate::c_long = 22i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:92`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L92)*

### `SYS_select`
```rust
const SYS_select: crate::c_long = 23i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:93`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L93)*

### `SYS_sched_yield`
```rust
const SYS_sched_yield: crate::c_long = 24i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:94`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L94)*

### `SYS_mremap`
```rust
const SYS_mremap: crate::c_long = 25i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:95`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L95)*

### `SYS_msync`
```rust
const SYS_msync: crate::c_long = 26i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:96`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L96)*

### `SYS_mincore`
```rust
const SYS_mincore: crate::c_long = 27i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:97`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L97)*

### `SYS_madvise`
```rust
const SYS_madvise: crate::c_long = 28i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:98`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L98)*

### `SYS_shmget`
```rust
const SYS_shmget: crate::c_long = 29i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:99`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L99)*

### `SYS_shmat`
```rust
const SYS_shmat: crate::c_long = 30i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:100`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L100)*

### `SYS_shmctl`
```rust
const SYS_shmctl: crate::c_long = 31i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:101`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L101)*

### `SYS_dup`
```rust
const SYS_dup: crate::c_long = 32i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:102`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L102)*

### `SYS_dup2`
```rust
const SYS_dup2: crate::c_long = 33i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:103`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L103)*

### `SYS_pause`
```rust
const SYS_pause: crate::c_long = 34i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:104`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L104)*

### `SYS_nanosleep`
```rust
const SYS_nanosleep: crate::c_long = 35i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:105`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L105)*

### `SYS_getitimer`
```rust
const SYS_getitimer: crate::c_long = 36i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:106`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L106)*

### `SYS_alarm`
```rust
const SYS_alarm: crate::c_long = 37i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:107`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L107)*

### `SYS_setitimer`
```rust
const SYS_setitimer: crate::c_long = 38i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:108`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L108)*

### `SYS_getpid`
```rust
const SYS_getpid: crate::c_long = 39i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:109`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L109)*

### `SYS_sendfile`
```rust
const SYS_sendfile: crate::c_long = 40i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:110`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L110)*

### `SYS_socket`
```rust
const SYS_socket: crate::c_long = 41i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:111`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L111)*

### `SYS_connect`
```rust
const SYS_connect: crate::c_long = 42i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:112`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L112)*

### `SYS_accept`
```rust
const SYS_accept: crate::c_long = 43i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:113`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L113)*

### `SYS_sendto`
```rust
const SYS_sendto: crate::c_long = 44i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:114`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L114)*

### `SYS_recvfrom`
```rust
const SYS_recvfrom: crate::c_long = 45i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:115`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L115)*

### `SYS_sendmsg`
```rust
const SYS_sendmsg: crate::c_long = 46i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:116`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L116)*

### `SYS_recvmsg`
```rust
const SYS_recvmsg: crate::c_long = 47i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:117`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L117)*

### `SYS_shutdown`
```rust
const SYS_shutdown: crate::c_long = 48i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:118`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L118)*

### `SYS_bind`
```rust
const SYS_bind: crate::c_long = 49i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:119`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L119)*

### `SYS_listen`
```rust
const SYS_listen: crate::c_long = 50i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:120`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L120)*

### `SYS_getsockname`
```rust
const SYS_getsockname: crate::c_long = 51i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:121`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L121)*

### `SYS_getpeername`
```rust
const SYS_getpeername: crate::c_long = 52i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:122`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L122)*

### `SYS_socketpair`
```rust
const SYS_socketpair: crate::c_long = 53i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:123`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L123)*

### `SYS_setsockopt`
```rust
const SYS_setsockopt: crate::c_long = 54i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:124`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L124)*

### `SYS_getsockopt`
```rust
const SYS_getsockopt: crate::c_long = 55i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:125`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L125)*

### `SYS_clone`
```rust
const SYS_clone: crate::c_long = 56i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:126`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L126)*

### `SYS_fork`
```rust
const SYS_fork: crate::c_long = 57i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:127`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L127)*

### `SYS_vfork`
```rust
const SYS_vfork: crate::c_long = 58i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:128`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L128)*

### `SYS_execve`
```rust
const SYS_execve: crate::c_long = 59i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:129`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L129)*

### `SYS_exit`
```rust
const SYS_exit: crate::c_long = 60i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:130`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L130)*

### `SYS_wait4`
```rust
const SYS_wait4: crate::c_long = 61i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:131`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L131)*

### `SYS_kill`
```rust
const SYS_kill: crate::c_long = 62i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:132`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L132)*

### `SYS_uname`
```rust
const SYS_uname: crate::c_long = 63i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:133`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L133)*

### `SYS_semget`
```rust
const SYS_semget: crate::c_long = 64i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:134`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L134)*

### `SYS_semop`
```rust
const SYS_semop: crate::c_long = 65i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:135`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L135)*

### `SYS_semctl`
```rust
const SYS_semctl: crate::c_long = 66i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:136`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L136)*

### `SYS_shmdt`
```rust
const SYS_shmdt: crate::c_long = 67i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:137`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L137)*

### `SYS_msgget`
```rust
const SYS_msgget: crate::c_long = 68i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:138`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L138)*

### `SYS_msgsnd`
```rust
const SYS_msgsnd: crate::c_long = 69i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:139`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L139)*

### `SYS_msgrcv`
```rust
const SYS_msgrcv: crate::c_long = 70i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:140`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L140)*

### `SYS_msgctl`
```rust
const SYS_msgctl: crate::c_long = 71i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:141`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L141)*

### `SYS_fcntl`
```rust
const SYS_fcntl: crate::c_long = 72i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:142`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L142)*

### `SYS_flock`
```rust
const SYS_flock: crate::c_long = 73i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:143`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L143)*

### `SYS_fsync`
```rust
const SYS_fsync: crate::c_long = 74i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:144`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L144)*

### `SYS_fdatasync`
```rust
const SYS_fdatasync: crate::c_long = 75i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:145`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L145)*

### `SYS_truncate`
```rust
const SYS_truncate: crate::c_long = 76i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:146`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L146)*

### `SYS_ftruncate`
```rust
const SYS_ftruncate: crate::c_long = 77i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:147`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L147)*

### `SYS_getdents`
```rust
const SYS_getdents: crate::c_long = 78i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:148`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L148)*

### `SYS_getcwd`
```rust
const SYS_getcwd: crate::c_long = 79i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:149`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L149)*

### `SYS_chdir`
```rust
const SYS_chdir: crate::c_long = 80i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:150`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L150)*

### `SYS_fchdir`
```rust
const SYS_fchdir: crate::c_long = 81i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:151`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L151)*

### `SYS_rename`
```rust
const SYS_rename: crate::c_long = 82i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:152`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L152)*

### `SYS_mkdir`
```rust
const SYS_mkdir: crate::c_long = 83i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:153`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L153)*

### `SYS_rmdir`
```rust
const SYS_rmdir: crate::c_long = 84i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:154`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L154)*

### `SYS_creat`
```rust
const SYS_creat: crate::c_long = 85i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:155`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L155)*

### `SYS_link`
```rust
const SYS_link: crate::c_long = 86i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:156`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L156)*

### `SYS_unlink`
```rust
const SYS_unlink: crate::c_long = 87i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:157`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L157)*

### `SYS_symlink`
```rust
const SYS_symlink: crate::c_long = 88i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:158`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L158)*

### `SYS_readlink`
```rust
const SYS_readlink: crate::c_long = 89i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:159`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L159)*

### `SYS_chmod`
```rust
const SYS_chmod: crate::c_long = 90i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:160`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L160)*

### `SYS_fchmod`
```rust
const SYS_fchmod: crate::c_long = 91i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:161`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L161)*

### `SYS_chown`
```rust
const SYS_chown: crate::c_long = 92i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:162`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L162)*

### `SYS_fchown`
```rust
const SYS_fchown: crate::c_long = 93i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:163`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L163)*

### `SYS_lchown`
```rust
const SYS_lchown: crate::c_long = 94i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:164`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L164)*

### `SYS_umask`
```rust
const SYS_umask: crate::c_long = 95i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:165`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L165)*

### `SYS_gettimeofday`
```rust
const SYS_gettimeofday: crate::c_long = 96i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:166`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L166)*

### `SYS_getrlimit`
```rust
const SYS_getrlimit: crate::c_long = 97i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:167`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L167)*

### `SYS_getrusage`
```rust
const SYS_getrusage: crate::c_long = 98i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:168`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L168)*

### `SYS_sysinfo`
```rust
const SYS_sysinfo: crate::c_long = 99i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:169`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L169)*

### `SYS_times`
```rust
const SYS_times: crate::c_long = 100i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:170`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L170)*

### `SYS_ptrace`
```rust
const SYS_ptrace: crate::c_long = 101i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:171`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L171)*

### `SYS_getuid`
```rust
const SYS_getuid: crate::c_long = 102i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:172`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L172)*

### `SYS_syslog`
```rust
const SYS_syslog: crate::c_long = 103i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:173`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L173)*

### `SYS_getgid`
```rust
const SYS_getgid: crate::c_long = 104i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:174`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L174)*

### `SYS_setuid`
```rust
const SYS_setuid: crate::c_long = 105i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:175`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L175)*

### `SYS_setgid`
```rust
const SYS_setgid: crate::c_long = 106i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:176`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L176)*

### `SYS_geteuid`
```rust
const SYS_geteuid: crate::c_long = 107i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:177`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L177)*

### `SYS_getegid`
```rust
const SYS_getegid: crate::c_long = 108i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:178`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L178)*

### `SYS_setpgid`
```rust
const SYS_setpgid: crate::c_long = 109i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:179`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L179)*

### `SYS_getppid`
```rust
const SYS_getppid: crate::c_long = 110i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:180`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L180)*

### `SYS_getpgrp`
```rust
const SYS_getpgrp: crate::c_long = 111i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:181`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L181)*

### `SYS_setsid`
```rust
const SYS_setsid: crate::c_long = 112i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:182`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L182)*

### `SYS_setreuid`
```rust
const SYS_setreuid: crate::c_long = 113i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:183`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L183)*

### `SYS_setregid`
```rust
const SYS_setregid: crate::c_long = 114i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:184`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L184)*

### `SYS_getgroups`
```rust
const SYS_getgroups: crate::c_long = 115i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:185`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L185)*

### `SYS_setgroups`
```rust
const SYS_setgroups: crate::c_long = 116i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:186`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L186)*

### `SYS_setresuid`
```rust
const SYS_setresuid: crate::c_long = 117i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:187`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L187)*

### `SYS_getresuid`
```rust
const SYS_getresuid: crate::c_long = 118i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:188`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L188)*

### `SYS_setresgid`
```rust
const SYS_setresgid: crate::c_long = 119i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:189`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L189)*

### `SYS_getresgid`
```rust
const SYS_getresgid: crate::c_long = 120i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:190`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L190)*

### `SYS_getpgid`
```rust
const SYS_getpgid: crate::c_long = 121i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:191`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L191)*

### `SYS_setfsuid`
```rust
const SYS_setfsuid: crate::c_long = 122i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:192`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L192)*

### `SYS_setfsgid`
```rust
const SYS_setfsgid: crate::c_long = 123i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:193`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L193)*

### `SYS_getsid`
```rust
const SYS_getsid: crate::c_long = 124i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:194`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L194)*

### `SYS_capget`
```rust
const SYS_capget: crate::c_long = 125i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:195`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L195)*

### `SYS_capset`
```rust
const SYS_capset: crate::c_long = 126i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:196`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L196)*

### `SYS_rt_sigpending`
```rust
const SYS_rt_sigpending: crate::c_long = 127i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:197`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L197)*

### `SYS_rt_sigtimedwait`
```rust
const SYS_rt_sigtimedwait: crate::c_long = 128i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:198`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L198)*

### `SYS_rt_sigqueueinfo`
```rust
const SYS_rt_sigqueueinfo: crate::c_long = 129i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:199`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L199)*

### `SYS_rt_sigsuspend`
```rust
const SYS_rt_sigsuspend: crate::c_long = 130i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:200`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L200)*

### `SYS_sigaltstack`
```rust
const SYS_sigaltstack: crate::c_long = 131i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:201`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L201)*

### `SYS_utime`
```rust
const SYS_utime: crate::c_long = 132i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:202`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L202)*

### `SYS_mknod`
```rust
const SYS_mknod: crate::c_long = 133i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:203`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L203)*

### `SYS_uselib`
```rust
const SYS_uselib: crate::c_long = 134i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:204`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L204)*

### `SYS_personality`
```rust
const SYS_personality: crate::c_long = 135i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:205`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L205)*

### `SYS_ustat`
```rust
const SYS_ustat: crate::c_long = 136i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:206`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L206)*

### `SYS_statfs`
```rust
const SYS_statfs: crate::c_long = 137i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:207`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L207)*

### `SYS_fstatfs`
```rust
const SYS_fstatfs: crate::c_long = 138i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:208`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L208)*

### `SYS_sysfs`
```rust
const SYS_sysfs: crate::c_long = 139i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:209`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L209)*

### `SYS_getpriority`
```rust
const SYS_getpriority: crate::c_long = 140i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:210`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L210)*

### `SYS_setpriority`
```rust
const SYS_setpriority: crate::c_long = 141i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:211`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L211)*

### `SYS_sched_setparam`
```rust
const SYS_sched_setparam: crate::c_long = 142i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:212`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L212)*

### `SYS_sched_getparam`
```rust
const SYS_sched_getparam: crate::c_long = 143i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:213`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L213)*

### `SYS_sched_setscheduler`
```rust
const SYS_sched_setscheduler: crate::c_long = 144i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:214`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L214)*

### `SYS_sched_getscheduler`
```rust
const SYS_sched_getscheduler: crate::c_long = 145i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:215`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L215)*

### `SYS_sched_get_priority_max`
```rust
const SYS_sched_get_priority_max: crate::c_long = 146i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:216`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L216)*

### `SYS_sched_get_priority_min`
```rust
const SYS_sched_get_priority_min: crate::c_long = 147i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:217`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L217)*

### `SYS_sched_rr_get_interval`
```rust
const SYS_sched_rr_get_interval: crate::c_long = 148i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:218`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L218)*

### `SYS_mlock`
```rust
const SYS_mlock: crate::c_long = 149i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:219`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L219)*

### `SYS_munlock`
```rust
const SYS_munlock: crate::c_long = 150i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:220`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L220)*

### `SYS_mlockall`
```rust
const SYS_mlockall: crate::c_long = 151i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:221`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L221)*

### `SYS_munlockall`
```rust
const SYS_munlockall: crate::c_long = 152i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:222`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L222)*

### `SYS_vhangup`
```rust
const SYS_vhangup: crate::c_long = 153i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:223`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L223)*

### `SYS_modify_ldt`
```rust
const SYS_modify_ldt: crate::c_long = 154i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:224`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L224)*

### `SYS_pivot_root`
```rust
const SYS_pivot_root: crate::c_long = 155i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:225`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L225)*

### `SYS__sysctl`
```rust
const SYS__sysctl: crate::c_long = 156i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:226`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L226)*

### `SYS_prctl`
```rust
const SYS_prctl: crate::c_long = 157i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:227`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L227)*

### `SYS_arch_prctl`
```rust
const SYS_arch_prctl: crate::c_long = 158i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:228`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L228)*

### `SYS_adjtimex`
```rust
const SYS_adjtimex: crate::c_long = 159i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:229`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L229)*

### `SYS_setrlimit`
```rust
const SYS_setrlimit: crate::c_long = 160i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:230`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L230)*

### `SYS_chroot`
```rust
const SYS_chroot: crate::c_long = 161i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:231`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L231)*

### `SYS_sync`
```rust
const SYS_sync: crate::c_long = 162i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:232`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L232)*

### `SYS_acct`
```rust
const SYS_acct: crate::c_long = 163i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:233`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L233)*

### `SYS_settimeofday`
```rust
const SYS_settimeofday: crate::c_long = 164i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:234`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L234)*

### `SYS_mount`
```rust
const SYS_mount: crate::c_long = 165i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:235`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L235)*

### `SYS_umount2`
```rust
const SYS_umount2: crate::c_long = 166i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:236`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L236)*

### `SYS_swapon`
```rust
const SYS_swapon: crate::c_long = 167i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:237`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L237)*

### `SYS_swapoff`
```rust
const SYS_swapoff: crate::c_long = 168i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:238`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L238)*

### `SYS_reboot`
```rust
const SYS_reboot: crate::c_long = 169i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:239`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L239)*

### `SYS_sethostname`
```rust
const SYS_sethostname: crate::c_long = 170i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:240`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L240)*

### `SYS_setdomainname`
```rust
const SYS_setdomainname: crate::c_long = 171i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:241`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L241)*

### `SYS_iopl`
```rust
const SYS_iopl: crate::c_long = 172i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:242`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L242)*

### `SYS_ioperm`
```rust
const SYS_ioperm: crate::c_long = 173i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:243`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L243)*

### `SYS_create_module`
```rust
const SYS_create_module: crate::c_long = 174i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:245`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L245)*

### `SYS_init_module`
```rust
const SYS_init_module: crate::c_long = 175i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:246`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L246)*

### `SYS_delete_module`
```rust
const SYS_delete_module: crate::c_long = 176i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:247`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L247)*

### `SYS_get_kernel_syms`
```rust
const SYS_get_kernel_syms: crate::c_long = 177i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:249`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L249)*

### `SYS_query_module`
```rust
const SYS_query_module: crate::c_long = 178i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:251`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L251)*

### `SYS_quotactl`
```rust
const SYS_quotactl: crate::c_long = 179i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:252`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L252)*

### `SYS_nfsservctl`
```rust
const SYS_nfsservctl: crate::c_long = 180i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:253`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L253)*

### `SYS_getpmsg`
```rust
const SYS_getpmsg: crate::c_long = 181i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:254`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L254)*

### `SYS_putpmsg`
```rust
const SYS_putpmsg: crate::c_long = 182i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:255`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L255)*

### `SYS_afs_syscall`
```rust
const SYS_afs_syscall: crate::c_long = 183i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:256`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L256)*

### `SYS_tuxcall`
```rust
const SYS_tuxcall: crate::c_long = 184i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:257`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L257)*

### `SYS_security`
```rust
const SYS_security: crate::c_long = 185i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:258`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L258)*

### `SYS_gettid`
```rust
const SYS_gettid: crate::c_long = 186i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:259`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L259)*

### `SYS_readahead`
```rust
const SYS_readahead: crate::c_long = 187i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:260`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L260)*

### `SYS_setxattr`
```rust
const SYS_setxattr: crate::c_long = 188i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:261`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L261)*

### `SYS_lsetxattr`
```rust
const SYS_lsetxattr: crate::c_long = 189i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:262`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L262)*

### `SYS_fsetxattr`
```rust
const SYS_fsetxattr: crate::c_long = 190i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:263`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L263)*

### `SYS_getxattr`
```rust
const SYS_getxattr: crate::c_long = 191i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:264`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L264)*

### `SYS_lgetxattr`
```rust
const SYS_lgetxattr: crate::c_long = 192i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:265`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L265)*

### `SYS_fgetxattr`
```rust
const SYS_fgetxattr: crate::c_long = 193i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:266`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L266)*

### `SYS_listxattr`
```rust
const SYS_listxattr: crate::c_long = 194i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:267`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L267)*

### `SYS_llistxattr`
```rust
const SYS_llistxattr: crate::c_long = 195i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:268`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L268)*

### `SYS_flistxattr`
```rust
const SYS_flistxattr: crate::c_long = 196i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:269`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L269)*

### `SYS_removexattr`
```rust
const SYS_removexattr: crate::c_long = 197i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:270`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L270)*

### `SYS_lremovexattr`
```rust
const SYS_lremovexattr: crate::c_long = 198i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:271`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L271)*

### `SYS_fremovexattr`
```rust
const SYS_fremovexattr: crate::c_long = 199i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:272`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L272)*

### `SYS_tkill`
```rust
const SYS_tkill: crate::c_long = 200i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:273`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L273)*

### `SYS_time`
```rust
const SYS_time: crate::c_long = 201i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:274`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L274)*

### `SYS_futex`
```rust
const SYS_futex: crate::c_long = 202i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:275`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L275)*

### `SYS_sched_setaffinity`
```rust
const SYS_sched_setaffinity: crate::c_long = 203i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:276`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L276)*

### `SYS_sched_getaffinity`
```rust
const SYS_sched_getaffinity: crate::c_long = 204i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:277`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L277)*

### `SYS_set_thread_area`
```rust
const SYS_set_thread_area: crate::c_long = 205i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:278`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L278)*

### `SYS_io_setup`
```rust
const SYS_io_setup: crate::c_long = 206i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:279`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L279)*

### `SYS_io_destroy`
```rust
const SYS_io_destroy: crate::c_long = 207i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:280`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L280)*

### `SYS_io_getevents`
```rust
const SYS_io_getevents: crate::c_long = 208i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:281`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L281)*

### `SYS_io_submit`
```rust
const SYS_io_submit: crate::c_long = 209i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:282`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L282)*

### `SYS_io_cancel`
```rust
const SYS_io_cancel: crate::c_long = 210i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:283`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L283)*

### `SYS_get_thread_area`
```rust
const SYS_get_thread_area: crate::c_long = 211i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:284`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L284)*

### `SYS_lookup_dcookie`
```rust
const SYS_lookup_dcookie: crate::c_long = 212i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:285`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L285)*

### `SYS_epoll_create`
```rust
const SYS_epoll_create: crate::c_long = 213i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:286`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L286)*

### `SYS_epoll_ctl_old`
```rust
const SYS_epoll_ctl_old: crate::c_long = 214i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:287`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L287)*

### `SYS_epoll_wait_old`
```rust
const SYS_epoll_wait_old: crate::c_long = 215i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:288`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L288)*

### `SYS_remap_file_pages`
```rust
const SYS_remap_file_pages: crate::c_long = 216i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:289`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L289)*

### `SYS_getdents64`
```rust
const SYS_getdents64: crate::c_long = 217i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:290`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L290)*

### `SYS_set_tid_address`
```rust
const SYS_set_tid_address: crate::c_long = 218i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:291`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L291)*

### `SYS_restart_syscall`
```rust
const SYS_restart_syscall: crate::c_long = 219i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:292`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L292)*

### `SYS_semtimedop`
```rust
const SYS_semtimedop: crate::c_long = 220i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:293`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L293)*

### `SYS_fadvise64`
```rust
const SYS_fadvise64: crate::c_long = 221i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:294`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L294)*

### `SYS_timer_create`
```rust
const SYS_timer_create: crate::c_long = 222i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:295`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L295)*

### `SYS_timer_settime`
```rust
const SYS_timer_settime: crate::c_long = 223i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:296`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L296)*

### `SYS_timer_gettime`
```rust
const SYS_timer_gettime: crate::c_long = 224i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:297`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L297)*

### `SYS_timer_getoverrun`
```rust
const SYS_timer_getoverrun: crate::c_long = 225i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:298`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L298)*

### `SYS_timer_delete`
```rust
const SYS_timer_delete: crate::c_long = 226i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:299`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L299)*

### `SYS_clock_settime`
```rust
const SYS_clock_settime: crate::c_long = 227i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:300`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L300)*

### `SYS_clock_gettime`
```rust
const SYS_clock_gettime: crate::c_long = 228i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:301`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L301)*

### `SYS_clock_getres`
```rust
const SYS_clock_getres: crate::c_long = 229i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:302`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L302)*

### `SYS_clock_nanosleep`
```rust
const SYS_clock_nanosleep: crate::c_long = 230i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:303`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L303)*

### `SYS_exit_group`
```rust
const SYS_exit_group: crate::c_long = 231i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:304`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L304)*

### `SYS_epoll_wait`
```rust
const SYS_epoll_wait: crate::c_long = 232i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:305`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L305)*

### `SYS_epoll_ctl`
```rust
const SYS_epoll_ctl: crate::c_long = 233i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:306`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L306)*

### `SYS_tgkill`
```rust
const SYS_tgkill: crate::c_long = 234i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:307`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L307)*

### `SYS_utimes`
```rust
const SYS_utimes: crate::c_long = 235i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:308`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L308)*

### `SYS_vserver`
```rust
const SYS_vserver: crate::c_long = 236i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:309`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L309)*

### `SYS_mbind`
```rust
const SYS_mbind: crate::c_long = 237i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:310`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L310)*

### `SYS_set_mempolicy`
```rust
const SYS_set_mempolicy: crate::c_long = 238i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:311`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L311)*

### `SYS_get_mempolicy`
```rust
const SYS_get_mempolicy: crate::c_long = 239i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:312`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L312)*

### `SYS_mq_open`
```rust
const SYS_mq_open: crate::c_long = 240i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:313`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L313)*

### `SYS_mq_unlink`
```rust
const SYS_mq_unlink: crate::c_long = 241i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:314`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L314)*

### `SYS_mq_timedsend`
```rust
const SYS_mq_timedsend: crate::c_long = 242i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:315`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L315)*

### `SYS_mq_timedreceive`
```rust
const SYS_mq_timedreceive: crate::c_long = 243i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:316`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L316)*

### `SYS_mq_notify`
```rust
const SYS_mq_notify: crate::c_long = 244i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:317`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L317)*

### `SYS_mq_getsetattr`
```rust
const SYS_mq_getsetattr: crate::c_long = 245i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:318`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L318)*

### `SYS_kexec_load`
```rust
const SYS_kexec_load: crate::c_long = 246i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:319`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L319)*

### `SYS_waitid`
```rust
const SYS_waitid: crate::c_long = 247i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:320`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L320)*

### `SYS_add_key`
```rust
const SYS_add_key: crate::c_long = 248i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:321`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L321)*

### `SYS_request_key`
```rust
const SYS_request_key: crate::c_long = 249i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:322`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L322)*

### `SYS_keyctl`
```rust
const SYS_keyctl: crate::c_long = 250i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:323`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L323)*

### `SYS_ioprio_set`
```rust
const SYS_ioprio_set: crate::c_long = 251i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:324`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L324)*

### `SYS_ioprio_get`
```rust
const SYS_ioprio_get: crate::c_long = 252i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:325`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L325)*

### `SYS_inotify_init`
```rust
const SYS_inotify_init: crate::c_long = 253i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:326`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L326)*

### `SYS_inotify_add_watch`
```rust
const SYS_inotify_add_watch: crate::c_long = 254i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:327`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L327)*

### `SYS_inotify_rm_watch`
```rust
const SYS_inotify_rm_watch: crate::c_long = 255i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:328`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L328)*

### `SYS_migrate_pages`
```rust
const SYS_migrate_pages: crate::c_long = 256i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:329`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L329)*

### `SYS_openat`
```rust
const SYS_openat: crate::c_long = 257i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:330`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L330)*

### `SYS_mkdirat`
```rust
const SYS_mkdirat: crate::c_long = 258i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:331`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L331)*

### `SYS_mknodat`
```rust
const SYS_mknodat: crate::c_long = 259i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:332`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L332)*

### `SYS_fchownat`
```rust
const SYS_fchownat: crate::c_long = 260i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:333`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L333)*

### `SYS_futimesat`
```rust
const SYS_futimesat: crate::c_long = 261i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:334`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L334)*

### `SYS_newfstatat`
```rust
const SYS_newfstatat: crate::c_long = 262i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:335`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L335)*

### `SYS_unlinkat`
```rust
const SYS_unlinkat: crate::c_long = 263i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:336`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L336)*

### `SYS_renameat`
```rust
const SYS_renameat: crate::c_long = 264i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:337`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L337)*

### `SYS_linkat`
```rust
const SYS_linkat: crate::c_long = 265i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:338`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L338)*

### `SYS_symlinkat`
```rust
const SYS_symlinkat: crate::c_long = 266i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:339`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L339)*

### `SYS_readlinkat`
```rust
const SYS_readlinkat: crate::c_long = 267i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:340`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L340)*

### `SYS_fchmodat`
```rust
const SYS_fchmodat: crate::c_long = 268i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:341`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L341)*

### `SYS_faccessat`
```rust
const SYS_faccessat: crate::c_long = 269i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:342`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L342)*

### `SYS_pselect6`
```rust
const SYS_pselect6: crate::c_long = 270i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:343`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L343)*

### `SYS_ppoll`
```rust
const SYS_ppoll: crate::c_long = 271i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:344`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L344)*

### `SYS_unshare`
```rust
const SYS_unshare: crate::c_long = 272i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:345`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L345)*

### `SYS_set_robust_list`
```rust
const SYS_set_robust_list: crate::c_long = 273i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:346`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L346)*

### `SYS_get_robust_list`
```rust
const SYS_get_robust_list: crate::c_long = 274i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:347`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L347)*

### `SYS_splice`
```rust
const SYS_splice: crate::c_long = 275i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:348`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L348)*

### `SYS_tee`
```rust
const SYS_tee: crate::c_long = 276i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:349`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L349)*

### `SYS_sync_file_range`
```rust
const SYS_sync_file_range: crate::c_long = 277i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:350`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L350)*

### `SYS_vmsplice`
```rust
const SYS_vmsplice: crate::c_long = 278i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:351`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L351)*

### `SYS_move_pages`
```rust
const SYS_move_pages: crate::c_long = 279i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:352`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L352)*

### `SYS_utimensat`
```rust
const SYS_utimensat: crate::c_long = 280i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:353`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L353)*

### `SYS_epoll_pwait`
```rust
const SYS_epoll_pwait: crate::c_long = 281i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:354`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L354)*

### `SYS_signalfd`
```rust
const SYS_signalfd: crate::c_long = 282i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:355`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L355)*

### `SYS_timerfd_create`
```rust
const SYS_timerfd_create: crate::c_long = 283i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:356`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L356)*

### `SYS_eventfd`
```rust
const SYS_eventfd: crate::c_long = 284i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:357`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L357)*

### `SYS_fallocate`
```rust
const SYS_fallocate: crate::c_long = 285i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:358`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L358)*

### `SYS_timerfd_settime`
```rust
const SYS_timerfd_settime: crate::c_long = 286i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:359`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L359)*

### `SYS_timerfd_gettime`
```rust
const SYS_timerfd_gettime: crate::c_long = 287i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:360`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L360)*

### `SYS_accept4`
```rust
const SYS_accept4: crate::c_long = 288i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:361`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L361)*

### `SYS_signalfd4`
```rust
const SYS_signalfd4: crate::c_long = 289i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:362`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L362)*

### `SYS_eventfd2`
```rust
const SYS_eventfd2: crate::c_long = 290i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:363`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L363)*

### `SYS_epoll_create1`
```rust
const SYS_epoll_create1: crate::c_long = 291i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:364`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L364)*

### `SYS_dup3`
```rust
const SYS_dup3: crate::c_long = 292i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:365`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L365)*

### `SYS_pipe2`
```rust
const SYS_pipe2: crate::c_long = 293i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:366`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L366)*

### `SYS_inotify_init1`
```rust
const SYS_inotify_init1: crate::c_long = 294i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:367`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L367)*

### `SYS_preadv`
```rust
const SYS_preadv: crate::c_long = 295i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:368`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L368)*

### `SYS_pwritev`
```rust
const SYS_pwritev: crate::c_long = 296i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:369`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L369)*

### `SYS_rt_tgsigqueueinfo`
```rust
const SYS_rt_tgsigqueueinfo: crate::c_long = 297i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:370`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L370)*

### `SYS_perf_event_open`
```rust
const SYS_perf_event_open: crate::c_long = 298i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:371`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L371)*

### `SYS_recvmmsg`
```rust
const SYS_recvmmsg: crate::c_long = 299i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:372`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L372)*

### `SYS_fanotify_init`
```rust
const SYS_fanotify_init: crate::c_long = 300i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:373`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L373)*

### `SYS_fanotify_mark`
```rust
const SYS_fanotify_mark: crate::c_long = 301i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:374`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L374)*

### `SYS_prlimit64`
```rust
const SYS_prlimit64: crate::c_long = 302i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:375`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L375)*

### `SYS_name_to_handle_at`
```rust
const SYS_name_to_handle_at: crate::c_long = 303i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:376`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L376)*

### `SYS_open_by_handle_at`
```rust
const SYS_open_by_handle_at: crate::c_long = 304i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:377`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L377)*

### `SYS_clock_adjtime`
```rust
const SYS_clock_adjtime: crate::c_long = 305i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:378`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L378)*

### `SYS_syncfs`
```rust
const SYS_syncfs: crate::c_long = 306i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:379`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L379)*

### `SYS_sendmmsg`
```rust
const SYS_sendmmsg: crate::c_long = 307i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:380`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L380)*

### `SYS_setns`
```rust
const SYS_setns: crate::c_long = 308i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:381`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L381)*

### `SYS_getcpu`
```rust
const SYS_getcpu: crate::c_long = 309i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:382`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L382)*

### `SYS_process_vm_readv`
```rust
const SYS_process_vm_readv: crate::c_long = 310i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:383`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L383)*

### `SYS_process_vm_writev`
```rust
const SYS_process_vm_writev: crate::c_long = 311i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:384`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L384)*

### `SYS_kcmp`
```rust
const SYS_kcmp: crate::c_long = 312i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:385`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L385)*

### `SYS_finit_module`
```rust
const SYS_finit_module: crate::c_long = 313i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:386`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L386)*

### `SYS_sched_setattr`
```rust
const SYS_sched_setattr: crate::c_long = 314i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:387`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L387)*

### `SYS_sched_getattr`
```rust
const SYS_sched_getattr: crate::c_long = 315i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:388`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L388)*

### `SYS_renameat2`
```rust
const SYS_renameat2: crate::c_long = 316i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:389`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L389)*

### `SYS_seccomp`
```rust
const SYS_seccomp: crate::c_long = 317i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:390`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L390)*

### `SYS_getrandom`
```rust
const SYS_getrandom: crate::c_long = 318i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:391`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L391)*

### `SYS_memfd_create`
```rust
const SYS_memfd_create: crate::c_long = 319i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:392`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L392)*

### `SYS_kexec_file_load`
```rust
const SYS_kexec_file_load: crate::c_long = 320i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:393`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L393)*

### `SYS_bpf`
```rust
const SYS_bpf: crate::c_long = 321i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:394`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L394)*

### `SYS_execveat`
```rust
const SYS_execveat: crate::c_long = 322i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:395`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L395)*

### `SYS_userfaultfd`
```rust
const SYS_userfaultfd: crate::c_long = 323i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:396`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L396)*

### `SYS_membarrier`
```rust
const SYS_membarrier: crate::c_long = 324i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:397`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L397)*

### `SYS_mlock2`
```rust
const SYS_mlock2: crate::c_long = 325i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:398`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L398)*

### `SYS_copy_file_range`
```rust
const SYS_copy_file_range: crate::c_long = 326i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:399`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L399)*

### `SYS_preadv2`
```rust
const SYS_preadv2: crate::c_long = 327i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:400`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L400)*

### `SYS_pwritev2`
```rust
const SYS_pwritev2: crate::c_long = 328i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:401`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L401)*

### `SYS_pkey_mprotect`
```rust
const SYS_pkey_mprotect: crate::c_long = 329i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:402`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L402)*

### `SYS_pkey_alloc`
```rust
const SYS_pkey_alloc: crate::c_long = 330i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:403`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L403)*

### `SYS_pkey_free`
```rust
const SYS_pkey_free: crate::c_long = 331i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:404`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L404)*

### `SYS_statx`
```rust
const SYS_statx: crate::c_long = 332i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:405`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L405)*

### `SYS_rseq`
```rust
const SYS_rseq: crate::c_long = 334i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:406`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L406)*

### `SYS_pidfd_send_signal`
```rust
const SYS_pidfd_send_signal: crate::c_long = 424i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:407`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L407)*

### `SYS_io_uring_setup`
```rust
const SYS_io_uring_setup: crate::c_long = 425i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:408`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L408)*

### `SYS_io_uring_enter`
```rust
const SYS_io_uring_enter: crate::c_long = 426i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:409`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L409)*

### `SYS_io_uring_register`
```rust
const SYS_io_uring_register: crate::c_long = 427i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:410`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L410)*

### `SYS_open_tree`
```rust
const SYS_open_tree: crate::c_long = 428i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:411`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L411)*

### `SYS_move_mount`
```rust
const SYS_move_mount: crate::c_long = 429i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:412`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L412)*

### `SYS_fsopen`
```rust
const SYS_fsopen: crate::c_long = 430i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:413`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L413)*

### `SYS_fsconfig`
```rust
const SYS_fsconfig: crate::c_long = 431i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:414`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L414)*

### `SYS_fsmount`
```rust
const SYS_fsmount: crate::c_long = 432i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:415`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L415)*

### `SYS_fspick`
```rust
const SYS_fspick: crate::c_long = 433i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:416`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L416)*

### `SYS_pidfd_open`
```rust
const SYS_pidfd_open: crate::c_long = 434i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:417`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L417)*

### `SYS_clone3`
```rust
const SYS_clone3: crate::c_long = 435i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:418`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L418)*

### `SYS_close_range`
```rust
const SYS_close_range: crate::c_long = 436i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:419`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L419)*

### `SYS_openat2`
```rust
const SYS_openat2: crate::c_long = 437i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:420`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L420)*

### `SYS_pidfd_getfd`
```rust
const SYS_pidfd_getfd: crate::c_long = 438i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:421`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L421)*

### `SYS_faccessat2`
```rust
const SYS_faccessat2: crate::c_long = 439i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:422`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L422)*

### `SYS_process_madvise`
```rust
const SYS_process_madvise: crate::c_long = 440i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:423`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L423)*

### `SYS_epoll_pwait2`
```rust
const SYS_epoll_pwait2: crate::c_long = 441i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:424`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L424)*

### `SYS_mount_setattr`
```rust
const SYS_mount_setattr: crate::c_long = 442i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:425`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L425)*

### `SYS_quotactl_fd`
```rust
const SYS_quotactl_fd: crate::c_long = 443i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:426`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L426)*

### `SYS_landlock_create_ruleset`
```rust
const SYS_landlock_create_ruleset: crate::c_long = 444i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:427`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L427)*

### `SYS_landlock_add_rule`
```rust
const SYS_landlock_add_rule: crate::c_long = 445i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:428`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L428)*

### `SYS_landlock_restrict_self`
```rust
const SYS_landlock_restrict_self: crate::c_long = 446i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:429`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L429)*

### `SYS_memfd_secret`
```rust
const SYS_memfd_secret: crate::c_long = 447i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:430`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L430)*

### `SYS_process_mrelease`
```rust
const SYS_process_mrelease: crate::c_long = 448i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:431`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L431)*

### `SYS_futex_waitv`
```rust
const SYS_futex_waitv: crate::c_long = 449i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:432`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L432)*

### `SYS_set_mempolicy_home_node`
```rust
const SYS_set_mempolicy_home_node: crate::c_long = 450i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:433`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L433)*

### `SYS_fchmodat2`
```rust
const SYS_fchmodat2: crate::c_long = 452i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:434`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L434)*

### `SYS_mseal`
```rust
const SYS_mseal: crate::c_long = 462i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:435`](../../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L435)*

