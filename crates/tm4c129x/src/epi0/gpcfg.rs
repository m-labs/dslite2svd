#[doc = "Reader of register GPCFG"]
pub type R = crate::R<u32, super::GPCFG>;
#[doc = "Writer for register GPCFG"]
pub type W = crate::W<u32, super::GPCFG>;
#[doc = "Register GPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Size of Data Bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIZE_A {
    #[doc = "0: 8 Bits Wide (EPI0S0 to EPI0S7)"]
    _4BIT,
    #[doc = "1: 16 Bits Wide (EPI0S0 to EPI0S15)"]
    _16BIT,
    #[doc = "2: 24 Bits Wide (EPI0S0 to EPI0S23)"]
    _24BIT,
    #[doc = "3: 32 Bits Wide (EPI0S0 to EPI0S31)"]
    _32BIT,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        match variant {
            DSIZE_A::_4BIT => 0,
            DSIZE_A::_16BIT => 1,
            DSIZE_A::_24BIT => 2,
            DSIZE_A::_32BIT => 3,
        }
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, DSIZE_A>;
impl DSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIZE_A {
        match self.bits {
            0 => DSIZE_A::_4BIT,
            1 => DSIZE_A::_16BIT,
            2 => DSIZE_A::_24BIT,
            3 => DSIZE_A::_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == DSIZE_A::_4BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == DSIZE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == DSIZE_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == DSIZE_A::_32BIT
    }
}
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DSIZE_A::_4BIT)
    }
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(DSIZE_A::_16BIT)
    }
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(DSIZE_A::_24BIT)
    }
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(DSIZE_A::_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Address Bus Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASIZE_A {
    #[doc = "0: No address"]
    NONE,
    #[doc = "1: Up to 4 bits wide"]
    _4BIT,
    #[doc = "2: Up to 12 bits wide. This size cannot be used with 24-bit data"]
    _12BIT,
    #[doc = "3: Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    _20BIT,
}
impl From<ASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ASIZE_A) -> Self {
        match variant {
            ASIZE_A::NONE => 0,
            ASIZE_A::_4BIT => 1,
            ASIZE_A::_12BIT => 2,
            ASIZE_A::_20BIT => 3,
        }
    }
}
#[doc = "Reader of field `ASIZE`"]
pub type ASIZE_R = crate::R<u8, ASIZE_A>;
impl ASIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASIZE_A {
        match self.bits {
            0 => ASIZE_A::NONE,
            1 => ASIZE_A::_4BIT,
            2 => ASIZE_A::_12BIT,
            3 => ASIZE_A::_20BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ASIZE_A::NONE
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == ASIZE_A::_4BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == ASIZE_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_20BIT`"]
    #[inline(always)]
    pub fn is_20bit(&self) -> bool {
        *self == ASIZE_A::_20BIT
    }
}
#[doc = "Write proxy for field `ASIZE`"]
pub struct ASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ASIZE_A::NONE)
    }
    #[doc = "Up to 4 bits wide"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(ASIZE_A::_4BIT)
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(ASIZE_A::_12BIT)
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    #[inline(always)]
    pub fn _20bit(self) -> &'a mut W {
        self.variant(ASIZE_A::_20BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `WR2CYC`"]
pub type WR2CYC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR2CYC`"]
pub struct WR2CYC_W<'a> {
    w: &'a mut W,
}
impl<'a> WR2CYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `FRMCNT`"]
pub type FRMCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRMCNT`"]
pub struct FRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `FRM50`"]
pub type FRM50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM50`"]
pub struct FRM50_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM50_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CLKPIN`"]
pub type CLKPIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPIN`"]
pub struct CLKPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn asize(&self) -> ASIZE_R {
        ASIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn wr2cyc(&self) -> WR2CYC_R {
        WR2CYC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn frm50(&self) -> FRM50_R {
        FRM50_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn clkpin(&self) -> CLKPIN_R {
        CLKPIN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn asize(&mut self) -> ASIZE_W {
        ASIZE_W { w: self }
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn wr2cyc(&mut self) -> WR2CYC_W {
        WR2CYC_W { w: self }
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FRMCNT_W {
        FRMCNT_W { w: self }
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn frm50(&mut self) -> FRM50_W {
        FRM50_W { w: self }
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn clkpin(&mut self) -> CLKPIN_W {
        CLKPIN_W { w: self }
    }
}
