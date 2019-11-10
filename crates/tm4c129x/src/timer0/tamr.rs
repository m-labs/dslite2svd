#[doc = "Reader of register TAMR"]
pub type R = crate::R<u32, super::TAMR>;
#[doc = "Writer for register TAMR"]
pub type W = crate::W<u32, super::TAMR>;
#[doc = "Register TAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPTM Timer A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMR_A {
    #[doc = "1: One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "2: Periodic Timer mode"]
    PERIOD,
    #[doc = "3: Capture mode"]
    CAP,
}
impl From<TAMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMR_A) -> Self {
        match variant {
            TAMR_A::_1_SHOT => 1,
            TAMR_A::PERIOD => 2,
            TAMR_A::CAP => 3,
        }
    }
}
#[doc = "Reader of field `TAMR`"]
pub type TAMR_R = crate::R<u8, TAMR_A>;
impl TAMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAMR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TAMR_A::_1_SHOT),
            2 => Val(TAMR_A::PERIOD),
            3 => Val(TAMR_A::CAP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SHOT`"]
    #[inline(always)]
    pub fn is_1_shot(&self) -> bool {
        *self == TAMR_A::_1_SHOT
    }
    #[doc = "Checks if the value of the field is `PERIOD`"]
    #[inline(always)]
    pub fn is_period(&self) -> bool {
        *self == TAMR_A::PERIOD
    }
    #[doc = "Checks if the value of the field is `CAP`"]
    #[inline(always)]
    pub fn is_cap(&self) -> bool {
        *self == TAMR_A::CAP
    }
}
#[doc = "Write proxy for field `TAMR`"]
pub struct TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn _1_shot(self) -> &'a mut W {
        self.variant(TAMR_A::_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn period(self) -> &'a mut W {
        self.variant(TAMR_A::PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn cap(self) -> &'a mut W {
        self.variant(TAMR_A::CAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TACMR`"]
pub type TACMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACMR`"]
pub struct TACMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMR_W<'a> {
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
#[doc = "Reader of field `TAAMS`"]
pub type TAAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAAMS`"]
pub struct TAAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAAMS_W<'a> {
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
#[doc = "Reader of field `TACDIR`"]
pub type TACDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACDIR`"]
pub struct TACDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACDIR_W<'a> {
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
#[doc = "Reader of field `TAMIE`"]
pub type TAMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMIE`"]
pub struct TAMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIE_W<'a> {
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
#[doc = "Reader of field `TAWOT`"]
pub type TAWOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAWOT`"]
pub struct TAWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAWOT_W<'a> {
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
#[doc = "Reader of field `TASNAPS`"]
pub type TASNAPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TASNAPS`"]
pub struct TASNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TASNAPS_W<'a> {
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
#[doc = "Reader of field `TAILD`"]
pub type TAILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAILD`"]
pub struct TAILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILD_W<'a> {
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
#[doc = "Reader of field `TAPWMIE`"]
pub type TAPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPWMIE`"]
pub struct TAPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWMIE_W<'a> {
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
#[doc = "Reader of field `TAMRSU`"]
pub type TAMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMRSU`"]
pub struct TAMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMRSU_W<'a> {
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
#[doc = "Reader of field `TAPLO`"]
pub type TAPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPLO`"]
pub struct TAPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPLO_W<'a> {
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
#[doc = "Reader of field `TACINTD`"]
pub type TACINTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACINTD`"]
pub struct TACINTD_W<'a> {
    w: &'a mut W,
}
impl<'a> TACINTD_W<'a> {
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
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn tacmr(&self) -> TACMR_R {
        TACMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn taams(&self) -> TAAMS_R {
        TAAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn tacdir(&self) -> TACDIR_R {
        TACDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn tamie(&self) -> TAMIE_R {
        TAMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn tawot(&self) -> TAWOT_R {
        TAWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn tasnaps(&self) -> TASNAPS_R {
        TASNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn taild(&self) -> TAILD_R {
        TAILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tapwmie(&self) -> TAPWMIE_R {
        TAPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn tamrsu(&self) -> TAMRSU_R {
        TAMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn taplo(&self) -> TAPLO_R {
        TAPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tacintd(&self) -> TACINTD_R {
        TACINTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&self) -> TCACT_R {
        TCACT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn tamr(&mut self) -> TAMR_W {
        TAMR_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn tacmr(&mut self) -> TACMR_W {
        TACMR_W { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn taams(&mut self) -> TAAMS_W {
        TAAMS_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn tacdir(&mut self) -> TACDIR_W {
        TACDIR_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn tamie(&mut self) -> TAMIE_W {
        TAMIE_W { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn tawot(&mut self) -> TAWOT_W {
        TAWOT_W { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn tasnaps(&mut self) -> TASNAPS_W {
        TASNAPS_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn taild(&mut self) -> TAILD_W {
        TAILD_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tapwmie(&mut self) -> TAPWMIE_W {
        TAPWMIE_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn tamrsu(&mut self) -> TAMRSU_W {
        TAMRSU_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn taplo(&mut self) -> TAPLO_W {
        TAPLO_W { w: self }
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tacintd(&mut self) -> TACINTD_W {
        TACINTD_W { w: self }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&mut self) -> TCACT_W {
        TCACT_W { w: self }
    }
}
