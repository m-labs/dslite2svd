#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Size Information"]
    pub eesize: EESIZE,
    #[doc = "0x04 - EEPROM Current Block"]
    pub eeblock: EEBLOCK,
    #[doc = "0x08 - EEPROM Current Offset"]
    pub eeoffset: EEOFFSET,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - EEPROM Read-Write"]
    pub eerdwr: EERDWR,
    #[doc = "0x14 - EEPROM Read-Write with Increment"]
    pub eerdwrinc: EERDWRINC,
    #[doc = "0x18 - EEPROM Done Status"]
    pub eedone: EEDONE,
    #[doc = "0x1c - EEPROM Support Control and Status"]
    pub eesupp: EESUPP,
    #[doc = "0x20 - EEPROM Unlock"]
    pub eeunlock: EEUNLOCK,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - EEPROM Protection"]
    pub eeprot: EEPROT,
    #[doc = "0x34 - EEPROM Password"]
    pub eepass0: EEPASS0,
    #[doc = "0x38 - EEPROM Password"]
    pub eepass1: EEPASS1,
    #[doc = "0x3c - EEPROM Password"]
    pub eepass2: EEPASS2,
    #[doc = "0x40 - EEPROM Interrupt"]
    pub eeint: EEINT,
    _reserved13: [u8; 12usize],
    #[doc = "0x50 - EEPROM Block Hide"]
    pub eehide: EEHIDE,
    _reserved14: [u8; 44usize],
    #[doc = "0x80 - EEPROM Debug Mass Erase"]
    pub eedbgme: EEDBGME,
    _reserved15: [u8; 3900usize],
    #[doc = "0xfc0 - EEPROM Peripheral Properties"]
    pub pp: PP,
}
#[doc = "EEPROM Size Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eesize](eesize) module"]
pub type EESIZE = crate::Reg<u32, _EESIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EESIZE;
#[doc = "`read()` method returns [eesize::R](eesize::R) reader structure"]
impl crate::Readable for EESIZE {}
#[doc = "EEPROM Size Information"]
pub mod eesize;
#[doc = "EEPROM Current Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eeblock](eeblock) module"]
pub type EEBLOCK = crate::Reg<u32, _EEBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEBLOCK;
#[doc = "`read()` method returns [eeblock::R](eeblock::R) reader structure"]
impl crate::Readable for EEBLOCK {}
#[doc = "`write(|w| ..)` method takes [eeblock::W](eeblock::W) writer structure"]
impl crate::Writable for EEBLOCK {}
#[doc = "EEPROM Current Block"]
pub mod eeblock;
#[doc = "EEPROM Current Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eeoffset](eeoffset) module"]
pub type EEOFFSET = crate::Reg<u32, _EEOFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEOFFSET;
#[doc = "`read()` method returns [eeoffset::R](eeoffset::R) reader structure"]
impl crate::Readable for EEOFFSET {}
#[doc = "`write(|w| ..)` method takes [eeoffset::W](eeoffset::W) writer structure"]
impl crate::Writable for EEOFFSET {}
#[doc = "EEPROM Current Offset"]
pub mod eeoffset;
#[doc = "EEPROM Read-Write\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eerdwr](eerdwr) module"]
pub type EERDWR = crate::Reg<u32, _EERDWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EERDWR;
#[doc = "`read()` method returns [eerdwr::R](eerdwr::R) reader structure"]
impl crate::Readable for EERDWR {}
#[doc = "`write(|w| ..)` method takes [eerdwr::W](eerdwr::W) writer structure"]
impl crate::Writable for EERDWR {}
#[doc = "EEPROM Read-Write"]
pub mod eerdwr;
#[doc = "EEPROM Read-Write with Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eerdwrinc](eerdwrinc) module"]
pub type EERDWRINC = crate::Reg<u32, _EERDWRINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EERDWRINC;
#[doc = "`read()` method returns [eerdwrinc::R](eerdwrinc::R) reader structure"]
impl crate::Readable for EERDWRINC {}
#[doc = "`write(|w| ..)` method takes [eerdwrinc::W](eerdwrinc::W) writer structure"]
impl crate::Writable for EERDWRINC {}
#[doc = "EEPROM Read-Write with Increment"]
pub mod eerdwrinc;
#[doc = "EEPROM Done Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eedone](eedone) module"]
pub type EEDONE = crate::Reg<u32, _EEDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEDONE;
#[doc = "`read()` method returns [eedone::R](eedone::R) reader structure"]
impl crate::Readable for EEDONE {}
#[doc = "EEPROM Done Status"]
pub mod eedone;
#[doc = "EEPROM Support Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eesupp](eesupp) module"]
pub type EESUPP = crate::Reg<u32, _EESUPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EESUPP;
#[doc = "`read()` method returns [eesupp::R](eesupp::R) reader structure"]
impl crate::Readable for EESUPP {}
#[doc = "`write(|w| ..)` method takes [eesupp::W](eesupp::W) writer structure"]
impl crate::Writable for EESUPP {}
#[doc = "EEPROM Support Control and Status"]
pub mod eesupp;
#[doc = "EEPROM Unlock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eeunlock](eeunlock) module"]
pub type EEUNLOCK = crate::Reg<u32, _EEUNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEUNLOCK;
#[doc = "`read()` method returns [eeunlock::R](eeunlock::R) reader structure"]
impl crate::Readable for EEUNLOCK {}
#[doc = "`write(|w| ..)` method takes [eeunlock::W](eeunlock::W) writer structure"]
impl crate::Writable for EEUNLOCK {}
#[doc = "EEPROM Unlock"]
pub mod eeunlock;
#[doc = "EEPROM Protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eeprot](eeprot) module"]
pub type EEPROT = crate::Reg<u32, _EEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPROT;
#[doc = "`read()` method returns [eeprot::R](eeprot::R) reader structure"]
impl crate::Readable for EEPROT {}
#[doc = "`write(|w| ..)` method takes [eeprot::W](eeprot::W) writer structure"]
impl crate::Writable for EEPROT {}
#[doc = "EEPROM Protection"]
pub mod eeprot;
#[doc = "EEPROM Password\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eepass0](eepass0) module"]
pub type EEPASS0 = crate::Reg<u32, _EEPASS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPASS0;
#[doc = "`read()` method returns [eepass0::R](eepass0::R) reader structure"]
impl crate::Readable for EEPASS0 {}
#[doc = "`write(|w| ..)` method takes [eepass0::W](eepass0::W) writer structure"]
impl crate::Writable for EEPASS0 {}
#[doc = "EEPROM Password"]
pub mod eepass0;
#[doc = "EEPROM Password\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eepass1](eepass1) module"]
pub type EEPASS1 = crate::Reg<u32, _EEPASS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPASS1;
#[doc = "`read()` method returns [eepass1::R](eepass1::R) reader structure"]
impl crate::Readable for EEPASS1 {}
#[doc = "`write(|w| ..)` method takes [eepass1::W](eepass1::W) writer structure"]
impl crate::Writable for EEPASS1 {}
#[doc = "EEPROM Password"]
pub mod eepass1;
#[doc = "EEPROM Password\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eepass2](eepass2) module"]
pub type EEPASS2 = crate::Reg<u32, _EEPASS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPASS2;
#[doc = "`read()` method returns [eepass2::R](eepass2::R) reader structure"]
impl crate::Readable for EEPASS2 {}
#[doc = "`write(|w| ..)` method takes [eepass2::W](eepass2::W) writer structure"]
impl crate::Writable for EEPASS2 {}
#[doc = "EEPROM Password"]
pub mod eepass2;
#[doc = "EEPROM Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eeint](eeint) module"]
pub type EEINT = crate::Reg<u32, _EEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEINT;
#[doc = "`read()` method returns [eeint::R](eeint::R) reader structure"]
impl crate::Readable for EEINT {}
#[doc = "`write(|w| ..)` method takes [eeint::W](eeint::W) writer structure"]
impl crate::Writable for EEINT {}
#[doc = "EEPROM Interrupt"]
pub mod eeint;
#[doc = "EEPROM Block Hide\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eehide](eehide) module"]
pub type EEHIDE = crate::Reg<u32, _EEHIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEHIDE;
#[doc = "`read()` method returns [eehide::R](eehide::R) reader structure"]
impl crate::Readable for EEHIDE {}
#[doc = "`write(|w| ..)` method takes [eehide::W](eehide::W) writer structure"]
impl crate::Writable for EEHIDE {}
#[doc = "EEPROM Block Hide"]
pub mod eehide;
#[doc = "EEPROM Debug Mass Erase\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eedbgme](eedbgme) module"]
pub type EEDBGME = crate::Reg<u32, _EEDBGME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEDBGME;
#[doc = "`read()` method returns [eedbgme::R](eedbgme::R) reader structure"]
impl crate::Readable for EEDBGME {}
#[doc = "`write(|w| ..)` method takes [eedbgme::W](eedbgme::W) writer structure"]
impl crate::Writable for EEDBGME {}
#[doc = "EEPROM Debug Mass Erase"]
pub mod eedbgme;
#[doc = "EEPROM Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "EEPROM Peripheral Properties"]
pub mod pp;
