#[doc = "Reader of register CRCDIN"]
pub type R = crate::R<u32, super::CRCDIN>;
#[doc = "Writer for register CRCDIN"]
pub type W = crate::W<u32, super::CRCDIN>;
#[doc = "Register CRCDIN `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCDIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAIN`"]
pub type DATAIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATAIN`"]
pub struct DATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Input"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Input"]
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W {
        DATAIN_W { w: self }
    }
}
