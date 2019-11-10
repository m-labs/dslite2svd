#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - High-Speed Capable"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 0x01) != 0)
    }
}
