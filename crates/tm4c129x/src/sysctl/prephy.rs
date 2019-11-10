#[doc = "Reader of register PREPHY"]
pub type R = crate::R<u32, super::PREPHY>;
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Module Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
}
