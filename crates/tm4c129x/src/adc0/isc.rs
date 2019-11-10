#[doc = "Reader of register ISC"]
pub type R = crate::R<u32, super::ISC>;
#[doc = "Writer for register ISC"]
pub type W = crate::W<u32, super::ISC>;
#[doc = "Register ISC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
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
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
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
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
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
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
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
#[doc = "Reader of field `DMAIN0`"]
pub type DMAIN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIN0`"]
pub struct DMAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIN0_W<'a> {
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
#[doc = "Reader of field `DMAIN1`"]
pub type DMAIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIN1`"]
pub struct DMAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIN1_W<'a> {
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
#[doc = "Reader of field `DMAIN2`"]
pub type DMAIN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIN2`"]
pub struct DMAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIN2_W<'a> {
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
#[doc = "Reader of field `DMAIN3`"]
pub type DMAIN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIN3`"]
pub struct DMAIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIN3_W<'a> {
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
#[doc = "Reader of field `DCINSS0`"]
pub type DCINSS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINSS0`"]
pub struct DCINSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINSS0_W<'a> {
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
#[doc = "Reader of field `DCINSS1`"]
pub type DCINSS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINSS1`"]
pub struct DCINSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINSS1_W<'a> {
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
#[doc = "Reader of field `DCINSS2`"]
pub type DCINSS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINSS2`"]
pub struct DCINSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINSS2_W<'a> {
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
#[doc = "Reader of field `DCINSS3`"]
pub type DCINSS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCINSS3`"]
pub struct DCINSS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCINSS3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain0(&self) -> DMAIN0_R {
        DMAIN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain1(&self) -> DMAIN1_R {
        DMAIN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain2(&self) -> DMAIN2_R {
        DMAIN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain3(&self) -> DMAIN3_R {
        DMAIN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn dcinss0(&self) -> DCINSS0_R {
        DCINSS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn dcinss1(&self) -> DCINSS1_R {
        DCINSS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn dcinss2(&self) -> DCINSS2_R {
        DCINSS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn dcinss3(&self) -> DCINSS3_R {
        DCINSS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain0(&mut self) -> DMAIN0_W {
        DMAIN0_W { w: self }
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain1(&mut self) -> DMAIN1_W {
        DMAIN1_W { w: self }
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain2(&mut self) -> DMAIN2_W {
        DMAIN2_W { w: self }
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn dmain3(&mut self) -> DMAIN3_W {
        DMAIN3_W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn dcinss0(&mut self) -> DCINSS0_W {
        DCINSS0_W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn dcinss1(&mut self) -> DCINSS1_W {
        DCINSS1_W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn dcinss2(&mut self) -> DCINSS2_W {
        DCINSS2_W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn dcinss3(&mut self) -> DCINSS3_W {
        DCINSS3_W { w: self }
    }
}
