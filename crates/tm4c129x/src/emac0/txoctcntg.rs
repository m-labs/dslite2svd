#[doc = "Reader of register TXOCTCNTG"]
pub type R = crate::R<u32, super::TXOCTCNTG>;
#[doc = "Reader of field `TXOCTG`"]
pub type TXOCTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline(always)]
    pub fn txoctg(&self) -> TXOCTG_R {
        TXOCTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
