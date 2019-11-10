#[doc = "Reader of register PRGPIO"]
pub type R = crate::R<u32, super::PRGPIO>;
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
#[doc = "Reader of field `R8`"]
pub type R8_R = crate::R<bool, bool>;
#[doc = "Reader of field `R9`"]
pub type R9_R = crate::R<bool, bool>;
#[doc = "Reader of field `R10`"]
pub type R10_R = crate::R<bool, bool>;
#[doc = "Reader of field `R11`"]
pub type R11_R = crate::R<bool, bool>;
#[doc = "Reader of field `R12`"]
pub type R12_R = crate::R<bool, bool>;
#[doc = "Reader of field `R13`"]
pub type R13_R = crate::R<bool, bool>;
#[doc = "Reader of field `R14`"]
pub type R14_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Peripheral Ready"]
    #[inline(always)]
    pub fn r6(&self) -> R6_R {
        R6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Peripheral Ready"]
    #[inline(always)]
    pub fn r7(&self) -> R7_R {
        R7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Peripheral Ready"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Peripheral Ready"]
    #[inline(always)]
    pub fn r9(&self) -> R9_R {
        R9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Peripheral Ready"]
    #[inline(always)]
    pub fn r10(&self) -> R10_R {
        R10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Peripheral Ready"]
    #[inline(always)]
    pub fn r11(&self) -> R11_R {
        R11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Peripheral Ready"]
    #[inline(always)]
    pub fn r12(&self) -> R12_R {
        R12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Peripheral Ready"]
    #[inline(always)]
    pub fn r13(&self) -> R13_R {
        R13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Peripheral Ready"]
    #[inline(always)]
    pub fn r14(&self) -> R14_R {
        R14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
