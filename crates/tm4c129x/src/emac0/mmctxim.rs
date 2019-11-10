#[doc = "Reader of register MMCTXIM"]
pub type R = crate::R<u32, super::MMCTXIM>;
#[doc = "Writer for register MMCTXIM"]
pub type W = crate::W<u32, super::MMCTXIM>;
#[doc = "Register MMCTXIM `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCTXIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GBF`"]
pub type GBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GBF`"]
pub struct GBF_W<'a> {
    w: &'a mut W,
}
impl<'a> GBF_W<'a> {
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
#[doc = "Reader of field `SCOLLGF`"]
pub type SCOLLGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCOLLGF`"]
pub struct SCOLLGF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCOLLGF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCOLLGF`"]
pub type MCOLLGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCOLLGF`"]
pub struct MCOLLGF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOLLGF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `OCTCNT`"]
pub type OCTCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTCNT`"]
pub struct OCTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn gbf(&self) -> GBF_R {
        GBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn scollgf(&self) -> SCOLLGF_R {
        SCOLLGF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn mcollgf(&self) -> MCOLLGF_R {
        MCOLLGF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn octcnt(&self) -> OCTCNT_R {
        OCTCNT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn gbf(&mut self) -> GBF_W {
        GBF_W { w: self }
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn scollgf(&mut self) -> SCOLLGF_W {
        SCOLLGF_W { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn mcollgf(&mut self) -> MCOLLGF_W {
        MCOLLGF_W { w: self }
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn octcnt(&mut self) -> OCTCNT_W {
        OCTCNT_W { w: self }
    }
}
