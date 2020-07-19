#[doc = "Reader of register CRCCTRL"]
pub type R = crate::R<u32, super::CRCCTRL>;
#[doc = "Writer for register CRCCTRL"]
pub type W = crate::W<u32, super::CRCCTRL>;
#[doc = "Register CRCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Operation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Polynomial 0x8005"]
    P8055 = 0,
    #[doc = "1: Polynomial 0x1021"]
    P1021 = 1,
    #[doc = "2: Polynomial 0x4C11DB7"]
    P4C11DB7 = 2,
    #[doc = "3: Polynomial 0x1EDC6F41"]
    P1EDC6F41 = 3,
    #[doc = "8: TCP checksum"]
    TCPCHKSUM = 8,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPE_A::P8055),
            1 => Val(TYPE_A::P1021),
            2 => Val(TYPE_A::P4C11DB7),
            3 => Val(TYPE_A::P1EDC6F41),
            8 => Val(TYPE_A::TCPCHKSUM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P8055`"]
    #[inline(always)]
    pub fn is_p8055(&self) -> bool {
        *self == TYPE_A::P8055
    }
    #[doc = "Checks if the value of the field is `P1021`"]
    #[inline(always)]
    pub fn is_p1021(&self) -> bool {
        *self == TYPE_A::P1021
    }
    #[doc = "Checks if the value of the field is `P4C11DB7`"]
    #[inline(always)]
    pub fn is_p4c11db7(&self) -> bool {
        *self == TYPE_A::P4C11DB7
    }
    #[doc = "Checks if the value of the field is `P1EDC6F41`"]
    #[inline(always)]
    pub fn is_p1edc6f41(&self) -> bool {
        *self == TYPE_A::P1EDC6F41
    }
    #[doc = "Checks if the value of the field is `TCPCHKSUM`"]
    #[inline(always)]
    pub fn is_tcpchksum(&self) -> bool {
        *self == TYPE_A::TCPCHKSUM
    }
}
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Polynomial 0x8005"]
    #[inline(always)]
    pub fn p8055(self) -> &'a mut W {
        self.variant(TYPE_A::P8055)
    }
    #[doc = "Polynomial 0x1021"]
    #[inline(always)]
    pub fn p1021(self) -> &'a mut W {
        self.variant(TYPE_A::P1021)
    }
    #[doc = "Polynomial 0x4C11DB7"]
    #[inline(always)]
    pub fn p4c11db7(self) -> &'a mut W {
        self.variant(TYPE_A::P4C11DB7)
    }
    #[doc = "Polynomial 0x1EDC6F41"]
    #[inline(always)]
    pub fn p1edc6f41(self) -> &'a mut W {
        self.variant(TYPE_A::P1EDC6F41)
    }
    #[doc = "TCP checksum"]
    #[inline(always)]
    pub fn tcpchksum(self) -> &'a mut W {
        self.variant(TYPE_A::TCPCHKSUM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Endian Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENDIAN_A {
    #[doc = "0: Configuration unchanged. (B3, B2, B1, B0)"]
    SBHW = 0,
    #[doc = "1: Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    SHW = 1,
    #[doc = "2: Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    SHWNB = 2,
    #[doc = "3: Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    SBSW = 3,
}
impl From<ENDIAN_A> for u8 {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENDIAN`"]
pub type ENDIAN_R = crate::R<u8, ENDIAN_A>;
impl ENDIAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            0 => ENDIAN_A::SBHW,
            1 => ENDIAN_A::SHW,
            2 => ENDIAN_A::SHWNB,
            3 => ENDIAN_A::SBSW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SBHW`"]
    #[inline(always)]
    pub fn is_sbhw(&self) -> bool {
        *self == ENDIAN_A::SBHW
    }
    #[doc = "Checks if the value of the field is `SHW`"]
    #[inline(always)]
    pub fn is_shw(&self) -> bool {
        *self == ENDIAN_A::SHW
    }
    #[doc = "Checks if the value of the field is `SHWNB`"]
    #[inline(always)]
    pub fn is_shwnb(&self) -> bool {
        *self == ENDIAN_A::SHWNB
    }
    #[doc = "Checks if the value of the field is `SBSW`"]
    #[inline(always)]
    pub fn is_sbsw(&self) -> bool {
        *self == ENDIAN_A::SBSW
    }
}
#[doc = "Write proxy for field `ENDIAN`"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIAN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    #[inline(always)]
    pub fn sbhw(self) -> &'a mut W {
        self.variant(ENDIAN_A::SBHW)
    }
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    #[inline(always)]
    pub fn shw(self) -> &'a mut W {
        self.variant(ENDIAN_A::SHW)
    }
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    #[inline(always)]
    pub fn shwnb(self) -> &'a mut W {
        self.variant(ENDIAN_A::SHWNB)
    }
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    #[inline(always)]
    pub fn sbsw(self) -> &'a mut W {
        self.variant(ENDIAN_A::SBSW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `BR`"]
pub type BR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BR`"]
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
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
#[doc = "Reader of field `OBR`"]
pub type OBR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBR`"]
pub struct OBR_W<'a> {
    w: &'a mut W,
}
impl<'a> OBR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESINV`"]
pub type RESINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESINV`"]
pub struct RESINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RESINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "CRC Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INIT_A {
    #[doc = "0: Use the CRCSEED register context as the starting value"]
    SEED = 0,
    #[doc = "2: Initialize to all '0s'"]
    _0 = 2,
    #[doc = "3: Initialize to all '1s'"]
    _1 = 3,
}
impl From<INIT_A> for u8 {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<u8, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INIT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INIT_A::SEED),
            2 => Val(INIT_A::_0),
            3 => Val(INIT_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SEED`"]
    #[inline(always)]
    pub fn is_seed(&self) -> bool {
        *self == INIT_A::SEED
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIT_A::_1
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use the CRCSEED register context as the starting value"]
    #[inline(always)]
    pub fn seed(self) -> &'a mut W {
        self.variant(INIT_A::SEED)
    }
    #[doc = "Initialize to all '0s'"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INIT_A::_0)
    }
    #[doc = "Initialize to all '1s'"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INIT_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn obr(&self) -> OBR_R {
        OBR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn resinv(&self) -> RESINV_R {
        RESINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn obr(&mut self) -> OBR_W {
        OBR_W { w: self }
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn resinv(&mut self) -> RESINV_W {
        RESINV_W { w: self }
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
}
