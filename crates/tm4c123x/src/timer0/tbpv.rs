#[doc = "Reader of register TBPV"]
pub type R = crate::R<u32, super::TBPV>;
#[doc = "Reader of field `PSV`"]
pub type PSV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B Prescaler Value"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new((self.bits & 0xffff) as u16)
    }
}
