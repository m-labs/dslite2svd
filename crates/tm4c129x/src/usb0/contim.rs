#[doc = "Reader of register CONTIM"]
pub type R = crate::R<u8, super::CONTIM>;
#[doc = "Writer for register CONTIM"]
pub type W = crate::W<u8, super::CONTIM>;
#[doc = "Register CONTIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTIM {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTID`"]
pub type WTID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTID`"]
pub struct WTID_W<'a> {
    w: &'a mut W,
}
impl<'a> WTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WTCON`"]
pub type WTCON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTCON`"]
pub struct WTCON_W<'a> {
    w: &'a mut W,
}
impl<'a> WTCON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    pub fn wtid(&self) -> WTID_R {
        WTID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    pub fn wtcon(&self) -> WTCON_R {
        WTCON_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    pub fn wtid(&mut self) -> WTID_W {
        WTID_W { w: self }
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    pub fn wtcon(&mut self) -> WTCON_W {
        WTCON_W { w: self }
    }
}
