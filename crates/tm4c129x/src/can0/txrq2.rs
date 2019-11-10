#[doc = "Reader of register TXRQ2"]
pub type R = crate::R<u32, super::TXRQ2>;
#[doc = "Reader of field `TXRQST`"]
pub type TXRQST_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission Request Bits"]
    #[inline(always)]
    pub fn txrqst(&self) -> TXRQST_R {
        TXRQST_R::new((self.bits & 0xffff) as u16)
    }
}
