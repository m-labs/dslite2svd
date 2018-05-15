#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Master Slave Address"]
    pub msa: MSA,
    #[doc = "0x04 - I2C Master Control/Status"]
    pub mcs: MCS,
    #[doc = "0x08 - I2C Master Data"]
    pub mdr: MDR,
    #[doc = "0x0c - I2C Master Timer Period"]
    pub mtpr: MTPR,
    #[doc = "0x10 - I2C Master Interrupt Mask"]
    pub mimr: MIMR,
    #[doc = "0x14 - I2C Master Raw Interrupt Status"]
    pub mris: MRIS,
    #[doc = "0x18 - I2C Master Masked Interrupt Status"]
    pub mmis: MMIS,
    #[doc = "0x1c - I2C Master Interrupt Clear"]
    pub micr: MICR,
    #[doc = "0x20 - I2C Master Configuration"]
    pub mcr: MCR,
    #[doc = "0x24 - I2C Master Clock Low Timeout Count"]
    pub mclkocnt: MCLKOCNT,
    _reserved0: [u8; 4usize],
    #[doc = "0x2c - I2C Master Bus Monitor"]
    pub mbmon: MBMON,
    _reserved1: [u8; 8usize],
    #[doc = "0x38 - I2C Master Configuration 2"]
    pub mcr2: MCR2,
    _reserved2: [u8; 1988usize],
    #[doc = "0x800 - I2C Slave Own Address"]
    pub soar: SOAR,
    #[doc = "0x804 - I2C Slave Control/Status"]
    pub scsr: SCSR,
    #[doc = "0x808 - I2C Slave Data"]
    pub sdr: SDR,
    #[doc = "0x80c - I2C Slave Interrupt Mask"]
    pub simr: SIMR,
    #[doc = "0x810 - I2C Slave Raw Interrupt Status"]
    pub sris: SRIS,
    #[doc = "0x814 - I2C Slave Masked Interrupt Status"]
    pub smis: SMIS,
    #[doc = "0x818 - I2C Slave Interrupt Clear"]
    pub sicr: SICR,
    #[doc = "0x81c - I2C Slave Own Address 2"]
    pub soar2: SOAR2,
    #[doc = "0x820 - I2C Slave ACK Control"]
    pub sackctl: SACKCTL,
    _reserved3: [u8; 1948usize],
    #[doc = "0xfc0 - I2C Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - I2C Peripheral Configuration"]
    pub pc: PC,
}
#[doc = "I2C Master Slave Address"]
pub struct MSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Slave Address"]
pub mod msa;
#[doc = "I2C Master Control/Status"]
pub struct MCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C Master Data"]
pub struct MDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Data"]
pub mod mdr;
#[doc = "I2C Master Timer Period"]
pub struct MTPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "I2C Master Interrupt Mask"]
pub struct MIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Interrupt Mask"]
pub mod mimr;
#[doc = "I2C Master Raw Interrupt Status"]
pub struct MRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Raw Interrupt Status"]
pub mod mris;
#[doc = "I2C Master Masked Interrupt Status"]
pub struct MMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Masked Interrupt Status"]
pub mod mmis;
#[doc = "I2C Master Interrupt Clear"]
pub struct MICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Interrupt Clear"]
pub mod micr;
#[doc = "I2C Master Configuration"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Configuration"]
pub mod mcr;
#[doc = "I2C Master Clock Low Timeout Count"]
pub struct MCLKOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Clock Low Timeout Count"]
pub mod mclkocnt;
#[doc = "I2C Master Bus Monitor"]
pub struct MBMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Bus Monitor"]
pub mod mbmon;
#[doc = "I2C Master Configuration 2"]
pub struct MCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Configuration 2"]
pub mod mcr2;
#[doc = "I2C Slave Own Address"]
pub struct SOAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "I2C Slave Control/Status"]
pub struct SCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C Slave Data"]
pub struct SDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Data"]
pub mod sdr;
#[doc = "I2C Slave Interrupt Mask"]
pub struct SIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Interrupt Mask"]
pub mod simr;
#[doc = "I2C Slave Raw Interrupt Status"]
pub struct SRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Raw Interrupt Status"]
pub mod sris;
#[doc = "I2C Slave Masked Interrupt Status"]
pub struct SMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Masked Interrupt Status"]
pub mod smis;
#[doc = "I2C Slave Interrupt Clear"]
pub struct SICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Interrupt Clear"]
pub mod sicr;
#[doc = "I2C Slave Own Address 2"]
pub struct SOAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Own Address 2"]
pub mod soar2;
#[doc = "I2C Slave ACK Control"]
pub struct SACKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave ACK Control"]
pub mod sackctl;
#[doc = "I2C Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Peripheral Properties"]
pub mod pp;
#[doc = "I2C Peripheral Configuration"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Peripheral Configuration"]
pub mod pc;
