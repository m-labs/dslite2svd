#[doc = "Writer for register RXPOLLD"]
pub type W = crate::W<u32, super::RXPOLLD>;
#[doc = "Register RXPOLLD `reset()`'s with value 0"]
impl crate::ResetValue for super::RXPOLLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RPD`"]
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
}
