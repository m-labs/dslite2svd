#[doc = "Reader of register USBPDS"]
pub type R = crate::R<u32, super::USBPDS>;
#[doc = "Power Domain Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRSTAT_A {
    #[doc = "0: OFF"]
    OFF = 0,
    #[doc = "3: ON"]
    ON = 3,
}
impl From<PWRSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRSTAT`"]
pub type PWRSTAT_R = crate::R<u8, PWRSTAT_A>;
impl PWRSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRSTAT_A::OFF),
            3 => Val(PWRSTAT_A::ON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWRSTAT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWRSTAT_A::ON
    }
}
#[doc = "Memory Array Power Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEMSTAT_A {
    #[doc = "0: Array OFF"]
    OFF = 0,
    #[doc = "1: SRAM Retention"]
    RETAIN = 1,
    #[doc = "3: Array On"]
    ON = 3,
}
impl From<MEMSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: MEMSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEMSTAT`"]
pub type MEMSTAT_R = crate::R<u8, MEMSTAT_A>;
impl MEMSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MEMSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MEMSTAT_A::OFF),
            1 => Val(MEMSTAT_A::RETAIN),
            3 => Val(MEMSTAT_A::ON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MEMSTAT_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETAIN`"]
    #[inline(always)]
    pub fn is_retain(&self) -> bool {
        *self == MEMSTAT_A::RETAIN
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == MEMSTAT_A::ON
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn pwrstat(&self) -> PWRSTAT_R {
        PWRSTAT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn memstat(&self) -> MEMSTAT_R {
        MEMSTAT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
