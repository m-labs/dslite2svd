#[doc = "Reader of register SDRAMCFG"]
pub type R = crate::R<u32, super::SDRAMCFG>;
#[doc = "Writer for register SDRAMCFG"]
pub type W = crate::W<u32, super::SDRAMCFG>;
#[doc = "Register SDRAMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Size of SDRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: 64 megabits (8MB)"]
    _8MB,
    #[doc = "1: 128 megabits (16MB)"]
    _16MB,
    #[doc = "2: 256 megabits (32MB)"]
    _32MB,
    #[doc = "3: 512 megabits (64MB)"]
    _64MB,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::_8MB => 0,
            SIZE_A::_16MB => 1,
            SIZE_A::_32MB => 2,
            SIZE_A::_64MB => 3,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            0 => SIZE_A::_8MB,
            1 => SIZE_A::_16MB,
            2 => SIZE_A::_32MB,
            3 => SIZE_A::_64MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8MB`"]
    #[inline(always)]
    pub fn is_8mb(&self) -> bool {
        *self == SIZE_A::_8MB
    }
    #[doc = "Checks if the value of the field is `_16MB`"]
    #[inline(always)]
    pub fn is_16mb(&self) -> bool {
        *self == SIZE_A::_16MB
    }
    #[doc = "Checks if the value of the field is `_32MB`"]
    #[inline(always)]
    pub fn is_32mb(&self) -> bool {
        *self == SIZE_A::_32MB
    }
    #[doc = "Checks if the value of the field is `_64MB`"]
    #[inline(always)]
    pub fn is_64mb(&self) -> bool {
        *self == SIZE_A::_64MB
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 megabits (8MB)"]
    #[inline(always)]
    pub fn _8mb(self) -> &'a mut W {
        self.variant(SIZE_A::_8MB)
    }
    #[doc = "128 megabits (16MB)"]
    #[inline(always)]
    pub fn _16mb(self) -> &'a mut W {
        self.variant(SIZE_A::_16MB)
    }
    #[doc = "256 megabits (32MB)"]
    #[inline(always)]
    pub fn _32mb(self) -> &'a mut W {
        self.variant(SIZE_A::_32MB)
    }
    #[doc = "512 megabits (64MB)"]
    #[inline(always)]
    pub fn _64mb(self) -> &'a mut W {
        self.variant(SIZE_A::_64MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RFSH`"]
pub type RFSH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RFSH`"]
pub struct RFSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RFSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "EPI Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQ_A {
    #[doc = "0: 0 - 15 MHz"]
    NONE,
    #[doc = "1: 15 - 30 MHz"]
    _15MHZ,
    #[doc = "2: 30 - 50 MHz"]
    _30MHZ,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        match variant {
            FREQ_A::NONE => 0,
            FREQ_A::_15MHZ => 1,
            FREQ_A::_30MHZ => 2,
        }
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u8, FREQ_A>;
impl FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FREQ_A::NONE),
            1 => Val(FREQ_A::_15MHZ),
            2 => Val(FREQ_A::_30MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FREQ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_15MHZ`"]
    #[inline(always)]
    pub fn is_15mhz(&self) -> bool {
        *self == FREQ_A::_15MHZ
    }
    #[doc = "Checks if the value of the field is `_30MHZ`"]
    #[inline(always)]
    pub fn is_30mhz(&self) -> bool {
        *self == FREQ_A::_30MHZ
    }
}
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 - 15 MHz"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FREQ_A::NONE)
    }
    #[doc = "15 - 30 MHz"]
    #[inline(always)]
    pub fn _15mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_15MHZ)
    }
    #[doc = "30 - 50 MHz"]
    #[inline(always)]
    pub fn _30mhz(self) -> &'a mut W {
        self.variant(FREQ_A::_30MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn rfsh(&self) -> RFSH_R {
        RFSH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn rfsh(&mut self) -> RFSH_W {
        RFSH_W { w: self }
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
}
