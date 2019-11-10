#[doc = "Reader of register NAKLMT"]
pub type R = crate::R<u8, super::NAKLMT>;
#[doc = "Writer for register NAKLMT"]
pub type W = crate::W<u8, super::NAKLMT>;
#[doc = "Register NAKLMT `reset()`'s with value 0"]
impl crate::ResetValue for super::NAKLMT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NAKLMT`"]
pub type NAKLMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NAKLMT`"]
pub struct NAKLMT_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKLMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    pub fn naklmt(&self) -> NAKLMT_R {
        NAKLMT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    pub fn naklmt(&mut self) -> NAKLMT_W {
        NAKLMT_W { w: self }
    }
}
