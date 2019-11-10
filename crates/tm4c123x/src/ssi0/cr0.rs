#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SSI Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_A {
    #[doc = "3: 4-bit data"]
    _4,
    #[doc = "4: 5-bit data"]
    _5,
    #[doc = "5: 6-bit data"]
    _6,
    #[doc = "6: 7-bit data"]
    _7,
    #[doc = "7: 8-bit data"]
    _8,
    #[doc = "8: 9-bit data"]
    _9,
    #[doc = "9: 10-bit data"]
    _10,
    #[doc = "10: 11-bit data"]
    _11,
    #[doc = "11: 12-bit data"]
    _12,
    #[doc = "12: 13-bit data"]
    _13,
    #[doc = "13: 14-bit data"]
    _14,
    #[doc = "14: 15-bit data"]
    _15,
    #[doc = "15: 16-bit data"]
    _16,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        match variant {
            DSS_A::_4 => 3,
            DSS_A::_5 => 4,
            DSS_A::_6 => 5,
            DSS_A::_7 => 6,
            DSS_A::_8 => 7,
            DSS_A::_9 => 8,
            DSS_A::_10 => 9,
            DSS_A::_11 => 10,
            DSS_A::_12 => 11,
            DSS_A::_13 => 12,
            DSS_A::_14 => 13,
            DSS_A::_15 => 14,
            DSS_A::_16 => 15,
        }
    }
}
#[doc = "Reader of field `DSS`"]
pub type DSS_R = crate::R<u8, DSS_A>;
impl DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DSS_A::_4),
            4 => Val(DSS_A::_5),
            5 => Val(DSS_A::_6),
            6 => Val(DSS_A::_7),
            7 => Val(DSS_A::_8),
            8 => Val(DSS_A::_9),
            9 => Val(DSS_A::_10),
            10 => Val(DSS_A::_11),
            11 => Val(DSS_A::_12),
            12 => Val(DSS_A::_13),
            13 => Val(DSS_A::_14),
            14 => Val(DSS_A::_15),
            15 => Val(DSS_A::_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DSS_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == DSS_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == DSS_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == DSS_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DSS_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == DSS_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DSS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DSS_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == DSS_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == DSS_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == DSS_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == DSS_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DSS_A::_16
    }
}
#[doc = "Write proxy for field `DSS`"]
pub struct DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DSS_A::_4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(DSS_A::_5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(DSS_A::_6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(DSS_A::_7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DSS_A::_8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(DSS_A::_9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DSS_A::_10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DSS_A::_11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(DSS_A::_12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(DSS_A::_13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(DSS_A::_14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(DSS_A::_15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DSS_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "SSI Frame Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: Freescale SPI Frame Format"]
    MOTO,
    #[doc = "1: Synchronous Serial Frame Format"]
    TI,
    #[doc = "2: MICROWIRE Frame Format"]
    NMW,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        match variant {
            FRF_A::MOTO => 0,
            FRF_A::TI => 1,
            FRF_A::NMW => 2,
        }
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<u8, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRF_A::MOTO),
            1 => Val(FRF_A::TI),
            2 => Val(FRF_A::NMW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOTO`"]
    #[inline(always)]
    pub fn is_moto(&self) -> bool {
        *self == FRF_A::MOTO
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::TI
    }
    #[doc = "Checks if the value of the field is `NMW`"]
    #[inline(always)]
    pub fn is_nmw(&self) -> bool {
        *self == FRF_A::NMW
    }
}
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn moto(self) -> &'a mut W {
        self.variant(FRF_A::MOTO)
    }
    #[doc = "Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
    }
    #[doc = "MICROWIRE Frame Format"]
    #[inline(always)]
    pub fn nmw(self) -> &'a mut W {
        self.variant(FRF_A::NMW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPO`"]
pub type SPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPO`"]
pub struct SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPH`"]
pub type SPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPH`"]
pub struct SPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCR`"]
pub type SCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR`"]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn dss(&mut self) -> DSS_W {
        DSS_W { w: self }
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W {
        SPO_W { w: self }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn sph(&mut self) -> SPH_W {
        SPH_W { w: self }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
}
