#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - GPTM Timer A Mode"]
    pub tamr: TAMR,
    #[doc = "0x08 - GPTM Timer B Mode"]
    pub tbmr: TBMR,
    #[doc = "0x0c - GPTM Control"]
    pub ctl: CTL,
    #[doc = "0x10 - GPTM Synchronize"]
    pub sync: SYNC,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - GPTM Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - GPTM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x20 - GPTM Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x24 - GPTM Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x28 - GPTM Timer A Interval Load"]
    pub tailr: TAILR,
    #[doc = "0x2c - GPTM Timer B Interval Load"]
    pub tbilr: TBILR,
    #[doc = "0x30 - GPTM Timer A Match"]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - GPTM Timer B Match"]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - GPTM Timer A Prescale"]
    pub tapr: TAPR,
    #[doc = "0x3c - GPTM Timer B Prescale"]
    pub tbpr: TBPR,
    #[doc = "0x40 - GPTM TimerA Prescale Match"]
    pub tapmr: TAPMR,
    #[doc = "0x44 - GPTM TimerB Prescale Match"]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - GPTM Timer A"]
    pub tar: TAR,
    #[doc = "0x4c - GPTM Timer B"]
    pub tbr: TBR,
    #[doc = "0x50 - GPTM Timer A Value"]
    pub tav: TAV,
    #[doc = "0x54 - GPTM Timer B Value"]
    pub tbv: TBV,
    #[doc = "0x58 - GPTM RTC Predivide"]
    pub rtcpd: RTCPD,
    #[doc = "0x5c - GPTM Timer A Prescale Snapshot"]
    pub taps: TAPS,
    #[doc = "0x60 - GPTM Timer B Prescale Snapshot"]
    pub tbps: TBPS,
    _reserved24: [u8; 8usize],
    #[doc = "0x6c - GPTM DMA Event"]
    pub dmaev: DMAEV,
    #[doc = "0x70 - GPTM ADC Event"]
    pub adcev: ADCEV,
    _reserved26: [u8; 3916usize],
    #[doc = "0xfc0 - GPTM Peripheral Properties"]
    pub pp: PP,
    _reserved27: [u8; 4usize],
    #[doc = "0xfc8 - GPTM Clock Configuration"]
    pub cc: CC,
}
#[doc = "GPTM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "GPTM Configuration"]
pub mod cfg;
#[doc = "GPTM Timer A Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tamr](tamr) module"]
pub type TAMR = crate::Reg<u32, _TAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMR;
#[doc = "`read()` method returns [tamr::R](tamr::R) reader structure"]
impl crate::Readable for TAMR {}
#[doc = "`write(|w| ..)` method takes [tamr::W](tamr::W) writer structure"]
impl crate::Writable for TAMR {}
#[doc = "GPTM Timer A Mode"]
pub mod tamr;
#[doc = "GPTM Timer B Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbmr](tbmr) module"]
pub type TBMR = crate::Reg<u32, _TBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMR;
#[doc = "`read()` method returns [tbmr::R](tbmr::R) reader structure"]
impl crate::Readable for TBMR {}
#[doc = "`write(|w| ..)` method takes [tbmr::W](tbmr::W) writer structure"]
impl crate::Writable for TBMR {}
#[doc = "GPTM Timer B Mode"]
pub mod tbmr;
#[doc = "GPTM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "GPTM Control"]
pub mod ctl;
#[doc = "GPTM Synchronize\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "GPTM Synchronize"]
pub mod sync;
#[doc = "GPTM Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "GPTM Interrupt Mask"]
pub mod imr;
#[doc = "GPTM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "GPTM Raw Interrupt Status"]
pub mod ris;
#[doc = "GPTM Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "GPTM Masked Interrupt Status"]
pub mod mis;
#[doc = "GPTM Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "GPTM Interrupt Clear"]
pub mod icr;
#[doc = "GPTM Timer A Interval Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tailr](tailr) module"]
pub type TAILR = crate::Reg<u32, _TAILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAILR;
#[doc = "`read()` method returns [tailr::R](tailr::R) reader structure"]
impl crate::Readable for TAILR {}
#[doc = "`write(|w| ..)` method takes [tailr::W](tailr::W) writer structure"]
impl crate::Writable for TAILR {}
#[doc = "GPTM Timer A Interval Load"]
pub mod tailr;
#[doc = "GPTM Timer B Interval Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbilr](tbilr) module"]
pub type TBILR = crate::Reg<u32, _TBILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBILR;
#[doc = "`read()` method returns [tbilr::R](tbilr::R) reader structure"]
impl crate::Readable for TBILR {}
#[doc = "`write(|w| ..)` method takes [tbilr::W](tbilr::W) writer structure"]
impl crate::Writable for TBILR {}
#[doc = "GPTM Timer B Interval Load"]
pub mod tbilr;
#[doc = "GPTM Timer A Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tamatchr](tamatchr) module"]
pub type TAMATCHR = crate::Reg<u32, _TAMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMATCHR;
#[doc = "`read()` method returns [tamatchr::R](tamatchr::R) reader structure"]
impl crate::Readable for TAMATCHR {}
#[doc = "`write(|w| ..)` method takes [tamatchr::W](tamatchr::W) writer structure"]
impl crate::Writable for TAMATCHR {}
#[doc = "GPTM Timer A Match"]
pub mod tamatchr;
#[doc = "GPTM Timer B Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbmatchr](tbmatchr) module"]
pub type TBMATCHR = crate::Reg<u32, _TBMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMATCHR;
#[doc = "`read()` method returns [tbmatchr::R](tbmatchr::R) reader structure"]
impl crate::Readable for TBMATCHR {}
#[doc = "`write(|w| ..)` method takes [tbmatchr::W](tbmatchr::W) writer structure"]
impl crate::Writable for TBMATCHR {}
#[doc = "GPTM Timer B Match"]
pub mod tbmatchr;
#[doc = "GPTM Timer A Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tapr](tapr) module"]
pub type TAPR = crate::Reg<u32, _TAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPR;
#[doc = "`read()` method returns [tapr::R](tapr::R) reader structure"]
impl crate::Readable for TAPR {}
#[doc = "`write(|w| ..)` method takes [tapr::W](tapr::W) writer structure"]
impl crate::Writable for TAPR {}
#[doc = "GPTM Timer A Prescale"]
pub mod tapr;
#[doc = "GPTM Timer B Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbpr](tbpr) module"]
pub type TBPR = crate::Reg<u32, _TBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPR;
#[doc = "`read()` method returns [tbpr::R](tbpr::R) reader structure"]
impl crate::Readable for TBPR {}
#[doc = "`write(|w| ..)` method takes [tbpr::W](tbpr::W) writer structure"]
impl crate::Writable for TBPR {}
#[doc = "GPTM Timer B Prescale"]
pub mod tbpr;
#[doc = "GPTM TimerA Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tapmr](tapmr) module"]
pub type TAPMR = crate::Reg<u32, _TAPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPMR;
#[doc = "`read()` method returns [tapmr::R](tapmr::R) reader structure"]
impl crate::Readable for TAPMR {}
#[doc = "`write(|w| ..)` method takes [tapmr::W](tapmr::W) writer structure"]
impl crate::Writable for TAPMR {}
#[doc = "GPTM TimerA Prescale Match"]
pub mod tapmr;
#[doc = "GPTM TimerB Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbpmr](tbpmr) module"]
pub type TBPMR = crate::Reg<u32, _TBPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPMR;
#[doc = "`read()` method returns [tbpmr::R](tbpmr::R) reader structure"]
impl crate::Readable for TBPMR {}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](tbpmr::W) writer structure"]
impl crate::Writable for TBPMR {}
#[doc = "GPTM TimerB Prescale Match"]
pub mod tbpmr;
#[doc = "GPTM Timer A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "GPTM Timer A"]
pub mod tar;
#[doc = "GPTM Timer B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbr](tbr) module"]
pub type TBR = crate::Reg<u32, _TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBR;
#[doc = "`read()` method returns [tbr::R](tbr::R) reader structure"]
impl crate::Readable for TBR {}
#[doc = "GPTM Timer B"]
pub mod tbr;
#[doc = "GPTM Timer A Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tav](tav) module"]
pub type TAV = crate::Reg<u32, _TAV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAV;
#[doc = "`read()` method returns [tav::R](tav::R) reader structure"]
impl crate::Readable for TAV {}
#[doc = "`write(|w| ..)` method takes [tav::W](tav::W) writer structure"]
impl crate::Writable for TAV {}
#[doc = "GPTM Timer A Value"]
pub mod tav;
#[doc = "GPTM Timer B Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbv](tbv) module"]
pub type TBV = crate::Reg<u32, _TBV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBV;
#[doc = "`read()` method returns [tbv::R](tbv::R) reader structure"]
impl crate::Readable for TBV {}
#[doc = "`write(|w| ..)` method takes [tbv::W](tbv::W) writer structure"]
impl crate::Writable for TBV {}
#[doc = "GPTM Timer B Value"]
pub mod tbv;
#[doc = "GPTM RTC Predivide\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcpd](rtcpd) module"]
pub type RTCPD = crate::Reg<u32, _RTCPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPD;
#[doc = "`read()` method returns [rtcpd::R](rtcpd::R) reader structure"]
impl crate::Readable for RTCPD {}
#[doc = "GPTM RTC Predivide"]
pub mod rtcpd;
#[doc = "GPTM Timer A Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [taps](taps) module"]
pub type TAPS = crate::Reg<u32, _TAPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPS;
#[doc = "`read()` method returns [taps::R](taps::R) reader structure"]
impl crate::Readable for TAPS {}
#[doc = "GPTM Timer A Prescale Snapshot"]
pub mod taps;
#[doc = "GPTM Timer B Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbps](tbps) module"]
pub type TBPS = crate::Reg<u32, _TBPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPS;
#[doc = "`read()` method returns [tbps::R](tbps::R) reader structure"]
impl crate::Readable for TBPS {}
#[doc = "GPTM Timer B Prescale Snapshot"]
pub mod tbps;
#[doc = "GPTM DMA Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmaev](dmaev) module"]
pub type DMAEV = crate::Reg<u32, _DMAEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAEV;
#[doc = "`read()` method returns [dmaev::R](dmaev::R) reader structure"]
impl crate::Readable for DMAEV {}
#[doc = "`write(|w| ..)` method takes [dmaev::W](dmaev::W) writer structure"]
impl crate::Writable for DMAEV {}
#[doc = "GPTM DMA Event"]
pub mod dmaev;
#[doc = "GPTM ADC Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcev](adcev) module"]
pub type ADCEV = crate::Reg<u32, _ADCEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCEV;
#[doc = "`read()` method returns [adcev::R](adcev::R) reader structure"]
impl crate::Readable for ADCEV {}
#[doc = "`write(|w| ..)` method takes [adcev::W](adcev::W) writer structure"]
impl crate::Writable for ADCEV {}
#[doc = "GPTM ADC Event"]
pub mod adcev;
#[doc = "GPTM Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "GPTM Peripheral Properties"]
pub mod pp;
#[doc = "GPTM Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "GPTM Clock Configuration"]
pub mod cc;
