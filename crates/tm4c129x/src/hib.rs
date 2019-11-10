#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernation RTC Counter"]
    pub rtcc: RTCC,
    #[doc = "0x04 - Hibernation RTC Match 0"]
    pub rtcm0: RTCM0,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Hibernation RTC Load"]
    pub rtcld: RTCLD,
    #[doc = "0x10 - Hibernation Control"]
    pub ctl: CTL,
    #[doc = "0x14 - Hibernation Interrupt Mask"]
    pub im: IM,
    #[doc = "0x18 - Hibernation Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - Hibernation Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - Hibernation Interrupt Clear"]
    pub ic: IC,
    #[doc = "0x24 - Hibernation RTC Trim"]
    pub rtct: RTCT,
    #[doc = "0x28 - Hibernation RTC Sub Seconds"]
    pub rtcss: RTCSS,
    #[doc = "0x2c - Hibernation IO Configuration"]
    pub io: IO,
    #[doc = "0x30 - Hibernation Data"]
    pub data: DATA,
    _reserved12: [u8; 716usize],
    #[doc = "0x300 - Hibernation Calendar Control"]
    pub calctl: CALCTL,
    _reserved13: [u8; 12usize],
    #[doc = "0x310 - Hibernation Calendar 0"]
    pub cal0: CAL0,
    #[doc = "0x314 - Hibernation Calendar 1"]
    pub cal1: CAL1,
    _reserved15: [u8; 8usize],
    #[doc = "0x320 - Hibernation Calendar Load 0"]
    pub calld0: CALLD0,
    #[doc = "0x324 - Hibernation Calendar Load"]
    pub calld1: CALLD1,
    _reserved17: [u8; 8usize],
    #[doc = "0x330 - Hibernation Calendar Match 0"]
    pub calm0: CALM0,
    #[doc = "0x334 - Hibernation Calendar Match 1"]
    pub calm1: CALM1,
    _reserved19: [u8; 40usize],
    #[doc = "0x360 - Hibernation Lock"]
    pub lock: LOCK,
    _reserved20: [u8; 156usize],
    #[doc = "0x400 - HIB Tamper Control"]
    pub tpctl: TPCTL,
    #[doc = "0x404 - HIB Tamper Status"]
    pub tpstat: TPSTAT,
    _reserved22: [u8; 8usize],
    #[doc = "0x410 - HIB Tamper I/O Control"]
    pub tpio: TPIO,
    _reserved23: [u8; 204usize],
    #[doc = "0x4e0 - HIB Tamper Log 0"]
    pub tplog0: TPLOG0,
    #[doc = "0x4e4 - HIB Tamper Log 1"]
    pub tplog1: TPLOG1,
    #[doc = "0x4e8 - HIB Tamper Log 2"]
    pub tplog2: TPLOG2,
    #[doc = "0x4ec - HIB Tamper Log 3"]
    pub tplog3: TPLOG3,
    #[doc = "0x4f0 - HIB Tamper Log 4"]
    pub tplog4: TPLOG4,
    #[doc = "0x4f4 - HIB Tamper Log 5"]
    pub tplog5: TPLOG5,
    #[doc = "0x4f8 - HIB Tamper Log 6"]
    pub tplog6: TPLOG6,
    #[doc = "0x4fc - HIB Tamper Log 7"]
    pub tplog7: TPLOG7,
    _reserved31: [u8; 2752usize],
    #[doc = "0xfc0 - Hibernation Peripheral Properties"]
    pub pp: PP,
    _reserved32: [u8; 4usize],
    #[doc = "0xfc8 - Hibernation Clock Control"]
    pub cc: CC,
}
#[doc = "Hibernation RTC Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcc](rtcc) module"]
pub type RTCC = crate::Reg<u32, _RTCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCC;
#[doc = "`read()` method returns [rtcc::R](rtcc::R) reader structure"]
impl crate::Readable for RTCC {}
#[doc = "Hibernation RTC Counter"]
pub mod rtcc;
#[doc = "Hibernation RTC Match 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcm0](rtcm0) module"]
pub type RTCM0 = crate::Reg<u32, _RTCM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCM0;
#[doc = "`read()` method returns [rtcm0::R](rtcm0::R) reader structure"]
impl crate::Readable for RTCM0 {}
#[doc = "`write(|w| ..)` method takes [rtcm0::W](rtcm0::W) writer structure"]
impl crate::Writable for RTCM0 {}
#[doc = "Hibernation RTC Match 0"]
pub mod rtcm0;
#[doc = "Hibernation RTC Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcld](rtcld) module"]
pub type RTCLD = crate::Reg<u32, _RTCLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCLD;
#[doc = "`read()` method returns [rtcld::R](rtcld::R) reader structure"]
impl crate::Readable for RTCLD {}
#[doc = "`write(|w| ..)` method takes [rtcld::W](rtcld::W) writer structure"]
impl crate::Writable for RTCLD {}
#[doc = "Hibernation RTC Load"]
pub mod rtcld;
#[doc = "Hibernation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Hibernation Control"]
pub mod ctl;
#[doc = "Hibernation Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "Hibernation Interrupt Mask"]
pub mod im;
#[doc = "Hibernation Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Hibernation Raw Interrupt Status"]
pub mod ris;
#[doc = "Hibernation Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Hibernation Masked Interrupt Status"]
pub mod mis;
#[doc = "Hibernation Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ic](ic) module"]
pub type IC = crate::Reg<u32, _IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC;
#[doc = "`read()` method returns [ic::R](ic::R) reader structure"]
impl crate::Readable for IC {}
#[doc = "`write(|w| ..)` method takes [ic::W](ic::W) writer structure"]
impl crate::Writable for IC {}
#[doc = "Hibernation Interrupt Clear"]
pub mod ic;
#[doc = "Hibernation RTC Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtct](rtct) module"]
pub type RTCT = crate::Reg<u32, _RTCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCT;
#[doc = "`read()` method returns [rtct::R](rtct::R) reader structure"]
impl crate::Readable for RTCT {}
#[doc = "`write(|w| ..)` method takes [rtct::W](rtct::W) writer structure"]
impl crate::Writable for RTCT {}
#[doc = "Hibernation RTC Trim"]
pub mod rtct;
#[doc = "Hibernation RTC Sub Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcss](rtcss) module"]
pub type RTCSS = crate::Reg<u32, _RTCSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSS;
#[doc = "`read()` method returns [rtcss::R](rtcss::R) reader structure"]
impl crate::Readable for RTCSS {}
#[doc = "`write(|w| ..)` method takes [rtcss::W](rtcss::W) writer structure"]
impl crate::Writable for RTCSS {}
#[doc = "Hibernation RTC Sub Seconds"]
pub mod rtcss;
#[doc = "Hibernation IO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [io](io) module"]
pub type IO = crate::Reg<u32, _IO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO;
#[doc = "`read()` method returns [io::R](io::R) reader structure"]
impl crate::Readable for IO {}
#[doc = "`write(|w| ..)` method takes [io::W](io::W) writer structure"]
impl crate::Writable for IO {}
#[doc = "Hibernation IO Configuration"]
pub mod io;
#[doc = "Hibernation Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Hibernation Data"]
pub mod data;
#[doc = "Hibernation Calendar Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calctl](calctl) module"]
pub type CALCTL = crate::Reg<u32, _CALCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCTL;
#[doc = "`read()` method returns [calctl::R](calctl::R) reader structure"]
impl crate::Readable for CALCTL {}
#[doc = "`write(|w| ..)` method takes [calctl::W](calctl::W) writer structure"]
impl crate::Writable for CALCTL {}
#[doc = "Hibernation Calendar Control"]
pub mod calctl;
#[doc = "Hibernation Calendar 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal0](cal0) module"]
pub type CAL0 = crate::Reg<u32, _CAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL0;
#[doc = "`read()` method returns [cal0::R](cal0::R) reader structure"]
impl crate::Readable for CAL0 {}
#[doc = "Hibernation Calendar 0"]
pub mod cal0;
#[doc = "Hibernation Calendar 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal1](cal1) module"]
pub type CAL1 = crate::Reg<u32, _CAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL1;
#[doc = "`read()` method returns [cal1::R](cal1::R) reader structure"]
impl crate::Readable for CAL1 {}
#[doc = "Hibernation Calendar 1"]
pub mod cal1;
#[doc = "Hibernation Calendar Load 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calld0](calld0) module"]
pub type CALLD0 = crate::Reg<u32, _CALLD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALLD0;
#[doc = "`write(|w| ..)` method takes [calld0::W](calld0::W) writer structure"]
impl crate::Writable for CALLD0 {}
#[doc = "Hibernation Calendar Load 0"]
pub mod calld0;
#[doc = "Hibernation Calendar Load\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calld1](calld1) module"]
pub type CALLD1 = crate::Reg<u32, _CALLD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALLD1;
#[doc = "`write(|w| ..)` method takes [calld1::W](calld1::W) writer structure"]
impl crate::Writable for CALLD1 {}
#[doc = "Hibernation Calendar Load"]
pub mod calld1;
#[doc = "Hibernation Calendar Match 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calm0](calm0) module"]
pub type CALM0 = crate::Reg<u32, _CALM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALM0;
#[doc = "`read()` method returns [calm0::R](calm0::R) reader structure"]
impl crate::Readable for CALM0 {}
#[doc = "`write(|w| ..)` method takes [calm0::W](calm0::W) writer structure"]
impl crate::Writable for CALM0 {}
#[doc = "Hibernation Calendar Match 0"]
pub mod calm0;
#[doc = "Hibernation Calendar Match 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calm1](calm1) module"]
pub type CALM1 = crate::Reg<u32, _CALM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALM1;
#[doc = "`read()` method returns [calm1::R](calm1::R) reader structure"]
impl crate::Readable for CALM1 {}
#[doc = "`write(|w| ..)` method takes [calm1::W](calm1::W) writer structure"]
impl crate::Writable for CALM1 {}
#[doc = "Hibernation Calendar Match 1"]
pub mod calm1;
#[doc = "Hibernation Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Hibernation Lock"]
pub mod lock;
#[doc = "HIB Tamper Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpctl](tpctl) module"]
pub type TPCTL = crate::Reg<u32, _TPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPCTL;
#[doc = "`read()` method returns [tpctl::R](tpctl::R) reader structure"]
impl crate::Readable for TPCTL {}
#[doc = "`write(|w| ..)` method takes [tpctl::W](tpctl::W) writer structure"]
impl crate::Writable for TPCTL {}
#[doc = "HIB Tamper Control"]
pub mod tpctl;
#[doc = "HIB Tamper Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpstat](tpstat) module"]
pub type TPSTAT = crate::Reg<u32, _TPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPSTAT;
#[doc = "`read()` method returns [tpstat::R](tpstat::R) reader structure"]
impl crate::Readable for TPSTAT {}
#[doc = "`write(|w| ..)` method takes [tpstat::W](tpstat::W) writer structure"]
impl crate::Writable for TPSTAT {}
#[doc = "HIB Tamper Status"]
pub mod tpstat;
#[doc = "HIB Tamper I/O Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpio](tpio) module"]
pub type TPIO = crate::Reg<u32, _TPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIO;
#[doc = "`read()` method returns [tpio::R](tpio::R) reader structure"]
impl crate::Readable for TPIO {}
#[doc = "`write(|w| ..)` method takes [tpio::W](tpio::W) writer structure"]
impl crate::Writable for TPIO {}
#[doc = "HIB Tamper I/O Control"]
pub mod tpio;
#[doc = "HIB Tamper Log 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog0](tplog0) module"]
pub type TPLOG0 = crate::Reg<u32, _TPLOG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG0;
#[doc = "`read()` method returns [tplog0::R](tplog0::R) reader structure"]
impl crate::Readable for TPLOG0 {}
#[doc = "HIB Tamper Log 0"]
pub mod tplog0;
#[doc = "HIB Tamper Log 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog1](tplog1) module"]
pub type TPLOG1 = crate::Reg<u32, _TPLOG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG1;
#[doc = "`read()` method returns [tplog1::R](tplog1::R) reader structure"]
impl crate::Readable for TPLOG1 {}
#[doc = "HIB Tamper Log 1"]
pub mod tplog1;
#[doc = "HIB Tamper Log 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog2](tplog2) module"]
pub type TPLOG2 = crate::Reg<u32, _TPLOG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG2;
#[doc = "`read()` method returns [tplog2::R](tplog2::R) reader structure"]
impl crate::Readable for TPLOG2 {}
#[doc = "HIB Tamper Log 2"]
pub mod tplog2;
#[doc = "HIB Tamper Log 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog3](tplog3) module"]
pub type TPLOG3 = crate::Reg<u32, _TPLOG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG3;
#[doc = "`read()` method returns [tplog3::R](tplog3::R) reader structure"]
impl crate::Readable for TPLOG3 {}
#[doc = "HIB Tamper Log 3"]
pub mod tplog3;
#[doc = "HIB Tamper Log 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog4](tplog4) module"]
pub type TPLOG4 = crate::Reg<u32, _TPLOG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG4;
#[doc = "`read()` method returns [tplog4::R](tplog4::R) reader structure"]
impl crate::Readable for TPLOG4 {}
#[doc = "HIB Tamper Log 4"]
pub mod tplog4;
#[doc = "HIB Tamper Log 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog5](tplog5) module"]
pub type TPLOG5 = crate::Reg<u32, _TPLOG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG5;
#[doc = "`read()` method returns [tplog5::R](tplog5::R) reader structure"]
impl crate::Readable for TPLOG5 {}
#[doc = "HIB Tamper Log 5"]
pub mod tplog5;
#[doc = "HIB Tamper Log 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog6](tplog6) module"]
pub type TPLOG6 = crate::Reg<u32, _TPLOG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG6;
#[doc = "`read()` method returns [tplog6::R](tplog6::R) reader structure"]
impl crate::Readable for TPLOG6 {}
#[doc = "HIB Tamper Log 6"]
pub mod tplog6;
#[doc = "HIB Tamper Log 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tplog7](tplog7) module"]
pub type TPLOG7 = crate::Reg<u32, _TPLOG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLOG7;
#[doc = "`read()` method returns [tplog7::R](tplog7::R) reader structure"]
impl crate::Readable for TPLOG7 {}
#[doc = "HIB Tamper Log 7"]
pub mod tplog7;
#[doc = "Hibernation Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "Hibernation Peripheral Properties"]
pub mod pp;
#[doc = "Hibernation Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Hibernation Clock Control"]
pub mod cc;
