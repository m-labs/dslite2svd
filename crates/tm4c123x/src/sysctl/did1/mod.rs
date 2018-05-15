#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `QUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALR {
    #[doc = "Engineering Sample (unqualified)"]
    ES,
    #[doc = "Pilot Production (unqualified)"]
    PP,
    #[doc = "Fully Qualified"]
    FQ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl QUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QUALR::ES => 0,
            QUALR::PP => 1,
            QUALR::FQ => 2,
            QUALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QUALR {
        match value {
            0 => QUALR::ES,
            1 => QUALR::PP,
            2 => QUALR::FQ,
            i => QUALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ES`"]
    #[inline]
    pub fn is_es(&self) -> bool {
        *self == QUALR::ES
    }
    #[doc = "Checks if the value of the field is `PP`"]
    #[inline]
    pub fn is_pp(&self) -> bool {
        *self == QUALR::PP
    }
    #[doc = "Checks if the value of the field is `FQ`"]
    #[inline]
    pub fn is_fq(&self) -> bool {
        *self == QUALR::FQ
    }
}
#[doc = r" Value of the field"]
pub struct ROHSR {
    bits: bool,
}
impl ROHSR {
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
#[doc = "Possible values of the field `PKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKGR {
    #[doc = "QFP package"]
    QFP,
    #[doc = "BGA package"]
    BGA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PKGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PKGR::QFP => 1,
            PKGR::BGA => 2,
            PKGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PKGR {
        match value {
            1 => PKGR::QFP,
            2 => PKGR::BGA,
            i => PKGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `QFP`"]
    #[inline]
    pub fn is_qfp(&self) -> bool {
        *self == PKGR::QFP
    }
    #[doc = "Checks if the value of the field is `BGA`"]
    #[inline]
    pub fn is_bga(&self) -> bool {
        *self == PKGR::BGA
    }
}
#[doc = "Possible values of the field `TEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPR {
    #[doc = "Industrial temperature range"]
    I,
    #[doc = "Extended temperature range"]
    E,
    #[doc = "Available in both industrial temperature range (-40C to 85C) and extended temperature range (-40C to 105C) devices. See"]
    IE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TEMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TEMPR::I => 1,
            TEMPR::E => 2,
            TEMPR::IE => 3,
            TEMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TEMPR {
        match value {
            1 => TEMPR::I,
            2 => TEMPR::E,
            3 => TEMPR::IE,
            i => TEMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == TEMPR::I
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline]
    pub fn is_e(&self) -> bool {
        *self == TEMPR::E
    }
    #[doc = "Checks if the value of the field is `IE`"]
    #[inline]
    pub fn is_ie(&self) -> bool {
        *self == TEMPR::IE
    }
}
#[doc = "Possible values of the field `PINCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCNTR {
    #[doc = "100-pin LQFP package"]
    _100,
    #[doc = "64-pin LQFP package"]
    _64,
    #[doc = "144-pin LQFP package"]
    _144,
    #[doc = "157-pin BGA package"]
    _157,
    #[doc = "128-pin TQFP package"]
    _128,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCNTR::_100 => 2,
            PINCNTR::_64 => 3,
            PINCNTR::_144 => 4,
            PINCNTR::_157 => 5,
            PINCNTR::_128 => 6,
            PINCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCNTR {
        match value {
            2 => PINCNTR::_100,
            3 => PINCNTR::_64,
            4 => PINCNTR::_144,
            5 => PINCNTR::_157,
            6 => PINCNTR::_128,
            i => PINCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PINCNTR::_100
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PINCNTR::_64
    }
    #[doc = "Checks if the value of the field is `_144`"]
    #[inline]
    pub fn is_144(&self) -> bool {
        *self == PINCNTR::_144
    }
    #[doc = "Checks if the value of the field is `_157`"]
    #[inline]
    pub fn is_157(&self) -> bool {
        *self == PINCNTR::_157
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PINCNTR::_128
    }
}
#[doc = r" Value of the field"]
pub struct PRTNOR {
    bits: u8,
}
impl PRTNOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAMR {
    bits: u8,
}
impl FAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VERR {
    bits: u8,
}
impl VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline]
    pub fn qual(&self) -> QUALR {
        QUALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline]
    pub fn rohs(&self) -> ROHSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROHSR { bits }
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline]
    pub fn pkg(&self) -> PKGR {
        PKGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline]
    pub fn temp(&self) -> TEMPR {
        TEMPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline]
    pub fn pincnt(&self) -> PINCNTR {
        PINCNTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline]
    pub fn prtno(&self) -> PRTNOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRTNOR { bits }
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline]
    pub fn fam(&self) -> FAMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAMR { bits }
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline]
    pub fn ver(&self) -> VERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERR { bits }
    }
}
