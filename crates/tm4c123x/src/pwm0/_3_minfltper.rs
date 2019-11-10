#[doc = "Reader of register _3_MINFLTPER"]
pub type R = crate::R<u32, super::_3_MINFLTPER>;
#[doc = "Writer for register _3_MINFLTPER"]
pub type W = crate::W<u32, super::_3_MINFLTPER>;
#[doc = "Register _3_MINFLTPER `reset()`'s with value 0"]
impl crate::ResetValue for super::_3_MINFLTPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFP`"]
pub type MFP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFP`"]
pub struct MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn mfp(&self) -> MFP_R {
        MFP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn mfp(&mut self) -> MFP_W {
        MFP_W { w: self }
    }
}
