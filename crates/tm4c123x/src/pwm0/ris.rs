#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `INTPWM0`"]
pub type INTPWM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTPWM1`"]
pub type INTPWM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTPWM2`"]
pub type INTPWM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTPWM3`"]
pub type INTPWM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTFAULT0`"]
pub type INTFAULT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTFAULT1`"]
pub type INTFAULT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Asserted"]
    #[inline(always)]
    pub fn intpwm0(&self) -> INTPWM0_R {
        INTPWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Asserted"]
    #[inline(always)]
    pub fn intpwm1(&self) -> INTPWM1_R {
        INTPWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Asserted"]
    #[inline(always)]
    pub fn intpwm2(&self) -> INTPWM2_R {
        INTPWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Asserted"]
    #[inline(always)]
    pub fn intpwm3(&self) -> INTPWM3_R {
        INTPWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt Fault PWM 0"]
    #[inline(always)]
    pub fn intfault0(&self) -> INTFAULT0_R {
        INTFAULT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt Fault PWM 1"]
    #[inline(always)]
    pub fn intfault1(&self) -> INTFAULT1_R {
        INTFAULT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
