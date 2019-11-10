#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIBLOCK`"]
pub type HIBLOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIBLOCK`"]
pub struct HIBLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HIbernate Lock"]
    #[inline(always)]
    pub fn hiblock(&self) -> HIBLOCK_R {
        HIBLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HIbernate Lock"]
    #[inline(always)]
    pub fn hiblock(&mut self) -> HIBLOCK_W {
        HIBLOCK_W { w: self }
    }
}
