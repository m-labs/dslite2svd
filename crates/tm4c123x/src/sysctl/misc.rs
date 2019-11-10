#[doc = "Reader of register MISC"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register MISC"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOR1MIS`"]
pub type BOR1MIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR1MIS`"]
pub struct BOR1MIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR1MIS_W<'a> {
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
#[doc = "Reader of field `MOFMIS`"]
pub type MOFMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOFMIS`"]
pub struct MOFMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOFMIS_W<'a> {
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
#[doc = "Reader of field `PLLLMIS`"]
pub type PLLLMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLLMIS`"]
pub struct PLLLMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `USBPLLLMIS`"]
pub type USBPLLLMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPLLLMIS`"]
pub struct USBPLLLMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPLLLMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MOSCPUPMIS`"]
pub type MOSCPUPMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCPUPMIS`"]
pub struct MOSCPUPMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCPUPMIS_W<'a> {
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
#[doc = "Reader of field `VDDAMIS`"]
pub type VDDAMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDAMIS`"]
pub struct VDDAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDAMIS_W<'a> {
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
#[doc = "Reader of field `BOR0MIS`"]
pub type BOR0MIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR0MIS`"]
pub struct BOR0MIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR0MIS_W<'a> {
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
    #[doc = "Bit 1 - VDD under BOR1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn bor1mis(&self) -> BOR1MIS_R {
        BOR1MIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn mofmis(&self) -> MOFMIS_R {
        MOFMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn plllmis(&self) -> PLLLMIS_R {
        PLLLMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn usbplllmis(&self) -> USBPLLLMIS_R {
        USBPLLLMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn moscpupmis(&self) -> MOSCPUPMIS_R {
        MOSCPUPMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Masked Interrupt Status"]
    #[inline(always)]
    pub fn vddamis(&self) -> VDDAMIS_R {
        VDDAMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDD under BOR0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn bor0mis(&self) -> BOR0MIS_R {
        BOR0MIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VDD under BOR1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn bor1mis(&mut self) -> BOR1MIS_W {
        BOR1MIS_W { w: self }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn mofmis(&mut self) -> MOFMIS_W {
        MOFMIS_W { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn plllmis(&mut self) -> PLLLMIS_W {
        PLLLMIS_W { w: self }
    }
    #[doc = "Bit 7 - USB PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn usbplllmis(&mut self) -> USBPLLLMIS_W {
        USBPLLLMIS_W { w: self }
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn moscpupmis(&mut self) -> MOSCPUPMIS_W {
        MOSCPUPMIS_W { w: self }
    }
    #[doc = "Bit 10 - VDDA Power OK Masked Interrupt Status"]
    #[inline(always)]
    pub fn vddamis(&mut self) -> VDDAMIS_W {
        VDDAMIS_W { w: self }
    }
    #[doc = "Bit 11 - VDD under BOR0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn bor0mis(&mut self) -> BOR0MIS_W {
        BOR0MIS_W { w: self }
    }
}
