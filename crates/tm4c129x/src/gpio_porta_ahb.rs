#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1020usize],
    #[doc = "0x3fc - GPIO Data"]
    pub data: DATA,
    #[doc = "0x400 - GPIO Direction"]
    pub dir: DIR,
    #[doc = "0x404 - GPIO Interrupt Sense"]
    pub is: IS,
    #[doc = "0x408 - GPIO Interrupt Both Edges"]
    pub ibe: IBE,
    #[doc = "0x40c - GPIO Interrupt Event"]
    pub iev: IEV,
    #[doc = "0x410 - GPIO Interrupt Mask"]
    pub im: IM,
    #[doc = "0x414 - GPIO Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x418 - GPIO Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x41c - GPIO Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x420 - GPIO Alternate Function Select"]
    pub afsel: AFSEL,
    _reserved10: [u8; 220usize],
    #[doc = "0x500 - GPIO 2-mA Drive Select"]
    pub dr2r: DR2R,
    #[doc = "0x504 - GPIO 4-mA Drive Select"]
    pub dr4r: DR4R,
    #[doc = "0x508 - GPIO 8-mA Drive Select"]
    pub dr8r: DR8R,
    #[doc = "0x50c - GPIO Open Drain Select"]
    pub odr: ODR,
    #[doc = "0x510 - GPIO Pull-Up Select"]
    pub pur: PUR,
    #[doc = "0x514 - GPIO Pull-Down Select"]
    pub pdr: PDR,
    #[doc = "0x518 - GPIO Slew Rate Control Select"]
    pub slr: SLR,
    #[doc = "0x51c - GPIO Digital Enable"]
    pub den: DEN,
    #[doc = "0x520 - GPIO Lock"]
    pub lock: LOCK,
    #[doc = "0x524 - GPIO Commit"]
    pub cr: CR,
    #[doc = "0x528 - GPIO Analog Mode Select"]
    pub amsel: AMSEL,
    #[doc = "0x52c - GPIO Port Control"]
    pub pctl: PCTL,
    #[doc = "0x530 - GPIO ADC Control"]
    pub adcctl: ADCCTL,
    #[doc = "0x534 - GPIO DMA Control"]
    pub dmactl: DMACTL,
    #[doc = "0x538 - GPIO Select Interrupt"]
    pub si: SI,
    #[doc = "0x53c - GPIO 12-mA Drive Select"]
    pub dr12r: DR12R,
    #[doc = "0x540 - GPIO Wake Pin Enable"]
    pub wakepen: WAKEPEN,
    #[doc = "0x544 - GPIO Wake Level"]
    pub wakelvl: WAKELVL,
    #[doc = "0x548 - GPIO Wake Status"]
    pub wakestat: WAKESTAT,
    _reserved29: [u8; 2676usize],
    #[doc = "0xfc0 - GPIO Peripheral Property"]
    pub pp: PP,
    #[doc = "0xfc4 - GPIO Peripheral Configuration"]
    pub pc: PC,
}
#[doc = "GPIO Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "GPIO Data"]
pub mod data;
#[doc = "GPIO Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "GPIO Direction"]
pub mod dir;
#[doc = "GPIO Interrupt Sense\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [is](is) module"]
pub type IS = crate::Reg<u32, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "`write(|w| ..)` method takes [is::W](is::W) writer structure"]
impl crate::Writable for IS {}
#[doc = "GPIO Interrupt Sense"]
pub mod is;
#[doc = "GPIO Interrupt Both Edges\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ibe](ibe) module"]
pub type IBE = crate::Reg<u32, _IBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBE;
#[doc = "`read()` method returns [ibe::R](ibe::R) reader structure"]
impl crate::Readable for IBE {}
#[doc = "`write(|w| ..)` method takes [ibe::W](ibe::W) writer structure"]
impl crate::Writable for IBE {}
#[doc = "GPIO Interrupt Both Edges"]
pub mod ibe;
#[doc = "GPIO Interrupt Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iev](iev) module"]
pub type IEV = crate::Reg<u32, _IEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEV;
#[doc = "`read()` method returns [iev::R](iev::R) reader structure"]
impl crate::Readable for IEV {}
#[doc = "`write(|w| ..)` method takes [iev::W](iev::W) writer structure"]
impl crate::Writable for IEV {}
#[doc = "GPIO Interrupt Event"]
pub mod iev;
#[doc = "GPIO Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "GPIO Interrupt Mask"]
pub mod im;
#[doc = "GPIO Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "GPIO Raw Interrupt Status"]
pub mod ris;
#[doc = "GPIO Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "GPIO Masked Interrupt Status"]
pub mod mis;
#[doc = "GPIO Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "GPIO Interrupt Clear"]
pub mod icr;
#[doc = "GPIO Alternate Function Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [afsel](afsel) module"]
pub type AFSEL = crate::Reg<u32, _AFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSEL;
#[doc = "`read()` method returns [afsel::R](afsel::R) reader structure"]
impl crate::Readable for AFSEL {}
#[doc = "`write(|w| ..)` method takes [afsel::W](afsel::W) writer structure"]
impl crate::Writable for AFSEL {}
#[doc = "GPIO Alternate Function Select"]
pub mod afsel;
#[doc = "GPIO 2-mA Drive Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr2r](dr2r) module"]
pub type DR2R = crate::Reg<u32, _DR2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR2R;
#[doc = "`read()` method returns [dr2r::R](dr2r::R) reader structure"]
impl crate::Readable for DR2R {}
#[doc = "`write(|w| ..)` method takes [dr2r::W](dr2r::W) writer structure"]
impl crate::Writable for DR2R {}
#[doc = "GPIO 2-mA Drive Select"]
pub mod dr2r;
#[doc = "GPIO 4-mA Drive Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr4r](dr4r) module"]
pub type DR4R = crate::Reg<u32, _DR4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR4R;
#[doc = "`read()` method returns [dr4r::R](dr4r::R) reader structure"]
impl crate::Readable for DR4R {}
#[doc = "`write(|w| ..)` method takes [dr4r::W](dr4r::W) writer structure"]
impl crate::Writable for DR4R {}
#[doc = "GPIO 4-mA Drive Select"]
pub mod dr4r;
#[doc = "GPIO 8-mA Drive Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr8r](dr8r) module"]
pub type DR8R = crate::Reg<u32, _DR8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR8R;
#[doc = "`read()` method returns [dr8r::R](dr8r::R) reader structure"]
impl crate::Readable for DR8R {}
#[doc = "`write(|w| ..)` method takes [dr8r::W](dr8r::W) writer structure"]
impl crate::Writable for DR8R {}
#[doc = "GPIO 8-mA Drive Select"]
pub mod dr8r;
#[doc = "GPIO Open Drain Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [odr](odr) module"]
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
#[doc = "`read()` method returns [odr::R](odr::R) reader structure"]
impl crate::Readable for ODR {}
#[doc = "`write(|w| ..)` method takes [odr::W](odr::W) writer structure"]
impl crate::Writable for ODR {}
#[doc = "GPIO Open Drain Select"]
pub mod odr;
#[doc = "GPIO Pull-Up Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pur](pur) module"]
pub type PUR = crate::Reg<u32, _PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUR;
#[doc = "`read()` method returns [pur::R](pur::R) reader structure"]
impl crate::Readable for PUR {}
#[doc = "`write(|w| ..)` method takes [pur::W](pur::W) writer structure"]
impl crate::Writable for PUR {}
#[doc = "GPIO Pull-Up Select"]
pub mod pur;
#[doc = "GPIO Pull-Down Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdr](pdr) module"]
pub type PDR = crate::Reg<u32, _PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR;
#[doc = "`read()` method returns [pdr::R](pdr::R) reader structure"]
impl crate::Readable for PDR {}
#[doc = "`write(|w| ..)` method takes [pdr::W](pdr::W) writer structure"]
impl crate::Writable for PDR {}
#[doc = "GPIO Pull-Down Select"]
pub mod pdr;
#[doc = "GPIO Slew Rate Control Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slr](slr) module"]
pub type SLR = crate::Reg<u32, _SLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLR;
#[doc = "`read()` method returns [slr::R](slr::R) reader structure"]
impl crate::Readable for SLR {}
#[doc = "`write(|w| ..)` method takes [slr::W](slr::W) writer structure"]
impl crate::Writable for SLR {}
#[doc = "GPIO Slew Rate Control Select"]
pub mod slr;
#[doc = "GPIO Digital Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [den](den) module"]
pub type DEN = crate::Reg<u32, _DEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEN;
#[doc = "`read()` method returns [den::R](den::R) reader structure"]
impl crate::Readable for DEN {}
#[doc = "`write(|w| ..)` method takes [den::W](den::W) writer structure"]
impl crate::Writable for DEN {}
#[doc = "GPIO Digital Enable"]
pub mod den;
#[doc = "GPIO Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "GPIO Lock"]
pub mod lock;
#[doc = "GPIO Commit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "GPIO Commit"]
pub mod cr;
#[doc = "GPIO Analog Mode Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [amsel](amsel) module"]
pub type AMSEL = crate::Reg<u32, _AMSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMSEL;
#[doc = "`read()` method returns [amsel::R](amsel::R) reader structure"]
impl crate::Readable for AMSEL {}
#[doc = "`write(|w| ..)` method takes [amsel::W](amsel::W) writer structure"]
impl crate::Writable for AMSEL {}
#[doc = "GPIO Analog Mode Select"]
pub mod amsel;
#[doc = "GPIO Port Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pctl](pctl) module"]
pub type PCTL = crate::Reg<u32, _PCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCTL;
#[doc = "`read()` method returns [pctl::R](pctl::R) reader structure"]
impl crate::Readable for PCTL {}
#[doc = "`write(|w| ..)` method takes [pctl::W](pctl::W) writer structure"]
impl crate::Writable for PCTL {}
#[doc = "GPIO Port Control"]
pub mod pctl;
#[doc = "GPIO ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcctl](adcctl) module"]
pub type ADCCTL = crate::Reg<u32, _ADCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL;
#[doc = "`read()` method returns [adcctl::R](adcctl::R) reader structure"]
impl crate::Readable for ADCCTL {}
#[doc = "`write(|w| ..)` method takes [adcctl::W](adcctl::W) writer structure"]
impl crate::Writable for ADCCTL {}
#[doc = "GPIO ADC Control"]
pub mod adcctl;
#[doc = "GPIO DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "GPIO DMA Control"]
pub mod dmactl;
#[doc = "GPIO Select Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [si](si) module"]
pub type SI = crate::Reg<u32, _SI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SI;
#[doc = "`read()` method returns [si::R](si::R) reader structure"]
impl crate::Readable for SI {}
#[doc = "`write(|w| ..)` method takes [si::W](si::W) writer structure"]
impl crate::Writable for SI {}
#[doc = "GPIO Select Interrupt"]
pub mod si;
#[doc = "GPIO 12-mA Drive Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr12r](dr12r) module"]
pub type DR12R = crate::Reg<u32, _DR12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR12R;
#[doc = "`read()` method returns [dr12r::R](dr12r::R) reader structure"]
impl crate::Readable for DR12R {}
#[doc = "`write(|w| ..)` method takes [dr12r::W](dr12r::W) writer structure"]
impl crate::Writable for DR12R {}
#[doc = "GPIO 12-mA Drive Select"]
pub mod dr12r;
#[doc = "GPIO Wake Pin Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakepen](wakepen) module"]
pub type WAKEPEN = crate::Reg<u32, _WAKEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEPEN;
#[doc = "`read()` method returns [wakepen::R](wakepen::R) reader structure"]
impl crate::Readable for WAKEPEN {}
#[doc = "`write(|w| ..)` method takes [wakepen::W](wakepen::W) writer structure"]
impl crate::Writable for WAKEPEN {}
#[doc = "GPIO Wake Pin Enable"]
pub mod wakepen;
#[doc = "GPIO Wake Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakelvl](wakelvl) module"]
pub type WAKELVL = crate::Reg<u32, _WAKELVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKELVL;
#[doc = "`read()` method returns [wakelvl::R](wakelvl::R) reader structure"]
impl crate::Readable for WAKELVL {}
#[doc = "`write(|w| ..)` method takes [wakelvl::W](wakelvl::W) writer structure"]
impl crate::Writable for WAKELVL {}
#[doc = "GPIO Wake Level"]
pub mod wakelvl;
#[doc = "GPIO Wake Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakestat](wakestat) module"]
pub type WAKESTAT = crate::Reg<u32, _WAKESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKESTAT;
#[doc = "`read()` method returns [wakestat::R](wakestat::R) reader structure"]
impl crate::Readable for WAKESTAT {}
#[doc = "GPIO Wake Status"]
pub mod wakestat;
#[doc = "GPIO Peripheral Property\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "GPIO Peripheral Property"]
pub mod pp;
#[doc = "GPIO Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "GPIO Peripheral Configuration"]
pub mod pc;
