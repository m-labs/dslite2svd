#[doc = "Reader of register MCR2"]
pub type R = crate::R<u32, super::MCR2>;
#[doc = "Writer for register MCR2"]
pub type W = crate::W<u32, super::MCR2>;
#[doc = "Register MCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C Glitch Filter Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GFPW_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: 1 clock"]
    _1 = 1,
    #[doc = "2: 2 clocks"]
    _2 = 2,
    #[doc = "3: 3 clocks"]
    _3 = 3,
    #[doc = "4: 4 clocks"]
    _4 = 4,
    #[doc = "5: 8 clocks"]
    _8 = 5,
    #[doc = "6: 16 clocks"]
    _16 = 6,
    #[doc = "7: 31 clocks"]
    _31 = 7,
}
impl From<GFPW_A> for u8 {
    #[inline(always)]
    fn from(variant: GFPW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GFPW`"]
pub type GFPW_R = crate::R<u8, GFPW_A>;
impl GFPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFPW_A {
        match self.bits {
            0 => GFPW_A::BYPASS,
            1 => GFPW_A::_1,
            2 => GFPW_A::_2,
            3 => GFPW_A::_3,
            4 => GFPW_A::_4,
            5 => GFPW_A::_8,
            6 => GFPW_A::_16,
            7 => GFPW_A::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == GFPW_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GFPW_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == GFPW_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == GFPW_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == GFPW_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == GFPW_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == GFPW_A::_16
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == GFPW_A::_31
    }
}
#[doc = "Write proxy for field `GFPW`"]
pub struct GFPW_W<'a> {
    w: &'a mut W,
}
impl<'a> GFPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFPW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(GFPW_A::BYPASS)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GFPW_A::_1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(GFPW_A::_2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(GFPW_A::_3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(GFPW_A::_4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(GFPW_A::_8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(GFPW_A::_16)
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(GFPW_A::_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline(always)]
    pub fn gfpw(&self) -> GFPW_R {
        GFPW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline(always)]
    pub fn gfpw(&mut self) -> GFPW_W {
        GFPW_W { w: self }
    }
}
