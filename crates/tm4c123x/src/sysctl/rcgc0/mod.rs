#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RCGC0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct WDT0R {
    bits: bool,
}
impl WDT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HIBR {
    bits: bool,
}
impl HIBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `ADC0SPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0SPDR {
    #[doc = "125K samples/second"]
    _125K,
    #[doc = "250K samples/second"]
    _250K,
    #[doc = "500K samples/second"]
    _500K,
    #[doc = "1M samples/second"]
    _1M,
}
impl ADC0SPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0SPDR::_125K => 0,
            ADC0SPDR::_250K => 1,
            ADC0SPDR::_500K => 2,
            ADC0SPDR::_1M => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0SPDR {
        match value {
            0 => ADC0SPDR::_125K,
            1 => ADC0SPDR::_250K,
            2 => ADC0SPDR::_500K,
            3 => ADC0SPDR::_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline]
    pub fn is_125k(&self) -> bool {
        *self == ADC0SPDR::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline]
    pub fn is_250k(&self) -> bool {
        *self == ADC0SPDR::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline]
    pub fn is_500k(&self) -> bool {
        *self == ADC0SPDR::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline]
    pub fn is_1m(&self) -> bool {
        *self == ADC0SPDR::_1M
    }
}
#[doc = "Possible values of the field `ADC1SPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1SPDR {
    #[doc = "125K samples/second"]
    _125K,
    #[doc = "250K samples/second"]
    _250K,
    #[doc = "500K samples/second"]
    _500K,
    #[doc = "1M samples/second"]
    _1M,
}
impl ADC1SPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC1SPDR::_125K => 0,
            ADC1SPDR::_250K => 1,
            ADC1SPDR::_500K => 2,
            ADC1SPDR::_1M => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC1SPDR {
        match value {
            0 => ADC1SPDR::_125K,
            1 => ADC1SPDR::_250K,
            2 => ADC1SPDR::_500K,
            3 => ADC1SPDR::_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline]
    pub fn is_125k(&self) -> bool {
        *self == ADC1SPDR::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline]
    pub fn is_250k(&self) -> bool {
        *self == ADC1SPDR::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline]
    pub fn is_500k(&self) -> bool {
        *self == ADC1SPDR::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline]
    pub fn is_1m(&self) -> bool {
        *self == ADC1SPDR::_1M
    }
}
#[doc = r" Value of the field"]
pub struct ADC0R {
    bits: bool,
}
impl ADC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ADC1R {
    bits: bool,
}
impl ADC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PWM0R {
    bits: bool,
}
impl PWM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CAN0R {
    bits: bool,
}
impl CAN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CAN1R {
    bits: bool,
}
impl CAN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WDT1R {
    bits: bool,
}
impl WDT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline]
    pub fn wdt0(&self) -> WDT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDT0R { bits }
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline]
    pub fn hib(&self) -> HIBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HIBR { bits }
    }
    #[doc = "Bits 8:9 - ADC0 Sample Speed"]
    #[inline]
    pub fn adc0spd(&self) -> ADC0SPDR {
        ADC0SPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - ADC1 Sample Speed"]
    #[inline]
    pub fn adc1spd(&self) -> ADC1SPDR {
        ADC1SPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline]
    pub fn adc0(&self) -> ADC0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC0R { bits }
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline]
    pub fn adc1(&self) -> ADC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC1R { bits }
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline]
    pub fn pwm0(&self) -> PWM0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWM0R { bits }
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline]
    pub fn can0(&self) -> CAN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAN0R { bits }
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline]
    pub fn can1(&self) -> CAN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAN1R { bits }
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline]
    pub fn wdt1(&self) -> WDT1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDT1R { bits }
    }
}
