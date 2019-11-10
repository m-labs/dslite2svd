#[doc = "Reader of register CALCTL"]
pub type R = crate::R<u32, super::CALCTL>;
#[doc = "Writer for register CALCTL"]
pub type W = crate::W<u32, super::CALCTL>;
#[doc = "Register CALCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CALCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALEN`"]
pub type CALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALEN`"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
#[doc = "Reader of field `CAL24`"]
pub type CAL24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL24`"]
pub struct CAL24_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL24_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTC Calendar/Counter Mode Select"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Calendar Mode"]
    #[inline(always)]
    pub fn cal24(&self) -> CAL24_R {
        CAL24_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Calendar/Counter Mode Select"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    #[doc = "Bit 2 - Calendar Mode"]
    #[inline(always)]
    pub fn cal24(&mut self) -> CAL24_W {
        CAL24_W { w: self }
    }
}
