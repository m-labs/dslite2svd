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
#[doc = "ADC Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: PLL VCO divided by CLKDIV"]
    SYSPLL,
    #[doc = "1: PIOSC"]
    PIOSC,
    #[doc = "2: MOSC"]
    MOSC,
}
impl From<CS_A> for u8 {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        match variant {
            CS_A::SYSPLL => 0,
            CS_A::PIOSC => 1,
            CS_A::MOSC => 2,
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
            1 => Val(CS_A::PIOSC),
            2 => Val(CS_A::MOSC),
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
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline(always)]
    pub fn is_mosc(&self) -> bool {
        *self == CS_A::MOSC
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
    #[doc = "PLL VCO divided by CLKDIV"]
    #[inline(always)]
    pub fn syspll(self) -> &'a mut W {
        self.variant(CS_A::SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(CS_A::PIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn mosc(self) -> &'a mut W {
        self.variant(CS_A::MOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - PLL VCO Clock Divisor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bits 4:9 - PLL VCO Clock Divisor"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
}
