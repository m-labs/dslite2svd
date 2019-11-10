#[doc = "Reader of register DR8R"]
pub type R = crate::R<u32, super::DR8R>;
#[doc = "Writer for register DR8R"]
pub type W = crate::W<u32, super::DR8R>;
#[doc = "Register DR8R `reset()`'s with value 0"]
impl crate::ResetValue for super::DR8R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRV8`"]
pub type DRV8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRV8`"]
pub struct DRV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn drv8(&self) -> DRV8_R {
        DRV8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn drv8(&mut self) -> DRV8_W {
        DRV8_W { w: self }
    }
}
