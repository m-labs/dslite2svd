#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - CRC Control"]
    pub crcctrl: CRCCTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x410 - CRC SEED/Context"]
    pub crcseed: CRCSEED,
    #[doc = "0x414 - CRC Data Input"]
    pub crcdin: CRCDIN,
    #[doc = "0x418 - CRC Post Processing Result"]
    pub crcrsltpp: CRCRSLTPP,
}
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcctrl](crcctrl) module"]
pub type CRCCTRL = crate::Reg<u32, _CRCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCTRL;
#[doc = "`read()` method returns [crcctrl::R](crcctrl::R) reader structure"]
impl crate::Readable for CRCCTRL {}
#[doc = "`write(|w| ..)` method takes [crcctrl::W](crcctrl::W) writer structure"]
impl crate::Writable for CRCCTRL {}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC SEED/Context\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcseed](crcseed) module"]
pub type CRCSEED = crate::Reg<u32, _CRCSEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCSEED;
#[doc = "`read()` method returns [crcseed::R](crcseed::R) reader structure"]
impl crate::Readable for CRCSEED {}
#[doc = "`write(|w| ..)` method takes [crcseed::W](crcseed::W) writer structure"]
impl crate::Writable for CRCSEED {}
#[doc = "CRC SEED/Context"]
pub mod crcseed;
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdin](crcdin) module"]
pub type CRCDIN = crate::Reg<u32, _CRCDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDIN;
#[doc = "`read()` method returns [crcdin::R](crcdin::R) reader structure"]
impl crate::Readable for CRCDIN {}
#[doc = "`write(|w| ..)` method takes [crcdin::W](crcdin::W) writer structure"]
impl crate::Writable for CRCDIN {}
#[doc = "CRC Data Input"]
pub mod crcdin;
#[doc = "CRC Post Processing Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcrsltpp](crcrsltpp) module"]
pub type CRCRSLTPP = crate::Reg<u32, _CRCRSLTPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCRSLTPP;
#[doc = "`read()` method returns [crcrsltpp::R](crcrsltpp::R) reader structure"]
impl crate::Readable for CRCRSLTPP {}
#[doc = "CRC Post Processing Result"]
pub mod crcrsltpp;
