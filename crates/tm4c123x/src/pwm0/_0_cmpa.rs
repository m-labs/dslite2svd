#[doc = "Reader of register _0_CMPA"]
pub type R = crate::R<u32, super::_0_CMPA>;
#[doc = "Writer for register _0_CMPA"]
pub type W = crate::W<u32, super::_0_CMPA>;
#[doc = "Register _0_CMPA `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_CMPA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPA`"]
pub type CMPA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPA`"]
pub struct CMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn cmpa(&mut self) -> CMPA_W {
        CMPA_W { w: self }
    }
}
