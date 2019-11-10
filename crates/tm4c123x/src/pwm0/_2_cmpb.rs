#[doc = "Reader of register _2_CMPB"]
pub type R = crate::R<u32, super::_2_CMPB>;
#[doc = "Writer for register _2_CMPB"]
pub type W = crate::W<u32, super::_2_CMPB>;
#[doc = "Register _2_CMPB `reset()`'s with value 0"]
impl crate::ResetValue for super::_2_CMPB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPB`"]
pub type COMPB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPB`"]
pub struct COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    pub fn compb(&self) -> COMPB_R {
        COMPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    pub fn compb(&mut self) -> COMPB_W {
        COMPB_W { w: self }
    }
}
