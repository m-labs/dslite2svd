#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "64 bytes of EEPROM"]
    _64,
    #[doc = "128 bytes of EEPROM"]
    _128,
    #[doc = "256 bytes of EEPROM"]
    _256,
    #[doc = "512 bytes of EEPROM"]
    _512,
    #[doc = "1 KB of EEPROM"]
    _1K,
    #[doc = "2 KB of EEPROM"]
    _2K,
    #[doc = "3 KB of EEPROM"]
    _3K,
    #[doc = "4 KB of EEPROM"]
    _4K,
    #[doc = "5 KB of EEPROM"]
    _5K,
    #[doc = "6 KB of EEPROM"]
    _6K,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SIZER::_64 => 0,
            SIZER::_128 => 1,
            SIZER::_256 => 3,
            SIZER::_512 => 7,
            SIZER::_1K => 15,
            SIZER::_2K => 31,
            SIZER::_3K => 63,
            SIZER::_4K => 127,
            SIZER::_5K => 255,
            SIZER::_6K => 511,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SIZER {
        match value {
            0 => SIZER::_64,
            1 => SIZER::_128,
            3 => SIZER::_256,
            7 => SIZER::_512,
            15 => SIZER::_1K,
            31 => SIZER::_2K,
            63 => SIZER::_3K,
            127 => SIZER::_4K,
            255 => SIZER::_5K,
            511 => SIZER::_6K,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == SIZER::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == SIZER::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == SIZER::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == SIZER::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline]
    pub fn is_1k(&self) -> bool {
        *self == SIZER::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline]
    pub fn is_2k(&self) -> bool {
        *self == SIZER::_2K
    }
    #[doc = "Checks if the value of the field is `_3K`"]
    #[inline]
    pub fn is_3k(&self) -> bool {
        *self == SIZER::_3K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == SIZER::_4K
    }
    #[doc = "Checks if the value of the field is `_5K`"]
    #[inline]
    pub fn is_5k(&self) -> bool {
        *self == SIZER::_5K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline]
    pub fn is_6k(&self) -> bool {
        *self == SIZER::_6K
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - EEPROM Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
