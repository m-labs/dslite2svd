#[doc = "Reader of register TXTYPE2"]
pub type R = crate::R<u8, super::TXTYPE2>;
#[doc = "Writer for register TXTYPE2"]
pub type W = crate::W<u8, super::TXTYPE2>;
#[doc = "Register TXTYPE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TXTYPE2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEP`"]
pub type TEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEP`"]
pub struct TEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTO_A {
    #[doc = "0: Control"]
    CTRL,
    #[doc = "1: Isochronous"]
    ISOC,
    #[doc = "2: Bulk"]
    BULK,
    #[doc = "3: Interrupt"]
    INT,
}
impl From<PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: PROTO_A) -> Self {
        match variant {
            PROTO_A::CTRL => 0,
            PROTO_A::ISOC => 1,
            PROTO_A::BULK => 2,
            PROTO_A::INT => 3,
        }
    }
}
#[doc = "Reader of field `PROTO`"]
pub type PROTO_R = crate::R<u8, PROTO_A>;
impl PROTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTO_A {
        match self.bits {
            0 => PROTO_A::CTRL,
            1 => PROTO_A::ISOC,
            2 => PROTO_A::BULK,
            3 => PROTO_A::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == PROTO_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISOC`"]
    #[inline(always)]
    pub fn is_isoc(&self) -> bool {
        *self == PROTO_A::ISOC
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == PROTO_A::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == PROTO_A::INT
    }
}
#[doc = "Write proxy for field `PROTO`"]
pub struct PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PROTO_A::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn isoc(self) -> &'a mut W {
        self.variant(PROTO_A::ISOC)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(PROTO_A::BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(PROTO_A::INT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "0: Default"]
    DFLT,
    #[doc = "1: High"]
    HIGH,
    #[doc = "2: Full"]
    FULL,
    #[doc = "3: Low"]
    LOW,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        match variant {
            SPEED_A::DFLT => 0,
            SPEED_A::HIGH => 1,
            SPEED_A::FULL => 2,
            SPEED_A::LOW => 3,
        }
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::DFLT,
            1 => SPEED_A::HIGH,
            2 => SPEED_A::FULL,
            3 => SPEED_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DFLT`"]
    #[inline(always)]
    pub fn is_dflt(&self) -> bool {
        *self == SPEED_A::DFLT
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SPEED_A::FULL
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEED_A::LOW
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn dflt(self) -> &'a mut W {
        self.variant(SPEED_A::DFLT)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(SPEED_A::FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEED_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn tep(&self) -> TEP_R {
        TEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn proto(&self) -> PROTO_R {
        PROTO_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn tep(&mut self) -> TEP_W {
        TEP_W { w: self }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn proto(&mut self) -> PROTO_W {
        PROTO_W { w: self }
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
}
