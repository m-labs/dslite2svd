#[doc = "Reader of register MEMTIM0"]
pub type R = crate::R<u32, super::MEMTIM0>;
#[doc = "Writer for register MEMTIM0"]
pub type W = crate::W<u32, super::MEMTIM0>;
#[doc = "Register MEMTIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMTIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FWS`"]
pub type FWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWS`"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FBCE`"]
pub type FBCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBCE`"]
pub struct FBCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCE_W<'a> {
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
#[doc = "Flash Bank Clock High Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FBCHT_A {
    #[doc = "0: 1/2 system clock period"]
    _0_5 = 0,
    #[doc = "1: 1 system clock period"]
    _1 = 1,
    #[doc = "2: 1.5 system clock periods"]
    _1_5 = 2,
    #[doc = "3: 2 system clock periods"]
    _2 = 3,
    #[doc = "4: 2.5 system clock periods"]
    _2_5 = 4,
    #[doc = "5: 3 system clock periods"]
    _3 = 5,
    #[doc = "6: 3.5 system clock periods"]
    _3_5 = 6,
    #[doc = "7: 4 system clock periods"]
    _4 = 7,
    #[doc = "8: 4.5 system clock periods"]
    _4_5 = 8,
}
impl From<FBCHT_A> for u8 {
    #[inline(always)]
    fn from(variant: FBCHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FBCHT`"]
pub type FBCHT_R = crate::R<u8, FBCHT_A>;
impl FBCHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FBCHT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FBCHT_A::_0_5),
            1 => Val(FBCHT_A::_1),
            2 => Val(FBCHT_A::_1_5),
            3 => Val(FBCHT_A::_2),
            4 => Val(FBCHT_A::_2_5),
            5 => Val(FBCHT_A::_3),
            6 => Val(FBCHT_A::_3_5),
            7 => Val(FBCHT_A::_4),
            8 => Val(FBCHT_A::_4_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5`"]
    #[inline(always)]
    pub fn is_0_5(&self) -> bool {
        *self == FBCHT_A::_0_5
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FBCHT_A::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == FBCHT_A::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == FBCHT_A::_2
    }
    #[doc = "Checks if the value of the field is `_2_5`"]
    #[inline(always)]
    pub fn is_2_5(&self) -> bool {
        *self == FBCHT_A::_2_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == FBCHT_A::_3
    }
    #[doc = "Checks if the value of the field is `_3_5`"]
    #[inline(always)]
    pub fn is_3_5(&self) -> bool {
        *self == FBCHT_A::_3_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FBCHT_A::_4
    }
    #[doc = "Checks if the value of the field is `_4_5`"]
    #[inline(always)]
    pub fn is_4_5(&self) -> bool {
        *self == FBCHT_A::_4_5
    }
}
#[doc = "Write proxy for field `FBCHT`"]
pub struct FBCHT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBCHT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn _0_5(self) -> &'a mut W {
        self.variant(FBCHT_A::_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FBCHT_A::_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(FBCHT_A::_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FBCHT_A::_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn _2_5(self) -> &'a mut W {
        self.variant(FBCHT_A::_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FBCHT_A::_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn _3_5(self) -> &'a mut W {
        self.variant(FBCHT_A::_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FBCHT_A::_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn _4_5(self) -> &'a mut W {
        self.variant(FBCHT_A::_4_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `EWS`"]
pub type EWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EWS`"]
pub struct EWS_W<'a> {
    w: &'a mut W,
}
impl<'a> EWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EBCE`"]
pub type EBCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBCE`"]
pub struct EBCE_W<'a> {
    w: &'a mut W,
}
impl<'a> EBCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "EEPROM Clock High Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EBCHT_A {
    #[doc = "0: 1/2 system clock period"]
    _0_5 = 0,
    #[doc = "1: 1 system clock period"]
    _1 = 1,
    #[doc = "2: 1.5 system clock periods"]
    _1_5 = 2,
    #[doc = "3: 2 system clock periods"]
    _2 = 3,
    #[doc = "4: 2.5 system clock periods"]
    _2_5 = 4,
    #[doc = "5: 3 system clock periods"]
    _3 = 5,
    #[doc = "6: 3.5 system clock periods"]
    _3_5 = 6,
    #[doc = "7: 4 system clock periods"]
    _4 = 7,
    #[doc = "8: 4.5 system clock periods"]
    _4_5 = 8,
}
impl From<EBCHT_A> for u8 {
    #[inline(always)]
    fn from(variant: EBCHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EBCHT`"]
pub type EBCHT_R = crate::R<u8, EBCHT_A>;
impl EBCHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EBCHT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EBCHT_A::_0_5),
            1 => Val(EBCHT_A::_1),
            2 => Val(EBCHT_A::_1_5),
            3 => Val(EBCHT_A::_2),
            4 => Val(EBCHT_A::_2_5),
            5 => Val(EBCHT_A::_3),
            6 => Val(EBCHT_A::_3_5),
            7 => Val(EBCHT_A::_4),
            8 => Val(EBCHT_A::_4_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5`"]
    #[inline(always)]
    pub fn is_0_5(&self) -> bool {
        *self == EBCHT_A::_0_5
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EBCHT_A::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == EBCHT_A::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == EBCHT_A::_2
    }
    #[doc = "Checks if the value of the field is `_2_5`"]
    #[inline(always)]
    pub fn is_2_5(&self) -> bool {
        *self == EBCHT_A::_2_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == EBCHT_A::_3
    }
    #[doc = "Checks if the value of the field is `_3_5`"]
    #[inline(always)]
    pub fn is_3_5(&self) -> bool {
        *self == EBCHT_A::_3_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == EBCHT_A::_4
    }
    #[doc = "Checks if the value of the field is `_4_5`"]
    #[inline(always)]
    pub fn is_4_5(&self) -> bool {
        *self == EBCHT_A::_4_5
    }
}
#[doc = "Write proxy for field `EBCHT`"]
pub struct EBCHT_W<'a> {
    w: &'a mut W,
}
impl<'a> EBCHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBCHT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn _0_5(self) -> &'a mut W {
        self.variant(EBCHT_A::_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EBCHT_A::_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(EBCHT_A::_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(EBCHT_A::_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn _2_5(self) -> &'a mut W {
        self.variant(EBCHT_A::_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(EBCHT_A::_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn _3_5(self) -> &'a mut W {
        self.variant(EBCHT_A::_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(EBCHT_A::_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn _4_5(self) -> &'a mut W {
        self.variant(EBCHT_A::_4_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn fbce(&self) -> FBCE_R {
        FBCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn fbcht(&self) -> FBCHT_R {
        FBCHT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn ews(&self) -> EWS_R {
        EWS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn ebce(&self) -> EBCE_R {
        EBCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn ebcht(&self) -> EBCHT_R {
        EBCHT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn fbce(&mut self) -> FBCE_W {
        FBCE_W { w: self }
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn fbcht(&mut self) -> FBCHT_W {
        FBCHT_W { w: self }
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn ews(&mut self) -> EWS_W {
        EWS_W { w: self }
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn ebce(&mut self) -> EBCE_W {
        EBCE_W { w: self }
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn ebcht(&mut self) -> EBCHT_W {
        EBCHT_W { w: self }
    }
}
