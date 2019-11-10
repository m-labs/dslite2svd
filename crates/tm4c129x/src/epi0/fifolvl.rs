#[doc = "Reader of register FIFOLVL"]
pub type R = crate::R<u32, super::FIFOLVL>;
#[doc = "Writer for register FIFOLVL"]
pub type W = crate::W<u32, super::FIFOLVL>;
#[doc = "Register FIFOLVL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOLVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Read FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFIFO_A {
    #[doc = "1: Trigger when there are 1 or more entries in the NBRFIFO"]
    _1,
    #[doc = "2: Trigger when there are 2 or more entries in the NBRFIFO"]
    _2,
    #[doc = "3: Trigger when there are 4 or more entries in the NBRFIFO"]
    _4,
    #[doc = "4: Trigger when there are 6 or more entries in the NBRFIFO"]
    _6,
    #[doc = "5: Trigger when there are 7 or more entries in the NBRFIFO"]
    _7,
    #[doc = "6: Trigger when there are 8 entries in the NBRFIFO"]
    _8,
}
impl From<RDFIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: RDFIFO_A) -> Self {
        match variant {
            RDFIFO_A::_1 => 1,
            RDFIFO_A::_2 => 2,
            RDFIFO_A::_4 => 3,
            RDFIFO_A::_6 => 4,
            RDFIFO_A::_7 => 5,
            RDFIFO_A::_8 => 6,
        }
    }
}
#[doc = "Reader of field `RDFIFO`"]
pub type RDFIFO_R = crate::R<u8, RDFIFO_A>;
impl RDFIFO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDFIFO_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RDFIFO_A::_1),
            2 => Val(RDFIFO_A::_2),
            3 => Val(RDFIFO_A::_4),
            4 => Val(RDFIFO_A::_6),
            5 => Val(RDFIFO_A::_7),
            6 => Val(RDFIFO_A::_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDFIFO_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RDFIFO_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RDFIFO_A::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == RDFIFO_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == RDFIFO_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RDFIFO_A::_8
    }
}
#[doc = "Write proxy for field `RDFIFO`"]
pub struct RDFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RDFIFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDFIFO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDFIFO_A::_1)
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RDFIFO_A::_2)
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RDFIFO_A::_4)
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(RDFIFO_A::_6)
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(RDFIFO_A::_7)
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RDFIFO_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Write FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRFIFO_A {
    #[doc = "0: Interrupt is triggered while WRFIFO is empty."]
    EMPT,
    #[doc = "2: Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    _2,
    #[doc = "3: Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    _1,
    #[doc = "4: Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    NFULL,
}
impl From<WRFIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: WRFIFO_A) -> Self {
        match variant {
            WRFIFO_A::EMPT => 0,
            WRFIFO_A::_2 => 2,
            WRFIFO_A::_1 => 3,
            WRFIFO_A::NFULL => 4,
        }
    }
}
#[doc = "Reader of field `WRFIFO`"]
pub type WRFIFO_R = crate::R<u8, WRFIFO_A>;
impl WRFIFO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WRFIFO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WRFIFO_A::EMPT),
            2 => Val(WRFIFO_A::_2),
            3 => Val(WRFIFO_A::_1),
            4 => Val(WRFIFO_A::NFULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMPT`"]
    #[inline(always)]
    pub fn is_empt(&self) -> bool {
        *self == WRFIFO_A::EMPT
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == WRFIFO_A::_2
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRFIFO_A::_1
    }
    #[doc = "Checks if the value of the field is `NFULL`"]
    #[inline(always)]
    pub fn is_nfull(&self) -> bool {
        *self == WRFIFO_A::NFULL
    }
}
#[doc = "Write proxy for field `WRFIFO`"]
pub struct WRFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> WRFIFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRFIFO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    #[inline(always)]
    pub fn empt(self) -> &'a mut W {
        self.variant(WRFIFO_A::EMPT)
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WRFIFO_A::_2)
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRFIFO_A::_1)
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    #[inline(always)]
    pub fn nfull(self) -> &'a mut W {
        self.variant(WRFIFO_A::NFULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSERR`"]
pub type RSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSERR`"]
pub struct RSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSERR_W<'a> {
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
#[doc = "Reader of field `WFERR`"]
pub type WFERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFERR`"]
pub struct WFERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFERR_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn rdfifo(&self) -> RDFIFO_R {
        RDFIFO_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn wrfifo(&self) -> WRFIFO_R {
        WRFIFO_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn rserr(&self) -> RSERR_R {
        RSERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn wferr(&self) -> WFERR_R {
        WFERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn rdfifo(&mut self) -> RDFIFO_W {
        RDFIFO_W { w: self }
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn wrfifo(&mut self) -> WRFIFO_W {
        WRFIFO_W { w: self }
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn rserr(&mut self) -> RSERR_W {
        RSERR_W { w: self }
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn wferr(&mut self) -> WFERR_W {
        WFERR_W { w: self }
    }
}
