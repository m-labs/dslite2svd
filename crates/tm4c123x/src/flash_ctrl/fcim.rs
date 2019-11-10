#[doc = "Reader of register FCIM"]
pub type R = crate::R<u32, super::FCIM>;
#[doc = "Writer for register FCIM"]
pub type W = crate::W<u32, super::FCIM>;
#[doc = "Register FCIM `reset()`'s with value 0"]
impl crate::ResetValue for super::FCIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMASK`"]
pub type AMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMASK`"]
pub struct AMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AMASK_W<'a> {
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
#[doc = "Reader of field `PMASK`"]
pub type PMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMASK`"]
pub struct PMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMASK_W<'a> {
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
#[doc = "Reader of field `EMASK`"]
pub type EMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMASK`"]
pub struct EMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EMASK_W<'a> {
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
#[doc = "Reader of field `VOLTMASK`"]
pub type VOLTMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VOLTMASK`"]
pub struct VOLTMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLTMASK_W<'a> {
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
#[doc = "Reader of field `INVDMASK`"]
pub type INVDMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVDMASK`"]
pub struct INVDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INVDMASK_W<'a> {
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
#[doc = "Reader of field `ERMASK`"]
pub type ERMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERMASK`"]
pub struct ERMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERMASK_W<'a> {
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
#[doc = "Reader of field `PROGMASK`"]
pub type PROGMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGMASK`"]
pub struct PROGMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGMASK_W<'a> {
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
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn amask(&self) -> AMASK_R {
        AMASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn pmask(&self) -> PMASK_R {
        PMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn emask(&self) -> EMASK_R {
        EMASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn voltmask(&self) -> VOLTMASK_R {
        VOLTMASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn invdmask(&self) -> INVDMASK_R {
        INVDMASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn ermask(&self) -> ERMASK_R {
        ERMASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn progmask(&self) -> PROGMASK_R {
        PROGMASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn amask(&mut self) -> AMASK_W {
        AMASK_W { w: self }
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn pmask(&mut self) -> PMASK_W {
        PMASK_W { w: self }
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn emask(&mut self) -> EMASK_W {
        EMASK_W { w: self }
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn voltmask(&mut self) -> VOLTMASK_W {
        VOLTMASK_W { w: self }
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn invdmask(&mut self) -> INVDMASK_W {
        INVDMASK_W { w: self }
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn ermask(&mut self) -> ERMASK_W {
        ERMASK_W { w: self }
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn progmask(&mut self) -> PROGMASK_W {
        PROGMASK_W { w: self }
    }
}
