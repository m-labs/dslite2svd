#[doc = "Reader of register EMUX"]
pub type R = crate::R<u32, super::EMUX>;
#[doc = "Writer for register EMUX"]
pub type W = crate::W<u32, super::EMUX>;
#[doc = "Register EMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SS0 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM0_A {
    #[doc = "0: Processor (default)"]
    PROCESSOR,
    #[doc = "1: Analog Comparator 0"]
    COMP0,
    #[doc = "2: Analog Comparator 1"]
    COMP1,
    #[doc = "4: External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "5: Timer"]
    TIMER,
    #[doc = "6: PWM generator 0"]
    PWM0,
    #[doc = "7: PWM generator 1"]
    PWM1,
    #[doc = "8: PWM generator 2"]
    PWM2,
    #[doc = "9: PWM generator 3"]
    PWM3,
    #[doc = "15: Always (continuously sample)"]
    ALWAYS,
}
impl From<EM0_A> for u8 {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        match variant {
            EM0_A::PROCESSOR => 0,
            EM0_A::COMP0 => 1,
            EM0_A::COMP1 => 2,
            EM0_A::EXTERNAL => 4,
            EM0_A::TIMER => 5,
            EM0_A::PWM0 => 6,
            EM0_A::PWM1 => 7,
            EM0_A::PWM2 => 8,
            EM0_A::PWM3 => 9,
            EM0_A::ALWAYS => 15,
        }
    }
}
#[doc = "Reader of field `EM0`"]
pub type EM0_R = crate::R<u8, EM0_A>;
impl EM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM0_A::PROCESSOR),
            1 => Val(EM0_A::COMP0),
            2 => Val(EM0_A::COMP1),
            4 => Val(EM0_A::EXTERNAL),
            5 => Val(EM0_A::TIMER),
            6 => Val(EM0_A::PWM0),
            7 => Val(EM0_A::PWM1),
            8 => Val(EM0_A::PWM2),
            9 => Val(EM0_A::PWM3),
            15 => Val(EM0_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline(always)]
    pub fn is_processor(&self) -> bool {
        *self == EM0_A::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EM0_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EM0_A::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == EM0_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM0_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM0_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM0_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM0_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM0_A::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == EM0_A::ALWAYS
    }
}
#[doc = "Write proxy for field `EM0`"]
pub struct EM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM0_A::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM0_A::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM0_A::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(EM0_A::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM0_A::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM0_A::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM0_A::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM0_A::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM0_A::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(EM0_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "SS1 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM1_A {
    #[doc = "0: Processor (default)"]
    PROCESSOR,
    #[doc = "1: Analog Comparator 0"]
    COMP0,
    #[doc = "2: Analog Comparator 1"]
    COMP1,
    #[doc = "4: External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "5: Timer"]
    TIMER,
    #[doc = "6: PWM generator 0"]
    PWM0,
    #[doc = "7: PWM generator 1"]
    PWM1,
    #[doc = "8: PWM generator 2"]
    PWM2,
    #[doc = "9: PWM generator 3"]
    PWM3,
    #[doc = "15: Always (continuously sample)"]
    ALWAYS,
}
impl From<EM1_A> for u8 {
    #[inline(always)]
    fn from(variant: EM1_A) -> Self {
        match variant {
            EM1_A::PROCESSOR => 0,
            EM1_A::COMP0 => 1,
            EM1_A::COMP1 => 2,
            EM1_A::EXTERNAL => 4,
            EM1_A::TIMER => 5,
            EM1_A::PWM0 => 6,
            EM1_A::PWM1 => 7,
            EM1_A::PWM2 => 8,
            EM1_A::PWM3 => 9,
            EM1_A::ALWAYS => 15,
        }
    }
}
#[doc = "Reader of field `EM1`"]
pub type EM1_R = crate::R<u8, EM1_A>;
impl EM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM1_A::PROCESSOR),
            1 => Val(EM1_A::COMP0),
            2 => Val(EM1_A::COMP1),
            4 => Val(EM1_A::EXTERNAL),
            5 => Val(EM1_A::TIMER),
            6 => Val(EM1_A::PWM0),
            7 => Val(EM1_A::PWM1),
            8 => Val(EM1_A::PWM2),
            9 => Val(EM1_A::PWM3),
            15 => Val(EM1_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline(always)]
    pub fn is_processor(&self) -> bool {
        *self == EM1_A::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EM1_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EM1_A::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == EM1_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM1_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM1_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM1_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM1_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM1_A::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == EM1_A::ALWAYS
    }
}
#[doc = "Write proxy for field `EM1`"]
pub struct EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM1_A::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM1_A::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM1_A::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(EM1_A::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM1_A::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM1_A::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM1_A::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM1_A::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM1_A::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(EM1_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "SS2 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM2_A {
    #[doc = "0: Processor (default)"]
    PROCESSOR,
    #[doc = "1: Analog Comparator 0"]
    COMP0,
    #[doc = "2: Analog Comparator 1"]
    COMP1,
    #[doc = "4: External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "5: Timer"]
    TIMER,
    #[doc = "6: PWM generator 0"]
    PWM0,
    #[doc = "7: PWM generator 1"]
    PWM1,
    #[doc = "8: PWM generator 2"]
    PWM2,
    #[doc = "9: PWM generator 3"]
    PWM3,
    #[doc = "15: Always (continuously sample)"]
    ALWAYS,
}
impl From<EM2_A> for u8 {
    #[inline(always)]
    fn from(variant: EM2_A) -> Self {
        match variant {
            EM2_A::PROCESSOR => 0,
            EM2_A::COMP0 => 1,
            EM2_A::COMP1 => 2,
            EM2_A::EXTERNAL => 4,
            EM2_A::TIMER => 5,
            EM2_A::PWM0 => 6,
            EM2_A::PWM1 => 7,
            EM2_A::PWM2 => 8,
            EM2_A::PWM3 => 9,
            EM2_A::ALWAYS => 15,
        }
    }
}
#[doc = "Reader of field `EM2`"]
pub type EM2_R = crate::R<u8, EM2_A>;
impl EM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM2_A::PROCESSOR),
            1 => Val(EM2_A::COMP0),
            2 => Val(EM2_A::COMP1),
            4 => Val(EM2_A::EXTERNAL),
            5 => Val(EM2_A::TIMER),
            6 => Val(EM2_A::PWM0),
            7 => Val(EM2_A::PWM1),
            8 => Val(EM2_A::PWM2),
            9 => Val(EM2_A::PWM3),
            15 => Val(EM2_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline(always)]
    pub fn is_processor(&self) -> bool {
        *self == EM2_A::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EM2_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EM2_A::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == EM2_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM2_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM2_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM2_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM2_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM2_A::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == EM2_A::ALWAYS
    }
}
#[doc = "Write proxy for field `EM2`"]
pub struct EM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM2_A::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM2_A::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM2_A::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(EM2_A::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM2_A::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM2_A::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM2_A::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM2_A::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM2_A::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(EM2_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "SS3 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM3_A {
    #[doc = "0: Processor (default)"]
    PROCESSOR,
    #[doc = "1: Analog Comparator 0"]
    COMP0,
    #[doc = "2: Analog Comparator 1"]
    COMP1,
    #[doc = "4: External (GPIO Pins)"]
    EXTERNAL,
    #[doc = "5: Timer"]
    TIMER,
    #[doc = "6: PWM generator 0"]
    PWM0,
    #[doc = "7: PWM generator 1"]
    PWM1,
    #[doc = "8: PWM generator 2"]
    PWM2,
    #[doc = "9: PWM generator 3"]
    PWM3,
    #[doc = "15: Always (continuously sample)"]
    ALWAYS,
}
impl From<EM3_A> for u8 {
    #[inline(always)]
    fn from(variant: EM3_A) -> Self {
        match variant {
            EM3_A::PROCESSOR => 0,
            EM3_A::COMP0 => 1,
            EM3_A::COMP1 => 2,
            EM3_A::EXTERNAL => 4,
            EM3_A::TIMER => 5,
            EM3_A::PWM0 => 6,
            EM3_A::PWM1 => 7,
            EM3_A::PWM2 => 8,
            EM3_A::PWM3 => 9,
            EM3_A::ALWAYS => 15,
        }
    }
}
#[doc = "Reader of field `EM3`"]
pub type EM3_R = crate::R<u8, EM3_A>;
impl EM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM3_A::PROCESSOR),
            1 => Val(EM3_A::COMP0),
            2 => Val(EM3_A::COMP1),
            4 => Val(EM3_A::EXTERNAL),
            5 => Val(EM3_A::TIMER),
            6 => Val(EM3_A::PWM0),
            7 => Val(EM3_A::PWM1),
            8 => Val(EM3_A::PWM2),
            9 => Val(EM3_A::PWM3),
            15 => Val(EM3_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROCESSOR`"]
    #[inline(always)]
    pub fn is_processor(&self) -> bool {
        *self == EM3_A::PROCESSOR
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EM3_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EM3_A::COMP1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == EM3_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == EM3_A::TIMER
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == EM3_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == EM3_A::PWM1
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == EM3_A::PWM2
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == EM3_A::PWM3
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == EM3_A::ALWAYS
    }
}
#[doc = "Write proxy for field `EM3`"]
pub struct EM3_W<'a> {
    w: &'a mut W,
}
impl<'a> EM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn processor(self) -> &'a mut W {
        self.variant(EM3_A::PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(EM3_A::COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(EM3_A::COMP1)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(EM3_A::EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(EM3_A::TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(EM3_A::PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(EM3_A::PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(EM3_A::PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(EM3_A::PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(EM3_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W { w: self }
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W { w: self }
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W { w: self }
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W {
        EM3_W { w: self }
    }
}
