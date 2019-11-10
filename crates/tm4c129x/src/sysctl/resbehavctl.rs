#[doc = "Reader of register RESBEHAVCTL"]
pub type R = crate::R<u32, super::RESBEHAVCTL>;
#[doc = "Writer for register RESBEHAVCTL"]
pub type W = crate::W<u32, super::RESBEHAVCTL>;
#[doc = "Register RESBEHAVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RESBEHAVCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External RST Pin Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRES_A {
    #[doc = "2: External RST assertion issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "3: External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl From<EXTRES_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTRES_A) -> Self {
        match variant {
            EXTRES_A::SYSRST => 2,
            EXTRES_A::POR => 3,
        }
    }
}
#[doc = "Reader of field `EXTRES`"]
pub type EXTRES_R = crate::R<u8, EXTRES_A>;
impl EXTRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTRES_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(EXTRES_A::SYSRST),
            3 => Val(EXTRES_A::POR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline(always)]
    pub fn is_sysrst(&self) -> bool {
        *self == EXTRES_A::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == EXTRES_A::POR
    }
}
#[doc = "Write proxy for field `EXTRES`"]
pub struct EXTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTRES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(EXTRES_A::SYSRST)
    }
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(EXTRES_A::POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "BOR Reset operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOR_A {
    #[doc = "2: Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "3: Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl From<BOR_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_A) -> Self {
        match variant {
            BOR_A::SYSRST => 2,
            BOR_A::POR => 3,
        }
    }
}
#[doc = "Reader of field `BOR`"]
pub type BOR_R = crate::R<u8, BOR_A>;
impl BOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOR_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(BOR_A::SYSRST),
            3 => Val(BOR_A::POR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline(always)]
    pub fn is_sysrst(&self) -> bool {
        *self == BOR_A::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == BOR_A::POR
    }
}
#[doc = "Write proxy for field `BOR`"]
pub struct BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(BOR_A::SYSRST)
    }
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(BOR_A::POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Watchdog 0 Reset Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG0_A {
    #[doc = "2: Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "3: Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl From<WDOG0_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOG0_A) -> Self {
        match variant {
            WDOG0_A::SYSRST => 2,
            WDOG0_A::POR => 3,
        }
    }
}
#[doc = "Reader of field `WDOG0`"]
pub type WDOG0_R = crate::R<u8, WDOG0_A>;
impl WDOG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WDOG0_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(WDOG0_A::SYSRST),
            3 => Val(WDOG0_A::POR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline(always)]
    pub fn is_sysrst(&self) -> bool {
        *self == WDOG0_A::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == WDOG0_A::POR
    }
}
#[doc = "Write proxy for field `WDOG0`"]
pub struct WDOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(WDOG0_A::SYSRST)
    }
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(WDOG0_A::POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Watchdog 1 Reset Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG1_A {
    #[doc = "2: Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "3: Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl From<WDOG1_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOG1_A) -> Self {
        match variant {
            WDOG1_A::SYSRST => 2,
            WDOG1_A::POR => 3,
        }
    }
}
#[doc = "Reader of field `WDOG1`"]
pub type WDOG1_R = crate::R<u8, WDOG1_A>;
impl WDOG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WDOG1_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(WDOG1_A::SYSRST),
            3 => Val(WDOG1_A::POR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline(always)]
    pub fn is_sysrst(&self) -> bool {
        *self == WDOG1_A::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == WDOG1_A::POR
    }
}
#[doc = "Write proxy for field `WDOG1`"]
pub struct WDOG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(WDOG1_A::SYSRST)
    }
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(WDOG1_A::POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn extres(&self) -> EXTRES_R {
        EXTRES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn bor(&self) -> BOR_R {
        BOR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn extres(&mut self) -> EXTRES_W {
        EXTRES_W { w: self }
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn bor(&mut self) -> BOR_W {
        BOR_W { w: self }
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W { w: self }
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> WDOG1_W {
        WDOG1_W { w: self }
    }
}
