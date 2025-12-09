*[linux_raw_sys](../index.md) / [ioctl](index.md)*

---

# Module `ioctl`

## Contents

- [Constants](#constants)
  - [`FIONREAD`](#fionread)
  - [`FIONBIO`](#fionbio)
  - [`FIOCLEX`](#fioclex)
  - [`FIONCLEX`](#fionclex)
  - [`FIOASYNC`](#fioasync)
  - [`FIOQSIZE`](#fioqsize)
  - [`TCXONC`](#tcxonc)
  - [`TCFLSH`](#tcflsh)
  - [`TIOCSCTTY`](#tiocsctty)
  - [`TIOCSPGRP`](#tiocspgrp)
  - [`TIOCOUTQ`](#tiocoutq)
  - [`TIOCSTI`](#tiocsti)
  - [`TIOCSWINSZ`](#tiocswinsz)
  - [`TIOCMGET`](#tiocmget)
  - [`TIOCMBIS`](#tiocmbis)
  - [`TIOCMBIC`](#tiocmbic)
  - [`TIOCMSET`](#tiocmset)
  - [`TIOCSSOFTCAR`](#tiocssoftcar)
  - [`TIOCLINUX`](#tioclinux)
  - [`TIOCCONS`](#tioccons)
  - [`TIOCSSERIAL`](#tiocsserial)
  - [`TIOCPKT`](#tiocpkt)
  - [`TIOCNOTTY`](#tiocnotty)
  - [`TIOCSETD`](#tiocsetd)
  - [`TIOCSBRK`](#tiocsbrk)
  - [`TIOCCBRK`](#tioccbrk)
  - [`TIOCSRS485`](#tiocsrs485)
  - [`TIOCSPTLCK`](#tiocsptlck)
  - [`TIOCSIG`](#tiocsig)
  - [`TIOCVHANGUP`](#tiocvhangup)
  - [`TIOCSERCONFIG`](#tiocserconfig)
  - [`TIOCSERGWILD`](#tiocsergwild)
  - [`TIOCSERSWILD`](#tiocserswild)
  - [`TIOCSLCKTRMIOS`](#tiocslcktrmios)
  - [`TIOCSERGSTRUCT`](#tiocsergstruct)
  - [`TIOCSERGETLSR`](#tiocsergetlsr)
  - [`TIOCSERGETMULTI`](#tiocsergetmulti)
  - [`TIOCSERSETMULTI`](#tiocsersetmulti)
  - [`TIOCMIWAIT`](#tiocmiwait)
  - [`TCGETS`](#tcgets)
  - [`TCGETA`](#tcgeta)
  - [`TCSBRK`](#tcsbrk)
  - [`TCSBRKP`](#tcsbrkp)
  - [`TCSETA`](#tcseta)
  - [`TCSETAF`](#tcsetaf)
  - [`TCSETAW`](#tcsetaw)
  - [`TIOCEXCL`](#tiocexcl)
  - [`TIOCNXCL`](#tiocnxcl)
  - [`TIOCGDEV`](#tiocgdev)
  - [`TIOCGEXCL`](#tiocgexcl)
  - [`TIOCGICOUNT`](#tiocgicount)
  - [`TIOCGLCKTRMIOS`](#tiocglcktrmios)
  - [`TIOCGPGRP`](#tiocgpgrp)
  - [`TIOCGPKT`](#tiocgpkt)
  - [`TIOCGPTLCK`](#tiocgptlck)
  - [`TIOCGPTN`](#tiocgptn)
  - [`TIOCGPTPEER`](#tiocgptpeer)
  - [`TIOCGRS485`](#tiocgrs485)
  - [`TIOCGSERIAL`](#tiocgserial)
  - [`TIOCGSID`](#tiocgsid)
  - [`TIOCGSOFTCAR`](#tiocgsoftcar)
  - [`TIOCGWINSZ`](#tiocgwinsz)
  - [`TCGETS2`](#tcgets2)
  - [`TCGETX`](#tcgetx)
  - [`TCSETS`](#tcsets)
  - [`TCSETS2`](#tcsets2)
  - [`TCSETSF`](#tcsetsf)
  - [`TCSETSF2`](#tcsetsf2)
  - [`TCSETSW`](#tcsetsw)
  - [`TCSETSW2`](#tcsetsw2)
  - [`TCSETX`](#tcsetx)
  - [`TCSETXF`](#tcsetxf)
  - [`TCSETXW`](#tcsetxw)
  - [`TIOCGETD`](#tiocgetd)
  - [`MTIOCGET`](#mtiocget)
  - [`BLKSSZGET`](#blksszget)
  - [`BLKPBSZGET`](#blkpbszget)
  - [`BLKROSET`](#blkroset)
  - [`BLKROGET`](#blkroget)
  - [`BLKRRPART`](#blkrrpart)
  - [`BLKGETSIZE`](#blkgetsize)
  - [`BLKFLSBUF`](#blkflsbuf)
  - [`BLKRASET`](#blkraset)
  - [`BLKRAGET`](#blkraget)
  - [`BLKFRASET`](#blkfraset)
  - [`BLKFRAGET`](#blkfraget)
  - [`BLKSECTSET`](#blksectset)
  - [`BLKSECTGET`](#blksectget)
  - [`BLKPG`](#blkpg)
  - [`BLKBSZGET`](#blkbszget)
  - [`BLKBSZSET`](#blkbszset)
  - [`BLKGETSIZE64`](#blkgetsize64)
  - [`BLKTRACESETUP`](#blktracesetup)
  - [`BLKTRACESTART`](#blktracestart)
  - [`BLKTRACESTOP`](#blktracestop)
  - [`BLKTRACETEARDOWN`](#blktraceteardown)
  - [`BLKDISCARD`](#blkdiscard)
  - [`BLKIOMIN`](#blkiomin)
  - [`BLKIOOPT`](#blkioopt)
  - [`BLKALIGNOFF`](#blkalignoff)
  - [`BLKDISCARDZEROES`](#blkdiscardzeroes)
  - [`BLKSECDISCARD`](#blksecdiscard)
  - [`BLKROTATIONAL`](#blkrotational)
  - [`BLKZEROOUT`](#blkzeroout)
  - [`FIEMAP_MAX_OFFSET`](#fiemap_max_offset)
  - [`FIEMAP_FLAG_SYNC`](#fiemap_flag_sync)
  - [`FIEMAP_FLAG_XATTR`](#fiemap_flag_xattr)
  - [`FIEMAP_FLAG_CACHE`](#fiemap_flag_cache)
  - [`FIEMAP_FLAGS_COMPAT`](#fiemap_flags_compat)
  - [`FIEMAP_EXTENT_LAST`](#fiemap_extent_last)
  - [`FIEMAP_EXTENT_UNKNOWN`](#fiemap_extent_unknown)
  - [`FIEMAP_EXTENT_DELALLOC`](#fiemap_extent_delalloc)
  - [`FIEMAP_EXTENT_ENCODED`](#fiemap_extent_encoded)
  - [`FIEMAP_EXTENT_DATA_ENCRYPTED`](#fiemap_extent_data_encrypted)
  - [`FIEMAP_EXTENT_NOT_ALIGNED`](#fiemap_extent_not_aligned)
  - [`FIEMAP_EXTENT_DATA_INLINE`](#fiemap_extent_data_inline)
  - [`FIEMAP_EXTENT_DATA_TAIL`](#fiemap_extent_data_tail)
  - [`FIEMAP_EXTENT_UNWRITTEN`](#fiemap_extent_unwritten)
  - [`FIEMAP_EXTENT_MERGED`](#fiemap_extent_merged)
  - [`FIEMAP_EXTENT_SHARED`](#fiemap_extent_shared)
  - [`UFFDIO_REGISTER`](#uffdio_register)
  - [`UFFDIO_UNREGISTER`](#uffdio_unregister)
  - [`UFFDIO_WAKE`](#uffdio_wake)
  - [`UFFDIO_COPY`](#uffdio_copy)
  - [`UFFDIO_ZEROPAGE`](#uffdio_zeropage)
  - [`UFFDIO_WRITEPROTECT`](#uffdio_writeprotect)
  - [`UFFDIO_API`](#uffdio_api)
  - [`NS_GET_USERNS`](#ns_get_userns)
  - [`NS_GET_PARENT`](#ns_get_parent)
  - [`NS_GET_NSTYPE`](#ns_get_nstype)
  - [`KDGETLED`](#kdgetled)
  - [`KDSETLED`](#kdsetled)
  - [`KDGKBLED`](#kdgkbled)
  - [`KDSKBLED`](#kdskbled)
  - [`KDGKBTYPE`](#kdgkbtype)
  - [`KDADDIO`](#kdaddio)
  - [`KDDELIO`](#kddelio)
  - [`KDENABIO`](#kdenabio)
  - [`KDDISABIO`](#kddisabio)
  - [`KDSETMODE`](#kdsetmode)
  - [`KDGETMODE`](#kdgetmode)
  - [`KDMKTONE`](#kdmktone)
  - [`KIOCSOUND`](#kiocsound)
  - [`GIO_CMAP`](#gio_cmap)
  - [`PIO_CMAP`](#pio_cmap)
  - [`GIO_FONT`](#gio_font)
  - [`GIO_FONTX`](#gio_fontx)
  - [`PIO_FONT`](#pio_font)
  - [`PIO_FONTX`](#pio_fontx)
  - [`PIO_FONTRESET`](#pio_fontreset)
  - [`GIO_SCRNMAP`](#gio_scrnmap)
  - [`GIO_UNISCRNMAP`](#gio_uniscrnmap)
  - [`PIO_SCRNMAP`](#pio_scrnmap)
  - [`PIO_UNISCRNMAP`](#pio_uniscrnmap)
  - [`GIO_UNIMAP`](#gio_unimap)
  - [`PIO_UNIMAP`](#pio_unimap)
  - [`PIO_UNIMAPCLR`](#pio_unimapclr)
  - [`KDGKBMODE`](#kdgkbmode)
  - [`KDSKBMODE`](#kdskbmode)
  - [`KDGKBMETA`](#kdgkbmeta)
  - [`KDSKBMETA`](#kdskbmeta)
  - [`KDGKBENT`](#kdgkbent)
  - [`KDSKBENT`](#kdskbent)
  - [`KDGKBSENT`](#kdgkbsent)
  - [`KDSKBSENT`](#kdskbsent)
  - [`KDGKBDIACR`](#kdgkbdiacr)
  - [`KDGETKEYCODE`](#kdgetkeycode)
  - [`KDSETKEYCODE`](#kdsetkeycode)
  - [`KDSIGACCEPT`](#kdsigaccept)
  - [`VT_OPENQRY`](#vt_openqry)
  - [`VT_GETMODE`](#vt_getmode)
  - [`VT_SETMODE`](#vt_setmode)
  - [`VT_GETSTATE`](#vt_getstate)
  - [`VT_RELDISP`](#vt_reldisp)
  - [`VT_ACTIVATE`](#vt_activate)
  - [`VT_WAITACTIVE`](#vt_waitactive)
  - [`VT_DISALLOCATE`](#vt_disallocate)
  - [`VT_RESIZE`](#vt_resize)
  - [`VT_RESIZEX`](#vt_resizex)
  - [`FIOSETOWN`](#fiosetown)
  - [`SIOCSPGRP`](#siocspgrp)
  - [`FIOGETOWN`](#fiogetown)
  - [`SIOCGPGRP`](#siocgpgrp)
  - [`SIOCATMARK`](#siocatmark)
  - [`SIOCGSTAMP`](#siocgstamp)
  - [`TIOCINQ`](#tiocinq)
  - [`SIOCADDRT`](#siocaddrt)
  - [`SIOCDELRT`](#siocdelrt)
  - [`SIOCGIFNAME`](#siocgifname)
  - [`SIOCSIFLINK`](#siocsiflink)
  - [`SIOCGIFCONF`](#siocgifconf)
  - [`SIOCGIFFLAGS`](#siocgifflags)
  - [`SIOCSIFFLAGS`](#siocsifflags)
  - [`SIOCGIFADDR`](#siocgifaddr)
  - [`SIOCSIFADDR`](#siocsifaddr)
  - [`SIOCGIFDSTADDR`](#siocgifdstaddr)
  - [`SIOCSIFDSTADDR`](#siocsifdstaddr)
  - [`SIOCGIFBRDADDR`](#siocgifbrdaddr)
  - [`SIOCSIFBRDADDR`](#siocsifbrdaddr)
  - [`SIOCGIFNETMASK`](#siocgifnetmask)
  - [`SIOCSIFNETMASK`](#siocsifnetmask)
  - [`SIOCGIFMETRIC`](#siocgifmetric)
  - [`SIOCSIFMETRIC`](#siocsifmetric)
  - [`SIOCGIFMEM`](#siocgifmem)
  - [`SIOCSIFMEM`](#siocsifmem)
  - [`SIOCGIFMTU`](#siocgifmtu)
  - [`SIOCSIFMTU`](#siocsifmtu)
  - [`SIOCSIFHWADDR`](#siocsifhwaddr)
  - [`SIOCGIFENCAP`](#siocgifencap)
  - [`SIOCSIFENCAP`](#siocsifencap)
  - [`SIOCGIFHWADDR`](#siocgifhwaddr)
  - [`SIOCGIFSLAVE`](#siocgifslave)
  - [`SIOCSIFSLAVE`](#siocsifslave)
  - [`SIOCADDMULTI`](#siocaddmulti)
  - [`SIOCDELMULTI`](#siocdelmulti)
  - [`SIOCDARP`](#siocdarp)
  - [`SIOCGARP`](#siocgarp)
  - [`SIOCSARP`](#siocsarp)
  - [`SIOCDRARP`](#siocdrarp)
  - [`SIOCGRARP`](#siocgrarp)
  - [`SIOCSRARP`](#siocsrarp)
  - [`SIOCGIFMAP`](#siocgifmap)
  - [`SIOCSIFMAP`](#siocsifmap)
  - [`SIOCRTMSG`](#siocrtmsg)
  - [`SIOCSIFNAME`](#siocsifname)
  - [`SIOCGIFINDEX`](#siocgifindex)
  - [`SIOGIFINDEX`](#siogifindex)
  - [`SIOCSIFPFLAGS`](#siocsifpflags)
  - [`SIOCGIFPFLAGS`](#siocgifpflags)
  - [`SIOCDIFADDR`](#siocdifaddr)
  - [`SIOCSIFHWBROADCAST`](#siocsifhwbroadcast)
  - [`SIOCGIFCOUNT`](#siocgifcount)
  - [`SIOCGIFBR`](#siocgifbr)
  - [`SIOCSIFBR`](#siocsifbr)
  - [`SIOCGIFTXQLEN`](#siocgiftxqlen)
  - [`SIOCSIFTXQLEN`](#siocsiftxqlen)
  - [`SIOCADDDLCI`](#siocadddlci)
  - [`SIOCDELDLCI`](#siocdeldlci)
  - [`SIOCDEVPRIVATE`](#siocdevprivate)
  - [`SIOCPROTOPRIVATE`](#siocprotoprivate)
  - [`FIBMAP`](#fibmap)
  - [`FIGETBSZ`](#figetbsz)
  - [`FIFREEZE`](#fifreeze)
  - [`FITHAW`](#fithaw)
  - [`FITRIM`](#fitrim)
  - [`FICLONE`](#ficlone)
  - [`FICLONERANGE`](#ficlonerange)
  - [`FIDEDUPERANGE`](#fideduperange)
  - [`FS_IOC_GETFLAGS`](#fs_ioc_getflags)
  - [`FS_IOC_SETFLAGS`](#fs_ioc_setflags)
  - [`FS_IOC_GETVERSION`](#fs_ioc_getversion)
  - [`FS_IOC_SETVERSION`](#fs_ioc_setversion)
  - [`FS_IOC_FIEMAP`](#fs_ioc_fiemap)
  - [`FS_IOC32_GETFLAGS`](#fs_ioc32_getflags)
  - [`FS_IOC32_SETFLAGS`](#fs_ioc32_setflags)
  - [`FS_IOC32_GETVERSION`](#fs_ioc32_getversion)
  - [`FS_IOC32_SETVERSION`](#fs_ioc32_setversion)
  - [`FS_IOC_FSGETXATTR`](#fs_ioc_fsgetxattr)
  - [`FS_IOC_FSSETXATTR`](#fs_ioc_fssetxattr)
  - [`FS_IOC_GETFSLABEL`](#fs_ioc_getfslabel)
  - [`FS_IOC_SETFSLABEL`](#fs_ioc_setfslabel)
  - [`EXT4_IOC_GETVERSION`](#ext4_ioc_getversion)
  - [`EXT4_IOC_SETVERSION`](#ext4_ioc_setversion)
  - [`EXT4_IOC_GETVERSION_OLD`](#ext4_ioc_getversion_old)
  - [`EXT4_IOC_SETVERSION_OLD`](#ext4_ioc_setversion_old)
  - [`EXT4_IOC_GETRSVSZ`](#ext4_ioc_getrsvsz)
  - [`EXT4_IOC_SETRSVSZ`](#ext4_ioc_setrsvsz)
  - [`EXT4_IOC_GROUP_EXTEND`](#ext4_ioc_group_extend)
  - [`EXT4_IOC_MIGRATE`](#ext4_ioc_migrate)
  - [`EXT4_IOC_ALLOC_DA_BLKS`](#ext4_ioc_alloc_da_blks)
  - [`EXT4_IOC_RESIZE_FS`](#ext4_ioc_resize_fs)
  - [`EXT4_IOC_SWAP_BOOT`](#ext4_ioc_swap_boot)
  - [`EXT4_IOC_PRECACHE_EXTENTS`](#ext4_ioc_precache_extents)
  - [`EXT4_IOC_CLEAR_ES_CACHE`](#ext4_ioc_clear_es_cache)
  - [`EXT4_IOC_GETSTATE`](#ext4_ioc_getstate)
  - [`EXT4_IOC_GET_ES_CACHE`](#ext4_ioc_get_es_cache)
  - [`EXT4_IOC_CHECKPOINT`](#ext4_ioc_checkpoint)
  - [`EXT4_IOC_SHUTDOWN`](#ext4_ioc_shutdown)
  - [`EXT4_IOC32_GETVERSION`](#ext4_ioc32_getversion)
  - [`EXT4_IOC32_SETVERSION`](#ext4_ioc32_setversion)
  - [`EXT4_IOC32_GETRSVSZ`](#ext4_ioc32_getrsvsz)
  - [`EXT4_IOC32_SETRSVSZ`](#ext4_ioc32_setrsvsz)
  - [`EXT4_IOC32_GROUP_EXTEND`](#ext4_ioc32_group_extend)
  - [`EXT4_IOC32_GETVERSION_OLD`](#ext4_ioc32_getversion_old)
  - [`EXT4_IOC32_SETVERSION_OLD`](#ext4_ioc32_setversion_old)
  - [`VIDIOC_SUBDEV_QUERYSTD`](#vidioc_subdev_querystd)
  - [`AUTOFS_DEV_IOCTL_CLOSEMOUNT`](#autofs_dev_ioctl_closemount)
  - [`LIRC_SET_SEND_CARRIER`](#lirc_set_send_carrier)
  - [`AUTOFS_IOC_PROTOSUBVER`](#autofs_ioc_protosubver)
  - [`PTP_SYS_OFFSET_PRECISE`](#ptp_sys_offset_precise)
  - [`FSI_SCOM_WRITE`](#fsi_scom_write)
  - [`ATM_GETCIRANGE`](#atm_getcirange)
  - [`DMA_BUF_SET_NAME_B`](#dma_buf_set_name_b)
  - [`RIO_CM_EP_GET_LIST_SIZE`](#rio_cm_ep_get_list_size)
  - [`TUNSETPERSIST`](#tunsetpersist)
  - [`FS_IOC_GET_ENCRYPTION_POLICY`](#fs_ioc_get_encryption_policy)
  - [`CEC_RECEIVE`](#cec_receive)
  - [`MGSL_IOCGPARAMS`](#mgsl_iocgparams)
  - [`ENI_SETMULT`](#eni_setmult)
  - [`RIO_GET_EVENT_MASK`](#rio_get_event_mask)
  - [`LIRC_GET_MAX_TIMEOUT`](#lirc_get_max_timeout)
  - [`USBDEVFS_CLAIMINTERFACE`](#usbdevfs_claiminterface)
  - [`CHIOMOVE`](#chiomove)
  - [`SONYPI_IOCGBATFLAGS`](#sonypi_iocgbatflags)
  - [`BTRFS_IOC_SYNC`](#btrfs_ioc_sync)
  - [`VIDIOC_TRY_FMT`](#vidioc_try_fmt)
  - [`LIRC_SET_REC_MODE`](#lirc_set_rec_mode)
  - [`VIDIOC_DQEVENT`](#vidioc_dqevent)
  - [`RPMSG_DESTROY_EPT_IOCTL`](#rpmsg_destroy_ept_ioctl)
  - [`UVCIOC_CTRL_MAP`](#uvcioc_ctrl_map)
  - [`VHOST_SET_BACKEND_FEATURES`](#vhost_set_backend_features)
  - [`VHOST_VSOCK_SET_GUEST_CID`](#vhost_vsock_set_guest_cid)
  - [`UI_SET_KEYBIT`](#ui_set_keybit)
  - [`LIRC_SET_REC_TIMEOUT`](#lirc_set_rec_timeout)
  - [`FS_IOC_GET_ENCRYPTION_KEY_STATUS`](#fs_ioc_get_encryption_key_status)
  - [`BTRFS_IOC_TREE_SEARCH_V2`](#btrfs_ioc_tree_search_v2)
  - [`VHOST_SET_VRING_BASE`](#vhost_set_vring_base)
  - [`RIO_ENABLE_DOORBELL_RANGE`](#rio_enable_doorbell_range)
  - [`VIDIOC_TRY_EXT_CTRLS`](#vidioc_try_ext_ctrls)
  - [`LIRC_GET_REC_MODE`](#lirc_get_rec_mode)
  - [`PPGETTIME`](#ppgettime)
  - [`BTRFS_IOC_RM_DEV`](#btrfs_ioc_rm_dev)
  - [`ATM_SETBACKEND`](#atm_setbackend)
  - [`FSL_HV_IOCTL_PARTITION_START`](#fsl_hv_ioctl_partition_start)
  - [`FBIO_WAITEVENT`](#fbio_waitevent)
  - [`SWITCHTEC_IOCTL_PORT_TO_PFF`](#switchtec_ioctl_port_to_pff)
  - [`NVME_IOCTL_IO_CMD`](#nvme_ioctl_io_cmd)
  - [`IPMICTL_RECEIVE_MSG_TRUNC`](#ipmictl_receive_msg_trunc)
  - [`FDTWADDLE`](#fdtwaddle)
  - [`NVME_IOCTL_SUBMIT_IO`](#nvme_ioctl_submit_io)
  - [`NILFS_IOCTL_SYNC`](#nilfs_ioctl_sync)
  - [`VIDIOC_SUBDEV_S_DV_TIMINGS`](#vidioc_subdev_s_dv_timings)
  - [`ASPEED_LPC_CTRL_IOCTL_GET_SIZE`](#aspeed_lpc_ctrl_ioctl_get_size)
  - [`DM_DEV_STATUS`](#dm_dev_status)
  - [`TEE_IOC_CLOSE_SESSION`](#tee_ioc_close_session)
  - [`NS_GETPSTAT`](#ns_getpstat)
  - [`UI_SET_PROPBIT`](#ui_set_propbit)
  - [`TUNSETFILTEREBPF`](#tunsetfilterebpf)
  - [`RIO_MPORT_MAINT_COMPTAG_SET`](#rio_mport_maint_comptag_set)
  - [`AUTOFS_DEV_IOCTL_VERSION`](#autofs_dev_ioctl_version)
  - [`WDIOC_SETOPTIONS`](#wdioc_setoptions)
  - [`VHOST_SCSI_SET_ENDPOINT`](#vhost_scsi_set_endpoint)
  - [`MGSL_IOCGTXIDLE`](#mgsl_iocgtxidle)
  - [`ATM_ADDLECSADDR`](#atm_addlecsaddr)
  - [`FSL_HV_IOCTL_GETPROP`](#fsl_hv_ioctl_getprop)
  - [`FDGETPRM`](#fdgetprm)
  - [`HIDIOCAPPLICATION`](#hidiocapplication)
  - [`ENI_MEMDUMP`](#eni_memdump)
  - [`PTP_SYS_OFFSET2`](#ptp_sys_offset2)
  - [`VIDIOC_SUBDEV_G_DV_TIMINGS`](#vidioc_subdev_g_dv_timings)
  - [`DMA_BUF_SET_NAME_A`](#dma_buf_set_name_a)
  - [`PTP_PIN_GETFUNC`](#ptp_pin_getfunc)
  - [`PTP_SYS_OFFSET_EXTENDED`](#ptp_sys_offset_extended)
  - [`DFL_FPGA_PORT_UINT_SET_IRQ`](#dfl_fpga_port_uint_set_irq)
  - [`RTC_EPOCH_READ`](#rtc_epoch_read)
  - [`VIDIOC_SUBDEV_S_SELECTION`](#vidioc_subdev_s_selection)
  - [`VIDIOC_QUERY_EXT_CTRL`](#vidioc_query_ext_ctrl)
  - [`ATM_GETLECSADDR`](#atm_getlecsaddr)
  - [`FSL_HV_IOCTL_PARTITION_STOP`](#fsl_hv_ioctl_partition_stop)
  - [`SONET_GETDIAG`](#sonet_getdiag)
  - [`ATMMPC_DATA`](#atmmpc_data)
  - [`IPMICTL_UNREGISTER_FOR_CMD_CHANS`](#ipmictl_unregister_for_cmd_chans)
  - [`HIDIOCGCOLLECTIONINDEX`](#hidiocgcollectionindex)
  - [`RPMSG_CREATE_EPT_IOCTL`](#rpmsg_create_ept_ioctl)
  - [`GPIOHANDLE_GET_LINE_VALUES_IOCTL`](#gpiohandle_get_line_values_ioctl)
  - [`UI_DEV_SETUP`](#ui_dev_setup)
  - [`ISST_IF_IO_CMD`](#isst_if_io_cmd)
  - [`RIO_MPORT_MAINT_READ_REMOTE`](#rio_mport_maint_read_remote)
  - [`VIDIOC_OMAP3ISP_HIST_CFG`](#vidioc_omap3isp_hist_cfg)
  - [`BLKGETNRZONES`](#blkgetnrzones)
  - [`VIDIOC_G_MODULATOR`](#vidioc_g_modulator)
  - [`VBG_IOCTL_WRITE_CORE_DUMP`](#vbg_ioctl_write_core_dump)
  - [`USBDEVFS_SETINTERFACE`](#usbdevfs_setinterface)
  - [`PPPIOCGCHAN`](#pppiocgchan)
  - [`EVIOCGVERSION`](#eviocgversion)
  - [`VHOST_NET_SET_BACKEND`](#vhost_net_set_backend)
  - [`USBDEVFS_REAPURBNDELAY`](#usbdevfs_reapurbndelay)
  - [`RNDZAPENTCNT`](#rndzapentcnt)
  - [`VIDIOC_G_PARM`](#vidioc_g_parm)
  - [`TUNGETDEVNETNS`](#tungetdevnetns)
  - [`LIRC_SET_MEASURE_CARRIER_MODE`](#lirc_set_measure_carrier_mode)
  - [`VHOST_SET_VRING_ERR`](#vhost_set_vring_err)
  - [`VDUSE_VQ_SETUP`](#vduse_vq_setup)
  - [`AUTOFS_IOC_SETTIMEOUT`](#autofs_ioc_settimeout)
  - [`VIDIOC_S_FREQUENCY`](#vidioc_s_frequency)
  - [`F2FS_IOC_SEC_TRIM_FILE`](#f2fs_ioc_sec_trim_file)
  - [`FS_IOC_REMOVE_ENCRYPTION_KEY`](#fs_ioc_remove_encryption_key)
  - [`WDIOC_GETPRETIMEOUT`](#wdioc_getpretimeout)
  - [`USBDEVFS_DROP_PRIVILEGES`](#usbdevfs_drop_privileges)
  - [`BTRFS_IOC_SNAP_CREATE_V2`](#btrfs_ioc_snap_create_v2)
  - [`VHOST_VSOCK_SET_RUNNING`](#vhost_vsock_set_running)
  - [`STP_SET_OPTIONS`](#stp_set_options)
  - [`FBIO_RADEON_GET_MIRROR`](#fbio_radeon_get_mirror)
  - [`IVTVFB_IOC_DMA_FRAME`](#ivtvfb_ioc_dma_frame)
  - [`IPMICTL_SEND_COMMAND`](#ipmictl_send_command)
  - [`VIDIOC_G_ENC_INDEX`](#vidioc_g_enc_index)
  - [`DFL_FPGA_FME_PORT_PR`](#dfl_fpga_fme_port_pr)
  - [`CHIOSVOLTAG`](#chiosvoltag)
  - [`ATM_SETESIF`](#atm_setesif)
  - [`FW_CDEV_IOC_SEND_RESPONSE`](#fw_cdev_ioc_send_response)
  - [`PMU_IOC_GET_MODEL`](#pmu_ioc_get_model)
  - [`JSIOCGBTNMAP`](#jsiocgbtnmap)
  - [`USBDEVFS_HUB_PORTINFO`](#usbdevfs_hub_portinfo)
  - [`VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS`](#vbg_ioctl_interrupt_all_wait_for_events)
  - [`FDCLRPRM`](#fdclrprm)
  - [`BTRFS_IOC_SCRUB`](#btrfs_ioc_scrub)
  - [`USBDEVFS_DISCONNECT`](#usbdevfs_disconnect)
  - [`TUNSETVNETBE`](#tunsetvnetbe)
  - [`ATMTCP_REMOVE`](#atmtcp_remove)
  - [`VHOST_VDPA_GET_CONFIG`](#vhost_vdpa_get_config)
  - [`PPPIOCGNPMODE`](#pppiocgnpmode)
  - [`FDGETDRVPRM`](#fdgetdrvprm)
  - [`TUNSETVNETLE`](#tunsetvnetle)
  - [`PHN_SETREG`](#phn_setreg)
  - [`PPPIOCDETACH`](#pppiocdetach)
  - [`MMTIMER_GETRES`](#mmtimer_getres)
  - [`VIDIOC_SUBDEV_ENUMSTD`](#vidioc_subdev_enumstd)
  - [`PPGETFLAGS`](#ppgetflags)
  - [`VDUSE_DEV_GET_FEATURES`](#vduse_dev_get_features)
  - [`CAPI_MANUFACTURER_CMD`](#capi_manufacturer_cmd)
  - [`VIDIOC_G_TUNER`](#vidioc_g_tuner)
  - [`DM_TABLE_STATUS`](#dm_table_status)
  - [`DM_DEV_ARM_POLL`](#dm_dev_arm_poll)
  - [`NE_CREATE_VM`](#ne_create_vm)
  - [`MEDIA_IOC_ENUM_LINKS`](#media_ioc_enum_links)
  - [`F2FS_IOC_PRECACHE_EXTENTS`](#f2fs_ioc_precache_extents)
  - [`DFL_FPGA_PORT_DMA_MAP`](#dfl_fpga_port_dma_map)
  - [`MGSL_IOCGXCTRL`](#mgsl_iocgxctrl)
  - [`FW_CDEV_IOC_SEND_REQUEST`](#fw_cdev_ioc_send_request)
  - [`SONYPI_IOCGBLUE`](#sonypi_iocgblue)
  - [`F2FS_IOC_DECOMPRESS_FILE`](#f2fs_ioc_decompress_file)
  - [`I2OHTML`](#i2ohtml)
  - [`VFIO_GET_API_VERSION`](#vfio_get_api_version)
  - [`IDT77105_GETSTATZ`](#idt77105_getstatz)
  - [`I2OPARMSET`](#i2oparmset)
  - [`TEE_IOC_CANCEL`](#tee_ioc_cancel)
  - [`PTP_SYS_OFFSET_PRECISE2`](#ptp_sys_offset_precise2)
  - [`DFL_FPGA_PORT_RESET`](#dfl_fpga_port_reset)
  - [`PPPIOCGASYNCMAP`](#pppiocgasyncmap)
  - [`EVIOCGKEYCODE_V2`](#eviocgkeycode_v2)
  - [`DM_DEV_SET_GEOMETRY`](#dm_dev_set_geometry)
  - [`HIDIOCSUSAGE`](#hidiocsusage)
  - [`FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE`](#fw_cdev_ioc_deallocate_iso_resource_once)
  - [`PTP_EXTTS_REQUEST`](#ptp_extts_request)
  - [`SWITCHTEC_IOCTL_EVENT_CTL`](#switchtec_ioctl_event_ctl)
  - [`WDIOC_SETPRETIMEOUT`](#wdioc_setpretimeout)
  - [`VHOST_SCSI_CLEAR_ENDPOINT`](#vhost_scsi_clear_endpoint)
  - [`JSIOCGAXES`](#jsiocgaxes)
  - [`HIDIOCSFLAG`](#hidiocsflag)
  - [`PTP_PEROUT_REQUEST2`](#ptp_perout_request2)
  - [`PPWDATA`](#ppwdata)
  - [`PTP_CLOCK_GETCAPS`](#ptp_clock_getcaps)
  - [`FDGETMAXERRS`](#fdgetmaxerrs)
  - [`TUNSETQUEUE`](#tunsetqueue)
  - [`PTP_ENABLE_PPS`](#ptp_enable_pps)
  - [`SIOCSIFATMTCP`](#siocsifatmtcp)
  - [`CEC_ADAP_G_LOG_ADDRS`](#cec_adap_g_log_addrs)
  - [`ND_IOCTL_ARS_CAP`](#nd_ioctl_ars_cap)
  - [`NBD_SET_BLKSIZE`](#nbd_set_blksize)
  - [`NBD_SET_TIMEOUT`](#nbd_set_timeout)
  - [`VHOST_SCSI_GET_ABI_VERSION`](#vhost_scsi_get_abi_version)
  - [`RIO_UNMAP_INBOUND`](#rio_unmap_inbound)
  - [`ATM_QUERYLOOP`](#atm_queryloop)
  - [`DFL_FPGA_GET_API_VERSION`](#dfl_fpga_get_api_version)
  - [`USBDEVFS_WAIT_FOR_RESUME`](#usbdevfs_wait_for_resume)
  - [`FBIO_CURSOR`](#fbio_cursor)
  - [`RNDCLEARPOOL`](#rndclearpool)
  - [`VIDIOC_QUERYSTD`](#vidioc_querystd)
  - [`DMA_BUF_IOCTL_SYNC`](#dma_buf_ioctl_sync)
  - [`SCIF_RECV`](#scif_recv)
  - [`PTP_PIN_GETFUNC2`](#ptp_pin_getfunc2)
  - [`FW_CDEV_IOC_ALLOCATE`](#fw_cdev_ioc_allocate)
  - [`CEC_ADAP_G_CAPS`](#cec_adap_g_caps)
  - [`VIDIOC_G_FBUF`](#vidioc_g_fbuf)
  - [`PTP_ENABLE_PPS2`](#ptp_enable_pps2)
  - [`PCITEST_CLEAR_IRQ`](#pcitest_clear_irq)
  - [`IPMICTL_SET_GETS_EVENTS_CMD`](#ipmictl_set_gets_events_cmd)
  - [`BTRFS_IOC_DEVICES_READY`](#btrfs_ioc_devices_ready)
  - [`JSIOCGAXMAP`](#jsiocgaxmap)
  - [`FW_CDEV_IOC_GET_CYCLE_TIMER`](#fw_cdev_ioc_get_cycle_timer)
  - [`FW_CDEV_IOC_SET_ISO_CHANNELS`](#fw_cdev_ioc_set_iso_channels)
  - [`RTC_WIE_OFF`](#rtc_wie_off)
  - [`PPGETMODE`](#ppgetmode)
  - [`VIDIOC_DBG_G_REGISTER`](#vidioc_dbg_g_register)
  - [`PTP_SYS_OFFSET`](#ptp_sys_offset)
  - [`BTRFS_IOC_SPACE_INFO`](#btrfs_ioc_space_info)
  - [`VIDIOC_SUBDEV_ENUM_FRAME_SIZE`](#vidioc_subdev_enum_frame_size)
  - [`ND_IOCTL_VENDOR`](#nd_ioctl_vendor)
  - [`SCIF_VREADFROM`](#scif_vreadfrom)
  - [`BTRFS_IOC_TRANS_START`](#btrfs_ioc_trans_start)
  - [`INOTIFY_IOC_SETNEXTWD`](#inotify_ioc_setnextwd)
  - [`SNAPSHOT_GET_IMAGE_SIZE`](#snapshot_get_image_size)
  - [`TUNDETACHFILTER`](#tundetachfilter)
  - [`ND_IOCTL_CLEAR_ERROR`](#nd_ioctl_clear_error)
  - [`IOC_PR_CLEAR`](#ioc_pr_clear)
  - [`SCIF_READFROM`](#scif_readfrom)
  - [`PPPIOCGDEBUG`](#pppiocgdebug)
  - [`BLKGETZONESZ`](#blkgetzonesz)
  - [`HIDIOCGUSAGES`](#hidiocgusages)
  - [`SONYPI_IOCGTEMP`](#sonypi_iocgtemp)
  - [`UI_SET_MSCBIT`](#ui_set_mscbit)
  - [`APM_IOC_SUSPEND`](#apm_ioc_suspend)
  - [`BTRFS_IOC_TREE_SEARCH`](#btrfs_ioc_tree_search)
  - [`RTC_PLL_GET`](#rtc_pll_get)
  - [`RIO_CM_EP_GET_LIST`](#rio_cm_ep_get_list)
  - [`USBDEVFS_DISCSIGNAL`](#usbdevfs_discsignal)
  - [`LIRC_GET_MIN_TIMEOUT`](#lirc_get_min_timeout)
  - [`SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY`](#switchtec_ioctl_event_summary_legacy)
  - [`DM_TARGET_MSG`](#dm_target_msg)
  - [`SONYPI_IOCGBAT1REM`](#sonypi_iocgbat1rem)
  - [`EVIOCSFF`](#eviocsff)
  - [`TUNSETGROUP`](#tunsetgroup)
  - [`EVIOCGKEYCODE`](#eviocgkeycode)
  - [`KCOV_REMOTE_ENABLE`](#kcov_remote_enable)
  - [`ND_IOCTL_GET_CONFIG_SIZE`](#nd_ioctl_get_config_size)
  - [`FDEJECT`](#fdeject)
  - [`TUNSETOFFLOAD`](#tunsetoffload)
  - [`PPPIOCCONNECT`](#pppiocconnect)
  - [`ATM_ADDADDR`](#atm_addaddr)
  - [`VDUSE_DEV_INJECT_CONFIG_IRQ`](#vduse_dev_inject_config_irq)
  - [`AUTOFS_DEV_IOCTL_ASKUMOUNT`](#autofs_dev_ioctl_askumount)
  - [`VHOST_VDPA_GET_STATUS`](#vhost_vdpa_get_status)
  - [`CCISS_PASSTHRU`](#cciss_passthru)
  - [`MGSL_IOCCLRMODCOUNT`](#mgsl_iocclrmodcount)
  - [`TEE_IOC_SUPPL_SEND`](#tee_ioc_suppl_send)
  - [`ATMARPD_CTRL`](#atmarpd_ctrl)
  - [`UI_ABS_SETUP`](#ui_abs_setup)
  - [`UI_DEV_DESTROY`](#ui_dev_destroy)
  - [`BTRFS_IOC_QUOTA_CTL`](#btrfs_ioc_quota_ctl)
  - [`RTC_AIE_ON`](#rtc_aie_on)
  - [`AUTOFS_IOC_EXPIRE`](#autofs_ioc_expire)
  - [`PPPIOCSDEBUG`](#pppiocsdebug)
  - [`GPIO_V2_LINE_SET_VALUES_IOCTL`](#gpio_v2_line_set_values_ioctl)
  - [`PPPIOCSMRU`](#pppiocsmru)
  - [`CCISS_DEREGDISK`](#cciss_deregdisk)
  - [`UI_DEV_CREATE`](#ui_dev_create)
  - [`FUSE_DEV_IOC_CLONE`](#fuse_dev_ioc_clone)
  - [`BTRFS_IOC_START_SYNC`](#btrfs_ioc_start_sync)
  - [`NILFS_IOCTL_DELETE_CHECKPOINT`](#nilfs_ioctl_delete_checkpoint)
  - [`SNAPSHOT_AVAIL_SWAP_SIZE`](#snapshot_avail_swap_size)
  - [`DM_TABLE_CLEAR`](#dm_table_clear)
  - [`CCISS_GETINTINFO`](#cciss_getintinfo)
  - [`PPPIOCSASYNCMAP`](#pppiocsasyncmap)
  - [`I2OEVTGET`](#i2oevtget)
  - [`NVME_IOCTL_RESET`](#nvme_ioctl_reset)
  - [`PPYIELD`](#ppyield)
  - [`NVME_IOCTL_IO64_CMD`](#nvme_ioctl_io64_cmd)
  - [`TUNSETCARRIER`](#tunsetcarrier)
  - [`DM_DEV_WAIT`](#dm_dev_wait)
  - [`RTC_WIE_ON`](#rtc_wie_on)
  - [`MEDIA_IOC_DEVICE_INFO`](#media_ioc_device_info)
  - [`RIO_CM_CHAN_CREATE`](#rio_cm_chan_create)
  - [`MGSL_IOCSPARAMS`](#mgsl_iocsparams)
  - [`RTC_SET_TIME`](#rtc_set_time)
  - [`VHOST_RESET_OWNER`](#vhost_reset_owner)
  - [`IOC_OPAL_PSID_REVERT_TPR`](#ioc_opal_psid_revert_tpr)
  - [`AUTOFS_DEV_IOCTL_OPENMOUNT`](#autofs_dev_ioctl_openmount)
  - [`UDF_GETEABLOCK`](#udf_geteablock)
  - [`VFIO_IOMMU_MAP_DMA`](#vfio_iommu_map_dma)
  - [`VIDIOC_SUBSCRIBE_EVENT`](#vidioc_subscribe_event)
  - [`HIDIOCGFLAG`](#hidiocgflag)
  - [`HIDIOCGUCODE`](#hidiocgucode)
  - [`VIDIOC_OMAP3ISP_AF_CFG`](#vidioc_omap3isp_af_cfg)
  - [`DM_REMOVE_ALL`](#dm_remove_all)
  - [`ASPEED_LPC_CTRL_IOCTL_MAP`](#aspeed_lpc_ctrl_ioctl_map)
  - [`CCISS_GETFIRMVER`](#cciss_getfirmver)
  - [`ND_IOCTL_ARS_START`](#nd_ioctl_ars_start)
  - [`PPPIOCSMRRU`](#pppiocsmrru)
  - [`CEC_ADAP_S_LOG_ADDRS`](#cec_adap_s_log_addrs)
  - [`RPROC_GET_SHUTDOWN_ON_RELEASE`](#rproc_get_shutdown_on_release)
  - [`DMA_HEAP_IOCTL_ALLOC`](#dma_heap_ioctl_alloc)
  - [`PPSETTIME`](#ppsettime)
  - [`RTC_ALM_READ`](#rtc_alm_read)
  - [`VDUSE_SET_API_VERSION`](#vduse_set_api_version)
  - [`RIO_MPORT_MAINT_WRITE_REMOTE`](#rio_mport_maint_write_remote)
  - [`VIDIOC_SUBDEV_S_CROP`](#vidioc_subdev_s_crop)
  - [`USBDEVFS_CONNECT`](#usbdevfs_connect)
  - [`SYNC_IOC_FILE_INFO`](#sync_ioc_file_info)
  - [`ATMARP_MKIP`](#atmarp_mkip)
  - [`VFIO_IOMMU_SPAPR_TCE_GET_INFO`](#vfio_iommu_spapr_tce_get_info)
  - [`CCISS_GETHEARTBEAT`](#cciss_getheartbeat)
  - [`ATM_RSTADDR`](#atm_rstaddr)
  - [`NBD_SET_SIZE`](#nbd_set_size)
  - [`UDF_GETVOLIDENT`](#udf_getvolident)
  - [`GPIO_V2_LINE_GET_VALUES_IOCTL`](#gpio_v2_line_get_values_ioctl)
  - [`MGSL_IOCSTXIDLE`](#mgsl_iocstxidle)
  - [`FSL_HV_IOCTL_SETPROP`](#fsl_hv_ioctl_setprop)
  - [`BTRFS_IOC_GET_DEV_STATS`](#btrfs_ioc_get_dev_stats)
  - [`PPRSTATUS`](#pprstatus)
  - [`MGSL_IOCTXENABLE`](#mgsl_ioctxenable)
  - [`UDF_GETEASIZE`](#udf_geteasize)
  - [`NVME_IOCTL_ADMIN64_CMD`](#nvme_ioctl_admin64_cmd)
  - [`VHOST_SET_OWNER`](#vhost_set_owner)
  - [`RIO_ALLOC_DMA`](#rio_alloc_dma)
  - [`RIO_CM_CHAN_ACCEPT`](#rio_cm_chan_accept)
  - [`I2OHRTGET`](#i2ohrtget)
  - [`ATM_SETCIRANGE`](#atm_setcirange)
  - [`HPET_IE_ON`](#hpet_ie_on)
  - [`PERF_EVENT_IOC_ID`](#perf_event_ioc_id)
  - [`TUNSETSNDBUF`](#tunsetsndbuf)
  - [`PTP_PIN_SETFUNC`](#ptp_pin_setfunc)
  - [`PPPIOCDISCONN`](#pppiocdisconn)
  - [`VIDIOC_QUERYCTRL`](#vidioc_queryctrl)
  - [`PPEXCL`](#ppexcl)
  - [`PCITEST_MSI`](#pcitest_msi)
  - [`FDWERRORCLR`](#fdwerrorclr)
  - [`AUTOFS_IOC_FAIL`](#autofs_ioc_fail)
  - [`USBDEVFS_IOCTL`](#usbdevfs_ioctl)
  - [`VIDIOC_S_STD`](#vidioc_s_std)
  - [`F2FS_IOC_RESIZE_FS`](#f2fs_ioc_resize_fs)
  - [`SONET_SETDIAG`](#sonet_setdiag)
  - [`BTRFS_IOC_DEFRAG`](#btrfs_ioc_defrag)
  - [`CCISS_GETDRIVVER`](#cciss_getdrivver)
  - [`IPMICTL_GET_TIMING_PARMS_CMD`](#ipmictl_get_timing_parms_cmd)
  - [`HPET_IRQFREQ`](#hpet_irqfreq)
  - [`ATM_GETESI`](#atm_getesi)
  - [`CCISS_GETLUNINFO`](#cciss_getluninfo)
  - [`AUTOFS_DEV_IOCTL_ISMOUNTPOINT`](#autofs_dev_ioctl_ismountpoint)
  - [`TEE_IOC_SHM_ALLOC`](#tee_ioc_shm_alloc)
  - [`PERF_EVENT_IOC_SET_BPF`](#perf_event_ioc_set_bpf)
  - [`UDMABUF_CREATE_LIST`](#udmabuf_create_list)
  - [`VHOST_SET_LOG_BASE`](#vhost_set_log_base)
  - [`ZATM_GETPOOL`](#zatm_getpool)
  - [`BR2684_SETFILT`](#br2684_setfilt)
  - [`RNDGETPOOL`](#rndgetpool)
  - [`PPS_GETPARAMS`](#pps_getparams)
  - [`IOC_PR_RESERVE`](#ioc_pr_reserve)
  - [`VIDIOC_TRY_DECODER_CMD`](#vidioc_try_decoder_cmd)
  - [`RIO_CM_CHAN_CLOSE`](#rio_cm_chan_close)
  - [`VIDIOC_DV_TIMINGS_CAP`](#vidioc_dv_timings_cap)
  - [`IOCTL_MEI_CONNECT_CLIENT_VTAG`](#ioctl_mei_connect_client_vtag)
  - [`PMU_IOC_GET_BACKLIGHT`](#pmu_ioc_get_backlight)
  - [`USBDEVFS_GET_CAPABILITIES`](#usbdevfs_get_capabilities)
  - [`SCIF_WRITETO`](#scif_writeto)
  - [`UDF_RELOCATE_BLOCKS`](#udf_relocate_blocks)
  - [`FSL_HV_IOCTL_PARTITION_RESTART`](#fsl_hv_ioctl_partition_restart)
  - [`CCISS_REGNEWD`](#cciss_regnewd)
  - [`FAT_IOCTL_SET_ATTRIBUTES`](#fat_ioctl_set_attributes)
  - [`VIDIOC_CREATE_BUFS`](#vidioc_create_bufs)
  - [`CAPI_GET_VERSION`](#capi_get_version)
  - [`SWITCHTEC_IOCTL_EVENT_SUMMARY`](#switchtec_ioctl_event_summary)
  - [`VFIO_EEH_PE_OP`](#vfio_eeh_pe_op)
  - [`FW_CDEV_IOC_CREATE_ISO_CONTEXT`](#fw_cdev_ioc_create_iso_context)
  - [`F2FS_IOC_RELEASE_COMPRESS_BLOCKS`](#f2fs_ioc_release_compress_blocks)
  - [`NBD_SET_SIZE_BLOCKS`](#nbd_set_size_blocks)
  - [`IPMI_BMC_IOCTL_SET_SMS_ATN`](#ipmi_bmc_ioctl_set_sms_atn)
  - [`ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG`](#aspeed_p2a_ctrl_ioctl_get_memory_config)
  - [`VIDIOC_S_AUDOUT`](#vidioc_s_audout)
  - [`VIDIOC_S_FMT`](#vidioc_s_fmt)
  - [`PPPIOCATTACH`](#pppiocattach)
  - [`VHOST_GET_VRING_BUSYLOOP_TIMEOUT`](#vhost_get_vring_busyloop_timeout)
  - [`FS_IOC_MEASURE_VERITY`](#fs_ioc_measure_verity)
  - [`CCISS_BIG_PASSTHRU`](#cciss_big_passthru)
  - [`IPMICTL_SET_MY_LUN_CMD`](#ipmictl_set_my_lun_cmd)
  - [`PCITEST_LEGACY_IRQ`](#pcitest_legacy_irq)
  - [`USBDEVFS_SUBMITURB`](#usbdevfs_submiturb)
  - [`AUTOFS_IOC_READY`](#autofs_ioc_ready)
  - [`BTRFS_IOC_SEND`](#btrfs_ioc_send)
  - [`VIDIOC_G_EXT_CTRLS`](#vidioc_g_ext_ctrls)
  - [`JSIOCSBTNMAP`](#jsiocsbtnmap)
  - [`PPPIOCSFLAGS`](#pppiocsflags)
  - [`NVRAM_INIT`](#nvram_init)
  - [`RFKILL_IOCTL_NOINPUT`](#rfkill_ioctl_noinput)
  - [`BTRFS_IOC_BALANCE`](#btrfs_ioc_balance)
  - [`FS_IOC_GETFSMAP`](#fs_ioc_getfsmap)
  - [`IPMICTL_GET_MY_CHANNEL_LUN_CMD`](#ipmictl_get_my_channel_lun_cmd)
  - [`STP_POLICY_ID_GET`](#stp_policy_id_get)
  - [`PPSETFLAGS`](#ppsetflags)
  - [`CEC_ADAP_S_PHYS_ADDR`](#cec_adap_s_phys_addr)
  - [`ATMTCP_CREATE`](#atmtcp_create)
  - [`IPMI_BMC_IOCTL_FORCE_ABORT`](#ipmi_bmc_ioctl_force_abort)
  - [`PPPIOCGXASYNCMAP`](#pppiocgxasyncmap)
  - [`VHOST_SET_VRING_CALL`](#vhost_set_vring_call)
  - [`LIRC_GET_FEATURES`](#lirc_get_features)
  - [`GSMIOC_DISABLE_NET`](#gsmioc_disable_net)
  - [`AUTOFS_IOC_CATATONIC`](#autofs_ioc_catatonic)
  - [`NBD_DO_IT`](#nbd_do_it)
  - [`LIRC_SET_REC_CARRIER_RANGE`](#lirc_set_rec_carrier_range)
  - [`IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD`](#ipmictl_get_my_channel_address_cmd)
  - [`EVIOCSCLOCKID`](#eviocsclockid)
  - [`USBDEVFS_FREE_STREAMS`](#usbdevfs_free_streams)
  - [`FSI_SCOM_RESET`](#fsi_scom_reset)
  - [`PMU_IOC_GRAB_BACKLIGHT`](#pmu_ioc_grab_backlight)
  - [`VIDIOC_SUBDEV_S_FMT`](#vidioc_subdev_s_fmt)
  - [`FDDEFPRM`](#fddefprm)
  - [`TEE_IOC_INVOKE`](#tee_ioc_invoke)
  - [`USBDEVFS_BULK`](#usbdevfs_bulk)
  - [`SCIF_VWRITETO`](#scif_vwriteto)
  - [`SONYPI_IOCSBRT`](#sonypi_iocsbrt)
  - [`BTRFS_IOC_FILE_EXTENT_SAME`](#btrfs_ioc_file_extent_same)
  - [`RTC_PIE_ON`](#rtc_pie_on)
  - [`BTRFS_IOC_SCAN_DEV`](#btrfs_ioc_scan_dev)
  - [`PPPIOCXFERUNIT`](#pppiocxferunit)
  - [`WDIOC_GETTIMEOUT`](#wdioc_gettimeout)
  - [`BTRFS_IOC_SET_RECEIVED_SUBVOL`](#btrfs_ioc_set_received_subvol)
  - [`DFL_FPGA_PORT_ERR_SET_IRQ`](#dfl_fpga_port_err_set_irq)
  - [`FBIO_WAITFORVSYNC`](#fbio_waitforvsync)
  - [`RTC_PIE_OFF`](#rtc_pie_off)
  - [`EVIOCGRAB`](#eviocgrab)
  - [`PMU_IOC_SET_BACKLIGHT`](#pmu_ioc_set_backlight)
  - [`EVIOCGREP`](#eviocgrep)
  - [`PERF_EVENT_IOC_MODIFY_ATTRIBUTES`](#perf_event_ioc_modify_attributes)
  - [`UFFDIO_CONTINUE`](#uffdio_continue)
  - [`VDUSE_GET_API_VERSION`](#vduse_get_api_version)
  - [`RTC_RD_TIME`](#rtc_rd_time)
  - [`FDMSGOFF`](#fdmsgoff)
  - [`IPMICTL_REGISTER_FOR_CMD_CHANS`](#ipmictl_register_for_cmd_chans)
  - [`CAPI_GET_ERRCODE`](#capi_get_errcode)
  - [`PCITEST_SET_IRQTYPE`](#pcitest_set_irqtype)
  - [`VIDIOC_SUBDEV_S_EDID`](#vidioc_subdev_s_edid)
  - [`MATROXFB_SET_OUTPUT_MODE`](#matroxfb_set_output_mode)
  - [`RIO_DEV_ADD`](#rio_dev_add)
  - [`VIDIOC_ENUM_FREQ_BANDS`](#vidioc_enum_freq_bands)
  - [`FBIO_RADEON_SET_MIRROR`](#fbio_radeon_set_mirror)
  - [`PCITEST_GET_IRQTYPE`](#pcitest_get_irqtype)
  - [`JSIOCGVERSION`](#jsiocgversion)
  - [`SONYPI_IOCSBLUE`](#sonypi_iocsblue)
  - [`SNAPSHOT_PREF_IMAGE_SIZE`](#snapshot_pref_image_size)
  - [`F2FS_IOC_GET_FEATURES`](#f2fs_ioc_get_features)
  - [`SCIF_REG`](#scif_reg)
  - [`NILFS_IOCTL_CLEAN_SEGMENTS`](#nilfs_ioctl_clean_segments)
  - [`FW_CDEV_IOC_INITIATE_BUS_RESET`](#fw_cdev_ioc_initiate_bus_reset)
  - [`RIO_WAIT_FOR_ASYNC`](#rio_wait_for_async)
  - [`VHOST_SET_VRING_NUM`](#vhost_set_vring_num)
  - [`AUTOFS_DEV_IOCTL_PROTOVER`](#autofs_dev_ioctl_protover)
  - [`RIO_FREE_DMA`](#rio_free_dma)
  - [`MGSL_IOCRXENABLE`](#mgsl_iocrxenable)
  - [`IOCTL_VM_SOCKETS_GET_LOCAL_CID`](#ioctl_vm_sockets_get_local_cid)
  - [`IPMICTL_SET_TIMING_PARMS_CMD`](#ipmictl_set_timing_parms_cmd)
  - [`PPPIOCGL2TPSTATS`](#pppiocgl2tpstats)
  - [`PERF_EVENT_IOC_PERIOD`](#perf_event_ioc_period)
  - [`PTP_PIN_SETFUNC2`](#ptp_pin_setfunc2)
  - [`CHIOEXCHANGE`](#chioexchange)
  - [`NILFS_IOCTL_GET_SUINFO`](#nilfs_ioctl_get_suinfo)
  - [`CEC_DQEVENT`](#cec_dqevent)
  - [`UI_SET_SWBIT`](#ui_set_swbit)
  - [`VHOST_VDPA_SET_CONFIG`](#vhost_vdpa_set_config)
  - [`TUNSETIFF`](#tunsetiff)
  - [`CHIOPOSITION`](#chioposition)
  - [`IPMICTL_SET_MAINTENANCE_MODE_CMD`](#ipmictl_set_maintenance_mode_cmd)
  - [`BTRFS_IOC_DEFAULT_SUBVOL`](#btrfs_ioc_default_subvol)
  - [`RIO_UNMAP_OUTBOUND`](#rio_unmap_outbound)
  - [`CAPI_CLR_FLAGS`](#capi_clr_flags)
  - [`FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE`](#fw_cdev_ioc_allocate_iso_resource_once)
  - [`MATROXFB_GET_OUTPUT_CONNECTION`](#matroxfb_get_output_connection)
  - [`EVIOCSMASK`](#eviocsmask)
  - [`BTRFS_IOC_FORGET_DEV`](#btrfs_ioc_forget_dev)
  - [`CXL_MEM_QUERY_COMMANDS`](#cxl_mem_query_commands)
  - [`CEC_S_MODE`](#cec_s_mode)
  - [`MGSL_IOCSIF`](#mgsl_iocsif)
  - [`SWITCHTEC_IOCTL_PFF_TO_PORT`](#switchtec_ioctl_pff_to_port)
  - [`PPSETMODE`](#ppsetmode)
  - [`VFIO_DEVICE_SET_IRQS`](#vfio_device_set_irqs)
  - [`VIDIOC_PREPARE_BUF`](#vidioc_prepare_buf)
  - [`CEC_ADAP_G_CONNECTOR_INFO`](#cec_adap_g_connector_info)
  - [`IOC_OPAL_WRITE_SHADOW_MBR`](#ioc_opal_write_shadow_mbr)
  - [`VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL`](#vidioc_subdev_enum_frame_interval)
  - [`UDMABUF_CREATE`](#udmabuf_create)
  - [`SONET_CLRDIAG`](#sonet_clrdiag)
  - [`PHN_SET_REG`](#phn_set_reg)
  - [`RNDADDTOENTCNT`](#rndaddtoentcnt)
  - [`VBG_IOCTL_CHECK_BALLOON`](#vbg_ioctl_check_balloon)
  - [`VIDIOC_OMAP3ISP_STAT_REQ`](#vidioc_omap3isp_stat_req)
  - [`PPS_FETCH`](#pps_fetch)
  - [`RTC_AIE_OFF`](#rtc_aie_off)
  - [`VFIO_GROUP_SET_CONTAINER`](#vfio_group_set_container)
  - [`FW_CDEV_IOC_RECEIVE_PHY_PACKETS`](#fw_cdev_ioc_receive_phy_packets)
  - [`VFIO_IOMMU_SPAPR_TCE_REMOVE`](#vfio_iommu_spapr_tce_remove)
  - [`VFIO_IOMMU_GET_INFO`](#vfio_iommu_get_info)
  - [`DM_DEV_SUSPEND`](#dm_dev_suspend)
  - [`F2FS_IOC_GET_COMPRESS_OPTION`](#f2fs_ioc_get_compress_option)
  - [`FW_CDEV_IOC_STOP_ISO`](#fw_cdev_ioc_stop_iso)
  - [`GPIO_V2_GET_LINEINFO_IOCTL`](#gpio_v2_get_lineinfo_ioctl)
  - [`ATMMPC_CTRL`](#atmmpc_ctrl)
  - [`PPPIOCSXASYNCMAP`](#pppiocsxasyncmap)
  - [`CHIOGSTATUS`](#chiogstatus)
  - [`FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE`](#fw_cdev_ioc_allocate_iso_resource)
  - [`RIO_MPORT_MAINT_PORT_IDX_GET`](#rio_mport_maint_port_idx_get)
  - [`CAPI_SET_FLAGS`](#capi_set_flags)
  - [`VFIO_GROUP_GET_DEVICE_FD`](#vfio_group_get_device_fd)
  - [`VHOST_SET_MEM_TABLE`](#vhost_set_mem_table)
  - [`MATROXFB_SET_OUTPUT_CONNECTION`](#matroxfb_set_output_connection)
  - [`DFL_FPGA_PORT_GET_REGION_INFO`](#dfl_fpga_port_get_region_info)
  - [`VHOST_GET_FEATURES`](#vhost_get_features)
  - [`LIRC_GET_REC_RESOLUTION`](#lirc_get_rec_resolution)
  - [`PACKET_CTRL_CMD`](#packet_ctrl_cmd)
  - [`LIRC_SET_TRANSMITTER_MASK`](#lirc_set_transmitter_mask)
  - [`BTRFS_IOC_ADD_DEV`](#btrfs_ioc_add_dev)
  - [`JSIOCGCORR`](#jsiocgcorr)
  - [`VIDIOC_G_FMT`](#vidioc_g_fmt)
  - [`RTC_EPOCH_SET`](#rtc_epoch_set)
  - [`CAPI_GET_PROFILE`](#capi_get_profile)
  - [`ATM_GETLOOP`](#atm_getloop)
  - [`SCIF_LISTEN`](#scif_listen)
  - [`NBD_CLEAR_QUE`](#nbd_clear_que)
  - [`F2FS_IOC_MOVE_RANGE`](#f2fs_ioc_move_range)
  - [`LIRC_GET_LENGTH`](#lirc_get_length)
  - [`I8K_SET_FAN`](#i8k_set_fan)
  - [`FDSETMAXERRS`](#fdsetmaxerrs)
  - [`VIDIOC_SUBDEV_QUERYCAP`](#vidioc_subdev_querycap)
  - [`SNAPSHOT_SET_SWAP_AREA`](#snapshot_set_swap_area)
  - [`LIRC_GET_REC_TIMEOUT`](#lirc_get_rec_timeout)
  - [`EVIOCRMFF`](#eviocrmff)
  - [`GPIO_GET_LINEEVENT_IOCTL`](#gpio_get_lineevent_ioctl)
  - [`PPRDATA`](#pprdata)
  - [`RIO_MPORT_GET_PROPERTIES`](#rio_mport_get_properties)
  - [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz)
  - [`GPIO_GET_LINEINFO_IOCTL`](#gpio_get_lineinfo_ioctl)
  - [`GSMIOC_GETCONF`](#gsmioc_getconf)
  - [`LIRC_GET_SEND_MODE`](#lirc_get_send_mode)
  - [`PPPIOCSACTIVE`](#pppiocsactive)
  - [`SIOCGSTAMPNS_NEW`](#siocgstampns_new)
  - [`IPMICTL_RECEIVE_MSG`](#ipmictl_receive_msg)
  - [`LIRC_SET_SEND_DUTY_CYCLE`](#lirc_set_send_duty_cycle)
  - [`UI_END_FF_ERASE`](#ui_end_ff_erase)
  - [`SWITCHTEC_IOCTL_FLASH_PART_INFO`](#switchtec_ioctl_flash_part_info)
  - [`FW_CDEV_IOC_SEND_PHY_PACKET`](#fw_cdev_ioc_send_phy_packet)
  - [`NBD_SET_FLAGS`](#nbd_set_flags)
  - [`VFIO_DEVICE_GET_REGION_INFO`](#vfio_device_get_region_info)
  - [`REISERFS_IOC_UNPACK`](#reiserfs_ioc_unpack)
  - [`FW_CDEV_IOC_REMOVE_DESCRIPTOR`](#fw_cdev_ioc_remove_descriptor)
  - [`RIO_SET_EVENT_MASK`](#rio_set_event_mask)
  - [`SNAPSHOT_ALLOC_SWAP_PAGE`](#snapshot_alloc_swap_page)
  - [`VDUSE_VQ_INJECT_IRQ`](#vduse_vq_inject_irq)
  - [`I2OPASSTHRU`](#i2opassthru)
  - [`IOC_OPAL_SET_PW`](#ioc_opal_set_pw)
  - [`FSI_SCOM_READ`](#fsi_scom_read)
  - [`VHOST_VDPA_GET_DEVICE_ID`](#vhost_vdpa_get_device_id)
  - [`VIDIOC_QBUF`](#vidioc_qbuf)
  - [`VIDIOC_S_TUNER`](#vidioc_s_tuner)
  - [`TUNGETVNETHDRSZ`](#tungetvnethdrsz)
  - [`CAPI_NCCI_GETUNIT`](#capi_ncci_getunit)
  - [`DFL_FPGA_PORT_UINT_GET_IRQ_NUM`](#dfl_fpga_port_uint_get_irq_num)
  - [`VIDIOC_OMAP3ISP_STAT_EN`](#vidioc_omap3isp_stat_en)
  - [`GPIO_V2_LINE_SET_CONFIG_IOCTL`](#gpio_v2_line_set_config_ioctl)
  - [`TEE_IOC_VERSION`](#tee_ioc_version)
  - [`VIDIOC_LOG_STATUS`](#vidioc_log_status)
  - [`IPMICTL_SEND_COMMAND_SETTIME`](#ipmictl_send_command_settime)
  - [`VHOST_SET_LOG_FD`](#vhost_set_log_fd)
  - [`SCIF_SEND`](#scif_send)
  - [`VIDIOC_SUBDEV_G_FMT`](#vidioc_subdev_g_fmt)
  - [`NS_ADJBUFLEV`](#ns_adjbuflev)
  - [`VIDIOC_DBG_S_REGISTER`](#vidioc_dbg_s_register)
  - [`NILFS_IOCTL_RESIZE`](#nilfs_ioctl_resize)
  - [`PHN_GETREG`](#phn_getreg)
  - [`I2OSWDL`](#i2oswdl)
  - [`VBG_IOCTL_VMMDEV_REQUEST_BIG`](#vbg_ioctl_vmmdev_request_big)
  - [`JSIOCGBUTTONS`](#jsiocgbuttons)
  - [`VFIO_IOMMU_ENABLE`](#vfio_iommu_enable)
  - [`DM_DEV_RENAME`](#dm_dev_rename)
  - [`MEDIA_IOC_SETUP_LINK`](#media_ioc_setup_link)
  - [`VIDIOC_ENUMOUTPUT`](#vidioc_enumoutput)
  - [`STP_POLICY_ID_SET`](#stp_policy_id_set)
  - [`VHOST_VDPA_SET_CONFIG_CALL`](#vhost_vdpa_set_config_call)
  - [`VIDIOC_SUBDEV_G_CROP`](#vidioc_subdev_g_crop)
  - [`VIDIOC_S_CROP`](#vidioc_s_crop)
  - [`WDIOC_GETTEMP`](#wdioc_gettemp)
  - [`IOC_OPAL_ADD_USR_TO_LR`](#ioc_opal_add_usr_to_lr)
  - [`UI_SET_LEDBIT`](#ui_set_ledbit)
  - [`NBD_SET_SOCK`](#nbd_set_sock)
  - [`BTRFS_IOC_SNAP_DESTROY_V2`](#btrfs_ioc_snap_destroy_v2)
  - [`HIDIOCGCOLLECTIONINFO`](#hidiocgcollectioninfo)
  - [`I2OSWUL`](#i2oswul)
  - [`IOCTL_MEI_NOTIFY_GET`](#ioctl_mei_notify_get)
  - [`FDFMTTRK`](#fdfmttrk)
  - [`MMTIMER_GETBITS`](#mmtimer_getbits)
  - [`VIDIOC_ENUMSTD`](#vidioc_enumstd)
  - [`VHOST_GET_VRING_BASE`](#vhost_get_vring_base)
  - [`VFIO_DEVICE_IOEVENTFD`](#vfio_device_ioeventfd)
  - [`ATMARP_SETENTRY`](#atmarp_setentry)
  - [`CCISS_REVALIDVOLS`](#cciss_revalidvols)
  - [`MGSL_IOCLOOPTXDONE`](#mgsl_ioclooptxdone)
  - [`RTC_VL_READ`](#rtc_vl_read)
  - [`ND_IOCTL_ARS_STATUS`](#nd_ioctl_ars_status)
  - [`RIO_DEV_DEL`](#rio_dev_del)
  - [`VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES`](#vbg_ioctl_acquire_guest_capabilities)
  - [`VIDIOC_SUBDEV_DV_TIMINGS_CAP`](#vidioc_subdev_dv_timings_cap)
  - [`SONYPI_IOCSFAN`](#sonypi_iocsfan)
  - [`SPIOCSTYPE`](#spiocstype)
  - [`IPMICTL_REGISTER_FOR_CMD`](#ipmictl_register_for_cmd)
  - [`I8K_GET_FAN`](#i8k_get_fan)
  - [`TUNGETVNETBE`](#tungetvnetbe)
  - [`AUTOFS_DEV_IOCTL_FAIL`](#autofs_dev_ioctl_fail)
  - [`UI_END_FF_UPLOAD`](#ui_end_ff_upload)
  - [`TOSH_SMM`](#tosh_smm)
  - [`SONYPI_IOCGBAT2REM`](#sonypi_iocgbat2rem)
  - [`F2FS_IOC_GET_COMPRESS_BLOCKS`](#f2fs_ioc_get_compress_blocks)
  - [`PPPIOCSNPMODE`](#pppiocsnpmode)
  - [`USBDEVFS_CONTROL`](#usbdevfs_control)
  - [`HIDIOCGUSAGE`](#hidiocgusage)
  - [`TUNSETTXFILTER`](#tunsettxfilter)
  - [`TUNGETVNETLE`](#tungetvnetle)
  - [`VIDIOC_ENUM_DV_TIMINGS`](#vidioc_enum_dv_timings)
  - [`BTRFS_IOC_INO_PATHS`](#btrfs_ioc_ino_paths)
  - [`MGSL_IOCGXSYNC`](#mgsl_iocgxsync)
  - [`HIDIOCGFIELDINFO`](#hidiocgfieldinfo)
  - [`VIDIOC_SUBDEV_G_STD`](#vidioc_subdev_g_std)
  - [`I2OVALIDATE`](#i2ovalidate)
  - [`VIDIOC_TRY_ENCODER_CMD`](#vidioc_try_encoder_cmd)
  - [`NILFS_IOCTL_GET_CPINFO`](#nilfs_ioctl_get_cpinfo)
  - [`VIDIOC_G_FREQUENCY`](#vidioc_g_frequency)
  - [`VFAT_IOCTL_READDIR_SHORT`](#vfat_ioctl_readdir_short)
  - [`ND_IOCTL_GET_CONFIG_DATA`](#nd_ioctl_get_config_data)
  - [`F2FS_IOC_RESERVE_COMPRESS_BLOCKS`](#f2fs_ioc_reserve_compress_blocks)
  - [`FDGETDRVSTAT`](#fdgetdrvstat)
  - [`SYNC_IOC_MERGE`](#sync_ioc_merge)
  - [`VIDIOC_S_DV_TIMINGS`](#vidioc_s_dv_timings)
  - [`PPPIOCBRIDGECHAN`](#pppiocbridgechan)
  - [`LIRC_SET_SEND_MODE`](#lirc_set_send_mode)
  - [`RIO_ENABLE_PORTWRITE_RANGE`](#rio_enable_portwrite_range)
  - [`ATM_GETTYPE`](#atm_gettype)
  - [`PHN_GETREGS`](#phn_getregs)
  - [`FDSETEMSGTRESH`](#fdsetemsgtresh)
  - [`NILFS_IOCTL_GET_VINFO`](#nilfs_ioctl_get_vinfo)
  - [`MGSL_IOCWAITEVENT`](#mgsl_iocwaitevent)
  - [`CAPI_INSTALLED`](#capi_installed)
  - [`EVIOCGMASK`](#eviocgmask)
  - [`BTRFS_IOC_SUBVOL_GETFLAGS`](#btrfs_ioc_subvol_getflags)
  - [`FSL_HV_IOCTL_PARTITION_GET_STATUS`](#fsl_hv_ioctl_partition_get_status)
  - [`MEDIA_IOC_ENUM_ENTITIES`](#media_ioc_enum_entities)
  - [`GSMIOC_GETFIRST`](#gsmioc_getfirst)
  - [`FW_CDEV_IOC_FLUSH_ISO`](#fw_cdev_ioc_flush_iso)
  - [`VIDIOC_DBG_G_CHIP_INFO`](#vidioc_dbg_g_chip_info)
  - [`F2FS_IOC_RELEASE_VOLATILE_WRITE`](#f2fs_ioc_release_volatile_write)
  - [`CAPI_GET_SERIAL`](#capi_get_serial)
  - [`FDSETDRVPRM`](#fdsetdrvprm)
  - [`IOC_OPAL_SAVE`](#ioc_opal_save)
  - [`VIDIOC_G_DV_TIMINGS`](#vidioc_g_dv_timings)
  - [`TUNSETIFINDEX`](#tunsetifindex)
  - [`CCISS_SETINTINFO`](#cciss_setintinfo)
  - [`RTC_VL_CLR`](#rtc_vl_clr)
  - [`VIDIOC_REQBUFS`](#vidioc_reqbufs)
  - [`USBDEVFS_REAPURBNDELAY32`](#usbdevfs_reapurbndelay32)
  - [`TEE_IOC_SHM_REGISTER`](#tee_ioc_shm_register)
  - [`USBDEVFS_SETCONFIGURATION`](#usbdevfs_setconfiguration)
  - [`CCISS_GETNODENAME`](#cciss_getnodename)
  - [`VIDIOC_SUBDEV_S_FRAME_INTERVAL`](#vidioc_subdev_s_frame_interval)
  - [`VIDIOC_ENUM_FRAMESIZES`](#vidioc_enum_framesizes)
  - [`VFIO_DEVICE_PCI_HOT_RESET`](#vfio_device_pci_hot_reset)
  - [`FW_CDEV_IOC_SEND_BROADCAST_REQUEST`](#fw_cdev_ioc_send_broadcast_request)
  - [`LPSETTIMEOUT_NEW`](#lpsettimeout_new)
  - [`RIO_CM_MPORT_GET_LIST`](#rio_cm_mport_get_list)
  - [`FW_CDEV_IOC_QUEUE_ISO`](#fw_cdev_ioc_queue_iso)
  - [`FDRAWCMD`](#fdrawcmd)
  - [`SCIF_UNREG`](#scif_unreg)
  - [`PPPIOCGIDLE64`](#pppiocgidle64)
  - [`USBDEVFS_RELEASEINTERFACE`](#usbdevfs_releaseinterface)
  - [`VIDIOC_CROPCAP`](#vidioc_cropcap)
  - [`DFL_FPGA_PORT_GET_INFO`](#dfl_fpga_port_get_info)
  - [`PHN_SET_REGS`](#phn_set_regs)
  - [`ATMLEC_DATA`](#atmlec_data)
  - [`PPPOEIOCDFWD`](#pppoeiocdfwd)
  - [`VIDIOC_S_SELECTION`](#vidioc_s_selection)
  - [`SNAPSHOT_FREE_SWAP_PAGES`](#snapshot_free_swap_pages)
  - [`BTRFS_IOC_LOGICAL_INO`](#btrfs_ioc_logical_ino)
  - [`VIDIOC_S_CTRL`](#vidioc_s_ctrl)
  - [`ZATM_SETPOOL`](#zatm_setpool)
  - [`MTIOCPOS`](#mtiocpos)
  - [`PMU_IOC_SLEEP`](#pmu_ioc_sleep)
  - [`AUTOFS_DEV_IOCTL_PROTOSUBVER`](#autofs_dev_ioctl_protosubver)
  - [`VBG_IOCTL_CHANGE_FILTER_MASK`](#vbg_ioctl_change_filter_mask)
  - [`NILFS_IOCTL_GET_SUSTAT`](#nilfs_ioctl_get_sustat)
  - [`VIDIOC_QUERYCAP`](#vidioc_querycap)
  - [`HPET_INFO`](#hpet_info)
  - [`VIDIOC_AM437X_CCDC_CFG`](#vidioc_am437x_ccdc_cfg)
  - [`DM_LIST_DEVICES`](#dm_list_devices)
  - [`TUNSETOWNER`](#tunsetowner)
  - [`VBG_IOCTL_CHANGE_GUEST_CAPABILITIES`](#vbg_ioctl_change_guest_capabilities)
  - [`RNDADDENTROPY`](#rndaddentropy)
  - [`USBDEVFS_RESET`](#usbdevfs_reset)
  - [`BTRFS_IOC_SUBVOL_CREATE`](#btrfs_ioc_subvol_create)
  - [`USBDEVFS_FORBID_SUSPEND`](#usbdevfs_forbid_suspend)
  - [`FDGETDRVTYP`](#fdgetdrvtyp)
  - [`PPWCONTROL`](#ppwcontrol)
  - [`VIDIOC_ENUM_FRAMEINTERVALS`](#vidioc_enum_frameintervals)
  - [`KCOV_DISABLE`](#kcov_disable)
  - [`IOC_OPAL_ACTIVATE_LSP`](#ioc_opal_activate_lsp)
  - [`VHOST_VDPA_GET_IOVA_RANGE`](#vhost_vdpa_get_iova_range)
  - [`PPPIOCSPASS`](#pppiocspass)
  - [`RIO_CM_CHAN_CONNECT`](#rio_cm_chan_connect)
  - [`I2OSWDEL`](#i2oswdel)
  - [`FS_IOC_SET_ENCRYPTION_POLICY`](#fs_ioc_set_encryption_policy)
  - [`IOC_OPAL_MBR_DONE`](#ioc_opal_mbr_done)
  - [`PPPIOCSMAXCID`](#pppiocsmaxcid)
  - [`PPSETPHASE`](#ppsetphase)
  - [`VHOST_VDPA_SET_VRING_ENABLE`](#vhost_vdpa_set_vring_enable)
  - [`USBDEVFS_GET_SPEED`](#usbdevfs_get_speed)
  - [`SONET_GETFRAMING`](#sonet_getframing)
  - [`VIDIOC_QUERYBUF`](#vidioc_querybuf)
  - [`VIDIOC_S_EDID`](#vidioc_s_edid)
  - [`BTRFS_IOC_QGROUP_ASSIGN`](#btrfs_ioc_qgroup_assign)
  - [`PPS_GETCAP`](#pps_getcap)
  - [`SNAPSHOT_PLATFORM_SUPPORT`](#snapshot_platform_support)
  - [`LIRC_SET_REC_TIMEOUT_REPORTS`](#lirc_set_rec_timeout_reports)
  - [`SCIF_GET_NODEIDS`](#scif_get_nodeids)
  - [`NBD_DISCONNECT`](#nbd_disconnect)
  - [`VIDIOC_SUBDEV_G_FRAME_INTERVAL`](#vidioc_subdev_g_frame_interval)
  - [`VFIO_IOMMU_DISABLE`](#vfio_iommu_disable)
  - [`SNAPSHOT_CREATE_IMAGE`](#snapshot_create_image)
  - [`SNAPSHOT_POWER_OFF`](#snapshot_power_off)
  - [`APM_IOC_STANDBY`](#apm_ioc_standby)
  - [`PPPIOCGUNIT`](#pppiocgunit)
  - [`AUTOFS_IOC_EXPIRE_MULTI`](#autofs_ioc_expire_multi)
  - [`SCIF_BIND`](#scif_bind)
  - [`IOC_WATCH_QUEUE_SET_SIZE`](#ioc_watch_queue_set_size)
  - [`NILFS_IOCTL_CHANGE_CPMODE`](#nilfs_ioctl_change_cpmode)
  - [`IOC_OPAL_LOCK_UNLOCK`](#ioc_opal_lock_unlock)
  - [`F2FS_IOC_SET_PIN_FILE`](#f2fs_ioc_set_pin_file)
  - [`PPPIOCGRASYNCMAP`](#pppiocgrasyncmap)
  - [`MMTIMER_MMAPAVAIL`](#mmtimer_mmapavail)
  - [`I2OPASSTHRU32`](#i2opassthru32)
  - [`DFL_FPGA_FME_PORT_RELEASE`](#dfl_fpga_fme_port_release)
  - [`VIDIOC_SUBDEV_QUERY_DV_TIMINGS`](#vidioc_subdev_query_dv_timings)
  - [`UI_SET_SNDBIT`](#ui_set_sndbit)
  - [`VIDIOC_G_AUDOUT`](#vidioc_g_audout)
  - [`RTC_PLL_SET`](#rtc_pll_set)
  - [`VIDIOC_ENUMAUDIO`](#vidioc_enumaudio)
  - [`AUTOFS_DEV_IOCTL_TIMEOUT`](#autofs_dev_ioctl_timeout)
  - [`VBG_IOCTL_DRIVER_VERSION_INFO`](#vbg_ioctl_driver_version_info)
  - [`VHOST_SCSI_GET_EVENTS_MISSED`](#vhost_scsi_get_events_missed)
  - [`VHOST_SET_VRING_ADDR`](#vhost_set_vring_addr)
  - [`VDUSE_CREATE_DEV`](#vduse_create_dev)
  - [`FDFLUSH`](#fdflush)
  - [`VBG_IOCTL_WAIT_FOR_EVENTS`](#vbg_ioctl_wait_for_events)
  - [`DFL_FPGA_FME_ERR_SET_IRQ`](#dfl_fpga_fme_err_set_irq)
  - [`F2FS_IOC_GET_PIN_FILE`](#f2fs_ioc_get_pin_file)
  - [`SCIF_CONNECT`](#scif_connect)
  - [`BLKREPORTZONE`](#blkreportzone)
  - [`AUTOFS_IOC_ASKUMOUNT`](#autofs_ioc_askumount)
  - [`ATM_ADDPARTY`](#atm_addparty)
  - [`FDSETPRM`](#fdsetprm)
  - [`ATM_GETSTATZ`](#atm_getstatz)
  - [`ISST_IF_MSR_COMMAND`](#isst_if_msr_command)
  - [`BTRFS_IOC_GET_SUBVOL_INFO`](#btrfs_ioc_get_subvol_info)
  - [`VIDIOC_UNSUBSCRIBE_EVENT`](#vidioc_unsubscribe_event)
  - [`SEV_ISSUE_CMD`](#sev_issue_cmd)
  - [`GPIOHANDLE_SET_LINE_VALUES_IOCTL`](#gpiohandle_set_line_values_ioctl)
  - [`PCITEST_COPY`](#pcitest_copy)
  - [`IPMICTL_GET_MY_ADDRESS_CMD`](#ipmictl_get_my_address_cmd)
  - [`CHIOGPICKER`](#chiogpicker)
  - [`CAPI_NCCI_OPENCOUNT`](#capi_ncci_opencount)
  - [`CXL_MEM_SEND_COMMAND`](#cxl_mem_send_command)
  - [`PERF_EVENT_IOC_SET_FILTER`](#perf_event_ioc_set_filter)
  - [`IOC_OPAL_REVERT_TPR`](#ioc_opal_revert_tpr)
  - [`CHIOGVPARAMS`](#chiogvparams)
  - [`PTP_PEROUT_REQUEST`](#ptp_perout_request)
  - [`FSI_SCOM_CHECK`](#fsi_scom_check)
  - [`RTC_IRQP_READ`](#rtc_irqp_read)
  - [`RIO_MPORT_MAINT_READ_LOCAL`](#rio_mport_maint_read_local)
  - [`HIDIOCGRDESCSIZE`](#hidiocgrdescsize)
  - [`UI_GET_VERSION`](#ui_get_version)
  - [`NILFS_IOCTL_GET_CPSTAT`](#nilfs_ioctl_get_cpstat)
  - [`CCISS_GETBUSTYPES`](#cciss_getbustypes)
  - [`VFIO_IOMMU_SPAPR_TCE_CREATE`](#vfio_iommu_spapr_tce_create)
  - [`VIDIOC_EXPBUF`](#vidioc_expbuf)
  - [`UI_SET_RELBIT`](#ui_set_relbit)
  - [`VFIO_SET_IOMMU`](#vfio_set_iommu)
  - [`VIDIOC_S_MODULATOR`](#vidioc_s_modulator)
  - [`TUNGETFILTER`](#tungetfilter)
  - [`CCISS_SETNODENAME`](#cciss_setnodename)
  - [`FBIO_GETCONTROL2`](#fbio_getcontrol2)
  - [`TUNSETDEBUG`](#tunsetdebug)
  - [`DM_DEV_REMOVE`](#dm_dev_remove)
  - [`HIDIOCSUSAGES`](#hidiocsusages)
  - [`FS_IOC_ADD_ENCRYPTION_KEY`](#fs_ioc_add_encryption_key)
  - [`FBIOGET_VBLANK`](#fbioget_vblank)
  - [`ATM_GETSTAT`](#atm_getstat)
  - [`VIDIOC_G_JPEGCOMP`](#vidioc_g_jpegcomp)
  - [`TUNATTACHFILTER`](#tunattachfilter)
  - [`UI_SET_ABSBIT`](#ui_set_absbit)
  - [`DFL_FPGA_PORT_ERR_GET_IRQ_NUM`](#dfl_fpga_port_err_get_irq_num)
  - [`USBDEVFS_REAPURB32`](#usbdevfs_reapurb32)
  - [`BTRFS_IOC_TRANS_END`](#btrfs_ioc_trans_end)
  - [`CAPI_REGISTER`](#capi_register)
  - [`F2FS_IOC_COMPRESS_FILE`](#f2fs_ioc_compress_file)
  - [`USBDEVFS_DISCARDURB`](#usbdevfs_discardurb)
  - [`HE_GET_REG`](#he_get_reg)
  - [`ATM_SETLOOP`](#atm_setloop)
  - [`ATMSIGD_CTRL`](#atmsigd_ctrl)
  - [`CIOC_KERNEL_VERSION`](#cioc_kernel_version)
  - [`BTRFS_IOC_CLONE_RANGE`](#btrfs_ioc_clone_range)
  - [`SNAPSHOT_UNFREEZE`](#snapshot_unfreeze)
  - [`F2FS_IOC_START_VOLATILE_WRITE`](#f2fs_ioc_start_volatile_write)
  - [`PMU_IOC_HAS_ADB`](#pmu_ioc_has_adb)
  - [`I2OGETIOPS`](#i2ogetiops)
  - [`VIDIOC_S_FBUF`](#vidioc_s_fbuf)
  - [`PPRCONTROL`](#pprcontrol)
  - [`CHIOSPICKER`](#chiospicker)
  - [`VFIO_IOMMU_SPAPR_REGISTER_MEMORY`](#vfio_iommu_spapr_register_memory)
  - [`TUNGETSNDBUF`](#tungetsndbuf)
  - [`GSMIOC_SETCONF`](#gsmioc_setconf)
  - [`IOC_PR_PREEMPT`](#ioc_pr_preempt)
  - [`KCOV_INIT_TRACE`](#kcov_init_trace)
  - [`SONYPI_IOCGBAT1CAP`](#sonypi_iocgbat1cap)
  - [`SWITCHTEC_IOCTL_FLASH_INFO`](#switchtec_ioctl_flash_info)
  - [`MTIOCTOP`](#mtioctop)
  - [`VHOST_VDPA_SET_STATUS`](#vhost_vdpa_set_status)
  - [`VHOST_SCSI_SET_EVENTS_MISSED`](#vhost_scsi_set_events_missed)
  - [`VFIO_IOMMU_DIRTY_PAGES`](#vfio_iommu_dirty_pages)
  - [`BTRFS_IOC_SCRUB_PROGRESS`](#btrfs_ioc_scrub_progress)
  - [`PPPIOCGMRU`](#pppiocgmru)
  - [`BTRFS_IOC_DEV_REPLACE`](#btrfs_ioc_dev_replace)
  - [`PPPIOCGFLAGS`](#pppiocgflags)
  - [`NILFS_IOCTL_SET_SUINFO`](#nilfs_ioctl_set_suinfo)
  - [`FW_CDEV_IOC_GET_CYCLE_TIMER2`](#fw_cdev_ioc_get_cycle_timer2)
  - [`ATM_DELLECSADDR`](#atm_dellecsaddr)
  - [`FW_CDEV_IOC_GET_SPEED`](#fw_cdev_ioc_get_speed)
  - [`PPPIOCGIDLE32`](#pppiocgidle32)
  - [`VFIO_DEVICE_RESET`](#vfio_device_reset)
  - [`GPIO_GET_LINEINFO_UNWATCH_IOCTL`](#gpio_get_lineinfo_unwatch_ioctl)
  - [`WDIOC_GETSTATUS`](#wdioc_getstatus)
  - [`BTRFS_IOC_SET_FEATURES`](#btrfs_ioc_set_features)
  - [`IOCTL_MEI_CONNECT_CLIENT`](#ioctl_mei_connect_client)
  - [`VIDIOC_OMAP3ISP_AEWB_CFG`](#vidioc_omap3isp_aewb_cfg)
  - [`PCITEST_READ`](#pcitest_read)
  - [`VFIO_GROUP_GET_STATUS`](#vfio_group_get_status)
  - [`MATROXFB_GET_ALL_OUTPUTS`](#matroxfb_get_all_outputs)
  - [`USBDEVFS_CLEAR_HALT`](#usbdevfs_clear_halt)
  - [`VIDIOC_DECODER_CMD`](#vidioc_decoder_cmd)
  - [`VIDIOC_G_AUDIO`](#vidioc_g_audio)
  - [`CCISS_RESCANDISK`](#cciss_rescandisk)
  - [`RIO_DISABLE_PORTWRITE_RANGE`](#rio_disable_portwrite_range)
  - [`IOC_OPAL_SECURE_ERASE_LR`](#ioc_opal_secure_erase_lr)
  - [`USBDEVFS_REAPURB`](#usbdevfs_reapurb)
  - [`DFL_FPGA_CHECK_EXTENSION`](#dfl_fpga_check_extension)
  - [`AUTOFS_IOC_PROTOVER`](#autofs_ioc_protover)
  - [`FSL_HV_IOCTL_MEMCPY`](#fsl_hv_ioctl_memcpy)
  - [`BTRFS_IOC_GET_FEATURES`](#btrfs_ioc_get_features)
  - [`PCITEST_MSIX`](#pcitest_msix)
  - [`BTRFS_IOC_DEFRAG_RANGE`](#btrfs_ioc_defrag_range)
  - [`UI_BEGIN_FF_ERASE`](#ui_begin_ff_erase)
  - [`DM_GET_TARGET_VERSION`](#dm_get_target_version)
  - [`PPPIOCGIDLE`](#pppiocgidle)
  - [`NVRAM_SETCKS`](#nvram_setcks)
  - [`WDIOC_GETSUPPORT`](#wdioc_getsupport)
  - [`GSMIOC_ENABLE_NET`](#gsmioc_enable_net)
  - [`GPIO_GET_CHIPINFO_IOCTL`](#gpio_get_chipinfo_ioctl)
  - [`NE_ADD_VCPU`](#ne_add_vcpu)
  - [`EVIOCSKEYCODE_V2`](#eviocskeycode_v2)
  - [`PTP_SYS_OFFSET_EXTENDED2`](#ptp_sys_offset_extended2)
  - [`SCIF_FENCE_WAIT`](#scif_fence_wait)
  - [`RIO_TRANSFER`](#rio_transfer)
  - [`FSL_HV_IOCTL_DOORBELL`](#fsl_hv_ioctl_doorbell)
  - [`RIO_MPORT_MAINT_WRITE_LOCAL`](#rio_mport_maint_write_local)
  - [`I2OEVTREG`](#i2oevtreg)
  - [`I2OPARMGET`](#i2oparmget)
  - [`EVIOCGID`](#eviocgid)
  - [`BTRFS_IOC_QGROUP_CREATE`](#btrfs_ioc_qgroup_create)
  - [`AUTOFS_DEV_IOCTL_SETPIPEFD`](#autofs_dev_ioctl_setpipefd)
  - [`VIDIOC_S_PARM`](#vidioc_s_parm)
  - [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf)
  - [`ATM_GETNAMES`](#atm_getnames)
  - [`VIDIOC_QUERYMENU`](#vidioc_querymenu)
  - [`DFL_FPGA_PORT_DMA_UNMAP`](#dfl_fpga_port_dma_unmap)
  - [`I2OLCTGET`](#i2olctget)
  - [`FS_IOC_GET_ENCRYPTION_PWSALT`](#fs_ioc_get_encryption_pwsalt)
  - [`NS_SETBUFLEV`](#ns_setbuflev)
  - [`BLKCLOSEZONE`](#blkclosezone)
  - [`SONET_GETFRSENSE`](#sonet_getfrsense)
  - [`UI_SET_EVBIT`](#ui_set_evbit)
  - [`DM_LIST_VERSIONS`](#dm_list_versions)
  - [`HIDIOCGSTRING`](#hidiocgstring)
  - [`PPPIOCATTCHAN`](#pppiocattchan)
  - [`VDUSE_DEV_SET_CONFIG`](#vduse_dev_set_config)
  - [`TUNGETFEATURES`](#tungetfeatures)
  - [`VFIO_GROUP_UNSET_CONTAINER`](#vfio_group_unset_container)
  - [`IPMICTL_SET_MY_ADDRESS_CMD`](#ipmictl_set_my_address_cmd)
  - [`CCISS_REGNEWDISK`](#cciss_regnewdisk)
  - [`VIDIOC_QUERY_DV_TIMINGS`](#vidioc_query_dv_timings)
  - [`PHN_SETREGS`](#phn_setregs)
  - [`FAT_IOCTL_GET_ATTRIBUTES`](#fat_ioctl_get_attributes)
  - [`FSL_MC_SEND_MC_COMMAND`](#fsl_mc_send_mc_command)
  - [`TUNGETIFF`](#tungetiff)
  - [`PTP_CLOCK_GETCAPS2`](#ptp_clock_getcaps2)
  - [`BTRFS_IOC_RESIZE`](#btrfs_ioc_resize)
  - [`VHOST_SET_VRING_ENDIAN`](#vhost_set_vring_endian)
  - [`PPS_KC_BIND`](#pps_kc_bind)
  - [`F2FS_IOC_WRITE_CHECKPOINT`](#f2fs_ioc_write_checkpoint)
  - [`UI_SET_FFBIT`](#ui_set_ffbit)
  - [`IPMICTL_GET_MY_LUN_CMD`](#ipmictl_get_my_lun_cmd)
  - [`CEC_ADAP_G_PHYS_ADDR`](#cec_adap_g_phys_addr)
  - [`CEC_G_MODE`](#cec_g_mode)
  - [`USBDEVFS_RESETEP`](#usbdevfs_resetep)
  - [`MEDIA_REQUEST_IOC_QUEUE`](#media_request_ioc_queue)
  - [`USBDEVFS_ALLOC_STREAMS`](#usbdevfs_alloc_streams)
  - [`MGSL_IOCSXCTRL`](#mgsl_iocsxctrl)
  - [`MEDIA_IOC_G_TOPOLOGY`](#media_ioc_g_topology)
  - [`PPPIOCUNBRIDGECHAN`](#pppiocunbridgechan)
  - [`F2FS_IOC_COMMIT_ATOMIC_WRITE`](#f2fs_ioc_commit_atomic_write)
  - [`ISST_IF_GET_PLATFORM_INFO`](#isst_if_get_platform_info)
  - [`SCIF_FENCE_MARK`](#scif_fence_mark)
  - [`USBDEVFS_RELEASE_PORT`](#usbdevfs_release_port)
  - [`VFIO_CHECK_EXTENSION`](#vfio_check_extension)
  - [`BTRFS_IOC_QGROUP_LIMIT`](#btrfs_ioc_qgroup_limit)
  - [`FAT_IOCTL_GET_VOLUME_ID`](#fat_ioctl_get_volume_id)
  - [`UI_SET_PHYS`](#ui_set_phys)
  - [`FDWERRORGET`](#fdwerrorget)
  - [`VIDIOC_SUBDEV_G_EDID`](#vidioc_subdev_g_edid)
  - [`MGSL_IOCGSTATS`](#mgsl_iocgstats)
  - [`RPROC_SET_SHUTDOWN_ON_RELEASE`](#rproc_set_shutdown_on_release)
  - [`SIOCGSTAMP_NEW`](#siocgstamp_new)
  - [`RTC_WKALM_RD`](#rtc_wkalm_rd)
  - [`PHN_GET_REG`](#phn_get_reg)
  - [`DELL_WMI_SMBIOS_CMD`](#dell_wmi_smbios_cmd)
  - [`PHN_NOT_OH`](#phn_not_oh)
  - [`PPGETMODES`](#ppgetmodes)
  - [`CHIOGPARAMS`](#chiogparams)
  - [`VFIO_DEVICE_GET_GFX_DMABUF`](#vfio_device_get_gfx_dmabuf)
  - [`VHOST_SET_VRING_BUSYLOOP_TIMEOUT`](#vhost_set_vring_busyloop_timeout)
  - [`VIDIOC_SUBDEV_G_SELECTION`](#vidioc_subdev_g_selection)
  - [`BTRFS_IOC_RM_DEV_V2`](#btrfs_ioc_rm_dev_v2)
  - [`MGSL_IOCWAITGPIO`](#mgsl_iocwaitgpio)
  - [`PMU_IOC_CAN_SLEEP`](#pmu_ioc_can_sleep)
  - [`KCOV_ENABLE`](#kcov_enable)
  - [`BTRFS_IOC_CLONE`](#btrfs_ioc_clone)
  - [`F2FS_IOC_DEFRAGMENT`](#f2fs_ioc_defragment)
  - [`FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE`](#fw_cdev_ioc_deallocate_iso_resource)
  - [`AGPIOC_ALLOCATE`](#agpioc_allocate)
  - [`NE_SET_USER_MEMORY_REGION`](#ne_set_user_memory_region)
  - [`MGSL_IOCTXABORT`](#mgsl_ioctxabort)
  - [`MGSL_IOCSGPIO`](#mgsl_iocsgpio)
  - [`LIRC_SET_REC_CARRIER`](#lirc_set_rec_carrier)
  - [`F2FS_IOC_FLUSH_DEVICE`](#f2fs_ioc_flush_device)
  - [`SNAPSHOT_ATOMIC_RESTORE`](#snapshot_atomic_restore)
  - [`RTC_UIE_OFF`](#rtc_uie_off)
  - [`BT_BMC_IOCTL_SMS_ATN`](#bt_bmc_ioctl_sms_atn)
  - [`NVME_IOCTL_ID`](#nvme_ioctl_id)
  - [`NE_START_ENCLAVE`](#ne_start_enclave)
  - [`VIDIOC_STREAMON`](#vidioc_streamon)
  - [`FDPOLLDRVSTAT`](#fdpolldrvstat)
  - [`AUTOFS_DEV_IOCTL_READY`](#autofs_dev_ioctl_ready)
  - [`VIDIOC_ENUMAUDOUT`](#vidioc_enumaudout)
  - [`VIDIOC_SUBDEV_S_STD`](#vidioc_subdev_s_std)
  - [`WDIOC_GETTIMELEFT`](#wdioc_gettimeleft)
  - [`ATM_GETLINKRATE`](#atm_getlinkrate)
  - [`RTC_WKALM_SET`](#rtc_wkalm_set)
  - [`VHOST_GET_BACKEND_FEATURES`](#vhost_get_backend_features)
  - [`ATMARP_ENCAP`](#atmarp_encap)
  - [`CAPI_GET_FLAGS`](#capi_get_flags)
  - [`IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD`](#ipmictl_set_my_channel_address_cmd)
  - [`DFL_FPGA_FME_PORT_ASSIGN`](#dfl_fpga_fme_port_assign)
  - [`NS_GET_OWNER_UID`](#ns_get_owner_uid)
  - [`VIDIOC_OVERLAY`](#vidioc_overlay)
  - [`BTRFS_IOC_WAIT_SYNC`](#btrfs_ioc_wait_sync)
  - [`GPIOHANDLE_SET_CONFIG_IOCTL`](#gpiohandle_set_config_ioctl)
  - [`VHOST_GET_VRING_ENDIAN`](#vhost_get_vring_endian)
  - [`ATM_GETADDR`](#atm_getaddr)
  - [`PHN_GET_REGS`](#phn_get_regs)
  - [`AUTOFS_DEV_IOCTL_REQUESTER`](#autofs_dev_ioctl_requester)
  - [`AUTOFS_DEV_IOCTL_EXPIRE`](#autofs_dev_ioctl_expire)
  - [`SNAPSHOT_S2RAM`](#snapshot_s2ram)
  - [`JSIOCSAXMAP`](#jsiocsaxmap)
  - [`F2FS_IOC_SET_COMPRESS_OPTION`](#f2fs_ioc_set_compress_option)
  - [`VBG_IOCTL_HGCM_DISCONNECT`](#vbg_ioctl_hgcm_disconnect)
  - [`SCIF_FENCE_SIGNAL`](#scif_fence_signal)
  - [`VFIO_DEVICE_GET_PCI_HOT_RESET_INFO`](#vfio_device_get_pci_hot_reset_info)
  - [`VIDIOC_SUBDEV_ENUM_MBUS_CODE`](#vidioc_subdev_enum_mbus_code)
  - [`MMTIMER_GETOFFSET`](#mmtimer_getoffset)
  - [`RIO_CM_CHAN_LISTEN`](#rio_cm_chan_listen)
  - [`ATM_SETSC`](#atm_setsc)
  - [`F2FS_IOC_SHUTDOWN`](#f2fs_ioc_shutdown)
  - [`NVME_IOCTL_RESCAN`](#nvme_ioctl_rescan)
  - [`BLKOPENZONE`](#blkopenzone)
  - [`DM_VERSION`](#dm_version)
  - [`CEC_TRANSMIT`](#cec_transmit)
  - [`FS_IOC_GET_ENCRYPTION_POLICY_EX`](#fs_ioc_get_encryption_policy_ex)
  - [`SIOCMKCLIP`](#siocmkclip)
  - [`IPMI_BMC_IOCTL_CLEAR_SMS_ATN`](#ipmi_bmc_ioctl_clear_sms_atn)
  - [`HIDIOCGVERSION`](#hidiocgversion)
  - [`VIDIOC_S_INPUT`](#vidioc_s_input)
  - [`VIDIOC_G_CROP`](#vidioc_g_crop)
  - [`LIRC_SET_WIDEBAND_RECEIVER`](#lirc_set_wideband_receiver)
  - [`EVIOCGEFFECTS`](#eviocgeffects)
  - [`UVCIOC_CTRL_QUERY`](#uvcioc_ctrl_query)
  - [`IOC_OPAL_GENERIC_TABLE_RW`](#ioc_opal_generic_table_rw)
  - [`FS_IOC_READ_VERITY_METADATA`](#fs_ioc_read_verity_metadata)
  - [`ND_IOCTL_SET_CONFIG_DATA`](#nd_ioctl_set_config_data)
  - [`USBDEVFS_GETDRIVER`](#usbdevfs_getdriver)
  - [`IDT77105_GETSTAT`](#idt77105_getstat)
  - [`HIDIOCINITREPORT`](#hidiocinitreport)
  - [`VFIO_DEVICE_GET_INFO`](#vfio_device_get_info)
  - [`RIO_CM_CHAN_RECEIVE`](#rio_cm_chan_receive)
  - [`RNDGETENTCNT`](#rndgetentcnt)
  - [`PPPIOCNEWUNIT`](#pppiocnewunit)
  - [`BTRFS_IOC_INO_LOOKUP`](#btrfs_ioc_ino_lookup)
  - [`FDRESET`](#fdreset)
  - [`IOC_PR_REGISTER`](#ioc_pr_register)
  - [`HIDIOCSREPORT`](#hidiocsreport)
  - [`TEE_IOC_OPEN_SESSION`](#tee_ioc_open_session)
  - [`TEE_IOC_SUPPL_RECV`](#tee_ioc_suppl_recv)
  - [`BTRFS_IOC_BALANCE_CTL`](#btrfs_ioc_balance_ctl)
  - [`GPIO_GET_LINEINFO_WATCH_IOCTL`](#gpio_get_lineinfo_watch_ioctl)
  - [`HIDIOCGRAWINFO`](#hidiocgrawinfo)
  - [`PPPIOCSCOMPRESS`](#pppiocscompress)
  - [`USBDEVFS_CONNECTINFO`](#usbdevfs_connectinfo)
  - [`BLKRESETZONE`](#blkresetzone)
  - [`CHIOINITELEM`](#chioinitelem)
  - [`NILFS_IOCTL_SET_ALLOC_RANGE`](#nilfs_ioctl_set_alloc_range)
  - [`AUTOFS_DEV_IOCTL_CATATONIC`](#autofs_dev_ioctl_catatonic)
  - [`RIO_MPORT_MAINT_HDID_SET`](#rio_mport_maint_hdid_set)
  - [`PPGETPHASE`](#ppgetphase)
  - [`USBDEVFS_DISCONNECT_CLAIM`](#usbdevfs_disconnect_claim)
  - [`FDMSGON`](#fdmsgon)
  - [`VIDIOC_G_SLICED_VBI_CAP`](#vidioc_g_sliced_vbi_cap)
  - [`BTRFS_IOC_BALANCE_V2`](#btrfs_ioc_balance_v2)
  - [`MEDIA_REQUEST_IOC_REINIT`](#media_request_ioc_reinit)
  - [`IOC_OPAL_ERASE_LR`](#ioc_opal_erase_lr)
  - [`FDFMTBEG`](#fdfmtbeg)
  - [`RNDRESEEDCRNG`](#rndreseedcrng)
  - [`ISST_IF_GET_PHY_ID`](#isst_if_get_phy_id)
  - [`TUNSETNOCSUM`](#tunsetnocsum)
  - [`SONET_GETSTAT`](#sonet_getstat)
  - [`TFD_IOC_SET_TICKS`](#tfd_ioc_set_ticks)
  - [`PPDATADIR`](#ppdatadir)
  - [`IOC_OPAL_ENABLE_DISABLE_MBR`](#ioc_opal_enable_disable_mbr)
  - [`GPIO_V2_GET_LINE_IOCTL`](#gpio_v2_get_line_ioctl)
  - [`RIO_CM_CHAN_SEND`](#rio_cm_chan_send)
  - [`PPWCTLONIRQ`](#ppwctlonirq)
  - [`SONYPI_IOCGBRT`](#sonypi_iocgbrt)
  - [`IOC_PR_RELEASE`](#ioc_pr_release)
  - [`PPCLRIRQ`](#ppclrirq)
  - [`IPMICTL_SET_MY_CHANNEL_LUN_CMD`](#ipmictl_set_my_channel_lun_cmd)
  - [`MGSL_IOCSXSYNC`](#mgsl_iocsxsync)
  - [`HPET_IE_OFF`](#hpet_ie_off)
  - [`IOC_OPAL_ACTIVATE_USR`](#ioc_opal_activate_usr)
  - [`SONET_SETFRAMING`](#sonet_setframing)
  - [`PERF_EVENT_IOC_PAUSE_OUTPUT`](#perf_event_ioc_pause_output)
  - [`BTRFS_IOC_LOGICAL_INO_V2`](#btrfs_ioc_logical_ino_v2)
  - [`VBG_IOCTL_HGCM_CONNECT`](#vbg_ioctl_hgcm_connect)
  - [`BLKFINISHZONE`](#blkfinishzone)
  - [`EVIOCREVOKE`](#eviocrevoke)
  - [`VFIO_DEVICE_FEATURE`](#vfio_device_feature)
  - [`CCISS_GETPCIINFO`](#cciss_getpciinfo)
  - [`ISST_IF_MBOX_COMMAND`](#isst_if_mbox_command)
  - [`SCIF_ACCEPTREQ`](#scif_acceptreq)
  - [`PERF_EVENT_IOC_QUERY_BPF`](#perf_event_ioc_query_bpf)
  - [`VIDIOC_STREAMOFF`](#vidioc_streamoff)
  - [`VDUSE_DESTROY_DEV`](#vduse_destroy_dev)
  - [`FDGETFDCSTAT`](#fdgetfdcstat)
  - [`VIDIOC_S_PRIORITY`](#vidioc_s_priority)
  - [`SNAPSHOT_FREEZE`](#snapshot_freeze)
  - [`VIDIOC_ENUMINPUT`](#vidioc_enuminput)
  - [`ZATM_GETPOOLZ`](#zatm_getpoolz)
  - [`RIO_DISABLE_DOORBELL_RANGE`](#rio_disable_doorbell_range)
  - [`GPIO_V2_GET_LINEINFO_WATCH_IOCTL`](#gpio_v2_get_lineinfo_watch_ioctl)
  - [`VIDIOC_G_STD`](#vidioc_g_std)
  - [`USBDEVFS_ALLOW_SUSPEND`](#usbdevfs_allow_suspend)
  - [`SONET_GETSTATZ`](#sonet_getstatz)
  - [`SCIF_ACCEPTREG`](#scif_acceptreg)
  - [`VIDIOC_ENCODER_CMD`](#vidioc_encoder_cmd)
  - [`PPPIOCSRASYNCMAP`](#pppiocsrasyncmap)
  - [`IOCTL_MEI_NOTIFY_SET`](#ioctl_mei_notify_set)
  - [`BTRFS_IOC_QUOTA_RESCAN_STATUS`](#btrfs_ioc_quota_rescan_status)
  - [`F2FS_IOC_GARBAGE_COLLECT`](#f2fs_ioc_garbage_collect)
  - [`ATMLEC_CTRL`](#atmlec_ctrl)
  - [`MATROXFB_GET_AVAILABLE_OUTPUTS`](#matroxfb_get_available_outputs)
  - [`DM_DEV_CREATE`](#dm_dev_create)
  - [`VHOST_VDPA_GET_VRING_NUM`](#vhost_vdpa_get_vring_num)
  - [`VIDIOC_G_CTRL`](#vidioc_g_ctrl)
  - [`NBD_CLEAR_SOCK`](#nbd_clear_sock)
  - [`VFIO_DEVICE_QUERY_GFX_PLANE`](#vfio_device_query_gfx_plane)
  - [`WDIOC_KEEPALIVE`](#wdioc_keepalive)
  - [`NVME_IOCTL_SUBSYS_RESET`](#nvme_ioctl_subsys_reset)
  - [`PTP_EXTTS_REQUEST2`](#ptp_extts_request2)
  - [`PCITEST_BAR`](#pcitest_bar)
  - [`MGSL_IOCGGPIO`](#mgsl_iocggpio)
  - [`EVIOCSREP`](#eviocsrep)
  - [`VFIO_DEVICE_GET_IRQ_INFO`](#vfio_device_get_irq_info)
  - [`HPET_DPI`](#hpet_dpi)
  - [`VDUSE_VQ_SETUP_KICKFD`](#vduse_vq_setup_kickfd)
  - [`ND_IOCTL_CALL`](#nd_ioctl_call)
  - [`HIDIOCGDEVINFO`](#hidiocgdevinfo)
  - [`DM_TABLE_DEPS`](#dm_table_deps)
  - [`BTRFS_IOC_DEV_INFO`](#btrfs_ioc_dev_info)
  - [`VDUSE_IOTLB_GET_FD`](#vduse_iotlb_get_fd)
  - [`FW_CDEV_IOC_GET_INFO`](#fw_cdev_ioc_get_info)
  - [`VIDIOC_G_PRIORITY`](#vidioc_g_priority)
  - [`ATM_NEWBACKENDIF`](#atm_newbackendif)
  - [`VIDIOC_S_EXT_CTRLS`](#vidioc_s_ext_ctrls)
  - [`VIDIOC_SUBDEV_ENUM_DV_TIMINGS`](#vidioc_subdev_enum_dv_timings)
  - [`VIDIOC_OMAP3ISP_CCDC_CFG`](#vidioc_omap3isp_ccdc_cfg)
  - [`VIDIOC_S_HW_FREQ_SEEK`](#vidioc_s_hw_freq_seek)
  - [`DM_TABLE_LOAD`](#dm_table_load)
  - [`F2FS_IOC_START_ATOMIC_WRITE`](#f2fs_ioc_start_atomic_write)
  - [`VIDIOC_G_OUTPUT`](#vidioc_g_output)
  - [`ATM_DROPPARTY`](#atm_dropparty)
  - [`CHIOGELEM`](#chiogelem)
  - [`BTRFS_IOC_GET_SUPPORTED_FEATURES`](#btrfs_ioc_get_supported_features)
  - [`EVIOCSKEYCODE`](#eviocskeycode)
  - [`NE_GET_IMAGE_LOAD_INFO`](#ne_get_image_load_info)
  - [`TUNSETLINK`](#tunsetlink)
  - [`FW_CDEV_IOC_ADD_DESCRIPTOR`](#fw_cdev_ioc_add_descriptor)
  - [`BTRFS_IOC_SCRUB_CANCEL`](#btrfs_ioc_scrub_cancel)
  - [`PPS_SETPARAMS`](#pps_setparams)
  - [`IOC_OPAL_LR_SETUP`](#ioc_opal_lr_setup)
  - [`FW_CDEV_IOC_DEALLOCATE`](#fw_cdev_ioc_deallocate)
  - [`WDIOC_SETTIMEOUT`](#wdioc_settimeout)
  - [`IOC_WATCH_QUEUE_SET_FILTER`](#ioc_watch_queue_set_filter)
  - [`CAPI_GET_MANUFACTURER`](#capi_get_manufacturer)
  - [`VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY`](#vfio_iommu_spapr_unregister_memory)
  - [`ASPEED_P2A_CTRL_IOCTL_SET_WINDOW`](#aspeed_p2a_ctrl_ioctl_set_window)
  - [`VIDIOC_G_EDID`](#vidioc_g_edid)
  - [`F2FS_IOC_GARBAGE_COLLECT_RANGE`](#f2fs_ioc_garbage_collect_range)
  - [`RIO_MAP_INBOUND`](#rio_map_inbound)
  - [`IOC_OPAL_TAKE_OWNERSHIP`](#ioc_opal_take_ownership)
  - [`USBDEVFS_CLAIM_PORT`](#usbdevfs_claim_port)
  - [`VIDIOC_S_AUDIO`](#vidioc_s_audio)
  - [`FS_IOC_GET_ENCRYPTION_NONCE`](#fs_ioc_get_encryption_nonce)
  - [`FW_CDEV_IOC_SEND_STREAM_PACKET`](#fw_cdev_ioc_send_stream_packet)
  - [`BTRFS_IOC_SNAP_DESTROY`](#btrfs_ioc_snap_destroy)
  - [`SNAPSHOT_FREE`](#snapshot_free)
  - [`I8K_GET_SPEED`](#i8k_get_speed)
  - [`HIDIOCGREPORT`](#hidiocgreport)
  - [`HPET_EPI`](#hpet_epi)
  - [`JSIOCSCORR`](#jsiocscorr)
  - [`IOC_PR_PREEMPT_ABORT`](#ioc_pr_preempt_abort)
  - [`RIO_MAP_OUTBOUND`](#rio_map_outbound)
  - [`ATM_SETESI`](#atm_setesi)
  - [`FW_CDEV_IOC_START_ISO`](#fw_cdev_ioc_start_iso)
  - [`ATM_DELADDR`](#atm_deladdr)
  - [`PPFCONTROL`](#ppfcontrol)
  - [`SONYPI_IOCGFAN`](#sonypi_iocgfan)
  - [`RTC_IRQP_SET`](#rtc_irqp_set)
  - [`PCITEST_WRITE`](#pcitest_write)
  - [`PPCLAIM`](#ppclaim)
  - [`VIDIOC_S_JPEGCOMP`](#vidioc_s_jpegcomp)
  - [`IPMICTL_UNREGISTER_FOR_CMD`](#ipmictl_unregister_for_cmd)
  - [`VHOST_SET_FEATURES`](#vhost_set_features)
  - [`TOSHIBA_ACPI_SCI`](#toshiba_acpi_sci)
  - [`VIDIOC_DQBUF`](#vidioc_dqbuf)
  - [`BTRFS_IOC_BALANCE_PROGRESS`](#btrfs_ioc_balance_progress)
  - [`BTRFS_IOC_SUBVOL_SETFLAGS`](#btrfs_ioc_subvol_setflags)
  - [`ATMLEC_MCAST`](#atmlec_mcast)
  - [`MMTIMER_GETFREQ`](#mmtimer_getfreq)
  - [`VIDIOC_G_SELECTION`](#vidioc_g_selection)
  - [`RTC_ALM_SET`](#rtc_alm_set)
  - [`PPPOEIOCSFWD`](#pppoeiocsfwd)
  - [`IPMICTL_GET_MAINTENANCE_MODE_CMD`](#ipmictl_get_maintenance_mode_cmd)
  - [`FS_IOC_ENABLE_VERITY`](#fs_ioc_enable_verity)
  - [`NILFS_IOCTL_GET_BDESCS`](#nilfs_ioctl_get_bdescs)
  - [`FDFMTEND`](#fdfmtend)
  - [`DMA_BUF_SET_NAME`](#dma_buf_set_name)
  - [`UI_BEGIN_FF_UPLOAD`](#ui_begin_ff_upload)
  - [`RTC_UIE_ON`](#rtc_uie_on)
  - [`PPRELEASE`](#pprelease)
  - [`VFIO_IOMMU_UNMAP_DMA`](#vfio_iommu_unmap_dma)
  - [`VIDIOC_OMAP3ISP_PRV_CFG`](#vidioc_omap3isp_prv_cfg)
  - [`GPIO_GET_LINEHANDLE_IOCTL`](#gpio_get_linehandle_ioctl)
  - [`VFAT_IOCTL_READDIR_BOTH`](#vfat_ioctl_readdir_both)
  - [`NVME_IOCTL_ADMIN_CMD`](#nvme_ioctl_admin_cmd)
  - [`VHOST_SET_VRING_KICK`](#vhost_set_vring_kick)
  - [`BTRFS_IOC_SUBVOL_CREATE_V2`](#btrfs_ioc_subvol_create_v2)
  - [`BTRFS_IOC_SNAP_CREATE`](#btrfs_ioc_snap_create)
  - [`SONYPI_IOCGBAT2CAP`](#sonypi_iocgbat2cap)
  - [`PPNEGOT`](#ppnegot)
  - [`NBD_PRINT_DEBUG`](#nbd_print_debug)
  - [`BTRFS_IOC_INO_LOOKUP_USER`](#btrfs_ioc_ino_lookup_user)
  - [`BTRFS_IOC_GET_SUBVOL_ROOTREF`](#btrfs_ioc_get_subvol_rootref)
  - [`FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS`](#fs_ioc_remove_encryption_key_all_users)
  - [`BTRFS_IOC_FS_INFO`](#btrfs_ioc_fs_info)
  - [`VIDIOC_ENUM_FMT`](#vidioc_enum_fmt)
  - [`VIDIOC_G_INPUT`](#vidioc_g_input)
  - [`VTPM_PROXY_IOC_NEW_DEV`](#vtpm_proxy_ioc_new_dev)
  - [`DFL_FPGA_FME_ERR_GET_IRQ_NUM`](#dfl_fpga_fme_err_get_irq_num)
  - [`ND_IOCTL_DIMM_FLAGS`](#nd_ioctl_dimm_flags)
  - [`BTRFS_IOC_QUOTA_RESCAN`](#btrfs_ioc_quota_rescan)
  - [`MMTIMER_GETCOUNTER`](#mmtimer_getcounter)
  - [`MATROXFB_GET_OUTPUT_MODE`](#matroxfb_get_output_mode)
  - [`BTRFS_IOC_QUOTA_RESCAN_WAIT`](#btrfs_ioc_quota_rescan_wait)
  - [`RIO_CM_CHAN_BIND`](#rio_cm_chan_bind)
  - [`HIDIOCGRDESC`](#hidiocgrdesc)
  - [`MGSL_IOCGIF`](#mgsl_iocgif)
  - [`VIDIOC_S_OUTPUT`](#vidioc_s_output)
  - [`HIDIOCGREPORTINFO`](#hidiocgreportinfo)
  - [`WDIOC_GETBOOTSTATUS`](#wdioc_getbootstatus)
  - [`VDUSE_VQ_GET_INFO`](#vduse_vq_get_info)
  - [`ACRN_IOCTL_ASSIGN_PCIDEV`](#acrn_ioctl_assign_pcidev)
  - [`BLKGETDISKSEQ`](#blkgetdiskseq)
  - [`ACRN_IOCTL_PM_GET_CPU_STATE`](#acrn_ioctl_pm_get_cpu_state)
  - [`ACRN_IOCTL_DESTROY_VM`](#acrn_ioctl_destroy_vm)
  - [`ACRN_IOCTL_SET_PTDEV_INTR`](#acrn_ioctl_set_ptdev_intr)
  - [`ACRN_IOCTL_CREATE_IOREQ_CLIENT`](#acrn_ioctl_create_ioreq_client)
  - [`ACRN_IOCTL_IRQFD`](#acrn_ioctl_irqfd)
  - [`ACRN_IOCTL_CREATE_VM`](#acrn_ioctl_create_vm)
  - [`ACRN_IOCTL_INJECT_MSI`](#acrn_ioctl_inject_msi)
  - [`ACRN_IOCTL_ATTACH_IOREQ_CLIENT`](#acrn_ioctl_attach_ioreq_client)
  - [`ACRN_IOCTL_RESET_PTDEV_INTR`](#acrn_ioctl_reset_ptdev_intr)
  - [`ACRN_IOCTL_NOTIFY_REQUEST_FINISH`](#acrn_ioctl_notify_request_finish)
  - [`ACRN_IOCTL_SET_IRQLINE`](#acrn_ioctl_set_irqline)
  - [`ACRN_IOCTL_START_VM`](#acrn_ioctl_start_vm)
  - [`ACRN_IOCTL_SET_VCPU_REGS`](#acrn_ioctl_set_vcpu_regs)
  - [`ACRN_IOCTL_SET_MEMSEG`](#acrn_ioctl_set_memseg)
  - [`ACRN_IOCTL_PAUSE_VM`](#acrn_ioctl_pause_vm)
  - [`ACRN_IOCTL_CLEAR_VM_IOREQ`](#acrn_ioctl_clear_vm_ioreq)
  - [`ACRN_IOCTL_UNSET_MEMSEG`](#acrn_ioctl_unset_memseg)
  - [`ACRN_IOCTL_IOEVENTFD`](#acrn_ioctl_ioeventfd)
  - [`ACRN_IOCTL_DEASSIGN_PCIDEV`](#acrn_ioctl_deassign_pcidev)
  - [`ACRN_IOCTL_RESET_VM`](#acrn_ioctl_reset_vm)
  - [`ACRN_IOCTL_DESTROY_IOREQ_CLIENT`](#acrn_ioctl_destroy_ioreq_client)
  - [`ACRN_IOCTL_VM_INTR_MONITOR`](#acrn_ioctl_vm_intr_monitor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FIONREAD`](#fionread) | const |  |
| [`FIONBIO`](#fionbio) | const |  |
| [`FIOCLEX`](#fioclex) | const |  |
| [`FIONCLEX`](#fionclex) | const |  |
| [`FIOASYNC`](#fioasync) | const |  |
| [`FIOQSIZE`](#fioqsize) | const |  |
| [`TCXONC`](#tcxonc) | const |  |
| [`TCFLSH`](#tcflsh) | const |  |
| [`TIOCSCTTY`](#tiocsctty) | const |  |
| [`TIOCSPGRP`](#tiocspgrp) | const |  |
| [`TIOCOUTQ`](#tiocoutq) | const |  |
| [`TIOCSTI`](#tiocsti) | const |  |
| [`TIOCSWINSZ`](#tiocswinsz) | const |  |
| [`TIOCMGET`](#tiocmget) | const |  |
| [`TIOCMBIS`](#tiocmbis) | const |  |
| [`TIOCMBIC`](#tiocmbic) | const |  |
| [`TIOCMSET`](#tiocmset) | const |  |
| [`TIOCSSOFTCAR`](#tiocssoftcar) | const |  |
| [`TIOCLINUX`](#tioclinux) | const |  |
| [`TIOCCONS`](#tioccons) | const |  |
| [`TIOCSSERIAL`](#tiocsserial) | const |  |
| [`TIOCPKT`](#tiocpkt) | const |  |
| [`TIOCNOTTY`](#tiocnotty) | const |  |
| [`TIOCSETD`](#tiocsetd) | const |  |
| [`TIOCSBRK`](#tiocsbrk) | const |  |
| [`TIOCCBRK`](#tioccbrk) | const |  |
| [`TIOCSRS485`](#tiocsrs485) | const |  |
| [`TIOCSPTLCK`](#tiocsptlck) | const |  |
| [`TIOCSIG`](#tiocsig) | const |  |
| [`TIOCVHANGUP`](#tiocvhangup) | const |  |
| [`TIOCSERCONFIG`](#tiocserconfig) | const |  |
| [`TIOCSERGWILD`](#tiocsergwild) | const |  |
| [`TIOCSERSWILD`](#tiocserswild) | const |  |
| [`TIOCSLCKTRMIOS`](#tiocslcktrmios) | const |  |
| [`TIOCSERGSTRUCT`](#tiocsergstruct) | const |  |
| [`TIOCSERGETLSR`](#tiocsergetlsr) | const |  |
| [`TIOCSERGETMULTI`](#tiocsergetmulti) | const |  |
| [`TIOCSERSETMULTI`](#tiocsersetmulti) | const |  |
| [`TIOCMIWAIT`](#tiocmiwait) | const |  |
| [`TCGETS`](#tcgets) | const |  |
| [`TCGETA`](#tcgeta) | const |  |
| [`TCSBRK`](#tcsbrk) | const |  |
| [`TCSBRKP`](#tcsbrkp) | const |  |
| [`TCSETA`](#tcseta) | const |  |
| [`TCSETAF`](#tcsetaf) | const |  |
| [`TCSETAW`](#tcsetaw) | const |  |
| [`TIOCEXCL`](#tiocexcl) | const |  |
| [`TIOCNXCL`](#tiocnxcl) | const |  |
| [`TIOCGDEV`](#tiocgdev) | const |  |
| [`TIOCGEXCL`](#tiocgexcl) | const |  |
| [`TIOCGICOUNT`](#tiocgicount) | const |  |
| [`TIOCGLCKTRMIOS`](#tiocglcktrmios) | const |  |
| [`TIOCGPGRP`](#tiocgpgrp) | const |  |
| [`TIOCGPKT`](#tiocgpkt) | const |  |
| [`TIOCGPTLCK`](#tiocgptlck) | const |  |
| [`TIOCGPTN`](#tiocgptn) | const |  |
| [`TIOCGPTPEER`](#tiocgptpeer) | const |  |
| [`TIOCGRS485`](#tiocgrs485) | const |  |
| [`TIOCGSERIAL`](#tiocgserial) | const |  |
| [`TIOCGSID`](#tiocgsid) | const |  |
| [`TIOCGSOFTCAR`](#tiocgsoftcar) | const |  |
| [`TIOCGWINSZ`](#tiocgwinsz) | const |  |
| [`TCGETS2`](#tcgets2) | const |  |
| [`TCGETX`](#tcgetx) | const |  |
| [`TCSETS`](#tcsets) | const |  |
| [`TCSETS2`](#tcsets2) | const |  |
| [`TCSETSF`](#tcsetsf) | const |  |
| [`TCSETSF2`](#tcsetsf2) | const |  |
| [`TCSETSW`](#tcsetsw) | const |  |
| [`TCSETSW2`](#tcsetsw2) | const |  |
| [`TCSETX`](#tcsetx) | const |  |
| [`TCSETXF`](#tcsetxf) | const |  |
| [`TCSETXW`](#tcsetxw) | const |  |
| [`TIOCGETD`](#tiocgetd) | const |  |
| [`MTIOCGET`](#mtiocget) | const |  |
| [`BLKSSZGET`](#blksszget) | const |  |
| [`BLKPBSZGET`](#blkpbszget) | const |  |
| [`BLKROSET`](#blkroset) | const |  |
| [`BLKROGET`](#blkroget) | const |  |
| [`BLKRRPART`](#blkrrpart) | const |  |
| [`BLKGETSIZE`](#blkgetsize) | const |  |
| [`BLKFLSBUF`](#blkflsbuf) | const |  |
| [`BLKRASET`](#blkraset) | const |  |
| [`BLKRAGET`](#blkraget) | const |  |
| [`BLKFRASET`](#blkfraset) | const |  |
| [`BLKFRAGET`](#blkfraget) | const |  |
| [`BLKSECTSET`](#blksectset) | const |  |
| [`BLKSECTGET`](#blksectget) | const |  |
| [`BLKPG`](#blkpg) | const |  |
| [`BLKBSZGET`](#blkbszget) | const |  |
| [`BLKBSZSET`](#blkbszset) | const |  |
| [`BLKGETSIZE64`](#blkgetsize64) | const |  |
| [`BLKTRACESETUP`](#blktracesetup) | const |  |
| [`BLKTRACESTART`](#blktracestart) | const |  |
| [`BLKTRACESTOP`](#blktracestop) | const |  |
| [`BLKTRACETEARDOWN`](#blktraceteardown) | const |  |
| [`BLKDISCARD`](#blkdiscard) | const |  |
| [`BLKIOMIN`](#blkiomin) | const |  |
| [`BLKIOOPT`](#blkioopt) | const |  |
| [`BLKALIGNOFF`](#blkalignoff) | const |  |
| [`BLKDISCARDZEROES`](#blkdiscardzeroes) | const |  |
| [`BLKSECDISCARD`](#blksecdiscard) | const |  |
| [`BLKROTATIONAL`](#blkrotational) | const |  |
| [`BLKZEROOUT`](#blkzeroout) | const |  |
| [`FIEMAP_MAX_OFFSET`](#fiemap_max_offset) | const |  |
| [`FIEMAP_FLAG_SYNC`](#fiemap_flag_sync) | const |  |
| [`FIEMAP_FLAG_XATTR`](#fiemap_flag_xattr) | const |  |
| [`FIEMAP_FLAG_CACHE`](#fiemap_flag_cache) | const |  |
| [`FIEMAP_FLAGS_COMPAT`](#fiemap_flags_compat) | const |  |
| [`FIEMAP_EXTENT_LAST`](#fiemap_extent_last) | const |  |
| [`FIEMAP_EXTENT_UNKNOWN`](#fiemap_extent_unknown) | const |  |
| [`FIEMAP_EXTENT_DELALLOC`](#fiemap_extent_delalloc) | const |  |
| [`FIEMAP_EXTENT_ENCODED`](#fiemap_extent_encoded) | const |  |
| [`FIEMAP_EXTENT_DATA_ENCRYPTED`](#fiemap_extent_data_encrypted) | const |  |
| [`FIEMAP_EXTENT_NOT_ALIGNED`](#fiemap_extent_not_aligned) | const |  |
| [`FIEMAP_EXTENT_DATA_INLINE`](#fiemap_extent_data_inline) | const |  |
| [`FIEMAP_EXTENT_DATA_TAIL`](#fiemap_extent_data_tail) | const |  |
| [`FIEMAP_EXTENT_UNWRITTEN`](#fiemap_extent_unwritten) | const |  |
| [`FIEMAP_EXTENT_MERGED`](#fiemap_extent_merged) | const |  |
| [`FIEMAP_EXTENT_SHARED`](#fiemap_extent_shared) | const |  |
| [`UFFDIO_REGISTER`](#uffdio_register) | const |  |
| [`UFFDIO_UNREGISTER`](#uffdio_unregister) | const |  |
| [`UFFDIO_WAKE`](#uffdio_wake) | const |  |
| [`UFFDIO_COPY`](#uffdio_copy) | const |  |
| [`UFFDIO_ZEROPAGE`](#uffdio_zeropage) | const |  |
| [`UFFDIO_WRITEPROTECT`](#uffdio_writeprotect) | const |  |
| [`UFFDIO_API`](#uffdio_api) | const |  |
| [`NS_GET_USERNS`](#ns_get_userns) | const |  |
| [`NS_GET_PARENT`](#ns_get_parent) | const |  |
| [`NS_GET_NSTYPE`](#ns_get_nstype) | const |  |
| [`KDGETLED`](#kdgetled) | const |  |
| [`KDSETLED`](#kdsetled) | const |  |
| [`KDGKBLED`](#kdgkbled) | const |  |
| [`KDSKBLED`](#kdskbled) | const |  |
| [`KDGKBTYPE`](#kdgkbtype) | const |  |
| [`KDADDIO`](#kdaddio) | const |  |
| [`KDDELIO`](#kddelio) | const |  |
| [`KDENABIO`](#kdenabio) | const |  |
| [`KDDISABIO`](#kddisabio) | const |  |
| [`KDSETMODE`](#kdsetmode) | const |  |
| [`KDGETMODE`](#kdgetmode) | const |  |
| [`KDMKTONE`](#kdmktone) | const |  |
| [`KIOCSOUND`](#kiocsound) | const |  |
| [`GIO_CMAP`](#gio_cmap) | const |  |
| [`PIO_CMAP`](#pio_cmap) | const |  |
| [`GIO_FONT`](#gio_font) | const |  |
| [`GIO_FONTX`](#gio_fontx) | const |  |
| [`PIO_FONT`](#pio_font) | const |  |
| [`PIO_FONTX`](#pio_fontx) | const |  |
| [`PIO_FONTRESET`](#pio_fontreset) | const |  |
| [`GIO_SCRNMAP`](#gio_scrnmap) | const |  |
| [`GIO_UNISCRNMAP`](#gio_uniscrnmap) | const |  |
| [`PIO_SCRNMAP`](#pio_scrnmap) | const |  |
| [`PIO_UNISCRNMAP`](#pio_uniscrnmap) | const |  |
| [`GIO_UNIMAP`](#gio_unimap) | const |  |
| [`PIO_UNIMAP`](#pio_unimap) | const |  |
| [`PIO_UNIMAPCLR`](#pio_unimapclr) | const |  |
| [`KDGKBMODE`](#kdgkbmode) | const |  |
| [`KDSKBMODE`](#kdskbmode) | const |  |
| [`KDGKBMETA`](#kdgkbmeta) | const |  |
| [`KDSKBMETA`](#kdskbmeta) | const |  |
| [`KDGKBENT`](#kdgkbent) | const |  |
| [`KDSKBENT`](#kdskbent) | const |  |
| [`KDGKBSENT`](#kdgkbsent) | const |  |
| [`KDSKBSENT`](#kdskbsent) | const |  |
| [`KDGKBDIACR`](#kdgkbdiacr) | const |  |
| [`KDGETKEYCODE`](#kdgetkeycode) | const |  |
| [`KDSETKEYCODE`](#kdsetkeycode) | const |  |
| [`KDSIGACCEPT`](#kdsigaccept) | const |  |
| [`VT_OPENQRY`](#vt_openqry) | const |  |
| [`VT_GETMODE`](#vt_getmode) | const |  |
| [`VT_SETMODE`](#vt_setmode) | const |  |
| [`VT_GETSTATE`](#vt_getstate) | const |  |
| [`VT_RELDISP`](#vt_reldisp) | const |  |
| [`VT_ACTIVATE`](#vt_activate) | const |  |
| [`VT_WAITACTIVE`](#vt_waitactive) | const |  |
| [`VT_DISALLOCATE`](#vt_disallocate) | const |  |
| [`VT_RESIZE`](#vt_resize) | const |  |
| [`VT_RESIZEX`](#vt_resizex) | const |  |
| [`FIOSETOWN`](#fiosetown) | const |  |
| [`SIOCSPGRP`](#siocspgrp) | const |  |
| [`FIOGETOWN`](#fiogetown) | const |  |
| [`SIOCGPGRP`](#siocgpgrp) | const |  |
| [`SIOCATMARK`](#siocatmark) | const |  |
| [`SIOCGSTAMP`](#siocgstamp) | const |  |
| [`TIOCINQ`](#tiocinq) | const |  |
| [`SIOCADDRT`](#siocaddrt) | const |  |
| [`SIOCDELRT`](#siocdelrt) | const |  |
| [`SIOCGIFNAME`](#siocgifname) | const |  |
| [`SIOCSIFLINK`](#siocsiflink) | const |  |
| [`SIOCGIFCONF`](#siocgifconf) | const |  |
| [`SIOCGIFFLAGS`](#siocgifflags) | const |  |
| [`SIOCSIFFLAGS`](#siocsifflags) | const |  |
| [`SIOCGIFADDR`](#siocgifaddr) | const |  |
| [`SIOCSIFADDR`](#siocsifaddr) | const |  |
| [`SIOCGIFDSTADDR`](#siocgifdstaddr) | const |  |
| [`SIOCSIFDSTADDR`](#siocsifdstaddr) | const |  |
| [`SIOCGIFBRDADDR`](#siocgifbrdaddr) | const |  |
| [`SIOCSIFBRDADDR`](#siocsifbrdaddr) | const |  |
| [`SIOCGIFNETMASK`](#siocgifnetmask) | const |  |
| [`SIOCSIFNETMASK`](#siocsifnetmask) | const |  |
| [`SIOCGIFMETRIC`](#siocgifmetric) | const |  |
| [`SIOCSIFMETRIC`](#siocsifmetric) | const |  |
| [`SIOCGIFMEM`](#siocgifmem) | const |  |
| [`SIOCSIFMEM`](#siocsifmem) | const |  |
| [`SIOCGIFMTU`](#siocgifmtu) | const |  |
| [`SIOCSIFMTU`](#siocsifmtu) | const |  |
| [`SIOCSIFHWADDR`](#siocsifhwaddr) | const |  |
| [`SIOCGIFENCAP`](#siocgifencap) | const |  |
| [`SIOCSIFENCAP`](#siocsifencap) | const |  |
| [`SIOCGIFHWADDR`](#siocgifhwaddr) | const |  |
| [`SIOCGIFSLAVE`](#siocgifslave) | const |  |
| [`SIOCSIFSLAVE`](#siocsifslave) | const |  |
| [`SIOCADDMULTI`](#siocaddmulti) | const |  |
| [`SIOCDELMULTI`](#siocdelmulti) | const |  |
| [`SIOCDARP`](#siocdarp) | const |  |
| [`SIOCGARP`](#siocgarp) | const |  |
| [`SIOCSARP`](#siocsarp) | const |  |
| [`SIOCDRARP`](#siocdrarp) | const |  |
| [`SIOCGRARP`](#siocgrarp) | const |  |
| [`SIOCSRARP`](#siocsrarp) | const |  |
| [`SIOCGIFMAP`](#siocgifmap) | const |  |
| [`SIOCSIFMAP`](#siocsifmap) | const |  |
| [`SIOCRTMSG`](#siocrtmsg) | const |  |
| [`SIOCSIFNAME`](#siocsifname) | const |  |
| [`SIOCGIFINDEX`](#siocgifindex) | const |  |
| [`SIOGIFINDEX`](#siogifindex) | const |  |
| [`SIOCSIFPFLAGS`](#siocsifpflags) | const |  |
| [`SIOCGIFPFLAGS`](#siocgifpflags) | const |  |
| [`SIOCDIFADDR`](#siocdifaddr) | const |  |
| [`SIOCSIFHWBROADCAST`](#siocsifhwbroadcast) | const |  |
| [`SIOCGIFCOUNT`](#siocgifcount) | const |  |
| [`SIOCGIFBR`](#siocgifbr) | const |  |
| [`SIOCSIFBR`](#siocsifbr) | const |  |
| [`SIOCGIFTXQLEN`](#siocgiftxqlen) | const |  |
| [`SIOCSIFTXQLEN`](#siocsiftxqlen) | const |  |
| [`SIOCADDDLCI`](#siocadddlci) | const |  |
| [`SIOCDELDLCI`](#siocdeldlci) | const |  |
| [`SIOCDEVPRIVATE`](#siocdevprivate) | const |  |
| [`SIOCPROTOPRIVATE`](#siocprotoprivate) | const |  |
| [`FIBMAP`](#fibmap) | const |  |
| [`FIGETBSZ`](#figetbsz) | const |  |
| [`FIFREEZE`](#fifreeze) | const |  |
| [`FITHAW`](#fithaw) | const |  |
| [`FITRIM`](#fitrim) | const |  |
| [`FICLONE`](#ficlone) | const |  |
| [`FICLONERANGE`](#ficlonerange) | const |  |
| [`FIDEDUPERANGE`](#fideduperange) | const |  |
| [`FS_IOC_GETFLAGS`](#fs_ioc_getflags) | const |  |
| [`FS_IOC_SETFLAGS`](#fs_ioc_setflags) | const |  |
| [`FS_IOC_GETVERSION`](#fs_ioc_getversion) | const |  |
| [`FS_IOC_SETVERSION`](#fs_ioc_setversion) | const |  |
| [`FS_IOC_FIEMAP`](#fs_ioc_fiemap) | const |  |
| [`FS_IOC32_GETFLAGS`](#fs_ioc32_getflags) | const |  |
| [`FS_IOC32_SETFLAGS`](#fs_ioc32_setflags) | const |  |
| [`FS_IOC32_GETVERSION`](#fs_ioc32_getversion) | const |  |
| [`FS_IOC32_SETVERSION`](#fs_ioc32_setversion) | const |  |
| [`FS_IOC_FSGETXATTR`](#fs_ioc_fsgetxattr) | const |  |
| [`FS_IOC_FSSETXATTR`](#fs_ioc_fssetxattr) | const |  |
| [`FS_IOC_GETFSLABEL`](#fs_ioc_getfslabel) | const |  |
| [`FS_IOC_SETFSLABEL`](#fs_ioc_setfslabel) | const |  |
| [`EXT4_IOC_GETVERSION`](#ext4_ioc_getversion) | const |  |
| [`EXT4_IOC_SETVERSION`](#ext4_ioc_setversion) | const |  |
| [`EXT4_IOC_GETVERSION_OLD`](#ext4_ioc_getversion_old) | const |  |
| [`EXT4_IOC_SETVERSION_OLD`](#ext4_ioc_setversion_old) | const |  |
| [`EXT4_IOC_GETRSVSZ`](#ext4_ioc_getrsvsz) | const |  |
| [`EXT4_IOC_SETRSVSZ`](#ext4_ioc_setrsvsz) | const |  |
| [`EXT4_IOC_GROUP_EXTEND`](#ext4_ioc_group_extend) | const |  |
| [`EXT4_IOC_MIGRATE`](#ext4_ioc_migrate) | const |  |
| [`EXT4_IOC_ALLOC_DA_BLKS`](#ext4_ioc_alloc_da_blks) | const |  |
| [`EXT4_IOC_RESIZE_FS`](#ext4_ioc_resize_fs) | const |  |
| [`EXT4_IOC_SWAP_BOOT`](#ext4_ioc_swap_boot) | const |  |
| [`EXT4_IOC_PRECACHE_EXTENTS`](#ext4_ioc_precache_extents) | const |  |
| [`EXT4_IOC_CLEAR_ES_CACHE`](#ext4_ioc_clear_es_cache) | const |  |
| [`EXT4_IOC_GETSTATE`](#ext4_ioc_getstate) | const |  |
| [`EXT4_IOC_GET_ES_CACHE`](#ext4_ioc_get_es_cache) | const |  |
| [`EXT4_IOC_CHECKPOINT`](#ext4_ioc_checkpoint) | const |  |
| [`EXT4_IOC_SHUTDOWN`](#ext4_ioc_shutdown) | const |  |
| [`EXT4_IOC32_GETVERSION`](#ext4_ioc32_getversion) | const |  |
| [`EXT4_IOC32_SETVERSION`](#ext4_ioc32_setversion) | const |  |
| [`EXT4_IOC32_GETRSVSZ`](#ext4_ioc32_getrsvsz) | const |  |
| [`EXT4_IOC32_SETRSVSZ`](#ext4_ioc32_setrsvsz) | const |  |
| [`EXT4_IOC32_GROUP_EXTEND`](#ext4_ioc32_group_extend) | const |  |
| [`EXT4_IOC32_GETVERSION_OLD`](#ext4_ioc32_getversion_old) | const |  |
| [`EXT4_IOC32_SETVERSION_OLD`](#ext4_ioc32_setversion_old) | const |  |
| [`VIDIOC_SUBDEV_QUERYSTD`](#vidioc_subdev_querystd) | const |  |
| [`AUTOFS_DEV_IOCTL_CLOSEMOUNT`](#autofs_dev_ioctl_closemount) | const |  |
| [`LIRC_SET_SEND_CARRIER`](#lirc_set_send_carrier) | const |  |
| [`AUTOFS_IOC_PROTOSUBVER`](#autofs_ioc_protosubver) | const |  |
| [`PTP_SYS_OFFSET_PRECISE`](#ptp_sys_offset_precise) | const |  |
| [`FSI_SCOM_WRITE`](#fsi_scom_write) | const |  |
| [`ATM_GETCIRANGE`](#atm_getcirange) | const |  |
| [`DMA_BUF_SET_NAME_B`](#dma_buf_set_name_b) | const |  |
| [`RIO_CM_EP_GET_LIST_SIZE`](#rio_cm_ep_get_list_size) | const |  |
| [`TUNSETPERSIST`](#tunsetpersist) | const |  |
| [`FS_IOC_GET_ENCRYPTION_POLICY`](#fs_ioc_get_encryption_policy) | const |  |
| [`CEC_RECEIVE`](#cec_receive) | const |  |
| [`MGSL_IOCGPARAMS`](#mgsl_iocgparams) | const |  |
| [`ENI_SETMULT`](#eni_setmult) | const |  |
| [`RIO_GET_EVENT_MASK`](#rio_get_event_mask) | const |  |
| [`LIRC_GET_MAX_TIMEOUT`](#lirc_get_max_timeout) | const |  |
| [`USBDEVFS_CLAIMINTERFACE`](#usbdevfs_claiminterface) | const |  |
| [`CHIOMOVE`](#chiomove) | const |  |
| [`SONYPI_IOCGBATFLAGS`](#sonypi_iocgbatflags) | const |  |
| [`BTRFS_IOC_SYNC`](#btrfs_ioc_sync) | const |  |
| [`VIDIOC_TRY_FMT`](#vidioc_try_fmt) | const |  |
| [`LIRC_SET_REC_MODE`](#lirc_set_rec_mode) | const |  |
| [`VIDIOC_DQEVENT`](#vidioc_dqevent) | const |  |
| [`RPMSG_DESTROY_EPT_IOCTL`](#rpmsg_destroy_ept_ioctl) | const |  |
| [`UVCIOC_CTRL_MAP`](#uvcioc_ctrl_map) | const |  |
| [`VHOST_SET_BACKEND_FEATURES`](#vhost_set_backend_features) | const |  |
| [`VHOST_VSOCK_SET_GUEST_CID`](#vhost_vsock_set_guest_cid) | const |  |
| [`UI_SET_KEYBIT`](#ui_set_keybit) | const |  |
| [`LIRC_SET_REC_TIMEOUT`](#lirc_set_rec_timeout) | const |  |
| [`FS_IOC_GET_ENCRYPTION_KEY_STATUS`](#fs_ioc_get_encryption_key_status) | const |  |
| [`BTRFS_IOC_TREE_SEARCH_V2`](#btrfs_ioc_tree_search_v2) | const |  |
| [`VHOST_SET_VRING_BASE`](#vhost_set_vring_base) | const |  |
| [`RIO_ENABLE_DOORBELL_RANGE`](#rio_enable_doorbell_range) | const |  |
| [`VIDIOC_TRY_EXT_CTRLS`](#vidioc_try_ext_ctrls) | const |  |
| [`LIRC_GET_REC_MODE`](#lirc_get_rec_mode) | const |  |
| [`PPGETTIME`](#ppgettime) | const |  |
| [`BTRFS_IOC_RM_DEV`](#btrfs_ioc_rm_dev) | const |  |
| [`ATM_SETBACKEND`](#atm_setbackend) | const |  |
| [`FSL_HV_IOCTL_PARTITION_START`](#fsl_hv_ioctl_partition_start) | const |  |
| [`FBIO_WAITEVENT`](#fbio_waitevent) | const |  |
| [`SWITCHTEC_IOCTL_PORT_TO_PFF`](#switchtec_ioctl_port_to_pff) | const |  |
| [`NVME_IOCTL_IO_CMD`](#nvme_ioctl_io_cmd) | const |  |
| [`IPMICTL_RECEIVE_MSG_TRUNC`](#ipmictl_receive_msg_trunc) | const |  |
| [`FDTWADDLE`](#fdtwaddle) | const |  |
| [`NVME_IOCTL_SUBMIT_IO`](#nvme_ioctl_submit_io) | const |  |
| [`NILFS_IOCTL_SYNC`](#nilfs_ioctl_sync) | const |  |
| [`VIDIOC_SUBDEV_S_DV_TIMINGS`](#vidioc_subdev_s_dv_timings) | const |  |
| [`ASPEED_LPC_CTRL_IOCTL_GET_SIZE`](#aspeed_lpc_ctrl_ioctl_get_size) | const |  |
| [`DM_DEV_STATUS`](#dm_dev_status) | const |  |
| [`TEE_IOC_CLOSE_SESSION`](#tee_ioc_close_session) | const |  |
| [`NS_GETPSTAT`](#ns_getpstat) | const |  |
| [`UI_SET_PROPBIT`](#ui_set_propbit) | const |  |
| [`TUNSETFILTEREBPF`](#tunsetfilterebpf) | const |  |
| [`RIO_MPORT_MAINT_COMPTAG_SET`](#rio_mport_maint_comptag_set) | const |  |
| [`AUTOFS_DEV_IOCTL_VERSION`](#autofs_dev_ioctl_version) | const |  |
| [`WDIOC_SETOPTIONS`](#wdioc_setoptions) | const |  |
| [`VHOST_SCSI_SET_ENDPOINT`](#vhost_scsi_set_endpoint) | const |  |
| [`MGSL_IOCGTXIDLE`](#mgsl_iocgtxidle) | const |  |
| [`ATM_ADDLECSADDR`](#atm_addlecsaddr) | const |  |
| [`FSL_HV_IOCTL_GETPROP`](#fsl_hv_ioctl_getprop) | const |  |
| [`FDGETPRM`](#fdgetprm) | const |  |
| [`HIDIOCAPPLICATION`](#hidiocapplication) | const |  |
| [`ENI_MEMDUMP`](#eni_memdump) | const |  |
| [`PTP_SYS_OFFSET2`](#ptp_sys_offset2) | const |  |
| [`VIDIOC_SUBDEV_G_DV_TIMINGS`](#vidioc_subdev_g_dv_timings) | const |  |
| [`DMA_BUF_SET_NAME_A`](#dma_buf_set_name_a) | const |  |
| [`PTP_PIN_GETFUNC`](#ptp_pin_getfunc) | const |  |
| [`PTP_SYS_OFFSET_EXTENDED`](#ptp_sys_offset_extended) | const |  |
| [`DFL_FPGA_PORT_UINT_SET_IRQ`](#dfl_fpga_port_uint_set_irq) | const |  |
| [`RTC_EPOCH_READ`](#rtc_epoch_read) | const |  |
| [`VIDIOC_SUBDEV_S_SELECTION`](#vidioc_subdev_s_selection) | const |  |
| [`VIDIOC_QUERY_EXT_CTRL`](#vidioc_query_ext_ctrl) | const |  |
| [`ATM_GETLECSADDR`](#atm_getlecsaddr) | const |  |
| [`FSL_HV_IOCTL_PARTITION_STOP`](#fsl_hv_ioctl_partition_stop) | const |  |
| [`SONET_GETDIAG`](#sonet_getdiag) | const |  |
| [`ATMMPC_DATA`](#atmmpc_data) | const |  |
| [`IPMICTL_UNREGISTER_FOR_CMD_CHANS`](#ipmictl_unregister_for_cmd_chans) | const |  |
| [`HIDIOCGCOLLECTIONINDEX`](#hidiocgcollectionindex) | const |  |
| [`RPMSG_CREATE_EPT_IOCTL`](#rpmsg_create_ept_ioctl) | const |  |
| [`GPIOHANDLE_GET_LINE_VALUES_IOCTL`](#gpiohandle_get_line_values_ioctl) | const |  |
| [`UI_DEV_SETUP`](#ui_dev_setup) | const |  |
| [`ISST_IF_IO_CMD`](#isst_if_io_cmd) | const |  |
| [`RIO_MPORT_MAINT_READ_REMOTE`](#rio_mport_maint_read_remote) | const |  |
| [`VIDIOC_OMAP3ISP_HIST_CFG`](#vidioc_omap3isp_hist_cfg) | const |  |
| [`BLKGETNRZONES`](#blkgetnrzones) | const |  |
| [`VIDIOC_G_MODULATOR`](#vidioc_g_modulator) | const |  |
| [`VBG_IOCTL_WRITE_CORE_DUMP`](#vbg_ioctl_write_core_dump) | const |  |
| [`USBDEVFS_SETINTERFACE`](#usbdevfs_setinterface) | const |  |
| [`PPPIOCGCHAN`](#pppiocgchan) | const |  |
| [`EVIOCGVERSION`](#eviocgversion) | const |  |
| [`VHOST_NET_SET_BACKEND`](#vhost_net_set_backend) | const |  |
| [`USBDEVFS_REAPURBNDELAY`](#usbdevfs_reapurbndelay) | const |  |
| [`RNDZAPENTCNT`](#rndzapentcnt) | const |  |
| [`VIDIOC_G_PARM`](#vidioc_g_parm) | const |  |
| [`TUNGETDEVNETNS`](#tungetdevnetns) | const |  |
| [`LIRC_SET_MEASURE_CARRIER_MODE`](#lirc_set_measure_carrier_mode) | const |  |
| [`VHOST_SET_VRING_ERR`](#vhost_set_vring_err) | const |  |
| [`VDUSE_VQ_SETUP`](#vduse_vq_setup) | const |  |
| [`AUTOFS_IOC_SETTIMEOUT`](#autofs_ioc_settimeout) | const |  |
| [`VIDIOC_S_FREQUENCY`](#vidioc_s_frequency) | const |  |
| [`F2FS_IOC_SEC_TRIM_FILE`](#f2fs_ioc_sec_trim_file) | const |  |
| [`FS_IOC_REMOVE_ENCRYPTION_KEY`](#fs_ioc_remove_encryption_key) | const |  |
| [`WDIOC_GETPRETIMEOUT`](#wdioc_getpretimeout) | const |  |
| [`USBDEVFS_DROP_PRIVILEGES`](#usbdevfs_drop_privileges) | const |  |
| [`BTRFS_IOC_SNAP_CREATE_V2`](#btrfs_ioc_snap_create_v2) | const |  |
| [`VHOST_VSOCK_SET_RUNNING`](#vhost_vsock_set_running) | const |  |
| [`STP_SET_OPTIONS`](#stp_set_options) | const |  |
| [`FBIO_RADEON_GET_MIRROR`](#fbio_radeon_get_mirror) | const |  |
| [`IVTVFB_IOC_DMA_FRAME`](#ivtvfb_ioc_dma_frame) | const |  |
| [`IPMICTL_SEND_COMMAND`](#ipmictl_send_command) | const |  |
| [`VIDIOC_G_ENC_INDEX`](#vidioc_g_enc_index) | const |  |
| [`DFL_FPGA_FME_PORT_PR`](#dfl_fpga_fme_port_pr) | const |  |
| [`CHIOSVOLTAG`](#chiosvoltag) | const |  |
| [`ATM_SETESIF`](#atm_setesif) | const |  |
| [`FW_CDEV_IOC_SEND_RESPONSE`](#fw_cdev_ioc_send_response) | const |  |
| [`PMU_IOC_GET_MODEL`](#pmu_ioc_get_model) | const |  |
| [`JSIOCGBTNMAP`](#jsiocgbtnmap) | const |  |
| [`USBDEVFS_HUB_PORTINFO`](#usbdevfs_hub_portinfo) | const |  |
| [`VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS`](#vbg_ioctl_interrupt_all_wait_for_events) | const |  |
| [`FDCLRPRM`](#fdclrprm) | const |  |
| [`BTRFS_IOC_SCRUB`](#btrfs_ioc_scrub) | const |  |
| [`USBDEVFS_DISCONNECT`](#usbdevfs_disconnect) | const |  |
| [`TUNSETVNETBE`](#tunsetvnetbe) | const |  |
| [`ATMTCP_REMOVE`](#atmtcp_remove) | const |  |
| [`VHOST_VDPA_GET_CONFIG`](#vhost_vdpa_get_config) | const |  |
| [`PPPIOCGNPMODE`](#pppiocgnpmode) | const |  |
| [`FDGETDRVPRM`](#fdgetdrvprm) | const |  |
| [`TUNSETVNETLE`](#tunsetvnetle) | const |  |
| [`PHN_SETREG`](#phn_setreg) | const |  |
| [`PPPIOCDETACH`](#pppiocdetach) | const |  |
| [`MMTIMER_GETRES`](#mmtimer_getres) | const |  |
| [`VIDIOC_SUBDEV_ENUMSTD`](#vidioc_subdev_enumstd) | const |  |
| [`PPGETFLAGS`](#ppgetflags) | const |  |
| [`VDUSE_DEV_GET_FEATURES`](#vduse_dev_get_features) | const |  |
| [`CAPI_MANUFACTURER_CMD`](#capi_manufacturer_cmd) | const |  |
| [`VIDIOC_G_TUNER`](#vidioc_g_tuner) | const |  |
| [`DM_TABLE_STATUS`](#dm_table_status) | const |  |
| [`DM_DEV_ARM_POLL`](#dm_dev_arm_poll) | const |  |
| [`NE_CREATE_VM`](#ne_create_vm) | const |  |
| [`MEDIA_IOC_ENUM_LINKS`](#media_ioc_enum_links) | const |  |
| [`F2FS_IOC_PRECACHE_EXTENTS`](#f2fs_ioc_precache_extents) | const |  |
| [`DFL_FPGA_PORT_DMA_MAP`](#dfl_fpga_port_dma_map) | const |  |
| [`MGSL_IOCGXCTRL`](#mgsl_iocgxctrl) | const |  |
| [`FW_CDEV_IOC_SEND_REQUEST`](#fw_cdev_ioc_send_request) | const |  |
| [`SONYPI_IOCGBLUE`](#sonypi_iocgblue) | const |  |
| [`F2FS_IOC_DECOMPRESS_FILE`](#f2fs_ioc_decompress_file) | const |  |
| [`I2OHTML`](#i2ohtml) | const |  |
| [`VFIO_GET_API_VERSION`](#vfio_get_api_version) | const |  |
| [`IDT77105_GETSTATZ`](#idt77105_getstatz) | const |  |
| [`I2OPARMSET`](#i2oparmset) | const |  |
| [`TEE_IOC_CANCEL`](#tee_ioc_cancel) | const |  |
| [`PTP_SYS_OFFSET_PRECISE2`](#ptp_sys_offset_precise2) | const |  |
| [`DFL_FPGA_PORT_RESET`](#dfl_fpga_port_reset) | const |  |
| [`PPPIOCGASYNCMAP`](#pppiocgasyncmap) | const |  |
| [`EVIOCGKEYCODE_V2`](#eviocgkeycode_v2) | const |  |
| [`DM_DEV_SET_GEOMETRY`](#dm_dev_set_geometry) | const |  |
| [`HIDIOCSUSAGE`](#hidiocsusage) | const |  |
| [`FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE`](#fw_cdev_ioc_deallocate_iso_resource_once) | const |  |
| [`PTP_EXTTS_REQUEST`](#ptp_extts_request) | const |  |
| [`SWITCHTEC_IOCTL_EVENT_CTL`](#switchtec_ioctl_event_ctl) | const |  |
| [`WDIOC_SETPRETIMEOUT`](#wdioc_setpretimeout) | const |  |
| [`VHOST_SCSI_CLEAR_ENDPOINT`](#vhost_scsi_clear_endpoint) | const |  |
| [`JSIOCGAXES`](#jsiocgaxes) | const |  |
| [`HIDIOCSFLAG`](#hidiocsflag) | const |  |
| [`PTP_PEROUT_REQUEST2`](#ptp_perout_request2) | const |  |
| [`PPWDATA`](#ppwdata) | const |  |
| [`PTP_CLOCK_GETCAPS`](#ptp_clock_getcaps) | const |  |
| [`FDGETMAXERRS`](#fdgetmaxerrs) | const |  |
| [`TUNSETQUEUE`](#tunsetqueue) | const |  |
| [`PTP_ENABLE_PPS`](#ptp_enable_pps) | const |  |
| [`SIOCSIFATMTCP`](#siocsifatmtcp) | const |  |
| [`CEC_ADAP_G_LOG_ADDRS`](#cec_adap_g_log_addrs) | const |  |
| [`ND_IOCTL_ARS_CAP`](#nd_ioctl_ars_cap) | const |  |
| [`NBD_SET_BLKSIZE`](#nbd_set_blksize) | const |  |
| [`NBD_SET_TIMEOUT`](#nbd_set_timeout) | const |  |
| [`VHOST_SCSI_GET_ABI_VERSION`](#vhost_scsi_get_abi_version) | const |  |
| [`RIO_UNMAP_INBOUND`](#rio_unmap_inbound) | const |  |
| [`ATM_QUERYLOOP`](#atm_queryloop) | const |  |
| [`DFL_FPGA_GET_API_VERSION`](#dfl_fpga_get_api_version) | const |  |
| [`USBDEVFS_WAIT_FOR_RESUME`](#usbdevfs_wait_for_resume) | const |  |
| [`FBIO_CURSOR`](#fbio_cursor) | const |  |
| [`RNDCLEARPOOL`](#rndclearpool) | const |  |
| [`VIDIOC_QUERYSTD`](#vidioc_querystd) | const |  |
| [`DMA_BUF_IOCTL_SYNC`](#dma_buf_ioctl_sync) | const |  |
| [`SCIF_RECV`](#scif_recv) | const |  |
| [`PTP_PIN_GETFUNC2`](#ptp_pin_getfunc2) | const |  |
| [`FW_CDEV_IOC_ALLOCATE`](#fw_cdev_ioc_allocate) | const |  |
| [`CEC_ADAP_G_CAPS`](#cec_adap_g_caps) | const |  |
| [`VIDIOC_G_FBUF`](#vidioc_g_fbuf) | const |  |
| [`PTP_ENABLE_PPS2`](#ptp_enable_pps2) | const |  |
| [`PCITEST_CLEAR_IRQ`](#pcitest_clear_irq) | const |  |
| [`IPMICTL_SET_GETS_EVENTS_CMD`](#ipmictl_set_gets_events_cmd) | const |  |
| [`BTRFS_IOC_DEVICES_READY`](#btrfs_ioc_devices_ready) | const |  |
| [`JSIOCGAXMAP`](#jsiocgaxmap) | const |  |
| [`FW_CDEV_IOC_GET_CYCLE_TIMER`](#fw_cdev_ioc_get_cycle_timer) | const |  |
| [`FW_CDEV_IOC_SET_ISO_CHANNELS`](#fw_cdev_ioc_set_iso_channels) | const |  |
| [`RTC_WIE_OFF`](#rtc_wie_off) | const |  |
| [`PPGETMODE`](#ppgetmode) | const |  |
| [`VIDIOC_DBG_G_REGISTER`](#vidioc_dbg_g_register) | const |  |
| [`PTP_SYS_OFFSET`](#ptp_sys_offset) | const |  |
| [`BTRFS_IOC_SPACE_INFO`](#btrfs_ioc_space_info) | const |  |
| [`VIDIOC_SUBDEV_ENUM_FRAME_SIZE`](#vidioc_subdev_enum_frame_size) | const |  |
| [`ND_IOCTL_VENDOR`](#nd_ioctl_vendor) | const |  |
| [`SCIF_VREADFROM`](#scif_vreadfrom) | const |  |
| [`BTRFS_IOC_TRANS_START`](#btrfs_ioc_trans_start) | const |  |
| [`INOTIFY_IOC_SETNEXTWD`](#inotify_ioc_setnextwd) | const |  |
| [`SNAPSHOT_GET_IMAGE_SIZE`](#snapshot_get_image_size) | const |  |
| [`TUNDETACHFILTER`](#tundetachfilter) | const |  |
| [`ND_IOCTL_CLEAR_ERROR`](#nd_ioctl_clear_error) | const |  |
| [`IOC_PR_CLEAR`](#ioc_pr_clear) | const |  |
| [`SCIF_READFROM`](#scif_readfrom) | const |  |
| [`PPPIOCGDEBUG`](#pppiocgdebug) | const |  |
| [`BLKGETZONESZ`](#blkgetzonesz) | const |  |
| [`HIDIOCGUSAGES`](#hidiocgusages) | const |  |
| [`SONYPI_IOCGTEMP`](#sonypi_iocgtemp) | const |  |
| [`UI_SET_MSCBIT`](#ui_set_mscbit) | const |  |
| [`APM_IOC_SUSPEND`](#apm_ioc_suspend) | const |  |
| [`BTRFS_IOC_TREE_SEARCH`](#btrfs_ioc_tree_search) | const |  |
| [`RTC_PLL_GET`](#rtc_pll_get) | const |  |
| [`RIO_CM_EP_GET_LIST`](#rio_cm_ep_get_list) | const |  |
| [`USBDEVFS_DISCSIGNAL`](#usbdevfs_discsignal) | const |  |
| [`LIRC_GET_MIN_TIMEOUT`](#lirc_get_min_timeout) | const |  |
| [`SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY`](#switchtec_ioctl_event_summary_legacy) | const |  |
| [`DM_TARGET_MSG`](#dm_target_msg) | const |  |
| [`SONYPI_IOCGBAT1REM`](#sonypi_iocgbat1rem) | const |  |
| [`EVIOCSFF`](#eviocsff) | const |  |
| [`TUNSETGROUP`](#tunsetgroup) | const |  |
| [`EVIOCGKEYCODE`](#eviocgkeycode) | const |  |
| [`KCOV_REMOTE_ENABLE`](#kcov_remote_enable) | const |  |
| [`ND_IOCTL_GET_CONFIG_SIZE`](#nd_ioctl_get_config_size) | const |  |
| [`FDEJECT`](#fdeject) | const |  |
| [`TUNSETOFFLOAD`](#tunsetoffload) | const |  |
| [`PPPIOCCONNECT`](#pppiocconnect) | const |  |
| [`ATM_ADDADDR`](#atm_addaddr) | const |  |
| [`VDUSE_DEV_INJECT_CONFIG_IRQ`](#vduse_dev_inject_config_irq) | const |  |
| [`AUTOFS_DEV_IOCTL_ASKUMOUNT`](#autofs_dev_ioctl_askumount) | const |  |
| [`VHOST_VDPA_GET_STATUS`](#vhost_vdpa_get_status) | const |  |
| [`CCISS_PASSTHRU`](#cciss_passthru) | const |  |
| [`MGSL_IOCCLRMODCOUNT`](#mgsl_iocclrmodcount) | const |  |
| [`TEE_IOC_SUPPL_SEND`](#tee_ioc_suppl_send) | const |  |
| [`ATMARPD_CTRL`](#atmarpd_ctrl) | const |  |
| [`UI_ABS_SETUP`](#ui_abs_setup) | const |  |
| [`UI_DEV_DESTROY`](#ui_dev_destroy) | const |  |
| [`BTRFS_IOC_QUOTA_CTL`](#btrfs_ioc_quota_ctl) | const |  |
| [`RTC_AIE_ON`](#rtc_aie_on) | const |  |
| [`AUTOFS_IOC_EXPIRE`](#autofs_ioc_expire) | const |  |
| [`PPPIOCSDEBUG`](#pppiocsdebug) | const |  |
| [`GPIO_V2_LINE_SET_VALUES_IOCTL`](#gpio_v2_line_set_values_ioctl) | const |  |
| [`PPPIOCSMRU`](#pppiocsmru) | const |  |
| [`CCISS_DEREGDISK`](#cciss_deregdisk) | const |  |
| [`UI_DEV_CREATE`](#ui_dev_create) | const |  |
| [`FUSE_DEV_IOC_CLONE`](#fuse_dev_ioc_clone) | const |  |
| [`BTRFS_IOC_START_SYNC`](#btrfs_ioc_start_sync) | const |  |
| [`NILFS_IOCTL_DELETE_CHECKPOINT`](#nilfs_ioctl_delete_checkpoint) | const |  |
| [`SNAPSHOT_AVAIL_SWAP_SIZE`](#snapshot_avail_swap_size) | const |  |
| [`DM_TABLE_CLEAR`](#dm_table_clear) | const |  |
| [`CCISS_GETINTINFO`](#cciss_getintinfo) | const |  |
| [`PPPIOCSASYNCMAP`](#pppiocsasyncmap) | const |  |
| [`I2OEVTGET`](#i2oevtget) | const |  |
| [`NVME_IOCTL_RESET`](#nvme_ioctl_reset) | const |  |
| [`PPYIELD`](#ppyield) | const |  |
| [`NVME_IOCTL_IO64_CMD`](#nvme_ioctl_io64_cmd) | const |  |
| [`TUNSETCARRIER`](#tunsetcarrier) | const |  |
| [`DM_DEV_WAIT`](#dm_dev_wait) | const |  |
| [`RTC_WIE_ON`](#rtc_wie_on) | const |  |
| [`MEDIA_IOC_DEVICE_INFO`](#media_ioc_device_info) | const |  |
| [`RIO_CM_CHAN_CREATE`](#rio_cm_chan_create) | const |  |
| [`MGSL_IOCSPARAMS`](#mgsl_iocsparams) | const |  |
| [`RTC_SET_TIME`](#rtc_set_time) | const |  |
| [`VHOST_RESET_OWNER`](#vhost_reset_owner) | const |  |
| [`IOC_OPAL_PSID_REVERT_TPR`](#ioc_opal_psid_revert_tpr) | const |  |
| [`AUTOFS_DEV_IOCTL_OPENMOUNT`](#autofs_dev_ioctl_openmount) | const |  |
| [`UDF_GETEABLOCK`](#udf_geteablock) | const |  |
| [`VFIO_IOMMU_MAP_DMA`](#vfio_iommu_map_dma) | const |  |
| [`VIDIOC_SUBSCRIBE_EVENT`](#vidioc_subscribe_event) | const |  |
| [`HIDIOCGFLAG`](#hidiocgflag) | const |  |
| [`HIDIOCGUCODE`](#hidiocgucode) | const |  |
| [`VIDIOC_OMAP3ISP_AF_CFG`](#vidioc_omap3isp_af_cfg) | const |  |
| [`DM_REMOVE_ALL`](#dm_remove_all) | const |  |
| [`ASPEED_LPC_CTRL_IOCTL_MAP`](#aspeed_lpc_ctrl_ioctl_map) | const |  |
| [`CCISS_GETFIRMVER`](#cciss_getfirmver) | const |  |
| [`ND_IOCTL_ARS_START`](#nd_ioctl_ars_start) | const |  |
| [`PPPIOCSMRRU`](#pppiocsmrru) | const |  |
| [`CEC_ADAP_S_LOG_ADDRS`](#cec_adap_s_log_addrs) | const |  |
| [`RPROC_GET_SHUTDOWN_ON_RELEASE`](#rproc_get_shutdown_on_release) | const |  |
| [`DMA_HEAP_IOCTL_ALLOC`](#dma_heap_ioctl_alloc) | const |  |
| [`PPSETTIME`](#ppsettime) | const |  |
| [`RTC_ALM_READ`](#rtc_alm_read) | const |  |
| [`VDUSE_SET_API_VERSION`](#vduse_set_api_version) | const |  |
| [`RIO_MPORT_MAINT_WRITE_REMOTE`](#rio_mport_maint_write_remote) | const |  |
| [`VIDIOC_SUBDEV_S_CROP`](#vidioc_subdev_s_crop) | const |  |
| [`USBDEVFS_CONNECT`](#usbdevfs_connect) | const |  |
| [`SYNC_IOC_FILE_INFO`](#sync_ioc_file_info) | const |  |
| [`ATMARP_MKIP`](#atmarp_mkip) | const |  |
| [`VFIO_IOMMU_SPAPR_TCE_GET_INFO`](#vfio_iommu_spapr_tce_get_info) | const |  |
| [`CCISS_GETHEARTBEAT`](#cciss_getheartbeat) | const |  |
| [`ATM_RSTADDR`](#atm_rstaddr) | const |  |
| [`NBD_SET_SIZE`](#nbd_set_size) | const |  |
| [`UDF_GETVOLIDENT`](#udf_getvolident) | const |  |
| [`GPIO_V2_LINE_GET_VALUES_IOCTL`](#gpio_v2_line_get_values_ioctl) | const |  |
| [`MGSL_IOCSTXIDLE`](#mgsl_iocstxidle) | const |  |
| [`FSL_HV_IOCTL_SETPROP`](#fsl_hv_ioctl_setprop) | const |  |
| [`BTRFS_IOC_GET_DEV_STATS`](#btrfs_ioc_get_dev_stats) | const |  |
| [`PPRSTATUS`](#pprstatus) | const |  |
| [`MGSL_IOCTXENABLE`](#mgsl_ioctxenable) | const |  |
| [`UDF_GETEASIZE`](#udf_geteasize) | const |  |
| [`NVME_IOCTL_ADMIN64_CMD`](#nvme_ioctl_admin64_cmd) | const |  |
| [`VHOST_SET_OWNER`](#vhost_set_owner) | const |  |
| [`RIO_ALLOC_DMA`](#rio_alloc_dma) | const |  |
| [`RIO_CM_CHAN_ACCEPT`](#rio_cm_chan_accept) | const |  |
| [`I2OHRTGET`](#i2ohrtget) | const |  |
| [`ATM_SETCIRANGE`](#atm_setcirange) | const |  |
| [`HPET_IE_ON`](#hpet_ie_on) | const |  |
| [`PERF_EVENT_IOC_ID`](#perf_event_ioc_id) | const |  |
| [`TUNSETSNDBUF`](#tunsetsndbuf) | const |  |
| [`PTP_PIN_SETFUNC`](#ptp_pin_setfunc) | const |  |
| [`PPPIOCDISCONN`](#pppiocdisconn) | const |  |
| [`VIDIOC_QUERYCTRL`](#vidioc_queryctrl) | const |  |
| [`PPEXCL`](#ppexcl) | const |  |
| [`PCITEST_MSI`](#pcitest_msi) | const |  |
| [`FDWERRORCLR`](#fdwerrorclr) | const |  |
| [`AUTOFS_IOC_FAIL`](#autofs_ioc_fail) | const |  |
| [`USBDEVFS_IOCTL`](#usbdevfs_ioctl) | const |  |
| [`VIDIOC_S_STD`](#vidioc_s_std) | const |  |
| [`F2FS_IOC_RESIZE_FS`](#f2fs_ioc_resize_fs) | const |  |
| [`SONET_SETDIAG`](#sonet_setdiag) | const |  |
| [`BTRFS_IOC_DEFRAG`](#btrfs_ioc_defrag) | const |  |
| [`CCISS_GETDRIVVER`](#cciss_getdrivver) | const |  |
| [`IPMICTL_GET_TIMING_PARMS_CMD`](#ipmictl_get_timing_parms_cmd) | const |  |
| [`HPET_IRQFREQ`](#hpet_irqfreq) | const |  |
| [`ATM_GETESI`](#atm_getesi) | const |  |
| [`CCISS_GETLUNINFO`](#cciss_getluninfo) | const |  |
| [`AUTOFS_DEV_IOCTL_ISMOUNTPOINT`](#autofs_dev_ioctl_ismountpoint) | const |  |
| [`TEE_IOC_SHM_ALLOC`](#tee_ioc_shm_alloc) | const |  |
| [`PERF_EVENT_IOC_SET_BPF`](#perf_event_ioc_set_bpf) | const |  |
| [`UDMABUF_CREATE_LIST`](#udmabuf_create_list) | const |  |
| [`VHOST_SET_LOG_BASE`](#vhost_set_log_base) | const |  |
| [`ZATM_GETPOOL`](#zatm_getpool) | const |  |
| [`BR2684_SETFILT`](#br2684_setfilt) | const |  |
| [`RNDGETPOOL`](#rndgetpool) | const |  |
| [`PPS_GETPARAMS`](#pps_getparams) | const |  |
| [`IOC_PR_RESERVE`](#ioc_pr_reserve) | const |  |
| [`VIDIOC_TRY_DECODER_CMD`](#vidioc_try_decoder_cmd) | const |  |
| [`RIO_CM_CHAN_CLOSE`](#rio_cm_chan_close) | const |  |
| [`VIDIOC_DV_TIMINGS_CAP`](#vidioc_dv_timings_cap) | const |  |
| [`IOCTL_MEI_CONNECT_CLIENT_VTAG`](#ioctl_mei_connect_client_vtag) | const |  |
| [`PMU_IOC_GET_BACKLIGHT`](#pmu_ioc_get_backlight) | const |  |
| [`USBDEVFS_GET_CAPABILITIES`](#usbdevfs_get_capabilities) | const |  |
| [`SCIF_WRITETO`](#scif_writeto) | const |  |
| [`UDF_RELOCATE_BLOCKS`](#udf_relocate_blocks) | const |  |
| [`FSL_HV_IOCTL_PARTITION_RESTART`](#fsl_hv_ioctl_partition_restart) | const |  |
| [`CCISS_REGNEWD`](#cciss_regnewd) | const |  |
| [`FAT_IOCTL_SET_ATTRIBUTES`](#fat_ioctl_set_attributes) | const |  |
| [`VIDIOC_CREATE_BUFS`](#vidioc_create_bufs) | const |  |
| [`CAPI_GET_VERSION`](#capi_get_version) | const |  |
| [`SWITCHTEC_IOCTL_EVENT_SUMMARY`](#switchtec_ioctl_event_summary) | const |  |
| [`VFIO_EEH_PE_OP`](#vfio_eeh_pe_op) | const |  |
| [`FW_CDEV_IOC_CREATE_ISO_CONTEXT`](#fw_cdev_ioc_create_iso_context) | const |  |
| [`F2FS_IOC_RELEASE_COMPRESS_BLOCKS`](#f2fs_ioc_release_compress_blocks) | const |  |
| [`NBD_SET_SIZE_BLOCKS`](#nbd_set_size_blocks) | const |  |
| [`IPMI_BMC_IOCTL_SET_SMS_ATN`](#ipmi_bmc_ioctl_set_sms_atn) | const |  |
| [`ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG`](#aspeed_p2a_ctrl_ioctl_get_memory_config) | const |  |
| [`VIDIOC_S_AUDOUT`](#vidioc_s_audout) | const |  |
| [`VIDIOC_S_FMT`](#vidioc_s_fmt) | const |  |
| [`PPPIOCATTACH`](#pppiocattach) | const |  |
| [`VHOST_GET_VRING_BUSYLOOP_TIMEOUT`](#vhost_get_vring_busyloop_timeout) | const |  |
| [`FS_IOC_MEASURE_VERITY`](#fs_ioc_measure_verity) | const |  |
| [`CCISS_BIG_PASSTHRU`](#cciss_big_passthru) | const |  |
| [`IPMICTL_SET_MY_LUN_CMD`](#ipmictl_set_my_lun_cmd) | const |  |
| [`PCITEST_LEGACY_IRQ`](#pcitest_legacy_irq) | const |  |
| [`USBDEVFS_SUBMITURB`](#usbdevfs_submiturb) | const |  |
| [`AUTOFS_IOC_READY`](#autofs_ioc_ready) | const |  |
| [`BTRFS_IOC_SEND`](#btrfs_ioc_send) | const |  |
| [`VIDIOC_G_EXT_CTRLS`](#vidioc_g_ext_ctrls) | const |  |
| [`JSIOCSBTNMAP`](#jsiocsbtnmap) | const |  |
| [`PPPIOCSFLAGS`](#pppiocsflags) | const |  |
| [`NVRAM_INIT`](#nvram_init) | const |  |
| [`RFKILL_IOCTL_NOINPUT`](#rfkill_ioctl_noinput) | const |  |
| [`BTRFS_IOC_BALANCE`](#btrfs_ioc_balance) | const |  |
| [`FS_IOC_GETFSMAP`](#fs_ioc_getfsmap) | const |  |
| [`IPMICTL_GET_MY_CHANNEL_LUN_CMD`](#ipmictl_get_my_channel_lun_cmd) | const |  |
| [`STP_POLICY_ID_GET`](#stp_policy_id_get) | const |  |
| [`PPSETFLAGS`](#ppsetflags) | const |  |
| [`CEC_ADAP_S_PHYS_ADDR`](#cec_adap_s_phys_addr) | const |  |
| [`ATMTCP_CREATE`](#atmtcp_create) | const |  |
| [`IPMI_BMC_IOCTL_FORCE_ABORT`](#ipmi_bmc_ioctl_force_abort) | const |  |
| [`PPPIOCGXASYNCMAP`](#pppiocgxasyncmap) | const |  |
| [`VHOST_SET_VRING_CALL`](#vhost_set_vring_call) | const |  |
| [`LIRC_GET_FEATURES`](#lirc_get_features) | const |  |
| [`GSMIOC_DISABLE_NET`](#gsmioc_disable_net) | const |  |
| [`AUTOFS_IOC_CATATONIC`](#autofs_ioc_catatonic) | const |  |
| [`NBD_DO_IT`](#nbd_do_it) | const |  |
| [`LIRC_SET_REC_CARRIER_RANGE`](#lirc_set_rec_carrier_range) | const |  |
| [`IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD`](#ipmictl_get_my_channel_address_cmd) | const |  |
| [`EVIOCSCLOCKID`](#eviocsclockid) | const |  |
| [`USBDEVFS_FREE_STREAMS`](#usbdevfs_free_streams) | const |  |
| [`FSI_SCOM_RESET`](#fsi_scom_reset) | const |  |
| [`PMU_IOC_GRAB_BACKLIGHT`](#pmu_ioc_grab_backlight) | const |  |
| [`VIDIOC_SUBDEV_S_FMT`](#vidioc_subdev_s_fmt) | const |  |
| [`FDDEFPRM`](#fddefprm) | const |  |
| [`TEE_IOC_INVOKE`](#tee_ioc_invoke) | const |  |
| [`USBDEVFS_BULK`](#usbdevfs_bulk) | const |  |
| [`SCIF_VWRITETO`](#scif_vwriteto) | const |  |
| [`SONYPI_IOCSBRT`](#sonypi_iocsbrt) | const |  |
| [`BTRFS_IOC_FILE_EXTENT_SAME`](#btrfs_ioc_file_extent_same) | const |  |
| [`RTC_PIE_ON`](#rtc_pie_on) | const |  |
| [`BTRFS_IOC_SCAN_DEV`](#btrfs_ioc_scan_dev) | const |  |
| [`PPPIOCXFERUNIT`](#pppiocxferunit) | const |  |
| [`WDIOC_GETTIMEOUT`](#wdioc_gettimeout) | const |  |
| [`BTRFS_IOC_SET_RECEIVED_SUBVOL`](#btrfs_ioc_set_received_subvol) | const |  |
| [`DFL_FPGA_PORT_ERR_SET_IRQ`](#dfl_fpga_port_err_set_irq) | const |  |
| [`FBIO_WAITFORVSYNC`](#fbio_waitforvsync) | const |  |
| [`RTC_PIE_OFF`](#rtc_pie_off) | const |  |
| [`EVIOCGRAB`](#eviocgrab) | const |  |
| [`PMU_IOC_SET_BACKLIGHT`](#pmu_ioc_set_backlight) | const |  |
| [`EVIOCGREP`](#eviocgrep) | const |  |
| [`PERF_EVENT_IOC_MODIFY_ATTRIBUTES`](#perf_event_ioc_modify_attributes) | const |  |
| [`UFFDIO_CONTINUE`](#uffdio_continue) | const |  |
| [`VDUSE_GET_API_VERSION`](#vduse_get_api_version) | const |  |
| [`RTC_RD_TIME`](#rtc_rd_time) | const |  |
| [`FDMSGOFF`](#fdmsgoff) | const |  |
| [`IPMICTL_REGISTER_FOR_CMD_CHANS`](#ipmictl_register_for_cmd_chans) | const |  |
| [`CAPI_GET_ERRCODE`](#capi_get_errcode) | const |  |
| [`PCITEST_SET_IRQTYPE`](#pcitest_set_irqtype) | const |  |
| [`VIDIOC_SUBDEV_S_EDID`](#vidioc_subdev_s_edid) | const |  |
| [`MATROXFB_SET_OUTPUT_MODE`](#matroxfb_set_output_mode) | const |  |
| [`RIO_DEV_ADD`](#rio_dev_add) | const |  |
| [`VIDIOC_ENUM_FREQ_BANDS`](#vidioc_enum_freq_bands) | const |  |
| [`FBIO_RADEON_SET_MIRROR`](#fbio_radeon_set_mirror) | const |  |
| [`PCITEST_GET_IRQTYPE`](#pcitest_get_irqtype) | const |  |
| [`JSIOCGVERSION`](#jsiocgversion) | const |  |
| [`SONYPI_IOCSBLUE`](#sonypi_iocsblue) | const |  |
| [`SNAPSHOT_PREF_IMAGE_SIZE`](#snapshot_pref_image_size) | const |  |
| [`F2FS_IOC_GET_FEATURES`](#f2fs_ioc_get_features) | const |  |
| [`SCIF_REG`](#scif_reg) | const |  |
| [`NILFS_IOCTL_CLEAN_SEGMENTS`](#nilfs_ioctl_clean_segments) | const |  |
| [`FW_CDEV_IOC_INITIATE_BUS_RESET`](#fw_cdev_ioc_initiate_bus_reset) | const |  |
| [`RIO_WAIT_FOR_ASYNC`](#rio_wait_for_async) | const |  |
| [`VHOST_SET_VRING_NUM`](#vhost_set_vring_num) | const |  |
| [`AUTOFS_DEV_IOCTL_PROTOVER`](#autofs_dev_ioctl_protover) | const |  |
| [`RIO_FREE_DMA`](#rio_free_dma) | const |  |
| [`MGSL_IOCRXENABLE`](#mgsl_iocrxenable) | const |  |
| [`IOCTL_VM_SOCKETS_GET_LOCAL_CID`](#ioctl_vm_sockets_get_local_cid) | const |  |
| [`IPMICTL_SET_TIMING_PARMS_CMD`](#ipmictl_set_timing_parms_cmd) | const |  |
| [`PPPIOCGL2TPSTATS`](#pppiocgl2tpstats) | const |  |
| [`PERF_EVENT_IOC_PERIOD`](#perf_event_ioc_period) | const |  |
| [`PTP_PIN_SETFUNC2`](#ptp_pin_setfunc2) | const |  |
| [`CHIOEXCHANGE`](#chioexchange) | const |  |
| [`NILFS_IOCTL_GET_SUINFO`](#nilfs_ioctl_get_suinfo) | const |  |
| [`CEC_DQEVENT`](#cec_dqevent) | const |  |
| [`UI_SET_SWBIT`](#ui_set_swbit) | const |  |
| [`VHOST_VDPA_SET_CONFIG`](#vhost_vdpa_set_config) | const |  |
| [`TUNSETIFF`](#tunsetiff) | const |  |
| [`CHIOPOSITION`](#chioposition) | const |  |
| [`IPMICTL_SET_MAINTENANCE_MODE_CMD`](#ipmictl_set_maintenance_mode_cmd) | const |  |
| [`BTRFS_IOC_DEFAULT_SUBVOL`](#btrfs_ioc_default_subvol) | const |  |
| [`RIO_UNMAP_OUTBOUND`](#rio_unmap_outbound) | const |  |
| [`CAPI_CLR_FLAGS`](#capi_clr_flags) | const |  |
| [`FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE`](#fw_cdev_ioc_allocate_iso_resource_once) | const |  |
| [`MATROXFB_GET_OUTPUT_CONNECTION`](#matroxfb_get_output_connection) | const |  |
| [`EVIOCSMASK`](#eviocsmask) | const |  |
| [`BTRFS_IOC_FORGET_DEV`](#btrfs_ioc_forget_dev) | const |  |
| [`CXL_MEM_QUERY_COMMANDS`](#cxl_mem_query_commands) | const |  |
| [`CEC_S_MODE`](#cec_s_mode) | const |  |
| [`MGSL_IOCSIF`](#mgsl_iocsif) | const |  |
| [`SWITCHTEC_IOCTL_PFF_TO_PORT`](#switchtec_ioctl_pff_to_port) | const |  |
| [`PPSETMODE`](#ppsetmode) | const |  |
| [`VFIO_DEVICE_SET_IRQS`](#vfio_device_set_irqs) | const |  |
| [`VIDIOC_PREPARE_BUF`](#vidioc_prepare_buf) | const |  |
| [`CEC_ADAP_G_CONNECTOR_INFO`](#cec_adap_g_connector_info) | const |  |
| [`IOC_OPAL_WRITE_SHADOW_MBR`](#ioc_opal_write_shadow_mbr) | const |  |
| [`VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL`](#vidioc_subdev_enum_frame_interval) | const |  |
| [`UDMABUF_CREATE`](#udmabuf_create) | const |  |
| [`SONET_CLRDIAG`](#sonet_clrdiag) | const |  |
| [`PHN_SET_REG`](#phn_set_reg) | const |  |
| [`RNDADDTOENTCNT`](#rndaddtoentcnt) | const |  |
| [`VBG_IOCTL_CHECK_BALLOON`](#vbg_ioctl_check_balloon) | const |  |
| [`VIDIOC_OMAP3ISP_STAT_REQ`](#vidioc_omap3isp_stat_req) | const |  |
| [`PPS_FETCH`](#pps_fetch) | const |  |
| [`RTC_AIE_OFF`](#rtc_aie_off) | const |  |
| [`VFIO_GROUP_SET_CONTAINER`](#vfio_group_set_container) | const |  |
| [`FW_CDEV_IOC_RECEIVE_PHY_PACKETS`](#fw_cdev_ioc_receive_phy_packets) | const |  |
| [`VFIO_IOMMU_SPAPR_TCE_REMOVE`](#vfio_iommu_spapr_tce_remove) | const |  |
| [`VFIO_IOMMU_GET_INFO`](#vfio_iommu_get_info) | const |  |
| [`DM_DEV_SUSPEND`](#dm_dev_suspend) | const |  |
| [`F2FS_IOC_GET_COMPRESS_OPTION`](#f2fs_ioc_get_compress_option) | const |  |
| [`FW_CDEV_IOC_STOP_ISO`](#fw_cdev_ioc_stop_iso) | const |  |
| [`GPIO_V2_GET_LINEINFO_IOCTL`](#gpio_v2_get_lineinfo_ioctl) | const |  |
| [`ATMMPC_CTRL`](#atmmpc_ctrl) | const |  |
| [`PPPIOCSXASYNCMAP`](#pppiocsxasyncmap) | const |  |
| [`CHIOGSTATUS`](#chiogstatus) | const |  |
| [`FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE`](#fw_cdev_ioc_allocate_iso_resource) | const |  |
| [`RIO_MPORT_MAINT_PORT_IDX_GET`](#rio_mport_maint_port_idx_get) | const |  |
| [`CAPI_SET_FLAGS`](#capi_set_flags) | const |  |
| [`VFIO_GROUP_GET_DEVICE_FD`](#vfio_group_get_device_fd) | const |  |
| [`VHOST_SET_MEM_TABLE`](#vhost_set_mem_table) | const |  |
| [`MATROXFB_SET_OUTPUT_CONNECTION`](#matroxfb_set_output_connection) | const |  |
| [`DFL_FPGA_PORT_GET_REGION_INFO`](#dfl_fpga_port_get_region_info) | const |  |
| [`VHOST_GET_FEATURES`](#vhost_get_features) | const |  |
| [`LIRC_GET_REC_RESOLUTION`](#lirc_get_rec_resolution) | const |  |
| [`PACKET_CTRL_CMD`](#packet_ctrl_cmd) | const |  |
| [`LIRC_SET_TRANSMITTER_MASK`](#lirc_set_transmitter_mask) | const |  |
| [`BTRFS_IOC_ADD_DEV`](#btrfs_ioc_add_dev) | const |  |
| [`JSIOCGCORR`](#jsiocgcorr) | const |  |
| [`VIDIOC_G_FMT`](#vidioc_g_fmt) | const |  |
| [`RTC_EPOCH_SET`](#rtc_epoch_set) | const |  |
| [`CAPI_GET_PROFILE`](#capi_get_profile) | const |  |
| [`ATM_GETLOOP`](#atm_getloop) | const |  |
| [`SCIF_LISTEN`](#scif_listen) | const |  |
| [`NBD_CLEAR_QUE`](#nbd_clear_que) | const |  |
| [`F2FS_IOC_MOVE_RANGE`](#f2fs_ioc_move_range) | const |  |
| [`LIRC_GET_LENGTH`](#lirc_get_length) | const |  |
| [`I8K_SET_FAN`](#i8k_set_fan) | const |  |
| [`FDSETMAXERRS`](#fdsetmaxerrs) | const |  |
| [`VIDIOC_SUBDEV_QUERYCAP`](#vidioc_subdev_querycap) | const |  |
| [`SNAPSHOT_SET_SWAP_AREA`](#snapshot_set_swap_area) | const |  |
| [`LIRC_GET_REC_TIMEOUT`](#lirc_get_rec_timeout) | const |  |
| [`EVIOCRMFF`](#eviocrmff) | const |  |
| [`GPIO_GET_LINEEVENT_IOCTL`](#gpio_get_lineevent_ioctl) | const |  |
| [`PPRDATA`](#pprdata) | const |  |
| [`RIO_MPORT_GET_PROPERTIES`](#rio_mport_get_properties) | const |  |
| [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz) | const |  |
| [`GPIO_GET_LINEINFO_IOCTL`](#gpio_get_lineinfo_ioctl) | const |  |
| [`GSMIOC_GETCONF`](#gsmioc_getconf) | const |  |
| [`LIRC_GET_SEND_MODE`](#lirc_get_send_mode) | const |  |
| [`PPPIOCSACTIVE`](#pppiocsactive) | const |  |
| [`SIOCGSTAMPNS_NEW`](#siocgstampns_new) | const |  |
| [`IPMICTL_RECEIVE_MSG`](#ipmictl_receive_msg) | const |  |
| [`LIRC_SET_SEND_DUTY_CYCLE`](#lirc_set_send_duty_cycle) | const |  |
| [`UI_END_FF_ERASE`](#ui_end_ff_erase) | const |  |
| [`SWITCHTEC_IOCTL_FLASH_PART_INFO`](#switchtec_ioctl_flash_part_info) | const |  |
| [`FW_CDEV_IOC_SEND_PHY_PACKET`](#fw_cdev_ioc_send_phy_packet) | const |  |
| [`NBD_SET_FLAGS`](#nbd_set_flags) | const |  |
| [`VFIO_DEVICE_GET_REGION_INFO`](#vfio_device_get_region_info) | const |  |
| [`REISERFS_IOC_UNPACK`](#reiserfs_ioc_unpack) | const |  |
| [`FW_CDEV_IOC_REMOVE_DESCRIPTOR`](#fw_cdev_ioc_remove_descriptor) | const |  |
| [`RIO_SET_EVENT_MASK`](#rio_set_event_mask) | const |  |
| [`SNAPSHOT_ALLOC_SWAP_PAGE`](#snapshot_alloc_swap_page) | const |  |
| [`VDUSE_VQ_INJECT_IRQ`](#vduse_vq_inject_irq) | const |  |
| [`I2OPASSTHRU`](#i2opassthru) | const |  |
| [`IOC_OPAL_SET_PW`](#ioc_opal_set_pw) | const |  |
| [`FSI_SCOM_READ`](#fsi_scom_read) | const |  |
| [`VHOST_VDPA_GET_DEVICE_ID`](#vhost_vdpa_get_device_id) | const |  |
| [`VIDIOC_QBUF`](#vidioc_qbuf) | const |  |
| [`VIDIOC_S_TUNER`](#vidioc_s_tuner) | const |  |
| [`TUNGETVNETHDRSZ`](#tungetvnethdrsz) | const |  |
| [`CAPI_NCCI_GETUNIT`](#capi_ncci_getunit) | const |  |
| [`DFL_FPGA_PORT_UINT_GET_IRQ_NUM`](#dfl_fpga_port_uint_get_irq_num) | const |  |
| [`VIDIOC_OMAP3ISP_STAT_EN`](#vidioc_omap3isp_stat_en) | const |  |
| [`GPIO_V2_LINE_SET_CONFIG_IOCTL`](#gpio_v2_line_set_config_ioctl) | const |  |
| [`TEE_IOC_VERSION`](#tee_ioc_version) | const |  |
| [`VIDIOC_LOG_STATUS`](#vidioc_log_status) | const |  |
| [`IPMICTL_SEND_COMMAND_SETTIME`](#ipmictl_send_command_settime) | const |  |
| [`VHOST_SET_LOG_FD`](#vhost_set_log_fd) | const |  |
| [`SCIF_SEND`](#scif_send) | const |  |
| [`VIDIOC_SUBDEV_G_FMT`](#vidioc_subdev_g_fmt) | const |  |
| [`NS_ADJBUFLEV`](#ns_adjbuflev) | const |  |
| [`VIDIOC_DBG_S_REGISTER`](#vidioc_dbg_s_register) | const |  |
| [`NILFS_IOCTL_RESIZE`](#nilfs_ioctl_resize) | const |  |
| [`PHN_GETREG`](#phn_getreg) | const |  |
| [`I2OSWDL`](#i2oswdl) | const |  |
| [`VBG_IOCTL_VMMDEV_REQUEST_BIG`](#vbg_ioctl_vmmdev_request_big) | const |  |
| [`JSIOCGBUTTONS`](#jsiocgbuttons) | const |  |
| [`VFIO_IOMMU_ENABLE`](#vfio_iommu_enable) | const |  |
| [`DM_DEV_RENAME`](#dm_dev_rename) | const |  |
| [`MEDIA_IOC_SETUP_LINK`](#media_ioc_setup_link) | const |  |
| [`VIDIOC_ENUMOUTPUT`](#vidioc_enumoutput) | const |  |
| [`STP_POLICY_ID_SET`](#stp_policy_id_set) | const |  |
| [`VHOST_VDPA_SET_CONFIG_CALL`](#vhost_vdpa_set_config_call) | const |  |
| [`VIDIOC_SUBDEV_G_CROP`](#vidioc_subdev_g_crop) | const |  |
| [`VIDIOC_S_CROP`](#vidioc_s_crop) | const |  |
| [`WDIOC_GETTEMP`](#wdioc_gettemp) | const |  |
| [`IOC_OPAL_ADD_USR_TO_LR`](#ioc_opal_add_usr_to_lr) | const |  |
| [`UI_SET_LEDBIT`](#ui_set_ledbit) | const |  |
| [`NBD_SET_SOCK`](#nbd_set_sock) | const |  |
| [`BTRFS_IOC_SNAP_DESTROY_V2`](#btrfs_ioc_snap_destroy_v2) | const |  |
| [`HIDIOCGCOLLECTIONINFO`](#hidiocgcollectioninfo) | const |  |
| [`I2OSWUL`](#i2oswul) | const |  |
| [`IOCTL_MEI_NOTIFY_GET`](#ioctl_mei_notify_get) | const |  |
| [`FDFMTTRK`](#fdfmttrk) | const |  |
| [`MMTIMER_GETBITS`](#mmtimer_getbits) | const |  |
| [`VIDIOC_ENUMSTD`](#vidioc_enumstd) | const |  |
| [`VHOST_GET_VRING_BASE`](#vhost_get_vring_base) | const |  |
| [`VFIO_DEVICE_IOEVENTFD`](#vfio_device_ioeventfd) | const |  |
| [`ATMARP_SETENTRY`](#atmarp_setentry) | const |  |
| [`CCISS_REVALIDVOLS`](#cciss_revalidvols) | const |  |
| [`MGSL_IOCLOOPTXDONE`](#mgsl_ioclooptxdone) | const |  |
| [`RTC_VL_READ`](#rtc_vl_read) | const |  |
| [`ND_IOCTL_ARS_STATUS`](#nd_ioctl_ars_status) | const |  |
| [`RIO_DEV_DEL`](#rio_dev_del) | const |  |
| [`VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES`](#vbg_ioctl_acquire_guest_capabilities) | const |  |
| [`VIDIOC_SUBDEV_DV_TIMINGS_CAP`](#vidioc_subdev_dv_timings_cap) | const |  |
| [`SONYPI_IOCSFAN`](#sonypi_iocsfan) | const |  |
| [`SPIOCSTYPE`](#spiocstype) | const |  |
| [`IPMICTL_REGISTER_FOR_CMD`](#ipmictl_register_for_cmd) | const |  |
| [`I8K_GET_FAN`](#i8k_get_fan) | const |  |
| [`TUNGETVNETBE`](#tungetvnetbe) | const |  |
| [`AUTOFS_DEV_IOCTL_FAIL`](#autofs_dev_ioctl_fail) | const |  |
| [`UI_END_FF_UPLOAD`](#ui_end_ff_upload) | const |  |
| [`TOSH_SMM`](#tosh_smm) | const |  |
| [`SONYPI_IOCGBAT2REM`](#sonypi_iocgbat2rem) | const |  |
| [`F2FS_IOC_GET_COMPRESS_BLOCKS`](#f2fs_ioc_get_compress_blocks) | const |  |
| [`PPPIOCSNPMODE`](#pppiocsnpmode) | const |  |
| [`USBDEVFS_CONTROL`](#usbdevfs_control) | const |  |
| [`HIDIOCGUSAGE`](#hidiocgusage) | const |  |
| [`TUNSETTXFILTER`](#tunsettxfilter) | const |  |
| [`TUNGETVNETLE`](#tungetvnetle) | const |  |
| [`VIDIOC_ENUM_DV_TIMINGS`](#vidioc_enum_dv_timings) | const |  |
| [`BTRFS_IOC_INO_PATHS`](#btrfs_ioc_ino_paths) | const |  |
| [`MGSL_IOCGXSYNC`](#mgsl_iocgxsync) | const |  |
| [`HIDIOCGFIELDINFO`](#hidiocgfieldinfo) | const |  |
| [`VIDIOC_SUBDEV_G_STD`](#vidioc_subdev_g_std) | const |  |
| [`I2OVALIDATE`](#i2ovalidate) | const |  |
| [`VIDIOC_TRY_ENCODER_CMD`](#vidioc_try_encoder_cmd) | const |  |
| [`NILFS_IOCTL_GET_CPINFO`](#nilfs_ioctl_get_cpinfo) | const |  |
| [`VIDIOC_G_FREQUENCY`](#vidioc_g_frequency) | const |  |
| [`VFAT_IOCTL_READDIR_SHORT`](#vfat_ioctl_readdir_short) | const |  |
| [`ND_IOCTL_GET_CONFIG_DATA`](#nd_ioctl_get_config_data) | const |  |
| [`F2FS_IOC_RESERVE_COMPRESS_BLOCKS`](#f2fs_ioc_reserve_compress_blocks) | const |  |
| [`FDGETDRVSTAT`](#fdgetdrvstat) | const |  |
| [`SYNC_IOC_MERGE`](#sync_ioc_merge) | const |  |
| [`VIDIOC_S_DV_TIMINGS`](#vidioc_s_dv_timings) | const |  |
| [`PPPIOCBRIDGECHAN`](#pppiocbridgechan) | const |  |
| [`LIRC_SET_SEND_MODE`](#lirc_set_send_mode) | const |  |
| [`RIO_ENABLE_PORTWRITE_RANGE`](#rio_enable_portwrite_range) | const |  |
| [`ATM_GETTYPE`](#atm_gettype) | const |  |
| [`PHN_GETREGS`](#phn_getregs) | const |  |
| [`FDSETEMSGTRESH`](#fdsetemsgtresh) | const |  |
| [`NILFS_IOCTL_GET_VINFO`](#nilfs_ioctl_get_vinfo) | const |  |
| [`MGSL_IOCWAITEVENT`](#mgsl_iocwaitevent) | const |  |
| [`CAPI_INSTALLED`](#capi_installed) | const |  |
| [`EVIOCGMASK`](#eviocgmask) | const |  |
| [`BTRFS_IOC_SUBVOL_GETFLAGS`](#btrfs_ioc_subvol_getflags) | const |  |
| [`FSL_HV_IOCTL_PARTITION_GET_STATUS`](#fsl_hv_ioctl_partition_get_status) | const |  |
| [`MEDIA_IOC_ENUM_ENTITIES`](#media_ioc_enum_entities) | const |  |
| [`GSMIOC_GETFIRST`](#gsmioc_getfirst) | const |  |
| [`FW_CDEV_IOC_FLUSH_ISO`](#fw_cdev_ioc_flush_iso) | const |  |
| [`VIDIOC_DBG_G_CHIP_INFO`](#vidioc_dbg_g_chip_info) | const |  |
| [`F2FS_IOC_RELEASE_VOLATILE_WRITE`](#f2fs_ioc_release_volatile_write) | const |  |
| [`CAPI_GET_SERIAL`](#capi_get_serial) | const |  |
| [`FDSETDRVPRM`](#fdsetdrvprm) | const |  |
| [`IOC_OPAL_SAVE`](#ioc_opal_save) | const |  |
| [`VIDIOC_G_DV_TIMINGS`](#vidioc_g_dv_timings) | const |  |
| [`TUNSETIFINDEX`](#tunsetifindex) | const |  |
| [`CCISS_SETINTINFO`](#cciss_setintinfo) | const |  |
| [`RTC_VL_CLR`](#rtc_vl_clr) | const |  |
| [`VIDIOC_REQBUFS`](#vidioc_reqbufs) | const |  |
| [`USBDEVFS_REAPURBNDELAY32`](#usbdevfs_reapurbndelay32) | const |  |
| [`TEE_IOC_SHM_REGISTER`](#tee_ioc_shm_register) | const |  |
| [`USBDEVFS_SETCONFIGURATION`](#usbdevfs_setconfiguration) | const |  |
| [`CCISS_GETNODENAME`](#cciss_getnodename) | const |  |
| [`VIDIOC_SUBDEV_S_FRAME_INTERVAL`](#vidioc_subdev_s_frame_interval) | const |  |
| [`VIDIOC_ENUM_FRAMESIZES`](#vidioc_enum_framesizes) | const |  |
| [`VFIO_DEVICE_PCI_HOT_RESET`](#vfio_device_pci_hot_reset) | const |  |
| [`FW_CDEV_IOC_SEND_BROADCAST_REQUEST`](#fw_cdev_ioc_send_broadcast_request) | const |  |
| [`LPSETTIMEOUT_NEW`](#lpsettimeout_new) | const |  |
| [`RIO_CM_MPORT_GET_LIST`](#rio_cm_mport_get_list) | const |  |
| [`FW_CDEV_IOC_QUEUE_ISO`](#fw_cdev_ioc_queue_iso) | const |  |
| [`FDRAWCMD`](#fdrawcmd) | const |  |
| [`SCIF_UNREG`](#scif_unreg) | const |  |
| [`PPPIOCGIDLE64`](#pppiocgidle64) | const |  |
| [`USBDEVFS_RELEASEINTERFACE`](#usbdevfs_releaseinterface) | const |  |
| [`VIDIOC_CROPCAP`](#vidioc_cropcap) | const |  |
| [`DFL_FPGA_PORT_GET_INFO`](#dfl_fpga_port_get_info) | const |  |
| [`PHN_SET_REGS`](#phn_set_regs) | const |  |
| [`ATMLEC_DATA`](#atmlec_data) | const |  |
| [`PPPOEIOCDFWD`](#pppoeiocdfwd) | const |  |
| [`VIDIOC_S_SELECTION`](#vidioc_s_selection) | const |  |
| [`SNAPSHOT_FREE_SWAP_PAGES`](#snapshot_free_swap_pages) | const |  |
| [`BTRFS_IOC_LOGICAL_INO`](#btrfs_ioc_logical_ino) | const |  |
| [`VIDIOC_S_CTRL`](#vidioc_s_ctrl) | const |  |
| [`ZATM_SETPOOL`](#zatm_setpool) | const |  |
| [`MTIOCPOS`](#mtiocpos) | const |  |
| [`PMU_IOC_SLEEP`](#pmu_ioc_sleep) | const |  |
| [`AUTOFS_DEV_IOCTL_PROTOSUBVER`](#autofs_dev_ioctl_protosubver) | const |  |
| [`VBG_IOCTL_CHANGE_FILTER_MASK`](#vbg_ioctl_change_filter_mask) | const |  |
| [`NILFS_IOCTL_GET_SUSTAT`](#nilfs_ioctl_get_sustat) | const |  |
| [`VIDIOC_QUERYCAP`](#vidioc_querycap) | const |  |
| [`HPET_INFO`](#hpet_info) | const |  |
| [`VIDIOC_AM437X_CCDC_CFG`](#vidioc_am437x_ccdc_cfg) | const |  |
| [`DM_LIST_DEVICES`](#dm_list_devices) | const |  |
| [`TUNSETOWNER`](#tunsetowner) | const |  |
| [`VBG_IOCTL_CHANGE_GUEST_CAPABILITIES`](#vbg_ioctl_change_guest_capabilities) | const |  |
| [`RNDADDENTROPY`](#rndaddentropy) | const |  |
| [`USBDEVFS_RESET`](#usbdevfs_reset) | const |  |
| [`BTRFS_IOC_SUBVOL_CREATE`](#btrfs_ioc_subvol_create) | const |  |
| [`USBDEVFS_FORBID_SUSPEND`](#usbdevfs_forbid_suspend) | const |  |
| [`FDGETDRVTYP`](#fdgetdrvtyp) | const |  |
| [`PPWCONTROL`](#ppwcontrol) | const |  |
| [`VIDIOC_ENUM_FRAMEINTERVALS`](#vidioc_enum_frameintervals) | const |  |
| [`KCOV_DISABLE`](#kcov_disable) | const |  |
| [`IOC_OPAL_ACTIVATE_LSP`](#ioc_opal_activate_lsp) | const |  |
| [`VHOST_VDPA_GET_IOVA_RANGE`](#vhost_vdpa_get_iova_range) | const |  |
| [`PPPIOCSPASS`](#pppiocspass) | const |  |
| [`RIO_CM_CHAN_CONNECT`](#rio_cm_chan_connect) | const |  |
| [`I2OSWDEL`](#i2oswdel) | const |  |
| [`FS_IOC_SET_ENCRYPTION_POLICY`](#fs_ioc_set_encryption_policy) | const |  |
| [`IOC_OPAL_MBR_DONE`](#ioc_opal_mbr_done) | const |  |
| [`PPPIOCSMAXCID`](#pppiocsmaxcid) | const |  |
| [`PPSETPHASE`](#ppsetphase) | const |  |
| [`VHOST_VDPA_SET_VRING_ENABLE`](#vhost_vdpa_set_vring_enable) | const |  |
| [`USBDEVFS_GET_SPEED`](#usbdevfs_get_speed) | const |  |
| [`SONET_GETFRAMING`](#sonet_getframing) | const |  |
| [`VIDIOC_QUERYBUF`](#vidioc_querybuf) | const |  |
| [`VIDIOC_S_EDID`](#vidioc_s_edid) | const |  |
| [`BTRFS_IOC_QGROUP_ASSIGN`](#btrfs_ioc_qgroup_assign) | const |  |
| [`PPS_GETCAP`](#pps_getcap) | const |  |
| [`SNAPSHOT_PLATFORM_SUPPORT`](#snapshot_platform_support) | const |  |
| [`LIRC_SET_REC_TIMEOUT_REPORTS`](#lirc_set_rec_timeout_reports) | const |  |
| [`SCIF_GET_NODEIDS`](#scif_get_nodeids) | const |  |
| [`NBD_DISCONNECT`](#nbd_disconnect) | const |  |
| [`VIDIOC_SUBDEV_G_FRAME_INTERVAL`](#vidioc_subdev_g_frame_interval) | const |  |
| [`VFIO_IOMMU_DISABLE`](#vfio_iommu_disable) | const |  |
| [`SNAPSHOT_CREATE_IMAGE`](#snapshot_create_image) | const |  |
| [`SNAPSHOT_POWER_OFF`](#snapshot_power_off) | const |  |
| [`APM_IOC_STANDBY`](#apm_ioc_standby) | const |  |
| [`PPPIOCGUNIT`](#pppiocgunit) | const |  |
| [`AUTOFS_IOC_EXPIRE_MULTI`](#autofs_ioc_expire_multi) | const |  |
| [`SCIF_BIND`](#scif_bind) | const |  |
| [`IOC_WATCH_QUEUE_SET_SIZE`](#ioc_watch_queue_set_size) | const |  |
| [`NILFS_IOCTL_CHANGE_CPMODE`](#nilfs_ioctl_change_cpmode) | const |  |
| [`IOC_OPAL_LOCK_UNLOCK`](#ioc_opal_lock_unlock) | const |  |
| [`F2FS_IOC_SET_PIN_FILE`](#f2fs_ioc_set_pin_file) | const |  |
| [`PPPIOCGRASYNCMAP`](#pppiocgrasyncmap) | const |  |
| [`MMTIMER_MMAPAVAIL`](#mmtimer_mmapavail) | const |  |
| [`I2OPASSTHRU32`](#i2opassthru32) | const |  |
| [`DFL_FPGA_FME_PORT_RELEASE`](#dfl_fpga_fme_port_release) | const |  |
| [`VIDIOC_SUBDEV_QUERY_DV_TIMINGS`](#vidioc_subdev_query_dv_timings) | const |  |
| [`UI_SET_SNDBIT`](#ui_set_sndbit) | const |  |
| [`VIDIOC_G_AUDOUT`](#vidioc_g_audout) | const |  |
| [`RTC_PLL_SET`](#rtc_pll_set) | const |  |
| [`VIDIOC_ENUMAUDIO`](#vidioc_enumaudio) | const |  |
| [`AUTOFS_DEV_IOCTL_TIMEOUT`](#autofs_dev_ioctl_timeout) | const |  |
| [`VBG_IOCTL_DRIVER_VERSION_INFO`](#vbg_ioctl_driver_version_info) | const |  |
| [`VHOST_SCSI_GET_EVENTS_MISSED`](#vhost_scsi_get_events_missed) | const |  |
| [`VHOST_SET_VRING_ADDR`](#vhost_set_vring_addr) | const |  |
| [`VDUSE_CREATE_DEV`](#vduse_create_dev) | const |  |
| [`FDFLUSH`](#fdflush) | const |  |
| [`VBG_IOCTL_WAIT_FOR_EVENTS`](#vbg_ioctl_wait_for_events) | const |  |
| [`DFL_FPGA_FME_ERR_SET_IRQ`](#dfl_fpga_fme_err_set_irq) | const |  |
| [`F2FS_IOC_GET_PIN_FILE`](#f2fs_ioc_get_pin_file) | const |  |
| [`SCIF_CONNECT`](#scif_connect) | const |  |
| [`BLKREPORTZONE`](#blkreportzone) | const |  |
| [`AUTOFS_IOC_ASKUMOUNT`](#autofs_ioc_askumount) | const |  |
| [`ATM_ADDPARTY`](#atm_addparty) | const |  |
| [`FDSETPRM`](#fdsetprm) | const |  |
| [`ATM_GETSTATZ`](#atm_getstatz) | const |  |
| [`ISST_IF_MSR_COMMAND`](#isst_if_msr_command) | const |  |
| [`BTRFS_IOC_GET_SUBVOL_INFO`](#btrfs_ioc_get_subvol_info) | const |  |
| [`VIDIOC_UNSUBSCRIBE_EVENT`](#vidioc_unsubscribe_event) | const |  |
| [`SEV_ISSUE_CMD`](#sev_issue_cmd) | const |  |
| [`GPIOHANDLE_SET_LINE_VALUES_IOCTL`](#gpiohandle_set_line_values_ioctl) | const |  |
| [`PCITEST_COPY`](#pcitest_copy) | const |  |
| [`IPMICTL_GET_MY_ADDRESS_CMD`](#ipmictl_get_my_address_cmd) | const |  |
| [`CHIOGPICKER`](#chiogpicker) | const |  |
| [`CAPI_NCCI_OPENCOUNT`](#capi_ncci_opencount) | const |  |
| [`CXL_MEM_SEND_COMMAND`](#cxl_mem_send_command) | const |  |
| [`PERF_EVENT_IOC_SET_FILTER`](#perf_event_ioc_set_filter) | const |  |
| [`IOC_OPAL_REVERT_TPR`](#ioc_opal_revert_tpr) | const |  |
| [`CHIOGVPARAMS`](#chiogvparams) | const |  |
| [`PTP_PEROUT_REQUEST`](#ptp_perout_request) | const |  |
| [`FSI_SCOM_CHECK`](#fsi_scom_check) | const |  |
| [`RTC_IRQP_READ`](#rtc_irqp_read) | const |  |
| [`RIO_MPORT_MAINT_READ_LOCAL`](#rio_mport_maint_read_local) | const |  |
| [`HIDIOCGRDESCSIZE`](#hidiocgrdescsize) | const |  |
| [`UI_GET_VERSION`](#ui_get_version) | const |  |
| [`NILFS_IOCTL_GET_CPSTAT`](#nilfs_ioctl_get_cpstat) | const |  |
| [`CCISS_GETBUSTYPES`](#cciss_getbustypes) | const |  |
| [`VFIO_IOMMU_SPAPR_TCE_CREATE`](#vfio_iommu_spapr_tce_create) | const |  |
| [`VIDIOC_EXPBUF`](#vidioc_expbuf) | const |  |
| [`UI_SET_RELBIT`](#ui_set_relbit) | const |  |
| [`VFIO_SET_IOMMU`](#vfio_set_iommu) | const |  |
| [`VIDIOC_S_MODULATOR`](#vidioc_s_modulator) | const |  |
| [`TUNGETFILTER`](#tungetfilter) | const |  |
| [`CCISS_SETNODENAME`](#cciss_setnodename) | const |  |
| [`FBIO_GETCONTROL2`](#fbio_getcontrol2) | const |  |
| [`TUNSETDEBUG`](#tunsetdebug) | const |  |
| [`DM_DEV_REMOVE`](#dm_dev_remove) | const |  |
| [`HIDIOCSUSAGES`](#hidiocsusages) | const |  |
| [`FS_IOC_ADD_ENCRYPTION_KEY`](#fs_ioc_add_encryption_key) | const |  |
| [`FBIOGET_VBLANK`](#fbioget_vblank) | const |  |
| [`ATM_GETSTAT`](#atm_getstat) | const |  |
| [`VIDIOC_G_JPEGCOMP`](#vidioc_g_jpegcomp) | const |  |
| [`TUNATTACHFILTER`](#tunattachfilter) | const |  |
| [`UI_SET_ABSBIT`](#ui_set_absbit) | const |  |
| [`DFL_FPGA_PORT_ERR_GET_IRQ_NUM`](#dfl_fpga_port_err_get_irq_num) | const |  |
| [`USBDEVFS_REAPURB32`](#usbdevfs_reapurb32) | const |  |
| [`BTRFS_IOC_TRANS_END`](#btrfs_ioc_trans_end) | const |  |
| [`CAPI_REGISTER`](#capi_register) | const |  |
| [`F2FS_IOC_COMPRESS_FILE`](#f2fs_ioc_compress_file) | const |  |
| [`USBDEVFS_DISCARDURB`](#usbdevfs_discardurb) | const |  |
| [`HE_GET_REG`](#he_get_reg) | const |  |
| [`ATM_SETLOOP`](#atm_setloop) | const |  |
| [`ATMSIGD_CTRL`](#atmsigd_ctrl) | const |  |
| [`CIOC_KERNEL_VERSION`](#cioc_kernel_version) | const |  |
| [`BTRFS_IOC_CLONE_RANGE`](#btrfs_ioc_clone_range) | const |  |
| [`SNAPSHOT_UNFREEZE`](#snapshot_unfreeze) | const |  |
| [`F2FS_IOC_START_VOLATILE_WRITE`](#f2fs_ioc_start_volatile_write) | const |  |
| [`PMU_IOC_HAS_ADB`](#pmu_ioc_has_adb) | const |  |
| [`I2OGETIOPS`](#i2ogetiops) | const |  |
| [`VIDIOC_S_FBUF`](#vidioc_s_fbuf) | const |  |
| [`PPRCONTROL`](#pprcontrol) | const |  |
| [`CHIOSPICKER`](#chiospicker) | const |  |
| [`VFIO_IOMMU_SPAPR_REGISTER_MEMORY`](#vfio_iommu_spapr_register_memory) | const |  |
| [`TUNGETSNDBUF`](#tungetsndbuf) | const |  |
| [`GSMIOC_SETCONF`](#gsmioc_setconf) | const |  |
| [`IOC_PR_PREEMPT`](#ioc_pr_preempt) | const |  |
| [`KCOV_INIT_TRACE`](#kcov_init_trace) | const |  |
| [`SONYPI_IOCGBAT1CAP`](#sonypi_iocgbat1cap) | const |  |
| [`SWITCHTEC_IOCTL_FLASH_INFO`](#switchtec_ioctl_flash_info) | const |  |
| [`MTIOCTOP`](#mtioctop) | const |  |
| [`VHOST_VDPA_SET_STATUS`](#vhost_vdpa_set_status) | const |  |
| [`VHOST_SCSI_SET_EVENTS_MISSED`](#vhost_scsi_set_events_missed) | const |  |
| [`VFIO_IOMMU_DIRTY_PAGES`](#vfio_iommu_dirty_pages) | const |  |
| [`BTRFS_IOC_SCRUB_PROGRESS`](#btrfs_ioc_scrub_progress) | const |  |
| [`PPPIOCGMRU`](#pppiocgmru) | const |  |
| [`BTRFS_IOC_DEV_REPLACE`](#btrfs_ioc_dev_replace) | const |  |
| [`PPPIOCGFLAGS`](#pppiocgflags) | const |  |
| [`NILFS_IOCTL_SET_SUINFO`](#nilfs_ioctl_set_suinfo) | const |  |
| [`FW_CDEV_IOC_GET_CYCLE_TIMER2`](#fw_cdev_ioc_get_cycle_timer2) | const |  |
| [`ATM_DELLECSADDR`](#atm_dellecsaddr) | const |  |
| [`FW_CDEV_IOC_GET_SPEED`](#fw_cdev_ioc_get_speed) | const |  |
| [`PPPIOCGIDLE32`](#pppiocgidle32) | const |  |
| [`VFIO_DEVICE_RESET`](#vfio_device_reset) | const |  |
| [`GPIO_GET_LINEINFO_UNWATCH_IOCTL`](#gpio_get_lineinfo_unwatch_ioctl) | const |  |
| [`WDIOC_GETSTATUS`](#wdioc_getstatus) | const |  |
| [`BTRFS_IOC_SET_FEATURES`](#btrfs_ioc_set_features) | const |  |
| [`IOCTL_MEI_CONNECT_CLIENT`](#ioctl_mei_connect_client) | const |  |
| [`VIDIOC_OMAP3ISP_AEWB_CFG`](#vidioc_omap3isp_aewb_cfg) | const |  |
| [`PCITEST_READ`](#pcitest_read) | const |  |
| [`VFIO_GROUP_GET_STATUS`](#vfio_group_get_status) | const |  |
| [`MATROXFB_GET_ALL_OUTPUTS`](#matroxfb_get_all_outputs) | const |  |
| [`USBDEVFS_CLEAR_HALT`](#usbdevfs_clear_halt) | const |  |
| [`VIDIOC_DECODER_CMD`](#vidioc_decoder_cmd) | const |  |
| [`VIDIOC_G_AUDIO`](#vidioc_g_audio) | const |  |
| [`CCISS_RESCANDISK`](#cciss_rescandisk) | const |  |
| [`RIO_DISABLE_PORTWRITE_RANGE`](#rio_disable_portwrite_range) | const |  |
| [`IOC_OPAL_SECURE_ERASE_LR`](#ioc_opal_secure_erase_lr) | const |  |
| [`USBDEVFS_REAPURB`](#usbdevfs_reapurb) | const |  |
| [`DFL_FPGA_CHECK_EXTENSION`](#dfl_fpga_check_extension) | const |  |
| [`AUTOFS_IOC_PROTOVER`](#autofs_ioc_protover) | const |  |
| [`FSL_HV_IOCTL_MEMCPY`](#fsl_hv_ioctl_memcpy) | const |  |
| [`BTRFS_IOC_GET_FEATURES`](#btrfs_ioc_get_features) | const |  |
| [`PCITEST_MSIX`](#pcitest_msix) | const |  |
| [`BTRFS_IOC_DEFRAG_RANGE`](#btrfs_ioc_defrag_range) | const |  |
| [`UI_BEGIN_FF_ERASE`](#ui_begin_ff_erase) | const |  |
| [`DM_GET_TARGET_VERSION`](#dm_get_target_version) | const |  |
| [`PPPIOCGIDLE`](#pppiocgidle) | const |  |
| [`NVRAM_SETCKS`](#nvram_setcks) | const |  |
| [`WDIOC_GETSUPPORT`](#wdioc_getsupport) | const |  |
| [`GSMIOC_ENABLE_NET`](#gsmioc_enable_net) | const |  |
| [`GPIO_GET_CHIPINFO_IOCTL`](#gpio_get_chipinfo_ioctl) | const |  |
| [`NE_ADD_VCPU`](#ne_add_vcpu) | const |  |
| [`EVIOCSKEYCODE_V2`](#eviocskeycode_v2) | const |  |
| [`PTP_SYS_OFFSET_EXTENDED2`](#ptp_sys_offset_extended2) | const |  |
| [`SCIF_FENCE_WAIT`](#scif_fence_wait) | const |  |
| [`RIO_TRANSFER`](#rio_transfer) | const |  |
| [`FSL_HV_IOCTL_DOORBELL`](#fsl_hv_ioctl_doorbell) | const |  |
| [`RIO_MPORT_MAINT_WRITE_LOCAL`](#rio_mport_maint_write_local) | const |  |
| [`I2OEVTREG`](#i2oevtreg) | const |  |
| [`I2OPARMGET`](#i2oparmget) | const |  |
| [`EVIOCGID`](#eviocgid) | const |  |
| [`BTRFS_IOC_QGROUP_CREATE`](#btrfs_ioc_qgroup_create) | const |  |
| [`AUTOFS_DEV_IOCTL_SETPIPEFD`](#autofs_dev_ioctl_setpipefd) | const |  |
| [`VIDIOC_S_PARM`](#vidioc_s_parm) | const |  |
| [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf) | const |  |
| [`ATM_GETNAMES`](#atm_getnames) | const |  |
| [`VIDIOC_QUERYMENU`](#vidioc_querymenu) | const |  |
| [`DFL_FPGA_PORT_DMA_UNMAP`](#dfl_fpga_port_dma_unmap) | const |  |
| [`I2OLCTGET`](#i2olctget) | const |  |
| [`FS_IOC_GET_ENCRYPTION_PWSALT`](#fs_ioc_get_encryption_pwsalt) | const |  |
| [`NS_SETBUFLEV`](#ns_setbuflev) | const |  |
| [`BLKCLOSEZONE`](#blkclosezone) | const |  |
| [`SONET_GETFRSENSE`](#sonet_getfrsense) | const |  |
| [`UI_SET_EVBIT`](#ui_set_evbit) | const |  |
| [`DM_LIST_VERSIONS`](#dm_list_versions) | const |  |
| [`HIDIOCGSTRING`](#hidiocgstring) | const |  |
| [`PPPIOCATTCHAN`](#pppiocattchan) | const |  |
| [`VDUSE_DEV_SET_CONFIG`](#vduse_dev_set_config) | const |  |
| [`TUNGETFEATURES`](#tungetfeatures) | const |  |
| [`VFIO_GROUP_UNSET_CONTAINER`](#vfio_group_unset_container) | const |  |
| [`IPMICTL_SET_MY_ADDRESS_CMD`](#ipmictl_set_my_address_cmd) | const |  |
| [`CCISS_REGNEWDISK`](#cciss_regnewdisk) | const |  |
| [`VIDIOC_QUERY_DV_TIMINGS`](#vidioc_query_dv_timings) | const |  |
| [`PHN_SETREGS`](#phn_setregs) | const |  |
| [`FAT_IOCTL_GET_ATTRIBUTES`](#fat_ioctl_get_attributes) | const |  |
| [`FSL_MC_SEND_MC_COMMAND`](#fsl_mc_send_mc_command) | const |  |
| [`TUNGETIFF`](#tungetiff) | const |  |
| [`PTP_CLOCK_GETCAPS2`](#ptp_clock_getcaps2) | const |  |
| [`BTRFS_IOC_RESIZE`](#btrfs_ioc_resize) | const |  |
| [`VHOST_SET_VRING_ENDIAN`](#vhost_set_vring_endian) | const |  |
| [`PPS_KC_BIND`](#pps_kc_bind) | const |  |
| [`F2FS_IOC_WRITE_CHECKPOINT`](#f2fs_ioc_write_checkpoint) | const |  |
| [`UI_SET_FFBIT`](#ui_set_ffbit) | const |  |
| [`IPMICTL_GET_MY_LUN_CMD`](#ipmictl_get_my_lun_cmd) | const |  |
| [`CEC_ADAP_G_PHYS_ADDR`](#cec_adap_g_phys_addr) | const |  |
| [`CEC_G_MODE`](#cec_g_mode) | const |  |
| [`USBDEVFS_RESETEP`](#usbdevfs_resetep) | const |  |
| [`MEDIA_REQUEST_IOC_QUEUE`](#media_request_ioc_queue) | const |  |
| [`USBDEVFS_ALLOC_STREAMS`](#usbdevfs_alloc_streams) | const |  |
| [`MGSL_IOCSXCTRL`](#mgsl_iocsxctrl) | const |  |
| [`MEDIA_IOC_G_TOPOLOGY`](#media_ioc_g_topology) | const |  |
| [`PPPIOCUNBRIDGECHAN`](#pppiocunbridgechan) | const |  |
| [`F2FS_IOC_COMMIT_ATOMIC_WRITE`](#f2fs_ioc_commit_atomic_write) | const |  |
| [`ISST_IF_GET_PLATFORM_INFO`](#isst_if_get_platform_info) | const |  |
| [`SCIF_FENCE_MARK`](#scif_fence_mark) | const |  |
| [`USBDEVFS_RELEASE_PORT`](#usbdevfs_release_port) | const |  |
| [`VFIO_CHECK_EXTENSION`](#vfio_check_extension) | const |  |
| [`BTRFS_IOC_QGROUP_LIMIT`](#btrfs_ioc_qgroup_limit) | const |  |
| [`FAT_IOCTL_GET_VOLUME_ID`](#fat_ioctl_get_volume_id) | const |  |
| [`UI_SET_PHYS`](#ui_set_phys) | const |  |
| [`FDWERRORGET`](#fdwerrorget) | const |  |
| [`VIDIOC_SUBDEV_G_EDID`](#vidioc_subdev_g_edid) | const |  |
| [`MGSL_IOCGSTATS`](#mgsl_iocgstats) | const |  |
| [`RPROC_SET_SHUTDOWN_ON_RELEASE`](#rproc_set_shutdown_on_release) | const |  |
| [`SIOCGSTAMP_NEW`](#siocgstamp_new) | const |  |
| [`RTC_WKALM_RD`](#rtc_wkalm_rd) | const |  |
| [`PHN_GET_REG`](#phn_get_reg) | const |  |
| [`DELL_WMI_SMBIOS_CMD`](#dell_wmi_smbios_cmd) | const |  |
| [`PHN_NOT_OH`](#phn_not_oh) | const |  |
| [`PPGETMODES`](#ppgetmodes) | const |  |
| [`CHIOGPARAMS`](#chiogparams) | const |  |
| [`VFIO_DEVICE_GET_GFX_DMABUF`](#vfio_device_get_gfx_dmabuf) | const |  |
| [`VHOST_SET_VRING_BUSYLOOP_TIMEOUT`](#vhost_set_vring_busyloop_timeout) | const |  |
| [`VIDIOC_SUBDEV_G_SELECTION`](#vidioc_subdev_g_selection) | const |  |
| [`BTRFS_IOC_RM_DEV_V2`](#btrfs_ioc_rm_dev_v2) | const |  |
| [`MGSL_IOCWAITGPIO`](#mgsl_iocwaitgpio) | const |  |
| [`PMU_IOC_CAN_SLEEP`](#pmu_ioc_can_sleep) | const |  |
| [`KCOV_ENABLE`](#kcov_enable) | const |  |
| [`BTRFS_IOC_CLONE`](#btrfs_ioc_clone) | const |  |
| [`F2FS_IOC_DEFRAGMENT`](#f2fs_ioc_defragment) | const |  |
| [`FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE`](#fw_cdev_ioc_deallocate_iso_resource) | const |  |
| [`AGPIOC_ALLOCATE`](#agpioc_allocate) | const |  |
| [`NE_SET_USER_MEMORY_REGION`](#ne_set_user_memory_region) | const |  |
| [`MGSL_IOCTXABORT`](#mgsl_ioctxabort) | const |  |
| [`MGSL_IOCSGPIO`](#mgsl_iocsgpio) | const |  |
| [`LIRC_SET_REC_CARRIER`](#lirc_set_rec_carrier) | const |  |
| [`F2FS_IOC_FLUSH_DEVICE`](#f2fs_ioc_flush_device) | const |  |
| [`SNAPSHOT_ATOMIC_RESTORE`](#snapshot_atomic_restore) | const |  |
| [`RTC_UIE_OFF`](#rtc_uie_off) | const |  |
| [`BT_BMC_IOCTL_SMS_ATN`](#bt_bmc_ioctl_sms_atn) | const |  |
| [`NVME_IOCTL_ID`](#nvme_ioctl_id) | const |  |
| [`NE_START_ENCLAVE`](#ne_start_enclave) | const |  |
| [`VIDIOC_STREAMON`](#vidioc_streamon) | const |  |
| [`FDPOLLDRVSTAT`](#fdpolldrvstat) | const |  |
| [`AUTOFS_DEV_IOCTL_READY`](#autofs_dev_ioctl_ready) | const |  |
| [`VIDIOC_ENUMAUDOUT`](#vidioc_enumaudout) | const |  |
| [`VIDIOC_SUBDEV_S_STD`](#vidioc_subdev_s_std) | const |  |
| [`WDIOC_GETTIMELEFT`](#wdioc_gettimeleft) | const |  |
| [`ATM_GETLINKRATE`](#atm_getlinkrate) | const |  |
| [`RTC_WKALM_SET`](#rtc_wkalm_set) | const |  |
| [`VHOST_GET_BACKEND_FEATURES`](#vhost_get_backend_features) | const |  |
| [`ATMARP_ENCAP`](#atmarp_encap) | const |  |
| [`CAPI_GET_FLAGS`](#capi_get_flags) | const |  |
| [`IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD`](#ipmictl_set_my_channel_address_cmd) | const |  |
| [`DFL_FPGA_FME_PORT_ASSIGN`](#dfl_fpga_fme_port_assign) | const |  |
| [`NS_GET_OWNER_UID`](#ns_get_owner_uid) | const |  |
| [`VIDIOC_OVERLAY`](#vidioc_overlay) | const |  |
| [`BTRFS_IOC_WAIT_SYNC`](#btrfs_ioc_wait_sync) | const |  |
| [`GPIOHANDLE_SET_CONFIG_IOCTL`](#gpiohandle_set_config_ioctl) | const |  |
| [`VHOST_GET_VRING_ENDIAN`](#vhost_get_vring_endian) | const |  |
| [`ATM_GETADDR`](#atm_getaddr) | const |  |
| [`PHN_GET_REGS`](#phn_get_regs) | const |  |
| [`AUTOFS_DEV_IOCTL_REQUESTER`](#autofs_dev_ioctl_requester) | const |  |
| [`AUTOFS_DEV_IOCTL_EXPIRE`](#autofs_dev_ioctl_expire) | const |  |
| [`SNAPSHOT_S2RAM`](#snapshot_s2ram) | const |  |
| [`JSIOCSAXMAP`](#jsiocsaxmap) | const |  |
| [`F2FS_IOC_SET_COMPRESS_OPTION`](#f2fs_ioc_set_compress_option) | const |  |
| [`VBG_IOCTL_HGCM_DISCONNECT`](#vbg_ioctl_hgcm_disconnect) | const |  |
| [`SCIF_FENCE_SIGNAL`](#scif_fence_signal) | const |  |
| [`VFIO_DEVICE_GET_PCI_HOT_RESET_INFO`](#vfio_device_get_pci_hot_reset_info) | const |  |
| [`VIDIOC_SUBDEV_ENUM_MBUS_CODE`](#vidioc_subdev_enum_mbus_code) | const |  |
| [`MMTIMER_GETOFFSET`](#mmtimer_getoffset) | const |  |
| [`RIO_CM_CHAN_LISTEN`](#rio_cm_chan_listen) | const |  |
| [`ATM_SETSC`](#atm_setsc) | const |  |
| [`F2FS_IOC_SHUTDOWN`](#f2fs_ioc_shutdown) | const |  |
| [`NVME_IOCTL_RESCAN`](#nvme_ioctl_rescan) | const |  |
| [`BLKOPENZONE`](#blkopenzone) | const |  |
| [`DM_VERSION`](#dm_version) | const |  |
| [`CEC_TRANSMIT`](#cec_transmit) | const |  |
| [`FS_IOC_GET_ENCRYPTION_POLICY_EX`](#fs_ioc_get_encryption_policy_ex) | const |  |
| [`SIOCMKCLIP`](#siocmkclip) | const |  |
| [`IPMI_BMC_IOCTL_CLEAR_SMS_ATN`](#ipmi_bmc_ioctl_clear_sms_atn) | const |  |
| [`HIDIOCGVERSION`](#hidiocgversion) | const |  |
| [`VIDIOC_S_INPUT`](#vidioc_s_input) | const |  |
| [`VIDIOC_G_CROP`](#vidioc_g_crop) | const |  |
| [`LIRC_SET_WIDEBAND_RECEIVER`](#lirc_set_wideband_receiver) | const |  |
| [`EVIOCGEFFECTS`](#eviocgeffects) | const |  |
| [`UVCIOC_CTRL_QUERY`](#uvcioc_ctrl_query) | const |  |
| [`IOC_OPAL_GENERIC_TABLE_RW`](#ioc_opal_generic_table_rw) | const |  |
| [`FS_IOC_READ_VERITY_METADATA`](#fs_ioc_read_verity_metadata) | const |  |
| [`ND_IOCTL_SET_CONFIG_DATA`](#nd_ioctl_set_config_data) | const |  |
| [`USBDEVFS_GETDRIVER`](#usbdevfs_getdriver) | const |  |
| [`IDT77105_GETSTAT`](#idt77105_getstat) | const |  |
| [`HIDIOCINITREPORT`](#hidiocinitreport) | const |  |
| [`VFIO_DEVICE_GET_INFO`](#vfio_device_get_info) | const |  |
| [`RIO_CM_CHAN_RECEIVE`](#rio_cm_chan_receive) | const |  |
| [`RNDGETENTCNT`](#rndgetentcnt) | const |  |
| [`PPPIOCNEWUNIT`](#pppiocnewunit) | const |  |
| [`BTRFS_IOC_INO_LOOKUP`](#btrfs_ioc_ino_lookup) | const |  |
| [`FDRESET`](#fdreset) | const |  |
| [`IOC_PR_REGISTER`](#ioc_pr_register) | const |  |
| [`HIDIOCSREPORT`](#hidiocsreport) | const |  |
| [`TEE_IOC_OPEN_SESSION`](#tee_ioc_open_session) | const |  |
| [`TEE_IOC_SUPPL_RECV`](#tee_ioc_suppl_recv) | const |  |
| [`BTRFS_IOC_BALANCE_CTL`](#btrfs_ioc_balance_ctl) | const |  |
| [`GPIO_GET_LINEINFO_WATCH_IOCTL`](#gpio_get_lineinfo_watch_ioctl) | const |  |
| [`HIDIOCGRAWINFO`](#hidiocgrawinfo) | const |  |
| [`PPPIOCSCOMPRESS`](#pppiocscompress) | const |  |
| [`USBDEVFS_CONNECTINFO`](#usbdevfs_connectinfo) | const |  |
| [`BLKRESETZONE`](#blkresetzone) | const |  |
| [`CHIOINITELEM`](#chioinitelem) | const |  |
| [`NILFS_IOCTL_SET_ALLOC_RANGE`](#nilfs_ioctl_set_alloc_range) | const |  |
| [`AUTOFS_DEV_IOCTL_CATATONIC`](#autofs_dev_ioctl_catatonic) | const |  |
| [`RIO_MPORT_MAINT_HDID_SET`](#rio_mport_maint_hdid_set) | const |  |
| [`PPGETPHASE`](#ppgetphase) | const |  |
| [`USBDEVFS_DISCONNECT_CLAIM`](#usbdevfs_disconnect_claim) | const |  |
| [`FDMSGON`](#fdmsgon) | const |  |
| [`VIDIOC_G_SLICED_VBI_CAP`](#vidioc_g_sliced_vbi_cap) | const |  |
| [`BTRFS_IOC_BALANCE_V2`](#btrfs_ioc_balance_v2) | const |  |
| [`MEDIA_REQUEST_IOC_REINIT`](#media_request_ioc_reinit) | const |  |
| [`IOC_OPAL_ERASE_LR`](#ioc_opal_erase_lr) | const |  |
| [`FDFMTBEG`](#fdfmtbeg) | const |  |
| [`RNDRESEEDCRNG`](#rndreseedcrng) | const |  |
| [`ISST_IF_GET_PHY_ID`](#isst_if_get_phy_id) | const |  |
| [`TUNSETNOCSUM`](#tunsetnocsum) | const |  |
| [`SONET_GETSTAT`](#sonet_getstat) | const |  |
| [`TFD_IOC_SET_TICKS`](#tfd_ioc_set_ticks) | const |  |
| [`PPDATADIR`](#ppdatadir) | const |  |
| [`IOC_OPAL_ENABLE_DISABLE_MBR`](#ioc_opal_enable_disable_mbr) | const |  |
| [`GPIO_V2_GET_LINE_IOCTL`](#gpio_v2_get_line_ioctl) | const |  |
| [`RIO_CM_CHAN_SEND`](#rio_cm_chan_send) | const |  |
| [`PPWCTLONIRQ`](#ppwctlonirq) | const |  |
| [`SONYPI_IOCGBRT`](#sonypi_iocgbrt) | const |  |
| [`IOC_PR_RELEASE`](#ioc_pr_release) | const |  |
| [`PPCLRIRQ`](#ppclrirq) | const |  |
| [`IPMICTL_SET_MY_CHANNEL_LUN_CMD`](#ipmictl_set_my_channel_lun_cmd) | const |  |
| [`MGSL_IOCSXSYNC`](#mgsl_iocsxsync) | const |  |
| [`HPET_IE_OFF`](#hpet_ie_off) | const |  |
| [`IOC_OPAL_ACTIVATE_USR`](#ioc_opal_activate_usr) | const |  |
| [`SONET_SETFRAMING`](#sonet_setframing) | const |  |
| [`PERF_EVENT_IOC_PAUSE_OUTPUT`](#perf_event_ioc_pause_output) | const |  |
| [`BTRFS_IOC_LOGICAL_INO_V2`](#btrfs_ioc_logical_ino_v2) | const |  |
| [`VBG_IOCTL_HGCM_CONNECT`](#vbg_ioctl_hgcm_connect) | const |  |
| [`BLKFINISHZONE`](#blkfinishzone) | const |  |
| [`EVIOCREVOKE`](#eviocrevoke) | const |  |
| [`VFIO_DEVICE_FEATURE`](#vfio_device_feature) | const |  |
| [`CCISS_GETPCIINFO`](#cciss_getpciinfo) | const |  |
| [`ISST_IF_MBOX_COMMAND`](#isst_if_mbox_command) | const |  |
| [`SCIF_ACCEPTREQ`](#scif_acceptreq) | const |  |
| [`PERF_EVENT_IOC_QUERY_BPF`](#perf_event_ioc_query_bpf) | const |  |
| [`VIDIOC_STREAMOFF`](#vidioc_streamoff) | const |  |
| [`VDUSE_DESTROY_DEV`](#vduse_destroy_dev) | const |  |
| [`FDGETFDCSTAT`](#fdgetfdcstat) | const |  |
| [`VIDIOC_S_PRIORITY`](#vidioc_s_priority) | const |  |
| [`SNAPSHOT_FREEZE`](#snapshot_freeze) | const |  |
| [`VIDIOC_ENUMINPUT`](#vidioc_enuminput) | const |  |
| [`ZATM_GETPOOLZ`](#zatm_getpoolz) | const |  |
| [`RIO_DISABLE_DOORBELL_RANGE`](#rio_disable_doorbell_range) | const |  |
| [`GPIO_V2_GET_LINEINFO_WATCH_IOCTL`](#gpio_v2_get_lineinfo_watch_ioctl) | const |  |
| [`VIDIOC_G_STD`](#vidioc_g_std) | const |  |
| [`USBDEVFS_ALLOW_SUSPEND`](#usbdevfs_allow_suspend) | const |  |
| [`SONET_GETSTATZ`](#sonet_getstatz) | const |  |
| [`SCIF_ACCEPTREG`](#scif_acceptreg) | const |  |
| [`VIDIOC_ENCODER_CMD`](#vidioc_encoder_cmd) | const |  |
| [`PPPIOCSRASYNCMAP`](#pppiocsrasyncmap) | const |  |
| [`IOCTL_MEI_NOTIFY_SET`](#ioctl_mei_notify_set) | const |  |
| [`BTRFS_IOC_QUOTA_RESCAN_STATUS`](#btrfs_ioc_quota_rescan_status) | const |  |
| [`F2FS_IOC_GARBAGE_COLLECT`](#f2fs_ioc_garbage_collect) | const |  |
| [`ATMLEC_CTRL`](#atmlec_ctrl) | const |  |
| [`MATROXFB_GET_AVAILABLE_OUTPUTS`](#matroxfb_get_available_outputs) | const |  |
| [`DM_DEV_CREATE`](#dm_dev_create) | const |  |
| [`VHOST_VDPA_GET_VRING_NUM`](#vhost_vdpa_get_vring_num) | const |  |
| [`VIDIOC_G_CTRL`](#vidioc_g_ctrl) | const |  |
| [`NBD_CLEAR_SOCK`](#nbd_clear_sock) | const |  |
| [`VFIO_DEVICE_QUERY_GFX_PLANE`](#vfio_device_query_gfx_plane) | const |  |
| [`WDIOC_KEEPALIVE`](#wdioc_keepalive) | const |  |
| [`NVME_IOCTL_SUBSYS_RESET`](#nvme_ioctl_subsys_reset) | const |  |
| [`PTP_EXTTS_REQUEST2`](#ptp_extts_request2) | const |  |
| [`PCITEST_BAR`](#pcitest_bar) | const |  |
| [`MGSL_IOCGGPIO`](#mgsl_iocggpio) | const |  |
| [`EVIOCSREP`](#eviocsrep) | const |  |
| [`VFIO_DEVICE_GET_IRQ_INFO`](#vfio_device_get_irq_info) | const |  |
| [`HPET_DPI`](#hpet_dpi) | const |  |
| [`VDUSE_VQ_SETUP_KICKFD`](#vduse_vq_setup_kickfd) | const |  |
| [`ND_IOCTL_CALL`](#nd_ioctl_call) | const |  |
| [`HIDIOCGDEVINFO`](#hidiocgdevinfo) | const |  |
| [`DM_TABLE_DEPS`](#dm_table_deps) | const |  |
| [`BTRFS_IOC_DEV_INFO`](#btrfs_ioc_dev_info) | const |  |
| [`VDUSE_IOTLB_GET_FD`](#vduse_iotlb_get_fd) | const |  |
| [`FW_CDEV_IOC_GET_INFO`](#fw_cdev_ioc_get_info) | const |  |
| [`VIDIOC_G_PRIORITY`](#vidioc_g_priority) | const |  |
| [`ATM_NEWBACKENDIF`](#atm_newbackendif) | const |  |
| [`VIDIOC_S_EXT_CTRLS`](#vidioc_s_ext_ctrls) | const |  |
| [`VIDIOC_SUBDEV_ENUM_DV_TIMINGS`](#vidioc_subdev_enum_dv_timings) | const |  |
| [`VIDIOC_OMAP3ISP_CCDC_CFG`](#vidioc_omap3isp_ccdc_cfg) | const |  |
| [`VIDIOC_S_HW_FREQ_SEEK`](#vidioc_s_hw_freq_seek) | const |  |
| [`DM_TABLE_LOAD`](#dm_table_load) | const |  |
| [`F2FS_IOC_START_ATOMIC_WRITE`](#f2fs_ioc_start_atomic_write) | const |  |
| [`VIDIOC_G_OUTPUT`](#vidioc_g_output) | const |  |
| [`ATM_DROPPARTY`](#atm_dropparty) | const |  |
| [`CHIOGELEM`](#chiogelem) | const |  |
| [`BTRFS_IOC_GET_SUPPORTED_FEATURES`](#btrfs_ioc_get_supported_features) | const |  |
| [`EVIOCSKEYCODE`](#eviocskeycode) | const |  |
| [`NE_GET_IMAGE_LOAD_INFO`](#ne_get_image_load_info) | const |  |
| [`TUNSETLINK`](#tunsetlink) | const |  |
| [`FW_CDEV_IOC_ADD_DESCRIPTOR`](#fw_cdev_ioc_add_descriptor) | const |  |
| [`BTRFS_IOC_SCRUB_CANCEL`](#btrfs_ioc_scrub_cancel) | const |  |
| [`PPS_SETPARAMS`](#pps_setparams) | const |  |
| [`IOC_OPAL_LR_SETUP`](#ioc_opal_lr_setup) | const |  |
| [`FW_CDEV_IOC_DEALLOCATE`](#fw_cdev_ioc_deallocate) | const |  |
| [`WDIOC_SETTIMEOUT`](#wdioc_settimeout) | const |  |
| [`IOC_WATCH_QUEUE_SET_FILTER`](#ioc_watch_queue_set_filter) | const |  |
| [`CAPI_GET_MANUFACTURER`](#capi_get_manufacturer) | const |  |
| [`VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY`](#vfio_iommu_spapr_unregister_memory) | const |  |
| [`ASPEED_P2A_CTRL_IOCTL_SET_WINDOW`](#aspeed_p2a_ctrl_ioctl_set_window) | const |  |
| [`VIDIOC_G_EDID`](#vidioc_g_edid) | const |  |
| [`F2FS_IOC_GARBAGE_COLLECT_RANGE`](#f2fs_ioc_garbage_collect_range) | const |  |
| [`RIO_MAP_INBOUND`](#rio_map_inbound) | const |  |
| [`IOC_OPAL_TAKE_OWNERSHIP`](#ioc_opal_take_ownership) | const |  |
| [`USBDEVFS_CLAIM_PORT`](#usbdevfs_claim_port) | const |  |
| [`VIDIOC_S_AUDIO`](#vidioc_s_audio) | const |  |
| [`FS_IOC_GET_ENCRYPTION_NONCE`](#fs_ioc_get_encryption_nonce) | const |  |
| [`FW_CDEV_IOC_SEND_STREAM_PACKET`](#fw_cdev_ioc_send_stream_packet) | const |  |
| [`BTRFS_IOC_SNAP_DESTROY`](#btrfs_ioc_snap_destroy) | const |  |
| [`SNAPSHOT_FREE`](#snapshot_free) | const |  |
| [`I8K_GET_SPEED`](#i8k_get_speed) | const |  |
| [`HIDIOCGREPORT`](#hidiocgreport) | const |  |
| [`HPET_EPI`](#hpet_epi) | const |  |
| [`JSIOCSCORR`](#jsiocscorr) | const |  |
| [`IOC_PR_PREEMPT_ABORT`](#ioc_pr_preempt_abort) | const |  |
| [`RIO_MAP_OUTBOUND`](#rio_map_outbound) | const |  |
| [`ATM_SETESI`](#atm_setesi) | const |  |
| [`FW_CDEV_IOC_START_ISO`](#fw_cdev_ioc_start_iso) | const |  |
| [`ATM_DELADDR`](#atm_deladdr) | const |  |
| [`PPFCONTROL`](#ppfcontrol) | const |  |
| [`SONYPI_IOCGFAN`](#sonypi_iocgfan) | const |  |
| [`RTC_IRQP_SET`](#rtc_irqp_set) | const |  |
| [`PCITEST_WRITE`](#pcitest_write) | const |  |
| [`PPCLAIM`](#ppclaim) | const |  |
| [`VIDIOC_S_JPEGCOMP`](#vidioc_s_jpegcomp) | const |  |
| [`IPMICTL_UNREGISTER_FOR_CMD`](#ipmictl_unregister_for_cmd) | const |  |
| [`VHOST_SET_FEATURES`](#vhost_set_features) | const |  |
| [`TOSHIBA_ACPI_SCI`](#toshiba_acpi_sci) | const |  |
| [`VIDIOC_DQBUF`](#vidioc_dqbuf) | const |  |
| [`BTRFS_IOC_BALANCE_PROGRESS`](#btrfs_ioc_balance_progress) | const |  |
| [`BTRFS_IOC_SUBVOL_SETFLAGS`](#btrfs_ioc_subvol_setflags) | const |  |
| [`ATMLEC_MCAST`](#atmlec_mcast) | const |  |
| [`MMTIMER_GETFREQ`](#mmtimer_getfreq) | const |  |
| [`VIDIOC_G_SELECTION`](#vidioc_g_selection) | const |  |
| [`RTC_ALM_SET`](#rtc_alm_set) | const |  |
| [`PPPOEIOCSFWD`](#pppoeiocsfwd) | const |  |
| [`IPMICTL_GET_MAINTENANCE_MODE_CMD`](#ipmictl_get_maintenance_mode_cmd) | const |  |
| [`FS_IOC_ENABLE_VERITY`](#fs_ioc_enable_verity) | const |  |
| [`NILFS_IOCTL_GET_BDESCS`](#nilfs_ioctl_get_bdescs) | const |  |
| [`FDFMTEND`](#fdfmtend) | const |  |
| [`DMA_BUF_SET_NAME`](#dma_buf_set_name) | const |  |
| [`UI_BEGIN_FF_UPLOAD`](#ui_begin_ff_upload) | const |  |
| [`RTC_UIE_ON`](#rtc_uie_on) | const |  |
| [`PPRELEASE`](#pprelease) | const |  |
| [`VFIO_IOMMU_UNMAP_DMA`](#vfio_iommu_unmap_dma) | const |  |
| [`VIDIOC_OMAP3ISP_PRV_CFG`](#vidioc_omap3isp_prv_cfg) | const |  |
| [`GPIO_GET_LINEHANDLE_IOCTL`](#gpio_get_linehandle_ioctl) | const |  |
| [`VFAT_IOCTL_READDIR_BOTH`](#vfat_ioctl_readdir_both) | const |  |
| [`NVME_IOCTL_ADMIN_CMD`](#nvme_ioctl_admin_cmd) | const |  |
| [`VHOST_SET_VRING_KICK`](#vhost_set_vring_kick) | const |  |
| [`BTRFS_IOC_SUBVOL_CREATE_V2`](#btrfs_ioc_subvol_create_v2) | const |  |
| [`BTRFS_IOC_SNAP_CREATE`](#btrfs_ioc_snap_create) | const |  |
| [`SONYPI_IOCGBAT2CAP`](#sonypi_iocgbat2cap) | const |  |
| [`PPNEGOT`](#ppnegot) | const |  |
| [`NBD_PRINT_DEBUG`](#nbd_print_debug) | const |  |
| [`BTRFS_IOC_INO_LOOKUP_USER`](#btrfs_ioc_ino_lookup_user) | const |  |
| [`BTRFS_IOC_GET_SUBVOL_ROOTREF`](#btrfs_ioc_get_subvol_rootref) | const |  |
| [`FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS`](#fs_ioc_remove_encryption_key_all_users) | const |  |
| [`BTRFS_IOC_FS_INFO`](#btrfs_ioc_fs_info) | const |  |
| [`VIDIOC_ENUM_FMT`](#vidioc_enum_fmt) | const |  |
| [`VIDIOC_G_INPUT`](#vidioc_g_input) | const |  |
| [`VTPM_PROXY_IOC_NEW_DEV`](#vtpm_proxy_ioc_new_dev) | const |  |
| [`DFL_FPGA_FME_ERR_GET_IRQ_NUM`](#dfl_fpga_fme_err_get_irq_num) | const |  |
| [`ND_IOCTL_DIMM_FLAGS`](#nd_ioctl_dimm_flags) | const |  |
| [`BTRFS_IOC_QUOTA_RESCAN`](#btrfs_ioc_quota_rescan) | const |  |
| [`MMTIMER_GETCOUNTER`](#mmtimer_getcounter) | const |  |
| [`MATROXFB_GET_OUTPUT_MODE`](#matroxfb_get_output_mode) | const |  |
| [`BTRFS_IOC_QUOTA_RESCAN_WAIT`](#btrfs_ioc_quota_rescan_wait) | const |  |
| [`RIO_CM_CHAN_BIND`](#rio_cm_chan_bind) | const |  |
| [`HIDIOCGRDESC`](#hidiocgrdesc) | const |  |
| [`MGSL_IOCGIF`](#mgsl_iocgif) | const |  |
| [`VIDIOC_S_OUTPUT`](#vidioc_s_output) | const |  |
| [`HIDIOCGREPORTINFO`](#hidiocgreportinfo) | const |  |
| [`WDIOC_GETBOOTSTATUS`](#wdioc_getbootstatus) | const |  |
| [`VDUSE_VQ_GET_INFO`](#vduse_vq_get_info) | const |  |
| [`ACRN_IOCTL_ASSIGN_PCIDEV`](#acrn_ioctl_assign_pcidev) | const |  |
| [`BLKGETDISKSEQ`](#blkgetdiskseq) | const |  |
| [`ACRN_IOCTL_PM_GET_CPU_STATE`](#acrn_ioctl_pm_get_cpu_state) | const |  |
| [`ACRN_IOCTL_DESTROY_VM`](#acrn_ioctl_destroy_vm) | const |  |
| [`ACRN_IOCTL_SET_PTDEV_INTR`](#acrn_ioctl_set_ptdev_intr) | const |  |
| [`ACRN_IOCTL_CREATE_IOREQ_CLIENT`](#acrn_ioctl_create_ioreq_client) | const |  |
| [`ACRN_IOCTL_IRQFD`](#acrn_ioctl_irqfd) | const |  |
| [`ACRN_IOCTL_CREATE_VM`](#acrn_ioctl_create_vm) | const |  |
| [`ACRN_IOCTL_INJECT_MSI`](#acrn_ioctl_inject_msi) | const |  |
| [`ACRN_IOCTL_ATTACH_IOREQ_CLIENT`](#acrn_ioctl_attach_ioreq_client) | const |  |
| [`ACRN_IOCTL_RESET_PTDEV_INTR`](#acrn_ioctl_reset_ptdev_intr) | const |  |
| [`ACRN_IOCTL_NOTIFY_REQUEST_FINISH`](#acrn_ioctl_notify_request_finish) | const |  |
| [`ACRN_IOCTL_SET_IRQLINE`](#acrn_ioctl_set_irqline) | const |  |
| [`ACRN_IOCTL_START_VM`](#acrn_ioctl_start_vm) | const |  |
| [`ACRN_IOCTL_SET_VCPU_REGS`](#acrn_ioctl_set_vcpu_regs) | const |  |
| [`ACRN_IOCTL_SET_MEMSEG`](#acrn_ioctl_set_memseg) | const |  |
| [`ACRN_IOCTL_PAUSE_VM`](#acrn_ioctl_pause_vm) | const |  |
| [`ACRN_IOCTL_CLEAR_VM_IOREQ`](#acrn_ioctl_clear_vm_ioreq) | const |  |
| [`ACRN_IOCTL_UNSET_MEMSEG`](#acrn_ioctl_unset_memseg) | const |  |
| [`ACRN_IOCTL_IOEVENTFD`](#acrn_ioctl_ioeventfd) | const |  |
| [`ACRN_IOCTL_DEASSIGN_PCIDEV`](#acrn_ioctl_deassign_pcidev) | const |  |
| [`ACRN_IOCTL_RESET_VM`](#acrn_ioctl_reset_vm) | const |  |
| [`ACRN_IOCTL_DESTROY_IOREQ_CLIENT`](#acrn_ioctl_destroy_ioreq_client) | const |  |
| [`ACRN_IOCTL_VM_INTR_MONITOR`](#acrn_ioctl_vm_intr_monitor) | const |  |

## Constants

### `FIONREAD`
```rust
const FIONREAD: u32 = 21_531u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:3`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L3)*

### `FIONBIO`
```rust
const FIONBIO: u32 = 21_537u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:4`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L4)*

### `FIOCLEX`
```rust
const FIOCLEX: u32 = 21_585u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:5`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L5)*

### `FIONCLEX`
```rust
const FIONCLEX: u32 = 21_584u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:6`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L6)*

### `FIOASYNC`
```rust
const FIOASYNC: u32 = 21_586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:7`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L7)*

### `FIOQSIZE`
```rust
const FIOQSIZE: u32 = 21_600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:8`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L8)*

### `TCXONC`
```rust
const TCXONC: u32 = 21_514u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:9`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L9)*

### `TCFLSH`
```rust
const TCFLSH: u32 = 21_515u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:10`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L10)*

### `TIOCSCTTY`
```rust
const TIOCSCTTY: u32 = 21_518u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:11`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L11)*

### `TIOCSPGRP`
```rust
const TIOCSPGRP: u32 = 21_520u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:12`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L12)*

### `TIOCOUTQ`
```rust
const TIOCOUTQ: u32 = 21_521u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:13`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L13)*

### `TIOCSTI`
```rust
const TIOCSTI: u32 = 21_522u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:14`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L14)*

### `TIOCSWINSZ`
```rust
const TIOCSWINSZ: u32 = 21_524u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:15`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L15)*

### `TIOCMGET`
```rust
const TIOCMGET: u32 = 21_525u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:16`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L16)*

### `TIOCMBIS`
```rust
const TIOCMBIS: u32 = 21_526u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:17`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L17)*

### `TIOCMBIC`
```rust
const TIOCMBIC: u32 = 21_527u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:18`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L18)*

### `TIOCMSET`
```rust
const TIOCMSET: u32 = 21_528u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:19`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L19)*

### `TIOCSSOFTCAR`
```rust
const TIOCSSOFTCAR: u32 = 21_530u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:20`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L20)*

### `TIOCLINUX`
```rust
const TIOCLINUX: u32 = 21_532u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:21`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L21)*

### `TIOCCONS`
```rust
const TIOCCONS: u32 = 21_533u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:22`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L22)*

### `TIOCSSERIAL`
```rust
const TIOCSSERIAL: u32 = 21_535u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:23`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L23)*

### `TIOCPKT`
```rust
const TIOCPKT: u32 = 21_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:24`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L24)*

### `TIOCNOTTY`
```rust
const TIOCNOTTY: u32 = 21_538u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:25`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L25)*

### `TIOCSETD`
```rust
const TIOCSETD: u32 = 21_539u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:26`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L26)*

### `TIOCSBRK`
```rust
const TIOCSBRK: u32 = 21_543u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:27`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L27)*

### `TIOCCBRK`
```rust
const TIOCCBRK: u32 = 21_544u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:28`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L28)*

### `TIOCSRS485`
```rust
const TIOCSRS485: u32 = 21_551u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:29`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L29)*

### `TIOCSPTLCK`
```rust
const TIOCSPTLCK: u32 = 1_074_025_521u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:30`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L30)*

### `TIOCSIG`
```rust
const TIOCSIG: u32 = 1_074_025_526u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:31`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L31)*

### `TIOCVHANGUP`
```rust
const TIOCVHANGUP: u32 = 21_559u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:32`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L32)*

### `TIOCSERCONFIG`
```rust
const TIOCSERCONFIG: u32 = 21_587u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:33`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L33)*

### `TIOCSERGWILD`
```rust
const TIOCSERGWILD: u32 = 21_588u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:34`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L34)*

### `TIOCSERSWILD`
```rust
const TIOCSERSWILD: u32 = 21_589u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:35`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L35)*

### `TIOCSLCKTRMIOS`
```rust
const TIOCSLCKTRMIOS: u32 = 21_591u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:36`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L36)*

### `TIOCSERGSTRUCT`
```rust
const TIOCSERGSTRUCT: u32 = 21_592u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:37`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L37)*

### `TIOCSERGETLSR`
```rust
const TIOCSERGETLSR: u32 = 21_593u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:38`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L38)*

### `TIOCSERGETMULTI`
```rust
const TIOCSERGETMULTI: u32 = 21_594u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:39`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L39)*

### `TIOCSERSETMULTI`
```rust
const TIOCSERSETMULTI: u32 = 21_595u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:40`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L40)*

### `TIOCMIWAIT`
```rust
const TIOCMIWAIT: u32 = 21_596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:41`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L41)*

### `TCGETS`
```rust
const TCGETS: u32 = 21_505u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:42`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L42)*

### `TCGETA`
```rust
const TCGETA: u32 = 21_509u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:43`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L43)*

### `TCSBRK`
```rust
const TCSBRK: u32 = 21_513u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:44`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L44)*

### `TCSBRKP`
```rust
const TCSBRKP: u32 = 21_541u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:45`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L45)*

### `TCSETA`
```rust
const TCSETA: u32 = 21_510u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:46`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L46)*

### `TCSETAF`
```rust
const TCSETAF: u32 = 21_512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:47`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L47)*

### `TCSETAW`
```rust
const TCSETAW: u32 = 21_511u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:48`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L48)*

### `TIOCEXCL`
```rust
const TIOCEXCL: u32 = 21_516u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:49`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L49)*

### `TIOCNXCL`
```rust
const TIOCNXCL: u32 = 21_517u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:50`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L50)*

### `TIOCGDEV`
```rust
const TIOCGDEV: u32 = 2_147_767_346u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:51`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L51)*

### `TIOCGEXCL`
```rust
const TIOCGEXCL: u32 = 2_147_767_360u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:52`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L52)*

### `TIOCGICOUNT`
```rust
const TIOCGICOUNT: u32 = 21_597u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:53`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L53)*

### `TIOCGLCKTRMIOS`
```rust
const TIOCGLCKTRMIOS: u32 = 21_590u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:54`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L54)*

### `TIOCGPGRP`
```rust
const TIOCGPGRP: u32 = 21_519u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:55`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L55)*

### `TIOCGPKT`
```rust
const TIOCGPKT: u32 = 2_147_767_352u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:56`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L56)*

### `TIOCGPTLCK`
```rust
const TIOCGPTLCK: u32 = 2_147_767_353u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:57`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L57)*

### `TIOCGPTN`
```rust
const TIOCGPTN: u32 = 2_147_767_344u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:58`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L58)*

### `TIOCGPTPEER`
```rust
const TIOCGPTPEER: u32 = 21_569u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:59`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L59)*

### `TIOCGRS485`
```rust
const TIOCGRS485: u32 = 21_550u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:60`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L60)*

### `TIOCGSERIAL`
```rust
const TIOCGSERIAL: u32 = 21_534u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:61`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L61)*

### `TIOCGSID`
```rust
const TIOCGSID: u32 = 21_545u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:62`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L62)*

### `TIOCGSOFTCAR`
```rust
const TIOCGSOFTCAR: u32 = 21_529u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:63`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L63)*

### `TIOCGWINSZ`
```rust
const TIOCGWINSZ: u32 = 21_523u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:64`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L64)*

### `TCGETS2`
```rust
const TCGETS2: u32 = 2_150_388_778u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:65`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L65)*

### `TCGETX`
```rust
const TCGETX: u32 = 21_554u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:66`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L66)*

### `TCSETS`
```rust
const TCSETS: u32 = 21_506u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:67`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L67)*

### `TCSETS2`
```rust
const TCSETS2: u32 = 1_076_646_955u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:68`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L68)*

### `TCSETSF`
```rust
const TCSETSF: u32 = 21_508u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:69`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L69)*

### `TCSETSF2`
```rust
const TCSETSF2: u32 = 1_076_646_957u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:70`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L70)*

### `TCSETSW`
```rust
const TCSETSW: u32 = 21_507u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:71`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L71)*

### `TCSETSW2`
```rust
const TCSETSW2: u32 = 1_076_646_956u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:72`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L72)*

### `TCSETX`
```rust
const TCSETX: u32 = 21_555u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:73`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L73)*

### `TCSETXF`
```rust
const TCSETXF: u32 = 21_556u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:74`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L74)*

### `TCSETXW`
```rust
const TCSETXW: u32 = 21_557u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:75`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L75)*

### `TIOCGETD`
```rust
const TIOCGETD: u32 = 21_540u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:76`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L76)*

### `MTIOCGET`
```rust
const MTIOCGET: u32 = 2_150_657_282u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:77`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L77)*

### `BLKSSZGET`
```rust
const BLKSSZGET: u32 = 4_712u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:78`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L78)*

### `BLKPBSZGET`
```rust
const BLKPBSZGET: u32 = 4_731u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:79`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L79)*

### `BLKROSET`
```rust
const BLKROSET: u32 = 4_701u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:80`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L80)*

### `BLKROGET`
```rust
const BLKROGET: u32 = 4_702u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:81`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L81)*

### `BLKRRPART`
```rust
const BLKRRPART: u32 = 4_703u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:82`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L82)*

### `BLKGETSIZE`
```rust
const BLKGETSIZE: u32 = 4_704u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:83`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L83)*

### `BLKFLSBUF`
```rust
const BLKFLSBUF: u32 = 4_705u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:84`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L84)*

### `BLKRASET`
```rust
const BLKRASET: u32 = 4_706u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:85`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L85)*

### `BLKRAGET`
```rust
const BLKRAGET: u32 = 4_707u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:86`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L86)*

### `BLKFRASET`
```rust
const BLKFRASET: u32 = 4_708u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:87`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L87)*

### `BLKFRAGET`
```rust
const BLKFRAGET: u32 = 4_709u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:88`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L88)*

### `BLKSECTSET`
```rust
const BLKSECTSET: u32 = 4_710u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:89`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L89)*

### `BLKSECTGET`
```rust
const BLKSECTGET: u32 = 4_711u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:90`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L90)*

### `BLKPG`
```rust
const BLKPG: u32 = 4_713u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:91`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L91)*

### `BLKBSZGET`
```rust
const BLKBSZGET: u32 = 2_148_012_656u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:92`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L92)*

### `BLKBSZSET`
```rust
const BLKBSZSET: u32 = 1_074_270_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:93`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L93)*

### `BLKGETSIZE64`
```rust
const BLKGETSIZE64: u32 = 2_148_012_658u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:94`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L94)*

### `BLKTRACESETUP`
```rust
const BLKTRACESETUP: u32 = 3_225_948_787u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:95`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L95)*

### `BLKTRACESTART`
```rust
const BLKTRACESTART: u32 = 4_724u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:96`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L96)*

### `BLKTRACESTOP`
```rust
const BLKTRACESTOP: u32 = 4_725u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:97`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L97)*

### `BLKTRACETEARDOWN`
```rust
const BLKTRACETEARDOWN: u32 = 4_726u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:98`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L98)*

### `BLKDISCARD`
```rust
const BLKDISCARD: u32 = 4_727u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:99`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L99)*

### `BLKIOMIN`
```rust
const BLKIOMIN: u32 = 4_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:100`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L100)*

### `BLKIOOPT`
```rust
const BLKIOOPT: u32 = 4_729u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:101`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L101)*

### `BLKALIGNOFF`
```rust
const BLKALIGNOFF: u32 = 4_730u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:102`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L102)*

### `BLKDISCARDZEROES`
```rust
const BLKDISCARDZEROES: u32 = 4_732u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:103`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L103)*

### `BLKSECDISCARD`
```rust
const BLKSECDISCARD: u32 = 4_733u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:104`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L104)*

### `BLKROTATIONAL`
```rust
const BLKROTATIONAL: u32 = 4_734u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:105`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L105)*

### `BLKZEROOUT`
```rust
const BLKZEROOUT: u32 = 4_735u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:106`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L106)*

### `FIEMAP_MAX_OFFSET`
```rust
const FIEMAP_MAX_OFFSET: i32 = -1i32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:107`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L107)*

### `FIEMAP_FLAG_SYNC`
```rust
const FIEMAP_FLAG_SYNC: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:108`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L108)*

### `FIEMAP_FLAG_XATTR`
```rust
const FIEMAP_FLAG_XATTR: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:109`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L109)*

### `FIEMAP_FLAG_CACHE`
```rust
const FIEMAP_FLAG_CACHE: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:110`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L110)*

### `FIEMAP_FLAGS_COMPAT`
```rust
const FIEMAP_FLAGS_COMPAT: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:111`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L111)*

### `FIEMAP_EXTENT_LAST`
```rust
const FIEMAP_EXTENT_LAST: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:112`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L112)*

### `FIEMAP_EXTENT_UNKNOWN`
```rust
const FIEMAP_EXTENT_UNKNOWN: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:113`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L113)*

### `FIEMAP_EXTENT_DELALLOC`
```rust
const FIEMAP_EXTENT_DELALLOC: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:114`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L114)*

### `FIEMAP_EXTENT_ENCODED`
```rust
const FIEMAP_EXTENT_ENCODED: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:115`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L115)*

### `FIEMAP_EXTENT_DATA_ENCRYPTED`
```rust
const FIEMAP_EXTENT_DATA_ENCRYPTED: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:116`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L116)*

### `FIEMAP_EXTENT_NOT_ALIGNED`
```rust
const FIEMAP_EXTENT_NOT_ALIGNED: u32 = 256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:117`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L117)*

### `FIEMAP_EXTENT_DATA_INLINE`
```rust
const FIEMAP_EXTENT_DATA_INLINE: u32 = 512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:118`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L118)*

### `FIEMAP_EXTENT_DATA_TAIL`
```rust
const FIEMAP_EXTENT_DATA_TAIL: u32 = 1_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:119`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L119)*

### `FIEMAP_EXTENT_UNWRITTEN`
```rust
const FIEMAP_EXTENT_UNWRITTEN: u32 = 2_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:120`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L120)*

### `FIEMAP_EXTENT_MERGED`
```rust
const FIEMAP_EXTENT_MERGED: u32 = 4_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:121`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L121)*

### `FIEMAP_EXTENT_SHARED`
```rust
const FIEMAP_EXTENT_SHARED: u32 = 8_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:122`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L122)*

### `UFFDIO_REGISTER`
```rust
const UFFDIO_REGISTER: u32 = 3_223_366_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:123`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L123)*

### `UFFDIO_UNREGISTER`
```rust
const UFFDIO_UNREGISTER: u32 = 2_148_575_745u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:124`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L124)*

### `UFFDIO_WAKE`
```rust
const UFFDIO_WAKE: u32 = 2_148_575_746u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:125`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L125)*

### `UFFDIO_COPY`
```rust
const UFFDIO_COPY: u32 = 3_223_890_435u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:126`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L126)*

### `UFFDIO_ZEROPAGE`
```rust
const UFFDIO_ZEROPAGE: u32 = 3_223_366_148u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:127`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L127)*

### `UFFDIO_WRITEPROTECT`
```rust
const UFFDIO_WRITEPROTECT: u32 = 3_222_841_862u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:128`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L128)*

### `UFFDIO_API`
```rust
const UFFDIO_API: u32 = 3_222_841_919u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:129`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L129)*

### `NS_GET_USERNS`
```rust
const NS_GET_USERNS: u32 = 46_849u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:130`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L130)*

### `NS_GET_PARENT`
```rust
const NS_GET_PARENT: u32 = 46_850u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:131`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L131)*

### `NS_GET_NSTYPE`
```rust
const NS_GET_NSTYPE: u32 = 46_851u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:132`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L132)*

### `KDGETLED`
```rust
const KDGETLED: u32 = 19_249u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:133`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L133)*

### `KDSETLED`
```rust
const KDSETLED: u32 = 19_250u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:134`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L134)*

### `KDGKBLED`
```rust
const KDGKBLED: u32 = 19_300u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:135`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L135)*

### `KDSKBLED`
```rust
const KDSKBLED: u32 = 19_301u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:136`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L136)*

### `KDGKBTYPE`
```rust
const KDGKBTYPE: u32 = 19_251u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:137`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L137)*

### `KDADDIO`
```rust
const KDADDIO: u32 = 19_252u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:138`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L138)*

### `KDDELIO`
```rust
const KDDELIO: u32 = 19_253u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:139`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L139)*

### `KDENABIO`
```rust
const KDENABIO: u32 = 19_254u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:140`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L140)*

### `KDDISABIO`
```rust
const KDDISABIO: u32 = 19_255u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:141`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L141)*

### `KDSETMODE`
```rust
const KDSETMODE: u32 = 19_258u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:142`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L142)*

### `KDGETMODE`
```rust
const KDGETMODE: u32 = 19_259u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:143`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L143)*

### `KDMKTONE`
```rust
const KDMKTONE: u32 = 19_248u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:144`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L144)*

### `KIOCSOUND`
```rust
const KIOCSOUND: u32 = 19_247u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:145`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L145)*

### `GIO_CMAP`
```rust
const GIO_CMAP: u32 = 19_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:146`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L146)*

### `PIO_CMAP`
```rust
const PIO_CMAP: u32 = 19_313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:147`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L147)*

### `GIO_FONT`
```rust
const GIO_FONT: u32 = 19_296u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:148`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L148)*

### `GIO_FONTX`
```rust
const GIO_FONTX: u32 = 19_307u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:149`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L149)*

### `PIO_FONT`
```rust
const PIO_FONT: u32 = 19_297u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:150`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L150)*

### `PIO_FONTX`
```rust
const PIO_FONTX: u32 = 19_308u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:151`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L151)*

### `PIO_FONTRESET`
```rust
const PIO_FONTRESET: u32 = 19_309u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:152`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L152)*

### `GIO_SCRNMAP`
```rust
const GIO_SCRNMAP: u32 = 19_264u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:153`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L153)*

### `GIO_UNISCRNMAP`
```rust
const GIO_UNISCRNMAP: u32 = 19_305u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:154`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L154)*

### `PIO_SCRNMAP`
```rust
const PIO_SCRNMAP: u32 = 19_265u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:155`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L155)*

### `PIO_UNISCRNMAP`
```rust
const PIO_UNISCRNMAP: u32 = 19_306u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:156`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L156)*

### `GIO_UNIMAP`
```rust
const GIO_UNIMAP: u32 = 19_302u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:157`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L157)*

### `PIO_UNIMAP`
```rust
const PIO_UNIMAP: u32 = 19_303u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:158`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L158)*

### `PIO_UNIMAPCLR`
```rust
const PIO_UNIMAPCLR: u32 = 19_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:159`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L159)*

### `KDGKBMODE`
```rust
const KDGKBMODE: u32 = 19_268u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:160`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L160)*

### `KDSKBMODE`
```rust
const KDSKBMODE: u32 = 19_269u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:161`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L161)*

### `KDGKBMETA`
```rust
const KDGKBMETA: u32 = 19_298u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:162`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L162)*

### `KDSKBMETA`
```rust
const KDSKBMETA: u32 = 19_299u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:163`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L163)*

### `KDGKBENT`
```rust
const KDGKBENT: u32 = 19_270u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:164`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L164)*

### `KDSKBENT`
```rust
const KDSKBENT: u32 = 19_271u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:165`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L165)*

### `KDGKBSENT`
```rust
const KDGKBSENT: u32 = 19_272u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:166`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L166)*

### `KDSKBSENT`
```rust
const KDSKBSENT: u32 = 19_273u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:167`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L167)*

### `KDGKBDIACR`
```rust
const KDGKBDIACR: u32 = 19_274u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:168`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L168)*

### `KDGETKEYCODE`
```rust
const KDGETKEYCODE: u32 = 19_276u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:169`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L169)*

### `KDSETKEYCODE`
```rust
const KDSETKEYCODE: u32 = 19_277u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:170`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L170)*

### `KDSIGACCEPT`
```rust
const KDSIGACCEPT: u32 = 19_278u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:171`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L171)*

### `VT_OPENQRY`
```rust
const VT_OPENQRY: u32 = 22_016u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:172`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L172)*

### `VT_GETMODE`
```rust
const VT_GETMODE: u32 = 22_017u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:173`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L173)*

### `VT_SETMODE`
```rust
const VT_SETMODE: u32 = 22_018u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:174`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L174)*

### `VT_GETSTATE`
```rust
const VT_GETSTATE: u32 = 22_019u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:175`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L175)*

### `VT_RELDISP`
```rust
const VT_RELDISP: u32 = 22_021u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:176`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L176)*

### `VT_ACTIVATE`
```rust
const VT_ACTIVATE: u32 = 22_022u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:177`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L177)*

### `VT_WAITACTIVE`
```rust
const VT_WAITACTIVE: u32 = 22_023u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:178`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L178)*

### `VT_DISALLOCATE`
```rust
const VT_DISALLOCATE: u32 = 22_024u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:179`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L179)*

### `VT_RESIZE`
```rust
const VT_RESIZE: u32 = 22_025u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:180`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L180)*

### `VT_RESIZEX`
```rust
const VT_RESIZEX: u32 = 22_026u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:181`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L181)*

### `FIOSETOWN`
```rust
const FIOSETOWN: u32 = 35_073u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:182`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L182)*

### `SIOCSPGRP`
```rust
const SIOCSPGRP: u32 = 35_074u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:183`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L183)*

### `FIOGETOWN`
```rust
const FIOGETOWN: u32 = 35_075u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:184`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L184)*

### `SIOCGPGRP`
```rust
const SIOCGPGRP: u32 = 35_076u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:185`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L185)*

### `SIOCATMARK`
```rust
const SIOCATMARK: u32 = 35_077u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:186`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L186)*

### `SIOCGSTAMP`
```rust
const SIOCGSTAMP: u32 = 35_078u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:187`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L187)*

### `TIOCINQ`
```rust
const TIOCINQ: u32 = 21_531u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:188`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L188)*

### `SIOCADDRT`
```rust
const SIOCADDRT: u32 = 35_083u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:189`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L189)*

### `SIOCDELRT`
```rust
const SIOCDELRT: u32 = 35_084u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:190`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L190)*

### `SIOCGIFNAME`
```rust
const SIOCGIFNAME: u32 = 35_088u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:191`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L191)*

### `SIOCSIFLINK`
```rust
const SIOCSIFLINK: u32 = 35_089u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:192`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L192)*

### `SIOCGIFCONF`
```rust
const SIOCGIFCONF: u32 = 35_090u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:193`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L193)*

### `SIOCGIFFLAGS`
```rust
const SIOCGIFFLAGS: u32 = 35_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:194`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L194)*

### `SIOCSIFFLAGS`
```rust
const SIOCSIFFLAGS: u32 = 35_092u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:195`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L195)*

### `SIOCGIFADDR`
```rust
const SIOCGIFADDR: u32 = 35_093u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:196`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L196)*

### `SIOCSIFADDR`
```rust
const SIOCSIFADDR: u32 = 35_094u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:197`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L197)*

### `SIOCGIFDSTADDR`
```rust
const SIOCGIFDSTADDR: u32 = 35_095u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:198`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L198)*

### `SIOCSIFDSTADDR`
```rust
const SIOCSIFDSTADDR: u32 = 35_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:199`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L199)*

### `SIOCGIFBRDADDR`
```rust
const SIOCGIFBRDADDR: u32 = 35_097u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:200`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L200)*

### `SIOCSIFBRDADDR`
```rust
const SIOCSIFBRDADDR: u32 = 35_098u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:201`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L201)*

### `SIOCGIFNETMASK`
```rust
const SIOCGIFNETMASK: u32 = 35_099u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:202`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L202)*

### `SIOCSIFNETMASK`
```rust
const SIOCSIFNETMASK: u32 = 35_100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:203`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L203)*

### `SIOCGIFMETRIC`
```rust
const SIOCGIFMETRIC: u32 = 35_101u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:204`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L204)*

### `SIOCSIFMETRIC`
```rust
const SIOCSIFMETRIC: u32 = 35_102u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:205`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L205)*

### `SIOCGIFMEM`
```rust
const SIOCGIFMEM: u32 = 35_103u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:206`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L206)*

### `SIOCSIFMEM`
```rust
const SIOCSIFMEM: u32 = 35_104u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:207`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L207)*

### `SIOCGIFMTU`
```rust
const SIOCGIFMTU: u32 = 35_105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:208`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L208)*

### `SIOCSIFMTU`
```rust
const SIOCSIFMTU: u32 = 35_106u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:209`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L209)*

### `SIOCSIFHWADDR`
```rust
const SIOCSIFHWADDR: u32 = 35_108u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:210`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L210)*

### `SIOCGIFENCAP`
```rust
const SIOCGIFENCAP: u32 = 35_109u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:211`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L211)*

### `SIOCSIFENCAP`
```rust
const SIOCSIFENCAP: u32 = 35_110u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:212`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L212)*

### `SIOCGIFHWADDR`
```rust
const SIOCGIFHWADDR: u32 = 35_111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:213`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L213)*

### `SIOCGIFSLAVE`
```rust
const SIOCGIFSLAVE: u32 = 35_113u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:214`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L214)*

### `SIOCSIFSLAVE`
```rust
const SIOCSIFSLAVE: u32 = 35_120u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:215`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L215)*

### `SIOCADDMULTI`
```rust
const SIOCADDMULTI: u32 = 35_121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:216`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L216)*

### `SIOCDELMULTI`
```rust
const SIOCDELMULTI: u32 = 35_122u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:217`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L217)*

### `SIOCDARP`
```rust
const SIOCDARP: u32 = 35_155u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:218`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L218)*

### `SIOCGARP`
```rust
const SIOCGARP: u32 = 35_156u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:219`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L219)*

### `SIOCSARP`
```rust
const SIOCSARP: u32 = 35_157u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:220`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L220)*

### `SIOCDRARP`
```rust
const SIOCDRARP: u32 = 35_168u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:221`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L221)*

### `SIOCGRARP`
```rust
const SIOCGRARP: u32 = 35_169u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:222`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L222)*

### `SIOCSRARP`
```rust
const SIOCSRARP: u32 = 35_170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:223`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L223)*

### `SIOCGIFMAP`
```rust
const SIOCGIFMAP: u32 = 35_184u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:224`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L224)*

### `SIOCSIFMAP`
```rust
const SIOCSIFMAP: u32 = 35_185u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:225`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L225)*

### `SIOCRTMSG`
```rust
const SIOCRTMSG: u32 = 35_085u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:226`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L226)*

### `SIOCSIFNAME`
```rust
const SIOCSIFNAME: u32 = 35_107u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:227`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L227)*

### `SIOCGIFINDEX`
```rust
const SIOCGIFINDEX: u32 = 35_123u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:228`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L228)*

### `SIOGIFINDEX`
```rust
const SIOGIFINDEX: u32 = 35_123u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:229`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L229)*

### `SIOCSIFPFLAGS`
```rust
const SIOCSIFPFLAGS: u32 = 35_124u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:230`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L230)*

### `SIOCGIFPFLAGS`
```rust
const SIOCGIFPFLAGS: u32 = 35_125u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:231`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L231)*

### `SIOCDIFADDR`
```rust
const SIOCDIFADDR: u32 = 35_126u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:232`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L232)*

### `SIOCSIFHWBROADCAST`
```rust
const SIOCSIFHWBROADCAST: u32 = 35_127u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:233`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L233)*

### `SIOCGIFCOUNT`
```rust
const SIOCGIFCOUNT: u32 = 35_128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:234`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L234)*

### `SIOCGIFBR`
```rust
const SIOCGIFBR: u32 = 35_136u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:235`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L235)*

### `SIOCSIFBR`
```rust
const SIOCSIFBR: u32 = 35_137u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:236`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L236)*

### `SIOCGIFTXQLEN`
```rust
const SIOCGIFTXQLEN: u32 = 35_138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:237`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L237)*

### `SIOCSIFTXQLEN`
```rust
const SIOCSIFTXQLEN: u32 = 35_139u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:238`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L238)*

### `SIOCADDDLCI`
```rust
const SIOCADDDLCI: u32 = 35_200u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:239`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L239)*

### `SIOCDELDLCI`
```rust
const SIOCDELDLCI: u32 = 35_201u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:240`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L240)*

### `SIOCDEVPRIVATE`
```rust
const SIOCDEVPRIVATE: u32 = 35_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:241`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L241)*

### `SIOCPROTOPRIVATE`
```rust
const SIOCPROTOPRIVATE: u32 = 35_296u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:242`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L242)*

### `FIBMAP`
```rust
const FIBMAP: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:243`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L243)*

### `FIGETBSZ`
```rust
const FIGETBSZ: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:244`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L244)*

### `FIFREEZE`
```rust
const FIFREEZE: u32 = 3_221_510_263u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:245`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L245)*

### `FITHAW`
```rust
const FITHAW: u32 = 3_221_510_264u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:246`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L246)*

### `FITRIM`
```rust
const FITRIM: u32 = 3_222_820_985u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:247`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L247)*

### `FICLONE`
```rust
const FICLONE: u32 = 1_074_041_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:248`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L248)*

### `FICLONERANGE`
```rust
const FICLONERANGE: u32 = 1_075_876_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:249`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L249)*

### `FIDEDUPERANGE`
```rust
const FIDEDUPERANGE: u32 = 3_222_836_278u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:250`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L250)*

### `FS_IOC_GETFLAGS`
```rust
const FS_IOC_GETFLAGS: u32 = 2_148_034_049u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:251`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L251)*

### `FS_IOC_SETFLAGS`
```rust
const FS_IOC_SETFLAGS: u32 = 1_074_292_226u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:252`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L252)*

### `FS_IOC_GETVERSION`
```rust
const FS_IOC_GETVERSION: u32 = 2_148_038_145u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:253`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L253)*

### `FS_IOC_SETVERSION`
```rust
const FS_IOC_SETVERSION: u32 = 1_074_296_322u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:254`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L254)*

### `FS_IOC_FIEMAP`
```rust
const FS_IOC_FIEMAP: u32 = 3_223_348_747u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:255`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L255)*

### `FS_IOC32_GETFLAGS`
```rust
const FS_IOC32_GETFLAGS: u32 = 2_147_771_905u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:256`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L256)*

### `FS_IOC32_SETFLAGS`
```rust
const FS_IOC32_SETFLAGS: u32 = 1_074_030_082u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:257`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L257)*

### `FS_IOC32_GETVERSION`
```rust
const FS_IOC32_GETVERSION: u32 = 2_147_776_001u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:258`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L258)*

### `FS_IOC32_SETVERSION`
```rust
const FS_IOC32_SETVERSION: u32 = 1_074_034_178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:259`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L259)*

### `FS_IOC_FSGETXATTR`
```rust
const FS_IOC_FSGETXATTR: u32 = 2_149_341_215u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:260`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L260)*

### `FS_IOC_FSSETXATTR`
```rust
const FS_IOC_FSSETXATTR: u32 = 1_075_599_392u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:261`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L261)*

### `FS_IOC_GETFSLABEL`
```rust
const FS_IOC_GETFSLABEL: u32 = 2_164_298_801u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:262`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L262)*

### `FS_IOC_SETFSLABEL`
```rust
const FS_IOC_SETFSLABEL: u32 = 1_090_556_978u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:263`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L263)*

### `EXT4_IOC_GETVERSION`
```rust
const EXT4_IOC_GETVERSION: u32 = 2_148_034_051u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:264`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L264)*

### `EXT4_IOC_SETVERSION`
```rust
const EXT4_IOC_SETVERSION: u32 = 1_074_292_228u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:265`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L265)*

### `EXT4_IOC_GETVERSION_OLD`
```rust
const EXT4_IOC_GETVERSION_OLD: u32 = 2_148_038_145u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:266`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L266)*

### `EXT4_IOC_SETVERSION_OLD`
```rust
const EXT4_IOC_SETVERSION_OLD: u32 = 1_074_296_322u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:267`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L267)*

### `EXT4_IOC_GETRSVSZ`
```rust
const EXT4_IOC_GETRSVSZ: u32 = 2_148_034_053u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:268`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L268)*

### `EXT4_IOC_SETRSVSZ`
```rust
const EXT4_IOC_SETRSVSZ: u32 = 1_074_292_230u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:269`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L269)*

### `EXT4_IOC_GROUP_EXTEND`
```rust
const EXT4_IOC_GROUP_EXTEND: u32 = 1_074_292_231u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:270`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L270)*

### `EXT4_IOC_MIGRATE`
```rust
const EXT4_IOC_MIGRATE: u32 = 26_121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:271`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L271)*

### `EXT4_IOC_ALLOC_DA_BLKS`
```rust
const EXT4_IOC_ALLOC_DA_BLKS: u32 = 26_124u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:272`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L272)*

### `EXT4_IOC_RESIZE_FS`
```rust
const EXT4_IOC_RESIZE_FS: u32 = 1_074_292_240u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:273`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L273)*

### `EXT4_IOC_SWAP_BOOT`
```rust
const EXT4_IOC_SWAP_BOOT: u32 = 26_129u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:274`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L274)*

### `EXT4_IOC_PRECACHE_EXTENTS`
```rust
const EXT4_IOC_PRECACHE_EXTENTS: u32 = 26_130u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:275`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L275)*

### `EXT4_IOC_CLEAR_ES_CACHE`
```rust
const EXT4_IOC_CLEAR_ES_CACHE: u32 = 26_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:276`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L276)*

### `EXT4_IOC_GETSTATE`
```rust
const EXT4_IOC_GETSTATE: u32 = 1_074_030_121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:277`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L277)*

### `EXT4_IOC_GET_ES_CACHE`
```rust
const EXT4_IOC_GET_ES_CACHE: u32 = 3_223_348_778u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:278`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L278)*

### `EXT4_IOC_CHECKPOINT`
```rust
const EXT4_IOC_CHECKPOINT: u32 = 1_074_030_123u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:279`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L279)*

### `EXT4_IOC_SHUTDOWN`
```rust
const EXT4_IOC_SHUTDOWN: u32 = 2_147_768_445u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:280`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L280)*

### `EXT4_IOC32_GETVERSION`
```rust
const EXT4_IOC32_GETVERSION: u32 = 2_147_771_907u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:281`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L281)*

### `EXT4_IOC32_SETVERSION`
```rust
const EXT4_IOC32_SETVERSION: u32 = 1_074_030_084u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:282`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L282)*

### `EXT4_IOC32_GETRSVSZ`
```rust
const EXT4_IOC32_GETRSVSZ: u32 = 2_147_771_909u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:283`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L283)*

### `EXT4_IOC32_SETRSVSZ`
```rust
const EXT4_IOC32_SETRSVSZ: u32 = 1_074_030_086u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:284`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L284)*

### `EXT4_IOC32_GROUP_EXTEND`
```rust
const EXT4_IOC32_GROUP_EXTEND: u32 = 1_074_030_087u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:285`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L285)*

### `EXT4_IOC32_GETVERSION_OLD`
```rust
const EXT4_IOC32_GETVERSION_OLD: u32 = 2_147_776_001u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:286`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L286)*

### `EXT4_IOC32_SETVERSION_OLD`
```rust
const EXT4_IOC32_SETVERSION_OLD: u32 = 1_074_034_178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:287`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L287)*

### `VIDIOC_SUBDEV_QUERYSTD`
```rust
const VIDIOC_SUBDEV_QUERYSTD: u32 = 2_148_030_015u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:288`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L288)*

### `AUTOFS_DEV_IOCTL_CLOSEMOUNT`
```rust
const AUTOFS_DEV_IOCTL_CLOSEMOUNT: u32 = 3_222_836_085u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:289`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L289)*

### `LIRC_SET_SEND_CARRIER`
```rust
const LIRC_SET_SEND_CARRIER: u32 = 1_074_030_867u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:290`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L290)*

### `AUTOFS_IOC_PROTOSUBVER`
```rust
const AUTOFS_IOC_PROTOSUBVER: u32 = 2_147_783_527u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:291`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L291)*

### `PTP_SYS_OFFSET_PRECISE`
```rust
const PTP_SYS_OFFSET_PRECISE: u32 = 3_225_435_400u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:292`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L292)*

### `FSI_SCOM_WRITE`
```rust
const FSI_SCOM_WRITE: u32 = 3_223_352_066u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:293`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L293)*

### `ATM_GETCIRANGE`
```rust
const ATM_GETCIRANGE: u32 = 1_074_815_370u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:294`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L294)*

### `DMA_BUF_SET_NAME_B`
```rust
const DMA_BUF_SET_NAME_B: u32 = 1_074_291_201u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:295`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L295)*

### `RIO_CM_EP_GET_LIST_SIZE`
```rust
const RIO_CM_EP_GET_LIST_SIZE: u32 = 3_221_512_961u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:296`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L296)*

### `TUNSETPERSIST`
```rust
const TUNSETPERSIST: u32 = 1_074_025_675u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:297`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L297)*

### `FS_IOC_GET_ENCRYPTION_POLICY`
```rust
const FS_IOC_GET_ENCRYPTION_POLICY: u32 = 1_074_554_389u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:298`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L298)*

### `CEC_RECEIVE`
```rust
const CEC_RECEIVE: u32 = 3_224_920_326u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:299`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L299)*

### `MGSL_IOCGPARAMS`
```rust
const MGSL_IOCGPARAMS: u32 = 2_150_657_281u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:300`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L300)*

### `ENI_SETMULT`
```rust
const ENI_SETMULT: u32 = 1_074_815_335u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:301`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L301)*

### `RIO_GET_EVENT_MASK`
```rust
const RIO_GET_EVENT_MASK: u32 = 2_147_773_710u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:302`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L302)*

### `LIRC_GET_MAX_TIMEOUT`
```rust
const LIRC_GET_MAX_TIMEOUT: u32 = 2_147_772_681u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:303`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L303)*

### `USBDEVFS_CLAIMINTERFACE`
```rust
const USBDEVFS_CLAIMINTERFACE: u32 = 2_147_767_567u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:304`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L304)*

### `CHIOMOVE`
```rust
const CHIOMOVE: u32 = 1_075_077_889u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:305`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L305)*

### `SONYPI_IOCGBATFLAGS`
```rust
const SONYPI_IOCGBATFLAGS: u32 = 2_147_579_399u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:306`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L306)*

### `BTRFS_IOC_SYNC`
```rust
const BTRFS_IOC_SYNC: u32 = 37_896u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:307`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L307)*

### `VIDIOC_TRY_FMT`
```rust
const VIDIOC_TRY_FMT: u32 = 3_234_879_040u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:308`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L308)*

### `LIRC_SET_REC_MODE`
```rust
const LIRC_SET_REC_MODE: u32 = 1_074_030_866u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:309`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L309)*

### `VIDIOC_DQEVENT`
```rust
const VIDIOC_DQEVENT: u32 = 2_156_418_649u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:310`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L310)*

### `RPMSG_DESTROY_EPT_IOCTL`
```rust
const RPMSG_DESTROY_EPT_IOCTL: u32 = 46_338u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:311`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L311)*

### `UVCIOC_CTRL_MAP`
```rust
const UVCIOC_CTRL_MAP: u32 = 3_227_546_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:312`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L312)*

### `VHOST_SET_BACKEND_FEATURES`
```rust
const VHOST_SET_BACKEND_FEATURES: u32 = 1_074_310_949u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:313`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L313)*

### `VHOST_VSOCK_SET_GUEST_CID`
```rust
const VHOST_VSOCK_SET_GUEST_CID: u32 = 1_074_311_008u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:314`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L314)*

### `UI_SET_KEYBIT`
```rust
const UI_SET_KEYBIT: u32 = 1_074_025_829u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:315`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L315)*

### `LIRC_SET_REC_TIMEOUT`
```rust
const LIRC_SET_REC_TIMEOUT: u32 = 1_074_030_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:316`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L316)*

### `FS_IOC_GET_ENCRYPTION_KEY_STATUS`
```rust
const FS_IOC_GET_ENCRYPTION_KEY_STATUS: u32 = 3_229_640_218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:317`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L317)*

### `BTRFS_IOC_TREE_SEARCH_V2`
```rust
const BTRFS_IOC_TREE_SEARCH_V2: u32 = 3_228_603_409u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:318`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L318)*

### `VHOST_SET_VRING_BASE`
```rust
const VHOST_SET_VRING_BASE: u32 = 1_074_310_930u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:319`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L319)*

### `RIO_ENABLE_DOORBELL_RANGE`
```rust
const RIO_ENABLE_DOORBELL_RANGE: u32 = 1_074_294_025u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:320`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L320)*

### `VIDIOC_TRY_EXT_CTRLS`
```rust
const VIDIOC_TRY_EXT_CTRLS: u32 = 3_223_344_713u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:321`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L321)*

### `LIRC_GET_REC_MODE`
```rust
const LIRC_GET_REC_MODE: u32 = 2_147_772_674u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:322`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L322)*

### `PPGETTIME`
```rust
const PPGETTIME: u32 = 2_148_561_045u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:323`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L323)*

### `BTRFS_IOC_RM_DEV`
```rust
const BTRFS_IOC_RM_DEV: u32 = 1_342_215_179u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:324`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L324)*

### `ATM_SETBACKEND`
```rust
const ATM_SETBACKEND: u32 = 1_073_897_970u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:325`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L325)*

### `FSL_HV_IOCTL_PARTITION_START`
```rust
const FSL_HV_IOCTL_PARTITION_START: u32 = 3_222_318_851u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:326`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L326)*

### `FBIO_WAITEVENT`
```rust
const FBIO_WAITEVENT: u32 = 18_056u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:327`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L327)*

### `SWITCHTEC_IOCTL_PORT_TO_PFF`
```rust
const SWITCHTEC_IOCTL_PORT_TO_PFF: u32 = 3_222_034_245u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:328`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L328)*

### `NVME_IOCTL_IO_CMD`
```rust
const NVME_IOCTL_IO_CMD: u32 = 3_225_964_099u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:329`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L329)*

### `IPMICTL_RECEIVE_MSG_TRUNC`
```rust
const IPMICTL_RECEIVE_MSG_TRUNC: u32 = 3_224_398_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:330`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L330)*

### `FDTWADDLE`
```rust
const FDTWADDLE: u32 = 601u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:331`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L331)*

### `NVME_IOCTL_SUBMIT_IO`
```rust
const NVME_IOCTL_SUBMIT_IO: u32 = 1_076_907_586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:332`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L332)*

### `NILFS_IOCTL_SYNC`
```rust
const NILFS_IOCTL_SYNC: u32 = 2_148_036_234u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:333`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L333)*

### `VIDIOC_SUBDEV_S_DV_TIMINGS`
```rust
const VIDIOC_SUBDEV_S_DV_TIMINGS: u32 = 3_229_898_327u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:334`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L334)*

### `ASPEED_LPC_CTRL_IOCTL_GET_SIZE`
```rust
const ASPEED_LPC_CTRL_IOCTL_GET_SIZE: u32 = 3_222_319_616u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:335`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L335)*

### `DM_DEV_STATUS`
```rust
const DM_DEV_STATUS: u32 = 3_241_737_479u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:336`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L336)*

### `TEE_IOC_CLOSE_SESSION`
```rust
const TEE_IOC_CLOSE_SESSION: u32 = 2_147_787_781u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:337`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L337)*

### `NS_GETPSTAT`
```rust
const NS_GETPSTAT: u32 = 3_222_298_977u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:338`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L338)*

### `UI_SET_PROPBIT`
```rust
const UI_SET_PROPBIT: u32 = 1_074_025_838u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:339`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L339)*

### `TUNSETFILTEREBPF`
```rust
const TUNSETFILTEREBPF: u32 = 2_147_767_521u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:340`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L340)*

### `RIO_MPORT_MAINT_COMPTAG_SET`
```rust
const RIO_MPORT_MAINT_COMPTAG_SET: u32 = 1_074_031_874u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:341`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L341)*

### `AUTOFS_DEV_IOCTL_VERSION`
```rust
const AUTOFS_DEV_IOCTL_VERSION: u32 = 3_222_836_081u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:342`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L342)*

### `WDIOC_SETOPTIONS`
```rust
const WDIOC_SETOPTIONS: u32 = 2_147_768_068u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:343`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L343)*

### `VHOST_SCSI_SET_ENDPOINT`
```rust
const VHOST_SCSI_SET_ENDPOINT: u32 = 1_088_991_040u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:344`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L344)*

### `MGSL_IOCGTXIDLE`
```rust
const MGSL_IOCGTXIDLE: u32 = 27_907u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:345`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L345)*

### `ATM_ADDLECSADDR`
```rust
const ATM_ADDLECSADDR: u32 = 1_074_815_374u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:346`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L346)*

### `FSL_HV_IOCTL_GETPROP`
```rust
const FSL_HV_IOCTL_GETPROP: u32 = 3_223_891_719u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:347`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L347)*

### `FDGETPRM`
```rust
const FDGETPRM: u32 = 2_149_581_316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:348`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L348)*

### `HIDIOCAPPLICATION`
```rust
const HIDIOCAPPLICATION: u32 = 18_434u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:349`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L349)*

### `ENI_MEMDUMP`
```rust
const ENI_MEMDUMP: u32 = 1_074_815_328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:350`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L350)*

### `PTP_SYS_OFFSET2`
```rust
const PTP_SYS_OFFSET2: u32 = 1_128_283_406u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:351`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L351)*

### `VIDIOC_SUBDEV_G_DV_TIMINGS`
```rust
const VIDIOC_SUBDEV_G_DV_TIMINGS: u32 = 3_229_898_328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:352`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L352)*

### `DMA_BUF_SET_NAME_A`
```rust
const DMA_BUF_SET_NAME_A: u32 = 1_074_029_057u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:353`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L353)*

### `PTP_PIN_GETFUNC`
```rust
const PTP_PIN_GETFUNC: u32 = 3_227_532_550u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:354`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L354)*

### `PTP_SYS_OFFSET_EXTENDED`
```rust
const PTP_SYS_OFFSET_EXTENDED: u32 = 3_300_932_873u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:355`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L355)*

### `DFL_FPGA_PORT_UINT_SET_IRQ`
```rust
const DFL_FPGA_PORT_UINT_SET_IRQ: u32 = 1_074_312_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:356`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L356)*

### `RTC_EPOCH_READ`
```rust
const RTC_EPOCH_READ: u32 = 2_148_036_621u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:357`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L357)*

### `VIDIOC_SUBDEV_S_SELECTION`
```rust
const VIDIOC_SUBDEV_S_SELECTION: u32 = 3_225_441_854u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:358`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L358)*

### `VIDIOC_QUERY_EXT_CTRL`
```rust
const VIDIOC_QUERY_EXT_CTRL: u32 = 3_236_451_943u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:359`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L359)*

### `ATM_GETLECSADDR`
```rust
const ATM_GETLECSADDR: u32 = 1_074_815_376u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:360`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L360)*

### `FSL_HV_IOCTL_PARTITION_STOP`
```rust
const FSL_HV_IOCTL_PARTITION_STOP: u32 = 3_221_794_564u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:361`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L361)*

### `SONET_GETDIAG`
```rust
const SONET_GETDIAG: u32 = 2_147_770_644u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:362`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L362)*

### `ATMMPC_DATA`
```rust
const ATMMPC_DATA: u32 = 25_049u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:363`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L363)*

### `IPMICTL_UNREGISTER_FOR_CMD_CHANS`
```rust
const IPMICTL_UNREGISTER_FOR_CMD_CHANS: u32 = 2_148_296_989u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:364`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L364)*

### `HIDIOCGCOLLECTIONINDEX`
```rust
const HIDIOCGCOLLECTIONINDEX: u32 = 1_075_333_136u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:365`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L365)*

### `RPMSG_CREATE_EPT_IOCTL`
```rust
const RPMSG_CREATE_EPT_IOCTL: u32 = 1_076_409_601u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:366`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L366)*

### `GPIOHANDLE_GET_LINE_VALUES_IOCTL`
```rust
const GPIOHANDLE_GET_LINE_VALUES_IOCTL: u32 = 3_225_465_864u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:367`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L367)*

### `UI_DEV_SETUP`
```rust
const UI_DEV_SETUP: u32 = 1_079_792_899u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:368`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L368)*

### `ISST_IF_IO_CMD`
```rust
const ISST_IF_IO_CMD: u32 = 1_074_331_138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:369`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L369)*

### `RIO_MPORT_MAINT_READ_REMOTE`
```rust
const RIO_MPORT_MAINT_READ_REMOTE: u32 = 2_149_084_423u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:370`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L370)*

### `VIDIOC_OMAP3ISP_HIST_CFG`
```rust
const VIDIOC_OMAP3ISP_HIST_CFG: u32 = 3_224_393_412u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:371`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L371)*

### `BLKGETNRZONES`
```rust
const BLKGETNRZONES: u32 = 2_147_750_533u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:372`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L372)*

### `VIDIOC_G_MODULATOR`
```rust
const VIDIOC_G_MODULATOR: u32 = 3_225_703_990u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:373`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L373)*

### `VBG_IOCTL_WRITE_CORE_DUMP`
```rust
const VBG_IOCTL_WRITE_CORE_DUMP: u32 = 3_223_082_515u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:374`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L374)*

### `USBDEVFS_SETINTERFACE`
```rust
const USBDEVFS_SETINTERFACE: u32 = 2_148_029_700u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:375`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L375)*

### `PPPIOCGCHAN`
```rust
const PPPIOCGCHAN: u32 = 2_147_775_543u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:376`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L376)*

### `EVIOCGVERSION`
```rust
const EVIOCGVERSION: u32 = 2_147_763_457u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:377`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L377)*

### `VHOST_NET_SET_BACKEND`
```rust
const VHOST_NET_SET_BACKEND: u32 = 1_074_310_960u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:378`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L378)*

### `USBDEVFS_REAPURBNDELAY`
```rust
const USBDEVFS_REAPURBNDELAY: u32 = 1_074_287_885u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:379`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L379)*

### `RNDZAPENTCNT`
```rust
const RNDZAPENTCNT: u32 = 20_996u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:380`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L380)*

### `VIDIOC_G_PARM`
```rust
const VIDIOC_G_PARM: u32 = 3_234_616_853u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:381`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L381)*

### `TUNGETDEVNETNS`
```rust
const TUNGETDEVNETNS: u32 = 21_731u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:382`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L382)*

### `LIRC_SET_MEASURE_CARRIER_MODE`
```rust
const LIRC_SET_MEASURE_CARRIER_MODE: u32 = 1_074_030_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:383`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L383)*

### `VHOST_SET_VRING_ERR`
```rust
const VHOST_SET_VRING_ERR: u32 = 1_074_310_946u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:384`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L384)*

### `VDUSE_VQ_SETUP`
```rust
const VDUSE_VQ_SETUP: u32 = 1_075_872_020u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:385`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L385)*

### `AUTOFS_IOC_SETTIMEOUT`
```rust
const AUTOFS_IOC_SETTIMEOUT: u32 = 3_221_787_492u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:386`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L386)*

### `VIDIOC_S_FREQUENCY`
```rust
const VIDIOC_S_FREQUENCY: u32 = 1_076_647_481u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:387`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L387)*

### `F2FS_IOC_SEC_TRIM_FILE`
```rust
const F2FS_IOC_SEC_TRIM_FILE: u32 = 1_075_377_428u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:388`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L388)*

### `FS_IOC_REMOVE_ENCRYPTION_KEY`
```rust
const FS_IOC_REMOVE_ENCRYPTION_KEY: u32 = 3_225_445_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:389`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L389)*

### `WDIOC_GETPRETIMEOUT`
```rust
const WDIOC_GETPRETIMEOUT: u32 = 2_147_768_073u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:390`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L390)*

### `USBDEVFS_DROP_PRIVILEGES`
```rust
const USBDEVFS_DROP_PRIVILEGES: u32 = 1_074_025_758u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:391`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L391)*

### `BTRFS_IOC_SNAP_CREATE_V2`
```rust
const BTRFS_IOC_SNAP_CREATE_V2: u32 = 1_342_215_191u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:392`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L392)*

### `VHOST_VSOCK_SET_RUNNING`
```rust
const VHOST_VSOCK_SET_RUNNING: u32 = 1_074_048_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:393`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L393)*

### `STP_SET_OPTIONS`
```rust
const STP_SET_OPTIONS: u32 = 1_074_275_586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:394`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L394)*

### `FBIO_RADEON_GET_MIRROR`
```rust
const FBIO_RADEON_GET_MIRROR: u32 = 2_148_024_323u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:395`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L395)*

### `IVTVFB_IOC_DMA_FRAME`
```rust
const IVTVFB_IOC_DMA_FRAME: u32 = 1_075_336_896u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:396`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L396)*

### `IPMICTL_SEND_COMMAND`
```rust
const IPMICTL_SEND_COMMAND: u32 = 2_150_131_981u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:397`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L397)*

### `VIDIOC_G_ENC_INDEX`
```rust
const VIDIOC_G_ENC_INDEX: u32 = 2_283_296_332u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:398`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L398)*

### `DFL_FPGA_FME_PORT_PR`
```rust
const DFL_FPGA_FME_PORT_PR: u32 = 46_720u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:399`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L399)*

### `CHIOSVOLTAG`
```rust
const CHIOSVOLTAG: u32 = 1_076_912_914u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:400`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L400)*

### `ATM_SETESIF`
```rust
const ATM_SETESIF: u32 = 1_074_815_373u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:401`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L401)*

### `FW_CDEV_IOC_SEND_RESPONSE`
```rust
const FW_CDEV_IOC_SEND_RESPONSE: u32 = 1_075_323_652u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:402`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L402)*

### `PMU_IOC_GET_MODEL`
```rust
const PMU_IOC_GET_MODEL: u32 = 2_148_024_835u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:403`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L403)*

### `JSIOCGBTNMAP`
```rust
const JSIOCGBTNMAP: u32 = 2_214_619_700u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:404`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L404)*

### `USBDEVFS_HUB_PORTINFO`
```rust
const USBDEVFS_HUB_PORTINFO: u32 = 2_155_894_035u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:405`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L405)*

### `VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS`
```rust
const VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS: u32 = 3_222_820_363u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:406`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L406)*

### `FDCLRPRM`
```rust
const FDCLRPRM: u32 = 577u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:407`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L407)*

### `BTRFS_IOC_SCRUB`
```rust
const BTRFS_IOC_SCRUB: u32 = 3_288_372_251u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:408`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L408)*

### `USBDEVFS_DISCONNECT`
```rust
const USBDEVFS_DISCONNECT: u32 = 21_782u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:409`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L409)*

### `TUNSETVNETBE`
```rust
const TUNSETVNETBE: u32 = 1_074_025_694u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:410`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L410)*

### `ATMTCP_REMOVE`
```rust
const ATMTCP_REMOVE: u32 = 24_975u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:411`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L411)*

### `VHOST_VDPA_GET_CONFIG`
```rust
const VHOST_VDPA_GET_CONFIG: u32 = 2_148_052_851u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:412`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L412)*

### `PPPIOCGNPMODE`
```rust
const PPPIOCGNPMODE: u32 = 3_221_779_532u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:413`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L413)*

### `FDGETDRVPRM`
```rust
const FDGETDRVPRM: u32 = 2_155_872_785u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:414`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L414)*

### `TUNSETVNETLE`
```rust
const TUNSETVNETLE: u32 = 1_074_025_692u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:415`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L415)*

### `PHN_SETREG`
```rust
const PHN_SETREG: u32 = 1_074_294_790u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:416`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L416)*

### `PPPIOCDETACH`
```rust
const PPPIOCDETACH: u32 = 1_074_033_724u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:417`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L417)*

### `MMTIMER_GETRES`
```rust
const MMTIMER_GETRES: u32 = 2_148_035_841u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:418`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L418)*

### `VIDIOC_SUBDEV_ENUMSTD`
```rust
const VIDIOC_SUBDEV_ENUMSTD: u32 = 3_225_966_105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:419`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L419)*

### `PPGETFLAGS`
```rust
const PPGETFLAGS: u32 = 2_147_774_618u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:420`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L420)*

### `VDUSE_DEV_GET_FEATURES`
```rust
const VDUSE_DEV_GET_FEATURES: u32 = 2_148_040_977u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:421`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L421)*

### `CAPI_MANUFACTURER_CMD`
```rust
const CAPI_MANUFACTURER_CMD: u32 = 3_222_291_232u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:422`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L422)*

### `VIDIOC_G_TUNER`
```rust
const VIDIOC_G_TUNER: u32 = 3_226_752_541u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:423`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L423)*

### `DM_TABLE_STATUS`
```rust
const DM_TABLE_STATUS: u32 = 3_241_737_484u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:424`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L424)*

### `DM_DEV_ARM_POLL`
```rust
const DM_DEV_ARM_POLL: u32 = 3_241_737_488u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:425`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L425)*

### `NE_CREATE_VM`
```rust
const NE_CREATE_VM: u32 = 2_148_052_512u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:426`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L426)*

### `MEDIA_IOC_ENUM_LINKS`
```rust
const MEDIA_IOC_ENUM_LINKS: u32 = 3_223_878_658u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:427`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L427)*

### `F2FS_IOC_PRECACHE_EXTENTS`
```rust
const F2FS_IOC_PRECACHE_EXTENTS: u32 = 62_735u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:428`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L428)*

### `DFL_FPGA_PORT_DMA_MAP`
```rust
const DFL_FPGA_PORT_DMA_MAP: u32 = 46_659u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:429`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L429)*

### `MGSL_IOCGXCTRL`
```rust
const MGSL_IOCGXCTRL: u32 = 27_926u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:430`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L430)*

### `FW_CDEV_IOC_SEND_REQUEST`
```rust
const FW_CDEV_IOC_SEND_REQUEST: u32 = 1_076_372_225u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:431`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L431)*

### `SONYPI_IOCGBLUE`
```rust
const SONYPI_IOCGBLUE: u32 = 2_147_579_400u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:432`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L432)*

### `F2FS_IOC_DECOMPRESS_FILE`
```rust
const F2FS_IOC_DECOMPRESS_FILE: u32 = 62_743u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:433`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L433)*

### `I2OHTML`
```rust
const I2OHTML: u32 = 3_224_398_089u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:434`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L434)*

### `VFIO_GET_API_VERSION`
```rust
const VFIO_GET_API_VERSION: u32 = 15_204u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:435`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L435)*

### `IDT77105_GETSTATZ`
```rust
const IDT77105_GETSTATZ: u32 = 1_074_815_283u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:436`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L436)*

### `I2OPARMSET`
```rust
const I2OPARMSET: u32 = 3_223_873_795u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:437`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L437)*

### `TEE_IOC_CANCEL`
```rust
const TEE_IOC_CANCEL: u32 = 2_148_049_924u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:438`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L438)*

### `PTP_SYS_OFFSET_PRECISE2`
```rust
const PTP_SYS_OFFSET_PRECISE2: u32 = 3_225_435_409u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:439`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L439)*

### `DFL_FPGA_PORT_RESET`
```rust
const DFL_FPGA_PORT_RESET: u32 = 46_656u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:440`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L440)*

### `PPPIOCGASYNCMAP`
```rust
const PPPIOCGASYNCMAP: u32 = 2_147_775_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:441`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L441)*

### `EVIOCGKEYCODE_V2`
```rust
const EVIOCGKEYCODE_V2: u32 = 2_150_122_756u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:442`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L442)*

### `DM_DEV_SET_GEOMETRY`
```rust
const DM_DEV_SET_GEOMETRY: u32 = 3_241_737_487u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:443`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L443)*

### `HIDIOCSUSAGE`
```rust
const HIDIOCSUSAGE: u32 = 1_075_333_132u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:444`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L444)*

### `FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE`
```rust
const FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE: u32 = 1_075_323_664u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:445`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L445)*

### `PTP_EXTTS_REQUEST`
```rust
const PTP_EXTTS_REQUEST: u32 = 1_074_806_018u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:446`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L446)*

### `SWITCHTEC_IOCTL_EVENT_CTL`
```rust
const SWITCHTEC_IOCTL_EVENT_CTL: u32 = 3_223_869_251u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:447`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L447)*

### `WDIOC_SETPRETIMEOUT`
```rust
const WDIOC_SETPRETIMEOUT: u32 = 3_221_509_896u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:448`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L448)*

### `VHOST_SCSI_CLEAR_ENDPOINT`
```rust
const VHOST_SCSI_CLEAR_ENDPOINT: u32 = 1_088_991_041u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:449`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L449)*

### `JSIOCGAXES`
```rust
const JSIOCGAXES: u32 = 2_147_576_337u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:450`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L450)*

### `HIDIOCSFLAG`
```rust
const HIDIOCSFLAG: u32 = 1_074_022_415u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:451`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L451)*

### `PTP_PEROUT_REQUEST2`
```rust
const PTP_PEROUT_REQUEST2: u32 = 1_077_427_468u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:452`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L452)*

### `PPWDATA`
```rust
const PPWDATA: u32 = 1_073_836_166u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:453`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L453)*

### `PTP_CLOCK_GETCAPS`
```rust
const PTP_CLOCK_GETCAPS: u32 = 2_152_742_145u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:454`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L454)*

### `FDGETMAXERRS`
```rust
const FDGETMAXERRS: u32 = 2_148_794_894u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:455`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L455)*

### `TUNSETQUEUE`
```rust
const TUNSETQUEUE: u32 = 1_074_025_689u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:456`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L456)*

### `PTP_ENABLE_PPS`
```rust
const PTP_ENABLE_PPS: u32 = 1_074_019_588u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:457`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L457)*

### `SIOCSIFATMTCP`
```rust
const SIOCSIFATMTCP: u32 = 24_960u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:458`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L458)*

### `CEC_ADAP_G_LOG_ADDRS`
```rust
const CEC_ADAP_G_LOG_ADDRS: u32 = 2_153_537_795u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:459`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L459)*

### `ND_IOCTL_ARS_CAP`
```rust
const ND_IOCTL_ARS_CAP: u32 = 3_223_342_593u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:460`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L460)*

### `NBD_SET_BLKSIZE`
```rust
const NBD_SET_BLKSIZE: u32 = 43_777u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:461`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L461)*

### `NBD_SET_TIMEOUT`
```rust
const NBD_SET_TIMEOUT: u32 = 43_785u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:462`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L462)*

### `VHOST_SCSI_GET_ABI_VERSION`
```rust
const VHOST_SCSI_GET_ABI_VERSION: u32 = 1_074_048_834u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:463`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L463)*

### `RIO_UNMAP_INBOUND`
```rust
const RIO_UNMAP_INBOUND: u32 = 1_074_294_034u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:464`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L464)*

### `ATM_QUERYLOOP`
```rust
const ATM_QUERYLOOP: u32 = 1_074_815_316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:465`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L465)*

### `DFL_FPGA_GET_API_VERSION`
```rust
const DFL_FPGA_GET_API_VERSION: u32 = 46_592u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:466`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L466)*

### `USBDEVFS_WAIT_FOR_RESUME`
```rust
const USBDEVFS_WAIT_FOR_RESUME: u32 = 21_795u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:467`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L467)*

### `FBIO_CURSOR`
```rust
const FBIO_CURSOR: u32 = 3_228_059_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:468`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L468)*

### `RNDCLEARPOOL`
```rust
const RNDCLEARPOOL: u32 = 20_998u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:469`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L469)*

### `VIDIOC_QUERYSTD`
```rust
const VIDIOC_QUERYSTD: u32 = 2_148_030_015u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:470`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L470)*

### `DMA_BUF_IOCTL_SYNC`
```rust
const DMA_BUF_IOCTL_SYNC: u32 = 1_074_291_200u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:471`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L471)*

### `SCIF_RECV`
```rust
const SCIF_RECV: u32 = 3_222_827_783u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:472`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L472)*

### `PTP_PIN_GETFUNC2`
```rust
const PTP_PIN_GETFUNC2: u32 = 3_227_532_559u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:473`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L473)*

### `FW_CDEV_IOC_ALLOCATE`
```rust
const FW_CDEV_IOC_ALLOCATE: u32 = 3_223_331_586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:474`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L474)*

### `CEC_ADAP_G_CAPS`
```rust
const CEC_ADAP_G_CAPS: u32 = 3_226_231_040u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:475`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L475)*

### `VIDIOC_G_FBUF`
```rust
const VIDIOC_G_FBUF: u32 = 2_150_651_402u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:476`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L476)*

### `PTP_ENABLE_PPS2`
```rust
const PTP_ENABLE_PPS2: u32 = 1_074_019_597u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:477`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L477)*

### `PCITEST_CLEAR_IRQ`
```rust
const PCITEST_CLEAR_IRQ: u32 = 20_496u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:478`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L478)*

### `IPMICTL_SET_GETS_EVENTS_CMD`
```rust
const IPMICTL_SET_GETS_EVENTS_CMD: u32 = 2_147_772_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:479`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L479)*

### `BTRFS_IOC_DEVICES_READY`
```rust
const BTRFS_IOC_DEVICES_READY: u32 = 2_415_957_031u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:480`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L480)*

### `JSIOCGAXMAP`
```rust
const JSIOCGAXMAP: u32 = 2_151_705_138u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:481`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L481)*

### `FW_CDEV_IOC_GET_CYCLE_TIMER`
```rust
const FW_CDEV_IOC_GET_CYCLE_TIMER: u32 = 2_148_541_196u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:482`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L482)*

### `FW_CDEV_IOC_SET_ISO_CHANNELS`
```rust
const FW_CDEV_IOC_SET_ISO_CHANNELS: u32 = 1_074_799_383u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:483`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L483)*

### `RTC_WIE_OFF`
```rust
const RTC_WIE_OFF: u32 = 28_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:484`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L484)*

### `PPGETMODE`
```rust
const PPGETMODE: u32 = 2_147_774_616u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:485`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L485)*

### `VIDIOC_DBG_G_REGISTER`
```rust
const VIDIOC_DBG_G_REGISTER: u32 = 3_224_917_584u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:486`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L486)*

### `PTP_SYS_OFFSET`
```rust
const PTP_SYS_OFFSET: u32 = 1_128_283_397u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:487`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L487)*

### `BTRFS_IOC_SPACE_INFO`
```rust
const BTRFS_IOC_SPACE_INFO: u32 = 3_222_311_956u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:488`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L488)*

### `VIDIOC_SUBDEV_ENUM_FRAME_SIZE`
```rust
const VIDIOC_SUBDEV_ENUM_FRAME_SIZE: u32 = 3_225_441_866u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:489`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L489)*

### `ND_IOCTL_VENDOR`
```rust
const ND_IOCTL_VENDOR: u32 = 3_221_769_737u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:490`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L490)*

### `SCIF_VREADFROM`
```rust
const SCIF_VREADFROM: u32 = 3_223_876_364u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:491`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L491)*

### `BTRFS_IOC_TRANS_START`
```rust
const BTRFS_IOC_TRANS_START: u32 = 37_894u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:492`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L492)*

### `INOTIFY_IOC_SETNEXTWD`
```rust
const INOTIFY_IOC_SETNEXTWD: u32 = 1_074_022_656u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:493`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L493)*

### `SNAPSHOT_GET_IMAGE_SIZE`
```rust
const SNAPSHOT_GET_IMAGE_SIZE: u32 = 2_148_021_006u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:494`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L494)*

### `TUNDETACHFILTER`
```rust
const TUNDETACHFILTER: u32 = 1_074_812_118u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:495`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L495)*

### `ND_IOCTL_CLEAR_ERROR`
```rust
const ND_IOCTL_CLEAR_ERROR: u32 = 3_223_342_596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:496`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L496)*

### `IOC_PR_CLEAR`
```rust
const IOC_PR_CLEAR: u32 = 1_074_819_277u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:497`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L497)*

### `SCIF_READFROM`
```rust
const SCIF_READFROM: u32 = 3_223_876_362u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:498`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L498)*

### `PPPIOCGDEBUG`
```rust
const PPPIOCGDEBUG: u32 = 2_147_775_553u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:499`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L499)*

### `BLKGETZONESZ`
```rust
const BLKGETZONESZ: u32 = 2_147_750_532u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:500`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L500)*

### `HIDIOCGUSAGES`
```rust
const HIDIOCGUSAGES: u32 = 3_491_514_387u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:501`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L501)*

### `SONYPI_IOCGTEMP`
```rust
const SONYPI_IOCGTEMP: u32 = 2_147_579_404u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:502`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L502)*

### `UI_SET_MSCBIT`
```rust
const UI_SET_MSCBIT: u32 = 1_074_025_832u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:503`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L503)*

### `APM_IOC_SUSPEND`
```rust
const APM_IOC_SUSPEND: u32 = 16_642u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:504`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L504)*

### `BTRFS_IOC_TREE_SEARCH`
```rust
const BTRFS_IOC_TREE_SEARCH: u32 = 3_489_698_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:505`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L505)*

### `RTC_PLL_GET`
```rust
const RTC_PLL_GET: u32 = 2_149_609_489u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:506`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L506)*

### `RIO_CM_EP_GET_LIST`
```rust
const RIO_CM_EP_GET_LIST: u32 = 3_221_512_962u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:507`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L507)*

### `USBDEVFS_DISCSIGNAL`
```rust
const USBDEVFS_DISCSIGNAL: u32 = 2_148_553_998u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:508`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L508)*

### `LIRC_GET_MIN_TIMEOUT`
```rust
const LIRC_GET_MIN_TIMEOUT: u32 = 2_147_772_680u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:509`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L509)*

### `SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY`
```rust
const SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY: u32 = 2_174_244_674u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:510`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L510)*

### `DM_TARGET_MSG`
```rust
const DM_TARGET_MSG: u32 = 3_241_737_486u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:511`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L511)*

### `SONYPI_IOCGBAT1REM`
```rust
const SONYPI_IOCGBAT1REM: u32 = 2_147_644_931u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:512`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L512)*

### `EVIOCSFF`
```rust
const EVIOCSFF: u32 = 1_076_905_344u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:513`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L513)*

### `TUNSETGROUP`
```rust
const TUNSETGROUP: u32 = 1_074_025_678u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:514`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L514)*

### `EVIOCGKEYCODE`
```rust
const EVIOCGKEYCODE: u32 = 2_148_025_604u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:515`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L515)*

### `KCOV_REMOTE_ENABLE`
```rust
const KCOV_REMOTE_ENABLE: u32 = 1_075_340_134u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:516`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L516)*

### `ND_IOCTL_GET_CONFIG_SIZE`
```rust
const ND_IOCTL_GET_CONFIG_SIZE: u32 = 3_222_031_876u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:517`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L517)*

### `FDEJECT`
```rust
const FDEJECT: u32 = 602u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:518`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L518)*

### `TUNSETOFFLOAD`
```rust
const TUNSETOFFLOAD: u32 = 1_074_025_680u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:519`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L519)*

### `PPPIOCCONNECT`
```rust
const PPPIOCCONNECT: u32 = 1_074_033_722u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:520`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L520)*

### `ATM_ADDADDR`
```rust
const ATM_ADDADDR: u32 = 1_074_815_368u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:521`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L521)*

### `VDUSE_DEV_INJECT_CONFIG_IRQ`
```rust
const VDUSE_DEV_INJECT_CONFIG_IRQ: u32 = 33_043u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:522`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L522)*

### `AUTOFS_DEV_IOCTL_ASKUMOUNT`
```rust
const AUTOFS_DEV_IOCTL_ASKUMOUNT: u32 = 3_222_836_093u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:523`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L523)*

### `VHOST_VDPA_GET_STATUS`
```rust
const VHOST_VDPA_GET_STATUS: u32 = 2_147_594_097u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:524`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L524)*

### `CCISS_PASSTHRU`
```rust
const CCISS_PASSTHRU: u32 = 3_227_009_547u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:525`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L525)*

### `MGSL_IOCCLRMODCOUNT`
```rust
const MGSL_IOCCLRMODCOUNT: u32 = 27_919u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:526`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L526)*

### `TEE_IOC_SUPPL_SEND`
```rust
const TEE_IOC_SUPPL_SEND: u32 = 2_148_574_215u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:527`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L527)*

### `ATMARPD_CTRL`
```rust
const ATMARPD_CTRL: u32 = 25_057u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:528`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L528)*

### `UI_ABS_SETUP`
```rust
const UI_ABS_SETUP: u32 = 1_075_598_596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:529`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L529)*

### `UI_DEV_DESTROY`
```rust
const UI_DEV_DESTROY: u32 = 21_762u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:530`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L530)*

### `BTRFS_IOC_QUOTA_CTL`
```rust
const BTRFS_IOC_QUOTA_CTL: u32 = 3_222_311_976u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:531`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L531)*

### `RTC_AIE_ON`
```rust
const RTC_AIE_ON: u32 = 28_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:532`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L532)*

### `AUTOFS_IOC_EXPIRE`
```rust
const AUTOFS_IOC_EXPIRE: u32 = 2_165_085_029u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:533`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L533)*

### `PPPIOCSDEBUG`
```rust
const PPPIOCSDEBUG: u32 = 1_074_033_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:534`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L534)*

### `GPIO_V2_LINE_SET_VALUES_IOCTL`
```rust
const GPIO_V2_LINE_SET_VALUES_IOCTL: u32 = 3_222_320_143u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:535`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L535)*

### `PPPIOCSMRU`
```rust
const PPPIOCSMRU: u32 = 1_074_033_746u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:536`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L536)*

### `CCISS_DEREGDISK`
```rust
const CCISS_DEREGDISK: u32 = 16_908u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:537`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L537)*

### `UI_DEV_CREATE`
```rust
const UI_DEV_CREATE: u32 = 21_761u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:538`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L538)*

### `FUSE_DEV_IOC_CLONE`
```rust
const FUSE_DEV_IOC_CLONE: u32 = 2_147_804_416u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:539`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L539)*

### `BTRFS_IOC_START_SYNC`
```rust
const BTRFS_IOC_START_SYNC: u32 = 2_148_045_848u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:540`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L540)*

### `NILFS_IOCTL_DELETE_CHECKPOINT`
```rust
const NILFS_IOCTL_DELETE_CHECKPOINT: u32 = 1_074_294_401u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:541`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L541)*

### `SNAPSHOT_AVAIL_SWAP_SIZE`
```rust
const SNAPSHOT_AVAIL_SWAP_SIZE: u32 = 2_148_021_011u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:542`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L542)*

### `DM_TABLE_CLEAR`
```rust
const DM_TABLE_CLEAR: u32 = 3_241_737_482u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:543`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L543)*

### `CCISS_GETINTINFO`
```rust
const CCISS_GETINTINFO: u32 = 2_148_024_834u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:544`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L544)*

### `PPPIOCSASYNCMAP`
```rust
const PPPIOCSASYNCMAP: u32 = 1_074_033_751u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:545`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L545)*

### `I2OEVTGET`
```rust
const I2OEVTGET: u32 = 2_154_326_283u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:546`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L546)*

### `NVME_IOCTL_RESET`
```rust
const NVME_IOCTL_RESET: u32 = 20_036u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:547`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L547)*

### `PPYIELD`
```rust
const PPYIELD: u32 = 28_813u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:548`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L548)*

### `NVME_IOCTL_IO64_CMD`
```rust
const NVME_IOCTL_IO64_CMD: u32 = 3_226_488_392u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:549`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L549)*

### `TUNSETCARRIER`
```rust
const TUNSETCARRIER: u32 = 1_074_025_698u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:550`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L550)*

### `DM_DEV_WAIT`
```rust
const DM_DEV_WAIT: u32 = 3_241_737_480u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:551`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L551)*

### `RTC_WIE_ON`
```rust
const RTC_WIE_ON: u32 = 28_687u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:552`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L552)*

### `MEDIA_IOC_DEVICE_INFO`
```rust
const MEDIA_IOC_DEVICE_INFO: u32 = 3_238_034_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:553`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L553)*

### `RIO_CM_CHAN_CREATE`
```rust
const RIO_CM_CHAN_CREATE: u32 = 3_221_381_891u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:554`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L554)*

### `MGSL_IOCSPARAMS`
```rust
const MGSL_IOCSPARAMS: u32 = 1_076_915_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:555`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L555)*

### `RTC_SET_TIME`
```rust
const RTC_SET_TIME: u32 = 1_076_129_802u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:556`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L556)*

### `VHOST_RESET_OWNER`
```rust
const VHOST_RESET_OWNER: u32 = 44_802u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:557`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L557)*

### `IOC_OPAL_PSID_REVERT_TPR`
```rust
const IOC_OPAL_PSID_REVERT_TPR: u32 = 1_091_072_232u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:558`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L558)*

### `AUTOFS_DEV_IOCTL_OPENMOUNT`
```rust
const AUTOFS_DEV_IOCTL_OPENMOUNT: u32 = 3_222_836_084u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:559`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L559)*

### `UDF_GETEABLOCK`
```rust
const UDF_GETEABLOCK: u32 = 2_148_035_649u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:560`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L560)*

### `VFIO_IOMMU_MAP_DMA`
```rust
const VFIO_IOMMU_MAP_DMA: u32 = 15_217u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:561`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L561)*

### `VIDIOC_SUBSCRIBE_EVENT`
```rust
const VIDIOC_SUBSCRIBE_EVENT: u32 = 1_075_861_082u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:562`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L562)*

### `HIDIOCGFLAG`
```rust
const HIDIOCGFLAG: u32 = 2_147_764_238u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:563`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L563)*

### `HIDIOCGUCODE`
```rust
const HIDIOCGUCODE: u32 = 3_222_816_781u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:564`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L564)*

### `VIDIOC_OMAP3ISP_AF_CFG`
```rust
const VIDIOC_OMAP3ISP_AF_CFG: u32 = 3_226_228_421u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:565`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L565)*

### `DM_REMOVE_ALL`
```rust
const DM_REMOVE_ALL: u32 = 3_241_737_473u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:566`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L566)*

### `ASPEED_LPC_CTRL_IOCTL_MAP`
```rust
const ASPEED_LPC_CTRL_IOCTL_MAP: u32 = 1_074_835_969u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:567`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L567)*

### `CCISS_GETFIRMVER`
```rust
const CCISS_GETFIRMVER: u32 = 2_147_762_696u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:568`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L568)*

### `ND_IOCTL_ARS_START`
```rust
const ND_IOCTL_ARS_START: u32 = 3_223_342_594u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:569`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L569)*

### `PPPIOCSMRRU`
```rust
const PPPIOCSMRRU: u32 = 1_074_033_723u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:570`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L570)*

### `CEC_ADAP_S_LOG_ADDRS`
```rust
const CEC_ADAP_S_LOG_ADDRS: u32 = 3_227_279_620u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:571`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L571)*

### `RPROC_GET_SHUTDOWN_ON_RELEASE`
```rust
const RPROC_GET_SHUTDOWN_ON_RELEASE: u32 = 2_147_792_642u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:572`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L572)*

### `DMA_HEAP_IOCTL_ALLOC`
```rust
const DMA_HEAP_IOCTL_ALLOC: u32 = 3_222_816_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:573`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L573)*

### `PPSETTIME`
```rust
const PPSETTIME: u32 = 1_074_819_222u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:574`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L574)*

### `RTC_ALM_READ`
```rust
const RTC_ALM_READ: u32 = 2_149_871_624u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:575`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L575)*

### `VDUSE_SET_API_VERSION`
```rust
const VDUSE_SET_API_VERSION: u32 = 1_074_299_137u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:576`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L576)*

### `RIO_MPORT_MAINT_WRITE_REMOTE`
```rust
const RIO_MPORT_MAINT_WRITE_REMOTE: u32 = 1_075_342_600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:577`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L577)*

### `VIDIOC_SUBDEV_S_CROP`
```rust
const VIDIOC_SUBDEV_S_CROP: u32 = 3_224_917_564u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:578`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L578)*

### `USBDEVFS_CONNECT`
```rust
const USBDEVFS_CONNECT: u32 = 21_783u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:579`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L579)*

### `SYNC_IOC_FILE_INFO`
```rust
const SYNC_IOC_FILE_INFO: u32 = 3_224_911_364u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:580`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L580)*

### `ATMARP_MKIP`
```rust
const ATMARP_MKIP: u32 = 25_058u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:581`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L581)*

### `VFIO_IOMMU_SPAPR_TCE_GET_INFO`
```rust
const VFIO_IOMMU_SPAPR_TCE_GET_INFO: u32 = 15_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:582`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L582)*

### `CCISS_GETHEARTBEAT`
```rust
const CCISS_GETHEARTBEAT: u32 = 2_147_762_694u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:583`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L583)*

### `ATM_RSTADDR`
```rust
const ATM_RSTADDR: u32 = 1_074_815_367u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:584`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L584)*

### `NBD_SET_SIZE`
```rust
const NBD_SET_SIZE: u32 = 43_778u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:585`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L585)*

### `UDF_GETVOLIDENT`
```rust
const UDF_GETVOLIDENT: u32 = 2_148_035_650u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:586`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L586)*

### `GPIO_V2_LINE_GET_VALUES_IOCTL`
```rust
const GPIO_V2_LINE_GET_VALUES_IOCTL: u32 = 3_222_320_142u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:587`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L587)*

### `MGSL_IOCSTXIDLE`
```rust
const MGSL_IOCSTXIDLE: u32 = 27_906u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:588`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L588)*

### `FSL_HV_IOCTL_SETPROP`
```rust
const FSL_HV_IOCTL_SETPROP: u32 = 3_223_891_720u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:589`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L589)*

### `BTRFS_IOC_GET_DEV_STATS`
```rust
const BTRFS_IOC_GET_DEV_STATS: u32 = 3_288_896_564u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:590`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L590)*

### `PPRSTATUS`
```rust
const PPRSTATUS: u32 = 2_147_577_985u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:591`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L591)*

### `MGSL_IOCTXENABLE`
```rust
const MGSL_IOCTXENABLE: u32 = 27_908u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:592`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L592)*

### `UDF_GETEASIZE`
```rust
const UDF_GETEASIZE: u32 = 2_147_773_504u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:593`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L593)*

### `NVME_IOCTL_ADMIN64_CMD`
```rust
const NVME_IOCTL_ADMIN64_CMD: u32 = 3_226_488_391u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:594`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L594)*

### `VHOST_SET_OWNER`
```rust
const VHOST_SET_OWNER: u32 = 44_801u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:595`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L595)*

### `RIO_ALLOC_DMA`
```rust
const RIO_ALLOC_DMA: u32 = 3_222_826_259u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:596`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L596)*

### `RIO_CM_CHAN_ACCEPT`
```rust
const RIO_CM_CHAN_ACCEPT: u32 = 3_221_775_111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:597`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L597)*

### `I2OHRTGET`
```rust
const I2OHRTGET: u32 = 3_222_825_217u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:598`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L598)*

### `ATM_SETCIRANGE`
```rust
const ATM_SETCIRANGE: u32 = 1_074_815_371u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:599`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L599)*

### `HPET_IE_ON`
```rust
const HPET_IE_ON: u32 = 26_625u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:600`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L600)*

### `PERF_EVENT_IOC_ID`
```rust
const PERF_EVENT_IOC_ID: u32 = 2_148_017_159u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:601`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L601)*

### `TUNSETSNDBUF`
```rust
const TUNSETSNDBUF: u32 = 1_074_025_684u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:602`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L602)*

### `PTP_PIN_SETFUNC`
```rust
const PTP_PIN_SETFUNC: u32 = 1_080_048_903u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:603`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L603)*

### `PPPIOCDISCONN`
```rust
const PPPIOCDISCONN: u32 = 29_753u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:604`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L604)*

### `VIDIOC_QUERYCTRL`
```rust
const VIDIOC_QUERYCTRL: u32 = 3_225_703_972u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:605`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L605)*

### `PPEXCL`
```rust
const PPEXCL: u32 = 28_815u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:606`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L606)*

### `PCITEST_MSI`
```rust
const PCITEST_MSI: u32 = 1_074_024_451u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:607`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L607)*

### `FDWERRORCLR`
```rust
const FDWERRORCLR: u32 = 598u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:608`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L608)*

### `AUTOFS_IOC_FAIL`
```rust
const AUTOFS_IOC_FAIL: u32 = 37_729u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:609`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L609)*

### `USBDEVFS_IOCTL`
```rust
const USBDEVFS_IOCTL: u32 = 3_222_295_826u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:610`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L610)*

### `VIDIOC_S_STD`
```rust
const VIDIOC_S_STD: u32 = 1_074_288_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:611`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L611)*

### `F2FS_IOC_RESIZE_FS`
```rust
const F2FS_IOC_RESIZE_FS: u32 = 1_074_328_848u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:612`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L612)*

### `SONET_SETDIAG`
```rust
const SONET_SETDIAG: u32 = 3_221_512_466u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:613`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L613)*

### `BTRFS_IOC_DEFRAG`
```rust
const BTRFS_IOC_DEFRAG: u32 = 1_342_215_170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:614`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L614)*

### `CCISS_GETDRIVVER`
```rust
const CCISS_GETDRIVVER: u32 = 2_147_762_697u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:615`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L615)*

### `IPMICTL_GET_TIMING_PARMS_CMD`
```rust
const IPMICTL_GET_TIMING_PARMS_CMD: u32 = 2_148_034_839u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:616`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L616)*

### `HPET_IRQFREQ`
```rust
const HPET_IRQFREQ: u32 = 1_074_292_742u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:617`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L617)*

### `ATM_GETESI`
```rust
const ATM_GETESI: u32 = 1_074_815_365u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:618`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L618)*

### `CCISS_GETLUNINFO`
```rust
const CCISS_GETLUNINFO: u32 = 2_148_286_993u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:619`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L619)*

### `AUTOFS_DEV_IOCTL_ISMOUNTPOINT`
```rust
const AUTOFS_DEV_IOCTL_ISMOUNTPOINT: u32 = 3_222_836_094u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:620`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L620)*

### `TEE_IOC_SHM_ALLOC`
```rust
const TEE_IOC_SHM_ALLOC: u32 = 3_222_316_033u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:621`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L621)*

### `PERF_EVENT_IOC_SET_BPF`
```rust
const PERF_EVENT_IOC_SET_BPF: u32 = 1_074_013_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:622`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L622)*

### `UDMABUF_CREATE_LIST`
```rust
const UDMABUF_CREATE_LIST: u32 = 1_074_296_131u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:623`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L623)*

### `VHOST_SET_LOG_BASE`
```rust
const VHOST_SET_LOG_BASE: u32 = 1_074_310_916u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:624`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L624)*

### `ZATM_GETPOOL`
```rust
const ZATM_GETPOOL: u32 = 1_074_815_329u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:625`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L625)*

### `BR2684_SETFILT`
```rust
const BR2684_SETFILT: u32 = 1_075_601_808u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:626`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L626)*

### `RNDGETPOOL`
```rust
const RNDGETPOOL: u32 = 2_148_028_930u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:627`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L627)*

### `PPS_GETPARAMS`
```rust
const PPS_GETPARAMS: u32 = 2_148_036_769u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:628`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L628)*

### `IOC_PR_RESERVE`
```rust
const IOC_PR_RESERVE: u32 = 1_074_819_273u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:629`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L629)*

### `VIDIOC_TRY_DECODER_CMD`
```rust
const VIDIOC_TRY_DECODER_CMD: u32 = 3_225_966_177u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:630`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L630)*

### `RIO_CM_CHAN_CLOSE`
```rust
const RIO_CM_CHAN_CLOSE: u32 = 1_073_898_244u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:631`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L631)*

### `VIDIOC_DV_TIMINGS_CAP`
```rust
const VIDIOC_DV_TIMINGS_CAP: u32 = 3_230_684_772u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:632`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L632)*

### `IOCTL_MEI_CONNECT_CLIENT_VTAG`
```rust
const IOCTL_MEI_CONNECT_CLIENT_VTAG: u32 = 3_222_554_628u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:633`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L633)*

### `PMU_IOC_GET_BACKLIGHT`
```rust
const PMU_IOC_GET_BACKLIGHT: u32 = 2_148_024_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:634`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L634)*

### `USBDEVFS_GET_CAPABILITIES`
```rust
const USBDEVFS_GET_CAPABILITIES: u32 = 2_147_767_578u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:635`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L635)*

### `SCIF_WRITETO`
```rust
const SCIF_WRITETO: u32 = 3_223_876_363u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:636`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L636)*

### `UDF_RELOCATE_BLOCKS`
```rust
const UDF_RELOCATE_BLOCKS: u32 = 3_221_777_475u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:637`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L637)*

### `FSL_HV_IOCTL_PARTITION_RESTART`
```rust
const FSL_HV_IOCTL_PARTITION_RESTART: u32 = 3_221_794_561u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:638`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L638)*

### `CCISS_REGNEWD`
```rust
const CCISS_REGNEWD: u32 = 16_910u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:639`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L639)*

### `FAT_IOCTL_SET_ATTRIBUTES`
```rust
const FAT_IOCTL_SET_ATTRIBUTES: u32 = 1_074_033_169u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:640`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L640)*

### `VIDIOC_CREATE_BUFS`
```rust
const VIDIOC_CREATE_BUFS: u32 = 3_238_024_796u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:641`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L641)*

### `CAPI_GET_VERSION`
```rust
const CAPI_GET_VERSION: u32 = 3_222_291_207u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:642`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L642)*

### `SWITCHTEC_IOCTL_EVENT_SUMMARY`
```rust
const SWITCHTEC_IOCTL_EVENT_SUMMARY: u32 = 2_228_770_626u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:643`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L643)*

### `VFIO_EEH_PE_OP`
```rust
const VFIO_EEH_PE_OP: u32 = 15_225u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:644`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L644)*

### `FW_CDEV_IOC_CREATE_ISO_CONTEXT`
```rust
const FW_CDEV_IOC_CREATE_ISO_CONTEXT: u32 = 3_223_331_592u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:645`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L645)*

### `F2FS_IOC_RELEASE_COMPRESS_BLOCKS`
```rust
const F2FS_IOC_RELEASE_COMPRESS_BLOCKS: u32 = 2_148_070_674u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:646`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L646)*

### `NBD_SET_SIZE_BLOCKS`
```rust
const NBD_SET_SIZE_BLOCKS: u32 = 43_783u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:647`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L647)*

### `IPMI_BMC_IOCTL_SET_SMS_ATN`
```rust
const IPMI_BMC_IOCTL_SET_SMS_ATN: u32 = 45_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:648`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L648)*

### `ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG`
```rust
const ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG: u32 = 3_222_319_873u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:649`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L649)*

### `VIDIOC_S_AUDOUT`
```rust
const VIDIOC_S_AUDOUT: u32 = 1_077_171_762u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:650`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L650)*

### `VIDIOC_S_FMT`
```rust
const VIDIOC_S_FMT: u32 = 3_234_878_981u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:651`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L651)*

### `PPPIOCATTACH`
```rust
const PPPIOCATTACH: u32 = 1_074_033_725u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:652`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L652)*

### `VHOST_GET_VRING_BUSYLOOP_TIMEOUT`
```rust
const VHOST_GET_VRING_BUSYLOOP_TIMEOUT: u32 = 1_074_310_948u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:653`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L653)*

### `FS_IOC_MEASURE_VERITY`
```rust
const FS_IOC_MEASURE_VERITY: u32 = 3_221_513_862u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:654`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L654)*

### `CCISS_BIG_PASSTHRU`
```rust
const CCISS_BIG_PASSTHRU: u32 = 3_227_533_842u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:655`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L655)*

### `IPMICTL_SET_MY_LUN_CMD`
```rust
const IPMICTL_SET_MY_LUN_CMD: u32 = 2_147_772_691u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:656`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L656)*

### `PCITEST_LEGACY_IRQ`
```rust
const PCITEST_LEGACY_IRQ: u32 = 20_482u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:657`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L657)*

### `USBDEVFS_SUBMITURB`
```rust
const USBDEVFS_SUBMITURB: u32 = 2_151_175_434u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:658`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L658)*

### `AUTOFS_IOC_READY`
```rust
const AUTOFS_IOC_READY: u32 = 37_728u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:659`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L659)*

### `BTRFS_IOC_SEND`
```rust
const BTRFS_IOC_SEND: u32 = 1_078_498_342u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:660`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L660)*

### `VIDIOC_G_EXT_CTRLS`
```rust
const VIDIOC_G_EXT_CTRLS: u32 = 3_223_344_711u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:661`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L661)*

### `JSIOCSBTNMAP`
```rust
const JSIOCSBTNMAP: u32 = 1_140_877_875u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:662`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L662)*

### `PPPIOCSFLAGS`
```rust
const PPPIOCSFLAGS: u32 = 1_074_033_753u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:663`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L663)*

### `NVRAM_INIT`
```rust
const NVRAM_INIT: u32 = 28_736u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:664`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L664)*

### `RFKILL_IOCTL_NOINPUT`
```rust
const RFKILL_IOCTL_NOINPUT: u32 = 20_993u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:665`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L665)*

### `BTRFS_IOC_BALANCE`
```rust
const BTRFS_IOC_BALANCE: u32 = 1_342_215_180u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:666`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L666)*

### `FS_IOC_GETFSMAP`
```rust
const FS_IOC_GETFSMAP: u32 = 3_233_830_971u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:667`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L667)*

### `IPMICTL_GET_MY_CHANNEL_LUN_CMD`
```rust
const IPMICTL_GET_MY_CHANNEL_LUN_CMD: u32 = 2_147_772_699u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:668`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L668)*

### `STP_POLICY_ID_GET`
```rust
const STP_POLICY_ID_GET: u32 = 2_148_541_697u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:669`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L669)*

### `PPSETFLAGS`
```rust
const PPSETFLAGS: u32 = 1_074_032_795u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:670`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L670)*

### `CEC_ADAP_S_PHYS_ADDR`
```rust
const CEC_ADAP_S_PHYS_ADDR: u32 = 1_073_897_730u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:671`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L671)*

### `ATMTCP_CREATE`
```rust
const ATMTCP_CREATE: u32 = 24_974u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:672`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L672)*

### `IPMI_BMC_IOCTL_FORCE_ABORT`
```rust
const IPMI_BMC_IOCTL_FORCE_ABORT: u32 = 45_314u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:673`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L673)*

### `PPPIOCGXASYNCMAP`
```rust
const PPPIOCGXASYNCMAP: u32 = 2_149_610_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:674`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L674)*

### `VHOST_SET_VRING_CALL`
```rust
const VHOST_SET_VRING_CALL: u32 = 1_074_310_945u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:675`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L675)*

### `LIRC_GET_FEATURES`
```rust
const LIRC_GET_FEATURES: u32 = 2_147_772_672u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:676`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L676)*

### `GSMIOC_DISABLE_NET`
```rust
const GSMIOC_DISABLE_NET: u32 = 18_179u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:677`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L677)*

### `AUTOFS_IOC_CATATONIC`
```rust
const AUTOFS_IOC_CATATONIC: u32 = 37_730u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:678`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L678)*

### `NBD_DO_IT`
```rust
const NBD_DO_IT: u32 = 43_779u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:679`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L679)*

### `LIRC_SET_REC_CARRIER_RANGE`
```rust
const LIRC_SET_REC_CARRIER_RANGE: u32 = 1_074_030_879u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:680`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L680)*

### `IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD`
```rust
const IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD: u32 = 2_147_772_697u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:681`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L681)*

### `EVIOCSCLOCKID`
```rust
const EVIOCSCLOCKID: u32 = 1_074_021_792u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:682`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L682)*

### `USBDEVFS_FREE_STREAMS`
```rust
const USBDEVFS_FREE_STREAMS: u32 = 2_148_029_725u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:683`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L683)*

### `FSI_SCOM_RESET`
```rust
const FSI_SCOM_RESET: u32 = 1_074_033_411u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:684`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L684)*

### `PMU_IOC_GRAB_BACKLIGHT`
```rust
const PMU_IOC_GRAB_BACKLIGHT: u32 = 2_148_024_838u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:685`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L685)*

### `VIDIOC_SUBDEV_S_FMT`
```rust
const VIDIOC_SUBDEV_S_FMT: u32 = 3_227_014_661u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:686`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L686)*

### `FDDEFPRM`
```rust
const FDDEFPRM: u32 = 1_075_839_555u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:687`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L687)*

### `TEE_IOC_INVOKE`
```rust
const TEE_IOC_INVOKE: u32 = 2_148_574_211u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:688`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L688)*

### `USBDEVFS_BULK`
```rust
const USBDEVFS_BULK: u32 = 3_222_820_098u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:689`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L689)*

### `SCIF_VWRITETO`
```rust
const SCIF_VWRITETO: u32 = 3_223_876_365u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:690`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L690)*

### `SONYPI_IOCSBRT`
```rust
const SONYPI_IOCSBRT: u32 = 1_073_837_568u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:691`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L691)*

### `BTRFS_IOC_FILE_EXTENT_SAME`
```rust
const BTRFS_IOC_FILE_EXTENT_SAME: u32 = 3_222_836_278u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:692`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L692)*

### `RTC_PIE_ON`
```rust
const RTC_PIE_ON: u32 = 28_677u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:693`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L693)*

### `BTRFS_IOC_SCAN_DEV`
```rust
const BTRFS_IOC_SCAN_DEV: u32 = 1_342_215_172u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:694`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L694)*

### `PPPIOCXFERUNIT`
```rust
const PPPIOCXFERUNIT: u32 = 29_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:695`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L695)*

### `WDIOC_GETTIMEOUT`
```rust
const WDIOC_GETTIMEOUT: u32 = 2_147_768_071u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:696`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L696)*

### `BTRFS_IOC_SET_RECEIVED_SUBVOL`
```rust
const BTRFS_IOC_SET_RECEIVED_SUBVOL: u32 = 3_234_370_597u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:697`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L697)*

### `DFL_FPGA_PORT_ERR_SET_IRQ`
```rust
const DFL_FPGA_PORT_ERR_SET_IRQ: u32 = 1_074_312_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:698`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L698)*

### `FBIO_WAITFORVSYNC`
```rust
const FBIO_WAITFORVSYNC: u32 = 1_074_021_920u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:699`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L699)*

### `RTC_PIE_OFF`
```rust
const RTC_PIE_OFF: u32 = 28_678u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:700`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L700)*

### `EVIOCGRAB`
```rust
const EVIOCGRAB: u32 = 1_074_021_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:701`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L701)*

### `PMU_IOC_SET_BACKLIGHT`
```rust
const PMU_IOC_SET_BACKLIGHT: u32 = 1_074_283_010u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:702`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L702)*

### `EVIOCGREP`
```rust
const EVIOCGREP: u32 = 2_148_025_603u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:703`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L703)*

### `PERF_EVENT_IOC_MODIFY_ATTRIBUTES`
```rust
const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: u32 = 1_074_275_339u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:704`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L704)*

### `UFFDIO_CONTINUE`
```rust
const UFFDIO_CONTINUE: u32 = 3_223_366_151u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:705`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L705)*

### `VDUSE_GET_API_VERSION`
```rust
const VDUSE_GET_API_VERSION: u32 = 2_148_040_960u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:706`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L706)*

### `RTC_RD_TIME`
```rust
const RTC_RD_TIME: u32 = 2_149_871_625u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:707`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L707)*

### `FDMSGOFF`
```rust
const FDMSGOFF: u32 = 582u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:708`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L708)*

### `IPMICTL_REGISTER_FOR_CMD_CHANS`
```rust
const IPMICTL_REGISTER_FOR_CMD_CHANS: u32 = 2_148_296_988u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:709`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L709)*

### `CAPI_GET_ERRCODE`
```rust
const CAPI_GET_ERRCODE: u32 = 2_147_631_905u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:710`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L710)*

### `PCITEST_SET_IRQTYPE`
```rust
const PCITEST_SET_IRQTYPE: u32 = 1_074_024_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:711`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L711)*

### `VIDIOC_SUBDEV_S_EDID`
```rust
const VIDIOC_SUBDEV_S_EDID: u32 = 3_223_868_969u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:712`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L712)*

### `MATROXFB_SET_OUTPUT_MODE`
```rust
const MATROXFB_SET_OUTPUT_MODE: u32 = 1_074_294_522u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:713`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L713)*

### `RIO_DEV_ADD`
```rust
const RIO_DEV_ADD: u32 = 1_075_866_903u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:714`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L714)*

### `VIDIOC_ENUM_FREQ_BANDS`
```rust
const VIDIOC_ENUM_FREQ_BANDS: u32 = 3_225_441_893u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:715`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L715)*

### `FBIO_RADEON_SET_MIRROR`
```rust
const FBIO_RADEON_SET_MIRROR: u32 = 1_074_282_500u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:716`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L716)*

### `PCITEST_GET_IRQTYPE`
```rust
const PCITEST_GET_IRQTYPE: u32 = 20_489u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:717`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L717)*

### `JSIOCGVERSION`
```rust
const JSIOCGVERSION: u32 = 2_147_772_929u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:718`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L718)*

### `SONYPI_IOCSBLUE`
```rust
const SONYPI_IOCSBLUE: u32 = 1_073_837_577u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:719`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L719)*

### `SNAPSHOT_PREF_IMAGE_SIZE`
```rust
const SNAPSHOT_PREF_IMAGE_SIZE: u32 = 13_074u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:720`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L720)*

### `F2FS_IOC_GET_FEATURES`
```rust
const F2FS_IOC_GET_FEATURES: u32 = 2_147_808_524u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:721`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L721)*

### `SCIF_REG`
```rust
const SCIF_REG: u32 = 3_223_876_360u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:722`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L722)*

### `NILFS_IOCTL_CLEAN_SEGMENTS`
```rust
const NILFS_IOCTL_CLEAN_SEGMENTS: u32 = 1_081_634_440u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:723`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L723)*

### `FW_CDEV_IOC_INITIATE_BUS_RESET`
```rust
const FW_CDEV_IOC_INITIATE_BUS_RESET: u32 = 1_074_012_933u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:724`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L724)*

### `RIO_WAIT_FOR_ASYNC`
```rust
const RIO_WAIT_FOR_ASYNC: u32 = 1_074_294_038u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:725`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L725)*

### `VHOST_SET_VRING_NUM`
```rust
const VHOST_SET_VRING_NUM: u32 = 1_074_310_928u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:726`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L726)*

### `AUTOFS_DEV_IOCTL_PROTOVER`
```rust
const AUTOFS_DEV_IOCTL_PROTOVER: u32 = 3_222_836_082u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:727`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L727)*

### `RIO_FREE_DMA`
```rust
const RIO_FREE_DMA: u32 = 1_074_294_036u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:728`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L728)*

### `MGSL_IOCRXENABLE`
```rust
const MGSL_IOCRXENABLE: u32 = 27_909u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:729`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L729)*

### `IOCTL_VM_SOCKETS_GET_LOCAL_CID`
```rust
const IOCTL_VM_SOCKETS_GET_LOCAL_CID: u32 = 1_977u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:730`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L730)*

### `IPMICTL_SET_TIMING_PARMS_CMD`
```rust
const IPMICTL_SET_TIMING_PARMS_CMD: u32 = 2_148_034_838u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:731`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L731)*

### `PPPIOCGL2TPSTATS`
```rust
const PPPIOCGL2TPSTATS: u32 = 2_152_231_990u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:732`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L732)*

### `PERF_EVENT_IOC_PERIOD`
```rust
const PERF_EVENT_IOC_PERIOD: u32 = 1_074_275_332u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:733`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L733)*

### `PTP_PIN_SETFUNC2`
```rust
const PTP_PIN_SETFUNC2: u32 = 1_080_048_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:734`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L734)*

### `CHIOEXCHANGE`
```rust
const CHIOEXCHANGE: u32 = 1_075_602_178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:735`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L735)*

### `NILFS_IOCTL_GET_SUINFO`
```rust
const NILFS_IOCTL_GET_SUINFO: u32 = 2_149_084_804u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:736`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L736)*

### `CEC_DQEVENT`
```rust
const CEC_DQEVENT: u32 = 3_226_493_191u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:737`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L737)*

### `UI_SET_SWBIT`
```rust
const UI_SET_SWBIT: u32 = 1_074_025_837u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:738`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L738)*

### `VHOST_VDPA_SET_CONFIG`
```rust
const VHOST_VDPA_SET_CONFIG: u32 = 1_074_311_028u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:739`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L739)*

### `TUNSETIFF`
```rust
const TUNSETIFF: u32 = 1_074_025_674u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:740`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L740)*

### `CHIOPOSITION`
```rust
const CHIOPOSITION: u32 = 1_074_553_603u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:741`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L741)*

### `IPMICTL_SET_MAINTENANCE_MODE_CMD`
```rust
const IPMICTL_SET_MAINTENANCE_MODE_CMD: u32 = 1_074_030_879u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:742`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L742)*

### `BTRFS_IOC_DEFAULT_SUBVOL`
```rust
const BTRFS_IOC_DEFAULT_SUBVOL: u32 = 1_074_304_019u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:743`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L743)*

### `RIO_UNMAP_OUTBOUND`
```rust
const RIO_UNMAP_OUTBOUND: u32 = 1_076_391_184u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:744`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L744)*

### `CAPI_CLR_FLAGS`
```rust
const CAPI_CLR_FLAGS: u32 = 2_147_762_981u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:745`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L745)*

### `FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE`
```rust
const FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE: u32 = 1_075_323_663u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:746`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L746)*

### `MATROXFB_GET_OUTPUT_CONNECTION`
```rust
const MATROXFB_GET_OUTPUT_CONNECTION: u32 = 2_148_036_344u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:747`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L747)*

### `EVIOCSMASK`
```rust
const EVIOCSMASK: u32 = 1_074_808_211u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:748`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L748)*

### `BTRFS_IOC_FORGET_DEV`
```rust
const BTRFS_IOC_FORGET_DEV: u32 = 1_342_215_173u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:749`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L749)*

### `CXL_MEM_QUERY_COMMANDS`
```rust
const CXL_MEM_QUERY_COMMANDS: u32 = 2_148_060_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:750`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L750)*

### `CEC_S_MODE`
```rust
const CEC_S_MODE: u32 = 1_074_028_809u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:751`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L751)*

### `MGSL_IOCSIF`
```rust
const MGSL_IOCSIF: u32 = 27_914u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:752`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L752)*

### `SWITCHTEC_IOCTL_PFF_TO_PORT`
```rust
const SWITCHTEC_IOCTL_PFF_TO_PORT: u32 = 3_222_034_244u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:753`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L753)*

### `PPSETMODE`
```rust
const PPSETMODE: u32 = 1_074_032_768u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:754`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L754)*

### `VFIO_DEVICE_SET_IRQS`
```rust
const VFIO_DEVICE_SET_IRQS: u32 = 15_214u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:755`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L755)*

### `VIDIOC_PREPARE_BUF`
```rust
const VIDIOC_PREPARE_BUF: u32 = 3_227_014_749u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:756`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L756)*

### `CEC_ADAP_G_CONNECTOR_INFO`
```rust
const CEC_ADAP_G_CONNECTOR_INFO: u32 = 2_151_964_938u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:757`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L757)*

### `IOC_OPAL_WRITE_SHADOW_MBR`
```rust
const IOC_OPAL_WRITE_SHADOW_MBR: u32 = 1_092_645_098u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:758`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L758)*

### `VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL`
```rust
const VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL: u32 = 3_225_441_867u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:759`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L759)*

### `UDMABUF_CREATE`
```rust
const UDMABUF_CREATE: u32 = 1_075_344_706u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:760`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L760)*

### `SONET_CLRDIAG`
```rust
const SONET_CLRDIAG: u32 = 3_221_512_467u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:761`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L761)*

### `PHN_SET_REG`
```rust
const PHN_SET_REG: u32 = 1_074_294_785u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:762`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L762)*

### `RNDADDTOENTCNT`
```rust
const RNDADDTOENTCNT: u32 = 1_074_024_961u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:763`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L763)*

### `VBG_IOCTL_CHECK_BALLOON`
```rust
const VBG_IOCTL_CHECK_BALLOON: u32 = 3_223_344_657u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:764`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L764)*

### `VIDIOC_OMAP3ISP_STAT_REQ`
```rust
const VIDIOC_OMAP3ISP_STAT_REQ: u32 = 3_223_869_126u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:765`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L765)*

### `PPS_FETCH`
```rust
const PPS_FETCH: u32 = 3_221_778_596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:766`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L766)*

### `RTC_AIE_OFF`
```rust
const RTC_AIE_OFF: u32 = 28_674u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:767`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L767)*

### `VFIO_GROUP_SET_CONTAINER`
```rust
const VFIO_GROUP_SET_CONTAINER: u32 = 15_208u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:768`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L768)*

### `FW_CDEV_IOC_RECEIVE_PHY_PACKETS`
```rust
const FW_CDEV_IOC_RECEIVE_PHY_PACKETS: u32 = 1_074_275_094u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:769`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L769)*

### `VFIO_IOMMU_SPAPR_TCE_REMOVE`
```rust
const VFIO_IOMMU_SPAPR_TCE_REMOVE: u32 = 15_224u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:770`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L770)*

### `VFIO_IOMMU_GET_INFO`
```rust
const VFIO_IOMMU_GET_INFO: u32 = 15_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:771`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L771)*

### `DM_DEV_SUSPEND`
```rust
const DM_DEV_SUSPEND: u32 = 3_241_737_478u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:772`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L772)*

### `F2FS_IOC_GET_COMPRESS_OPTION`
```rust
const F2FS_IOC_GET_COMPRESS_OPTION: u32 = 2_147_677_461u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:773`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L773)*

### `FW_CDEV_IOC_STOP_ISO`
```rust
const FW_CDEV_IOC_STOP_ISO: u32 = 1_074_012_939u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:774`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L774)*

### `GPIO_V2_GET_LINEINFO_IOCTL`
```rust
const GPIO_V2_GET_LINEINFO_IOCTL: u32 = 3_238_048_773u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:775`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L775)*

### `ATMMPC_CTRL`
```rust
const ATMMPC_CTRL: u32 = 25_048u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:776`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L776)*

### `PPPIOCSXASYNCMAP`
```rust
const PPPIOCSXASYNCMAP: u32 = 1_075_868_751u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:777`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L777)*

### `CHIOGSTATUS`
```rust
const CHIOGSTATUS: u32 = 1_074_815_752u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:778`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L778)*

### `FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE`
```rust
const FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE: u32 = 3_222_807_309u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:779`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L779)*

### `RIO_MPORT_MAINT_PORT_IDX_GET`
```rust
const RIO_MPORT_MAINT_PORT_IDX_GET: u32 = 2_147_773_699u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:780`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L780)*

### `CAPI_SET_FLAGS`
```rust
const CAPI_SET_FLAGS: u32 = 2_147_762_980u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:781`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L781)*

### `VFIO_GROUP_GET_DEVICE_FD`
```rust
const VFIO_GROUP_GET_DEVICE_FD: u32 = 15_210u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:782`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L782)*

### `VHOST_SET_MEM_TABLE`
```rust
const VHOST_SET_MEM_TABLE: u32 = 1_074_310_915u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:783`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L783)*

### `MATROXFB_SET_OUTPUT_CONNECTION`
```rust
const MATROXFB_SET_OUTPUT_CONNECTION: u32 = 1_074_294_520u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:784`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L784)*

### `DFL_FPGA_PORT_GET_REGION_INFO`
```rust
const DFL_FPGA_PORT_GET_REGION_INFO: u32 = 46_658u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:785`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L785)*

### `VHOST_GET_FEATURES`
```rust
const VHOST_GET_FEATURES: u32 = 2_148_052_736u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:786`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L786)*

### `LIRC_GET_REC_RESOLUTION`
```rust
const LIRC_GET_REC_RESOLUTION: u32 = 2_147_772_679u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:787`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L787)*

### `PACKET_CTRL_CMD`
```rust
const PACKET_CTRL_CMD: u32 = 3_222_820_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:788`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L788)*

### `LIRC_SET_TRANSMITTER_MASK`
```rust
const LIRC_SET_TRANSMITTER_MASK: u32 = 1_074_030_871u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:789`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L789)*

### `BTRFS_IOC_ADD_DEV`
```rust
const BTRFS_IOC_ADD_DEV: u32 = 1_342_215_178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:790`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L790)*

### `JSIOCGCORR`
```rust
const JSIOCGCORR: u32 = 2_149_870_114u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:791`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L791)*

### `VIDIOC_G_FMT`
```rust
const VIDIOC_G_FMT: u32 = 3_234_878_980u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:792`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L792)*

### `RTC_EPOCH_SET`
```rust
const RTC_EPOCH_SET: u32 = 1_074_294_798u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:793`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L793)*

### `CAPI_GET_PROFILE`
```rust
const CAPI_GET_PROFILE: u32 = 3_225_436_937u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:794`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L794)*

### `ATM_GETLOOP`
```rust
const ATM_GETLOOP: u32 = 1_074_815_314u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:795`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L795)*

### `SCIF_LISTEN`
```rust
const SCIF_LISTEN: u32 = 1_074_033_410u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:796`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L796)*

### `NBD_CLEAR_QUE`
```rust
const NBD_CLEAR_QUE: u32 = 43_781u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:797`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L797)*

### `F2FS_IOC_MOVE_RANGE`
```rust
const F2FS_IOC_MOVE_RANGE: u32 = 3_223_385_353u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:798`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L798)*

### `LIRC_GET_LENGTH`
```rust
const LIRC_GET_LENGTH: u32 = 2_147_772_687u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:799`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L799)*

### `I8K_SET_FAN`
```rust
const I8K_SET_FAN: u32 = 3_221_776_775u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:800`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L800)*

### `FDSETMAXERRS`
```rust
const FDSETMAXERRS: u32 = 1_075_053_132u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:801`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L801)*

### `VIDIOC_SUBDEV_QUERYCAP`
```rust
const VIDIOC_SUBDEV_QUERYCAP: u32 = 2_151_699_968u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:802`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L802)*

### `SNAPSHOT_SET_SWAP_AREA`
```rust
const SNAPSHOT_SET_SWAP_AREA: u32 = 1_074_541_325u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:803`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L803)*

### `LIRC_GET_REC_TIMEOUT`
```rust
const LIRC_GET_REC_TIMEOUT: u32 = 2_147_772_708u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:804`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L804)*

### `EVIOCRMFF`
```rust
const EVIOCRMFF: u32 = 1_074_021_761u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:805`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L805)*

### `GPIO_GET_LINEEVENT_IOCTL`
```rust
const GPIO_GET_LINEEVENT_IOCTL: u32 = 3_224_417_284u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:806`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L806)*

### `PPRDATA`
```rust
const PPRDATA: u32 = 2_147_577_989u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:807`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L807)*

### `RIO_MPORT_GET_PROPERTIES`
```rust
const RIO_MPORT_GET_PROPERTIES: u32 = 2_150_657_284u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:808`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L808)*

### `TUNSETVNETHDRSZ`
```rust
const TUNSETVNETHDRSZ: u32 = 1_074_025_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:809`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L809)*

### `GPIO_GET_LINEINFO_IOCTL`
```rust
const GPIO_GET_LINEINFO_IOCTL: u32 = 3_225_990_146u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:810`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L810)*

### `GSMIOC_GETCONF`
```rust
const GSMIOC_GETCONF: u32 = 2_152_482_560u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:811`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L811)*

### `LIRC_GET_SEND_MODE`
```rust
const LIRC_GET_SEND_MODE: u32 = 2_147_772_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:812`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L812)*

### `PPPIOCSACTIVE`
```rust
const PPPIOCSACTIVE: u32 = 1_074_820_166u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:813`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L813)*

### `SIOCGSTAMPNS_NEW`
```rust
const SIOCGSTAMPNS_NEW: u32 = 2_148_567_303u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:814`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L814)*

### `IPMICTL_RECEIVE_MSG`
```rust
const IPMICTL_RECEIVE_MSG: u32 = 3_224_398_092u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:815`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L815)*

### `LIRC_SET_SEND_DUTY_CYCLE`
```rust
const LIRC_SET_SEND_DUTY_CYCLE: u32 = 1_074_030_869u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:816`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L816)*

### `UI_END_FF_ERASE`
```rust
const UI_END_FF_ERASE: u32 = 1_074_550_219u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:817`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L817)*

### `SWITCHTEC_IOCTL_FLASH_PART_INFO`
```rust
const SWITCHTEC_IOCTL_FLASH_PART_INFO: u32 = 3_222_296_385u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:818`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L818)*

### `FW_CDEV_IOC_SEND_PHY_PACKET`
```rust
const FW_CDEV_IOC_SEND_PHY_PACKET: u32 = 3_222_807_317u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:819`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L819)*

### `NBD_SET_FLAGS`
```rust
const NBD_SET_FLAGS: u32 = 43_786u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:820`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L820)*

### `VFIO_DEVICE_GET_REGION_INFO`
```rust
const VFIO_DEVICE_GET_REGION_INFO: u32 = 15_212u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:821`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L821)*

### `REISERFS_IOC_UNPACK`
```rust
const REISERFS_IOC_UNPACK: u32 = 1_074_318_593u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:822`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L822)*

### `FW_CDEV_IOC_REMOVE_DESCRIPTOR`
```rust
const FW_CDEV_IOC_REMOVE_DESCRIPTOR: u32 = 1_074_012_935u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:823`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L823)*

### `RIO_SET_EVENT_MASK`
```rust
const RIO_SET_EVENT_MASK: u32 = 1_074_031_885u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:824`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L824)*

### `SNAPSHOT_ALLOC_SWAP_PAGE`
```rust
const SNAPSHOT_ALLOC_SWAP_PAGE: u32 = 2_148_021_012u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:825`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L825)*

### `VDUSE_VQ_INJECT_IRQ`
```rust
const VDUSE_VQ_INJECT_IRQ: u32 = 1_074_037_015u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:826`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L826)*

### `I2OPASSTHRU`
```rust
const I2OPASSTHRU: u32 = 2_148_559_116u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:827`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L827)*

### `IOC_OPAL_SET_PW`
```rust
const IOC_OPAL_SET_PW: u32 = 1_109_422_304u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:828`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L828)*

### `FSI_SCOM_READ`
```rust
const FSI_SCOM_READ: u32 = 3_223_352_065u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:829`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L829)*

### `VHOST_VDPA_GET_DEVICE_ID`
```rust
const VHOST_VDPA_GET_DEVICE_ID: u32 = 2_147_790_704u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:830`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L830)*

### `VIDIOC_QBUF`
```rust
const VIDIOC_QBUF: u32 = 3_227_014_671u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:831`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L831)*

### `VIDIOC_S_TUNER`
```rust
const VIDIOC_S_TUNER: u32 = 1_079_268_894u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:832`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L832)*

### `TUNGETVNETHDRSZ`
```rust
const TUNGETVNETHDRSZ: u32 = 2_147_767_511u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:833`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L833)*

### `CAPI_NCCI_GETUNIT`
```rust
const CAPI_NCCI_GETUNIT: u32 = 2_147_762_983u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:834`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L834)*

### `DFL_FPGA_PORT_UINT_GET_IRQ_NUM`
```rust
const DFL_FPGA_PORT_UINT_GET_IRQ_NUM: u32 = 2_147_792_455u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:835`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L835)*

### `VIDIOC_OMAP3ISP_STAT_EN`
```rust
const VIDIOC_OMAP3ISP_STAT_EN: u32 = 3_221_771_975u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:836`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L836)*

### `GPIO_V2_LINE_SET_CONFIG_IOCTL`
```rust
const GPIO_V2_LINE_SET_CONFIG_IOCTL: u32 = 3_239_097_357u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:837`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L837)*

### `TEE_IOC_VERSION`
```rust
const TEE_IOC_VERSION: u32 = 2_148_312_064u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:838`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L838)*

### `VIDIOC_LOG_STATUS`
```rust
const VIDIOC_LOG_STATUS: u32 = 22_086u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:839`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L839)*

### `IPMICTL_SEND_COMMAND_SETTIME`
```rust
const IPMICTL_SEND_COMMAND_SETTIME: u32 = 2_150_656_277u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:840`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L840)*

### `VHOST_SET_LOG_FD`
```rust
const VHOST_SET_LOG_FD: u32 = 1_074_048_775u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:841`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L841)*

### `SCIF_SEND`
```rust
const SCIF_SEND: u32 = 3_222_827_782u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:842`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L842)*

### `VIDIOC_SUBDEV_G_FMT`
```rust
const VIDIOC_SUBDEV_G_FMT: u32 = 3_227_014_660u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:843`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L843)*

### `NS_ADJBUFLEV`
```rust
const NS_ADJBUFLEV: u32 = 24_931u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:844`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L844)*

### `VIDIOC_DBG_S_REGISTER`
```rust
const VIDIOC_DBG_S_REGISTER: u32 = 1_077_433_935u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:845`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L845)*

### `NILFS_IOCTL_RESIZE`
```rust
const NILFS_IOCTL_RESIZE: u32 = 1_074_294_411u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:846`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L846)*

### `PHN_GETREG`
```rust
const PHN_GETREG: u32 = 3_221_778_437u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:847`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L847)*

### `I2OSWDL`
```rust
const I2OSWDL: u32 = 3_224_398_085u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:848`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L848)*

### `VBG_IOCTL_VMMDEV_REQUEST_BIG`
```rust
const VBG_IOCTL_VMMDEV_REQUEST_BIG: u32 = 22_019u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:849`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L849)*

### `JSIOCGBUTTONS`
```rust
const JSIOCGBUTTONS: u32 = 2_147_576_338u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:850`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L850)*

### `VFIO_IOMMU_ENABLE`
```rust
const VFIO_IOMMU_ENABLE: u32 = 15_219u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:851`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L851)*

### `DM_DEV_RENAME`
```rust
const DM_DEV_RENAME: u32 = 3_241_737_477u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:852`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L852)*

### `MEDIA_IOC_SETUP_LINK`
```rust
const MEDIA_IOC_SETUP_LINK: u32 = 3_224_665_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:853`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L853)*

### `VIDIOC_ENUMOUTPUT`
```rust
const VIDIOC_ENUMOUTPUT: u32 = 3_225_966_128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:854`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L854)*

### `STP_POLICY_ID_SET`
```rust
const STP_POLICY_ID_SET: u32 = 3_222_283_520u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:855`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L855)*

### `VHOST_VDPA_SET_CONFIG_CALL`
```rust
const VHOST_VDPA_SET_CONFIG_CALL: u32 = 1_074_048_887u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:856`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L856)*

### `VIDIOC_SUBDEV_G_CROP`
```rust
const VIDIOC_SUBDEV_G_CROP: u32 = 3_224_917_563u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:857`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L857)*

### `VIDIOC_S_CROP`
```rust
const VIDIOC_S_CROP: u32 = 1_075_074_620u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:858`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L858)*

### `WDIOC_GETTEMP`
```rust
const WDIOC_GETTEMP: u32 = 2_147_768_067u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:859`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L859)*

### `IOC_OPAL_ADD_USR_TO_LR`
```rust
const IOC_OPAL_ADD_USR_TO_LR: u32 = 1_092_120_804u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:860`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L860)*

### `UI_SET_LEDBIT`
```rust
const UI_SET_LEDBIT: u32 = 1_074_025_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:861`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L861)*

### `NBD_SET_SOCK`
```rust
const NBD_SET_SOCK: u32 = 43_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:862`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L862)*

### `BTRFS_IOC_SNAP_DESTROY_V2`
```rust
const BTRFS_IOC_SNAP_DESTROY_V2: u32 = 1_342_215_231u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:863`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L863)*

### `HIDIOCGCOLLECTIONINFO`
```rust
const HIDIOCGCOLLECTIONINFO: u32 = 3_222_292_497u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:864`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L864)*

### `I2OSWUL`
```rust
const I2OSWUL: u32 = 3_224_398_086u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:865`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L865)*

### `IOCTL_MEI_NOTIFY_GET`
```rust
const IOCTL_MEI_NOTIFY_GET: u32 = 2_147_764_227u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:866`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L866)*

### `FDFMTTRK`
```rust
const FDFMTTRK: u32 = 1_074_528_840u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:867`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L867)*

### `MMTIMER_GETBITS`
```rust
const MMTIMER_GETBITS: u32 = 27_908u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:868`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L868)*

### `VIDIOC_ENUMSTD`
```rust
const VIDIOC_ENUMSTD: u32 = 3_225_966_105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:869`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L869)*

### `VHOST_GET_VRING_BASE`
```rust
const VHOST_GET_VRING_BASE: u32 = 3_221_794_578u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:870`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L870)*

### `VFIO_DEVICE_IOEVENTFD`
```rust
const VFIO_DEVICE_IOEVENTFD: u32 = 15_220u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:871`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L871)*

### `ATMARP_SETENTRY`
```rust
const ATMARP_SETENTRY: u32 = 25_059u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:872`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L872)*

### `CCISS_REVALIDVOLS`
```rust
const CCISS_REVALIDVOLS: u32 = 16_906u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:873`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L873)*

### `MGSL_IOCLOOPTXDONE`
```rust
const MGSL_IOCLOOPTXDONE: u32 = 27_913u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:874`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L874)*

### `RTC_VL_READ`
```rust
const RTC_VL_READ: u32 = 2_147_774_483u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:875`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L875)*

### `ND_IOCTL_ARS_STATUS`
```rust
const ND_IOCTL_ARS_STATUS: u32 = 3_224_391_171u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:876`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L876)*

### `RIO_DEV_DEL`
```rust
const RIO_DEV_DEL: u32 = 1_075_866_904u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:877`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L877)*

### `VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES`
```rust
const VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES: u32 = 3_223_606_797u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:878`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L878)*

### `VIDIOC_SUBDEV_DV_TIMINGS_CAP`
```rust
const VIDIOC_SUBDEV_DV_TIMINGS_CAP: u32 = 3_230_684_772u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:879`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L879)*

### `SONYPI_IOCSFAN`
```rust
const SONYPI_IOCSFAN: u32 = 1_073_837_579u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:880`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L880)*

### `SPIOCSTYPE`
```rust
const SPIOCSTYPE: u32 = 1_074_295_041u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:881`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L881)*

### `IPMICTL_REGISTER_FOR_CMD`
```rust
const IPMICTL_REGISTER_FOR_CMD: u32 = 2_147_641_614u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:882`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L882)*

### `I8K_GET_FAN`
```rust
const I8K_GET_FAN: u32 = 3_221_776_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:883`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L883)*

### `TUNGETVNETBE`
```rust
const TUNGETVNETBE: u32 = 2_147_767_519u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:884`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L884)*

### `AUTOFS_DEV_IOCTL_FAIL`
```rust
const AUTOFS_DEV_IOCTL_FAIL: u32 = 3_222_836_087u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:885`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L885)*

### `UI_END_FF_UPLOAD`
```rust
const UI_END_FF_UPLOAD: u32 = 1_080_579_529u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:886`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L886)*

### `TOSH_SMM`
```rust
const TOSH_SMM: u32 = 3_222_828_176u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:887`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L887)*

### `SONYPI_IOCGBAT2REM`
```rust
const SONYPI_IOCGBAT2REM: u32 = 2_147_644_933u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:888`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L888)*

### `F2FS_IOC_GET_COMPRESS_BLOCKS`
```rust
const F2FS_IOC_GET_COMPRESS_BLOCKS: u32 = 2_148_070_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:889`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L889)*

### `PPPIOCSNPMODE`
```rust
const PPPIOCSNPMODE: u32 = 1_074_295_883u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:890`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L890)*

### `USBDEVFS_CONTROL`
```rust
const USBDEVFS_CONTROL: u32 = 3_222_820_096u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:891`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L891)*

### `HIDIOCGUSAGE`
```rust
const HIDIOCGUSAGE: u32 = 3_222_816_779u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:892`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L892)*

### `TUNSETTXFILTER`
```rust
const TUNSETTXFILTER: u32 = 1_074_025_681u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:893`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L893)*

### `TUNGETVNETLE`
```rust
const TUNGETVNETLE: u32 = 2_147_767_517u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:894`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L894)*

### `VIDIOC_ENUM_DV_TIMINGS`
```rust
const VIDIOC_ENUM_DV_TIMINGS: u32 = 3_230_946_914u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:895`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L895)*

### `BTRFS_IOC_INO_PATHS`
```rust
const BTRFS_IOC_INO_PATHS: u32 = 3_224_933_411u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:896`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L896)*

### `MGSL_IOCGXSYNC`
```rust
const MGSL_IOCGXSYNC: u32 = 27_924u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:897`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L897)*

### `HIDIOCGFIELDINFO`
```rust
const HIDIOCGFIELDINFO: u32 = 3_224_913_930u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:898`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L898)*

### `VIDIOC_SUBDEV_G_STD`
```rust
const VIDIOC_SUBDEV_G_STD: u32 = 2_148_029_975u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:899`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L899)*

### `I2OVALIDATE`
```rust
const I2OVALIDATE: u32 = 2_147_772_680u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:900`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L900)*

### `VIDIOC_TRY_ENCODER_CMD`
```rust
const VIDIOC_TRY_ENCODER_CMD: u32 = 3_223_869_006u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:901`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L901)*

### `NILFS_IOCTL_GET_CPINFO`
```rust
const NILFS_IOCTL_GET_CPINFO: u32 = 2_149_084_802u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:902`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L902)*

### `VIDIOC_G_FREQUENCY`
```rust
const VIDIOC_G_FREQUENCY: u32 = 3_224_131_128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:903`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L903)*

### `VFAT_IOCTL_READDIR_SHORT`
```rust
const VFAT_IOCTL_READDIR_SHORT: u32 = 2_184_212_994u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:904`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L904)*

### `ND_IOCTL_GET_CONFIG_DATA`
```rust
const ND_IOCTL_GET_CONFIG_DATA: u32 = 3_222_031_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:905`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L905)*

### `F2FS_IOC_RESERVE_COMPRESS_BLOCKS`
```rust
const F2FS_IOC_RESERVE_COMPRESS_BLOCKS: u32 = 2_148_070_675u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:906`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L906)*

### `FDGETDRVSTAT`
```rust
const FDGETDRVSTAT: u32 = 2_152_727_058u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:907`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L907)*

### `SYNC_IOC_MERGE`
```rust
const SYNC_IOC_MERGE: u32 = 3_224_387_075u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:908`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L908)*

### `VIDIOC_S_DV_TIMINGS`
```rust
const VIDIOC_S_DV_TIMINGS: u32 = 3_229_898_327u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:909`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L909)*

### `PPPIOCBRIDGECHAN`
```rust
const PPPIOCBRIDGECHAN: u32 = 1_074_033_717u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:910`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L910)*

### `LIRC_SET_SEND_MODE`
```rust
const LIRC_SET_SEND_MODE: u32 = 1_074_030_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:911`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L911)*

### `RIO_ENABLE_PORTWRITE_RANGE`
```rust
const RIO_ENABLE_PORTWRITE_RANGE: u32 = 1_074_818_315u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:912`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L912)*

### `ATM_GETTYPE`
```rust
const ATM_GETTYPE: u32 = 1_074_815_364u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:913`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L913)*

### `PHN_GETREGS`
```rust
const PHN_GETREGS: u32 = 3_223_875_591u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:914`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L914)*

### `FDSETEMSGTRESH`
```rust
const FDSETEMSGTRESH: u32 = 586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:915`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L915)*

### `NILFS_IOCTL_GET_VINFO`
```rust
const NILFS_IOCTL_GET_VINFO: u32 = 3_222_826_630u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:916`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L916)*

### `MGSL_IOCWAITEVENT`
```rust
const MGSL_IOCWAITEVENT: u32 = 3_221_515_528u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:917`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L917)*

### `CAPI_INSTALLED`
```rust
const CAPI_INSTALLED: u32 = 2_147_631_906u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:918`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L918)*

### `EVIOCGMASK`
```rust
const EVIOCGMASK: u32 = 2_148_550_034u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:919`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L919)*

### `BTRFS_IOC_SUBVOL_GETFLAGS`
```rust
const BTRFS_IOC_SUBVOL_GETFLAGS: u32 = 2_148_045_849u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:920`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L920)*

### `FSL_HV_IOCTL_PARTITION_GET_STATUS`
```rust
const FSL_HV_IOCTL_PARTITION_GET_STATUS: u32 = 3_222_056_706u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:921`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L921)*

### `MEDIA_IOC_ENUM_ENTITIES`
```rust
const MEDIA_IOC_ENUM_ENTITIES: u32 = 3_238_034_433u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:922`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L922)*

### `GSMIOC_GETFIRST`
```rust
const GSMIOC_GETFIRST: u32 = 2_147_763_972u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:923`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L923)*

### `FW_CDEV_IOC_FLUSH_ISO`
```rust
const FW_CDEV_IOC_FLUSH_ISO: u32 = 1_074_012_952u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:924`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L924)*

### `VIDIOC_DBG_G_CHIP_INFO`
```rust
const VIDIOC_DBG_G_CHIP_INFO: u32 = 3_234_354_790u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:925`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L925)*

### `F2FS_IOC_RELEASE_VOLATILE_WRITE`
```rust
const F2FS_IOC_RELEASE_VOLATILE_WRITE: u32 = 62_724u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:926`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L926)*

### `CAPI_GET_SERIAL`
```rust
const CAPI_GET_SERIAL: u32 = 3_221_504_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:927`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L927)*

### `FDSETDRVPRM`
```rust
const FDSETDRVPRM: u32 = 1_082_131_088u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:928`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L928)*

### `IOC_OPAL_SAVE`
```rust
const IOC_OPAL_SAVE: u32 = 1_092_120_796u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:929`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L929)*

### `VIDIOC_G_DV_TIMINGS`
```rust
const VIDIOC_G_DV_TIMINGS: u32 = 3_229_898_328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:930`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L930)*

### `TUNSETIFINDEX`
```rust
const TUNSETIFINDEX: u32 = 1_074_025_690u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:931`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L931)*

### `CCISS_SETINTINFO`
```rust
const CCISS_SETINTINFO: u32 = 1_074_283_011u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:932`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L932)*

### `RTC_VL_CLR`
```rust
const RTC_VL_CLR: u32 = 28_692u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:933`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L933)*

### `VIDIOC_REQBUFS`
```rust
const VIDIOC_REQBUFS: u32 = 3_222_558_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:934`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L934)*

### `USBDEVFS_REAPURBNDELAY32`
```rust
const USBDEVFS_REAPURBNDELAY32: u32 = 1_074_025_741u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:935`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L935)*

### `TEE_IOC_SHM_REGISTER`
```rust
const TEE_IOC_SHM_REGISTER: u32 = 3_222_840_329u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:936`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L936)*

### `USBDEVFS_SETCONFIGURATION`
```rust
const USBDEVFS_SETCONFIGURATION: u32 = 2_147_767_557u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:937`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L937)*

### `CCISS_GETNODENAME`
```rust
const CCISS_GETNODENAME: u32 = 2_148_549_124u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:938`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L938)*

### `VIDIOC_SUBDEV_S_FRAME_INTERVAL`
```rust
const VIDIOC_SUBDEV_S_FRAME_INTERVAL: u32 = 3_224_393_238u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:939`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L939)*

### `VIDIOC_ENUM_FRAMESIZES`
```rust
const VIDIOC_ENUM_FRAMESIZES: u32 = 3_224_131_146u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:940`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L940)*

### `VFIO_DEVICE_PCI_HOT_RESET`
```rust
const VFIO_DEVICE_PCI_HOT_RESET: u32 = 15_217u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:941`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L941)*

### `FW_CDEV_IOC_SEND_BROADCAST_REQUEST`
```rust
const FW_CDEV_IOC_SEND_BROADCAST_REQUEST: u32 = 1_076_372_242u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:942`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L942)*

### `LPSETTIMEOUT_NEW`
```rust
const LPSETTIMEOUT_NEW: u32 = 1_074_791_951u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:943`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L943)*

### `RIO_CM_MPORT_GET_LIST`
```rust
const RIO_CM_MPORT_GET_LIST: u32 = 3_221_512_971u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:944`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L944)*

### `FW_CDEV_IOC_QUEUE_ISO`
```rust
const FW_CDEV_IOC_QUEUE_ISO: u32 = 3_222_807_305u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:945`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L945)*

### `FDRAWCMD`
```rust
const FDRAWCMD: u32 = 600u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:946`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L946)*

### `SCIF_UNREG`
```rust
const SCIF_UNREG: u32 = 3_222_303_497u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:947`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L947)*

### `PPPIOCGIDLE64`
```rust
const PPPIOCGIDLE64: u32 = 2_148_561_983u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:948`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L948)*

### `USBDEVFS_RELEASEINTERFACE`
```rust
const USBDEVFS_RELEASEINTERFACE: u32 = 2_147_767_568u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:949`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L949)*

### `VIDIOC_CROPCAP`
```rust
const VIDIOC_CROPCAP: u32 = 3_224_131_130u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:950`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L950)*

### `DFL_FPGA_PORT_GET_INFO`
```rust
const DFL_FPGA_PORT_GET_INFO: u32 = 46_657u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:951`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L951)*

### `PHN_SET_REGS`
```rust
const PHN_SET_REGS: u32 = 1_074_294_787u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:952`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L952)*

### `ATMLEC_DATA`
```rust
const ATMLEC_DATA: u32 = 25_041u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:953`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L953)*

### `PPPOEIOCDFWD`
```rust
const PPPOEIOCDFWD: u32 = 45_313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:954`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L954)*

### `VIDIOC_S_SELECTION`
```rust
const VIDIOC_S_SELECTION: u32 = 3_225_441_887u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:955`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L955)*

### `SNAPSHOT_FREE_SWAP_PAGES`
```rust
const SNAPSHOT_FREE_SWAP_PAGES: u32 = 13_065u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:956`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L956)*

### `BTRFS_IOC_LOGICAL_INO`
```rust
const BTRFS_IOC_LOGICAL_INO: u32 = 3_224_933_412u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:957`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L957)*

### `VIDIOC_S_CTRL`
```rust
const VIDIOC_S_CTRL: u32 = 3_221_771_804u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:958`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L958)*

### `ZATM_SETPOOL`
```rust
const ZATM_SETPOOL: u32 = 1_074_815_331u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:959`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L959)*

### `MTIOCPOS`
```rust
const MTIOCPOS: u32 = 2_148_035_843u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:960`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L960)*

### `PMU_IOC_SLEEP`
```rust
const PMU_IOC_SLEEP: u32 = 16_896u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:961`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L961)*

### `AUTOFS_DEV_IOCTL_PROTOSUBVER`
```rust
const AUTOFS_DEV_IOCTL_PROTOSUBVER: u32 = 3_222_836_083u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:962`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L962)*

### `VBG_IOCTL_CHANGE_FILTER_MASK`
```rust
const VBG_IOCTL_CHANGE_FILTER_MASK: u32 = 3_223_344_652u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:963`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L963)*

### `NILFS_IOCTL_GET_SUSTAT`
```rust
const NILFS_IOCTL_GET_SUSTAT: u32 = 2_150_657_669u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:964`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L964)*

### `VIDIOC_QUERYCAP`
```rust
const VIDIOC_QUERYCAP: u32 = 2_154_321_408u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:965`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L965)*

### `HPET_INFO`
```rust
const HPET_INFO: u32 = 2_149_083_139u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:966`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L966)*

### `VIDIOC_AM437X_CCDC_CFG`
```rust
const VIDIOC_AM437X_CCDC_CFG: u32 = 1_074_288_321u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:967`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L967)*

### `DM_LIST_DEVICES`
```rust
const DM_LIST_DEVICES: u32 = 3_241_737_474u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:968`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L968)*

### `TUNSETOWNER`
```rust
const TUNSETOWNER: u32 = 1_074_025_676u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:969`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L969)*

### `VBG_IOCTL_CHANGE_GUEST_CAPABILITIES`
```rust
const VBG_IOCTL_CHANGE_GUEST_CAPABILITIES: u32 = 3_223_344_654u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:970`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L970)*

### `RNDADDENTROPY`
```rust
const RNDADDENTROPY: u32 = 1_074_287_107u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:971`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L971)*

### `USBDEVFS_RESET`
```rust
const USBDEVFS_RESET: u32 = 21_780u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:972`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L972)*

### `BTRFS_IOC_SUBVOL_CREATE`
```rust
const BTRFS_IOC_SUBVOL_CREATE: u32 = 1_342_215_182u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:973`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L973)*

### `USBDEVFS_FORBID_SUSPEND`
```rust
const USBDEVFS_FORBID_SUSPEND: u32 = 21_793u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:974`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L974)*

### `FDGETDRVTYP`
```rust
const FDGETDRVTYP: u32 = 2_148_532_751u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:975`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L975)*

### `PPWCONTROL`
```rust
const PPWCONTROL: u32 = 1_073_836_164u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:976`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L976)*

### `VIDIOC_ENUM_FRAMEINTERVALS`
```rust
const VIDIOC_ENUM_FRAMEINTERVALS: u32 = 3_224_655_435u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:977`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L977)*

### `KCOV_DISABLE`
```rust
const KCOV_DISABLE: u32 = 25_445u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:978`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L978)*

### `IOC_OPAL_ACTIVATE_LSP`
```rust
const IOC_OPAL_ACTIVATE_LSP: u32 = 1_092_120_799u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:979`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L979)*

### `VHOST_VDPA_GET_IOVA_RANGE`
```rust
const VHOST_VDPA_GET_IOVA_RANGE: u32 = 2_148_577_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:980`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L980)*

### `PPPIOCSPASS`
```rust
const PPPIOCSPASS: u32 = 1_074_820_167u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:981`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L981)*

### `RIO_CM_CHAN_CONNECT`
```rust
const RIO_CM_CHAN_CONNECT: u32 = 1_074_291_464u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:982`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L982)*

### `I2OSWDEL`
```rust
const I2OSWDEL: u32 = 3_224_398_087u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:983`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L983)*

### `FS_IOC_SET_ENCRYPTION_POLICY`
```rust
const FS_IOC_SET_ENCRYPTION_POLICY: u32 = 2_148_296_211u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:984`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L984)*

### `IOC_OPAL_MBR_DONE`
```rust
const IOC_OPAL_MBR_DONE: u32 = 1_091_596_521u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:985`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L985)*

### `PPPIOCSMAXCID`
```rust
const PPPIOCSMAXCID: u32 = 1_074_033_745u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:986`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L986)*

### `PPSETPHASE`
```rust
const PPSETPHASE: u32 = 1_074_032_788u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:987`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L987)*

### `VHOST_VDPA_SET_VRING_ENABLE`
```rust
const VHOST_VDPA_SET_VRING_ENABLE: u32 = 1_074_311_029u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:988`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L988)*

### `USBDEVFS_GET_SPEED`
```rust
const USBDEVFS_GET_SPEED: u32 = 21_791u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:989`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L989)*

### `SONET_GETFRAMING`
```rust
const SONET_GETFRAMING: u32 = 2_147_770_646u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:990`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L990)*

### `VIDIOC_QUERYBUF`
```rust
const VIDIOC_QUERYBUF: u32 = 3_227_014_665u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:991`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L991)*

### `VIDIOC_S_EDID`
```rust
const VIDIOC_S_EDID: u32 = 3_223_868_969u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:992`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L992)*

### `BTRFS_IOC_QGROUP_ASSIGN`
```rust
const BTRFS_IOC_QGROUP_ASSIGN: u32 = 1_075_352_617u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:993`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L993)*

### `PPS_GETCAP`
```rust
const PPS_GETCAP: u32 = 2_148_036_771u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:994`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L994)*

### `SNAPSHOT_PLATFORM_SUPPORT`
```rust
const SNAPSHOT_PLATFORM_SUPPORT: u32 = 13_071u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:995`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L995)*

### `LIRC_SET_REC_TIMEOUT_REPORTS`
```rust
const LIRC_SET_REC_TIMEOUT_REPORTS: u32 = 1_074_030_873u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:996`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L996)*

### `SCIF_GET_NODEIDS`
```rust
const SCIF_GET_NODEIDS: u32 = 3_222_827_790u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:997`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L997)*

### `NBD_DISCONNECT`
```rust
const NBD_DISCONNECT: u32 = 43_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:998`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L998)*

### `VIDIOC_SUBDEV_G_FRAME_INTERVAL`
```rust
const VIDIOC_SUBDEV_G_FRAME_INTERVAL: u32 = 3_224_393_237u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:999`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L999)*

### `VFIO_IOMMU_DISABLE`
```rust
const VFIO_IOMMU_DISABLE: u32 = 15_220u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1000`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1000)*

### `SNAPSHOT_CREATE_IMAGE`
```rust
const SNAPSHOT_CREATE_IMAGE: u32 = 1_074_017_041u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1001`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1001)*

### `SNAPSHOT_POWER_OFF`
```rust
const SNAPSHOT_POWER_OFF: u32 = 13_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1002`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1002)*

### `APM_IOC_STANDBY`
```rust
const APM_IOC_STANDBY: u32 = 16_641u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1003`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1003)*

### `PPPIOCGUNIT`
```rust
const PPPIOCGUNIT: u32 = 2_147_775_574u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1004`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1004)*

### `AUTOFS_IOC_EXPIRE_MULTI`
```rust
const AUTOFS_IOC_EXPIRE_MULTI: u32 = 1_074_041_702u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1005`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1005)*

### `SCIF_BIND`
```rust
const SCIF_BIND: u32 = 3_221_779_201u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1006`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1006)*

### `IOC_WATCH_QUEUE_SET_SIZE`
```rust
const IOC_WATCH_QUEUE_SET_SIZE: u32 = 22_368u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1007`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1007)*

### `NILFS_IOCTL_CHANGE_CPMODE`
```rust
const NILFS_IOCTL_CHANGE_CPMODE: u32 = 1_074_818_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1008`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1008)*

### `IOC_OPAL_LOCK_UNLOCK`
```rust
const IOC_OPAL_LOCK_UNLOCK: u32 = 1_092_120_797u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1009`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1009)*

### `F2FS_IOC_SET_PIN_FILE`
```rust
const F2FS_IOC_SET_PIN_FILE: u32 = 1_074_066_701u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1010`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1010)*

### `PPPIOCGRASYNCMAP`
```rust
const PPPIOCGRASYNCMAP: u32 = 2_147_775_573u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1011`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1011)*

### `MMTIMER_MMAPAVAIL`
```rust
const MMTIMER_MMAPAVAIL: u32 = 27_910u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1012`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1012)*

### `I2OPASSTHRU32`
```rust
const I2OPASSTHRU32: u32 = 2_148_034_828u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1013`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1013)*

### `DFL_FPGA_FME_PORT_RELEASE`
```rust
const DFL_FPGA_FME_PORT_RELEASE: u32 = 1_074_050_689u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1014`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1014)*

### `VIDIOC_SUBDEV_QUERY_DV_TIMINGS`
```rust
const VIDIOC_SUBDEV_QUERY_DV_TIMINGS: u32 = 2_156_156_515u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1015`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1015)*

### `UI_SET_SNDBIT`
```rust
const UI_SET_SNDBIT: u32 = 1_074_025_834u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1016`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1016)*

### `VIDIOC_G_AUDOUT`
```rust
const VIDIOC_G_AUDOUT: u32 = 2_150_913_585u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1017`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1017)*

### `RTC_PLL_SET`
```rust
const RTC_PLL_SET: u32 = 1_075_867_666u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1018`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1018)*

### `VIDIOC_ENUMAUDIO`
```rust
const VIDIOC_ENUMAUDIO: u32 = 3_224_655_425u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1019`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1019)*

### `AUTOFS_DEV_IOCTL_TIMEOUT`
```rust
const AUTOFS_DEV_IOCTL_TIMEOUT: u32 = 3_222_836_090u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1020`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1020)*

### `VBG_IOCTL_DRIVER_VERSION_INFO`
```rust
const VBG_IOCTL_DRIVER_VERSION_INFO: u32 = 3_224_131_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1021`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1021)*

### `VHOST_SCSI_GET_EVENTS_MISSED`
```rust
const VHOST_SCSI_GET_EVENTS_MISSED: u32 = 1_074_048_836u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1022`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1022)*

### `VHOST_SET_VRING_ADDR`
```rust
const VHOST_SET_VRING_ADDR: u32 = 1_076_408_081u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1023`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1023)*

### `VDUSE_CREATE_DEV`
```rust
const VDUSE_CREATE_DEV: u32 = 1_095_794_946u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1024`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1024)*

### `FDFLUSH`
```rust
const FDFLUSH: u32 = 587u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1025`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1025)*

### `VBG_IOCTL_WAIT_FOR_EVENTS`
```rust
const VBG_IOCTL_WAIT_FOR_EVENTS: u32 = 3_223_344_650u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1026`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1026)*

### `DFL_FPGA_FME_ERR_SET_IRQ`
```rust
const DFL_FPGA_FME_ERR_SET_IRQ: u32 = 1_074_312_836u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1027`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1027)*

### `F2FS_IOC_GET_PIN_FILE`
```rust
const F2FS_IOC_GET_PIN_FILE: u32 = 2_147_808_526u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1028`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1028)*

### `SCIF_CONNECT`
```rust
const SCIF_CONNECT: u32 = 3_221_779_203u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1029`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1029)*

### `BLKREPORTZONE`
```rust
const BLKREPORTZONE: u32 = 3_222_278_786u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1030`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1030)*

### `AUTOFS_IOC_ASKUMOUNT`
```rust
const AUTOFS_IOC_ASKUMOUNT: u32 = 2_147_783_536u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1031`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1031)*

### `ATM_ADDPARTY`
```rust
const ATM_ADDPARTY: u32 = 1_074_815_476u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1032`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1032)*

### `FDSETPRM`
```rust
const FDSETPRM: u32 = 1_075_839_554u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1033`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1033)*

### `ATM_GETSTATZ`
```rust
const ATM_GETSTATZ: u32 = 1_074_815_313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1034`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1034)*

### `ISST_IF_MSR_COMMAND`
```rust
const ISST_IF_MSR_COMMAND: u32 = 3_221_814_788u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1035`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1035)*

### `BTRFS_IOC_GET_SUBVOL_INFO`
```rust
const BTRFS_IOC_GET_SUBVOL_INFO: u32 = 2_180_551_740u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1036`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1036)*

### `VIDIOC_UNSUBSCRIBE_EVENT`
```rust
const VIDIOC_UNSUBSCRIBE_EVENT: u32 = 1_075_861_083u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1037`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1037)*

### `SEV_ISSUE_CMD`
```rust
const SEV_ISSUE_CMD: u32 = 3_222_295_296u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1038`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1038)*

### `GPIOHANDLE_SET_LINE_VALUES_IOCTL`
```rust
const GPIOHANDLE_SET_LINE_VALUES_IOCTL: u32 = 3_225_465_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1039`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1039)*

### `PCITEST_COPY`
```rust
const PCITEST_COPY: u32 = 1_074_286_598u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1040`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1040)*

### `IPMICTL_GET_MY_ADDRESS_CMD`
```rust
const IPMICTL_GET_MY_ADDRESS_CMD: u32 = 2_147_772_690u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1041`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1041)*

### `CHIOGPICKER`
```rust
const CHIOGPICKER: u32 = 2_147_771_140u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1042`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1042)*

### `CAPI_NCCI_OPENCOUNT`
```rust
const CAPI_NCCI_OPENCOUNT: u32 = 2_147_762_982u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1043`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1043)*

### `CXL_MEM_SEND_COMMAND`
```rust
const CXL_MEM_SEND_COMMAND: u32 = 3_224_423_938u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1044`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1044)*

### `PERF_EVENT_IOC_SET_FILTER`
```rust
const PERF_EVENT_IOC_SET_FILTER: u32 = 1_074_275_334u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1045`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1045)*

### `IOC_OPAL_REVERT_TPR`
```rust
const IOC_OPAL_REVERT_TPR: u32 = 1_091_072_226u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1046`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1046)*

### `CHIOGVPARAMS`
```rust
const CHIOGVPARAMS: u32 = 2_154_849_043u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1047`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1047)*

### `PTP_PEROUT_REQUEST`
```rust
const PTP_PEROUT_REQUEST: u32 = 1_077_427_459u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1048`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1048)*

### `FSI_SCOM_CHECK`
```rust
const FSI_SCOM_CHECK: u32 = 2_147_775_232u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1049`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1049)*

### `RTC_IRQP_READ`
```rust
const RTC_IRQP_READ: u32 = 2_148_036_619u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1050`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1050)*

### `RIO_MPORT_MAINT_READ_LOCAL`
```rust
const RIO_MPORT_MAINT_READ_LOCAL: u32 = 2_149_084_421u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1051`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1051)*

### `HIDIOCGRDESCSIZE`
```rust
const HIDIOCGRDESCSIZE: u32 = 2_147_764_225u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1052`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1052)*

### `UI_GET_VERSION`
```rust
const UI_GET_VERSION: u32 = 2_147_767_597u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1053`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1053)*

### `NILFS_IOCTL_GET_CPSTAT`
```rust
const NILFS_IOCTL_GET_CPSTAT: u32 = 2_149_084_803u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1054`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1054)*

### `CCISS_GETBUSTYPES`
```rust
const CCISS_GETBUSTYPES: u32 = 2_147_762_695u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1055`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1055)*

### `VFIO_IOMMU_SPAPR_TCE_CREATE`
```rust
const VFIO_IOMMU_SPAPR_TCE_CREATE: u32 = 15_223u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1056`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1056)*

### `VIDIOC_EXPBUF`
```rust
const VIDIOC_EXPBUF: u32 = 3_225_441_808u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1057`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1057)*

### `UI_SET_RELBIT`
```rust
const UI_SET_RELBIT: u32 = 1_074_025_830u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1058`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1058)*

### `VFIO_SET_IOMMU`
```rust
const VFIO_SET_IOMMU: u32 = 15_206u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1059`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1059)*

### `VIDIOC_S_MODULATOR`
```rust
const VIDIOC_S_MODULATOR: u32 = 1_078_220_343u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1060`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1060)*

### `TUNGETFILTER`
```rust
const TUNGETFILTER: u32 = 2_148_553_947u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1061`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1061)*

### `CCISS_SETNODENAME`
```rust
const CCISS_SETNODENAME: u32 = 1_074_807_301u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1062`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1062)*

### `FBIO_GETCONTROL2`
```rust
const FBIO_GETCONTROL2: u32 = 2_148_025_993u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1063`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1063)*

### `TUNSETDEBUG`
```rust
const TUNSETDEBUG: u32 = 1_074_025_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1064`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1064)*

### `DM_DEV_REMOVE`
```rust
const DM_DEV_REMOVE: u32 = 3_241_737_476u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1065`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1065)*

### `HIDIOCSUSAGES`
```rust
const HIDIOCSUSAGES: u32 = 1_344_030_740u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1066`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1066)*

### `FS_IOC_ADD_ENCRYPTION_KEY`
```rust
const FS_IOC_ADD_ENCRYPTION_KEY: u32 = 3_226_494_487u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1067`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1067)*

### `FBIOGET_VBLANK`
```rust
const FBIOGET_VBLANK: u32 = 2_149_598_738u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1068`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1068)*

### `ATM_GETSTAT`
```rust
const ATM_GETSTAT: u32 = 1_074_815_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1069`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1069)*

### `VIDIOC_G_JPEGCOMP`
```rust
const VIDIOC_G_JPEGCOMP: u32 = 2_156_680_765u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1070`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1070)*

### `TUNATTACHFILTER`
```rust
const TUNATTACHFILTER: u32 = 1_074_812_117u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1071`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1071)*

### `UI_SET_ABSBIT`
```rust
const UI_SET_ABSBIT: u32 = 1_074_025_831u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1072`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1072)*

### `DFL_FPGA_PORT_ERR_GET_IRQ_NUM`
```rust
const DFL_FPGA_PORT_ERR_GET_IRQ_NUM: u32 = 2_147_792_453u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1073`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1073)*

### `USBDEVFS_REAPURB32`
```rust
const USBDEVFS_REAPURB32: u32 = 1_074_025_740u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1074`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1074)*

### `BTRFS_IOC_TRANS_END`
```rust
const BTRFS_IOC_TRANS_END: u32 = 37_895u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1075`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1075)*

### `CAPI_REGISTER`
```rust
const CAPI_REGISTER: u32 = 1_074_545_409u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1076`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1076)*

### `F2FS_IOC_COMPRESS_FILE`
```rust
const F2FS_IOC_COMPRESS_FILE: u32 = 62_744u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1077`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1077)*

### `USBDEVFS_DISCARDURB`
```rust
const USBDEVFS_DISCARDURB: u32 = 21_771u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1078`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1078)*

### `HE_GET_REG`
```rust
const HE_GET_REG: u32 = 1_074_815_328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1079`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1079)*

### `ATM_SETLOOP`
```rust
const ATM_SETLOOP: u32 = 1_074_815_315u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1080`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1080)*

### `ATMSIGD_CTRL`
```rust
const ATMSIGD_CTRL: u32 = 25_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1081`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1081)*

### `CIOC_KERNEL_VERSION`
```rust
const CIOC_KERNEL_VERSION: u32 = 3_221_775_114u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1082`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1082)*

### `BTRFS_IOC_CLONE_RANGE`
```rust
const BTRFS_IOC_CLONE_RANGE: u32 = 1_075_876_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1083`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1083)*

### `SNAPSHOT_UNFREEZE`
```rust
const SNAPSHOT_UNFREEZE: u32 = 13_058u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1084`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1084)*

### `F2FS_IOC_START_VOLATILE_WRITE`
```rust
const F2FS_IOC_START_VOLATILE_WRITE: u32 = 62_723u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1085`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1085)*

### `PMU_IOC_HAS_ADB`
```rust
const PMU_IOC_HAS_ADB: u32 = 2_148_024_836u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1086`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1086)*

### `I2OGETIOPS`
```rust
const I2OGETIOPS: u32 = 2_149_607_680u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1087`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1087)*

### `VIDIOC_S_FBUF`
```rust
const VIDIOC_S_FBUF: u32 = 1_076_909_579u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1088`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1088)*

### `PPRCONTROL`
```rust
const PPRCONTROL: u32 = 2_147_577_987u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1089`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1089)*

### `CHIOSPICKER`
```rust
const CHIOSPICKER: u32 = 1_074_029_317u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1090`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1090)*

### `VFIO_IOMMU_SPAPR_REGISTER_MEMORY`
```rust
const VFIO_IOMMU_SPAPR_REGISTER_MEMORY: u32 = 15_221u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1091`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1091)*

### `TUNGETSNDBUF`
```rust
const TUNGETSNDBUF: u32 = 2_147_767_507u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1092`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1092)*

### `GSMIOC_SETCONF`
```rust
const GSMIOC_SETCONF: u32 = 1_078_740_737u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1093`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1093)*

### `IOC_PR_PREEMPT`
```rust
const IOC_PR_PREEMPT: u32 = 1_075_343_563u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1094`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1094)*

### `KCOV_INIT_TRACE`
```rust
const KCOV_INIT_TRACE: u32 = 2_148_033_281u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1095`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1095)*

### `SONYPI_IOCGBAT1CAP`
```rust
const SONYPI_IOCGBAT1CAP: u32 = 2_147_644_930u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1096`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1096)*

### `SWITCHTEC_IOCTL_FLASH_INFO`
```rust
const SWITCHTEC_IOCTL_FLASH_INFO: u32 = 2_148_554_560u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1097`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1097)*

### `MTIOCTOP`
```rust
const MTIOCTOP: u32 = 1_074_294_017u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1098`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1098)*

### `VHOST_VDPA_SET_STATUS`
```rust
const VHOST_VDPA_SET_STATUS: u32 = 1_073_852_274u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1099`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1099)*

### `VHOST_SCSI_SET_EVENTS_MISSED`
```rust
const VHOST_SCSI_SET_EVENTS_MISSED: u32 = 1_074_048_835u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1100`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1100)*

### `VFIO_IOMMU_DIRTY_PAGES`
```rust
const VFIO_IOMMU_DIRTY_PAGES: u32 = 15_221u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1101`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1101)*

### `BTRFS_IOC_SCRUB_PROGRESS`
```rust
const BTRFS_IOC_SCRUB_PROGRESS: u32 = 3_288_372_253u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1102`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1102)*

### `PPPIOCGMRU`
```rust
const PPPIOCGMRU: u32 = 2_147_775_571u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1103`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1103)*

### `BTRFS_IOC_DEV_REPLACE`
```rust
const BTRFS_IOC_DEV_REPLACE: u32 = 3_391_657_013u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1104`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1104)*

### `PPPIOCGFLAGS`
```rust
const PPPIOCGFLAGS: u32 = 2_147_775_578u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1105`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1105)*

### `NILFS_IOCTL_SET_SUINFO`
```rust
const NILFS_IOCTL_SET_SUINFO: u32 = 1_075_342_989u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1106`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1106)*

### `FW_CDEV_IOC_GET_CYCLE_TIMER2`
```rust
const FW_CDEV_IOC_GET_CYCLE_TIMER2: u32 = 3_222_807_316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1107`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1107)*

### `ATM_DELLECSADDR`
```rust
const ATM_DELLECSADDR: u32 = 1_074_815_375u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1108`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1108)*

### `FW_CDEV_IOC_GET_SPEED`
```rust
const FW_CDEV_IOC_GET_SPEED: u32 = 8_977u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1109`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1109)*

### `PPPIOCGIDLE32`
```rust
const PPPIOCGIDLE32: u32 = 2_148_037_695u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1110`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1110)*

### `VFIO_DEVICE_RESET`
```rust
const VFIO_DEVICE_RESET: u32 = 15_215u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1111`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1111)*

### `GPIO_GET_LINEINFO_UNWATCH_IOCTL`
```rust
const GPIO_GET_LINEINFO_UNWATCH_IOCTL: u32 = 3_221_533_708u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1112`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1112)*

### `WDIOC_GETSTATUS`
```rust
const WDIOC_GETSTATUS: u32 = 2_147_768_065u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1113`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1113)*

### `BTRFS_IOC_SET_FEATURES`
```rust
const BTRFS_IOC_SET_FEATURES: u32 = 1_076_925_497u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1114`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1114)*

### `IOCTL_MEI_CONNECT_CLIENT`
```rust
const IOCTL_MEI_CONNECT_CLIENT: u32 = 3_222_292_481u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1115`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1115)*

### `VIDIOC_OMAP3ISP_AEWB_CFG`
```rust
const VIDIOC_OMAP3ISP_AEWB_CFG: u32 = 3_223_344_835u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1116`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1116)*

### `PCITEST_READ`
```rust
const PCITEST_READ: u32 = 1_074_286_597u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1117`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1117)*

### `VFIO_GROUP_GET_STATUS`
```rust
const VFIO_GROUP_GET_STATUS: u32 = 15_207u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1118`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1118)*

### `MATROXFB_GET_ALL_OUTPUTS`
```rust
const MATROXFB_GET_ALL_OUTPUTS: u32 = 2_148_036_347u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1119`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1119)*

### `USBDEVFS_CLEAR_HALT`
```rust
const USBDEVFS_CLEAR_HALT: u32 = 2_147_767_573u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1120`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1120)*

### `VIDIOC_DECODER_CMD`
```rust
const VIDIOC_DECODER_CMD: u32 = 3_225_966_176u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1121`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1121)*

### `VIDIOC_G_AUDIO`
```rust
const VIDIOC_G_AUDIO: u32 = 2_150_913_569u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1122`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1122)*

### `CCISS_RESCANDISK`
```rust
const CCISS_RESCANDISK: u32 = 16_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1123`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1123)*

### `RIO_DISABLE_PORTWRITE_RANGE`
```rust
const RIO_DISABLE_PORTWRITE_RANGE: u32 = 1_074_818_316u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1124`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1124)*

### `IOC_OPAL_SECURE_ERASE_LR`
```rust
const IOC_OPAL_SECURE_ERASE_LR: u32 = 1_091_596_519u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1125`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1125)*

### `USBDEVFS_REAPURB`
```rust
const USBDEVFS_REAPURB: u32 = 1_074_287_884u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1126`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1126)*

### `DFL_FPGA_CHECK_EXTENSION`
```rust
const DFL_FPGA_CHECK_EXTENSION: u32 = 46_593u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1127`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1127)*

### `AUTOFS_IOC_PROTOVER`
```rust
const AUTOFS_IOC_PROTOVER: u32 = 2_147_783_523u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1128`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1128)*

### `FSL_HV_IOCTL_MEMCPY`
```rust
const FSL_HV_IOCTL_MEMCPY: u32 = 3_223_891_717u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1129`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1129)*

### `BTRFS_IOC_GET_FEATURES`
```rust
const BTRFS_IOC_GET_FEATURES: u32 = 2_149_094_457u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1130`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1130)*

### `PCITEST_MSIX`
```rust
const PCITEST_MSIX: u32 = 1_074_024_455u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1131`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1131)*

### `BTRFS_IOC_DEFRAG_RANGE`
```rust
const BTRFS_IOC_DEFRAG_RANGE: u32 = 1_076_925_456u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1132`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1132)*

### `UI_BEGIN_FF_ERASE`
```rust
const UI_BEGIN_FF_ERASE: u32 = 3_222_033_866u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1133`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1133)*

### `DM_GET_TARGET_VERSION`
```rust
const DM_GET_TARGET_VERSION: u32 = 3_241_737_489u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1134`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1134)*

### `PPPIOCGIDLE`
```rust
const PPPIOCGIDLE: u32 = 2_148_561_983u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1135`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1135)*

### `NVRAM_SETCKS`
```rust
const NVRAM_SETCKS: u32 = 28_737u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1136`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1136)*

### `WDIOC_GETSUPPORT`
```rust
const WDIOC_GETSUPPORT: u32 = 2_150_127_360u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1137`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1137)*

### `GSMIOC_ENABLE_NET`
```rust
const GSMIOC_ENABLE_NET: u32 = 1_077_167_874u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1138`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1138)*

### `GPIO_GET_CHIPINFO_IOCTL`
```rust
const GPIO_GET_CHIPINFO_IOCTL: u32 = 2_151_986_177u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1139`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1139)*

### `NE_ADD_VCPU`
```rust
const NE_ADD_VCPU: u32 = 3_221_532_193u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1140`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1140)*

### `EVIOCSKEYCODE_V2`
```rust
const EVIOCSKEYCODE_V2: u32 = 1_076_380_932u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1141`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1141)*

### `PTP_SYS_OFFSET_EXTENDED2`
```rust
const PTP_SYS_OFFSET_EXTENDED2: u32 = 3_300_932_882u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1142`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1142)*

### `SCIF_FENCE_WAIT`
```rust
const SCIF_FENCE_WAIT: u32 = 3_221_517_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1143`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1143)*

### `RIO_TRANSFER`
```rust
const RIO_TRANSFER: u32 = 3_222_826_261u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1144`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1144)*

### `FSL_HV_IOCTL_DOORBELL`
```rust
const FSL_HV_IOCTL_DOORBELL: u32 = 3_221_794_566u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1145`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1145)*

### `RIO_MPORT_MAINT_WRITE_LOCAL`
```rust
const RIO_MPORT_MAINT_WRITE_LOCAL: u32 = 1_075_342_598u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1146`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1146)*

### `I2OEVTREG`
```rust
const I2OEVTREG: u32 = 1_074_555_146u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1147`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1147)*

### `I2OPARMGET`
```rust
const I2OPARMGET: u32 = 3_223_873_796u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1148`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1148)*

### `EVIOCGID`
```rust
const EVIOCGID: u32 = 2_148_025_602u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1149`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1149)*

### `BTRFS_IOC_QGROUP_CREATE`
```rust
const BTRFS_IOC_QGROUP_CREATE: u32 = 1_074_828_330u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1150`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1150)*

### `AUTOFS_DEV_IOCTL_SETPIPEFD`
```rust
const AUTOFS_DEV_IOCTL_SETPIPEFD: u32 = 3_222_836_088u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1151`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1151)*

### `VIDIOC_S_PARM`
```rust
const VIDIOC_S_PARM: u32 = 3_234_616_854u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1152`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1152)*

### `TUNSETSTEERINGEBPF`
```rust
const TUNSETSTEERINGEBPF: u32 = 2_147_767_520u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1153`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1153)*

### `ATM_GETNAMES`
```rust
const ATM_GETNAMES: u32 = 1_074_815_363u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1154`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1154)*

### `VIDIOC_QUERYMENU`
```rust
const VIDIOC_QUERYMENU: u32 = 3_224_131_109u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1155`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1155)*

### `DFL_FPGA_PORT_DMA_UNMAP`
```rust
const DFL_FPGA_PORT_DMA_UNMAP: u32 = 46_660u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1156`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1156)*

### `I2OLCTGET`
```rust
const I2OLCTGET: u32 = 3_222_825_218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1157`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1157)*

### `FS_IOC_GET_ENCRYPTION_PWSALT`
```rust
const FS_IOC_GET_ENCRYPTION_PWSALT: u32 = 1_074_816_532u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1158`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1158)*

### `NS_SETBUFLEV`
```rust
const NS_SETBUFLEV: u32 = 1_074_815_330u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1159`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1159)*

### `BLKCLOSEZONE`
```rust
const BLKCLOSEZONE: u32 = 1_074_795_143u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1160`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1160)*

### `SONET_GETFRSENSE`
```rust
const SONET_GETFRSENSE: u32 = 2_147_901_719u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1161`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1161)*

### `UI_SET_EVBIT`
```rust
const UI_SET_EVBIT: u32 = 1_074_025_828u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1162`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1162)*

### `DM_LIST_VERSIONS`
```rust
const DM_LIST_VERSIONS: u32 = 3_241_737_485u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1163`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1163)*

### `HIDIOCGSTRING`
```rust
const HIDIOCGSTRING: u32 = 2_164_541_444u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1164`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1164)*

### `PPPIOCATTCHAN`
```rust
const PPPIOCATTCHAN: u32 = 1_074_033_720u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1165`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1165)*

### `VDUSE_DEV_SET_CONFIG`
```rust
const VDUSE_DEV_SET_CONFIG: u32 = 1_074_299_154u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1166`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1166)*

### `TUNGETFEATURES`
```rust
const TUNGETFEATURES: u32 = 2_147_767_503u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1167`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1167)*

### `VFIO_GROUP_UNSET_CONTAINER`
```rust
const VFIO_GROUP_UNSET_CONTAINER: u32 = 15_209u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1168`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1168)*

### `IPMICTL_SET_MY_ADDRESS_CMD`
```rust
const IPMICTL_SET_MY_ADDRESS_CMD: u32 = 2_147_772_689u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1169`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1169)*

### `CCISS_REGNEWDISK`
```rust
const CCISS_REGNEWDISK: u32 = 1_074_020_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1170`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1170)*

### `VIDIOC_QUERY_DV_TIMINGS`
```rust
const VIDIOC_QUERY_DV_TIMINGS: u32 = 2_156_156_515u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1171`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1171)*

### `PHN_SETREGS`
```rust
const PHN_SETREGS: u32 = 1_076_391_944u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1172`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1172)*

### `FAT_IOCTL_GET_ATTRIBUTES`
```rust
const FAT_IOCTL_GET_ATTRIBUTES: u32 = 2_147_774_992u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1173`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1173)*

### `FSL_MC_SEND_MC_COMMAND`
```rust
const FSL_MC_SEND_MC_COMMAND: u32 = 3_225_440_992u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1174`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1174)*

### `TUNGETIFF`
```rust
const TUNGETIFF: u32 = 2_147_767_506u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1175`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1175)*

### `PTP_CLOCK_GETCAPS2`
```rust
const PTP_CLOCK_GETCAPS2: u32 = 2_152_742_154u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1176`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1176)*

### `BTRFS_IOC_RESIZE`
```rust
const BTRFS_IOC_RESIZE: u32 = 1_342_215_171u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1177`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1177)*

### `VHOST_SET_VRING_ENDIAN`
```rust
const VHOST_SET_VRING_ENDIAN: u32 = 1_074_310_931u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1178`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1178)*

### `PPS_KC_BIND`
```rust
const PPS_KC_BIND: u32 = 1_074_294_949u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1179`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1179)*

### `F2FS_IOC_WRITE_CHECKPOINT`
```rust
const F2FS_IOC_WRITE_CHECKPOINT: u32 = 62_727u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1180`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1180)*

### `UI_SET_FFBIT`
```rust
const UI_SET_FFBIT: u32 = 1_074_025_835u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1181`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1181)*

### `IPMICTL_GET_MY_LUN_CMD`
```rust
const IPMICTL_GET_MY_LUN_CMD: u32 = 2_147_772_692u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1182`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1182)*

### `CEC_ADAP_G_PHYS_ADDR`
```rust
const CEC_ADAP_G_PHYS_ADDR: u32 = 2_147_639_553u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1183`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1183)*

### `CEC_G_MODE`
```rust
const CEC_G_MODE: u32 = 2_147_770_632u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1184`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1184)*

### `USBDEVFS_RESETEP`
```rust
const USBDEVFS_RESETEP: u32 = 2_147_767_555u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1185`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1185)*

### `MEDIA_REQUEST_IOC_QUEUE`
```rust
const MEDIA_REQUEST_IOC_QUEUE: u32 = 31_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1186`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1186)*

### `USBDEVFS_ALLOC_STREAMS`
```rust
const USBDEVFS_ALLOC_STREAMS: u32 = 2_148_029_724u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1187`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1187)*

### `MGSL_IOCSXCTRL`
```rust
const MGSL_IOCSXCTRL: u32 = 27_925u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1188`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1188)*

### `MEDIA_IOC_G_TOPOLOGY`
```rust
const MEDIA_IOC_G_TOPOLOGY: u32 = 3_225_975_812u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1189`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1189)*

### `PPPIOCUNBRIDGECHAN`
```rust
const PPPIOCUNBRIDGECHAN: u32 = 29_748u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1190`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1190)*

### `F2FS_IOC_COMMIT_ATOMIC_WRITE`
```rust
const F2FS_IOC_COMMIT_ATOMIC_WRITE: u32 = 62_722u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1191`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1191)*

### `ISST_IF_GET_PLATFORM_INFO`
```rust
const ISST_IF_GET_PLATFORM_INFO: u32 = 2_148_072_960u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1192`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1192)*

### `SCIF_FENCE_MARK`
```rust
const SCIF_FENCE_MARK: u32 = 3_222_303_503u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1193`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1193)*

### `USBDEVFS_RELEASE_PORT`
```rust
const USBDEVFS_RELEASE_PORT: u32 = 2_147_767_577u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1194`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1194)*

### `VFIO_CHECK_EXTENSION`
```rust
const VFIO_CHECK_EXTENSION: u32 = 15_205u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1195`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1195)*

### `BTRFS_IOC_QGROUP_LIMIT`
```rust
const BTRFS_IOC_QGROUP_LIMIT: u32 = 2_150_667_307u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1196`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1196)*

### `FAT_IOCTL_GET_VOLUME_ID`
```rust
const FAT_IOCTL_GET_VOLUME_ID: u32 = 2_147_774_995u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1197`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1197)*

### `UI_SET_PHYS`
```rust
const UI_SET_PHYS: u32 = 1_074_287_980u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1198`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1198)*

### `FDWERRORGET`
```rust
const FDWERRORGET: u32 = 2_150_105_623u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1199`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1199)*

### `VIDIOC_SUBDEV_G_EDID`
```rust
const VIDIOC_SUBDEV_G_EDID: u32 = 3_223_868_968u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1200`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1200)*

### `MGSL_IOCGSTATS`
```rust
const MGSL_IOCGSTATS: u32 = 27_911u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1201`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1201)*

### `RPROC_SET_SHUTDOWN_ON_RELEASE`
```rust
const RPROC_SET_SHUTDOWN_ON_RELEASE: u32 = 1_074_050_817u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1202`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1202)*

### `SIOCGSTAMP_NEW`
```rust
const SIOCGSTAMP_NEW: u32 = 2_148_567_302u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1203`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1203)*

### `RTC_WKALM_RD`
```rust
const RTC_WKALM_RD: u32 = 2_150_133_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1204`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1204)*

### `PHN_GET_REG`
```rust
const PHN_GET_REG: u32 = 3_221_778_432u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1205`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1205)*

### `DELL_WMI_SMBIOS_CMD`
```rust
const DELL_WMI_SMBIOS_CMD: u32 = 3_224_655_616u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1206`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1206)*

### `PHN_NOT_OH`
```rust
const PHN_NOT_OH: u32 = 28_676u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1207`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1207)*

### `PPGETMODES`
```rust
const PPGETMODES: u32 = 2_147_774_615u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1208`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1208)*

### `CHIOGPARAMS`
```rust
const CHIOGPARAMS: u32 = 2_148_819_718u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1209`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1209)*

### `VFIO_DEVICE_GET_GFX_DMABUF`
```rust
const VFIO_DEVICE_GET_GFX_DMABUF: u32 = 15_219u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1210`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1210)*

### `VHOST_SET_VRING_BUSYLOOP_TIMEOUT`
```rust
const VHOST_SET_VRING_BUSYLOOP_TIMEOUT: u32 = 1_074_310_947u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1211`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1211)*

### `VIDIOC_SUBDEV_G_SELECTION`
```rust
const VIDIOC_SUBDEV_G_SELECTION: u32 = 3_225_441_853u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1212`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1212)*

### `BTRFS_IOC_RM_DEV_V2`
```rust
const BTRFS_IOC_RM_DEV_V2: u32 = 1_342_215_226u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1213`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1213)*

### `MGSL_IOCWAITGPIO`
```rust
const MGSL_IOCWAITGPIO: u32 = 3_222_301_970u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1214`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1214)*

### `PMU_IOC_CAN_SLEEP`
```rust
const PMU_IOC_CAN_SLEEP: u32 = 2_148_024_837u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1215`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1215)*

### `KCOV_ENABLE`
```rust
const KCOV_ENABLE: u32 = 25_444u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1216`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1216)*

### `BTRFS_IOC_CLONE`
```rust
const BTRFS_IOC_CLONE: u32 = 1_074_041_865u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1217`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1217)*

### `F2FS_IOC_DEFRAGMENT`
```rust
const F2FS_IOC_DEFRAGMENT: u32 = 3_222_336_776u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1218`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1218)*

### `FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE`
```rust
const FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE: u32 = 1_074_012_942u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1219`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1219)*

### `AGPIOC_ALLOCATE`
```rust
const AGPIOC_ALLOCATE: u32 = 3_221_766_406u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1220`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1220)*

### `NE_SET_USER_MEMORY_REGION`
```rust
const NE_SET_USER_MEMORY_REGION: u32 = 1_075_359_267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1221`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1221)*

### `MGSL_IOCTXABORT`
```rust
const MGSL_IOCTXABORT: u32 = 27_910u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1222`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1222)*

### `MGSL_IOCSGPIO`
```rust
const MGSL_IOCSGPIO: u32 = 1_074_818_320u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1223`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1223)*

### `LIRC_SET_REC_CARRIER`
```rust
const LIRC_SET_REC_CARRIER: u32 = 1_074_030_868u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1224`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1224)*

### `F2FS_IOC_FLUSH_DEVICE`
```rust
const F2FS_IOC_FLUSH_DEVICE: u32 = 1_074_328_842u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1225`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1225)*

### `SNAPSHOT_ATOMIC_RESTORE`
```rust
const SNAPSHOT_ATOMIC_RESTORE: u32 = 13_060u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1226`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1226)*

### `RTC_UIE_OFF`
```rust
const RTC_UIE_OFF: u32 = 28_676u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1227`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1227)*

### `BT_BMC_IOCTL_SMS_ATN`
```rust
const BT_BMC_IOCTL_SMS_ATN: u32 = 45_312u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1228`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1228)*

### `NVME_IOCTL_ID`
```rust
const NVME_IOCTL_ID: u32 = 20_032u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1229`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1229)*

### `NE_START_ENCLAVE`
```rust
const NE_START_ENCLAVE: u32 = 3_222_318_628u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1230`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1230)*

### `VIDIOC_STREAMON`
```rust
const VIDIOC_STREAMON: u32 = 1_074_026_002u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1231`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1231)*

### `FDPOLLDRVSTAT`
```rust
const FDPOLLDRVSTAT: u32 = 2_152_727_059u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1232`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1232)*

### `AUTOFS_DEV_IOCTL_READY`
```rust
const AUTOFS_DEV_IOCTL_READY: u32 = 3_222_836_086u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1233`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1233)*

### `VIDIOC_ENUMAUDOUT`
```rust
const VIDIOC_ENUMAUDOUT: u32 = 3_224_655_426u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1234`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1234)*

### `VIDIOC_SUBDEV_S_STD`
```rust
const VIDIOC_SUBDEV_S_STD: u32 = 1_074_288_152u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1235`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1235)*

### `WDIOC_GETTIMELEFT`
```rust
const WDIOC_GETTIMELEFT: u32 = 2_147_768_074u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1236`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1236)*

### `ATM_GETLINKRATE`
```rust
const ATM_GETLINKRATE: u32 = 1_074_815_361u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1237`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1237)*

### `RTC_WKALM_SET`
```rust
const RTC_WKALM_SET: u32 = 1_076_391_951u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1238`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1238)*

### `VHOST_GET_BACKEND_FEATURES`
```rust
const VHOST_GET_BACKEND_FEATURES: u32 = 2_148_052_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1239`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1239)*

### `ATMARP_ENCAP`
```rust
const ATMARP_ENCAP: u32 = 25_061u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1240`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1240)*

### `CAPI_GET_FLAGS`
```rust
const CAPI_GET_FLAGS: u32 = 2_147_762_979u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1241`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1241)*

### `IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD`
```rust
const IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD: u32 = 2_147_772_696u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1242`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1242)*

### `DFL_FPGA_FME_PORT_ASSIGN`
```rust
const DFL_FPGA_FME_PORT_ASSIGN: u32 = 1_074_050_690u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1243`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1243)*

### `NS_GET_OWNER_UID`
```rust
const NS_GET_OWNER_UID: u32 = 46_852u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1244`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1244)*

### `VIDIOC_OVERLAY`
```rust
const VIDIOC_OVERLAY: u32 = 1_074_025_998u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1245`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1245)*

### `BTRFS_IOC_WAIT_SYNC`
```rust
const BTRFS_IOC_WAIT_SYNC: u32 = 1_074_304_022u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1246`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1246)*

### `GPIOHANDLE_SET_CONFIG_IOCTL`
```rust
const GPIOHANDLE_SET_CONFIG_IOCTL: u32 = 3_226_776_586u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1247`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1247)*

### `VHOST_GET_VRING_ENDIAN`
```rust
const VHOST_GET_VRING_ENDIAN: u32 = 1_074_310_932u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1248`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1248)*

### `ATM_GETADDR`
```rust
const ATM_GETADDR: u32 = 1_074_815_366u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1249`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1249)*

### `PHN_GET_REGS`
```rust
const PHN_GET_REGS: u32 = 3_221_778_434u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1250`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1250)*

### `AUTOFS_DEV_IOCTL_REQUESTER`
```rust
const AUTOFS_DEV_IOCTL_REQUESTER: u32 = 3_222_836_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1251`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1251)*

### `AUTOFS_DEV_IOCTL_EXPIRE`
```rust
const AUTOFS_DEV_IOCTL_EXPIRE: u32 = 3_222_836_092u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1252`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1252)*

### `SNAPSHOT_S2RAM`
```rust
const SNAPSHOT_S2RAM: u32 = 13_067u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1253`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1253)*

### `JSIOCSAXMAP`
```rust
const JSIOCSAXMAP: u32 = 1_077_963_313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1254`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1254)*

### `F2FS_IOC_SET_COMPRESS_OPTION`
```rust
const F2FS_IOC_SET_COMPRESS_OPTION: u32 = 1_073_935_638u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1255`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1255)*

### `VBG_IOCTL_HGCM_DISCONNECT`
```rust
const VBG_IOCTL_HGCM_DISCONNECT: u32 = 3_223_082_501u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1256`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1256)*

### `SCIF_FENCE_SIGNAL`
```rust
const SCIF_FENCE_SIGNAL: u32 = 3_223_876_369u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1257`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1257)*

### `VFIO_DEVICE_GET_PCI_HOT_RESET_INFO`
```rust
const VFIO_DEVICE_GET_PCI_HOT_RESET_INFO: u32 = 15_216u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1258`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1258)*

### `VIDIOC_SUBDEV_ENUM_MBUS_CODE`
```rust
const VIDIOC_SUBDEV_ENUM_MBUS_CODE: u32 = 3_224_393_218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1259`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1259)*

### `MMTIMER_GETOFFSET`
```rust
const MMTIMER_GETOFFSET: u32 = 27_904u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1260`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1260)*

### `RIO_CM_CHAN_LISTEN`
```rust
const RIO_CM_CHAN_LISTEN: u32 = 1_073_898_246u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1261`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1261)*

### `ATM_SETSC`
```rust
const ATM_SETSC: u32 = 1_074_029_041u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1262`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1262)*

### `F2FS_IOC_SHUTDOWN`
```rust
const F2FS_IOC_SHUTDOWN: u32 = 2_147_768_445u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1263`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1263)*

### `NVME_IOCTL_RESCAN`
```rust
const NVME_IOCTL_RESCAN: u32 = 20_038u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1264`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1264)*

### `BLKOPENZONE`
```rust
const BLKOPENZONE: u32 = 1_074_795_142u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1265`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1265)*

### `DM_VERSION`
```rust
const DM_VERSION: u32 = 3_241_737_472u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1266`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1266)*

### `CEC_TRANSMIT`
```rust
const CEC_TRANSMIT: u32 = 3_224_920_325u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1267`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1267)*

### `FS_IOC_GET_ENCRYPTION_POLICY_EX`
```rust
const FS_IOC_GET_ENCRYPTION_POLICY_EX: u32 = 3_221_841_430u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1268`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1268)*

### `SIOCMKCLIP`
```rust
const SIOCMKCLIP: u32 = 25_056u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1269`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1269)*

### `IPMI_BMC_IOCTL_CLEAR_SMS_ATN`
```rust
const IPMI_BMC_IOCTL_CLEAR_SMS_ATN: u32 = 45_313u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1270`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1270)*

### `HIDIOCGVERSION`
```rust
const HIDIOCGVERSION: u32 = 2_147_764_225u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1271`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1271)*

### `VIDIOC_S_INPUT`
```rust
const VIDIOC_S_INPUT: u32 = 3_221_509_671u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1272`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1272)*

### `VIDIOC_G_CROP`
```rust
const VIDIOC_G_CROP: u32 = 3_222_558_267u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1273`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1273)*

### `LIRC_SET_WIDEBAND_RECEIVER`
```rust
const LIRC_SET_WIDEBAND_RECEIVER: u32 = 1_074_030_883u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1274`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1274)*

### `EVIOCGEFFECTS`
```rust
const EVIOCGEFFECTS: u32 = 2_147_763_588u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1275`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1275)*

### `UVCIOC_CTRL_QUERY`
```rust
const UVCIOC_CTRL_QUERY: u32 = 3_222_304_033u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1276`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1276)*

### `IOC_OPAL_GENERIC_TABLE_RW`
```rust
const IOC_OPAL_GENERIC_TABLE_RW: u32 = 1_094_217_963u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1277`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1277)*

### `FS_IOC_READ_VERITY_METADATA`
```rust
const FS_IOC_READ_VERITY_METADATA: u32 = 3_223_873_159u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1278`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1278)*

### `ND_IOCTL_SET_CONFIG_DATA`
```rust
const ND_IOCTL_SET_CONFIG_DATA: u32 = 3_221_769_734u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1279`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1279)*

### `USBDEVFS_GETDRIVER`
```rust
const USBDEVFS_GETDRIVER: u32 = 1_090_802_952u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1280`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1280)*

### `IDT77105_GETSTAT`
```rust
const IDT77105_GETSTAT: u32 = 1_074_815_282u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1281`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1281)*

### `HIDIOCINITREPORT`
```rust
const HIDIOCINITREPORT: u32 = 18_437u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1282`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1282)*

### `VFIO_DEVICE_GET_INFO`
```rust
const VFIO_DEVICE_GET_INFO: u32 = 15_211u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1283`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1283)*

### `RIO_CM_CHAN_RECEIVE`
```rust
const RIO_CM_CHAN_RECEIVE: u32 = 3_222_299_402u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1284`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1284)*

### `RNDGETENTCNT`
```rust
const RNDGETENTCNT: u32 = 2_147_766_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1285`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1285)*

### `PPPIOCNEWUNIT`
```rust
const PPPIOCNEWUNIT: u32 = 3_221_517_374u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1286`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1286)*

### `BTRFS_IOC_INO_LOOKUP`
```rust
const BTRFS_IOC_INO_LOOKUP: u32 = 3_489_698_834u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1287`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1287)*

### `FDRESET`
```rust
const FDRESET: u32 = 596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1288`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1288)*

### `IOC_PR_REGISTER`
```rust
const IOC_PR_REGISTER: u32 = 1_075_343_560u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1289`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1289)*

### `HIDIOCSREPORT`
```rust
const HIDIOCSREPORT: u32 = 1_074_546_696u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1290`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1290)*

### `TEE_IOC_OPEN_SESSION`
```rust
const TEE_IOC_OPEN_SESSION: u32 = 2_148_574_210u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1291`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1291)*

### `TEE_IOC_SUPPL_RECV`
```rust
const TEE_IOC_SUPPL_RECV: u32 = 2_148_574_214u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1292`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1292)*

### `BTRFS_IOC_BALANCE_CTL`
```rust
const BTRFS_IOC_BALANCE_CTL: u32 = 1_074_041_889u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1293`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1293)*

### `GPIO_GET_LINEINFO_WATCH_IOCTL`
```rust
const GPIO_GET_LINEINFO_WATCH_IOCTL: u32 = 3_225_990_155u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1294`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1294)*

### `HIDIOCGRAWINFO`
```rust
const HIDIOCGRAWINFO: u32 = 2_148_026_371u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1295`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1295)*

### `PPPIOCSCOMPRESS`
```rust
const PPPIOCSCOMPRESS: u32 = 1_074_820_173u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1296`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1296)*

### `USBDEVFS_CONNECTINFO`
```rust
const USBDEVFS_CONNECTINFO: u32 = 1_074_287_889u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1297`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1297)*

### `BLKRESETZONE`
```rust
const BLKRESETZONE: u32 = 1_074_795_139u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1298`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1298)*

### `CHIOINITELEM`
```rust
const CHIOINITELEM: u32 = 25_361u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1299`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1299)*

### `NILFS_IOCTL_SET_ALLOC_RANGE`
```rust
const NILFS_IOCTL_SET_ALLOC_RANGE: u32 = 1_074_818_700u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1300`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1300)*

### `AUTOFS_DEV_IOCTL_CATATONIC`
```rust
const AUTOFS_DEV_IOCTL_CATATONIC: u32 = 3_222_836_089u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1301`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1301)*

### `RIO_MPORT_MAINT_HDID_SET`
```rust
const RIO_MPORT_MAINT_HDID_SET: u32 = 1_073_900_801u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1302`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1302)*

### `PPGETPHASE`
```rust
const PPGETPHASE: u32 = 2_147_774_617u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1303`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1303)*

### `USBDEVFS_DISCONNECT_CLAIM`
```rust
const USBDEVFS_DISCONNECT_CLAIM: u32 = 2_164_806_939u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1304`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1304)*

### `FDMSGON`
```rust
const FDMSGON: u32 = 581u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1305`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1305)*

### `VIDIOC_G_SLICED_VBI_CAP`
```rust
const VIDIOC_G_SLICED_VBI_CAP: u32 = 3_228_849_733u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1306`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1306)*

### `BTRFS_IOC_BALANCE_V2`
```rust
const BTRFS_IOC_BALANCE_V2: u32 = 3_288_372_256u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1307`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1307)*

### `MEDIA_REQUEST_IOC_REINIT`
```rust
const MEDIA_REQUEST_IOC_REINIT: u32 = 31_873u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1308`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1308)*

### `IOC_OPAL_ERASE_LR`
```rust
const IOC_OPAL_ERASE_LR: u32 = 1_091_596_518u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1309`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1309)*

### `FDFMTBEG`
```rust
const FDFMTBEG: u32 = 583u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1310`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1310)*

### `RNDRESEEDCRNG`
```rust
const RNDRESEEDCRNG: u32 = 20_999u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1311`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1311)*

### `ISST_IF_GET_PHY_ID`
```rust
const ISST_IF_GET_PHY_ID: u32 = 3_221_814_785u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1312`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1312)*

### `TUNSETNOCSUM`
```rust
const TUNSETNOCSUM: u32 = 1_074_025_672u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1313`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1313)*

### `SONET_GETSTAT`
```rust
const SONET_GETSTAT: u32 = 2_149_867_792u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1314`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1314)*

### `TFD_IOC_SET_TICKS`
```rust
const TFD_IOC_SET_TICKS: u32 = 1_074_287_616u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1315`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1315)*

### `PPDATADIR`
```rust
const PPDATADIR: u32 = 1_074_032_784u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1316`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1316)*

### `IOC_OPAL_ENABLE_DISABLE_MBR`
```rust
const IOC_OPAL_ENABLE_DISABLE_MBR: u32 = 1_091_596_517u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1317`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1317)*

### `GPIO_V2_GET_LINE_IOCTL`
```rust
const GPIO_V2_GET_LINE_IOCTL: u32 = 3_260_068_871u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1318`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1318)*

### `RIO_CM_CHAN_SEND`
```rust
const RIO_CM_CHAN_SEND: u32 = 1_074_815_753u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1319`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1319)*

### `PPWCTLONIRQ`
```rust
const PPWCTLONIRQ: u32 = 1_073_836_178u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1320`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1320)*

### `SONYPI_IOCGBRT`
```rust
const SONYPI_IOCGBRT: u32 = 2_147_579_392u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1321`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1321)*

### `IOC_PR_RELEASE`
```rust
const IOC_PR_RELEASE: u32 = 1_074_819_274u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1322`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1322)*

### `PPCLRIRQ`
```rust
const PPCLRIRQ: u32 = 2_147_774_611u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1323`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1323)*

### `IPMICTL_SET_MY_CHANNEL_LUN_CMD`
```rust
const IPMICTL_SET_MY_CHANNEL_LUN_CMD: u32 = 2_147_772_698u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1324`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1324)*

### `MGSL_IOCSXSYNC`
```rust
const MGSL_IOCSXSYNC: u32 = 27_923u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1325`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1325)*

### `HPET_IE_OFF`
```rust
const HPET_IE_OFF: u32 = 26_626u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1326`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1326)*

### `IOC_OPAL_ACTIVATE_USR`
```rust
const IOC_OPAL_ACTIVATE_USR: u32 = 1_091_596_513u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1327`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1327)*

### `SONET_SETFRAMING`
```rust
const SONET_SETFRAMING: u32 = 1_074_028_821u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1328`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1328)*

### `PERF_EVENT_IOC_PAUSE_OUTPUT`
```rust
const PERF_EVENT_IOC_PAUSE_OUTPUT: u32 = 1_074_013_193u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1329`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1329)*

### `BTRFS_IOC_LOGICAL_INO_V2`
```rust
const BTRFS_IOC_LOGICAL_INO_V2: u32 = 3_224_933_435u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1330`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1330)*

### `VBG_IOCTL_HGCM_CONNECT`
```rust
const VBG_IOCTL_HGCM_CONNECT: u32 = 3_231_471_108u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1331`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1331)*

### `BLKFINISHZONE`
```rust
const BLKFINISHZONE: u32 = 1_074_795_144u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1332`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1332)*

### `EVIOCREVOKE`
```rust
const EVIOCREVOKE: u32 = 1_074_021_777u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1333`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1333)*

### `VFIO_DEVICE_FEATURE`
```rust
const VFIO_DEVICE_FEATURE: u32 = 15_221u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1334`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1334)*

### `CCISS_GETPCIINFO`
```rust
const CCISS_GETPCIINFO: u32 = 2_148_024_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1335`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1335)*

### `ISST_IF_MBOX_COMMAND`
```rust
const ISST_IF_MBOX_COMMAND: u32 = 3_221_814_787u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1336`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1336)*

### `SCIF_ACCEPTREQ`
```rust
const SCIF_ACCEPTREQ: u32 = 3_222_303_492u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1337`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1337)*

### `PERF_EVENT_IOC_QUERY_BPF`
```rust
const PERF_EVENT_IOC_QUERY_BPF: u32 = 3_221_758_986u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1338`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1338)*

### `VIDIOC_STREAMOFF`
```rust
const VIDIOC_STREAMOFF: u32 = 1_074_026_003u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1339`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1339)*

### `VDUSE_DESTROY_DEV`
```rust
const VDUSE_DESTROY_DEV: u32 = 1_090_552_067u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1340`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1340)*

### `FDGETFDCSTAT`
```rust
const FDGETFDCSTAT: u32 = 2_150_105_621u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1341`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1341)*

### `VIDIOC_S_PRIORITY`
```rust
const VIDIOC_S_PRIORITY: u32 = 1_074_026_052u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1342`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1342)*

### `SNAPSHOT_FREEZE`
```rust
const SNAPSHOT_FREEZE: u32 = 13_057u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1343`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1343)*

### `VIDIOC_ENUMINPUT`
```rust
const VIDIOC_ENUMINPUT: u32 = 3_226_490_394u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1344`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1344)*

### `ZATM_GETPOOLZ`
```rust
const ZATM_GETPOOLZ: u32 = 1_074_815_330u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1345`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1345)*

### `RIO_DISABLE_DOORBELL_RANGE`
```rust
const RIO_DISABLE_DOORBELL_RANGE: u32 = 1_074_294_026u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1346`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1346)*

### `GPIO_V2_GET_LINEINFO_WATCH_IOCTL`
```rust
const GPIO_V2_GET_LINEINFO_WATCH_IOCTL: u32 = 3_238_048_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1347`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1347)*

### `VIDIOC_G_STD`
```rust
const VIDIOC_G_STD: u32 = 2_148_029_975u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1348`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1348)*

### `USBDEVFS_ALLOW_SUSPEND`
```rust
const USBDEVFS_ALLOW_SUSPEND: u32 = 21_794u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1349`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1349)*

### `SONET_GETSTATZ`
```rust
const SONET_GETSTATZ: u32 = 2_149_867_793u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1350`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1350)*

### `SCIF_ACCEPTREG`
```rust
const SCIF_ACCEPTREG: u32 = 3_221_779_205u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1351`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1351)*

### `VIDIOC_ENCODER_CMD`
```rust
const VIDIOC_ENCODER_CMD: u32 = 3_223_869_005u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1352`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1352)*

### `PPPIOCSRASYNCMAP`
```rust
const PPPIOCSRASYNCMAP: u32 = 1_074_033_748u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1353`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1353)*

### `IOCTL_MEI_NOTIFY_SET`
```rust
const IOCTL_MEI_NOTIFY_SET: u32 = 1_074_022_402u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1354`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1354)*

### `BTRFS_IOC_QUOTA_RESCAN_STATUS`
```rust
const BTRFS_IOC_QUOTA_RESCAN_STATUS: u32 = 2_151_715_885u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1355`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1355)*

### `F2FS_IOC_GARBAGE_COLLECT`
```rust
const F2FS_IOC_GARBAGE_COLLECT: u32 = 1_074_066_694u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1356`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1356)*

### `ATMLEC_CTRL`
```rust
const ATMLEC_CTRL: u32 = 25_040u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1357`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1357)*

### `MATROXFB_GET_AVAILABLE_OUTPUTS`
```rust
const MATROXFB_GET_AVAILABLE_OUTPUTS: u32 = 2_148_036_345u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1358`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1358)*

### `DM_DEV_CREATE`
```rust
const DM_DEV_CREATE: u32 = 3_241_737_475u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1359`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1359)*

### `VHOST_VDPA_GET_VRING_NUM`
```rust
const VHOST_VDPA_GET_VRING_NUM: u32 = 2_147_659_638u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1360`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1360)*

### `VIDIOC_G_CTRL`
```rust
const VIDIOC_G_CTRL: u32 = 3_221_771_803u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1361`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1361)*

### `NBD_CLEAR_SOCK`
```rust
const NBD_CLEAR_SOCK: u32 = 43_780u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1362`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1362)*

### `VFIO_DEVICE_QUERY_GFX_PLANE`
```rust
const VFIO_DEVICE_QUERY_GFX_PLANE: u32 = 15_218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1363`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1363)*

### `WDIOC_KEEPALIVE`
```rust
const WDIOC_KEEPALIVE: u32 = 2_147_768_069u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1364`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1364)*

### `NVME_IOCTL_SUBSYS_RESET`
```rust
const NVME_IOCTL_SUBSYS_RESET: u32 = 20_037u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1365`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1365)*

### `PTP_EXTTS_REQUEST2`
```rust
const PTP_EXTTS_REQUEST2: u32 = 1_074_806_027u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1366`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1366)*

### `PCITEST_BAR`
```rust
const PCITEST_BAR: u32 = 20_481u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1367`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1367)*

### `MGSL_IOCGGPIO`
```rust
const MGSL_IOCGGPIO: u32 = 2_148_560_145u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1368`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1368)*

### `EVIOCSREP`
```rust
const EVIOCSREP: u32 = 1_074_283_779u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1369`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1369)*

### `VFIO_DEVICE_GET_IRQ_INFO`
```rust
const VFIO_DEVICE_GET_IRQ_INFO: u32 = 15_213u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1370`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1370)*

### `HPET_DPI`
```rust
const HPET_DPI: u32 = 26_629u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1371`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1371)*

### `VDUSE_VQ_SETUP_KICKFD`
```rust
const VDUSE_VQ_SETUP_KICKFD: u32 = 1_074_299_158u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1372`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1372)*

### `ND_IOCTL_CALL`
```rust
const ND_IOCTL_CALL: u32 = 3_225_439_754u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1373`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1373)*

### `HIDIOCGDEVINFO`
```rust
const HIDIOCGDEVINFO: u32 = 2_149_337_091u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1374`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1374)*

### `DM_TABLE_DEPS`
```rust
const DM_TABLE_DEPS: u32 = 3_241_737_483u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1375`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1375)*

### `BTRFS_IOC_DEV_INFO`
```rust
const BTRFS_IOC_DEV_INFO: u32 = 3_489_698_846u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1376`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1376)*

### `VDUSE_IOTLB_GET_FD`
```rust
const VDUSE_IOTLB_GET_FD: u32 = 3_223_355_664u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1377`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1377)*

### `FW_CDEV_IOC_GET_INFO`
```rust
const FW_CDEV_IOC_GET_INFO: u32 = 3_223_855_872u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1378`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1378)*

### `VIDIOC_G_PRIORITY`
```rust
const VIDIOC_G_PRIORITY: u32 = 2_147_767_875u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1379`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1379)*

### `ATM_NEWBACKENDIF`
```rust
const ATM_NEWBACKENDIF: u32 = 1_073_897_971u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1380`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1380)*

### `VIDIOC_S_EXT_CTRLS`
```rust
const VIDIOC_S_EXT_CTRLS: u32 = 3_223_344_712u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1381`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1381)*

### `VIDIOC_SUBDEV_ENUM_DV_TIMINGS`
```rust
const VIDIOC_SUBDEV_ENUM_DV_TIMINGS: u32 = 3_230_946_914u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1382`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1382)*

### `VIDIOC_OMAP3ISP_CCDC_CFG`
```rust
const VIDIOC_OMAP3ISP_CCDC_CFG: u32 = 3_224_917_697u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1383`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1383)*

### `VIDIOC_S_HW_FREQ_SEEK`
```rust
const VIDIOC_S_HW_FREQ_SEEK: u32 = 1_076_909_650u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1384`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1384)*

### `DM_TABLE_LOAD`
```rust
const DM_TABLE_LOAD: u32 = 3_241_737_481u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1385`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1385)*

### `F2FS_IOC_START_ATOMIC_WRITE`
```rust
const F2FS_IOC_START_ATOMIC_WRITE: u32 = 62_721u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1386`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1386)*

### `VIDIOC_G_OUTPUT`
```rust
const VIDIOC_G_OUTPUT: u32 = 2_147_767_854u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1387`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1387)*

### `ATM_DROPPARTY`
```rust
const ATM_DROPPARTY: u32 = 1_074_029_045u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1388`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1388)*

### `CHIOGELEM`
```rust
const CHIOGELEM: u32 = 1_080_845_072u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1389`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1389)*

### `BTRFS_IOC_GET_SUPPORTED_FEATURES`
```rust
const BTRFS_IOC_GET_SUPPORTED_FEATURES: u32 = 2_152_240_185u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1390`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1390)*

### `EVIOCSKEYCODE`
```rust
const EVIOCSKEYCODE: u32 = 1_074_283_780u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1391`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1391)*

### `NE_GET_IMAGE_LOAD_INFO`
```rust
const NE_GET_IMAGE_LOAD_INFO: u32 = 3_222_318_626u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1392`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1392)*

### `TUNSETLINK`
```rust
const TUNSETLINK: u32 = 1_074_025_677u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1393`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1393)*

### `FW_CDEV_IOC_ADD_DESCRIPTOR`
```rust
const FW_CDEV_IOC_ADD_DESCRIPTOR: u32 = 3_222_807_302u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1394`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1394)*

### `BTRFS_IOC_SCRUB_CANCEL`
```rust
const BTRFS_IOC_SCRUB_CANCEL: u32 = 37_916u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1395`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1395)*

### `PPS_SETPARAMS`
```rust
const PPS_SETPARAMS: u32 = 1_074_294_946u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1396`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1396)*

### `IOC_OPAL_LR_SETUP`
```rust
const IOC_OPAL_LR_SETUP: u32 = 1_093_169_379u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1397`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1397)*

### `FW_CDEV_IOC_DEALLOCATE`
```rust
const FW_CDEV_IOC_DEALLOCATE: u32 = 1_074_012_931u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1398`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1398)*

### `WDIOC_SETTIMEOUT`
```rust
const WDIOC_SETTIMEOUT: u32 = 3_221_509_894u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1399`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1399)*

### `IOC_WATCH_QUEUE_SET_FILTER`
```rust
const IOC_WATCH_QUEUE_SET_FILTER: u32 = 22_369u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1400`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1400)*

### `CAPI_GET_MANUFACTURER`
```rust
const CAPI_GET_MANUFACTURER: u32 = 3_221_504_774u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1401`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1401)*

### `VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY`
```rust
const VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY: u32 = 15_222u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1402`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1402)*

### `ASPEED_P2A_CTRL_IOCTL_SET_WINDOW`
```rust
const ASPEED_P2A_CTRL_IOCTL_SET_WINDOW: u32 = 1_074_836_224u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1403`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1403)*

### `VIDIOC_G_EDID`
```rust
const VIDIOC_G_EDID: u32 = 3_223_868_968u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1404`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1404)*

### `F2FS_IOC_GARBAGE_COLLECT_RANGE`
```rust
const F2FS_IOC_GARBAGE_COLLECT_RANGE: u32 = 1_075_377_419u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1405`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1405)*

### `RIO_MAP_INBOUND`
```rust
const RIO_MAP_INBOUND: u32 = 3_223_874_833u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1406`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1406)*

### `IOC_OPAL_TAKE_OWNERSHIP`
```rust
const IOC_OPAL_TAKE_OWNERSHIP: u32 = 1_091_072_222u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1407`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1407)*

### `USBDEVFS_CLAIM_PORT`
```rust
const USBDEVFS_CLAIM_PORT: u32 = 2_147_767_576u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1408`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1408)*

### `VIDIOC_S_AUDIO`
```rust
const VIDIOC_S_AUDIO: u32 = 1_077_171_746u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1409`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1409)*

### `FS_IOC_GET_ENCRYPTION_NONCE`
```rust
const FS_IOC_GET_ENCRYPTION_NONCE: u32 = 2_148_558_363u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1410`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1410)*

### `FW_CDEV_IOC_SEND_STREAM_PACKET`
```rust
const FW_CDEV_IOC_SEND_STREAM_PACKET: u32 = 1_076_372_243u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1411`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1411)*

### `BTRFS_IOC_SNAP_DESTROY`
```rust
const BTRFS_IOC_SNAP_DESTROY: u32 = 1_342_215_183u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1412`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1412)*

### `SNAPSHOT_FREE`
```rust
const SNAPSHOT_FREE: u32 = 13_061u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1413`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1413)*

### `I8K_GET_SPEED`
```rust
const I8K_GET_SPEED: u32 = 3_221_776_773u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1414`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1414)*

### `HIDIOCGREPORT`
```rust
const HIDIOCGREPORT: u32 = 1_074_546_695u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1415`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1415)*

### `HPET_EPI`
```rust
const HPET_EPI: u32 = 26_628u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1416`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1416)*

### `JSIOCSCORR`
```rust
const JSIOCSCORR: u32 = 1_076_128_289u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1417`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1417)*

### `IOC_PR_PREEMPT_ABORT`
```rust
const IOC_PR_PREEMPT_ABORT: u32 = 1_075_343_564u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1418`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1418)*

### `RIO_MAP_OUTBOUND`
```rust
const RIO_MAP_OUTBOUND: u32 = 3_223_874_831u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1419`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1419)*

### `ATM_SETESI`
```rust
const ATM_SETESI: u32 = 1_074_815_372u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1420`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1420)*

### `FW_CDEV_IOC_START_ISO`
```rust
const FW_CDEV_IOC_START_ISO: u32 = 1_074_799_370u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1421`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1421)*

### `ATM_DELADDR`
```rust
const ATM_DELADDR: u32 = 1_074_815_369u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1422`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1422)*

### `PPFCONTROL`
```rust
const PPFCONTROL: u32 = 1_073_901_710u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1423`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1423)*

### `SONYPI_IOCGFAN`
```rust
const SONYPI_IOCGFAN: u32 = 2_147_579_402u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1424`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1424)*

### `RTC_IRQP_SET`
```rust
const RTC_IRQP_SET: u32 = 1_074_294_796u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1425`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1425)*

### `PCITEST_WRITE`
```rust
const PCITEST_WRITE: u32 = 1_074_286_596u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1426`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1426)*

### `PPCLAIM`
```rust
const PPCLAIM: u32 = 28_811u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1427`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1427)*

### `VIDIOC_S_JPEGCOMP`
```rust
const VIDIOC_S_JPEGCOMP: u32 = 1_082_938_942u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1428`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1428)*

### `IPMICTL_UNREGISTER_FOR_CMD`
```rust
const IPMICTL_UNREGISTER_FOR_CMD: u32 = 2_147_641_615u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1429`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1429)*

### `VHOST_SET_FEATURES`
```rust
const VHOST_SET_FEATURES: u32 = 1_074_310_912u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1430`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1430)*

### `TOSHIBA_ACPI_SCI`
```rust
const TOSHIBA_ACPI_SCI: u32 = 3_222_828_177u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1431`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1431)*

### `VIDIOC_DQBUF`
```rust
const VIDIOC_DQBUF: u32 = 3_227_014_673u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1432`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1432)*

### `BTRFS_IOC_BALANCE_PROGRESS`
```rust
const BTRFS_IOC_BALANCE_PROGRESS: u32 = 2_214_630_434u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1433`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1433)*

### `BTRFS_IOC_SUBVOL_SETFLAGS`
```rust
const BTRFS_IOC_SUBVOL_SETFLAGS: u32 = 1_074_304_026u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1434`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1434)*

### `ATMLEC_MCAST`
```rust
const ATMLEC_MCAST: u32 = 25_042u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1435`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1435)*

### `MMTIMER_GETFREQ`
```rust
const MMTIMER_GETFREQ: u32 = 2_148_035_842u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1436`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1436)*

### `VIDIOC_G_SELECTION`
```rust
const VIDIOC_G_SELECTION: u32 = 3_225_441_886u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1437`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1437)*

### `RTC_ALM_SET`
```rust
const RTC_ALM_SET: u32 = 1_076_129_799u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1438`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1438)*

### `PPPOEIOCSFWD`
```rust
const PPPOEIOCSFWD: u32 = 1_074_311_424u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1439`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1439)*

### `IPMICTL_GET_MAINTENANCE_MODE_CMD`
```rust
const IPMICTL_GET_MAINTENANCE_MODE_CMD: u32 = 2_147_772_702u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1440`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1440)*

### `FS_IOC_ENABLE_VERITY`
```rust
const FS_IOC_ENABLE_VERITY: u32 = 1_082_156_677u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1441`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1441)*

### `NILFS_IOCTL_GET_BDESCS`
```rust
const NILFS_IOCTL_GET_BDESCS: u32 = 3_222_826_631u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1442`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1442)*

### `FDFMTEND`
```rust
const FDFMTEND: u32 = 585u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1443`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1443)*

### `DMA_BUF_SET_NAME`
```rust
const DMA_BUF_SET_NAME: u32 = 1_074_291_201u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1444`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1444)*

### `UI_BEGIN_FF_UPLOAD`
```rust
const UI_BEGIN_FF_UPLOAD: u32 = 3_228_063_176u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1445`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1445)*

### `RTC_UIE_ON`
```rust
const RTC_UIE_ON: u32 = 28_675u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1446`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1446)*

### `PPRELEASE`
```rust
const PPRELEASE: u32 = 28_812u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1447`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1447)*

### `VFIO_IOMMU_UNMAP_DMA`
```rust
const VFIO_IOMMU_UNMAP_DMA: u32 = 15_218u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1448`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1448)*

### `VIDIOC_OMAP3ISP_PRV_CFG`
```rust
const VIDIOC_OMAP3ISP_PRV_CFG: u32 = 3_228_587_714u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1449`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1449)*

### `GPIO_GET_LINEHANDLE_IOCTL`
```rust
const GPIO_GET_LINEHANDLE_IOCTL: u32 = 3_245_126_659u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1450`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1450)*

### `VFAT_IOCTL_READDIR_BOTH`
```rust
const VFAT_IOCTL_READDIR_BOTH: u32 = 2_184_212_993u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1451`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1451)*

### `NVME_IOCTL_ADMIN_CMD`
```rust
const NVME_IOCTL_ADMIN_CMD: u32 = 3_225_964_097u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1452`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1452)*

### `VHOST_SET_VRING_KICK`
```rust
const VHOST_SET_VRING_KICK: u32 = 1_074_310_944u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1453`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1453)*

### `BTRFS_IOC_SUBVOL_CREATE_V2`
```rust
const BTRFS_IOC_SUBVOL_CREATE_V2: u32 = 1_342_215_192u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1454`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1454)*

### `BTRFS_IOC_SNAP_CREATE`
```rust
const BTRFS_IOC_SNAP_CREATE: u32 = 1_342_215_169u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1455`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1455)*

### `SONYPI_IOCGBAT2CAP`
```rust
const SONYPI_IOCGBAT2CAP: u32 = 2_147_644_932u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1456`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1456)*

### `PPNEGOT`
```rust
const PPNEGOT: u32 = 1_074_032_785u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1457`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1457)*

### `NBD_PRINT_DEBUG`
```rust
const NBD_PRINT_DEBUG: u32 = 43_782u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1458`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1458)*

### `BTRFS_IOC_INO_LOOKUP_USER`
```rust
const BTRFS_IOC_INO_LOOKUP_USER: u32 = 3_489_698_878u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1459`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1459)*

### `BTRFS_IOC_GET_SUBVOL_ROOTREF`
```rust
const BTRFS_IOC_GET_SUBVOL_ROOTREF: u32 = 3_489_698_877u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1460`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1460)*

### `FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS`
```rust
const FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS: u32 = 3_225_445_913u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1461`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1461)*

### `BTRFS_IOC_FS_INFO`
```rust
const BTRFS_IOC_FS_INFO: u32 = 2_214_630_431u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1462`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1462)*

### `VIDIOC_ENUM_FMT`
```rust
const VIDIOC_ENUM_FMT: u32 = 3_225_441_794u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1463`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1463)*

### `VIDIOC_G_INPUT`
```rust
const VIDIOC_G_INPUT: u32 = 2_147_767_846u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1464`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1464)*

### `VTPM_PROXY_IOC_NEW_DEV`
```rust
const VTPM_PROXY_IOC_NEW_DEV: u32 = 3_222_577_408u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1465`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1465)*

### `DFL_FPGA_FME_ERR_GET_IRQ_NUM`
```rust
const DFL_FPGA_FME_ERR_GET_IRQ_NUM: u32 = 2_147_792_515u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1466`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1466)*

### `ND_IOCTL_DIMM_FLAGS`
```rust
const ND_IOCTL_DIMM_FLAGS: u32 = 3_221_769_731u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1467`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1467)*

### `BTRFS_IOC_QUOTA_RESCAN`
```rust
const BTRFS_IOC_QUOTA_RESCAN: u32 = 1_077_974_060u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1468`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1468)*

### `MMTIMER_GETCOUNTER`
```rust
const MMTIMER_GETCOUNTER: u32 = 2_148_035_849u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1469`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1469)*

### `MATROXFB_GET_OUTPUT_MODE`
```rust
const MATROXFB_GET_OUTPUT_MODE: u32 = 3_221_778_170u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1470`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1470)*

### `BTRFS_IOC_QUOTA_RESCAN_WAIT`
```rust
const BTRFS_IOC_QUOTA_RESCAN_WAIT: u32 = 37_934u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1471`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1471)*

### `RIO_CM_CHAN_BIND`
```rust
const RIO_CM_CHAN_BIND: u32 = 1_074_291_461u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1472`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1472)*

### `HIDIOCGRDESC`
```rust
const HIDIOCGRDESC: u32 = 2_416_199_682u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1473`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1473)*

### `MGSL_IOCGIF`
```rust
const MGSL_IOCGIF: u32 = 27_915u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1474`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1474)*

### `VIDIOC_S_OUTPUT`
```rust
const VIDIOC_S_OUTPUT: u32 = 3_221_509_679u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1475`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1475)*

### `HIDIOCGREPORTINFO`
```rust
const HIDIOCGREPORTINFO: u32 = 3_222_030_345u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1476`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1476)*

### `WDIOC_GETBOOTSTATUS`
```rust
const WDIOC_GETBOOTSTATUS: u32 = 2_147_768_066u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1477`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1477)*

### `VDUSE_VQ_GET_INFO`
```rust
const VDUSE_VQ_GET_INFO: u32 = 3_224_404_245u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1478`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1478)*

### `ACRN_IOCTL_ASSIGN_PCIDEV`
```rust
const ACRN_IOCTL_ASSIGN_PCIDEV: u32 = 1_076_142_677u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1479`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1479)*

### `BLKGETDISKSEQ`
```rust
const BLKGETDISKSEQ: u32 = 2_148_012_672u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1480`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1480)*

### `ACRN_IOCTL_PM_GET_CPU_STATE`
```rust
const ACRN_IOCTL_PM_GET_CPU_STATE: u32 = 3_221_791_328u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1481`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1481)*

### `ACRN_IOCTL_DESTROY_VM`
```rust
const ACRN_IOCTL_DESTROY_VM: u32 = 41_489u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1482`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1482)*

### `ACRN_IOCTL_SET_PTDEV_INTR`
```rust
const ACRN_IOCTL_SET_PTDEV_INTR: u32 = 1_075_094_099u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1483`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1483)*

### `ACRN_IOCTL_CREATE_IOREQ_CLIENT`
```rust
const ACRN_IOCTL_CREATE_IOREQ_CLIENT: u32 = 41_522u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1484`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1484)*

### `ACRN_IOCTL_IRQFD`
```rust
const ACRN_IOCTL_IRQFD: u32 = 1_075_356_273u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1485`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1485)*

### `ACRN_IOCTL_CREATE_VM`
```rust
const ACRN_IOCTL_CREATE_VM: u32 = 3_224_412_688u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1486`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1486)*

### `ACRN_IOCTL_INJECT_MSI`
```rust
const ACRN_IOCTL_INJECT_MSI: u32 = 1_074_831_907u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1487`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1487)*

### `ACRN_IOCTL_ATTACH_IOREQ_CLIENT`
```rust
const ACRN_IOCTL_ATTACH_IOREQ_CLIENT: u32 = 41_523u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1488`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1488)*

### `ACRN_IOCTL_RESET_PTDEV_INTR`
```rust
const ACRN_IOCTL_RESET_PTDEV_INTR: u32 = 1_075_094_100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1489`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1489)*

### `ACRN_IOCTL_NOTIFY_REQUEST_FINISH`
```rust
const ACRN_IOCTL_NOTIFY_REQUEST_FINISH: u32 = 1_074_307_633u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1490`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1490)*

### `ACRN_IOCTL_SET_IRQLINE`
```rust
const ACRN_IOCTL_SET_IRQLINE: u32 = 1_074_307_621u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1491`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1491)*

### `ACRN_IOCTL_START_VM`
```rust
const ACRN_IOCTL_START_VM: u32 = 41_490u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1492`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1492)*

### `ACRN_IOCTL_SET_VCPU_REGS`
```rust
const ACRN_IOCTL_SET_VCPU_REGS: u32 = 1_093_181_974u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1493`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1493)*

### `ACRN_IOCTL_SET_MEMSEG`
```rust
const ACRN_IOCTL_SET_MEMSEG: u32 = 1_075_880_513u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1494`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1494)*

### `ACRN_IOCTL_PAUSE_VM`
```rust
const ACRN_IOCTL_PAUSE_VM: u32 = 41_491u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1495`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1495)*

### `ACRN_IOCTL_CLEAR_VM_IOREQ`
```rust
const ACRN_IOCTL_CLEAR_VM_IOREQ: u32 = 41_525u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1496`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1496)*

### `ACRN_IOCTL_UNSET_MEMSEG`
```rust
const ACRN_IOCTL_UNSET_MEMSEG: u32 = 1_075_880_514u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1497`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1497)*

### `ACRN_IOCTL_IOEVENTFD`
```rust
const ACRN_IOCTL_IOEVENTFD: u32 = 1_075_880_560u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1498`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1498)*

### `ACRN_IOCTL_DEASSIGN_PCIDEV`
```rust
const ACRN_IOCTL_DEASSIGN_PCIDEV: u32 = 1_076_142_678u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1499`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1499)*

### `ACRN_IOCTL_RESET_VM`
```rust
const ACRN_IOCTL_RESET_VM: u32 = 41_493u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1500`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1500)*

### `ACRN_IOCTL_DESTROY_IOREQ_CLIENT`
```rust
const ACRN_IOCTL_DESTROY_IOREQ_CLIENT: u32 = 41_524u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1501`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1501)*

### `ACRN_IOCTL_VM_INTR_MONITOR`
```rust
const ACRN_IOCTL_VM_INTR_MONITOR: u32 = 1_074_307_620u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/ioctl.rs:1502`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/ioctl.rs#L1502)*

