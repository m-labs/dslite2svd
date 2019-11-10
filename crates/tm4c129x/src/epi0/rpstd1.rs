#[doc = "Reader of register RPSTD1"]
pub type R = crate::R<u32, super::RPSTD1>;
#[doc = "Writer for register RPSTD1"]
pub type W = crate::W<u32, super::RPSTD1>;
#[doc = "Register RPSTD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RPSTD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POSTCNT`"]
pub type POSTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POSTCNT`"]
pub struct POSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn postcnt(&self) -> POSTCNT_R {
        POSTCNT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn postcnt(&mut self) -> POSTCNT_W {
        POSTCNT_W { w: self }
    }
}
