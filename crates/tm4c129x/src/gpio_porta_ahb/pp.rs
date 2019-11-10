#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `EDE`"]
pub type EDE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Extended Drive Enable"]
    #[inline(always)]
    pub fn ede(&self) -> EDE_R {
        EDE_R::new((self.bits & 0x01) != 0)
    }
}
