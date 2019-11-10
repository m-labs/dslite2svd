#[doc = "Reader of register ADCEV"]
pub type R = crate::R<u32, super::ADCEV>;
#[doc = "Writer for register ADCEV"]
pub type W = crate::W<u32, super::ADCEV>;
#[doc = "Register ADCEV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TATOADCEN`"]
pub type TATOADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOADCEN`"]
pub struct TATOADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOADCEN_W<'a> {
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
#[doc = "Reader of field `CAMADCEN`"]
pub type CAMADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMADCEN`"]
pub struct CAMADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMADCEN_W<'a> {
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
#[doc = "Reader of field `CAEADCEN`"]
pub type CAEADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEADCEN`"]
pub struct CAEADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEADCEN_W<'a> {
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
#[doc = "Reader of field `RTCADCEN`"]
pub type RTCADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCADCEN`"]
pub struct RTCADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCADCEN_W<'a> {
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
#[doc = "Reader of field `TAMADCEN`"]
pub type TAMADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMADCEN`"]
pub struct TAMADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMADCEN_W<'a> {
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
#[doc = "Reader of field `TBTOADCEN`"]
pub type TBTOADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOADCEN`"]
pub struct TBTOADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOADCEN_W<'a> {
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
#[doc = "Reader of field `CBMADCEN`"]
pub type CBMADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMADCEN`"]
pub struct CBMADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMADCEN_W<'a> {
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
#[doc = "Reader of field `CBEADCEN`"]
pub type CBEADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEADCEN`"]
pub struct CBEADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEADCEN_W<'a> {
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
#[doc = "Reader of field `TBMADCEN`"]
pub type TBMADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMADCEN`"]
pub struct TBMADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMADCEN_W<'a> {
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
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tatoadcen(&self) -> TATOADCEN_R {
        TATOADCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn camadcen(&self) -> CAMADCEN_R {
        CAMADCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn caeadcen(&self) -> CAEADCEN_R {
        CAEADCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn rtcadcen(&self) -> RTCADCEN_R {
        RTCADCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tamadcen(&self) -> TAMADCEN_R {
        TAMADCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tbtoadcen(&self) -> TBTOADCEN_R {
        TBTOADCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn cbmadcen(&self) -> CBMADCEN_R {
        CBMADCEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn cbeadcen(&self) -> CBEADCEN_R {
        CBEADCEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tbmadcen(&self) -> TBMADCEN_R {
        TBMADCEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tatoadcen(&mut self) -> TATOADCEN_W {
        TATOADCEN_W { w: self }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn camadcen(&mut self) -> CAMADCEN_W {
        CAMADCEN_W { w: self }
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn caeadcen(&mut self) -> CAEADCEN_W {
        CAEADCEN_W { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn rtcadcen(&mut self) -> RTCADCEN_W {
        RTCADCEN_W { w: self }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tamadcen(&mut self) -> TAMADCEN_W {
        TAMADCEN_W { w: self }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tbtoadcen(&mut self) -> TBTOADCEN_W {
        TBTOADCEN_W { w: self }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn cbmadcen(&mut self) -> CBMADCEN_W {
        CBMADCEN_W { w: self }
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn cbeadcen(&mut self) -> CBEADCEN_W {
        CBEADCEN_W { w: self }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn tbmadcen(&mut self) -> TBMADCEN_W {
        TBMADCEN_W { w: self }
    }
}
