#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct MOSCDISR {
    bits: bool,
}
impl MOSCDISR {
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
#[doc = "Possible values of the field `OSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSRCR {
    #[doc = "MOSC"]
    MAIN,
    #[doc = "IOSC"]
    INT,
    #[doc = "IOSC/4"]
    INT4,
    #[doc = "LFIOSC"]
    _30,
}
impl OSCSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCSRCR::MAIN => 0,
            OSCSRCR::INT => 1,
            OSCSRCR::INT4 => 2,
            OSCSRCR::_30 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCSRCR {
        match value {
            0 => OSCSRCR::MAIN,
            1 => OSCSRCR::INT,
            2 => OSCSRCR::INT4,
            3 => OSCSRCR::_30,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline]
    pub fn is_main(&self) -> bool {
        *self == OSCSRCR::MAIN
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == OSCSRCR::INT
    }
    #[doc = "Checks if the value of the field is `INT4`"]
    #[inline]
    pub fn is_int4(&self) -> bool {
        *self == OSCSRCR::INT4
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == OSCSRCR::_30
    }
}
#[doc = "Possible values of the field `XTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALR {
    #[doc = "4 MHz"]
    _4MHZ,
    #[doc = "4.096 MHz"]
    _4_09MHZ,
    #[doc = "4.9152 MHz"]
    _4_91MHZ,
    #[doc = "5 MHz"]
    _5MHZ,
    #[doc = "5.12 MHz"]
    _5_12MHZ,
    #[doc = "6 MHz"]
    _6MHZ,
    #[doc = "6.144 MHz"]
    _6_14MHZ,
    #[doc = "7.3728 MHz"]
    _7_37MHZ,
    #[doc = "8 MHz"]
    _8MHZ,
    #[doc = "8.192 MHz"]
    _8_19MHZ,
    #[doc = "10 MHz"]
    _10MHZ,
    #[doc = "12 MHz"]
    _12MHZ,
    #[doc = "12.288 MHz"]
    _12_2MHZ,
    #[doc = "13.56 MHz"]
    _13_5MHZ,
    #[doc = "14.31818 MHz"]
    _14_3MHZ,
    #[doc = "16 MHz"]
    _16MHZ,
    #[doc = "16.384 MHz"]
    _16_3MHZ,
    #[doc = "18.0 MHz (USB)"]
    _18MHZ,
    #[doc = "20.0 MHz (USB)"]
    _20MHZ,
    #[doc = "24.0 MHz (USB)"]
    _24MHZ,
    #[doc = "25.0 MHz (USB)"]
    _25MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XTALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XTALR::_4MHZ => 6,
            XTALR::_4_09MHZ => 7,
            XTALR::_4_91MHZ => 8,
            XTALR::_5MHZ => 9,
            XTALR::_5_12MHZ => 10,
            XTALR::_6MHZ => 11,
            XTALR::_6_14MHZ => 12,
            XTALR::_7_37MHZ => 13,
            XTALR::_8MHZ => 14,
            XTALR::_8_19MHZ => 15,
            XTALR::_10MHZ => 16,
            XTALR::_12MHZ => 17,
            XTALR::_12_2MHZ => 18,
            XTALR::_13_5MHZ => 19,
            XTALR::_14_3MHZ => 20,
            XTALR::_16MHZ => 21,
            XTALR::_16_3MHZ => 22,
            XTALR::_18MHZ => 23,
            XTALR::_20MHZ => 24,
            XTALR::_24MHZ => 25,
            XTALR::_25MHZ => 26,
            XTALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XTALR {
        match value {
            6 => XTALR::_4MHZ,
            7 => XTALR::_4_09MHZ,
            8 => XTALR::_4_91MHZ,
            9 => XTALR::_5MHZ,
            10 => XTALR::_5_12MHZ,
            11 => XTALR::_6MHZ,
            12 => XTALR::_6_14MHZ,
            13 => XTALR::_7_37MHZ,
            14 => XTALR::_8MHZ,
            15 => XTALR::_8_19MHZ,
            16 => XTALR::_10MHZ,
            17 => XTALR::_12MHZ,
            18 => XTALR::_12_2MHZ,
            19 => XTALR::_13_5MHZ,
            20 => XTALR::_14_3MHZ,
            21 => XTALR::_16MHZ,
            22 => XTALR::_16_3MHZ,
            23 => XTALR::_18MHZ,
            24 => XTALR::_20MHZ,
            25 => XTALR::_24MHZ,
            26 => XTALR::_25MHZ,
            i => XTALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline]
    pub fn is_4mhz(&self) -> bool {
        *self == XTALR::_4MHZ
    }
    #[doc = "Checks if the value of the field is `_4_09MHZ`"]
    #[inline]
    pub fn is_4_09mhz(&self) -> bool {
        *self == XTALR::_4_09MHZ
    }
    #[doc = "Checks if the value of the field is `_4_91MHZ`"]
    #[inline]
    pub fn is_4_91mhz(&self) -> bool {
        *self == XTALR::_4_91MHZ
    }
    #[doc = "Checks if the value of the field is `_5MHZ`"]
    #[inline]
    pub fn is_5mhz(&self) -> bool {
        *self == XTALR::_5MHZ
    }
    #[doc = "Checks if the value of the field is `_5_12MHZ`"]
    #[inline]
    pub fn is_5_12mhz(&self) -> bool {
        *self == XTALR::_5_12MHZ
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline]
    pub fn is_6mhz(&self) -> bool {
        *self == XTALR::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_6_14MHZ`"]
    #[inline]
    pub fn is_6_14mhz(&self) -> bool {
        *self == XTALR::_6_14MHZ
    }
    #[doc = "Checks if the value of the field is `_7_37MHZ`"]
    #[inline]
    pub fn is_7_37mhz(&self) -> bool {
        *self == XTALR::_7_37MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline]
    pub fn is_8mhz(&self) -> bool {
        *self == XTALR::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_8_19MHZ`"]
    #[inline]
    pub fn is_8_19mhz(&self) -> bool {
        *self == XTALR::_8_19MHZ
    }
    #[doc = "Checks if the value of the field is `_10MHZ`"]
    #[inline]
    pub fn is_10mhz(&self) -> bool {
        *self == XTALR::_10MHZ
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline]
    pub fn is_12mhz(&self) -> bool {
        *self == XTALR::_12MHZ
    }
    #[doc = "Checks if the value of the field is `_12_2MHZ`"]
    #[inline]
    pub fn is_12_2mhz(&self) -> bool {
        *self == XTALR::_12_2MHZ
    }
    #[doc = "Checks if the value of the field is `_13_5MHZ`"]
    #[inline]
    pub fn is_13_5mhz(&self) -> bool {
        *self == XTALR::_13_5MHZ
    }
    #[doc = "Checks if the value of the field is `_14_3MHZ`"]
    #[inline]
    pub fn is_14_3mhz(&self) -> bool {
        *self == XTALR::_14_3MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline]
    pub fn is_16mhz(&self) -> bool {
        *self == XTALR::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_16_3MHZ`"]
    #[inline]
    pub fn is_16_3mhz(&self) -> bool {
        *self == XTALR::_16_3MHZ
    }
    #[doc = "Checks if the value of the field is `_18MHZ`"]
    #[inline]
    pub fn is_18mhz(&self) -> bool {
        *self == XTALR::_18MHZ
    }
    #[doc = "Checks if the value of the field is `_20MHZ`"]
    #[inline]
    pub fn is_20mhz(&self) -> bool {
        *self == XTALR::_20MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline]
    pub fn is_24mhz(&self) -> bool {
        *self == XTALR::_24MHZ
    }
    #[doc = "Checks if the value of the field is `_25MHZ`"]
    #[inline]
    pub fn is_25mhz(&self) -> bool {
        *self == XTALR::_25MHZ
    }
}
#[doc = r" Value of the field"]
pub struct BYPASSR {
    bits: bool,
}
impl BYPASSR {
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
pub struct PWRDNR {
    bits: bool,
}
impl PWRDNR {
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
#[doc = "Possible values of the field `PWMDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMDIVR {
    #[doc = "PWM clock /2"]
    _2,
    #[doc = "PWM clock /4"]
    _4,
    #[doc = "PWM clock /8"]
    _8,
    #[doc = "PWM clock /16"]
    _16,
    #[doc = "PWM clock /32"]
    _32,
    #[doc = "PWM clock /64"]
    _64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWMDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWMDIVR::_2 => 0,
            PWMDIVR::_4 => 1,
            PWMDIVR::_8 => 2,
            PWMDIVR::_16 => 3,
            PWMDIVR::_32 => 4,
            PWMDIVR::_64 => 5,
            PWMDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWMDIVR {
        match value {
            0 => PWMDIVR::_2,
            1 => PWMDIVR::_4,
            2 => PWMDIVR::_8,
            3 => PWMDIVR::_16,
            4 => PWMDIVR::_32,
            5 => PWMDIVR::_64,
            i => PWMDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PWMDIVR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == PWMDIVR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PWMDIVR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PWMDIVR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PWMDIVR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PWMDIVR::_64
    }
}
#[doc = r" Value of the field"]
pub struct USEPWMDIVR {
    bits: bool,
}
impl USEPWMDIVR {
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
pub struct USESYSDIVR {
    bits: bool,
}
impl USESYSDIVR {
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
pub struct SYSDIVR {
    bits: u8,
}
impl SYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACGR {
    bits: bool,
}
impl ACGR {
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
#[doc = r" Proxy"]
pub struct _MOSCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSCDISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSRC`"]
pub enum OSCSRCW {
    #[doc = "MOSC"]
    MAIN,
    #[doc = "IOSC"]
    INT,
    #[doc = "IOSC/4"]
    INT4,
    #[doc = "LFIOSC"]
    _30,
}
impl OSCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCSRCW::MAIN => 0,
            OSCSRCW::INT => 1,
            OSCSRCW::INT4 => 2,
            OSCSRCW::_30 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MOSC"]
    #[inline]
    pub fn main(self) -> &'a mut W {
        self.variant(OSCSRCW::MAIN)
    }
    #[doc = "IOSC"]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(OSCSRCW::INT)
    }
    #[doc = "IOSC/4"]
    #[inline]
    pub fn int4(self) -> &'a mut W {
        self.variant(OSCSRCW::INT4)
    }
    #[doc = "LFIOSC"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(OSCSRCW::_30)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XTAL`"]
pub enum XTALW {
    #[doc = "4 MHz"]
    _4MHZ,
    #[doc = "4.096 MHz"]
    _4_09MHZ,
    #[doc = "4.9152 MHz"]
    _4_91MHZ,
    #[doc = "5 MHz"]
    _5MHZ,
    #[doc = "5.12 MHz"]
    _5_12MHZ,
    #[doc = "6 MHz"]
    _6MHZ,
    #[doc = "6.144 MHz"]
    _6_14MHZ,
    #[doc = "7.3728 MHz"]
    _7_37MHZ,
    #[doc = "8 MHz"]
    _8MHZ,
    #[doc = "8.192 MHz"]
    _8_19MHZ,
    #[doc = "10 MHz"]
    _10MHZ,
    #[doc = "12 MHz"]
    _12MHZ,
    #[doc = "12.288 MHz"]
    _12_2MHZ,
    #[doc = "13.56 MHz"]
    _13_5MHZ,
    #[doc = "14.31818 MHz"]
    _14_3MHZ,
    #[doc = "16 MHz"]
    _16MHZ,
    #[doc = "16.384 MHz"]
    _16_3MHZ,
    #[doc = "18.0 MHz (USB)"]
    _18MHZ,
    #[doc = "20.0 MHz (USB)"]
    _20MHZ,
    #[doc = "24.0 MHz (USB)"]
    _24MHZ,
    #[doc = "25.0 MHz (USB)"]
    _25MHZ,
}
impl XTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XTALW::_4MHZ => 6,
            XTALW::_4_09MHZ => 7,
            XTALW::_4_91MHZ => 8,
            XTALW::_5MHZ => 9,
            XTALW::_5_12MHZ => 10,
            XTALW::_6MHZ => 11,
            XTALW::_6_14MHZ => 12,
            XTALW::_7_37MHZ => 13,
            XTALW::_8MHZ => 14,
            XTALW::_8_19MHZ => 15,
            XTALW::_10MHZ => 16,
            XTALW::_12MHZ => 17,
            XTALW::_12_2MHZ => 18,
            XTALW::_13_5MHZ => 19,
            XTALW::_14_3MHZ => 20,
            XTALW::_16MHZ => 21,
            XTALW::_16_3MHZ => 22,
            XTALW::_18MHZ => 23,
            XTALW::_20MHZ => 24,
            XTALW::_24MHZ => 25,
            XTALW::_25MHZ => 26,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4 MHz"]
    #[inline]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(XTALW::_4MHZ)
    }
    #[doc = "4.096 MHz"]
    #[inline]
    pub fn _4_09mhz(self) -> &'a mut W {
        self.variant(XTALW::_4_09MHZ)
    }
    #[doc = "4.9152 MHz"]
    #[inline]
    pub fn _4_91mhz(self) -> &'a mut W {
        self.variant(XTALW::_4_91MHZ)
    }
    #[doc = "5 MHz"]
    #[inline]
    pub fn _5mhz(self) -> &'a mut W {
        self.variant(XTALW::_5MHZ)
    }
    #[doc = "5.12 MHz"]
    #[inline]
    pub fn _5_12mhz(self) -> &'a mut W {
        self.variant(XTALW::_5_12MHZ)
    }
    #[doc = "6 MHz"]
    #[inline]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(XTALW::_6MHZ)
    }
    #[doc = "6.144 MHz"]
    #[inline]
    pub fn _6_14mhz(self) -> &'a mut W {
        self.variant(XTALW::_6_14MHZ)
    }
    #[doc = "7.3728 MHz"]
    #[inline]
    pub fn _7_37mhz(self) -> &'a mut W {
        self.variant(XTALW::_7_37MHZ)
    }
    #[doc = "8 MHz"]
    #[inline]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(XTALW::_8MHZ)
    }
    #[doc = "8.192 MHz"]
    #[inline]
    pub fn _8_19mhz(self) -> &'a mut W {
        self.variant(XTALW::_8_19MHZ)
    }
    #[doc = "10 MHz"]
    #[inline]
    pub fn _10mhz(self) -> &'a mut W {
        self.variant(XTALW::_10MHZ)
    }
    #[doc = "12 MHz"]
    #[inline]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(XTALW::_12MHZ)
    }
    #[doc = "12.288 MHz"]
    #[inline]
    pub fn _12_2mhz(self) -> &'a mut W {
        self.variant(XTALW::_12_2MHZ)
    }
    #[doc = "13.56 MHz"]
    #[inline]
    pub fn _13_5mhz(self) -> &'a mut W {
        self.variant(XTALW::_13_5MHZ)
    }
    #[doc = "14.31818 MHz"]
    #[inline]
    pub fn _14_3mhz(self) -> &'a mut W {
        self.variant(XTALW::_14_3MHZ)
    }
    #[doc = "16 MHz"]
    #[inline]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(XTALW::_16MHZ)
    }
    #[doc = "16.384 MHz"]
    #[inline]
    pub fn _16_3mhz(self) -> &'a mut W {
        self.variant(XTALW::_16_3MHZ)
    }
    #[doc = "18.0 MHz (USB)"]
    #[inline]
    pub fn _18mhz(self) -> &'a mut W {
        self.variant(XTALW::_18MHZ)
    }
    #[doc = "20.0 MHz (USB)"]
    #[inline]
    pub fn _20mhz(self) -> &'a mut W {
        self.variant(XTALW::_20MHZ)
    }
    #[doc = "24.0 MHz (USB)"]
    #[inline]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(XTALW::_24MHZ)
    }
    #[doc = "25.0 MHz (USB)"]
    #[inline]
    pub fn _25mhz(self) -> &'a mut W {
        self.variant(XTALW::_25MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWRDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRDNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMDIV`"]
pub enum PWMDIVW {
    #[doc = "PWM clock /2"]
    _2,
    #[doc = "PWM clock /4"]
    _4,
    #[doc = "PWM clock /8"]
    _8,
    #[doc = "PWM clock /16"]
    _16,
    #[doc = "PWM clock /32"]
    _32,
    #[doc = "PWM clock /64"]
    _64,
}
impl PWMDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWMDIVW::_2 => 0,
            PWMDIVW::_4 => 1,
            PWMDIVW::_8 => 2,
            PWMDIVW::_16 => 3,
            PWMDIVW::_32 => 4,
            PWMDIVW::_64 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM clock /2"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(PWMDIVW::_2)
    }
    #[doc = "PWM clock /4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(PWMDIVW::_4)
    }
    #[doc = "PWM clock /8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PWMDIVW::_8)
    }
    #[doc = "PWM clock /16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PWMDIVW::_16)
    }
    #[doc = "PWM clock /32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(PWMDIVW::_32)
    }
    #[doc = "PWM clock /64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(PWMDIVW::_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USEPWMDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _USEPWMDIVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USESYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _USESYSDIVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACGW<'a> {
    w: &'a mut W,
}
impl<'a> _ACGW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline]
    pub fn moscdis(&self) -> MOSCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOSCDISR { bits }
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline]
    pub fn oscsrc(&self) -> OSCSRCR {
        OSCSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline]
    pub fn xtal(&self) -> XTALR {
        XTALR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASSR { bits }
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline]
    pub fn pwrdn(&self) -> PWRDNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWRDNR { bits }
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline]
    pub fn pwmdiv(&self) -> PWMDIVR {
        PWMDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline]
    pub fn usepwmdiv(&self) -> USEPWMDIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USEPWMDIVR { bits }
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline]
    pub fn usesysdiv(&self) -> USESYSDIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USESYSDIVR { bits }
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline]
    pub fn sysdiv(&self) -> SYSDIVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYSDIVR { bits }
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline]
    pub fn acg(&self) -> ACGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACGR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline]
    pub fn moscdis(&mut self) -> _MOSCDISW {
        _MOSCDISW { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline]
    pub fn oscsrc(&mut self) -> _OSCSRCW {
        _OSCSRCW { w: self }
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline]
    pub fn xtal(&mut self) -> _XTALW {
        _XTALW { w: self }
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline]
    pub fn pwrdn(&mut self) -> _PWRDNW {
        _PWRDNW { w: self }
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline]
    pub fn pwmdiv(&mut self) -> _PWMDIVW {
        _PWMDIVW { w: self }
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline]
    pub fn usepwmdiv(&mut self) -> _USEPWMDIVW {
        _USEPWMDIVW { w: self }
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline]
    pub fn usesysdiv(&mut self) -> _USESYSDIVW {
        _USESYSDIVW { w: self }
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline]
    pub fn sysdiv(&mut self) -> _SYSDIVW {
        _SYSDIVW { w: self }
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline]
    pub fn acg(&mut self) -> _ACGW {
        _ACGW { w: self }
    }
}
