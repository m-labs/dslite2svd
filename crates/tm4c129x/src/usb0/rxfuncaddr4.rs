#[doc = "Reader of register RXFUNCADDR4"]
pub type R = crate::R<u8, super::RXFUNCADDR4>;
#[doc = "Writer for register RXFUNCADDR4"]
pub type W = crate::W<u8, super::RXFUNCADDR4>;
#[doc = "Register RXFUNCADDR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFUNCADDR4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
