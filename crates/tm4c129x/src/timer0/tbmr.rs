#[doc = "Reader of register TBMR"]
pub type R = crate::R<u32, super::TBMR>;
#[doc = "Writer for register TBMR"]
pub type W = crate::W<u32, super::TBMR>;
#[doc = "Register TBMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPTM Timer B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMR_A {
    #[doc = "1: One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "2: Periodic Timer mode"]
    PERIOD,
    #[doc = "3: Capture mode"]
    CAP,
}
impl From<TBMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TBMR_A) -> Self {
        match variant {
            TBMR_A::_1_SHOT => 1,
            TBMR_A::PERIOD => 2,
            TBMR_A::CAP => 3,
        }
    }
}
#[doc = "Reader of field `TBMR`"]
pub type TBMR_R = crate::R<u8, TBMR_A>;
impl TBMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TBMR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TBMR_A::_1_SHOT),
            2 => Val(TBMR_A::PERIOD),
            3 => Val(TBMR_A::CAP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SHOT`"]
    #[inline(always)]
    pub fn is_1_shot(&self) -> bool {
        *self == TBMR_A::_1_SHOT
    }
    #[doc = "Checks if the value of the field is `PERIOD`"]
    #[inline(always)]
    pub fn is_period(&self) -> bool {
        *self == TBMR_A::PERIOD
    }
    #[doc = "Checks if the value of the field is `CAP`"]
    #[inline(always)]
    pub fn is_cap(&self) -> bool {
        *self == TBMR_A::CAP
    }
}
#[doc = "Write proxy for field `TBMR`"]
pub struct TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn _1_shot(self) -> &'a mut W {
        self.variant(TBMR_A::_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn period(self) -> &'a mut W {
        self.variant(TBMR_A::PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn cap(self) -> &'a mut W {
        self.variant(TBMR_A::CAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TBCMR`"]
pub type TBCMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCMR`"]
pub struct TBCMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TBAMS`"]
pub type TBAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBAMS`"]
pub struct TBAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBAMS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TBCDIR`"]
pub type TBCDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCDIR`"]
pub struct TBCDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TBMIE`"]
pub type TBMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIE`"]
pub struct TBMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TBWOT`"]
pub type TBWOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBWOT`"]
pub struct TBWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBWOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TBSNAPS`"]
pub type TBSNAPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBSNAPS`"]
pub struct TBSNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSNAPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TBILD`"]
pub type TBILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBILD`"]
pub struct TBILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILD_W<'a> {
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
#[doc = "Reader of field `TBPWMIE`"]
pub type TBPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWMIE`"]
pub struct TBPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TBMRSU`"]
pub type TBMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMRSU`"]
pub struct TBMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMRSU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBPLO`"]
pub type TBPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPLO`"]
pub struct TBPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPLO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TBCINTD`"]
pub type TBCINTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCINTD`"]
pub struct TBCINTD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCINTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCACT_A {
    #[doc = "0: Disable compare operations"]
    NONE,
    #[doc = "1: Toggle State on Time-Out"]
    TOGGLE,
    #[doc = "2: Clear CCP on Time-Out"]
    CLRTO,
    #[doc = "3: Set CCP on Time-Out"]
    SETTO,
    #[doc = "4: Set CCP immediately and toggle on Time-Out"]
    SETTOGTO,
    #[doc = "5: Clear CCP immediately and toggle on Time-Out"]
    CLRTOGTO,
    #[doc = "6: Set CCP immediately and clear on Time-Out"]
    SETCLRTO,
    #[doc = "7: Clear CCP immediately and set on Time-Out"]
    CLRSETTO,
}
impl From<TCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCACT_A) -> Self {
        match variant {
            TCACT_A::NONE => 0,
            TCACT_A::TOGGLE => 1,
            TCACT_A::CLRTO => 2,
            TCACT_A::SETTO => 3,
            TCACT_A::SETTOGTO => 4,
            TCACT_A::CLRTOGTO => 5,
            TCACT_A::SETCLRTO => 6,
            TCACT_A::CLRSETTO => 7,
        }
    }
}
#[doc = "Reader of field `TCACT`"]
pub type TCACT_R = crate::R<u8, TCACT_A>;
impl TCACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCACT_A {
        match self.bits {
            0 => TCACT_A::NONE,
            1 => TCACT_A::TOGGLE,
            2 => TCACT_A::CLRTO,
            3 => TCACT_A::SETTO,
            4 => TCACT_A::SETTOGTO,
            5 => TCACT_A::CLRTOGTO,
            6 => TCACT_A::SETCLRTO,
            7 => TCACT_A::CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TCACT_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == TCACT_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLRTO`"]
    #[inline(always)]
    pub fn is_clrto(&self) -> bool {
        *self == TCACT_A::CLRTO
    }
    #[doc = "Checks if the value of the field is `SETTO`"]
    #[inline(always)]
    pub fn is_setto(&self) -> bool {
        *self == TCACT_A::SETTO
    }
    #[doc = "Checks if the value of the field is `SETTOGTO`"]
    #[inline(always)]
    pub fn is_settogto(&self) -> bool {
        *self == TCACT_A::SETTOGTO
    }
    #[doc = "Checks if the value of the field is `CLRTOGTO`"]
    #[inline(always)]
    pub fn is_clrtogto(&self) -> bool {
        *self == TCACT_A::CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `SETCLRTO`"]
    #[inline(always)]
    pub fn is_setclrto(&self) -> bool {
        *self == TCACT_A::SETCLRTO
    }
    #[doc = "Checks if the value of the field is `CLRSETTO`"]
    #[inline(always)]
    pub fn is_clrsetto(&self) -> bool {
        *self == TCACT_A::CLRSETTO
    }
}
#[doc = "Write proxy for field `TCACT`"]
pub struct TCACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TCACT_A::NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(TCACT_A::TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline(always)]
    pub fn clrto(self) -> &'a mut W {
        self.variant(TCACT_A::CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline(always)]
    pub fn setto(self) -> &'a mut W {
        self.variant(TCACT_A::SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn settogto(self) -> &'a mut W {
        self.variant(TCACT_A::SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn clrtogto(self) -> &'a mut W {
        self.variant(TCACT_A::CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn setclrto(self) -> &'a mut W {
        self.variant(TCACT_A::SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline(always)]
    pub fn clrsetto(self) -> &'a mut W {
        self.variant(TCACT_A::CLRSETTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcmr(&self) -> TBCMR_R {
        TBCMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn tbwot(&self) -> TBWOT_R {
        TBWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TBSNAPS_R {
        TBSNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tbcintd(&self) -> TBCINTD_R {
        TBCINTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&self) -> TCACT_R {
        TCACT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W {
        TBMR_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcmr(&mut self) -> TBCMR_W {
        TBCMR_W { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn tbams(&mut self) -> TBAMS_W {
        TBAMS_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TBCDIR_W {
        TBCDIR_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TBMIE_W {
        TBMIE_W { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn tbwot(&mut self) -> TBWOT_W {
        TBWOT_W { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn tbsnaps(&mut self) -> TBSNAPS_W {
        TBSNAPS_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&mut self) -> TBILD_W {
        TBILD_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W {
        TBPWMIE_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TBMRSU_W {
        TBMRSU_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn tbplo(&mut self) -> TBPLO_W {
        TBPLO_W { w: self }
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tbcintd(&mut self) -> TBCINTD_W {
        TBCINTD_W { w: self }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&mut self) -> TCACT_W {
        TCACT_W { w: self }
    }
}
