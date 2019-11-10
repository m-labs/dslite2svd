#[doc = "Reader of register ACRIS"]
pub type R = crate::R<u32, super::ACRIS>;
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Status"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Status"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
