#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
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
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn rtcalt0(&self) -> RTCALT0_R {
        RTCALT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn lowbat(&self) -> LOWBAT_R {
        LOWBAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn extw(&self) -> EXTW_R {
        EXTW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn rtcalt0(&mut self) -> RTCALT0_W {
        RTCALT0_W { w: self }
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn lowbat(&mut self) -> LOWBAT_W {
        LOWBAT_W { w: self }
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn extw(&mut self) -> EXTW_W {
        EXTW_W { w: self }
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn wc(&mut self) -> WC_W {
        WC_W { w: self }
    }
}
