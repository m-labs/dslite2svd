#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Maximum Conversion Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_A {
    #[doc = "7: Full conversion rate (FCONV) as defined by TADC and NSH"]
    FULL,
}
impl From<MCR_A> for u8 {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        match variant {
            MCR_A::FULL => 7,
        }
    }
}
#[doc = "Reader of field `MCR`"]
pub type MCR_R = crate::R<u8, MCR_A>;
impl MCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCR_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(MCR_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == MCR_A::FULL
    }
}
#[doc = "Reader of field `CH`"]
pub type CH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<u8, u8>;
#[doc = "ADC Architecture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE_A {
    #[doc = "0: SAR"]
    SAR,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        match variant {
            TYPE_A::SAR => 0,
        }
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
#[doc = "Reader of field `APSHT`"]
pub type APSHT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Maximum Conversion Rate"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 0x0f) as u8)
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
    #[doc = "Bit 24 - Application-Programmable Sample-and-Hold Time"]
    #[inline(always)]
    pub fn apsht(&self) -> APSHT_R {
        APSHT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
