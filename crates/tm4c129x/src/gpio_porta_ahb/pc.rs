#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Writer for register PC"]
pub type W = crate::W<u32, super::PC>;
#[doc = "Register PC `reset()`'s with value 0"]
impl crate::ResetValue for super::PC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Extended Drive Mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDM0_A {
    #[doc = "0: Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    DISABLE,
    #[doc = "1: An additional 6 mA option is provided"]
    _6MA,
    #[doc = "3: A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    PLUS2MA,
}
impl From<EDM0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDM0_A) -> Self {
        match variant {
            EDM0_A::DISABLE => 0,
            EDM0_A::_6MA => 1,
            EDM0_A::PLUS2MA => 3,
        }
    }
}
#[doc = "Reader of field `EDM0`"]
pub type EDM0_R = crate::R<u8, EDM0_A>;
impl EDM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EDM0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EDM0_A::DISABLE),
            1 => Val(EDM0_A::_6MA),
            3 => Val(EDM0_A::PLUS2MA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDM0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_6MA`"]
    #[inline(always)]
    pub fn is_6ma(&self) -> bool {
        *self == EDM0_A::_6MA
    }
    #[doc = "Checks if the value of the field is `PLUS2MA`"]
    #[inline(always)]
    pub fn is_plus2ma(&self) -> bool {
        *self == EDM0_A::PLUS2MA
    }
}
#[doc = "Write proxy for field `EDM0`"]
pub struct EDM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDM0_A::DISABLE)
    }
    #[doc = "An additional 6 mA option is provided"]
    #[inline(always)]
    pub fn _6ma(self) -> &'a mut W {
        self.variant(EDM0_A::_6MA)
    }
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    #[inline(always)]
    pub fn plus2ma(self) -> &'a mut W {
        self.variant(EDM0_A::PLUS2MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `EDM1`"]
pub type EDM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM1`"]
pub struct EDM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `EDM2`"]
pub type EDM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM2`"]
pub struct EDM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EDM3`"]
pub type EDM3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM3`"]
pub struct EDM3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EDM4`"]
pub type EDM4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM4`"]
pub struct EDM4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EDM5`"]
pub type EDM5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM5`"]
pub struct EDM5_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `EDM6`"]
pub type EDM6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM6`"]
pub struct EDM6_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `EDM7`"]
pub type EDM7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDM7`"]
pub struct EDM7_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn edm0(&self) -> EDM0_R {
        EDM0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn edm1(&self) -> EDM1_R {
        EDM1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn edm2(&self) -> EDM2_R {
        EDM2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn edm3(&self) -> EDM3_R {
        EDM3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn edm4(&self) -> EDM4_R {
        EDM4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn edm5(&self) -> EDM5_R {
        EDM5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn edm6(&self) -> EDM6_R {
        EDM6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn edm7(&self) -> EDM7_R {
        EDM7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn edm0(&mut self) -> EDM0_W {
        EDM0_W { w: self }
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn edm1(&mut self) -> EDM1_W {
        EDM1_W { w: self }
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn edm2(&mut self) -> EDM2_W {
        EDM2_W { w: self }
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn edm3(&mut self) -> EDM3_W {
        EDM3_W { w: self }
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn edm4(&mut self) -> EDM4_W {
        EDM4_W { w: self }
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn edm5(&mut self) -> EDM5_W {
        EDM5_W { w: self }
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn edm6(&mut self) -> EDM6_W {
        EDM6_W { w: self }
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn edm7(&mut self) -> EDM7_W {
        EDM7_W { w: self }
    }
}
