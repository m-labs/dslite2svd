#[doc = "Reader of register SUBSECINC"]
pub type R = crate::R<u32, super::SUBSECINC>;
#[doc = "Writer for register SUBSECINC"]
pub type W = crate::W<u32, super::SUBSECINC>;
#[doc = "Register SUBSECINC `reset()`'s with value 0"]
impl crate::ResetValue for super::SUBSECINC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSINC`"]
pub type SSINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSINC`"]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
}
