#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SIZE_A {
    #[doc = "511: 1024 KB of Flash"]
    _1MB = 511,
}
impl From<SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
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
            511 => Val(SIZE_A::_1MB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1MB`"]
    #[inline(always)]
    pub fn is_1mb(&self) -> bool {
        *self == SIZE_A::_1MB
    }
}
#[doc = "Flash Sector Size of the physical bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAINSS_A {
    #[doc = "0: 1 KB"]
    _1KB = 0,
    #[doc = "1: 2 KB"]
    _2KB = 1,
    #[doc = "2: 4 KB"]
    _4KB = 2,
    #[doc = "3: 8 KB"]
    _8KB = 3,
    #[doc = "4: 16 KB"]
    _16KB = 4,
}
impl From<MAINSS_A> for u8 {
    #[inline(always)]
    fn from(variant: MAINSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAINSS`"]
pub type MAINSS_R = crate::R<u8, MAINSS_A>;
impl MAINSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAINSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAINSS_A::_1KB),
            1 => Val(MAINSS_A::_2KB),
            2 => Val(MAINSS_A::_4KB),
            3 => Val(MAINSS_A::_8KB),
            4 => Val(MAINSS_A::_16KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == MAINSS_A::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline(always)]
    pub fn is_2kb(&self) -> bool {
        *self == MAINSS_A::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == MAINSS_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == MAINSS_A::_8KB
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == MAINSS_A::_16KB
    }
}
#[doc = "EEPROM Sector Size of the physical bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EESS_A {
    #[doc = "0: 1 KB"]
    _1KB = 0,
    #[doc = "1: 2 KB"]
    _2KB = 1,
    #[doc = "2: 4 KB"]
    _4KB = 2,
    #[doc = "3: 8 KB"]
    _8KB = 3,
}
impl From<EESS_A> for u8 {
    #[inline(always)]
    fn from(variant: EESS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EESS`"]
pub type EESS_R = crate::R<u8, EESS_A>;
impl EESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EESS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EESS_A::_1KB),
            1 => Val(EESS_A::_2KB),
            2 => Val(EESS_A::_4KB),
            3 => Val(EESS_A::_8KB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1KB`"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == EESS_A::_1KB
    }
    #[doc = "Checks if the value of the field is `_2KB`"]
    #[inline(always)]
    pub fn is_2kb(&self) -> bool {
        *self == EESS_A::_2KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == EESS_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == EESS_A::_8KB
    }
}
#[doc = "Reader of field `DFA`"]
pub type DFA_R = crate::R<bool, bool>;
#[doc = "Reader of field `FMM`"]
pub type FMM_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFC`"]
pub type PFC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline(always)]
    pub fn mainss(&self) -> MAINSS_R {
        MAINSS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline(always)]
    pub fn eess(&self) -> EESS_R {
        EESS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline(always)]
    pub fn dfa(&self) -> DFA_R {
        DFA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline(always)]
    pub fn fmm(&self) -> FMM_R {
        FMM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline(always)]
    pub fn pfc(&self) -> PFC_R {
        PFC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
