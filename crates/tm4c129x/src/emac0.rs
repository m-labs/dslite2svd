#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Ethernet MAC Frame Filter"]
    pub framefltr: FRAMEFLTR,
    #[doc = "0x08 - Ethernet MAC Hash Table High"]
    pub hashtblh: HASHTBLH,
    #[doc = "0x0c - Ethernet MAC Hash Table Low"]
    pub hashtbll: HASHTBLL,
    #[doc = "0x10 - Ethernet MAC MII Address"]
    pub miiaddr: MIIADDR,
    #[doc = "0x14 - Ethernet MAC MII Data Register"]
    pub miidata: MIIDATA,
    #[doc = "0x18 - Ethernet MAC Flow Control"]
    pub flowctl: FLOWCTL,
    #[doc = "0x1c - Ethernet MAC VLAN Tag"]
    pub vlantg: VLANTG,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - Ethernet MAC Status"]
    pub status: STATUS,
    #[doc = "0x28 - Ethernet MAC Remote Wake-Up Frame Filter"]
    pub rwuff: RWUFF,
    #[doc = "0x2c - Ethernet MAC PMT Control and Status Register"]
    pub pmtctlstat: PMTCTLSTAT,
    _reserved11: [u8; 8usize],
    #[doc = "0x38 - Ethernet MAC Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x3c - Ethernet MAC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x40 - Ethernet MAC Address 0 High"]
    pub addr0h: ADDR0H,
    #[doc = "0x44 - Ethernet MAC Address 0 Low Register"]
    pub addr0l: ADDR0L,
    #[doc = "0x48 - Ethernet MAC Address 1 High"]
    pub addr1h: ADDR1H,
    #[doc = "0x4c - Ethernet MAC Address 1 Low"]
    pub addr1l: ADDR1L,
    #[doc = "0x50 - Ethernet MAC Address 2 High"]
    pub addr2h: ADDR2H,
    #[doc = "0x54 - Ethernet MAC Address 2 Low"]
    pub addr2l: ADDR2L,
    #[doc = "0x58 - Ethernet MAC Address 3 High"]
    pub addr3h: ADDR3H,
    #[doc = "0x5c - Ethernet MAC Address 3 Low"]
    pub addr3l: ADDR3L,
    _reserved21: [u8; 124usize],
    #[doc = "0xdc - Ethernet MAC Watchdog Timeout"]
    pub wdogto: WDOGTO,
    _reserved22: [u8; 32usize],
    #[doc = "0x100 - Ethernet MAC MMC Control"]
    pub mmcctrl: MMCCTRL,
    #[doc = "0x104 - Ethernet MAC MMC Receive Raw Interrupt Status"]
    pub mmcrxris: MMCRXRIS,
    #[doc = "0x108 - Ethernet MAC MMC Transmit Raw Interrupt Status"]
    pub mmctxris: MMCTXRIS,
    #[doc = "0x10c - Ethernet MAC MMC Receive Interrupt Mask"]
    pub mmcrxim: MMCRXIM,
    #[doc = "0x110 - Ethernet MAC MMC Transmit Interrupt Mask"]
    pub mmctxim: MMCTXIM,
    _reserved27: [u8; 4usize],
    #[doc = "0x118 - Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
    pub txcntgb: TXCNTGB,
    _reserved28: [u8; 48usize],
    #[doc = "0x14c - Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
    pub txcntscol: TXCNTSCOL,
    #[doc = "0x150 - Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
    pub txcntmcol: TXCNTMCOL,
    _reserved30: [u8; 16usize],
    #[doc = "0x164 - Ethernet MAC Transmit Octet Count Good"]
    pub txoctcntg: TXOCTCNTG,
    _reserved31: [u8; 24usize],
    #[doc = "0x180 - Ethernet MAC Receive Frame Count for Good and Bad Frames"]
    pub rxcntgb: RXCNTGB,
    _reserved32: [u8; 16usize],
    #[doc = "0x194 - Ethernet MAC Receive Frame Count for CRC Error Frames"]
    pub rxcntcrcerr: RXCNTCRCERR,
    #[doc = "0x198 - Ethernet MAC Receive Frame Count for Alignment Error Frames"]
    pub rxcntalgnerr: RXCNTALGNERR,
    _reserved34: [u8; 40usize],
    #[doc = "0x1c4 - Ethernet MAC Receive Frame Count for Good Unicast Frames"]
    pub rxcntguni: RXCNTGUNI,
    _reserved35: [u8; 956usize],
    #[doc = "0x584 - Ethernet MAC VLAN Tag Inclusion or Replacement"]
    pub vlnincrep: VLNINCREP,
    #[doc = "0x588 - Ethernet MAC VLAN Hash Table"]
    pub vlanhash: VLANHASH,
    _reserved37: [u8; 372usize],
    #[doc = "0x700 - Ethernet MAC Timestamp Control"]
    pub timstctrl: TIMSTCTRL,
    #[doc = "0x704 - Ethernet MAC Sub-Second Increment"]
    pub subsecinc: SUBSECINC,
    #[doc = "0x708 - Ethernet MAC System Time - Seconds"]
    pub timsec: TIMSEC,
    #[doc = "0x70c - Ethernet MAC System Time - Nanoseconds"]
    pub timnano: TIMNANO,
    #[doc = "0x710 - Ethernet MAC System Time - Seconds Update"]
    pub timsecu: TIMSECU,
    #[doc = "0x714 - Ethernet MAC System Time - Nanoseconds Update"]
    pub timnanou: TIMNANOU,
    #[doc = "0x718 - Ethernet MAC Timestamp Addend"]
    pub timadd: TIMADD,
    #[doc = "0x71c - Ethernet MAC Target Time Seconds"]
    pub targsec: TARGSEC,
    #[doc = "0x720 - Ethernet MAC Target Time Nanoseconds"]
    pub targnano: TARGNANO,
    #[doc = "0x724 - Ethernet MAC System Time-Higher Word Seconds"]
    pub hwordsec: HWORDSEC,
    #[doc = "0x728 - Ethernet MAC Timestamp Status"]
    pub timstat: TIMSTAT,
    #[doc = "0x72c - Ethernet MAC PPS Control"]
    pub ppsctrl: PPSCTRL,
    _reserved49: [u8; 48usize],
    #[doc = "0x760 - Ethernet MAC PPS0 Interval"]
    pub pps0intvl: PPS0INTVL,
    #[doc = "0x764 - Ethernet MAC PPS0 Width"]
    pub pps0width: PPS0WIDTH,
    _reserved51: [u8; 1176usize],
    #[doc = "0xc00 - Ethernet MAC DMA Bus Mode"]
    pub dmabusmod: DMABUSMOD,
    #[doc = "0xc04 - Ethernet MAC Transmit Poll Demand"]
    pub txpolld: TXPOLLD,
    #[doc = "0xc08 - Ethernet MAC Receive Poll Demand"]
    pub rxpolld: RXPOLLD,
    #[doc = "0xc0c - Ethernet MAC Receive Descriptor List Address"]
    pub rxdladdr: RXDLADDR,
    #[doc = "0xc10 - Ethernet MAC Transmit Descriptor List Address"]
    pub txdladdr: TXDLADDR,
    #[doc = "0xc14 - Ethernet MAC DMA Interrupt Status"]
    pub dmaris: DMARIS,
    #[doc = "0xc18 - Ethernet MAC DMA Operation Mode"]
    pub dmaopmode: DMAOPMODE,
    #[doc = "0xc1c - Ethernet MAC DMA Interrupt Mask Register"]
    pub dmaim: DMAIM,
    #[doc = "0xc20 - Ethernet MAC Missed Frame and Buffer Overflow Counter"]
    pub mfboc: MFBOC,
    #[doc = "0xc24 - Ethernet MAC Receive Interrupt Watchdog Timer"]
    pub rxintwdt: RXINTWDT,
    _reserved61: [u8; 32usize],
    #[doc = "0xc48 - Ethernet MAC Current Host Transmit Descriptor"]
    pub hostxdesc: HOSTXDESC,
    #[doc = "0xc4c - Ethernet MAC Current Host Receive Descriptor"]
    pub hosrxdesc: HOSRXDESC,
    #[doc = "0xc50 - Ethernet MAC Current Host Transmit Buffer Address"]
    pub hostxba: HOSTXBA,
    #[doc = "0xc54 - Ethernet MAC Current Host Receive Buffer Address"]
    pub hosrxba: HOSRXBA,
    _reserved65: [u8; 872usize],
    #[doc = "0xfc0 - Ethernet MAC Peripheral Property Register"]
    pub pp: PP,
    #[doc = "0xfc4 - Ethernet MAC Peripheral Configuration Register"]
    pub pc: PC,
    #[doc = "0xfc8 - Ethernet MAC Clock Configuration Register"]
    pub cc: CC,
    _reserved68: [u8; 4usize],
    #[doc = "0xfd0 - Ethernet PHY Raw Interrupt Status"]
    pub ephyris: EPHYRIS,
    #[doc = "0xfd4 - Ethernet PHY Interrupt Mask"]
    pub ephyim: EPHYIM,
    #[doc = "0xfd8 - Ethernet PHY Masked Interrupt Status and Clear"]
    pub ephymisc: EPHYMISC,
}
#[doc = "Ethernet MAC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Ethernet MAC Configuration"]
pub mod cfg;
#[doc = "Ethernet MAC Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framefltr](framefltr) module"]
pub type FRAMEFLTR = crate::Reg<u32, _FRAMEFLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEFLTR;
#[doc = "`read()` method returns [framefltr::R](framefltr::R) reader structure"]
impl crate::Readable for FRAMEFLTR {}
#[doc = "`write(|w| ..)` method takes [framefltr::W](framefltr::W) writer structure"]
impl crate::Writable for FRAMEFLTR {}
#[doc = "Ethernet MAC Frame Filter"]
pub mod framefltr;
#[doc = "Ethernet MAC Hash Table High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hashtblh](hashtblh) module"]
pub type HASHTBLH = crate::Reg<u32, _HASHTBLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHTBLH;
#[doc = "`read()` method returns [hashtblh::R](hashtblh::R) reader structure"]
impl crate::Readable for HASHTBLH {}
#[doc = "`write(|w| ..)` method takes [hashtblh::W](hashtblh::W) writer structure"]
impl crate::Writable for HASHTBLH {}
#[doc = "Ethernet MAC Hash Table High"]
pub mod hashtblh;
#[doc = "Ethernet MAC Hash Table Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hashtbll](hashtbll) module"]
pub type HASHTBLL = crate::Reg<u32, _HASHTBLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHTBLL;
#[doc = "`read()` method returns [hashtbll::R](hashtbll::R) reader structure"]
impl crate::Readable for HASHTBLL {}
#[doc = "`write(|w| ..)` method takes [hashtbll::W](hashtbll::W) writer structure"]
impl crate::Writable for HASHTBLL {}
#[doc = "Ethernet MAC Hash Table Low"]
pub mod hashtbll;
#[doc = "Ethernet MAC MII Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [miiaddr](miiaddr) module"]
pub type MIIADDR = crate::Reg<u32, _MIIADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIIADDR;
#[doc = "`read()` method returns [miiaddr::R](miiaddr::R) reader structure"]
impl crate::Readable for MIIADDR {}
#[doc = "`write(|w| ..)` method takes [miiaddr::W](miiaddr::W) writer structure"]
impl crate::Writable for MIIADDR {}
#[doc = "Ethernet MAC MII Address"]
pub mod miiaddr;
#[doc = "Ethernet MAC MII Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [miidata](miidata) module"]
pub type MIIDATA = crate::Reg<u32, _MIIDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIIDATA;
#[doc = "`read()` method returns [miidata::R](miidata::R) reader structure"]
impl crate::Readable for MIIDATA {}
#[doc = "`write(|w| ..)` method takes [miidata::W](miidata::W) writer structure"]
impl crate::Writable for MIIDATA {}
#[doc = "Ethernet MAC MII Data Register"]
pub mod miidata;
#[doc = "Ethernet MAC Flow Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flowctl](flowctl) module"]
pub type FLOWCTL = crate::Reg<u32, _FLOWCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOWCTL;
#[doc = "`read()` method returns [flowctl::R](flowctl::R) reader structure"]
impl crate::Readable for FLOWCTL {}
#[doc = "`write(|w| ..)` method takes [flowctl::W](flowctl::W) writer structure"]
impl crate::Writable for FLOWCTL {}
#[doc = "Ethernet MAC Flow Control"]
pub mod flowctl;
#[doc = "Ethernet MAC VLAN Tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vlantg](vlantg) module"]
pub type VLANTG = crate::Reg<u32, _VLANTG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLANTG;
#[doc = "`read()` method returns [vlantg::R](vlantg::R) reader structure"]
impl crate::Readable for VLANTG {}
#[doc = "`write(|w| ..)` method takes [vlantg::W](vlantg::W) writer structure"]
impl crate::Writable for VLANTG {}
#[doc = "Ethernet MAC VLAN Tag"]
pub mod vlantg;
#[doc = "Ethernet MAC Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Ethernet MAC Status"]
pub mod status;
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rwuff](rwuff) module"]
pub type RWUFF = crate::Reg<u32, _RWUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWUFF;
#[doc = "`read()` method returns [rwuff::R](rwuff::R) reader structure"]
impl crate::Readable for RWUFF {}
#[doc = "`write(|w| ..)` method takes [rwuff::W](rwuff::W) writer structure"]
impl crate::Writable for RWUFF {}
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter"]
pub mod rwuff;
#[doc = "Ethernet MAC PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmtctlstat](pmtctlstat) module"]
pub type PMTCTLSTAT = crate::Reg<u32, _PMTCTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMTCTLSTAT;
#[doc = "`read()` method returns [pmtctlstat::R](pmtctlstat::R) reader structure"]
impl crate::Readable for PMTCTLSTAT {}
#[doc = "`write(|w| ..)` method takes [pmtctlstat::W](pmtctlstat::W) writer structure"]
impl crate::Writable for PMTCTLSTAT {}
#[doc = "Ethernet MAC PMT Control and Status Register"]
pub mod pmtctlstat;
#[doc = "Ethernet MAC Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Ethernet MAC Raw Interrupt Status"]
pub mod ris;
#[doc = "Ethernet MAC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "Ethernet MAC Interrupt Mask"]
pub mod im;
#[doc = "Ethernet MAC Address 0 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr0h](addr0h) module"]
pub type ADDR0H = crate::Reg<u32, _ADDR0H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0H;
#[doc = "`read()` method returns [addr0h::R](addr0h::R) reader structure"]
impl crate::Readable for ADDR0H {}
#[doc = "`write(|w| ..)` method takes [addr0h::W](addr0h::W) writer structure"]
impl crate::Writable for ADDR0H {}
#[doc = "Ethernet MAC Address 0 High"]
pub mod addr0h;
#[doc = "Ethernet MAC Address 0 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr0l](addr0l) module"]
pub type ADDR0L = crate::Reg<u32, _ADDR0L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0L;
#[doc = "`read()` method returns [addr0l::R](addr0l::R) reader structure"]
impl crate::Readable for ADDR0L {}
#[doc = "`write(|w| ..)` method takes [addr0l::W](addr0l::W) writer structure"]
impl crate::Writable for ADDR0L {}
#[doc = "Ethernet MAC Address 0 Low Register"]
pub mod addr0l;
#[doc = "Ethernet MAC Address 1 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr1h](addr1h) module"]
pub type ADDR1H = crate::Reg<u32, _ADDR1H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1H;
#[doc = "`read()` method returns [addr1h::R](addr1h::R) reader structure"]
impl crate::Readable for ADDR1H {}
#[doc = "`write(|w| ..)` method takes [addr1h::W](addr1h::W) writer structure"]
impl crate::Writable for ADDR1H {}
#[doc = "Ethernet MAC Address 1 High"]
pub mod addr1h;
#[doc = "Ethernet MAC Address 1 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr1l](addr1l) module"]
pub type ADDR1L = crate::Reg<u32, _ADDR1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1L;
#[doc = "`read()` method returns [addr1l::R](addr1l::R) reader structure"]
impl crate::Readable for ADDR1L {}
#[doc = "`write(|w| ..)` method takes [addr1l::W](addr1l::W) writer structure"]
impl crate::Writable for ADDR1L {}
#[doc = "Ethernet MAC Address 1 Low"]
pub mod addr1l;
#[doc = "Ethernet MAC Address 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr2h](addr2h) module"]
pub type ADDR2H = crate::Reg<u32, _ADDR2H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR2H;
#[doc = "`read()` method returns [addr2h::R](addr2h::R) reader structure"]
impl crate::Readable for ADDR2H {}
#[doc = "`write(|w| ..)` method takes [addr2h::W](addr2h::W) writer structure"]
impl crate::Writable for ADDR2H {}
#[doc = "Ethernet MAC Address 2 High"]
pub mod addr2h;
#[doc = "Ethernet MAC Address 2 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr2l](addr2l) module"]
pub type ADDR2L = crate::Reg<u32, _ADDR2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR2L;
#[doc = "`read()` method returns [addr2l::R](addr2l::R) reader structure"]
impl crate::Readable for ADDR2L {}
#[doc = "`write(|w| ..)` method takes [addr2l::W](addr2l::W) writer structure"]
impl crate::Writable for ADDR2L {}
#[doc = "Ethernet MAC Address 2 Low"]
pub mod addr2l;
#[doc = "Ethernet MAC Address 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr3h](addr3h) module"]
pub type ADDR3H = crate::Reg<u32, _ADDR3H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR3H;
#[doc = "`read()` method returns [addr3h::R](addr3h::R) reader structure"]
impl crate::Readable for ADDR3H {}
#[doc = "`write(|w| ..)` method takes [addr3h::W](addr3h::W) writer structure"]
impl crate::Writable for ADDR3H {}
#[doc = "Ethernet MAC Address 3 High"]
pub mod addr3h;
#[doc = "Ethernet MAC Address 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr3l](addr3l) module"]
pub type ADDR3L = crate::Reg<u32, _ADDR3L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR3L;
#[doc = "`read()` method returns [addr3l::R](addr3l::R) reader structure"]
impl crate::Readable for ADDR3L {}
#[doc = "`write(|w| ..)` method takes [addr3l::W](addr3l::W) writer structure"]
impl crate::Writable for ADDR3L {}
#[doc = "Ethernet MAC Address 3 Low"]
pub mod addr3l;
#[doc = "Ethernet MAC Watchdog Timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogto](wdogto) module"]
pub type WDOGTO = crate::Reg<u32, _WDOGTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGTO;
#[doc = "`read()` method returns [wdogto::R](wdogto::R) reader structure"]
impl crate::Readable for WDOGTO {}
#[doc = "`write(|w| ..)` method takes [wdogto::W](wdogto::W) writer structure"]
impl crate::Writable for WDOGTO {}
#[doc = "Ethernet MAC Watchdog Timeout"]
pub mod wdogto;
#[doc = "Ethernet MAC MMC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmcctrl](mmcctrl) module"]
pub type MMCCTRL = crate::Reg<u32, _MMCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCCTRL;
#[doc = "`read()` method returns [mmcctrl::R](mmcctrl::R) reader structure"]
impl crate::Readable for MMCCTRL {}
#[doc = "`write(|w| ..)` method takes [mmcctrl::W](mmcctrl::W) writer structure"]
impl crate::Writable for MMCCTRL {}
#[doc = "Ethernet MAC MMC Control"]
pub mod mmcctrl;
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmcrxris](mmcrxris) module"]
pub type MMCRXRIS = crate::Reg<u32, _MMCRXRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRXRIS;
#[doc = "`read()` method returns [mmcrxris::R](mmcrxris::R) reader structure"]
impl crate::Readable for MMCRXRIS {}
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status"]
pub mod mmcrxris;
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmctxris](mmctxris) module"]
pub type MMCTXRIS = crate::Reg<u32, _MMCTXRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTXRIS;
#[doc = "`read()` method returns [mmctxris::R](mmctxris::R) reader structure"]
impl crate::Readable for MMCTXRIS {}
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status"]
pub mod mmctxris;
#[doc = "Ethernet MAC MMC Receive Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmcrxim](mmcrxim) module"]
pub type MMCRXIM = crate::Reg<u32, _MMCRXIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRXIM;
#[doc = "`read()` method returns [mmcrxim::R](mmcrxim::R) reader structure"]
impl crate::Readable for MMCRXIM {}
#[doc = "`write(|w| ..)` method takes [mmcrxim::W](mmcrxim::W) writer structure"]
impl crate::Writable for MMCRXIM {}
#[doc = "Ethernet MAC MMC Receive Interrupt Mask"]
pub mod mmcrxim;
#[doc = "Ethernet MAC MMC Transmit Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmctxim](mmctxim) module"]
pub type MMCTXIM = crate::Reg<u32, _MMCTXIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTXIM;
#[doc = "`read()` method returns [mmctxim::R](mmctxim::R) reader structure"]
impl crate::Readable for MMCTXIM {}
#[doc = "`write(|w| ..)` method takes [mmctxim::W](mmctxim::W) writer structure"]
impl crate::Writable for MMCTXIM {}
#[doc = "Ethernet MAC MMC Transmit Interrupt Mask"]
pub mod mmctxim;
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txcntgb](txcntgb) module"]
pub type TXCNTGB = crate::Reg<u32, _TXCNTGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCNTGB;
#[doc = "`read()` method returns [txcntgb::R](txcntgb::R) reader structure"]
impl crate::Readable for TXCNTGB {}
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
pub mod txcntgb;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txcntscol](txcntscol) module"]
pub type TXCNTSCOL = crate::Reg<u32, _TXCNTSCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCNTSCOL;
#[doc = "`read()` method returns [txcntscol::R](txcntscol::R) reader structure"]
impl crate::Readable for TXCNTSCOL {}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod txcntscol;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txcntmcol](txcntmcol) module"]
pub type TXCNTMCOL = crate::Reg<u32, _TXCNTMCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCNTMCOL;
#[doc = "`read()` method returns [txcntmcol::R](txcntmcol::R) reader structure"]
impl crate::Readable for TXCNTMCOL {}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
pub mod txcntmcol;
#[doc = "Ethernet MAC Transmit Octet Count Good\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txoctcntg](txoctcntg) module"]
pub type TXOCTCNTG = crate::Reg<u32, _TXOCTCNTG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXOCTCNTG;
#[doc = "`read()` method returns [txoctcntg::R](txoctcntg::R) reader structure"]
impl crate::Readable for TXOCTCNTG {}
#[doc = "Ethernet MAC Transmit Octet Count Good"]
pub mod txoctcntg;
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxcntgb](rxcntgb) module"]
pub type RXCNTGB = crate::Reg<u32, _RXCNTGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCNTGB;
#[doc = "`read()` method returns [rxcntgb::R](rxcntgb::R) reader structure"]
impl crate::Readable for RXCNTGB {}
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames"]
pub mod rxcntgb;
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxcntcrcerr](rxcntcrcerr) module"]
pub type RXCNTCRCERR = crate::Reg<u32, _RXCNTCRCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCNTCRCERR;
#[doc = "`read()` method returns [rxcntcrcerr::R](rxcntcrcerr::R) reader structure"]
impl crate::Readable for RXCNTCRCERR {}
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames"]
pub mod rxcntcrcerr;
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxcntalgnerr](rxcntalgnerr) module"]
pub type RXCNTALGNERR = crate::Reg<u32, _RXCNTALGNERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCNTALGNERR;
#[doc = "`read()` method returns [rxcntalgnerr::R](rxcntalgnerr::R) reader structure"]
impl crate::Readable for RXCNTALGNERR {}
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames"]
pub mod rxcntalgnerr;
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxcntguni](rxcntguni) module"]
pub type RXCNTGUNI = crate::Reg<u32, _RXCNTGUNI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCNTGUNI;
#[doc = "`read()` method returns [rxcntguni::R](rxcntguni::R) reader structure"]
impl crate::Readable for RXCNTGUNI {}
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames"]
pub mod rxcntguni;
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vlnincrep](vlnincrep) module"]
pub type VLNINCREP = crate::Reg<u32, _VLNINCREP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLNINCREP;
#[doc = "`read()` method returns [vlnincrep::R](vlnincrep::R) reader structure"]
impl crate::Readable for VLNINCREP {}
#[doc = "`write(|w| ..)` method takes [vlnincrep::W](vlnincrep::W) writer structure"]
impl crate::Writable for VLNINCREP {}
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement"]
pub mod vlnincrep;
#[doc = "Ethernet MAC VLAN Hash Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vlanhash](vlanhash) module"]
pub type VLANHASH = crate::Reg<u32, _VLANHASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLANHASH;
#[doc = "`read()` method returns [vlanhash::R](vlanhash::R) reader structure"]
impl crate::Readable for VLANHASH {}
#[doc = "`write(|w| ..)` method takes [vlanhash::W](vlanhash::W) writer structure"]
impl crate::Writable for VLANHASH {}
#[doc = "Ethernet MAC VLAN Hash Table"]
pub mod vlanhash;
#[doc = "Ethernet MAC Timestamp Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timstctrl](timstctrl) module"]
pub type TIMSTCTRL = crate::Reg<u32, _TIMSTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSTCTRL;
#[doc = "`read()` method returns [timstctrl::R](timstctrl::R) reader structure"]
impl crate::Readable for TIMSTCTRL {}
#[doc = "`write(|w| ..)` method takes [timstctrl::W](timstctrl::W) writer structure"]
impl crate::Writable for TIMSTCTRL {}
#[doc = "Ethernet MAC Timestamp Control"]
pub mod timstctrl;
#[doc = "Ethernet MAC Sub-Second Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subsecinc](subsecinc) module"]
pub type SUBSECINC = crate::Reg<u32, _SUBSECINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSECINC;
#[doc = "`read()` method returns [subsecinc::R](subsecinc::R) reader structure"]
impl crate::Readable for SUBSECINC {}
#[doc = "`write(|w| ..)` method takes [subsecinc::W](subsecinc::W) writer structure"]
impl crate::Writable for SUBSECINC {}
#[doc = "Ethernet MAC Sub-Second Increment"]
pub mod subsecinc;
#[doc = "Ethernet MAC System Time - Seconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timsec](timsec) module"]
pub type TIMSEC = crate::Reg<u32, _TIMSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSEC;
#[doc = "`read()` method returns [timsec::R](timsec::R) reader structure"]
impl crate::Readable for TIMSEC {}
#[doc = "Ethernet MAC System Time - Seconds"]
pub mod timsec;
#[doc = "Ethernet MAC System Time - Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timnano](timnano) module"]
pub type TIMNANO = crate::Reg<u32, _TIMNANO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMNANO;
#[doc = "`read()` method returns [timnano::R](timnano::R) reader structure"]
impl crate::Readable for TIMNANO {}
#[doc = "Ethernet MAC System Time - Nanoseconds"]
pub mod timnano;
#[doc = "Ethernet MAC System Time - Seconds Update\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timsecu](timsecu) module"]
pub type TIMSECU = crate::Reg<u32, _TIMSECU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSECU;
#[doc = "`read()` method returns [timsecu::R](timsecu::R) reader structure"]
impl crate::Readable for TIMSECU {}
#[doc = "`write(|w| ..)` method takes [timsecu::W](timsecu::W) writer structure"]
impl crate::Writable for TIMSECU {}
#[doc = "Ethernet MAC System Time - Seconds Update"]
pub mod timsecu;
#[doc = "Ethernet MAC System Time - Nanoseconds Update\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timnanou](timnanou) module"]
pub type TIMNANOU = crate::Reg<u32, _TIMNANOU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMNANOU;
#[doc = "`read()` method returns [timnanou::R](timnanou::R) reader structure"]
impl crate::Readable for TIMNANOU {}
#[doc = "`write(|w| ..)` method takes [timnanou::W](timnanou::W) writer structure"]
impl crate::Writable for TIMNANOU {}
#[doc = "Ethernet MAC System Time - Nanoseconds Update"]
pub mod timnanou;
#[doc = "Ethernet MAC Timestamp Addend\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timadd](timadd) module"]
pub type TIMADD = crate::Reg<u32, _TIMADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMADD;
#[doc = "`read()` method returns [timadd::R](timadd::R) reader structure"]
impl crate::Readable for TIMADD {}
#[doc = "`write(|w| ..)` method takes [timadd::W](timadd::W) writer structure"]
impl crate::Writable for TIMADD {}
#[doc = "Ethernet MAC Timestamp Addend"]
pub mod timadd;
#[doc = "Ethernet MAC Target Time Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [targsec](targsec) module"]
pub type TARGSEC = crate::Reg<u32, _TARGSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARGSEC;
#[doc = "`read()` method returns [targsec::R](targsec::R) reader structure"]
impl crate::Readable for TARGSEC {}
#[doc = "`write(|w| ..)` method takes [targsec::W](targsec::W) writer structure"]
impl crate::Writable for TARGSEC {}
#[doc = "Ethernet MAC Target Time Seconds"]
pub mod targsec;
#[doc = "Ethernet MAC Target Time Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [targnano](targnano) module"]
pub type TARGNANO = crate::Reg<u32, _TARGNANO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARGNANO;
#[doc = "`read()` method returns [targnano::R](targnano::R) reader structure"]
impl crate::Readable for TARGNANO {}
#[doc = "`write(|w| ..)` method takes [targnano::W](targnano::W) writer structure"]
impl crate::Writable for TARGNANO {}
#[doc = "Ethernet MAC Target Time Nanoseconds"]
pub mod targnano;
#[doc = "Ethernet MAC System Time-Higher Word Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwordsec](hwordsec) module"]
pub type HWORDSEC = crate::Reg<u32, _HWORDSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWORDSEC;
#[doc = "`read()` method returns [hwordsec::R](hwordsec::R) reader structure"]
impl crate::Readable for HWORDSEC {}
#[doc = "`write(|w| ..)` method takes [hwordsec::W](hwordsec::W) writer structure"]
impl crate::Writable for HWORDSEC {}
#[doc = "Ethernet MAC System Time-Higher Word Seconds"]
pub mod hwordsec;
#[doc = "Ethernet MAC Timestamp Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timstat](timstat) module"]
pub type TIMSTAT = crate::Reg<u32, _TIMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSTAT;
#[doc = "`read()` method returns [timstat::R](timstat::R) reader structure"]
impl crate::Readable for TIMSTAT {}
#[doc = "Ethernet MAC Timestamp Status"]
pub mod timstat;
#[doc = "Ethernet MAC PPS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppsctrl](ppsctrl) module"]
pub type PPSCTRL = crate::Reg<u32, _PPSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSCTRL;
#[doc = "`read()` method returns [ppsctrl::R](ppsctrl::R) reader structure"]
impl crate::Readable for PPSCTRL {}
#[doc = "`write(|w| ..)` method takes [ppsctrl::W](ppsctrl::W) writer structure"]
impl crate::Writable for PPSCTRL {}
#[doc = "Ethernet MAC PPS Control"]
pub mod ppsctrl;
#[doc = "Ethernet MAC PPS0 Interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pps0intvl](pps0intvl) module"]
pub type PPS0INTVL = crate::Reg<u32, _PPS0INTVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPS0INTVL;
#[doc = "`read()` method returns [pps0intvl::R](pps0intvl::R) reader structure"]
impl crate::Readable for PPS0INTVL {}
#[doc = "`write(|w| ..)` method takes [pps0intvl::W](pps0intvl::W) writer structure"]
impl crate::Writable for PPS0INTVL {}
#[doc = "Ethernet MAC PPS0 Interval"]
pub mod pps0intvl;
#[doc = "Ethernet MAC PPS0 Width\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pps0width](pps0width) module"]
pub type PPS0WIDTH = crate::Reg<u32, _PPS0WIDTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPS0WIDTH;
#[doc = "`read()` method returns [pps0width::R](pps0width::R) reader structure"]
impl crate::Readable for PPS0WIDTH {}
#[doc = "`write(|w| ..)` method takes [pps0width::W](pps0width::W) writer structure"]
impl crate::Writable for PPS0WIDTH {}
#[doc = "Ethernet MAC PPS0 Width"]
pub mod pps0width;
#[doc = "Ethernet MAC DMA Bus Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmabusmod](dmabusmod) module"]
pub type DMABUSMOD = crate::Reg<u32, _DMABUSMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABUSMOD;
#[doc = "`read()` method returns [dmabusmod::R](dmabusmod::R) reader structure"]
impl crate::Readable for DMABUSMOD {}
#[doc = "`write(|w| ..)` method takes [dmabusmod::W](dmabusmod::W) writer structure"]
impl crate::Writable for DMABUSMOD {}
#[doc = "Ethernet MAC DMA Bus Mode"]
pub mod dmabusmod;
#[doc = "Ethernet MAC Transmit Poll Demand\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpolld](txpolld) module"]
pub type TXPOLLD = crate::Reg<u32, _TXPOLLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPOLLD;
#[doc = "`write(|w| ..)` method takes [txpolld::W](txpolld::W) writer structure"]
impl crate::Writable for TXPOLLD {}
#[doc = "Ethernet MAC Transmit Poll Demand"]
pub mod txpolld;
#[doc = "Ethernet MAC Receive Poll Demand\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxpolld](rxpolld) module"]
pub type RXPOLLD = crate::Reg<u32, _RXPOLLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPOLLD;
#[doc = "`write(|w| ..)` method takes [rxpolld::W](rxpolld::W) writer structure"]
impl crate::Writable for RXPOLLD {}
#[doc = "Ethernet MAC Receive Poll Demand"]
pub mod rxpolld;
#[doc = "Ethernet MAC Receive Descriptor List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdladdr](rxdladdr) module"]
pub type RXDLADDR = crate::Reg<u32, _RXDLADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDLADDR;
#[doc = "`read()` method returns [rxdladdr::R](rxdladdr::R) reader structure"]
impl crate::Readable for RXDLADDR {}
#[doc = "`write(|w| ..)` method takes [rxdladdr::W](rxdladdr::W) writer structure"]
impl crate::Writable for RXDLADDR {}
#[doc = "Ethernet MAC Receive Descriptor List Address"]
pub mod rxdladdr;
#[doc = "Ethernet MAC Transmit Descriptor List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdladdr](txdladdr) module"]
pub type TXDLADDR = crate::Reg<u32, _TXDLADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDLADDR;
#[doc = "`read()` method returns [txdladdr::R](txdladdr::R) reader structure"]
impl crate::Readable for TXDLADDR {}
#[doc = "`write(|w| ..)` method takes [txdladdr::W](txdladdr::W) writer structure"]
impl crate::Writable for TXDLADDR {}
#[doc = "Ethernet MAC Transmit Descriptor List Address"]
pub mod txdladdr;
#[doc = "Ethernet MAC DMA Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaris](dmaris) module"]
pub type DMARIS = crate::Reg<u32, _DMARIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARIS;
#[doc = "`read()` method returns [dmaris::R](dmaris::R) reader structure"]
impl crate::Readable for DMARIS {}
#[doc = "`write(|w| ..)` method takes [dmaris::W](dmaris::W) writer structure"]
impl crate::Writable for DMARIS {}
#[doc = "Ethernet MAC DMA Interrupt Status"]
pub mod dmaris;
#[doc = "Ethernet MAC DMA Operation Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaopmode](dmaopmode) module"]
pub type DMAOPMODE = crate::Reg<u32, _DMAOPMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAOPMODE;
#[doc = "`read()` method returns [dmaopmode::R](dmaopmode::R) reader structure"]
impl crate::Readable for DMAOPMODE {}
#[doc = "`write(|w| ..)` method takes [dmaopmode::W](dmaopmode::W) writer structure"]
impl crate::Writable for DMAOPMODE {}
#[doc = "Ethernet MAC DMA Operation Mode"]
pub mod dmaopmode;
#[doc = "Ethernet MAC DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaim](dmaim) module"]
pub type DMAIM = crate::Reg<u32, _DMAIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIM;
#[doc = "`read()` method returns [dmaim::R](dmaim::R) reader structure"]
impl crate::Readable for DMAIM {}
#[doc = "`write(|w| ..)` method takes [dmaim::W](dmaim::W) writer structure"]
impl crate::Writable for DMAIM {}
#[doc = "Ethernet MAC DMA Interrupt Mask Register"]
pub mod dmaim;
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mfboc](mfboc) module"]
pub type MFBOC = crate::Reg<u32, _MFBOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFBOC;
#[doc = "`read()` method returns [mfboc::R](mfboc::R) reader structure"]
impl crate::Readable for MFBOC {}
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter"]
pub mod mfboc;
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxintwdt](rxintwdt) module"]
pub type RXINTWDT = crate::Reg<u32, _RXINTWDT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTWDT;
#[doc = "`read()` method returns [rxintwdt::R](rxintwdt::R) reader structure"]
impl crate::Readable for RXINTWDT {}
#[doc = "`write(|w| ..)` method takes [rxintwdt::W](rxintwdt::W) writer structure"]
impl crate::Writable for RXINTWDT {}
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer"]
pub mod rxintwdt;
#[doc = "Ethernet MAC Current Host Transmit Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hostxdesc](hostxdesc) module"]
pub type HOSTXDESC = crate::Reg<u32, _HOSTXDESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSTXDESC;
#[doc = "`read()` method returns [hostxdesc::R](hostxdesc::R) reader structure"]
impl crate::Readable for HOSTXDESC {}
#[doc = "Ethernet MAC Current Host Transmit Descriptor"]
pub mod hostxdesc;
#[doc = "Ethernet MAC Current Host Receive Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hosrxdesc](hosrxdesc) module"]
pub type HOSRXDESC = crate::Reg<u32, _HOSRXDESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSRXDESC;
#[doc = "`read()` method returns [hosrxdesc::R](hosrxdesc::R) reader structure"]
impl crate::Readable for HOSRXDESC {}
#[doc = "Ethernet MAC Current Host Receive Descriptor"]
pub mod hosrxdesc;
#[doc = "Ethernet MAC Current Host Transmit Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hostxba](hostxba) module"]
pub type HOSTXBA = crate::Reg<u32, _HOSTXBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSTXBA;
#[doc = "`read()` method returns [hostxba::R](hostxba::R) reader structure"]
impl crate::Readable for HOSTXBA {}
#[doc = "Ethernet MAC Current Host Transmit Buffer Address"]
pub mod hostxba;
#[doc = "Ethernet MAC Current Host Receive Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hosrxba](hosrxba) module"]
pub type HOSRXBA = crate::Reg<u32, _HOSRXBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSRXBA;
#[doc = "`read()` method returns [hosrxba::R](hosrxba::R) reader structure"]
impl crate::Readable for HOSRXBA {}
#[doc = "Ethernet MAC Current Host Receive Buffer Address"]
pub mod hosrxba;
#[doc = "Ethernet MAC Peripheral Property Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "Ethernet MAC Peripheral Property Register"]
pub mod pp;
#[doc = "Ethernet MAC Peripheral Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "Ethernet MAC Peripheral Configuration Register"]
pub mod pc;
#[doc = "Ethernet MAC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "Ethernet MAC Clock Configuration Register"]
pub mod cc;
#[doc = "Ethernet PHY Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ephyris](ephyris) module"]
pub type EPHYRIS = crate::Reg<u32, _EPHYRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPHYRIS;
#[doc = "`read()` method returns [ephyris::R](ephyris::R) reader structure"]
impl crate::Readable for EPHYRIS {}
#[doc = "Ethernet PHY Raw Interrupt Status"]
pub mod ephyris;
#[doc = "Ethernet PHY Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ephyim](ephyim) module"]
pub type EPHYIM = crate::Reg<u32, _EPHYIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPHYIM;
#[doc = "`read()` method returns [ephyim::R](ephyim::R) reader structure"]
impl crate::Readable for EPHYIM {}
#[doc = "`write(|w| ..)` method takes [ephyim::W](ephyim::W) writer structure"]
impl crate::Writable for EPHYIM {}
#[doc = "Ethernet PHY Interrupt Mask"]
pub mod ephyim;
#[doc = "Ethernet PHY Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ephymisc](ephymisc) module"]
pub type EPHYMISC = crate::Reg<u32, _EPHYMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPHYMISC;
#[doc = "`read()` method returns [ephymisc::R](ephymisc::R) reader structure"]
impl crate::Readable for EPHYMISC {}
#[doc = "`write(|w| ..)` method takes [ephymisc::W](ephymisc::W) writer structure"]
impl crate::Writable for EPHYMISC {}
#[doc = "Ethernet PHY Masked Interrupt Status and Clear"]
pub mod ephymisc;
