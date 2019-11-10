#[doc = "Reader of register AFSEL"]
pub type R = crate::R<u32, super::AFSEL>;
#[doc = "Writer for register AFSEL"]
pub type W = crate::W<u32, super::AFSEL>;
#[doc = "Register AFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::AFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AFSEL`"]
pub type AFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFSEL`"]
pub struct AFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL_W<'a> {
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
    pub fn afsel(&self) -> AFSEL_R {
        AFSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn afsel(&mut self) -> AFSEL_W {
        AFSEL_W { w: self }
    }
}
