#[doc = "Reader of register SSOP1"]
pub type R = crate::R<u32, super::SSOP1>;
#[doc = "Writer for register SSOP1"]
pub type W = crate::W<u32, super::SSOP1>;
#[doc = "Register SSOP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSOP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0DCOP`"]
pub type S0DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0DCOP`"]
pub struct S0DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DCOP_W<'a> {
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
#[doc = "Reader of field `S1DCOP`"]
pub type S1DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1DCOP`"]
pub struct S1DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S1DCOP_W<'a> {
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
#[doc = "Reader of field `S2DCOP`"]
pub type S2DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S2DCOP`"]
pub struct S2DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S2DCOP_W<'a> {
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
#[doc = "Reader of field `S3DCOP`"]
pub type S3DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3DCOP`"]
pub struct S3DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S3DCOP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s0dcop(&self) -> S0DCOP_R {
        S0DCOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s1dcop(&self) -> S1DCOP_R {
        S1DCOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s2dcop(&self) -> S2DCOP_R {
        S2DCOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s3dcop(&self) -> S3DCOP_R {
        S3DCOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s0dcop(&mut self) -> S0DCOP_W {
        S0DCOP_W { w: self }
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s1dcop(&mut self) -> S1DCOP_W {
        S1DCOP_W { w: self }
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s2dcop(&mut self) -> S2DCOP_W {
        S2DCOP_W { w: self }
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s3dcop(&mut self) -> S3DCOP_W {
        S3DCOP_W { w: self }
    }
}
