#[doc = "Reader of register TIMNANOU"]
pub type R = crate::R<u32, super::TIMNANOU>;
#[doc = "Writer for register TIMNANOU"]
pub type W = crate::W<u32, super::TIMNANOU>;
#[doc = "Register TIMNANOU `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMNANOU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSSS`"]
pub type TSSS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSSS`"]
pub struct TSSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `ADDSUB`"]
pub type ADDSUB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDSUB`"]
pub struct ADDSUB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSUB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W {
        TSSS_W { w: self }
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W {
        ADDSUB_W { w: self }
    }
}
