#[doc = "Reader of register PPS0INTVL"]
pub type R = crate::R<u32, super::PPS0INTVL>;
#[doc = "Writer for register PPS0INTVL"]
pub type W = crate::W<u32, super::PPS0INTVL>;
#[doc = "Register PPS0INTVL `reset()`'s with value 0"]
impl crate::ResetValue for super::PPS0INTVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPS0INT`"]
pub type PPS0INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PPS0INT`"]
pub struct PPS0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS0INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PPS0 Output Signal Interval"]
    #[inline(always)]
    pub fn pps0int(&self) -> PPS0INT_R {
        PPS0INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS0 Output Signal Interval"]
    #[inline(always)]
    pub fn pps0int(&mut self) -> PPS0INT_W {
        PPS0INT_W { w: self }
    }
}
