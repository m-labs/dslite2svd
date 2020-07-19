#[doc = "Reader of register ENUPD"]
pub type R = crate::R<u32, super::ENUPD>;
#[doc = "Writer for register ENUPD"]
pub type W = crate::W<u32, super::ENUPD>;
#[doc = "Register ENUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::ENUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MnPWM0 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD0_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD0`"]
pub type ENUPD0_R = crate::R<u8, ENUPD0_A>;
impl ENUPD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD0_A::IMM),
            2 => Val(ENUPD0_A::LSYNC),
            3 => Val(ENUPD0_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD0_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD0_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD0_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD0`"]
pub struct ENUPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD0_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD0_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD0_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "MnPWM1 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD1_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD1_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD1`"]
pub type ENUPD1_R = crate::R<u8, ENUPD1_A>;
impl ENUPD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD1_A::IMM),
            2 => Val(ENUPD1_A::LSYNC),
            3 => Val(ENUPD1_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD1_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD1_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD1_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD1`"]
pub struct ENUPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD1_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD1_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD1_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "MnPWM2 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD2_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD2_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD2`"]
pub type ENUPD2_R = crate::R<u8, ENUPD2_A>;
impl ENUPD2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD2_A::IMM),
            2 => Val(ENUPD2_A::LSYNC),
            3 => Val(ENUPD2_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD2_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD2_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD2_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD2`"]
pub struct ENUPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD2_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD2_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD2_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "MnPWM3 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD3_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD3_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD3`"]
pub type ENUPD3_R = crate::R<u8, ENUPD3_A>;
impl ENUPD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD3_A::IMM),
            2 => Val(ENUPD3_A::LSYNC),
            3 => Val(ENUPD3_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD3_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD3_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD3_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD3`"]
pub struct ENUPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD3_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD3_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD3_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "MnPWM4 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD4_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD4_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD4`"]
pub type ENUPD4_R = crate::R<u8, ENUPD4_A>;
impl ENUPD4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD4_A::IMM),
            2 => Val(ENUPD4_A::LSYNC),
            3 => Val(ENUPD4_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD4_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD4_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD4_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD4`"]
pub struct ENUPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD4_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD4_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD4_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "MnPWM5 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD5_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD5_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD5`"]
pub type ENUPD5_R = crate::R<u8, ENUPD5_A>;
impl ENUPD5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD5_A::IMM),
            2 => Val(ENUPD5_A::LSYNC),
            3 => Val(ENUPD5_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD5_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD5_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD5_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD5`"]
pub struct ENUPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD5_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD5_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD5_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "MnPWM6 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD6_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD6_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD6`"]
pub type ENUPD6_R = crate::R<u8, ENUPD6_A>;
impl ENUPD6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD6_A::IMM),
            2 => Val(ENUPD6_A::LSYNC),
            3 => Val(ENUPD6_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD6_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD6_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD6_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD6`"]
pub struct ENUPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD6_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD6_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD6_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "MnPWM7 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUPD7_A {
    #[doc = "0: Immediate"]
    IMM = 0,
    #[doc = "2: Locally Synchronized"]
    LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    GSYNC = 3,
}
impl From<ENUPD7_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUPD7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENUPD7`"]
pub type ENUPD7_R = crate::R<u8, ENUPD7_A>;
impl ENUPD7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENUPD7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENUPD7_A::IMM),
            2 => Val(ENUPD7_A::LSYNC),
            3 => Val(ENUPD7_A::GSYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMM`"]
    #[inline(always)]
    pub fn is_imm(&self) -> bool {
        *self == ENUPD7_A::IMM
    }
    #[doc = "Checks if the value of the field is `LSYNC`"]
    #[inline(always)]
    pub fn is_lsync(&self) -> bool {
        *self == ENUPD7_A::LSYNC
    }
    #[doc = "Checks if the value of the field is `GSYNC`"]
    #[inline(always)]
    pub fn is_gsync(&self) -> bool {
        *self == ENUPD7_A::GSYNC
    }
}
#[doc = "Write proxy for field `ENUPD7`"]
pub struct ENUPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUPD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENUPD7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn imm(self) -> &'a mut W {
        self.variant(ENUPD7_A::IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn lsync(self) -> &'a mut W {
        self.variant(ENUPD7_A::LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn gsync(self) -> &'a mut W {
        self.variant(ENUPD7_A::GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd0(&self) -> ENUPD0_R {
        ENUPD0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd1(&self) -> ENUPD1_R {
        ENUPD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd2(&self) -> ENUPD2_R {
        ENUPD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd3(&self) -> ENUPD3_R {
        ENUPD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd4(&self) -> ENUPD4_R {
        ENUPD4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd5(&self) -> ENUPD5_R {
        ENUPD5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd6(&self) -> ENUPD6_R {
        ENUPD6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd7(&self) -> ENUPD7_R {
        ENUPD7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd0(&mut self) -> ENUPD0_W {
        ENUPD0_W { w: self }
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd1(&mut self) -> ENUPD1_W {
        ENUPD1_W { w: self }
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd2(&mut self) -> ENUPD2_W {
        ENUPD2_W { w: self }
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd3(&mut self) -> ENUPD3_W {
        ENUPD3_W { w: self }
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd4(&mut self) -> ENUPD4_W {
        ENUPD4_W { w: self }
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd5(&mut self) -> ENUPD5_W {
        ENUPD5_W { w: self }
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd6(&mut self) -> ENUPD6_W {
        ENUPD6_W { w: self }
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn enupd7(&mut self) -> ENUPD7_W {
        ENUPD7_W { w: self }
    }
}
