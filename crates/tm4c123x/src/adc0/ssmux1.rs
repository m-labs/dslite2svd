#[doc = "Reader of register SSMUX1"]
pub type R = crate::R<u32, super::SSMUX1>;
#[doc = "Writer for register SSMUX1"]
pub type W = crate::W<u32, super::SSMUX1>;
#[doc = "Register SSMUX1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSMUX1 {
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
}
