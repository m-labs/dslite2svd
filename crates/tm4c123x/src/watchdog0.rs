#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load"]
    pub load: LOAD,
    #[doc = "0x04 - Watchdog Value"]
    pub value: VALUE,
    #[doc = "0x08 - Watchdog Control"]
    pub ctl: CTL,
    #[doc = "0x0c - Watchdog Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x14 - Watchdog Masked Interrupt Status"]
    pub mis: MIS,
    _reserved6: [u8; 1024usize],
    #[doc = "0x418 - Watchdog Test"]
    pub test: TEST,
    _reserved7: [u8; 2020usize],
    #[doc = "0xc00 - Watchdog Lock"]
    pub lock: LOCK,
}
#[doc = "Watchdog Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [load](load) module"]
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Watchdog Load"]
pub mod load;
#[doc = "Watchdog Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [value](value) module"]
pub type VALUE = crate::Reg<u32, _VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE;
#[doc = "`read()` method returns [value::R](value::R) reader structure"]
impl crate::Readable for VALUE {}
#[doc = "Watchdog Value"]
pub mod value;
#[doc = "Watchdog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Watchdog Control"]
pub mod ctl;
#[doc = "Watchdog Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Watchdog Interrupt Clear"]
pub mod icr;
#[doc = "Watchdog Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Watchdog Raw Interrupt Status"]
pub mod ris;
#[doc = "Watchdog Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Watchdog Masked Interrupt Status"]
pub mod mis;
#[doc = "Watchdog Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Watchdog Test"]
pub mod test;
#[doc = "Watchdog Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Watchdog Lock"]
pub mod lock;
