#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Exception Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x04 - System Exception Interrupt Mask"]
    pub im: IM,
    #[doc = "0x08 - System Exception Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x0c - System Exception Interrupt Clear"]
    pub ic: IC,
}
#[doc = "System Exception Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "System Exception Raw Interrupt Status"]
pub mod ris;
#[doc = "System Exception Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "System Exception Interrupt Mask"]
pub mod im;
#[doc = "System Exception Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "System Exception Masked Interrupt Status"]
pub mod mis;
#[doc = "System Exception Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](ic) module"]
pub type IC = crate::Reg<u32, _IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC;
#[doc = "`write(|w| ..)` method takes [ic::W](ic::W) writer structure"]
impl crate::Writable for IC {}
#[doc = "System Exception Interrupt Clear"]
pub mod ic;
