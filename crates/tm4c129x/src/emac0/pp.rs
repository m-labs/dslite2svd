#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Ethernet PHY Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PHYTYPE_A {
    #[doc = "0: No PHY"]
    NONE = 0,
    #[doc = "3: Snowflake class PHY"]
    _1 = 3,
}
impl From<PHYTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PHYTYPE`"]
pub type PHYTYPE_R = crate::R<u8, PHYTYPE_A>;
impl PHYTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PHYTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PHYTYPE_A::NONE),
            3 => Val(PHYTYPE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PHYTYPE_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHYTYPE_A::_1
    }
}
#[doc = "Reader of field `MACTYPE`"]
pub type MACTYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline(always)]
    pub fn phytype(&self) -> PHYTYPE_R {
        PHYTYPE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline(always)]
    pub fn mactype(&self) -> MACTYPE_R {
        MACTYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
