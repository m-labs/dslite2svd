#[doc = "Reader of register IF2MSK1"]
pub type R = crate::R<u32, super::IF2MSK1>;
#[doc = "Writer for register IF2MSK1"]
pub type W = crate::W<u32, super::IF2MSK1>;
#[doc = "Register IF2MSK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IF2MSK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMSK`"]
pub type IDMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDMSK`"]
pub struct IDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Identifier Mask"]
    #[inline(always)]
    pub fn idmsk(&self) -> IDMSK_R {
        IDMSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Identifier Mask"]
    #[inline(always)]
    pub fn idmsk(&mut self) -> IDMSK_W {
        IDMSK_W { w: self }
    }
}
