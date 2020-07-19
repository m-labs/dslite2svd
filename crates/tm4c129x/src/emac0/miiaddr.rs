#[doc = "Reader of register MIIADDR"]
pub type R = crate::R<u32, super::MIIADDR>;
#[doc = "Writer for register MIIADDR"]
pub type W = crate::W<u32, super::MIIADDR>;
#[doc = "Register MIIADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MIIADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIIB`"]
pub type MIIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIIB`"]
pub struct MIIB_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIB_W<'a> {
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
#[doc = "Reader of field `MIIW`"]
pub type MIIW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIIW`"]
pub struct MIIW_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIW_W<'a> {
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
#[doc = "Clock Reference Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CR_A {
    #[doc = "0: The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    _60_100 = 0,
    #[doc = "1: The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    _100_150 = 1,
    #[doc = "2: The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    _20_35 = 2,
    #[doc = "3: The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    _35_60 = 3,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u8, CR_A>;
impl CR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CR_A::_60_100),
            1 => Val(CR_A::_100_150),
            2 => Val(CR_A::_20_35),
            3 => Val(CR_A::_35_60),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_60_100`"]
    #[inline(always)]
    pub fn is_60_100(&self) -> bool {
        *self == CR_A::_60_100
    }
    #[doc = "Checks if the value of the field is `_100_150`"]
    #[inline(always)]
    pub fn is_100_150(&self) -> bool {
        *self == CR_A::_100_150
    }
    #[doc = "Checks if the value of the field is `_20_35`"]
    #[inline(always)]
    pub fn is_20_35(&self) -> bool {
        *self == CR_A::_20_35
    }
    #[doc = "Checks if the value of the field is `_35_60`"]
    #[inline(always)]
    pub fn is_35_60(&self) -> bool {
        *self == CR_A::_35_60
    }
}
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    #[inline(always)]
    pub fn _60_100(self) -> &'a mut W {
        self.variant(CR_A::_60_100)
    }
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    #[inline(always)]
    pub fn _100_150(self) -> &'a mut W {
        self.variant(CR_A::_100_150)
    }
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    #[inline(always)]
    pub fn _20_35(self) -> &'a mut W {
        self.variant(CR_A::_20_35)
    }
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    #[inline(always)]
    pub fn _35_60(self) -> &'a mut W {
        self.variant(CR_A::_35_60)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `MII`"]
pub type MII_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MII`"]
pub struct MII_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `PLA`"]
pub type PLA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLA`"]
pub struct PLA_W<'a> {
    w: &'a mut W,
}
impl<'a> PLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn miib(&self) -> MIIB_R {
        MIIB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn miiw(&self) -> MIIW_R {
        MIIW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pla(&self) -> PLA_R {
        PLA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn miib(&mut self) -> MIIB_W {
        MIIB_W { w: self }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn miiw(&mut self) -> MIIW_W {
        MIIW_W { w: self }
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pla(&mut self) -> PLA_W {
        PLA_W { w: self }
    }
}
