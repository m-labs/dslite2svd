#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1020usize],
    #[doc = "0x3fc - GPIO Data"]
    pub data: DATA,
    #[doc = "0x400 - GPIO Direction"]
    pub dir: DIR,
    #[doc = "0x404 - GPIO Interrupt Sense"]
    pub is: IS,
    #[doc = "0x408 - GPIO Interrupt Both Edges"]
    pub ibe: IBE,
    #[doc = "0x40c - GPIO Interrupt Event"]
    pub iev: IEV,
    #[doc = "0x410 - GPIO Interrupt Mask"]
    pub im: IM,
    #[doc = "0x414 - GPIO Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x418 - GPIO Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x41c - GPIO Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x420 - GPIO Alternate Function Select"]
    pub afsel: AFSEL,
    _reserved1: [u8; 220usize],
    #[doc = "0x500 - GPIO 2-mA Drive Select"]
    pub dr2r: DR2R,
    #[doc = "0x504 - GPIO 4-mA Drive Select"]
    pub dr4r: DR4R,
    #[doc = "0x508 - GPIO 8-mA Drive Select"]
    pub dr8r: DR8R,
    #[doc = "0x50c - GPIO Open Drain Select"]
    pub odr: ODR,
    #[doc = "0x510 - GPIO Pull-Up Select"]
    pub pur: PUR,
    #[doc = "0x514 - GPIO Pull-Down Select"]
    pub pdr: PDR,
    #[doc = "0x518 - GPIO Slew Rate Control Select"]
    pub slr: SLR,
    #[doc = "0x51c - GPIO Digital Enable"]
    pub den: DEN,
    #[doc = "0x520 - GPIO Lock"]
    pub lock: LOCK,
    #[doc = "0x524 - GPIO Commit"]
    pub cr: CR,
    #[doc = "0x528 - GPIO Analog Mode Select"]
    pub amsel: AMSEL,
    #[doc = "0x52c - GPIO Port Control"]
    pub pctl: PCTL,
    #[doc = "0x530 - GPIO ADC Control"]
    pub adcctl: ADCCTL,
    #[doc = "0x534 - GPIO DMA Control"]
    pub dmactl: DMACTL,
}
#[doc = "GPIO Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Data"]
pub mod data;
#[doc = "GPIO Direction"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Direction"]
pub mod dir;
#[doc = "GPIO Interrupt Sense"]
pub struct IS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Sense"]
pub mod is;
#[doc = "GPIO Interrupt Both Edges"]
pub struct IBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Both Edges"]
pub mod ibe;
#[doc = "GPIO Interrupt Event"]
pub struct IEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Event"]
pub mod iev;
#[doc = "GPIO Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Mask"]
pub mod im;
#[doc = "GPIO Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Raw Interrupt Status"]
pub mod ris;
#[doc = "GPIO Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Masked Interrupt Status"]
pub mod mis;
#[doc = "GPIO Interrupt Clear"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Clear"]
pub mod icr;
#[doc = "GPIO Alternate Function Select"]
pub struct AFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Alternate Function Select"]
pub mod afsel;
#[doc = "GPIO 2-mA Drive Select"]
pub struct DR2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO 2-mA Drive Select"]
pub mod dr2r;
#[doc = "GPIO 4-mA Drive Select"]
pub struct DR4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO 4-mA Drive Select"]
pub mod dr4r;
#[doc = "GPIO 8-mA Drive Select"]
pub struct DR8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO 8-mA Drive Select"]
pub mod dr8r;
#[doc = "GPIO Open Drain Select"]
pub struct ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Open Drain Select"]
pub mod odr;
#[doc = "GPIO Pull-Up Select"]
pub struct PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Pull-Up Select"]
pub mod pur;
#[doc = "GPIO Pull-Down Select"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Pull-Down Select"]
pub mod pdr;
#[doc = "GPIO Slew Rate Control Select"]
pub struct SLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Slew Rate Control Select"]
pub mod slr;
#[doc = "GPIO Digital Enable"]
pub struct DEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Digital Enable"]
pub mod den;
#[doc = "GPIO Lock"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Lock"]
pub mod lock;
#[doc = "GPIO Commit"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Commit"]
pub mod cr;
#[doc = "GPIO Analog Mode Select"]
pub struct AMSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Analog Mode Select"]
pub mod amsel;
#[doc = "GPIO Port Control"]
pub struct PCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port Control"]
pub mod pctl;
#[doc = "GPIO ADC Control"]
pub struct ADCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO ADC Control"]
pub mod adcctl;
#[doc = "GPIO DMA Control"]
pub struct DMACTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO DMA Control"]
pub mod dmactl;
