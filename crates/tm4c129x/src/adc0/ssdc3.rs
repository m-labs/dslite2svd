#[doc = "Reader of register SSDC3"]
pub type R = crate::R<u32, super::SSDC3>;
#[doc = "Writer for register SSDC3"]
pub type W = crate::W<u32, super::SSDC3>;
#[doc = "Register SSDC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSDC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0DCSEL`"]
pub type S0DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0DCSEL`"]
pub struct S0DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn s0dcsel(&self) -> S0DCSEL_R {
        S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn s0dcsel(&mut self) -> S0DCSEL_W {
        S0DCSEL_W { w: self }
    }
}
