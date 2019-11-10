#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `FAULT0`"]
pub type FAULT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAULT1`"]
pub type FAULT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Generator 0 Fault Status"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Generator 1 Fault Status"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
