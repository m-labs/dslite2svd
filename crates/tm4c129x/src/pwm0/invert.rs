#[doc = "Reader of register INVERT"]
pub type R = crate::R<u32, super::INVERT>;
#[doc = "Writer for register INVERT"]
pub type W = crate::W<u32, super::INVERT>;
#[doc = "Register INVERT `reset()`'s with value 0"]
impl crate::ResetValue for super::INVERT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM0INV`"]
pub type PWM0INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM0INV`"]
pub struct PWM0INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0INV_W<'a> {
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
#[doc = "Reader of field `PWM1INV`"]
pub type PWM1INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM1INV`"]
pub struct PWM1INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1INV_W<'a> {
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
#[doc = "Reader of field `PWM2INV`"]
pub type PWM2INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM2INV`"]
pub struct PWM2INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2INV_W<'a> {
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
#[doc = "Reader of field `PWM3INV`"]
pub type PWM3INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM3INV`"]
pub struct PWM3INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3INV_W<'a> {
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
#[doc = "Reader of field `PWM4INV`"]
pub type PWM4INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM4INV`"]
pub struct PWM4INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM4INV_W<'a> {
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
#[doc = "Reader of field `PWM5INV`"]
pub type PWM5INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM5INV`"]
pub struct PWM5INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM5INV_W<'a> {
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
#[doc = "Reader of field `PWM6INV`"]
pub type PWM6INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM6INV`"]
pub struct PWM6INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM6INV_W<'a> {
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
#[doc = "Reader of field `PWM7INV`"]
pub type PWM7INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM7INV`"]
pub struct PWM7INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM7INV_W<'a> {
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
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm0inv(&self) -> PWM0INV_R {
        PWM0INV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm1inv(&self) -> PWM1INV_R {
        PWM1INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm2inv(&self) -> PWM2INV_R {
        PWM2INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm3inv(&self) -> PWM3INV_R {
        PWM3INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm4inv(&self) -> PWM4INV_R {
        PWM4INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm5inv(&self) -> PWM5INV_R {
        PWM5INV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm6inv(&self) -> PWM6INV_R {
        PWM6INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm7inv(&self) -> PWM7INV_R {
        PWM7INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm0inv(&mut self) -> PWM0INV_W {
        PWM0INV_W { w: self }
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm1inv(&mut self) -> PWM1INV_W {
        PWM1INV_W { w: self }
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm2inv(&mut self) -> PWM2INV_W {
        PWM2INV_W { w: self }
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm3inv(&mut self) -> PWM3INV_W {
        PWM3INV_W { w: self }
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm4inv(&mut self) -> PWM4INV_W {
        PWM4INV_W { w: self }
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm5inv(&mut self) -> PWM5INV_W {
        PWM5INV_W { w: self }
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm6inv(&mut self) -> PWM6INV_W {
        PWM6INV_W { w: self }
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm7inv(&mut self) -> PWM7INV_W {
        PWM7INV_W { w: self }
    }
}
