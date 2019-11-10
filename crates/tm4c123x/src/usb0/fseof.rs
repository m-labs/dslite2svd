#[doc = "Reader of register FSEOF"]
pub type R = crate::R<u8, super::FSEOF>;
#[doc = "Writer for register FSEOF"]
pub type W = crate::W<u8, super::FSEOF>;
#[doc = "Register FSEOF `reset()`'s with value 0"]
impl crate::ResetValue for super::FSEOF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSEOFG`"]
pub type FSEOFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSEOFG`"]
pub struct FSEOFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEOFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Full-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn fseofg(&self) -> FSEOFG_R {
        FSEOFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Full-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn fseofg(&mut self) -> FSEOFG_W {
        FSEOFG_W { w: self }
    }
}
