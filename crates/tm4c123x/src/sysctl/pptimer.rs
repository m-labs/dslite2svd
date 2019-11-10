#[doc = "Reader of register PPTIMER"]
pub type R = crate::R<u32, super::PPTIMER>;
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1`"]
pub type P1_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2`"]
pub type P2_R = crate::R<bool, bool>;
#[doc = "Reader of field `P3`"]
pub type P3_R = crate::R<bool, bool>;
#[doc = "Reader of field `P4`"]
pub type P4_R = crate::R<bool, bool>;
#[doc = "Reader of field `P5`"]
pub type P5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Present"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Present"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Present"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Present"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Present"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Present"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
