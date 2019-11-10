#[doc = "Reader of register EPIDX"]
pub type R = crate::R<u8, super::EPIDX>;
#[doc = "Writer for register EPIDX"]
pub type W = crate::W<u8, super::EPIDX>;
#[doc = "Register EPIDX `reset()`'s with value 0"]
impl crate::ResetValue for super::EPIDX {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPIDX`"]
pub type EPIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPIDX`"]
pub struct EPIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint Index"]
    #[inline(always)]
    pub fn epidx(&self) -> EPIDX_R {
        EPIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Index"]
    #[inline(always)]
    pub fn epidx(&mut self) -> EPIDX_W {
        EPIDX_W { w: self }
    }
}
