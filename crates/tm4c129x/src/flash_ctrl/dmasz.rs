#[doc = "Reader of register DMASZ"]
pub type R = crate::R<u32, super::DMASZ>;
#[doc = "Writer for register DMASZ"]
pub type W = crate::W<u32, super::DMASZ>;
#[doc = "Register DMASZ `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - uDMA-accessible Memory Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - uDMA-accessible Memory Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}
