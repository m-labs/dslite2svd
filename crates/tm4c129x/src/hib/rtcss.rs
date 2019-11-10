#[doc = "Reader of register RTCSS"]
pub type R = crate::R<u32, super::RTCSS>;
#[doc = "Writer for register RTCSS"]
pub type W = crate::W<u32, super::RTCSS>;
#[doc = "Register RTCSS `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCSSC`"]
pub type RTCSSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTCSSC`"]
pub struct RTCSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `RTCSSM`"]
pub type RTCSSM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTCSSM`"]
pub struct RTCSSM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn rtcssc(&self) -> RTCSSC_R {
        RTCSSC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn rtcssm(&self) -> RTCSSM_R {
        RTCSSM_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn rtcssc(&mut self) -> RTCSSC_W {
        RTCSSC_W { w: self }
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn rtcssm(&mut self) -> RTCSSM_W {
        RTCSSM_W { w: self }
    }
}
