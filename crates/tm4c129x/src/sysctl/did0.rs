#[doc = "Reader of register DID0"]
pub type R = crate::R<u32, super::DID0>;
#[doc = "Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIN_A {
    #[doc = "0: Initial device, or a major revision update"]
    _0,
    #[doc = "1: First metal layer change"]
    _1,
    #[doc = "2: Second metal layer change"]
    _2,
}
impl From<MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: MIN_A) -> Self {
        match variant {
            MIN_A::_0 => 0,
            MIN_A::_1 => 1,
            MIN_A::_2 => 2,
        }
    }
}
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, MIN_A>;
impl MIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIN_A::_0),
            1 => Val(MIN_A::_1),
            2 => Val(MIN_A::_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == MIN_A::_2
    }
}
#[doc = "Major Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJ_A {
    #[doc = "0: Revision A (initial device)"]
    REVA,
    #[doc = "1: Revision B (first base layer revision)"]
    REVB,
    #[doc = "2: Revision C (second base layer revision)"]
    REVC,
}
impl From<MAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJ_A) -> Self {
        match variant {
            MAJ_A::REVA => 0,
            MAJ_A::REVB => 1,
            MAJ_A::REVC => 2,
        }
    }
}
#[doc = "Reader of field `MAJ`"]
pub type MAJ_R = crate::R<u8, MAJ_A>;
impl MAJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAJ_A::REVA),
            1 => Val(MAJ_A::REVB),
            2 => Val(MAJ_A::REVC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REVA`"]
    #[inline(always)]
    pub fn is_reva(&self) -> bool {
        *self == MAJ_A::REVA
    }
    #[doc = "Checks if the value of the field is `REVB`"]
    #[inline(always)]
    pub fn is_revb(&self) -> bool {
        *self == MAJ_A::REVB
    }
    #[doc = "Checks if the value of the field is `REVC`"]
    #[inline(always)]
    pub fn is_revc(&self) -> bool {
        *self == MAJ_A::REVC
    }
}
#[doc = "Device Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLASS_A {
    #[doc = "10: Tiva(TM) TM4C129-class microcontrollers"]
    TM4C129,
}
impl From<CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLASS_A) -> Self {
        match variant {
            CLASS_A::TM4C129 => 10,
        }
    }
}
#[doc = "Reader of field `CLASS`"]
pub type CLASS_R = crate::R<u8, CLASS_A>;
impl CLASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLASS_A> {
        use crate::Variant::*;
        match self.bits {
            10 => Val(CLASS_A::TM4C129),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TM4C129`"]
    #[inline(always)]
    pub fn is_tm4c129(&self) -> bool {
        *self == CLASS_A::TM4C129
    }
}
#[doc = "DID0 Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_A {
    #[doc = "1: Second version of the DID0 register format."]
    _1,
}
impl From<VER_A> for u8 {
    #[inline(always)]
    fn from(variant: VER_A) -> Self {
        match variant {
            VER_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, VER_A>;
impl VER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VER_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(VER_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VER_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn maj(&self) -> MAJ_R {
        MAJ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
