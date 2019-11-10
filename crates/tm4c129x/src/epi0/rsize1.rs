#[doc = "Reader of register RSIZE1"]
pub type R = crate::R<u32, super::RSIZE1>;
#[doc = "Writer for register RSIZE1"]
pub type W = crate::W<u32, super::RSIZE1>;
#[doc = "Register RSIZE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSIZE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "1: Byte (8 bits)"]
    _8BIT,
    #[doc = "2: Half-word (16 bits)"]
    _16BIT,
    #[doc = "3: Word (32 bits)"]
    _32BIT,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::_8BIT => 1,
            SIZE_A::_16BIT => 2,
            SIZE_A::_32BIT => 3,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SIZE_A::_8BIT),
            2 => Val(SIZE_A::_16BIT),
            3 => Val(SIZE_A::_32BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == SIZE_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == SIZE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == SIZE_A::_32BIT
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte (8 bits)"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(SIZE_A::_8BIT)
    }
    #[doc = "Half-word (16 bits)"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(SIZE_A::_16BIT)
    }
    #[doc = "Word (32 bits)"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(SIZE_A::_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}
