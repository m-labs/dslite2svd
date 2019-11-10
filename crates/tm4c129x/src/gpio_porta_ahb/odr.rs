#[doc = "Reader of register ODR"]
pub type R = crate::R<u32, super::ODR>;
#[doc = "Writer for register ODR"]
pub type W = crate::W<u32, super::ODR>;
#[doc = "Register ODR `reset()`'s with value 0"]
impl crate::ResetValue for super::ODR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ODE`"]
pub type ODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODE`"]
pub struct ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ode(&self) -> ODE_R {
        ODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ode(&mut self) -> ODE_W {
        ODE_W { w: self }
    }
}
