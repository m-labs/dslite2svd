#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Writer for register PC"]
pub type W = crate::W<u32, super::PC>;
#[doc = "Register PC `reset()`'s with value 0"]
impl crate::ResetValue for super::PC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ULPIEN`"]
pub type ULPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULPIEN`"]
pub struct ULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - ULPI Enable"]
    #[inline(always)]
    pub fn ulpien(&self) -> ULPIEN_R {
        ULPIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ULPI Enable"]
    #[inline(always)]
    pub fn ulpien(&mut self) -> ULPIEN_W {
        ULPIEN_W { w: self }
    }
}
