#[doc = "Reader of register SSTSH0"]
pub type R = crate::R<u32, super::SSTSH0>;
#[doc = "Writer for register SSTSH0"]
pub type W = crate::W<u32, super::SSTSH0>;
#[doc = "Register SSTSH0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSTSH0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1st Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH0_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH0_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH0`"]
pub type TSH0_R = crate::R<u8, TSH0_A>;
impl TSH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH0_A::_4),
            2 => Val(TSH0_A::_8),
            4 => Val(TSH0_A::_16),
            6 => Val(TSH0_A::_32),
            8 => Val(TSH0_A::_64),
            10 => Val(TSH0_A::_128),
            12 => Val(TSH0_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH0_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH0_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH0_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH0_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH0_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH0_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH0_A::_256
    }
}
#[doc = "Write proxy for field `TSH0`"]
pub struct TSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH0_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH0_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH0_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH0_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH0_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH0_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH0_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "2nd Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH1_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH1`"]
pub type TSH1_R = crate::R<u8, TSH1_A>;
impl TSH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH1_A::_4),
            2 => Val(TSH1_A::_8),
            4 => Val(TSH1_A::_16),
            6 => Val(TSH1_A::_32),
            8 => Val(TSH1_A::_64),
            10 => Val(TSH1_A::_128),
            12 => Val(TSH1_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH1_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH1_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH1_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH1_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH1_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH1_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH1_A::_256
    }
}
#[doc = "Write proxy for field `TSH1`"]
pub struct TSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH1_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH1_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH1_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH1_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH1_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH1_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH1_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "3rd Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH2_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH2`"]
pub type TSH2_R = crate::R<u8, TSH2_A>;
impl TSH2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH2_A::_4),
            2 => Val(TSH2_A::_8),
            4 => Val(TSH2_A::_16),
            6 => Val(TSH2_A::_32),
            8 => Val(TSH2_A::_64),
            10 => Val(TSH2_A::_128),
            12 => Val(TSH2_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH2_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH2_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH2_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH2_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH2_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH2_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH2_A::_256
    }
}
#[doc = "Write proxy for field `TSH2`"]
pub struct TSH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH2_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH2_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH2_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH2_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH2_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH2_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH2_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "4th Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH3_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH3_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH3`"]
pub type TSH3_R = crate::R<u8, TSH3_A>;
impl TSH3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH3_A::_4),
            2 => Val(TSH3_A::_8),
            4 => Val(TSH3_A::_16),
            6 => Val(TSH3_A::_32),
            8 => Val(TSH3_A::_64),
            10 => Val(TSH3_A::_128),
            12 => Val(TSH3_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH3_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH3_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH3_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH3_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH3_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH3_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH3_A::_256
    }
}
#[doc = "Write proxy for field `TSH3`"]
pub struct TSH3_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH3_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH3_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH3_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH3_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH3_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH3_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH3_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "5th Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH4_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH4_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH4`"]
pub type TSH4_R = crate::R<u8, TSH4_A>;
impl TSH4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH4_A::_4),
            2 => Val(TSH4_A::_8),
            4 => Val(TSH4_A::_16),
            6 => Val(TSH4_A::_32),
            8 => Val(TSH4_A::_64),
            10 => Val(TSH4_A::_128),
            12 => Val(TSH4_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH4_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH4_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH4_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH4_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH4_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH4_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH4_A::_256
    }
}
#[doc = "Write proxy for field `TSH4`"]
pub struct TSH4_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH4_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH4_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH4_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH4_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH4_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH4_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH4_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "6th Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH5_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH5_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH5`"]
pub type TSH5_R = crate::R<u8, TSH5_A>;
impl TSH5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH5_A::_4),
            2 => Val(TSH5_A::_8),
            4 => Val(TSH5_A::_16),
            6 => Val(TSH5_A::_32),
            8 => Val(TSH5_A::_64),
            10 => Val(TSH5_A::_128),
            12 => Val(TSH5_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH5_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH5_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH5_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH5_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH5_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH5_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH5_A::_256
    }
}
#[doc = "Write proxy for field `TSH5`"]
pub struct TSH5_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH5_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH5_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH5_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH5_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH5_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH5_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH5_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "7th Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH6_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH6_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH6`"]
pub type TSH6_R = crate::R<u8, TSH6_A>;
impl TSH6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH6_A::_4),
            2 => Val(TSH6_A::_8),
            4 => Val(TSH6_A::_16),
            6 => Val(TSH6_A::_32),
            8 => Val(TSH6_A::_64),
            10 => Val(TSH6_A::_128),
            12 => Val(TSH6_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH6_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH6_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH6_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH6_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH6_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH6_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH6_A::_256
    }
}
#[doc = "Write proxy for field `TSH6`"]
pub struct TSH6_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH6_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH6_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH6_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH6_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH6_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH6_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH6_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "8th Sample and Hold Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSH7_A {
    #[doc = "0: N_SH = 4"]
    _4 = 0,
    #[doc = "2: N_SH = 8"]
    _8 = 2,
    #[doc = "4: N_SH = 16"]
    _16 = 4,
    #[doc = "6: N_SH = 32"]
    _32 = 6,
    #[doc = "8: N_SH = 64"]
    _64 = 8,
    #[doc = "10: N_SH = 128"]
    _128 = 10,
    #[doc = "12: N_SH = 256"]
    _256 = 12,
}
impl From<TSH7_A> for u8 {
    #[inline(always)]
    fn from(variant: TSH7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSH7`"]
pub type TSH7_R = crate::R<u8, TSH7_A>;
impl TSH7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSH7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSH7_A::_4),
            2 => Val(TSH7_A::_8),
            4 => Val(TSH7_A::_16),
            6 => Val(TSH7_A::_32),
            8 => Val(TSH7_A::_64),
            10 => Val(TSH7_A::_128),
            12 => Val(TSH7_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TSH7_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TSH7_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TSH7_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TSH7_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TSH7_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TSH7_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TSH7_A::_256
    }
}
#[doc = "Write proxy for field `TSH7`"]
pub struct TSH7_W<'a> {
    w: &'a mut W,
}
impl<'a> TSH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSH7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "N_SH = 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH7_A::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH7_A::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH7_A::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH7_A::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH7_A::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH7_A::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH7_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&self) -> TSH0_R {
        TSH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh1(&self) -> TSH1_R {
        TSH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh2(&self) -> TSH2_R {
        TSH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh3(&self) -> TSH3_R {
        TSH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh4(&self) -> TSH4_R {
        TSH4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh5(&self) -> TSH5_R {
        TSH5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh6(&self) -> TSH6_R {
        TSH6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh7(&self) -> TSH7_R {
        TSH7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh0(&mut self) -> TSH0_W {
        TSH0_W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh1(&mut self) -> TSH1_W {
        TSH1_W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh2(&mut self) -> TSH2_W {
        TSH2_W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh3(&mut self) -> TSH3_W {
        TSH3_W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh4(&mut self) -> TSH4_W {
        TSH4_W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh5(&mut self) -> TSH5_W {
        TSH5_W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh6(&mut self) -> TSH6_W {
        TSH6_W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn tsh7(&mut self) -> TSH7_W {
        TSH7_W { w: self }
    }
}
