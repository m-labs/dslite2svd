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
#[doc = "Conversion Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_A {
    #[doc = "1: Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    _1_8,
    #[doc = "3: Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    _1_4,
    #[doc = "5: Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    _1_2,
    #[doc = "7: Full conversion rate (FCONV) as defined by TADC and NSH"]
    FULL,
}
impl From<MCR_A> for u8 {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        match variant {
            MCR_A::_1_8 => 1,
            MCR_A::_1_4 => 3,
            MCR_A::_1_2 => 5,
            MCR_A::FULL => 7,
        }
    }
}
#[doc = "Reader of field `MCR`"]
pub type MCR_R = crate::R<u8, MCR_A>;
impl MCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MCR_A::_1_8),
            3 => Val(MCR_A::_1_4),
            5 => Val(MCR_A::_1_2),
            7 => Val(MCR_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == MCR_A::_1_8
    }
    #[doc = "Checks if the value of the field is `_1_4`"]
    #[inline(always)]
    pub fn is_1_4(&self) -> bool {
        *self == MCR_A::_1_4
    }
    #[doc = "Checks if the value of the field is `_1_2`"]
    #[inline(always)]
    pub fn is_1_2(&self) -> bool {
        *self == MCR_A::_1_2
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == MCR_A::FULL
    }
}
#[doc = "Write proxy for field `MCR`"]
pub struct MCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(MCR_A::_1_8)
    }
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn _1_4(self) -> &'a mut W {
        self.variant(MCR_A::_1_4)
    }
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn _1_2(self) -> &'a mut W {
        self.variant(MCR_A::_1_2)
    }
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(MCR_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W {
        MCR_W { w: self }
    }
}
