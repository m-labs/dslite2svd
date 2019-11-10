#[doc = "Reader of register VDCRIS"]
pub type R = crate::R<u32, super::VDCRIS>;
#[doc = "Reader of field `VD`"]
pub type VD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Raw Interrupt Status"]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 0x01) != 0)
    }
}
