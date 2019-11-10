#[doc = "Reader of register DEVCTL"]
pub type R = crate::R<u8, super::DEVCTL>;
#[doc = "Writer for register DEVCTL"]
pub type W = crate::W<u8, super::DEVCTL>;
#[doc = "Register DEVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SESSION`"]
pub type SESSION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSION`"]
pub struct SESSION_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSION_W<'a> {
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
#[doc = "Reader of field `HOSTREQ`"]
pub type HOSTREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTREQ`"]
pub struct HOSTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTREQ_W<'a> {
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
#[doc = "Reader of field `HOST`"]
pub type HOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST`"]
pub struct HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_W<'a> {
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
#[doc = "VBUS Level (OTG only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_A {
    #[doc = "0: Below SessionEnd"]
    NONE,
    #[doc = "1: Above SessionEnd, below AValid"]
    SEND,
    #[doc = "2: Above AValid, below VBUSValid"]
    AVALID,
    #[doc = "3: Above VBUSValid"]
    VALID,
}
impl From<VBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUS_A) -> Self {
        match variant {
            VBUS_A::NONE => 0,
            VBUS_A::SEND => 1,
            VBUS_A::AVALID => 2,
            VBUS_A::VALID => 3,
        }
    }
}
#[doc = "Reader of field `VBUS`"]
pub type VBUS_R = crate::R<u8, VBUS_A>;
impl VBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_A {
        match self.bits {
            0 => VBUS_A::NONE,
            1 => VBUS_A::SEND,
            2 => VBUS_A::AVALID,
            3 => VBUS_A::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VBUS_A::NONE
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        *self == VBUS_A::SEND
    }
    #[doc = "Checks if the value of the field is `AVALID`"]
    #[inline(always)]
    pub fn is_avalid(&self) -> bool {
        *self == VBUS_A::AVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VBUS_A::VALID
    }
}
#[doc = "Write proxy for field `VBUS`"]
pub struct VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VBUS_A::NONE)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn send(self) -> &'a mut W {
        self.variant(VBUS_A::SEND)
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline(always)]
    pub fn avalid(self) -> &'a mut W {
        self.variant(VBUS_A::AVALID)
    }
    #[doc = "Above VBUSValid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VBUS_A::VALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `LSDEV`"]
pub type LSDEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSDEV`"]
pub struct LSDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSDEV_W<'a> {
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
#[doc = "Reader of field `FSDEV`"]
pub type FSDEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSDEV`"]
pub struct FSDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDEV_W<'a> {
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
#[doc = "Reader of field `DEV`"]
pub type DEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV`"]
pub struct DEV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_W<'a> {
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
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn session(&self) -> SESSION_R {
        SESSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn hostreq(&self) -> HOSTREQ_R {
        HOSTREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn fsdev(&self) -> FSDEV_R {
        FSDEV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn dev(&self) -> DEV_R {
        DEV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn session(&mut self) -> SESSION_W {
        SESSION_W { w: self }
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn hostreq(&mut self) -> HOSTREQ_W {
        HOSTREQ_W { w: self }
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn host(&mut self) -> HOST_W {
        HOST_W { w: self }
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W { w: self }
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W {
        LSDEV_W { w: self }
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn fsdev(&mut self) -> FSDEV_W {
        FSDEV_W { w: self }
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn dev(&mut self) -> DEV_W {
        DEV_W { w: self }
    }
}
