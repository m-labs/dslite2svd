#[doc = "Reader of register EEPASS2"]
pub type R = crate::R<u32, super::EEPASS2>;
#[doc = "Writer for register EEPASS2"]
pub type W = crate::W<u32, super::EEPASS2>;
#[doc = "Register EEPASS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EEPASS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PASS`"]
pub type PASS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PASS`"]
pub struct PASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    pub fn pass(&self) -> PASS_R {
        PASS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    pub fn pass(&mut self) -> PASS_W {
        PASS_W { w: self }
    }
}
