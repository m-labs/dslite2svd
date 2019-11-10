#[doc = "Reader of register SRCCM"]
pub type R = crate::R<u32, super::SRCCM>;
#[doc = "Writer for register SRCCM"]
pub type W = crate::W<u32, super::SRCCM>;
#[doc = "Register SRCCM `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCCM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R0`"]
pub struct R0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0_W<'a> {
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
    #[doc = "Bit 0 - CRC and Cryptographic Modules Software Reset"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Software Reset"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
}
