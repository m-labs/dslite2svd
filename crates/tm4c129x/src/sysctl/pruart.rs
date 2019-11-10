#[doc = "Reader of register PRUART"]
pub type R = crate::R<u32, super::PRUART>;
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
#[doc = "Reader of field `R6`"]
pub type R6_R = crate::R<bool, bool>;
#[doc = "Reader of field `R7`"]
pub type R7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Peripheral Ready"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Peripheral Ready"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Peripheral Ready"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Peripheral Ready"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Peripheral Ready"]
    #[inline(always)]
    pub fn r6(&self) -> R6_R {
        R6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Peripheral Ready"]
    #[inline(always)]
    pub fn r7(&self) -> R7_R {
        R7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
