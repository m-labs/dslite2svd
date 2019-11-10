#[doc = "Reader of register LSEOF"]
pub type R = crate::R<u8, super::LSEOF>;
#[doc = "Writer for register LSEOF"]
pub type W = crate::W<u8, super::LSEOF>;
#[doc = "Register LSEOF `reset()`'s with value 0"]
impl crate::ResetValue for super::LSEOF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSEOFG`"]
pub type LSEOFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSEOFG`"]
pub struct LSEOFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEOFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn lseofg(&self) -> LSEOFG_R {
        LSEOFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn lseofg(&mut self) -> LSEOFG_W {
        LSEOFG_W { w: self }
    }
}
