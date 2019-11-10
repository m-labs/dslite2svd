#[doc = "Reader of register _0_MINFLTPER"]
pub type R = crate::R<u32, super::_0_MINFLTPER>;
#[doc = "Writer for register _0_MINFLTPER"]
pub type W = crate::W<u32, super::_0_MINFLTPER>;
#[doc = "Register _0_MINFLTPER `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_MINFLTPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MINFLTPER`"]
pub type MINFLTPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MINFLTPER`"]
pub struct MINFLTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MINFLTPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn minfltper(&self) -> MINFLTPER_R {
        MINFLTPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn minfltper(&mut self) -> MINFLTPER_W {
        MINFLTPER_W { w: self }
    }
}
