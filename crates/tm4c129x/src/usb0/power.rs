#[doc = "Reader of register POWER"]
pub type R = crate::R<u8, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u8, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRDNPHY`"]
pub type PWRDNPHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDNPHY`"]
pub struct PWRDNPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDNPHY_W<'a> {
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
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HSMODE`"]
pub type HSMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSMODE`"]
pub struct HSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMODE_W<'a> {
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
#[doc = "Reader of field `HSENAB`"]
pub type HSENAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSENAB`"]
pub struct HSENAB_W<'a> {
    w: &'a mut W,
}
impl<'a> HSENAB_W<'a> {
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
#[doc = "Reader of field `SOFTCONN`"]
pub type SOFTCONN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTCONN`"]
pub struct SOFTCONN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTCONN_W<'a> {
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
#[doc = "Reader of field `ISOUP`"]
pub type ISOUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOUP`"]
pub struct ISOUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOUP_W<'a> {
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
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    pub fn pwrdnphy(&self) -> PWRDNPHY_R {
        PWRDNPHY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High Speed Enable"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High Speed Enable"]
    #[inline(always)]
    pub fn hsenab(&self) -> HSENAB_R {
        HSENAB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    pub fn softconn(&self) -> SOFTCONN_R {
        SOFTCONN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    pub fn isoup(&self) -> ISOUP_R {
        ISOUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    pub fn pwrdnphy(&mut self) -> PWRDNPHY_W {
        PWRDNPHY_W { w: self }
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 4 - High Speed Enable"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HSMODE_W {
        HSMODE_W { w: self }
    }
    #[doc = "Bit 5 - High Speed Enable"]
    #[inline(always)]
    pub fn hsenab(&mut self) -> HSENAB_W {
        HSENAB_W { w: self }
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    pub fn softconn(&mut self) -> SOFTCONN_W {
        SOFTCONN_W { w: self }
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    pub fn isoup(&mut self) -> ISOUP_W {
        ISOUP_W { w: self }
    }
}
