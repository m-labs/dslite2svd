#[doc = "Reader of register CTO"]
pub type R = crate::R<u16, super::CTO>;
#[doc = "Writer for register CTO"]
pub type W = crate::W<u16, super::CTO>;
#[doc = "Register CTO `reset()`'s with value 0"]
impl crate::ResetValue for super::CTO {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCTV`"]
pub type CCTV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCTV`"]
pub struct CCTV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout Value"]
    #[inline(always)]
    pub fn cctv(&self) -> CCTV_R {
        CCTV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout Value"]
    #[inline(always)]
    pub fn cctv(&mut self) -> CCTV_W {
        CCTV_W { w: self }
    }
}
