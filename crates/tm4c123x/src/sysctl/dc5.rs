#[doc = "Reader of register DC5"]
pub type R = crate::R<u32, super::DC5>;
#[doc = "Reader of field `PWM0`"]
pub type PWM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM1`"]
pub type PWM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM2`"]
pub type PWM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM3`"]
pub type PWM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM4`"]
pub type PWM4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM5`"]
pub type PWM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM6`"]
pub type PWM6_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM7`"]
pub type PWM7_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMESYNC`"]
pub type PWMESYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMEFLT`"]
pub type PWMEFLT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMFAULT0`"]
pub type PWMFAULT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMFAULT1`"]
pub type PWMFAULT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMFAULT2`"]
pub type PWMFAULT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMFAULT3`"]
pub type PWMFAULT3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PWM0 Pin Present"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Pin Present"]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Pin Present"]
    #[inline(always)]
    pub fn pwm2(&self) -> PWM2_R {
        PWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Pin Present"]
    #[inline(always)]
    pub fn pwm3(&self) -> PWM3_R {
        PWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM4 Pin Present"]
    #[inline(always)]
    pub fn pwm4(&self) -> PWM4_R {
        PWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM5 Pin Present"]
    #[inline(always)]
    pub fn pwm5(&self) -> PWM5_R {
        PWM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM6 Pin Present"]
    #[inline(always)]
    pub fn pwm6(&self) -> PWM6_R {
        PWM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PWM7 Pin Present"]
    #[inline(always)]
    pub fn pwm7(&self) -> PWM7_R {
        PWM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM Extended SYNC Active"]
    #[inline(always)]
    pub fn pwmesync(&self) -> PWMESYNC_R {
        PWMESYNC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM Extended Fault Active"]
    #[inline(always)]
    pub fn pwmeflt(&self) -> PWMEFLT_R {
        PWMEFLT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PWM Fault 0 Pin Present"]
    #[inline(always)]
    pub fn pwmfault0(&self) -> PWMFAULT0_R {
        PWMFAULT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PWM Fault 1 Pin Present"]
    #[inline(always)]
    pub fn pwmfault1(&self) -> PWMFAULT1_R {
        PWMFAULT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PWM Fault 2 Pin Present"]
    #[inline(always)]
    pub fn pwmfault2(&self) -> PWMFAULT2_R {
        PWMFAULT2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PWM Fault 3 Pin Present"]
    #[inline(always)]
    pub fn pwmfault3(&self) -> PWMFAULT3_R {
        PWMFAULT3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
