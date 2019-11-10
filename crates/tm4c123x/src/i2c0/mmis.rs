#[doc = "Reader of register MMIS"]
pub type R = crate::R<u32, super::MMIS>;
#[doc = "Reader of field `MIS`"]
pub type MIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKMIS`"]
pub type CLKMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn clkmis(&self) -> CLKMIS_R {
        CLKMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
