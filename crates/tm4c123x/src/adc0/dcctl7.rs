#[doc = "Reader of register DCCTL7"]
pub type R = crate::R<u32, super::DCCTL7>;
#[doc = "Writer for register DCCTL7"]
pub type W = crate::W<u32, super::DCCTL7>;
#[doc = "Register DCCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCCTL7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparison Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIM_A {
    #[doc = "0: Always"]
    ALWAYS,
    #[doc = "1: Once"]
    ONCE,
    #[doc = "2: Hysteresis Always"]
    HALWAYS,
    #[doc = "3: Hysteresis Once"]
    HONCE,
}
impl From<CIM_A> for u8 {
    #[inline(always)]
    fn from(variant: CIM_A) -> Self {
        match variant {
            CIM_A::ALWAYS => 0,
            CIM_A::ONCE => 1,
            CIM_A::HALWAYS => 2,
            CIM_A::HONCE => 3,
        }
    }
}
#[doc = "Reader of field `CIM`"]
pub type CIM_R = crate::R<u8, CIM_A>;
impl CIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIM_A {
        match self.bits {
            0 => CIM_A::ALWAYS,
            1 => CIM_A::ONCE,
            2 => CIM_A::HALWAYS,
            3 => CIM_A::HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == CIM_A::ALWAYS
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == CIM_A::ONCE
    }
    #[doc = "Checks if the value of the field is `HALWAYS`"]
    #[inline(always)]
    pub fn is_halways(&self) -> bool {
        *self == CIM_A::HALWAYS
    }
    #[doc = "Checks if the value of the field is `HONCE`"]
    #[inline(always)]
    pub fn is_honce(&self) -> bool {
        *self == CIM_A::HONCE
    }
}
#[doc = "Write proxy for field `CIM`"]
pub struct CIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(CIM_A::ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(CIM_A::ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn halways(self) -> &'a mut W {
        self.variant(CIM_A::HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn honce(self) -> &'a mut W {
        self.variant(CIM_A::HONCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Comparison Interrupt Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIC_A {
    #[doc = "0: Low Band"]
    LOW,
    #[doc = "1: Mid Band"]
    MID,
    #[doc = "3: High Band"]
    HIGH,
}
impl From<CIC_A> for u8 {
    #[inline(always)]
    fn from(variant: CIC_A) -> Self {
        match variant {
            CIC_A::LOW => 0,
            CIC_A::MID => 1,
            CIC_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `CIC`"]
pub type CIC_R = crate::R<u8, CIC_A>;
impl CIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIC_A::LOW),
            1 => Val(CIC_A::MID),
            3 => Val(CIC_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CIC_A::LOW
    }
    #[doc = "Checks if the value of the field is `MID`"]
    #[inline(always)]
    pub fn is_mid(&self) -> bool {
        *self == CIC_A::MID
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CIC_A::HIGH
    }
}
#[doc = "Write proxy for field `CIC`"]
pub struct CIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CIC_A::LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn mid(self) -> &'a mut W {
        self.variant(CIC_A::MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CIC_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CIE`"]
pub type CIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CIE`"]
pub struct CIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIE_W<'a> {
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
#[doc = "Comparison Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTM_A {
    #[doc = "0: Always"]
    ALWAYS,
    #[doc = "1: Once"]
    ONCE,
    #[doc = "2: Hysteresis Always"]
    HALWAYS,
    #[doc = "3: Hysteresis Once"]
    HONCE,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        match variant {
            CTM_A::ALWAYS => 0,
            CTM_A::ONCE => 1,
            CTM_A::HALWAYS => 2,
            CTM_A::HONCE => 3,
        }
    }
}
#[doc = "Reader of field `CTM`"]
pub type CTM_R = crate::R<u8, CTM_A>;
impl CTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::ALWAYS,
            1 => CTM_A::ONCE,
            2 => CTM_A::HALWAYS,
            3 => CTM_A::HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == CTM_A::ALWAYS
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == CTM_A::ONCE
    }
    #[doc = "Checks if the value of the field is `HALWAYS`"]
    #[inline(always)]
    pub fn is_halways(&self) -> bool {
        *self == CTM_A::HALWAYS
    }
    #[doc = "Checks if the value of the field is `HONCE`"]
    #[inline(always)]
    pub fn is_honce(&self) -> bool {
        *self == CTM_A::HONCE
    }
}
#[doc = "Write proxy for field `CTM`"]
pub struct CTM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(CTM_A::ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(CTM_A::ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn halways(self) -> &'a mut W {
        self.variant(CTM_A::HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn honce(self) -> &'a mut W {
        self.variant(CTM_A::HONCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Comparison Trigger Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTC_A {
    #[doc = "0: Low Band"]
    LOW,
    #[doc = "1: Mid Band"]
    MID,
    #[doc = "3: High Band"]
    HIGH,
}
impl From<CTC_A> for u8 {
    #[inline(always)]
    fn from(variant: CTC_A) -> Self {
        match variant {
            CTC_A::LOW => 0,
            CTC_A::MID => 1,
            CTC_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `CTC`"]
pub type CTC_R = crate::R<u8, CTC_A>;
impl CTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTC_A::LOW),
            1 => Val(CTC_A::MID),
            3 => Val(CTC_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CTC_A::LOW
    }
    #[doc = "Checks if the value of the field is `MID`"]
    #[inline(always)]
    pub fn is_mid(&self) -> bool {
        *self == CTC_A::MID
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CTC_A::HIGH
    }
}
#[doc = "Write proxy for field `CTC`"]
pub struct CTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CTC_A::LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn mid(self) -> &'a mut W {
        self.variant(CTC_A::MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CTC_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CTE`"]
pub type CTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTE`"]
pub struct CTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTE_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn cim(&self) -> CIM_R {
        CIM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn cic(&self) -> CIC_R {
        CIC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn ctc(&self) -> CTC_R {
        CTC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn cte(&self) -> CTE_R {
        CTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn cim(&mut self) -> CIM_W {
        CIM_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn cic(&mut self) -> CIC_W {
        CIC_W { w: self }
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W {
        CIE_W { w: self }
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn ctm(&mut self) -> CTM_W {
        CTM_W { w: self }
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn ctc(&mut self) -> CTC_W {
        CTC_W { w: self }
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn cte(&mut self) -> CTE_W {
        CTE_W { w: self }
    }
}
