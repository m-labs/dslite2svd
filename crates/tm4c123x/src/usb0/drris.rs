#[doc = "Reader of register DRRIS"]
pub type R = crate::R<u32, super::DRRIS>;
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RESUME Interrupt Status"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new((self.bits & 0x01) != 0)
    }
}
