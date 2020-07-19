#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Writer for register PC"]
pub type W = crate::W<u32, super::PC>;
#[doc = "Register PC `reset()`'s with value 0"]
impl crate::ResetValue for super::PC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SR_A {
    #[doc = "1: 125 ksps"]
    _125K = 1,
    #[doc = "3: 250 ksps"]
    _250K = 3,
    #[doc = "5: 500 ksps"]
    _500K = 5,
    #[doc = "7: 1 Msps"]
    _1M = 7,
}
impl From<SR_A> for u8 {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<u8, SR_A>;
impl SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SR_A::_125K),
            3 => Val(SR_A::_250K),
            5 => Val(SR_A::_500K),
            7 => Val(SR_A::_1M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline(always)]
    pub fn is_125k(&self) -> bool {
        *self == SR_A::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline(always)]
    pub fn is_250k(&self) -> bool {
        *self == SR_A::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline(always)]
    pub fn is_500k(&self) -> bool {
        *self == SR_A::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == SR_A::_1M
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "125 ksps"]
    #[inline(always)]
    pub fn _125k(self) -> &'a mut W {
        self.variant(SR_A::_125K)
    }
    #[doc = "250 ksps"]
    #[inline(always)]
    pub fn _250k(self) -> &'a mut W {
        self.variant(SR_A::_250K)
    }
    #[doc = "500 ksps"]
    #[inline(always)]
    pub fn _500k(self) -> &'a mut W {
        self.variant(SR_A::_500K)
    }
    #[doc = "1 Msps"]
    #[inline(always)]
    pub fn _1m(self) -> &'a mut W {
        self.variant(SR_A::_1M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
}
