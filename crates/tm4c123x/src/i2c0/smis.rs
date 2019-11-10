#[doc = "Reader of register SMIS"]
pub type R = crate::R<u32, super::SMIS>;
#[doc = "Reader of field `DATAMIS`"]
pub type DATAMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTMIS`"]
pub type STARTMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOPMIS`"]
pub type STOPMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
