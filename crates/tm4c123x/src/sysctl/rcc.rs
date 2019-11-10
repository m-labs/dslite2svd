#[doc = "Reader of register RCC"]
pub type R = crate::R<u32, super::RCC>;
#[doc = "Writer for register RCC"]
pub type W = crate::W<u32, super::RCC>;
#[doc = "Register RCC `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOSCDIS`"]
pub type MOSCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCDIS`"]
pub struct MOSCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSRC_A {
    #[doc = "0: MOSC"]
    MAIN,
    #[doc = "1: IOSC"]
    INT,
    #[doc = "2: IOSC/4"]
    INT4,
    #[doc = "3: LFIOSC"]
    _30,
}
impl From<OSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSRC_A) -> Self {
        match variant {
            OSCSRC_A::MAIN => 0,
            OSCSRC_A::INT => 1,
            OSCSRC_A::INT4 => 2,
            OSCSRC_A::_30 => 3,
        }
    }
}
#[doc = "Reader of field `OSCSRC`"]
pub type OSCSRC_R = crate::R<u8, OSCSRC_A>;
impl OSCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSRC_A {
        match self.bits {
            0 => OSCSRC_A::MAIN,
            1 => OSCSRC_A::INT,
            2 => OSCSRC_A::INT4,
            3 => OSCSRC_A::_30,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == OSCSRC_A::MAIN
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == OSCSRC_A::INT
    }
    #[doc = "Checks if the value of the field is `INT4`"]
    #[inline(always)]
    pub fn is_int4(&self) -> bool {
        *self == OSCSRC_A::INT4
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == OSCSRC_A::_30
    }
}
#[doc = "Write proxy for field `OSCSRC`"]
pub struct OSCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(OSCSRC_A::MAIN)
    }
    #[doc = "IOSC"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(OSCSRC_A::INT)
    }
    #[doc = "IOSC/4"]
    #[inline(always)]
    pub fn int4(self) -> &'a mut W {
        self.variant(OSCSRC_A::INT4)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(OSCSRC_A::_30)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Crystal Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_A {
    #[doc = "6: 4 MHz"]
    _4MHZ,
    #[doc = "7: 4.096 MHz"]
    _4_09MHZ,
    #[doc = "8: 4.9152 MHz"]
    _4_91MHZ,
    #[doc = "9: 5 MHz"]
    _5MHZ,
    #[doc = "10: 5.12 MHz"]
    _5_12MHZ,
    #[doc = "11: 6 MHz"]
    _6MHZ,
    #[doc = "12: 6.144 MHz"]
    _6_14MHZ,
    #[doc = "13: 7.3728 MHz"]
    _7_37MHZ,
    #[doc = "14: 8 MHz"]
    _8MHZ,
    #[doc = "15: 8.192 MHz"]
    _8_19MHZ,
    #[doc = "16: 10 MHz"]
    _10MHZ,
    #[doc = "17: 12 MHz"]
    _12MHZ,
    #[doc = "18: 12.288 MHz"]
    _12_2MHZ,
    #[doc = "19: 13.56 MHz"]
    _13_5MHZ,
    #[doc = "20: 14.31818 MHz"]
    _14_3MHZ,
    #[doc = "21: 16 MHz"]
    _16MHZ,
    #[doc = "22: 16.384 MHz"]
    _16_3MHZ,
    #[doc = "23: 18.0 MHz (USB)"]
    _18MHZ,
    #[doc = "24: 20.0 MHz (USB)"]
    _20MHZ,
    #[doc = "25: 24.0 MHz (USB)"]
    _24MHZ,
    #[doc = "26: 25.0 MHz (USB)"]
    _25MHZ,
}
impl From<XTAL_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_A) -> Self {
        match variant {
            XTAL_A::_4MHZ => 6,
            XTAL_A::_4_09MHZ => 7,
            XTAL_A::_4_91MHZ => 8,
            XTAL_A::_5MHZ => 9,
            XTAL_A::_5_12MHZ => 10,
            XTAL_A::_6MHZ => 11,
            XTAL_A::_6_14MHZ => 12,
            XTAL_A::_7_37MHZ => 13,
            XTAL_A::_8MHZ => 14,
            XTAL_A::_8_19MHZ => 15,
            XTAL_A::_10MHZ => 16,
            XTAL_A::_12MHZ => 17,
            XTAL_A::_12_2MHZ => 18,
            XTAL_A::_13_5MHZ => 19,
            XTAL_A::_14_3MHZ => 20,
            XTAL_A::_16MHZ => 21,
            XTAL_A::_16_3MHZ => 22,
            XTAL_A::_18MHZ => 23,
            XTAL_A::_20MHZ => 24,
            XTAL_A::_24MHZ => 25,
            XTAL_A::_25MHZ => 26,
        }
    }
}
#[doc = "Reader of field `XTAL`"]
pub type XTAL_R = crate::R<u8, XTAL_A>;
impl XTAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(XTAL_A::_4MHZ),
            7 => Val(XTAL_A::_4_09MHZ),
            8 => Val(XTAL_A::_4_91MHZ),
            9 => Val(XTAL_A::_5MHZ),
            10 => Val(XTAL_A::_5_12MHZ),
            11 => Val(XTAL_A::_6MHZ),
            12 => Val(XTAL_A::_6_14MHZ),
            13 => Val(XTAL_A::_7_37MHZ),
            14 => Val(XTAL_A::_8MHZ),
            15 => Val(XTAL_A::_8_19MHZ),
            16 => Val(XTAL_A::_10MHZ),
            17 => Val(XTAL_A::_12MHZ),
            18 => Val(XTAL_A::_12_2MHZ),
            19 => Val(XTAL_A::_13_5MHZ),
            20 => Val(XTAL_A::_14_3MHZ),
            21 => Val(XTAL_A::_16MHZ),
            22 => Val(XTAL_A::_16_3MHZ),
            23 => Val(XTAL_A::_18MHZ),
            24 => Val(XTAL_A::_20MHZ),
            25 => Val(XTAL_A::_24MHZ),
            26 => Val(XTAL_A::_25MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == XTAL_A::_4MHZ
    }
    #[doc = "Checks if the value of the field is `_4_09MHZ`"]
    #[inline(always)]
    pub fn is_4_09mhz(&self) -> bool {
        *self == XTAL_A::_4_09MHZ
    }
    #[doc = "Checks if the value of the field is `_4_91MHZ`"]
    #[inline(always)]
    pub fn is_4_91mhz(&self) -> bool {
        *self == XTAL_A::_4_91MHZ
    }
    #[doc = "Checks if the value of the field is `_5MHZ`"]
    #[inline(always)]
    pub fn is_5mhz(&self) -> bool {
        *self == XTAL_A::_5MHZ
    }
    #[doc = "Checks if the value of the field is `_5_12MHZ`"]
    #[inline(always)]
    pub fn is_5_12mhz(&self) -> bool {
        *self == XTAL_A::_5_12MHZ
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline(always)]
    pub fn is_6mhz(&self) -> bool {
        *self == XTAL_A::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_6_14MHZ`"]
    #[inline(always)]
    pub fn is_6_14mhz(&self) -> bool {
        *self == XTAL_A::_6_14MHZ
    }
    #[doc = "Checks if the value of the field is `_7_37MHZ`"]
    #[inline(always)]
    pub fn is_7_37mhz(&self) -> bool {
        *self == XTAL_A::_7_37MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == XTAL_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_8_19MHZ`"]
    #[inline(always)]
    pub fn is_8_19mhz(&self) -> bool {
        *self == XTAL_A::_8_19MHZ
    }
    #[doc = "Checks if the value of the field is `_10MHZ`"]
    #[inline(always)]
    pub fn is_10mhz(&self) -> bool {
        *self == XTAL_A::_10MHZ
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline(always)]
    pub fn is_12mhz(&self) -> bool {
        *self == XTAL_A::_12MHZ
    }
    #[doc = "Checks if the value of the field is `_12_2MHZ`"]
    #[inline(always)]
    pub fn is_12_2mhz(&self) -> bool {
        *self == XTAL_A::_12_2MHZ
    }
    #[doc = "Checks if the value of the field is `_13_5MHZ`"]
    #[inline(always)]
    pub fn is_13_5mhz(&self) -> bool {
        *self == XTAL_A::_13_5MHZ
    }
    #[doc = "Checks if the value of the field is `_14_3MHZ`"]
    #[inline(always)]
    pub fn is_14_3mhz(&self) -> bool {
        *self == XTAL_A::_14_3MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == XTAL_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_16_3MHZ`"]
    #[inline(always)]
    pub fn is_16_3mhz(&self) -> bool {
        *self == XTAL_A::_16_3MHZ
    }
    #[doc = "Checks if the value of the field is `_18MHZ`"]
    #[inline(always)]
    pub fn is_18mhz(&self) -> bool {
        *self == XTAL_A::_18MHZ
    }
    #[doc = "Checks if the value of the field is `_20MHZ`"]
    #[inline(always)]
    pub fn is_20mhz(&self) -> bool {
        *self == XTAL_A::_20MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == XTAL_A::_24MHZ
    }
    #[doc = "Checks if the value of the field is `_25MHZ`"]
    #[inline(always)]
    pub fn is_25mhz(&self) -> bool {
        *self == XTAL_A::_25MHZ
    }
}
#[doc = "Write proxy for field `XTAL`"]
pub struct XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_4MHZ)
    }
    #[doc = "4.096 MHz"]
    #[inline(always)]
    pub fn _4_09mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_4_09MHZ)
    }
    #[doc = "4.9152 MHz"]
    #[inline(always)]
    pub fn _4_91mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_4_91MHZ)
    }
    #[doc = "5 MHz"]
    #[inline(always)]
    pub fn _5mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_5MHZ)
    }
    #[doc = "5.12 MHz"]
    #[inline(always)]
    pub fn _5_12mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_5_12MHZ)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_6MHZ)
    }
    #[doc = "6.144 MHz"]
    #[inline(always)]
    pub fn _6_14mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_6_14MHZ)
    }
    #[doc = "7.3728 MHz"]
    #[inline(always)]
    pub fn _7_37mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_7_37MHZ)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_8MHZ)
    }
    #[doc = "8.192 MHz"]
    #[inline(always)]
    pub fn _8_19mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_8_19MHZ)
    }
    #[doc = "10 MHz"]
    #[inline(always)]
    pub fn _10mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_10MHZ)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_12MHZ)
    }
    #[doc = "12.288 MHz"]
    #[inline(always)]
    pub fn _12_2mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_12_2MHZ)
    }
    #[doc = "13.56 MHz"]
    #[inline(always)]
    pub fn _13_5mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_13_5MHZ)
    }
    #[doc = "14.31818 MHz"]
    #[inline(always)]
    pub fn _14_3mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_14_3MHZ)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_16MHZ)
    }
    #[doc = "16.384 MHz"]
    #[inline(always)]
    pub fn _16_3mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_16_3MHZ)
    }
    #[doc = "18.0 MHz (USB)"]
    #[inline(always)]
    pub fn _18mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_18MHZ)
    }
    #[doc = "20.0 MHz (USB)"]
    #[inline(always)]
    pub fn _20mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_20MHZ)
    }
    #[doc = "24.0 MHz (USB)"]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_24MHZ)
    }
    #[doc = "25.0 MHz (USB)"]
    #[inline(always)]
    pub fn _25mhz(self) -> &'a mut W {
        self.variant(XTAL_A::_25MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PWRDN`"]
pub type PWRDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDN`"]
pub struct PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PWM Unit Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMDIV_A {
    #[doc = "0: PWM clock /2"]
    _2,
    #[doc = "1: PWM clock /4"]
    _4,
    #[doc = "2: PWM clock /8"]
    _8,
    #[doc = "3: PWM clock /16"]
    _16,
    #[doc = "4: PWM clock /32"]
    _32,
    #[doc = "5: PWM clock /64"]
    _64,
}
impl From<PWMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMDIV_A) -> Self {
        match variant {
            PWMDIV_A::_2 => 0,
            PWMDIV_A::_4 => 1,
            PWMDIV_A::_8 => 2,
            PWMDIV_A::_16 => 3,
            PWMDIV_A::_32 => 4,
            PWMDIV_A::_64 => 5,
        }
    }
}
#[doc = "Reader of field `PWMDIV`"]
pub type PWMDIV_R = crate::R<u8, PWMDIV_A>;
impl PWMDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWMDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWMDIV_A::_2),
            1 => Val(PWMDIV_A::_4),
            2 => Val(PWMDIV_A::_8),
            3 => Val(PWMDIV_A::_16),
            4 => Val(PWMDIV_A::_32),
            5 => Val(PWMDIV_A::_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PWMDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PWMDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PWMDIV_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PWMDIV_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PWMDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PWMDIV_A::_64
    }
}
#[doc = "Write proxy for field `PWMDIV`"]
pub struct PWMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM clock /2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PWMDIV_A::_2)
    }
    #[doc = "PWM clock /4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PWMDIV_A::_4)
    }
    #[doc = "PWM clock /8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PWMDIV_A::_8)
    }
    #[doc = "PWM clock /16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PWMDIV_A::_16)
    }
    #[doc = "PWM clock /32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PWMDIV_A::_32)
    }
    #[doc = "PWM clock /64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PWMDIV_A::_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `USEPWMDIV`"]
