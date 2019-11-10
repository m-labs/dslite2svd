#[doc = "Reader of register AMSEL"]
pub type R = crate::R<u32, super::AMSEL>;
#[doc = "Writer for register AMSEL"]
pub type W = crate::W<u32, super::AMSEL>;
#[doc = "Register AMSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::AMSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMSEL`"]
pub type AMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMSEL`"]
pub struct AMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AMSEL_W<'a> {
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
    pub fn amsel(&self) -> AMSEL_R {
        AMSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn amsel(&mut self) -> AMSEL_W {
        AMSEL_W { w: self }
    }
}
