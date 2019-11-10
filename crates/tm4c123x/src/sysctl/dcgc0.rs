#[doc = "Reader of register DCGC0"]
pub type R = crate::R<u32, super::DCGC0>;
#[doc = "Reader of field `WDT0`"]
pub type WDT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HIB`"]
pub type HIB_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM0`"]
pub type PWM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN0`"]
pub type CAN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN1`"]
pub type CAN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT1`"]
pub type WDT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
