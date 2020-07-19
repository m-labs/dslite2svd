#[doc = "Reader of register _2_CTL"]
pub type R = crate::R<u32, super::_2_CTL>;
#[doc = "Writer for register _2_CTL"]
pub type W = crate::W<u32, super::_2_CTL>;
#[doc = "Register _2_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::_2_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Reader of field `DEBUG`"]
pub type DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUG`"]
pub struct DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_W<'a> {
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
#[doc = "Reader of field `LOADUPD`"]
pub type LOADUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOADUPD`"]
pub struct LOADUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADUPD_W<'a> {
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
#[doc = "Reader of field `CMPAUPD`"]
pub type CMPAUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPAUPD`"]
pub struct CMPAUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPAUPD_W<'a> {
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
#[doc = "Reader of field `CMPBUPD`"]
pub type CMPBUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPBUPD`"]
pub struct CMPBUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPBUPD_W<'a> {
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
#[doc = "PWMnGENA Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GENAUPD_A {
    #[doc = "0: Immediate"]
    I = 0,
    #[doc = "2: Locally Synchronized"]
    LS = 2,
    #[doc = "3: Globally Synchronized"]
    GS = 3,
}
impl From<GENAUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: GENAUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GENAUPD`"]
pub type GENAUPD_R = crate::R<u8, GENAUPD_A>;
impl GENAUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GENAUPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GENAUPD_A::I),
            2 => Val(GENAUPD_A::LS),
            3 => Val(GENAUPD_A::GS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == GENAUPD_A::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == GENAUPD_A::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline(always)]
    pub fn is_gs(&self) -> bool {
        *self == GENAUPD_A::GS
    }
}
#[doc = "Write proxy for field `GENAUPD`"]
pub struct GENAUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> GENAUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GENAUPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(GENAUPD_A::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(GENAUPD_A::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gs(self) -> &'a mut W {
        self.variant(GENAUPD_A::GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "PWMnGENB Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GENBUPD_A {
    #[doc = "0: Immediate"]
    I = 0,
    #[doc = "2: Locally Synchronized"]
    LS = 2,
    #[doc = "3: Globally Synchronized"]
    GS = 3,
}
impl From<GENBUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: GENBUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GENBUPD`"]
pub type GENBUPD_R = crate::R<u8, GENBUPD_A>;
impl GENBUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GENBUPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GENBUPD_A::I),
            2 => Val(GENBUPD_A::LS),
            3 => Val(GENBUPD_A::GS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == GENBUPD_A::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == GENBUPD_A::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline(always)]
    pub fn is_gs(&self) -> bool {
        *self == GENBUPD_A::GS
    }
}
#[doc = "Write proxy for field `GENBUPD`"]
pub struct GENBUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> GENBUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GENBUPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(GENBUPD_A::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(GENBUPD_A::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gs(self) -> &'a mut W {
        self.variant(GENBUPD_A::GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "PWMnDBCTL Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBCTLUPD_A {
    #[doc = "0: Immediate"]
    I = 0,
    #[doc = "2: Locally Synchronized"]
    LS = 2,
    #[doc = "3: Globally Synchronized"]
    GS = 3,
}
impl From<DBCTLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DBCTLUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBCTLUPD`"]
pub type DBCTLUPD_R = crate::R<u8, DBCTLUPD_A>;
impl DBCTLUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBCTLUPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBCTLUPD_A::I),
            2 => Val(DBCTLUPD_A::LS),
            3 => Val(DBCTLUPD_A::GS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == DBCTLUPD_A::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == DBCTLUPD_A::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline(always)]
    pub fn is_gs(&self) -> bool {
        *self == DBCTLUPD_A::GS
    }
}
#[doc = "Write proxy for field `DBCTLUPD`"]
pub struct DBCTLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCTLUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBCTLUPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(DBCTLUPD_A::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBCTLUPD_A::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBCTLUPD_A::GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "PWMnDBRISE Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBRISEUPD_A {
    #[doc = "0: Immediate"]
    I = 0,
    #[doc = "2: Locally Synchronized"]
    LS = 2,
    #[doc = "3: Globally Synchronized"]
    GS = 3,
}
impl From<DBRISEUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DBRISEUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBRISEUPD`"]
pub type DBRISEUPD_R = crate::R<u8, DBRISEUPD_A>;
impl DBRISEUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBRISEUPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBRISEUPD_A::I),
            2 => Val(DBRISEUPD_A::LS),
            3 => Val(DBRISEUPD_A::GS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == DBRISEUPD_A::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == DBRISEUPD_A::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline(always)]
    pub fn is_gs(&self) -> bool {
        *self == DBRISEUPD_A::GS
    }
}
#[doc = "Write proxy for field `DBRISEUPD`"]
pub struct DBRISEUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRISEUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBRISEUPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(DBRISEUPD_A::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBRISEUPD_A::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBRISEUPD_A::GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "PWMnDBFALL Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBFALLUPD_A {
    #[doc = "0: Immediate"]
    I = 0,
    #[doc = "2: Locally Synchronized"]
    LS = 2,
    #[doc = "3: Globally Synchronized"]
    GS = 3,
}
impl From<DBFALLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DBFALLUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBFALLUPD`"]
pub type DBFALLUPD_R = crate::R<u8, DBFALLUPD_A>;
impl DBFALLUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBFALLUPD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBFALLUPD_A::I),
            2 => Val(DBFALLUPD_A::LS),
            3 => Val(DBFALLUPD_A::GS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == DBFALLUPD_A::I
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == DBFALLUPD_A::LS
    }
    #[doc = "Checks if the value of the field is `GS`"]
    #[inline(always)]
    pub fn is_gs(&self) -> bool {
        *self == DBFALLUPD_A::GS
    }
}
#[doc = "Write proxy for field `DBFALLUPD`"]
pub struct DBFALLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBFALLUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBFALLUPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(DBFALLUPD_A::I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(DBFALLUPD_A::LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gs(self) -> &'a mut W {
        self.variant(DBFALLUPD_A::GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `FLTSRC`"]
pub type FLTSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLTSRC`"]
pub struct FLTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTSRC_W<'a> {
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
#[doc = "Reader of field `MINFLTPER`"]
pub type MINFLTPER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINFLTPER`"]
pub struct MINFLTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MINFLTPER_W<'a> {
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
#[doc = "Reader of field `LATCH`"]
pub type LATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATCH`"]
pub struct LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn debug(&self) -> DEBUG_R {
        DEBUG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn loadupd(&self) -> LOADUPD_R {
        LOADUPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn cmpaupd(&self) -> CMPAUPD_R {
        CMPAUPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn cmpbupd(&self) -> CMPBUPD_R {
        CMPBUPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn genaupd(&self) -> GENAUPD_R {
        GENAUPD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn genbupd(&self) -> GENBUPD_R {
        GENBUPD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn dbctlupd(&self) -> DBCTLUPD_R {
        DBCTLUPD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn dbriseupd(&self) -> DBRISEUPD_R {
        DBRISEUPD_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn dbfallupd(&self) -> DBFALLUPD_R {
        DBFALLUPD_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn fltsrc(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn minfltper(&self) -> MINFLTPER_R {
        MINFLTPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn latch(&self) -> LATCH_R {
        LATCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn debug(&mut self) -> DEBUG_W {
        DEBUG_W { w: self }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn loadupd(&mut self) -> LOADUPD_W {
        LOADUPD_W { w: self }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn cmpaupd(&mut self) -> CMPAUPD_W {
        CMPAUPD_W { w: self }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn cmpbupd(&mut self) -> CMPBUPD_W {
        CMPBUPD_W { w: self }
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn genaupd(&mut self) -> GENAUPD_W {
        GENAUPD_W { w: self }
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn genbupd(&mut self) -> GENBUPD_W {
        GENBUPD_W { w: self }
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn dbctlupd(&mut self) -> DBCTLUPD_W {
        DBCTLUPD_W { w: self }
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn dbriseupd(&mut self) -> DBRISEUPD_W {
        DBRISEUPD_W { w: self }
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn dbfallupd(&mut self) -> DBFALLUPD_W {
        DBFALLUPD_W { w: self }
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn fltsrc(&mut self) -> FLTSRC_W {
        FLTSRC_W { w: self }
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn minfltper(&mut self) -> MINFLTPER_W {
        MINFLTPER_W { w: self }
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn latch(&mut self) -> LATCH_W {
        LATCH_W { w: self }
    }
}
