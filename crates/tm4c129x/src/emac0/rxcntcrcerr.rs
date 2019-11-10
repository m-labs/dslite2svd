#[doc = "Reader of register RXCNTCRCERR"]
pub type R = crate::R<u32, super::RXCNTCRCERR>;
#[doc = "Reader of field `RXCRCERR`"]
pub type RXCRCERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with CRC error"]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
