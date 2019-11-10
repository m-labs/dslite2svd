#[doc = "Reader of register ULPIREGCTL"]
pub type R = crate::R<u8, super::ULPIREGCTL>;
#[doc = "Writer for register ULPIREGCTL"]
pub type W = crate::W<u8, super::ULPIREGCTL>;
#[doc = "Register ULPIREGCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ULPIREGCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGACC`"]
pub type REGACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGACC`"]
pub struct REGACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REGACC_W<'a> {
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
#[doc = "Reader of field `REGCMPLT`"]
pub type REGCMPLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGCMPLT`"]
pub struct REGCMPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> REGCMPLT_W<'a> {
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
#[doc = "Reader of field `RDWR`"]
pub type RDWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDWR`"]
pub struct RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn regacc(&self) -> REGACC_R {
        REGACC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn regcmplt(&self) -> REGCMPLT_R {
        REGCMPLT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn regacc(&mut self) -> REGACC_W {
        REGACC_W { w: self }
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn regcmplt(&mut self) -> REGCMPLT_W {
        REGCMPLT_W { w: self }
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn rdwr(&mut self) -> RDWR_W {
        RDWR_W { w: self }
    }
}
