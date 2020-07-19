#[doc = "Reader of register TST"]
pub type R = crate::R<u32, super::TST>;
#[doc = "Writer for register TST"]
pub type W = crate::W<u32, super::TST>;
#[doc = "Register TST `reset()`'s with value 0"]
impl crate::ResetValue for super::TST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASIC`"]
pub type BASIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BASIC`"]
pub struct BASIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_W<'a> {
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
#[doc = "Reader of field `SILENT`"]
pub type SILENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SILENT`"]
pub struct SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SILENT_W<'a> {
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
#[doc = "Reader of field `LBACK`"]
pub type LBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBACK`"]
pub struct LBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBACK_W<'a> {
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
#[doc = "Transmit Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: CAN Module Control"]
    CANCTL = 0,
    #[doc = "1: Sample Point"]
    SAMPLE = 1,
    #[doc = "2: Driven Low"]
    DOMINANT = 2,
    #[doc = "3: Driven High"]
    RECESSIVE = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::CANCTL,
            1 => TX_A::SAMPLE,
            2 => TX_A::DOMINANT,
            3 => TX_A::RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CANCTL`"]
    #[inline(always)]
    pub fn is_canctl(&self) -> bool {
        *self == TX_A::CANCTL
    }
    #[doc = "Checks if the value of the field is `SAMPLE`"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == TX_A::SAMPLE
    }
    #[doc = "Checks if the value of the field is `DOMINANT`"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == TX_A::DOMINANT
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == TX_A::RECESSIVE
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CAN Module Control"]
    #[inline(always)]
    pub fn canctl(self) -> &'a mut W {
        self.variant(TX_A::CANCTL)
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut W {
        self.variant(TX_A::SAMPLE)
    }
    #[doc = "Driven Low"]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut W {
        self.variant(TX_A::DOMINANT)
    }
    #[doc = "Driven High"]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut W {
        self.variant(TX_A::RECESSIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
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
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W { w: self }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W { w: self }
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
