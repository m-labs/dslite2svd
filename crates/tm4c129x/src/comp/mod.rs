#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Masked Interrupt Status"]
    pub acmis: ACMIS,
    #[doc = "0x04 - Analog Comparator Raw Interrupt Status"]
    pub acris: ACRIS,
    #[doc = "0x08 - Analog Comparator Interrupt Enable"]
    pub acinten: ACINTEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Analog Comparator Reference Voltage Control"]
    pub acrefctl: ACREFCTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Analog Comparator Status 0"]
    pub acstat0: ACSTAT0,
    #[doc = "0x24 - Analog Comparator Control 0"]
    pub acctl0: ACCTL0,
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - Analog Comparator Status 1"]
    pub acstat1: ACSTAT1,
    #[doc = "0x44 - Analog Comparator Control 1"]
    pub acctl1: ACCTL1,
    _reserved3: [u8; 24usize],
    #[doc = "0x60 - Analog Comparator Status 2"]
    pub acstat2: ACSTAT2,
    #[doc = "0x64 - Analog Comparator Control 2"]
    pub acctl2: ACCTL2,
    _reserved4: [u8; 3928usize],
    #[doc = "0xfc0 - Analog Comparator Peripheral Properties"]
    pub pp: PP,
}
#[doc = "Analog Comparator Masked Interrupt Status"]
pub struct ACMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Masked Interrupt Status"]
pub mod acmis;
#[doc = "Analog Comparator Raw Interrupt Status"]
pub struct ACRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Raw Interrupt Status"]
pub mod acris;
#[doc = "Analog Comparator Interrupt Enable"]
pub struct ACINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Interrupt Enable"]
pub mod acinten;
#[doc = "Analog Comparator Reference Voltage Control"]
pub struct ACREFCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Reference Voltage Control"]
pub mod acrefctl;
#[doc = "Analog Comparator Status 0"]
pub struct ACSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Status 0"]
pub mod acstat0;
#[doc = "Analog Comparator Control 0"]
pub struct ACCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Control 0"]
pub mod acctl0;
#[doc = "Analog Comparator Status 1"]
pub struct ACSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Status 1"]
pub mod acstat1;
#[doc = "Analog Comparator Control 1"]
pub struct ACCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Control 1"]
pub mod acctl1;
#[doc = "Analog Comparator Status 2"]
pub struct ACSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Status 2"]
pub mod acstat2;
#[doc = "Analog Comparator Control 2"]
pub struct ACCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Control 2"]
pub mod acctl2;
#[doc = "Analog Comparator Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Peripheral Properties"]
pub mod pp;
