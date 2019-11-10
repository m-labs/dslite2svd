#[doc = "Reader of register RCGCSSI"]
pub type R = crate::R<u32, super::RCGCSSI>;
#[doc = "Writer for register RCGCSSI"]
pub type W = crate::W<u32, super::RCGCSSI>;
#[doc = "Register RCGCSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::RCGCSSI {
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
impl R {
    #[doc = "Bit 0 - SSI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
    #[doc = "Bit 1 - SSI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r1(&mut self) -> R1_W {
        R1_W { w: self }
    }
    #[doc = "Bit 2 - SSI Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r2(&mut self) -> R2_W {
        R2_W { w: self }
    }
    #[doc = "Bit 3 - SSI Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn r3(&mut self) -> R3_W {
        R3_W { w: self }
    }
}
