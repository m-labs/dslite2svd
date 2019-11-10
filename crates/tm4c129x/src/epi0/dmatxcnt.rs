#[doc = "Reader of register DMATXCNT"]
pub type R = crate::R<u32, super::DMATXCNT>;
#[doc = "Writer for register DMATXCNT"]
pub type W = crate::W<u32, super::DMATXCNT>;
#[doc = "Register DMATXCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXCNT`"]
pub type TXCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXCNT`"]
pub struct TXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA Count"]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Count"]
    #[inline(always)]
    pub fn txcnt(&mut self) -> TXCNT_W {
        TXCNT_W { w: self }
    }
}
