#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Voltage Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF_A {
    #[doc = "0: VDDA and GNDA are the voltage references"]
    INTERNAL = 0,
}
impl From<VREF_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREF`"]
pub type VREF_R = crate::R<bool, VREF_A>;
impl VREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VREF_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(VREF_A::INTERNAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == VREF_A::INTERNAL
    }
}
#[doc = "Write proxy for field `VREF`"]
pub struct VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREF_A) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "VDDA and GNDA are the voltage references"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(VREF_A::INTERNAL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Reference Select"]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Reference Select"]
    #[inline(always)]
    pub fn vref(&mut self) -> VREF_W {
        VREF_W { w: self }
    }
}
