#[doc = "Reader of register FCMISC"]
pub type R = crate::R<u32, super::FCMISC>;
#[doc = "Writer for register FCMISC"]
pub type W = crate::W<u32, super::FCMISC>;
#[doc = "Register FCMISC `reset()`'s with value 0"]
impl crate::ResetValue for super::FCMISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMISC`"]
pub type AMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMISC`"]
pub struct AMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMISC_W<'a> {
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
#[doc = "Reader of field `PMISC`"]
pub type PMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMISC`"]
pub struct PMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> PMISC_W<'a> {
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
#[doc = "Reader of field `EMISC`"]
pub type EMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMISC`"]
pub struct EMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMISC_W<'a> {
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
#[doc = "Reader of field `VOLTMISC`"]
pub type VOLTMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VOLTMISC`"]
pub struct VOLTMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLTMISC_W<'a> {
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
#[doc = "Reader of field `INVDMISC`"]
pub type INVDMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVDMISC`"]
pub struct INVDMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVDMISC_W<'a> {
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
#[doc = "Reader of field `ERMISC`"]
pub type ERMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERMISC`"]
pub struct ERMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERMISC_W<'a> {
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
#[doc = "Reader of field `PROGMISC`"]
pub type PROGMISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGMISC`"]
pub struct PROGMISC_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGMISC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn amisc(&self) -> AMISC_R {
        AMISC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn pmisc(&self) -> PMISC_R {
        PMISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn emisc(&self) -> EMISC_R {
        EMISC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn voltmisc(&self) -> VOLTMISC_R {
        VOLTMISC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn invdmisc(&self) -> INVDMISC_R {
        INVDMISC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn ermisc(&self) -> ERMISC_R {
        ERMISC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn progmisc(&self) -> PROGMISC_R {
        PROGMISC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn amisc(&mut self) -> AMISC_W {
        AMISC_W { w: self }
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn pmisc(&mut self) -> PMISC_W {
        PMISC_W { w: self }
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn emisc(&mut self) -> EMISC_W {
        EMISC_W { w: self }
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn voltmisc(&mut self) -> VOLTMISC_W {
        VOLTMISC_W { w: self }
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn invdmisc(&mut self) -> INVDMISC_W {
        INVDMISC_W { w: self }
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn ermisc(&mut self) -> ERMISC_W {
        ERMISC_W { w: self }
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn progmisc(&mut self) -> PROGMISC_W {
        PROGMISC_W { w: self }
    }
}
