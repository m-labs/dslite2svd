#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `TATORIS`"]
pub type TATORIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAMRIS`"]
pub type CAMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAERIS`"]
pub type CAERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCRIS`"]
pub type RTCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMRIS`"]
pub type TAMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBTORIS`"]
pub type TBTORIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBMRIS`"]
pub type CBMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBERIS`"]
pub type CBERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBMRIS`"]
pub type TBMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUERIS`"]
pub type WUERIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn tatoris(&self) -> TATORIS_R {
        TATORIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn camris(&self) -> CAMRIS_R {
        CAMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn caeris(&self) -> CAERIS_R {
        CAERIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn rtcris(&self) -> RTCRIS_R {
        RTCRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn tbtoris(&self) -> TBTORIS_R {
        TBTORIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn cbmris(&self) -> CBMRIS_R {
        CBMRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn cberis(&self) -> CBERIS_R {
        CBERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn tbmris(&self) -> TBMRIS_R {
        TBMRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 32/64-Bit Wide GPTM Write Update Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn wueris(&self) -> WUERIS_R {
        WUERIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
