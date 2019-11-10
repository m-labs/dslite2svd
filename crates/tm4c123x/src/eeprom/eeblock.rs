#[doc = "Reader of register EEBLOCK"]
pub type R = crate::R<u32, super::EEBLOCK>;
#[doc = "Writer for register EEBLOCK"]
pub type W = crate::W<u32, super::EEBLOCK>;
#[doc = "Register EEBLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::EEBLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCK`"]
pub type BLOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCK`"]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
}