pub type USEPWMDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEPWMDIV`"]
pub struct USEPWMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USEPWMDIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `USESYSDIV`"]
pub type USESYSDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USESYSDIV`"]
pub struct USESYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USESYSDIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SYSDIV`"]
pub type SYSDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSDIV`"]
pub struct SYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `ACG`"]
pub type ACG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACG`"]
pub struct ACG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    pub fn moscdis(&self) -> MOSCDIS_R {
        MOSCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    pub fn oscsrc(&self) -> OSCSRC_R {
        OSCSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline(always)]
    pub fn xtal(&self) -> XTAL_R {
        XTAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    pub fn pwrdn(&self) -> PWRDN_R {
        PWRDN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    pub fn pwmdiv(&self) -> PWMDIV_R {
        PWMDIV_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    pub fn usepwmdiv(&self) -> USEPWMDIV_R {
        USEPWMDIV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    pub fn usesysdiv(&self) -> USESYSDIV_R {
        USESYSDIV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    pub fn acg(&self) -> ACG_R {
        ACG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    pub fn moscdis(&mut self) -> MOSCDIS_W {
        MOSCDIS_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    pub fn oscsrc(&mut self) -> OSCSRC_W {
        OSCSRC_W { w: self }
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline(always)]
    pub fn xtal(&mut self) -> XTAL_W {
        XTAL_W { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    pub fn pwrdn(&mut self) -> PWRDN_W {
        PWRDN_W { w: self }
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    pub fn pwmdiv(&mut self) -> PWMDIV_W {
        PWMDIV_W { w: self }
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    pub fn usepwmdiv(&mut self) -> USEPWMDIV_W {
        USEPWMDIV_W { w: self }
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    pub fn usesysdiv(&mut self) -> USESYSDIV_W {
        USESYSDIV_W { w: self }
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W {
        SYSDIV_W { w: self }
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    pub fn acg(&mut self) -> ACG_W {
        ACG_W { w: self }
    }
}
