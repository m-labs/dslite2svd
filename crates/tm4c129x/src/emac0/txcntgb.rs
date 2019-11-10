#[doc = "Reader of register TXCNTGB"]
pub type R = crate::R<u32, super::TXCNTGB>;
#[doc = "Reader of field `TXFRMGB`"]
pub type TXFRMGB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn txfrmgb(&self) -> TXFRMGB_R {
        TXFRMGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
