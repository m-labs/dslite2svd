#[doc = "Reader of register EPC"]
pub type R = crate::R<u32, super::EPC>;
#[doc = "Writer for register EPC"]
pub type W = crate::W<u32, super::EPC>;
#[doc = "Register EPC `reset()`'s with value 0"]
impl crate::ResetValue for super::EPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Power Supply Enable Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEN_A {
    #[doc = "0: Power Enable Active Low"]
    LOW,
    #[doc = "1: Power Enable Active High"]
    HIGH,
    #[doc = "2: Power Enable High if VBUS Low (OTG only)"]
    VBLOW,
    #[doc = "3: Power Enable High if VBUS High (OTG only)"]
    VBHIGH,
}
impl From<EPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EPEN_A) -> Self {
        match variant {
            EPEN_A::LOW => 0,
            EPEN_A::HIGH => 1,
            EPEN_A::VBLOW => 2,
            EPEN_A::VBHIGH => 3,
        }
    }
}
#[doc = "Reader of field `EPEN`"]
pub type EPEN_R = crate::R<u8, EPEN_A>;
impl EPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEN_A {
        match self.bits {
            0 => EPEN_A::LOW,
            1 => EPEN_A::HIGH,
            2 => EPEN_A::VBLOW,
            3 => EPEN_A::VBHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EPEN_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EPEN_A::HIGH
    }
    #[doc = "Checks if the value of the field is `VBLOW`"]
    #[inline(always)]
    pub fn is_vblow(&self) -> bool {
        *self == EPEN_A::VBLOW
    }
    #[doc = "Checks if the value of the field is `VBHIGH`"]
    #[inline(always)]
    pub fn is_vbhigh(&self) -> bool {
        *self == EPEN_A::VBHIGH
    }
}
#[doc = "Write proxy for field `EPEN`"]
pub struct EPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Power Enable Active Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EPEN_A::LOW)
    }
    #[doc = "Power Enable Active High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EPEN_A::HIGH)
    }
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    #[inline(always)]
    pub fn vblow(self) -> &'a mut W {
        self.variant(EPEN_A::VBLOW)
    }
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    #[inline(always)]
    pub fn vbhigh(self) -> &'a mut W {
        self.variant(EPEN_A::VBHIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `EPENDE`"]
pub type EPENDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPENDE`"]
pub struct EPENDE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPENDE_W<'a> {
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
#[doc = "Reader of field `PFLTEN`"]
pub type PFLTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFLTEN`"]
pub struct PFLTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFLTEN_W<'a> {
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
#[doc = "Reader of field `PFLTSEN_HIGH`"]
pub type PFLTSEN_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFLTSEN_HIGH`"]
pub struct PFLTSEN_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> PFLTSEN_HIGH_W<'a> {
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
#[doc = "Reader of field `PFLTAEN`"]
pub type PFLTAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFLTAEN`"]
pub struct PFLTAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFLTAEN_W<'a> {
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
#[doc = "Power Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLTACT_A {
    #[doc = "0: Unchanged"]
    UNCHG,
    #[doc = "1: Tristate"]
    TRIS,
    #[doc = "2: Low"]
    LOW,
    #[doc = "3: High"]
    HIGH,
}
impl From<PFLTACT_A> for u8 {
    #[inline(always)]
    fn from(variant: PFLTACT_A) -> Self {
        match variant {
            PFLTACT_A::UNCHG => 0,
            PFLTACT_A::TRIS => 1,
            PFLTACT_A::LOW => 2,
            PFLTACT_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `PFLTACT`"]
pub type PFLTACT_R = crate::R<u8, PFLTACT_A>;
impl PFLTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFLTACT_A {
        match self.bits {
            0 => PFLTACT_A::UNCHG,
            1 => PFLTACT_A::TRIS,
            2 => PFLTACT_A::LOW,
            3 => PFLTACT_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNCHG`"]
    #[inline(always)]
    pub fn is_unchg(&self) -> bool {
        *self == PFLTACT_A::UNCHG
    }
    #[doc = "Checks if the value of the field is `TRIS`"]
    #[inline(always)]
    pub fn is_tris(&self) -> bool {
        *self == PFLTACT_A::TRIS
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PFLTACT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PFLTACT_A::HIGH
    }
}
#[doc = "Write proxy for field `PFLTACT`"]
pub struct PFLTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PFLTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFLTACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Unchanged"]
    #[inline(always)]
    pub fn unchg(self) -> &'a mut W {
        self.variant(PFLTACT_A::UNCHG)
    }
    #[doc = "Tristate"]
    #[inline(always)]
    pub fn tris(self) -> &'a mut W {
        self.variant(PFLTACT_A::TRIS)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PFLTACT_A::LOW)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PFLTACT_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn epende(&self) -> EPENDE_R {
        EPENDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn pflten(&self) -> PFLTEN_R {
        PFLTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn pfltsen_high(&self) -> PFLTSEN_HIGH_R {
        PFLTSEN_HIGH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn pfltaen(&self) -> PFLTAEN_R {
        PFLTAEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn pfltact(&self) -> PFLTACT_R {
        PFLTACT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn epen(&mut self) -> EPEN_W {
        EPEN_W { w: self }
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn epende(&mut self) -> EPENDE_W {
        EPENDE_W { w: self }
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn pflten(&mut self) -> PFLTEN_W {
        PFLTEN_W { w: self }
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn pfltsen_high(&mut self) -> PFLTSEN_HIGH_W {
        PFLTSEN_HIGH_W { w: self }
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn pfltaen(&mut self) -> PFLTAEN_W {
        PFLTAEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn pfltact(&mut self) -> PFLTACT_W {
        PFLTACT_W { w: self }
    }
}
