#[doc = "Reader of register TPLOG6"]
pub type R = crate::R<u32, super::TPLOG6>;
#[doc = "Reader of field `TIME`"]
pub type TIME_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tamper Log Calendar Information"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
