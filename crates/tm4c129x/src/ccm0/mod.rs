#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - CRC Control"]
    pub crcctrl: CRCCTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x410 - CRC SEED/Context"]
    pub crcseed: CRCSEED,
    #[doc = "0x414 - CRC Data Input"]
    pub crcdin: CRCDIN,
    #[doc = "0x418 - CRC Post Processing Result"]
    pub crcrsltpp: CRCRSLTPP,
}
#[doc = "CRC Control"]
pub struct CRCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC SEED/Context"]
pub struct CRCSEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC SEED/Context"]
pub mod crcseed;
#[doc = "CRC Data Input"]
pub struct CRCDIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Input"]
pub mod crcdin;
#[doc = "CRC Post Processing Result"]
pub struct CRCRSLTPP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Post Processing Result"]
pub mod crcrsltpp;
