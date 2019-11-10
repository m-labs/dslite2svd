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
#[doc = "Reader of field `ADEN0`"]
pub type ADEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEN0`"]
pub struct ADEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN0_W<'a> {
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
#[doc = "Reader of field `ADEN1`"]
pub type ADEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEN1`"]
pub struct ADEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN1_W<'a> {
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
#[doc = "Reader of field `ADEN2`"]
pub type ADEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEN2`"]
pub struct ADEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADEN3`"]
pub type ADEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEN3`"]
pub struct ADEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn aden0(&self) -> ADEN0_R {
        ADEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn aden1(&self) -> ADEN1_R {
        ADEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn aden2(&self) -> ADEN2_R {
        ADEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn aden3(&self) -> ADEN3_R {
        ADEN3_R::new(((self.bits >> 11) & 0x01) != 0)
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
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn aden0(&mut self) -> ADEN0_W {
        ADEN0_W { w: self }
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn aden1(&mut self) -> ADEN1_W {
        ADEN1_W { w: self }
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn aden2(&mut self) -> ADEN2_W {
        ADEN2_W { w: self }
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn aden3(&mut self) -> ADEN3_W {
        ADEN3_W { w: self }
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
