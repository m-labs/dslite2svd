#[doc = "Reader of register REQMASKSET"]
pub type R = crate::R<u32, super::REQMASKSET>;
#[doc = "Writer for register REQMASKSET"]
pub type W = crate::W<u32, super::REQMASKSET>;
#[doc = "Register REQMASKSET `reset()`'s with value 0"]
impl crate::ResetValue for super::REQMASKSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SET`"]
pub type SET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SET`"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Request Mask Set"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Request Mask Set"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
}
