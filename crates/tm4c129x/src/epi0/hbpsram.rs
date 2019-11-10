#[doc = "Reader of register HBPSRAM"]
pub type R = crate::R<u32, super::HBPSRAM>;
#[doc = "Writer for register HBPSRAM"]
pub type W = crate::W<u32, super::HBPSRAM>;
#[doc = "Register HBPSRAM `reset()`'s with value 0"]
impl crate::ResetValue for super::HBPSRAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - PSRAM Config Register"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - PSRAM Config Register"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
}
