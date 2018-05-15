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
#[doc = "Possible values of the field `PHYTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYTYPER {
    #[doc = "No PHY"]
    NONE,
    #[doc = "Snowflake class PHY"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PHYTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHYTYPER::NONE => 0,
            PHYTYPER::_1 => 3,
            PHYTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHYTYPER {
        match value {
            0 => PHYTYPER::NONE,
            3 => PHYTYPER::_1,
            i => PHYTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PHYTYPER::NONE
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PHYTYPER::_1
    }
}
#[doc = r" Value of the field"]
pub struct MACTYPER {
    bits: u8,
}
impl MACTYPER {
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
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline]
    pub fn phytype(&self) -> PHYTYPER {
        PHYTYPER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline]
    pub fn mactype(&self) -> MACTYPER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MACTYPER { bits }
    }
}
