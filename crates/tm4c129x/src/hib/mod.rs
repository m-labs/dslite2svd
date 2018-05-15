#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernation RTC Counter"]
    pub rtcc: RTCC,
    #[doc = "0x04 - Hibernation RTC Match 0"]
    pub rtcm0: RTCM0,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Hibernation RTC Load"]
    pub rtcld: RTCLD,
    #[doc = "0x10 - Hibernation Control"]
    pub ctl: CTL,
    #[doc = "0x14 - Hibernation Interrupt Mask"]
    pub im: IM,
    #[doc = "0x18 - Hibernation Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - Hibernation Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - Hibernation Interrupt Clear"]
    pub ic: IC,
    #[doc = "0x24 - Hibernation RTC Trim"]
    pub rtct: RTCT,
    #[doc = "0x28 - Hibernation RTC Sub Seconds"]
    pub rtcss: RTCSS,
    #[doc = "0x2c - Hibernation IO Configuration"]
    pub io: IO,
    #[doc = "0x30 - Hibernation Data"]
    pub data: DATA,
    _reserved1: [u8; 716usize],
    #[doc = "0x300 - Hibernation Calendar Control"]
    pub calctl: CALCTL,
    _reserved2: [u8; 12usize],
    #[doc = "0x310 - Hibernation Calendar 0"]
    pub cal0: CAL0,
    #[doc = "0x314 - Hibernation Calendar 1"]
    pub cal1: CAL1,
    _reserved3: [u8; 8usize],
    #[doc = "0x320 - Hibernation Calendar Load 0"]
    pub calld0: CALLD0,
    #[doc = "0x324 - Hibernation Calendar Load"]
    pub calld1: CALLD1,
    _reserved4: [u8; 8usize],
    #[doc = "0x330 - Hibernation Calendar Match 0"]
    pub calm0: CALM0,
    #[doc = "0x334 - Hibernation Calendar Match 1"]
    pub calm1: CALM1,
    _reserved5: [u8; 40usize],
    #[doc = "0x360 - Hibernation Lock"]
    pub lock: LOCK,
    _reserved6: [u8; 156usize],
    #[doc = "0x400 - HIB Tamper Control"]
    pub tpctl: TPCTL,
    #[doc = "0x404 - HIB Tamper Status"]
    pub tpstat: TPSTAT,
    _reserved7: [u8; 8usize],
    #[doc = "0x410 - HIB Tamper I/O Control"]
    pub tpio: TPIO,
    _reserved8: [u8; 204usize],
    #[doc = "0x4e0 - HIB Tamper Log 0"]
    pub tplog0: TPLOG0,
    #[doc = "0x4e4 - HIB Tamper Log 1"]
    pub tplog1: TPLOG1,
    #[doc = "0x4e8 - HIB Tamper Log 2"]
    pub tplog2: TPLOG2,
    #[doc = "0x4ec - HIB Tamper Log 3"]
    pub tplog3: TPLOG3,
    #[doc = "0x4f0 - HIB Tamper Log 4"]
    pub tplog4: TPLOG4,
    #[doc = "0x4f4 - HIB Tamper Log 5"]
    pub tplog5: TPLOG5,
    #[doc = "0x4f8 - HIB Tamper Log 6"]
    pub tplog6: TPLOG6,
    #[doc = "0x4fc - HIB Tamper Log 7"]
    pub tplog7: TPLOG7,
    _reserved9: [u8; 2752usize],
    #[doc = "0xfc0 - Hibernation Peripheral Properties"]
    pub pp: PP,
    _reserved10: [u8; 4usize],
    #[doc = "0xfc8 - Hibernation Clock Control"]
    pub cc: CC,
}
#[doc = "Hibernation RTC Counter"]
pub struct RTCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation RTC Counter"]
pub mod rtcc;
#[doc = "Hibernation RTC Match 0"]
pub struct RTCM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation RTC Match 0"]
pub mod rtcm0;
#[doc = "Hibernation RTC Load"]
pub struct RTCLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation RTC Load"]
pub mod rtcld;
#[doc = "Hibernation Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Control"]
pub mod ctl;
#[doc = "Hibernation Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Interrupt Mask"]
pub mod im;
#[doc = "Hibernation Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Raw Interrupt Status"]
pub mod ris;
#[doc = "Hibernation Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Masked Interrupt Status"]
pub mod mis;
#[doc = "Hibernation Interrupt Clear"]
pub struct IC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Interrupt Clear"]
pub mod ic;
#[doc = "Hibernation RTC Trim"]
pub struct RTCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation RTC Trim"]
pub mod rtct;
#[doc = "Hibernation RTC Sub Seconds"]
pub struct RTCSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation RTC Sub Seconds"]
pub mod rtcss;
#[doc = "Hibernation IO Configuration"]
pub struct IO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation IO Configuration"]
pub mod io;
#[doc = "Hibernation Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Data"]
pub mod data;
#[doc = "Hibernation Calendar Control"]
pub struct CALCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar Control"]
pub mod calctl;
#[doc = "Hibernation Calendar 0"]
pub struct CAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar 0"]
pub mod cal0;
#[doc = "Hibernation Calendar 1"]
pub struct CAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar 1"]
pub mod cal1;
#[doc = "Hibernation Calendar Load 0"]
pub struct CALLD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar Load 0"]
pub mod calld0;
#[doc = "Hibernation Calendar Load"]
pub struct CALLD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar Load"]
pub mod calld1;
#[doc = "Hibernation Calendar Match 0"]
pub struct CALM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar Match 0"]
pub mod calm0;
#[doc = "Hibernation Calendar Match 1"]
pub struct CALM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Calendar Match 1"]
pub mod calm1;
#[doc = "Hibernation Lock"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Lock"]
pub mod lock;
#[doc = "HIB Tamper Control"]
pub struct TPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Control"]
pub mod tpctl;
#[doc = "HIB Tamper Status"]
pub struct TPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Status"]
pub mod tpstat;
#[doc = "HIB Tamper I/O Control"]
pub struct TPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper I/O Control"]
pub mod tpio;
#[doc = "HIB Tamper Log 0"]
pub struct TPLOG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 0"]
pub mod tplog0;
#[doc = "HIB Tamper Log 1"]
pub struct TPLOG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 1"]
pub mod tplog1;
#[doc = "HIB Tamper Log 2"]
pub struct TPLOG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 2"]
pub mod tplog2;
#[doc = "HIB Tamper Log 3"]
pub struct TPLOG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 3"]
pub mod tplog3;
#[doc = "HIB Tamper Log 4"]
pub struct TPLOG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 4"]
pub mod tplog4;
#[doc = "HIB Tamper Log 5"]
pub struct TPLOG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 5"]
pub mod tplog5;
#[doc = "HIB Tamper Log 6"]
pub struct TPLOG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 6"]
pub mod tplog6;
#[doc = "HIB Tamper Log 7"]
pub struct TPLOG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HIB Tamper Log 7"]
pub mod tplog7;
#[doc = "Hibernation Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Peripheral Properties"]
pub mod pp;
#[doc = "Hibernation Clock Control"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Clock Control"]
pub mod cc;
