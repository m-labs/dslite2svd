#[doc = r"Register block"]
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
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Flash Memory Control 2"]
    pub fmc2: FMC2,
    _reserved7: [u8; 12usize],
    #[doc = "0x30 - Flash Write Buffer Valid"]
    pub fwbval: FWBVAL,
    _reserved8: [u8; 8usize],
    #[doc = "0x3c - Flash Program/Erase Key"]
    pub flpekey: FLPEKEY,
    _reserved9: [u8; 192usize],
    #[doc = "0x100 - Flash Write Buffer n"]
    pub fwbn: FWBN,
    _reserved10: [u8; 3772usize],
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
    _reserved16: [u8; 252usize],
    #[doc = "0x10d4 - Reset Vector Pointer"]
    pub rvp: RVP,
    _reserved17: [u8; 248usize],
    #[doc = "0x11d0 - Boot Configuration"]
    pub bootcfg: BOOTCFG,
    _reserved18: [u8; 12usize],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: USERREG0,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: USERREG1,
    #[doc = "0x11e8 - User Register 2"]
    pub userreg2: USERREG2,
    #[doc = "0x11ec - User Register 3"]
    pub userreg3: USERREG3,
    _reserved22: [u8; 16usize],
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
    _reserved38: [u8; 448usize],
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
#[doc = "Flash Memory Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fma](fma) module"]
pub type FMA = crate::Reg<u32, _FMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMA;
#[doc = "`read()` method returns [fma::R](fma::R) reader structure"]
impl crate::Readable for FMA {}
#[doc = "`write(|w| ..)` method takes [fma::W](fma::W) writer structure"]
impl crate::Writable for FMA {}
#[doc = "Flash Memory Address"]
pub mod fma;
#[doc = "Flash Memory Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmd](fmd) module"]
pub type FMD = crate::Reg<u32, _FMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD;
#[doc = "`read()` method returns [fmd::R](fmd::R) reader structure"]
impl crate::Readable for FMD {}
#[doc = "`write(|w| ..)` method takes [fmd::W](fmd::W) writer structure"]
impl crate::Writable for FMD {}
#[doc = "Flash Memory Data"]
pub mod fmd;
#[doc = "Flash Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc](fmc) module"]
pub type FMC = crate::Reg<u32, _FMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC;
#[doc = "`read()` method returns [fmc::R](fmc::R) reader structure"]
impl crate::Readable for FMC {}
#[doc = "`write(|w| ..)` method takes [fmc::W](fmc::W) writer structure"]
impl crate::Writable for FMC {}
#[doc = "Flash Memory Control"]
pub mod fmc;
#[doc = "Flash Controller Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcris](fcris) module"]
pub type FCRIS = crate::Reg<u32, _FCRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCRIS;
#[doc = "`read()` method returns [fcris::R](fcris::R) reader structure"]
impl crate::Readable for FCRIS {}
#[doc = "Flash Controller Raw Interrupt Status"]
pub mod fcris;
#[doc = "Flash Controller Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcim](fcim) module"]
pub type FCIM = crate::Reg<u32, _FCIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCIM;
#[doc = "`read()` method returns [fcim::R](fcim::R) reader structure"]
impl crate::Readable for FCIM {}
#[doc = "`write(|w| ..)` method takes [fcim::W](fcim::W) writer structure"]
impl crate::Writable for FCIM {}
#[doc = "Flash Controller Interrupt Mask"]
pub mod fcim;
#[doc = "Flash Controller Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcmisc](fcmisc) module"]
pub type FCMISC = crate::Reg<u32, _FCMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCMISC;
#[doc = "`read()` method returns [fcmisc::R](fcmisc::R) reader structure"]
impl crate::Readable for FCMISC {}
#[doc = "`write(|w| ..)` method takes [fcmisc::W](fcmisc::W) writer structure"]
impl crate::Writable for FCMISC {}
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub mod fcmisc;
#[doc = "Flash Memory Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc2](fmc2) module"]
pub type FMC2 = crate::Reg<u32, _FMC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC2;
#[doc = "`read()` method returns [fmc2::R](fmc2::R) reader structure"]
impl crate::Readable for FMC2 {}
#[doc = "`write(|w| ..)` method takes [fmc2::W](fmc2::W) writer structure"]
impl crate::Writable for FMC2 {}
#[doc = "Flash Memory Control 2"]
pub mod fmc2;
#[doc = "Flash Write Buffer Valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbval](fwbval) module"]
pub type FWBVAL = crate::Reg<u32, _FWBVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWBVAL;
#[doc = "`read()` method returns [fwbval::R](fwbval::R) reader structure"]
impl crate::Readable for FWBVAL {}
#[doc = "`write(|w| ..)` method takes [fwbval::W](fwbval::W) writer structure"]
impl crate::Writable for FWBVAL {}
#[doc = "Flash Write Buffer Valid"]
pub mod fwbval;
#[doc = "Flash Program/Erase Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flpekey](flpekey) module"]
pub type FLPEKEY = crate::Reg<u32, _FLPEKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLPEKEY;
#[doc = "`read()` method returns [flpekey::R](flpekey::R) reader structure"]
impl crate::Readable for FLPEKEY {}
#[doc = "`write(|w| ..)` method takes [flpekey::W](flpekey::W) writer structure"]
impl crate::Writable for FLPEKEY {}
#[doc = "Flash Program/Erase Key"]
pub mod flpekey;
#[doc = "Flash Write Buffer n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbn](fwbn) module"]
pub type FWBN = crate::Reg<u32, _FWBN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWBN;
#[doc = "`read()` method returns [fwbn::R](fwbn::R) reader structure"]
impl crate::Readable for FWBN {}
#[doc = "`write(|w| ..)` method takes [fwbn::W](fwbn::W) writer structure"]
impl crate::Writable for FWBN {}
#[doc = "Flash Write Buffer n"]
pub mod fwbn;
#[doc = "Flash Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "Flash Peripheral Properties"]
pub mod pp;
#[doc = "SRAM Size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssize](ssize) module"]
pub type SSIZE = crate::Reg<u32, _SSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZE;
#[doc = "`read()` method returns [ssize::R](ssize::R) reader structure"]
impl crate::Readable for SSIZE {}
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "Flash Configuration Register"]
pub mod conf;
#[doc = "ROM Software Map\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [romswmap](romswmap) module"]
pub type ROMSWMAP = crate::Reg<u32, _ROMSWMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMSWMAP;
#[doc = "`read()` method returns [romswmap::R](romswmap::R) reader structure"]
impl crate::Readable for ROMSWMAP {}
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "Flash DMA Address Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasz](dmasz) module"]
pub type DMASZ = crate::Reg<u32, _DMASZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASZ;
#[doc = "`read()` method returns [dmasz::R](dmasz::R) reader structure"]
impl crate::Readable for DMASZ {}
#[doc = "`write(|w| ..)` method takes [dmasz::W](dmasz::W) writer structure"]
impl crate::Writable for DMASZ {}
#[doc = "Flash DMA Address Size"]
pub mod dmasz;
#[doc = "Flash DMA Starting Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmast](dmast) module"]
pub type DMAST = crate::Reg<u32, _DMAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAST;
#[doc = "`read()` method returns [dmast::R](dmast::R) reader structure"]
impl crate::Readable for DMAST {}
#[doc = "`write(|w| ..)` method takes [dmast::W](dmast::W) writer structure"]
impl crate::Writable for DMAST {}
#[doc = "Flash DMA Starting Address"]
pub mod dmast;
#[doc = "Reset Vector Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rvp](rvp) module"]
pub type RVP = crate::Reg<u32, _RVP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RVP;
#[doc = "`read()` method returns [rvp::R](rvp::R) reader structure"]
impl crate::Readable for RVP {}
#[doc = "`write(|w| ..)` method takes [rvp::W](rvp::W) writer structure"]
impl crate::Writable for RVP {}
#[doc = "Reset Vector Pointer"]
pub mod rvp;
#[doc = "Boot Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootcfg](bootcfg) module"]
pub type BOOTCFG = crate::Reg<u32, _BOOTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOTCFG;
#[doc = "`read()` method returns [bootcfg::R](bootcfg::R) reader structure"]
impl crate::Readable for BOOTCFG {}
#[doc = "Boot Configuration"]
pub mod bootcfg;
#[doc = "User Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg0](userreg0) module"]
pub type USERREG0 = crate::Reg<u32, _USERREG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG0;
#[doc = "`read()` method returns [userreg0::R](userreg0::R) reader structure"]
impl crate::Readable for USERREG0 {}
#[doc = "`write(|w| ..)` method takes [userreg0::W](userreg0::W) writer structure"]
impl crate::Writable for USERREG0 {}
#[doc = "User Register 0"]
pub mod userreg0;
#[doc = "User Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg1](userreg1) module"]
pub type USERREG1 = crate::Reg<u32, _USERREG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG1;
#[doc = "`read()` method returns [userreg1::R](userreg1::R) reader structure"]
impl crate::Readable for USERREG1 {}
#[doc = "`write(|w| ..)` method takes [userreg1::W](userreg1::W) writer structure"]
impl crate::Writable for USERREG1 {}
#[doc = "User Register 1"]
pub mod userreg1;
#[doc = "User Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg2](userreg2) module"]
pub type USERREG2 = crate::Reg<u32, _USERREG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG2;
#[doc = "`read()` method returns [userreg2::R](userreg2::R) reader structure"]
impl crate::Readable for USERREG2 {}
#[doc = "`write(|w| ..)` method takes [userreg2::W](userreg2::W) writer structure"]
impl crate::Writable for USERREG2 {}
#[doc = "User Register 2"]
pub mod userreg2;
#[doc = "User Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg3](userreg3) module"]
pub type USERREG3 = crate::Reg<u32, _USERREG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG3;
#[doc = "`read()` method returns [userreg3::R](userreg3::R) reader structure"]
impl crate::Readable for USERREG3 {}
#[doc = "`write(|w| ..)` method takes [userreg3::W](userreg3::W) writer structure"]
impl crate::Writable for USERREG3 {}
#[doc = "User Register 3"]
pub mod userreg3;
#[doc = "Flash Memory Protection Read Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre0](fmpre0) module"]
pub type FMPRE0 = crate::Reg<u32, _FMPRE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE0;
#[doc = "`read()` method returns [fmpre0::R](fmpre0::R) reader structure"]
impl crate::Readable for FMPRE0 {}
#[doc = "`write(|w| ..)` method takes [fmpre0::W](fmpre0::W) writer structure"]
impl crate::Writable for FMPRE0 {}
#[doc = "Flash Memory Protection Read Enable 0"]
pub mod fmpre0;
#[doc = "Flash Memory Protection Read Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre1](fmpre1) module"]
pub type FMPRE1 = crate::Reg<u32, _FMPRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE1;
#[doc = "`read()` method returns [fmpre1::R](fmpre1::R) reader structure"]
impl crate::Readable for FMPRE1 {}
#[doc = "`write(|w| ..)` method takes [fmpre1::W](fmpre1::W) writer structure"]
impl crate::Writable for FMPRE1 {}
#[doc = "Flash Memory Protection Read Enable 1"]
pub mod fmpre1;
#[doc = "Flash Memory Protection Read Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre2](fmpre2) module"]
pub type FMPRE2 = crate::Reg<u32, _FMPRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE2;
#[doc = "`read()` method returns [fmpre2::R](fmpre2::R) reader structure"]
impl crate::Readable for FMPRE2 {}
#[doc = "`write(|w| ..)` method takes [fmpre2::W](fmpre2::W) writer structure"]
impl crate::Writable for FMPRE2 {}
#[doc = "Flash Memory Protection Read Enable 2"]
pub mod fmpre2;
#[doc = "Flash Memory Protection Read Enable 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre3](fmpre3) module"]
pub type FMPRE3 = crate::Reg<u32, _FMPRE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE3;
#[doc = "`read()` method returns [fmpre3::R](fmpre3::R) reader structure"]
impl crate::Readable for FMPRE3 {}
#[doc = "`write(|w| ..)` method takes [fmpre3::W](fmpre3::W) writer structure"]
impl crate::Writable for FMPRE3 {}
#[doc = "Flash Memory Protection Read Enable 3"]
pub mod fmpre3;
#[doc = "Flash Memory Protection Read Enable 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre4](fmpre4) module"]
pub type FMPRE4 = crate::Reg<u32, _FMPRE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE4;
#[doc = "`read()` method returns [fmpre4::R](fmpre4::R) reader structure"]
impl crate::Readable for FMPRE4 {}
#[doc = "`write(|w| ..)` method takes [fmpre4::W](fmpre4::W) writer structure"]
impl crate::Writable for FMPRE4 {}
#[doc = "Flash Memory Protection Read Enable 4"]
pub mod fmpre4;
#[doc = "Flash Memory Protection Read Enable 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre5](fmpre5) module"]
pub type FMPRE5 = crate::Reg<u32, _FMPRE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE5;
#[doc = "`read()` method returns [fmpre5::R](fmpre5::R) reader structure"]
impl crate::Readable for FMPRE5 {}
#[doc = "`write(|w| ..)` method takes [fmpre5::W](fmpre5::W) writer structure"]
impl crate::Writable for FMPRE5 {}
#[doc = "Flash Memory Protection Read Enable 5"]
pub mod fmpre5;
#[doc = "Flash Memory Protection Read Enable 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre6](fmpre6) module"]
pub type FMPRE6 = crate::Reg<u32, _FMPRE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE6;
#[doc = "`read()` method returns [fmpre6::R](fmpre6::R) reader structure"]
impl crate::Readable for FMPRE6 {}
#[doc = "`write(|w| ..)` method takes [fmpre6::W](fmpre6::W) writer structure"]
impl crate::Writable for FMPRE6 {}
#[doc = "Flash Memory Protection Read Enable 6"]
pub mod fmpre6;
#[doc = "Flash Memory Protection Read Enable 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre7](fmpre7) module"]
pub type FMPRE7 = crate::Reg<u32, _FMPRE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE7;
#[doc = "`read()` method returns [fmpre7::R](fmpre7::R) reader structure"]
impl crate::Readable for FMPRE7 {}
#[doc = "`write(|w| ..)` method takes [fmpre7::W](fmpre7::W) writer structure"]
impl crate::Writable for FMPRE7 {}
#[doc = "Flash Memory Protection Read Enable 7"]
pub mod fmpre7;
#[doc = "Flash Memory Protection Read Enable 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre8](fmpre8) module"]
pub type FMPRE8 = crate::Reg<u32, _FMPRE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE8;
#[doc = "`read()` method returns [fmpre8::R](fmpre8::R) reader structure"]
impl crate::Readable for FMPRE8 {}
#[doc = "`write(|w| ..)` method takes [fmpre8::W](fmpre8::W) writer structure"]
impl crate::Writable for FMPRE8 {}
#[doc = "Flash Memory Protection Read Enable 8"]
pub mod fmpre8;
#[doc = "Flash Memory Protection Read Enable 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre9](fmpre9) module"]
pub type FMPRE9 = crate::Reg<u32, _FMPRE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE9;
#[doc = "`read()` method returns [fmpre9::R](fmpre9::R) reader structure"]
impl crate::Readable for FMPRE9 {}
#[doc = "`write(|w| ..)` method takes [fmpre9::W](fmpre9::W) writer structure"]
impl crate::Writable for FMPRE9 {}
#[doc = "Flash Memory Protection Read Enable 9"]
pub mod fmpre9;
#[doc = "Flash Memory Protection Read Enable 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre10](fmpre10) module"]
pub type FMPRE10 = crate::Reg<u32, _FMPRE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE10;
#[doc = "`read()` method returns [fmpre10::R](fmpre10::R) reader structure"]
impl crate::Readable for FMPRE10 {}
#[doc = "`write(|w| ..)` method takes [fmpre10::W](fmpre10::W) writer structure"]
impl crate::Writable for FMPRE10 {}
#[doc = "Flash Memory Protection Read Enable 10"]
pub mod fmpre10;
#[doc = "Flash Memory Protection Read Enable 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre11](fmpre11) module"]
pub type FMPRE11 = crate::Reg<u32, _FMPRE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE11;
#[doc = "`read()` method returns [fmpre11::R](fmpre11::R) reader structure"]
impl crate::Readable for FMPRE11 {}
#[doc = "`write(|w| ..)` method takes [fmpre11::W](fmpre11::W) writer structure"]
impl crate::Writable for FMPRE11 {}
#[doc = "Flash Memory Protection Read Enable 11"]
pub mod fmpre11;
#[doc = "Flash Memory Protection Read Enable 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre12](fmpre12) module"]
pub type FMPRE12 = crate::Reg<u32, _FMPRE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE12;
#[doc = "`read()` method returns [fmpre12::R](fmpre12::R) reader structure"]
impl crate::Readable for FMPRE12 {}
#[doc = "`write(|w| ..)` method takes [fmpre12::W](fmpre12::W) writer structure"]
impl crate::Writable for FMPRE12 {}
#[doc = "Flash Memory Protection Read Enable 12"]
pub mod fmpre12;
#[doc = "Flash Memory Protection Read Enable 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre13](fmpre13) module"]
pub type FMPRE13 = crate::Reg<u32, _FMPRE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE13;
#[doc = "`read()` method returns [fmpre13::R](fmpre13::R) reader structure"]
impl crate::Readable for FMPRE13 {}
#[doc = "`write(|w| ..)` method takes [fmpre13::W](fmpre13::W) writer structure"]
impl crate::Writable for FMPRE13 {}
#[doc = "Flash Memory Protection Read Enable 13"]
pub mod fmpre13;
#[doc = "Flash Memory Protection Read Enable 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre14](fmpre14) module"]
pub type FMPRE14 = crate::Reg<u32, _FMPRE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE14;
#[doc = "`read()` method returns [fmpre14::R](fmpre14::R) reader structure"]
impl crate::Readable for FMPRE14 {}
#[doc = "`write(|w| ..)` method takes [fmpre14::W](fmpre14::W) writer structure"]
impl crate::Writable for FMPRE14 {}
#[doc = "Flash Memory Protection Read Enable 14"]
pub mod fmpre14;
#[doc = "Flash Memory Protection Read Enable 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre15](fmpre15) module"]
pub type FMPRE15 = crate::Reg<u32, _FMPRE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE15;
#[doc = "`read()` method returns [fmpre15::R](fmpre15::R) reader structure"]
impl crate::Readable for FMPRE15 {}
#[doc = "`write(|w| ..)` method takes [fmpre15::W](fmpre15::W) writer structure"]
impl crate::Writable for FMPRE15 {}
#[doc = "Flash Memory Protection Read Enable 15"]
pub mod fmpre15;
#[doc = "Flash Memory Protection Program Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe0](fmppe0) module"]
pub type FMPPE0 = crate::Reg<u32, _FMPPE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE0;
#[doc = "`read()` method returns [fmppe0::R](fmppe0::R) reader structure"]
impl crate::Readable for FMPPE0 {}
#[doc = "`write(|w| ..)` method takes [fmppe0::W](fmppe0::W) writer structure"]
impl crate::Writable for FMPPE0 {}
#[doc = "Flash Memory Protection Program Enable 0"]
pub mod fmppe0;
#[doc = "Flash Memory Protection Program Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe1](fmppe1) module"]
pub type FMPPE1 = crate::Reg<u32, _FMPPE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE1;
#[doc = "`read()` method returns [fmppe1::R](fmppe1::R) reader structure"]
impl crate::Readable for FMPPE1 {}
#[doc = "`write(|w| ..)` method takes [fmppe1::W](fmppe1::W) writer structure"]
impl crate::Writable for FMPPE1 {}
#[doc = "Flash Memory Protection Program Enable 1"]
pub mod fmppe1;
#[doc = "Flash Memory Protection Program Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe2](fmppe2) module"]
pub type FMPPE2 = crate::Reg<u32, _FMPPE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE2;
#[doc = "`read()` method returns [fmppe2::R](fmppe2::R) reader structure"]
impl crate::Readable for FMPPE2 {}
#[doc = "`write(|w| ..)` method takes [fmppe2::W](fmppe2::W) writer structure"]
impl crate::Writable for FMPPE2 {}
#[doc = "Flash Memory Protection Program Enable 2"]
pub mod fmppe2;
#[doc = "Flash Memory Protection Program Enable 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe3](fmppe3) module"]
pub type FMPPE3 = crate::Reg<u32, _FMPPE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE3;
#[doc = "`read()` method returns [fmppe3::R](fmppe3::R) reader structure"]
impl crate::Readable for FMPPE3 {}
#[doc = "`write(|w| ..)` method takes [fmppe3::W](fmppe3::W) writer structure"]
impl crate::Writable for FMPPE3 {}
#[doc = "Flash Memory Protection Program Enable 3"]
pub mod fmppe3;
#[doc = "Flash Memory Protection Program Enable 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe4](fmppe4) module"]
pub type FMPPE4 = crate::Reg<u32, _FMPPE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE4;
#[doc = "`read()` method returns [fmppe4::R](fmppe4::R) reader structure"]
impl crate::Readable for FMPPE4 {}
#[doc = "`write(|w| ..)` method takes [fmppe4::W](fmppe4::W) writer structure"]
impl crate::Writable for FMPPE4 {}
#[doc = "Flash Memory Protection Program Enable 4"]
pub mod fmppe4;
#[doc = "Flash Memory Protection Program Enable 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe5](fmppe5) module"]
pub type FMPPE5 = crate::Reg<u32, _FMPPE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE5;
#[doc = "`read()` method returns [fmppe5::R](fmppe5::R) reader structure"]
impl crate::Readable for FMPPE5 {}
#[doc = "`write(|w| ..)` method takes [fmppe5::W](fmppe5::W) writer structure"]
impl crate::Writable for FMPPE5 {}
#[doc = "Flash Memory Protection Program Enable 5"]
pub mod fmppe5;
#[doc = "Flash Memory Protection Program Enable 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe6](fmppe6) module"]
pub type FMPPE6 = crate::Reg<u32, _FMPPE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE6;
#[doc = "`read()` method returns [fmppe6::R](fmppe6::R) reader structure"]
impl crate::Readable for FMPPE6 {}
#[doc = "`write(|w| ..)` method takes [fmppe6::W](fmppe6::W) writer structure"]
impl crate::Writable for FMPPE6 {}
#[doc = "Flash Memory Protection Program Enable 6"]
pub mod fmppe6;
#[doc = "Flash Memory Protection Program Enable 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe7](fmppe7) module"]
pub type FMPPE7 = crate::Reg<u32, _FMPPE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE7;
#[doc = "`read()` method returns [fmppe7::R](fmppe7::R) reader structure"]
impl crate::Readable for FMPPE7 {}
#[doc = "`write(|w| ..)` method takes [fmppe7::W](fmppe7::W) writer structure"]
impl crate::Writable for FMPPE7 {}
#[doc = "Flash Memory Protection Program Enable 7"]
pub mod fmppe7;
#[doc = "Flash Memory Protection Program Enable 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe8](fmppe8) module"]
pub type FMPPE8 = crate::Reg<u32, _FMPPE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE8;
#[doc = "`read()` method returns [fmppe8::R](fmppe8::R) reader structure"]
impl crate::Readable for FMPPE8 {}
#[doc = "`write(|w| ..)` method takes [fmppe8::W](fmppe8::W) writer structure"]
impl crate::Writable for FMPPE8 {}
#[doc = "Flash Memory Protection Program Enable 8"]
pub mod fmppe8;
#[doc = "Flash Memory Protection Program Enable 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe9](fmppe9) module"]
pub type FMPPE9 = crate::Reg<u32, _FMPPE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE9;
#[doc = "`read()` method returns [fmppe9::R](fmppe9::R) reader structure"]
impl crate::Readable for FMPPE9 {}
#[doc = "`write(|w| ..)` method takes [fmppe9::W](fmppe9::W) writer structure"]
impl crate::Writable for FMPPE9 {}
#[doc = "Flash Memory Protection Program Enable 9"]
pub mod fmppe9;
#[doc = "Flash Memory Protection Program Enable 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe10](fmppe10) module"]
pub type FMPPE10 = crate::Reg<u32, _FMPPE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE10;
#[doc = "`read()` method returns [fmppe10::R](fmppe10::R) reader structure"]
impl crate::Readable for FMPPE10 {}
#[doc = "`write(|w| ..)` method takes [fmppe10::W](fmppe10::W) writer structure"]
impl crate::Writable for FMPPE10 {}
#[doc = "Flash Memory Protection Program Enable 10"]
pub mod fmppe10;
#[doc = "Flash Memory Protection Program Enable 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe11](fmppe11) module"]
pub type FMPPE11 = crate::Reg<u32, _FMPPE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE11;
#[doc = "`read()` method returns [fmppe11::R](fmppe11::R) reader structure"]
impl crate::Readable for FMPPE11 {}
#[doc = "`write(|w| ..)` method takes [fmppe11::W](fmppe11::W) writer structure"]
impl crate::Writable for FMPPE11 {}
#[doc = "Flash Memory Protection Program Enable 11"]
pub mod fmppe11;
#[doc = "Flash Memory Protection Program Enable 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe12](fmppe12) module"]
pub type FMPPE12 = crate::Reg<u32, _FMPPE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE12;
#[doc = "`read()` method returns [fmppe12::R](fmppe12::R) reader structure"]
impl crate::Readable for FMPPE12 {}
#[doc = "`write(|w| ..)` method takes [fmppe12::W](fmppe12::W) writer structure"]
impl crate::Writable for FMPPE12 {}
#[doc = "Flash Memory Protection Program Enable 12"]
pub mod fmppe12;
#[doc = "Flash Memory Protection Program Enable 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe13](fmppe13) module"]
pub type FMPPE13 = crate::Reg<u32, _FMPPE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE13;
#[doc = "`read()` method returns [fmppe13::R](fmppe13::R) reader structure"]
impl crate::Readable for FMPPE13 {}
#[doc = "`write(|w| ..)` method takes [fmppe13::W](fmppe13::W) writer structure"]
impl crate::Writable for FMPPE13 {}
#[doc = "Flash Memory Protection Program Enable 13"]
pub mod fmppe13;
#[doc = "Flash Memory Protection Program Enable 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe14](fmppe14) module"]
pub type FMPPE14 = crate::Reg<u32, _FMPPE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE14;
#[doc = "`read()` method returns [fmppe14::R](fmppe14::R) reader structure"]
impl crate::Readable for FMPPE14 {}
#[doc = "`write(|w| ..)` method takes [fmppe14::W](fmppe14::W) writer structure"]
impl crate::Writable for FMPPE14 {}
#[doc = "Flash Memory Protection Program Enable 14"]
pub mod fmppe14;
#[doc = "Flash Memory Protection Program Enable 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe15](fmppe15) module"]
pub type FMPPE15 = crate::Reg<u32, _FMPPE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE15;
#[doc = "`read()` method returns [fmppe15::R](fmppe15::R) reader structure"]
impl crate::Readable for FMPPE15 {}
#[doc = "`write(|w| ..)` method takes [fmppe15::W](fmppe15::W) writer structure"]
impl crate::Writable for FMPPE15 {}
#[doc = "Flash Memory Protection Program Enable 15"]
pub mod fmppe15;
