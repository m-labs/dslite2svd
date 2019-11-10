#[doc = "Reader of register EEHIDE0"]
pub type R = crate::R<u32, super::EEHIDE0>;
#[doc = "Writer for register EEHIDE0"]
pub type W = crate::W<u32, super::EEHIDE0>;
#[doc = "Register EEHIDE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EEHIDE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HN`"]
pub type HN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HN`"]
pub struct HN_W<'a> {
    w: &'a mut W,
}
impl<'a> HN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:31 - Hide Block"]
    #[inline(always)]
    pub fn hn(&self) -> HN_R {
        HN_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:31 - Hide Block"]
    #[inline(always)]
    pub fn hn(&mut self) -> HN_W {
        HN_W { w: self }
    }
}
