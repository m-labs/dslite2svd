#[doc = "Reader of register MMCCTRL"]
pub type R = crate::R<u32, super::MMCCTRL>;
#[doc = "Writer for register MMCCTRL"]
pub type W = crate::W<u32, super::MMCCTRL>;
#[doc = "Register MMCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTRST`"]
pub type CNTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTRST`"]
pub struct CNTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTRST_W<'a> {
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
#[doc = "Reader of field `CNTSTPRO`"]
pub type CNTSTPRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTSTPRO`"]
pub struct CNTSTPRO_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTPRO_W<'a> {
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
#[doc = "Reader of field `RSTONRD`"]
pub type RSTONRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTONRD`"]
pub struct RSTONRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTONRD_W<'a> {
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
#[doc = "Reader of field `CNTFREEZ`"]
pub type CNTFREEZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTFREEZ`"]
pub struct CNTFREEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTFREEZ_W<'a> {
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
#[doc = "Reader of field `CNTPRST`"]
pub type CNTPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRST`"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
#[doc = "Reader of field `CNTPRSTLVL`"]
pub type CNTPRSTLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRSTLVL`"]
pub struct CNTPRSTLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRSTLVL_W<'a> {
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
#[doc = "Reader of field `UCDBC`"]
pub type UCDBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCDBC`"]
pub struct UCDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDBC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn cntstpro(&self) -> CNTSTPRO_R {
        CNTSTPRO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CNTRST_W {
        CNTRST_W { w: self }
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn cntstpro(&mut self) -> CNTSTPRO_W {
        CNTSTPRO_W { w: self }
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RSTONRD_W {
        RSTONRD_W { w: self }
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W {
        CNTFREEZ_W { w: self }
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W {
        CNTPRSTLVL_W { w: self }
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UCDBC_W {
        UCDBC_W { w: self }
    }
}
