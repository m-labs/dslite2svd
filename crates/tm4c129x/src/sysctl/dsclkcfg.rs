#[doc = "Reader of register DSCLKCFG"]
pub type R = crate::R<u32, super::DSCLKCFG>;
#[doc = "Writer for register DSCLKCFG"]
pub type W = crate::W<u32, super::DSCLKCFG>;
#[doc = "Register DSCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSSYSDIV`"]
pub type DSSYSDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSSYSDIV`"]
pub struct DSSYSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DSSYSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Deep Sleep Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSOSCSRC_A {
    #[doc = "0: PIOSC"]
    PIOSC,
    #[doc = "2: LFIOSC"]
    LFIOSC,
    #[doc = "3: MOSC"]
    MOSC,
    #[doc = "4: Hibernation Module RTCOSC"]
    RTC,
}
impl From<DSOSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSOSCSRC_A) -> Self {
        match variant {
            DSOSCSRC_A::PIOSC => 0,
            DSOSCSRC_A::LFIOSC => 2,
            DSOSCSRC_A::MOSC => 3,
            DSOSCSRC_A::RTC => 4,
        }
    }
}
#[doc = "Reader of field `DSOSCSRC`"]
pub type DSOSCSRC_R = crate::R<u8, DSOSCSRC_A>;
impl DSOSCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSOSCSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSOSCSRC_A::PIOSC),
            2 => Val(DSOSCSRC_A::LFIOSC),
            3 => Val(DSOSCSRC_A::MOSC),
            4 => Val(DSOSCSRC_A::RTC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline(always)]
    pub fn is_piosc(&self) -> bool {
        *self == DSOSCSRC_A::PIOSC
    }
    #[doc = "Checks if the value of the field is `LFIOSC`"]
    #[inline(always)]
    pub fn is_lfiosc(&self) -> bool {
        *self == DSOSCSRC_A::LFIOSC
    }
    #[doc = "Checks if the value of the field is `MOSC`"]
    #[inline(always)]
    pub fn is_mosc(&self) -> bool {
        *self == DSOSCSRC_A::MOSC
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == DSOSCSRC_A::RTC
    }
}
#[doc = "Write proxy for field `DSOSCSRC`"]
pub struct DSOSCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSOSCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSOSCSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn piosc(self) -> &'a mut W {
        self.variant(DSOSCSRC_A::PIOSC)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn lfiosc(self) -> &'a mut W {
        self.variant(DSOSCSRC_A::LFIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn mosc(self) -> &'a mut W {
        self.variant(DSOSCSRC_A::MOSC)
    }
    #[doc = "Hibernation Module RTCOSC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(DSOSCSRC_A::RTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MOSCDPD`"]
pub type MOSCDPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCDPD`"]
pub struct MOSCDPD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCDPD_W<'a> {
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
#[doc = "Reader of field `PIOSCPD`"]
pub type PIOSCPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIOSCPD`"]
pub struct PIOSCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PIOSCPD_W<'a> {
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
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn dssysdiv(&self) -> DSSYSDIV_R {
        DSSYSDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn dsoscsrc(&self) -> DSOSCSRC_R {
        DSOSCSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn moscdpd(&self) -> MOSCDPD_R {
        MOSCDPD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn pioscpd(&self) -> PIOSCPD_R {
        PIOSCPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn dssysdiv(&mut self) -> DSSYSDIV_W {
        DSSYSDIV_W { w: self }
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn dsoscsrc(&mut self) -> DSOSCSRC_W {
        DSOSCSRC_W { w: self }
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn moscdpd(&mut self) -> MOSCDPD_W {
        MOSCDPD_W { w: self }
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn pioscpd(&mut self) -> PIOSCPD_W {
        PIOSCPD_W { w: self }
    }
}
