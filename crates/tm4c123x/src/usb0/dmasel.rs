#[doc = "Reader of register DMASEL"]
pub type R = crate::R<u32, super::DMASEL>;
#[doc = "Writer for register DMASEL"]
pub type W = crate::W<u32, super::DMASEL>;
#[doc = "Register DMASEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAARX`"]
pub type DMAARX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMAARX`"]
pub struct DMAARX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAARX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DMAATX`"]
pub type DMAATX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMAATX`"]
pub struct DMAATX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAATX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMABRX`"]
pub type DMABRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMABRX`"]
pub struct DMABRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DMABTX`"]
pub type DMABTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMABTX`"]
pub struct DMABTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMACRX`"]
pub type DMACRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMACRX`"]
pub struct DMACRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMACTX`"]
pub type DMACTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMACTX`"]
pub struct DMACTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline(always)]
    pub fn dmaarx(&self) -> DMAARX_R {
        DMAARX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline(always)]
    pub fn dmaatx(&self) -> DMAATX_R {
        DMAATX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline(always)]
    pub fn dmabrx(&self) -> DMABRX_R {
        DMABRX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline(always)]
    pub fn dmabtx(&self) -> DMABTX_R {
        DMABTX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline(always)]
    pub fn dmacrx(&self) -> DMACRX_R {
        DMACRX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline(always)]
    pub fn dmactx(&self) -> DMACTX_R {
        DMACTX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline(always)]
    pub fn dmaarx(&mut self) -> DMAARX_W {
        DMAARX_W { w: self }
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline(always)]
    pub fn dmaatx(&mut self) -> DMAATX_W {
        DMAATX_W { w: self }
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline(always)]
    pub fn dmabrx(&mut self) -> DMABRX_W {
        DMABRX_W { w: self }
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline(always)]
    pub fn dmabtx(&mut self) -> DMABTX_W {
        DMABTX_W { w: self }
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline(always)]
    pub fn dmacrx(&mut self) -> DMACRX_W {
        DMACRX_W { w: self }
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline(always)]
    pub fn dmactx(&mut self) -> DMACTX_W {
        DMACTX_W { w: self }
    }
}
