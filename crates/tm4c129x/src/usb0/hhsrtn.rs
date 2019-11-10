#[doc = "Reader of register HHSRTN"]
pub type R = crate::R<u16, super::HHSRTN>;
#[doc = "Writer for register HHSRTN"]
pub type W = crate::W<u16, super::HHSRTN>;
#[doc = "Register HHSRTN `reset()`'s with value 0"]
impl crate::ResetValue for super::HHSRTN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HHSRTN`"]
pub type HHSRTN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HHSRTN`"]
pub struct HHSRTN_W<'a> {
    w: &'a mut W,
}
impl<'a> HHSRTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - HIgh Speed to UTM Operating Delay"]
    #[inline(always)]
    pub fn hhsrtn(&self) -> HHSRTN_R {
        HHSRTN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HIgh Speed to UTM Operating Delay"]
    #[inline(always)]
    pub fn hhsrtn(&mut self) -> HHSRTN_W {
        HHSRTN_W { w: self }
    }
}
