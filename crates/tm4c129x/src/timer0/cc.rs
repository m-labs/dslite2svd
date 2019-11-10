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
#[doc = "Reader of field `ALTCLK`"]
pub type ALTCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTCLK`"]
pub struct ALTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTCLK_W<'a> {
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
    #[doc = "Bit 0 - Alternate Clock Source"]
    #[inline(always)]
    pub fn altclk(&self) -> ALTCLK_R {
        ALTCLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate Clock Source"]
    #[inline(always)]
    pub fn altclk(&mut self) -> ALTCLK_W {
        ALTCLK_W { w: self }
    }
}
