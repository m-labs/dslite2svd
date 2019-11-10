#[doc = "Reader of register TXCNTSCOL"]
pub type R = crate::R<u32, super::TXCNTSCOL>;
#[doc = "Reader of field `TXSNGLCOLG`"]
pub type TXSNGLCOLG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
