#[doc = "Reader of register RXCNTALGNERR"]
pub type R = crate::R<u32, super::RXCNTALGNERR>;
#[doc = "Reader of field `RXALGNERR`"]
pub type RXALGNERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with alignment (dribble) error"]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RXALGNERR_R {
        RXALGNERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
