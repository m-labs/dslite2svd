#[doc = "Reader of register EPHYRIS"]
pub type R = crate::R<u32, super::EPHYRIS>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Raw Interrupt Status"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
}
