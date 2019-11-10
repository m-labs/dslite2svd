#[doc = "Reader of register TXMAXP3"]
pub type R = crate::R<u16, super::TXMAXP3>;
#[doc = "Writer for register TXMAXP3"]
pub type W = crate::W<u16, super::TXMAXP3>;
#[doc = "Register TXMAXP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TXMAXP3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXLOAD`"]
pub type MAXLOAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXLOAD`"]
pub struct MAXLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u16) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn maxload(&self) -> MAXLOAD_R {
        MAXLOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn maxload(&mut self) -> MAXLOAD_W {
        MAXLOAD_W { w: self }
    }
}
