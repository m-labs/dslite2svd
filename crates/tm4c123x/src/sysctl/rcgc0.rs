#[doc = "Reader of register RCGC0"]
pub type R = crate::R<u32, super::RCGC0>;
#[doc = "Reader of field `WDT0`"]
pub type WDT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HIB`"]
pub type HIB_R = crate::R<bool, bool>;
#[doc = "ADC0 Sample Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC0SPD_A {
    #[doc = "0: 125K samples/second"]
    _125K = 0,
    #[doc = "1: 250K samples/second"]
    _250K = 1,
    #[doc = "2: 500K samples/second"]
    _500K = 2,
    #[doc = "3: 1M samples/second"]
    _1M = 3,
}
impl From<ADC0SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0SPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC0SPD`"]
pub type ADC0SPD_R = crate::R<u8, ADC0SPD_A>;
impl ADC0SPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0SPD_A {
        match self.bits {
            0 => ADC0SPD_A::_125K,
            1 => ADC0SPD_A::_250K,
            2 => ADC0SPD_A::_500K,
            3 => ADC0SPD_A::_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline(always)]
    pub fn is_125k(&self) -> bool {
        *self == ADC0SPD_A::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline(always)]
    pub fn is_250k(&self) -> bool {
        *self == ADC0SPD_A::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline(always)]
    pub fn is_500k(&self) -> bool {
        *self == ADC0SPD_A::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == ADC0SPD_A::_1M
    }
}
#[doc = "ADC1 Sample Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC1SPD_A {
    #[doc = "0: 125K samples/second"]
    _125K = 0,
    #[doc = "1: 250K samples/second"]
    _250K = 1,
    #[doc = "2: 500K samples/second"]
    _500K = 2,
    #[doc = "3: 1M samples/second"]
    _1M = 3,
}
impl From<ADC1SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1SPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC1SPD`"]
pub type ADC1SPD_R = crate::R<u8, ADC1SPD_A>;
impl ADC1SPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1SPD_A {
        match self.bits {
            0 => ADC1SPD_A::_125K,
            1 => ADC1SPD_A::_250K,
            2 => ADC1SPD_A::_500K,
            3 => ADC1SPD_A::_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline(always)]
    pub fn is_125k(&self) -> bool {
        *self == ADC1SPD_A::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline(always)]
    pub fn is_250k(&self) -> bool {
        *self == ADC1SPD_A::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline(always)]
    pub fn is_500k(&self) -> bool {
        *self == ADC1SPD_A::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == ADC1SPD_A::_1M
    }
}
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
    #[doc = "Bits 8:9 - ADC0 Sample Speed"]
    #[inline(always)]
    pub fn adc0spd(&self) -> ADC0SPD_R {
        ADC0SPD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ADC1 Sample Speed"]
    #[inline(always)]
    pub fn adc1spd(&self) -> ADC1SPD_R {
        ADC1SPD_R::new(((self.bits >> 10) & 0x03) as u8)
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
