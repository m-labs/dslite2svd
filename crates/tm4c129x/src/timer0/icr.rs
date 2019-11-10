#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TATOCINT`"]
pub struct TATOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOCINT_W<'a> {
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
#[doc = "Write proxy for field `CAMCINT`"]
pub struct CAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMCINT_W<'a> {
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
#[doc = "Write proxy for field `CAECINT`"]
pub struct CAECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECINT_W<'a> {
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
#[doc = "Write proxy for field `RTCCINT`"]
pub struct RTCCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCINT_W<'a> {
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
#[doc = "Write proxy for field `TAMCINT`"]
pub struct TAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMCINT_W<'a> {
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
#[doc = "Write proxy for field `DMAAINT`"]
pub struct DMAAINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAAINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `TBTOCINT`"]
pub struct TBTOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOCINT_W<'a> {
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
#[doc = "Write proxy for field `CBMCINT`"]
pub struct CBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMCINT_W<'a> {
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
#[doc = "Write proxy for field `CBECINT`"]
pub struct CBECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBECINT_W<'a> {
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
#[doc = "Write proxy for field `TBMCINT`"]
pub struct TBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMCINT_W<'a> {
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
#[doc = "Write proxy for field `DMABINT`"]
pub struct DMABINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn tatocint(&mut self) -> TATOCINT_W {
        TATOCINT_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn camcint(&mut self) -> CAMCINT_W {
        CAMCINT_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn caecint(&mut self) -> CAECINT_W {
        CAECINT_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Clear"]
    #[inline(always)]
    pub fn rtccint(&mut self) -> RTCCINT_W {
        RTCCINT_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Clear"]
    #[inline(always)]
    pub fn tamcint(&mut self) -> TAMCINT_W {
        TAMCINT_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn dmaaint(&mut self) -> DMAAINT_W {
        DMAAINT_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn tbtocint(&mut self) -> TBTOCINT_W {
        TBTOCINT_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn cbmcint(&mut self) -> CBMCINT_W {
        CBMCINT_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn cbecint(&mut self) -> CBECINT_W {
        CBECINT_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Clear"]
    #[inline(always)]
    pub fn tbmcint(&mut self) -> TBMCINT_W {
        TBMCINT_W { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn dmabint(&mut self) -> DMABINT_W {
        DMABINT_W { w: self }
    }
}
