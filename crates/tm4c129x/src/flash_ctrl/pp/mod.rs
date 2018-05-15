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
    #[doc = "1024 KB of Flash"]
    _1MB,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SIZER::_1MB => 511,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SIZER {
        match value {
            511 => SIZER::_1MB,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1MB`"]
    #[inline]
    pub fn is_1mb(&self) -> bool {
        *self == SIZER::_1MB
    }
}
#[doc = "Possible values of the field `MAINSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINSSR {
    #[doc = "1 KB"]
    _1KB,
    #[doc = "2 KB"]
    _2KB,
    #[doc = "4 KB"]
    _4KB,
    #[doc = "8 KB"]
    _8KB,
    #[doc = "16 KB"]
    _16KB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAINSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAINSSR::_1KB => 0,
            MAINSSR::_2KB => 1,
            MAINSSR::_4KB => 2,
            MAINSSR::_8KB => 3,
            MAINSSR::_16KB => 4,
            MAINSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAINSSR {
        match value {
            0 => MAINSSR::_1KB,
            1 => MAINSSR::_2KB,
            2 => MAINSSR::_4KB,
            3 => MAINSSR::_8KB,
            4 => MAINSSR::_16KB,
            i => MAINSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline]
    pub fn is_1kb(&self) -> bool {
        *self == MAINSSR::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline]
    pub fn is_2kb(&self) -> bool {
        *self == MAINSSR::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == MAINSSR::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == MAINSSR::_8KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline]
    pub fn is_16kb(&self) -> bool {
        *self == MAINSSR::_16KB
    }
}
#[doc = "Possible values of the field `EESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EESSR {
    #[doc = "1 KB"]
    _1KB,
    #[doc = "2 KB"]
    _2KB,
    #[doc = "4 KB"]
    _4KB,
    #[doc = "8 KB"]
    _8KB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EESSR::_1KB => 0,
            EESSR::_2KB => 1,
            EESSR::_4KB => 2,
            EESSR::_8KB => 3,
            EESSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EESSR {
        match value {
            0 => EESSR::_1KB,
            1 => EESSR::_2KB,
            2 => EESSR::_4KB,
            3 => EESSR::_8KB,
            i => EESSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline]
    pub fn is_1kb(&self) -> bool {
        *self == EESSR::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline]
    pub fn is_2kb(&self) -> bool {
        *self == EESSR::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == EESSR::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline]
    pub fn is_8kb(&self) -> bool {
        *self == EESSR::_8KB
    }
}
#[doc = r" Value of the field"]
pub struct DFAR {
    bits: bool,
}
impl DFAR {
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
pub struct FMMR {
    bits: bool,
}
impl FMMR {
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
pub struct PFCR {
    bits: bool,
}
impl PFCR {
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
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline]
    pub fn mainss(&self) -> MAINSSR {
        MAINSSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline]
    pub fn eess(&self) -> EESSR {
        EESSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline]
    pub fn dfa(&self) -> DFAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DFAR { bits }
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline]
    pub fn fmm(&self) -> FMMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FMMR { bits }
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline]
    pub fn pfc(&self) -> PFCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFCR { bits }
    }
}
