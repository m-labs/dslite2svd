#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    pub stat: STAT,
    #[doc = "0x04 - DMA Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - DMA Channel Control Base Pointer"]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - DMA Alternate Channel Control Base Pointer"]
    pub altbase: ALTBASE,
    #[doc = "0x10 - DMA Channel Wait-on-Request Status"]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - DMA Channel Software Request"]
    pub swreq: SWREQ,
    #[doc = "0x18 - DMA Channel Useburst Set"]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - DMA Channel Useburst Clear"]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - DMA Channel Request Mask Set"]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - DMA Channel Request Mask Clear"]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - DMA Channel Enable Set"]
    pub enaset: ENASET,
    #[doc = "0x2c - DMA Channel Enable Clear"]
    pub enaclr: ENACLR,
    #[doc = "0x30 - DMA Channel Primary Alternate Set"]
    pub altset: ALTSET,
    #[doc = "0x34 - DMA Channel Primary Alternate Clear"]
    pub altclr: ALTCLR,
    #[doc = "0x38 - DMA Channel Priority Set"]
    pub prioset: PRIOSET,
    #[doc = "0x3c - DMA Channel Priority Clear"]
    pub prioclr: PRIOCLR,
    _reserved0: [u8; 12usize],
    #[doc = "0x4c - DMA Bus Error Clear"]
    pub errclr: ERRCLR,
    _reserved1: [u8; 1200usize],
    #[doc = "0x500 - DMA Channel Assignment"]
    pub chasgn: CHASGN,
    _reserved2: [u8; 12usize],
    #[doc = "0x510 - DMA Channel Map Select 0"]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - DMA Channel Map Select 1"]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - DMA Channel Map Select 2"]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - DMA Channel Map Select 3"]
    pub chmap3: CHMAP3,
}
#[doc = "DMA Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status"]
pub mod stat;
#[doc = "DMA Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "DMA Channel Control Base Pointer"]
pub struct CTLBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Control Base Pointer"]
pub mod ctlbase;
#[doc = "DMA Alternate Channel Control Base Pointer"]
pub struct ALTBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Alternate Channel Control Base Pointer"]
pub mod altbase;
#[doc = "DMA Channel Wait-on-Request Status"]
pub struct WAITSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Wait-on-Request Status"]
pub mod waitstat;
#[doc = "DMA Channel Software Request"]
pub struct SWREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Software Request"]
pub mod swreq;
#[doc = "DMA Channel Useburst Set"]
pub struct USEBURSTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Useburst Set"]
pub mod useburstset;
#[doc = "DMA Channel Useburst Clear"]
pub struct USEBURSTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Useburst Clear"]
pub mod useburstclr;
#[doc = "DMA Channel Request Mask Set"]
pub struct REQMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Request Mask Set"]
pub mod reqmaskset;
#[doc = "DMA Channel Request Mask Clear"]
pub struct REQMASKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Request Mask Clear"]
pub mod reqmaskclr;
#[doc = "DMA Channel Enable Set"]
pub struct ENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Enable Set"]
pub mod enaset;
#[doc = "DMA Channel Enable Clear"]
pub struct ENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Enable Clear"]
pub mod enaclr;
#[doc = "DMA Channel Primary Alternate Set"]
pub struct ALTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Primary Alternate Set"]
pub mod altset;
#[doc = "DMA Channel Primary Alternate Clear"]
pub struct ALTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Primary Alternate Clear"]
pub mod altclr;
#[doc = "DMA Channel Priority Set"]
pub struct PRIOSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Priority Set"]
pub mod prioset;
#[doc = "DMA Channel Priority Clear"]
pub struct PRIOCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Priority Clear"]
pub mod prioclr;
#[doc = "DMA Bus Error Clear"]
pub struct ERRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Bus Error Clear"]
pub mod errclr;
#[doc = "DMA Channel Assignment"]
pub struct CHASGN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Assignment"]
pub mod chasgn;
#[doc = "DMA Channel Map Select 0"]
pub struct CHMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Map Select 0"]
pub mod chmap0;
#[doc = "DMA Channel Map Select 1"]
pub struct CHMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Map Select 1"]
pub mod chmap1;
#[doc = "DMA Channel Map Select 2"]
pub struct CHMAP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Map Select 2"]
pub mod chmap2;
#[doc = "DMA Channel Map Select 3"]
pub struct CHMAP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Map Select 3"]
pub mod chmap3;
