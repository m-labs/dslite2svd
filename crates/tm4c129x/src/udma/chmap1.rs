#[doc = "Reader of register CHMAP1"]
pub type R = crate::R<u32, super::CHMAP1>;
#[doc = "Writer for register CHMAP1"]
pub type W = crate::W<u32, super::CHMAP1>;
#[doc = "Register CHMAP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHMAP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH8SEL`"]
pub type CH8SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH8SEL`"]
pub struct CH8SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH9SEL`"]
pub type CH9SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH9SEL`"]
pub struct CH9SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH10SEL`"]
pub type CH10SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH10SEL`"]
pub struct CH10SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH11SEL`"]
pub type CH11SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH11SEL`"]
pub struct CH11SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH12SEL`"]
pub type CH12SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH12SEL`"]
pub struct CH12SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH13SEL`"]
pub type CH13SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH13SEL`"]
pub struct CH13SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CH14SEL`"]
pub type CH14SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH14SEL`"]
pub struct CH14SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH15SEL`"]
pub type CH15SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH15SEL`"]
pub struct CH15SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn ch8sel(&self) -> CH8SEL_R {
        CH8SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn ch9sel(&self) -> CH9SEL_R {
        CH9SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn ch10sel(&self) -> CH10SEL_R {
        CH10SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn ch11sel(&self) -> CH11SEL_R {
        CH11SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn ch12sel(&self) -> CH12SEL_R {
        CH12SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn ch13sel(&self) -> CH13SEL_R {
        CH13SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn ch14sel(&self) -> CH14SEL_R {
        CH14SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn ch15sel(&self) -> CH15SEL_R {
        CH15SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn ch8sel(&mut self) -> CH8SEL_W {
        CH8SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn ch9sel(&mut self) -> CH9SEL_W {
        CH9SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn ch10sel(&mut self) -> CH10SEL_W {
        CH10SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn ch11sel(&mut self) -> CH11SEL_W {
        CH11SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn ch12sel(&mut self) -> CH12SEL_W {
        CH12SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn ch13sel(&mut self) -> CH13SEL_W {
        CH13SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn ch14sel(&mut self) -> CH14SEL_W {
        CH14SEL_W { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn ch15sel(&mut self) -> CH15SEL_W {
        CH15SEL_W { w: self }
    }
}
