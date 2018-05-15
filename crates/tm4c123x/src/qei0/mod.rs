#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QEI Control"]
    pub ctl: CTL,
    #[doc = "0x04 - QEI Status"]
    pub stat: STAT,
    #[doc = "0x08 - QEI Position"]
    pub pos: POS,
    #[doc = "0x0c - QEI Maximum Position"]
    pub maxpos: MAXPOS,
    #[doc = "0x10 - QEI Timer Load"]
    pub load: LOAD,
    #[doc = "0x14 - QEI Timer"]
    pub time: TIME,
    #[doc = "0x18 - QEI Velocity Counter"]
    pub count: COUNT,
    #[doc = "0x1c - QEI Velocity"]
    pub speed: SPEED,
    #[doc = "0x20 - QEI Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x24 - QEI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x28 - QEI Interrupt Status and Clear"]
    pub isc: ISC,
}
#[doc = "QEI Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Control"]
pub mod ctl;
#[doc = "QEI Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Status"]
pub mod stat;
#[doc = "QEI Position"]
pub struct POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Position"]
pub mod pos;
#[doc = "QEI Maximum Position"]
pub struct MAXPOS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Maximum Position"]
pub mod maxpos;
#[doc = "QEI Timer Load"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Timer Load"]
pub mod load;
#[doc = "QEI Timer"]
pub struct TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Timer"]
pub mod time;
#[doc = "QEI Velocity Counter"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Velocity Counter"]
pub mod count;
#[doc = "QEI Velocity"]
pub struct SPEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Velocity"]
pub mod speed;
#[doc = "QEI Interrupt Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Interrupt Enable"]
pub mod inten;
#[doc = "QEI Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Raw Interrupt Status"]
pub mod ris;
#[doc = "QEI Interrupt Status and Clear"]
pub struct ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QEI Interrupt Status and Clear"]
pub mod isc;
