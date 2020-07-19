#[doc = "Reader of register GPCS"]
pub type R = crate::R<u32, super::GPCS>;
#[doc = "Writer for register GPCS"]
pub type W = crate::W<u32, super::GPCS>;
#[doc = "Register GPCS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVMOD_A {
    #[doc = "0: Use USB0VBUS and USB0ID pin"]
    OTG = 0,
    #[doc = "2: Force USB0VBUS and USB0ID low"]
    HOST = 2,
    #[doc = "3: Force USB0VBUS and USB0ID high"]
    DEV = 3,
    #[doc = "4: Use USB0VBUS and force USB0ID low"]
    HOSTVBUS = 4,
    #[doc = "5: Use USB0VBUS and force USB0ID high"]
    DEVVBUS = 5,
}
impl From<DEVMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEVMOD`"]
pub type DEVMOD_R = crate::R<u8, DEVMOD_A>;
impl DEVMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEVMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEVMOD_A::OTG),
            2 => Val(DEVMOD_A::HOST),
            3 => Val(DEVMOD_A::DEV),
            4 => Val(DEVMOD_A::HOSTVBUS),
            5 => Val(DEVMOD_A::DEVVBUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline(always)]
    pub fn is_otg(&self) -> bool {
        *self == DEVMOD_A::OTG
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == DEVMOD_A::HOST
    }
    #[doc = "Checks if the value of the field is `DEV`"]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == DEVMOD_A::DEV
    }
    #[doc = "Checks if the value of the field is `HOSTVBUS`"]
    #[inline(always)]
    pub fn is_hostvbus(&self) -> bool {
        *self == DEVMOD_A::HOSTVBUS
    }
    #[doc = "Checks if the value of the field is `DEVVBUS`"]
    #[inline(always)]
    pub fn is_devvbus(&self) -> bool {
        *self == DEVMOD_A::DEVVBUS
    }
}
#[doc = "Write proxy for field `DEVMOD`"]
pub struct DEVMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use USB0VBUS and USB0ID pin"]
    #[inline(always)]
    pub fn otg(self) -> &'a mut W {
        self.variant(DEVMOD_A::OTG)
    }
    #[doc = "Force USB0VBUS and USB0ID low"]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(DEVMOD_A::HOST)
    }
    #[doc = "Force USB0VBUS and USB0ID high"]
    #[inline(always)]
    pub fn dev(self) -> &'a mut W {
        self.variant(DEVMOD_A::DEV)
    }
    #[doc = "Use USB0VBUS and force USB0ID low"]
    #[inline(always)]
    pub fn hostvbus(self) -> &'a mut W {
        self.variant(DEVMOD_A::HOSTVBUS)
    }
    #[doc = "Use USB0VBUS and force USB0ID high"]
    #[inline(always)]
    pub fn devvbus(self) -> &'a mut W {
        self.variant(DEVMOD_A::DEVVBUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn devmod(&self) -> DEVMOD_R {
        DEVMOD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn devmod(&mut self) -> DEVMOD_W {
        DEVMOD_W { w: self }
    }
}
