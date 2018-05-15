#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Exception Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x04 - System Exception Interrupt Mask"]
    pub im: IM,
    #[doc = "0x08 - System Exception Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x0c - System Exception Interrupt Clear"]
    pub ic: IC,
}
#[doc = "System Exception Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Raw Interrupt Status"]
pub mod ris;
#[doc = "System Exception Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Interrupt Mask"]
pub mod im;
#[doc = "System Exception Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Masked Interrupt Status"]
pub mod mis;
#[doc = "System Exception Interrupt Clear"]
pub struct IC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Interrupt Clear"]
pub mod ic;
