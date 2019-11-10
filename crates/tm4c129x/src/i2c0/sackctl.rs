#[doc = "Reader of register SACKCTL"]
pub type R = crate::R<u32, super::SACKCTL>;
#[doc = "Writer for register SACKCTL"]
pub type W = crate::W<u32, super::SACKCTL>;
#[doc = "Register SACKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SACKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACKOEN`"]
pub type ACKOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKOEN`"]
pub struct ACKOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKOEN_W<'a> {
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
#[doc = "Reader of field `ACKOVAL`"]
pub type ACKOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKOVAL`"]
pub struct ACKOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKOVAL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn ackoen(&self) -> ACKOEN_R {
        ACKOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    pub fn ackoval(&self) -> ACKOVAL_R {
        ACKOVAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn ackoen(&mut self) -> ACKOEN_W {
        ACKOEN_W { w: self }
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    pub fn ackoval(&mut self) -> ACKOVAL_W {
        ACKOVAL_W { w: self }
    }
}
