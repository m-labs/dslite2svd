#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "EEPROM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: 64 bytes of EEPROM"]
    _64,
    #[doc = "1: 128 bytes of EEPROM"]
    _128,
    #[doc = "3: 256 bytes of EEPROM"]
    _256,
    #[doc = "7: 512 bytes of EEPROM"]
    _512,
    #[doc = "15: 1 KB of EEPROM"]
    _1K,
    #[doc = "31: 2 KB of EEPROM"]
    _2K,
    #[doc = "63: 3 KB of EEPROM"]
    _3K,
    #[doc = "127: 4 KB of EEPROM"]
    _4K,
    #[doc = "255: 5 KB of EEPROM"]
    _5K,
    #[doc = "511: 6 KB of EEPROM"]
    _6K,
}
impl From<SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::_64 => 0,
            SIZE_A::_128 => 1,
            SIZE_A::_256 => 3,
            SIZE_A::_512 => 7,
            SIZE_A::_1K => 15,
            SIZE_A::_2K => 31,
            SIZE_A::_3K => 63,
            SIZE_A::_4K => 127,
            SIZE_A::_5K => 255,
            SIZE_A::_6K => 511,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u16, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::_64),
            1 => Val(SIZE_A::_128),
            3 => Val(SIZE_A::_256),
            7 => Val(SIZE_A::_512),
            15 => Val(SIZE_A::_1K),
            31 => Val(SIZE_A::_2K),
            63 => Val(SIZE_A::_3K),
            127 => Val(SIZE_A::_4K),
            255 => Val(SIZE_A::_5K),
            511 => Val(SIZE_A::_6K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == SIZE_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == SIZE_A::_2K
    }
    #[doc = "Checks if the value of the field is `_3K`"]
    #[inline(always)]
    pub fn is_3k(&self) -> bool {
        *self == SIZE_A::_3K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SIZE_A::_4K
    }
    #[doc = "Checks if the value of the field is `_5K`"]
    #[inline(always)]
    pub fn is_5k(&self) -> bool {
        *self == SIZE_A::_5K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SIZE_A::_6K
    }
}
impl R {
    #[doc = "Bits 0:15 - EEPROM Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
