#[doc = "Reader of register VPLEN"]
pub type R = crate::R<u8, super::VPLEN>;
#[doc = "Writer for register VPLEN"]
pub type W = crate::W<u8, super::VPLEN>;
#[doc = "Register VPLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::VPLEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VPLEN`"]
pub type VPLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VPLEN`"]
pub struct VPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VPLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    pub fn vplen(&self) -> VPLEN_R {
        VPLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    pub fn vplen(&mut self) -> VPLEN_W {
        VPLEN_W { w: self }
    }
}
