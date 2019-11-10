#[doc = "Reader of register SSDC0"]
pub type R = crate::R<u32, super::SSDC0>;
#[doc = "Writer for register SSDC0"]
pub type W = crate::W<u32, super::SSDC0>;
#[doc = "Register SSDC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSDC0 {
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
#[doc = "Reader of field `S4DCSEL`"]
pub type S4DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S4DCSEL`"]
pub struct S4DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S4DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `S5DCSEL`"]
pub type S5DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S5DCSEL`"]
pub struct S5DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S5DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `S6DCSEL`"]
pub type S6DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S6DCSEL`"]
pub struct S6DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S6DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `S7DCSEL`"]
pub type S7DCSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S7DCSEL`"]
pub struct S7DCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S7DCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
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
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline(always)]
    pub fn s4dcsel(&self) -> S4DCSEL_R {
        S4DCSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline(always)]
    pub fn s5dcsel(&self) -> S5DCSEL_R {
        S5DCSEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline(always)]
    pub fn s6dcsel(&self) -> S6DCSEL_R {
        S6DCSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline(always)]
    pub fn s7dcsel(&self) -> S7DCSEL_R {
        S7DCSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline(always)]
    pub fn s4dcsel(&mut self) -> S4DCSEL_W {
        S4DCSEL_W { w: self }
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline(always)]
    pub fn s5dcsel(&mut self) -> S5DCSEL_W {
        S5DCSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline(always)]
    pub fn s6dcsel(&mut self) -> S6DCSEL_W {
        S6DCSEL_W { w: self }
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline(always)]
    pub fn s7dcsel(&mut self) -> S7DCSEL_W {
        S7DCSEL_W { w: self }
    }
}
