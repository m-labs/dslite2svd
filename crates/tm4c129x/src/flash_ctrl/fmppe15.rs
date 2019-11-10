#[doc = "Reader of register FMPPE15"]
pub type R = crate::R<u32, super::FMPPE15>;
#[doc = "Writer for register FMPPE15"]
pub type W = crate::W<u32, super::FMPPE15>;
#[doc = "Register FMPPE15 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMPPE15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROG_ENABLE`"]
pub type PROG_ENABLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PROG_ENABLE`"]
pub struct PROG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn prog_enable(&self) -> PROG_ENABLE_R {
        PROG_ENABLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn prog_enable(&mut self) -> PROG_ENABLE_W {
        PROG_ENABLE_W { w: self }
    }
}
