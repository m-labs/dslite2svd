#[doc = "Reader of register LCRH"]
pub type R = crate::R<u32, super::LCRH>;
#[doc = "Writer for register LCRH"]
pub type W = crate::W<u32, super::LCRH>;
#[doc = "Register LCRH `reset()`'s with value 0"]
impl crate::ResetValue for super::LCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRK`"]
pub type BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRK`"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
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
#[doc = "Reader of field `PEN`"]
pub type PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEN`"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
#[doc = "Reader of field `EPS`"]
pub type EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPS`"]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
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
#[doc = "Reader of field `STP2`"]
pub type STP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STP2`"]
pub struct STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> STP2_W<'a> {
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
#[doc = "Reader of field `FEN`"]
pub type FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEN`"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
#[doc = "UART Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "0: 5 bits (default)"]
    _5 = 0,
    #[doc = "1: 6 bits"]
    _6 = 1,
    #[doc = "2: 7 bits"]
    _7 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLEN`"]
pub type WLEN_R = crate::R<u8, WLEN_A>;
impl WLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEN_A {
        match self.bits {
            0 => WLEN_A::_5,
            1 => WLEN_A::_6,
            2 => WLEN_A::_7,
            3 => WLEN_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == WLEN_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == WLEN_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == WLEN_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WLEN_A::_8
    }
}
#[doc = "Write proxy for field `WLEN`"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(WLEN_A::_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WLEN_A::_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(WLEN_A::_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WLEN_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPS`"]
pub type SPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPS`"]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W { w: self }
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
}
