#[doc = r" Register block"]
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
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Ethernet MAC Status"]
    pub status: STATUS,
    #[doc = "0x28 - Ethernet MAC Remote Wake-Up Frame Filter"]
    pub rwuff: RWUFF,
    #[doc = "0x2c - Ethernet MAC PMT Control and Status Register"]
    pub pmtctlstat: PMTCTLSTAT,
    _reserved1: [u8; 8usize],
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
    _reserved2: [u8; 124usize],
    #[doc = "0xdc - Ethernet MAC Watchdog Timeout"]
    pub wdogto: WDOGTO,
    _reserved3: [u8; 32usize],
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
    _reserved4: [u8; 4usize],
    #[doc = "0x118 - Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
    pub txcntgb: TXCNTGB,
    _reserved5: [u8; 48usize],
    #[doc = "0x14c - Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
    pub txcntscol: TXCNTSCOL,
    #[doc = "0x150 - Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
    pub txcntmcol: TXCNTMCOL,
    _reserved6: [u8; 16usize],
    #[doc = "0x164 - Ethernet MAC Transmit Octet Count Good"]
    pub txoctcntg: TXOCTCNTG,
    _reserved7: [u8; 24usize],
    #[doc = "0x180 - Ethernet MAC Receive Frame Count for Good and Bad Frames"]
    pub rxcntgb: RXCNTGB,
    _reserved8: [u8; 16usize],
    #[doc = "0x194 - Ethernet MAC Receive Frame Count for CRC Error Frames"]
    pub rxcntcrcerr: RXCNTCRCERR,
    #[doc = "0x198 - Ethernet MAC Receive Frame Count for Alignment Error Frames"]
    pub rxcntalgnerr: RXCNTALGNERR,
    _reserved9: [u8; 40usize],
    #[doc = "0x1c4 - Ethernet MAC Receive Frame Count for Good Unicast Frames"]
    pub rxcntguni: RXCNTGUNI,
    _reserved10: [u8; 956usize],
    #[doc = "0x584 - Ethernet MAC VLAN Tag Inclusion or Replacement"]
    pub vlnincrep: VLNINCREP,
    #[doc = "0x588 - Ethernet MAC VLAN Hash Table"]
    pub vlanhash: VLANHASH,
    _reserved11: [u8; 372usize],
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
    _reserved12: [u8; 48usize],
    #[doc = "0x760 - Ethernet MAC PPS0 Interval"]
    pub pps0intvl: PPS0INTVL,
    #[doc = "0x764 - Ethernet MAC PPS0 Width"]
    pub pps0width: PPS0WIDTH,
    _reserved13: [u8; 1176usize],
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
    _reserved14: [u8; 32usize],
    #[doc = "0xc48 - Ethernet MAC Current Host Transmit Descriptor"]
    pub hostxdesc: HOSTXDESC,
    #[doc = "0xc4c - Ethernet MAC Current Host Receive Descriptor"]
    pub hosrxdesc: HOSRXDESC,
    #[doc = "0xc50 - Ethernet MAC Current Host Transmit Buffer Address"]
    pub hostxba: HOSTXBA,
    #[doc = "0xc54 - Ethernet MAC Current Host Receive Buffer Address"]
    pub hosrxba: HOSRXBA,
    _reserved15: [u8; 872usize],
    #[doc = "0xfc0 - Ethernet MAC Peripheral Property Register"]
    pub pp: PP,
    #[doc = "0xfc4 - Ethernet MAC Peripheral Configuration Register"]
    pub pc: PC,
    #[doc = "0xfc8 - Ethernet MAC Clock Configuration Register"]
    pub cc: CC,
    _reserved16: [u8; 4usize],
    #[doc = "0xfd0 - Ethernet PHY Raw Interrupt Status"]
    pub ephyris: EPHYRIS,
    #[doc = "0xfd4 - Ethernet PHY Interrupt Mask"]
    pub ephyim: EPHYIM,
    #[doc = "0xfd8 - Ethernet PHY Masked Interrupt Status and Clear"]
    pub ephymisc: EPHYMISC,
}
#[doc = "Ethernet MAC Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Configuration"]
pub mod cfg;
#[doc = "Ethernet MAC Frame Filter"]
pub struct FRAMEFLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Frame Filter"]
pub mod framefltr;
#[doc = "Ethernet MAC Hash Table High"]
pub struct HASHTBLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Hash Table High"]
pub mod hashtblh;
#[doc = "Ethernet MAC Hash Table Low"]
pub struct HASHTBLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Hash Table Low"]
pub mod hashtbll;
#[doc = "Ethernet MAC MII Address"]
pub struct MIIADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII Address"]
pub mod miiaddr;
#[doc = "Ethernet MAC MII Data Register"]
pub struct MIIDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII Data Register"]
pub mod miidata;
#[doc = "Ethernet MAC Flow Control"]
pub struct FLOWCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Flow Control"]
pub mod flowctl;
#[doc = "Ethernet MAC VLAN Tag"]
pub struct VLANTG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN Tag"]
pub mod vlantg;
#[doc = "Ethernet MAC Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Status"]
pub mod status;
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter"]
pub struct RWUFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter"]
pub mod rwuff;
#[doc = "Ethernet MAC PMT Control and Status Register"]
pub struct PMTCTLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PMT Control and Status Register"]
pub mod pmtctlstat;
#[doc = "Ethernet MAC Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Raw Interrupt Status"]
pub mod ris;
#[doc = "Ethernet MAC Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Interrupt Mask"]
pub mod im;
#[doc = "Ethernet MAC Address 0 High"]
pub struct ADDR0H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 0 High"]
pub mod addr0h;
#[doc = "Ethernet MAC Address 0 Low Register"]
pub struct ADDR0L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 0 Low Register"]
pub mod addr0l;
#[doc = "Ethernet MAC Address 1 High"]
pub struct ADDR1H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 1 High"]
pub mod addr1h;
#[doc = "Ethernet MAC Address 1 Low"]
pub struct ADDR1L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 1 Low"]
pub mod addr1l;
#[doc = "Ethernet MAC Address 2 High"]
pub struct ADDR2H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 2 High"]
pub mod addr2h;
#[doc = "Ethernet MAC Address 2 Low"]
pub struct ADDR2L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 2 Low"]
pub mod addr2l;
#[doc = "Ethernet MAC Address 3 High"]
pub struct ADDR3H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 3 High"]
pub mod addr3h;
#[doc = "Ethernet MAC Address 3 Low"]
pub struct ADDR3L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Address 3 Low"]
pub mod addr3l;
#[doc = "Ethernet MAC Watchdog Timeout"]
pub struct WDOGTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Watchdog Timeout"]
pub mod wdogto;
#[doc = "Ethernet MAC MMC Control"]
pub struct MMCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MMC Control"]
pub mod mmcctrl;
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status"]
pub struct MMCRXRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status"]
pub mod mmcrxris;
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status"]
pub struct MMCTXRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status"]
pub mod mmctxris;
#[doc = "Ethernet MAC MMC Receive Interrupt Mask"]
pub struct MMCRXIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MMC Receive Interrupt Mask"]
pub mod mmcrxim;
#[doc = "Ethernet MAC MMC Transmit Interrupt Mask"]
pub struct MMCTXIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MMC Transmit Interrupt Mask"]
pub mod mmctxim;
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
pub struct TXCNTGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
pub mod txcntgb;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
pub struct TXCNTSCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod txcntscol;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
pub struct TXCNTMCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
pub mod txcntmcol;
#[doc = "Ethernet MAC Transmit Octet Count Good"]
pub struct TXOCTCNTG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Octet Count Good"]
pub mod txoctcntg;
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames"]
pub struct RXCNTGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames"]
pub mod rxcntgb;
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames"]
pub struct RXCNTCRCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames"]
pub mod rxcntcrcerr;
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames"]
pub struct RXCNTALGNERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames"]
pub mod rxcntalgnerr;
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames"]
pub struct RXCNTGUNI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames"]
pub mod rxcntguni;
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement"]
pub struct VLNINCREP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement"]
pub mod vlnincrep;
#[doc = "Ethernet MAC VLAN Hash Table"]
pub struct VLANHASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN Hash Table"]
pub mod vlanhash;
#[doc = "Ethernet MAC Timestamp Control"]
pub struct TIMSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Timestamp Control"]
pub mod timstctrl;
#[doc = "Ethernet MAC Sub-Second Increment"]
pub struct SUBSECINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Sub-Second Increment"]
pub mod subsecinc;
#[doc = "Ethernet MAC System Time - Seconds"]
pub struct TIMSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC System Time - Seconds"]
pub mod timsec;
#[doc = "Ethernet MAC System Time - Nanoseconds"]
pub struct TIMNANO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC System Time - Nanoseconds"]
pub mod timnano;
#[doc = "Ethernet MAC System Time - Seconds Update"]
pub struct TIMSECU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC System Time - Seconds Update"]
pub mod timsecu;
#[doc = "Ethernet MAC System Time - Nanoseconds Update"]
pub struct TIMNANOU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC System Time - Nanoseconds Update"]
pub mod timnanou;
#[doc = "Ethernet MAC Timestamp Addend"]
pub struct TIMADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Timestamp Addend"]
pub mod timadd;
#[doc = "Ethernet MAC Target Time Seconds"]
pub struct TARGSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Target Time Seconds"]
pub mod targsec;
#[doc = "Ethernet MAC Target Time Nanoseconds"]
pub struct TARGNANO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Target Time Nanoseconds"]
pub mod targnano;
#[doc = "Ethernet MAC System Time-Higher Word Seconds"]
pub struct HWORDSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC System Time-Higher Word Seconds"]
pub mod hwordsec;
#[doc = "Ethernet MAC Timestamp Status"]
pub struct TIMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Timestamp Status"]
pub mod timstat;
#[doc = "Ethernet MAC PPS Control"]
pub struct PPSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PPS Control"]
pub mod ppsctrl;
#[doc = "Ethernet MAC PPS0 Interval"]
pub struct PPS0INTVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PPS0 Interval"]
pub mod pps0intvl;
#[doc = "Ethernet MAC PPS0 Width"]
pub struct PPS0WIDTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PPS0 Width"]
pub mod pps0width;
#[doc = "Ethernet MAC DMA Bus Mode"]
pub struct DMABUSMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC DMA Bus Mode"]
pub mod dmabusmod;
#[doc = "Ethernet MAC Transmit Poll Demand"]
pub struct TXPOLLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Poll Demand"]
pub mod txpolld;
#[doc = "Ethernet MAC Receive Poll Demand"]
pub struct RXPOLLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Poll Demand"]
pub mod rxpolld;
#[doc = "Ethernet MAC Receive Descriptor List Address"]
pub struct RXDLADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Descriptor List Address"]
pub mod rxdladdr;
#[doc = "Ethernet MAC Transmit Descriptor List Address"]
pub struct TXDLADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Transmit Descriptor List Address"]
pub mod txdladdr;
#[doc = "Ethernet MAC DMA Interrupt Status"]
pub struct DMARIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC DMA Interrupt Status"]
pub mod dmaris;
#[doc = "Ethernet MAC DMA Operation Mode"]
pub struct DMAOPMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC DMA Operation Mode"]
pub mod dmaopmode;
#[doc = "Ethernet MAC DMA Interrupt Mask Register"]
pub struct DMAIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC DMA Interrupt Mask Register"]
pub mod dmaim;
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter"]
pub struct MFBOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter"]
pub mod mfboc;
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer"]
pub struct RXINTWDT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer"]
pub mod rxintwdt;
#[doc = "Ethernet MAC Current Host Transmit Descriptor"]
pub struct HOSTXDESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Current Host Transmit Descriptor"]
pub mod hostxdesc;
#[doc = "Ethernet MAC Current Host Receive Descriptor"]
pub struct HOSRXDESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Current Host Receive Descriptor"]
pub mod hosrxdesc;
#[doc = "Ethernet MAC Current Host Transmit Buffer Address"]
pub struct HOSTXBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Current Host Transmit Buffer Address"]
pub mod hostxba;
#[doc = "Ethernet MAC Current Host Receive Buffer Address"]
pub struct HOSRXBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Current Host Receive Buffer Address"]
pub mod hosrxba;
#[doc = "Ethernet MAC Peripheral Property Register"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Peripheral Property Register"]
pub mod pp;
#[doc = "Ethernet MAC Peripheral Configuration Register"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Peripheral Configuration Register"]
pub mod pc;
#[doc = "Ethernet MAC Clock Configuration Register"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Clock Configuration Register"]
pub mod cc;
#[doc = "Ethernet PHY Raw Interrupt Status"]
pub struct EPHYRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Raw Interrupt Status"]
pub mod ephyris;
#[doc = "Ethernet PHY Interrupt Mask"]
pub struct EPHYIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Interrupt Mask"]
pub mod ephyim;
#[doc = "Ethernet PHY Masked Interrupt Status and Clear"]
pub struct EPHYMISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Masked Interrupt Status and Clear"]
pub mod ephymisc;
