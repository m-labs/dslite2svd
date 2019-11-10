#[doc = "Reader of register SSCTL0"]
pub type R = crate::R<u32, super::SSCTL0>;
#[doc = "Writer for register SSCTL0"]
pub type W = crate::W<u32, super::SSCTL0>;
#[doc = "Register SSCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D0`"]
pub type D0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D0`"]
pub struct D0_W<'a> {
    w: &'a mut W,
}
impl<'a> D0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `END0`"]
pub type END0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END0`"]
pub struct END0_W<'a> {
    w: &'a mut W,
}
impl<'a> END0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `IE0`"]
pub type IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE0`"]
pub struct IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IE0_W<'a> {
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
#[doc = "Reader of field `TS0`"]
pub type TS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS0`"]
pub struct TS0_W<'a> {
    w: &'a mut W,
}
impl<'a> TS0_W<'a> {
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
#[doc = "Reader of field `D1`"]
pub type D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D1`"]
pub struct D1_W<'a> {
    w: &'a mut W,
}
impl<'a> D1_W<'a> {
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
#[doc = "Reader of field `END1`"]
pub type END1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END1`"]
pub struct END1_W<'a> {
    w: &'a mut W,
}
impl<'a> END1_W<'a> {
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
#[doc = "Reader of field `IE1`"]
pub type IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE1`"]
pub struct IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IE1_W<'a> {
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
#[doc = "Reader of field `TS1`"]
pub type TS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1`"]
pub struct TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_W<'a> {
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
#[doc = "Reader of field `D2`"]
pub type D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D2`"]
pub struct D2_W<'a> {
    w: &'a mut W,
}
impl<'a> D2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `END2`"]
pub type END2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END2`"]
pub struct END2_W<'a> {
    w: &'a mut W,
}
impl<'a> END2_W<'a> {
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
#[doc = "Reader of field `IE2`"]
pub type IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE2`"]
pub struct IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IE2_W<'a> {
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
#[doc = "Reader of field `TS2`"]
pub type TS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS2`"]
pub struct TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS2_W<'a> {
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
#[doc = "Reader of field `D3`"]
pub type D3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D3`"]
pub struct D3_W<'a> {
    w: &'a mut W,
}
impl<'a> D3_W<'a> {
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
#[doc = "Reader of field `END3`"]
pub type END3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END3`"]
pub struct END3_W<'a> {
    w: &'a mut W,
}
impl<'a> END3_W<'a> {
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
#[doc = "Reader of field `IE3`"]
pub type IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE3`"]
pub struct IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IE3_W<'a> {
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
#[doc = "Reader of field `TS3`"]
pub type TS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS3`"]
pub struct TS3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS3_W<'a> {
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
#[doc = "Reader of field `D4`"]
pub type D4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D4`"]
pub struct D4_W<'a> {
    w: &'a mut W,
}
impl<'a> D4_W<'a> {
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
#[doc = "Reader of field `END4`"]
pub type END4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END4`"]
pub struct END4_W<'a> {
    w: &'a mut W,
}
impl<'a> END4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `IE4`"]
pub type IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE4`"]
pub struct IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> IE4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TS4`"]
pub type TS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS4`"]
pub struct TS4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `D5`"]
pub type D5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D5`"]
pub struct D5_W<'a> {
    w: &'a mut W,
}
impl<'a> D5_W<'a> {
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
#[doc = "Reader of field `END5`"]
pub type END5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END5`"]
pub struct END5_W<'a> {
    w: &'a mut W,
}
impl<'a> END5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `IE5`"]
pub type IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE5`"]
pub struct IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> IE5_W<'a> {
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
#[doc = "Reader of field `TS5`"]
pub type TS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS5`"]
pub struct TS5_W<'a> {
    w: &'a mut W,
}
impl<'a> TS5_W<'a> {
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
#[doc = "Reader of field `D6`"]
pub type D6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D6`"]
pub struct D6_W<'a> {
    w: &'a mut W,
}
impl<'a> D6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `END6`"]
pub type END6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END6`"]
pub struct END6_W<'a> {
    w: &'a mut W,
}
impl<'a> END6_W<'a> {
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
#[doc = "Reader of field `IE6`"]
pub type IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE6`"]
pub struct IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> IE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TS6`"]
pub type TS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS6`"]
pub struct TS6_W<'a> {
    w: &'a mut W,
}
impl<'a> TS6_W<'a> {
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
#[doc = "Reader of field `D7`"]
pub type D7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D7`"]
pub struct D7_W<'a> {
    w: &'a mut W,
}
impl<'a> D7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `END7`"]
pub type END7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END7`"]
pub struct END7_W<'a> {
    w: &'a mut W,
}
impl<'a> END7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IE7`"]
pub type IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE7`"]
pub struct IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> IE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TS7`"]
pub type TS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS7`"]
pub struct TS7_W<'a> {
    w: &'a mut W,
}
impl<'a> TS7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn end0(&self) -> END0_R {
        END0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie0(&self) -> IE0_R {
        IE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts0(&self) -> TS0_R {
        TS0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn end1(&self) -> END1_R {
        END1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie1(&self) -> IE1_R {
        IE1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn end2(&self) -> END2_R {
        END2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie2(&self) -> IE2_R {
        IE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end3(&self) -> END3_R {
        END3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie3(&self) -> IE3_R {
        IE3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d4(&self) -> D4_R {
        D4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end4(&self) -> END4_R {
        END4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie4(&self) -> IE4_R {
        IE4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end5(&self) -> END5_R {
        END5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie5(&self) -> IE5_R {
        IE5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts5(&self) -> TS5_R {
        TS5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d6(&self) -> D6_R {
        D6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end6(&self) -> END6_R {
        END6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie6(&self) -> IE6_R {
        IE6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts6(&self) -> TS6_R {
        TS6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d7(&self) -> D7_R {
        D7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end7(&self) -> END7_R {
        END7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie7(&self) -> IE7_R {
        IE7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts7(&self) -> TS7_R {
        TS7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W {
        D0_W { w: self }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn end0(&mut self) -> END0_W {
        END0_W { w: self }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W {
        IE0_W { w: self }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts0(&mut self) -> TS0_W {
        TS0_W { w: self }
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn d1(&mut self) -> D1_W {
        D1_W { w: self }
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn end1(&mut self) -> END1_W {
        END1_W { w: self }
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie1(&mut self) -> IE1_W {
        IE1_W { w: self }
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W {
        D2_W { w: self }
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn end2(&mut self) -> END2_W {
        END2_W { w: self }
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie2(&mut self) -> IE2_W {
        IE2_W { w: self }
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W {
        D3_W { w: self }
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end3(&mut self) -> END3_W {
        END3_W { w: self }
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie3(&mut self) -> IE3_W {
        IE3_W { w: self }
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts3(&mut self) -> TS3_W {
        TS3_W { w: self }
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W {
        D4_W { w: self }
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end4(&mut self) -> END4_W {
        END4_W { w: self }
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie4(&mut self) -> IE4_W {
        IE4_W { w: self }
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts4(&mut self) -> TS4_W {
        TS4_W { w: self }
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W {
        D5_W { w: self }
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end5(&mut self) -> END5_W {
        END5_W { w: self }
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie5(&mut self) -> IE5_W {
        IE5_W { w: self }
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts5(&mut self) -> TS5_W {
        TS5_W { w: self }
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d6(&mut self) -> D6_W {
        D6_W { w: self }
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end6(&mut self) -> END6_W {
        END6_W { w: self }
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie6(&mut self) -> IE6_W {
        IE6_W { w: self }
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts6(&mut self) -> TS6_W {
        TS6_W { w: self }
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn d7(&mut self) -> D7_W {
        D7_W { w: self }
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn end7(&mut self) -> END7_W {
        END7_W { w: self }
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn ie7(&mut self) -> IE7_W {
        IE7_W { w: self }
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn ts7(&mut self) -> TS7_W {
        TS7_W { w: self }
    }
}
