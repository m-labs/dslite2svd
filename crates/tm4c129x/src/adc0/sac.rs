#[doc = "Reader of register SAC"]
pub type R = crate::R<u32, super::SAC>;
#[doc = "Writer for register SAC"]
pub type W = crate::W<u32, super::SAC>;
#[doc = "Register SAC `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Averaging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVG_A {
    #[doc = "0: No hardware oversampling"]
    OFF,
    #[doc = "1: 2x hardware oversampling"]
    _2X,
    #[doc = "2: 4x hardware oversampling"]
    _4X,
    #[doc = "3: 8x hardware oversampling"]
    _8X,
    #[doc = "4: 16x hardware oversampling"]
    _16X,
    #[doc = "5: 32x hardware oversampling"]
    _32X,
    #[doc = "6: 64x hardware oversampling"]
    _64X,
}
impl From<AVG_A> for u8 {
    #[inline(always)]
    fn from(variant: AVG_A) -> Self {
        match variant {
            AVG_A::OFF => 0,
            AVG_A::_2X => 1,
            AVG_A::_4X => 2,
            AVG_A::_8X => 3,
            AVG_A::_16X => 4,
            AVG_A::_32X => 5,
            AVG_A::_64X => 6,
        }
    }
}
#[doc = "Reader of field `AVG`"]
pub type AVG_R = crate::R<u8, AVG_A>;
impl AVG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AVG_A::OFF),
            1 => Val(AVG_A::_2X),
            2 => Val(AVG_A::_4X),
            3 => Val(AVG_A::_8X),
            4 => Val(AVG_A::_16X),
            5 => Val(AVG_A::_32X),
            6 => Val(AVG_A::_64X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AVG_A::OFF
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == AVG_A::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == AVG_A::_4X
    }
    #[doc = "Checks if the value of the field is `_8X`"]
    #[inline(always)]
    pub fn is_8x(&self) -> bool {
        *self == AVG_A::_8X
    }
    #[doc = "Checks if the value of the field is `_16X`"]
    #[inline(always)]
    pub fn is_16x(&self) -> bool {
        *self == AVG_A::_16X
    }
    #[doc = "Checks if the value of the field is `_32X`"]
    #[inline(always)]
    pub fn is_32x(&self) -> bool {
        *self == AVG_A::_32X
    }
    #[doc = "Checks if the value of the field is `_64X`"]
    #[inline(always)]
    pub fn is_64x(&self) -> bool {
        *self == AVG_A::_64X
    }
}
#[doc = "Write proxy for field `AVG`"]
pub struct AVG_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AVG_A::OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut W {
        self.variant(AVG_A::_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut W {
        self.variant(AVG_A::_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn _8x(self) -> &'a mut W {
        self.variant(AVG_A::_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn _16x(self) -> &'a mut W {
        self.variant(AVG_A::_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn _32x(self) -> &'a mut W {
        self.variant(AVG_A::_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn _64x(self) -> &'a mut W {
        self.variant(AVG_A::_64X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn avg(&self) -> AVG_R {
        AVG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn avg(&mut self) -> AVG_W {
        AVG_W { w: self }
    }
}
