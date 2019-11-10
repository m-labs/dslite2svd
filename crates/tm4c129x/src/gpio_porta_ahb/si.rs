#[doc = "Reader of register SI"]
pub type R = crate::R<u32, super::SI>;
#[doc = "Writer for register SI"]
pub type W = crate::W<u32, super::SI>;
#[doc = "Register SI `reset()`'s with value 0"]
impl crate::ResetValue for super::SI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUM`"]
pub type SUM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUM`"]
pub struct SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SUM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Summary Interrupt"]
    #[inline(always)]
    pub fn sum(&self) -> SUM_R {
        SUM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Summary Interrupt"]
    #[inline(always)]
    pub fn sum(&mut self) -> SUM_W {
        SUM_W { w: self }
    }
}
