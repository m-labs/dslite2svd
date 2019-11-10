#[doc = "Reader of register HOSTXBA"]
pub type R = crate::R<u32, super::HOSTXBA>;
#[doc = "Reader of field `CURTXBUFA`"]
pub type CURTXBUFA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtxbufa(&self) -> CURTXBUFA_R {
        CURTXBUFA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
