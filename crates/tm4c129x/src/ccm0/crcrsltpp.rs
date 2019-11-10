#[doc = "Reader of register CRCRSLTPP"]
pub type R = crate::R<u32, super::CRCRSLTPP>;
#[doc = "Reader of field `RSLTPP`"]
pub type RSLTPP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Post Processing Result"]
    #[inline(always)]
    pub fn rsltpp(&self) -> RSLTPP_R {
        RSLTPP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
