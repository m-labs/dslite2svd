#[doc = "Reader of register HSEOF"]
pub type R = crate::R<u8, super::HSEOF>;
#[doc = "Writer for register HSEOF"]
pub type W = crate::W<u8, super::HSEOF>;
#[doc = "Register HSEOF `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEOF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSEOFG`"]
pub type HSEOFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSEOFG`"]
pub struct HSEOFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEOFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HIgh-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn hseofg(&self) -> HSEOFG_R {
        HSEOFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HIgh-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn hseofg(&mut self) -> HSEOFG_W {
        HSEOFG_W { w: self }
    }
}
