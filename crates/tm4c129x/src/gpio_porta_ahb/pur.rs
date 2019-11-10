#[doc = "Reader of register PUR"]
pub type R = crate::R<u32, super::PUR>;
#[doc = "Writer for register PUR"]
pub type W = crate::W<u32, super::PUR>;
#[doc = "Register PUR `reset()`'s with value 0"]
impl crate::ResetValue for super::PUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PUE`"]
pub type PUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUE`"]
pub struct PUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUE_W<'a> {
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
    pub fn pue(&self) -> PUE_R {
        PUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pue(&mut self) -> PUE_W {
        PUE_W { w: self }
    }
}
