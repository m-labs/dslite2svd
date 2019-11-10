#[doc = "Reader of register ACCTL1"]
pub type R = crate::R<u32, super::ACCTL1>;
#[doc = "Writer for register ACCTL1"]
pub type W = crate::W<u32, super::ACCTL1>;
#[doc = "Register ACCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CINV`"]
pub type CINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CINV`"]
pub struct CINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CINV_W<'a> {
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
#[doc = "Interrupt Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISEN_A {
    #[doc = "0: Level sense, see ISLVAL"]
    LEVEL,
    #[doc = "1: Falling edge"]
    FALL,
    #[doc = "2: Rising edge"]
    RISE,
    #[doc = "3: Either edge"]
    BOTH,
}
impl From<ISEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISEN_A) -> Self {
        match variant {
            ISEN_A::LEVEL => 0,
            ISEN_A::FALL => 1,
            ISEN_A::RISE => 2,
            ISEN_A::BOTH => 3,
        }
    }
}
#[doc = "Reader of field `ISEN`"]
pub type ISEN_R = crate::R<u8, ISEN_A>;
impl ISEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEN_A {
        match self.bits {
            0 => ISEN_A::LEVEL,
            1 => ISEN_A::FALL,
            2 => ISEN_A::RISE,
            3 => ISEN_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISEN_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == ISEN_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == ISEN_A::RISE
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ISEN_A::BOTH
    }
}
#[doc = "Write proxy for field `ISEN`"]
pub struct ISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISEN_A::LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(ISEN_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(ISEN_A::RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ISEN_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ISLVAL`"]
pub type ISLVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISLVAL`"]
pub struct ISLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ISLVAL_W<'a> {
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
#[doc = "Trigger Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    #[doc = "0: Level sense, see TSLVAL"]
    LEVEL,
    #[doc = "1: Falling edge"]
    FALL,
    #[doc = "2: Rising edge"]
    RISE,
    #[doc = "3: Either edge"]
    BOTH,
}
impl From<TSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        match variant {
            TSEN_A::LEVEL => 0,
            TSEN_A::FALL => 1,
            TSEN_A::RISE => 2,
            TSEN_A::BOTH => 3,
        }
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<u8, TSEN_A>;
impl TSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            0 => TSEN_A::LEVEL,
            1 => TSEN_A::FALL,
            2 => TSEN_A::RISE,
            3 => TSEN_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TSEN_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TSEN_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == TSEN_A::RISE
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == TSEN_A::BOTH
    }
}
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TSEN_A::LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(TSEN_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(TSEN_A::RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(TSEN_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSLVAL`"]
pub type TSLVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSLVAL`"]
pub struct TSLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSLVAL_W<'a> {
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
#[doc = "Analog Source Positive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRCP_A {
    #[doc = "0: Pin value of Cn+"]
    PIN,
    #[doc = "1: Pin value of C0+"]
    PIN0,
    #[doc = "2: Internal voltage reference"]
    REF,
}
impl From<ASRCP_A> for u8 {
    #[inline(always)]
    fn from(variant: ASRCP_A) -> Self {
        match variant {
            ASRCP_A::PIN => 0,
            ASRCP_A::PIN0 => 1,
            ASRCP_A::REF => 2,
        }
    }
}
#[doc = "Reader of field `ASRCP`"]
pub type ASRCP_R = crate::R<u8, ASRCP_A>;
impl ASRCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ASRCP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ASRCP_A::PIN),
            1 => Val(ASRCP_A::PIN0),
            2 => Val(ASRCP_A::REF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == ASRCP_A::PIN
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == ASRCP_A::PIN0
    }
    #[doc = "Checks if the value of the field is `REF`"]
    #[inline(always)]
    pub fn is_ref_(&self) -> bool {
        *self == ASRCP_A::REF
    }
}
#[doc = "Write proxy for field `ASRCP`"]
pub struct ASRCP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRCP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(ASRCP_A::PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(ASRCP_A::PIN0)
    }
    #[doc = "Internal voltage reference"]
    #[inline(always)]
    pub fn ref_(self) -> &'a mut W {
        self.variant(ASRCP_A::REF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `TOEN`"]
pub type TOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOEN`"]
pub struct TOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOEN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn cinv(&self) -> CINV_R {
        CINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn isen(&self) -> ISEN_R {
        ISEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn islval(&self) -> ISLVAL_R {
        ISLVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn tslval(&self) -> TSLVAL_R {
        TSLVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn asrcp(&self) -> ASRCP_R {
        ASRCP_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn cinv(&mut self) -> CINV_W {
        CINV_W { w: self }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn isen(&mut self) -> ISEN_W {
        ISEN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn islval(&mut self) -> ISLVAL_W {
        ISLVAL_W { w: self }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn tslval(&mut self) -> TSLVAL_W {
        TSLVAL_W { w: self }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn asrcp(&mut self) -> ASRCP_W {
        ASRCP_W { w: self }
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn toen(&mut self) -> TOEN_W {
        TOEN_W { w: self }
    }
}
