#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `FLASHSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHSZR {
    #[doc = "8 KB of Flash"]
    _8KB,
    #[doc = "16 KB of Flash"]
    _16KB,
    #[doc = "32 KB of Flash"]
    _32KB,
    #[doc = "64 KB of Flash"]
    _64KB,
    #[doc = "96 KB of Flash"]
    _96KB,
    #[doc = "128 KB of Flash"]
    _128K,
    #[doc = "192 KB of Flash"]
    _192K,
    #[doc = "256 KB of Flash"]
    _256K,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FLASHSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FLASHSZR::_8KB => 3,
            FLASHSZR::_16KB => 7,
            FLASHSZR::_32KB => 15,
            FLASHSZR::_64KB => 31,
            FLASHSZR::_96KB => 47,
            FLASHSZR::_128K => 63,
            FLASHSZR::_192K => 95,
            FLASHSZR::_256K => 127,
            FLASHSZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FLASHSZR {
        match value {
            3 => FLASHSZR::_8KB,
            7 => FLASHSZR::_16KB,
            15 => FLASHSZR::_32KB,
            31 => FLASHSZR::_64KB,
            47 => FLASHSZR::_96KB,
            63 => FLASHSZR::_128K,
            95 => FLASHSZR::_192K,
            127 => FLASHSZR::_256K,
            i => FLASHSZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == FLASHSZR::_8KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline]
    pub fn is_16kb(&self) -> bool {
        *self == FLASHSZR::_16KB
    }
    #[doc = "Checks if the value of the field is `_32KB`"]
    #[inline]
    pub fn is_32kb(&self) -> bool {
        *self == FLASHSZR::_32KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline]
    pub fn is_64kb(&self) -> bool {
        *self == FLASHSZR::_64KB
    }
    #[doc = "Checks if the value of the field is `_96KB`"]
    #[inline]
    pub fn is_96kb(&self) -> bool {
        *self == FLASHSZR::_96KB
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == FLASHSZR::_128K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline]
    pub fn is_192k(&self) -> bool {
        *self == FLASHSZR::_192K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == FLASHSZR::_256K
    }
}
#[doc = "Possible values of the field `SRAMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSZR {
    #[doc = "2 KB of SRAM"]
    _2KB,
    #[doc = "4 KB of SRAM"]
    _4KB,
    #[doc = "6 KB of SRAM"]
    _6KB,
    #[doc = "8 KB of SRAM"]
    _8KB,
    #[doc = "12 KB of SRAM"]
    _12KB,
    #[doc = "16 KB of SRAM"]
    _16KB,
    #[doc = "20 KB of SRAM"]
    _20KB,
    #[doc = "24 KB of SRAM"]
    _24KB,
    #[doc = "32 KB of SRAM"]
    _32KB,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SRAMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SRAMSZR::_2KB => 7,
            SRAMSZR::_4KB => 15,
            SRAMSZR::_6KB => 23,
            SRAMSZR::_8KB => 31,
            SRAMSZR::_12KB => 47,
            SRAMSZR::_16KB => 63,
            SRAMSZR::_20KB => 79,
            SRAMSZR::_24KB => 95,
            SRAMSZR::_32KB => 127,
            SRAMSZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SRAMSZR {
        match value {
            7 => SRAMSZR::_2KB,
            15 => SRAMSZR::_4KB,
            23 => SRAMSZR::_6KB,
            31 => SRAMSZR::_8KB,
            47 => SRAMSZR::_12KB,
            63 => SRAMSZR::_16KB,
            79 => SRAMSZR::_20KB,
            95 => SRAMSZR::_24KB,
            127 => SRAMSZR::_32KB,
            i => SRAMSZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline]
    pub fn is_2kb(&self) -> bool {
        *self == SRAMSZR::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == SRAMSZR::_4KB
    }
    #[doc = "Checks if the value of the field is `_6KB`"]
    #[inline]
    pub fn is_6kb(&self) -> bool {
        *self == SRAMSZR::_6KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == SRAMSZR::_8KB
    }
    #[doc = "Checks if the value of the field is `_12KB`"]
    #[inline]
    pub fn is_12kb(&self) -> bool {
        *self == SRAMSZR::_12KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline]
    pub fn is_16kb(&self) -> bool {
        *self == SRAMSZR::_16KB
    }
    #[doc = "Checks if the value of the field is `_20KB`"]
    #[inline]
    pub fn is_20kb(&self) -> bool {
        *self == SRAMSZR::_20KB
    }
    #[doc = "Checks if the value of the field is `_24KB`"]
    #[inline]
    pub fn is_24kb(&self) -> bool {
        *self == SRAMSZR::_24KB
    }
    #[doc = "Checks if the value of the field is `_32KB`"]
    #[inline]
    pub fn is_32kb(&self) -> bool {
        *self == SRAMSZR::_32KB
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline]
    pub fn flashsz(&self) -> FLASHSZR {
        FLASHSZR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline]
    pub fn sramsz(&self) -> SRAMSZR {
        SRAMSZR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
