#[doc = "Reader of register IC"]
pub type R = crate::R<u32, super::IC>;
#[doc = "Writer for register IC"]
pub type W = crate::W<u32, super::IC>;
#[doc = "Register IC `reset()`'s with value 0"]
impl crate::ResetValue for super::IC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCALT0`"]
pub type RTCALT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCALT0`"]
pub struct RTCALT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALT0_W<'a> {
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
#[doc = "Reader of field `LOWBAT`"]
pub type LOWBAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOWBAT`"]
pub struct LOWBAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWBAT_W<'a> {
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
#[doc = "Reader of field `EXTW`"]
pub type EXTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTW`"]
pub struct EXTW_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTW_W<'a> {
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
#[doc = "Reader of field `WC`"]
pub type WC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WC`"]
pub struct WC_W<'a> {
    w: &'a mut W,
}
impl<'a> WC_W<'a> {
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
#[doc = "Reader of field `PADIOWK`"]
pub type PADIOWK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADIOWK`"]
pub struct PADIOWK_W<'a> {
    w: &'a mut W,
}
impl<'a> PADIOWK_W<'a> {
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
#[doc = "Reader of field `RSTWK`"]
pub type RSTWK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTWK`"]
pub struct RSTWK_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTWK_W<'a> {
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
#[doc = "Reader of field `VDDFAIL`"]
pub type VDDFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDFAIL`"]
pub struct VDDFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDFAIL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    pub fn rtcalt0(&self) -> RTCALT0_R {
        RTCALT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Clear"]
    #[inline(always)]
    pub fn lowbat(&self) -> LOWBAT_R {
        LOWBAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn extw(&self) -> EXTW_R {
        EXTW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Interrupt Clear"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn padiowk(&self) -> PADIOWK_R {
        PADIOWK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn rstwk(&self) -> RSTWK_R {
        RSTWK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Clear"]
    #[inline(always)]
    pub fn vddfail(&self) -> VDDFAIL_R {
        VDDFAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    pub fn rtcalt0(&mut self) -> RTCALT0_W {
        RTCALT0_W { w: self }
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Clear"]
    #[inline(always)]
    pub fn lowbat(&mut self) -> LOWBAT_W {
        LOWBAT_W { w: self }
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn extw(&mut self) -> EXTW_W {
        EXTW_W { w: self }
    }
    #[doc = "Bit 4 - Write Complete/Capable Interrupt Clear"]
    #[inline(always)]
    pub fn wc(&mut self) -> WC_W {
        WC_W { w: self }
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn padiowk(&mut self) -> PADIOWK_W {
        PADIOWK_W { w: self }
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn rstwk(&mut self) -> RSTWK_W {
        RSTWK_W { w: self }
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Clear"]
    #[inline(always)]
    pub fn vddfail(&mut self) -> VDDFAIL_W {
        VDDFAIL_W { w: self }
    }
}
