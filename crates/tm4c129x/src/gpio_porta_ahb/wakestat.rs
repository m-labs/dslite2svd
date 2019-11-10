#[doc = "Reader of register WAKESTAT"]
pub type R = crate::R<u32, super::WAKESTAT>;
#[doc = "Reader of field `STAT4`"]
pub type STAT4_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - P\\[4\\] Wake Status"]
    #[inline(always)]
    pub fn stat4(&self) -> STAT4_R {
        STAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
