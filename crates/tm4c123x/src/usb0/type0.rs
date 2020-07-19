#[doc = "Reader of register TYPE0"]
pub type R = crate::R<u8, super::TYPE0>;
#[doc = "Writer for register TYPE0"]
pub type W = crate::W<u8, super::TYPE0>;
#[doc = "Register TYPE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TYPE0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "2: Full"]
    FULL = 2,
    #[doc = "3: Low"]
    LOW = 3,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SPEED_A::FULL),
            3 => Val(SPEED_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SPEED_A::FULL
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEED_A::LOW
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(SPEED_A::FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEED_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
}
