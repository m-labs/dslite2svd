#[doc = "Reader of register ACTSS"]
pub type R = crate::R<u32, super::ACTSS>;
#[doc = "Writer for register ACTSS"]
pub type W = crate::W<u32, super::ACTSS>;
#[doc = "Register ACTSS `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASEN0`"]
pub type ASEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN0`"]
pub struct ASEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN0_W<'a> {
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
#[doc = "Reader of field `ASEN1`"]
pub type ASEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN1`"]
pub struct ASEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN1_W<'a> {
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
#[doc = "Reader of field `ASEN2`"]
pub type ASEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN2`"]
pub struct ASEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN2_W<'a> {
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
#[doc = "Reader of field `ASEN3`"]
pub type ASEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASEN3`"]
pub struct ASEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASEN3_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn asen0(&self) -> ASEN0_R {
        ASEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn asen1(&self) -> ASEN1_R {
        ASEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn asen2(&self) -> ASEN2_R {
        ASEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn asen3(&self) -> ASEN3_R {
        ASEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn asen0(&mut self) -> ASEN0_W {
        ASEN0_W { w: self }
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn asen1(&mut self) -> ASEN1_W {
        ASEN1_W { w: self }
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn asen2(&mut self) -> ASEN2_W {
        ASEN2_W { w: self }
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn asen3(&mut self) -> ASEN3_W {
        ASEN3_W { w: self }
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
