#[doc = "Reader of register SSEMUX3"]
pub type R = crate::R<u32, super::SSEMUX3>;
#[doc = "Writer for register SSEMUX3"]
pub type W = crate::W<u32, super::SSEMUX3>;
#[doc = "Register SSEMUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSEMUX3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMUX0`"]
pub type EMUX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX0`"]
pub struct EMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX0_W<'a> {
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
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux0(&self) -> EMUX0_R {
        EMUX0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux0(&mut self) -> EMUX0_W {
        EMUX0_W { w: self }
    }
}
