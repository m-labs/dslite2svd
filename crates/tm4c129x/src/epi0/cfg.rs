#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: General Purpose"]
    NONE = 0,
    #[doc = "1: SDRAM"]
    SDRAM = 1,
    #[doc = "2: 8-Bit Host-Bus (HB8)"]
    HB8 = 2,
    #[doc = "3: 16-Bit Host-Bus (HB16)"]
    HB16 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::NONE),
            1 => Val(MODE_A::SDRAM),
            2 => Val(MODE_A::HB8),
            3 => Val(MODE_A::HB16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MODE_A::SDRAM
    }
    #[doc = "Checks if the value of the field is `HB8`"]
    #[inline(always)]
    pub fn is_hb8(&self) -> bool {
        *self == MODE_A::HB8
    }
    #[doc = "Checks if the value of the field is `HB16`"]
    #[inline(always)]
    pub fn is_hb16(&self) -> bool {
        *self == MODE_A::HB16
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General Purpose"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(MODE_A::NONE)
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MODE_A::SDRAM)
    }
    #[doc = "8-Bit Host-Bus (HB8)"]
    #[inline(always)]
    pub fn hb8(self) -> &'a mut W {
        self.variant(MODE_A::HB8)
    }
    #[doc = "16-Bit Host-Bus (HB16)"]
    #[inline(always)]
    pub fn hb16(self) -> &'a mut W {
        self.variant(MODE_A::HB16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `BLKEN`"]
pub type BLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKEN`"]
pub struct BLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `INTDIV`"]
pub type INTDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTDIV`"]
pub struct INTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INTDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn blken(&self) -> BLKEN_R {
        BLKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn blken(&mut self) -> BLKEN_W {
        BLKEN_W { w: self }
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn intdiv(&mut self) -> INTDIV_W {
        INTDIV_W { w: self }
    }
}
