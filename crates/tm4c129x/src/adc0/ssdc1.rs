#[doc = "Reader of register SSDC1"]
pub type R = crate::R<u32, super::SSDC1>;
#[doc = "Writer for register SSDC1"]
pub type W = crate::W<u32, super::SSDC1>;
#[doc = "Register SSDC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSDC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S0DCSEL`"]
pub type S0DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0DCSEL`"]
pub struct S0DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `S1DCSEL`"]
pub type S1DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1DCSEL`"]
pub struct S1DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S1DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `S2DCSEL`"]
pub type S2DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S2DCSEL`"]
pub struct S2DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S2DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `S3DCSEL`"]
pub type S3DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S3DCSEL`"]
pub struct S3DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S3DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn s0dcsel(&self) -> S0DCSEL_R {
        S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn s1dcsel(&self) -> S1DCSEL_R {
        S1DCSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn s2dcsel(&self) -> S2DCSEL_R {
        S2DCSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn s3dcsel(&self) -> S3DCSEL_R {
        S3DCSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn s0dcsel(&mut self) -> S0DCSEL_W {
        S0DCSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn s1dcsel(&mut self) -> S1DCSEL_W {
        S1DCSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn s2dcsel(&mut self) -> S2DCSEL_W {
        S2DCSEL_W { w: self }
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn s3dcsel(&mut self) -> S3DCSEL_W {
        S3DCSEL_W { w: self }
    }
}
