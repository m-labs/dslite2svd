#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    pub stat: STAT,
    #[doc = "0x04 - DMA Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - DMA Channel Control Base Pointer"]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - DMA Alternate Channel Control Base Pointer"]
    pub altbase: ALTBASE,
    #[doc = "0x10 - DMA Channel Wait-on-Request Status"]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - DMA Channel Software Request"]
    pub swreq: SWREQ,
    #[doc = "0x18 - DMA Channel Useburst Set"]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - DMA Channel Useburst Clear"]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - DMA Channel Request Mask Set"]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - DMA Channel Request Mask Clear"]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - DMA Channel Enable Set"]
    pub enaset: ENASET,
    #[doc = "0x2c - DMA Channel Enable Clear"]
    pub enaclr: ENACLR,
    #[doc = "0x30 - DMA Channel Primary Alternate Set"]
    pub altset: ALTSET,
    #[doc = "0x34 - DMA Channel Primary Alternate Clear"]
    pub altclr: ALTCLR,
    #[doc = "0x38 - DMA Channel Priority Set"]
    pub prioset: PRIOSET,
    #[doc = "0x3c - DMA Channel Priority Clear"]
    pub prioclr: PRIOCLR,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - DMA Bus Error Clear"]
    pub errclr: ERRCLR,
    _reserved17: [u8; 1200usize],
    #[doc = "0x500 - DMA Channel Assignment"]
    pub chasgn: CHASGN,
    #[doc = "0x504 - DMA Channel Interrupt Status"]
    pub chis: CHIS,
    _reserved19: [u8; 8usize],
    #[doc = "0x510 - DMA Channel Map Select 0"]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - DMA Channel Map Select 1"]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - DMA Channel Map Select 2"]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - DMA Channel Map Select 3"]
    pub chmap3: CHMAP3,
}
#[doc = "DMA Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "DMA Status"]
pub mod stat;
#[doc = "DMA Configuration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "DMA Channel Control Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlbase](ctlbase) module"]
pub type CTLBASE = crate::Reg<u32, _CTLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLBASE;
#[doc = "`read()` method returns [ctlbase::R](ctlbase::R) reader structure"]
impl crate::Readable for CTLBASE {}
#[doc = "`write(|w| ..)` method takes [ctlbase::W](ctlbase::W) writer structure"]
impl crate::Writable for CTLBASE {}
#[doc = "DMA Channel Control Base Pointer"]
pub mod ctlbase;
#[doc = "DMA Alternate Channel Control Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altbase](altbase) module"]
pub type ALTBASE = crate::Reg<u32, _ALTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTBASE;
#[doc = "`read()` method returns [altbase::R](altbase::R) reader structure"]
impl crate::Readable for ALTBASE {}
#[doc = "DMA Alternate Channel Control Base Pointer"]
pub mod altbase;
#[doc = "DMA Channel Wait-on-Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitstat](waitstat) module"]
pub type WAITSTAT = crate::Reg<u32, _WAITSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAITSTAT;
#[doc = "`read()` method returns [waitstat::R](waitstat::R) reader structure"]
impl crate::Readable for WAITSTAT {}
#[doc = "DMA Channel Wait-on-Request Status"]
pub mod waitstat;
#[doc = "DMA Channel Software Request\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](swreq) module"]
pub type SWREQ = crate::Reg<u32, _SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWREQ;
#[doc = "`write(|w| ..)` method takes [swreq::W](swreq::W) writer structure"]
impl crate::Writable for SWREQ {}
#[doc = "DMA Channel Software Request"]
pub mod swreq;
#[doc = "DMA Channel Useburst Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstset](useburstset) module"]
pub type USEBURSTSET = crate::Reg<u32, _USEBURSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTSET;
#[doc = "`read()` method returns [useburstset::R](useburstset::R) reader structure"]
impl crate::Readable for USEBURSTSET {}
#[doc = "`write(|w| ..)` method takes [useburstset::W](useburstset::W) writer structure"]
impl crate::Writable for USEBURSTSET {}
#[doc = "DMA Channel Useburst Set"]
pub mod useburstset;
#[doc = "DMA Channel Useburst Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstclr](useburstclr) module"]
pub type USEBURSTCLR = crate::Reg<u32, _USEBURSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTCLR;
#[doc = "`write(|w| ..)` method takes [useburstclr::W](useburstclr::W) writer structure"]
impl crate::Writable for USEBURSTCLR {}
#[doc = "DMA Channel Useburst Clear"]
pub mod useburstclr;
#[doc = "DMA Channel Request Mask Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskset](reqmaskset) module"]
pub type REQMASKSET = crate::Reg<u32, _REQMASKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKSET;
#[doc = "`read()` method returns [reqmaskset::R](reqmaskset::R) reader structure"]
impl crate::Readable for REQMASKSET {}
#[doc = "`write(|w| ..)` method takes [reqmaskset::W](reqmaskset::W) writer structure"]
impl crate::Writable for REQMASKSET {}
#[doc = "DMA Channel Request Mask Set"]
pub mod reqmaskset;
#[doc = "DMA Channel Request Mask Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskclr](reqmaskclr) module"]
pub type REQMASKCLR = crate::Reg<u32, _REQMASKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKCLR;
#[doc = "`write(|w| ..)` method takes [reqmaskclr::W](reqmaskclr::W) writer structure"]
impl crate::Writable for REQMASKCLR {}
#[doc = "DMA Channel Request Mask Clear"]
pub mod reqmaskclr;
#[doc = "DMA Channel Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaset](enaset) module"]
pub type ENASET = crate::Reg<u32, _ENASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENASET;
#[doc = "`read()` method returns [enaset::R](enaset::R) reader structure"]
impl crate::Readable for ENASET {}
#[doc = "`write(|w| ..)` method takes [enaset::W](enaset::W) writer structure"]
impl crate::Writable for ENASET {}
#[doc = "DMA Channel Enable Set"]
pub mod enaset;
#[doc = "DMA Channel Enable Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaclr](enaclr) module"]
pub type ENACLR = crate::Reg<u32, _ENACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENACLR;
#[doc = "`write(|w| ..)` method takes [enaclr::W](enaclr::W) writer structure"]
impl crate::Writable for ENACLR {}
#[doc = "DMA Channel Enable Clear"]
pub mod enaclr;
#[doc = "DMA Channel Primary Alternate Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altset](altset) module"]
pub type ALTSET = crate::Reg<u32, _ALTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTSET;
#[doc = "`read()` method returns [altset::R](altset::R) reader structure"]
impl crate::Readable for ALTSET {}
#[doc = "`write(|w| ..)` method takes [altset::W](altset::W) writer structure"]
impl crate::Writable for ALTSET {}
#[doc = "DMA Channel Primary Alternate Set"]
pub mod altset;
#[doc = "DMA Channel Primary Alternate Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altclr](altclr) module"]
pub type ALTCLR = crate::Reg<u32, _ALTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCLR;
#[doc = "`write(|w| ..)` method takes [altclr::W](altclr::W) writer structure"]
impl crate::Writable for ALTCLR {}
#[doc = "DMA Channel Primary Alternate Clear"]
pub mod altclr;
#[doc = "DMA Channel Priority Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioset](prioset) module"]
pub type PRIOSET = crate::Reg<u32, _PRIOSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOSET;
#[doc = "`read()` method returns [prioset::R](prioset::R) reader structure"]
impl crate::Readable for PRIOSET {}
#[doc = "`write(|w| ..)` method takes [prioset::W](prioset::W) writer structure"]
impl crate::Writable for PRIOSET {}
#[doc = "DMA Channel Priority Set"]
pub mod prioset;
#[doc = "DMA Channel Priority Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioclr](prioclr) module"]
pub type PRIOCLR = crate::Reg<u32, _PRIOCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOCLR;
#[doc = "`write(|w| ..)` method takes [prioclr::W](prioclr::W) writer structure"]
impl crate::Writable for PRIOCLR {}
#[doc = "DMA Channel Priority Clear"]
pub mod prioclr;
#[doc = "DMA Bus Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errclr](errclr) module"]
pub type ERRCLR = crate::Reg<u32, _ERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRCLR;
#[doc = "`read()` method returns [errclr::R](errclr::R) reader structure"]
impl crate::Readable for ERRCLR {}
#[doc = "`write(|w| ..)` method takes [errclr::W](errclr::W) writer structure"]
impl crate::Writable for ERRCLR {}
#[doc = "DMA Bus Error Clear"]
pub mod errclr;
#[doc = "DMA Channel Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chasgn](chasgn) module"]
pub type CHASGN = crate::Reg<u32, _CHASGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHASGN;
#[doc = "`read()` method returns [chasgn::R](chasgn::R) reader structure"]
impl crate::Readable for CHASGN {}
#[doc = "`write(|w| ..)` method takes [chasgn::W](chasgn::W) writer structure"]
impl crate::Writable for CHASGN {}
#[doc = "DMA Channel Assignment"]
pub mod chasgn;
#[doc = "DMA Channel Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chis](chis) module"]
pub type CHIS = crate::Reg<u32, _CHIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIS;
#[doc = "`read()` method returns [chis::R](chis::R) reader structure"]
impl crate::Readable for CHIS {}
#[doc = "`write(|w| ..)` method takes [chis::W](chis::W) writer structure"]
impl crate::Writable for CHIS {}
#[doc = "DMA Channel Interrupt Status"]
pub mod chis;
#[doc = "DMA Channel Map Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap0](chmap0) module"]
pub type CHMAP0 = crate::Reg<u32, _CHMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP0;
#[doc = "`read()` method returns [chmap0::R](chmap0::R) reader structure"]
impl crate::Readable for CHMAP0 {}
#[doc = "`write(|w| ..)` method takes [chmap0::W](chmap0::W) writer structure"]
impl crate::Writable for CHMAP0 {}
#[doc = "DMA Channel Map Select 0"]
pub mod chmap0;
#[doc = "DMA Channel Map Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap1](chmap1) module"]
pub type CHMAP1 = crate::Reg<u32, _CHMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP1;
#[doc = "`read()` method returns [chmap1::R](chmap1::R) reader structure"]
impl crate::Readable for CHMAP1 {}
#[doc = "`write(|w| ..)` method takes [chmap1::W](chmap1::W) writer structure"]
impl crate::Writable for CHMAP1 {}
#[doc = "DMA Channel Map Select 1"]
pub mod chmap1;
#[doc = "DMA Channel Map Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap2](chmap2) module"]
pub type CHMAP2 = crate::Reg<u32, _CHMAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP2;
#[doc = "`read()` method returns [chmap2::R](chmap2::R) reader structure"]
impl crate::Readable for CHMAP2 {}
#[doc = "`write(|w| ..)` method takes [chmap2::W](chmap2::W) writer structure"]
impl crate::Writable for CHMAP2 {}
#[doc = "DMA Channel Map Select 2"]
pub mod chmap2;
#[doc = "DMA Channel Map Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap3](chmap3) module"]
pub type CHMAP3 = crate::Reg<u32, _CHMAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP3;
#[doc = "`read()` method returns [chmap3::R](chmap3::R) reader structure"]
impl crate::Readable for CHMAP3 {}
#[doc = "`write(|w| ..)` method takes [chmap3::W](chmap3::W) writer structure"]
impl crate::Writable for CHMAP3 {}
#[doc = "DMA Channel Map Select 3"]
pub mod chmap3;
