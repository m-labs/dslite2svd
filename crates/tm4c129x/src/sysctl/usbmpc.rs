#[doc = "Reader of register USBMPC"]
pub type R = crate::R<u32, super::USBMPC>;
#[doc = "Writer for register USBMPC"]
pub type W = crate::W<u32, super::USBMPC>;
#[doc = "Register USBMPC `reset()`'s with value 0"]
impl crate::ResetValue for super::USBMPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Memory Array Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRCTL_A {
    #[doc = "0: Array OFF"]
    OFF = 0,
    #[doc = "1: SRAM Retention"]
    RETAIN = 1,
    #[doc = "3: Array On"]
    ON = 3,
}
impl From<PWRCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRCTL`"]
pub type PWRCTL_R = crate::R<u8, PWRCTL_A>;
impl PWRCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRCTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRCTL_A::OFF),
            1 => Val(PWRCTL_A::RETAIN),
            3 => Val(PWRCTL_A::ON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWRCTL_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETAIN`"]
    #[inline(always)]
    pub fn is_retain(&self) -> bool {
        *self == PWRCTL_A::RETAIN
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWRCTL_A::ON
    }
}
#[doc = "Write proxy for field `PWRCTL`"]
pub struct PWRCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRCTL_A::OFF)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub fn retain(self) -> &'a mut W {
        self.variant(PWRCTL_A::RETAIN)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRCTL_A::ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn pwrctl(&self) -> PWRCTL_R {
        PWRCTL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn pwrctl(&mut self) -> PWRCTL_W {
        PWRCTL_W { w: self }
    }
}
