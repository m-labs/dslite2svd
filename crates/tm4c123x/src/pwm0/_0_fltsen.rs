#[doc = "Reader of register _0_FLTSEN"]
pub type R = crate::R<u32, super::_0_FLTSEN>;
#[doc = "Writer for register _0_FLTSEN"]
pub type W = crate::W<u32, super::_0_FLTSEN>;
#[doc = "Register _0_FLTSEN `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_FLTSEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAULT0`"]
pub type FAULT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT0`"]
pub struct FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_W<'a> {
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
#[doc = "Reader of field `FAULT1`"]
pub type FAULT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT1`"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
}
