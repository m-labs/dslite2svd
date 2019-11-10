#[doc = "Reader of register DMAEV"]
pub type R = crate::R<u32, super::DMAEV>;
#[doc = "Writer for register DMAEV"]
pub type W = crate::W<u32, super::DMAEV>;
#[doc = "Register DMAEV `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TATODMAEN`"]
pub type TATODMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATODMAEN`"]
pub struct TATODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TATODMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CAMDMAEN`"]
pub type CAMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMDMAEN`"]
pub struct CAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CAEDMAEN`"]
pub type CAEDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEDMAEN`"]
pub struct CAEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCDMAEN`"]
pub type RTCDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCDMAEN`"]
pub struct RTCDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TAMDMAEN`"]
pub type TAMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMDMAEN`"]
pub struct TAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TBTODMAEN`"]
pub type TBTODMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTODMAEN`"]
pub struct TBTODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTODMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CBMDMAEN`"]
pub type CBMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMDMAEN`"]
pub struct CBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CBEDMAEN`"]
pub type CBEDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEDMAEN`"]
pub struct CBEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBMDMAEN`"]
pub type TBMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMDMAEN`"]
pub struct TBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&self) -> TATODMAEN_R {
        TATODMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&self) -> CAMDMAEN_R {
        CAMDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&self) -> CAEDMAEN_R {
        CAEDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn rtcdmaen(&self) -> RTCDMAEN_R {
        RTCDMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&self) -> TAMDMAEN_R {
        TAMDMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&self) -> TBTODMAEN_R {
        TBTODMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&self) -> CBMDMAEN_R {
        CBMDMAEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&self) -> CBEDMAEN_R {
        CBEDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&self) -> TBMDMAEN_R {
        TBMDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&mut self) -> TATODMAEN_W {
        TATODMAEN_W { w: self }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&mut self) -> CAMDMAEN_W {
        CAMDMAEN_W { w: self }
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&mut self) -> CAEDMAEN_W {
        CAEDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn rtcdmaen(&mut self) -> RTCDMAEN_W {
        RTCDMAEN_W { w: self }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&mut self) -> TAMDMAEN_W {
        TAMDMAEN_W { w: self }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&mut self) -> TBTODMAEN_W {
        TBTODMAEN_W { w: self }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&mut self) -> CBMDMAEN_W {
        CBMDMAEN_W { w: self }
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&mut self) -> CBEDMAEN_W {
        CBEDMAEN_W { w: self }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&mut self) -> TBMDMAEN_W {
        TBMDMAEN_W { w: self }
    }
}
