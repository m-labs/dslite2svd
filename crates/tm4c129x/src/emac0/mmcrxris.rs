#[doc = "Reader of register MMCRXRIS"]
pub type R = crate::R<u32, super::MMCRXRIS>;
#[doc = "Reader of field `GBF`"]
pub type GBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALGNERR`"]
pub type ALGNERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `UCGF`"]
pub type UCGF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn gbf(&self) -> GBF_R {
        GBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn algnerr(&self) -> ALGNERR_R {
        ALGNERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn ucgf(&self) -> UCGF_R {
        UCGF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
