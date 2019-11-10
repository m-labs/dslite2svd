#[doc = "Reader of register CC"]
pub type R = crate::R<u32, super::CC>;
#[doc = "Writer for register CC"]
pub type W = crate::W<u32, super::CC>;
#[doc = "Register CC `reset()`'s with value 0"]
impl crate::ResetValue for super::CC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCLKEN`"]
pub type SYSCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCLKEN`"]
pub struct SYSCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKEN_W<'a> {
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
    #[doc = "Bit 0 - RTCOSC to System Clock Enable"]
    #[inline(always)]
    pub fn sysclken(&self) -> SYSCLKEN_R {
        SYSCLKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCOSC to System Clock Enable"]
    #[inline(always)]
    pub fn sysclken(&mut self) -> SYSCLKEN_W {
        SYSCLKEN_W { w: self }
    }
}
