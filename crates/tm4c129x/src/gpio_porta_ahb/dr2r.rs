#[doc = "Reader of register DR2R"]
pub type R = crate::R<u32, super::DR2R>;
#[doc = "Writer for register DR2R"]
pub type W = crate::W<u32, super::DR2R>;
#[doc = "Register DR2R `reset()`'s with value 0"]
impl crate::ResetValue for super::DR2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRV2`"]
pub type DRV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRV2`"]
pub struct DRV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV2_W<'a> {
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
    pub fn drv2(&self) -> DRV2_R {
        DRV2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn drv2(&mut self) -> DRV2_W {
        DRV2_W { w: self }
    }
}
