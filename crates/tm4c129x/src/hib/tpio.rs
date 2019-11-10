#[doc = "Reader of register TPIO"]
pub type R = crate::R<u32, super::TPIO>;
#[doc = "Writer for register TPIO"]
pub type W = crate::W<u32, super::TPIO>;
#[doc = "Register TPIO `reset()`'s with value 0"]
impl crate::ResetValue for super::TPIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN0`"]
pub type EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN0`"]
pub struct EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN0_W<'a> {
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
#[doc = "Reader of field `LEV0`"]
pub type LEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV0`"]
pub struct LEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV0_W<'a> {
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
#[doc = "Reader of field `PUEN0`"]
pub type PUEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUEN0`"]
pub struct PUEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUEN0_W<'a> {
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
#[doc = "Reader of field `GFLTR0`"]
pub type GFLTR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLTR0`"]
pub struct GFLTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLTR0_W<'a> {
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
#[doc = "Reader of field `EN1`"]
pub type EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN1`"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
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
#[doc = "Reader of field `LEV1`"]
pub type LEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV1`"]
pub struct LEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV1_W<'a> {
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
#[doc = "Reader of field `PUEN1`"]
pub type PUEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUEN1`"]
pub struct PUEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUEN1_W<'a> {
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
#[doc = "Reader of field `GFLTR1`"]
pub type GFLTR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLTR1`"]
pub struct GFLTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLTR1_W<'a> {
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
#[doc = "Reader of field `EN2`"]
pub type EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN2`"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
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
#[doc = "Reader of field `LEV2`"]
pub type LEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV2`"]
pub struct LEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV2_W<'a> {
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
#[doc = "Reader of field `PUEN2`"]
pub type PUEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUEN2`"]
pub struct PUEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUEN2_W<'a> {
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
#[doc = "Reader of field `GFLTR2`"]
pub type GFLTR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLTR2`"]
pub struct GFLTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLTR2_W<'a> {
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
#[doc = "Reader of field `EN3`"]
pub type EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN3`"]
pub struct EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN3_W<'a> {
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
#[doc = "Reader of field `LEV3`"]
pub type LEV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV3`"]
pub struct LEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV3_W<'a> {
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
#[doc = "Reader of field `PUEN3`"]
pub type PUEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUEN3`"]
pub struct PUEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUEN3_W<'a> {
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
#[doc = "Reader of field `GFLTR3`"]
pub type GFLTR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLTR3`"]
pub struct GFLTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLTR3_W<'a> {
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
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn lev0(&self) -> LEV0_R {
        LEV0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen0(&self) -> PUEN0_R {
        PUEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr0(&self) -> GFLTR0_R {
        GFLTR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn lev1(&self) -> LEV1_R {
        LEV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen1(&self) -> PUEN1_R {
        PUEN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr1(&self) -> GFLTR1_R {
        GFLTR1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn lev2(&self) -> LEV2_R {
        LEV2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen2(&self) -> PUEN2_R {
        PUEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr2(&self) -> GFLTR2_R {
        GFLTR2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn lev3(&self) -> LEV3_R {
        LEV3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen3(&self) -> PUEN3_R {
        PUEN3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr3(&self) -> GFLTR3_R {
        GFLTR3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W {
        EN0_W { w: self }
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn lev0(&mut self) -> LEV0_W {
        LEV0_W { w: self }
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen0(&mut self) -> PUEN0_W {
        PUEN0_W { w: self }
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr0(&mut self) -> GFLTR0_W {
        GFLTR0_W { w: self }
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn lev1(&mut self) -> LEV1_W {
        LEV1_W { w: self }
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen1(&mut self) -> PUEN1_W {
        PUEN1_W { w: self }
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr1(&mut self) -> GFLTR1_W {
        GFLTR1_W { w: self }
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn lev2(&mut self) -> LEV2_W {
        LEV2_W { w: self }
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen2(&mut self) -> PUEN2_W {
        PUEN2_W { w: self }
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr2(&mut self) -> GFLTR2_W {
        GFLTR2_W { w: self }
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W {
        EN3_W { w: self }
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn lev3(&mut self) -> LEV3_W {
        LEV3_W { w: self }
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn puen3(&mut self) -> PUEN3_W {
        PUEN3_W { w: self }
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn gfltr3(&mut self) -> GFLTR3_W {
        GFLTR3_W { w: self }
    }
}
