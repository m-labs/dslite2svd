#[doc = "Reader of register DR12R"]
pub type R = crate::R<u32, super::DR12R>;
#[doc = "Writer for register DR12R"]
pub type W = crate::W<u32, super::DR12R>;
#[doc = "Register DR12R `reset()`'s with value 0"]
impl crate::ResetValue for super::DR12R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRV12`"]
pub type DRV12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRV12`"]
pub struct DRV12_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV12_W<'a> {
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
    pub fn drv12(&self) -> DRV12_R {
        DRV12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn drv12(&mut self) -> DRV12_W {
        DRV12_W { w: self }
    }
}
