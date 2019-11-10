#[doc = "Reader of register FAULTVAL"]
pub type R = crate::R<u32, super::FAULTVAL>;
#[doc = "Writer for register FAULTVAL"]
pub type W = crate::W<u32, super::FAULTVAL>;
#[doc = "Register FAULTVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULTVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM0`"]
pub type PWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM0`"]
pub struct PWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_W<'a> {
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
#[doc = "Reader of field `PWM1`"]
pub type PWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM1`"]
pub struct PWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_W<'a> {
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
#[doc = "Reader of field `PWM2`"]
pub type PWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM2`"]
pub struct PWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_W<'a> {
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
#[doc = "Reader of field `PWM3`"]
pub type PWM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM3`"]
pub struct PWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3_W<'a> {
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
#[doc = "Reader of field `PWM4`"]
pub type PWM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM4`"]
pub struct PWM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM4_W<'a> {
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
#[doc = "Reader of field `PWM5`"]
pub type PWM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM5`"]
pub struct PWM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM5_W<'a> {
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
#[doc = "Reader of field `PWM6`"]
pub type PWM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM6`"]
pub struct PWM6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM6_W<'a> {
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
#[doc = "Reader of field `PWM7`"]
pub type PWM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM7`"]
pub struct PWM7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM7_W<'a> {
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
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm2(&self) -> PWM2_R {
        PWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm3(&self) -> PWM3_R {
        PWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm4(&self) -> PWM4_R {
        PWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm5(&self) -> PWM5_R {
        PWM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm6(&self) -> PWM6_R {
        PWM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm7(&self) -> PWM7_R {
        PWM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm0(&mut self) -> PWM0_W {
        PWM0_W { w: self }
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm1(&mut self) -> PWM1_W {
        PWM1_W { w: self }
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm2(&mut self) -> PWM2_W {
        PWM2_W { w: self }
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm3(&mut self) -> PWM3_W {
        PWM3_W { w: self }
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm4(&mut self) -> PWM4_W {
        PWM4_W { w: self }
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm5(&mut self) -> PWM5_W {
        PWM5_W { w: self }
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm6(&mut self) -> PWM6_W {
        PWM6_W { w: self }
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm7(&mut self) -> PWM7_W {
        PWM7_W { w: self }
    }
}
