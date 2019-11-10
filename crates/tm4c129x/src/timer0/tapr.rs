#[doc = "Reader of register TAPR"]
pub type R = crate::R<u32, super::TAPR>;
#[doc = "Writer for register TAPR"]
pub type W = crate::W<u32, super::TAPR>;
#[doc = "Register TAPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAPSR`"]
pub type TAPSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAPSR`"]
pub struct TAPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A Prescale"]
    #[inline(always)]
    pub fn tapsr(&self) -> TAPSR_R {
        TAPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A Prescale"]
    #[inline(always)]
    pub fn tapsr(&mut self) -> TAPSR_W {
        TAPSR_W { w: self }
    }
}
