#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Maximum ADC Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSR_A {
    #[doc = "1: 125 ksps"]
    _125K = 1,
    #[doc = "3: 250 ksps"]
    _250K = 3,
    #[doc = "5: 500 ksps"]
    _500K = 5,
    #[doc = "7: 1 Msps"]
    _1M = 7,
}
impl From<MSR_A> for u8 {
    #[inline(always)]
    fn from(variant: MSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSR`"]
pub type MSR_R = crate::R<u8, MSR_A>;
impl MSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MSR_A::_125K),
            3 => Val(MSR_A::_250K),
            5 => Val(MSR_A::_500K),
            7 => Val(MSR_A::_1M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline(always)]
    pub fn is_125k(&self) -> bool {
        *self == MSR_A::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline(always)]
    pub fn is_250k(&self) -> bool {
        *self == MSR_A::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline(always)]
    pub fn is_500k(&self) -> bool {
        *self == MSR_A::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == MSR_A::_1M
    }
}
#[doc = "Reader of field `CH`"]
pub type CH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<u8, u8>;
#[doc = "ADC Architecture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: SAR"]
    SAR = 0,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPE_A::SAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAR`"]
    #[inline(always)]
    pub fn is_sar(&self) -> bool {
        *self == TYPE_A::SAR
    }
}
#[doc = "Reader of field `RSL`"]
pub type RSL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Maximum ADC Sample Rate"]
    #[inline(always)]
    pub fn msr(&self) -> MSR_R {
        MSR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn rsl(&self) -> RSL_R {
        RSL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
