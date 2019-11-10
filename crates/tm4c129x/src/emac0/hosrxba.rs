#[doc = "Reader of register HOSRXBA"]
pub type R = crate::R<u32, super::HOSRXBA>;
#[doc = "Reader of field `CURRXBUFA`"]
pub type CURRXBUFA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currxbufa(&self) -> CURRXBUFA_R {
        CURRXBUFA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
