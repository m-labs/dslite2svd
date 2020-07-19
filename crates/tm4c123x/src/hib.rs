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
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Hibernation Data"]
    pub data: DATA,
}
#[doc = "Hibernation RTC Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcc](rtcc) module"]
pub type RTCC = crate::Reg<u32, _RTCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCC;
#[doc = "`read()` method returns [rtcc::R](rtcc::R) reader structure"]
impl crate::Readable for RTCC {}
#[doc = "Hibernation RTC Counter"]
pub mod rtcc;
#[doc = "Hibernation RTC Match 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcm0](rtcm0) module"]
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
#[doc = "Hibernation RTC Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcld](rtcld) module"]
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
#[doc = "Hibernation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
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
#[doc = "Hibernation Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
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
#[doc = "Hibernation Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Hibernation Raw Interrupt Status"]
pub mod ris;
#[doc = "Hibernation Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Hibernation Masked Interrupt Status"]
pub mod mis;
#[doc = "Hibernation Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](ic) module"]
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
#[doc = "Hibernation RTC Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtct](rtct) module"]
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
#[doc = "Hibernation RTC Sub Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcss](rtcss) module"]
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
#[doc = "Hibernation Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
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
