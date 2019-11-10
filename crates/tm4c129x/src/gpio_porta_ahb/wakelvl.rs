#[doc = "Reader of register WAKELVL"]
pub type R = crate::R<u32, super::WAKELVL>;
#[doc = "Writer for register WAKELVL"]
pub type W = crate::W<u32, super::WAKELVL>;
#[doc = "Register WAKELVL `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKELVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKELVL4`"]
pub type WAKELVL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKELVL4`"]
pub struct WAKELVL4_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKELVL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - P\\[4\\] Wake Level"]
    #[inline(always)]
    pub fn wakelvl4(&self) -> WAKELVL4_R {
        WAKELVL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - P\\[4\\] Wake Level"]
    #[inline(always)]
    pub fn wakelvl4(&mut self) -> WAKELVL4_W {
        WAKELVL4_W { w: self }
    }
}
