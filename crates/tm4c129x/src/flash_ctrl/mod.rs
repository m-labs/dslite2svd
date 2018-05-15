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
    _reserved2: [u8; 8usize],
    #[doc = "0x3c - Flash Program/Erase Key"]
    pub flpekey: FLPEKEY,
    _reserved3: [u8; 192usize],
    #[doc = "0x100 - Flash Write Buffer n"]
    pub fwbn: FWBN,
    _reserved4: [u8; 3772usize],
    #[doc = "0xfc0 - Flash Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - SRAM Size"]
    pub ssize: SSIZE,
    #[doc = "0xfc8 - Flash Configuration Register"]
    pub conf: CONF,
    #[doc = "0xfcc - ROM Software Map"]
    pub romswmap: ROMSWMAP,
    #[doc = "0xfd0 - Flash DMA Address Size"]
    pub dmasz: DMASZ,
    #[doc = "0xfd4 - Flash DMA Starting Address"]
    pub dmast: DMAST,
    _reserved5: [u8; 252usize],
    #[doc = "0x10d4 - Reset Vector Pointer"]
    pub rvp: RVP,
    _reserved6: [u8; 248usize],
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
    #[doc = "0x1210 - Flash Memory Protection Read Enable 4"]
    pub fmpre4: FMPRE4,
    #[doc = "0x1214 - Flash Memory Protection Read Enable 5"]
    pub fmpre5: FMPRE5,
    #[doc = "0x1218 - Flash Memory Protection Read Enable 6"]
    pub fmpre6: FMPRE6,
    #[doc = "0x121c - Flash Memory Protection Read Enable 7"]
    pub fmpre7: FMPRE7,
    #[doc = "0x1220 - Flash Memory Protection Read Enable 8"]
    pub fmpre8: FMPRE8,
    #[doc = "0x1224 - Flash Memory Protection Read Enable 9"]
    pub fmpre9: FMPRE9,
    #[doc = "0x1228 - Flash Memory Protection Read Enable 10"]
    pub fmpre10: FMPRE10,
    #[doc = "0x122c - Flash Memory Protection Read Enable 11"]
    pub fmpre11: FMPRE11,
    #[doc = "0x1230 - Flash Memory Protection Read Enable 12"]
    pub fmpre12: FMPRE12,
    #[doc = "0x1234 - Flash Memory Protection Read Enable 13"]
    pub fmpre13: FMPRE13,
    #[doc = "0x1238 - Flash Memory Protection Read Enable 14"]
    pub fmpre14: FMPRE14,
    #[doc = "0x123c - Flash Memory Protection Read Enable 15"]
    pub fmpre15: FMPRE15,
    _reserved9: [u8; 448usize],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: FMPPE0,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: FMPPE1,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: FMPPE2,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: FMPPE3,
    #[doc = "0x1410 - Flash Memory Protection Program Enable 4"]
    pub fmppe4: FMPPE4,
    #[doc = "0x1414 - Flash Memory Protection Program Enable 5"]
    pub fmppe5: FMPPE5,
    #[doc = "0x1418 - Flash Memory Protection Program Enable 6"]
    pub fmppe6: FMPPE6,
    #[doc = "0x141c - Flash Memory Protection Program Enable 7"]
    pub fmppe7: FMPPE7,
    #[doc = "0x1420 - Flash Memory Protection Program Enable 8"]
    pub fmppe8: FMPPE8,
    #[doc = "0x1424 - Flash Memory Protection Program Enable 9"]
    pub fmppe9: FMPPE9,
    #[doc = "0x1428 - Flash Memory Protection Program Enable 10"]
    pub fmppe10: FMPPE10,
    #[doc = "0x142c - Flash Memory Protection Program Enable 11"]
    pub fmppe11: FMPPE11,
    #[doc = "0x1430 - Flash Memory Protection Program Enable 12"]
    pub fmppe12: FMPPE12,
    #[doc = "0x1434 - Flash Memory Protection Program Enable 13"]
    pub fmppe13: FMPPE13,
    #[doc = "0x1438 - Flash Memory Protection Program Enable 14"]
    pub fmppe14: FMPPE14,
    #[doc = "0x143c - Flash Memory Protection Program Enable 15"]
    pub fmppe15: FMPPE15,
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
#[doc = "Flash Program/Erase Key"]
pub struct FLPEKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Program/Erase Key"]
pub mod flpekey;
#[doc = "Flash Write Buffer n"]
pub struct FWBN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Buffer n"]
pub mod fwbn;
#[doc = "Flash Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Peripheral Properties"]
pub mod pp;
#[doc = "SRAM Size"]
pub struct SSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "Flash Configuration Register"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Configuration Register"]
pub mod conf;
#[doc = "ROM Software Map"]
pub struct ROMSWMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "Flash DMA Address Size"]
pub struct DMASZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash DMA Address Size"]
pub mod dmasz;
#[doc = "Flash DMA Starting Address"]
pub struct DMAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash DMA Starting Address"]
pub mod dmast;
#[doc = "Reset Vector Pointer"]
pub struct RVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Vector Pointer"]
pub mod rvp;
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
#[doc = "Flash Memory Protection Read Enable 4"]
pub struct FMPRE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 4"]
pub mod fmpre4;
#[doc = "Flash Memory Protection Read Enable 5"]
pub struct FMPRE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 5"]
pub mod fmpre5;
#[doc = "Flash Memory Protection Read Enable 6"]
pub struct FMPRE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 6"]
pub mod fmpre6;
#[doc = "Flash Memory Protection Read Enable 7"]
pub struct FMPRE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 7"]
pub mod fmpre7;
#[doc = "Flash Memory Protection Read Enable 8"]
pub struct FMPRE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 8"]
pub mod fmpre8;
#[doc = "Flash Memory Protection Read Enable 9"]
pub struct FMPRE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 9"]
pub mod fmpre9;
#[doc = "Flash Memory Protection Read Enable 10"]
pub struct FMPRE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 10"]
pub mod fmpre10;
#[doc = "Flash Memory Protection Read Enable 11"]
pub struct FMPRE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 11"]
pub mod fmpre11;
#[doc = "Flash Memory Protection Read Enable 12"]
pub struct FMPRE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 12"]
pub mod fmpre12;
#[doc = "Flash Memory Protection Read Enable 13"]
pub struct FMPRE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 13"]
pub mod fmpre13;
#[doc = "Flash Memory Protection Read Enable 14"]
pub struct FMPRE14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 14"]
pub mod fmpre14;
#[doc = "Flash Memory Protection Read Enable 15"]
pub struct FMPRE15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Read Enable 15"]
pub mod fmpre15;
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
#[doc = "Flash Memory Protection Program Enable 4"]
pub struct FMPPE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 4"]
pub mod fmppe4;
#[doc = "Flash Memory Protection Program Enable 5"]
pub struct FMPPE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 5"]
pub mod fmppe5;
#[doc = "Flash Memory Protection Program Enable 6"]
pub struct FMPPE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 6"]
pub mod fmppe6;
#[doc = "Flash Memory Protection Program Enable 7"]
pub struct FMPPE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 7"]
pub mod fmppe7;
#[doc = "Flash Memory Protection Program Enable 8"]
pub struct FMPPE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 8"]
pub mod fmppe8;
#[doc = "Flash Memory Protection Program Enable 9"]
pub struct FMPPE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 9"]
pub mod fmppe9;
#[doc = "Flash Memory Protection Program Enable 10"]
pub struct FMPPE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 10"]
pub mod fmppe10;
#[doc = "Flash Memory Protection Program Enable 11"]
pub struct FMPPE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 11"]
pub mod fmppe11;
#[doc = "Flash Memory Protection Program Enable 12"]
pub struct FMPPE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 12"]
pub mod fmppe12;
#[doc = "Flash Memory Protection Program Enable 13"]
pub struct FMPPE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 13"]
pub mod fmppe13;
#[doc = "Flash Memory Protection Program Enable 14"]
pub struct FMPPE14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 14"]
pub mod fmppe14;
#[doc = "Flash Memory Protection Program Enable 15"]
pub struct FMPPE15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Memory Protection Program Enable 15"]
pub mod fmppe15;
