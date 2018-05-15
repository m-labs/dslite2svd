#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load"]
    pub load: LOAD,
    #[doc = "0x04 - Watchdog Value"]
    pub value: VALUE,
    #[doc = "0x08 - Watchdog Control"]
    pub ctl: CTL,
    #[doc = "0x0c - Watchdog Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x14 - Watchdog Masked Interrupt Status"]
    pub mis: MIS,
    _reserved0: [u8; 1024usize],
    #[doc = "0x418 - Watchdog Test"]
    pub test: TEST,
    _reserved1: [u8; 2020usize],
    #[doc = "0xc00 - Watchdog Lock"]
    pub lock: LOCK,
}
#[doc = "Watchdog Load"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Load"]
pub mod load;
#[doc = "Watchdog Value"]
pub struct VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Value"]
pub mod value;
#[doc = "Watchdog Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Control"]
pub mod ctl;
#[doc = "Watchdog Interrupt Clear"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Interrupt Clear"]
pub mod icr;
#[doc = "Watchdog Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Raw Interrupt Status"]
pub mod ris;
#[doc = "Watchdog Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Masked Interrupt Status"]
pub mod mis;
#[doc = "Watchdog Test"]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Test"]
pub mod test;
#[doc = "Watchdog Lock"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Lock"]
pub mod lock;
