#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Active Sample Sequencer"]
    pub actss: ACTSS,
    #[doc = "0x04 - ADC Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x08 - ADC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x0c - ADC Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x10 - ADC Overflow Status"]
    pub ostat: OSTAT,
    #[doc = "0x14 - ADC Event Multiplexer Select"]
    pub emux: EMUX,
    #[doc = "0x18 - ADC Underflow Status"]
    pub ustat: USTAT,
    #[doc = "0x1c - ADC Trigger Source Select"]
    pub tssel: TSSEL,
    #[doc = "0x20 - ADC Sample Sequencer Priority"]
    pub sspri: SSPRI,
    #[doc = "0x24 - ADC Sample Phase Control"]
    pub spc: SPC,
    #[doc = "0x28 - ADC Processor Sample Sequence Initiate"]
    pub pssi: PSSI,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - ADC Sample Averaging Control"]
    pub sac: SAC,
    #[doc = "0x34 - ADC Digital Comparator Interrupt Status and Clear"]
    pub dcisc: DCISC,
    #[doc = "0x38 - ADC Control"]
    pub ctl: CTL,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - ADC Sample Sequence Input Multiplexer Select 0"]
    pub ssmux0: SSMUX0,
    #[doc = "0x44 - ADC Sample Sequence Control 0"]
    pub ssctl0: SSCTL0,
    #[doc = "0x48 - ADC Sample Sequence Result FIFO 0"]
    pub ssfifo0: SSFIFO0,
    #[doc = "0x4c - ADC Sample Sequence FIFO 0 Status"]
    pub ssfstat0: SSFSTAT0,
    #[doc = "0x50 - ADC Sample Sequence 0 Operation"]
    pub ssop0: SSOP0,
    #[doc = "0x54 - ADC Sample Sequence 0 Digital Comparator Select"]
    pub ssdc0: SSDC0,
    #[doc = "0x58 - ADC Sample Sequence Extended Input Multiplexer Select 0"]
    pub ssemux0: SSEMUX0,
    #[doc = "0x5c - ADC Sample Sequence 0 Sample and Hold Time"]
    pub sstsh0: SSTSH0,
    #[doc = "0x60 - ADC Sample Sequence Input Multiplexer Select 1"]
    pub ssmux1: SSMUX1,
    #[doc = "0x64 - ADC Sample Sequence Control 1"]
    pub ssctl1: SSCTL1,
    #[doc = "0x68 - ADC Sample Sequence Result FIFO 1"]
    pub ssfifo1: SSFIFO1,
    #[doc = "0x6c - ADC Sample Sequence FIFO 1 Status"]
    pub ssfstat1: SSFSTAT1,
    #[doc = "0x70 - ADC Sample Sequence 1 Operation"]
    pub ssop1: SSOP1,
    #[doc = "0x74 - ADC Sample Sequence 1 Digital Comparator Select"]
    pub ssdc1: SSDC1,
    #[doc = "0x78 - ADC Sample Sequence Extended Input Multiplexer Select 1"]
    pub ssemux1: SSEMUX1,
    #[doc = "0x7c - ADC Sample Sequence 1 Sample and Hold Time"]
    pub sstsh1: SSTSH1,
    #[doc = "0x80 - ADC Sample Sequence Input Multiplexer Select 2"]
    pub ssmux2: SSMUX2,
    #[doc = "0x84 - ADC Sample Sequence Control 2"]
    pub ssctl2: SSCTL2,
    #[doc = "0x88 - ADC Sample Sequence Result FIFO 2"]
    pub ssfifo2: SSFIFO2,
    #[doc = "0x8c - ADC Sample Sequence FIFO 2 Status"]
    pub ssfstat2: SSFSTAT2,
    #[doc = "0x90 - ADC Sample Sequence 2 Operation"]
    pub ssop2: SSOP2,
    #[doc = "0x94 - ADC Sample Sequence 2 Digital Comparator Select"]
    pub ssdc2: SSDC2,
    #[doc = "0x98 - ADC Sample Sequence Extended Input Multiplexer Select 2"]
    pub ssemux2: SSEMUX2,
    #[doc = "0x9c - ADC Sample Sequence 2 Sample and Hold Time"]
    pub sstsh2: SSTSH2,
    #[doc = "0xa0 - ADC Sample Sequence Input Multiplexer Select 3"]
    pub ssmux3: SSMUX3,
    #[doc = "0xa4 - ADC Sample Sequence Control 3"]
    pub ssctl3: SSCTL3,
    #[doc = "0xa8 - ADC Sample Sequence Result FIFO 3"]
    pub ssfifo3: SSFIFO3,
    #[doc = "0xac - ADC Sample Sequence FIFO 3 Status"]
    pub ssfstat3: SSFSTAT3,
    #[doc = "0xb0 - ADC Sample Sequence 3 Operation"]
    pub ssop3: SSOP3,
    #[doc = "0xb4 - ADC Sample Sequence 3 Digital Comparator Select"]
    pub ssdc3: SSDC3,
    #[doc = "0xb8 - ADC Sample Sequence Extended Input Multiplexer Select 3"]
    pub ssemux3: SSEMUX3,
    #[doc = "0xbc - ADC Sample Sequence 3 Sample and Hold Time"]
    pub sstsh3: SSTSH3,
    _reserved46: [u8; 3136usize],
    #[doc = "0xd00 - ADC Digital Comparator Reset Initial Conditions"]
    pub dcric: DCRIC,
    _reserved47: [u8; 252usize],
    #[doc = "0xe00 - ADC Digital Comparator Control 0"]
    pub dcctl0: DCCTL0,
    #[doc = "0xe04 - ADC Digital Comparator Control 1"]
    pub dcctl1: DCCTL1,
    #[doc = "0xe08 - ADC Digital Comparator Control 2"]
    pub dcctl2: DCCTL2,
    #[doc = "0xe0c - ADC Digital Comparator Control 3"]
    pub dcctl3: DCCTL3,
    #[doc = "0xe10 - ADC Digital Comparator Control 4"]
    pub dcctl4: DCCTL4,
    #[doc = "0xe14 - ADC Digital Comparator Control 5"]
    pub dcctl5: DCCTL5,
    #[doc = "0xe18 - ADC Digital Comparator Control 6"]
    pub dcctl6: DCCTL6,
    #[doc = "0xe1c - ADC Digital Comparator Control 7"]
    pub dcctl7: DCCTL7,
    _reserved55: [u8; 32usize],
    #[doc = "0xe40 - ADC Digital Comparator Range 0"]
    pub dccmp0: DCCMP0,
    #[doc = "0xe44 - ADC Digital Comparator Range 1"]
    pub dccmp1: DCCMP1,
    #[doc = "0xe48 - ADC Digital Comparator Range 2"]
    pub dccmp2: DCCMP2,
    #[doc = "0xe4c - ADC Digital Comparator Range 3"]
    pub dccmp3: DCCMP3,
    #[doc = "0xe50 - ADC Digital Comparator Range 4"]
    pub dccmp4: DCCMP4,
    #[doc = "0xe54 - ADC Digital Comparator Range 5"]
    pub dccmp5: DCCMP5,
    #[doc = "0xe58 - ADC Digital Comparator Range 6"]
    pub dccmp6: DCCMP6,
    #[doc = "0xe5c - ADC Digital Comparator Range 7"]
    pub dccmp7: DCCMP7,
    _reserved63: [u8; 352usize],
    #[doc = "0xfc0 - ADC Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - ADC Peripheral Configuration"]
    pub pc: PC,
    #[doc = "0xfc8 - ADC Clock Configuration"]
    pub cc: CC,
}
#[doc = "ADC Active Sample Sequencer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actss](actss) module"]
pub type ACTSS = crate::Reg<u32, _ACTSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTSS;
#[doc = "`read()` method returns [actss::R](actss::R) reader structure"]
impl crate::Readable for ACTSS {}
#[doc = "`write(|w| ..)` method takes [actss::W](actss::W) writer structure"]
impl crate::Writable for ACTSS {}
#[doc = "ADC Active Sample Sequencer"]
pub mod actss;
#[doc = "ADC Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "ADC Raw Interrupt Status"]
pub mod ris;
#[doc = "ADC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "ADC Interrupt Mask"]
pub mod im;
#[doc = "ADC Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](isc) module"]
pub type ISC = crate::Reg<u32, _ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISC;
#[doc = "`read()` method returns [isc::R](isc::R) reader structure"]
impl crate::Readable for ISC {}
#[doc = "`write(|w| ..)` method takes [isc::W](isc::W) writer structure"]
impl crate::Writable for ISC {}
#[doc = "ADC Interrupt Status and Clear"]
pub mod isc;
#[doc = "ADC Overflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostat](ostat) module"]
pub type OSTAT = crate::Reg<u32, _OSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSTAT;
#[doc = "`read()` method returns [ostat::R](ostat::R) reader structure"]
impl crate::Readable for OSTAT {}
#[doc = "`write(|w| ..)` method takes [ostat::W](ostat::W) writer structure"]
impl crate::Writable for OSTAT {}
#[doc = "ADC Overflow Status"]
pub mod ostat;
#[doc = "ADC Event Multiplexer Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emux](emux) module"]
pub type EMUX = crate::Reg<u32, _EMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUX;
#[doc = "`read()` method returns [emux::R](emux::R) reader structure"]
impl crate::Readable for EMUX {}
#[doc = "`write(|w| ..)` method takes [emux::W](emux::W) writer structure"]
impl crate::Writable for EMUX {}
#[doc = "ADC Event Multiplexer Select"]
pub mod emux;
#[doc = "ADC Underflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ustat](ustat) module"]
pub type USTAT = crate::Reg<u32, _USTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USTAT;
#[doc = "`read()` method returns [ustat::R](ustat::R) reader structure"]
impl crate::Readable for USTAT {}
#[doc = "`write(|w| ..)` method takes [ustat::W](ustat::W) writer structure"]
impl crate::Writable for USTAT {}
#[doc = "ADC Underflow Status"]
pub mod ustat;
#[doc = "ADC Trigger Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tssel](tssel) module"]
pub type TSSEL = crate::Reg<u32, _TSSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSEL;
#[doc = "`read()` method returns [tssel::R](tssel::R) reader structure"]
impl crate::Readable for TSSEL {}
#[doc = "`write(|w| ..)` method takes [tssel::W](tssel::W) writer structure"]
impl crate::Writable for TSSEL {}
#[doc = "ADC Trigger Source Select"]
pub mod tssel;
#[doc = "ADC Sample Sequencer Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspri](sspri) module"]
pub type SSPRI = crate::Reg<u32, _SSPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPRI;
#[doc = "`read()` method returns [sspri::R](sspri::R) reader structure"]
impl crate::Readable for SSPRI {}
#[doc = "`write(|w| ..)` method takes [sspri::W](sspri::W) writer structure"]
impl crate::Writable for SSPRI {}
#[doc = "ADC Sample Sequencer Priority"]
pub mod sspri;
#[doc = "ADC Sample Phase Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spc](spc) module"]
pub type SPC = crate::Reg<u32, _SPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC;
#[doc = "`read()` method returns [spc::R](spc::R) reader structure"]
impl crate::Readable for SPC {}
#[doc = "`write(|w| ..)` method takes [spc::W](spc::W) writer structure"]
impl crate::Writable for SPC {}
#[doc = "ADC Sample Phase Control"]
pub mod spc;
#[doc = "ADC Processor Sample Sequence Initiate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi](pssi) module"]
pub type PSSI = crate::Reg<u32, _PSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSI;
#[doc = "`read()` method returns [pssi::R](pssi::R) reader structure"]
impl crate::Readable for PSSI {}
#[doc = "`write(|w| ..)` method takes [pssi::W](pssi::W) writer structure"]
impl crate::Writable for PSSI {}
#[doc = "ADC Processor Sample Sequence Initiate"]
pub mod pssi;
#[doc = "ADC Sample Averaging Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac](sac) module"]
pub type SAC = crate::Reg<u32, _SAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC;
#[doc = "`read()` method returns [sac::R](sac::R) reader structure"]
impl crate::Readable for SAC {}
#[doc = "`write(|w| ..)` method takes [sac::W](sac::W) writer structure"]
impl crate::Writable for SAC {}
#[doc = "ADC Sample Averaging Control"]
pub mod sac;
#[doc = "ADC Digital Comparator Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcisc](dcisc) module"]
pub type DCISC = crate::Reg<u32, _DCISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCISC;
#[doc = "`read()` method returns [dcisc::R](dcisc::R) reader structure"]
impl crate::Readable for DCISC {}
#[doc = "`write(|w| ..)` method takes [dcisc::W](dcisc::W) writer structure"]
impl crate::Writable for DCISC {}
#[doc = "ADC Digital Comparator Interrupt Status and Clear"]
pub mod dcisc;
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "ADC Control"]
pub mod ctl;
#[doc = "ADC Sample Sequence Input Multiplexer Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux0](ssmux0) module"]
pub type SSMUX0 = crate::Reg<u32, _SSMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX0;
#[doc = "`read()` method returns [ssmux0::R](ssmux0::R) reader structure"]
impl crate::Readable for SSMUX0 {}
#[doc = "`write(|w| ..)` method takes [ssmux0::W](ssmux0::W) writer structure"]
impl crate::Writable for SSMUX0 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub mod ssmux0;
#[doc = "ADC Sample Sequence Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl0](ssctl0) module"]
pub type SSCTL0 = crate::Reg<u32, _SSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL0;
#[doc = "`read()` method returns [ssctl0::R](ssctl0::R) reader structure"]
impl crate::Readable for SSCTL0 {}
#[doc = "`write(|w| ..)` method takes [ssctl0::W](ssctl0::W) writer structure"]
impl crate::Writable for SSCTL0 {}
#[doc = "ADC Sample Sequence Control 0"]
pub mod ssctl0;
#[doc = "ADC Sample Sequence Result FIFO 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo0](ssfifo0) module"]
pub type SSFIFO0 = crate::Reg<u32, _SSFIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO0;
#[doc = "`read()` method returns [ssfifo0::R](ssfifo0::R) reader structure"]
impl crate::Readable for SSFIFO0 {}
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub mod ssfifo0;
#[doc = "ADC Sample Sequence FIFO 0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat0](ssfstat0) module"]
pub type SSFSTAT0 = crate::Reg<u32, _SSFSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT0;
#[doc = "`read()` method returns [ssfstat0::R](ssfstat0::R) reader structure"]
impl crate::Readable for SSFSTAT0 {}
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub mod ssfstat0;
#[doc = "ADC Sample Sequence 0 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop0](ssop0) module"]
pub type SSOP0 = crate::Reg<u32, _SSOP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSOP0;
#[doc = "`read()` method returns [ssop0::R](ssop0::R) reader structure"]
impl crate::Readable for SSOP0 {}
#[doc = "`write(|w| ..)` method takes [ssop0::W](ssop0::W) writer structure"]
impl crate::Writable for SSOP0 {}
#[doc = "ADC Sample Sequence 0 Operation"]
pub mod ssop0;
#[doc = "ADC Sample Sequence 0 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc0](ssdc0) module"]
pub type SSDC0 = crate::Reg<u32, _SSDC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDC0;
#[doc = "`read()` method returns [ssdc0::R](ssdc0::R) reader structure"]
impl crate::Readable for SSDC0 {}
#[doc = "`write(|w| ..)` method takes [ssdc0::W](ssdc0::W) writer structure"]
impl crate::Writable for SSDC0 {}
#[doc = "ADC Sample Sequence 0 Digital Comparator Select"]
pub mod ssdc0;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux0](ssemux0) module"]
pub type SSEMUX0 = crate::Reg<u32, _SSEMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSEMUX0;
#[doc = "`read()` method returns [ssemux0::R](ssemux0::R) reader structure"]
impl crate::Readable for SSEMUX0 {}
#[doc = "`write(|w| ..)` method takes [ssemux0::W](ssemux0::W) writer structure"]
impl crate::Writable for SSEMUX0 {}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0"]
pub mod ssemux0;
#[doc = "ADC Sample Sequence 0 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh0](sstsh0) module"]
pub type SSTSH0 = crate::Reg<u32, _SSTSH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTSH0;
#[doc = "`read()` method returns [sstsh0::R](sstsh0::R) reader structure"]
impl crate::Readable for SSTSH0 {}
#[doc = "`write(|w| ..)` method takes [sstsh0::W](sstsh0::W) writer structure"]
impl crate::Writable for SSTSH0 {}
#[doc = "ADC Sample Sequence 0 Sample and Hold Time"]
pub mod sstsh0;
#[doc = "ADC Sample Sequence Input Multiplexer Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux1](ssmux1) module"]
pub type SSMUX1 = crate::Reg<u32, _SSMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX1;
#[doc = "`read()` method returns [ssmux1::R](ssmux1::R) reader structure"]
impl crate::Readable for SSMUX1 {}
#[doc = "`write(|w| ..)` method takes [ssmux1::W](ssmux1::W) writer structure"]
impl crate::Writable for SSMUX1 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub mod ssmux1;
#[doc = "ADC Sample Sequence Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl1](ssctl1) module"]
pub type SSCTL1 = crate::Reg<u32, _SSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL1;
#[doc = "`read()` method returns [ssctl1::R](ssctl1::R) reader structure"]
impl crate::Readable for SSCTL1 {}
#[doc = "`write(|w| ..)` method takes [ssctl1::W](ssctl1::W) writer structure"]
impl crate::Writable for SSCTL1 {}
#[doc = "ADC Sample Sequence Control 1"]
pub mod ssctl1;
#[doc = "ADC Sample Sequence Result FIFO 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo1](ssfifo1) module"]
pub type SSFIFO1 = crate::Reg<u32, _SSFIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO1;
#[doc = "`read()` method returns [ssfifo1::R](ssfifo1::R) reader structure"]
impl crate::Readable for SSFIFO1 {}
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub mod ssfifo1;
#[doc = "ADC Sample Sequence FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat1](ssfstat1) module"]
pub type SSFSTAT1 = crate::Reg<u32, _SSFSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT1;
#[doc = "`read()` method returns [ssfstat1::R](ssfstat1::R) reader structure"]
impl crate::Readable for SSFSTAT1 {}
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub mod ssfstat1;
#[doc = "ADC Sample Sequence 1 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop1](ssop1) module"]
pub type SSOP1 = crate::Reg<u32, _SSOP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSOP1;
#[doc = "`read()` method returns [ssop1::R](ssop1::R) reader structure"]
impl crate::Readable for SSOP1 {}
#[doc = "`write(|w| ..)` method takes [ssop1::W](ssop1::W) writer structure"]
impl crate::Writable for SSOP1 {}
#[doc = "ADC Sample Sequence 1 Operation"]
pub mod ssop1;
#[doc = "ADC Sample Sequence 1 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc1](ssdc1) module"]
pub type SSDC1 = crate::Reg<u32, _SSDC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDC1;
#[doc = "`read()` method returns [ssdc1::R](ssdc1::R) reader structure"]
impl crate::Readable for SSDC1 {}
#[doc = "`write(|w| ..)` method takes [ssdc1::W](ssdc1::W) writer structure"]
impl crate::Writable for SSDC1 {}
#[doc = "ADC Sample Sequence 1 Digital Comparator Select"]
pub mod ssdc1;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux1](ssemux1) module"]
pub type SSEMUX1 = crate::Reg<u32, _SSEMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSEMUX1;
#[doc = "`read()` method returns [ssemux1::R](ssemux1::R) reader structure"]
impl crate::Readable for SSEMUX1 {}
#[doc = "`write(|w| ..)` method takes [ssemux1::W](ssemux1::W) writer structure"]
impl crate::Writable for SSEMUX1 {}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1"]
pub mod ssemux1;
#[doc = "ADC Sample Sequence 1 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh1](sstsh1) module"]
pub type SSTSH1 = crate::Reg<u32, _SSTSH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTSH1;
#[doc = "`read()` method returns [sstsh1::R](sstsh1::R) reader structure"]
impl crate::Readable for SSTSH1 {}
#[doc = "`write(|w| ..)` method takes [sstsh1::W](sstsh1::W) writer structure"]
impl crate::Writable for SSTSH1 {}
#[doc = "ADC Sample Sequence 1 Sample and Hold Time"]
pub mod sstsh1;
#[doc = "ADC Sample Sequence Input Multiplexer Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux2](ssmux2) module"]
pub type SSMUX2 = crate::Reg<u32, _SSMUX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX2;
#[doc = "`read()` method returns [ssmux2::R](ssmux2::R) reader structure"]
impl crate::Readable for SSMUX2 {}
#[doc = "`write(|w| ..)` method takes [ssmux2::W](ssmux2::W) writer structure"]
impl crate::Writable for SSMUX2 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub mod ssmux2;
#[doc = "ADC Sample Sequence Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl2](ssctl2) module"]
pub type SSCTL2 = crate::Reg<u32, _SSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL2;
#[doc = "`read()` method returns [ssctl2::R](ssctl2::R) reader structure"]
impl crate::Readable for SSCTL2 {}
#[doc = "`write(|w| ..)` method takes [ssctl2::W](ssctl2::W) writer structure"]
impl crate::Writable for SSCTL2 {}
#[doc = "ADC Sample Sequence Control 2"]
pub mod ssctl2;
#[doc = "ADC Sample Sequence Result FIFO 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo2](ssfifo2) module"]
pub type SSFIFO2 = crate::Reg<u32, _SSFIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO2;
#[doc = "`read()` method returns [ssfifo2::R](ssfifo2::R) reader structure"]
impl crate::Readable for SSFIFO2 {}
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub mod ssfifo2;
#[doc = "ADC Sample Sequence FIFO 2 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat2](ssfstat2) module"]
pub type SSFSTAT2 = crate::Reg<u32, _SSFSTAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT2;
#[doc = "`read()` method returns [ssfstat2::R](ssfstat2::R) reader structure"]
impl crate::Readable for SSFSTAT2 {}
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub mod ssfstat2;
#[doc = "ADC Sample Sequence 2 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop2](ssop2) module"]
pub type SSOP2 = crate::Reg<u32, _SSOP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSOP2;
#[doc = "`read()` method returns [ssop2::R](ssop2::R) reader structure"]
impl crate::Readable for SSOP2 {}
#[doc = "`write(|w| ..)` method takes [ssop2::W](ssop2::W) writer structure"]
impl crate::Writable for SSOP2 {}
#[doc = "ADC Sample Sequence 2 Operation"]
pub mod ssop2;
#[doc = "ADC Sample Sequence 2 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc2](ssdc2) module"]
pub type SSDC2 = crate::Reg<u32, _SSDC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDC2;
#[doc = "`read()` method returns [ssdc2::R](ssdc2::R) reader structure"]
impl crate::Readable for SSDC2 {}
#[doc = "`write(|w| ..)` method takes [ssdc2::W](ssdc2::W) writer structure"]
impl crate::Writable for SSDC2 {}
#[doc = "ADC Sample Sequence 2 Digital Comparator Select"]
pub mod ssdc2;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux2](ssemux2) module"]
pub type SSEMUX2 = crate::Reg<u32, _SSEMUX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSEMUX2;
#[doc = "`read()` method returns [ssemux2::R](ssemux2::R) reader structure"]
impl crate::Readable for SSEMUX2 {}
#[doc = "`write(|w| ..)` method takes [ssemux2::W](ssemux2::W) writer structure"]
impl crate::Writable for SSEMUX2 {}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 2"]
pub mod ssemux2;
#[doc = "ADC Sample Sequence 2 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh2](sstsh2) module"]
pub type SSTSH2 = crate::Reg<u32, _SSTSH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTSH2;
#[doc = "`read()` method returns [sstsh2::R](sstsh2::R) reader structure"]
impl crate::Readable for SSTSH2 {}
#[doc = "`write(|w| ..)` method takes [sstsh2::W](sstsh2::W) writer structure"]
impl crate::Writable for SSTSH2 {}
#[doc = "ADC Sample Sequence 2 Sample and Hold Time"]
pub mod sstsh2;
#[doc = "ADC Sample Sequence Input Multiplexer Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux3](ssmux3) module"]
pub type SSMUX3 = crate::Reg<u32, _SSMUX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX3;
#[doc = "`read()` method returns [ssmux3::R](ssmux3::R) reader structure"]
impl crate::Readable for SSMUX3 {}
#[doc = "`write(|w| ..)` method takes [ssmux3::W](ssmux3::W) writer structure"]
impl crate::Writable for SSMUX3 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub mod ssmux3;
#[doc = "ADC Sample Sequence Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl3](ssctl3) module"]
pub type SSCTL3 = crate::Reg<u32, _SSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL3;
#[doc = "`read()` method returns [ssctl3::R](ssctl3::R) reader structure"]
impl crate::Readable for SSCTL3 {}
#[doc = "`write(|w| ..)` method takes [ssctl3::W](ssctl3::W) writer structure"]
impl crate::Writable for SSCTL3 {}
#[doc = "ADC Sample Sequence Control 3"]
pub mod ssctl3;
#[doc = "ADC Sample Sequence Result FIFO 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo3](ssfifo3) module"]
pub type SSFIFO3 = crate::Reg<u32, _SSFIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO3;
#[doc = "`read()` method returns [ssfifo3::R](ssfifo3::R) reader structure"]
impl crate::Readable for SSFIFO3 {}
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub mod ssfifo3;
#[doc = "ADC Sample Sequence FIFO 3 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat3](ssfstat3) module"]
pub type SSFSTAT3 = crate::Reg<u32, _SSFSTAT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT3;
#[doc = "`read()` method returns [ssfstat3::R](ssfstat3::R) reader structure"]
impl crate::Readable for SSFSTAT3 {}
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub mod ssfstat3;
#[doc = "ADC Sample Sequence 3 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop3](ssop3) module"]
pub type SSOP3 = crate::Reg<u32, _SSOP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSOP3;
#[doc = "`read()` method returns [ssop3::R](ssop3::R) reader structure"]
impl crate::Readable for SSOP3 {}
#[doc = "`write(|w| ..)` method takes [ssop3::W](ssop3::W) writer structure"]
impl crate::Writable for SSOP3 {}
#[doc = "ADC Sample Sequence 3 Operation"]
pub mod ssop3;
#[doc = "ADC Sample Sequence 3 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc3](ssdc3) module"]
pub type SSDC3 = crate::Reg<u32, _SSDC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDC3;
#[doc = "`read()` method returns [ssdc3::R](ssdc3::R) reader structure"]
impl crate::Readable for SSDC3 {}
#[doc = "`write(|w| ..)` method takes [ssdc3::W](ssdc3::W) writer structure"]
impl crate::Writable for SSDC3 {}
#[doc = "ADC Sample Sequence 3 Digital Comparator Select"]
pub mod ssdc3;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux3](ssemux3) module"]
pub type SSEMUX3 = crate::Reg<u32, _SSEMUX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSEMUX3;
#[doc = "`read()` method returns [ssemux3::R](ssemux3::R) reader structure"]
impl crate::Readable for SSEMUX3 {}
#[doc = "`write(|w| ..)` method takes [ssemux3::W](ssemux3::W) writer structure"]
impl crate::Writable for SSEMUX3 {}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3"]
pub mod ssemux3;
#[doc = "ADC Sample Sequence 3 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh3](sstsh3) module"]
pub type SSTSH3 = crate::Reg<u32, _SSTSH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTSH3;
#[doc = "`read()` method returns [sstsh3::R](sstsh3::R) reader structure"]
impl crate::Readable for SSTSH3 {}
#[doc = "`write(|w| ..)` method takes [sstsh3::W](sstsh3::W) writer structure"]
impl crate::Writable for SSTSH3 {}
#[doc = "ADC Sample Sequence 3 Sample and Hold Time"]
pub mod sstsh3;
#[doc = "ADC Digital Comparator Reset Initial Conditions\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcric](dcric) module"]
pub type DCRIC = crate::Reg<u32, _DCRIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRIC;
#[doc = "`write(|w| ..)` method takes [dcric::W](dcric::W) writer structure"]
impl crate::Writable for DCRIC {}
#[doc = "ADC Digital Comparator Reset Initial Conditions"]
pub mod dcric;
#[doc = "ADC Digital Comparator Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl0](dcctl0) module"]
pub type DCCTL0 = crate::Reg<u32, _DCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL0;
#[doc = "`read()` method returns [dcctl0::R](dcctl0::R) reader structure"]
impl crate::Readable for DCCTL0 {}
#[doc = "`write(|w| ..)` method takes [dcctl0::W](dcctl0::W) writer structure"]
impl crate::Writable for DCCTL0 {}
#[doc = "ADC Digital Comparator Control 0"]
pub mod dcctl0;
#[doc = "ADC Digital Comparator Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl1](dcctl1) module"]
pub type DCCTL1 = crate::Reg<u32, _DCCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL1;
#[doc = "`read()` method returns [dcctl1::R](dcctl1::R) reader structure"]
impl crate::Readable for DCCTL1 {}
#[doc = "`write(|w| ..)` method takes [dcctl1::W](dcctl1::W) writer structure"]
impl crate::Writable for DCCTL1 {}
#[doc = "ADC Digital Comparator Control 1"]
pub mod dcctl1;
#[doc = "ADC Digital Comparator Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl2](dcctl2) module"]
pub type DCCTL2 = crate::Reg<u32, _DCCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL2;
#[doc = "`read()` method returns [dcctl2::R](dcctl2::R) reader structure"]
impl crate::Readable for DCCTL2 {}
#[doc = "`write(|w| ..)` method takes [dcctl2::W](dcctl2::W) writer structure"]
impl crate::Writable for DCCTL2 {}
#[doc = "ADC Digital Comparator Control 2"]
pub mod dcctl2;
#[doc = "ADC Digital Comparator Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl3](dcctl3) module"]
pub type DCCTL3 = crate::Reg<u32, _DCCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL3;
#[doc = "`read()` method returns [dcctl3::R](dcctl3::R) reader structure"]
impl crate::Readable for DCCTL3 {}
#[doc = "`write(|w| ..)` method takes [dcctl3::W](dcctl3::W) writer structure"]
impl crate::Writable for DCCTL3 {}
#[doc = "ADC Digital Comparator Control 3"]
pub mod dcctl3;
#[doc = "ADC Digital Comparator Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl4](dcctl4) module"]
pub type DCCTL4 = crate::Reg<u32, _DCCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL4;
#[doc = "`read()` method returns [dcctl4::R](dcctl4::R) reader structure"]
impl crate::Readable for DCCTL4 {}
#[doc = "`write(|w| ..)` method takes [dcctl4::W](dcctl4::W) writer structure"]
impl crate::Writable for DCCTL4 {}
#[doc = "ADC Digital Comparator Control 4"]
pub mod dcctl4;
#[doc = "ADC Digital Comparator Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl5](dcctl5) module"]
pub type DCCTL5 = crate::Reg<u32, _DCCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL5;
#[doc = "`read()` method returns [dcctl5::R](dcctl5::R) reader structure"]
impl crate::Readable for DCCTL5 {}
#[doc = "`write(|w| ..)` method takes [dcctl5::W](dcctl5::W) writer structure"]
impl crate::Writable for DCCTL5 {}
#[doc = "ADC Digital Comparator Control 5"]
pub mod dcctl5;
#[doc = "ADC Digital Comparator Control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl6](dcctl6) module"]
pub type DCCTL6 = crate::Reg<u32, _DCCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL6;
#[doc = "`read()` method returns [dcctl6::R](dcctl6::R) reader structure"]
impl crate::Readable for DCCTL6 {}
#[doc = "`write(|w| ..)` method takes [dcctl6::W](dcctl6::W) writer structure"]
impl crate::Writable for DCCTL6 {}
#[doc = "ADC Digital Comparator Control 6"]
pub mod dcctl6;
#[doc = "ADC Digital Comparator Control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl7](dcctl7) module"]
pub type DCCTL7 = crate::Reg<u32, _DCCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCTL7;
#[doc = "`read()` method returns [dcctl7::R](dcctl7::R) reader structure"]
impl crate::Readable for DCCTL7 {}
#[doc = "`write(|w| ..)` method takes [dcctl7::W](dcctl7::W) writer structure"]
impl crate::Writable for DCCTL7 {}
#[doc = "ADC Digital Comparator Control 7"]
pub mod dcctl7;
#[doc = "ADC Digital Comparator Range 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp0](dccmp0) module"]
pub type DCCMP0 = crate::Reg<u32, _DCCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP0;
#[doc = "`read()` method returns [dccmp0::R](dccmp0::R) reader structure"]
impl crate::Readable for DCCMP0 {}
#[doc = "`write(|w| ..)` method takes [dccmp0::W](dccmp0::W) writer structure"]
impl crate::Writable for DCCMP0 {}
#[doc = "ADC Digital Comparator Range 0"]
pub mod dccmp0;
#[doc = "ADC Digital Comparator Range 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp1](dccmp1) module"]
pub type DCCMP1 = crate::Reg<u32, _DCCMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP1;
#[doc = "`read()` method returns [dccmp1::R](dccmp1::R) reader structure"]
impl crate::Readable for DCCMP1 {}
#[doc = "`write(|w| ..)` method takes [dccmp1::W](dccmp1::W) writer structure"]
impl crate::Writable for DCCMP1 {}
#[doc = "ADC Digital Comparator Range 1"]
pub mod dccmp1;
#[doc = "ADC Digital Comparator Range 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp2](dccmp2) module"]
pub type DCCMP2 = crate::Reg<u32, _DCCMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP2;
#[doc = "`read()` method returns [dccmp2::R](dccmp2::R) reader structure"]
impl crate::Readable for DCCMP2 {}
#[doc = "`write(|w| ..)` method takes [dccmp2::W](dccmp2::W) writer structure"]
impl crate::Writable for DCCMP2 {}
#[doc = "ADC Digital Comparator Range 2"]
pub mod dccmp2;
#[doc = "ADC Digital Comparator Range 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp3](dccmp3) module"]
pub type DCCMP3 = crate::Reg<u32, _DCCMP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP3;
#[doc = "`read()` method returns [dccmp3::R](dccmp3::R) reader structure"]
impl crate::Readable for DCCMP3 {}
#[doc = "`write(|w| ..)` method takes [dccmp3::W](dccmp3::W) writer structure"]
impl crate::Writable for DCCMP3 {}
#[doc = "ADC Digital Comparator Range 3"]
pub mod dccmp3;
#[doc = "ADC Digital Comparator Range 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp4](dccmp4) module"]
pub type DCCMP4 = crate::Reg<u32, _DCCMP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP4;
#[doc = "`read()` method returns [dccmp4::R](dccmp4::R) reader structure"]
impl crate::Readable for DCCMP4 {}
#[doc = "`write(|w| ..)` method takes [dccmp4::W](dccmp4::W) writer structure"]
impl crate::Writable for DCCMP4 {}
#[doc = "ADC Digital Comparator Range 4"]
pub mod dccmp4;
#[doc = "ADC Digital Comparator Range 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp5](dccmp5) module"]
pub type DCCMP5 = crate::Reg<u32, _DCCMP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP5;
#[doc = "`read()` method returns [dccmp5::R](dccmp5::R) reader structure"]
impl crate::Readable for DCCMP5 {}
#[doc = "`write(|w| ..)` method takes [dccmp5::W](dccmp5::W) writer structure"]
impl crate::Writable for DCCMP5 {}
#[doc = "ADC Digital Comparator Range 5"]
pub mod dccmp5;
#[doc = "ADC Digital Comparator Range 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp6](dccmp6) module"]
pub type DCCMP6 = crate::Reg<u32, _DCCMP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP6;
#[doc = "`read()` method returns [dccmp6::R](dccmp6::R) reader structure"]
impl crate::Readable for DCCMP6 {}
#[doc = "`write(|w| ..)` method takes [dccmp6::W](dccmp6::W) writer structure"]
impl crate::Writable for DCCMP6 {}
#[doc = "ADC Digital Comparator Range 6"]
pub mod dccmp6;
#[doc = "ADC Digital Comparator Range 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp7](dccmp7) module"]
pub type DCCMP7 = crate::Reg<u32, _DCCMP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMP7;
#[doc = "`read()` method returns [dccmp7::R](dccmp7::R) reader structure"]
impl crate::Readable for DCCMP7 {}
#[doc = "`write(|w| ..)` method takes [dccmp7::W](dccmp7::W) writer structure"]
impl crate::Writable for DCCMP7 {}
#[doc = "ADC Digital Comparator Range 7"]
pub mod dccmp7;
#[doc = "ADC Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "ADC Peripheral Properties"]
pub mod pp;
#[doc = "ADC Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "ADC Peripheral Configuration"]
pub mod pc;
#[doc = "ADC Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "ADC Clock Configuration"]
pub mod cc;
