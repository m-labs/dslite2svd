#[doc = "Reader of register TXCNTMCOL"]
pub type R = crate::R<u32, super::TXCNTMCOL>;
#[doc = "Reader of field `TXMULTCOLG`"]
pub type TXMULTCOLG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline(always)]
    pub fn txmultcolg(&self) -> TXMULTCOLG_R {
        TXMULTCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
