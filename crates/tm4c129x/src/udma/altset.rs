#[doc = "Reader of register ALTSET"]
pub type R = crate::R<u32, super::ALTSET>;
#[doc = "Writer for register ALTSET"]
pub type W = crate::W<u32, super::ALTSET>;
#[doc = "Register ALTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTSET {
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
Alternate Set"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Alternate Set"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
}
