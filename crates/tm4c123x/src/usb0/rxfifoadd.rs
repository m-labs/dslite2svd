#[doc = "Reader of register RXFIFOADD"]
pub type R = crate::R<u16, super::RXFIFOADD>;
#[doc = "Writer for register RXFIFOADD"]
pub type W = crate::W<u16, super::RXFIFOADD>;
#[doc = "Register RXFIFOADD `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFIFOADD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u16) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
