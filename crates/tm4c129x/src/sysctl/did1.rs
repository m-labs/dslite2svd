#[doc = "Reader of register DID1"]
pub type R = crate::R<u32, super::DID1>;
#[doc = "Qualification Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QUAL_A {
    #[doc = "0: Engineering Sample (unqualified)"]
    ES = 0,
    #[doc = "1: Pilot Production (unqualified)"]
    PP = 1,
    #[doc = "2: Fully Qualified"]
    FQ = 2,
}
impl From<QUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: QUAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QUAL`"]
pub type QUAL_R = crate::R<u8, QUAL_A>;
impl QUAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, QUAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(QUAL_A::ES),
            1 => Val(QUAL_A::PP),
            2 => Val(QUAL_A::FQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ES`"]
    #[inline(always)]
    pub fn is_es(&self) -> bool {
        *self == QUAL_A::ES
    }
    #[doc = "Checks if the value of the field is `PP`"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == QUAL_A::PP
    }
    #[doc = "Checks if the value of the field is `FQ`"]
    #[inline(always)]
    pub fn is_fq(&self) -> bool {
        *self == QUAL_A::FQ
    }
}
#[doc = "Reader of field `ROHS`"]
pub type ROHS_R = crate::R<bool, bool>;
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PKG_A {
    #[doc = "1: QFP package"]
    QFP = 1,
    #[doc = "2: BGA package"]
    BGA = 2,
}
impl From<PKG_A> for u8 {
    #[inline(always)]
    fn from(variant: PKG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PKG`"]
pub type PKG_R = crate::R<u8, PKG_A>;
impl PKG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PKG_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PKG_A::QFP),
            2 => Val(PKG_A::BGA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QFP`"]
    #[inline(always)]
    pub fn is_qfp(&self) -> bool {
        *self == PKG_A::QFP
    }
    #[doc = "Checks if the value of the field is `BGA`"]
    #[inline(always)]
    pub fn is_bga(&self) -> bool {
        *self == PKG_A::BGA
    }
}
#[doc = "Temperature Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEMP_A {
    #[doc = "0: Commercial temperature range"]
    C = 0,
    #[doc = "1: Industrial temperature range"]
    I = 1,
    #[doc = "2: Extended temperature range"]
    E = 2,
}
impl From<TEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TEMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<u8, TEMP_A>;
impl TEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TEMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TEMP_A::C),
            1 => Val(TEMP_A::I),
            2 => Val(TEMP_A::E),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == TEMP_A::C
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == TEMP_A::I
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == TEMP_A::E
    }
}
#[doc = "Package Pin Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCNT_A {
    #[doc = "2: 100-pin LQFP package"]
    _100 = 2,
    #[doc = "3: 64-pin LQFP package"]
    _64 = 3,
    #[doc = "4: 144-pin LQFP package"]
    _144 = 4,
    #[doc = "5: 157-pin BGA package"]
    _157 = 5,
    #[doc = "6: 128-pin TQFP package"]
    _128 = 6,
}
impl From<PINCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINCNT`"]
pub type PINCNT_R = crate::R<u8, PINCNT_A>;
impl PINCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINCNT_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PINCNT_A::_100),
            3 => Val(PINCNT_A::_64),
            4 => Val(PINCNT_A::_144),
            5 => Val(PINCNT_A::_157),
            6 => Val(PINCNT_A::_128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PINCNT_A::_100
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PINCNT_A::_64
    }
    #[doc = "Checks if the value of the field is `_144`"]
    #[inline(always)]
    pub fn is_144(&self) -> bool {
        *self == PINCNT_A::_144
    }
    #[doc = "Checks if the value of the field is `_157`"]
    #[inline(always)]
    pub fn is_157(&self) -> bool {
        *self == PINCNT_A::_157
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PINCNT_A::_128
    }
}
#[doc = "Reader of field `PRTNO`"]
pub type PRTNO_R = crate::R<u8, u8>;
#[doc = "Reader of field `FAM`"]
pub type FAM_R = crate::R<u8, u8>;
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn qual(&self) -> QUAL_R {
        QUAL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn rohs(&self) -> ROHS_R {
        ROHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn pincnt(&self) -> PINCNT_R {
        PINCNT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn prtno(&self) -> PRTNO_R {
        PRTNO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn fam(&self) -> FAM_R {
        FAM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
