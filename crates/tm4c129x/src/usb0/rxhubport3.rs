#[doc = "Reader of register RXHUBPORT3"]
pub type R = crate::R<u8, super::RXHUBPORT3>;
#[doc = "Writer for register RXHUBPORT3"]
pub type W = crate::W<u8, super::RXHUBPORT3>;
#[doc = "Register RXHUBPORT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RXHUBPORT3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT`"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
}
