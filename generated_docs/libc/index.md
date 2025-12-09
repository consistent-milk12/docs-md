# Crate `libc`

libc - Raw FFI bindings to platforms' system libraries

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`new`](#new)
  - [`primitives`](#primitives)
  - [`unix`](#unix)
  - [`types`](#types)
  - [`prelude`](#prelude)
  - [`common`](#common)
  - [`linux_uapi`](#linux_uapi)
  - [`glibc`](#glibc)
  - [`linux_like`](#linux_like)
- [Structs](#structs)
  - [`group`](#group)
  - [`utimbuf`](#utimbuf)
  - [`timeval`](#timeval)
  - [`rlimit`](#rlimit)
  - [`rusage`](#rusage)
  - [`ipv6_mreq`](#ipv6_mreq)
  - [`hostent`](#hostent)
  - [`iovec`](#iovec)
  - [`pollfd`](#pollfd)
  - [`winsize`](#winsize)
  - [`linger`](#linger)
  - [`sigval`](#sigval)
  - [`itimerval`](#itimerval)
  - [`tms`](#tms)
  - [`servent`](#servent)
  - [`protoent`](#protoent)
  - [`in6_addr`](#in6_addr)
- [Enums](#enums)
  - [`DIR`](#dir)
  - [`FILE`](#file)
- [Functions](#functions)
  - [`c_void`](#c_void)
  - [`isalnum`](#isalnum)
  - [`isalpha`](#isalpha)
  - [`iscntrl`](#iscntrl)
  - [`isdigit`](#isdigit)
  - [`isgraph`](#isgraph)
  - [`islower`](#islower)
  - [`isprint`](#isprint)
  - [`ispunct`](#ispunct)
  - [`isspace`](#isspace)
  - [`isupper`](#isupper)
  - [`isxdigit`](#isxdigit)
  - [`isblank`](#isblank)
  - [`tolower`](#tolower)
  - [`toupper`](#toupper)
  - [`qsort`](#qsort)
  - [`bsearch`](#bsearch)
  - [`fopen`](#fopen)
  - [`freopen`](#freopen)
  - [`fflush`](#fflush)
  - [`fclose`](#fclose)
  - [`remove`](#remove)
  - [`rename`](#rename)
  - [`tmpfile`](#tmpfile)
  - [`setvbuf`](#setvbuf)
  - [`setbuf`](#setbuf)
  - [`getchar`](#getchar)
  - [`putchar`](#putchar)
  - [`fgetc`](#fgetc)
  - [`fgets`](#fgets)
  - [`fputc`](#fputc)
  - [`fputs`](#fputs)
  - [`puts`](#puts)
  - [`ungetc`](#ungetc)
  - [`fread`](#fread)
  - [`fwrite`](#fwrite)
  - [`fseek`](#fseek)
  - [`ftell`](#ftell)
  - [`rewind`](#rewind)
  - [`fgetpos`](#fgetpos)
  - [`fsetpos`](#fsetpos)
  - [`feof`](#feof)
  - [`ferror`](#ferror)
  - [`clearerr`](#clearerr)
  - [`perror`](#perror)
  - [`atof`](#atof)
  - [`atoi`](#atoi)
  - [`atol`](#atol)
  - [`atoll`](#atoll)
  - [`strtod`](#strtod)
  - [`strtof`](#strtof)
  - [`strtol`](#strtol)
  - [`strtoll`](#strtoll)
  - [`strtoul`](#strtoul)
  - [`strtoull`](#strtoull)
  - [`calloc`](#calloc)
  - [`malloc`](#malloc)
  - [`realloc`](#realloc)
  - [`free`](#free)
  - [`abort`](#abort)
  - [`exit`](#exit)
  - [`_exit`](#_exit)
  - [`system`](#system)
  - [`getenv`](#getenv)
  - [`strcpy`](#strcpy)
  - [`strncpy`](#strncpy)
  - [`stpcpy`](#stpcpy)
  - [`strcat`](#strcat)
  - [`strncat`](#strncat)
  - [`strcmp`](#strcmp)
  - [`strncmp`](#strncmp)
  - [`strcoll`](#strcoll)
  - [`strchr`](#strchr)
  - [`strrchr`](#strrchr)
  - [`strspn`](#strspn)
  - [`strcspn`](#strcspn)
  - [`strdup`](#strdup)
  - [`strndup`](#strndup)
  - [`strpbrk`](#strpbrk)
  - [`strstr`](#strstr)
  - [`strcasecmp`](#strcasecmp)
  - [`strncasecmp`](#strncasecmp)
  - [`strlen`](#strlen)
  - [`strnlen`](#strnlen)
  - [`strerror`](#strerror)
  - [`strtok`](#strtok)
  - [`strtok_r`](#strtok_r)
  - [`strxfrm`](#strxfrm)
  - [`strsignal`](#strsignal)
  - [`wcslen`](#wcslen)
  - [`wcstombs`](#wcstombs)
  - [`memchr`](#memchr)
  - [`wmemchr`](#wmemchr)
  - [`memcmp`](#memcmp)
  - [`memcpy`](#memcpy)
  - [`memmove`](#memmove)
  - [`memset`](#memset)
  - [`memccpy`](#memccpy)
  - [`getpwnam`](#getpwnam)
  - [`getpwuid`](#getpwuid)
  - [`fprintf`](#fprintf)
  - [`printf`](#printf)
  - [`snprintf`](#snprintf)
  - [`sprintf`](#sprintf)
  - [`fscanf`](#fscanf)
  - [`scanf`](#scanf)
  - [`sscanf`](#sscanf)
  - [`getchar_unlocked`](#getchar_unlocked)
  - [`putchar_unlocked`](#putchar_unlocked)
  - [`socket`](#socket)
  - [`connect`](#connect)
  - [`listen`](#listen)
  - [`accept`](#accept)
  - [`getpeername`](#getpeername)
  - [`getsockname`](#getsockname)
  - [`setsockopt`](#setsockopt)
  - [`socketpair`](#socketpair)
  - [`sendto`](#sendto)
  - [`shutdown`](#shutdown)
  - [`chmod`](#chmod)
  - [`fchmod`](#fchmod)
  - [`fstat`](#fstat)
  - [`mkdir`](#mkdir)
  - [`stat`](#stat)
  - [`pclose`](#pclose)
  - [`fdopen`](#fdopen)
  - [`fileno`](#fileno)
  - [`open`](#open)
  - [`creat`](#creat)
  - [`fcntl`](#fcntl)
  - [`opendir`](#opendir)
  - [`readdir`](#readdir)
  - [`closedir`](#closedir)
  - [`rewinddir`](#rewinddir)
  - [`fchmodat`](#fchmodat)
  - [`fchown`](#fchown)
  - [`fchownat`](#fchownat)
  - [`fstatat`](#fstatat)
  - [`linkat`](#linkat)
  - [`renameat`](#renameat)
  - [`symlinkat`](#symlinkat)
  - [`unlinkat`](#unlinkat)
  - [`access`](#access)
  - [`alarm`](#alarm)
  - [`chdir`](#chdir)
  - [`fchdir`](#fchdir)
  - [`chown`](#chown)
  - [`lchown`](#lchown)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`execl`](#execl)
  - [`execle`](#execle)
  - [`execlp`](#execlp)
  - [`execv`](#execv)
  - [`execve`](#execve)
  - [`execvp`](#execvp)
  - [`fork`](#fork)
  - [`fpathconf`](#fpathconf)
  - [`getcwd`](#getcwd)
  - [`getegid`](#getegid)
  - [`geteuid`](#geteuid)
  - [`getgid`](#getgid)
  - [`getgroups`](#getgroups)
  - [`getlogin`](#getlogin)
  - [`getopt`](#getopt)
  - [`getpgid`](#getpgid)
  - [`getpgrp`](#getpgrp)
  - [`getpid`](#getpid)
  - [`getppid`](#getppid)
  - [`getuid`](#getuid)
  - [`isatty`](#isatty)
  - [`link`](#link)
  - [`lseek`](#lseek)
  - [`pathconf`](#pathconf)
  - [`pipe`](#pipe)
  - [`posix_memalign`](#posix_memalign)
  - [`aligned_alloc`](#aligned_alloc)
  - [`read`](#read)
  - [`rmdir`](#rmdir)
  - [`seteuid`](#seteuid)
  - [`setegid`](#setegid)
  - [`setgid`](#setgid)
  - [`setpgid`](#setpgid)
  - [`setsid`](#setsid)
  - [`setuid`](#setuid)
  - [`setreuid`](#setreuid)
  - [`setregid`](#setregid)
  - [`sleep`](#sleep)
  - [`nanosleep`](#nanosleep)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`ttyname`](#ttyname)
  - [`ttyname_r`](#ttyname_r)
  - [`unlink`](#unlink)
  - [`wait`](#wait)
  - [`waitpid`](#waitpid)
  - [`write`](#write)
  - [`pread`](#pread)
  - [`pwrite`](#pwrite)
  - [`umask`](#umask)
  - [`utime`](#utime)
  - [`kill`](#kill)
  - [`killpg`](#killpg)
  - [`mlock`](#mlock)
  - [`munlock`](#munlock)
  - [`mlockall`](#mlockall)
  - [`munlockall`](#munlockall)
  - [`mmap`](#mmap)
  - [`munmap`](#munmap)
  - [`if_nametoindex`](#if_nametoindex)
  - [`if_indextoname`](#if_indextoname)
  - [`lstat`](#lstat)
  - [`fsync`](#fsync)
  - [`setenv`](#setenv)
  - [`unsetenv`](#unsetenv)
  - [`symlink`](#symlink)
  - [`truncate`](#truncate)
  - [`ftruncate`](#ftruncate)
  - [`signal`](#signal)
  - [`getrusage`](#getrusage)
  - [`realpath`](#realpath)
  - [`times`](#times)
  - [`pthread_self`](#pthread_self)
  - [`pthread_equal`](#pthread_equal)
  - [`pthread_join`](#pthread_join)
  - [`pthread_exit`](#pthread_exit)
  - [`pthread_attr_init`](#pthread_attr_init)
  - [`pthread_attr_destroy`](#pthread_attr_destroy)
  - [`pthread_attr_getstacksize`](#pthread_attr_getstacksize)
  - [`pthread_attr_setstacksize`](#pthread_attr_setstacksize)
  - [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate)
  - [`pthread_detach`](#pthread_detach)
  - [`sched_yield`](#sched_yield)
  - [`pthread_key_create`](#pthread_key_create)
  - [`pthread_key_delete`](#pthread_key_delete)
  - [`pthread_getspecific`](#pthread_getspecific)
  - [`pthread_setspecific`](#pthread_setspecific)
  - [`pthread_mutex_init`](#pthread_mutex_init)
  - [`pthread_mutex_destroy`](#pthread_mutex_destroy)
  - [`pthread_mutex_lock`](#pthread_mutex_lock)
  - [`pthread_mutex_trylock`](#pthread_mutex_trylock)
  - [`pthread_mutex_unlock`](#pthread_mutex_unlock)
  - [`pthread_mutexattr_init`](#pthread_mutexattr_init)
  - [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy)
  - [`pthread_mutexattr_settype`](#pthread_mutexattr_settype)
  - [`pthread_cond_init`](#pthread_cond_init)
  - [`pthread_cond_wait`](#pthread_cond_wait)
  - [`pthread_cond_timedwait`](#pthread_cond_timedwait)
  - [`pthread_cond_signal`](#pthread_cond_signal)
  - [`pthread_cond_broadcast`](#pthread_cond_broadcast)
  - [`pthread_cond_destroy`](#pthread_cond_destroy)
  - [`pthread_condattr_init`](#pthread_condattr_init)
  - [`pthread_condattr_destroy`](#pthread_condattr_destroy)
  - [`pthread_rwlock_init`](#pthread_rwlock_init)
  - [`pthread_rwlock_destroy`](#pthread_rwlock_destroy)
  - [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock)
  - [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock)
  - [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock)
  - [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock)
  - [`pthread_rwlock_unlock`](#pthread_rwlock_unlock)
  - [`pthread_rwlockattr_init`](#pthread_rwlockattr_init)
  - [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy)
  - [`getsockopt`](#getsockopt)
  - [`raise`](#raise)
  - [`utimes`](#utimes)
  - [`dlopen`](#dlopen)
  - [`dlerror`](#dlerror)
  - [`dlsym`](#dlsym)
  - [`dlclose`](#dlclose)
  - [`getaddrinfo`](#getaddrinfo)
  - [`freeaddrinfo`](#freeaddrinfo)
  - [`hstrerror`](#hstrerror)
  - [`gai_strerror`](#gai_strerror)
  - [`res_init`](#res_init)
  - [`gmtime_r`](#gmtime_r)
  - [`localtime_r`](#localtime_r)
  - [`mktime`](#mktime)
  - [`time`](#time)
  - [`gmtime`](#gmtime)
  - [`localtime`](#localtime)
  - [`difftime`](#difftime)
  - [`timegm`](#timegm)
  - [`mknod`](#mknod)
  - [`gethostname`](#gethostname)
  - [`endservent`](#endservent)
  - [`getservbyname`](#getservbyname)
  - [`getservbyport`](#getservbyport)
  - [`getservent`](#getservent)
  - [`setservent`](#setservent)
  - [`getprotobyname`](#getprotobyname)
  - [`getprotobynumber`](#getprotobynumber)
  - [`chroot`](#chroot)
  - [`usleep`](#usleep)
  - [`send`](#send)
  - [`recv`](#recv)
  - [`putenv`](#putenv)
  - [`poll`](#poll)
  - [`select`](#select)
  - [`setlocale`](#setlocale)
  - [`localeconv`](#localeconv)
  - [`sem_wait`](#sem_wait)
  - [`sem_trywait`](#sem_trywait)
  - [`sem_post`](#sem_post)
  - [`statvfs`](#statvfs)
  - [`fstatvfs`](#fstatvfs)
  - [`sigemptyset`](#sigemptyset)
  - [`sigaddset`](#sigaddset)
  - [`sigfillset`](#sigfillset)
  - [`sigdelset`](#sigdelset)
  - [`sigismember`](#sigismember)
  - [`sigprocmask`](#sigprocmask)
  - [`sigpending`](#sigpending)
  - [`sysconf`](#sysconf)
  - [`mkfifo`](#mkfifo)
  - [`fseeko`](#fseeko)
  - [`ftello`](#ftello)
  - [`tcdrain`](#tcdrain)
  - [`cfgetispeed`](#cfgetispeed)
  - [`cfgetospeed`](#cfgetospeed)
  - [`cfsetispeed`](#cfsetispeed)
  - [`cfsetospeed`](#cfsetospeed)
  - [`tcgetattr`](#tcgetattr)
  - [`tcsetattr`](#tcsetattr)
  - [`tcflow`](#tcflow)
  - [`tcflush`](#tcflush)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsendbreak`](#tcsendbreak)
  - [`mkstemp`](#mkstemp)
  - [`mkdtemp`](#mkdtemp)
  - [`tmpnam`](#tmpnam)
  - [`openlog`](#openlog)
  - [`closelog`](#closelog)
  - [`setlogmask`](#setlogmask)
  - [`syslog`](#syslog)
  - [`nice`](#nice)
  - [`grantpt`](#grantpt)
  - [`posix_openpt`](#posix_openpt)
  - [`ptsname`](#ptsname)
  - [`unlockpt`](#unlockpt)
  - [`strcasestr`](#strcasestr)
  - [`getline`](#getline)
  - [`lockf`](#lockf)
  - [`adjtime`](#adjtime)
  - [`stpncpy`](#stpncpy)
  - [`sigqueue`](#sigqueue)
  - [`confstr`](#confstr)
  - [`dladdr`](#dladdr)
  - [`flock`](#flock)
  - [`open_wmemstream`](#open_wmemstream)
  - [`getsid`](#getsid)
  - [`pause`](#pause)
  - [`mkdirat`](#mkdirat)
  - [`openat`](#openat)
  - [`fdopendir`](#fdopendir)
  - [`readdir_r`](#readdir_r)
  - [`readlinkat`](#readlinkat)
  - [`fmemopen`](#fmemopen)
  - [`open_memstream`](#open_memstream)
  - [`atexit`](#atexit)
  - [`sigaction`](#sigaction)
  - [`readlink`](#readlink)
  - [`pselect`](#pselect)
  - [`cfmakeraw`](#cfmakeraw)
  - [`cfsetspeed`](#cfsetspeed)
  - [`fnmatch`](#fnmatch)
  - [`htonl`](#htonl)
  - [`htons`](#htons)
  - [`ntohl`](#ntohl)
  - [`ntohs`](#ntohs)
- [Type Aliases](#type-aliases)
  - [`c_schar`](#c_schar)
  - [`c_uchar`](#c_uchar)
  - [`c_short`](#c_short)
  - [`c_ushort`](#c_ushort)
  - [`c_longlong`](#c_longlong)
  - [`c_ulonglong`](#c_ulonglong)
  - [`c_float`](#c_float)
  - [`c_double`](#c_double)
  - [`c_char`](#c_char)
  - [`c_int`](#c_int)
  - [`c_uint`](#c_uint)
  - [`c_long`](#c_long)
  - [`c_ulong`](#c_ulong)
  - [`int8_t`](#int8_t)
  - [`int16_t`](#int16_t)
  - [`int32_t`](#int32_t)
  - [`int64_t`](#int64_t)
  - [`uint8_t`](#uint8_t)
  - [`uint16_t`](#uint16_t)
  - [`uint32_t`](#uint32_t)
  - [`uint64_t`](#uint64_t)
  - [`intmax_t`](#intmax_t)
  - [`uintmax_t`](#uintmax_t)
  - [`size_t`](#size_t)
  - [`ptrdiff_t`](#ptrdiff_t)
  - [`intptr_t`](#intptr_t)
  - [`uintptr_t`](#uintptr_t)
  - [`ssize_t`](#ssize_t)
  - [`pid_t`](#pid_t)
  - [`in_addr_t`](#in_addr_t)
  - [`in_port_t`](#in_port_t)
  - [`sighandler_t`](#sighandler_t)
  - [`cc_t`](#cc_t)
  - [`uid_t`](#uid_t)
  - [`gid_t`](#gid_t)
  - [`locale_t`](#locale_t)
- [Constants](#constants)
  - [`INT_MIN`](#int_min)
  - [`INT_MAX`](#int_max)
  - [`SIG_DFL`](#sig_dfl)
  - [`SIG_IGN`](#sig_ign)
  - [`SIG_ERR`](#sig_err)
  - [`DT_UNKNOWN`](#dt_unknown)
  - [`DT_FIFO`](#dt_fifo)
  - [`DT_CHR`](#dt_chr)
  - [`DT_DIR`](#dt_dir)
  - [`DT_BLK`](#dt_blk)
  - [`DT_REG`](#dt_reg)
  - [`DT_LNK`](#dt_lnk)
  - [`DT_SOCK`](#dt_sock)
  - [`FD_CLOEXEC`](#fd_cloexec)
  - [`USRQUOTA`](#usrquota)
  - [`GRPQUOTA`](#grpquota)
  - [`SIGIOT`](#sigiot)
  - [`S_ISUID`](#s_isuid)
  - [`S_ISGID`](#s_isgid)
  - [`S_ISVTX`](#s_isvtx)
  - [`IF_NAMESIZE`](#if_namesize)
  - [`IFNAMSIZ`](#ifnamsiz)
  - [`LOG_EMERG`](#log_emerg)
  - [`LOG_ALERT`](#log_alert)
  - [`LOG_CRIT`](#log_crit)
  - [`LOG_ERR`](#log_err)
  - [`LOG_WARNING`](#log_warning)
  - [`LOG_NOTICE`](#log_notice)
  - [`LOG_INFO`](#log_info)
  - [`LOG_DEBUG`](#log_debug)
  - [`LOG_KERN`](#log_kern)
  - [`LOG_USER`](#log_user)
  - [`LOG_MAIL`](#log_mail)
  - [`LOG_DAEMON`](#log_daemon)
  - [`LOG_AUTH`](#log_auth)
  - [`LOG_SYSLOG`](#log_syslog)
  - [`LOG_LPR`](#log_lpr)
  - [`LOG_NEWS`](#log_news)
  - [`LOG_UUCP`](#log_uucp)
  - [`LOG_LOCAL0`](#log_local0)
  - [`LOG_LOCAL1`](#log_local1)
  - [`LOG_LOCAL2`](#log_local2)
  - [`LOG_LOCAL3`](#log_local3)
  - [`LOG_LOCAL4`](#log_local4)
  - [`LOG_LOCAL5`](#log_local5)
  - [`LOG_LOCAL6`](#log_local6)
  - [`LOG_LOCAL7`](#log_local7)
  - [`LOG_PID`](#log_pid)
  - [`LOG_CONS`](#log_cons)
  - [`LOG_ODELAY`](#log_odelay)
  - [`LOG_NDELAY`](#log_ndelay)
  - [`LOG_NOWAIT`](#log_nowait)
  - [`LOG_PRIMASK`](#log_primask)
  - [`LOG_FACMASK`](#log_facmask)
  - [`PRIO_MIN`](#prio_min)
  - [`PRIO_MAX`](#prio_max)
  - [`IPPROTO_ICMP`](#ipproto_icmp)
  - [`IPPROTO_ICMPV6`](#ipproto_icmpv6)
  - [`IPPROTO_TCP`](#ipproto_tcp)
  - [`IPPROTO_UDP`](#ipproto_udp)
  - [`IPPROTO_IP`](#ipproto_ip)
  - [`IPPROTO_IPV6`](#ipproto_ipv6)
  - [`INADDR_LOOPBACK`](#inaddr_loopback)
  - [`INADDR_ANY`](#inaddr_any)
  - [`INADDR_BROADCAST`](#inaddr_broadcast)
  - [`INADDR_NONE`](#inaddr_none)
  - [`IN6ADDR_LOOPBACK_INIT`](#in6addr_loopback_init)
  - [`IN6ADDR_ANY_INIT`](#in6addr_any_init)
  - [`ARPOP_REQUEST`](#arpop_request)
  - [`ARPOP_REPLY`](#arpop_reply)
  - [`ATF_COM`](#atf_com)
  - [`ATF_PERM`](#atf_perm)
  - [`ATF_PUBL`](#atf_publ)
  - [`ATF_USETRAILERS`](#atf_usetrailers)
  - [`FNM_PERIOD`](#fnm_period)
  - [`FNM_NOMATCH`](#fnm_nomatch)
  - [`FNM_CASEFOLD`](#fnm_casefold)
  - [`FNM_PATHNAME`](#fnm_pathname)
  - [`FNM_NOESCAPE`](#fnm_noescape)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`new`](#new) | mod | This module contains the future directory structure. |
| [`primitives`](#primitives) | mod | This module contains type aliases for C's platform-specific types and fixed-width integer types. |
| [`unix`](#unix) | mod | Definitions found commonly among almost all Unix derivatives |
| [`types`](#types) | mod | Platform-agnostic support types. |
| [`prelude`](#prelude) | mod | Frequently-used types that are available on all platforms |
| [`common`](#common) | mod | Interfaces that are common across multiple platforms |
| [`linux_uapi`](#linux_uapi) | mod | This directory maps to `include/uapi` in the Linux source tree. |
| [`glibc`](#glibc) | mod | GNU libc. |
| [`linux_like`](#linux_like) | mod |  |
| [`group`](#group) | struct |  |
| [`utimbuf`](#utimbuf) | struct |  |
| [`timeval`](#timeval) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`ipv6_mreq`](#ipv6_mreq) | struct |  |
| [`hostent`](#hostent) | struct |  |
| [`iovec`](#iovec) | struct |  |
| [`pollfd`](#pollfd) | struct |  |
| [`winsize`](#winsize) | struct |  |
| [`linger`](#linger) | struct |  |
| [`sigval`](#sigval) | struct |  |
| [`itimerval`](#itimerval) | struct |  |
| [`tms`](#tms) | struct |  |
| [`servent`](#servent) | struct |  |
| [`protoent`](#protoent) | struct |  |
| [`in6_addr`](#in6_addr) | struct |  |
| [`DIR`](#dir) | enum |  |
| [`FILE`](#file) | enum |  |
| [`c_void`](#c_void) | fn |  |
| [`isalnum`](#isalnum) | fn |  |
| [`isalpha`](#isalpha) | fn |  |
| [`iscntrl`](#iscntrl) | fn |  |
| [`isdigit`](#isdigit) | fn |  |
| [`isgraph`](#isgraph) | fn |  |
| [`islower`](#islower) | fn |  |
| [`isprint`](#isprint) | fn |  |
| [`ispunct`](#ispunct) | fn |  |
| [`isspace`](#isspace) | fn |  |
| [`isupper`](#isupper) | fn |  |
| [`isxdigit`](#isxdigit) | fn |  |
| [`isblank`](#isblank) | fn |  |
| [`tolower`](#tolower) | fn |  |
| [`toupper`](#toupper) | fn |  |
| [`qsort`](#qsort) | fn |  |
| [`bsearch`](#bsearch) | fn |  |
| [`fopen`](#fopen) | fn |  |
| [`freopen`](#freopen) | fn |  |
| [`fflush`](#fflush) | fn |  |
| [`fclose`](#fclose) | fn |  |
| [`remove`](#remove) | fn |  |
| [`rename`](#rename) | fn |  |
| [`tmpfile`](#tmpfile) | fn |  |
| [`setvbuf`](#setvbuf) | fn |  |
| [`setbuf`](#setbuf) | fn |  |
| [`getchar`](#getchar) | fn |  |
| [`putchar`](#putchar) | fn |  |
| [`fgetc`](#fgetc) | fn |  |
| [`fgets`](#fgets) | fn |  |
| [`fputc`](#fputc) | fn |  |
| [`fputs`](#fputs) | fn |  |
| [`puts`](#puts) | fn |  |
| [`ungetc`](#ungetc) | fn |  |
| [`fread`](#fread) | fn |  |
| [`fwrite`](#fwrite) | fn |  |
| [`fseek`](#fseek) | fn |  |
| [`ftell`](#ftell) | fn |  |
| [`rewind`](#rewind) | fn |  |
| [`fgetpos`](#fgetpos) | fn |  |
| [`fsetpos`](#fsetpos) | fn |  |
| [`feof`](#feof) | fn |  |
| [`ferror`](#ferror) | fn |  |
| [`clearerr`](#clearerr) | fn |  |
| [`perror`](#perror) | fn |  |
| [`atof`](#atof) | fn |  |
| [`atoi`](#atoi) | fn |  |
| [`atol`](#atol) | fn |  |
| [`atoll`](#atoll) | fn |  |
| [`strtod`](#strtod) | fn |  |
| [`strtof`](#strtof) | fn |  |
| [`strtol`](#strtol) | fn |  |
| [`strtoll`](#strtoll) | fn |  |
| [`strtoul`](#strtoul) | fn |  |
| [`strtoull`](#strtoull) | fn |  |
| [`calloc`](#calloc) | fn |  |
| [`malloc`](#malloc) | fn |  |
| [`realloc`](#realloc) | fn |  |
| [`free`](#free) | fn |  |
| [`abort`](#abort) | fn |  |
| [`exit`](#exit) | fn |  |
| [`_exit`](#_exit) | fn |  |
| [`system`](#system) | fn |  |
| [`getenv`](#getenv) | fn |  |
| [`strcpy`](#strcpy) | fn |  |
| [`strncpy`](#strncpy) | fn |  |
| [`stpcpy`](#stpcpy) | fn |  |
| [`strcat`](#strcat) | fn |  |
| [`strncat`](#strncat) | fn |  |
| [`strcmp`](#strcmp) | fn |  |
| [`strncmp`](#strncmp) | fn |  |
| [`strcoll`](#strcoll) | fn |  |
| [`strchr`](#strchr) | fn |  |
| [`strrchr`](#strrchr) | fn |  |
| [`strspn`](#strspn) | fn |  |
| [`strcspn`](#strcspn) | fn |  |
| [`strdup`](#strdup) | fn |  |
| [`strndup`](#strndup) | fn |  |
| [`strpbrk`](#strpbrk) | fn |  |
| [`strstr`](#strstr) | fn |  |
| [`strcasecmp`](#strcasecmp) | fn |  |
| [`strncasecmp`](#strncasecmp) | fn |  |
| [`strlen`](#strlen) | fn |  |
| [`strnlen`](#strnlen) | fn |  |
| [`strerror`](#strerror) | fn |  |
| [`strtok`](#strtok) | fn |  |
| [`strtok_r`](#strtok_r) | fn |  |
| [`strxfrm`](#strxfrm) | fn |  |
| [`strsignal`](#strsignal) | fn |  |
| [`wcslen`](#wcslen) | fn |  |
| [`wcstombs`](#wcstombs) | fn |  |
| [`memchr`](#memchr) | fn |  |
| [`wmemchr`](#wmemchr) | fn |  |
| [`memcmp`](#memcmp) | fn |  |
| [`memcpy`](#memcpy) | fn |  |
| [`memmove`](#memmove) | fn |  |
| [`memset`](#memset) | fn |  |
| [`memccpy`](#memccpy) | fn |  |
| [`getpwnam`](#getpwnam) | fn |  |
| [`getpwuid`](#getpwuid) | fn |  |
| [`fprintf`](#fprintf) | fn |  |
| [`printf`](#printf) | fn |  |
| [`snprintf`](#snprintf) | fn |  |
| [`sprintf`](#sprintf) | fn |  |
| [`fscanf`](#fscanf) | fn |  |
| [`scanf`](#scanf) | fn |  |
| [`sscanf`](#sscanf) | fn |  |
| [`getchar_unlocked`](#getchar_unlocked) | fn |  |
| [`putchar_unlocked`](#putchar_unlocked) | fn |  |
| [`socket`](#socket) | fn |  |
| [`connect`](#connect) | fn |  |
| [`listen`](#listen) | fn |  |
| [`accept`](#accept) | fn |  |
| [`getpeername`](#getpeername) | fn |  |
| [`getsockname`](#getsockname) | fn |  |
| [`setsockopt`](#setsockopt) | fn |  |
| [`socketpair`](#socketpair) | fn |  |
| [`sendto`](#sendto) | fn |  |
| [`shutdown`](#shutdown) | fn |  |
| [`chmod`](#chmod) | fn |  |
| [`fchmod`](#fchmod) | fn |  |
| [`fstat`](#fstat) | fn |  |
| [`mkdir`](#mkdir) | fn |  |
| [`stat`](#stat) | fn |  |
| [`pclose`](#pclose) | fn |  |
| [`fdopen`](#fdopen) | fn |  |
| [`fileno`](#fileno) | fn |  |
| [`open`](#open) | fn |  |
| [`creat`](#creat) | fn |  |
| [`fcntl`](#fcntl) | fn |  |
| [`opendir`](#opendir) | fn |  |
| [`readdir`](#readdir) | fn |  |
| [`closedir`](#closedir) | fn |  |
| [`rewinddir`](#rewinddir) | fn |  |
| [`fchmodat`](#fchmodat) | fn |  |
| [`fchown`](#fchown) | fn |  |
| [`fchownat`](#fchownat) | fn |  |
| [`fstatat`](#fstatat) | fn |  |
| [`linkat`](#linkat) | fn |  |
| [`renameat`](#renameat) | fn |  |
| [`symlinkat`](#symlinkat) | fn |  |
| [`unlinkat`](#unlinkat) | fn |  |
| [`access`](#access) | fn |  |
| [`alarm`](#alarm) | fn |  |
| [`chdir`](#chdir) | fn |  |
| [`fchdir`](#fchdir) | fn |  |
| [`chown`](#chown) | fn |  |
| [`lchown`](#lchown) | fn |  |
| [`close`](#close) | fn |  |
| [`dup`](#dup) | fn |  |
| [`dup2`](#dup2) | fn |  |
| [`execl`](#execl) | fn |  |
| [`execle`](#execle) | fn |  |
| [`execlp`](#execlp) | fn |  |
| [`execv`](#execv) | fn |  |
| [`execve`](#execve) | fn |  |
| [`execvp`](#execvp) | fn |  |
| [`fork`](#fork) | fn |  |
| [`fpathconf`](#fpathconf) | fn |  |
| [`getcwd`](#getcwd) | fn |  |
| [`getegid`](#getegid) | fn |  |
| [`geteuid`](#geteuid) | fn |  |
| [`getgid`](#getgid) | fn |  |
| [`getgroups`](#getgroups) | fn |  |
| [`getlogin`](#getlogin) | fn |  |
| [`getopt`](#getopt) | fn |  |
| [`getpgid`](#getpgid) | fn |  |
| [`getpgrp`](#getpgrp) | fn |  |
| [`getpid`](#getpid) | fn |  |
| [`getppid`](#getppid) | fn |  |
| [`getuid`](#getuid) | fn |  |
| [`isatty`](#isatty) | fn |  |
| [`link`](#link) | fn |  |
| [`lseek`](#lseek) | fn |  |
| [`pathconf`](#pathconf) | fn |  |
| [`pipe`](#pipe) | fn |  |
| [`posix_memalign`](#posix_memalign) | fn |  |
| [`aligned_alloc`](#aligned_alloc) | fn |  |
| [`read`](#read) | fn |  |
| [`rmdir`](#rmdir) | fn |  |
| [`seteuid`](#seteuid) | fn |  |
| [`setegid`](#setegid) | fn |  |
| [`setgid`](#setgid) | fn |  |
| [`setpgid`](#setpgid) | fn |  |
| [`setsid`](#setsid) | fn |  |
| [`setuid`](#setuid) | fn |  |
| [`setreuid`](#setreuid) | fn |  |
| [`setregid`](#setregid) | fn |  |
| [`sleep`](#sleep) | fn |  |
| [`nanosleep`](#nanosleep) | fn |  |
| [`tcgetpgrp`](#tcgetpgrp) | fn |  |
| [`tcsetpgrp`](#tcsetpgrp) | fn |  |
| [`ttyname`](#ttyname) | fn |  |
| [`ttyname_r`](#ttyname_r) | fn |  |
| [`unlink`](#unlink) | fn |  |
| [`wait`](#wait) | fn |  |
| [`waitpid`](#waitpid) | fn |  |
| [`write`](#write) | fn |  |
| [`pread`](#pread) | fn |  |
| [`pwrite`](#pwrite) | fn |  |
| [`umask`](#umask) | fn |  |
| [`utime`](#utime) | fn |  |
| [`kill`](#kill) | fn |  |
| [`killpg`](#killpg) | fn |  |
| [`mlock`](#mlock) | fn |  |
| [`munlock`](#munlock) | fn |  |
| [`mlockall`](#mlockall) | fn |  |
| [`munlockall`](#munlockall) | fn |  |
| [`mmap`](#mmap) | fn |  |
| [`munmap`](#munmap) | fn |  |
| [`if_nametoindex`](#if_nametoindex) | fn |  |
| [`if_indextoname`](#if_indextoname) | fn |  |
| [`lstat`](#lstat) | fn |  |
| [`fsync`](#fsync) | fn |  |
| [`setenv`](#setenv) | fn |  |
| [`unsetenv`](#unsetenv) | fn |  |
| [`symlink`](#symlink) | fn |  |
| [`truncate`](#truncate) | fn |  |
| [`ftruncate`](#ftruncate) | fn |  |
| [`signal`](#signal) | fn |  |
| [`getrusage`](#getrusage) | fn |  |
| [`realpath`](#realpath) | fn |  |
| [`times`](#times) | fn |  |
| [`pthread_self`](#pthread_self) | fn |  |
| [`pthread_equal`](#pthread_equal) | fn |  |
| [`pthread_join`](#pthread_join) | fn |  |
| [`pthread_exit`](#pthread_exit) | fn |  |
| [`pthread_attr_init`](#pthread_attr_init) | fn |  |
| [`pthread_attr_destroy`](#pthread_attr_destroy) | fn |  |
| [`pthread_attr_getstacksize`](#pthread_attr_getstacksize) | fn |  |
| [`pthread_attr_setstacksize`](#pthread_attr_setstacksize) | fn |  |
| [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate) | fn |  |
| [`pthread_detach`](#pthread_detach) | fn |  |
| [`sched_yield`](#sched_yield) | fn |  |
| [`pthread_key_create`](#pthread_key_create) | fn |  |
| [`pthread_key_delete`](#pthread_key_delete) | fn |  |
| [`pthread_getspecific`](#pthread_getspecific) | fn |  |
| [`pthread_setspecific`](#pthread_setspecific) | fn |  |
| [`pthread_mutex_init`](#pthread_mutex_init) | fn |  |
| [`pthread_mutex_destroy`](#pthread_mutex_destroy) | fn |  |
| [`pthread_mutex_lock`](#pthread_mutex_lock) | fn |  |
| [`pthread_mutex_trylock`](#pthread_mutex_trylock) | fn |  |
| [`pthread_mutex_unlock`](#pthread_mutex_unlock) | fn |  |
| [`pthread_mutexattr_init`](#pthread_mutexattr_init) | fn |  |
| [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy) | fn |  |
| [`pthread_mutexattr_settype`](#pthread_mutexattr_settype) | fn |  |
| [`pthread_cond_init`](#pthread_cond_init) | fn |  |
| [`pthread_cond_wait`](#pthread_cond_wait) | fn |  |
| [`pthread_cond_timedwait`](#pthread_cond_timedwait) | fn |  |
| [`pthread_cond_signal`](#pthread_cond_signal) | fn |  |
| [`pthread_cond_broadcast`](#pthread_cond_broadcast) | fn |  |
| [`pthread_cond_destroy`](#pthread_cond_destroy) | fn |  |
| [`pthread_condattr_init`](#pthread_condattr_init) | fn |  |
| [`pthread_condattr_destroy`](#pthread_condattr_destroy) | fn |  |
| [`pthread_rwlock_init`](#pthread_rwlock_init) | fn |  |
| [`pthread_rwlock_destroy`](#pthread_rwlock_destroy) | fn |  |
| [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock) | fn |  |
| [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock) | fn |  |
| [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock) | fn |  |
| [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock) | fn |  |
| [`pthread_rwlock_unlock`](#pthread_rwlock_unlock) | fn |  |
| [`pthread_rwlockattr_init`](#pthread_rwlockattr_init) | fn |  |
| [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy) | fn |  |
| [`getsockopt`](#getsockopt) | fn |  |
| [`raise`](#raise) | fn |  |
| [`utimes`](#utimes) | fn |  |
| [`dlopen`](#dlopen) | fn |  |
| [`dlerror`](#dlerror) | fn |  |
| [`dlsym`](#dlsym) | fn |  |
| [`dlclose`](#dlclose) | fn |  |
| [`getaddrinfo`](#getaddrinfo) | fn |  |
| [`freeaddrinfo`](#freeaddrinfo) | fn |  |
| [`hstrerror`](#hstrerror) | fn |  |
| [`gai_strerror`](#gai_strerror) | fn |  |
| [`res_init`](#res_init) | fn |  |
| [`gmtime_r`](#gmtime_r) | fn |  |
| [`localtime_r`](#localtime_r) | fn |  |
| [`mktime`](#mktime) | fn |  |
| [`time`](#time) | fn |  |
| [`gmtime`](#gmtime) | fn |  |
| [`localtime`](#localtime) | fn |  |
| [`difftime`](#difftime) | fn |  |
| [`timegm`](#timegm) | fn |  |
| [`mknod`](#mknod) | fn |  |
| [`gethostname`](#gethostname) | fn |  |
| [`endservent`](#endservent) | fn |  |
| [`getservbyname`](#getservbyname) | fn |  |
| [`getservbyport`](#getservbyport) | fn |  |
| [`getservent`](#getservent) | fn |  |
| [`setservent`](#setservent) | fn |  |
| [`getprotobyname`](#getprotobyname) | fn |  |
| [`getprotobynumber`](#getprotobynumber) | fn |  |
| [`chroot`](#chroot) | fn |  |
| [`usleep`](#usleep) | fn |  |
| [`send`](#send) | fn |  |
| [`recv`](#recv) | fn |  |
| [`putenv`](#putenv) | fn |  |
| [`poll`](#poll) | fn |  |
| [`select`](#select) | fn |  |
| [`setlocale`](#setlocale) | fn |  |
| [`localeconv`](#localeconv) | fn |  |
| [`sem_wait`](#sem_wait) | fn |  |
| [`sem_trywait`](#sem_trywait) | fn |  |
| [`sem_post`](#sem_post) | fn |  |
| [`statvfs`](#statvfs) | fn |  |
| [`fstatvfs`](#fstatvfs) | fn |  |
| [`sigemptyset`](#sigemptyset) | fn |  |
| [`sigaddset`](#sigaddset) | fn |  |
| [`sigfillset`](#sigfillset) | fn |  |
| [`sigdelset`](#sigdelset) | fn |  |
| [`sigismember`](#sigismember) | fn |  |
| [`sigprocmask`](#sigprocmask) | fn |  |
| [`sigpending`](#sigpending) | fn |  |
| [`sysconf`](#sysconf) | fn |  |
| [`mkfifo`](#mkfifo) | fn |  |
| [`fseeko`](#fseeko) | fn |  |
| [`ftello`](#ftello) | fn |  |
| [`tcdrain`](#tcdrain) | fn |  |
| [`cfgetispeed`](#cfgetispeed) | fn |  |
| [`cfgetospeed`](#cfgetospeed) | fn |  |
| [`cfsetispeed`](#cfsetispeed) | fn |  |
| [`cfsetospeed`](#cfsetospeed) | fn |  |
| [`tcgetattr`](#tcgetattr) | fn |  |
| [`tcsetattr`](#tcsetattr) | fn |  |
| [`tcflow`](#tcflow) | fn |  |
| [`tcflush`](#tcflush) | fn |  |
| [`tcgetsid`](#tcgetsid) | fn |  |
| [`tcsendbreak`](#tcsendbreak) | fn |  |
| [`mkstemp`](#mkstemp) | fn |  |
| [`mkdtemp`](#mkdtemp) | fn |  |
| [`tmpnam`](#tmpnam) | fn |  |
| [`openlog`](#openlog) | fn |  |
| [`closelog`](#closelog) | fn |  |
| [`setlogmask`](#setlogmask) | fn |  |
| [`syslog`](#syslog) | fn |  |
| [`nice`](#nice) | fn |  |
| [`grantpt`](#grantpt) | fn |  |
| [`posix_openpt`](#posix_openpt) | fn |  |
| [`ptsname`](#ptsname) | fn |  |
| [`unlockpt`](#unlockpt) | fn |  |
| [`strcasestr`](#strcasestr) | fn |  |
| [`getline`](#getline) | fn |  |
| [`lockf`](#lockf) | fn |  |
| [`adjtime`](#adjtime) | fn |  |
| [`stpncpy`](#stpncpy) | fn |  |
| [`sigqueue`](#sigqueue) | fn |  |
| [`confstr`](#confstr) | fn |  |
| [`dladdr`](#dladdr) | fn |  |
| [`flock`](#flock) | fn |  |
| [`open_wmemstream`](#open_wmemstream) | fn |  |
| [`getsid`](#getsid) | fn |  |
| [`pause`](#pause) | fn |  |
| [`mkdirat`](#mkdirat) | fn |  |
| [`openat`](#openat) | fn |  |
| [`fdopendir`](#fdopendir) | fn |  |
| [`readdir_r`](#readdir_r) | fn | The 64-bit libc on Solaris and illumos only has readdir_r. |
| [`readlinkat`](#readlinkat) | fn |  |
| [`fmemopen`](#fmemopen) | fn |  |
| [`open_memstream`](#open_memstream) | fn |  |
| [`atexit`](#atexit) | fn |  |
| [`sigaction`](#sigaction) | fn |  |
| [`readlink`](#readlink) | fn |  |
| [`pselect`](#pselect) | fn |  |
| [`cfmakeraw`](#cfmakeraw) | fn |  |
| [`cfsetspeed`](#cfsetspeed) | fn |  |
| [`fnmatch`](#fnmatch) | fn |  |
| [`htonl`](#htonl) | fn |  |
| [`htons`](#htons) | fn |  |
| [`ntohl`](#ntohl) | fn |  |
| [`ntohs`](#ntohs) | fn |  |
| [`c_schar`](#c_schar) | type |  |
| [`c_uchar`](#c_uchar) | type |  |
| [`c_short`](#c_short) | type |  |
| [`c_ushort`](#c_ushort) | type |  |
| [`c_longlong`](#c_longlong) | type |  |
| [`c_ulonglong`](#c_ulonglong) | type |  |
| [`c_float`](#c_float) | type |  |
| [`c_double`](#c_double) | type |  |
| [`c_char`](#c_char) | type |  |
| [`c_int`](#c_int) | type |  |
| [`c_uint`](#c_uint) | type |  |
| [`c_long`](#c_long) | type |  |
| [`c_ulong`](#c_ulong) | type |  |
| [`int8_t`](#int8_t) | type |  |
| [`int16_t`](#int16_t) | type |  |
| [`int32_t`](#int32_t) | type |  |
| [`int64_t`](#int64_t) | type |  |
| [`uint8_t`](#uint8_t) | type |  |
| [`uint16_t`](#uint16_t) | type |  |
| [`uint32_t`](#uint32_t) | type |  |
| [`uint64_t`](#uint64_t) | type |  |
| [`intmax_t`](#intmax_t) | type |  |
| [`uintmax_t`](#uintmax_t) | type |  |
| [`size_t`](#size_t) | type |  |
| [`ptrdiff_t`](#ptrdiff_t) | type |  |
| [`intptr_t`](#intptr_t) | type |  |
| [`uintptr_t`](#uintptr_t) | type |  |
| [`ssize_t`](#ssize_t) | type |  |
| [`pid_t`](#pid_t) | type |  |
| [`in_addr_t`](#in_addr_t) | type |  |
| [`in_port_t`](#in_port_t) | type |  |
| [`sighandler_t`](#sighandler_t) | type |  |
| [`cc_t`](#cc_t) | type |  |
| [`uid_t`](#uid_t) | type |  |
| [`gid_t`](#gid_t) | type |  |
| [`locale_t`](#locale_t) | type |  |
| [`INT_MIN`](#int_min) | const |  |
| [`INT_MAX`](#int_max) | const |  |
| [`SIG_DFL`](#sig_dfl) | const |  |
| [`SIG_IGN`](#sig_ign) | const |  |
| [`SIG_ERR`](#sig_err) | const |  |
| [`DT_UNKNOWN`](#dt_unknown) | const |  |
| [`DT_FIFO`](#dt_fifo) | const |  |
| [`DT_CHR`](#dt_chr) | const |  |
| [`DT_DIR`](#dt_dir) | const |  |
| [`DT_BLK`](#dt_blk) | const |  |
| [`DT_REG`](#dt_reg) | const |  |
| [`DT_LNK`](#dt_lnk) | const |  |
| [`DT_SOCK`](#dt_sock) | const |  |
| [`FD_CLOEXEC`](#fd_cloexec) | const |  |
| [`USRQUOTA`](#usrquota) | const |  |
| [`GRPQUOTA`](#grpquota) | const |  |
| [`SIGIOT`](#sigiot) | const |  |
| [`S_ISUID`](#s_isuid) | const |  |
| [`S_ISGID`](#s_isgid) | const |  |
| [`S_ISVTX`](#s_isvtx) | const |  |
| [`IF_NAMESIZE`](#if_namesize) | const |  |
| [`IFNAMSIZ`](#ifnamsiz) | const |  |
| [`LOG_EMERG`](#log_emerg) | const |  |
| [`LOG_ALERT`](#log_alert) | const |  |
| [`LOG_CRIT`](#log_crit) | const |  |
| [`LOG_ERR`](#log_err) | const |  |
| [`LOG_WARNING`](#log_warning) | const |  |
| [`LOG_NOTICE`](#log_notice) | const |  |
| [`LOG_INFO`](#log_info) | const |  |
| [`LOG_DEBUG`](#log_debug) | const |  |
| [`LOG_KERN`](#log_kern) | const |  |
| [`LOG_USER`](#log_user) | const |  |
| [`LOG_MAIL`](#log_mail) | const |  |
| [`LOG_DAEMON`](#log_daemon) | const |  |
| [`LOG_AUTH`](#log_auth) | const |  |
| [`LOG_SYSLOG`](#log_syslog) | const |  |
| [`LOG_LPR`](#log_lpr) | const |  |
| [`LOG_NEWS`](#log_news) | const |  |
| [`LOG_UUCP`](#log_uucp) | const |  |
| [`LOG_LOCAL0`](#log_local0) | const |  |
| [`LOG_LOCAL1`](#log_local1) | const |  |
| [`LOG_LOCAL2`](#log_local2) | const |  |
| [`LOG_LOCAL3`](#log_local3) | const |  |
| [`LOG_LOCAL4`](#log_local4) | const |  |
| [`LOG_LOCAL5`](#log_local5) | const |  |
| [`LOG_LOCAL6`](#log_local6) | const |  |
| [`LOG_LOCAL7`](#log_local7) | const |  |
| [`LOG_PID`](#log_pid) | const |  |
| [`LOG_CONS`](#log_cons) | const |  |
| [`LOG_ODELAY`](#log_odelay) | const |  |
| [`LOG_NDELAY`](#log_ndelay) | const |  |
| [`LOG_NOWAIT`](#log_nowait) | const |  |
| [`LOG_PRIMASK`](#log_primask) | const |  |
| [`LOG_FACMASK`](#log_facmask) | const |  |
| [`PRIO_MIN`](#prio_min) | const |  |
| [`PRIO_MAX`](#prio_max) | const |  |
| [`IPPROTO_ICMP`](#ipproto_icmp) | const |  |
| [`IPPROTO_ICMPV6`](#ipproto_icmpv6) | const |  |
| [`IPPROTO_TCP`](#ipproto_tcp) | const |  |
| [`IPPROTO_UDP`](#ipproto_udp) | const |  |
| [`IPPROTO_IP`](#ipproto_ip) | const |  |
| [`IPPROTO_IPV6`](#ipproto_ipv6) | const |  |
| [`INADDR_LOOPBACK`](#inaddr_loopback) | const |  |
| [`INADDR_ANY`](#inaddr_any) | const |  |
| [`INADDR_BROADCAST`](#inaddr_broadcast) | const |  |
| [`INADDR_NONE`](#inaddr_none) | const |  |
| [`IN6ADDR_LOOPBACK_INIT`](#in6addr_loopback_init) | const |  |
| [`IN6ADDR_ANY_INIT`](#in6addr_any_init) | const |  |
| [`ARPOP_REQUEST`](#arpop_request) | const |  |
| [`ARPOP_REPLY`](#arpop_reply) | const |  |
| [`ATF_COM`](#atf_com) | const |  |
| [`ATF_PERM`](#atf_perm) | const |  |
| [`ATF_PUBL`](#atf_publ) | const |  |
| [`ATF_USETRAILERS`](#atf_usetrailers) | const |  |
| [`FNM_PERIOD`](#fnm_period) | const |  |
| [`FNM_NOMATCH`](#fnm_nomatch) | const |  |
| [`FNM_CASEFOLD`](#fnm_casefold) | const |  |
| [`FNM_PATHNAME`](#fnm_pathname) | const |  |
| [`FNM_NOESCAPE`](#fnm_noescape) | const |  |

## Modules

- [`macros`](macros/index.md)
- [`new`](new/index.md) — This module contains the future directory structure. If possible, new definitions should
- [`primitives`](primitives/index.md) — This module contains type aliases for C's platform-specific types
- [`unix`](unix/index.md) — Definitions found commonly among almost all Unix derivatives
- [`types`](types/index.md) — Platform-agnostic support types.
- [`prelude`](prelude/index.md) — Frequently-used types that are available on all platforms
- [`common`](common/index.md) — Interfaces that are common across multiple platforms
- [`linux_uapi`](linux_uapi/index.md) — This directory maps to `include/uapi` in the Linux source tree.
- [`glibc`](glibc/index.md) — GNU libc.
- [`linux_like`](linux_like/index.md)

## Structs

### `group`

```rust
struct group {
    pub gr_name: *mut crate::c_char,
    pub gr_passwd: *mut crate::c_char,
    pub gr_gid: crate::gid_t,
    pub gr_mem: *mut *mut crate::c_char,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for group`

- <span id="group-clone"></span>`fn clone(&self) -> group` — [`group`](#group)

##### `impl Copy for group`

##### `impl Debug for group`

- <span id="group-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `utimbuf`

```rust
struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for utimbuf`

- <span id="utimbuf-clone"></span>`fn clone(&self) -> utimbuf` — [`utimbuf`](#utimbuf)

##### `impl Copy for utimbuf`

##### `impl Debug for utimbuf`

- <span id="utimbuf-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: crate::suseconds_t,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](#timeval)

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- <span id="timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](#rlimit)

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- <span id="rlimit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rusage`

```rust
struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: crate::c_long,
    pub ru_ixrss: crate::c_long,
    pub ru_idrss: crate::c_long,
    pub ru_isrss: crate::c_long,
    pub ru_minflt: crate::c_long,
    pub ru_majflt: crate::c_long,
    pub ru_nswap: crate::c_long,
    pub ru_inblock: crate::c_long,
    pub ru_oublock: crate::c_long,
    pub ru_msgsnd: crate::c_long,
    pub ru_msgrcv: crate::c_long,
    pub ru_nsignals: crate::c_long,
    pub ru_nvcsw: crate::c_long,
    pub ru_nivcsw: crate::c_long,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for rusage`

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](#rusage)

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- <span id="rusage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ipv6_mreq`

```rust
struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: crate::c_uint,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for ipv6_mreq`

- <span id="ipv6-mreq-clone"></span>`fn clone(&self) -> ipv6_mreq` — [`ipv6_mreq`](#ipv6-mreq)

##### `impl Copy for ipv6_mreq`

##### `impl Debug for ipv6_mreq`

- <span id="ipv6-mreq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `hostent`

```rust
struct hostent {
    pub h_name: *mut crate::c_char,
    pub h_aliases: *mut *mut crate::c_char,
    pub h_addrtype: crate::c_int,
    pub h_length: crate::c_int,
    pub h_addr_list: *mut *mut crate::c_char,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for hostent`

- <span id="hostent-clone"></span>`fn clone(&self) -> hostent` — [`hostent`](#hostent)

##### `impl Copy for hostent`

##### `impl Debug for hostent`

- <span id="hostent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::c_void,
    pub iov_len: size_t,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](#iovec)

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- <span id="iovec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pollfd`

```rust
struct pollfd {
    pub fd: crate::c_int,
    pub events: crate::c_short,
    pub revents: crate::c_short,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for pollfd`

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](#pollfd)

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- <span id="pollfd-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `winsize`

```rust
struct winsize {
    pub ws_row: crate::c_ushort,
    pub ws_col: crate::c_ushort,
    pub ws_xpixel: crate::c_ushort,
    pub ws_ypixel: crate::c_ushort,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](#winsize)

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- <span id="winsize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `linger`

```rust
struct linger {
    pub l_onoff: crate::c_int,
    pub l_linger: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for linger`

- <span id="linger-clone"></span>`fn clone(&self) -> linger` — [`linger`](#linger)

##### `impl Copy for linger`

##### `impl Debug for linger`

- <span id="linger-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigval`

```rust
struct sigval {
    pub sival_ptr: *mut crate::c_void,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for sigval`

- <span id="sigval-clone"></span>`fn clone(&self) -> sigval` — [`sigval`](#sigval)

##### `impl Copy for sigval`

##### `impl Debug for sigval`

- <span id="sigval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: crate::timeval,
    pub it_value: crate::timeval,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](#itimerval)

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- <span id="itimerval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tms`

```rust
struct tms {
    pub tms_utime: crate::clock_t,
    pub tms_stime: crate::clock_t,
    pub tms_cutime: crate::clock_t,
    pub tms_cstime: crate::clock_t,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for tms`

- <span id="tms-clone"></span>`fn clone(&self) -> tms` — [`tms`](#tms)

##### `impl Copy for tms`

##### `impl Debug for tms`

- <span id="tms-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `servent`

```rust
struct servent {
    pub s_name: *mut crate::c_char,
    pub s_aliases: *mut *mut crate::c_char,
    pub s_port: crate::c_int,
    pub s_proto: *mut crate::c_char,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for servent`

- <span id="servent-clone"></span>`fn clone(&self) -> servent` — [`servent`](#servent)

##### `impl Copy for servent`

##### `impl Debug for servent`

- <span id="servent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `protoent`

```rust
struct protoent {
    pub p_name: *mut crate::c_char,
    pub p_aliases: *mut *mut crate::c_char,
    pub p_proto: crate::c_int,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for protoent`

- <span id="protoent-clone"></span>`fn clone(&self) -> protoent` — [`protoent`](#protoent)

##### `impl Copy for protoent`

##### `impl Debug for protoent`

- <span id="protoent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_addr`

```rust
struct in6_addr {
    pub s6_addr: [u8; 16],
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for in6_addr`

- <span id="in6-addr-clone"></span>`fn clone(&self) -> in6_addr` — [`in6_addr`](#in6-addr)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- <span id="in6-addr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:40-42`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L40-L42)*

#### Trait Implementations

##### `impl Clone for DIR`

- <span id="dir-clone"></span>`fn clone(&self) -> DIR` — [`DIR`](#dir)

##### `impl Copy for DIR`

##### `impl Debug for DIR`

- <span id="dir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FILE`

```rust
enum FILE {
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:585-587`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L585-L587)*

#### Trait Implementations

##### `impl Clone for FILE`

- <span id="file-clone"></span>`fn clone(&self) -> FILE` — [`FILE`](#file)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

*Defined in [`libc-0.2.178/src/lib.rs:45`](../../.source_1765210505/libc-0.2.178/src/lib.rs#L45)*

### `isalnum`

```rust
unsafe fn isalnum(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:590`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L590)*

### `isalpha`

```rust
unsafe fn isalpha(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:591`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L591)*

### `iscntrl`

```rust
unsafe fn iscntrl(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:592`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L592)*

### `isdigit`

```rust
unsafe fn isdigit(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:593`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L593)*

### `isgraph`

```rust
unsafe fn isgraph(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:594`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L594)*

### `islower`

```rust
unsafe fn islower(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:595`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L595)*

### `isprint`

```rust
unsafe fn isprint(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:596`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L596)*

### `ispunct`

```rust
unsafe fn ispunct(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:597`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L597)*

### `isspace`

```rust
unsafe fn isspace(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:598`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L598)*

### `isupper`

```rust
unsafe fn isupper(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:599`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L599)*

### `isxdigit`

```rust
unsafe fn isxdigit(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:600`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L600)*

### `isblank`

```rust
unsafe fn isblank(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:601`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L601)*

### `tolower`

```rust
unsafe fn tolower(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:602`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L602)*

### `toupper`

```rust
unsafe fn toupper(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:603`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L603)*

### `qsort`

```rust
unsafe fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:604-609`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L604-L609)*

### `bsearch`

```rust
unsafe fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:610-616`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L610-L616)*

### `fopen`

```rust
unsafe fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:622`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L622)*

### `freopen`

```rust
unsafe fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:628`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L628)*

### `fflush`

```rust
unsafe fn fflush(file: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:630`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L630)*

### `fclose`

```rust
unsafe fn fclose(file: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:631`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L631)*

### `remove`

```rust
unsafe fn remove(filename: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:632`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L632)*

### `rename`

```rust
unsafe fn rename(oldname: *const c_char, newname: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:633`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L633)*

### `tmpfile`

```rust
unsafe fn tmpfile() -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:635`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L635)*

### `setvbuf`

```rust
unsafe fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:636`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L636)*

### `setbuf`

```rust
unsafe fn setbuf(stream: *mut FILE, buf: *mut c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:637`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L637)*

### `getchar`

```rust
unsafe fn getchar() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:638`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L638)*

### `putchar`

```rust
unsafe fn putchar(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:639`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L639)*

### `fgetc`

```rust
unsafe fn fgetc(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:640`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L640)*

### `fgets`

```rust
unsafe fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:641`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L641)*

### `fputc`

```rust
unsafe fn fputc(c: c_int, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:642`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L642)*

### `fputs`

```rust
unsafe fn fputs(s: *const c_char, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:647`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L647)*

### `puts`

```rust
unsafe fn puts(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:648`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L648)*

### `ungetc`

```rust
unsafe fn ungetc(c: c_int, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:649`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L649)*

### `fread`

```rust
unsafe fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:650`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L650)*

### `fwrite`

```rust
unsafe fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:655`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L655)*

### `fseek`

```rust
unsafe fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:656`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L656)*

### `ftell`

```rust
unsafe fn ftell(stream: *mut FILE) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:657`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L657)*

### `rewind`

```rust
unsafe fn rewind(stream: *mut FILE)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:658`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L658)*

### `fgetpos`

```rust
unsafe fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:661`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L661)*

### `fsetpos`

```rust
unsafe fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:664`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L664)*

### `feof`

```rust
unsafe fn feof(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:665`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L665)*

### `ferror`

```rust
unsafe fn ferror(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:666`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L666)*

### `clearerr`

```rust
unsafe fn clearerr(stream: *mut FILE)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:667`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L667)*

### `perror`

```rust
unsafe fn perror(s: *const c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:668`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L668)*

### `atof`

```rust
unsafe fn atof(s: *const c_char) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:669`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L669)*

### `atoi`

```rust
unsafe fn atoi(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:670`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L670)*

### `atol`

```rust
unsafe fn atol(s: *const c_char) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:671`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L671)*

### `atoll`

```rust
unsafe fn atoll(s: *const c_char) -> c_longlong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:672`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L672)*

### `strtod`

```rust
unsafe fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:677`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L677)*

### `strtof`

```rust
unsafe fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:678`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L678)*

### `strtol`

```rust
unsafe fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:679`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L679)*

### `strtoll`

```rust
unsafe fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:680`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L680)*

### `strtoul`

```rust
unsafe fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:681`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L681)*

### `strtoull`

```rust
unsafe fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:682`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L682)*

### `calloc`

```rust
unsafe fn calloc(nobj: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:684`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L684)*

### `malloc`

```rust
unsafe fn malloc(size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:686`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L686)*

### `realloc`

```rust
unsafe fn realloc(p: *mut c_void, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:687`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L687)*

### `free`

```rust
unsafe fn free(p: *mut c_void)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:688`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L688)*

### `abort`

```rust
unsafe fn abort() -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:689`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L689)*

### `exit`

```rust
unsafe fn exit(status: c_int) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:690`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L690)*

### `_exit`

```rust
unsafe fn _exit(status: c_int) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:691`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L691)*

### `system`

```rust
unsafe fn system(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:696`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L696)*

### `getenv`

```rust
unsafe fn getenv(s: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:697`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L697)*

### `strcpy`

```rust
unsafe fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:699`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L699)*

### `strncpy`

```rust
unsafe fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:700`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L700)*

### `stpcpy`

```rust
unsafe fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:701`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L701)*

### `strcat`

```rust
unsafe fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:702`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L702)*

### `strncat`

```rust
unsafe fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:703`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L703)*

### `strcmp`

```rust
unsafe fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:704`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L704)*

### `strncmp`

```rust
unsafe fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:705`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L705)*

### `strcoll`

```rust
unsafe fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:706`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L706)*

### `strchr`

```rust
unsafe fn strchr(cs: *const c_char, c: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:707`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L707)*

### `strrchr`

```rust
unsafe fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:708`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L708)*

### `strspn`

```rust
unsafe fn strspn(cs: *const c_char, ct: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:709`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L709)*

### `strcspn`

```rust
unsafe fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:710`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L710)*

### `strdup`

```rust
unsafe fn strdup(cs: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:711`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L711)*

### `strndup`

```rust
unsafe fn strndup(cs: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:712`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L712)*

### `strpbrk`

```rust
unsafe fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:713`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L713)*

### `strstr`

```rust
unsafe fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:714`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L714)*

### `strcasecmp`

```rust
unsafe fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:715`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L715)*

### `strncasecmp`

```rust
unsafe fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:716`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L716)*

### `strlen`

```rust
unsafe fn strlen(cs: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:717`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L717)*

### `strnlen`

```rust
unsafe fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:718`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L718)*

### `strerror`

```rust
unsafe fn strerror(n: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:723`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L723)*

### `strtok`

```rust
unsafe fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:724`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L724)*

### `strtok_r`

```rust
unsafe fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:725`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L725)*

### `strxfrm`

```rust
unsafe fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:726`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L726)*

### `strsignal`

```rust
unsafe fn strsignal(sig: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:727`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L727)*

### `wcslen`

```rust
unsafe fn wcslen(buf: *const wchar_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:728`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L728)*

### `wcstombs`

```rust
unsafe fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:729`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L729)*

### `memchr`

```rust
unsafe fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:731`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L731)*

### `wmemchr`

```rust
unsafe fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:732`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L732)*

### `memcmp`

```rust
unsafe fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:733`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L733)*

### `memcpy`

```rust
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:734`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L734)*

### `memmove`

```rust
unsafe fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:735`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L735)*

### `memset`

```rust
unsafe fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:736`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L736)*

### `memccpy`

```rust
unsafe fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:737`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L737)*

### `getpwnam`

```rust
unsafe fn getpwnam(name: *const c_char) -> *mut passwd
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:742`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L742)*

### `getpwuid`

```rust
unsafe fn getpwuid(uid: crate::uid_t) -> *mut passwd
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:744`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L744)*

### `fprintf`

```rust
unsafe fn fprintf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:746`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L746)*

### `printf`

```rust
unsafe fn printf(format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:747`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L747)*

### `snprintf`

```rust
unsafe fn snprintf(s: *mut c_char, n: size_t, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:748`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L748)*

### `sprintf`

```rust
unsafe fn sprintf(s: *mut c_char, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:749`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L749)*

### `fscanf`

```rust
unsafe fn fscanf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:754`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L754)*

### `scanf`

```rust
unsafe fn scanf(format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:759`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L759)*

### `sscanf`

```rust
unsafe fn sscanf(s: *const c_char, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:764`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L764)*

### `getchar_unlocked`

```rust
unsafe fn getchar_unlocked() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:765`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L765)*

### `putchar_unlocked`

```rust
unsafe fn putchar_unlocked(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:766`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L766)*

### `socket`

```rust
unsafe fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:773`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L773)*

### `connect`

```rust
unsafe fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:784`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L784)*

### `listen`

```rust
unsafe fn listen(socket: c_int, backlog: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:790`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L790)*

### `accept`

```rust
unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:798`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L798)*

### `getpeername`

```rust
unsafe fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:806-807`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L806-L807)*

### `getsockname`

```rust
unsafe fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:815-816`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L815-L816)*

### `setsockopt`

```rust
unsafe fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:819-825`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L819-L825)*

### `socketpair`

```rust
unsafe fn socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:834-839`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L834-L839)*

### `sendto`

```rust
unsafe fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:850-857`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L850-L857)*

### `shutdown`

```rust
unsafe fn shutdown(socket: c_int, how: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:859`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L859)*

### `chmod`

```rust
unsafe fn chmod(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:865`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L865)*

### `fchmod`

```rust
unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:870`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L870)*

### `fstat`

```rust
unsafe fn fstat(fildes: c_int, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:886`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L886)*

### `mkdir`

```rust
unsafe fn mkdir(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:888`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L888)*

### `stat`

```rust
unsafe fn stat(path: *const c_char, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:904`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L904)*

### `pclose`

```rust
unsafe fn pclose(stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:906`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L906)*

### `fdopen`

```rust
unsafe fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:911`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L911)*

### `fileno`

```rust
unsafe fn fileno(stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:912`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L912)*

### `open`

```rust
unsafe fn open(path: *const c_char, oflag: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:919`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L919)*

### `creat`

```rust
unsafe fn creat(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:925`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L925)*

### `fcntl`

```rust
unsafe fn fcntl(fd: c_int, cmd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:935`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L935)*

### `opendir`

```rust
unsafe fn opendir(dirname: *const c_char) -> *mut crate::DIR
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:946`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L946)*

### `readdir`

```rust
unsafe fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:958`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L958)*

### `closedir`

```rust
unsafe fn closedir(dirp: *mut crate::DIR) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:963`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L963)*

### `rewinddir`

```rust
unsafe fn rewinddir(dirp: *mut crate::DIR)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:972`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L972)*

### `fchmodat`

```rust
unsafe fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:974`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L974)*

### `fchown`

```rust
unsafe fn fchown(fd: c_int, owner: crate::uid_t, group: crate::gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:975`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L975)*

### `fchownat`

```rust
unsafe fn fchownat(dirfd: c_int, pathname: *const c_char, owner: crate::uid_t, group: crate::gid_t, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:976-982`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L976-L982)*

### `fstatat`

```rust
unsafe fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:996`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L996)*

### `linkat`

```rust
unsafe fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:997-1003`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L997-L1003)*

### `renameat`

```rust
unsafe fn renameat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1004-1009`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1004-L1009)*

### `symlinkat`

```rust
unsafe fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1010`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1010)*

### `unlinkat`

```rust
unsafe fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1011`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1011)*

### `access`

```rust
unsafe fn access(path: *const c_char, amode: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1013`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1013)*

### `alarm`

```rust
unsafe fn alarm(seconds: c_uint) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1014`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1014)*

### `chdir`

```rust
unsafe fn chdir(dir: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1015`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1015)*

### `fchdir`

```rust
unsafe fn fchdir(dirfd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1016`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1016)*

### `chown`

```rust
unsafe fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1017`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1017)*

### `lchown`

```rust
unsafe fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1022`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1022)*

### `close`

```rust
unsafe fn close(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1031`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1031)*

### `dup`

```rust
unsafe fn dup(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1032`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1032)*

### `dup2`

```rust
unsafe fn dup2(src: c_int, dst: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1033`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1033)*

### `execl`

```rust
unsafe fn execl(path: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1035`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1035)*

### `execle`

```rust
unsafe fn execle(path: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1036`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1036)*

### `execlp`

```rust
unsafe fn execlp(file: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1037`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1037)*

### `execv`

```rust
unsafe fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1040`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1040)*

### `execve`

```rust
unsafe fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1041-1045`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1041-L1045)*

### `execvp`

```rust
unsafe fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1046`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1046)*

### `fork`

```rust
unsafe fn fork() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1048`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1048)*

### `fpathconf`

```rust
unsafe fn fpathconf(filedes: c_int, name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1049`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1049)*

### `getcwd`

```rust
unsafe fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1050`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1050)*

### `getegid`

```rust
unsafe fn getegid() -> gid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1051`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1051)*

### `geteuid`

```rust
unsafe fn geteuid() -> uid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1052`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1052)*

### `getgid`

```rust
unsafe fn getgid() -> gid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1053`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1053)*

### `getgroups`

```rust
unsafe fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1054`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1054)*

### `getlogin`

```rust
unsafe fn getlogin() -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1056`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1056)*

### `getopt`

```rust
unsafe fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1061`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1061)*

### `getpgid`

```rust
unsafe fn getpgid(pid: pid_t) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1062`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1062)*

### `getpgrp`

```rust
unsafe fn getpgrp() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1063`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1063)*

### `getpid`

```rust
unsafe fn getpid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1064`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1064)*

### `getppid`

```rust
unsafe fn getppid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1065`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1065)*

### `getuid`

```rust
unsafe fn getuid() -> uid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1066`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1066)*

### `isatty`

```rust
unsafe fn isatty(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1067`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1067)*

### `link`

```rust
unsafe fn link(src: *const c_char, dst: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1069`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1069)*

### `lseek`

```rust
unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1071`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1071)*

### `pathconf`

```rust
unsafe fn pathconf(path: *const c_char, name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1072`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1072)*

### `pipe`

```rust
unsafe fn pipe(fds: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1073`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1073)*

### `posix_memalign`

```rust
unsafe fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1074`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1074)*

### `aligned_alloc`

```rust
unsafe fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1075`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1075)*

### `read`

```rust
unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1080`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1080)*

### `rmdir`

```rust
unsafe fn rmdir(path: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1081`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1081)*

### `seteuid`

```rust
unsafe fn seteuid(uid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1082`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1082)*

### `setegid`

```rust
unsafe fn setegid(gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1083`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1083)*

### `setgid`

```rust
unsafe fn setgid(gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1084`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1084)*

### `setpgid`

```rust
unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1085`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1085)*

### `setsid`

```rust
unsafe fn setsid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1086`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1086)*

### `setuid`

```rust
unsafe fn setuid(uid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1087`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1087)*

### `setreuid`

```rust
unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1088`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1088)*

### `setregid`

```rust
unsafe fn setregid(rgid: gid_t, egid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1089`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1089)*

### `sleep`

```rust
unsafe fn sleep(secs: c_uint) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1094`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1094)*

### `nanosleep`

```rust
unsafe fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1101`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1101)*

### `tcgetpgrp`

```rust
unsafe fn tcgetpgrp(fd: c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1102`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1102)*

### `tcsetpgrp`

```rust
unsafe fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1103`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1103)*

### `ttyname`

```rust
unsafe fn ttyname(fd: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1104`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1104)*

### `ttyname_r`

```rust
unsafe fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1113`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1113)*

### `unlink`

```rust
unsafe fn unlink(c: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1114`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1114)*

### `wait`

```rust
unsafe fn wait(status: *mut c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1119`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1119)*

### `waitpid`

```rust
unsafe fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1124`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1124)*

### `write`

```rust
unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1129`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1129)*

### `pread`

```rust
unsafe fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1135`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1135)*

### `pwrite`

```rust
unsafe fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1141`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1141)*

### `umask`

```rust
unsafe fn umask(mask: mode_t) -> mode_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1142`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1142)*

### `utime`

```rust
unsafe fn utime(file: *const c_char, buf: *const utimbuf) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1146`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1146)*

### `kill`

```rust
unsafe fn kill(pid: pid_t, sig: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1152`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1152)*

### `killpg`

```rust
unsafe fn killpg(pgrp: pid_t, sig: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1157`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1157)*

### `mlock`

```rust
unsafe fn mlock(addr: *const c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1159`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1159)*

### `munlock`

```rust
unsafe fn munlock(addr: *const c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1160`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1160)*

### `mlockall`

```rust
unsafe fn mlockall(flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1161`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1161)*

### `munlockall`

```rust
unsafe fn munlockall() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1162`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1162)*

### `mmap`

```rust
unsafe fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1169-1176`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1169-L1176)*

### `munmap`

```rust
unsafe fn munmap(addr: *mut c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1181`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1181)*

### `if_nametoindex`

```rust
unsafe fn if_nametoindex(ifname: *const c_char) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1183`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1183)*

### `if_indextoname`

```rust
unsafe fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1184`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1184)*

### `lstat`

```rust
unsafe fn lstat(path: *const c_char, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1200`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1200)*

### `fsync`

```rust
unsafe fn fsync(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1206`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1206)*

### `setenv`

```rust
unsafe fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1212`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1212)*

### `unsetenv`

```rust
unsafe fn unsetenv(name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1218`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1218)*

### `symlink`

```rust
unsafe fn symlink(path1: *const c_char, path2: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1220`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1220)*

### `truncate`

```rust
unsafe fn truncate(path: *const c_char, length: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1223`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1223)*

### `ftruncate`

```rust
unsafe fn ftruncate(fd: c_int, length: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1225`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1225)*

### `signal`

```rust
unsafe fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1227`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1227)*

### `getrusage`

```rust
unsafe fn getrusage(resource: c_int, usage: *mut rusage) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1231`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1231)*

### `realpath`

```rust
unsafe fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1243`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1243)*

### `times`

```rust
unsafe fn times(buf: *mut crate::tms) -> crate::clock_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1246`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1246)*

### `pthread_self`

```rust
unsafe fn pthread_self() -> crate::pthread_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1248`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1248)*

### `pthread_equal`

```rust
unsafe fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1249`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1249)*

### `pthread_join`

```rust
unsafe fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1254`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1254)*

### `pthread_exit`

```rust
unsafe fn pthread_exit(value: *mut c_void) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1255`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1255)*

### `pthread_attr_init`

```rust
unsafe fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1256`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1256)*

### `pthread_attr_destroy`

```rust
unsafe fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1257`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1257)*

### `pthread_attr_getstacksize`

```rust
unsafe fn pthread_attr_getstacksize(attr: *const crate::pthread_attr_t, stacksize: *mut size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1258-1261`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1258-L1261)*

### `pthread_attr_setstacksize`

```rust
unsafe fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1262-1263`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1262-L1263)*

### `pthread_attr_setdetachstate`

```rust
unsafe fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1264`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1264)*

### `pthread_detach`

```rust
unsafe fn pthread_detach(thread: crate::pthread_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1265`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1265)*

### `sched_yield`

```rust
unsafe fn sched_yield() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1267`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1267)*

### `pthread_key_create`

```rust
unsafe fn pthread_key_create(key: *mut crate::pthread_key_t, dtor: Option<fn(*mut c_void)>) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1268-1271`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1268-L1271)*

### `pthread_key_delete`

```rust
unsafe fn pthread_key_delete(key: crate::pthread_key_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1272`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1272)*

### `pthread_getspecific`

```rust
unsafe fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1273`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1273)*

### `pthread_setspecific`

```rust
unsafe fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1274`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1274)*

### `pthread_mutex_init`

```rust
unsafe fn pthread_mutex_init(lock: *mut crate::pthread_mutex_t, attr: *const crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1275-1278`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1275-L1278)*

### `pthread_mutex_destroy`

```rust
unsafe fn pthread_mutex_destroy(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1279`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1279)*

### `pthread_mutex_lock`

```rust
unsafe fn pthread_mutex_lock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1280`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1280)*

### `pthread_mutex_trylock`

```rust
unsafe fn pthread_mutex_trylock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1281`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1281)*

### `pthread_mutex_unlock`

```rust
unsafe fn pthread_mutex_unlock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1282`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1282)*

### `pthread_mutexattr_init`

```rust
unsafe fn pthread_mutexattr_init(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1284`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1284)*

### `pthread_mutexattr_destroy`

```rust
unsafe fn pthread_mutexattr_destroy(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1289`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1289)*

### `pthread_mutexattr_settype`

```rust
unsafe fn pthread_mutexattr_settype(attr: *mut crate::pthread_mutexattr_t, _type: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1290`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1290)*

### `pthread_cond_init`

```rust
unsafe fn pthread_cond_init(cond: *mut crate::pthread_cond_t, attr: *const crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1296-1299`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1296-L1299)*

### `pthread_cond_wait`

```rust
unsafe fn pthread_cond_wait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1304-1307`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1304-L1307)*

### `pthread_cond_timedwait`

```rust
unsafe fn pthread_cond_timedwait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1313-1317`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1313-L1317)*

### `pthread_cond_signal`

```rust
unsafe fn pthread_cond_signal(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1318`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1318)*

### `pthread_cond_broadcast`

```rust
unsafe fn pthread_cond_broadcast(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1319`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1319)*

### `pthread_cond_destroy`

```rust
unsafe fn pthread_cond_destroy(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1320`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1320)*

### `pthread_condattr_init`

```rust
unsafe fn pthread_condattr_init(attr: *mut crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1321`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1321)*

### `pthread_condattr_destroy`

```rust
unsafe fn pthread_condattr_destroy(attr: *mut crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1322`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1322)*

### `pthread_rwlock_init`

```rust
unsafe fn pthread_rwlock_init(lock: *mut crate::pthread_rwlock_t, attr: *const crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1327-1330`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1327-L1330)*

### `pthread_rwlock_destroy`

```rust
unsafe fn pthread_rwlock_destroy(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1335`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1335)*

### `pthread_rwlock_rdlock`

```rust
unsafe fn pthread_rwlock_rdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1340`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1340)*

### `pthread_rwlock_tryrdlock`

```rust
unsafe fn pthread_rwlock_tryrdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1345`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1345)*

### `pthread_rwlock_wrlock`

```rust
unsafe fn pthread_rwlock_wrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1350`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1350)*

### `pthread_rwlock_trywrlock`

```rust
unsafe fn pthread_rwlock_trywrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1355`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1355)*

### `pthread_rwlock_unlock`

```rust
unsafe fn pthread_rwlock_unlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1360`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1360)*

### `pthread_rwlockattr_init`

```rust
unsafe fn pthread_rwlockattr_init(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1361`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1361)*

### `pthread_rwlockattr_destroy`

```rust
unsafe fn pthread_rwlockattr_destroy(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1362`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1362)*

### `getsockopt`

```rust
unsafe fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut crate::socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1370-1376`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1370-L1376)*

### `raise`

```rust
unsafe fn raise(signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1377`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1377)*

### `utimes`

```rust
unsafe fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1381`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1381)*

### `dlopen`

```rust
unsafe fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1382`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1382)*

### `dlerror`

```rust
unsafe fn dlerror() -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1383`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1383)*

### `dlsym`

```rust
unsafe fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1384`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1384)*

### `dlclose`

```rust
unsafe fn dlclose(handle: *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1385`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1385)*

### `getaddrinfo`

```rust
unsafe fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1393-1398`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1393-L1398)*

### `freeaddrinfo`

```rust
unsafe fn freeaddrinfo(res: *mut addrinfo)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1401`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1401)*

### `hstrerror`

```rust
unsafe fn hstrerror(errcode: c_int) -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1402`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1402)*

### `gai_strerror`

```rust
unsafe fn gai_strerror(errcode: c_int) -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1403`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1403)*

### `res_init`

```rust
unsafe fn res_init() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1428`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1428)*

### `gmtime_r`

```rust
unsafe fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1434`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1434)*

### `localtime_r`

```rust
unsafe fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1439`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1439)*

### `mktime`

```rust
unsafe fn mktime(tm: *mut tm) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1448`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1448)*

### `time`

```rust
unsafe fn time(time: *mut time_t) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1453`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1453)*

### `gmtime`

```rust
unsafe fn gmtime(time_p: *const time_t) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1458`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1458)*

### `localtime`

```rust
unsafe fn localtime(time_p: *const time_t) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1463`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1463)*

### `difftime`

```rust
unsafe fn difftime(time1: time_t, time0: time_t) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1468`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1468)*

### `timegm`

```rust
unsafe fn timegm(tm: *mut crate::tm) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1474`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1474)*

### `mknod`

```rust
unsafe fn mknod(pathname: *const c_char, mode: mode_t, dev: crate::dev_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1481`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1481)*

### `gethostname`

```rust
unsafe fn gethostname(name: *mut c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1483`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1483)*

### `endservent`

```rust
unsafe fn endservent()
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1484`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1484)*

### `getservbyname`

```rust
unsafe fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1485`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1485)*

### `getservbyport`

```rust
unsafe fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1486`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1486)*

### `getservent`

```rust
unsafe fn getservent() -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1487`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1487)*

### `setservent`

```rust
unsafe fn setservent(stayopen: c_int)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1488`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1488)*

### `getprotobyname`

```rust
unsafe fn getprotobyname(name: *const c_char) -> *mut protoent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1489`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1489)*

### `getprotobynumber`

```rust
unsafe fn getprotobynumber(proto: c_int) -> *mut protoent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1490`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1490)*

### `chroot`

```rust
unsafe fn chroot(name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1491`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1491)*

### `usleep`

```rust
unsafe fn usleep(secs: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1499`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1499)*

### `send`

```rust
unsafe fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1505`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1505)*

### `recv`

```rust
unsafe fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1511`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1511)*

### `putenv`

```rust
unsafe fn putenv(string: *mut c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1517`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1517)*

### `poll`

```rust
unsafe fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1522`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1522)*

### `select`

```rust
unsafe fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1534-1540`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1534-L1540)*

### `setlocale`

```rust
unsafe fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1542`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1542)*

### `localeconv`

```rust
unsafe fn localeconv() -> *mut lconv
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1543`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1543)*

### `sem_wait`

```rust
unsafe fn sem_wait(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1549`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1549)*

### `sem_trywait`

```rust
unsafe fn sem_trywait(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1550`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1550)*

### `sem_post`

```rust
unsafe fn sem_post(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1551`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1551)*

### `statvfs`

```rust
unsafe fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1553`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1553)*

### `fstatvfs`

```rust
unsafe fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1555`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1555)*

### `sigemptyset`

```rust
unsafe fn sigemptyset(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1558`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1558)*

### `sigaddset`

```rust
unsafe fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1560`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1560)*

### `sigfillset`

```rust
unsafe fn sigfillset(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1562`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1562)*

### `sigdelset`

```rust
unsafe fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1564`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1564)*

### `sigismember`

```rust
unsafe fn sigismember(set: *const sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1566`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1566)*

### `sigprocmask`

```rust
unsafe fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1569`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1569)*

### `sigpending`

```rust
unsafe fn sigpending(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1571`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1571)*

### `sysconf`

```rust
unsafe fn sysconf(name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1574`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1574)*

### `mkfifo`

```rust
unsafe fn mkfifo(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1576`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1576)*

### `fseeko`

```rust
unsafe fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1579`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1579)*

### `ftello`

```rust
unsafe fn ftello(stream: *mut crate::FILE) -> off_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1581`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1581)*

### `tcdrain`

```rust
unsafe fn tcdrain(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1586`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1586)*

### `cfgetispeed`

```rust
unsafe fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1587`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1587)*

### `cfgetospeed`

```rust
unsafe fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1588`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1588)*

### `cfsetispeed`

```rust
unsafe fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1589`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1589)*

### `cfsetospeed`

```rust
unsafe fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1590`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1590)*

### `tcgetattr`

```rust
unsafe fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1591`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1591)*

### `tcsetattr`

```rust
unsafe fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1592`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1592)*

### `tcflow`

```rust
unsafe fn tcflow(fd: c_int, action: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1593`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1593)*

### `tcflush`

```rust
unsafe fn tcflush(fd: c_int, action: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1594`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1594)*

### `tcgetsid`

```rust
unsafe fn tcgetsid(fd: c_int) -> crate::pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1595`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1595)*

### `tcsendbreak`

```rust
unsafe fn tcsendbreak(fd: c_int, duration: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1596`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1596)*

### `mkstemp`

```rust
unsafe fn mkstemp(template: *mut c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1598`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1598)*

### `mkdtemp`

```rust
unsafe fn mkdtemp(template: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1599`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1599)*

### `tmpnam`

```rust
unsafe fn tmpnam(ptr: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1601`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1601)*

### `openlog`

```rust
unsafe fn openlog(ident: *const c_char, logopt: c_int, facility: c_int)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1603`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1603)*

### `closelog`

```rust
unsafe fn closelog()
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1604`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1604)*

### `setlogmask`

```rust
unsafe fn setlogmask(maskpri: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1605`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1605)*

### `syslog`

```rust
unsafe fn syslog(priority: c_int, message: *const c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1607`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1607)*

### `nice`

```rust
unsafe fn nice(incr: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1612`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1612)*

### `grantpt`

```rust
unsafe fn grantpt(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1614`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1614)*

### `posix_openpt`

```rust
unsafe fn posix_openpt(flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1615`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1615)*

### `ptsname`

```rust
unsafe fn ptsname(fd: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1616`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1616)*

### `unlockpt`

```rust
unsafe fn unlockpt(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1617`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1617)*

### `strcasestr`

```rust
unsafe fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1620`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1620)*

### `getline`

```rust
unsafe fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1621`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1621)*

### `lockf`

```rust
unsafe fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1624`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1624)*

### `adjtime`

```rust
unsafe fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1658`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1658)*

### `stpncpy`

```rust
unsafe fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1674`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1674)*

### `sigqueue`

```rust
unsafe fn sigqueue(pid: pid_t, sig: c_int, value: crate::sigval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1688`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1688)*

### `confstr`

```rust
unsafe fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1701`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1701)*

### `dladdr`

```rust
unsafe fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1709`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1709)*

### `flock`

```rust
unsafe fn flock(fd: c_int, operation: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1717`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1717)*

### `open_wmemstream`

```rust
unsafe fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1725`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1725)*

### `getsid`

```rust
unsafe fn getsid(pid: pid_t) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1733`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1733)*

### `pause`

```rust
unsafe fn pause() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1738`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1738)*

### `mkdirat`

```rust
unsafe fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1740`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1740)*

### `openat`

```rust
unsafe fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1742`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1742)*

### `fdopendir`

```rust
unsafe fn fdopendir(fd: c_int) -> *mut crate::DIR
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1752`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1752)*

### `readdir_r`

```rust
unsafe fn readdir_r(dirp: *mut crate::DIR, entry: *mut crate::dirent, result: *mut *mut crate::dirent) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1775-1779`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1775-L1779)*

The 64-bit libc on Solaris and illumos only has readdir_r. If a
32-bit Solaris or illumos target is ever created, it should use
__posix_readdir_r. See libc(3LIB) on Solaris or illumos:
https://illumos.org/man/3lib/libc
https://docs.oracle.com/cd/E36784_01/html/E36873/libc-3lib.html
https://www.unix.com/man-page/opensolaris/3LIB/libc/

### `readlinkat`

```rust
unsafe fn readlinkat(dirfd: c_int, pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1807-1812`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1807-L1812)*

### `fmemopen`

```rust
unsafe fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1813`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1813)*

### `open_memstream`

```rust
unsafe fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1814`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1814)*

### `atexit`

```rust
unsafe fn atexit(cb: fn()) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1815`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1815)*

### `sigaction`

```rust
unsafe fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1817-1818`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1817-L1818)*

### `readlink`

```rust
unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1819`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1819)*

### `pselect`

```rust
unsafe fn pselect(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1830-1837`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1830-L1837)*

### `cfmakeraw`

```rust
unsafe fn cfmakeraw(termios: *mut crate::termios)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1849`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1849)*

### `cfsetspeed`

```rust
unsafe fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1868`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1868)*

### `fnmatch`

```rust
unsafe fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1874`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1874)*

### `htonl`

```rust
const fn htonl(hostlong: u32) -> u32
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `htons`

```rust
const fn htons(hostshort: u16) -> u16
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `ntohl`

```rust
const fn ntohl(netlong: u32) -> u32
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `ntohs`

```rust
const fn ntohs(netshort: u16) -> u16
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

## Type Aliases

### `c_schar`

```rust
type c_schar = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:9`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L9)*

### `c_uchar`

```rust
type c_uchar = u8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:10`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L10)*

### `c_short`

```rust
type c_short = i16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:11`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L11)*

### `c_ushort`

```rust
type c_ushort = u16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:12`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L12)*

### `c_longlong`

```rust
type c_longlong = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:14`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L14)*

### `c_ulonglong`

```rust
type c_ulonglong = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:15`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L15)*

### `c_float`

```rust
type c_float = f32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:17`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L17)*

### `c_double`

```rust
type c_double = f64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:18`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L18)*

### `c_char`

```rust
type c_char = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:42`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L42)*

### `c_int`

```rust
type c_int = i32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:51`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L51)*

### `c_uint`

```rust
type c_uint = u32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:52`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L52)*

### `c_long`

```rust
type c_long = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:58`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L58)*

### `c_ulong`

```rust
type c_ulong = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:59`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L59)*

### `int8_t`

```rust
type int8_t = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:68`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L68)*

### `int16_t`

```rust
type int16_t = i16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:70`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L70)*

### `int32_t`

```rust
type int32_t = i32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:72`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L72)*

### `int64_t`

```rust
type int64_t = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:74`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L74)*

### `uint8_t`

```rust
type uint8_t = u8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:76`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L76)*

### `uint16_t`

```rust
type uint16_t = u16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:78`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L78)*

### `uint32_t`

```rust
type uint32_t = u32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:80`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L80)*

### `uint64_t`

```rust
type uint64_t = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:82`](../../.source_1765210505/libc-0.2.178/src/primitives.rs#L82)*

### `intmax_t`

```rust
type intmax_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:8`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L8)*

### `uintmax_t`

```rust
type uintmax_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:9`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L9)*

### `size_t`

```rust
type size_t = usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:11`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L11)*

### `ptrdiff_t`

```rust
type ptrdiff_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:12`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L12)*

### `intptr_t`

```rust
type intptr_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:13`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L13)*

### `uintptr_t`

```rust
type uintptr_t = usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:14`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L14)*

### `ssize_t`

```rust
type ssize_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:15`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L15)*

### `pid_t`

```rust
type pid_t = i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:17`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L17)*

### `in_addr_t`

```rust
type in_addr_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:18`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L18)*

### `in_port_t`

```rust
type in_port_t = u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:19`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L19)*

### `sighandler_t`

```rust
type sighandler_t = size_t;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:20`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L20)*

### `cc_t`

```rust
type cc_t = crate::c_uchar;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:21`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L21)*

### `uid_t`

```rust
type uid_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:35`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L35)*

### `gid_t`

```rust
type gid_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:36`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L36)*

### `locale_t`

```rust
type locale_t = *mut crate::c_void;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:45`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L45)*

## Constants

### `INT_MIN`
```rust
const INT_MIN: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:222`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L222)*

### `INT_MAX`
```rust
const INT_MAX: crate::c_int = 2_147_483_647i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:223`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L223)*

### `SIG_DFL`
```rust
const SIG_DFL: sighandler_t = 0usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:225`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L225)*

### `SIG_IGN`
```rust
const SIG_IGN: sighandler_t = 1usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:226`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L226)*

### `SIG_ERR`
```rust
const SIG_ERR: sighandler_t = 18_446_744_073_709_551_615usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:227`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L227)*

### `DT_UNKNOWN`
```rust
const DT_UNKNOWN: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:231`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L231)*

### `DT_FIFO`
```rust
const DT_FIFO: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:232`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L232)*

### `DT_CHR`
```rust
const DT_CHR: u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:233`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L233)*

### `DT_DIR`
```rust
const DT_DIR: u8 = 4u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:234`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L234)*

### `DT_BLK`
```rust
const DT_BLK: u8 = 6u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:235`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L235)*

### `DT_REG`
```rust
const DT_REG: u8 = 8u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:236`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L236)*

### `DT_LNK`
```rust
const DT_LNK: u8 = 10u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:237`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L237)*

### `DT_SOCK`
```rust
const DT_SOCK: u8 = 12u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:238`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L238)*

### `FD_CLOEXEC`
```rust
const FD_CLOEXEC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:243`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L243)*

### `USRQUOTA`
```rust
const USRQUOTA: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:249`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L249)*

### `GRPQUOTA`
```rust
const GRPQUOTA: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:250`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L250)*

### `SIGIOT`
```rust
const SIGIOT: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:253`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L253)*

### `S_ISUID`
```rust
const S_ISUID: mode_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:255`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L255)*

### `S_ISGID`
```rust
const S_ISGID: mode_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:256`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L256)*

### `S_ISVTX`
```rust
const S_ISVTX: mode_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:257`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L257)*

### `IF_NAMESIZE`
```rust
const IF_NAMESIZE: size_t = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:266`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L266)*

### `IFNAMSIZ`
```rust
const IFNAMSIZ: size_t = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:267`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L267)*

### `LOG_EMERG`
```rust
const LOG_EMERG: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:271`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L271)*

### `LOG_ALERT`
```rust
const LOG_ALERT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:272`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L272)*

### `LOG_CRIT`
```rust
const LOG_CRIT: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:273`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L273)*

### `LOG_ERR`
```rust
const LOG_ERR: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:274`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L274)*

### `LOG_WARNING`
```rust
const LOG_WARNING: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:275`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L275)*

### `LOG_NOTICE`
```rust
const LOG_NOTICE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:276`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L276)*

### `LOG_INFO`
```rust
const LOG_INFO: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:277`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L277)*

### `LOG_DEBUG`
```rust
const LOG_DEBUG: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:278`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L278)*

### `LOG_KERN`
```rust
const LOG_KERN: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:280`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L280)*

### `LOG_USER`
```rust
const LOG_USER: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:281`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L281)*

### `LOG_MAIL`
```rust
const LOG_MAIL: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:282`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L282)*

### `LOG_DAEMON`
```rust
const LOG_DAEMON: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:283`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L283)*

### `LOG_AUTH`
```rust
const LOG_AUTH: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:284`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L284)*

### `LOG_SYSLOG`
```rust
const LOG_SYSLOG: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:285`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L285)*

### `LOG_LPR`
```rust
const LOG_LPR: crate::c_int = 48i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:286`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L286)*

### `LOG_NEWS`
```rust
const LOG_NEWS: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:287`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L287)*

### `LOG_UUCP`
```rust
const LOG_UUCP: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:288`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L288)*

### `LOG_LOCAL0`
```rust
const LOG_LOCAL0: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:289`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L289)*

### `LOG_LOCAL1`
```rust
const LOG_LOCAL1: crate::c_int = 136i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:290`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L290)*

### `LOG_LOCAL2`
```rust
const LOG_LOCAL2: crate::c_int = 144i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:291`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L291)*

### `LOG_LOCAL3`
```rust
const LOG_LOCAL3: crate::c_int = 152i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:292`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L292)*

### `LOG_LOCAL4`
```rust
const LOG_LOCAL4: crate::c_int = 160i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:293`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L293)*

### `LOG_LOCAL5`
```rust
const LOG_LOCAL5: crate::c_int = 168i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:294`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L294)*

### `LOG_LOCAL6`
```rust
const LOG_LOCAL6: crate::c_int = 176i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:295`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L295)*

### `LOG_LOCAL7`
```rust
const LOG_LOCAL7: crate::c_int = 184i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:296`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L296)*

### `LOG_PID`
```rust
const LOG_PID: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:300`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L300)*

### `LOG_CONS`
```rust
const LOG_CONS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:301`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L301)*

### `LOG_ODELAY`
```rust
const LOG_ODELAY: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:302`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L302)*

### `LOG_NDELAY`
```rust
const LOG_NDELAY: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:303`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L303)*

### `LOG_NOWAIT`
```rust
const LOG_NOWAIT: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:304`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L304)*

### `LOG_PRIMASK`
```rust
const LOG_PRIMASK: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:307`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L307)*

### `LOG_FACMASK`
```rust
const LOG_FACMASK: crate::c_int = 1_016i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:308`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L308)*

### `PRIO_MIN`
```rust
const PRIO_MIN: crate::c_int = -20i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:312`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L312)*

### `PRIO_MAX`
```rust
const PRIO_MAX: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:313`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L313)*

### `IPPROTO_ICMP`
```rust
const IPPROTO_ICMP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:316`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L316)*

### `IPPROTO_ICMPV6`
```rust
const IPPROTO_ICMPV6: crate::c_int = 58i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:317`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L317)*

### `IPPROTO_TCP`
```rust
const IPPROTO_TCP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:318`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L318)*

### `IPPROTO_UDP`
```rust
const IPPROTO_UDP: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:319`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L319)*

### `IPPROTO_IP`
```rust
const IPPROTO_IP: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:320`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L320)*

### `IPPROTO_IPV6`
```rust
const IPPROTO_IPV6: crate::c_int = 41i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:321`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L321)*

### `INADDR_LOOPBACK`
```rust
const INADDR_LOOPBACK: in_addr_t = 2_130_706_433u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:323`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L323)*

### `INADDR_ANY`
```rust
const INADDR_ANY: in_addr_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:324`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L324)*

### `INADDR_BROADCAST`
```rust
const INADDR_BROADCAST: in_addr_t = 4_294_967_295u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:325`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L325)*

### `INADDR_NONE`
```rust
const INADDR_NONE: in_addr_t = 4_294_967_295u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:326`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L326)*

### `IN6ADDR_LOOPBACK_INIT`
```rust
const IN6ADDR_LOOPBACK_INIT: in6_addr;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:328-330`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L328-L330)*

### `IN6ADDR_ANY_INIT`
```rust
const IN6ADDR_ANY_INIT: in6_addr;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:332-334`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L332-L334)*

### `ARPOP_REQUEST`
```rust
const ARPOP_REQUEST: u16 = 1u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:336`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L336)*

### `ARPOP_REPLY`
```rust
const ARPOP_REPLY: u16 = 2u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:337`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L337)*

### `ATF_COM`
```rust
const ATF_COM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:339`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L339)*

### `ATF_PERM`
```rust
const ATF_PERM: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:340`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L340)*

### `ATF_PUBL`
```rust
const ATF_PUBL: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:341`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L341)*

### `ATF_USETRAILERS`
```rust
const ATF_USETRAILERS: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:342`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L342)*

### `FNM_PERIOD`
```rust
const FNM_PERIOD: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:348`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L348)*

### `FNM_NOMATCH`
```rust
const FNM_NOMATCH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:351`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L351)*

### `FNM_CASEFOLD`
```rust
const FNM_CASEFOLD: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:361`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L361)*

### `FNM_PATHNAME`
```rust
const FNM_PATHNAME: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:376`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L376)*

### `FNM_NOESCAPE`
```rust
const FNM_NOESCAPE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:395`](../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L395)*

