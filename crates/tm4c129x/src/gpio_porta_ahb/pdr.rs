#[doc = "Reader of register PDR"]
pub type R = crate::R<u32, super::PDR>;
#[doc = "Writer for register PDR"]
pub type W = crate::W<u32, super::PDR>;
#[doc = "Register PDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
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
    pub fn pde(&self) -> PDE_R {
        PDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
}
