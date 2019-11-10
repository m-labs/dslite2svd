#[doc = "Reader of register SRI2C"]
pub type R = crate::R<u32, super::SRI2C>;
#[doc = "Writer for register SRI2C"]
pub type W = crate::W<u32, super::SRI2C>;
#[doc = "Register SRI2C `reset()`'s with value 0"]
impl crate::ResetValue for super::SRI2C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R0`"]
pub struct R0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0_W<'a> {
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
#[doc = "Reader of field `R1`"]
pub type R1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R1`"]
pub struct R1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1_W<'a> {
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
#[doc = "Reader of field `R2`"]
pub type R2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R2`"]
pub struct R2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2_W<'a> {
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
#[doc = "Reader of field `R3`"]
pub type R3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R3`"]
pub struct R3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3_W<'a> {
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
#[doc = "Reader of field `R4`"]
pub type R4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R4`"]
pub struct R4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4_W<'a> {
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
#[doc = "Reader of field `R5`"]
pub type R5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5`"]
pub struct R5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5_W<'a> {
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
#[doc = "Reader of field `R6`"]
pub type R6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R6`"]
pub struct R6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6_W<'a> {
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
#[doc = "Reader of field `R7`"]
pub type R7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R7`"]
pub struct R7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7_W<'a> {
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
#[doc = "Reader of field `R8`"]
pub type R8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R8`"]
pub struct R8_W<'a> {
    w: &'a mut W,
}
impl<'a> R8_W<'a> {
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
#[doc = "Reader of field `R9`"]
pub type R9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R9`"]
pub struct R9_W<'a> {
    w: &'a mut W,
}
impl<'a> R9_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Module 0 Software Reset"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Software Reset"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Software Reset"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Software Reset"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Software Reset"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Software Reset"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Module 6 Software Reset"]
    #[inline(always)]
    pub fn r6(&self) -> R6_R {
        R6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Module 7 Software Reset"]
    #[inline(always)]
    pub fn r7(&self) -> R7_R {
        R7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Module 8 Software Reset"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Module 9 Software Reset"]
    #[inline(always)]
    pub fn r9(&self) -> R9_R {
        R9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Software Reset"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Module 1 Software Reset"]
    #[inline(always)]
    pub fn r1(&mut self) -> R1_W {
        R1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Module 2 Software Reset"]
    #[inline(always)]
    pub fn r2(&mut self) -> R2_W {
        R2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Module 3 Software Reset"]
    #[inline(always)]
    pub fn r3(&mut self) -> R3_W {
        R3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Module 4 Software Reset"]
    #[inline(always)]
    pub fn r4(&mut self) -> R4_W {
        R4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Module 5 Software Reset"]
    #[inline(always)]
    pub fn r5(&mut self) -> R5_W {
        R5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Module 6 Software Reset"]
    #[inline(always)]
    pub fn r6(&mut self) -> R6_W {
        R6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Module 7 Software Reset"]
    #[inline(always)]
    pub fn r7(&mut self) -> R7_W {
        R7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Module 8 Software Reset"]
    #[inline(always)]
    pub fn r8(&mut self) -> R8_W {
        R8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Module 9 Software Reset"]
    #[inline(always)]
    pub fn r9(&mut self) -> R9_W {
        R9_W { w: self }
    }
}
