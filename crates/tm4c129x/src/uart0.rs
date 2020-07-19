#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data"]
    pub dr: DR,
    _reserved_1_ecr: [u8; 4usize],
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - UART Flag"]
    pub fr: FR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - UART IrDA Low-Power Register"]
    pub ilpr: ILPR,
    #[doc = "0x24 - UART Integer Baud-Rate Divisor"]
    pub ibrd: IBRD,
    #[doc = "0x28 - UART Fractional Baud-Rate Divisor"]
    pub fbrd: FBRD,
    #[doc = "0x2c - UART Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - UART Control"]
    pub ctl: CTL,
    #[doc = "0x34 - UART Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - UART Interrupt Mask"]
    pub im: IM,
    #[doc = "0x3c - UART Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - UART Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - UART Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x48 - UART DMA Control"]
    pub dmactl: DMACTL,
    _reserved14: [u8; 88usize],
    #[doc = "0xa4 - UART 9-Bit Self Address"]
    pub _9bitaddr: _9BITADDR,
    #[doc = "0xa8 - UART 9-Bit Self Address Mask"]
    pub _9bitamask: _9BITAMASK,
    _reserved16: [u8; 3860usize],
    #[doc = "0xfc0 - UART Peripheral Properties"]
    pub pp: PP,
    _reserved17: [u8; 4usize],
    #[doc = "0xfc8 - UART Clock Configuration"]
    pub cc: CC,
}
impl RegisterBlock {
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn ecr(&self) -> &ECR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const ECR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn ecr_mut(&self) -> &mut ECR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut ECR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn rsr(&self) -> &RSR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RSR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn rsr_mut(&self) -> &mut RSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RSR) }
    }
}
#[doc = "UART Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "UART Data"]
pub mod dr;
#[doc = "UART Receive Status/Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "UART Receive Status/Error Clear"]
pub mod rsr;
#[doc = "UART Receive Status/Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "UART Receive Status/Error Clear"]
pub mod ecr;
#[doc = "UART Flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "UART Flag"]
pub mod fr;
#[doc = "UART IrDA Low-Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilpr](ilpr) module"]
pub type ILPR = crate::Reg<u32, _ILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILPR;
#[doc = "`read()` method returns [ilpr::R](ilpr::R) reader structure"]
impl crate::Readable for ILPR {}
#[doc = "`write(|w| ..)` method takes [ilpr::W](ilpr::W) writer structure"]
impl crate::Writable for ILPR {}
#[doc = "UART IrDA Low-Power Register"]
pub mod ilpr;
#[doc = "UART Integer Baud-Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](ibrd) module"]
pub type IBRD = crate::Reg<u32, _IBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBRD;
#[doc = "`read()` method returns [ibrd::R](ibrd::R) reader structure"]
impl crate::Readable for IBRD {}
#[doc = "`write(|w| ..)` method takes [ibrd::W](ibrd::W) writer structure"]
impl crate::Writable for IBRD {}
#[doc = "UART Integer Baud-Rate Divisor"]
pub mod ibrd;
#[doc = "UART Fractional Baud-Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](fbrd) module"]
pub type FBRD = crate::Reg<u32, _FBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBRD;
#[doc = "`read()` method returns [fbrd::R](fbrd::R) reader structure"]
impl crate::Readable for FBRD {}
#[doc = "`write(|w| ..)` method takes [fbrd::W](fbrd::W) writer structure"]
impl crate::Writable for FBRD {}
#[doc = "UART Fractional Baud-Rate Divisor"]
pub mod fbrd;
#[doc = "UART Line Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](lcrh) module"]
pub type LCRH = crate::Reg<u32, _LCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCRH;
#[doc = "`read()` method returns [lcrh::R](lcrh::R) reader structure"]
impl crate::Readable for LCRH {}
#[doc = "`write(|w| ..)` method takes [lcrh::W](lcrh::W) writer structure"]
impl crate::Writable for LCRH {}
#[doc = "UART Line Control"]
pub mod lcrh;
#[doc = "UART Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "UART Control"]
pub mod ctl;
#[doc = "UART Interrupt FIFO Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](ifls) module"]
pub type IFLS = crate::Reg<u32, _IFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLS;
#[doc = "`read()` method returns [ifls::R](ifls::R) reader structure"]
impl crate::Readable for IFLS {}
#[doc = "`write(|w| ..)` method takes [ifls::W](ifls::W) writer structure"]
impl crate::Writable for IFLS {}
#[doc = "UART Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "UART Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "UART Interrupt Mask"]
pub mod im;
#[doc = "UART Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "UART Raw Interrupt Status"]
pub mod ris;
#[doc = "UART Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "UART Masked Interrupt Status"]
pub mod mis;
#[doc = "UART Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "UART Interrupt Clear"]
pub mod icr;
#[doc = "UART DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "UART DMA Control"]
pub mod dmactl;
#[doc = "UART 9-Bit Self Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_9bitaddr](_9bitaddr) module"]
pub type _9BITADDR = crate::Reg<u32, __9BITADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __9BITADDR;
#[doc = "`read()` method returns [_9bitaddr::R](_9bitaddr::R) reader structure"]
impl crate::Readable for _9BITADDR {}
#[doc = "`write(|w| ..)` method takes [_9bitaddr::W](_9bitaddr::W) writer structure"]
impl crate::Writable for _9BITADDR {}
#[doc = "UART 9-Bit Self Address"]
pub mod _9bitaddr;
#[doc = "UART 9-Bit Self Address Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_9bitamask](_9bitamask) module"]
pub type _9BITAMASK = crate::Reg<u32, __9BITAMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __9BITAMASK;
#[doc = "`read()` method returns [_9bitamask::R](_9bitamask::R) reader structure"]
impl crate::Readable for _9BITAMASK {}
#[doc = "`write(|w| ..)` method takes [_9bitamask::W](_9bitamask::W) writer structure"]
impl crate::Writable for _9BITAMASK {}
#[doc = "UART 9-Bit Self Address Mask"]
pub mod _9bitamask;
#[doc = "UART Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "UART Peripheral Properties"]
pub mod pp;
#[doc = "UART Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "UART Clock Configuration"]
pub mod cc;
