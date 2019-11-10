#[doc = "Reader of register DC3"]
pub type R = crate::R<u32, super::DC3>;
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
#[doc = "Reader of field `C0MINUS`"]
pub type C0MINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C0PLUS`"]
pub type C0PLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C0O`"]
pub type C0O_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1MINUS`"]
pub type C1MINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1PLUS`"]
pub type C1PLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1O`"]
pub type C1O_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2MINUS`"]
pub type C2MINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2PLUS`"]
pub type C2PLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2O`"]
pub type C2O_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMFAULT`"]
pub type PWMFAULT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN0`"]
pub type ADC0AIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN1`"]
pub type ADC0AIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN2`"]
pub type ADC0AIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN3`"]
pub type ADC0AIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN4`"]
pub type ADC0AIN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN5`"]
pub type ADC0AIN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN6`"]
pub type ADC0AIN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN7`"]
pub type ADC0AIN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP0`"]
pub type CCP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP1`"]
pub type CCP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP2`"]
pub type CCP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP3`"]
pub type CCP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP4`"]
pub type CCP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP5`"]
pub type CCP5_R = crate::R<bool, bool>;
#[doc = "Reader of field `_32KHZ`"]
pub type _32KHZ_R = crate::R<bool, bool>;
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
    #[doc = "Bit 6 - C0- Pin Present"]
    #[inline(always)]
    pub fn c0minus(&self) -> C0MINUS_R {
        C0MINUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - C0+ Pin Present"]
    #[inline(always)]
    pub fn c0plus(&self) -> C0PLUS_R {
        C0PLUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - C0o Pin Present"]
    #[inline(always)]
    pub fn c0o(&self) -> C0O_R {
        C0O_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - C1- Pin Present"]
    #[inline(always)]
    pub fn c1minus(&self) -> C1MINUS_R {
        C1MINUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - C1+ Pin Present"]
    #[inline(always)]
    pub fn c1plus(&self) -> C1PLUS_R {
        C1PLUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - C1o Pin Present"]
    #[inline(always)]
    pub fn c1o(&self) -> C1O_R {
        C1O_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - C2- Pin Present"]
    #[inline(always)]
    pub fn c2minus(&self) -> C2MINUS_R {
        C2MINUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - C2+ Pin Present"]
    #[inline(always)]
    pub fn c2plus(&self) -> C2PLUS_R {
        C2PLUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - C2o Pin Present"]
    #[inline(always)]
    pub fn c2o(&self) -> C2O_R {
        C2O_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PWM Fault Pin Present"]
    #[inline(always)]
    pub fn pwmfault(&self) -> PWMFAULT_R {
        PWMFAULT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC Module 0 AIN0 Pin Present"]
    #[inline(always)]
    pub fn adc0ain0(&self) -> ADC0AIN0_R {
        ADC0AIN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Module 0 AIN1 Pin Present"]
    #[inline(always)]
    pub fn adc0ain1(&self) -> ADC0AIN1_R {
        ADC0AIN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC Module 0 AIN2 Pin Present"]
    #[inline(always)]
    pub fn adc0ain2(&self) -> ADC0AIN2_R {
        ADC0AIN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC Module 0 AIN3 Pin Present"]
    #[inline(always)]
    pub fn adc0ain3(&self) -> ADC0AIN3_R {
        ADC0AIN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC Module 0 AIN4 Pin Present"]
    #[inline(always)]
    pub fn adc0ain4(&self) -> ADC0AIN4_R {
        ADC0AIN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC Module 0 AIN5 Pin Present"]
    #[inline(always)]
    pub fn adc0ain5(&self) -> ADC0AIN5_R {
        ADC0AIN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC Module 0 AIN6 Pin Present"]
    #[inline(always)]
    pub fn adc0ain6(&self) -> ADC0AIN6_R {
        ADC0AIN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC Module 0 AIN7 Pin Present"]
    #[inline(always)]
    pub fn adc0ain7(&self) -> ADC0AIN7_R {
        ADC0AIN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - T0CCP0 Pin Present"]
    #[inline(always)]
    pub fn ccp0(&self) -> CCP0_R {
        CCP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - T0CCP1 Pin Present"]
    #[inline(always)]
    pub fn ccp1(&self) -> CCP1_R {
        CCP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - T1CCP0 Pin Present"]
    #[inline(always)]
    pub fn ccp2(&self) -> CCP2_R {
        CCP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - T1CCP1 Pin Present"]
    #[inline(always)]
    pub fn ccp3(&self) -> CCP3_R {
        CCP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - T2CCP0 Pin Present"]
    #[inline(always)]
    pub fn ccp4(&self) -> CCP4_R {
        CCP4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - T2CCP1 Pin Present"]
    #[inline(always)]
    pub fn ccp5(&self) -> CCP5_R {
        CCP5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 32KHz Input Clock Available"]
    #[inline(always)]
    pub fn _32khz(&self) -> _32KHZ_R {
        _32KHZ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
