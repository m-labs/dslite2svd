#[doc = "Reader of register EPCRIS"]
pub type R = crate::R<u32, super::EPCRIS>;
#[doc = "Reader of field `PF`"]
pub type PF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0x01) != 0)
    }
}
