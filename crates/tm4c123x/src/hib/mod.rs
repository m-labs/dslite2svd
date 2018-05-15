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
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Hibernation Data"]
    pub data: DATA,
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
#[doc = "Hibernation Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Data"]
pub mod data;
