#[doc = "Reader of register TARGSEC"]
pub type R = crate::R<u32, super::TARGSEC>;
#[doc = "Writer for register TARGSEC"]
pub type W = crate::W<u32, super::TARGSEC>;
#[doc = "Register TARGSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::TARGSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSTR`"]
pub type TSTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSTR`"]
pub struct TSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstr(&mut self) -> TSTR_W {
        TSTR_W { w: self }
    }
}
