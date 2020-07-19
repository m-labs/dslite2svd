#[doc = "Reader of register LDODPCTL"]
pub type R = crate::R<u32, super::LDODPCTL>;
#[doc = "Writer for register LDODPCTL"]
pub type W = crate::W<u32, super::LDODPCTL>;
#[doc = "Register LDODPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LDODPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LDO Output Voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLDO_A {
    #[doc = "18: 0.90 V"]
    _0_90V = 18,
    #[doc = "19: 0.95 V"]
    _0_95V = 19,
    #[doc = "20: 1.00 V"]
    _1_00V = 20,
    #[doc = "21: 1.05 V"]
    _1_05V = 21,
    #[doc = "22: 1.10 V"]
    _1_10V = 22,
    #[doc = "23: 1.15 V"]
    _1_15V = 23,
    #[doc = "24: 1.20 V"]
    _1_20V = 24,
    #[doc = "25: 1.25 V"]
    _1_25V = 25,
    #[doc = "26: 1.30 V"]
    _1_30V = 26,
    #[doc = "27: 1.35 V"]
    _1_35V = 27,
}
impl From<VLDO_A> for u8 {
    #[inline(always)]
    fn from(variant: VLDO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VLDO`"]
pub type VLDO_R = crate::R<u8, VLDO_A>;
impl VLDO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VLDO_A> {
        use crate::Variant::*;
        match self.bits {
            18 => Val(VLDO_A::_0_90V),
            19 => Val(VLDO_A::_0_95V),
            20 => Val(VLDO_A::_1_00V),
            21 => Val(VLDO_A::_1_05V),
            22 => Val(VLDO_A::_1_10V),
            23 => Val(VLDO_A::_1_15V),
            24 => Val(VLDO_A::_1_20V),
            25 => Val(VLDO_A::_1_25V),
            26 => Val(VLDO_A::_1_30V),
            27 => Val(VLDO_A::_1_35V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_90V`"]
    #[inline(always)]
    pub fn is_0_90v(&self) -> bool {
        *self == VLDO_A::_0_90V
    }
    #[doc = "Checks if the value of the field is `_0_95V`"]
    #[inline(always)]
    pub fn is_0_95v(&self) -> bool {
        *self == VLDO_A::_0_95V
    }
    #[doc = "Checks if the value of the field is `_1_00V`"]
    #[inline(always)]
    pub fn is_1_00v(&self) -> bool {
        *self == VLDO_A::_1_00V
    }
    #[doc = "Checks if the value of the field is `_1_05V`"]
    #[inline(always)]
    pub fn is_1_05v(&self) -> bool {
        *self == VLDO_A::_1_05V
    }
    #[doc = "Checks if the value of the field is `_1_10V`"]
    #[inline(always)]
    pub fn is_1_10v(&self) -> bool {
        *self == VLDO_A::_1_10V
    }
    #[doc = "Checks if the value of the field is `_1_15V`"]
    #[inline(always)]
    pub fn is_1_15v(&self) -> bool {
        *self == VLDO_A::_1_15V
    }
    #[doc = "Checks if the value of the field is `_1_20V`"]
    #[inline(always)]
    pub fn is_1_20v(&self) -> bool {
        *self == VLDO_A::_1_20V
    }
    #[doc = "Checks if the value of the field is `_1_25V`"]
    #[inline(always)]
    pub fn is_1_25v(&self) -> bool {
        *self == VLDO_A::_1_25V
    }
    #[doc = "Checks if the value of the field is `_1_30V`"]
    #[inline(always)]
    pub fn is_1_30v(&self) -> bool {
        *self == VLDO_A::_1_30V
    }
    #[doc = "Checks if the value of the field is `_1_35V`"]
    #[inline(always)]
    pub fn is_1_35v(&self) -> bool {
        *self == VLDO_A::_1_35V
    }
}
#[doc = "Write proxy for field `VLDO`"]
pub struct VLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> VLDO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLDO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.90 V"]
    #[inline(always)]
    pub fn _0_90v(self) -> &'a mut W {
        self.variant(VLDO_A::_0_90V)
    }
    #[doc = "0.95 V"]
    #[inline(always)]
    pub fn _0_95v(self) -> &'a mut W {
        self.variant(VLDO_A::_0_95V)
    }
    #[doc = "1.00 V"]
    #[inline(always)]
    pub fn _1_00v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_00V)
    }
    #[doc = "1.05 V"]
    #[inline(always)]
    pub fn _1_05v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_05V)
    }
    #[doc = "1.10 V"]
    #[inline(always)]
    pub fn _1_10v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn _1_15v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_15V)
    }
    #[doc = "1.20 V"]
    #[inline(always)]
    pub fn _1_20v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn _1_25v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_25V)
    }
    #[doc = "1.30 V"]
    #[inline(always)]
    pub fn _1_30v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_30V)
    }
    #[doc = "1.35 V"]
    #[inline(always)]
    pub fn _1_35v(self) -> &'a mut W {
        self.variant(VLDO_A::_1_35V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VADJEN`"]
pub type VADJEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VADJEN`"]
pub struct VADJEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VADJEN_W<'a> {
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
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn vldo(&self) -> VLDO_R {
        VLDO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn vadjen(&self) -> VADJEN_R {
        VADJEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn vldo(&mut self) -> VLDO_W {
        VLDO_W { w: self }
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn vadjen(&mut self) -> VADJEN_W {
        VADJEN_W { w: self }
    }
}
