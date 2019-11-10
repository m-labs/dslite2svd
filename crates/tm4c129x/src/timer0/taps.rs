#[doc = "Reader of register TAPS"]
pub type R = crate::R<u32, super::TAPS>;
#[doc = "Reader of field `PSS`"]
pub type PSS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
