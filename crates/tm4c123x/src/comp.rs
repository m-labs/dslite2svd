#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Masked Interrupt Status"]
    pub acmis: ACMIS,
    #[doc = "0x04 - Analog Comparator Raw Interrupt Status"]
    pub acris: ACRIS,
    #[doc = "0x08 - Analog Comparator Interrupt Enable"]
    pub acinten: ACINTEN,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Analog Comparator Reference Voltage Control"]
    pub acrefctl: ACREFCTL,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - Analog Comparator Status 0"]
    pub acstat0: ACSTAT0,
    #[doc = "0x24 - Analog Comparator Control 0"]
    pub acctl0: ACCTL0,
    _reserved6: [u8; 24usize],
    #[doc = "0x40 - Analog Comparator Status 1"]
    pub acstat1: ACSTAT1,
    #[doc = "0x44 - Analog Comparator Control 1"]
    pub acctl1: ACCTL1,
    _reserved8: [u8; 3960usize],
    #[doc = "0xfc0 - Analog Comparator Peripheral Properties"]
    pub pp: PP,
}
#[doc = "Analog Comparator Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acmis](acmis) module"]
pub type ACMIS = crate::Reg<u32, _ACMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACMIS;
#[doc = "`read()` method returns [acmis::R](acmis::R) reader structure"]
impl crate::Readable for ACMIS {}
#[doc = "`write(|w| ..)` method takes [acmis::W](acmis::W) writer structure"]
impl crate::Writable for ACMIS {}
#[doc = "Analog Comparator Masked Interrupt Status"]
pub mod acmis;
#[doc = "Analog Comparator Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acris](acris) module"]
pub type ACRIS = crate::Reg<u32, _ACRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACRIS;
#[doc = "`read()` method returns [acris::R](acris::R) reader structure"]
impl crate::Readable for ACRIS {}
#[doc = "Analog Comparator Raw Interrupt Status"]
pub mod acris;
#[doc = "Analog Comparator Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acinten](acinten) module"]
pub type ACINTEN = crate::Reg<u32, _ACINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACINTEN;
#[doc = "`read()` method returns [acinten::R](acinten::R) reader structure"]
impl crate::Readable for ACINTEN {}
#[doc = "`write(|w| ..)` method takes [acinten::W](acinten::W) writer structure"]
impl crate::Writable for ACINTEN {}
#[doc = "Analog Comparator Interrupt Enable"]
pub mod acinten;
#[doc = "Analog Comparator Reference Voltage Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acrefctl](acrefctl) module"]
pub type ACREFCTL = crate::Reg<u32, _ACREFCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACREFCTL;
#[doc = "`read()` method returns [acrefctl::R](acrefctl::R) reader structure"]
impl crate::Readable for ACREFCTL {}
#[doc = "`write(|w| ..)` method takes [acrefctl::W](acrefctl::W) writer structure"]
impl crate::Writable for ACREFCTL {}
#[doc = "Analog Comparator Reference Voltage Control"]
pub mod acrefctl;
#[doc = "Analog Comparator Status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acstat0](acstat0) module"]
pub type ACSTAT0 = crate::Reg<u32, _ACSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACSTAT0;
#[doc = "`read()` method returns [acstat0::R](acstat0::R) reader structure"]
impl crate::Readable for ACSTAT0 {}
#[doc = "Analog Comparator Status 0"]
pub mod acstat0;
#[doc = "Analog Comparator Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acctl0](acctl0) module"]
pub type ACCTL0 = crate::Reg<u32, _ACCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCTL0;
#[doc = "`read()` method returns [acctl0::R](acctl0::R) reader structure"]
impl crate::Readable for ACCTL0 {}
#[doc = "`write(|w| ..)` method takes [acctl0::W](acctl0::W) writer structure"]
impl crate::Writable for ACCTL0 {}
#[doc = "Analog Comparator Control 0"]
pub mod acctl0;
#[doc = "Analog Comparator Status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acstat1](acstat1) module"]
pub type ACSTAT1 = crate::Reg<u32, _ACSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACSTAT1;
#[doc = "`read()` method returns [acstat1::R](acstat1::R) reader structure"]
impl crate::Readable for ACSTAT1 {}
#[doc = "Analog Comparator Status 1"]
pub mod acstat1;
#[doc = "Analog Comparator Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acctl1](acctl1) module"]
pub type ACCTL1 = crate::Reg<u32, _ACCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCTL1;
#[doc = "`read()` method returns [acctl1::R](acctl1::R) reader structure"]
impl crate::Readable for ACCTL1 {}
#[doc = "`write(|w| ..)` method takes [acctl1::W](acctl1::W) writer structure"]
impl crate::Writable for ACCTL1 {}
#[doc = "Analog Comparator Control 1"]
pub mod acctl1;
#[doc = "Analog Comparator Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "Analog Comparator Peripheral Properties"]
pub mod pp;
