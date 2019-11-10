#[doc = "Reader of register PPWD"]
pub type R = crate::R<u32, super::PPWD>;
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1`"]
pub type P1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Present"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
