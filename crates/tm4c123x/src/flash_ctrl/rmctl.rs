#[doc = "Reader of register RMCTL"]
pub type R = crate::R<u32, super::RMCTL>;
#[doc = "Writer for register RMCTL"]
pub type W = crate::W<u32, super::RMCTL>;
#[doc = "Register RMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BA`"]
pub type BA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BA`"]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
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
    #[doc = "Bit 0 - Boot Alias"]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Alias"]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
}
