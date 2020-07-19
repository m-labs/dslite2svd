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
    _reserved8: [u8; 204usize],
    #[doc = "0x100 - Flash Write Buffer"]
    pub fwbn: [FWBN; 32],
    _reserved9: [u8; 3648usize],
    #[doc = "0xfc0 - Flash Size"]
    pub fsize: FSIZE,
    #[doc = "0xfc4 - SRAM Size"]
    pub ssize: SSIZE,
    _reserved11: [u8; 4usize],
    #[doc = "0xfcc - ROM Software Map"]
    pub romswmap: ROMSWMAP,
    _reserved12: [u8; 288usize],
    #[doc = "0x10f0 - ROM Control"]
    pub rmctl: RMCTL,
    _reserved13: [u8; 220usize],
    #[doc = "0x11d0 - Boot Configuration"]
    pub bootcfg: BOOTCFG,
    _reserved14: [u8; 12usize],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: USERREG0,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: USERREG1,
    #[doc = "0x11e8 - User Register 2"]
    pub userreg2: USERREG2,
    #[doc = "0x11ec - User Register 3"]
    pub userreg3: USERREG3,
    _reserved18: [u8; 16usize],
    #[doc = "0x1200 - Flash Memory Protection Read Enable 0"]
    pub fmpre0: FMPRE0,
    #[doc = "0x1204 - Flash Memory Protection Read Enable 1"]
    pub fmpre1: FMPRE1,
    #[doc = "0x1208 - Flash Memory Protection Read Enable 2"]
    pub fmpre2: FMPRE2,
    #[doc = "0x120c - Flash Memory Protection Read Enable 3"]
    pub fmpre3: FMPRE3,
    _reserved22: [u8; 496usize],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: FMPPE0,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: FMPPE1,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: FMPPE2,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: FMPPE3,
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
#[doc = "Flash Write Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbn](fwbn) module"]
pub type FWBN = crate::Reg<u32, _FWBN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWBN;
#[doc = "`read()` method returns [fwbn::R](fwbn::R) reader structure"]
impl crate::Readable for FWBN {}
#[doc = "`write(|w| ..)` method takes [fwbn::W](fwbn::W) writer structure"]
impl crate::Writable for FWBN {}
#[doc = "Flash Write Buffer"]
pub mod fwbn;
#[doc = "Flash Size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsize](fsize) module"]
pub type FSIZE = crate::Reg<u32, _FSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSIZE;
#[doc = "`read()` method returns [fsize::R](fsize::R) reader structure"]
impl crate::Readable for FSIZE {}
#[doc = "Flash Size"]
pub mod fsize;
#[doc = "SRAM Size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssize](ssize) module"]
pub type SSIZE = crate::Reg<u32, _SSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZE;
#[doc = "`read()` method returns [ssize::R](ssize::R) reader structure"]
impl crate::Readable for SSIZE {}
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "ROM Software Map\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [romswmap](romswmap) module"]
pub type ROMSWMAP = crate::Reg<u32, _ROMSWMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMSWMAP;
#[doc = "`read()` method returns [romswmap::R](romswmap::R) reader structure"]
impl crate::Readable for ROMSWMAP {}
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "ROM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmctl](rmctl) module"]
pub type RMCTL = crate::Reg<u32, _RMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMCTL;
#[doc = "`read()` method returns [rmctl::R](rmctl::R) reader structure"]
impl crate::Readable for RMCTL {}
#[doc = "`write(|w| ..)` method takes [rmctl::W](rmctl::W) writer structure"]
impl crate::Writable for RMCTL {}
#[doc = "ROM Control"]
pub mod rmctl;
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
