#[doc = "Reader of register PCTL"]
pub type R = crate::R<u32, super::PCTL>;
#[doc = "Writer for register PCTL"]
pub type W = crate::W<u32, super::PCTL>;
#[doc = "Register PCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMC0`"]
pub type PMC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC0`"]
pub struct PMC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PMC1`"]
pub type PMC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC1`"]
pub struct PMC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PMC2`"]
pub type PMC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC2`"]
pub struct PMC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PMC3`"]
pub type PMC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC3`"]
pub struct PMC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PMC4`"]
pub type PMC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC4`"]
pub struct PMC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PMC5`"]
pub type PMC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC5`"]
pub struct PMC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PMC6`"]
pub type PMC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC6`"]
pub struct PMC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PMC7`"]
pub type PMC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMC7`"]
pub struct PMC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Port Mux Control 0"]
    #[inline(always)]
    pub fn pmc0(&self) -> PMC0_R {
        PMC0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Port Mux Control 1"]
    #[inline(always)]
    pub fn pmc1(&self) -> PMC1_R {
        PMC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Port Mux Control 2"]
    #[inline(always)]
    pub fn pmc2(&self) -> PMC2_R {
        PMC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Port Mux Control 3"]
    #[inline(always)]
    pub fn pmc3(&self) -> PMC3_R {
        PMC3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Port Mux Control 4"]
    #[inline(always)]
    pub fn pmc4(&self) -> PMC4_R {
        PMC4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Port Mux Control 5"]
    #[inline(always)]
    pub fn pmc5(&self) -> PMC5_R {
        PMC5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Port Mux Control 6"]
    #[inline(always)]
    pub fn pmc6(&self) -> PMC6_R {
        PMC6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Port Mux Control 7"]
    #[inline(always)]
    pub fn pmc7(&self) -> PMC7_R {
        PMC7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Port Mux Control 0"]
    #[inline(always)]
    pub fn pmc0(&mut self) -> PMC0_W {
        PMC0_W { w: self }
    }
    #[doc = "Bits 4:7 - Port Mux Control 1"]
    #[inline(always)]
    pub fn pmc1(&mut self) -> PMC1_W {
        PMC1_W { w: self }
    }
    #[doc = "Bits 8:11 - Port Mux Control 2"]
    #[inline(always)]
    pub fn pmc2(&mut self) -> PMC2_W {
        PMC2_W { w: self }
    }
    #[doc = "Bits 12:15 - Port Mux Control 3"]
    #[inline(always)]
    pub fn pmc3(&mut self) -> PMC3_W {
        PMC3_W { w: self }
    }
    #[doc = "Bits 16:19 - Port Mux Control 4"]
    #[inline(always)]
    pub fn pmc4(&mut self) -> PMC4_W {
        PMC4_W { w: self }
    }
    #[doc = "Bits 20:23 - Port Mux Control 5"]
    #[inline(always)]
    pub fn pmc5(&mut self) -> PMC5_W {
        PMC5_W { w: self }
    }
    #[doc = "Bits 24:27 - Port Mux Control 6"]
    #[inline(always)]
    pub fn pmc6(&mut self) -> PMC6_W {
        PMC6_W { w: self }
    }
    #[doc = "Bits 28:31 - Port Mux Control 7"]
    #[inline(always)]
    pub fn pmc7(&mut self) -> PMC7_W {
        PMC7_W { w: self }
    }
}
