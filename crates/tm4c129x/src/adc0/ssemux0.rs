#[doc = "Reader of register SSEMUX0"]
pub type R = crate::R<u32, super::SSEMUX0>;
#[doc = "Writer for register SSEMUX0"]
pub type W = crate::W<u32, super::SSEMUX0>;
#[doc = "Register SSEMUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSEMUX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMUX0`"]
pub type EMUX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX0`"]
pub struct EMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX0_W<'a> {
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
#[doc = "Reader of field `EMUX1`"]
pub type EMUX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX1`"]
pub struct EMUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX1_W<'a> {
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
#[doc = "Reader of field `EMUX2`"]
pub type EMUX2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX2`"]
pub struct EMUX2_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX2_W<'a> {
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
#[doc = "Reader of field `EMUX3`"]
pub type EMUX3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX3`"]
pub struct EMUX3_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX3_W<'a> {
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
#[doc = "Reader of field `EMUX4`"]
pub type EMUX4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX4`"]
pub struct EMUX4_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX4_W<'a> {
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
#[doc = "Reader of field `EMUX5`"]
pub type EMUX5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX5`"]
pub struct EMUX5_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX5_W<'a> {
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
#[doc = "Reader of field `EMUX6`"]
pub type EMUX6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX6`"]
pub struct EMUX6_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX6_W<'a> {
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
#[doc = "Reader of field `EMUX7`"]
pub type EMUX7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMUX7`"]
pub struct EMUX7_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUX7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux0(&self) -> EMUX0_R {
        EMUX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux1(&self) -> EMUX1_R {
        EMUX1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux2(&self) -> EMUX2_R {
        EMUX2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux3(&self) -> EMUX3_R {
        EMUX3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux4(&self) -> EMUX4_R {
        EMUX4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux5(&self) -> EMUX5_R {
        EMUX5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux6(&self) -> EMUX6_R {
        EMUX6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux7(&self) -> EMUX7_R {
        EMUX7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux0(&mut self) -> EMUX0_W {
        EMUX0_W { w: self }
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux1(&mut self) -> EMUX1_W {
        EMUX1_W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux2(&mut self) -> EMUX2_W {
        EMUX2_W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux3(&mut self) -> EMUX3_W {
        EMUX3_W { w: self }
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux4(&mut self) -> EMUX4_W {
        EMUX4_W { w: self }
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux5(&mut self) -> EMUX5_W {
        EMUX5_W { w: self }
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux6(&mut self) -> EMUX6_W {
        EMUX6_W { w: self }
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn emux7(&mut self) -> EMUX7_W {
        EMUX7_W { w: self }
    }
}
