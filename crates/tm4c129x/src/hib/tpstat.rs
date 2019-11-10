#[doc = "Reader of register TPSTAT"]
pub type R = crate::R<u32, super::TPSTAT>;
#[doc = "Writer for register TPSTAT"]
pub type W = crate::W<u32, super::TPSTAT>;
#[doc = "Register TPSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TPSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCFAIL`"]
pub type XOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCFAIL`"]
pub struct XOSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `XOSCST`"]
pub type XOSCST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCST`"]
pub struct XOSCST_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Tamper Module Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Tamper disabled"]
    DISABLED,
    #[doc = "1: Tamper configured"]
    CONFIGED,
    #[doc = "2: Tamper pin event occurred"]
    ERROR,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::DISABLED => 0,
            STATE_A::CONFIGED => 1,
            STATE_A::ERROR => 2,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::DISABLED),
            1 => Val(STATE_A::CONFIGED),
            2 => Val(STATE_A::ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CONFIGED`"]
    #[inline(always)]
    pub fn is_configed(&self) -> bool {
        *self == STATE_A::CONFIGED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == STATE_A::ERROR
    }
}
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tamper disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STATE_A::DISABLED)
    }
    #[doc = "Tamper configured"]
    #[inline(always)]
    pub fn configed(self) -> &'a mut W {
        self.variant(STATE_A::CONFIGED)
    }
    #[doc = "Tamper pin event occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(STATE_A::ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn xoscst(&self) -> XOSCST_R {
        XOSCST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XOSCFAIL_W {
        XOSCFAIL_W { w: self }
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn xoscst(&mut self) -> XOSCST_W {
        XOSCST_W { w: self }
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
}
