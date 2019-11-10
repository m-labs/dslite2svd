#[doc = "Reader of register CHMAP0"]
pub type R = crate::R<u32, super::CHMAP0>;
#[doc = "Writer for register CHMAP0"]
pub type W = crate::W<u32, super::CHMAP0>;
#[doc = "Register CHMAP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHMAP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0SEL`"]
pub type CH0SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0SEL`"]
pub struct CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH1SEL`"]
pub type CH1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1SEL`"]
pub struct CH1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH2SEL`"]
pub type CH2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2SEL`"]
pub struct CH2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH3SEL`"]
pub type CH3SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3SEL`"]
pub struct CH3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH4SEL`"]
pub type CH4SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH4SEL`"]
pub struct CH4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH5SEL`"]
pub type CH5SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH5SEL`"]
pub struct CH5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CH6SEL`"]
pub type CH6SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH6SEL`"]
pub struct CH6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH7SEL`"]
pub type CH7SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH7SEL`"]
pub struct CH7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> CH0SEL_W {
        CH0SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> CH1SEL_W {
        CH1SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> CH2SEL_W {
        CH2SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> CH3SEL_W {
        CH3SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> CH4SEL_W {
        CH4SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> CH5SEL_W {
        CH5SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> CH6SEL_W {
        CH6SEL_W { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn ch7sel(&mut self) -> CH7SEL_W {
        CH7SEL_W { w: self }
    }
}
