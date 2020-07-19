#[doc = "Reader of register RSCLKCFG"]
pub type R = crate::R<u32, super::RSCLKCFG>;
#[doc = "Writer for register RSCLKCFG"]
pub type W = crate::W<u32, super::RSCLKCFG>;
#[doc = "Register RSCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RSCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSYSDIV`"]
pub type PSYSDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSYSDIV`"]
pub struct PSYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PSYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `OSYSDIV`"]
pub type OSYSDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OSYSDIV`"]
pub struct OSYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSCSRC_A {
    #[doc = "0: PIOSC is oscillator source"]
    PIOSC = 0,
    #[doc = "2: LFIOSC is oscillator source"]
    LFIOSC = 2,
    #[doc = "3: MOSC is oscillator source"]
    MOSC = 3,
    #[doc = "4: Hibernation Module RTC Oscillator (RTCOSC)"]
    RTC = 4,
}
impl From<OSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSCSRC`"]
pub type OSCSRC_R = crate::R<u8, OSCSRC_A>;
impl OSCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSCSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSCSRC_A::PIOSC),
            2 => Val(OSCSRC_A::LFIOSC),
            3 => Val(OSCSRC_A::MOSC),
            4 => Val(OSCSRC_A::RTC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == OSCSRC_A::PIOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline(always)]
    pub fn is_lfiosc(&self) -> bool {
        *self == OSCSRC_A::LFIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline(always)]
    pub fn is_mosc(&self) -> bool {
        *self == OSCSRC_A::MOSC
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == OSCSRC_A::RTC
    }
}
#[doc = "Write proxy for field `OSCSRC`"]
pub struct OSCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIOSC is oscillator source"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(OSCSRC_A::PIOSC)
    }
    #[doc = "LFIOSC is oscillator source"]
    #[inline(always)]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(OSCSRC_A::LFIOSC)
    }
    #[doc = "MOSC is oscillator source"]
    #[inline(always)]
    pub fn mosc(self) -> &'a mut W {
        self.variant(OSCSRC_A::MOSC)
    }
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(OSCSRC_A::RTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "PLL Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: PIOSC is PLL input clock source"]
    PIOSC = 0,
    #[doc = "3: MOSC is the PLL input clock source"]
    MOSC = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<u8, PLLSRC_A>;
impl PLLSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLLSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLLSRC_A::PIOSC),
            3 => Val(PLLSRC_A::MOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == PLLSRC_A::PIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline(always)]
    pub fn is_mosc(&self) -> bool {
        *self == PLLSRC_A::MOSC
    }
}
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIOSC is PLL input clock source"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(PLLSRC_A::PIOSC)
    }
    #[doc = "MOSC is the PLL input clock source"]
    #[inline(always)]
    pub fn mosc(self) -> &'a mut W {
        self.variant(PLLSRC_A::MOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `USEPLL`"]
pub type USEPLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEPLL`"]
pub struct USEPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> USEPLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ACG`"]
pub type ACG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACG`"]
pub struct ACG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `NEWFREQ`"]
pub type NEWFREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEWFREQ`"]
pub struct NEWFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWFREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MEMTIMU`"]
pub type MEMTIMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEMTIMU`"]
pub struct MEMTIMU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMTIMU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn psysdiv(&self) -> PSYSDIV_R {
        PSYSDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn osysdiv(&self) -> OSYSDIV_R {
        OSYSDIV_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn oscsrc(&self) -> OSCSRC_R {
        OSCSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn usepll(&self) -> USEPLL_R {
        USEPLL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn acg(&self) -> ACG_R {
        ACG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn newfreq(&self) -> NEWFREQ_R {
        NEWFREQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn memtimu(&self) -> MEMTIMU_R {
        MEMTIMU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn psysdiv(&mut self) -> PSYSDIV_W {
        PSYSDIV_W { w: self }
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn osysdiv(&mut self) -> OSYSDIV_W {
        OSYSDIV_W { w: self }
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn oscsrc(&mut self) -> OSCSRC_W {
        OSCSRC_W { w: self }
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn usepll(&mut self) -> USEPLL_W {
        USEPLL_W { w: self }
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn acg(&mut self) -> ACG_W {
        ACG_W { w: self }
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn newfreq(&mut self) -> NEWFREQ_W {
        NEWFREQ_W { w: self }
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn memtimu(&mut self) -> MEMTIMU_W {
        MEMTIMU_W { w: self }
    }
}
