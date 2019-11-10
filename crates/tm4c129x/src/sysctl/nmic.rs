#[doc = "Reader of register NMIC"]
pub type R = crate::R<u32, super::NMIC>;
#[doc = "Writer for register NMIC"]
pub type W = crate::W<u32, super::NMIC>;
#[doc = "Register NMIC `reset()`'s with value 0"]
impl crate::ResetValue for super::NMIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTERNAL`"]
pub type EXTERNAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTERNAL`"]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
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
#[doc = "Reader of field `POWER`"]
pub type POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWER`"]
pub struct POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_W<'a> {
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
#[doc = "Reader of field `WDT0`"]
pub type WDT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT0`"]
pub struct WDT0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_W<'a> {
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
#[doc = "Reader of field `WDT1`"]
pub type WDT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT1`"]
pub struct WDT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_W<'a> {
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
#[doc = "Reader of field `TAMPER`"]
pub type TAMPER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPER`"]
pub struct TAMPER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPER_W<'a> {
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
#[doc = "Reader of field `MOSCFAIL`"]
pub type MOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCFAIL`"]
pub struct MOSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn moscfail(&self) -> MOSCFAIL_R {
        MOSCFAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn power(&mut self) -> POWER_W {
        POWER_W { w: self }
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn wdt0(&mut self) -> WDT0_W {
        WDT0_W { w: self }
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn wdt1(&mut self) -> WDT1_W {
        WDT1_W { w: self }
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn tamper(&mut self) -> TAMPER_W {
        TAMPER_W { w: self }
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn moscfail(&mut self) -> MOSCFAIL_W {
        MOSCFAIL_W { w: self }
    }
}
