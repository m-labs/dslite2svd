#[doc = "Reader of register SSMUX0"]
pub type R = crate::R<u32, super::SSMUX0>;
#[doc = "Writer for register SSMUX0"]
pub type W = crate::W<u32, super::SSMUX0>;
#[doc = "Register SSMUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSMUX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUX0`"]
pub type MUX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX0`"]
pub struct MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MUX1`"]
pub type MUX1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX1`"]
pub struct MUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MUX2`"]
pub type MUX2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX2`"]
pub struct MUX2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MUX3`"]
pub type MUX3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX3`"]
pub struct MUX3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MUX4`"]
pub type MUX4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX4`"]
pub struct MUX4_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MUX5`"]
pub type MUX5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX5`"]
pub struct MUX5_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MUX6`"]
pub type MUX6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX6`"]
pub struct MUX6_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MUX7`"]
pub type MUX7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX7`"]
pub struct MUX7_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn mux1(&self) -> MUX1_R {
        MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn mux2(&self) -> MUX2_R {
        MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn mux3(&self) -> MUX3_R {
        MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn mux4(&self) -> MUX4_R {
        MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn mux6(&self) -> MUX6_R {
        MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn mux7(&self) -> MUX7_R {
        MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn mux0(&mut self) -> MUX0_W {
        MUX0_W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn mux1(&mut self) -> MUX1_W {
        MUX1_W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn mux2(&mut self) -> MUX2_W {
        MUX2_W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn mux3(&mut self) -> MUX3_W {
        MUX3_W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn mux4(&mut self) -> MUX4_W {
        MUX4_W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn mux5(&mut self) -> MUX5_W {
        MUX5_W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn mux6(&mut self) -> MUX6_W {
        MUX6_W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn mux7(&mut self) -> MUX7_W {
        MUX7_W { w: self }
    }
}
