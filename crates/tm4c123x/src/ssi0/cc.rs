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
#[doc = "SSI Baud Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: System clock (based on clock source and divisor factor)"]
    SYSPLL,
    #[doc = "5: PIOSC"]
    PIOSC,
}
impl From<CS_A> for u8 {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        match variant {
            CS_A::SYSPLL => 0,
            CS_A::PIOSC => 5,
        }
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<u8, CS_A>;
impl CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CS_A::SYSPLL),
            5 => Val(CS_A::PIOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSPLL`"]
    #[inline(always)]
    pub fn is_syspll(&self) -> bool {
        *self == CS_A::SYSPLL
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == CS_A::PIOSC
    }
}
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System clock (based on clock source and divisor factor)"]
    #[inline(always)]
    pub fn syspll(self) -> &'a mut W {
        self.variant(CS_A::SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(CS_A::PIOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
}
