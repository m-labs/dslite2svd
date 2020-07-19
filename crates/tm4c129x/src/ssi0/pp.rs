#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Reader of field `HSCLK`"]
pub type HSCLK_R = crate::R<bool, bool>;
#[doc = "Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Legacy SSI mode"]
    LEGACY = 0,
    #[doc = "1: Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    ADVBI = 1,
    #[doc = "2: Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    ADVBIQUAD = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::LEGACY),
            1 => Val(MODE_A::ADVBI),
            2 => Val(MODE_A::ADVBIQUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == MODE_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `ADVBI`"]
    #[inline(always)]
    pub fn is_advbi(&self) -> bool {
        *self == MODE_A::ADVBI
    }
    #[doc = "Checks if the value of the field is `ADVBIQUAD`"]
    #[inline(always)]
    pub fn is_advbiquad(&self) -> bool {
        *self == MODE_A::ADVBIQUAD
    }
}
#[doc = "Reader of field `FSSHLDFRM`"]
pub type FSSHLDFRM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline(always)]
    pub fn hsclk(&self) -> HSCLK_R {
        HSCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline(always)]
    pub fn fsshldfrm(&self) -> FSSHLDFRM_R {
        FSSHLDFRM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
