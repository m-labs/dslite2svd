#[doc = "Reader of register MBLEN"]
pub type R = crate::R<u32, super::MBLEN>;
#[doc = "Writer for register MBLEN"]
pub type W = crate::W<u32, super::MBLEN>;
#[doc = "Register MBLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MBLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTL`"]
pub type CNTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTL`"]
pub struct CNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Burst Length"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Burst Length"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W {
        CNTL_W { w: self }
    }
}
