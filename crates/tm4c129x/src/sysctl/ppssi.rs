#[doc = "Reader of register PPSSI"]
pub type R = crate::R<u32, super::PPSSI>;
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1`"]
pub type P1_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2`"]
pub type P2_R = crate::R<bool, bool>;
#[doc = "Reader of field `P3`"]
pub type P3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Present"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Present"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
