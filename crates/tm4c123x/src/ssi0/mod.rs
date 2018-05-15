#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SSI Control 0"]
    pub cr0: CR0,
    #[doc = "0x04 - SSI Control 1"]
    pub cr1: CR1,
    #[doc = "0x08 - SSI Data"]
    pub dr: DR,
    #[doc = "0x0c - SSI Status"]
    pub sr: SR,
    #[doc = "0x10 - SSI Clock Prescale"]
    pub cpsr: CPSR,
    #[doc = "0x14 - SSI Interrupt Mask"]
    pub im: IM,
    #[doc = "0x18 - SSI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - SSI Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - SSI Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x24 - SSI DMA Control"]
    pub dmactl: DMACTL,
    _reserved0: [u8; 4000usize],
    #[doc = "0xfc8 - SSI Clock Configuration"]
    pub cc: CC,
}
#[doc = "SSI Control 0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Control 0"]
pub mod cr0;
#[doc = "SSI Control 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Control 1"]
pub mod cr1;
#[doc = "SSI Data"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Data"]
pub mod dr;
#[doc = "SSI Status"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Status"]
pub mod sr;
#[doc = "SSI Clock Prescale"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Clock Prescale"]
pub mod cpsr;
#[doc = "SSI Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Interrupt Mask"]
pub mod im;
#[doc = "SSI Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Raw Interrupt Status"]
pub mod ris;
#[doc = "SSI Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Masked Interrupt Status"]
pub mod mis;
#[doc = "SSI Interrupt Clear"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Interrupt Clear"]
pub mod icr;
#[doc = "SSI DMA Control"]
pub struct DMACTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI DMA Control"]
pub mod dmactl;
#[doc = "SSI Clock Configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Clock Configuration"]
pub mod cc;
