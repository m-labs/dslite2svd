#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `PMT`"]
pub type PMT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMC`"]
pub type MMC_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCRX`"]
pub type MMCRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCTX`"]
pub type MMCTX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmt(&self) -> PMT_R {
        PMT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrx(&self) -> MMCRX_R {
        MMCRX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctx(&self) -> MMCTX_R {
        MMCTX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
