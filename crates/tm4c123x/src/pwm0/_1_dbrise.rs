#[doc = "Reader of register _1_DBRISE"]
pub type R = crate::R<u32, super::_1_DBRISE>;
#[doc = "Writer for register _1_DBRISE"]
pub type W = crate::W<u32, super::_1_DBRISE>;
#[doc = "Register _1_DBRISE `reset()`'s with value 0"]
impl crate::ResetValue for super::_1_DBRISE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RISEDELAY`"]
pub type RISEDELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RISEDELAY`"]
pub struct RISEDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RISEDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    pub fn risedelay(&self) -> RISEDELAY_R {
        RISEDELAY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    pub fn risedelay(&mut self) -> RISEDELAY_W {
        RISEDELAY_W { w: self }
    }
}
