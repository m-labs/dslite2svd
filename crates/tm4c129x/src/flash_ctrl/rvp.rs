#[doc = "Reader of register RVP"]
pub type R = crate::R<u32, super::RVP>;
#[doc = "Writer for register RVP"]
pub type W = crate::W<u32, super::RVP>;
#[doc = "Register RVP `reset()`'s with value 0"]
impl crate::ResetValue for super::RVP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RV`"]
pub type RV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RV`"]
pub struct RV_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reset Vector Pointer Address"]
    #[inline(always)]
    pub fn rv(&self) -> RV_R {
        RV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reset Vector Pointer Address"]
    #[inline(always)]
    pub fn rv(&mut self) -> RV_W {
        RV_W { w: self }
    }
}
