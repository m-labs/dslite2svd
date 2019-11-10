#[doc = "Reader of register SLR"]
pub type R = crate::R<u32, super::SLR>;
#[doc = "Writer for register SLR"]
pub type W = crate::W<u32, super::SLR>;
#[doc = "Register SLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLR`"]
pub type SLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLR`"]
pub struct SLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLR_W<'a> {
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
    pub fn slr(&self) -> SLR_R {
        SLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slr(&mut self) -> SLR_W {
        SLR_W { w: self }
    }
}
