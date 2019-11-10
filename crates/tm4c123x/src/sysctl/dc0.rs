#[doc = "Reader of register DC0"]
pub type R = crate::R<u32, super::DC0>;
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHSZ_A {
    #[doc = "3: 8 KB of Flash"]
    _8KB,
    #[doc = "7: 16 KB of Flash"]
    _16KB,
    #[doc = "15: 32 KB of Flash"]
    _32KB,
    #[doc = "31: 64 KB of Flash"]
    _64KB,
    #[doc = "47: 96 KB of Flash"]
    _96KB,
    #[doc = "63: 128 KB of Flash"]
    _128K,
    #[doc = "95: 192 KB of Flash"]
    _192K,
    #[doc = "127: 256 KB of Flash"]
    _256K,
}
impl From<FLASHSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: FLASHSZ_A) -> Self {
        match variant {
            FLASHSZ_A::_8KB => 3,
            FLASHSZ_A::_16KB => 7,
            FLASHSZ_A::_32KB => 15,
            FLASHSZ_A::_64KB => 31,
            FLASHSZ_A::_96KB => 47,
            FLASHSZ_A::_128K => 63,
            FLASHSZ_A::_192K => 95,
            FLASHSZ_A::_256K => 127,
        }
    }
}
#[doc = "Reader of field `FLASHSZ`"]
pub type FLASHSZ_R = crate::R<u16, FLASHSZ_A>;
impl FLASHSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FLASHSZ_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(FLASHSZ_A::_8KB),
            7 => Val(FLASHSZ_A::_16KB),
            15 => Val(FLASHSZ_A::_32KB),
            31 => Val(FLASHSZ_A::_64KB),
            47 => Val(FLASHSZ_A::_96KB),
            63 => Val(FLASHSZ_A::_128K),
            95 => Val(FLASHSZ_A::_192K),
            127 => Val(FLASHSZ_A::_256K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == FLASHSZ_A::_8KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == FLASHSZ_A::_16KB
    }
    #[doc = "Checks if the value of the field is `_32KB`"]
    #[inline(always)]
    pub fn is_32kb(&self) -> bool {
        *self == FLASHSZ_A::_32KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == FLASHSZ_A::_64KB
    }
    #[doc = "Checks if the value of the field is `_96KB`"]
    #[inline(always)]
    pub fn is_96kb(&self) -> bool {
        *self == FLASHSZ_A::_96KB
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == FLASHSZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == FLASHSZ_A::_192K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == FLASHSZ_A::_256K
    }
}
#[doc = "SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSZ_A {
    #[doc = "7: 2 KB of SRAM"]
    _2KB,
    #[doc = "15: 4 KB of SRAM"]
    _4KB,
    #[doc = "23: 6 KB of SRAM"]
    _6KB,
    #[doc = "31: 8 KB of SRAM"]
    _8KB,
    #[doc = "47: 12 KB of SRAM"]
    _12KB,
    #[doc = "63: 16 KB of SRAM"]
    _16KB,
    #[doc = "79: 20 KB of SRAM"]
    _20KB,
    #[doc = "95: 24 KB of SRAM"]
    _24KB,
    #[doc = "127: 32 KB of SRAM"]
    _32KB,
}
impl From<SRAMSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMSZ_A) -> Self {
        match variant {
            SRAMSZ_A::_2KB => 7,
            SRAMSZ_A::_4KB => 15,
            SRAMSZ_A::_6KB => 23,
            SRAMSZ_A::_8KB => 31,
            SRAMSZ_A::_12KB => 47,
            SRAMSZ_A::_16KB => 63,
            SRAMSZ_A::_20KB => 79,
            SRAMSZ_A::_24KB => 95,
            SRAMSZ_A::_32KB => 127,
        }
    }
}
#[doc = "Reader of field `SRAMSZ`"]
pub type SRAMSZ_R = crate::R<u16, SRAMSZ_A>;
impl SRAMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAMSZ_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(SRAMSZ_A::_2KB),
            15 => Val(SRAMSZ_A::_4KB),
            23 => Val(SRAMSZ_A::_6KB),
            31 => Val(SRAMSZ_A::_8KB),
            47 => Val(SRAMSZ_A::_12KB),
            63 => Val(SRAMSZ_A::_16KB),
            79 => Val(SRAMSZ_A::_20KB),
            95 => Val(SRAMSZ_A::_24KB),
            127 => Val(SRAMSZ_A::_32KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline(always)]
    pub fn is_2kb(&self) -> bool {
        *self == SRAMSZ_A::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == SRAMSZ_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_6KB`"]
    #[inline(always)]
    pub fn is_6kb(&self) -> bool {
        *self == SRAMSZ_A::_6KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == SRAMSZ_A::_8KB
    }
    #[doc = "Checks if the value of the field is `_12KB`"]
    #[inline(always)]
    pub fn is_12kb(&self) -> bool {
        *self == SRAMSZ_A::_12KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == SRAMSZ_A::_16KB
    }
    #[doc = "Checks if the value of the field is `_20KB`"]
    #[inline(always)]
    pub fn is_20kb(&self) -> bool {
        *self == SRAMSZ_A::_20KB
    }
    #[doc = "Checks if the value of the field is `_24KB`"]
    #[inline(always)]
    pub fn is_24kb(&self) -> bool {
        *self == SRAMSZ_A::_24KB
    }
    #[doc = "Checks if the value of the field is `_32KB`"]
    #[inline(always)]
    pub fn is_32kb(&self) -> bool {
        *self == SRAMSZ_A::_32KB
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flashsz(&self) -> FLASHSZ_R {
        FLASHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline(always)]
    pub fn sramsz(&self) -> SRAMSZ_R {
        SRAMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
