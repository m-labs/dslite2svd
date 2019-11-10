#[doc = "Reader of register SSOP0"]
pub type R = crate::R<u32, super::SSOP0>;
#[doc = "Writer for register SSOP0"]
pub type W = crate::W<u32, super::SSOP0>;
#[doc = "Register SSOP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSOP0 {
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
#[doc = "Reader of field `S4DCOP`"]
pub type S4DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4DCOP`"]
pub struct S4DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S4DCOP_W<'a> {
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
#[doc = "Reader of field `S5DCOP`"]
pub type S5DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5DCOP`"]
pub struct S5DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S5DCOP_W<'a> {
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
#[doc = "Reader of field `S6DCOP`"]
pub type S6DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S6DCOP`"]
pub struct S6DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S6DCOP_W<'a> {
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
#[doc = "Reader of field `S7DCOP`"]
pub type S7DCOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S7DCOP`"]
pub struct S7DCOP_W<'a> {
    w: &'a mut W,
}
impl<'a> S7DCOP_W<'a> {
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
    #[doc = "Bit 16 - Sample 4 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s4dcop(&self) -> S4DCOP_R {
        S4DCOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Sample 5 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s5dcop(&self) -> S5DCOP_R {
        S5DCOP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Sample 6 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s6dcop(&self) -> S6DCOP_R {
        S6DCOP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Sample 7 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s7dcop(&self) -> S7DCOP_R {
        S7DCOP_R::new(((self.bits >> 28) & 0x01) != 0)
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
    #[doc = "Bit 16 - Sample 4 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s4dcop(&mut self) -> S4DCOP_W {
        S4DCOP_W { w: self }
    }
    #[doc = "Bit 20 - Sample 5 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s5dcop(&mut self) -> S5DCOP_W {
        S5DCOP_W { w: self }
    }
    #[doc = "Bit 24 - Sample 6 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s6dcop(&mut self) -> S6DCOP_W {
        S6DCOP_W { w: self }
    }
    #[doc = "Bit 28 - Sample 7 Digital Comparator Operation"]
    #[inline(always)]
    pub fn s7dcop(&mut self) -> S7DCOP_W {
        S7DCOP_W { w: self }
    }
}
