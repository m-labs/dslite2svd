#[doc = "Reader of register PLLFREQ1"]
pub type R = crate::R<u32, super::PLLFREQ1>;
#[doc = "Writer for register PLLFREQ1"]
pub type W = crate::W<u32, super::PLLFREQ1>;
#[doc = "Register PLLFREQ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLFREQ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `N`"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `Q`"]
pub type Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Q`"]
pub struct Q_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W { w: self }
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn q(&mut self) -> Q_W {
        Q_W { w: self }
    }
}
