#[doc = "Reader of register MTPR"]
pub type R = crate::R<u32, super::MTPR>;
#[doc = "Writer for register MTPR"]
pub type W = crate::W<u32, super::MTPR>;
#[doc = "Register MTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPR`"]
pub type TPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPR`"]
pub struct TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS`"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
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
#[doc = "Glitch Suppression Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSEL_A {
    #[doc = "0: Bypass"]
    BYPASS,
    #[doc = "1: 1 clock"]
    _1,
    #[doc = "2: 2 clocks"]
    _2,
    #[doc = "3: 3 clocks"]
    _3,
    #[doc = "4: 4 clocks"]
    _4,
    #[doc = "5: 8 clocks"]
    _8,
    #[doc = "6: 16 clocks"]
    _16,
    #[doc = "7: 31 clocks"]
    _31,
}
impl From<PULSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULSEL_A) -> Self {
        match variant {
            PULSEL_A::BYPASS => 0,
            PULSEL_A::_1 => 1,
            PULSEL_A::_2 => 2,
            PULSEL_A::_3 => 3,
            PULSEL_A::_4 => 4,
            PULSEL_A::_8 => 5,
            PULSEL_A::_16 => 6,
            PULSEL_A::_31 => 7,
        }
    }
}
#[doc = "Reader of field `PULSEL`"]
pub type PULSEL_R = crate::R<u8, PULSEL_A>;
impl PULSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSEL_A {
        match self.bits {
            0 => PULSEL_A::BYPASS,
            1 => PULSEL_A::_1,
            2 => PULSEL_A::_2,
            3 => PULSEL_A::_3,
            4 => PULSEL_A::_4,
            5 => PULSEL_A::_8,
            6 => PULSEL_A::_16,
            7 => PULSEL_A::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == PULSEL_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PULSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PULSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PULSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PULSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PULSEL_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PULSEL_A::_16
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == PULSEL_A::_31
    }
}
#[doc = "Write proxy for field `PULSEL`"]
pub struct PULSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(PULSEL_A::BYPASS)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PULSEL_A::_1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PULSEL_A::_2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PULSEL_A::_3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PULSEL_A::_4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PULSEL_A::_8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PULSEL_A::_16)
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(PULSEL_A::_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn pulsel(&self) -> PULSEL_R {
        PULSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn tpr(&mut self) -> TPR_W {
        TPR_W { w: self }
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn pulsel(&mut self) -> PULSEL_W {
        PULSEL_W { w: self }
    }
}
