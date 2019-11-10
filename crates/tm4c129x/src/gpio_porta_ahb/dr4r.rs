#[doc = "Reader of register DR4R"]
pub type R = crate::R<u32, super::DR4R>;
#[doc = "Writer for register DR4R"]
pub type W = crate::W<u32, super::DR4R>;
#[doc = "Register DR4R `reset()`'s with value 0"]
impl crate::ResetValue for super::DR4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRV4`"]
pub type DRV4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRV4`"]
pub struct DRV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV4_W<'a> {
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
    pub fn drv4(&self) -> DRV4_R {
        DRV4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn drv4(&mut self) -> DRV4_W {
        DRV4_W { w: self }
    }
}
