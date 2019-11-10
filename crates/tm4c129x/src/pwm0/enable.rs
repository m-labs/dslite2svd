#[doc = "Reader of register ENABLE"]
pub type R = crate::R<u32, super::ENABLE>;
#[doc = "Writer for register ENABLE"]
pub type W = crate::W<u32, super::ENABLE>;
#[doc = "Register ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM0EN`"]
pub type PWM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM0EN`"]
pub struct PWM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0EN_W<'a> {
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
#[doc = "Reader of field `PWM1EN`"]
pub type PWM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM1EN`"]
pub struct PWM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1EN_W<'a> {
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
#[doc = "Reader of field `PWM2EN`"]
pub type PWM2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM2EN`"]
pub struct PWM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2EN_W<'a> {
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
#[doc = "Reader of field `PWM3EN`"]
pub type PWM3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM3EN`"]
pub struct PWM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3EN_W<'a> {
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
#[doc = "Reader of field `PWM4EN`"]
pub type PWM4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM4EN`"]
pub struct PWM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM4EN_W<'a> {
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
#[doc = "Reader of field `PWM5EN`"]
pub type PWM5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM5EN`"]
pub struct PWM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM5EN_W<'a> {
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
#[doc = "Reader of field `PWM6EN`"]
pub type PWM6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM6EN`"]
pub struct PWM6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM6EN_W<'a> {
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
#[doc = "Reader of field `PWM7EN`"]
pub type PWM7EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM7EN`"]
pub struct PWM7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM7EN_W<'a> {
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
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm0en(&self) -> PWM0EN_R {
        PWM0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm1en(&self) -> PWM1EN_R {
        PWM1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm2en(&self) -> PWM2EN_R {
        PWM2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm3en(&self) -> PWM3EN_R {
        PWM3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm4en(&self) -> PWM4EN_R {
        PWM4EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm5en(&self) -> PWM5EN_R {
        PWM5EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm6en(&self) -> PWM6EN_R {
        PWM6EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm7en(&self) -> PWM7EN_R {
        PWM7EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm0en(&mut self) -> PWM0EN_W {
        PWM0EN_W { w: self }
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm1en(&mut self) -> PWM1EN_W {
        PWM1EN_W { w: self }
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm2en(&mut self) -> PWM2EN_W {
        PWM2EN_W { w: self }
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm3en(&mut self) -> PWM3EN_W {
        PWM3EN_W { w: self }
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm4en(&mut self) -> PWM4EN_W {
        PWM4EN_W { w: self }
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm5en(&mut self) -> PWM5EN_W {
        PWM5EN_W { w: self }
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm6en(&mut self) -> PWM6EN_W {
        PWM6EN_W { w: self }
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm7en(&mut self) -> PWM7EN_W {
        PWM7EN_W { w: self }
    }
}
