#[doc = "Reader of register SYSPROP"]
pub type R = crate::R<u32, super::SYSPROP>;
#[doc = "Reader of field `FPU`"]
pub type FPU_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FPU Present"]
    #[inline(always)]
    pub fn fpu(&self) -> FPU_R {
        FPU_R::new((self.bits & 0x01) != 0)
    }
}
