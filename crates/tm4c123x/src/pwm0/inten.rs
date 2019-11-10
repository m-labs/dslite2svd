#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPWM0`"]
pub type INTPWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTPWM0`"]
pub struct INTPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPWM0_W<'a> {
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
#[doc = "Reader of field `INTPWM1`"]
pub type INTPWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTPWM1`"]
pub struct INTPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPWM1_W<'a> {
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
#[doc = "Reader of field `INTPWM2`"]
pub type INTPWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTPWM2`"]
pub struct INTPWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPWM2_W<'a> {
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
#[doc = "Reader of field `INTPWM3`"]
pub type INTPWM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTPWM3`"]
pub struct INTPWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPWM3_W<'a> {
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
#[doc = "Reader of field `INTFAULT0`"]
pub type INTFAULT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTFAULT0`"]
pub struct INTFAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFAULT0_W<'a> {
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
#[doc = "Reader of field `INTFAULT1`"]
pub type INTFAULT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTFAULT1`"]
pub struct INTFAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFAULT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm0(&self) -> INTPWM0_R {
        INTPWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm1(&self) -> INTPWM1_R {
        INTPWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm2(&self) -> INTPWM2_R {
        INTPWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm3(&self) -> INTPWM3_R {
        INTPWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    pub fn intfault0(&self) -> INTFAULT0_R {
        INTFAULT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    pub fn intfault1(&self) -> INTFAULT1_R {
        INTFAULT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm0(&mut self) -> INTPWM0_W {
        INTPWM0_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm1(&mut self) -> INTPWM1_W {
        INTPWM1_W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm2(&mut self) -> INTPWM2_W {
        INTPWM2_W { w: self }
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    pub fn intpwm3(&mut self) -> INTPWM3_W {
        INTPWM3_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    pub fn intfault0(&mut self) -> INTFAULT0_W {
        INTFAULT0_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    pub fn intfault1(&mut self) -> INTFAULT1_W {
        INTFAULT1_W { w: self }
    }
}
