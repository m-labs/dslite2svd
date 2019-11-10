#[doc = "Reader of register MMCTXRIS"]
pub type R = crate::R<u32, super::MMCTXRIS>;
#[doc = "Reader of field `GBF`"]
pub type GBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCOLLGF`"]
pub type SCOLLGF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCOLLGF`"]
pub type MCOLLGF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OCTCNT`"]
pub type OCTCNT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn gbf(&self) -> GBF_R {
        GBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn scollgf(&self) -> SCOLLGF_R {
        SCOLLGF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn mcollgf(&self) -> MCOLLGF_R {
        MCOLLGF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn octcnt(&self) -> OCTCNT_R {
        OCTCNT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
