#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Preamble Length for Transmit Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRELEN_A {
    #[doc = "0: 7 bytes of preamble"]
    _7 = 0,
    #[doc = "1: 5 bytes of preamble"]
    _5 = 1,
    #[doc = "2: 3 bytes of preamble"]
    _3 = 2,
}
impl From<PRELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PRELEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRELEN`"]
pub type PRELEN_R = crate::R<u8, PRELEN_A>;
impl PRELEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRELEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRELEN_A::_7),
            1 => Val(PRELEN_A::_5),
            2 => Val(PRELEN_A::_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PRELEN_A::_7
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PRELEN_A::_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PRELEN_A::_3
    }
}
#[doc = "Write proxy for field `PRELEN`"]
pub struct PRELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRELEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "7 bytes of preamble"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PRELEN_A::_7)
    }
    #[doc = "5 bytes of preamble"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PRELEN_A::_5)
    }
    #[doc = "3 bytes of preamble"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRELEN_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Reader of field `TE`"]
pub type TE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TE`"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DC`"]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
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
#[doc = "Back-Off Limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BL_A {
    #[doc = "0: k = min (n,10)"]
    _1024 = 0,
    #[doc = "1: k = min (n,8)"]
    _256 = 1,
    #[doc = "2: k = min (n,4)"]
    _8 = 2,
    #[doc = "3: k = min (n,1)"]
    _2 = 3,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<u8, BL_A>;
impl BL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::_1024,
            1 => BL_A::_256,
            2 => BL_A::_8,
            3 => BL_A::_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == BL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == BL_A::_256
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == BL_A::_8
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == BL_A::_2
    }
}
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "k = min (n,10)"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(BL_A::_1024)
    }
    #[doc = "k = min (n,8)"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(BL_A::_256)
    }
    #[doc = "k = min (n,4)"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(BL_A::_8)
    }
    #[doc = "k = min (n,1)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BL_A::_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `ACS`"]
pub type ACS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACS`"]
pub struct ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_W<'a> {
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
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DR`"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `IPC`"]
pub type IPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPC`"]
pub struct IPC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DUPM`"]
pub type DUPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUPM`"]
pub struct DUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> DUPM_W<'a> {
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
#[doc = "Reader of field `LOOPBM`"]
pub type LOOPBM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBM`"]
pub struct LOOPBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBM_W<'a> {
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
#[doc = "Reader of field `DRO`"]
pub type DRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRO`"]
pub struct DRO_W<'a> {
    w: &'a mut W,
}
impl<'a> DRO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FES`"]
pub type FES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FES`"]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DISCRS`"]
pub type DISCRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCRS`"]
pub struct DISCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Inter-Frame Gap (IFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFG_A {
    #[doc = "0: 96 bit times"]
    _96 = 0,
    #[doc = "1: 88 bit times"]
    _88 = 1,
    #[doc = "2: 80 bit times"]
    _80 = 2,
    #[doc = "3: 72 bit times"]
    _72 = 3,
    #[doc = "4: 64 bit times"]
    _64 = 4,
    #[doc = "5: 56 bit times"]
    _56 = 5,
    #[doc = "6: 48 bit times"]
    _48 = 6,
    #[doc = "7: 40 bit times"]
    _40 = 7,
}
impl From<IFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IFG`"]
pub type IFG_R = crate::R<u8, IFG_A>;
impl IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFG_A {
        match self.bits {
            0 => IFG_A::_96,
            1 => IFG_A::_88,
            2 => IFG_A::_80,
            3 => IFG_A::_72,
            4 => IFG_A::_64,
            5 => IFG_A::_56,
            6 => IFG_A::_48,
            7 => IFG_A::_40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == IFG_A::_96
    }
    #[doc = "Checks if the value of the field is `_88`"]
    #[inline(always)]
    pub fn is_88(&self) -> bool {
        *self == IFG_A::_88
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == IFG_A::_80
    }
    #[doc = "Checks if the value of the field is `_72`"]
    #[inline(always)]
    pub fn is_72(&self) -> bool {
        *self == IFG_A::_72
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == IFG_A::_64
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline(always)]
    pub fn is_56(&self) -> bool {
        *self == IFG_A::_56
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == IFG_A::_48
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == IFG_A::_40
    }
}
#[doc = "Write proxy for field `IFG`"]
pub struct IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut W {
        self.variant(IFG_A::_96)
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn _88(self) -> &'a mut W {
        self.variant(IFG_A::_88)
    }
    #[doc = "80 bit times"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut W {
        self.variant(IFG_A::_80)
    }
    #[doc = "72 bit times"]
    #[inline(always)]
    pub fn _72(self) -> &'a mut W {
        self.variant(IFG_A::_72)
    }
    #[doc = "64 bit times"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(IFG_A::_64)
    }
    #[doc = "56 bit times"]
    #[inline(always)]
    pub fn _56(self) -> &'a mut W {
        self.variant(IFG_A::_56)
    }
    #[doc = "48 bit times"]
    #[inline(always)]
    pub fn _48(self) -> &'a mut W {
        self.variant(IFG_A::_48)
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(IFG_A::_40)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `JFEN`"]
pub type JFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JFEN`"]
pub struct JFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `JD`"]
pub type JD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JD`"]
pub struct JD_W<'a> {
    w: &'a mut W,
}
impl<'a> JD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `WDDIS`"]
pub type WDDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDDIS`"]
pub struct WDDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CST`"]
pub type CST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CST`"]
pub struct CST_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TWOKPEN`"]
pub type TWOKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWOKPEN`"]
pub struct TWOKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOKPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dupm(&self) -> DUPM_R {
        DUPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn loopbm(&self) -> LOOPBM_R {
        LOOPBM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn dro(&self) -> DRO_R {
        DRO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn discrs(&self) -> DISCRS_R {
        DISCRS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn jfen(&self) -> JFEN_R {
        JFEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&self) -> WDDIS_R {
        WDDIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn twokpen(&self) -> TWOKPEN_R {
        TWOKPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W {
        PRELEN_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W {
        ACS_W { w: self }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W {
        IPC_W { w: self }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dupm(&mut self) -> DUPM_W {
        DUPM_W { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn loopbm(&mut self) -> LOOPBM_W {
        LOOPBM_W { w: self }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn dro(&mut self) -> DRO_W {
        DRO_W { w: self }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn discrs(&mut self) -> DISCRS_W {
        DISCRS_W { w: self }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W { w: self }
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn jfen(&mut self) -> JFEN_W {
        JFEN_W { w: self }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&mut self) -> WDDIS_W {
        WDDIS_W { w: self }
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W {
        CST_W { w: self }
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn twokpen(&mut self) -> TWOKPEN_W {
        TWOKPEN_W { w: self }
    }
}
