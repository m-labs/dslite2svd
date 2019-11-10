#[doc = "Reader of register HSSR"]
pub type R = crate::R<u32, super::HSSR>;
#[doc = "Writer for register HSSR"]
pub type W = crate::W<u32, super::HSSR>;
#[doc = "Register HSSR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CDOFF`"]
pub type CDOFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CDOFF`"]
pub struct CDOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn cdoff(&self) -> CDOFF_R {
        CDOFF_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn cdoff(&mut self) -> CDOFF_W {
        CDOFF_W { w: self }
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
