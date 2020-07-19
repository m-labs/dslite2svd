#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QEI Control"]
    pub ctl: CTL,
    #[doc = "0x04 - QEI Status"]
    pub stat: STAT,
    #[doc = "0x08 - QEI Position"]
    pub pos: POS,
    #[doc = "0x0c - QEI Maximum Position"]
    pub maxpos: MAXPOS,
    #[doc = "0x10 - QEI Timer Load"]
    pub load: LOAD,
    #[doc = "0x14 - QEI Timer"]
    pub time: TIME,
    #[doc = "0x18 - QEI Velocity Counter"]
    pub count: COUNT,
    #[doc = "0x1c - QEI Velocity"]
    pub speed: SPEED,
    #[doc = "0x20 - QEI Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x24 - QEI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x28 - QEI Interrupt Status and Clear"]
    pub isc: ISC,
}
#[doc = "QEI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "QEI Control"]
pub mod ctl;
#[doc = "QEI Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "QEI Status"]
pub mod stat;
#[doc = "QEI Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pos](pos) module"]
pub type POS = crate::Reg<u32, _POS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POS;
#[doc = "`read()` method returns [pos::R](pos::R) reader structure"]
impl crate::Readable for POS {}
#[doc = "`write(|w| ..)` method takes [pos::W](pos::W) writer structure"]
impl crate::Writable for POS {}
#[doc = "QEI Position"]
pub mod pos;
#[doc = "QEI Maximum Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpos](maxpos) module"]
pub type MAXPOS = crate::Reg<u32, _MAXPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXPOS;
#[doc = "`read()` method returns [maxpos::R](maxpos::R) reader structure"]
impl crate::Readable for MAXPOS {}
#[doc = "`write(|w| ..)` method takes [maxpos::W](maxpos::W) writer structure"]
impl crate::Writable for MAXPOS {}
#[doc = "QEI Maximum Position"]
pub mod maxpos;
#[doc = "QEI Timer Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "QEI Timer Load"]
pub mod load;
#[doc = "QEI Timer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "QEI Timer"]
pub mod time;
#[doc = "QEI Velocity Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "QEI Velocity Counter"]
pub mod count;
#[doc = "QEI Velocity\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [speed](speed) module"]
pub type SPEED = crate::Reg<u32, _SPEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPEED;
#[doc = "`read()` method returns [speed::R](speed::R) reader structure"]
impl crate::Readable for SPEED {}
#[doc = "QEI Velocity"]
pub mod speed;
#[doc = "QEI Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "QEI Interrupt Enable"]
pub mod inten;
#[doc = "QEI Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "QEI Raw Interrupt Status"]
pub mod ris;
#[doc = "QEI Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](isc) module"]
pub type ISC = crate::Reg<u32, _ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISC;
#[doc = "`read()` method returns [isc::R](isc::R) reader structure"]
impl crate::Readable for ISC {}
#[doc = "`write(|w| ..)` method takes [isc::W](isc::W) writer structure"]
impl crate::Writable for ISC {}
#[doc = "QEI Interrupt Status and Clear"]
pub mod isc;
