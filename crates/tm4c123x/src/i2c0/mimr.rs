#[doc = "Reader of register MIMR"]
pub type R = crate::R<u32, super::MIMR>;
#[doc = "Writer for register MIMR"]
pub type W = crate::W<u32, super::MIMR>;
#[doc = "Register MIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
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
#[doc = "Reader of field `CLKIM`"]
pub type CLKIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKIM`"]
pub struct CLKIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIM_W<'a> {
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
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn clkim(&self) -> CLKIM_R {
        CLKIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn clkim(&mut self) -> CLKIM_W {
        CLKIM_W { w: self }
    }
}
