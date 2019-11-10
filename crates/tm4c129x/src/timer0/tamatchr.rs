#[doc = "Reader of register TAMATCHR"]
pub type R = crate::R<u32, super::TAMATCHR>;
#[doc = "Writer for register TAMATCHR"]
pub type W = crate::W<u32, super::TAMATCHR>;
#[doc = "Register TAMATCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMATCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAMR`"]
pub type TAMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAMR`"]
pub struct TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A Match Register"]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A Match Register"]
    #[inline(always)]
    pub fn tamr(&mut self) -> TAMR_W {
        TAMR_W { w: self }
    }
}
