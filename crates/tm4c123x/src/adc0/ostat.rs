#[doc = "Reader of register OSTAT"]
pub type R = crate::R<u32, super::OSTAT>;
#[doc = "Writer for register OSTAT"]
pub type W = crate::W<u32, super::OSTAT>;
#[doc = "Register OSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OV0`"]
pub type OV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV0`"]
pub struct OV0_W<'a> {
    w: &'a mut W,
}
impl<'a> OV0_W<'a> {
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
#[doc = "Reader of field `OV1`"]
pub type OV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV1`"]
pub struct OV1_W<'a> {
    w: &'a mut W,
}
impl<'a> OV1_W<'a> {
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
#[doc = "Reader of field `OV2`"]
pub type OV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV2`"]
pub struct OV2_W<'a> {
    w: &'a mut W,
}
impl<'a> OV2_W<'a> {
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
#[doc = "Reader of field `OV3`"]
pub type OV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OV3`"]
pub struct OV3_W<'a> {
    w: &'a mut W,
}
impl<'a> OV3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn ov0(&self) -> OV0_R {
        OV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn ov1(&self) -> OV1_R {
        OV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn ov2(&self) -> OV2_R {
        OV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn ov3(&self) -> OV3_R {
        OV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn ov0(&mut self) -> OV0_W {
        OV0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn ov1(&mut self) -> OV1_W {
        OV1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn ov2(&mut self) -> OV2_W {
        OV2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn ov3(&mut self) -> OV3_W {
        OV3_W { w: self }
    }
}
