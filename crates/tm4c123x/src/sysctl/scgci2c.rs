#[doc = "Reader of register SCGCI2C"]
pub type R = crate::R<u32, super::SCGCI2C>;
#[doc = "Writer for register SCGCI2C"]
pub type W = crate::W<u32, super::SCGCI2C>;
#[doc = "Register SCGCI2C `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGCI2C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0`"]
pub type S0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0`"]
pub struct S0_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_W<'a> {
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
#[doc = "Reader of field `S1`"]
pub type S1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1`"]
pub struct S1_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_W<'a> {
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
#[doc = "Reader of field `S2`"]
pub type S2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S2`"]
pub struct S2_W<'a> {
    w: &'a mut W,
}
impl<'a> S2_W<'a> {
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
#[doc = "Reader of field `S3`"]
pub type S3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3`"]
pub struct S3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&self) -> S0_R {
        S0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&self) -> S1_R {
        S1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&self) -> S2_R {
        S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&self) -> S3_R {
        S3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W {
        S0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W {
        S1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W {
        S2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W {
        S3_W { w: self }
    }
}
