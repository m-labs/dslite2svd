#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Master Slave Address"]
    pub msa: MSA,
    #[doc = "0x04 - I2C Master Control/Status"]
    pub mcs: MCS,
    #[doc = "0x08 - I2C Master Data"]
    pub mdr: MDR,
    #[doc = "0x0c - I2C Master Timer Period"]
    pub mtpr: MTPR,
    #[doc = "0x10 - I2C Master Interrupt Mask"]
    pub mimr: MIMR,
    #[doc = "0x14 - I2C Master Raw Interrupt Status"]
    pub mris: MRIS,
    #[doc = "0x18 - I2C Master Masked Interrupt Status"]
    pub mmis: MMIS,
    #[doc = "0x1c - I2C Master Interrupt Clear"]
    pub micr: MICR,
    #[doc = "0x20 - I2C Master Configuration"]
    pub mcr: MCR,
    #[doc = "0x24 - I2C Master Clock Low Timeout Count"]
    pub mclkocnt: MCLKOCNT,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - I2C Master Bus Monitor"]
    pub mbmon: MBMON,
    #[doc = "0x30 - I2C Master Burst Length"]
    pub mblen: MBLEN,
    #[doc = "0x34 - I2C Master Burst Count"]
    pub mbcnt: MBCNT,
    _reserved13: [u8; 1992usize],
    #[doc = "0x800 - I2C Slave Own Address"]
    pub soar: SOAR,
    #[doc = "0x804 - I2C Slave Control/Status"]
    pub scsr: SCSR,
    #[doc = "0x808 - I2C Slave Data"]
    pub sdr: SDR,
    #[doc = "0x80c - I2C Slave Interrupt Mask"]
    pub simr: SIMR,
    #[doc = "0x810 - I2C Slave Raw Interrupt Status"]
    pub sris: SRIS,
    #[doc = "0x814 - I2C Slave Masked Interrupt Status"]
    pub smis: SMIS,
    #[doc = "0x818 - I2C Slave Interrupt Clear"]
    pub sicr: SICR,
    #[doc = "0x81c - I2C Slave Own Address 2"]
    pub soar2: SOAR2,
    #[doc = "0x820 - I2C Slave ACK Control"]
    pub sackctl: SACKCTL,
    _reserved22: [u8; 1756usize],
    #[doc = "0xf00 - I2C FIFO Data"]
    pub fifodata: FIFODATA,
    #[doc = "0xf04 - I2C FIFO Control"]
    pub fifoctl: FIFOCTL,
    #[doc = "0xf08 - I2C FIFO Status"]
    pub fifostatus: FIFOSTATUS,
    _reserved25: [u8; 180usize],
    #[doc = "0xfc0 - I2C Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - I2C Peripheral Configuration"]
    pub pc: PC,
}
#[doc = "I2C Master Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msa](msa) module"]
pub type MSA = crate::Reg<u32, _MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSA;
#[doc = "`read()` method returns [msa::R](msa::R) reader structure"]
impl crate::Readable for MSA {}
#[doc = "`write(|w| ..)` method takes [msa::W](msa::W) writer structure"]
impl crate::Writable for MSA {}
#[doc = "I2C Master Slave Address"]
pub mod msa;
#[doc = "I2C Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcs](mcs) module"]
pub type MCS = crate::Reg<u32, _MCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCS;
#[doc = "`read()` method returns [mcs::R](mcs::R) reader structure"]
impl crate::Readable for MCS {}
#[doc = "`write(|w| ..)` method takes [mcs::W](mcs::W) writer structure"]
impl crate::Writable for MCS {}
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C Master Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "I2C Master Data"]
pub mod mdr;
#[doc = "I2C Master Timer Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtpr](mtpr) module"]
pub type MTPR = crate::Reg<u32, _MTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTPR;
#[doc = "`read()` method returns [mtpr::R](mtpr::R) reader structure"]
impl crate::Readable for MTPR {}
#[doc = "`write(|w| ..)` method takes [mtpr::W](mtpr::W) writer structure"]
impl crate::Writable for MTPR {}
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "I2C Master Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mimr](mimr) module"]
pub type MIMR = crate::Reg<u32, _MIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIMR;
#[doc = "`read()` method returns [mimr::R](mimr::R) reader structure"]
impl crate::Readable for MIMR {}
#[doc = "`write(|w| ..)` method takes [mimr::W](mimr::W) writer structure"]
impl crate::Writable for MIMR {}
#[doc = "I2C Master Interrupt Mask"]
pub mod mimr;
#[doc = "I2C Master Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mris](mris) module"]
pub type MRIS = crate::Reg<u32, _MRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRIS;
#[doc = "`read()` method returns [mris::R](mris::R) reader structure"]
impl crate::Readable for MRIS {}
#[doc = "I2C Master Raw Interrupt Status"]
pub mod mris;
#[doc = "I2C Master Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmis](mmis) module"]
pub type MMIS = crate::Reg<u32, _MMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMIS;
#[doc = "`read()` method returns [mmis::R](mmis::R) reader structure"]
impl crate::Readable for MMIS {}
#[doc = "I2C Master Masked Interrupt Status"]
pub mod mmis;
#[doc = "I2C Master Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [micr](micr) module"]
pub type MICR = crate::Reg<u32, _MICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICR;
#[doc = "`write(|w| ..)` method takes [micr::W](micr::W) writer structure"]
impl crate::Writable for MICR {}
#[doc = "I2C Master Interrupt Clear"]
pub mod micr;
#[doc = "I2C Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "I2C Master Configuration"]
pub mod mcr;
#[doc = "I2C Master Clock Low Timeout Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclkocnt](mclkocnt) module"]
pub type MCLKOCNT = crate::Reg<u32, _MCLKOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKOCNT;
#[doc = "`read()` method returns [mclkocnt::R](mclkocnt::R) reader structure"]
impl crate::Readable for MCLKOCNT {}
#[doc = "`write(|w| ..)` method takes [mclkocnt::W](mclkocnt::W) writer structure"]
impl crate::Writable for MCLKOCNT {}
#[doc = "I2C Master Clock Low Timeout Count"]
pub mod mclkocnt;
#[doc = "I2C Master Bus Monitor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mbmon](mbmon) module"]
pub type MBMON = crate::Reg<u32, _MBMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBMON;
#[doc = "`read()` method returns [mbmon::R](mbmon::R) reader structure"]
impl crate::Readable for MBMON {}
#[doc = "I2C Master Bus Monitor"]
pub mod mbmon;
#[doc = "I2C Master Burst Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mblen](mblen) module"]
pub type MBLEN = crate::Reg<u32, _MBLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBLEN;
#[doc = "`read()` method returns [mblen::R](mblen::R) reader structure"]
impl crate::Readable for MBLEN {}
#[doc = "`write(|w| ..)` method takes [mblen::W](mblen::W) writer structure"]
impl crate::Writable for MBLEN {}
#[doc = "I2C Master Burst Length"]
pub mod mblen;
#[doc = "I2C Master Burst Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mbcnt](mbcnt) module"]
pub type MBCNT = crate::Reg<u32, _MBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBCNT;
#[doc = "`read()` method returns [mbcnt::R](mbcnt::R) reader structure"]
impl crate::Readable for MBCNT {}
#[doc = "I2C Master Burst Count"]
pub mod mbcnt;
#[doc = "I2C Slave Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [soar](soar) module"]
pub type SOAR = crate::Reg<u32, _SOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR;
#[doc = "`read()` method returns [soar::R](soar::R) reader structure"]
impl crate::Readable for SOAR {}
#[doc = "`write(|w| ..)` method takes [soar::W](soar::W) writer structure"]
impl crate::Writable for SOAR {}
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "I2C Slave Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C Slave Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdr](sdr) module"]
pub type SDR = crate::Reg<u32, _SDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDR;
#[doc = "`read()` method returns [sdr::R](sdr::R) reader structure"]
impl crate::Readable for SDR {}
#[doc = "`write(|w| ..)` method takes [sdr::W](sdr::W) writer structure"]
impl crate::Writable for SDR {}
#[doc = "I2C Slave Data"]
pub mod sdr;
#[doc = "I2C Slave Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [simr](simr) module"]
pub type SIMR = crate::Reg<u32, _SIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIMR;
#[doc = "`read()` method returns [simr::R](simr::R) reader structure"]
impl crate::Readable for SIMR {}
#[doc = "`write(|w| ..)` method takes [simr::W](simr::W) writer structure"]
impl crate::Writable for SIMR {}
#[doc = "I2C Slave Interrupt Mask"]
pub mod simr;
#[doc = "I2C Slave Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sris](sris) module"]
pub type SRIS = crate::Reg<u32, _SRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIS;
#[doc = "`read()` method returns [sris::R](sris::R) reader structure"]
impl crate::Readable for SRIS {}
#[doc = "I2C Slave Raw Interrupt Status"]
pub mod sris;
#[doc = "I2C Slave Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smis](smis) module"]
pub type SMIS = crate::Reg<u32, _SMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMIS;
#[doc = "`read()` method returns [smis::R](smis::R) reader structure"]
impl crate::Readable for SMIS {}
#[doc = "I2C Slave Masked Interrupt Status"]
pub mod smis;
#[doc = "I2C Slave Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sicr](sicr) module"]
pub type SICR = crate::Reg<u32, _SICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SICR;
#[doc = "`write(|w| ..)` method takes [sicr::W](sicr::W) writer structure"]
impl crate::Writable for SICR {}
#[doc = "I2C Slave Interrupt Clear"]
pub mod sicr;
#[doc = "I2C Slave Own Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [soar2](soar2) module"]
pub type SOAR2 = crate::Reg<u32, _SOAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR2;
#[doc = "`read()` method returns [soar2::R](soar2::R) reader structure"]
impl crate::Readable for SOAR2 {}
#[doc = "`write(|w| ..)` method takes [soar2::W](soar2::W) writer structure"]
impl crate::Writable for SOAR2 {}
#[doc = "I2C Slave Own Address 2"]
pub mod soar2;
#[doc = "I2C Slave ACK Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sackctl](sackctl) module"]
pub type SACKCTL = crate::Reg<u32, _SACKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SACKCTL;
#[doc = "`read()` method returns [sackctl::R](sackctl::R) reader structure"]
impl crate::Readable for SACKCTL {}
#[doc = "`write(|w| ..)` method takes [sackctl::W](sackctl::W) writer structure"]
impl crate::Writable for SACKCTL {}
#[doc = "I2C Slave ACK Control"]
pub mod sackctl;
#[doc = "I2C FIFO Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifodata](fifodata) module"]
pub type FIFODATA = crate::Reg<u32, _FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFODATA;
#[doc = "`read()` method returns [fifodata::R](fifodata::R) reader structure"]
impl crate::Readable for FIFODATA {}
#[doc = "`write(|w| ..)` method takes [fifodata::W](fifodata::W) writer structure"]
impl crate::Writable for FIFODATA {}
#[doc = "I2C FIFO Data"]
pub mod fifodata;
#[doc = "I2C FIFO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifoctl](fifoctl) module"]
pub type FIFOCTL = crate::Reg<u32, _FIFOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCTL;
#[doc = "`read()` method returns [fifoctl::R](fifoctl::R) reader structure"]
impl crate::Readable for FIFOCTL {}
#[doc = "`write(|w| ..)` method takes [fifoctl::W](fifoctl::W) writer structure"]
impl crate::Writable for FIFOCTL {}
#[doc = "I2C FIFO Control"]
pub mod fifoctl;
#[doc = "I2C FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifostatus](fifostatus) module"]
pub type FIFOSTATUS = crate::Reg<u32, _FIFOSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOSTATUS;
#[doc = "`read()` method returns [fifostatus::R](fifostatus::R) reader structure"]
impl crate::Readable for FIFOSTATUS {}
#[doc = "I2C FIFO Status"]
pub mod fifostatus;
#[doc = "I2C Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "I2C Peripheral Properties"]
pub mod pp;
#[doc = "I2C Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "I2C Peripheral Configuration"]
pub mod pc;
