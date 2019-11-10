#[doc = "Reader of register CHMAP2"]
pub type R = crate::R<u32, super::CHMAP2>;
#[doc = "Writer for register CHMAP2"]
pub type W = crate::W<u32, super::CHMAP2>;
#[doc = "Register CHMAP2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHMAP2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH16SEL`"]
pub type CH16SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH16SEL`"]
pub struct CH16SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH17SEL`"]
pub type CH17SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH17SEL`"]
pub struct CH17SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH18SEL`"]
pub type CH18SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH18SEL`"]
pub struct CH18SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH19SEL`"]
pub type CH19SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH19SEL`"]
pub struct CH19SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH20SEL`"]
pub type CH20SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH20SEL`"]
pub struct CH20SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH21SEL`"]
pub type CH21SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH21SEL`"]
pub struct CH21SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CH22SEL`"]
pub type CH22SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH22SEL`"]
pub struct CH22SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH23SEL`"]
pub type CH23SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH23SEL`"]
pub struct CH23SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn ch16sel(&self) -> CH16SEL_R {
        CH16SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn ch19sel(&self) -> CH19SEL_R {
        CH19SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn ch20sel(&self) -> CH20SEL_R {
        CH20SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn ch21sel(&self) -> CH21SEL_R {
        CH21SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn ch22sel(&self) -> CH22SEL_R {
        CH22SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn ch23sel(&self) -> CH23SEL_R {
        CH23SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn ch16sel(&mut self) -> CH16SEL_W {
        CH16SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn ch17sel(&mut self) -> CH17SEL_W {
        CH17SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn ch18sel(&mut self) -> CH18SEL_W {
        CH18SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn ch19sel(&mut self) -> CH19SEL_W {
        CH19SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn ch20sel(&mut self) -> CH20SEL_W {
        CH20SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn ch21sel(&mut self) -> CH21SEL_W {
        CH21SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn ch22sel(&mut self) -> CH22SEL_W {
        CH22SEL_W { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn ch23sel(&mut self) -> CH23SEL_W {
        CH23SEL_W { w: self }
    }
}
