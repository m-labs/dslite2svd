#[doc = "Reader of register CC"]
pub type R = crate::R<u32, super::CC>;
#[doc = "Writer for register CC"]
pub type W = crate::W<u32, super::CC>;
#[doc = "Register CC `reset()`'s with value 0"]
impl crate::ResetValue for super::CC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMDIV_A {
    #[doc = "0: /2"]
    _2,
    #[doc = "1: /4"]
    _4,
    #[doc = "2: /8"]
    _8,
    #[doc = "3: /16"]
    _16,
    #[doc = "4: /32"]
    _32,
    #[doc = "5: /64"]
    _64,
}
impl From<PWMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMDIV_A) -> Self {
        match variant {
            PWMDIV_A::_2 => 0,
            PWMDIV_A::_4 => 1,
            PWMDIV_A::_8 => 2,
            PWMDIV_A::_16 => 3,
            PWMDIV_A::_32 => 4,
            PWMDIV_A::_64 => 5,
        }
    }
}
#[doc = "Reader of field `PWMDIV`"]
pub type PWMDIV_R = crate::R<u8, PWMDIV_A>;
impl PWMDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWMDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWMDIV_A::_2),
            1 => Val(PWMDIV_A::_4),
            2 => Val(PWMDIV_A::_8),
            3 => Val(PWMDIV_A::_16),
            4 => Val(PWMDIV_A::_32),
            5 => Val(PWMDIV_A::_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PWMDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PWMDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PWMDIV_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PWMDIV_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PWMDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PWMDIV_A::_64
    }
}
#[doc = "Write proxy for field `PWMDIV`"]
pub struct PWMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PWMDIV_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PWMDIV_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PWMDIV_A::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PWMDIV_A::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PWMDIV_A::_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PWMDIV_A::_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `USEPWM`"]
pub type USEPWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEPWM`"]
pub struct USEPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> USEPWM_W<'a> {
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
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwmdiv(&self) -> PWMDIV_R {
        PWMDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn usepwm(&self) -> USEPWM_R {
        USEPWM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwmdiv(&mut self) -> PWMDIV_W {
        PWMDIV_W { w: self }
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn usepwm(&mut self) -> USEPWM_W {
        USEPWM_W { w: self }
    }
}
