#[doc = "Reader of register DC1"]
pub type R = crate::R<u32, super::DC1>;
#[doc = "Reader of field `JTAG`"]
pub type JTAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWD`"]
pub type SWD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWO`"]
pub type SWO_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT0`"]
pub type WDT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL`"]
pub type PLL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `HIB`"]
pub type HIB_R = crate::R<bool, bool>;
#[doc = "Reader of field `MPU`"]
pub type MPU_R = crate::R<bool, bool>;
#[doc = "Max ADC0 Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0SPD_A {
    #[doc = "0: 125K samples/second"]
    _125K,
    #[doc = "1: 250K samples/second"]
    _250K,
    #[doc = "2: 500K samples/second"]
    _500K,
    #[doc = "3: 1M samples/second"]
    _1M,
}
impl From<ADC0SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0SPD_A) -> Self {
        match variant {
            ADC0SPD_A::_125K => 0,
            ADC0SPD_A::_250K => 1,
            ADC0SPD_A::_500K => 2,
            ADC0SPD_A::_1M => 3,
        }
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
#[doc = "Max ADC1 Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1SPD_A {
    #[doc = "0: 125K samples/second"]
    _125K,
    #[doc = "1: 250K samples/second"]
    _250K,
    #[doc = "2: 500K samples/second"]
    _500K,
    #[doc = "3: 1M samples/second"]
    _1M,
}
impl From<ADC1SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1SPD_A) -> Self {
        match variant {
            ADC1SPD_A::_125K => 0,
            ADC1SPD_A::_250K => 1,
            ADC1SPD_A::_500K => 2,
            ADC1SPD_A::_1M => 3,
        }
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
#[doc = "System Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINSYSDIV_A {
    #[doc = "2: Specifies an 80-MHz CPU clock with a PLL divider of 2.5"]
    _80,
    #[doc = "3: Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    _50,
    #[doc = "4: Specifies a 40-MHz CPU clock with a PLL divider of 5"]
    _40,
    #[doc = "7: Specifies a 25-MHz clock with a PLL divider of 8"]
    _25,
    #[doc = "9: Specifies a 20-MHz clock with a PLL divider of 10"]
    _20,
}
impl From<MINSYSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MINSYSDIV_A) -> Self {
        match variant {
            MINSYSDIV_A::_80 => 2,
            MINSYSDIV_A::_50 => 3,
            MINSYSDIV_A::_40 => 4,
            MINSYSDIV_A::_25 => 7,
            MINSYSDIV_A::_20 => 9,
        }
    }
}
#[doc = "Reader of field `MINSYSDIV`"]
pub type MINSYSDIV_R = crate::R<u8, MINSYSDIV_A>;
impl MINSYSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MINSYSDIV_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(MINSYSDIV_A::_80),
            3 => Val(MINSYSDIV_A::_50),
            4 => Val(MINSYSDIV_A::_40),
            7 => Val(MINSYSDIV_A::_25),
            9 => Val(MINSYSDIV_A::_20),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == MINSYSDIV_A::_80
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == MINSYSDIV_A::_50
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == MINSYSDIV_A::_40
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == MINSYSDIV_A::_25
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == MINSYSDIV_A::_20
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM0`"]
pub type PWM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWM1`"]
pub type PWM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN0`"]
pub type CAN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN1`"]
pub type CAN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT1`"]
pub type WDT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - JTAG Present"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWD Present"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWO Trace Port Present"]
    #[inline(always)]
    pub fn swo(&self) -> SWO_R {
        SWO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL Present"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Temp Sensor Present"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hibernation Module Present"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Present"]
    #[inline(always)]
    pub fn mpu(&self) -> MPU_R {
        MPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Max ADC0 Speed"]
    #[inline(always)]
    pub fn adc0spd(&self) -> ADC0SPD_R {
        ADC0SPD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Max ADC1 Speed"]
    #[inline(always)]
    pub fn adc1spd(&self) -> ADC1SPD_R {
        ADC1SPD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - System Clock Divider"]
    #[inline(always)]
    pub fn minsysdiv(&self) -> MINSYSDIV_R {
        MINSYSDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC Module 0 Present"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Module 1 Present"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM Module 0 Present"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM Module 1 Present"]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CAN Module 0 Present"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CAN Module 1 Present"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Watchdog Timer1 Present"]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
