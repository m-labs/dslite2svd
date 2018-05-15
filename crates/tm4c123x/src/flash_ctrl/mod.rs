#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Memory Address"]
    pub fma: FMA,
    #[doc = "0x04 - Flash Memory Data"]
    pub fmd: FMD,
    #[doc = "0x08 - Flash Memory Control"]
    pub fmc: FMC,
    #[doc = "0x0c - Flash Controller Raw Interrupt Status"]
    pub fcris: FCRIS,
    #[doc = "0x10 - Flash Controller Interrupt Mask"]
    pub fcim: FCIM,
    #[doc = "0x14 - Flash Controller Masked Interrupt Status and Clear"]
    pub fcmisc: FCMISC,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Flash Memory Control 2"]
    pub fmc2: FMC2,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Flash Write Buffer Valid"]
    pub fwbval: FWBVAL,
    _reserved2: [u8; 204usize],
    #[doc = "0x100 - Flash Write Buffer"]
    pub fwbn: [FWBN; 32],
    _reserved3: [u8; 3648usize],
    #[doc = "0xfc0 - Flash Size"]
    pub fsize: FSIZE,
    #[doc = "0xfc4 - SRAM Size"]
    pub ssize: SSIZE,
    _reserved4: [u8; 4usize],
    #[doc = "0xfcc - ROM Software Map"]
    pub romswmap: ROMSWMAP,
    _reserved5: [u8; 288usize],
    #[doc = "0x10f0 - ROM Control"]
    pub rmctl: RMCTL,
    _reserved6: [u8; 220usize],
    #[doc = "0x11d0 - Boot Configuration"]
    pub bootcfg: BOOTCFG,
    _reserved7: [u8; 12usize],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: USERREG0,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: USERREG1,
    #[doc = "0x11e8 - User Register 2"]
    pub userreg2: USERREG2,
    #[doc = "0x11ec - User Register 3"]
    pub userreg3: USERREG3,
    _reserved8: [u8; 16usize],
    #[doc = "0x1200 - Flash Memory Protection Read Enable 0"]
    pub fmpre0: FMPRE0,
    #[doc = "0x1204 - Flash Memory Protection Read Enable 1"]
    pub fmpre1: FMPRE1,
    #[doc = "0x1208 - Flash Memory Protection Read Enable 2"]
    pub fmpre2: FMPRE2,
    #[doc = "0x120c - Flash Memory Protection Read Enable 3"]
    pub fmpre3: FMPRE3,
    _reserved9: [u8; 496usize],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: FMPPE0,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: FMPPE1,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: FMPPE2,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: FMPPE3,
}
#[doc = "Flash Memory Address"]
pub struct FMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Address"]
pub mod fma;
#[doc = "Flash Memory Data"]
pub struct FMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Data"]
pub mod fmd;
#[doc = "Flash Memory Control"]
pub struct FMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Control"]
pub mod fmc;
#[doc = "Flash Controller Raw Interrupt Status"]
pub struct FCRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Raw Interrupt Status"]
pub mod fcris;
#[doc = "Flash Controller Interrupt Mask"]
pub struct FCIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Interrupt Mask"]
pub mod fcim;
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub struct FCMISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub mod fcmisc;
#[doc = "Flash Memory Control 2"]
pub struct FMC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Control 2"]
pub mod fmc2;
#[doc = "Flash Write Buffer Valid"]
pub struct FWBVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Buffer Valid"]
pub mod fwbval;
#[doc = "Flash Write Buffer"]
pub struct FWBN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Buffer"]
pub mod fwbn;
#[doc = "Flash Size"]
pub struct FSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Size"]
pub mod fsize;
#[doc = "SRAM Size"]
pub struct SSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "ROM Software Map"]
pub struct ROMSWMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "ROM Control"]
pub struct RMCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Control"]
pub mod rmctl;
#[doc = "Boot Configuration"]
pub struct BOOTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boot Configuration"]
pub mod bootcfg;
#[doc = "User Register 0"]
pub struct USERREG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Register 0"]
pub mod userreg0;
#[doc = "User Register 1"]
pub struct USERREG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Register 1"]
pub mod userreg1;
#[doc = "User Register 2"]
pub struct USERREG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Register 2"]
pub mod userreg2;
#[doc = "User Register 3"]
pub struct USERREG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Register 3"]
pub mod userreg3;
#[doc = "Flash Memory Protection Read Enable 0"]
pub struct FMPRE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 0"]
pub mod fmpre0;
#[doc = "Flash Memory Protection Read Enable 1"]
pub struct FMPRE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 1"]
pub mod fmpre1;
#[doc = "Flash Memory Protection Read Enable 2"]
pub struct FMPRE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 2"]
pub mod fmpre2;
#[doc = "Flash Memory Protection Read Enable 3"]
pub struct FMPRE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 3"]
pub mod fmpre3;
#[doc = "Flash Memory Protection Program Enable 0"]
pub struct FMPPE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 0"]
pub mod fmppe0;
#[doc = "Flash Memory Protection Program Enable 1"]
pub struct FMPPE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 1"]
pub mod fmppe1;
#[doc = "Flash Memory Protection Program Enable 2"]
pub struct FMPPE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 2"]
pub mod fmppe2;
#[doc = "Flash Memory Protection Program Enable 3"]
pub struct FMPPE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 3"]
pub mod fmppe3;
