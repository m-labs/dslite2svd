#[doc = "Reader of register EEDBGME"]
pub type R = crate::R<u32, super::EEDBGME>;
#[doc = "Writer for register EEDBGME"]
pub type W = crate::W<u32, super::EEDBGME>;
#[doc = "Register EEDBGME `reset()`'s with value 0"]
impl crate::ResetValue for super::EEDBGME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ME`"]
pub type ME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ME`"]
pub struct ME_W<'a> {
    w: &'a mut W,
}
impl<'a> ME_W<'a> {
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
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn me(&self) -> ME_R {
        ME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn me(&mut self) -> ME_W {
        ME_W { w: self }
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
