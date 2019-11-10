#[doc = "Reader of register WAKEPEN"]
pub type R = crate::R<u32, super::WAKEPEN>;
#[doc = "Writer for register WAKEPEN"]
pub type W = crate::W<u32, super::WAKEPEN>;
#[doc = "Register WAKEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKEP4`"]
pub type WAKEP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEP4`"]
pub struct WAKEP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEP4_W<'a> {
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
    #[doc = "Bit 4 - P\\[4\\] Wake Enable"]
    #[inline(always)]
    pub fn wakep4(&self) -> WAKEP4_R {
        WAKEP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - P\\[4\\] Wake Enable"]
    #[inline(always)]
    pub fn wakep4(&mut self) -> WAKEP4_W {
        WAKEP4_W { w: self }
    }
}
