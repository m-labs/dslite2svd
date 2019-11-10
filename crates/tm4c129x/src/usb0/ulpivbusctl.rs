#[doc = "Reader of register ULPIVBUSCTL"]
pub type R = crate::R<u8, super::ULPIVBUSCTL>;
#[doc = "Writer for register ULPIVBUSCTL"]
pub type W = crate::W<u8, super::ULPIVBUSCTL>;
#[doc = "Register ULPIVBUSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ULPIVBUSCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USEEXTVBUS`"]
pub type USEEXTVBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEEXTVBUS`"]
pub struct USEEXTVBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> USEEXTVBUS_W<'a> {
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
#[doc = "Reader of field `USEEXTVBUSIND`"]
pub type USEEXTVBUSIND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEEXTVBUSIND`"]
pub struct USEEXTVBUSIND_W<'a> {
    w: &'a mut W,
}
impl<'a> USEEXTVBUSIND_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn useextvbus(&self) -> USEEXTVBUS_R {
        USEEXTVBUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn useextvbusind(&self) -> USEEXTVBUSIND_R {
        USEEXTVBUSIND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn useextvbus(&mut self) -> USEEXTVBUS_W {
        USEEXTVBUS_W { w: self }
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn useextvbusind(&mut self) -> USEEXTVBUSIND_W {
        USEEXTVBUSIND_W { w: self }
    }
}
