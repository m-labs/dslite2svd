#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GLOBALSYNC0`"]
pub type GLOBALSYNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBALSYNC0`"]
pub struct GLOBALSYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBALSYNC0_W<'a> {
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
#[doc = "Reader of field `GLOBALSYNC1`"]
pub type GLOBALSYNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBALSYNC1`"]
pub struct GLOBALSYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBALSYNC1_W<'a> {
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
#[doc = "Reader of field `GLOBALSYNC2`"]
pub type GLOBALSYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBALSYNC2`"]
pub struct GLOBALSYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBALSYNC2_W<'a> {
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
#[doc = "Reader of field `GLOBALSYNC3`"]
pub type GLOBALSYNC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBALSYNC3`"]
pub struct GLOBALSYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBALSYNC3_W<'a> {
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
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn globalsync0(&self) -> GLOBALSYNC0_R {
        GLOBALSYNC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn globalsync1(&self) -> GLOBALSYNC1_R {
        GLOBALSYNC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn globalsync2(&self) -> GLOBALSYNC2_R {
        GLOBALSYNC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    pub fn globalsync3(&self) -> GLOBALSYNC3_R {
        GLOBALSYNC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn globalsync0(&mut self) -> GLOBALSYNC0_W {
        GLOBALSYNC0_W { w: self }
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn globalsync1(&mut self) -> GLOBALSYNC1_W {
        GLOBALSYNC1_W { w: self }
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn globalsync2(&mut self) -> GLOBALSYNC2_W {
        GLOBALSYNC2_W { w: self }
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    pub fn globalsync3(&mut self) -> GLOBALSYNC3_W {
        GLOBALSYNC3_W { w: self }
    }
}
