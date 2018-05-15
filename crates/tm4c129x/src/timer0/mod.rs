#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - GPTM Timer A Mode"]
    pub tamr: TAMR,
    #[doc = "0x08 - GPTM Timer B Mode"]
    pub tbmr: TBMR,
    #[doc = "0x0c - GPTM Control"]
    pub ctl: CTL,
    #[doc = "0x10 - GPTM Synchronize"]
    pub sync: SYNC,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - GPTM Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - GPTM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x20 - GPTM Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x24 - GPTM Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x28 - GPTM Timer A Interval Load"]
    pub tailr: TAILR,
    #[doc = "0x2c - GPTM Timer B Interval Load"]
    pub tbilr: TBILR,
    #[doc = "0x30 - GPTM Timer A Match"]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - GPTM Timer B Match"]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - GPTM Timer A Prescale"]
    pub tapr: TAPR,
    #[doc = "0x3c - GPTM Timer B Prescale"]
    pub tbpr: TBPR,
    #[doc = "0x40 - GPTM TimerA Prescale Match"]
    pub tapmr: TAPMR,
    #[doc = "0x44 - GPTM TimerB Prescale Match"]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - GPTM Timer A"]
    pub tar: TAR,
    #[doc = "0x4c - GPTM Timer B"]
    pub tbr: TBR,
    #[doc = "0x50 - GPTM Timer A Value"]
    pub tav: TAV,
    #[doc = "0x54 - GPTM Timer B Value"]
    pub tbv: TBV,
    #[doc = "0x58 - GPTM RTC Predivide"]
    pub rtcpd: RTCPD,
    #[doc = "0x5c - GPTM Timer A Prescale Snapshot"]
    pub taps: TAPS,
    #[doc = "0x60 - GPTM Timer B Prescale Snapshot"]
    pub tbps: TBPS,
    _reserved1: [u8; 8usize],
    #[doc = "0x6c - GPTM DMA Event"]
    pub dmaev: DMAEV,
    #[doc = "0x70 - GPTM ADC Event"]
    pub adcev: ADCEV,
    _reserved2: [u8; 3916usize],
    #[doc = "0xfc0 - GPTM Peripheral Properties"]
    pub pp: PP,
    _reserved3: [u8; 4usize],
    #[doc = "0xfc8 - GPTM Clock Configuration"]
    pub cc: CC,
}
#[doc = "GPTM Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Configuration"]
pub mod cfg;
#[doc = "GPTM Timer A Mode"]
pub struct TAMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Mode"]
pub mod tamr;
#[doc = "GPTM Timer B Mode"]
pub struct TBMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Mode"]
pub mod tbmr;
#[doc = "GPTM Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Control"]
pub mod ctl;
#[doc = "GPTM Synchronize"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Synchronize"]
pub mod sync;
#[doc = "GPTM Interrupt Mask"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Interrupt Mask"]
pub mod imr;
#[doc = "GPTM Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Raw Interrupt Status"]
pub mod ris;
#[doc = "GPTM Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Masked Interrupt Status"]
pub mod mis;
#[doc = "GPTM Interrupt Clear"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Interrupt Clear"]
pub mod icr;
#[doc = "GPTM Timer A Interval Load"]
pub struct TAILR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Interval Load"]
pub mod tailr;
#[doc = "GPTM Timer B Interval Load"]
pub struct TBILR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Interval Load"]
pub mod tbilr;
#[doc = "GPTM Timer A Match"]
pub struct TAMATCHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Match"]
pub mod tamatchr;
#[doc = "GPTM Timer B Match"]
pub struct TBMATCHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Match"]
pub mod tbmatchr;
#[doc = "GPTM Timer A Prescale"]
pub struct TAPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Prescale"]
pub mod tapr;
#[doc = "GPTM Timer B Prescale"]
pub struct TBPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Prescale"]
pub mod tbpr;
#[doc = "GPTM TimerA Prescale Match"]
pub struct TAPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM TimerA Prescale Match"]
pub mod tapmr;
#[doc = "GPTM TimerB Prescale Match"]
pub struct TBPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM TimerB Prescale Match"]
pub mod tbpmr;
#[doc = "GPTM Timer A"]
pub struct TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A"]
pub mod tar;
#[doc = "GPTM Timer B"]
pub struct TBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B"]
pub mod tbr;
#[doc = "GPTM Timer A Value"]
pub struct TAV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Value"]
pub mod tav;
#[doc = "GPTM Timer B Value"]
pub struct TBV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Value"]
pub mod tbv;
#[doc = "GPTM RTC Predivide"]
pub struct RTCPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM RTC Predivide"]
pub mod rtcpd;
#[doc = "GPTM Timer A Prescale Snapshot"]
pub struct TAPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer A Prescale Snapshot"]
pub mod taps;
#[doc = "GPTM Timer B Prescale Snapshot"]
pub struct TBPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Timer B Prescale Snapshot"]
pub mod tbps;
#[doc = "GPTM DMA Event"]
pub struct DMAEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM DMA Event"]
pub mod dmaev;
#[doc = "GPTM ADC Event"]
pub struct ADCEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM ADC Event"]
pub mod adcev;
#[doc = "GPTM Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Peripheral Properties"]
pub mod pp;
#[doc = "GPTM Clock Configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM Clock Configuration"]
pub mod cc;
