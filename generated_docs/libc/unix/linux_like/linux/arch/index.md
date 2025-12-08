*[libc](../../../../index.md) / [unix](../../../index.md) / [linux_like](../../index.md) / [linux](../index.md) / [arch](index.md)*

---

# Module `arch`

## Contents

- [Modules](#modules)
  - [`generic`](#generic)
- [Structs](#structs)
  - [`termios2`](#termios2)
- [Constants](#constants)
  - [`SOL_SOCKET`](#sol_socket)
  - [`SO_REUSEADDR`](#so_reuseaddr)
  - [`SO_TYPE`](#so_type)
  - [`SO_ERROR`](#so_error)
  - [`SO_DONTROUTE`](#so_dontroute)
  - [`SO_BROADCAST`](#so_broadcast)
  - [`SO_SNDBUF`](#so_sndbuf)
  - [`SO_RCVBUF`](#so_rcvbuf)
  - [`SO_KEEPALIVE`](#so_keepalive)
  - [`SO_OOBINLINE`](#so_oobinline)
  - [`SO_NO_CHECK`](#so_no_check)
  - [`SO_PRIORITY`](#so_priority)
  - [`SO_LINGER`](#so_linger)
  - [`SO_BSDCOMPAT`](#so_bsdcompat)
  - [`SO_REUSEPORT`](#so_reuseport)
  - [`SO_PASSCRED`](#so_passcred)
  - [`SO_PEERCRED`](#so_peercred)
  - [`SO_RCVLOWAT`](#so_rcvlowat)
  - [`SO_SNDLOWAT`](#so_sndlowat)
  - [`SO_SECURITY_AUTHENTICATION`](#so_security_authentication)
  - [`SO_SECURITY_ENCRYPTION_TRANSPORT`](#so_security_encryption_transport)
  - [`SO_SECURITY_ENCRYPTION_NETWORK`](#so_security_encryption_network)
  - [`SO_BINDTODEVICE`](#so_bindtodevice)
  - [`SO_ATTACH_FILTER`](#so_attach_filter)
  - [`SO_DETACH_FILTER`](#so_detach_filter)
  - [`SO_GET_FILTER`](#so_get_filter)
  - [`SO_PEERNAME`](#so_peername)
  - [`SO_TIMESTAMP_OLD`](#so_timestamp_old)
  - [`SO_TIMESTAMPNS_OLD`](#so_timestampns_old)
  - [`SO_TIMESTAMPING_OLD`](#so_timestamping_old)
  - [`SO_RCVTIMEO_OLD`](#so_rcvtimeo_old)
  - [`SO_SNDTIMEO_OLD`](#so_sndtimeo_old)
  - [`SO_TIMESTAMP`](#so_timestamp)
  - [`SO_TIMESTAMPNS`](#so_timestampns)
  - [`SO_TIMESTAMPING`](#so_timestamping)
  - [`SO_RCVTIMEO`](#so_rcvtimeo)
  - [`SO_SNDTIMEO`](#so_sndtimeo)
  - [`SO_ACCEPTCONN`](#so_acceptconn)
  - [`SO_PEERSEC`](#so_peersec)
  - [`SO_SNDBUFFORCE`](#so_sndbufforce)
  - [`SO_RCVBUFFORCE`](#so_rcvbufforce)
  - [`SO_PASSSEC`](#so_passsec)
  - [`SO_MARK`](#so_mark)
  - [`SO_PROTOCOL`](#so_protocol)
  - [`SO_DOMAIN`](#so_domain)
  - [`SO_RXQ_OVFL`](#so_rxq_ovfl)
  - [`SO_WIFI_STATUS`](#so_wifi_status)
  - [`SCM_WIFI_STATUS`](#scm_wifi_status)
  - [`SO_PEEK_OFF`](#so_peek_off)
  - [`SO_NOFCS`](#so_nofcs)
  - [`SO_LOCK_FILTER`](#so_lock_filter)
  - [`SO_SELECT_ERR_QUEUE`](#so_select_err_queue)
  - [`SO_BUSY_POLL`](#so_busy_poll)
  - [`SO_MAX_PACING_RATE`](#so_max_pacing_rate)
  - [`SO_BPF_EXTENSIONS`](#so_bpf_extensions)
  - [`SO_INCOMING_CPU`](#so_incoming_cpu)
  - [`SO_ATTACH_BPF`](#so_attach_bpf)
  - [`SO_DETACH_BPF`](#so_detach_bpf)
  - [`SO_ATTACH_REUSEPORT_CBPF`](#so_attach_reuseport_cbpf)
  - [`SO_ATTACH_REUSEPORT_EBPF`](#so_attach_reuseport_ebpf)
  - [`SO_CNX_ADVICE`](#so_cnx_advice)
  - [`SCM_TIMESTAMPING_OPT_STATS`](#scm_timestamping_opt_stats)
  - [`SO_MEMINFO`](#so_meminfo)
  - [`SO_INCOMING_NAPI_ID`](#so_incoming_napi_id)
  - [`SO_COOKIE`](#so_cookie)
  - [`SCM_TIMESTAMPING_PKTINFO`](#scm_timestamping_pktinfo)
  - [`SO_PEERGROUPS`](#so_peergroups)
  - [`SO_ZEROCOPY`](#so_zerocopy)
  - [`SO_TXTIME`](#so_txtime)
  - [`SCM_TXTIME`](#scm_txtime)
  - [`SO_BINDTOIFINDEX`](#so_bindtoifindex)
  - [`SO_TIMESTAMP_NEW`](#so_timestamp_new)
  - [`SO_TIMESTAMPNS_NEW`](#so_timestampns_new)
  - [`SO_TIMESTAMPING_NEW`](#so_timestamping_new)
  - [`SO_RCVTIMEO_NEW`](#so_rcvtimeo_new)
  - [`SO_SNDTIMEO_NEW`](#so_sndtimeo_new)
  - [`SO_DETACH_REUSEPORT_BPF`](#so_detach_reuseport_bpf)
  - [`SO_PREFER_BUSY_POLL`](#so_prefer_busy_poll)
  - [`SO_BUSY_POLL_BUDGET`](#so_busy_poll_budget)
  - [`SO_NETNS_COOKIE`](#so_netns_cookie)
  - [`SO_BUF_LOCK`](#so_buf_lock)
  - [`SO_RESERVE_MEM`](#so_reserve_mem)
  - [`SO_TXREHASH`](#so_txrehash)
  - [`SO_RCVMARK`](#so_rcvmark)
  - [`SO_PASSPIDFD`](#so_passpidfd)
  - [`SO_PEERPIDFD`](#so_peerpidfd)
  - [`SO_DEVMEM_LINEAR`](#so_devmem_linear)
  - [`SO_DEVMEM_DMABUF`](#so_devmem_dmabuf)
  - [`SO_DEVMEM_DONTNEED`](#so_devmem_dontneed)
  - [`SCM_TIMESTAMPNS`](#scm_timestampns)
  - [`SCM_TIMESTAMPING`](#scm_timestamping)
  - [`SCM_DEVMEM_LINEAR`](#scm_devmem_linear)
  - [`SCM_DEVMEM_DMABUF`](#scm_devmem_dmabuf)
  - [`TCGETS`](#tcgets)
  - [`TCSETS`](#tcsets)
  - [`TCSETSW`](#tcsetsw)
  - [`TCSETSF`](#tcsetsf)
  - [`TCGETA`](#tcgeta)
  - [`TCSETA`](#tcseta)
  - [`TCSETAW`](#tcsetaw)
  - [`TCSETAF`](#tcsetaf)
  - [`TCSBRK`](#tcsbrk)
  - [`TCXONC`](#tcxonc)
  - [`TCFLSH`](#tcflsh)
  - [`TIOCEXCL`](#tiocexcl)
  - [`TIOCNXCL`](#tiocnxcl)
  - [`TIOCSCTTY`](#tiocsctty)
  - [`TIOCGPGRP`](#tiocgpgrp)
  - [`TIOCSPGRP`](#tiocspgrp)
  - [`TIOCOUTQ`](#tiocoutq)
  - [`TIOCSTI`](#tiocsti)
  - [`TIOCGWINSZ`](#tiocgwinsz)
  - [`TIOCSWINSZ`](#tiocswinsz)
  - [`TIOCMGET`](#tiocmget)
  - [`TIOCMBIS`](#tiocmbis)
  - [`TIOCMBIC`](#tiocmbic)
  - [`TIOCMSET`](#tiocmset)
  - [`TIOCGSOFTCAR`](#tiocgsoftcar)
  - [`TIOCSSOFTCAR`](#tiocssoftcar)
  - [`FIONREAD`](#fionread)
  - [`TIOCINQ`](#tiocinq)
  - [`TIOCLINUX`](#tioclinux)
  - [`TIOCCONS`](#tioccons)
  - [`TIOCGSERIAL`](#tiocgserial)
  - [`TIOCSSERIAL`](#tiocsserial)
  - [`TIOCPKT`](#tiocpkt)
  - [`FIONBIO`](#fionbio)
  - [`TIOCNOTTY`](#tiocnotty)
  - [`TIOCSETD`](#tiocsetd)
  - [`TIOCGETD`](#tiocgetd)
  - [`TCSBRKP`](#tcsbrkp)
  - [`TIOCSBRK`](#tiocsbrk)
  - [`TIOCCBRK`](#tioccbrk)
  - [`TIOCGSID`](#tiocgsid)
  - [`TCGETS2`](#tcgets2)
  - [`TCSETS2`](#tcsets2)
  - [`TCSETSW2`](#tcsetsw2)
  - [`TCSETSF2`](#tcsetsf2)
  - [`TIOCGRS485`](#tiocgrs485)
  - [`TIOCSRS485`](#tiocsrs485)
  - [`TIOCGPTN`](#tiocgptn)
  - [`TIOCSPTLCK`](#tiocsptlck)
  - [`TIOCGDEV`](#tiocgdev)
  - [`TCGETX`](#tcgetx)
  - [`TCSETX`](#tcsetx)
  - [`TCSETXF`](#tcsetxf)
  - [`TCSETXW`](#tcsetxw)
  - [`TIOCSIG`](#tiocsig)
  - [`TIOCVHANGUP`](#tiocvhangup)
  - [`TIOCGPKT`](#tiocgpkt)
  - [`TIOCGPTLCK`](#tiocgptlck)
  - [`TIOCGEXCL`](#tiocgexcl)
  - [`TIOCGPTPEER`](#tiocgptpeer)
  - [`FIONCLEX`](#fionclex)
  - [`FIOCLEX`](#fioclex)
  - [`FIOASYNC`](#fioasync)
  - [`TIOCSERCONFIG`](#tiocserconfig)
  - [`TIOCSERGWILD`](#tiocsergwild)
  - [`TIOCSERSWILD`](#tiocserswild)
  - [`TIOCGLCKTRMIOS`](#tiocglcktrmios)
  - [`TIOCSLCKTRMIOS`](#tiocslcktrmios)
  - [`TIOCSERGSTRUCT`](#tiocsergstruct)
  - [`TIOCSERGETLSR`](#tiocsergetlsr)
  - [`TIOCSERGETMULTI`](#tiocsergetmulti)
  - [`TIOCSERSETMULTI`](#tiocsersetmulti)
  - [`TIOCMIWAIT`](#tiocmiwait)
  - [`TIOCGICOUNT`](#tiocgicount)
  - [`BLKIOMIN`](#blkiomin)
  - [`BLKIOOPT`](#blkioopt)
  - [`BLKSSZGET`](#blksszget)
  - [`BLKPBSZGET`](#blkpbszget)
  - [`FIOQSIZE`](#fioqsize)
  - [`TIOCM_LE`](#tiocm_le)
  - [`TIOCM_DTR`](#tiocm_dtr)
  - [`TIOCM_RTS`](#tiocm_rts)
  - [`TIOCM_ST`](#tiocm_st)
  - [`TIOCM_SR`](#tiocm_sr)
  - [`TIOCM_CTS`](#tiocm_cts)
  - [`TIOCM_CAR`](#tiocm_car)
  - [`TIOCM_CD`](#tiocm_cd)
  - [`TIOCM_RNG`](#tiocm_rng)
  - [`TIOCM_RI`](#tiocm_ri)
  - [`TIOCM_DSR`](#tiocm_dsr)
  - [`BOTHER`](#bother)
  - [`IBSHIFT`](#ibshift)
  - [`IUCLC`](#iuclc)
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
  - [`RLIMIT_NLIMITS`](#rlimit_nlimits)
  - [`RLIM_NLIMITS`](#rlim_nlimits)
  - [`RLIM_INFINITY`](#rlim_infinity)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`generic`](#generic) | mod |  |
| [`termios2`](#termios2) | struct |  |
| [`SOL_SOCKET`](#sol_socket) | const |  |
| [`SO_REUSEADDR`](#so_reuseaddr) | const |  |
| [`SO_TYPE`](#so_type) | const |  |
| [`SO_ERROR`](#so_error) | const |  |
| [`SO_DONTROUTE`](#so_dontroute) | const |  |
| [`SO_BROADCAST`](#so_broadcast) | const |  |
| [`SO_SNDBUF`](#so_sndbuf) | const |  |
| [`SO_RCVBUF`](#so_rcvbuf) | const |  |
| [`SO_KEEPALIVE`](#so_keepalive) | const |  |
| [`SO_OOBINLINE`](#so_oobinline) | const |  |
| [`SO_NO_CHECK`](#so_no_check) | const |  |
| [`SO_PRIORITY`](#so_priority) | const |  |
| [`SO_LINGER`](#so_linger) | const |  |
| [`SO_BSDCOMPAT`](#so_bsdcompat) | const |  |
| [`SO_REUSEPORT`](#so_reuseport) | const |  |
| [`SO_PASSCRED`](#so_passcred) | const |  |
| [`SO_PEERCRED`](#so_peercred) | const |  |
| [`SO_RCVLOWAT`](#so_rcvlowat) | const |  |
| [`SO_SNDLOWAT`](#so_sndlowat) | const |  |
| [`SO_SECURITY_AUTHENTICATION`](#so_security_authentication) | const |  |
| [`SO_SECURITY_ENCRYPTION_TRANSPORT`](#so_security_encryption_transport) | const |  |
| [`SO_SECURITY_ENCRYPTION_NETWORK`](#so_security_encryption_network) | const |  |
| [`SO_BINDTODEVICE`](#so_bindtodevice) | const |  |
| [`SO_ATTACH_FILTER`](#so_attach_filter) | const |  |
| [`SO_DETACH_FILTER`](#so_detach_filter) | const |  |
| [`SO_GET_FILTER`](#so_get_filter) | const |  |
| [`SO_PEERNAME`](#so_peername) | const |  |
| [`SO_TIMESTAMP_OLD`](#so_timestamp_old) | const |  |
| [`SO_TIMESTAMPNS_OLD`](#so_timestampns_old) | const |  |
| [`SO_TIMESTAMPING_OLD`](#so_timestamping_old) | const |  |
| [`SO_RCVTIMEO_OLD`](#so_rcvtimeo_old) | const |  |
| [`SO_SNDTIMEO_OLD`](#so_sndtimeo_old) | const |  |
| [`SO_TIMESTAMP`](#so_timestamp) | const |  |
| [`SO_TIMESTAMPNS`](#so_timestampns) | const |  |
| [`SO_TIMESTAMPING`](#so_timestamping) | const |  |
| [`SO_RCVTIMEO`](#so_rcvtimeo) | const |  |
| [`SO_SNDTIMEO`](#so_sndtimeo) | const |  |
| [`SO_ACCEPTCONN`](#so_acceptconn) | const |  |
| [`SO_PEERSEC`](#so_peersec) | const |  |
| [`SO_SNDBUFFORCE`](#so_sndbufforce) | const |  |
| [`SO_RCVBUFFORCE`](#so_rcvbufforce) | const |  |
| [`SO_PASSSEC`](#so_passsec) | const |  |
| [`SO_MARK`](#so_mark) | const |  |
| [`SO_PROTOCOL`](#so_protocol) | const |  |
| [`SO_DOMAIN`](#so_domain) | const |  |
| [`SO_RXQ_OVFL`](#so_rxq_ovfl) | const |  |
| [`SO_WIFI_STATUS`](#so_wifi_status) | const |  |
| [`SCM_WIFI_STATUS`](#scm_wifi_status) | const |  |
| [`SO_PEEK_OFF`](#so_peek_off) | const |  |
| [`SO_NOFCS`](#so_nofcs) | const |  |
| [`SO_LOCK_FILTER`](#so_lock_filter) | const |  |
| [`SO_SELECT_ERR_QUEUE`](#so_select_err_queue) | const |  |
| [`SO_BUSY_POLL`](#so_busy_poll) | const |  |
| [`SO_MAX_PACING_RATE`](#so_max_pacing_rate) | const |  |
| [`SO_BPF_EXTENSIONS`](#so_bpf_extensions) | const |  |
| [`SO_INCOMING_CPU`](#so_incoming_cpu) | const |  |
| [`SO_ATTACH_BPF`](#so_attach_bpf) | const |  |
| [`SO_DETACH_BPF`](#so_detach_bpf) | const |  |
| [`SO_ATTACH_REUSEPORT_CBPF`](#so_attach_reuseport_cbpf) | const |  |
| [`SO_ATTACH_REUSEPORT_EBPF`](#so_attach_reuseport_ebpf) | const |  |
| [`SO_CNX_ADVICE`](#so_cnx_advice) | const |  |
| [`SCM_TIMESTAMPING_OPT_STATS`](#scm_timestamping_opt_stats) | const |  |
| [`SO_MEMINFO`](#so_meminfo) | const |  |
| [`SO_INCOMING_NAPI_ID`](#so_incoming_napi_id) | const |  |
| [`SO_COOKIE`](#so_cookie) | const |  |
| [`SCM_TIMESTAMPING_PKTINFO`](#scm_timestamping_pktinfo) | const |  |
| [`SO_PEERGROUPS`](#so_peergroups) | const |  |
| [`SO_ZEROCOPY`](#so_zerocopy) | const |  |
| [`SO_TXTIME`](#so_txtime) | const |  |
| [`SCM_TXTIME`](#scm_txtime) | const |  |
| [`SO_BINDTOIFINDEX`](#so_bindtoifindex) | const |  |
| [`SO_TIMESTAMP_NEW`](#so_timestamp_new) | const |  |
| [`SO_TIMESTAMPNS_NEW`](#so_timestampns_new) | const |  |
| [`SO_TIMESTAMPING_NEW`](#so_timestamping_new) | const |  |
| [`SO_RCVTIMEO_NEW`](#so_rcvtimeo_new) | const |  |
| [`SO_SNDTIMEO_NEW`](#so_sndtimeo_new) | const |  |
| [`SO_DETACH_REUSEPORT_BPF`](#so_detach_reuseport_bpf) | const |  |
| [`SO_PREFER_BUSY_POLL`](#so_prefer_busy_poll) | const |  |
| [`SO_BUSY_POLL_BUDGET`](#so_busy_poll_budget) | const |  |
| [`SO_NETNS_COOKIE`](#so_netns_cookie) | const |  |
| [`SO_BUF_LOCK`](#so_buf_lock) | const |  |
| [`SO_RESERVE_MEM`](#so_reserve_mem) | const |  |
| [`SO_TXREHASH`](#so_txrehash) | const |  |
| [`SO_RCVMARK`](#so_rcvmark) | const |  |
| [`SO_PASSPIDFD`](#so_passpidfd) | const |  |
| [`SO_PEERPIDFD`](#so_peerpidfd) | const |  |
| [`SO_DEVMEM_LINEAR`](#so_devmem_linear) | const |  |
| [`SO_DEVMEM_DMABUF`](#so_devmem_dmabuf) | const |  |
| [`SO_DEVMEM_DONTNEED`](#so_devmem_dontneed) | const |  |
| [`SCM_TIMESTAMPNS`](#scm_timestampns) | const |  |
| [`SCM_TIMESTAMPING`](#scm_timestamping) | const |  |
| [`SCM_DEVMEM_LINEAR`](#scm_devmem_linear) | const |  |
| [`SCM_DEVMEM_DMABUF`](#scm_devmem_dmabuf) | const |  |
| [`TCGETS`](#tcgets) | const |  |
| [`TCSETS`](#tcsets) | const |  |
| [`TCSETSW`](#tcsetsw) | const |  |
| [`TCSETSF`](#tcsetsf) | const |  |
| [`TCGETA`](#tcgeta) | const |  |
| [`TCSETA`](#tcseta) | const |  |
| [`TCSETAW`](#tcsetaw) | const |  |
| [`TCSETAF`](#tcsetaf) | const |  |
| [`TCSBRK`](#tcsbrk) | const |  |
| [`TCXONC`](#tcxonc) | const |  |
| [`TCFLSH`](#tcflsh) | const |  |
| [`TIOCEXCL`](#tiocexcl) | const |  |
| [`TIOCNXCL`](#tiocnxcl) | const |  |
| [`TIOCSCTTY`](#tiocsctty) | const |  |
| [`TIOCGPGRP`](#tiocgpgrp) | const |  |
| [`TIOCSPGRP`](#tiocspgrp) | const |  |
| [`TIOCOUTQ`](#tiocoutq) | const |  |
| [`TIOCSTI`](#tiocsti) | const |  |
| [`TIOCGWINSZ`](#tiocgwinsz) | const |  |
| [`TIOCSWINSZ`](#tiocswinsz) | const |  |
| [`TIOCMGET`](#tiocmget) | const |  |
| [`TIOCMBIS`](#tiocmbis) | const |  |
| [`TIOCMBIC`](#tiocmbic) | const |  |
| [`TIOCMSET`](#tiocmset) | const |  |
| [`TIOCGSOFTCAR`](#tiocgsoftcar) | const |  |
| [`TIOCSSOFTCAR`](#tiocssoftcar) | const |  |
| [`FIONREAD`](#fionread) | const |  |
| [`TIOCINQ`](#tiocinq) | const |  |
| [`TIOCLINUX`](#tioclinux) | const |  |
| [`TIOCCONS`](#tioccons) | const |  |
| [`TIOCGSERIAL`](#tiocgserial) | const |  |
| [`TIOCSSERIAL`](#tiocsserial) | const |  |
| [`TIOCPKT`](#tiocpkt) | const |  |
| [`FIONBIO`](#fionbio) | const |  |
| [`TIOCNOTTY`](#tiocnotty) | const |  |
| [`TIOCSETD`](#tiocsetd) | const |  |
| [`TIOCGETD`](#tiocgetd) | const |  |
| [`TCSBRKP`](#tcsbrkp) | const |  |
| [`TIOCSBRK`](#tiocsbrk) | const |  |
| [`TIOCCBRK`](#tioccbrk) | const |  |
| [`TIOCGSID`](#tiocgsid) | const |  |
| [`TCGETS2`](#tcgets2) | const |  |
| [`TCSETS2`](#tcsets2) | const |  |
| [`TCSETSW2`](#tcsetsw2) | const |  |
| [`TCSETSF2`](#tcsetsf2) | const |  |
| [`TIOCGRS485`](#tiocgrs485) | const |  |
| [`TIOCSRS485`](#tiocsrs485) | const |  |
| [`TIOCGPTN`](#tiocgptn) | const |  |
| [`TIOCSPTLCK`](#tiocsptlck) | const |  |
| [`TIOCGDEV`](#tiocgdev) | const |  |
| [`TCGETX`](#tcgetx) | const |  |
| [`TCSETX`](#tcsetx) | const |  |
| [`TCSETXF`](#tcsetxf) | const |  |
| [`TCSETXW`](#tcsetxw) | const |  |
| [`TIOCSIG`](#tiocsig) | const |  |
| [`TIOCVHANGUP`](#tiocvhangup) | const |  |
| [`TIOCGPKT`](#tiocgpkt) | const |  |
| [`TIOCGPTLCK`](#tiocgptlck) | const |  |
| [`TIOCGEXCL`](#tiocgexcl) | const |  |
| [`TIOCGPTPEER`](#tiocgptpeer) | const |  |
| [`FIONCLEX`](#fionclex) | const |  |
| [`FIOCLEX`](#fioclex) | const |  |
| [`FIOASYNC`](#fioasync) | const |  |
| [`TIOCSERCONFIG`](#tiocserconfig) | const |  |
| [`TIOCSERGWILD`](#tiocsergwild) | const |  |
| [`TIOCSERSWILD`](#tiocserswild) | const |  |
| [`TIOCGLCKTRMIOS`](#tiocglcktrmios) | const |  |
| [`TIOCSLCKTRMIOS`](#tiocslcktrmios) | const |  |
| [`TIOCSERGSTRUCT`](#tiocsergstruct) | const |  |
| [`TIOCSERGETLSR`](#tiocsergetlsr) | const |  |
| [`TIOCSERGETMULTI`](#tiocsergetmulti) | const |  |
| [`TIOCSERSETMULTI`](#tiocsersetmulti) | const |  |
| [`TIOCMIWAIT`](#tiocmiwait) | const |  |
| [`TIOCGICOUNT`](#tiocgicount) | const |  |
| [`BLKIOMIN`](#blkiomin) | const |  |
| [`BLKIOOPT`](#blkioopt) | const |  |
| [`BLKSSZGET`](#blksszget) | const |  |
| [`BLKPBSZGET`](#blkpbszget) | const |  |
| [`FIOQSIZE`](#fioqsize) | const |  |
| [`TIOCM_LE`](#tiocm_le) | const |  |
| [`TIOCM_DTR`](#tiocm_dtr) | const |  |
| [`TIOCM_RTS`](#tiocm_rts) | const |  |
| [`TIOCM_ST`](#tiocm_st) | const |  |
| [`TIOCM_SR`](#tiocm_sr) | const |  |
| [`TIOCM_CTS`](#tiocm_cts) | const |  |
| [`TIOCM_CAR`](#tiocm_car) | const |  |
| [`TIOCM_CD`](#tiocm_cd) | const |  |
| [`TIOCM_RNG`](#tiocm_rng) | const |  |
| [`TIOCM_RI`](#tiocm_ri) | const |  |
| [`TIOCM_DSR`](#tiocm_dsr) | const |  |
| [`BOTHER`](#bother) | const |  |
| [`IBSHIFT`](#ibshift) | const |  |
| [`IUCLC`](#iuclc) | const |  |
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
| [`RLIMIT_NLIMITS`](#rlimit_nlimits) | const |  |
| [`RLIM_NLIMITS`](#rlim_nlimits) | const |  |
| [`RLIM_INFINITY`](#rlim_infinity) | const |  |

## Modules

- [`generic`](generic/index.md) - 

## Structs

### `termios2`

```rust
struct termios2 {
    pub c_iflag: crate::tcflag_t,
    pub c_oflag: crate::tcflag_t,
    pub c_cflag: crate::tcflag_t,
    pub c_lflag: crate::tcflag_t,
    pub c_line: crate::cc_t,
    pub c_cc: [crate::cc_t; 19],
    pub c_ispeed: crate::speed_t,
    pub c_ospeed: crate::speed_t,
}
```

#### Trait Implementations

##### `impl Clone for termios2`

- <span id="termios2-clone"></span>`fn clone(&self) -> termios2` — [`termios2`](#termios2)

##### `impl Copy for termios2`

##### `impl Debug for termios2`

- <span id="termios2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `SOL_SOCKET`

```rust
const SOL_SOCKET: crate::c_int = 1i32;
```

### `SO_REUSEADDR`

```rust
const SO_REUSEADDR: crate::c_int = 2i32;
```

### `SO_TYPE`

```rust
const SO_TYPE: crate::c_int = 3i32;
```

### `SO_ERROR`

```rust
const SO_ERROR: crate::c_int = 4i32;
```

### `SO_DONTROUTE`

```rust
const SO_DONTROUTE: crate::c_int = 5i32;
```

### `SO_BROADCAST`

```rust
const SO_BROADCAST: crate::c_int = 6i32;
```

### `SO_SNDBUF`

```rust
const SO_SNDBUF: crate::c_int = 7i32;
```

### `SO_RCVBUF`

```rust
const SO_RCVBUF: crate::c_int = 8i32;
```

### `SO_KEEPALIVE`

```rust
const SO_KEEPALIVE: crate::c_int = 9i32;
```

### `SO_OOBINLINE`

```rust
const SO_OOBINLINE: crate::c_int = 10i32;
```

### `SO_NO_CHECK`

```rust
const SO_NO_CHECK: crate::c_int = 11i32;
```

### `SO_PRIORITY`

```rust
const SO_PRIORITY: crate::c_int = 12i32;
```

### `SO_LINGER`

```rust
const SO_LINGER: crate::c_int = 13i32;
```

### `SO_BSDCOMPAT`

```rust
const SO_BSDCOMPAT: crate::c_int = 14i32;
```

### `SO_REUSEPORT`

```rust
const SO_REUSEPORT: crate::c_int = 15i32;
```

### `SO_PASSCRED`

```rust
const SO_PASSCRED: crate::c_int = 16i32;
```

### `SO_PEERCRED`

```rust
const SO_PEERCRED: crate::c_int = 17i32;
```

### `SO_RCVLOWAT`

```rust
const SO_RCVLOWAT: crate::c_int = 18i32;
```

### `SO_SNDLOWAT`

```rust
const SO_SNDLOWAT: crate::c_int = 19i32;
```

### `SO_SECURITY_AUTHENTICATION`

```rust
const SO_SECURITY_AUTHENTICATION: crate::c_int = 22i32;
```

### `SO_SECURITY_ENCRYPTION_TRANSPORT`

```rust
const SO_SECURITY_ENCRYPTION_TRANSPORT: crate::c_int = 23i32;
```

### `SO_SECURITY_ENCRYPTION_NETWORK`

```rust
const SO_SECURITY_ENCRYPTION_NETWORK: crate::c_int = 24i32;
```

### `SO_BINDTODEVICE`

```rust
const SO_BINDTODEVICE: crate::c_int = 25i32;
```

### `SO_ATTACH_FILTER`

```rust
const SO_ATTACH_FILTER: crate::c_int = 26i32;
```

### `SO_DETACH_FILTER`

```rust
const SO_DETACH_FILTER: crate::c_int = 27i32;
```

### `SO_GET_FILTER`

```rust
const SO_GET_FILTER: crate::c_int = 26i32;
```

### `SO_PEERNAME`

```rust
const SO_PEERNAME: crate::c_int = 28i32;
```

### `SO_TIMESTAMP_OLD`

```rust
const SO_TIMESTAMP_OLD: crate::c_int = 29i32;
```

### `SO_TIMESTAMPNS_OLD`

```rust
const SO_TIMESTAMPNS_OLD: crate::c_int = 35i32;
```

### `SO_TIMESTAMPING_OLD`

```rust
const SO_TIMESTAMPING_OLD: crate::c_int = 37i32;
```

### `SO_RCVTIMEO_OLD`

```rust
const SO_RCVTIMEO_OLD: crate::c_int = 20i32;
```

### `SO_SNDTIMEO_OLD`

```rust
const SO_SNDTIMEO_OLD: crate::c_int = 21i32;
```

### `SO_TIMESTAMP`

```rust
const SO_TIMESTAMP: crate::c_int = 29i32;
```

### `SO_TIMESTAMPNS`

```rust
const SO_TIMESTAMPNS: crate::c_int = 35i32;
```

### `SO_TIMESTAMPING`

```rust
const SO_TIMESTAMPING: crate::c_int = 37i32;
```

### `SO_RCVTIMEO`

```rust
const SO_RCVTIMEO: crate::c_int = 20i32;
```

### `SO_SNDTIMEO`

```rust
const SO_SNDTIMEO: crate::c_int = 21i32;
```

### `SO_ACCEPTCONN`

```rust
const SO_ACCEPTCONN: crate::c_int = 30i32;
```

### `SO_PEERSEC`

```rust
const SO_PEERSEC: crate::c_int = 31i32;
```

### `SO_SNDBUFFORCE`

```rust
const SO_SNDBUFFORCE: crate::c_int = 32i32;
```

### `SO_RCVBUFFORCE`

```rust
const SO_RCVBUFFORCE: crate::c_int = 33i32;
```

### `SO_PASSSEC`

```rust
const SO_PASSSEC: crate::c_int = 34i32;
```

### `SO_MARK`

```rust
const SO_MARK: crate::c_int = 36i32;
```

### `SO_PROTOCOL`

```rust
const SO_PROTOCOL: crate::c_int = 38i32;
```

### `SO_DOMAIN`

```rust
const SO_DOMAIN: crate::c_int = 39i32;
```

### `SO_RXQ_OVFL`

```rust
const SO_RXQ_OVFL: crate::c_int = 40i32;
```

### `SO_WIFI_STATUS`

```rust
const SO_WIFI_STATUS: crate::c_int = 41i32;
```

### `SCM_WIFI_STATUS`

```rust
const SCM_WIFI_STATUS: crate::c_int = 41i32;
```

### `SO_PEEK_OFF`

```rust
const SO_PEEK_OFF: crate::c_int = 42i32;
```

### `SO_NOFCS`

```rust
const SO_NOFCS: crate::c_int = 43i32;
```

### `SO_LOCK_FILTER`

```rust
const SO_LOCK_FILTER: crate::c_int = 44i32;
```

### `SO_SELECT_ERR_QUEUE`

```rust
const SO_SELECT_ERR_QUEUE: crate::c_int = 45i32;
```

### `SO_BUSY_POLL`

```rust
const SO_BUSY_POLL: crate::c_int = 46i32;
```

### `SO_MAX_PACING_RATE`

```rust
const SO_MAX_PACING_RATE: crate::c_int = 47i32;
```

### `SO_BPF_EXTENSIONS`

```rust
const SO_BPF_EXTENSIONS: crate::c_int = 48i32;
```

### `SO_INCOMING_CPU`

```rust
const SO_INCOMING_CPU: crate::c_int = 49i32;
```

### `SO_ATTACH_BPF`

```rust
const SO_ATTACH_BPF: crate::c_int = 50i32;
```

### `SO_DETACH_BPF`

```rust
const SO_DETACH_BPF: crate::c_int = 27i32;
```

### `SO_ATTACH_REUSEPORT_CBPF`

```rust
const SO_ATTACH_REUSEPORT_CBPF: crate::c_int = 51i32;
```

### `SO_ATTACH_REUSEPORT_EBPF`

```rust
const SO_ATTACH_REUSEPORT_EBPF: crate::c_int = 52i32;
```

### `SO_CNX_ADVICE`

```rust
const SO_CNX_ADVICE: crate::c_int = 53i32;
```

### `SCM_TIMESTAMPING_OPT_STATS`

```rust
const SCM_TIMESTAMPING_OPT_STATS: crate::c_int = 54i32;
```

### `SO_MEMINFO`

```rust
const SO_MEMINFO: crate::c_int = 55i32;
```

### `SO_INCOMING_NAPI_ID`

```rust
const SO_INCOMING_NAPI_ID: crate::c_int = 56i32;
```

### `SO_COOKIE`

```rust
const SO_COOKIE: crate::c_int = 57i32;
```

### `SCM_TIMESTAMPING_PKTINFO`

```rust
const SCM_TIMESTAMPING_PKTINFO: crate::c_int = 58i32;
```

### `SO_PEERGROUPS`

```rust
const SO_PEERGROUPS: crate::c_int = 59i32;
```

### `SO_ZEROCOPY`

```rust
const SO_ZEROCOPY: crate::c_int = 60i32;
```

### `SO_TXTIME`

```rust
const SO_TXTIME: crate::c_int = 61i32;
```

### `SCM_TXTIME`

```rust
const SCM_TXTIME: crate::c_int = 61i32;
```

### `SO_BINDTOIFINDEX`

```rust
const SO_BINDTOIFINDEX: crate::c_int = 62i32;
```

### `SO_TIMESTAMP_NEW`

```rust
const SO_TIMESTAMP_NEW: crate::c_int = 63i32;
```

### `SO_TIMESTAMPNS_NEW`

```rust
const SO_TIMESTAMPNS_NEW: crate::c_int = 64i32;
```

### `SO_TIMESTAMPING_NEW`

```rust
const SO_TIMESTAMPING_NEW: crate::c_int = 65i32;
```

### `SO_RCVTIMEO_NEW`

```rust
const SO_RCVTIMEO_NEW: crate::c_int = 66i32;
```

### `SO_SNDTIMEO_NEW`

```rust
const SO_SNDTIMEO_NEW: crate::c_int = 67i32;
```

### `SO_DETACH_REUSEPORT_BPF`

```rust
const SO_DETACH_REUSEPORT_BPF: crate::c_int = 68i32;
```

### `SO_PREFER_BUSY_POLL`

```rust
const SO_PREFER_BUSY_POLL: crate::c_int = 69i32;
```

### `SO_BUSY_POLL_BUDGET`

```rust
const SO_BUSY_POLL_BUDGET: crate::c_int = 70i32;
```

### `SO_NETNS_COOKIE`

```rust
const SO_NETNS_COOKIE: crate::c_int = 71i32;
```

### `SO_BUF_LOCK`

```rust
const SO_BUF_LOCK: crate::c_int = 72i32;
```

### `SO_RESERVE_MEM`

```rust
const SO_RESERVE_MEM: crate::c_int = 73i32;
```

### `SO_TXREHASH`

```rust
const SO_TXREHASH: crate::c_int = 74i32;
```

### `SO_RCVMARK`

```rust
const SO_RCVMARK: crate::c_int = 75i32;
```

### `SO_PASSPIDFD`

```rust
const SO_PASSPIDFD: crate::c_int = 76i32;
```

### `SO_PEERPIDFD`

```rust
const SO_PEERPIDFD: crate::c_int = 77i32;
```

### `SO_DEVMEM_LINEAR`

```rust
const SO_DEVMEM_LINEAR: crate::c_int = 78i32;
```

### `SO_DEVMEM_DMABUF`

```rust
const SO_DEVMEM_DMABUF: crate::c_int = 79i32;
```

### `SO_DEVMEM_DONTNEED`

```rust
const SO_DEVMEM_DONTNEED: crate::c_int = 80i32;
```

### `SCM_TIMESTAMPNS`

```rust
const SCM_TIMESTAMPNS: crate::c_int = 35i32;
```

### `SCM_TIMESTAMPING`

```rust
const SCM_TIMESTAMPING: crate::c_int = 37i32;
```

### `SCM_DEVMEM_LINEAR`

```rust
const SCM_DEVMEM_LINEAR: crate::c_int = 78i32;
```

### `SCM_DEVMEM_DMABUF`

```rust
const SCM_DEVMEM_DMABUF: crate::c_int = 79i32;
```

### `TCGETS`

```rust
const TCGETS: crate::c_ulong = 21_505u64;
```

### `TCSETS`

```rust
const TCSETS: crate::c_ulong = 21_506u64;
```

### `TCSETSW`

```rust
const TCSETSW: crate::c_ulong = 21_507u64;
```

### `TCSETSF`

```rust
const TCSETSF: crate::c_ulong = 21_508u64;
```

### `TCGETA`

```rust
const TCGETA: crate::c_ulong = 21_509u64;
```

### `TCSETA`

```rust
const TCSETA: crate::c_ulong = 21_510u64;
```

### `TCSETAW`

```rust
const TCSETAW: crate::c_ulong = 21_511u64;
```

### `TCSETAF`

```rust
const TCSETAF: crate::c_ulong = 21_512u64;
```

### `TCSBRK`

```rust
const TCSBRK: crate::c_ulong = 21_513u64;
```

### `TCXONC`

```rust
const TCXONC: crate::c_ulong = 21_514u64;
```

### `TCFLSH`

```rust
const TCFLSH: crate::c_ulong = 21_515u64;
```

### `TIOCEXCL`

```rust
const TIOCEXCL: crate::c_ulong = 21_516u64;
```

### `TIOCNXCL`

```rust
const TIOCNXCL: crate::c_ulong = 21_517u64;
```

### `TIOCSCTTY`

```rust
const TIOCSCTTY: crate::c_ulong = 21_518u64;
```

### `TIOCGPGRP`

```rust
const TIOCGPGRP: crate::c_ulong = 21_519u64;
```

### `TIOCSPGRP`

```rust
const TIOCSPGRP: crate::c_ulong = 21_520u64;
```

### `TIOCOUTQ`

```rust
const TIOCOUTQ: crate::c_ulong = 21_521u64;
```

### `TIOCSTI`

```rust
const TIOCSTI: crate::c_ulong = 21_522u64;
```

### `TIOCGWINSZ`

```rust
const TIOCGWINSZ: crate::c_ulong = 21_523u64;
```

### `TIOCSWINSZ`

```rust
const TIOCSWINSZ: crate::c_ulong = 21_524u64;
```

### `TIOCMGET`

```rust
const TIOCMGET: crate::c_ulong = 21_525u64;
```

### `TIOCMBIS`

```rust
const TIOCMBIS: crate::c_ulong = 21_526u64;
```

### `TIOCMBIC`

```rust
const TIOCMBIC: crate::c_ulong = 21_527u64;
```

### `TIOCMSET`

```rust
const TIOCMSET: crate::c_ulong = 21_528u64;
```

### `TIOCGSOFTCAR`

```rust
const TIOCGSOFTCAR: crate::c_ulong = 21_529u64;
```

### `TIOCSSOFTCAR`

```rust
const TIOCSSOFTCAR: crate::c_ulong = 21_530u64;
```

### `FIONREAD`

```rust
const FIONREAD: crate::c_ulong = 21_531u64;
```

### `TIOCINQ`

```rust
const TIOCINQ: crate::c_ulong = 21_531u64;
```

### `TIOCLINUX`

```rust
const TIOCLINUX: crate::c_ulong = 21_532u64;
```

### `TIOCCONS`

```rust
const TIOCCONS: crate::c_ulong = 21_533u64;
```

### `TIOCGSERIAL`

```rust
const TIOCGSERIAL: crate::c_ulong = 21_534u64;
```

### `TIOCSSERIAL`

```rust
const TIOCSSERIAL: crate::c_ulong = 21_535u64;
```

### `TIOCPKT`

```rust
const TIOCPKT: crate::c_ulong = 21_536u64;
```

### `FIONBIO`

```rust
const FIONBIO: crate::c_ulong = 21_537u64;
```

### `TIOCNOTTY`

```rust
const TIOCNOTTY: crate::c_ulong = 21_538u64;
```

### `TIOCSETD`

```rust
const TIOCSETD: crate::c_ulong = 21_539u64;
```

### `TIOCGETD`

```rust
const TIOCGETD: crate::c_ulong = 21_540u64;
```

### `TCSBRKP`

```rust
const TCSBRKP: crate::c_ulong = 21_541u64;
```

### `TIOCSBRK`

```rust
const TIOCSBRK: crate::c_ulong = 21_543u64;
```

### `TIOCCBRK`

```rust
const TIOCCBRK: crate::c_ulong = 21_544u64;
```

### `TIOCGSID`

```rust
const TIOCGSID: crate::c_ulong = 21_545u64;
```

### `TCGETS2`

```rust
const TCGETS2: crate::c_ulong = 2_150_388_778u64;
```

### `TCSETS2`

```rust
const TCSETS2: crate::c_ulong = 1_076_646_955u64;
```

### `TCSETSW2`

```rust
const TCSETSW2: crate::c_ulong = 1_076_646_956u64;
```

### `TCSETSF2`

```rust
const TCSETSF2: crate::c_ulong = 1_076_646_957u64;
```

### `TIOCGRS485`

```rust
const TIOCGRS485: crate::c_ulong = 21_550u64;
```

### `TIOCSRS485`

```rust
const TIOCSRS485: crate::c_ulong = 21_551u64;
```

### `TIOCGPTN`

```rust
const TIOCGPTN: crate::c_ulong = 2_147_767_344u64;
```

### `TIOCSPTLCK`

```rust
const TIOCSPTLCK: crate::c_ulong = 1_074_025_521u64;
```

### `TIOCGDEV`

```rust
const TIOCGDEV: crate::c_ulong = 2_147_767_346u64;
```

### `TCGETX`

```rust
const TCGETX: crate::c_ulong = 21_554u64;
```

### `TCSETX`

```rust
const TCSETX: crate::c_ulong = 21_555u64;
```

### `TCSETXF`

```rust
const TCSETXF: crate::c_ulong = 21_556u64;
```

### `TCSETXW`

```rust
const TCSETXW: crate::c_ulong = 21_557u64;
```

### `TIOCSIG`

```rust
const TIOCSIG: crate::c_ulong = 1_074_025_526u64;
```

### `TIOCVHANGUP`

```rust
const TIOCVHANGUP: crate::c_ulong = 21_559u64;
```

### `TIOCGPKT`

```rust
const TIOCGPKT: crate::c_ulong = 2_147_767_352u64;
```

### `TIOCGPTLCK`

```rust
const TIOCGPTLCK: crate::c_ulong = 2_147_767_353u64;
```

### `TIOCGEXCL`

```rust
const TIOCGEXCL: crate::c_ulong = 2_147_767_360u64;
```

### `TIOCGPTPEER`

```rust
const TIOCGPTPEER: crate::c_ulong = 21_569u64;
```

### `FIONCLEX`

```rust
const FIONCLEX: crate::c_ulong = 21_584u64;
```

### `FIOCLEX`

```rust
const FIOCLEX: crate::c_ulong = 21_585u64;
```

### `FIOASYNC`

```rust
const FIOASYNC: crate::c_ulong = 21_586u64;
```

### `TIOCSERCONFIG`

```rust
const TIOCSERCONFIG: crate::c_ulong = 21_587u64;
```

### `TIOCSERGWILD`

```rust
const TIOCSERGWILD: crate::c_ulong = 21_588u64;
```

### `TIOCSERSWILD`

```rust
const TIOCSERSWILD: crate::c_ulong = 21_589u64;
```

### `TIOCGLCKTRMIOS`

```rust
const TIOCGLCKTRMIOS: crate::c_ulong = 21_590u64;
```

### `TIOCSLCKTRMIOS`

```rust
const TIOCSLCKTRMIOS: crate::c_ulong = 21_591u64;
```

### `TIOCSERGSTRUCT`

```rust
const TIOCSERGSTRUCT: crate::c_ulong = 21_592u64;
```

### `TIOCSERGETLSR`

```rust
const TIOCSERGETLSR: crate::c_ulong = 21_593u64;
```

### `TIOCSERGETMULTI`

```rust
const TIOCSERGETMULTI: crate::c_ulong = 21_594u64;
```

### `TIOCSERSETMULTI`

```rust
const TIOCSERSETMULTI: crate::c_ulong = 21_595u64;
```

### `TIOCMIWAIT`

```rust
const TIOCMIWAIT: crate::c_ulong = 21_596u64;
```

### `TIOCGICOUNT`

```rust
const TIOCGICOUNT: crate::c_ulong = 21_597u64;
```

### `BLKIOMIN`

```rust
const BLKIOMIN: crate::c_ulong = 4_728u64;
```

### `BLKIOOPT`

```rust
const BLKIOOPT: crate::c_ulong = 4_729u64;
```

### `BLKSSZGET`

```rust
const BLKSSZGET: crate::c_ulong = 4_712u64;
```

### `BLKPBSZGET`

```rust
const BLKPBSZGET: crate::c_ulong = 4_731u64;
```

### `FIOQSIZE`

```rust
const FIOQSIZE: crate::c_ulong = 21_600u64;
```

### `TIOCM_LE`

```rust
const TIOCM_LE: crate::c_int = 1i32;
```

### `TIOCM_DTR`

```rust
const TIOCM_DTR: crate::c_int = 2i32;
```

### `TIOCM_RTS`

```rust
const TIOCM_RTS: crate::c_int = 4i32;
```

### `TIOCM_ST`

```rust
const TIOCM_ST: crate::c_int = 8i32;
```

### `TIOCM_SR`

```rust
const TIOCM_SR: crate::c_int = 16i32;
```

### `TIOCM_CTS`

```rust
const TIOCM_CTS: crate::c_int = 32i32;
```

### `TIOCM_CAR`

```rust
const TIOCM_CAR: crate::c_int = 64i32;
```

### `TIOCM_CD`

```rust
const TIOCM_CD: crate::c_int = 64i32;
```

### `TIOCM_RNG`

```rust
const TIOCM_RNG: crate::c_int = 128i32;
```

### `TIOCM_RI`

```rust
const TIOCM_RI: crate::c_int = 128i32;
```

### `TIOCM_DSR`

```rust
const TIOCM_DSR: crate::c_int = 256i32;
```

### `BOTHER`

```rust
const BOTHER: crate::speed_t = 4_096u32;
```

### `IBSHIFT`

```rust
const IBSHIFT: crate::tcflag_t = 16u32;
```

### `IUCLC`

```rust
const IUCLC: crate::tcflag_t = 512u32;
```

### `RLIMIT_CPU`

```rust
const RLIMIT_CPU: crate::__rlimit_resource_t = 0u32;
```

### `RLIMIT_FSIZE`

```rust
const RLIMIT_FSIZE: crate::__rlimit_resource_t = 1u32;
```

### `RLIMIT_DATA`

```rust
const RLIMIT_DATA: crate::__rlimit_resource_t = 2u32;
```

### `RLIMIT_STACK`

```rust
const RLIMIT_STACK: crate::__rlimit_resource_t = 3u32;
```

### `RLIMIT_CORE`

```rust
const RLIMIT_CORE: crate::__rlimit_resource_t = 4u32;
```

### `RLIMIT_RSS`

```rust
const RLIMIT_RSS: crate::__rlimit_resource_t = 5u32;
```

### `RLIMIT_NPROC`

```rust
const RLIMIT_NPROC: crate::__rlimit_resource_t = 6u32;
```

### `RLIMIT_NOFILE`

```rust
const RLIMIT_NOFILE: crate::__rlimit_resource_t = 7u32;
```

### `RLIMIT_MEMLOCK`

```rust
const RLIMIT_MEMLOCK: crate::__rlimit_resource_t = 8u32;
```

### `RLIMIT_AS`

```rust
const RLIMIT_AS: crate::__rlimit_resource_t = 9u32;
```

### `RLIMIT_LOCKS`

```rust
const RLIMIT_LOCKS: crate::__rlimit_resource_t = 10u32;
```

### `RLIMIT_SIGPENDING`

```rust
const RLIMIT_SIGPENDING: crate::__rlimit_resource_t = 11u32;
```

### `RLIMIT_MSGQUEUE`

```rust
const RLIMIT_MSGQUEUE: crate::__rlimit_resource_t = 12u32;
```

### `RLIMIT_NICE`

```rust
const RLIMIT_NICE: crate::__rlimit_resource_t = 13u32;
```

### `RLIMIT_RTPRIO`

```rust
const RLIMIT_RTPRIO: crate::__rlimit_resource_t = 14u32;
```

### `RLIMIT_RTTIME`

```rust
const RLIMIT_RTTIME: crate::__rlimit_resource_t = 15u32;
```

### `RLIMIT_NLIMITS`

```rust
const RLIMIT_NLIMITS: crate::__rlimit_resource_t = 16u32;
```

### `RLIM_NLIMITS`

```rust
const RLIM_NLIMITS: crate::__rlimit_resource_t = 16u32;
```

### `RLIM_INFINITY`

```rust
const RLIM_INFINITY: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

