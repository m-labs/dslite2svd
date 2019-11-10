#[doc = "Reader of register FAULT"]
pub type R = crate::R<u32, super::FAULT>;
#[doc = "Writer for register FAULT"]
pub type W = crate::W<u32, super::FAULT>;
#[doc = "Register FAULT `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAULT0`"]
pub type FAULT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT0`"]
pub struct FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_W<'a> {
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
#[doc = "Reader of field `FAULT1`"]
pub type FAULT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT1`"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
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
#[doc = "Reader of field `FAULT2`"]
pub type FAULT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT2`"]
pub struct FAULT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_W<'a> {
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
#[doc = "Reader of field `FAULT3`"]
pub type FAULT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT3`"]
pub struct FAULT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT3_W<'a> {
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
#[doc = "Reader of field `FAULT4`"]
pub type FAULT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT4`"]
pub struct FAULT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT4_W<'a> {
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
#[doc = "Reader of field `FAULT5`"]
pub type FAULT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT5`"]
pub struct FAULT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT5_W<'a> {
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
#[doc = "Reader of field `FAULT6`"]
pub type FAULT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT6`"]
pub struct FAULT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT6_W<'a> {
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
#[doc = "Reader of field `FAULT7`"]
pub type FAULT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT7`"]
pub struct FAULT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT7_W<'a> {
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
    #[doc = "Bit 0 - MnPWM0 Fault"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Fault"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Fault"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Fault"]
    #[inline(always)]
    pub fn fault3(&self) -> FAULT3_R {
        FAULT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Fault"]
    #[inline(always)]
    pub fn fault4(&self) -> FAULT4_R {
        FAULT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Fault"]
    #[inline(always)]
    pub fn fault5(&self) -> FAULT5_R {
        FAULT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Fault"]
    #[inline(always)]
    pub fn fault6(&self) -> FAULT6_R {
        FAULT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Fault"]
    #[inline(always)]
    pub fn fault7(&self) -> FAULT7_R {
        FAULT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Fault"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 1 - MnPWM1 Fault"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 2 - MnPWM2 Fault"]
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT2_W {
        FAULT2_W { w: self }
    }
    #[doc = "Bit 3 - MnPWM3 Fault"]
    #[inline(always)]
    pub fn fault3(&mut self) -> FAULT3_W {
        FAULT3_W { w: self }
    }
    #[doc = "Bit 4 - MnPWM4 Fault"]
    #[inline(always)]
    pub fn fault4(&mut self) -> FAULT4_W {
        FAULT4_W { w: self }
    }
    #[doc = "Bit 5 - MnPWM5 Fault"]
    #[inline(always)]
    pub fn fault5(&mut self) -> FAULT5_W {
        FAULT5_W { w: self }
    }
    #[doc = "Bit 6 - MnPWM6 Fault"]
    #[inline(always)]
    pub fn fault6(&mut self) -> FAULT6_W {
        FAULT6_W { w: self }
    }
    #[doc = "Bit 7 - MnPWM7 Fault"]
    #[inline(always)]
    pub fn fault7(&mut self) -> FAULT7_W {
        FAULT7_W { w: self }
    }
}
