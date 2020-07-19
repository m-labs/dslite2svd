#[doc = "Reader of register ALTCLKCFG"]
pub type R = crate::R<u32, super::ALTCLKCFG>;
#[doc = "Writer for register ALTCLKCFG"]
pub type W = crate::W<u32, super::ALTCLKCFG>;
#[doc = "Register ALTCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Alternate Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALTCLK_A {
    #[doc = "0: PIOSC"]
    PIOSC = 0,
    #[doc = "3: Hibernation Module Real-time clock output (RTCOSC)"]
    RTCOSC = 3,
    #[doc = "4: Low-frequency internal oscillator (LFIOSC)"]
    LFIOSC = 4,
}
impl From<ALTCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALTCLK`"]
pub type ALTCLK_R = crate::R<u8, ALTCLK_A>;
impl ALTCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALTCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALTCLK_A::PIOSC),
            3 => Val(ALTCLK_A::RTCOSC),
            4 => Val(ALTCLK_A::LFIOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == ALTCLK_A::PIOSC
    }
    #[doc = "Checks if the value of the field is `RTCOSC`"]
    #[inline(always)]
    pub fn is_rtcosc(&self) -> bool {
        *self == ALTCLK_A::RTCOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline(always)]
    pub fn is_lfiosc(&self) -> bool {
        *self == ALTCLK_A::LFIOSC
    }
}
#[doc = "Write proxy for field `ALTCLK`"]
pub struct ALTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALTCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(ALTCLK_A::PIOSC)
    }
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    #[inline(always)]
    pub fn rtcosc(self) -> &'a mut W {
        self.variant(ALTCLK_A::RTCOSC)
    }
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    #[inline(always)]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(ALTCLK_A::LFIOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn altclk(&self) -> ALTCLK_R {
        ALTCLK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn altclk(&mut self) -> ALTCLK_W {
        ALTCLK_W { w: self }
    }
}
