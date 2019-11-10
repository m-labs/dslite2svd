#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
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
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
#[doc = "Reader of field `SIGMODE`"]
pub type SIGMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGMODE`"]
pub struct SIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGMODE_W<'a> {
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
#[doc = "Reader of field `CAPMODE`"]
pub type CAPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPMODE`"]
pub struct CAPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMODE_W<'a> {
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
#[doc = "Reader of field `RESMODE`"]
pub type RESMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESMODE`"]
pub struct RESMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESMODE_W<'a> {
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
#[doc = "Reader of field `VELEN`"]
pub type VELEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VELEN`"]
pub struct VELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VELEN_W<'a> {
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
#[doc = "Predivide Velocity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VELDIV_A {
    #[doc = "0: QEI clock /1"]
    _1,
    #[doc = "1: QEI clock /2"]
    _2,
    #[doc = "2: QEI clock /4"]
    _4,
    #[doc = "3: QEI clock /8"]
    _8,
    #[doc = "4: QEI clock /16"]
    _16,
    #[doc = "5: QEI clock /32"]
    _32,
    #[doc = "6: QEI clock /64"]
    _64,
    #[doc = "7: QEI clock /128"]
    _128,
}
impl From<VELDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: VELDIV_A) -> Self {
        match variant {
            VELDIV_A::_1 => 0,
            VELDIV_A::_2 => 1,
            VELDIV_A::_4 => 2,
            VELDIV_A::_8 => 3,
            VELDIV_A::_16 => 4,
            VELDIV_A::_32 => 5,
            VELDIV_A::_64 => 6,
            VELDIV_A::_128 => 7,
        }
    }
}
#[doc = "Reader of field `VELDIV`"]
pub type VELDIV_R = crate::R<u8, VELDIV_A>;
impl VELDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VELDIV_A {
        match self.bits {
            0 => VELDIV_A::_1,
            1 => VELDIV_A::_2,
            2 => VELDIV_A::_4,
            3 => VELDIV_A::_8,
            4 => VELDIV_A::_16,
            5 => VELDIV_A::_32,
            6 => VELDIV_A::_64,
            7 => VELDIV_A::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VELDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == VELDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == VELDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == VELDIV_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == VELDIV_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == VELDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == VELDIV_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == VELDIV_A::_128
    }
}
#[doc = "Write proxy for field `VELDIV`"]
pub struct VELDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VELDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VELDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VELDIV_A::_1)
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(VELDIV_A::_2)
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(VELDIV_A::_4)
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(VELDIV_A::_8)
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(VELDIV_A::_16)
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(VELDIV_A::_32)
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(VELDIV_A::_64)
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(VELDIV_A::_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `INVA`"]
pub type INVA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVA`"]
pub struct INVA_W<'a> {
    w: &'a mut W,
}
impl<'a> INVA_W<'a> {
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
#[doc = "Reader of field `INVB`"]
pub type INVB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVB`"]
pub struct INVB_W<'a> {
    w: &'a mut W,
}
impl<'a> INVB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `INVI`"]
pub type INVI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVI`"]
pub struct INVI_W<'a> {
    w: &'a mut W,
}
impl<'a> INVI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `STALLEN`"]
pub type STALLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLEN`"]
pub struct STALLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEN_W<'a> {
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
#[doc = "Reader of field `FILTEN`"]
pub type FILTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN`"]
pub struct FILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FILTCNT`"]
pub type FILTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTCNT`"]
pub struct FILTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn sigmode(&self) -> SIGMODE_R {
        SIGMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn resmode(&self) -> RESMODE_R {
        RESMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn velen(&self) -> VELEN_R {
        VELEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn veldiv(&self) -> VELDIV_R {
        VELDIV_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn invi(&self) -> INVI_R {
        INVI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn stallen(&self) -> STALLEN_R {
        STALLEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn filtcnt(&self) -> FILTCNT_R {
        FILTCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn sigmode(&mut self) -> SIGMODE_W {
        SIGMODE_W { w: self }
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn capmode(&mut self) -> CAPMODE_W {
        CAPMODE_W { w: self }
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn resmode(&mut self) -> RESMODE_W {
        RESMODE_W { w: self }
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn velen(&mut self) -> VELEN_W {
        VELEN_W { w: self }
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn veldiv(&mut self) -> VELDIV_W {
        VELDIV_W { w: self }
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn inva(&mut self) -> INVA_W {
        INVA_W { w: self }
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn invb(&mut self) -> INVB_W {
        INVB_W { w: self }
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn invi(&mut self) -> INVI_W {
        INVI_W { w: self }
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn stallen(&mut self) -> STALLEN_W {
        STALLEN_W { w: self }
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn filten(&mut self) -> FILTEN_W {
        FILTEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn filtcnt(&mut self) -> FILTCNT_W {
        FILTCNT_W { w: self }
    }
}
