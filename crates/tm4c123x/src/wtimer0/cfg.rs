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
#[doc = "GPTM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG_A {
    #[doc = "0: For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    _32_BIT_TIMER = 0,
    #[doc = "1: For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    _32_BIT_RTC = 1,
    #[doc = "4: For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    _16_BIT = 4,
}
impl From<CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG`"]
pub type CFG_R = crate::R<u8, CFG_A>;
impl CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFG_A::_32_BIT_TIMER),
            1 => Val(CFG_A::_32_BIT_RTC),
            4 => Val(CFG_A::_16_BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32_BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32_bit_timer(&self) -> bool {
        *self == CFG_A::_32_BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `_32_BIT_RTC`"]
    #[inline(always)]
    pub fn is_32_bit_rtc(&self) -> bool {
        *self == CFG_A::_32_BIT_RTC
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == CFG_A::_16_BIT
    }
}
#[doc = "Write proxy for field `CFG`"]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    #[inline(always)]
    pub fn _32_bit_timer(self) -> &'a mut W {
        self.variant(CFG_A::_32_BIT_TIMER)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn _32_bit_rtc(self) -> &'a mut W {
        self.variant(CFG_A::_32_BIT_RTC)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(CFG_A::_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
}
