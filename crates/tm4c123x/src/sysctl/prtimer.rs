#[doc = "Reader of register PRTIMER"]
pub type R = crate::R<u32, super::PRTIMER>;
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Reader of field `R1`"]
pub type R1_R = crate::R<bool, bool>;
#[doc = "Reader of field `R2`"]
pub type R2_R = crate::R<bool, bool>;
#[doc = "Reader of field `R3`"]
pub type R3_R = crate::R<bool, bool>;
#[doc = "Reader of field `R4`"]
pub type R4_R = crate::R<bool, bool>;
#[doc = "Reader of field `R5`"]
pub type R5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
