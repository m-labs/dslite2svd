#[doc = "Reader of register RXCNTGB"]
pub type R = crate::R<u32, super::RXCNTGB>;
#[doc = "Reader of field `RXFRMGB`"]
pub type RXFRMGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames"]
    #[inline(always)]
    pub fn rxfrmgb(&self) -> RXFRMGB_R {
        RXFRMGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
