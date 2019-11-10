#[doc = "Reader of register IE"]
pub type R = crate::R<u8, super::IE>;
#[doc = "Writer for register IE"]
pub type W = crate::W<u8, super::IE>;
#[doc = "Register IE `reset()`'s with value 0"]
impl crate::ResetValue for super::IE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUSPND`"]
pub type SUSPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPND`"]
pub struct SUSPND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPND_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BABBLE`"]
pub type BABBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BABBLE`"]
pub struct BABBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BABBLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CONN`"]
pub type CONN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN`"]
pub struct CONN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DISCON`"]
pub type DISCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCON`"]
pub struct DISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SESREQ`"]
pub type SESREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESREQ`"]
pub struct SESREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SESREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VBUSERR`"]
pub type VBUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSERR`"]
pub struct VBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn suspnd(&self) -> SUSPND_R {
        SUSPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn babble(&self) -> BABBLE_R {
        BABBLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn conn(&self) -> CONN_R {
        CONN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn discon(&self) -> DISCON_R {
        DISCON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn sesreq(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn vbuserr(&self) -> VBUSERR_R {
        VBUSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn suspnd(&mut self) -> SUSPND_W {
        SUSPND_W { w: self }
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn babble(&mut self) -> BABBLE_W {
        BABBLE_W { w: self }
    }
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn conn(&mut self) -> CONN_W {
        CONN_W { w: self }
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn discon(&mut self) -> DISCON_W {
        DISCON_W { w: self }
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn sesreq(&mut self) -> SESREQ_W {
        SESREQ_W { w: self }
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn vbuserr(&mut self) -> VBUSERR_W {
        VBUSERR_W { w: self }
    }
}
