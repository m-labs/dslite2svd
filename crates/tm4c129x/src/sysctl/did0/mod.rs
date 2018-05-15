#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DID0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `MIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINR {
    #[doc = "Initial device, or a major revision update"]
    _0,
    #[doc = "First metal layer change"]
    _1,
    #[doc = "Second metal layer change"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MINR::_0 => 0,
            MINR::_1 => 1,
            MINR::_2 => 2,
            MINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MINR {
        match value {
            0 => MINR::_0,
            1 => MINR::_1,
            2 => MINR::_2,
            i => MINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MINR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == MINR::_2
    }
}
#[doc = "Possible values of the field `MAJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJR {
    #[doc = "Revision A (initial device)"]
    REVA,
    #[doc = "Revision B (first base layer revision)"]
    REVB,
    #[doc = "Revision C (second base layer revision)"]
    REVC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAJR::REVA => 0,
            MAJR::REVB => 1,
            MAJR::REVC => 2,
            MAJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAJR {
        match value {
            0 => MAJR::REVA,
            1 => MAJR::REVB,
            2 => MAJR::REVC,
            i => MAJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REVA`"]
    #[inline]
    pub fn is_reva(&self) -> bool {
        *self == MAJR::REVA
    }
    #[doc = "Checks if the value of the field is `REVB`"]
    #[inline]
    pub fn is_revb(&self) -> bool {
        *self == MAJR::REVB
    }
    #[doc = "Checks if the value of the field is `REVC`"]
    #[inline]
    pub fn is_revc(&self) -> bool {
        *self == MAJR::REVC
    }
}
#[doc = "Possible values of the field `CLASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLASSR {
    #[doc = "Tiva(TM) TM4C129-class microcontrollers"]
    TM4C129,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLASSR::TM4C129 => 10,
            CLASSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLASSR {
        match value {
            10 => CLASSR::TM4C129,
            i => CLASSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TM4C129`"]
    #[inline]
    pub fn is_tm4c129(&self) -> bool {
        *self == CLASSR::TM4C129
    }
}
#[doc = "Possible values of the field `VER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VERR {
    #[doc = "Second version of the DID0 register format."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VERR::_1 => 1,
            VERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VERR {
        match value {
            1 => VERR::_1,
            i => VERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VERR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline]
    pub fn min(&self) -> MINR {
        MINR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline]
    pub fn maj(&self) -> MAJR {
        MAJR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline]
    pub fn class(&self) -> CLASSR {
        CLASSR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline]
    pub fn ver(&self) -> VERR {
        VERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
