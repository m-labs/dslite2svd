#[doc = "Reader of register TSSEL"]
pub type R = crate::R<u32, super::TSSEL>;
#[doc = "Writer for register TSSEL"]
pub type W = crate::W<u32, super::TSSEL>;
#[doc = "Register TSSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TSSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generator 0 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS0_A {
    #[doc = "0: Use Generator 0 (and its trigger) in PWM module 0"]
    _0,
    #[doc = "1: Use Generator 0 (and its trigger) in PWM module 1"]
    _1,
}
impl From<PS0_A> for u8 {
    #[inline(always)]
    fn from(variant: PS0_A) -> Self {
        match variant {
            PS0_A::_0 => 0,
            PS0_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `PS0`"]
pub type PS0_R = crate::R<u8, PS0_A>;
impl PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PS0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PS0_A::_0),
            1 => Val(PS0_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS0_A::_1
    }
}
#[doc = "Write proxy for field `PS0`"]
pub struct PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS0_A::_0)
    }
    #[doc = "Use Generator 0 (and its trigger) in PWM module 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS0_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Generator 1 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS1_A {
    #[doc = "0: Use Generator 1 (and its trigger) in PWM module 0"]
    _0,
    #[doc = "1: Use Generator 1 (and its trigger) in PWM module 1"]
    _1,
}
impl From<PS1_A> for u8 {
    #[inline(always)]
    fn from(variant: PS1_A) -> Self {
        match variant {
            PS1_A::_0 => 0,
            PS1_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `PS1`"]
pub type PS1_R = crate::R<u8, PS1_A>;
impl PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PS1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PS1_A::_0),
            1 => Val(PS1_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS1_A::_1
    }
}
#[doc = "Write proxy for field `PS1`"]
pub struct PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS1_A::_0)
    }
    #[doc = "Use Generator 1 (and its trigger) in PWM module 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS1_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Generator 2 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2_A {
    #[doc = "0: Use Generator 2 (and its trigger) in PWM module 0"]
    _0,
    #[doc = "1: Use Generator 2 (and its trigger) in PWM module 1"]
    _1,
}
impl From<PS2_A> for u8 {
    #[inline(always)]
    fn from(variant: PS2_A) -> Self {
        match variant {
            PS2_A::_0 => 0,
            PS2_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `PS2`"]
pub type PS2_R = crate::R<u8, PS2_A>;
impl PS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PS2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PS2_A::_0),
            1 => Val(PS2_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS2_A::_1
    }
}
#[doc = "Write proxy for field `PS2`"]
pub struct PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS2_A::_0)
    }
    #[doc = "Use Generator 2 (and its trigger) in PWM module 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS2_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Generator 3 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS3_A {
    #[doc = "0: Use Generator 3 (and its trigger) in PWM module 0"]
    _0,
    #[doc = "1: Use Generator 3 (and its trigger) in PWM module 1"]
    _1,
}
impl From<PS3_A> for u8 {
    #[inline(always)]
    fn from(variant: PS3_A) -> Self {
        match variant {
            PS3_A::_0 => 0,
            PS3_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `PS3`"]
pub type PS3_R = crate::R<u8, PS3_A>;
impl PS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PS3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PS3_A::_0),
            1 => Val(PS3_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS3_A::_1
    }
}
#[doc = "Write proxy for field `PS3`"]
pub struct PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS3_A::_0)
    }
    #[doc = "Use Generator 3 (and its trigger) in PWM module 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS3_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps0(&self) -> PS0_R {
        PS0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps1(&self) -> PS1_R {
        PS1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps2(&self) -> PS2_R {
        PS2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps3(&self) -> PS3_R {
        PS3_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps0(&mut self) -> PS0_W {
        PS0_W { w: self }
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps1(&mut self) -> PS1_W {
        PS1_W { w: self }
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps2(&mut self) -> PS2_W {
        PS2_W { w: self }
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn ps3(&mut self) -> PS3_W {
        PS3_W { w: self }
    }
}
