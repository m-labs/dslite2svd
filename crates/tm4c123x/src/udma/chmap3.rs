#[doc = "Reader of register CHMAP3"]
pub type R = crate::R<u32, super::CHMAP3>;
#[doc = "Writer for register CHMAP3"]
pub type W = crate::W<u32, super::CHMAP3>;
#[doc = "Register CHMAP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHMAP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH24SEL`"]
pub type CH24SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH24SEL`"]
pub struct CH24SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH25SEL`"]
pub type CH25SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH25SEL`"]
pub struct CH25SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH26SEL`"]
pub type CH26SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH26SEL`"]
pub struct CH26SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH26SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH27SEL`"]
pub type CH27SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH27SEL`"]
pub struct CH27SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH28SEL`"]
pub type CH28SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH28SEL`"]
pub struct CH28SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH29SEL`"]
pub type CH29SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH29SEL`"]
pub struct CH29SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CH30SEL`"]
pub type CH30SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH30SEL`"]
pub struct CH30SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH31SEL`"]
pub type CH31SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH31SEL`"]
pub struct CH31SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn ch24sel(&self) -> CH24SEL_R {
        CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn ch25sel(&self) -> CH25SEL_R {
        CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn ch26sel(&self) -> CH26SEL_R {
        CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn ch27sel(&self) -> CH27SEL_R {
        CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn ch28sel(&self) -> CH28SEL_R {
        CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn ch29sel(&self) -> CH29SEL_R {
        CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn ch30sel(&self) -> CH30SEL_R {
        CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn ch31sel(&self) -> CH31SEL_R {
        CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn ch24sel(&mut self) -> CH24SEL_W {
        CH24SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn ch25sel(&mut self) -> CH25SEL_W {
        CH25SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn ch26sel(&mut self) -> CH26SEL_W {
        CH26SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn ch27sel(&mut self) -> CH27SEL_W {
        CH27SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn ch28sel(&mut self) -> CH28SEL_W {
        CH28SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn ch29sel(&mut self) -> CH29SEL_W {
        CH29SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn ch30sel(&mut self) -> CH30SEL_W {
        CH30SEL_W { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn ch31sel(&mut self) -> CH31SEL_W {
        CH31SEL_W { w: self }
    }
}
