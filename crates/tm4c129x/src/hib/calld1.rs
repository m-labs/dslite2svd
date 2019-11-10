#[doc = "Writer for register CALLD1"]
pub type W = crate::W<u32, super::CALLD1>;
#[doc = "Register CALLD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CALLD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DOM`"]
pub struct DOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Write proxy for field `MON`"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `YEAR`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `DOW`"]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn dom(&mut self) -> DOM_W {
        DOM_W { w: self }
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
}
